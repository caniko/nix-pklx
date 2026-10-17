use miette::{bail, IntoDiagnostic, WrapErr};
use std::path::Path;

use crate::nix_serializer::{self, SerializeOptions};

fn evaluator(options: pklr::EvalOptions) -> pklr::Evaluator {
    let mut builder = pklr::EvaluatorBuilder::new().http_rewrites(options.http_rewrites);
    if let Some(agent) = options.agent {
        builder = builder.http_agent(agent);
    }
    builder.build()
}

/// Evaluate a .pkl file and return the raw pklr::Value.
pub async fn eval_to_value(path: &Path, options: pklr::EvalOptions) -> miette::Result<pklr::Value> {
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }

    let owned_path = path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let mut evaluator = evaluator(options);
        evaluator.set_base_path(owned_path.parent().unwrap_or_else(|| Path::new(".")));
        evaluator.eval_file(&owned_path)
    })
    .await
    .into_diagnostic()?
    .map_err(nix_serializer::Error::Pkl)
    .into_diagnostic()
    .wrap_err_with(|| format!("Failed to evaluate '{}'", path.display()))
}

/// Evaluate a Pkl source string and return the raw pklr::Value.
pub async fn eval_source_to_value(
    source: &str,
    options: pklr::EvalOptions,
) -> miette::Result<pklr::Value> {
    let source = source.to_owned();
    tokio::task::spawn_blocking(move || {
        evaluator(options).eval_source(&source, Path::new("<expr>"))
    })
    .await
    .into_diagnostic()?
    .map_err(nix_serializer::Error::Pkl)
    .into_diagnostic()
    .wrap_err("Failed to evaluate Pkl expression")
}

/// Evaluate a .pkl file and return a Nix expression string.
pub async fn eval_pkl(path: &Path, options: pklr::EvalOptions) -> miette::Result<String> {
    let value = eval_to_value(path, options).await?;
    Ok(nix_serializer::pkl_value_to_nix(&value))
}

/// Evaluate a .pkl file and return a Nix expression string with serializer options.
pub async fn eval_pkl_with_serializer_options(
    path: &Path,
    options: pklr::EvalOptions,
    serializer_options: SerializeOptions,
) -> miette::Result<String> {
    let value = eval_to_value(path, options).await?;
    Ok(nix_serializer::pkl_value_to_nix_with_options(
        &value,
        serializer_options,
    ))
}

/// Evaluate a Pkl source string and return a Nix expression string.
pub async fn eval_pkl_source(source: &str, options: pklr::EvalOptions) -> miette::Result<String> {
    let value = eval_source_to_value(source, options).await?;
    Ok(nix_serializer::pkl_value_to_nix(&value))
}

/// Evaluate a Pkl source string and return a Nix expression string with serializer options.
pub async fn eval_pkl_source_with_serializer_options(
    source: &str,
    options: pklr::EvalOptions,
    serializer_options: SerializeOptions,
) -> miette::Result<String> {
    let value = eval_source_to_value(source, options).await?;
    Ok(nix_serializer::pkl_value_to_nix_with_options(
        &value,
        serializer_options,
    ))
}

/// Analyze local import dependencies of a .pkl file.
pub fn analyze_pkl_imports(path: &Path) -> miette::Result<Vec<std::path::PathBuf>> {
    pklr::analyze_imports(path).map_err(|e| miette::miette!("Failed to analyze imports: {}", e))
}
