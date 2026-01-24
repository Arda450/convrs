// Gemeinsame Helper-Funktionen für Format-Konvertierungen

use crate::error::FormatError;
use serde_json::Value as JsonValue;



pub fn json_to_toml_value(json: &JsonValue) -> Result<toml::Value, FormatError> {
    match json {
        JsonValue::Null => {
            // TOML hat kein echtes "null" - wir verwenden einen leeren String als Ersatz
            Ok(toml::Value::String(String::new()))
        },
        JsonValue::Bool(b) => Ok(toml::Value::Boolean(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(FormatError::SerializationError(
                    "Invalid number for TOML".to_string()
                ))
            }
        },
        JsonValue::String(s) => Ok(toml::Value::String(s.clone())),
        JsonValue::Array(arr) => {
            // Rekursiv: Jedes Array-Element konvertieren
            let toml_arr: Result<Vec<toml::Value>, FormatError> = arr
                .iter()
                .map(json_to_toml_value)
                .collect();
            Ok(toml::Value::Array(toml_arr?))
        },
        JsonValue::Object(obj) => {
            // Rekursiv: Jeden Objekt-Wert konvertieren
            let mut toml_table = toml::map::Map::new();
            for (key, value) in obj {
                toml_table.insert(key.clone(), json_to_toml_value(value)?);
            }
            Ok(toml::Value::Table(toml_table))
        },
    }
}

