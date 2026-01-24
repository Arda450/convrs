// CLI-Interface - nur mit "cli" Feature kompiliert

#[cfg(feature = "cli")]
use convrs::{FileFormat, FormatError};
#[cfg(feature = "cli")]
use std::path::Path;
#[cfg(feature = "cli")]
use std::fs;
#[cfg(feature = "cli")]
use std::str::FromStr;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "convrs")]
#[command(about = "Format-Converter for JSON, YAML, TOML, CSV")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Konvertiert von einem Format zu einem anderen
    Convert {
        /// Eingabedatei
        #[arg(short, long)]
        input: String,
        
        /// Ausgabedatei
        #[arg(short, long)]
        output: String,
    },
}

#[cfg(feature = "cli")]
/// Elegante Konvertierung mit FileFormat Enum (viel sauberer als verschachtelte Matches!)
fn convert_based_on_extension(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // 1. Parse Extensions zu FileFormat Enum
    let input_ext = Path::new(input_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| FormatError::ParseError("No input file extension found".to_string()))?;
    
    let output_ext = Path::new(output_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| FormatError::ParseError("No output file extension found".to_string()))?;
    
    let input_format = FileFormat::from_str(input_ext)?;
    let output_format = FileFormat::from_str(output_ext)?;
    
    // 2. Datei lesen
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Error reading from {}: {}", input_path, e)))?;
    
    // 3. Konvertierung durchführen (eine Zeile!)
    let result = input_format.convert(&content, output_format)?;
    
    // 4. Datei schreiben
    fs::write(output_path, result)
        .map_err(|e| FormatError::IoError(format!("Error writing to {}: {}", output_path, e)))?;
    
    Ok(())
}

#[cfg(feature = "cli")]
fn main() {
    // clap parst automatisch die Argumente
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Convert { input, output } => {
            match convert_based_on_extension(&input, &output) {
                Ok(_) => println!("✓ Conversion successful: {} -> {}", input, output),
                Err(e) => {
                    eprintln!("✗ Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("✗ CLI-Feature nicht aktiviert!");
    eprintln!("Bitte kompiliere mit: cargo build --features cli");
    std::process::exit(1);
}
