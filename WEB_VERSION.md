# 🌐 ASP CLI - Web-Version

## 📋 Was ist Trunk?

**Trunk** ist ein Build-Tool für Rust WebAssembly (WASM) Anwendungen.

### Was Trunk macht:

1. **Kompiliert Rust zu WASM** → Dein Rust-Code läuft im Browser
2. **Bindet WASM an HTML** → Fügt automatisch `<script>`-Tags ein
3. **Startet Dev-Server** → Mit Live-Reload (wie `npm run dev`)
4. **Optimiert für Production** → Minifizierung, Kompression

**Analogie:** Trunk ist für Rust-WASM, was **Vite/Webpack** für JavaScript ist! 🚀

---

## 🚀 Server starten

### **Windows (PowerShell oder CMD):**

```cmd
.\start-web.bat
```

Oder manuell:

```cmd
trunk serve
```

### **Git Bash / Linux / Mac:**

```bash
./start-web.sh
```

Oder manuell:

```bash
trunk serve
```

---

## 📂 Projekt-Struktur

```
asp_cli/
├── web/
│   └── index.html          ← 🎯 Haupt-HTML (wird von Trunk benutzt)
├── Trunk.toml              ← Trunk-Konfiguration
├── start-web.bat           ← Windows-Start-Script
├── start-web.sh            ← Linux/Mac-Start-Script
├── src/
│   ├── main.rs             ← CLI-Version (Terminal)
│   ├── bin/
│   │   └── web.rs          ← Web-Version (WASM)
│   ├── formats/            ← Gemeinsame Konvertierungs-Logik
│   └── ui/
│       └── app.rs          ← UI-Logik für Web
└── dist/                   ← Build-Output (nach `trunk build`)
```

---

## 🔧 Wichtige Befehle

| Befehl                  | Beschreibung                               |
| ----------------------- | ------------------------------------------ |
| `trunk serve`           | Startet Dev-Server (http://127.0.0.1:8080) |
| `trunk serve --open`    | Startet Server + öffnet Browser            |
| `trunk build`           | Production-Build (Output: `dist/`)         |
| `trunk build --release` | Optimierter Production-Build               |
| `trunk clean`           | Löscht Build-Artefakte                     |

---

## 🌐 Nach dem Start

1. **Server läuft auf:** `http://127.0.0.1:8080`
2. **Browser öffnet sich automatisch** (wenn `open = true` in `Trunk.toml`)
3. **Live-Reload:** Änderungen werden automatisch neu geladen
4. **Stoppen:** `Ctrl+C` im Terminal

---

## 🐛 Troubleshooting

### Problem: "Port 8080 already in use"

```bash
# Windows
taskkill /F /IM trunk.exe

# Linux/Mac
pkill trunk
```

### Problem: "manifest path does not exist"

→ Stelle sicher, dass `Trunk.toml` auf `target = "web/index.html"` zeigt

### Problem: "main function not found"

→ `src/bin/web.rs` muss sowohl `fn main()` als auch `#[wasm_bindgen(start)]` haben

---

## 📊 CLI vs Web

| Aspekt       | CLI-Version                | Web-Version                  |
| ------------ | -------------------------- | ---------------------------- |
| **Starten**  | `cargo run --features cli` | `trunk serve`                |
| **Binary**   | `asp_cli.exe`              | WASM (läuft im Browser)      |
| **Input**    | Datei-Pfade                | File-Upload / Input-Field    |
| **Output**   | Datei auf Festplatte       | Download / Copy-to-Clipboard |
| **Use-Case** | Scripts, Automation        | Nicht-technische User, Demo  |

---

## 🎯 Deployment (Production)

### 1. Build erstellen:

```bash
trunk build --release
```

### 2. Output liegt in: `dist/`

```
dist/
├── index.html
├── asp_web_bg.wasm
└── asp_web.js
```

### 3. Deployen auf:

- **Vercel:** `vercel deploy dist/`
- **Netlify:** Drag & Drop `dist/` Ordner
- **GitHub Pages:** Push `dist/` zu `gh-pages` Branch
- **Eigener Server:** Kopiere `dist/` Inhalt

---

## 📝 Konfiguration

Siehe `Trunk.toml`:

```toml
[build]
target = "web/index.html"  # HTML-Einstiegspunkt
dist = "dist"              # Output-Verzeichnis

[serve]
port = 8080                # Dev-Server Port
open = true                # Browser automatisch öffnen
```

---

## 🎓 Weitere Infos

- **Trunk Docs:** https://trunkrs.dev/
- **Ratzilla (Terminal-UI):** https://github.com/orhun/ratzilla
- **WASM-Bindgen:** https://rustwasm.github.io/wasm-bindgen/

---

**Viel Erfolg! 🚀**
