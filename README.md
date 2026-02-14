# convrs - Data Format Converter

Ein leistungsstarkes Tool für bidirektionale Konvertierung zwischen verschiedenen Datenformaten (JSON, YAML, TOML, CSV).

Verfügbar als **CLI-Tool** und **Web-Version** (WASM).

## Features

- Bidirektionale Konvertierung zwischen JSON, YAML, TOML und CSV
- Schnelle Verarbeitung mit Rust
- Robuste Fehlerbehandlung
- CLI-Version mit `clap` für Terminal-Nutzung
- Web-Version mit WebAssembly für Browser-Nutzung
- Modulare Architektur

## Voraussetzungen

- **Rust** (mindestens 1.85.0, Edition 2024) – [rustup.rs](https://rustup.rs/)
- **Web-Version zusätzlich:** [Trunk](https://trunkrs.dev/) – wird per `cargo install trunk` installiert (kein separater Download)

---

## Installation & Verwendung

### CLI-Version

#### Installation (kurz)

```bash
git clone https://github.com/Arda450/convrs.git
cd convrs

# Global installieren (empfohlen)
cargo install --path crates/convrs-cli

# Oder lokal bauen
cargo build --release -p convrs-cli
# Binary: target/release/convrs (bzw. convrs.exe)
```

Falls `convrs` nicht gefunden wird: `~/.cargo/bin` muss im PATH sein (Linux/macOS: `export PATH="$HOME/.cargo/bin:$PATH"`).

#### Verwendung

**Grundlegende Syntax:**

```bash
convrs convert --input <input-datei> --output <output-datei>
# Oder mit kurzen Flags:
convrs convert -i <input-datei> -o <output-datei>
```

**Beispiele:**

```bash
# JSON zu YAML
convrs convert -i data.json -o data.yaml

# JSON zu TOML
convrs convert -i config.json -o config.toml

# YAML zu JSON
convrs convert -i data.yaml -o data.json

# JSON zu CSV
convrs convert -i users.json -o users.csv

# TOML zu YAML
convrs convert -i config.toml -o config.yaml
```

---

### Web-Version (WebAssembly)

Die Web-Version läuft im Browser. Zum Bauen und Starten wird Trunk benötigt (über Cargo installieren, kein separater Download):

```bash
cargo install trunk
trunk serve
```

Der Server läuft dann lokal auf `http://127.0.0.1:8080`.

- Browser öffnen auf `http://127.0.0.1:8080` (oder mit `trunk serve --open` automatisch)
- Stoppen: `Ctrl+C` im Terminal

#### Troubleshooting

**Problem: "Port 8080 already in use"**

```bash
# Windows
taskkill /F /IM trunk.exe

# Linux/Mac
pkill trunk
```

**Problem: "manifest path does not exist"**

→ Stelle sicher, dass `Trunk.toml` korrekt konfiguriert ist

---

## CLI vs Web - Vergleich

| Aspekt           | CLI-Version                    | Web-Version                  |
| ---------------- | ------------------------------ | ---------------------------- |
| **Starten**      | `convrs convert -i ... -o ...` | `trunk serve`                |
| **Binary**       | `convrs.exe`                   | WASM (läuft im Browser)      |
| **Input**        | Datei-Pfade                    | File-Upload / Input-Field    |
| **Output**       | Datei auf Festplatte           | Download / Copy-to-Clipboard |
| **Use-Case**     | Scripts, Automation            | Nicht-technische User, Demo  |
| **Installation** | `cargo install`                | Trunk + `trunk serve`        |

---

## Projektstruktur

```
convrs/
├── crates/
│   ├── convrs-cli/          # CLI Binary (Clap, File-I/O)
│   │   ├── src/main.rs, lib.rs
│   │   └── tests/cli.rs
│   ├── convrs-core/         # Shared Library (Konvertierungslogik)
│   │   ├── src/format.rs, error.rs, formats/
│   │   └── tests/conversion.rs
│   └── convrs-web/          # WebAssembly Binary
│       └── src/main.rs
├── index.html               # Web-Version Entry (Trunk)
├── Trunk.toml               # Trunk-Konfiguration
├── Cargo.toml               # Workspace-Root
└── README.md
```

---

## Unterstützte Formate

| Von → Nach | JSON | YAML | TOML | CSV |
| ---------- | ---- | ---- | ---- | --- |
| **JSON**   | ✅   | ✅   | ✅   | ✅  |
| **YAML**   | ✅   | ✅   | ✅   | ✅  |
| **TOML**   | ✅   | ✅   | ✅   | ✅  |
| **CSV**    | ✅   | ✅   | ✅   | ✅  |

---

## Entwicklung

```bash
# CLI bauen
cargo build -p convrs-cli

# CLI bauen (Release)
cargo build --release -p convrs-cli

# Tests ausführen
cargo test

# Code formatieren
cargo fmt

# Linting
cargo clippy
```

### Binary-Location nach Build

- **Debug:** `target/debug/convrs` oder `target/debug/convrs.exe` (Windows)
- **Release:** `target/release/convrs` oder `target/release/convrs.exe` (Windows)

---

## Beitragen

1. Fork das Repository
2. Erstellen Sie einen Feature-Branch (`git checkout -b feature/AmazingFeature`)
3. Commit Ihre Änderungen (`git commit -m 'Add some AmazingFeature'`)
4. Push zum Branch (`git push origin feature/AmazingFeature`)
5. Öffnen Sie einen Pull Request

---

## Lizenz

Dieses Projekt ist lizenziert unter MIT oder Apache-2.0 - siehe [LICENSE](LICENSE) Datei für Details.

---

## Autor

Arda Karadavut - [@Arda450](https://github.com/Arda450)
