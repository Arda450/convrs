// TOML zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;

/// Konvertiert TOML zu JSON
/// Pipeline: TOML → toml::Value (IR) → serde_json::Value → JSON String
pub fn convert_toml_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → serde_json::Value (IR)
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;

    // 3. JSON Value → Pretty-Printed String
    let json_string = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, json_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert TOML zu YAML
/// Pipeline: TOML → toml::Value (IR) → serde_yaml::Value → YAML String
pub fn convert_toml_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → serde_json::Value (als Zwischenschritt, da YAML und JSON ähnlich sind)
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // 3. JSON Value → YAML String
    let yaml_string = serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von YAML: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, yaml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert TOML zu TOML (Pretty-Printing / Formatierung)
/// Pipeline: TOML → toml::Value (IR) → TOML String (neu formatiert)
pub fn convert_toml_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → Pretty-Printed TOML String
    let toml_string = toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?;
    
    // 3. String in Datei schreiben
    fs::write(output_path, toml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert TOML zu CSV
/// ACHTUNG: Funktioniert nur mit einfachen Arrays von Objekten!
/// Pipeline: TOML → toml::Value (IR) → serde_json::Value → CSV
pub fn convert_toml_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → serde_json::Value (CSV-Crate arbeitet besser mit JSON)
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // 3. Prüfen ob es ein Array ist (CSV braucht Array von Objekten)
    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => {
            // Wenn es ein Objekt ist, packen wir es in ein Array
            vec![json_value]
        },
        _ => {
            return Err(FormatError::SerializationError(
                "TOML muss ein Array oder Objekt für CSV-Konvertierung sein".to_string()
            ));
        }
    };
    
    // 4. CSV-Writer erstellen
    let mut writer = csv::Writer::from_path(output_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Erstellen von CSV: {}", e)))?;
    
    // 5. Header schreiben (aus dem ersten Objekt)
    if let Some(first) = array.first() {
        if let serde_json::Value::Object(obj) = first {
            let headers: Vec<String> = obj.keys().cloned().collect();
            writer.write_record(&headers)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
        }
    }
    
    // 6. Daten schreiben
    for item in array {
        if let serde_json::Value::Object(obj) = item {
            let values: Vec<String> = obj.values()
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => String::new(),
                    _ => serde_json::to_string(v).unwrap_or_default(),
                })
                .collect();
            
            writer.write_record(&values)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Daten: {}", e)))?;
        }
    }
    
    // 7. Writer flushen (sicherstellen dass alles geschrieben wurde)
    writer.flush()
        .map_err(|e| FormatError::IoError(format!("Fehler beim Abschliessen von CSV: {}", e)))?;

    Ok(())
}

/// Validiert eine TOML-Datei ohne sie zu schreiben
/// Ähnlich wie validate_json, aber für TOML
/// 
/// Diese Funktion ist zentral für alle TOML-Konvertierungen:
/// Sie liest die Datei, parst sie und gibt ein toml::Value zurück
pub fn validate_toml(input_path: &str) -> Result<toml::Value, FormatError> {
    // 1. Datei lesen - content enthält den TOML-Text als String
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;

    // 2. TOML parsen und validieren
    // toml::from_str wandelt den String in eine toml::Value Struktur um
    // Dies ist die "Intermediate Representation" (IR)
    let toml_value: toml::Value = toml::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Parsen von TOML: {}", e)))?;

    // 3. Validiertes toml::Value zurückgeben
    Ok(toml_value)
}