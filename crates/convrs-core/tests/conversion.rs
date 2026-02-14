//! Integrationstests für convrs-core.
//!
//! Testen die öffentliche API (FileFormat::convert, FromStr, etc.)
//! über Formatgrenzen hinweg mit realistischen Eingabedaten.

use convrs_core::{FileFormat, FormatError};
use std::str::FromStr;

// hier befinden sich die roundtrip-tests: Format A → B → A

#[test]
fn roundtrip_json_yaml() {
    let json = r#"{"name":"convrs","version":1}"#;

    let yaml = FileFormat::Json
        .convert(json, FileFormat::Yaml)
        .expect("JSON → YAML");
    assert!(yaml.contains("name") && yaml.contains("convrs"));

    let back = FileFormat::Yaml
        .convert(&yaml, FileFormat::Json)
        .expect("YAML → JSON");
    assert!(back.contains("convrs") && back.contains("version"));
}

#[test]
fn roundtrip_toml_json() {
    let toml_input = "title = \"Test\"\n\n[section]\nkey = \"value\"";

    let json = FileFormat::Toml
        .convert(toml_input, FileFormat::Json)
        .expect("TOML → JSON");
    assert!(json.contains("title") && json.contains("section"));

    let toml_back = FileFormat::Json
        .convert(&json, FileFormat::Toml)
        .expect("JSON → TOML");
    assert!(toml_back.contains("key") && toml_back.contains("value"));
}

#[test]
fn roundtrip_csv_json() {
    let csv = "name,age\nAlice,30\nBob,25";

    let json = FileFormat::Csv
        .convert(csv, FileFormat::Json)
        .expect("CSV → JSON");
    assert!(json.contains("Alice") && json.contains("Bob"));

    let csv_back = FileFormat::Json
        .convert(&json, FileFormat::Csv)
        .expect("JSON → CSV");
    assert!(csv_back.contains("name") && csv_back.contains("age"));
    assert!(csv_back.contains("Alice") && csv_back.contains("30"));
}

#[test]
fn roundtrip_yaml_toml() {
    let yaml = "title: Test\ncount: 10";

    let toml = FileFormat::Yaml
        .convert(yaml, FileFormat::Toml)
        .expect("YAML → TOML");
    assert!(toml.contains("title") && toml.contains("Test"));

    let yaml_back = FileFormat::Toml
        .convert(&toml, FileFormat::Yaml)
        .expect("TOML → YAML");
    assert!(yaml_back.contains("title") && yaml_back.contains("Test"));
}

// hier befinden sich die identity-tests: Format → gleiches Format (Pretty-Printing)

#[test]
fn identity_all_formats() {
    let json = r#"{"a":1}"#;
    let toml = "key = \"value\"";
    let yaml = "name: test";
    let csv = "a,b\n1,2";

    assert!(FileFormat::Json.convert(json, FileFormat::Json).is_ok());
    assert!(FileFormat::Toml.convert(toml, FileFormat::Toml).is_ok());
    assert!(FileFormat::Yaml.convert(yaml, FileFormat::Yaml).is_ok());
    assert!(FileFormat::Csv.convert(csv, FileFormat::Csv).is_ok());
}

#[test]
fn identity_json_pretty_prints() {
    let minified = r#"{"name":"Alice","age":30}"#;
    let result = FileFormat::Json
        .convert(minified, FileFormat::Json)
        .unwrap();
    // Pretty-printed enthält Einrückung
    assert!(result.contains("  "));
    assert!(result.contains('\n'));
}

// hier befinden sich die format-parsing-tests

#[test]
fn parse_format_from_extension() {
    assert_eq!(FileFormat::from_str("json").unwrap(), FileFormat::Json);
    assert_eq!(FileFormat::from_str("yaml").unwrap(), FileFormat::Yaml);
    assert_eq!(FileFormat::from_str("yml").unwrap(), FileFormat::Yaml);
    assert_eq!(FileFormat::from_str("toml").unwrap(), FileFormat::Toml);
    assert_eq!(FileFormat::from_str("csv").unwrap(), FileFormat::Csv);
}

#[test]
fn parse_format_case_insensitive() {
    assert_eq!(FileFormat::from_str("JSON").unwrap(), FileFormat::Json);
    assert_eq!(FileFormat::from_str("Yaml").unwrap(), FileFormat::Yaml);
    assert_eq!(FileFormat::from_str("TOML").unwrap(), FileFormat::Toml);
}

#[test]
fn parse_format_unknown_fails() {
    assert!(FileFormat::from_str("xml").is_err());
    assert!(FileFormat::from_str("html").is_err());
    assert!(FileFormat::from_str("").is_err());
}

// hier befinden sich fehlerbehandlung-tests

#[test]
fn invalid_json_returns_parse_error() {
    let result = FileFormat::Json.convert("not valid json {{{", FileFormat::Yaml);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FormatError::ParseError(_)));
}

#[test]
fn invalid_toml_returns_parse_error() {
    let result = FileFormat::Toml.convert("not valid toml [ [ [", FileFormat::Json);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FormatError::ParseError(_)));
}

#[test]
fn invalid_csv_empty_returns_parse_error() {
    let result = FileFormat::Csv.convert("", FileFormat::Json);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FormatError::ParseError(_)));
}

// hier befinden sich die cross-format-chain-tests: A → B → C → D

#[test]
fn chain_csv_json_yaml_toml() {
    let csv = "name,age\nAlice,30";

    // CSV → JSON
    let json = FileFormat::Csv.convert(csv, FileFormat::Json).unwrap();
    assert!(json.contains("Alice"));

    // JSON → YAML
    let yaml = FileFormat::Json.convert(&json, FileFormat::Yaml).unwrap();
    assert!(yaml.contains("Alice"));

    // YAML → TOML
    let toml = FileFormat::Yaml.convert(&yaml, FileFormat::Toml).unwrap();
    assert!(toml.contains("Alice"));
}

// hier befinden sich die edge-case tests

#[test]
fn json_array_to_toml_wraps_in_data() {
    let input = r#"[{"x":1},{"x":2}]"#;
    let result = FileFormat::Json.convert(input, FileFormat::Toml).unwrap();
    // Arrays werden in [[data]] gewrappt
    assert!(result.contains("data"));
}

#[test]
fn nested_json_to_csv_flattens() {
    let input = r#"[{"user":{"name":"Alice","address":{"city":"Zürich"}}}]"#;
    let result = FileFormat::Json.convert(input, FileFormat::Csv).unwrap();
    assert!(result.contains("user_name"));
    assert!(result.contains("user_address_city"));
    assert!(result.contains("Alice"));
    assert!(result.contains("Zürich"));
}
