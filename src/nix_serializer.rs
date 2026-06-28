use pklr::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Pkl(#[from] pklr::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Serialize a `pklr::Value` into a Nix expression string.
pub fn pkl_value_to_nix(value: &Value) -> String {
    value_to_nix_inner(value, 0)
}

fn value_to_nix_inner(value: &Value, indent: usize) -> String {
    match value {
        Value::Null => "null".to_string(),

        Value::Bool(b) => b.to_string(),

        Value::Int(n) => n.to_string(),

        Value::Float(f) => format_nix_float(*f),

        Value::String(s) => format_nix_string(s),

        Value::Object(map, source) => {
            if map.is_empty() {
                return "{ }".to_string();
            }

            let mut out = "{ ".to_string();

            if let Some(src) = source {
                if let Some(ref type_name) = src.type_name {
                    out.push_str(&format!("__pkl_class = \"{}\"; ", type_name));
                }
            }

            for (k, v) in map.iter() {
                let key = if is_simple_nix_ident(k) {
                    k.clone()
                } else {
                    escape_nix_quoted_key(k)
                };
                let val = value_to_nix_inner(v, indent + 2);
                out.push_str(&format!("{} = {}; ", key, val));
            }

            out.push('}');
            out
        }

        Value::List(items) => {
            if items.is_empty() {
                return "[ ]".to_string();
            }
            let child_indent = indent + 2;
            let mut out = "[\n".to_string();
            for item in items {
                let val = value_to_nix_inner(item, child_indent);
                out.push_str(&format!("{}{}\n", " ".repeat(child_indent), val));
            }
            out.push_str(&format!("{}]", " ".repeat(indent)));
            out
        }

        Value::Lambda(..) => "\"<lambda>\"".to_string(),
    }
}

fn format_nix_float(f: f64) -> String {
    if f.is_infinite() {
        if f.is_sign_positive() {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else if f.is_nan() {
        "NaN".to_string()
    } else {
        let s = format!("{}", f);
        if !s.contains('.') && !s.contains('e') && !s.contains('E') {
            format!("{}.0", s)
        } else {
            s
        }
    }
}

fn format_nix_string(s: &str) -> String {
    if s.contains('\n') || s.contains("''") {
        let escaped = s
            .replace('\\', "\\\\")
            .replace('\"', "\\\"")
            .replace("${", "\\${");
        format!("\"{}\"", escaped)
    } else if s.contains("${") {
        let escaped = s.replace("${", "\\${");
        format!("\"{}\"", escaped)
    } else {
        format!("\"{}\"", s)
    }
}

fn is_simple_nix_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '\'' {
            return false;
        }
    }
    true
}

fn escape_nix_quoted_key(k: &str) -> String {
    let escaped = k
        .replace('\\', "\\\\")
        .replace('\"', "\\\"")
        .replace("${", "\\${");
    format!("\"{}\"", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use std::sync::Arc;

    #[test]
    fn test_null() {
        assert_eq!(pkl_value_to_nix(&Value::Null), "null");
    }

    #[test]
    fn test_bool() {
        assert_eq!(pkl_value_to_nix(&Value::Bool(true)), "true");
        assert_eq!(pkl_value_to_nix(&Value::Bool(false)), "false");
    }

    #[test]
    fn test_int() {
        assert_eq!(pkl_value_to_nix(&Value::Int(42)), "42");
        assert_eq!(pkl_value_to_nix(&Value::Int(-1)), "-1");
        assert_eq!(pkl_value_to_nix(&Value::Int(0)), "0");
    }

    #[test]
    fn test_float() {
        assert_eq!(pkl_value_to_nix(&Value::Float(2.5)), "2.5");
    }

    #[test]
    fn test_float_integer_value() {
        let result = pkl_value_to_nix(&Value::Float(42.0));
        assert!(result == "42.0" || result == "42");
    }

    #[test]
    fn test_string_simple() {
        assert_eq!(
            pkl_value_to_nix(&Value::String("hello".into())),
            "\"hello\""
        );
    }

    #[test]
    fn test_string_with_dollar_brace() {
        assert_eq!(
            pkl_value_to_nix(&Value::String("${interp}".into())),
            "\"\\${interp}\""
        );
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(pkl_value_to_nix(&Value::List(vec![])), "[ ]");
    }

    #[test]
    fn test_list() {
        let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        let result = pkl_value_to_nix(&list);
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_empty_object() {
        let map: IndexMap<String, Value> = IndexMap::new();
        assert_eq!(pkl_value_to_nix(&Value::Object(Arc::new(map), None)), "{ }");
    }

    #[test]
    fn test_object() {
        let mut map = IndexMap::new();
        map.insert("name".into(), Value::String("test".into()));
        map.insert("count".into(), Value::Int(42));
        let result = pkl_value_to_nix(&Value::Object(Arc::new(map), None));
        assert!(result.contains("name = \"test\""));
        assert!(result.contains("count = 42"));
    }

    #[test]
    fn test_nested_object() {
        let mut inner = IndexMap::new();
        inner.insert("x".into(), Value::Int(1));
        let mut outer = IndexMap::new();
        outer.insert("inner".into(), Value::Object(Arc::new(inner), None));
        let result = pkl_value_to_nix(&Value::Object(Arc::new(outer), None));
        assert!(result.contains("inner = {"));
        assert!(result.contains("x = 1"));
    }

    #[test]
    fn test_ident_key_with_hyphen() {
        let mut map = IndexMap::new();
        map.insert("my-key".into(), Value::Int(1));
        let result = pkl_value_to_nix(&Value::Object(Arc::new(map), None));
        assert!(result.contains("my-key = 1"));
    }

    #[test]
    fn test_quoted_key() {
        let mut map = IndexMap::new();
        map.insert("weird!key".into(), Value::Bool(true));
        let result = pkl_value_to_nix(&Value::Object(Arc::new(map), None));
        assert!(result.contains("\"weird!key\" = true"));
    }
}
