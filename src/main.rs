use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pklx", version, about = "Pkl ↔ Nix interop tool")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Evaluate a .pkl file and emit a Nix expression
    Eval {
        /// Path to the .pkl file
        file: std::path::PathBuf,

        /// Wrap the output in a NixOS module skeleton
        #[arg(long)]
        module: bool,

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
            module,
            output,
        } => {
            let nix = pklx::cli::eval_pkl(&file).await?;

            let result = if module {
                format!("{{ config, pkgs, ... }}: {{\n{}\n}}", nix)
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
            let deps = pklx::cli::analyze_pkl_imports(&file)?;
            for dep in deps {
                println!("{}", dep.display());
            }
        }
    }

    Ok(())
}
