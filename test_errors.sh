#!/bin/bash
# Test-Skript für Fehlermeldungen

echo "=== Test 1: Datei existiert nicht (IO-Fehler) ==="
cargo run -- nonexistent.json output.json

echo -e "\n=== Test 2: Ungültiges JSON (Parse-Fehler) ==="
echo "{ invalid json }" > invalid.json
cargo run -- invalid.json output.json
rm -f invalid.json

echo -e "\n=== Test 3: Erfolgreiche Konvertierung ==="
cargo run -- test.json output.json

