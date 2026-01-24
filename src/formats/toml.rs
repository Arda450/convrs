// TOML zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;

// ============================================================================
// hier befinden sich string zu string funktionen (Core-Logik für CLI und Web)
// ============================================================================

/// Konvertiert TOML String zu JSON String
pub fn toml_to_json_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;
    
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting JSON: {}", e)))
}

/// Konvertiert TOML String zu YAML String
pub fn toml_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;
    
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;
    
    serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting YAML: {}", e)))
}

/// Konvertiert TOML String zu TOML String (Pretty-Printing)
pub fn toml_to_toml_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;
    
    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error formatting TOML: {}", e)))
}

/// Konvertiert TOML String zu CSV String
pub fn toml_to_csv_string(input: &str) -> Result<String, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Invalid TOML: {}", e)))?;
    
    let mut json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Error converting: {}", e)))?;
    
    // Wenn es ein Objekt mit "data" Key ist (von CSV → TOML), extrahiere das Array
    if let serde_json::Value::Object(ref obj) = json_value {
        if let Some(data_value) = obj.get("data") {
            json_value = data_value.clone();
        }
    }
    
    // Prüfen ob es ein Array ist
    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json_value],
        _ => return Err(FormatError::SerializationError("TOML must be an array or object".to_string()))
    };
    
    if array.is_empty() {
        return Ok(String::new());
    }
    
    // Alle Objekte flattenen
    let flattened: Vec<_> = array.iter()
        .map(|v| flatten_json_value(v, ""))
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

// ============================================================================
// FILE-I/O WRAPPER FUNKTIONEN (nur für CLI)
// ============================================================================

/// Konvertiert TOML zu JSON (File-I/O Wrapper)
pub fn convert_toml_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = toml_to_json_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert TOML zu YAML (File-I/O Wrapper)
pub fn convert_toml_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = toml_to_yaml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert TOML zu TOML (File-I/O Wrapper)
pub fn convert_toml_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = toml_to_toml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert TOML zu CSV (File-I/O Wrapper)
pub fn convert_toml_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    let result = toml_to_csv_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Flattened ein JSON-Objekt zu einer flachen Map mit Unterstrich-Trennzeichen
/// 
/// Beispiel:
/// Input:  {"contact": {"email": "test@test.com", "phone": "+49"}}
/// Output: {"contact_email": "test@test.com", "contact_phone": "+49"}
fn flatten_json_value(value: &serde_json::Value, prefix: &str) -> std::collections::HashMap<String, String> {
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
                    let nested = flatten_json_value(val, &new_key);
                    result.extend(nested);
                } else {
                    result.insert(new_key, json_value_to_string(val));
                }
            }
        }
        _ => {
            // Primitive Werte direkt einfügen
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), json_value_to_string(value));
            }
        }
    }
    
    result
}

/// Konvertiert einen JSON-Wert zu einem String
fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

/// Validiert eine TOML-Datei ohne sie zu schreiben
/// Ähnlich wie validate_json, aber für TOML
/// 
/// Diese Funktion ist zentral für alle TOML-Konvertierungen:
/// Sie liest die Datei, parst sie und gibt ein toml::Value zurück
pub fn validate_toml(input_path: &str) -> Result<toml::Value, FormatError> {
    // 1. Datei lesen - content enthält den TOML-Text als String
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;

    // 2. TOML parsen und validieren
    // toml::from_str wandelt den String in eine toml::Value Struktur um
    // Dies ist die "Intermediate Representation" (IR)
    let toml_value: toml::Value = toml::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Error parsing TOML: {}", e)))?;

    // 3. Validiertes toml::Value zurückgeben
    Ok(toml_value)
}