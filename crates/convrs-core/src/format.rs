//! FileFormat Enum: Kern der Konvertierungsarchitektur.
//!
//! Repräsentiert die vier unterstützten Formate und stellt eine
//! zentrale `convert()`-Methode bereit, die alle 16 Kombinationen abdeckt.

use std::str::FromStr;
use crate::error::FormatError;
use crate::formats::json::{json_to_json_string, json_to_toml_string, json_to_yaml_string, json_to_csv_string};
use crate::formats::toml::{toml_to_json_string, toml_to_toml_string, toml_to_yaml_string, toml_to_csv_string};
use crate::formats::yaml::{yaml_to_json_string, yaml_to_toml_string, yaml_to_yaml_string, yaml_to_csv_string};
use crate::formats::csv::{csv_to_json_string, csv_to_toml_string, csv_to_yaml_string, csv_to_csv_string};

/// Unterstützte Datenformate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Json,
    Toml,
    Yaml,
    Csv,
}

impl FileFormat {
    /// Konvertiert einen Input-String vom aktuellen Format in das Zielformat.
    ///
    /// Alle 16 Kombinationen werden per Pattern Matching abgedeckt.
    
    pub fn convert(&self, input: &str, output_format: FileFormat) -> Result<String, FormatError> {
        match (self, output_format) {
            // für json als quelle
            (FileFormat::Json, FileFormat::Json) => json_to_json_string(input),
            (FileFormat::Json, FileFormat::Toml) => json_to_toml_string(input),
            (FileFormat::Json, FileFormat::Yaml) => json_to_yaml_string(input),
            (FileFormat::Json, FileFormat::Csv) => json_to_csv_string(input),

            // für toml als quelle
            (FileFormat::Toml, FileFormat::Json) => toml_to_json_string(input),
            (FileFormat::Toml, FileFormat::Toml) => toml_to_toml_string(input),
            (FileFormat::Toml, FileFormat::Yaml) => toml_to_yaml_string(input),
            (FileFormat::Toml, FileFormat::Csv) => toml_to_csv_string(input),

            // für yaml als quelle
            (FileFormat::Yaml, FileFormat::Json) => yaml_to_json_string(input),
            (FileFormat::Yaml, FileFormat::Toml) => yaml_to_toml_string(input),
            (FileFormat::Yaml, FileFormat::Yaml) => yaml_to_yaml_string(input),
            (FileFormat::Yaml, FileFormat::Csv) => yaml_to_csv_string(input),

            // für csv als quelle
            (FileFormat::Csv, FileFormat::Json) => csv_to_json_string(input),
            (FileFormat::Csv, FileFormat::Toml) => csv_to_toml_string(input),
            (FileFormat::Csv, FileFormat::Yaml) => csv_to_yaml_string(input),
            (FileFormat::Csv, FileFormat::Csv) => csv_to_csv_string(input),
        }
    }

    /// gibt den format-namen als string zurück
    pub fn as_str(&self) -> &'static str {
        match self {
            FileFormat::Json => "json",
            FileFormat::Toml => "toml",
            FileFormat::Yaml => "yaml",
            FileFormat::Csv => "csv",
        }
    }

    /// das gibt die standard-dateierweiterung für das format zurück.
    pub fn extension(&self) -> &'static str {
        self.as_str()
    }
}

// string in fileformat umwandeln

impl FromStr for FileFormat {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(FileFormat::Json),
            "toml" => Ok(FileFormat::Toml),
            "yaml" | "yml" => Ok(FileFormat::Yaml),
            "csv" => Ok(FileFormat::Csv),
            _ => Err(FormatError::ParseError(format!("Unknown format: {}", s))),
        }
    }
}

// fileformat in string für die ausgabe umwandeln
impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FormatError;

    // FromStr Tests

    #[test]
    fn test_from_str_lowercase() {
        assert_eq!("json".parse::<FileFormat>().unwrap(), FileFormat::Json);
        assert_eq!("toml".parse::<FileFormat>().unwrap(), FileFormat::Toml);
        assert_eq!("yaml".parse::<FileFormat>().unwrap(), FileFormat::Yaml);
        assert_eq!("csv".parse::<FileFormat>().unwrap(), FileFormat::Csv);
    }

    #[test]
    fn test_from_str_uppercase() {
        assert_eq!("JSON".parse::<FileFormat>().unwrap(), FileFormat::Json);
        assert_eq!("YAML".parse::<FileFormat>().unwrap(), FileFormat::Yaml);
        assert_eq!("CSV".parse::<FileFormat>().unwrap(), FileFormat::Csv);
    }

    #[test]
    fn test_from_str_yml_alias() {
        assert_eq!("yml".parse::<FileFormat>().unwrap(), FileFormat::Yaml);
        assert_eq!("YML".parse::<FileFormat>().unwrap(), FileFormat::Yaml);
    }

    #[test]
    fn test_from_str_unknown_format() {
        let err = "xml".parse::<FileFormat>().unwrap_err();
        assert!(matches!(err, FormatError::ParseError(_)));
        assert!(err.to_string().contains("Unknown format"));
    }

    // --- as_str / extension / Display Tests ---

    #[test]
    fn test_as_str() {
        assert_eq!(FileFormat::Json.as_str(), "json");
        assert_eq!(FileFormat::Toml.as_str(), "toml");
        assert_eq!(FileFormat::Yaml.as_str(), "yaml");
        assert_eq!(FileFormat::Csv.as_str(), "csv");
    }

    #[test]
    fn test_extension() {
        assert_eq!(FileFormat::Json.extension(), "json");
        assert_eq!(FileFormat::Yaml.extension(), "yaml");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FileFormat::Json), "json");
        assert_eq!(format!("{}", FileFormat::Csv), "csv");
    }

    // --- convert() Tests ---

    #[test]
    fn test_convert_json_to_json() {
        let input = r#"{"name":"Test","value":42}"#;
        let result = FileFormat::Json.convert(input, FileFormat::Json).unwrap();
        assert!(result.contains("\"name\""));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_convert_json_to_yaml() {
        let input = r#"{"name":"Test","value":42}"#;
        let result = FileFormat::Json.convert(input, FileFormat::Yaml).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_convert_json_to_toml() {
        let input = r#"{"title":"Hello","count":10}"#;
        let result = FileFormat::Json.convert(input, FileFormat::Toml).unwrap();
        assert!(result.contains("title"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_convert_json_to_csv() {
        let input = r#"[{"name":"Alice","age":"30"}]"#;
        let result = FileFormat::Json.convert(input, FileFormat::Csv).unwrap();
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_convert_toml_to_json() {
        let input = "title = \"Hello\"\ncount = 10";
        let result = FileFormat::Toml.convert(input, FileFormat::Json).unwrap();
        assert!(result.contains("title") && result.contains("Hello"));
    }

    #[test]
    fn test_convert_csv_to_json() {
        let input = "name,age\nAlice,30\nBob,25";
        let result = FileFormat::Csv.convert(input, FileFormat::Json).unwrap();
        assert!(result.contains("Alice") && result.contains("30"));
    }

    #[test]
    fn test_convert_invalid_json_fails() {
        let result = FileFormat::Json.convert("{ broken json ", FileFormat::Yaml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FormatError::ParseError(_)));
    }

    #[test]
    fn test_convert_invalid_toml_fails() {
        let result = FileFormat::Toml.convert("not valid toml [ [ [", FileFormat::Json);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FormatError::ParseError(_)));
    }

    #[test]
    fn test_convert_all_identity() {
        assert!(FileFormat::Json.convert(r#"{"a":1}"#, FileFormat::Json).is_ok());
        assert!(FileFormat::Toml.convert("key = \"value\"", FileFormat::Toml).is_ok());
        assert!(FileFormat::Yaml.convert("name: test", FileFormat::Yaml).is_ok());
        assert!(FileFormat::Csv.convert("a,b\n1,2", FileFormat::Csv).is_ok());
    }
}
