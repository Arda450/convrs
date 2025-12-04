// json to other formats

// Hauptmodul mit Exporten



// use serde::{Deserialize, Serialize};
use std::fs;
use crate::error::FormatError;

/// Beispiel-Structs für typsichere JSON-Verarbeitung
/// Diese können an deine spezifischen JSON-Strukturen angepasst werden
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Config {
//     pub host: String,
//     pub port: u16,
//     pub debug: bool,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Metadata {
//     pub author: String,
//     pub created: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ProjectData {
//     pub name: String,
//     pub version: String,
//     pub description: String,
//     pub config: Config,
//     pub features: Vec<String>,
//     pub metadata: Metadata,
// }





/// Konvertiert JSON zu formatiertem JSON
pub fn convert_json_to_json(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?;

    // JSON formatiert ausgeben (pretty print mit 2 Leerzeichen Einrückung)
    let formatted_json = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von JSON: {}", e)))?;

    // In Ausgabedatei schreiben
    fs::write(output_path, formatted_json)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

pub fn convert_json_to_toml(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?;

    // Zu TOML konvertieren
    let toml_value = toml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?;

    // In Ausgabedatei schreiben
    fs::write(output_path, toml_value)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}


/// Konvertiert JSON zu formatiertem YAML
pub fn convert_json_to_yaml(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?;

    // Zu YAML konvertieren
    let formatted_yaml = serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von YAML: {}", e)))?;

    // In Ausgabedatei schreiben
    fs::write(output_path, formatted_yaml)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}


/// Validiert eine JSON-Datei ohne sie zu schreiben
pub fn validate_json(input_path: &str) -> Result<serde_json::Value, FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;

        // serde_json::from_str prüft ob syntax korrekt ist
        // prüft ob syntax korrekt ist
    let json_value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Parsen von JSON: {}", e)))?;

    Ok(json_value)
}