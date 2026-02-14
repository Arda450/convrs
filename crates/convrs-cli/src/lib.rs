//! CLI-Bibliothek für Datenformat-Konvertierung.
//!
//! Stellt die Konvertierungslogik (File-I/O + Format-Erkennung) bereit,
//! die vom Binary genutzt wird.

use convrs_core::{FileFormat, FormatError};
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Konvertiert eine Datei vom Input- in das Output-Format.
/// Formate werden anhand der Dateiendungen erkannt.
pub fn convert_file(input_path: &str, output_path: &str) -> Result<(), FormatError> {
    // 1. Extensions parsen
    let input_ext = Path::new(input_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| FormatError::ParseError("No input file extension found".to_string()))?;

    let output_ext = Path::new(output_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| FormatError::ParseError("No output file extension found".to_string()))?;

        // hier wird das format aus der dateiendung erkannt und in das FileFormat enum umgewandelt
        // dadurch wird das format bestimmt
    let input_format = FileFormat::from_str(input_ext)?;
    let output_format = FileFormat::from_str(output_ext)?;

    // 2. Datei lesen und rohtext holen
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;

    // 3. Konvertierung vom input-format in das output-format
    let result = input_format.convert(&content, output_format)?;

    // 4. Ergebnis schreiben
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_file_missing_extension() {
        let result = convert_file("noext", "output.json");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("extension"));
    }

    #[test]
    fn test_convert_file_unknown_format() {
        let result = convert_file("input.xml", "output.json");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown format"));
    }

    #[test]
    fn test_convert_file_nonexistent_input() {
        let result = convert_file("nonexistent.json", "output.yaml");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Error reading"));
    }
}
