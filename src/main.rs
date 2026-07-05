use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pklx", version, about = "Pkl ↔ Nix interop tool")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Evaluate a .pkl file or expression and emit a Nix expression
    #[command(group(
        clap::ArgGroup::new("source")
            .required(true)
            .args(&["file", "expr"])
    ))]
    Eval {
        /// Path to the .pkl file (mutually exclusive with --expr)
        file: Option<std::path::PathBuf>,

        /// Evaluate a Pkl expression string instead of a file
        #[arg(long)]
        expr: Option<String>,

        /// HTTP URL rewrite rules in "source_prefix=target_prefix" format
        #[arg(long = "http-rewrite")]
        http_rewrite: Vec<String>,

        /// HTTP proxy URL (e.g. http://proxy:8080)
        #[arg(long = "http-proxy")]
        http_proxy: Option<String>,

        /// Wrap the output in a NixOS module skeleton
        #[arg(long)]
        module: bool,

        /// Emit data-only Nix without Pkl class metadata or null fields
        #[arg(long = "data-only")]
        data_only: bool,

        /// Write output to file instead of stdout
        #[arg(short, long)]
        output: Option<std::path::PathBuf>,
    },

    /// Convert a Nix-like expression to Pkl syntax
    #[command(name = "to-pkl")]
    ToPkl {
        /// Nix expression string
        expr: String,
    },

    /// Analyze imports of a .pkl file
    Analyze {
        /// Path to the .pkl file
        file: std::path::PathBuf,
    },
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Eval {
            file,
            expr,
            http_rewrite,
            http_proxy,
            module,
            data_only,
            output,
        } => {
            let mut options = pklr::EvalOptions::default();

            if !http_rewrite.is_empty() {
                options.http_rewrites = http_rewrite;
            }

            if let Some(proxy_url) = http_proxy {
                let proxy = pklr::reqwest::Proxy::all(&proxy_url)
                    .map_err(|e| miette::miette!("Invalid proxy URL '{}': {}", proxy_url, e))?;
                let client = pklr::reqwest::Client::builder()
                    .proxy(proxy)
                    .build()
                    .map_err(|e| miette::miette!("Failed to build HTTP client: {}", e))?;
                options.client = Some(client);
            }

            let serializer_options = if data_only {
                pklx::SerializeOptions::data_only()
            } else {
                pklx::SerializeOptions::default()
            };

            let nix = if let Some(source) = expr {
                pklx::eval_pkl_source_with_serializer_options(&source, options, serializer_options)
                    .await?
            } else {
                let file = file.expect("clap ensures --expr or file is present");
                pklx::eval_pkl_with_serializer_options(&file, options, serializer_options).await?
            };

            let result = if module {
                let inner = nix.trim();
                let inner = if inner.starts_with('{') && inner.ends_with('}') && inner.len() >= 2 {
                    inner[1..inner.len() - 1].trim()
                } else {
                    inner
                };
                format!("{{ config, pkgs, ... }}: {{\n  {}\n}}", inner)
            } else {
                nix
            };

            if let Some(path) = output {
                std::fs::write(&path, &result).map_err(|e| {
                    miette::miette!("Failed to write output file '{}': {}", path.display(), e)
                })?;
            } else {
                println!("{}", result);
            }
        }

        Command::ToPkl { expr } => {
            let pkl = pklx::cli::nix_to_pkl(&expr)?;
            println!("{}", pkl);
        }

        Command::Analyze { file } => {
            let deps = pklx::analyze_pkl_imports(&file)?;
            for dep in deps {
                println!("{}", dep.display());
            }
        }
    }

    Ok(())
}
