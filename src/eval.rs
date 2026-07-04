use miette::{bail, IntoDiagnostic, WrapErr};
use std::path::Path;

use crate::nix_serializer;

fn apply_options(evaluator: &mut pklr::Evaluator, options: &pklr::EvalOptions) {
    if let Some(client) = &options.client {
        evaluator.set_http_client(client.clone());
    }
    if !options.http_rewrites.is_empty() {
        evaluator.set_http_rewrites(&options.http_rewrites);
    }
}

/// Evaluate a .pkl file and return the raw pklr::Value.
pub async fn eval_to_value(path: &Path, options: pklr::EvalOptions) -> miette::Result<pklr::Value> {
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }

    let mut evaluator = pklr::Evaluator::new();
    evaluator.set_base_path(path.parent().unwrap_or_else(|| Path::new(".")));
    apply_options(&mut evaluator, &options);

    evaluator
        .eval_file_pub(path)
        .await
        .map_err(nix_serializer::Error::Pkl)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to evaluate '{}'", path.display()))
}

/// Evaluate a Pkl source string and return the raw pklr::Value.
pub async fn eval_source_to_value(
    source: &str,
    options: pklr::EvalOptions,
) -> miette::Result<pklr::Value> {
    let mut evaluator = pklr::Evaluator::new();
    apply_options(&mut evaluator, &options);

    evaluator
        .eval_source(source, Path::new("<expr>"))
        .await
        .map_err(nix_serializer::Error::Pkl)
        .into_diagnostic()
        .wrap_err("Failed to evaluate Pkl expression")
}

/// Evaluate a .pkl file and return a Nix expression string.
pub async fn eval_pkl(path: &Path, options: pklr::EvalOptions) -> miette::Result<String> {
    let value = eval_to_value(path, options).await?;
    Ok(nix_serializer::pkl_value_to_nix(&value))
}

/// Evaluate a Pkl source string and return a Nix expression string.
pub async fn eval_pkl_source(
    source: &str,
    options: pklr::EvalOptions,
) -> miette::Result<String> {
    let value = eval_source_to_value(source, options).await?;
    Ok(nix_serializer::pkl_value_to_nix(&value))
}

/// Analyze local import dependencies of a .pkl file.
pub fn analyze_pkl_imports(path: &Path) -> miette::Result<Vec<std::path::PathBuf>> {
    pklr::analyze_imports(path).map_err(|e| miette::miette!("Failed to analyze imports: {}", e))
}
