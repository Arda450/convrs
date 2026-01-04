// Beispiel: Elegante Verwendung des FileFormat Enums

use asp_cli::FileFormat;
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ============================================================================
    // Beispiel 1: Parse von Extension
    // ============================================================================
    
    let input_path = Path::new("data.json");
    let ext = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("json");
    
    let input_format: FileFormat = ext.parse()?;  // FromStr Trait!
    println!("Input Format: {}", input_format);
    
    // ============================================================================
    // Beispiel 2: Einfache Konvertierung
    // ============================================================================
    
    let json_input = r#"{"name": "Test", "value": 42}"#;
    let yaml_output = FileFormat::Json.convert(json_input, FileFormat::Yaml)?;
    
    println!("\nJSON Input:\n{}", json_input);
    println!("\nYAML Output:\n{}", yaml_output);
    
    // ============================================================================
    // Beispiel 3: Chaining
    // ============================================================================
    
    let json = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    
    // JSON → TOML → YAML → CSV
    let toml = FileFormat::Json.convert(json, FileFormat::Toml)?;
    let yaml = FileFormat::Toml.convert(&toml, FileFormat::Yaml)?;
    let csv = FileFormat::Yaml.convert(&yaml, FileFormat::Csv)?;
    
    println!("\nChained Conversion:");
    println!("JSON → TOML → YAML → CSV\n{}", csv);
    
    // ============================================================================
    // Beispiel 4: Pattern Matching
    // ============================================================================
    
    let format = FileFormat::Json;
    match format {
        FileFormat::Json => println!("\nWir arbeiten mit JSON!"),
        FileFormat::Yaml => println!("\nWir arbeiten mit YAML!"),
        FileFormat::Toml => println!("\nWir arbeiten mit TOML!"),
        FileFormat::Csv => println!("\nWir arbeiten mit CSV!"),
    }
    
    // ============================================================================
    // Beispiel 5: Runtime Format-Auswahl
    // ============================================================================
    
    let formats = vec!["json", "yaml", "toml", "csv"];
    
    for fmt_str in formats {
        let fmt = FileFormat::from_str(fmt_str)?;
        println!("Format: {} → Extension: {}", fmt, fmt.extension());
    }
    
    Ok(())
}

