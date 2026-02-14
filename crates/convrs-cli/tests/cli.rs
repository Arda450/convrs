//! hier befinden sich die Integrationstests für convrs-cli.
//!
//! Testen das CLI-Binary als Ganzes über std::process::Command.

use std::fs;
use std::process::Command;

/// Hilfsfunktion: Findet das convrs-Binary im target-Verzeichnis.
fn convrs_bin() -> Command {
    // In einem Workspace liegt das Binary unter target/debug/convrs
    Command::new(env!("CARGO_BIN_EXE_convrs"))
}

#[test]
fn cli_version_flag() {
    let output = convrs_bin()
        .arg("--version")
        .output()
        .expect("Failed to run convrs");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("convrs"));
}

#[test]
fn cli_help_flag() {
    let output = convrs_bin()
        .arg("--help")
        .output()
        .expect("Failed to run convrs");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Format-Converter"));
}

#[test]
fn cli_convert_json_to_yaml() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("input.json");
    let output_path = dir.path().join("output.yaml");

    fs::write(&input_path, r#"{"name":"Alice","age":30}"#).unwrap();

    let output = convrs_bin()
        .arg("convert")
        .arg("-i")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed to run convrs");

    assert!(output.status.success());

    let result = fs::read_to_string(&output_path).unwrap();
    assert!(result.contains("name"));
    assert!(result.contains("Alice"));
}

#[test]
fn cli_convert_csv_to_json() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("data.csv");
    let output_path = dir.path().join("data.json");

    fs::write(&input_path, "name,age\nBob,25").unwrap();

    let output = convrs_bin()
        .arg("convert")
        .arg("-i")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed to run convrs");

    assert!(output.status.success());

    let result = fs::read_to_string(&output_path).unwrap();
    assert!(result.contains("Bob"));
    assert!(result.contains("25"));
}

#[test]
fn cli_error_on_invalid_input() {
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_path = dir.path().join("broken.json");
    let output_path = dir.path().join("output.yaml");

    fs::write(&input_path, "{ broken json ").unwrap();

    let output = convrs_bin()
        .arg("convert")
        .arg("-i")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed to run convrs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error"));
}

#[test]
fn cli_error_on_nonexistent_file() {
    let output = convrs_bin()
        .arg("convert")
        .arg("-i")
        .arg("nonexistent.json")
        .arg("-o")
        .arg("output.yaml")
        .output()
        .expect("Failed to run convrs");

    assert!(!output.status.success());
}
