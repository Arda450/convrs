# Formative Prüfung zum Advanced Specialised Project (ASP)

## Lernlab zur Evaluation des Lernens und zur laufenden Dokumentation

**Projekt:** CLI Data Converter Tool in Rust  
**Datum:** [Aktuelles Datum einfügen]  
**Student:** Arda Karadavut

---

## Aufgabe 1: Lernprozess

### Ist dein Lernen motivierend?

**Ja, mit Schwankungen.**

**Positive Faktoren:**

- Praktische Relevanz für Karriere
- Sichtbare Fortschritte (JSON → TOML, YAML, CSV funktionieren)
- Technische Herausforderung (Rust Ownership, Serde)
- KI als Lernhilfe für komplexe Konzepte

**Herausforderungen:**

- Zu viel Zeit am Handy während der Arbeit → **Lösung:** Handy in anderem Raum während Arbeitszeit
- Später Tagesstart → **Lösung:** Wecker früher, Morgenroutine ab 8:00 Uhr
- Rust-Komplexität überwältigend → **Lösung:** Mehr Zeit für Grundlagen, iterative Schritte

**Maßnahmen:**

- Handy weglegen (9:00-12:00 und 14:00-17:00)
- Früherer Start (8:00 Uhr statt 10:00 Uhr)
- Tägliche To-Do-Liste (max. 3 Aufgaben)

### Führt deine aktuelle Vorgehensweise zu neuen Erkenntnissen?

**Ja, aber mit Verbesserungspotenzial.**

**Was funktioniert:**

- Iteratives Vorgehen (einfach → komplex)
- Praktisches Experimentieren (Code schreiben, sofort testen)
- KI-gestütztes Lernen (schnellere Erklärungen)
- Strukturierte Projektorganisation (Module)

**Was verbessert werden kann:**

- Dokumentation während des Codens → **Lösung:** 10 Min. Reflexion nach jeder Implementierung
- Tiefere Verständnis-Reflexion → **Lösung:** Code-Review mit "Warum?"-Fragen
- Systematischeres Testen → **Lösung:** Unit-Tests schreiben
- Mehr Zeit für Grundlagen → **Lösung:** 20% der Zeit für Dokumentationsstudium

**Verbesserungen:**

- 10 Min. Reflexion nach jeder Implementierung
- Unit-Tests für jede Konvertierung
- 20% Zeit für Dokumentationsstudium

---

## Aufgabe 2: Dokumentation der (Zwischen-)Ergebnisse

### In welcher Form hältst du Ergebnisse fest? Ist diese Form zweckdienlich?

**Aktuelle Formen:**

- Code-Kommentare (teilweise vorhanden)
- Git-Versionierung
- Lerntagebuch (sporadisch)
- Test-Dateien (nicht dokumentiert)

**Bewertung:** Teilweise zweckdienlich, aber Verbesserungspotenzial.

✅ **Gut:** Code-Kommentare, Git-Versionierung, Test-Dateien als Beispiele  
❌ **Fehlt:** README.md, technische Dokumentation, Architektur-Erklärungen, Herausforderungen/Lösungen dokumentiert

**Verbesserungen:**

- README.md (Installation, Verwendung, Beispiele)
- Technische Dokumentation (Design-Entscheidungen, TOML-Array-Problem)
- Systematischeres Lerntagebuch (wöchentlich)

### Sind deine Ergebnisse einfach und nachvollziehbar abrufbar?

**Teilweise.**

✅ **Gut:** Klare Projektstruktur (`src/formats/json.rs`, etc.), Git-Versionierung  
❌ **Fehlt:** README.md, Quellen-Dokumentation, Architektur-Dokumentation

**Verbesserungen:**

- README.md erstellen
- Quellenverzeichnis aktualisieren
- Architektur-Dokumentation (warum Serde statt Polars?)

### Lassen sich die bisherigen Ergebnisse leicht in eine Präsentation übertragen?

**Teilweise.**

✅ **Präsentierbar:** Live-Demos, Test-Dateien, Code-Struktur  
❌ **Fehlt:** Diagramme, Metriken, dokumentierte Herausforderungen/Lösungen

**Verbesserungen:**

- Architektur-Diagramm (draw.io)
- Datenfluss-Diagramm (JSON → Serde → TOML/YAML/CSV)
- Metriken sammeln (4 Formate, Zeilen Code)
- Screenshots von Terminal-Outputs

---

## Aufgabe 3: Lernfortschritt

### Bist du im Zeitrahmen?

**Ja, mit leichten Verzögerungen bei Dokumentation.**

**Stand (5.12.2025):**

- ✅ Phase 1: Grundlagen & Setup (Woche 1-4) - **Abgeschlossen**
- ✅ Meilenstein 1-3: JSON, YAML, TOML Support - **Abgeschlossen**
- ⚠️ Meilenstein 4: Testing - **Teilweise** (Pretty-Printing ✅, Tests fehlen)
- ⏳ Meilenstein 5: XML Support (Woche 9-10) - **Noch nicht begonnen**
- ⏳ Meilenstein 6: Dokumentation (Woche 8-9) - **Hinkt hinterher**

**Fortschritt:**

- 21.11: minigrep Projekt (Rust Book) ✅
- 23.11: Serde-Dokumentation ✅
- 27.11: JSON → TOML ✅
- 5.12: JSON → CSV, CLAP ✅

**Status:** 1-2 Wochen vor Plan bei Kern-Features, Dokumentation hinkt hinterher.

**Anpassungen:**

- Mehr Zeit für Unit-Tests vor XML
- Dokumentation parallel zur Entwicklung
- XML-Proof-of-Concept frühzeitig testen

### Sind die bisherigen Ergebnisse zufriedenstellend?

**Ja, mit Verbesserungspotenzial.**

✅ **Gut:** 4 funktionale Konvertierungen, robuste Fehlerbehandlung, CLAP-Integration, saubere Code-Struktur  
⚠️ **Fehlt:** Automatisierte Tests, Edge-Case-Tests, umfassende Dokumentation, bidirektionale Konvertierung

**Finales Ergebnis gefährdet?** Nein. Kern-Funktionalität implementiert, im Zeitrahmen, XML ist "Nice-to-Have".

### Was kannst du zum nächsten Meilenstein vorzeigen?

**Aktuell vorzeigbar:**

- Funktionierendes CLI-Tool (`cargo run -- test.json output.toml`)
- 4 Konvertierungen: JSON → JSON/TOML/YAML/CSV
- CLAP-Integration mit automatischer Format-Erkennung
- Robuste Fehlerbehandlung
- Modulare Code-Struktur

**Für nächsten Meilenstein (XML Support):**

- XML-Support (JSON → XML)
- Bidirektionale Konvertierung (TOML → JSON, etc.)
- Format-Detector
- Unit-Tests für alle Konvertierungen

### Wie entwickeln sich deine Hard Skills?

**Rust-Programmierung:**

- ✅ Praktische Erfahrung mit Projektstruktur, Serde, Error-Handling, externen Crates
- ⚠️ Ownership-Regeln und Pattern Matching noch nicht perfekt
- **Fortschritt:** Kann Projekte strukturieren, Dependencies verwalten, Fehlerbehandlung implementieren

**Datenformat-Verständnis:**

- ✅ Strukturelle Unterschiede zwischen JSON/YAML/TOML/CSV verstanden
- ✅ TOML-Limitationen (kein Array als Root), CSV für tabellarische Daten, Serde als IR
- **Fortschritt:** Kann Strukturen analysieren, konvertieren, CSV-Header generieren

**CLI-Entwicklung:**

- ✅ CLAP-Erfahrung, CLI-Best-Practices, Terminal-Output
- **Fortschritt:** Kann benutzerfreundliche CLI-Tools erstellen

**Software-Architektur:**

- ✅ Projektstruktur, Modularität, Trennung von Concerns
- **Fortschritt:** Kann Projekte in Module aufteilen, wiederverwendbaren Code schreiben

**Lernbedarf:** Testing (Unit-Tests), XML-Parsing, Performance-Optimierung, bidirektionale Konvertierung

### Wie entwickeln sich deine Soft Skills?

**Problemlösungsfähigkeit:**

- ✅ Systematisches Vorgehen, KI als Lernhilfe, iteratives Arbeiten
- ⚠️ Noch zu schnell zur Implementierung ohne Grundlagen

**Selbstorganisation:**

- ✅ Klare Projektstruktur, iterative Entwicklung
- ⚠️ Dokumentation hinkt hinterher, Zeitmanagement (zu viel Handy)

**Lernfähigkeit:**

- ✅ Langsames Lernen akzeptiert, KI-Nutzung, praktisches Lernen
- ⚠️ Zu wenig Reflexion über Gelerntes

**Kommunikation:**

- ✅ Code-Kommentare, strukturierte Problembeschreibung
- ⚠️ Noch keine Code-Reviews/Präsentationen

**Selbstreflexion:**

- ✅ Bewusstsein über Stärken/Schwächen, konkrete Maßnahmen identifiziert
- ⚠️ Noch nicht systematisch genug

---

## Reflexions- und Feedback-Elemente

### Process

**Zwischenergebnisse:** 8/10

- ✅ Funktionierendes CLI-Tool (4 Konvertierungen), robuste Fehlerbehandlung, saubere Struktur, CLAP-Integration
- ⚠️ Keine automatisierten Tests, Dokumentation hinkt hinterher, bidirektionale Konvertierung fehlt

**Konsequenzen aus Reflexionen:**

- ✅ Zeitmanagement, Dokumentations-Lücken, Testing-Bedarf identifiziert
- ⚠️ Maßnahmen identifiziert, aber noch nicht vollständig umgesetzt
- **Nächste Schritte:** Handy weglegen, früherer Start, wöchentliche Lerntagebuch-Einträge, README.md, Unit-Tests

**Umgang mit Anregungen/Kritik:**

- ✅ Offenheit für Feedback, Bereitschaft zu refactoren, Lernbereitschaft
- ⚠️ Noch keine Peer-Reviews, Feedback nicht systematisch dokumentiert
- **Nächste Schritte:** Feedback von Fachexperten einholen, Code-Reviews üben

**Selbstkritische Reflexion:** 7/10

- ✅ Zeitmanagement, Dokumentation, Testing, Verständnis, Reflexion identifiziert
- **Maßnahmen:** Handy weglegen, früherer Start (8:00 Uhr), wöchentliche Einträge, 10 Min. Reflexion nach Implementierung, 20% Zeit für Dokumentation, Unit-Tests

---

## Zusammenfassung

**Lernprozess:** Motivation vorhanden, Vorgehensweise führt zu Erkenntnissen, Verbesserungsmaßnahmen identifiziert  
**Dokumentation:** Code-Kommentare vorhanden, README.md fehlt, Quellen nicht systematisch dokumentiert  
**Lernfortschritt:** Im Zeitrahmen, Ergebnisse zufriedenstellend, Finales Ergebnis nicht gefährdet, Fortschritte in Hard/Soft Skills

**Nächste Schritte:**

- **Diese Woche:** README.md, Handy weglegen, früherer Start, Lerntagebuch-Eintrag
- **Nächste 2 Wochen:** Unit-Tests, Architektur-Dokumentation, XML-Proof-of-Concept, Quellenverzeichnis
- **Bis XML-Meilenstein:** XML-Support, bidirektionale Konvertierung, Format-Detector, Präsentationsmaterial

---

**Datum:** [Aktuelles Datum einfügen]  
**Unterschrift:** Arda Karadavut
