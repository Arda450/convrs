//! JSON-Konvertierungen: String-zu-String Funktionen.

use crate::error::FormatError;
use crate::formats::utils::{flatten_json, json_to_toml_value};

/// Konvertiert JSON String zu formatiertem JSON String (Pretty-Printing).
pub fn json_to_json_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;

    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting JSON: {}", e)))
}

/// Konvertiert JSON String zu YAML String.
pub fn json_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;

    serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting YAML: {}", e)))
}

/// Konvertiert JSON String zu TOML String.
pub fn json_to_toml_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;

    // TOML unterstützt kein Array als Root-Element, darum wird ein Wrapper mit dem Namen "data" erstellt.
    let toml_ready_value = match json_value {
        serde_json::Value::Array(arr) => serde_json::json!({ "data": arr }),
        other => other,
    };

    let toml_value = json_to_toml_value(&toml_ready_value)?;

    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error serializing TOML: {}", e)))
}

/// Konvertiert JSON String zu CSV String.
pub fn json_to_csv_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;

    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json_value],
        _ => {
            return Err(FormatError::SerializationError(
                "JSON must be an array or object for CSV".to_string(),
            ))
        }
    };

    if array.is_empty() {
        return Ok(String::new());
    }

    // Alle Objekte flatten
    let flattened: Vec<_> = array.iter().map(|v| flatten_json(v, "")).collect();

    // Header sammeln (BTreeSet für konsistente Reihenfolge)
    let mut all_headers = std::collections::BTreeSet::new();
    for obj in &flattened {
        for key in obj.keys() {
            all_headers.insert(key.clone());
        }
    }
    let headers: Vec<String> = all_headers.into_iter().collect();

    // CSV schreiben
    let mut writer = csv::Writer::from_writer(vec![]);

    writer
        .write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Error writing CSV header: {}", e)))?;

    for flat_obj in flattened {
        let row: Vec<String> = headers
            .iter()
            .map(|h| flat_obj.get(h).cloned().unwrap_or_default())
            .collect();
        writer
            .write_record(&row)
            .map_err(|e| FormatError::SerializationError(format!("Error writing CSV row: {}", e)))?;
    }

    let data = writer
        .into_inner()
        .map_err(|e| FormatError::SerializationError(format!("Error finishing CSV: {}", e)))?;

    String::from_utf8(data)
        .map_err(|e| FormatError::SerializationError(format!("Error converting to UTF-8: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FormatError;

    #[test]
    fn test_json_to_json_pretty_prints() {
        let input = r#"{"a":1,"b":"hello"}"#;
        let result = json_to_json_string(input).unwrap();
        assert!(result.contains("\"a\""));
        assert!(result.contains("\"b\""));
        assert!(result.contains('\n')); // Pretty-printed enthält Newlines
    }

    #[test]
    fn test_json_to_json_invalid_fails() {
        let result = json_to_json_string("{ invalid }");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FormatError::ParseError(_)));
    }

    #[test]
    fn test_json_to_yaml() {
        let input = r#"{"name":"Test","count":42}"#;
        let result = json_to_yaml_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Test"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_json_to_toml_object() {
        let input = r#"{"title":"Hello","section":{"key":"value"}}"#;
        let result = json_to_toml_string(input).unwrap();
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }

    #[test]
    fn test_json_to_toml_array_wraps_in_data() {
        let input = r#"[{"x":1},{"x":2}]"#;
        let result = json_to_toml_string(input).unwrap();
        assert!(result.contains("data") || result.contains("[["));
    }

    #[test]
    fn test_json_to_csv_array_of_objects() {
        let input = r#"[{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]"#;
        let result = json_to_csv_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_json_to_csv_single_object() {
        let input = r#"{"name":"Alice","age":30}"#;
        let result = json_to_csv_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_json_to_csv_empty_array() {
        let result = json_to_csv_string("[]").unwrap();
        assert!(result.is_empty());
    }
}
