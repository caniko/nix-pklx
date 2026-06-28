use miette::{bail, IntoDiagnostic, WrapErr};
use std::path::Path;

use crate::nix_serializer::{self, pkl_value_to_nix};

/// Evaluate a .pkl file and return a Nix expression string.
pub async fn eval_pkl(path: &Path) -> miette::Result<String> {
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }

    let mut evaluator = pklr::Evaluator::new();
    evaluator.set_base_path(path.parent().unwrap_or_else(|| Path::new(".")));

    let value = evaluator
        .eval_file_pub(path)
        .await
        .map_err(nix_serializer::Error::Pkl)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to evaluate '{}'", path.display()))?;

    Ok(pkl_value_to_nix(&value))
}

/// Analyze local import dependencies of a .pkl file.
pub fn analyze_pkl_imports(path: &Path) -> miette::Result<Vec<std::path::PathBuf>> {
    pklr::analyze_imports(path).map_err(|e| miette::miette!("Failed to analyze imports: {}", e))
}

/// Convert a Nix expression string to Pkl syntax.
/// Accepts JSON-compatible Nix-like expressions.
pub fn nix_to_pkl(expr: &str) -> miette::Result<String> {
    let value: serde_json::Value = serde_json::from_str(expr)
        .map_err(|e| miette::miette!("Failed to parse Nix expression: {}", e))?;

    Ok(json_value_to_pkl(&value, 0))
}

fn json_value_to_pkl(value: &serde_json::Value, indent: usize) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(f) = n.as_f64() {
                let s = format!("{}", f);
                if !s.contains('.') && !s.contains('e') && !s.contains('E') {
                    format!("{}.0", s)
                } else {
                    s
                }
            } else {
                n.to_string()
            }
        }
        serde_json::Value::String(s) => {
            format!("\"{}\"", s)
        }
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                return "new Listing {}".to_string();
            }
            let child_indent = indent + 2;
            let mut out = "new {\n".to_string();
            for item in arr {
                let val = json_value_to_pkl(item, child_indent);
                out.push_str(&format!("{}{}\n", " ".repeat(child_indent), val));
            }
            out.push_str(&format!("{}}}", " ".repeat(indent)));
            out
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                return "new {}".to_string();
            }
            let child_indent = indent + 2;
            let mut out = "new {\n".to_string();
            for (k, v) in obj {
                let val = json_value_to_pkl(v, child_indent);
                out.push_str(&format!("{}{} = {}\n", " ".repeat(child_indent), k, val));
            }
            out.push_str(&format!("{}}}", " ".repeat(indent)));
            out
        }
    }
}
