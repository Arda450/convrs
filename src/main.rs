// CLI-Interface hier implementieren

use asp_cli::formats::json::{convert_json_to_json, convert_json_to_toml, convert_json_to_yaml};
use asp_cli::error::FormatError;
use std::env;
use std::path::Path;

/// Entscheidet basierend auf Dateierweiterung, welche Konvertierungsfunktion aufgerufen werden soll
fn convert_based_on_extension(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Prüft die Dateierweiterung der Ausgabedatei
    let output_ext = Path::new(output_path)
        .extension() // gibt die dateierweiterung der datei zurück, also z.B. "json" oder "toml"
        .and_then(|ext| ext.to_str()) // konvertiert die extension zu einem string, weil rust-os funktionen nicht utf-8 sein können
        .unwrap_or("json");
    
    // match entscheidet, welche funktion aufgerufen werden soll, basierend auf der dateierweiterung
    match output_ext.to_lowercase().as_str() { // to_lowercase konvertiert die string in Kleinbuchstaben
        "json" => convert_json_to_json(input_path, output_path), // wenn mit json endet, zu json konvertieren
        "toml" => convert_json_to_toml(input_path, output_path), // wenn mit toml endet, zu toml konvertieren
        "yaml" => convert_json_to_yaml(input_path, output_path), // wenn mit yaml endet, zu yaml konvertieren
        _ => {
            // Fallback: Standard ist JSON
            convert_json_to_json(input_path, output_path)
        }
    }
}
 // hier fängts an
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Muss mindestens 3 Argumente haben");
        eprintln!("Verwendung: {} <input.json> <output.json>", args[0]);
        eprintln!("Beispiel: {} test.json output.json", args[0]);
        eprintln!("Beispiel: {} test.json output.toml", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = &args[2];
    
    match convert_based_on_extension(input_file, output_file) {
        Ok(_) => println!("✓ Konvertierung erfolgreich: {} -> {}", input_file, output_file),
        Err(e) => {
            eprintln!("✗ Fehler: {}", e);
            std::process::exit(1);
        }
    }
}
