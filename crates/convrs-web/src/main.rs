//! convrs Web: WebAssembly-Version des Datenformat-Converters.
//!
//! Nutzt convrs-core für die Konvertierungslogik und ergänzt
//! DOM-Interaktion via web-sys und wasm-bindgen für die web-version.
//! 
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlButtonElement, HtmlSelectElement, HtmlTextAreaElement};

use convrs_core::FileFormat;
use std::str::FromStr;

fn main() {
   
}

/// Prüft ob es ein echter Syntax-Fehler ist und nicht das falsche Format.
fn is_syntax_error(error_msg: &str) -> bool {
    if !error_msg.starts_with("Parse Error:") {
        return false;
    }

    // Fehler bei Zeile 1 Spalte 1-10 deuten fast immer auf falsches Format hin
    if error_msg.contains("line 1") && error_msg.contains("column")
        && let Some(col_start) = error_msg.find("column ") {
            let rest = &error_msg[col_start + 7..];
            let col_str = if let Some(col_end) = rest.find(|c: char| !c.is_ascii_digit()) {
                &rest[..col_end]
            } else {
                rest // Zahl am String-Ende
            };
            if let Ok(column) = col_str.parse::<usize>()
                && column <= 10 {
                    return false;
                }
        }

    true
}

/// Extrahiert Zeilennummer aus Fehlermeldung
/// Gibt nur eine Zeilennummer zurück, wenn es ein echter Syntax-Fehler ist.
fn extract_error_line(error_msg: &str) -> Option<usize> {
    if !is_syntax_error(error_msg) {
        return None;
    }

    if let Some(line_start) = error_msg.find("line ") {
        let rest = &error_msg[line_start + 5..];
        if let Some(line_end) = rest.find(|c: char| !c.is_ascii_digit() && c != ',')
            && let Ok(parsed_line) = rest[..line_end].trim_end_matches(',').parse::<usize>() {
                return Some(parsed_line);
            }
    }
    None
}

/// Führt die Konvertierung über die Core-Bibliothek (convrs-core) für die web-version durch.
fn perform_conversion(
    input_text: &str,
    input_format: &str,
    output_format: &str,
) -> Result<String, String> {
    let input_fmt =
        FileFormat::from_str(input_format).map_err(|e| format!("Invalid input format: {}", e))?;

    let output_fmt = FileFormat::from_str(output_format)
        .map_err(|e| format!("Invalid output format: {}", e))?;

    input_fmt
        .convert(input_text, output_fmt)
        .map_err(|e| e.to_string())
}

/// wasm entry-point: initialisiert das web-interface.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console::log_1(&"[convrs] initializing web interface...".into());

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");

    // HTML-Elemente holen
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

    // Convert button event handler
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
            status.set_inner_html("error: no input provided");
            return;
        }

        match perform_conversion(&input_text, &input_format_val, &output_format_val) {
            Ok(output_text) => {
                output_ta.set_value(&output_text);
                status.set_inner_html(&format!(
                    "success: {} → {} conversion complete",
                    input_format_val.to_lowercase(),
                    output_format_val.to_lowercase()
                ));
                let _ = input_ta.remove_attribute("data-error-line");
            }
            Err(error) => {
                output_ta.set_value(&format!("# CONVERSION ERROR\n\n{}", error));
                status.set_inner_html(&format!("error: {}", error));

                if let Some(error_line) = extract_error_line(&error) {
                    input_ta
                        .set_attribute("data-error-line", &error_line.to_string())
                        .unwrap();
                } else {
                    let _ = input_ta.remove_attribute("data-error-line");
                }
            }
        }
    }) as Box<dyn FnMut()>);

    convert_button.set_onclick(Some(convert_closure.as_ref().unchecked_ref()));
    convert_closure.forget();

    // Copy-Button Event-Handler
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform_conversion_json_to_yaml() {
        let result = perform_conversion(r#"{"name":"Test"}"#, "json", "yaml");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("name"));
    }

    #[test]
    fn test_perform_conversion_invalid_format() {
        let result = perform_conversion("{}", "xml", "json");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid input format"));
    }

    #[test]
    fn test_is_syntax_error_real_error() {
        assert!(is_syntax_error("Parse Error: Invalid JSON: line 31 column 5"));
    }

    #[test]
    fn test_is_syntax_error_format_mismatch() {
        // Fehler bei Zeile 1, Spalte 1-10 = wahrscheinlich falsches Format
        assert!(!is_syntax_error("Parse Error: Invalid JSON: line 1 column 3"));
    }

    #[test]
    fn test_is_syntax_error_non_parse() {
        assert!(!is_syntax_error("IO Error: file not found"));
    }

    #[test]
    fn test_extract_error_line() {
        assert_eq!(
            extract_error_line("Parse Error: Invalid JSON: line 31 column 5"),
            Some(31)
        );
    }

    #[test]
    fn test_extract_error_line_no_match() {
        assert_eq!(extract_error_line("IO Error: something"), None);
    }
}
