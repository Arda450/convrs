//! TOML-Konvertierungen: String-zu-String Funktionen.

use crate::error::FormatError;
use crate::formats::utils::flatten_json;

/// Konvertiert TOML String zu JSON String.
pub fn toml_to_json_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;

    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;

    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting JSON: {}", e)))
}

/// Konvertiert TOML String zu YAML String.
pub fn toml_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;

    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;

    serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting YAML: {}", e)))
}

/// Konvertiert TOML String zu TOML String (Pretty-Printing).
pub fn toml_to_toml_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;

    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting TOML: {}", e)))
}

/// Konvertiert TOML String zu CSV String.
pub fn toml_to_csv_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;

    let mut json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;

    // Wenn ein "data"-Wrapper existiert (von CSV → TOML), extrahiere das Array
    if let serde_json::Value::Object(ref obj) = json_value
        && let Some(data_value) = obj.get("data") {
            json_value = data_value.clone();
        }

    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json_value],
        _ => {
            return Err(FormatError::SerializationError(
                "TOML must be an array or object".to_string(),
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
    fn test_toml_to_json() {
        let input = "title = \"Hello\"\ncount = 10";
        let result = toml_to_json_string(input).unwrap();
        assert!(result.contains("title"));
        assert!(result.contains("Hello"));
        assert!(result.contains("10"));
    }

    #[test]
    fn test_toml_to_yaml() {
        let input = "title = \"Hello\"\ncount = 10";
        let result = toml_to_yaml_string(input).unwrap();
        assert!(result.contains("title"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_toml_to_toml_pretty() {
        let input = "title=\"Hello\"\ncount=10";
        let result = toml_to_toml_string(input).unwrap();
        assert!(result.contains("title"));
        // Pretty-printed TOML hat Spaces um '='
        assert!(result.contains("= "));
    }

    #[test]
    fn test_toml_to_csv_with_data_wrapper() {
        let input = "[[data]]\nname = \"Alice\"\nage = 30\n\n[[data]]\nname = \"Bob\"\nage = 25";
        let result = toml_to_csv_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_toml_invalid_fails() {
        let result = toml_to_json_string("not valid toml [ [ [");
        assert!(result.is_err());
    }
}
