// greift auf die html elemente zu via web_sys
// registriert event handlers für die buttons und textareas
// hier wird rust zu webassembly kompiliert, damit er im browser ausgeführt werden kann

use wasm_bindgen::prelude::*;
// javascript type casting
use wasm_bindgen::JsCast;
// Rust bindings für alle web api functions
use web_sys::{console, HtmlSelectElement, HtmlTextAreaElement, HtmlButtonElement};

// Importiere das elegante FileFormat Enum
use asp_cli::FileFormat;
use std::str::FromStr;

fn main() {
    // jede rust binary muss eine main funktion haben (Compiler requirement)
    // Für WASM wird die start() Funktion automatisch aufgerufen
}

// Elegante Konvertierung mit FileFormat Enum (viel sauberer!)
fn perform_conversion(input_text: &str, input_format: &str, output_format: &str) -> Result<String, String> {
    // Parse Format-Strings zu Enum
    let input_fmt = FileFormat::from_str(input_format)
        .map_err(|e| format!("Ungültiges Input-Format: {}", e))?;
    
    let output_fmt = FileFormat::from_str(output_format)
        .map_err(|e| format!("Ungültiges Output-Format: {}", e))?;
    
    // Konvertierung durchführen - nur EINE Zeile! kommt von der format.rs file
    input_fmt.convert(input_text, output_fmt)
        .map_err(|e| e.to_string())
}

// hier startet das webassembly

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Panic-Hook für bessere Fehlerausgaben, zeigt die stack trace an im browser an
    console_error_panic_hook::set_once();
    
    console::log_1(&"[asp_cli] initializing web interface...".into());
    
    // browser window zugriff
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    // rust bekommt direkten zugriff auf die html elemente mit get_element_by_id
    // Hole HTML-Elemente via id
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
            }
            Err(error) => {
                output_ta.set_value(&format!("# CONVERSION ERROR\n\n{}", error));
                status.set_inner_html(&format!("error: {}", error));
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
    
    console::log_1(&"[asp_cli] web interface ready".into());
    
    Ok(())
}