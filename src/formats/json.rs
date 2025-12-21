// json to other formats



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





// Konvertiert JSON zu formatiertem JSON
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
    // Zuerst input json datei validieren
    let json_value = validate_json(input_path)?;

    // TOML unterstützt kein Array als Root-Element
    // Wenn es ein Array ist, wrappen wir es in ein Objekt
    let toml_value = match json_value {
        serde_json::Value::Array(arr) => {
            // Array in ein Objekt wrappen mit dem Key "items"
            let wrapped = serde_json::json!({ "items": arr });
            toml::to_string(&wrapped)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?
        }
        _ => {
            // Für Objekte und andere Typen normal konvertieren
            toml::to_string(&json_value)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?
        }
    };

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


/// Konvertiert JSON zu CSV
/// csv crate ist eine Rust-Bibliothek für CSV-Verarbeitung und ist bereits in der Cargo.toml hinzugefügt
/// Wichtig: CSV ist flach, JSON kann verschachtelt sein.
/// Diese Funktion behandelt:
/// - Arrays von Objekten → CSV Zeilen
/// - Einzelnes Objekt → Eine CSV Zeile
/// - Verschachtelte Objekte → Werden als JSON-String gespeichert
pub fn convert_json_to_csv(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?; // liest input path und parst es zu einem serde_json::Value, falls fehler, dann FormatError

    // CSV Writer erstellen
    let mut writer = csv::Writer::from_path(output_path) // erzeugt einen writer, der in die output path schreibt
        .map_err(|e| FormatError::IoError(format!("Fehler beim Erstellen der CSV-Datei: {}", e)))?;

    match json_value { // match entscheidet, welche funktion aufgerufen werden soll, basierend auf dem json_value
       
        // Wenn es ein Array ist
        serde_json::Value::Array(arr) => {
            // wenn array leer, dann leere csv datei
            if arr.is_empty() {
                return Ok(());
            }

            // Erste Zeile: Header aus dem ersten Objekt extrahieren
            if let Some(first_obj) = arr.first() { // nimmt erstes objekt aus array
                if let serde_json::Value::Object(_) = first_obj {
                    let headers = extract_keys(first_obj); 
                    writer.write_record(&headers) // die keys des ersten objekts werden als headerzeile für die csv datei geschrieben
                        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Header: {}", e)))?;
                }
            }

            // Datenzeilen schreiben
            for item in arr { // item = objekt aus array, 
                if let serde_json::Value::Object(obj) = item { // prüft ob item ein objekt ist
                    let row = extract_values(&obj); // extrahiert die werte aus den objekten und speichert sie in einem vector
                    writer.write_record(&row)
                        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Zeile: {}", e)))?;
                }
            }
        }

        // Wenn es ein einzelnes Objekt ist, also ein item ist
        serde_json::Value::Object(obj) => {
            // Header schreiben
            let headers = extract_keys_from_map(&obj);
            writer.write_record(&headers)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Header: {}", e)))?;

            // Datenzeile schreiben
            let row = extract_values_from_map(&obj);
            writer.write_record(&row)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Zeile: {}", e)))?;
        }
        // Andere Typen (String, Number, etc.) → Eine Spalte
        _ => {
            writer.write_record(&["value"])
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Header: {}", e)))?;
            writer.write_record(&[json_value.to_string()])
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Zeile: {}", e)))?;
        }
    }

    writer.flush()
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben der CSV-Datei: {}", e)))?;

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


// __________________________________________________________________________________________________________________________
// Helper-Funktionen für CSV-Konvertierung

/// Extrahiert die Schlüssel aus einem JSON-Objekt (für Header)
fn extract_keys(value: &serde_json::Value) -> Vec<String> {
    if let serde_json::Value::Object(obj) = value {
        extract_keys_from_map(obj)
    } else {
        vec!["value".to_string()] // wenn nicht objekt, dann eine spalte mit dem wert "value"
    }
}

/// Extrahiert die Schlüssel aus einer Map
fn extract_keys_from_map(obj: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    obj.keys().cloned().collect()
}

/// Extrahiert die Werte aus einem JSON-Objekt (für Datenzeilen)
fn extract_values(obj: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    extract_values_from_map(obj)
}

/// Konvertiert JSON-Werte zu Strings für CSV
fn extract_values_from_map(obj: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    obj.values()
        .map(|v| value_to_string(v))
        .collect()
}

/// Konvertiert einen JSON-Wert zu einem String
/// Verschachtelte Objekte werden als JSON-String gespeichert
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