use crate::FormatError;
use crate::formats::json::{convert_json_to_json, convert_json_to_toml, convert_json_to_yaml, convert_json_to_csv};
use crate::formats::toml::{convert_toml_to_json, convert_toml_to_yaml, convert_toml_to_toml, convert_toml_to_csv};
use crate::formats::yaml::{convert_yaml_to_json, convert_yaml_to_yaml, convert_yaml_to_toml, convert_yaml_to_csv};
use crate::formats::csv::{convert_csv_to_json, convert_csv_to_yaml, convert_csv_to_toml, convert_csv_to_csv};
use std::path::Path;

pub struct App {
    pub input_format: usize,  // 0=JSON, 1=TOML, 2=YAML, 3=CSV
    pub output_format: usize,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub status: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_format: 0,
            output_format: 0,
            input_file: None,
            output_file: None,
            status: "Bereit - Wähle Input/Output Format".to_string(),
        }
    }

    /// Setzt die Eingabedatei und erkennt automatisch das Format
    pub fn set_input_file(&mut self, path: String) {
        self.input_format = Self::detect_format(&path);
        self.input_file = Some(path);
        self.update_status();
    }

    /// Setzt die Ausgabedatei
    pub fn set_output_file(&mut self, path: String) {
        self.output_format = Self::detect_format(&path);
        self.output_file = Some(path);
        self.update_status();
    }

    /// Erkennt Format anhand Dateierweiterung
    fn detect_format(path: &str) -> usize {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        match ext.to_lowercase().as_str() {
            "json" => 0,
            "toml" => 1,
            "yaml" | "yml" => 2,
            "csv" => 3,
            _ => 0, // Fallback zu JSON
        }
    }

    /// Führt die Konvertierung durch
    pub fn convert(&mut self) -> Result<(), FormatError> {
        let input = self.input_file.as_ref()
            .ok_or_else(|| FormatError::SerializationError("Keine Eingabedatei".to_string()))?;
        let output = self.output_file.as_ref()
            .ok_or_else(|| FormatError::SerializationError("Keine Ausgabedatei".to_string()))?;

        // Konvertierung basierend auf Input/Output-Format
        let result = match (self.input_format, self.output_format) {
            // JSON -> ...
            (0, 0) => convert_json_to_json(input, output),
            (0, 1) => convert_json_to_toml(input, output),
            (0, 2) => convert_json_to_yaml(input, output),
            (0, 3) => convert_json_to_csv(input, output),
            
            // TOML -> ...
            (1, 0) => convert_toml_to_json(input, output),
            (1, 1) => convert_toml_to_toml(input, output),
            (1, 2) => convert_toml_to_yaml(input, output),
            (1, 3) => convert_toml_to_csv(input, output),
            
            // YAML -> ...
            (2, 0) => convert_yaml_to_json(input, output),
            (2, 1) => convert_yaml_to_toml(input, output),
            (2, 2) => convert_yaml_to_yaml(input, output),
            (2, 3) => convert_yaml_to_csv(input, output),
            
            // CSV -> ...
            (3, 0) => convert_csv_to_json(input, output),
            (3, 1) => convert_csv_to_toml(input, output),
            (3, 2) => convert_csv_to_yaml(input, output),
            (3, 3) => convert_csv_to_csv(input, output),
            
            _ => Err(FormatError::SerializationError("Ungültiges Format".to_string())),
        };

        match result {
            Ok(_) => {
                self.status = format!("✓ Erfolgreich: {} → {}", 
                    Self::format_name(self.input_format),
                    Self::format_name(self.output_format)
                );
                Ok(())
            }
            Err(e) => {
                self.status = format!("✗ Fehler: {}", e);
                Err(e)
            }
        }
    }

    /// Gibt den Formatnamen zurück
    fn format_name(format: usize) -> &'static str {
        match format {
            0 => "JSON",
            1 => "TOML",
            2 => "YAML",
            3 => "CSV",
            _ => "Unbekannt",
        }
    }

    /// Aktualisiert den Status-Text
    fn update_status(&mut self) {
        if self.input_file.is_some() && self.output_file.is_some() {
            self.status = format!(
                "Bereit: {} → {} (Enter drücken)",
                Self::format_name(self.input_format),
                Self::format_name(self.output_format)
            );
        } else if self.input_file.is_some() {
            self.status = "Ausgabedatei wählen".to_string();
        } else {
            self.status = "Eingabedatei wählen".to_string();
        }
    }
}