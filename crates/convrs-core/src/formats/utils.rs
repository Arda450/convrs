//! Gemeinsame Helper-Funktionen für Format-Konvertierungen.

use crate::error::FormatError;
use serde_json::Value as JsonValue;

/// Konvertiert einen `serde_json::Value` rekursiv in einen `toml::Value`.
/// weil Serde keine direkte Konvertierung zwischen den beiden Value-Typen unterstützt.
pub fn json_to_toml_value(json: &JsonValue) -> Result<toml::Value, FormatError> {
    match json {
        JsonValue::Null => {
            // TOML hat kein "null" — leerer String als Ersatz
            Ok(toml::Value::String(String::new()))
        }
        JsonValue::Bool(b) => Ok(toml::Value::Boolean(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(FormatError::SerializationError(
                    "Invalid number for TOML".to_string(),
                ))
            }
        }
        JsonValue::String(s) => Ok(toml::Value::String(s.clone())),
        JsonValue::Array(arr) => {
            let toml_arr: Result<Vec<toml::Value>, FormatError> =
                arr.iter().map(json_to_toml_value).collect();
            Ok(toml::Value::Array(toml_arr?))
        }
        JsonValue::Object(obj) => {
            let mut toml_table = toml::map::Map::new();
            for (key, value) in obj {
                toml_table.insert(key.clone(), json_to_toml_value(value)?);
            }
            Ok(toml::Value::Table(toml_table))
        }
    }
}

/// Konvertiert einen JSON-Wert in einen flachen String.
/// helper func für flatten_json, um ein JSON-Objekt zu einer flachen Map mit Unterstrich-Trennzeichen zu flattenen damit sie als CSV geschrieben werden kann.
/// // wird in flatten_json verwendet
pub fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            serde_json::to_string(value).unwrap_or_default()
        }
    }
}

/// verwendung für CSV-Konvertierungen, (json/toml/yaml -> csv)
/// wird benötigt, um ein JSON-Objekt zu einer flachen Map mit Unterstrich-Trennzeichen zu flattenen damit sie als CSV geschrieben werden kann.
pub fn flatten_json(
    value: &serde_json::Value,
    prefix: &str,
) -> std::collections::HashMap<String, String> {
    use std::collections::HashMap;

    let mut result = HashMap::new();

    match value {
        serde_json::Value::Object(obj) => {
            for (key, val) in obj {
                let new_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}_{}", prefix, key)
                };

                if val.is_object() {
                    let nested = flatten_json(val, &new_key);
                    result.extend(nested);
                } else {
                    result.insert(new_key, json_value_to_string(val));
                }
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), json_value_to_string(value));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_toml_value_string() {
        let json = serde_json::json!("hello");
        let toml_val = json_to_toml_value(&json).unwrap();
        assert_eq!(toml_val, toml::Value::String("hello".to_string()));
    }

    #[test]
    fn test_json_to_toml_value_number() {
        let json = serde_json::json!(42);
        let toml_val = json_to_toml_value(&json).unwrap();
        assert_eq!(toml_val, toml::Value::Integer(42));
    }

    #[test]
    fn test_json_to_toml_value_bool() {
        let json = serde_json::json!(true);
        let toml_val = json_to_toml_value(&json).unwrap();
        assert_eq!(toml_val, toml::Value::Boolean(true));
    }

    #[test]
    fn test_json_to_toml_value_null_becomes_empty_string() {
        let json = serde_json::json!(null);
        let toml_val = json_to_toml_value(&json).unwrap();
        assert_eq!(toml_val, toml::Value::String(String::new()));
    }

    #[test]
    fn test_json_to_toml_value_object() {
        let json = serde_json::json!({"key": "value"});
        let toml_val = json_to_toml_value(&json).unwrap();
        assert!(matches!(toml_val, toml::Value::Table(_)));
    }

    #[test]
    fn test_json_value_to_string_primitives() {
        assert_eq!(json_value_to_string(&serde_json::json!("hello")), "hello");
        assert_eq!(json_value_to_string(&serde_json::json!(42)), "42");
        assert_eq!(json_value_to_string(&serde_json::json!(true)), "true");
        assert_eq!(json_value_to_string(&serde_json::json!(null)), "");
    }

    #[test]
    fn test_flatten_json_simple() {
        let json = serde_json::json!({"name": "Alice", "age": 30});
        let flat = flatten_json(&json, "");
        assert_eq!(flat.get("name").unwrap(), "Alice");
        assert_eq!(flat.get("age").unwrap(), "30");
    }

    #[test]
    fn test_flatten_json_nested() {
        let json = serde_json::json!({"user": {"name": "Alice", "address": {"city": "Zürich"}}});
        let flat = flatten_json(&json, "");
        assert_eq!(flat.get("user_name").unwrap(), "Alice");
        assert_eq!(flat.get("user_address_city").unwrap(), "Zürich");
    }
}
