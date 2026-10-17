use pklx::pklr::EvalOptions;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Cursor, Write};
use std::net::TcpListener;
use std::thread;
use std::time::{Duration, Instant};

// Loopback only, with bounded accept/read waits even if evaluation fails.
fn serve(body: Vec<u8>) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let (stream, _) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "no evaluator request");
                    thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("accept failed: {error}"),
            }
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut stream = BufReader::new(stream);
        let mut request = String::new();
        stream.read_line(&mut request).unwrap();
        loop {
            let mut header = String::new();
            assert_ne!(stream.read_line(&mut header).unwrap(), 0);
            if header == "\r\n" {
                break;
            }
        }
        write!(
            stream.get_mut(),
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )
        .unwrap();
        stream.get_mut().write_all(&body).unwrap();
        request
    });
    (address, server)
}

fn rewritten_options(address: &str) -> EvalOptions {
    EvalOptions {
        agent: Some(
            pklx::pklr::ureq::Agent::config_builder()
                .proxy(None)
                .timeout_global(Some(Duration::from_secs(5)))
                .build()
                .into(),
        ),
        http_rewrites: vec![format!("https://pklx.invalid/={address}/")],
    }
}

#[tokio::test]
async fn local_imports_and_typed_values_survive_the_evaluator_upgrade() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        name: String,
        ports: Vec<u16>,
    }
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("child.pkl"),
        "name = \"worker\"\nports = List(22, 1337)\n",
    )
    .unwrap();
    let file = directory.path().join("main.pkl");
    std::fs::write(
        &file,
        "import \"child.pkl\" as child\nname = child.name\nports = child.ports\n",
    )
    .unwrap();
    let actual: Config = pklx::eval_to_typed(&file, EvalOptions::default())
        .await
        .unwrap();
    assert_eq!(
        actual,
        Config {
            name: "worker".into(),
            ports: vec![22, 1337]
        }
    );
    assert_eq!(
        pklx::analyze_pkl_imports(&file).unwrap(),
        vec![directory.path().join("child.pkl")]
    );
}

#[tokio::test]
async fn class_metadata_and_data_only_output_are_preserved() {
    let source = "class Entry { name: String }\nentry = new Entry { name = \"worker\" }\nmissing: String? = null\n";
    let normal = pklx::eval_pkl_source(source, EvalOptions::default())
        .await
        .unwrap();
    assert!(normal.contains("__pkl_class = \"Entry\""), "{normal}");
    assert!(normal.contains("missing = null"), "{normal}");
    let data = pklx::eval_pkl_source_with_serializer_options(
        source,
        EvalOptions::default(),
        pklx::SerializeOptions::data_only(),
    )
    .await
    .unwrap();
    assert_eq!(data, "{ entry = { name = \"worker\"; }; }");
}

#[tokio::test]
async fn http_rewrites_and_custom_agents_reach_only_the_local_fixture() {
    let (address, server) = serve(b"answer = 42\n".to_vec());
    let value: serde_json::Value = pklx::eval_source_to_typed(
        "import \"https://pklx.invalid/Config.pkl\" as remote\nanswer = remote.answer\n",
        rewritten_options(&address),
    )
    .await
    .unwrap();
    assert_eq!(value, serde_json::json!({"answer": 42}));
    assert_eq!(server.join().unwrap(), "GET /Config.pkl HTTP/1.1\r\n");
}

#[tokio::test]
async fn package_zip_imports_use_the_fixed_archive_reader() {
    let mut archive = zip::ZipWriter::new(Cursor::new(Vec::new()));
    archive
        .start_file("Config.pkl", zip::write::SimpleFileOptions::default())
        .unwrap();
    archive.write_all(b"answer = 42\n").unwrap();
    let (address, server) = serve(archive.finish().unwrap().into_inner());
    let value: serde_json::Value = pklx::eval_source_to_typed("import \"package://pklx.invalid/pkg@1.0.0#/Config.pkl\" as package\nanswer = package.answer\n", rewritten_options(&address)).await.unwrap();
    assert_eq!(value, serde_json::json!({"answer": 42}));
    assert_eq!(server.join().unwrap(), "GET /pkg@1.0.0.zip HTTP/1.1\r\n");
}

#[test]
fn offline_evaluation_rejects_uncached_package_downloads() {
    let cache = tempfile::tempdir().unwrap();
    let mut evaluator = pklx::pklr::EvaluatorBuilder::new()
        .package_cache_dir(cache.path())
        .offline(true)
        .build();
    let error = evaluator.eval_source("import \"package://pklx.invalid/pkg@1.0.0#/Config.pkl\" as package\nanswer = package.answer\n", std::path::Path::new("<expr>")).unwrap_err();
    assert!(
        error.to_string().to_lowercase().contains("offline"),
        "{error}"
    );
}
