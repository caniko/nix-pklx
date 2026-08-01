use std::path::PathBuf;

use pklx::{eval_to_typed, eval_to_value, from_pkl_value, pklr};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    schema_version: u64,
    archive_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Header {
    schema_version: u64,
}

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/typed-with-import.pkl")
}

fn supported_header(value: &pklr::Value) -> Result<Header, String> {
    let header: Header = from_pkl_value(value).map_err(|error| error.to_string())?;
    if header.schema_version != 1 {
        return Err(format!(
            "unsupported schema version {}; expected 1",
            header.schema_version
        ));
    }
    Ok(header)
}

#[tokio::test]
async fn eval_to_typed_loads_relative_imports() {
    let manifest: Manifest = eval_to_typed(&fixture(), pklr::EvalOptions::default())
        .await
        .unwrap();

    assert_eq!(
        manifest,
        Manifest {
            schema_version: 2,
            archive_name: "photos".into(),
        }
    );
}

#[tokio::test]
async fn consumer_header_identifies_unsupported_schema_version() {
    let value = eval_to_value(&fixture(), pklr::EvalOptions::default())
        .await
        .unwrap();

    assert_eq!(
        supported_header(&value).unwrap_err(),
        "unsupported schema version 2; expected 1"
    );
}
