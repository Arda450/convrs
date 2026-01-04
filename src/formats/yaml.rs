// YAML zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;

// ============================================================================
// STRING-ZU-STRING FUNKTIONEN (Core-Logik für CLI und Web)
// ============================================================================

/// Konvertiert YAML String zu JSON String
pub fn yaml_to_json_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges YAML: {}", e)))?;
    
    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von JSON: {}", e)))
}

/// Konvertiert YAML String zu YAML String (Pretty-Printing)
pub fn yaml_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges YAML: {}", e)))?;
    
    serde_yaml::to_string(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von YAML: {}", e)))
}

/// Konvertiert YAML String zu TOML String
pub fn yaml_to_toml_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges YAML: {}", e)))?;
    
    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
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
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von TOML: {}", e)))
}

/// Konvertiert YAML String zu CSV String
pub fn yaml_to_csv_string(input: &str) -> Result<String, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges YAML: {}", e)))?;
    
    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // Prüfen ob es ein Array ist
    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => vec![json_value],
        _ => return Err(FormatError::SerializationError("YAML muss ein Array oder Objekt sein".to_string()))
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
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
    
    // Daten schreiben
    for flat_obj in flattened {
        let row: Vec<String> = headers.iter()
            .map(|h| flat_obj.get(h).cloned().unwrap_or_default())
            .collect();
        writer.write_record(&row)
            .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Zeile: {}", e)))?;
    }
    
    // Writer in String umwandeln
    let data = writer.into_inner()
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Abschliessen von CSV: {}", e)))?;
    
    String::from_utf8(data)
        .map_err(|e| FormatError::SerializationError(format!("Fehler bei UTF-8 Konvertierung: {}", e)))
}

// ============================================================================
// FILE-I/O WRAPPER FUNKTIONEN (nur für CLI)
// ============================================================================

/// Konvertiert YAML zu JSON (File-I/O Wrapper)
pub fn convert_yaml_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = yaml_to_json_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert YAML zu YAML (File-I/O Wrapper)
pub fn convert_yaml_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = yaml_to_yaml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert YAML zu TOML (File-I/O Wrapper)
pub fn convert_yaml_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = yaml_to_toml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert YAML zu CSV (File-I/O Wrapper)
pub fn convert_yaml_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = yaml_to_csv_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
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

/// Validiert eine YAML-Datei ohne sie zu schreiben
/// Ähnlich wie validate_json und validate_toml, aber für YAML
/// 
/// Diese Funktion ist zentral für alle YAML-Konvertierungen:
/// Sie liest die Datei, parst sie und gibt ein serde_yaml::Value zurück
pub fn validate_yaml(input_path: &str) -> Result<serde_yaml::Value, FormatError> {
    // 1. Datei lesen - content enthält den YAML-Text als String
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;

    // 2. YAML parsen und validieren
    // serde_yaml::from_str wandelt den String in eine serde_yaml::Value Struktur um
    // Dies ist die "Intermediate Representation" (IR)
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Parsen von YAML: {}", e)))?;

    // 3. Validiertes serde_yaml::Value zurückgeben
    Ok(yaml_value)
}