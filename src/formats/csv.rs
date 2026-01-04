// CSV zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;
use csv::ReaderBuilder;
use serde_json::Value as JsonValue;

// ============================================================================
// STRING-ZU-STRING FUNKTIONEN (Core-Logik für CLI und Web)
// ============================================================================

/// Konvertiert CSV String zu JSON String
pub fn csv_to_json_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    let json_value = JsonValue::Array(records);
    
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von JSON: {}", e)))
}

/// Konvertiert CSV String zu YAML String
pub fn csv_to_yaml_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    let json_value = JsonValue::Array(records);
    
    serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von YAML: {}", e)))
}

/// Konvertiert CSV String zu TOML String
pub fn csv_to_toml_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    let json_array = JsonValue::Array(records);
    
    // TOML braucht ein Objekt als Root
    let mut root_object = serde_json::Map::new();
    root_object.insert("data".to_string(), json_array);
    let json_value = JsonValue::Object(root_object);
    
    let toml_value = json_to_toml_value(&json_value)?;
    
    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von TOML: {}", e)))
}

/// Konvertiert CSV String zu CSV String (Formatierung)
pub fn csv_to_csv_string(input: &str) -> Result<String, FormatError> {
    let records = parse_csv_to_json_values(input)?;
    
    if records.is_empty() {
        return Ok(String::new());
    }
    
    // Header aus erstem Record extrahieren
    let headers: Vec<String> = if let JsonValue::Object(obj) = &records[0] {
        obj.keys().cloned().collect()
    } else {
        return Err(FormatError::SerializationError("CSV Records müssen Objekte sein".to_string()));
    };
    
    // CSV Writer in Memory
    let mut writer = csv::Writer::from_writer(vec![]);
    
    // Header schreiben
    writer.write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
    
    // Records schreiben
    for record in &records {
        if let JsonValue::Object(obj) = record {
            let row: Vec<String> = headers.iter()
                .map(|h| {
                    obj.get(h)
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                })
                .collect();
            writer.write_record(&row)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Zeile: {}", e)))?;
        }
    }
    
    // Writer in String umwandeln
    let data = writer.into_inner()
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Abschliessen von CSV: {}", e)))?;
    
    String::from_utf8(data)
        .map_err(|e| FormatError::SerializationError(format!("Fehler bei UTF-8 Konvertierung: {}", e)))
}

/// Hilfsfunktion: Parst CSV String zu JSON Values (flach, keine Dot-Notation)
fn parse_csv_to_json_values(input: &str) -> Result<Vec<JsonValue>, FormatError> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());
    
    let headers = reader.headers()
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen der CSV-Header: {}", e)))?
        .clone();
    
    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    
    let mut records = Vec::new();
    
    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen eines CSV-Records: {}", e)))?;
        
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

// ============================================================================
// FILE-I/O WRAPPER FUNKTIONEN (nur für CLI)
// ============================================================================

/// Konvertiert CSV zu JSON (File-I/O Wrapper)
pub fn convert_csv_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = csv_to_json_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert CSV zu YAML (File-I/O Wrapper)
pub fn convert_csv_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = csv_to_yaml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert CSV zu TOML (File-I/O Wrapper)
pub fn convert_csv_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = csv_to_toml_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Konvertiert CSV zu CSV (File-I/O Wrapper)
pub fn convert_csv_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;
    
    let result = csv_to_csv_string(&content)?;
    
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;
    
    Ok(())
}

/// Hilfsfunktion: Versucht den Typ eines String-Wertes zu erkennen
/// 
/// Reihenfolge:
/// 1. Boolean (true/false)
/// 2. Integer
/// 3. Float
/// 4. String (fallback)
fn infer_type(value: &str) -> JsonValue {
    // Leerer String → null
    if value.is_empty() {
        return JsonValue::Null;
    }
    
    // Boolean
    match value.to_lowercase().as_str() {
        "true" => return JsonValue::Bool(true),
        "false" => return JsonValue::Bool(false),
        _ => {}
    }
    
    // Integer
    if let Ok(num) = value.parse::<i64>() {
        return JsonValue::Number(num.into());
    }
    
    // Float
    if let Ok(num) = value.parse::<f64>() {
        if let Some(json_num) = serde_json::Number::from_f64(num) {
            return JsonValue::Number(json_num);
        }
    }
    
    // Fallback: String
    JsonValue::String(value.to_string())
}

/// Konvertiert CSV mit Dot-Notation zu JSON (verschachtelt)
/// Beispiel: "contact.email" wird zu {"contact": {"email": "..."}}
pub fn convert_csv_to_json_nested(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. CSV-Datei lesen und parsen (mit Dot-Notation Support)
    let records = read_csv_to_json_nested(input_path)?;
    
    // 2. Vec → JSON Array
    let json_value = JsonValue::Array(records);
    
    // 3. JSON Value → Pretty-Printed String
    let json_string = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, json_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

// Hinweis: read_csv_to_json_flat wurde durch parse_csv_to_json_values ersetzt (siehe oben)

/// Hilfsfunktion: Liest CSV mit Dot-Notation und erstellt verschachtelte Strukturen
/// 
/// Diese Funktion wird für die separate `convert_csv_to_json_nested()` Funktion verwendet.
/// Beispiel:
/// CSV: name,contact.email,contact.phone
/// JSON: {"name": "...", "contact": {"email": "...", "phone": "..."}}
fn read_csv_to_json_nested(input_path: &str) -> Result<Vec<JsonValue>, FormatError> {
    // 1. CSV-Reader erstellen
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von CSV {}: {}", input_path, e)))?;
    
    // 2. Header lesen
    let headers = reader.headers()
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen der CSV-Header: {}", e)))?
        .clone();
    
    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    
    // 3. Records lesen und in verschachtelte JSON-Objekte konvertieren
    let mut records = Vec::new();
    
    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen eines CSV-Records: {}", e)))?;
        
        // Jedes Record wird ein JSON-Objekt (möglicherweise verschachtelt)
        let mut root = serde_json::Map::new();
        
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = header_vec.get(i) {
                // Typ-Inferenz wie gewohnt
                let value = infer_type(field);
                
                // Verschachtelung durch Dot-Notation erstellen
                insert_nested_value(&mut root, header, value);
            }
        }
        
        records.push(JsonValue::Object(root));
    }
    
    Ok(records)
}

/// Hilfsfunktion: Fügt einen Wert in ein verschachteltes Objekt ein
/// 
/// Beispiel: insert_nested_value(obj, "contact.email", "test@test.com")
/// Erstellt: {"contact": {"email": "test@test.com"}}
fn insert_nested_value(
    obj: &mut serde_json::Map<String, JsonValue>,
    key: &str,
    value: JsonValue
) {
    let parts: Vec<&str> = key.split('.').collect();
    
    if parts.len() == 1 {
        // Einfacher Key ohne Verschachtelung
        obj.insert(key.to_string(), value);
    } else {
        // Verschachtelter Key: z.B. "contact.email"
        let first = parts[0];
        let rest = parts[1..].join(".");
        
        // Hole oder erstelle das verschachtelte Objekt
        let nested = obj.entry(first.to_string())
            .or_insert_with(|| JsonValue::Object(serde_json::Map::new()));
        
        // Stelle sicher, dass es ein Objekt ist
        if let JsonValue::Object(nested_map) = nested {
            // Rekursiv für tiefere Verschachtelung
            insert_nested_value(nested_map, &rest, value);
        }
    }
}

