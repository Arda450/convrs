// json to other formats

use std::fs;
use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;

/// Konvertiert JSON String zu formatiertem JSON String
pub fn json_to_json_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;
    
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting JSON: {}", e)))
}

/// Konvertiert JSON String zu TOML String
pub fn json_to_toml_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;
    
    // TOML unterstützt kein Array als Root-Element
    // Wenn es ein Array ist, wrappen wir es in ein Objekt
    let toml_ready_value = match json_value {
        serde_json::Value::Array(arr) => {
            serde_json::json!({ "data": arr })
        }
        _ => json_value
    };
    
    // JSON Value → TOML Value strukturell konvertieren
    let toml_value = json_to_toml_value(&toml_ready_value)?;
    
    // TOML Value → Pretty-Printed String
    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error serializing TOML: {}", e)))
}

/// Konvertiert JSON String zu YAML String
pub fn json_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;
    
    serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting YAML: {}", e)))
}

/// Konvertiert JSON String zu CSV String
pub fn json_to_csv_string(input: &str) -> Result<String, FormatError> {
    let json_value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid JSON: {}", e)))?;
    
    // JSON Array zu CSV
    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json_value],
        _ => return Err(FormatError::SerializationError("JSON must be an array or object for CSV".to_string()))
    };
    
    if array.is_empty() {
        return Ok(String::new());
    }
    
    // Alle Objekte flattenen
    let flattened: Vec<_> = array.iter()
        .map(|v| flatten_json(v, ""))
        .collect();
    
    // Header sammeln
    let mut all_headers = std::collections::BTreeSet::new();
    for obj in &flattened {
        for key in obj.keys() {
            all_headers.insert(key.clone());
        }
    }
    let headers: Vec<String> = all_headers.into_iter().collect();
    
    // CSV Writer in Memory
    let mut writer = csv::Writer::from_writer(vec![]);
    
    // Header schreiben
    writer.write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Error writing CSV header: {}", e)))?;
    
    // Daten schreiben
    for flat_obj in flattened {
        let row: Vec<String> = headers.iter()
            .map(|h| flat_obj.get(h).cloned().unwrap_or_default())
            .collect();
        writer.write_record(&row)
            .map_err(|e| FormatError::SerializationError(format!("Error writing CSV row: {}", e)))?;
    }
    
    // Writer in String umwandeln
    let data = writer.into_inner()
        .map_err(|e| FormatError::SerializationError(format!("Error finishing CSV: {}", e)))?;
    
    String::from_utf8(data)
        .map_err(|e| FormatError::SerializationError(format!("Error converting to UTF-8: {}", e)))
}

//  ende der web string zu stirng funktionen. unten werden sie weiter benutzt um die datei zu lesen und zu schreiben im cli

/// konvertiert json zu json für das cli
pub fn convert_json_to_json(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = json_to_json_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

/// konvertiert json zu toml für das cli
pub fn convert_json_to_toml(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = json_to_toml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert JSON zu YAML für das cli
pub fn convert_json_to_yaml(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // liest die datei ein und speichert den inhalt in der variable content
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    // ruft die string funktion zwischen zeile 38-45 auf und speichert den inhalt in der variable result
    let result = json_to_yaml_string(&content)?;
    // schreibt den inhalt von result in die datei output_path
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}


/// konvertiert json zu csv für das cli
pub fn convert_json_to_csv(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = json_to_csv_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}



/// Validiert eine JSON-Datei ohne sie zu schreiben
pub fn validate_json(input_path: &str) -> Result<serde_json::Value, FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;

        // serde_json::from_str prüft ob syntax korrekt ist
        // prüft ob syntax korrekt ist
    let json_value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Error parsing JSON: {}", e)))?;

    Ok(json_value)
}


// __________________________________________________________________________________________________________________________
// Helper-Funktionen für CSV-Konvertierung

/// Konvertiert einen JSON-Wert zu einem String
fn value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        // Verschachtelte Strukturen als JSON-String
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            serde_json::to_string(value).unwrap_or_else(|_| String::new())
        }
    }
}

/// Flattened ein JSON-Objekt zu einer flachen Map mit Unterstrich-Trennzeichen
/// 
/// Beispiel:
/// Input:  {"contact": {"email": "test@test.com", "phone": "+49"}}
/// Output: {"contact_email": "test@test.com", "contact_phone": "+49"}
fn flatten_json(value: &serde_json::Value, prefix: &str) -> std::collections::HashMap<String, String> {
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
                
                // Rekursiv für verschachtelte Objekte
                if val.is_object() {
                    let nested = flatten_json(val, &new_key);
                    result.extend(nested);
                } else {
                    result.insert(new_key, value_to_string(val));
                }
            }
        }
        _ => {
            // Primitive Werte direkt einfügen
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), value_to_string(value));
            }
        }
    }
    
    result
}