//! Nutzt convrs-core für die Konvertierungslogik und ergänzt
//! Argument-Parsing via Clap. Die Konvertierungslogik liegt in lib.rs, damit sie auch in anderen Projekten verwendet werden kann.

use clap::{Parser, Subcommand};

use convrs_cli::convert_file;

// hier befindet sich der CLI-Parser
#[derive(Parser)]
#[command(name = "convrs")]
#[command(about = "Format-Converter for JSON, YAML, TOML, CSV")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        /// Eingabedatei wird anhand der Dateiendung erkannt.
        #[arg(short, long)]
        input: String,

        /// dasselbe für die ausgabedatei
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, output } => match convert_file(&input, &output) {
            Ok(_) => println!("✓ Conversion successful: {} -> {}", input, output),
            Err(e) => {
                eprintln!("✗ Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
