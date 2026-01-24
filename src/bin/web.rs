// greift auf die html elemente zu via web_sys
// registriert event handlers für die buttons und textareas
// hier wird rust zu webassembly kompiliert, damit er im browser ausgeführt werden kann

use wasm_bindgen::prelude::*;
// javascript type casting
use wasm_bindgen::JsCast;
// Rust bindings für alle web api functions
use web_sys::{console, HtmlSelectElement, HtmlTextAreaElement, HtmlButtonElement};

// importiere fileformat enum
use convrs::FileFormat;
use std::str::FromStr;

fn main() {
    // jede rust binary muss eine main funktion haben (Compiler requirement)
    // Für WASM wird die start() Funktion automatisch aufgerufen
}

// Prüft ob es ein echter Syntax-Fehler ist (nicht nur falsches Format)
fn is_syntax_error(error_msg: &str) -> bool {
    // Nur Parse-Fehler können Syntax-Fehler sein
    if !error_msg.starts_with("Parse Error:") {
        return false;
    }
    
    // Fehler bei line 1 column 1-10 deuten fast immer auf falsches Format hin
    // z.B.: "expected value at line 1 column 3" = TOML wurde als JSON gelesen
    if error_msg.contains("line 1") && error_msg.contains("column") {
        // Extrahiere Column-Nummer
        if let Some(col_start) = error_msg.find("column ") {
            let rest = &error_msg[col_start + 7..];
            if let Some(col_end) = rest.find(|c: char| !c.is_ascii_digit()) {
                if let Ok(column) = rest[..col_end].parse::<usize>() {
                    // Fehler in den ersten 10 Spalten von Zeile 1 = wahrscheinlich falsches Format
                    if column <= 10 {
                        return false;
                    }
                }
            }
        }
    }
    
    true
}

// Extrahiert Zeilennummer aus Fehlermeldung (z.B. "line 31" -> 31)
// Gibt nur eine Zeilennummer zurück, wenn es ein echter Syntax-Fehler ist
fn extract_error_line(error_msg: &str) -> Option<usize> {
    // Nur bei echten Syntax-Fehlern eine Zeile extrahieren
    if !is_syntax_error(error_msg) {
        return None;
    }
    
    // Suche nach "line X" oder "at line X"
    if let Some(line_start) = error_msg.find("line ") {
        let rest = &error_msg[line_start + 5..];
        if let Some(line_end) = rest.find(|c: char| !c.is_ascii_digit() && c != ',') {
            if let Ok(parsed_line) = rest[..line_end].trim_end_matches(',').parse::<usize>() {
                return Some(parsed_line);
            }
        }
    }
    None
}

// konvertierung mit fileformat enum
fn perform_conversion(input_text: &str, input_format: &str, output_format: &str) -> Result<String, String> {
    // Parse Format-Strings zu Enum
    let input_fmt = FileFormat::from_str(input_format)
        .map_err(|e| format!("Invalid input format: {}", e))?;
    
    let output_fmt = FileFormat::from_str(output_format)
        .map_err(|e| format!("Invalid output format: {}", e))?;
    
    // Konvertierung durchführen, aber nur eine zeile. kommt von der format.rs file, um die datei zu lesen und zu schreiben
    input_fmt.convert(input_text, output_fmt)
        .map_err(|e| e.to_string())
}

// hier startet das webassembly

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Panic-Hook für bessere Fehlerausgaben, zeigt die stack trace an im browser an
    console_error_panic_hook::set_once();
    
    console::log_1(&"[convrs] initializing web interface...".into());
    
    // browser window zugriff
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    // rust bekommt direkten zugriff auf die html elemente mit get_element_by_id
    // Hole HTML-Elemente via id von index.html im root directory
    let input_textarea = document
        .get_element_by_id("input")
        .expect("input textarea not found")
        .dyn_into::<HtmlTextAreaElement>()?;
    
    let output_textarea = document
        .get_element_by_id("output")
        .expect("output textarea not found")
        .dyn_into::<HtmlTextAreaElement>()?;
    
    let input_format = document
        .get_element_by_id("input-format")
        .expect("input-format select not found")
        .dyn_into::<HtmlSelectElement>()?;
    
    let output_format = document
        .get_element_by_id("output-format")
        .expect("output-format select not found")
        .dyn_into::<HtmlSelectElement>()?;
    
    let convert_button = document
        .get_element_by_id("convert-btn")
        .expect("convert button not found")
        .dyn_into::<HtmlButtonElement>()?;
    
    let copy_button = document
        .get_element_by_id("copy-btn")
        .expect("copy button not found")
        .dyn_into::<HtmlButtonElement>()?;
    
    let status_div = document
        .get_element_by_id("status")
        .expect("status div not found");

    // convert button event
    let input_ta = input_textarea.clone();
    let output_ta = output_textarea.clone();
    let input_fmt = input_format.clone();
    let output_fmt = output_format.clone();
    let status = status_div.clone();
    
    // javascript closure, um die event handlers zu registrieren
    let convert_closure = Closure::wrap(Box::new(move || { // move nimmt ownership der variablen und gibt sie an die closure funktion
        let input_text = input_ta.value();
        let input_format_val = input_fmt.value();
        let output_format_val = output_fmt.value();
        
        if input_text.is_empty() {
            status.set_inner_html("error: no input provided");
            return;
        }
        
        // Echte Konvertierung durchführen, kommt von der web.rs file
        match perform_conversion(&input_text, &input_format_val, &output_format_val) {
            Ok(output_text) => {
                output_ta.set_value(&output_text); // zeigt ergebnis an im output textarea
                status.set_inner_html(&format!("success: {} → {} conversion complete", input_format_val.to_lowercase(), output_format_val.to_lowercase()));
                
                // remove error marker on success
                let _ = input_ta.remove_attribute("data-error-line");
            }
            Err(error) => {
                output_ta.set_value(&format!("# CONVERSION ERROR\n\n{}", error));
                status.set_inner_html(&format!("error: {}", error));
                
                // Extrahiere Fehlerzeile und speichere in data-Attribut für JavaScript
                if let Some(error_line) = extract_error_line(&error) {
                    input_ta.set_attribute("data-error-line", &error_line.to_string()).unwrap();
                } else {
                    let _ = input_ta.remove_attribute("data-error-line");
                }
            }
        }
    }) as Box<dyn FnMut()>);
    

    // register the event handler for the convert button
    convert_button.set_onclick(Some(convert_closure.as_ref().unchecked_ref()));
    convert_closure.forget();

    // Copy Button Event
    let output_ta_copy = output_textarea.clone();
    let status_copy = status_div.clone();
    
    let copy_closure = Closure::wrap(Box::new(move || {
        let text = output_ta_copy.value();
        
        if text.is_empty() {
            status_copy.set_inner_html("error: no output to copy");
            return;
        }
        
        if let Some(window) = web_sys::window() {
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&text);
            status_copy.set_inner_html("info: output copied to clipboard");
        }
    }) as Box<dyn FnMut()>);
    
    copy_button.set_onclick(Some(copy_closure.as_ref().unchecked_ref()));
    copy_closure.forget();
    
    console::log_1(&"[convrs] web interface ready".into());
    
    Ok(())
}