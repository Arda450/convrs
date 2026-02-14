//! CSV-Konvertierungen: String-zu-String Funktionen.

use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;
use csv::ReaderBuilder;
use serde_json::Value as JsonValue;

/// Konvertiert CSV String zu JSON String.
pub fn csv_to_json_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    let json_value = JsonValue::Array(records);

    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting JSON: {}", e)))
}

/// Konvertiert CSV String zu YAML String.
pub fn csv_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    let json_value = JsonValue::Array(records);

    serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting YAML: {}", e)))
}

/// Konvertiert CSV String zu TOML String.
pub fn csv_to_toml_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    let json_array = JsonValue::Array(records);

    // TOML braucht ein Objekt als Root
    let mut root_object = serde_json::Map::new();
    root_object.insert("data".to_string(), json_array);
    let json_value = JsonValue::Object(root_object);

    let toml_value = json_to_toml_value(&json_value)?;

    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting TOML: {}", e)))
}

/// Konvertiert CSV String zu CSV String.
pub fn csv_to_csv_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;

    if records.is_empty() {
        return Ok(String::new());
    }

    // Header aus erstem Record extrahieren
    let headers: Vec<String> = if let JsonValue::Object(obj) = &records[0] {
        obj.keys().cloned().collect()
    } else {
        return Err(FormatError::SerializationError(
            "CSV records must be objects".to_string(),
        ));
    };

    let mut writer = csv::Writer::from_writer(vec![]);

    writer
        .write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Error writing CSV header: {}", e)))?;

    for record in &records {
        if let JsonValue::Object(obj) = record {
            let row: Vec<String> = headers
                .iter()
                .map(|h| {
                    obj.get(h)
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                })
                .collect();
            writer.write_record(&row).map_err(|e| {
                FormatError::SerializationError(format!("Error writing CSV row: {}", e))
            })?;
        }
    }

    let data = writer
        .into_inner()
        .map_err(|e| FormatError::SerializationError(format!("Error finishing CSV: {}", e)))?;

    String::from_utf8(data)
        .map_err(|e| FormatError::SerializationError(format!("Error converting to UTF-8: {}", e)))
}

// private helper funktionen

/// Parst CSV String zu einer Liste von JSON-Objekten.
fn parse_csv_to_json_values(input: &str) -> Result<Vec<JsonValue>, FormatError> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        return Err(FormatError::ParseError("CSV input is empty".to_string()));
    }

    let first_line = lines[0].trim();
    if !first_line.contains(',') {
        return Err(FormatError::ParseError(
            "Invalid CSV format: First line contains no commas. CSV should contain comma-separated values, e.g.: name,age,city".to_string(),
        ));
    }

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(false)
        .from_reader(input.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| FormatError::ParseError(format!("Error reading CSV header: {}", e)))?
        .clone();

    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Error reading CSV record: {}", e)))?;

        let mut obj = serde_json::Map::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = header_vec.get(i) {
                let value = infer_type(field);
                obj.insert(header.clone(), value);
            }
        }

        records.push(JsonValue::Object(obj));
    }

    Ok(records)
}

/// Versucht den Typ eines CSV-String-Wertes zu erkennen.
///
/// Reihenfolge: Boolean → Integer → Float → String (Fallback).
fn infer_type(value: &str) -> JsonValue {
    if value.is_empty() {
        return JsonValue::Null;
    }

    match value.to_lowercase().as_str() {
        "true" => return JsonValue::Bool(true),
        "false" => return JsonValue::Bool(false),
        _ => {}
    }

    if let Ok(num) = value.parse::<i64>() {
        return JsonValue::Number(num.into());
    }

    if let Ok(num) = value.parse::<f64>()
        && let Some(json_num) = serde_json::Number::from_f64(num) {
            return JsonValue::Number(json_num);
        }

    JsonValue::String(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_json() {
        let input = "name,age\nAlice,30\nBob,25";
        let result = csv_to_json_string(input).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        assert!(result.contains("30"));
    }

    #[test]
    fn test_csv_to_yaml() {
        let input = "name,age\nAlice,30";
        let result = csv_to_yaml_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_csv_to_toml() {
        let input = "name,age\nAlice,30";
        let result = csv_to_toml_string(input).unwrap();
        assert!(result.contains("data"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_csv_to_csv_roundtrip() {
        let input = "name,age\nAlice,30\nBob,25";
        let result = csv_to_csv_string(input).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_csv_empty_fails() {
        let result = csv_to_json_string("");
        assert!(result.is_err());
    }

    #[test]
    fn test_csv_no_commas_fails() {
        let result = csv_to_json_string("just a single line without commas");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no commas"));
    }

    #[test]
    fn test_csv_inconsistent_columns_fails() {
        let result = csv_to_json_string("a,b,c\n1,2\n3,4,5");
        assert!(result.is_err());
    }

    #[test]
    fn test_infer_type_boolean() {
        assert_eq!(infer_type("true"), JsonValue::Bool(true));
        assert_eq!(infer_type("false"), JsonValue::Bool(false));
        assert_eq!(infer_type("TRUE"), JsonValue::Bool(true));
    }

    #[test]
    fn test_infer_type_number() {
        assert_eq!(infer_type("42"), JsonValue::Number(42.into()));
        assert_eq!(infer_type("0"), JsonValue::Number(0.into()));
    }

    #[test]
    fn test_infer_type_string_fallback() {
        assert_eq!(infer_type("hello"), JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_infer_type_empty_is_null() {
        assert_eq!(infer_type(""), JsonValue::Null);
    }
}
