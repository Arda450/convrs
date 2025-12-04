use asp_cli::formats::json::convert_json_to_toml;

fn main() {
    match convert_json_to_toml("test.json", "test_output2.toml") {
        Ok(_) => println!("✓ Erfolg"),
        Err(e) => println!("✗ Fehler: {}", e),
    }
}
