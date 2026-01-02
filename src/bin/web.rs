use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlSelectElement, HtmlTextAreaElement, HtmlButtonElement};

// Import für direkte String-Konvertierung
use csv::ReaderBuilder;
use serde_json::Value as JsonValue;
use asp_cli::formats::utils::json_to_toml_value;

fn main() {
    // Für WASM wird die start() Funktion automatisch aufgerufen
}

// String-zu-String Konvertierungsfunktionen für das Web-UI
fn perform_conversion(input_text: &str, input_format: &str, output_format: &str) -> Result<String, String> {
    match (input_format, output_format) {
        // JSON als Quelle
        ("JSON", "JSON") => json_to_json(input_text),
        ("JSON", "TOML") => json_to_toml(input_text),
        ("JSON", "YAML") => json_to_yaml(input_text),
        ("JSON", "CSV") => json_to_csv(input_text),
        
        // TOML als Quelle
        ("TOML", "JSON") => toml_to_json(input_text),
        ("TOML", "TOML") => toml_to_toml(input_text),
        ("TOML", "YAML") => toml_to_yaml(input_text),
        ("TOML", "CSV") => toml_to_csv(input_text),
        
        // YAML als Quelle
        ("YAML", "JSON") => yaml_to_json(input_text),
        ("YAML", "TOML") => yaml_to_toml(input_text),
        ("YAML", "YAML") => yaml_to_yaml(input_text),
        ("YAML", "CSV") => yaml_to_csv(input_text),
        
        // CSV als Quelle
        ("CSV", "JSON") => csv_to_json(input_text),
        ("CSV", "TOML") => csv_to_toml(input_text),
        ("CSV", "YAML") => csv_to_yaml(input_text),
        ("CSV", "CSV") => csv_to_csv(input_text),
        
        _ => Err(format!("Unbekannte Konvertierung: {} → {}", input_format, output_format)),
    }
}

// ============================================================================
// JSON Konvertierungen
// ============================================================================

fn json_to_json(input: &str) -> Result<String, String> {
    let value: JsonValue = serde_json::from_str(input)
        .map_err(|e| format!("Ungültiges JSON: {}", e))?;
    serde_json::to_string_pretty(&value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn json_to_toml(input: &str) -> Result<String, String> {
    let json_value: JsonValue = serde_json::from_str(input)
        .map_err(|e| format!("Ungültiges JSON: {}", e))?;
    
    // TOML kann kein Array als Root haben - wrappen in Objekt
    let wrapped_json = if json_value.is_array() {
        let mut root = serde_json::Map::new();
        root.insert("data".to_string(), json_value);
        JsonValue::Object(root)
    } else {
        json_value
    };
    
    let toml_value = json_to_toml_value(&wrapped_json)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    toml::to_string_pretty(&toml_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn json_to_yaml(input: &str) -> Result<String, String> {
    let value: JsonValue = serde_json::from_str(input)
        .map_err(|e| format!("Ungültiges JSON: {}", e))?;
    serde_yaml::to_string(&value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))
}

fn json_to_csv(input: &str) -> Result<String, String> {
    let json_value: JsonValue = serde_json::from_str(input)
        .map_err(|e| format!("Ungültiges JSON: {}", e))?;
    
    // JSON Array zu CSV
    if let JsonValue::Array(records) = json_value {
        if records.is_empty() {
            return Ok(String::new());
        }
        
        let mut wtr = csv::Writer::from_writer(vec![]);
        
        // Flatten der Records
        let flattened_records: Vec<_> = records.iter()
            .map(|r| flatten_value(r, ""))
            .collect();
        
        if flattened_records.is_empty() {
            return Ok(String::new());
        }
        
        // Sammle alle eindeutigen Keys für Header
        let mut headers = std::collections::HashSet::new();
        for record in &flattened_records {
            for key in record.keys() {
                headers.insert(key.clone());
            }
        }
        let mut headers: Vec<String> = headers.into_iter().collect();
        headers.sort();
        
        // Header schreiben
        wtr.write_record(&headers).map_err(|e| format!("CSV Error: {}", e))?;
        
        // Daten schreiben
        for record in &flattened_records {
            let row: Vec<String> = headers.iter()
                .map(|k| record.get(k).cloned().unwrap_or_default())
                .collect();
            wtr.write_record(&row).map_err(|e| format!("CSV Error: {}", e))?;
        }
        
        String::from_utf8(wtr.into_inner().map_err(|e| format!("CSV Error: {}", e))?)
            .map_err(|e| format!("UTF-8 Error: {}", e))
    } else if let JsonValue::Object(_) = json_value {
        // Einzelnes Objekt in Array wrappen
        let array = JsonValue::Array(vec![json_value]);
        json_to_csv(&serde_json::to_string(&array).unwrap())
    } else {
        Err("JSON muss ein Array oder Objekt sein für CSV".to_string())
    }
}

// Hilfsfunktion zum Flatten eines JSON Values
fn flatten_value(value: &JsonValue, prefix: &str) -> std::collections::HashMap<String, String> {
    let mut result = std::collections::HashMap::new();
    
    match value {
        JsonValue::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}_{}", prefix, key)
                };
                
                match val {
                    JsonValue::Object(_) | JsonValue::Array(_) => {
                        let nested = flatten_value(val, &new_prefix);
                        result.extend(nested);
                    }
                    _ => {
                        result.insert(new_prefix, value_to_string(val));
                    }
                }
            }
        }
        JsonValue::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let new_prefix = format!("{}_{}", prefix, i);
                let nested = flatten_value(val, &new_prefix);
                result.extend(nested);
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), value_to_string(value));
            }
        }
    }
    
    result
}

// Hilfsfunktion zum Konvertieren von JSON Value zu String
fn value_to_string(value: &JsonValue) -> String {
    match value {
        JsonValue::String(s) => s.clone(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => String::new(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

// ============================================================================
// TOML Konvertierungen
// ============================================================================

fn toml_to_json(input: &str) -> Result<String, String> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| format!("Ungültiges TOML: {}", e))?;
    let json_value: JsonValue = serde_json::to_value(&toml_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn toml_to_toml(input: &str) -> Result<String, String> {
    let value: toml::Value = toml::from_str(input)
        .map_err(|e| format!("Ungültiges TOML: {}", e))?;
    toml::to_string_pretty(&value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn toml_to_yaml(input: &str) -> Result<String, String> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| format!("Ungültiges TOML: {}", e))?;
    let json_value: JsonValue = serde_json::to_value(&toml_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    serde_yaml::to_string(&json_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn toml_to_csv(input: &str) -> Result<String, String> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| format!("Ungültiges TOML: {}", e))?;
    let json_value: JsonValue = serde_json::to_value(&toml_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    json_to_csv(&serde_json::to_string(&json_value).unwrap())
}

// ============================================================================
// YAML Konvertierungen
// ============================================================================

fn yaml_to_json(input: &str) -> Result<String, String> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| format!("Ungültiges YAML: {}", e))?;
    let json_value: JsonValue = serde_json::to_value(&yaml_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn yaml_to_toml(input: &str) -> Result<String, String> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| format!("Ungültiges YAML: {}", e))?;
    let json_value: JsonValue = serde_json::to_value(&yaml_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    let toml_value = json_to_toml_value(&json_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    toml::to_string_pretty(&toml_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn yaml_to_yaml(input: &str) -> Result<String, String> {
    let value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| format!("Ungültiges YAML: {}", e))?;
    serde_yaml::to_string(&value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn yaml_to_csv(input: &str) -> Result<String, String> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| format!("Ungültiges YAML: {}", e))?;
    let json_value: JsonValue = serde_json::to_value(&yaml_value)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    json_to_csv(&serde_json::to_string(&json_value).unwrap())
}

// ============================================================================
// CSV Konvertierungen
// ============================================================================

fn csv_to_json(input: &str) -> Result<String, String> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());
    
    let headers = reader.headers()
        .map_err(|e| format!("CSV Error: {}", e))?
        .clone();
    
    let mut records = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| format!("CSV Error: {}", e))?;
        let mut obj = serde_json::Map::new();
        
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                obj.insert(header.to_string(), JsonValue::String(field.to_string()));
            }
        }
        records.push(JsonValue::Object(obj));
    }
    
    let json_value = JsonValue::Array(records);
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn csv_to_toml(input: &str) -> Result<String, String> {
    let json_str = csv_to_json(input)?;
    let json_value: JsonValue = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON Error: {}", e))?;
    
    // TOML braucht ein Objekt als Root
    let mut root = serde_json::Map::new();
    root.insert("data".to_string(), json_value);
    let wrapped = JsonValue::Object(root);
    
    let toml_value = json_to_toml_value(&wrapped)
        .map_err(|e| format!("Fehler bei der Konvertierung: {}", e))?;
    toml::to_string_pretty(&toml_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn csv_to_yaml(input: &str) -> Result<String, String> {
    let json_str = csv_to_json(input)?;
    let json_value: JsonValue = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON Error: {}", e))?;
    serde_yaml::to_string(&json_value)
        .map_err(|e| format!("Fehler beim Formatieren: {}", e))
}

fn csv_to_csv(input: &str) -> Result<String, String> {
    // CSV zu CSV = Formatierung
    let json_str = csv_to_json(input)?;
    let json_value: JsonValue = serde_json::from_str(&json_str)
        .map_err(|e| format!("JSON Error: {}", e))?;
    json_to_csv(&serde_json::to_string(&json_value).unwrap())
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Panic-Hook für bessere Fehlerausgaben
    console_error_panic_hook::set_once();
    
    console::log_1(&"ASP CLI Web-Version wird initialisiert...".into());
    
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    
    // Hole HTML-Elemente
    let input_textarea = document
        .get_element_by_id("input")
        .expect("input textarea nicht gefunden")
        .dyn_into::<HtmlTextAreaElement>()?;
    
    let output_textarea = document
        .get_element_by_id("output")
        .expect("output textarea nicht gefunden")
        .dyn_into::<HtmlTextAreaElement>()?;
    
    let input_format = document
        .get_element_by_id("input-format")
        .expect("input-format select nicht gefunden")
        .dyn_into::<HtmlSelectElement>()?;
    
    let output_format = document
        .get_element_by_id("output-format")
        .expect("output-format select nicht gefunden")
        .dyn_into::<HtmlSelectElement>()?;
    
    let convert_button = document
        .get_element_by_id("convert-btn")
        .expect("convert button nicht gefunden")
        .dyn_into::<HtmlButtonElement>()?;
    
    let copy_button = document
        .get_element_by_id("copy-btn")
        .expect("copy button nicht gefunden")
        .dyn_into::<HtmlButtonElement>()?;
    
    let status_div = document
        .get_element_by_id("status")
        .expect("status div nicht gefunden");

    // Convert Button Event
    let input_ta = input_textarea.clone();
    let output_ta = output_textarea.clone();
    let input_fmt = input_format.clone();
    let output_fmt = output_format.clone();
    let status = status_div.clone();
    
    let convert_closure = Closure::wrap(Box::new(move || {
        let input_text = input_ta.value();
        let input_format_val = input_fmt.value();
        let output_format_val = output_fmt.value();
        
        if input_text.is_empty() {
            status.set_inner_html("❌ Error: Input is empty!");
            return;
        }
        
        // Echte Konvertierung durchführen
        match perform_conversion(&input_text, &input_format_val, &output_format_val) {
            Ok(output_text) => {
                output_ta.set_value(&output_text);
                status.set_inner_html(&format!("✅ Converted {} → {}", input_format_val, output_format_val));
            }
            Err(error) => {
                output_ta.set_value(&format!("❌ Conversion Error:\n\n{}", error));
                status.set_inner_html(&format!("❌ Error: {}", error));
            }
        }
    }) as Box<dyn FnMut()>);
    
    convert_button.set_onclick(Some(convert_closure.as_ref().unchecked_ref()));
    convert_closure.forget();

    // Copy Button Event
    let output_ta_copy = output_textarea.clone();
    let status_copy = status_div.clone();
    
    let copy_closure = Closure::wrap(Box::new(move || {
        let text = output_ta_copy.value();
        
        if text.is_empty() {
            status_copy.set_inner_html("❌ Nothing to copy!");
            return;
        }
        
        if let Some(window) = web_sys::window() {
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&text);
            status_copy.set_inner_html("✅ Copied to clipboard!");
        }
    }) as Box<dyn FnMut()>);
    
    copy_button.set_onclick(Some(copy_closure.as_ref().unchecked_ref()));
    copy_closure.forget();
    
    console::log_1(&"ASP CLI Web-Version bereit!".into());
    
    Ok(())
}