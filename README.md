# convrs - Data Format Converter

Ein leistungsstarkes Tool für bidirektionale Konvertierung zwischen verschiedenen Datenformaten (JSON, YAML, TOML, CSV).

Verfügbar als **CLI-Tool** und **Web-Version** (WASM).

## ✨ Features

- 🔄 Bidirektionale Konvertierung zwischen JSON, YAML, TOML und CSV
- ⚡ Schnelle Verarbeitung mit Rust
- 🛡️ Robuste Fehlerbehandlung
- 🎯 CLI-Version mit `clap` für Terminal-Nutzung
- 🌐 Web-Version mit WebAssembly für Browser-Nutzung
- 📦 Modulare Architektur

## 📋 Voraussetzungen

### Für CLI-Version:
- Rust (mindestens Version 1.70.0)
- Cargo

### Für Web-Version (zusätzlich):
- [Trunk](https://trunkrs.dev/) - WASM Build-Tool
  ```bash
  cargo install trunk
  ```

---

## 🚀 Installation & Verwendung

### 📟 CLI-Version

#### Installation

**Option 1: Lokale Installation (Entwicklung)**

```bash
# Repository klonen
git clone https://github.com/Arda450/convrs.git
cd convrs

# Projekt bauen
cargo build --release --features cli

# Tool testen
cargo run --features cli -- convert -i input.json -o output.yaml
```

**Option 2: Globale Installation (Empfohlen)**

```bash
# Repository klonen
git clone https://github.com/Arda450/convrs.git
cd convrs

# Global installieren (Binary wird in ~/.cargo/bin/ installiert)
cargo install --path . --features cli

# Jetzt können Sie 'convrs' von überall aufrufen:
convrs convert -i input.json -o output.toml
```

**Option 3: Direkt von crates.io (Zukünftig)**

```bash
# Sobald auf crates.io veröffentlicht:
cargo install convrs
```

#### PATH-Konfiguration

Nach der Installation muss `~/.cargo/bin/` in Ihrem PATH sein:

**Linux/macOS:**

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Fügen Sie diese Zeile zu `~/.bashrc` oder `~/.zshrc` hinzu.

**Windows:**

- `~/.cargo/bin` ist normalerweise automatisch im PATH
- Falls nicht: Systemsteuerung → System → Erweiterte Systemeinstellungen → Umgebungsvariablen → PATH bearbeiten

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

### 🌐 Web-Version (WebAssembly)

Die Web-Version läuft direkt im Browser ohne Installation!

#### Was ist Trunk?

**Trunk** ist ein Build-Tool für Rust WebAssembly (WASM) Anwendungen.

- **Kompiliert Rust zu WASM** → Dein Rust-Code läuft im Browser
- **Bindet WASM an HTML** → Fügt automatisch `<script>`-Tags ein
- **Startet Dev-Server** → Mit Live-Reload (wie `npm run dev`)
- **Optimiert für Production** → Minifizierung, Kompression

**Analogie:** Trunk ist für Rust-WASM, was **Vite/Webpack** für JavaScript ist! 🚀

#### Server starten

**Windows (PowerShell oder CMD):**

```cmd
.\start-web.bat
```

Oder manuell:

```cmd
trunk serve
```

**Git Bash / Linux / Mac:**

```bash
./start-web.sh
```

Oder manuell:

```bash
trunk serve
```

#### Nach dem Start

1. **Server läuft auf:** `http://127.0.0.1:8080`
2. **Browser öffnet sich automatisch** (wenn `open = true` in `Trunk.toml`)
3. **Live-Reload:** Änderungen werden automatisch neu geladen
4. **Stoppen:** `Ctrl+C` im Terminal

#### Wichtige Trunk-Befehle

| Befehl | Beschreibung |
| ----------------------- | ------------------------------------------ |
| `trunk serve` | Startet Dev-Server (http://127.0.0.1:8080) |
| `trunk serve --open` | Startet Server + öffnet Browser |
| `trunk build` | Production-Build (Output: `dist/`) |
| `trunk build --release` | Optimierter Production-Build |
| `trunk clean` | Löscht Build-Artefakte |

#### Deployment (Production)

**1. Build erstellen:**

```bash
trunk build --release
```

**2. Output liegt in: `dist/`**

```
dist/
├── index.html
├── convrs-web_bg.wasm
└── convrs-web.js
```

**3. Deployen auf:**

- **Vercel:** `vercel deploy dist/`
- **Netlify:** Drag & Drop `dist/` Ordner
- **GitHub Pages:** Push `dist/` zu `gh-pages` Branch
- **Eigener Server:** Kopiere `dist/` Inhalt

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

**Problem: "main function not found"**

→ `src/bin/web.rs` muss sowohl `fn main()` als auch `#[wasm_bindgen(start)]` haben

---

## 📊 CLI vs Web - Vergleich

| Aspekt | CLI-Version | Web-Version |
| ------------ | -------------------------- | ---------------------------- |
| **Starten** | `convrs convert -i ... -o ...` | `trunk serve` |
| **Binary** | `convrs.exe` | WASM (läuft im Browser) |
| **Input** | Datei-Pfade | File-Upload / Input-Field |
| **Output** | Datei auf Festplatte | Download / Copy-to-Clipboard |
| **Use-Case** | Scripts, Automation | Nicht-technische User, Demo |
| **Installation** | `cargo install` | Keine (Browser) |

---

## 📂 Projektstruktur

```
convrs/
├── src/
│   ├── main.rs              # CLI Entry-Point
│   ├── lib.rs               # Library-Root
│   ├── bin/
│   │   └── web.rs           # Web-Version (WASM)
│   ├── cli/
│   │   └── mod.rs           # CLI-Logik (Clap)
│   ├── formats/
│   │   ├── mod.rs
│   │   ├── json.rs          # JSON-Konvertierungen
│   │   ├── yaml.rs          # YAML-Konvertierungen
│   │   ├── toml.rs          # TOML-Konvertierungen
│   │   ├── csv.rs           # CSV-Konvertierungen
│   │   └── utils.rs         # Gemeinsame Helper-Funktionen
│   ├── format.rs            # FileFormat Enum
│   └── error.rs             # Fehlerbehandlung
├── docs/                    # VitePress Dokumentation
├── typst/                   # E-Learning Materialien
├── index.html               # Web-Version Entry (Trunk)
├── Trunk.toml               # Trunk-Konfiguration
├── Cargo.toml               # Dependencies
├── package.json             # Node Dependencies (Docs)
├── start-web.bat/sh         # Start-Skripte
└── README.md
```

---

## 🎯 Unterstützte Formate

| Von → Nach | JSON | YAML | TOML | CSV |
| ---------- | ---- | ---- | ---- | --- |
| **JSON** | ✅ | ✅ | ✅ | ✅ |
| **YAML** | ✅ | ✅ | ✅ | ✅ |
| **TOML** | ✅ | ✅ | ✅ | ✅ |
| **CSV** | ✅ | ✅ | ✅ | ✅ |

---

## 🛠️ Entwicklung

```bash
# Projekt bauen (CLI)
cargo build --features cli

# Projekt bauen (Web)
cargo build --features web --target wasm32-unknown-unknown

# Tests ausführen
cargo test

# Code formatieren
cargo fmt

# Linting
cargo clippy

# Release-Build erstellen
cargo build --release --features cli
```

### Binary-Location nach Build

- **Debug:** `target/debug/convrs` oder `target/debug/convrs.exe` (Windows)
- **Release:** `target/release/convrs` oder `target/release/convrs.exe` (Windows)

---

## 🤝 Beitragen

1. Fork das Repository
2. Erstellen Sie einen Feature-Branch (`git checkout -b feature/AmazingFeature`)
3. Commit Ihre Änderungen (`git commit -m 'Add some AmazingFeature'`)
4. Push zum Branch (`git push origin feature/AmazingFeature`)
5. Öffnen Sie einen Pull Request

---

## 📄 Lizenz

Dieses Projekt ist lizenziert unter MIT oder Apache-2.0 - siehe [LICENSE](LICENSE) Datei für Details.

---

## 👤 Autor

Arda Karadavut - [@Arda450](https://github.com/Arda450)