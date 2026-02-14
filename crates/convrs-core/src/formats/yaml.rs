//! YAML-Konvertierungen: Hier befinden sich die String-zu-String Funktionen.

use crate::error::FormatError;
use crate::formats::utils::{flatten_json, json_to_toml_value};

/// Konvertiert YAML String zu JSON String.
pub fn yaml_to_json_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid YAML: {}", e)))?;

    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;

    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting JSON: {}", e)))
}

/// Konvertiert YAML String zu YAML String.
pub fn yaml_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid YAML: {}", e)))?;

    serde_yaml::to_string(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting YAML: {}", e)))
}

/// Konvertiert YAML String zu TOML String.
pub fn yaml_to_toml_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid YAML: {}", e)))?;

    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;

    // TOML kann kein Array als Root haben
    let toml_ready_value = if json_value.is_array() {
        let mut root = serde_json::Map::new();
        root.insert("data".to_string(), json_value);
        serde_json::Value::Object(root)
    } else {
        json_value
    };

    let toml_value = json_to_toml_value(&toml_ready_value)?;

    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting TOML: {}", e)))
}

/// Konvertiert YAML String zu CSV String.
pub fn yaml_to_csv_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid YAML: {}", e)))?;

    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;

    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json_value],
        _ => {
            return Err(FormatError::SerializationError(
                "YAML must be an array or object".to_string(),
            ))
        }
    };

    if array.is_empty() {
        return Ok(String::new());
    }

    let flattened: Vec<_> = array.iter().map(|v| flatten_json(v, "")).collect();

    let mut all_headers = std::collections::BTreeSet::new();
    for obj in &flattened {
        for key in obj.keys() {
            all_headers.insert(key.clone());
        }
    }
    let headers: Vec<String> = all_headers.into_iter().collect();

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

    #[test]
    fn test_yaml_to_json() {
        let input = "name: Alice\nage: 30";
        let result = yaml_to_json_string(input).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("30"));
    }

    #[test]
    fn test_yaml_to_yaml_pretty() {
        let input = "name: Alice\nage: 30";
        let result = yaml_to_yaml_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_yaml_to_toml() {
        let input = "title: Hello\ncount: 10";
        let result = yaml_to_toml_string(input).unwrap();
        assert!(result.contains("title"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_yaml_to_csv() {
        let input = "- name: Alice\n  age: 30\n- name: Bob\n  age: 25";
        let result = yaml_to_csv_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_yaml_invalid_fails() {
        let result = yaml_to_json_string("  invalid:\n yaml\n  : broken");
        assert!(result.is_err());
    }
}
