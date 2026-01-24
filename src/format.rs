// Elegante Format-Konvertierung mit Enum + Trait Pattern

use std::str::FromStr;
use crate::error::FormatError;

// Import aller String-Konvertierungsfunktionen
use crate::formats::json::{json_to_json_string, json_to_toml_string, json_to_yaml_string, json_to_csv_string};
use crate::formats::toml::{toml_to_json_string, toml_to_toml_string, toml_to_yaml_string, toml_to_csv_string};
use crate::formats::yaml::{yaml_to_json_string, yaml_to_toml_string, yaml_to_yaml_string, yaml_to_csv_string};
use crate::formats::csv::{csv_to_json_string, csv_to_toml_string, csv_to_yaml_string, csv_to_csv_string};

/// Unterstützte Dateiformate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Json,
    Toml,
    Yaml,
    Csv,
}

impl FileFormat {
    pub fn convert(&self, input: &str, output_format: FileFormat) -> Result<String, FormatError> {
        match (self, output_format) {
            // JSON als Quelle
            (FileFormat::Json, FileFormat::Json) => json_to_json_string(input),
            (FileFormat::Json, FileFormat::Toml) => json_to_toml_string(input),
            (FileFormat::Json, FileFormat::Yaml) => json_to_yaml_string(input),
            (FileFormat::Json, FileFormat::Csv) => json_to_csv_string(input),
            
            // TOML als Quelle
            (FileFormat::Toml, FileFormat::Json) => toml_to_json_string(input),
            (FileFormat::Toml, FileFormat::Toml) => toml_to_toml_string(input),
            (FileFormat::Toml, FileFormat::Yaml) => toml_to_yaml_string(input),
            (FileFormat::Toml, FileFormat::Csv) => toml_to_csv_string(input),
            
            // YAML als Quelle
            (FileFormat::Yaml, FileFormat::Json) => yaml_to_json_string(input),
            (FileFormat::Yaml, FileFormat::Toml) => yaml_to_toml_string(input),
            (FileFormat::Yaml, FileFormat::Yaml) => yaml_to_yaml_string(input),
            (FileFormat::Yaml, FileFormat::Csv) => yaml_to_csv_string(input),
            
            // CSV als Quelle
            (FileFormat::Csv, FileFormat::Json) => csv_to_json_string(input),
            (FileFormat::Csv, FileFormat::Toml) => csv_to_toml_string(input),
            (FileFormat::Csv, FileFormat::Yaml) => csv_to_yaml_string(input),
            (FileFormat::Csv, FileFormat::Csv) => csv_to_csv_string(input),
        }
    }
    
    /// Gibt den Format-Namen als String zurück
    pub fn as_str(&self) -> &'static str {
        match self {
            FileFormat::Json => "json",
            FileFormat::Toml => "toml",
            FileFormat::Yaml => "yaml",
            FileFormat::Csv => "csv",
        }
    }
    
    /// Gibt die Standard-Dateierweiterung zurück
    pub fn extension(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for FileFormat {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(FileFormat::Json), // "json" string zu fileformat enum konvertieren
            "toml" => Ok(FileFormat::Toml),
            "yaml" | "yml" => Ok(FileFormat::Yaml),
            "csv" => Ok(FileFormat::Csv),
            _ => Err(FormatError::ParseError(format!("Unknown format: {}", s))),
        }
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("json".parse::<FileFormat>().unwrap(), FileFormat::Json);
        assert_eq!("YAML".parse::<FileFormat>().unwrap(), FileFormat::Yaml);
        assert_eq!("yml".parse::<FileFormat>().unwrap(), FileFormat::Yaml);
        assert_eq!("toml".parse::<FileFormat>().unwrap(), FileFormat::Toml);
        assert_eq!("CSV".parse::<FileFormat>().unwrap(), FileFormat::Csv);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(FileFormat::Json.as_str(), "json");
        assert_eq!(FileFormat::Toml.as_str(), "toml");
        assert_eq!(FileFormat::Yaml.as_str(), "yaml");
        assert_eq!(FileFormat::Csv.as_str(), "csv");
    }

    #[test]
    fn test_convert() {
        let json_input = r#"{"name":"Test","value":42}"#;
        let result = FileFormat::Json.convert(json_input, FileFormat::Json);
        assert!(result.is_ok());
    }
}

