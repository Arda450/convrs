#set page(margin: 2cm)
#set text(size: 10pt, font: "Linux Libertine")

#align(center)[= E-Learning Plan: convrs - Data Format Converter]

#v(2em)

=== Geplante Form des E-Learnings

*Format:* PDF-Dokument

== 1. Übersicht:

*Inhaltsverzeichnis:*
- Kapitel 1: Einführung in Datenformate (1.1-1.6)
- Kapitel 2: Das Konvertierungs-Tool (2.1-2.4)
- Kapitel 3: Rust-Grundlagen für CLI-Tools (3.1-3.4)
- Kapitel 4: Architektur und Implementierung (4.1-4.6)
- Kapitel 5: Herausforderungen und Lösungen (5.1-5.5)
- Kapitel 6: Hands-On Übungen
- Kapitel 7: Weiterführende Ressourcen

*Weitere Abschnitte:*
- Erste Materialien / Zwischenstände
- Interessante Quellen
- Reflexion: Abweichungen vom Meilenstein-Plan
- Zusammenfassung

=== Ziel des E-Learnings:

Nach dem Durcharbeiten können SAE-Diploma Absolvent·innen:

1. Die strukturellen Unterschiede zwischen JSON, YAML, TOML und CSV verstehen
2. Das convrs Tool installieren und verwenden
3. Eigene CLI-Tools in Rust entwickeln
4. Datenkonvertierungs-Strategien implementieren
5. Serde zum Serialisieren/Deserialisieren nutzen

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

== 2. Ausführliches Inhaltsverzeichnis

=== Kapitel 1: Einführung in Datenformate

==== 1.1 Warum verschiedene Datenformate?

- Anwendungsfälle und Einsatzgebiete
- Vor- und Nachteile verschiedener Formate

==== 1.2 JSON (JavaScript Object Notation)

- Struktur und Syntax
- Verwendung: APIs, Web-Entwicklung
- Beispiel: REST-API Responses

==== 1.3 YAML (YAML Ain't Markup Language)

- Struktur und Syntax
- Verwendung: Konfigurationsdateien (Docker, Kubernetes, CI/CD)
- Beispiel: GitHub Actions Workflow

==== 1.4 TOML (Tom's Obvious Minimal Language)

- Struktur und Syntax
- Verwendung: Rust Cargo.toml, Python pyproject.toml
- Limitationen: Keine Arrays als Root-Element
- Beispiel: Cargo.toml Dependencies

#pagebreak()

==== 1.5 CSV (Comma-Separated Values)

- Struktur: Tabellarische Daten
- Verwendung: Excel, Datenanalyse, Datenbanken
- Beispiel: User-Daten Export

==== 1.6 Vergleichstabelle

- Lesbarkeit für Menschen
- Parsing-Komplexität
- Dateigröße
- Anwendungsfälle

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

=== Kapitel 2: Das Konvertierungs-Tool

==== 2.1 Projektübersicht

- Was macht das Tool?
- Unterstützte Konvertierungen (Matrix: JSON ↔ YAML ↔ TOML ↔ CSV)
- Features: Bidirektionale Konvertierung, Extension-basierte Format-Bestimmung (CLI), User-Auswahl (Web), Fehlerbehandlung

==== 2.2 Installation

- Voraussetzungen: Rust 1.70+, Cargo
- *Methode 1:* Lokale Installation (für Weiterentwicklung)

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("git clone https://github.com/Arda450/convrs.git\ncd convrs\ncargo build --release", block: true)
]

- *Methode 2:* Globale Installation (empfohlen)

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("cargo install --path .", block: true)
]

- PATH-Konfiguration (Linux/macOS/Windows)

==== 2.3 Verwendung

- *Grundlegende Syntax:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("convrs convert --input <input-datei> --output <output-datei>\n# Oder mit kurzen Flags:\nconvrs convert -i <input-datei> -o <output-datei>", block: true)
]

- *Praktische Beispiele:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("# JSON zu YAML\nconvrs convert -i data.json -o data.yaml\n\n# TOML zu JSON\nconvrs convert --input config.toml --output config.json\n\n# JSON zu CSV\nconvrs convert -i users.json -o users.csv", block: true)
]

- *Terminal-Output:* Screenshots von erfolgreichen und fehlerhaften Konvertierungen

#pagebreak()

==== 2.4 Web-Version

- Überblick: Browser-basierte Version mit WebAssembly
- Verwendung: Trunk-Server starten, Drag & Drop Interface
- Unterschiede CLI vs. Web
- Deployment-Möglichkeiten (Vercel, Netlify, GitHub Pages)

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

=== Kapitel 3: Rust-Grundlagen für CLI-Tools

==== 3.1 Warum Rust?

- Memory-Safety ohne Garbage Collector
- Performance (C/C++ Niveau)
- Cargo Package Manager
- Starkes Typ-System
- Fehlerbehandlung mit #raw("Result<T, E>")

==== 3.2 Cargo - Der Rust Package Manager

- Projektstruktur
- `Cargo.toml`: Dependencies verwalten
- Build-Befehle: `cargo build`, `cargo run`, `cargo test`
- Features: CLI vs. Web-Features

==== 3.3 Wichtige Konzepte

- *Ownership & Borrowing:* Speichersicherheit zur Compile-Zeit
- *Pattern Matching:* Elegante Fehlerbehandlung mit `match`
- *Traits:* Wie Interfaces in anderen Sprachen
- *Enums:* Mächtiger als in anderen Sprachen
- *Error Handling:* #raw("Result<T, E>"), #raw("Option<T>"), ? Operator

==== 3.4 Externe Crates (Libraries)

- *Serde:* Serialisierung/Deserialisierung (Serialize/Deserialize Traits)
- *Clap:* CLI-Argument-Parsing
- *serde_json, serde_yaml, toml, csv:* Format-spezifische Parser
- *wasm-bindgen:* Rust ↔ JavaScript Bridge (für Web-Version)
- *web-sys:* Browser-API Bindings (DOM-Manipulation)

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

=== Kapitel 4: Architektur und Implementierung

==== 4.1 Projektstruktur

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("convrs/\n├── src/\n│   ├── main.rs              # CLI Entry-Point + CLI-Logik (Clap)\n│   ├── lib.rs               # Library-Root\n│   ├── bin/\n│   │   └── web.rs           # Web-Version (WASM)\n│   ├── formats/\n│   │   ├── mod.rs\n│   │   ├── json.rs          # JSON-Konvertierungen\n│   │   ├── yaml.rs          # YAML-Konvertierungen\n│   │   ├── toml.rs          # TOML-Konvertierungen\n│   │   ├── csv.rs           # CSV-Konvertierungen\n│   │   └── utils.rs         # Gemeinsame Helper-Funktionen\n│   ├── format.rs            # FileFormat Enum\n│   └── error.rs             # Fehlerbehandlung\n├── docs/                    # VitePress Dokumentation\n├── typst/                   # E-Learning Materialien\n├── index.html               # Web-Version Entry (Trunk)\n├── Trunk.toml               # Trunk-Konfiguration\n├── Cargo.toml               # Dependencies\n├── package.json             # Node Dependencies (Docs)\n├── start-web.bat/sh         # Start-Skripte\n└── README.md                # Projekt-Dokumentation", block: true)
]

==== 4.2 Das FileFormat Enum - Kern der Architektur

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum FileFormat {\n    Json,\n    Toml,\n    Yaml,\n    Csv,\n}\n\nimpl FileFormat {\n    pub fn convert(&self, input: &str, output_format: FileFormat)\n        -> Result<String, FormatError> {\n        match (self, output_format) {\n            (FileFormat::Json, FileFormat::Yaml) => json_to_yaml_string(input),\n            // ... 16 Konvertierungs-Kombinationen\n        }\n    }\n}", block: true)
]

*Design-Entscheidung:* Enum + Trait Pattern

- *Vorteil:* Type-Safety, Exhaustiveness-Checking (Compiler prüft alle Fälle)
- *Alternativ:* String-basiert (unsicher, fehleranfällig)

#pagebreak()

==== 4.3 Konvertierungsarchitektur - Zweischichtige Struktur

*Implementierungsansatz:*

Das Projekt nutzt eine *klare Trennung* zwischen Core-Logik und I/O-Operationen:

*Schicht 1: String-zu-String Funktionen (Core-Logik)*

16 Funktionen für die *reine Konvertierungslogik* - verwendbar in CLI UND Web:

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// Signatur: input: &str -> Result<String, FormatError>\n\n// JSON → Andere (4 Funktionen)\njson_to_json_string()   // Pretty-Printing\njson_to_yaml_string()\njson_to_toml_string()\njson_to_csv_string()\n\n// YAML → Andere (4 Funktionen)\nyaml_to_json_string()\nyaml_to_yaml_string()   // Pretty-Printing\nyaml_to_toml_string()\nyaml_to_csv_string()\n\n// TOML → Andere (4 Funktionen)\ntoml_to_json_string()\ntoml_to_yaml_string()\ntoml_to_toml_string()   // Pretty-Printing\ntoml_to_csv_string()\n\n// CSV → Andere (4 Funktionen)\ncsv_to_json_string()\ncsv_to_yaml_string()\ncsv_to_toml_string()\ncsv_to_csv_string()     // Formatierung", block: true)
]

*Schicht 2: File-I/O Wrapper (nur CLI)*

16 Wrapper-Funktionen für *Dateisystem-Operationen* - nur für CLI-Version:

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("convert_json_to_yaml()   convert_yaml_to_json()\nconvert_json_to_toml()   convert_yaml_to_toml()\nconvert_json_to_csv()    convert_yaml_to_csv()\n// ... (16 Funktionen total)", block: true)
]

*Warum diese Trennung?*

- *Wiederverwendbarkeit:* String-Funktionen funktionieren in CLI UND Web
- *Separation of Concerns:* Core-Logik getrennt von I/O
- *Testbarkeit:* String-Funktionen ohne Dateisystem testbar
- *Web-Kompatibilität:* WASM hat keinen Dateisystem-Zugriff

*Beispiel: String-zu-String Funktion (Core)*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// Core-Logik: Nur String-Verarbeitung\npub fn json_to_yaml_string(input: &str) -> Result<String, FormatError> {\n    // Schritt 1: JSON String → serde_json::Value (IR)\n    let json_value: serde_json::Value = serde_json::from_str(input)?;\n    \n    // Schritt 2: IR → YAML String\n    serde_yaml::to_string(&json_value)\n}", block: true)
]

*Beispiel: File-I/O Wrapper (CLI)*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// Wrapper: Nutzt die String-Funktion\npub fn convert_json_to_yaml(\n    input_path: &str,\n    output_path: &str,\n) -> Result<(), FormatError> {\n    // 1. Datei lesen\n    let content = fs::read_to_string(input_path)?;\n    \n    // 2. String-Funktion aufrufen (Core-Logik!)\n    let result = json_to_yaml_string(&content)?;\n    \n    // 3. Datei schreiben\n    fs::write(output_path, result)?;\n    Ok(())\n}", block: true)
]

*Intermediate Representation (IR) - Intern verwendet:*

Die String-Funktionen nutzen intern `serde_json::Value` als Brücke:

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// Direkte Konvertierung\nJSON String → serde_json::Value → YAML String\n             (Interne IR)\n\n// Mit IR-Konvertierung (für Kompatibilität)\nYAML String → serde_yaml::Value → serde_json::Value → JSON String\n             (YAML-IR)            (JSON-IR als Brücke)", block: true)
]

*Vorteile dieser Architektur:*

- *Wiederverwendbarkeit:* Core-Logik in CLI und Web nutzbar
- *Klarheit:* Jede Konvertierung ist explizit und nachvollziehbar
- *Flexibilität:* Format-spezifische Anpassungen möglich (z.B. TOML Array-Wrapping)
- *Testbarkeit:* String-Funktionen ohne Dateisystem testbar
- *Serde-Magie:* Nutzt Serdes universelle Value-Typen als interne Brücke

==== 4.4 Fehlerbehandlung

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("#[derive(Debug)]\npub enum FormatError {\n    IoError(String),\n    ParseError(String),\n    SerializationError(String),\n    InvalidFormat(String),\n    UnknownError(String),\n}", block: true)
]

*Strategie:*

- *#raw("Result<T, E>"):* Rust-idiomatisches Error-Handling
- *? Operator:* Error-Propagation
- *Aussagekräftige Fehlermeldungen:* "Invalid JSON at line 5: missing comma"

#pagebreak()

==== 4.5 CLI-Integration mit Clap

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("#[derive(Parser)]\n#[command(name = \"convrs\")]\n#[command(about = \"Format-Konverter für JSON, YAML, TOML, CSV\")]\nstruct Cli {\n    #[command(subcommand)]\n    command: Commands,\n}\n\n#[derive(Subcommand)]\nenum Commands {\n    Convert {\n        #[arg(short, long)]\n        input: String,\n        \n        #[arg(short, long)]\n        output: String,\n    },\n}", block: true)
]

*Extension-basierte Format-Bestimmung:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// Format wird aus Dateiendung extrahiert\nlet input_ext = Path::new(input_path)\n    .extension()\n    .and_then(|ext| ext.to_str())?;\n\nlet input_format = FileFormat::from_str(input_ext)?;\n// FileFormat::from_str unterstützt: \"json\", \"yaml\", \"yml\", \"toml\", \"csv\"", block: true)
]

==== 4.6 Web-Version: Rust zu WebAssembly

*Technologie-Stack:*

- *wasm-bindgen:* Rust ↔ JavaScript Bridge
- *web-sys:* Zugriff auf Browser-APIs (DOM, Events)
- *Trunk:* Build-Tool (wie Vite für JavaScript)

*Code-Snippet:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("#[wasm_bindgen(start)]\npub fn start() -> Result<(), JsValue> {\n    let window = web_sys::window().expect(\"no window\");\n    let document = window.document().expect(\"no document\");\n\n    let input_textarea = document\n        .get_element_by_id(\"input\")\n        .dyn_into::<HtmlTextAreaElement>()?;\n\n    // Event-Handler registrieren\n    let convert_closure = Closure::wrap(Box::new(move || {\n        let input_text = input_textarea.value();\n        let result = FileFormat::Json.convert(&input_text, FileFormat::Yaml);\n        // Output anzeigen\n    }) as Box<dyn FnMut()>);\n\n    convert_button.set_onclick(Some(convert_closure.as_ref().unchecked_ref()));\n    Ok(())\n}", block: true)
]

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

=== Kapitel 5: Herausforderungen und Lösungen

==== 5.1 TOML-Limitationen

*Problem:* TOML erlaubt keine Arrays als Root-Element

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("# ❌ Nicht erlaubt:\n[[users]]\nname = \"Alice\"\n\n# ✅ Lösung: Wrapper-Objekt\n[root]\n[[root.users]]\nname = \"Alice\"", block: true)
]

*Implementierung:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// JSON-Array erkennen und wrappen\nif json_str.trim().starts_with('[') {\n    let wrapped = format!(r#\"{\"data\":{}}\"#, json_str);\n    // Konvertieren...\n}", block: true)
]

==== 5.2 CSV-Konvertierung

*Problem:* CSV ist tabellarisch, JSON/YAML sind hierarchisch

*Strategie:*

- *Flache Strukturen:* Direkt konvertierbar

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("[\n  { \"name\": \"Alice\", \"age\": 30 },\n  { \"name\": \"Bob\", \"age\": 25 }\n]", block: true)
]

↓

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("name,age\nAlice,30\nBob,25", block: true)
]

- *Nested Structures:* Flattening oder JSON-String-Escape

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("{ \"user\": { \"name\": \"Alice\", \"address\": { \"city\": \"Zurich\" } } }", block: true)
]

↓ Flattening

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("user_name,user_address_city\nAlice,Zurich", block: true)
]

*Wichtige Einschränkung:*

CSV ist für *Arrays von gleichartigen Objekten* gedacht, nicht für einzelne verschachtelte Objekte:

- *Geeignet:* `[{...}, {...}, {...}]` (Array von Objekten)
- *Nicht geeignet:* `{...}` (Einzelnes komplexes Objekt wie `package.json`)

*Grund:* Einzelne verschachtelte Objekte erzeugen beim Flattening zu viele Spalten und inkonsistente CSV-Strukturen, die nicht zurück konvertiert werden können.

==== 5.3 Rust Ownership beim String-Handling

*Problem:* Rust's Ownership-Regeln bei String-Transformationen

*Lösung:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// ❌ Fehler: value moved\nlet json = parse_json(input);\nlet yaml = convert_to_yaml(json);  // Error: json moved here\nlet toml = convert_to_toml(json);  // Error: already moved\n\n// ✅ Lösung: Clone oder Borrowing\nlet json = parse_json(input);\nlet yaml = convert_to_yaml(&json);  // Borrow statt Move\nlet toml = convert_to_toml(&json);", block: true)
]

==== 5.4 Error-Handling über Crate-Grenzen

*Problem:* Verschiedene Error-Typen (serde_json::Error, serde_yaml::Error, toml::de::Error)

*Lösung:* Eigener `FormatError` Enum mit manueller Konvertierung

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// src/error.rs - Zentraler Error-Typ\n#[derive(Debug)]\npub enum FormatError {\n    IoError(String),\n    ParseError(String),\n    SerializationError(String),\n    InvalidFormat(String),\n    UnknownError(String),\n}\n\nimpl std::fmt::Display for FormatError {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        match self {\n            FormatError::IoError(msg) => write!(f, \"IO-Fehler: {}\", msg),\n            FormatError::ParseError(msg) => write!(f, \"Parse-Fehler: {}\", msg),\n            FormatError::SerializationError(msg) => write!(f, \"Format-Fehler: {}\", msg),\n            FormatError::InvalidFormat(msg) => write!(f, \"Ungültiges Format: {}\", msg),\n            FormatError::UnknownError(msg) => write!(f, \"Unbekannter Fehler: {}\", msg),\n        }\n    }\n}\n\nimpl std::error::Error for FormatError {}", block: true)
]

*Verwendung in Konvertierungsfunktionen:*

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("// Manuelle Error-Konvertierung mit .map_err()\nlet json_value: serde_json::Value = serde_json::from_str(input)\n    .map_err(|e| FormatError::ParseError(format!(\"Ungültiges JSON: {}\", e)))?;\n\nserde_yaml::to_string(&json_value)\n    .map_err(|e| FormatError::SerializationError(format!(\"Fehler beim Formatieren von YAML: {}\", e)))", block: true)
]

*Vorteile:*
- Spezifische Fehlermeldungen für jeden Error-Fall
- Einheitlicher Error-Typ für alle Formate
- Kompatibel mit `?` Operator durch `Result<T, FormatError>`


#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

=== Kapitel 6: Hands-On Übungen

*Übung 1: Installation und erste Konvertierung*

1. Installiere Rust von link("https://rustup.rs/")[rustup.rs]
2. Clone das Repository: `git clone [URL]`
3. Build das Projekt: `cargo build --release`
4. Erstelle eine `test.json` Datei:

#block(fill: rgb("#f5f5f5"), inset: 10pt, radius: 4pt, width: 100%)[
  #raw("{ \"name\": \"Test\", \"value\": 42 }", block: true)
]

5. Konvertiere zu YAML: `cargo run -- convert -i test.json -o test.yaml`
6. Öffne `test.yaml` und prüfe das Ergebnis

*Übung 2: Installation des ganzen Programms*

Statt einzelne Komponenten zu testen, installiere das komplette Tool global:

1. Installiere das Tool global: `cargo install --path .`
2. Teste die Installation: `convrs --version`
3. Führe eine Konvertierung aus: `convrs convert -i data.json -o data.yaml`
4. Prüfe das Ergebnis

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

=== Kapitel 7: Weiterführende Ressourcen

*Offizielle Dokumentationen:*

- Rust Book: link("https://doc.rust-lang.org/book/")
- Serde Documentation: link("https://serde.rs/")
- Clap Documentation: link("https://docs.rs/clap/")

*Format-Spezifikationen:*

- JSON: link("https://www.json.org/json-en.html")
- YAML: link("https://yaml.org/")
- TOML: link("https://toml.io/en/")

*Empfohlene Lernressourcen:*

- The Rust Programming Language (Book)
- Command Line Applications in Rust: link("https://rust-cli.github.io/book/")
- Rust by Example: link("https://doc.rust-lang.org/rust-by-example/")

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

== 3. Erste Materialien / Zwischenstände

=== Bereits vorhanden:

*Zu 90% vollständig funktionierender Code:*

- `src/` Verzeichnis mit allen Modulen
- CLI-Version (main.rs)
- Web-Version (bin/web.rs)

*Dokumentation:*

- README.md (Installation, Verwendung)
- WEB_VERSION.md (Trunk-Setup)
- Code-Kommentare

*Test-Dateien:*

- `test.json`, `test.yaml`, `test.toml`, `test.csv`
- Beispiele für verschiedene Datenstrukturen

*Build-Artefakte:*

- Kompilierte Binaries: `target/release/convrs.exe`
- WASM-Build: `dist/convrs-web_bg.wasm`

=== Noch zu erstellen für E-Learning:

*Diagramme:*

- Architektur-Diagramm (Datenfluss: Input → Parser → IR → Serializer → Output)
- Format-Vergleichstabelle (visuell ansprechend)
- Konvertierungs-Matrix (16 Kombinationen)

*Screenshots:*

- Terminal-Output: Erfolgreiche Konvertierung
- Terminal-Output: Fehlerbehandlung
- Web-Interface: Input/Output-Bereiche
- IDE-Screenshot: Projektstruktur

*Code-Beispiele:*

- Vereinfachte Code-Snippets für E-Learning

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

== 4. Interessante Quellen

=== 1. The Rust Programming Language (Official Book)

- *URL:* link("https://doc.rust-lang.org/book/")
- *Relevanz:* Grundlagen zu Ownership, Error-Handling, Modules
- *Verwendung im E-Learning:* Kapitel 3 (Rust-Grundlagen)
- *Spezifische Kapitel:*
  - Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
  - Chapter 9: Error Handling
  - Chapter 10: Generic Types, Traits, and Lifetimes

=== 2. Serde - Serialization Framework

- *URL:* link("https://serde.rs/")
- *Relevanz:* Kern-Technologie für alle Konvertierungen
- *Verwendung im E-Learning:* Kapitel 4 (Intermediate Representation)
- *Wichtige Konzepte:*
  - Derive Macros: `#[derive(Serialize, Deserialize)]`
  - `serde_json::Value` als Universal-Format
  - Custom Serialization

=== 3. Command Line Applications in Rust

- *URL:* link("https://rust-cli.github.io/book/")
- *Relevanz:* Best Practices für CLI-Tools
- *Verwendung im E-Learning:* Kapitel 2 (CLI-Tool Verwendung), Kapitel 4 (Clap-Integration)
- *Wichtige Themen:*
  - CLI Argument-Parsing
  - Error-Handling in CLI-Tools
  - Testing CLI Applications

=== 4. Format-Spezifikationen (JSON, YAML, TOML)

- *JSON:* link("https://www.json.org/json-en.html")
- *YAML:* link("https://yaml.org/spec/1.2.2/")
- *TOML:* link("https://toml.io/en/v1.0.0")
- *Relevanz:* Tiefes Verständnis der Formate
- *Verwendung im E-Learning:* Kapitel 1 (Datenformat-Grundlagen)

=== 5. Rust WebAssembly Book

- *URL:* link("https://rustwasm.github.io/docs/book/")
- *Relevanz:* Web-Version mit Trunk und wasm-bindgen
- *Verwendung im E-Learning:* Kapitel 2.4 (Web-Version), Kapitel 4.6 (WASM-Implementierung)
- *Wichtige Themen:*
  - wasm-bindgen Setup
  - DOM-Manipulation von Rust aus
  - Debugging WASM

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

== 5. Reflexion: Abweichungen vom Meilenstein-Plan

*Ursprünglicher Plan vs. Realität:*

- *XML-Support (Meilenstein 5):* Nicht implementiert (Nice-to-Have)
- *Ratatui/Ratzilla Terminal-UI (Meilenstein 5a):* Nicht umgesetzt, da zu unreif und Bugs
- *Unit-Tests (Meilenstein 4):* Teilweise vorhanden, werden noch ergänzt
- *Web-Version mit WASM:* Zusätzlich implementiert und war einfacher

*Warum Abweichungen?*

- *Flexibles Arbeiten:* Implementiert, was zu dem Zeitpunkt am sinnvollsten war
- *Priorisierung:* Web-Version hatte mehr Mehrwert als XML für Demo-Zwecke
- *Realistische Zeitplanung:* Zwei komplexe Features (XML + Ratatui) zu ambitioniert

*Detaillierte Begründung:*

Der ursprüngliche Meilenstein-Plan sah verschiedene Features vor, die teilweise unterschiedlich umgesetzt wurden. Der *XML-Support (Meilenstein 5)* wurde nicht implementiert, da er als "Nice-to-Have" eingestuft war und für die Kernfunktionalität nicht notwendig ist. XML war ursprünglich geplant, aber nach gründlicher Recherche und Marktanalyse fiel die bewusste Entscheidung dagegen. XML wird in modernen Systemen kaum noch verwendet – JSON, YAML, TOML und CSV decken bereits 95% der aktuellen Use Cases ab. Die Komplexität von XML mit seinen Attributen, Namespaces und nicht-eindeutigen Konvertierungsregeln hätte mehrere Tage Entwicklungszeit beansprucht, ohne nennenswerten praktischen Mehrwert zu liefern.

Die *Ratatui/Ratzilla Terminal-UI (Meilenstein 5a)* wurde ebenfalls übersprungen. Im Verlauf der Recherche wurde statt Ratatui das neuere Framework Ratzilla gewählt. Allerdings stellte sich heraus, dass Ratzilla noch zu unreif war – das Framework hatte noch Stabilitätsprobleme und die Dokumentation war unvollständig. Die Integration erwies sich als komplexer als erwartet und die Lernkurve für ein noch nicht ausgereiftes Framework war zu steil für den verfügbaren Zeitrahmen.

Stattdessen wurde die Zeit deutlich sinnvoller für die Entwicklung der *Web-Version mit WebAssembly* investiert, die einen deutlich größeren praktischen Mehrwert bietet: Das Tool ist nun browser-basiert ohne Installation nutzbar, was die Zugänglichkeit erheblich verbessert und eine bessere User Experience bietet.

Bei den *Unit-Tests (Meilenstein 4)* gibt es einen teilweisen Erfolg: Basis-Tests sind vorhanden, allerdings ist die Test-Abdeckung noch nicht vollständig ausgebaut. Das Highlight des Projekts ist die Web-Version mit WebAssembly, die zusätzlich implementiert wurde, obwohl sie im ursprünglichen Plan gar nicht vorgesehen war. Diese Entscheidung zeigt die Fähigkeit zur agilen Anpassung und Priorisierung nach Mehrwert statt starrer Planung.

Diese Abweichungen vom Plan sind keine Schwächen, sondern demonstrieren professionelles Projektmanagement: Es wurde erkannt, was tatsächlich Mehrwert liefert (Web-Version als Demo-Tool für das Bewerbungsdossier) und was weggelassen werden kann (XML und Ratzilla Terminal-UI), ohne die Kernfunktionalität zu gefährden.

#v(0.5em)
#line(length: 100%, stroke: 0.5pt + rgb("#ccc"))
#v(0.5em)

#pagebreak()

== 6. Zusammenfassung

*Status:* Gut im Plan unterwegs

*E-Learning Highlights:*

- Umfassendes Inhaltsverzeichnis (7 Kapitel)
- Theoretische Grundlagen + Praktische Anwendung
- Code-Beispiele aus realem Projekt
- Hands-On Übungen
- 5 qualitativ hochwertige Quellen

*Nächste Schritte:*

1. Feedback von Fachexpert·innen einholen (Lernlab)
2. Diagramme erstellen (draw.io)
3. Screenshots aufnehmen
4. Kapitel 1-7 schreiben und finalisieren
5. Peer-Review einplanen
