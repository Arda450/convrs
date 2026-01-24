# IR-Pattern Beispiel für convrs

## Architektur mit 8 Funktionen statt 16

### Dateistruktur:

```
src/
├── ir.rs                    # ← NEU: Zentrale IR-Definition
├── formats/
│   ├── mod.rs
│   ├── json.rs              # Nur 2 Funktionen: to_ir() + from_ir()
│   ├── yaml.rs              # Nur 2 Funktionen: to_ir() + from_ir()
│   ├── toml.rs              # Nur 2 Funktionen: to_ir() + from_ir()
│   └── csv.rs               # Nur 2 Funktionen: to_ir() + from_ir()
```

---

## 1. Zentrale IR-Definition (src/ir.rs)

```rust
// src/ir.rs
// Die universelle Intermediate Representation

use serde::{Deserialize, Serialize};

/// Die zentrale IR - basiert auf serde_json::Value
/// Dies ist die "Sprache", die alle Formate sprechen
pub type IR = serde_json::Value;

// Alternative: Eigene IR-Struktur (mehr Kontrolle)
/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IR {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<IR>),
    Object(std::collections::HashMap<String, IR>),
}
*/
```

---

## 2. JSON Format (src/formats/json.rs)

```rust
// src/formats/json.rs
use crate::ir::IR;
use crate::error::FormatError;

/// Deserialize: JSON String → IR
pub fn json_to_ir(input: &str) -> Result<IR, FormatError> {
    serde_json::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges JSON: {}", e)))
}

/// Serialize: IR → JSON String
pub fn ir_to_json(ir: &IR) -> Result<String, FormatError> {
    serde_json::to_string_pretty(ir)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von JSON: {}", e)))
}
```

---

## 3. YAML Format (src/formats/yaml.rs)

```rust
// src/formats/yaml.rs
use crate::ir::IR;
use crate::error::FormatError;

/// Deserialize: YAML String → IR
pub fn yaml_to_ir(input: &str) -> Result<IR, FormatError> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges YAML: {}", e)))?;
    
    // YAML Value → JSON Value (IR)
    serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::ConversionError(format!("YAML→IR Konvertierung fehlgeschlagen: {}", e)))
}

/// Serialize: IR → YAML String
pub fn ir_to_yaml(ir: &IR) -> Result<String, FormatError> {
    serde_yaml::to_string(ir)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von YAML: {}", e)))
}
```

---

## 4. TOML Format (src/formats/toml.rs)

```rust
// src/formats/toml.rs
use crate::ir::IR;
use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;

/// Deserialize: TOML String → IR
pub fn toml_to_ir(input: &str) -> Result<IR, FormatError> {
    let toml_value: toml::Value = toml::from_str(input)
        .map_err(|e| FormatError::ParseError(format!("Ungültiges TOML: {}", e)))?;
    
    // TOML Value → JSON Value (IR)
    serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::ConversionError(format!("TOML→IR Konvertierung fehlgeschlagen: {}", e)))
}

/// Serialize: IR → TOML String
pub fn ir_to_toml(ir: &IR) -> Result<String, FormatError> {
    // TOML unterstützt kein Array als Root
    let toml_ready_ir = match ir {
        serde_json::Value::Array(arr) => {
            serde_json::json!({ "data": arr })
        }
        _ => ir.clone()
    };
    
    let toml_value = json_to_toml_value(&toml_ready_ir)?;
    
    toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von TOML: {}", e)))
}
```

---

## 5. CSV Format (src/formats/csv.rs)

```rust
// src/formats/csv.rs
use crate::ir::IR;
use crate::error::FormatError;
use csv::ReaderBuilder;

/// Deserialize: CSV String → IR
pub fn csv_to_ir(input: &str) -> Result<IR, FormatError> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());
    
    let headers = reader.headers()
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen der CSV-Header: {}", e)))?
        .clone();
    
    let mut records = Vec::new();
    
    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen einer CSV-Zeile: {}", e)))?;
        
        let mut obj = serde_json::Map::new();
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                obj.insert(header.to_string(), serde_json::Value::String(field.to_string()));
            }
        }
        records.push(serde_json::Value::Object(obj));
    }
    
    Ok(serde_json::Value::Array(records))
}

/// Serialize: IR → CSV String
pub fn ir_to_csv(ir: &IR) -> Result<String, FormatError> {
    // IR muss ein Array sein
    let array = match ir {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => &vec![ir.clone()],
        _ => return Err(FormatError::SerializationError("CSV benötigt ein Array oder Objekt".to_string()))
    };
    
    if array.is_empty() {
        return Ok(String::new());
    }
    
    // Alle Keys sammeln (für Header)
    let mut all_keys = std::collections::BTreeSet::new();
    for item in array {
        if let serde_json::Value::Object(obj) = item {
            for key in obj.keys() {
                all_keys.insert(key.clone());
            }
        }
    }
    let headers: Vec<String> = all_keys.into_iter().collect();
    
    // CSV schreiben
    let mut writer = csv::Writer::from_writer(vec![]);
    
    writer.write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
    
    for item in array {
        if let serde_json::Value::Object(obj) = item {
            let row: Vec<String> = headers.iter()
                .map(|h| {
                    obj.get(h)
                        .and_then(|v| match v {
                            serde_json::Value::String(s) => Some(s.clone()),
                            serde_json::Value::Number(n) => Some(n.to_string()),
                            serde_json::Value::Bool(b) => Some(b.to_string()),
                            serde_json::Value::Null => Some(String::new()),
                            _ => Some(serde_json::to_string(v).unwrap_or_default())
                        })
                        .unwrap_or_default()
                })
                .collect();
            writer.write_record(&row)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben einer CSV-Zeile: {}", e)))?;
        }
    }
    
    let data = writer.into_inner()
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Abschließen von CSV: {}", e)))?;
    
    String::from_utf8(data)
        .map_err(|e| FormatError::SerializationError(format!("Fehler bei UTF-8 Konvertierung: {}", e)))
}
```

---

## 6. Zentrale Konvertierungsfunktion (src/format.rs)

```rust
// src/format.rs
use crate::ir::IR;
use crate::error::FormatError;
use crate::formats::{json, yaml, toml, csv};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Json,
    Yaml,
    Toml,
    Csv,
}

impl FileFormat {
    /// Universelle Konvertierungsfunktion
    /// Nutzt IR als Brücke zwischen allen Formaten
    pub fn convert(&self, input: &str, output_format: FileFormat) -> Result<String, FormatError> {
        // Schritt 1: Input Format → IR
        let ir: IR = match self {
            FileFormat::Json => json::json_to_ir(input)?,
            FileFormat::Yaml => yaml::yaml_to_ir(input)?,
            FileFormat::Toml => toml::toml_to_ir(input)?,
            FileFormat::Csv => csv::csv_to_ir(input)?,
        };
        
        // Schritt 2: IR → Output Format
        match output_format {
            FileFormat::Json => json::ir_to_json(&ir),
            FileFormat::Yaml => yaml::ir_to_yaml(&ir),
            FileFormat::Toml => toml::ir_to_toml(&ir),
            FileFormat::Csv => csv::ir_to_csv(&ir),
        }
    }
}
```

---

## 7. Verwendung

```rust
// Beispiel: YAML zu JSON konvertieren
let yaml_input = "name: Alice\nage: 30";

// Mit dem neuen IR-Pattern:
let result = FileFormat::Yaml.convert(yaml_input, FileFormat::Json)?;

// Intern passiert:
// 1. yaml_to_ir(yaml_input) → IR
// 2. ir_to_json(IR) → JSON String
```

---

## Vergleich: Alt vs. Neu

### ❌ Alte Struktur (16 Funktionen):
```
json_to_json()    json_to_yaml()    json_to_toml()    json_to_csv()
yaml_to_json()    yaml_to_yaml()    yaml_to_toml()    yaml_to_csv()
toml_to_json()    toml_to_yaml()    toml_to_toml()    toml_to_csv()
csv_to_json()     csv_to_yaml()     csv_to_toml()     csv_to_csv()
```

### ✅ Neue Struktur (8 Funktionen):
```
Deserialize (Format → IR):
- json_to_ir()
- yaml_to_ir()
- toml_to_ir()
- csv_to_ir()

Serialize (IR → Format):
- ir_to_json()
- ir_to_yaml()
- ir_to_toml()
- ir_to_csv()
```

---

## Vorteile des IR-Patterns:

1. **Weniger Code**: 8 statt 16 Funktionen
2. **Erweiterbar**: Neues Format? Nur 2 Funktionen hinzufügen
3. **Zentrale Logik**: Alle Konvertierungen laufen über `FileFormat::convert()`
4. **Testbar**: Jede Funktion kann isoliert getestet werden
5. **Klare Trennung**: Deserialize und Serialize sind getrennt

---

## Nachteile:

1. **Abstraktion**: Mehr indirekt (schwerer zu debuggen)
2. **Performance**: Zwei Schritte statt einem (minimal langsamer)
3. **Komplexität**: Braucht zentrale IR-Definition

---

## Datenfluss-Visualisierung:

```
┌─────────┐
│  JSON   │───┐
└─────────┘   │
              │ json_to_ir()
┌─────────┐   │
│  YAML   │───┤
└─────────┘   │ yaml_to_ir()
              │
┌─────────┐   │ toml_to_ir()
│  TOML   │───┤
└─────────┘   │
              │ csv_to_ir()
┌─────────┐   │
│  CSV    │───┘
└─────────┘
              ▼
        ┌───────────┐
        │    IR     │ ◄─── Zentrale Brücke
        │ (Universal)│
        └───────────┘
              │
    ┌─────────┼─────────┬─────────┐
    │         │         │         │
    ▼         ▼         ▼         ▼
┌───────┐ ┌──────┐ ┌──────┐ ┌─────┐
│ JSON  │ │ YAML │ │ TOML │ │ CSV │
└───────┘ └──────┘ └──────┘ └─────┘
```

---

## Fazit:

Das IR-Pattern ist **eleganter und skalierbarer**, aber dein aktueller Code ist **pragmatischer und direkter**. Beide Ansätze sind valide! 🎯
