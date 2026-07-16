# Notewise — GNOME Note Editor (Arbeitstitel)

Ein unendlicher Canvas-Notizeditor für GNOME (GTK4 + libadwaita), der Word-Android-Features,
OneNote-Stift/Touch-Bedienung und LaTeX/Plot-Support vereint.

## Tech-Stack (entschieden)

- **Sprache:** Rust (starkes Typsystem, gtk4-rs/libadwaita-rs reif, gute Performance für Canvas/Stift)
- **UI:** GTK 4.22 + libadwaita 1.9 (modernes GNOME-Look, HIG-konform)
- **Build:** Cargo (mit `gio` resource bundling) — Meson nur optional für System-Installation später
- **Canvas:** Eigenes `gtk4::Widget` mit `snapshot`-Drawing (GSK), da unendliche Canvas + Stift+Text frei
  positioniert gelöst sein will. GtkTextView/etc. sind seiten-/zeilenbasiert und nicht geeignet.
- **Stift/Input:** GTK4 Event-Controller `GestureStylus`, `GestureZoom`, `GestureDrag`, `EventControllerMotion`
- **Math/Plot:** `plotters`-Crate (reines Rust, PDF/SVG/Pixbuf) für Funktionsgraphen
- **LaTeX-Rendering:** `katex`-Crate (Rust-Binding) ODER externes `katex`/`mathjax` via WebKit-WebView
  — Entscheidung offen; erst mal KaTeX-rc (offline) versuchen.
- **Speicherformat:** Pro Notiz ein Verzeichnis `~/Documents/Notewise/<titel>/` mit `note.json`
  (Canvas-Objekte), Media-Dateien, `thumb.png` Vorschau.
- **Drucken:** GTK `PrintOperation` + Cairo-PDF-Backend; Invertierung bei schwarzem Blatt in Druck-Pipeline.

### Warum Rust statt C/Vala
gtk4-rs ist offiziell unterstützt, libadwaita-rs aktuell, Memory-Safety, hervorragendes Crate-Ökosystem
(plotters, serde, image, cairo-rs). C wäre mühsamer, Vala hat kleinere Community für neue Widgets.

### Warum eigener Canvas statt GtkTextView
OneNote-artige Frei-Positionierung von Textblöcken, Stiftstrichen, Bildern, Tabellen auf **einer**
unendlichen Ebene ist mit GtkTextView nicht abbildbar. Wir brauchen ein eigenes Canvas-Widget, das
verschiedene "Items" (Text-Block, Stroke, Table, Image, …) verwaltet und via GSK rendert.

---

## Todo-Liste (wichtigste → unwichtigste Features)

Reihenfolge = Implementierungsreihenfolge. Nach **jeder** Phase Test durch Benutzer, dann weiter.

### Phase 1 — Fundament (jetzt)
1. **Projektgerüst**: Cargo-Projekt, libadwaita `Application` + `AdwApplicationWindow` mit Headerbar,
   "Neue Notiz"-Button, leerem Canvas-Widget. Läuft und sieht aus wie eine GNOME-App.

### Phase 2 — Canvas & Text
2. **Unendlicher Canvas**: Pan (Mittelfinger/Drag), Zoom (Ctrl+Scroll / Pinch später), Koordinaten-
   Transformation World↔Screen. Hintergrund standardmäßig weiß.
3. **Text-Tool**: Klick → Textcursor an World-Pos, Tippen schreibt. Rich-Text:
   - **Fett / Kursiv / Unterstrichen**
   - **Schriftart** (System-Schriften via Pango)
   - **Schriftgröße** (Auswahl + Rad)
   - **Farbe** (ColorChooser)
   - Absätze, **Textausrichtung** links/rechts/mitig
   Implementation: Pango-Attributed-Text pro Text-Item, gerendert via `PangoLayout` im snapshot.

### Phase 3 — Listen & Tabellen
4. **Listen**: Pro Zeile = Absatz. Aufzählungszeichen • , numeriert 1. 2. 3., **Checkbox** anklickbar
   (Toggle-Status im Item). Toolbar-Umschalter.
5. **Tabellen**: Raster aus Zellen, jede Zelle = Mini-Text-Item. Zeilen/Spalten add/remove, Zelle
   aktivieren durch Klick. Erst einfache Version (kein Zellverbund).

### Phase 4 — Stift
6. **Stiftwerkzeug** (Grafiktablet):
   - Freihandstriche als Polyline-Item, Pressure → Strichbreite (via `GestureStylus::axis`).
   - **Farben**: Palette + zuletzt benutzte Farben speichern.
   - **Marker**: Halbtransparenter breiter Strich.
   - **Radierer = ganzer Strich**: Trifft Radierer einen Strich, wird das gesamte Strich-Item
     (vom Setdown bis Liftoff) gelöscht — wie Samsung Notes.
   - **Stylus-Sekundärknopf → Radierer**: Beim Drücken des secondary button wechselt das Werkzeug
     temporär auf Radierer.

### Phase 5 — Touch & Speichern
7. **Touch-Gesten**: Pinch-to-Zoom (zwei Finger), Zwei-Finger-Pan, Werkzeug-Palette per Touch erreichbar.
   Schreibstift hebt Touch ab (GTK4 `event_is_emulated` / `tool` prüfen).
8. **Speichern / Ordner / Vorschau**:
   - Speicherort `~/Documents/Notewise/`.
   - **Titel-Eingabe** oben in der Headerbar (editable `AdwEntryRow`-ähnlich oder `GtkEditableLabel`);
     leer → Default-Titel = aktuelles Datum+Uhrzeit (`2026-07-16 14-32`).
   - **Ordner** anlegbar; Notizen/Ordner in Listenansicht (Start-Screen).
   - **Vorschau**: Thumbnail-PNG des Canvas (skaliert) als `thumb.png` gespeichert.

### Phase 6 — Hintergrund & Medien
9. **Hintergrundmuster**: Karriert / Liniert / Blanko / Schwarz. Mustergröße wählbar.
   - Bei **schwarzem Blatt**: Text-/Strichfarben invertieren (weiß/dunkel → schwarz/hell) für
     Bildschirmansicht; Farben an Hintergrund anpassen (wie Samsung Notes).
   - **Drucken**: schwarzes Blatt → weißes Blatt mit schwarzer Schrift (Invertierung nur in Druck-
     Pipeline, Original bleibt schwarz).
10. **Medien einfügen**: Bilder (PNG/JPG), PDF-Seiten (via `poppler-rs` → Pixbuf), Website-Link
    mit Vorschau-Cards (URL + OG-Image Fetch, offline = nur URL-Text).

### Phase 7 — LaTeX & Plot
11. **LaTeX**: Inline-LaTeX in Text-Item via `$...$` oder eigener "Formel-Item". Rendering mit KaTeX
    (offline-rc) → SVG → Pixbuf. Vorerst keine MathJax-WebView, um Abhängigkeiten klein zu halten.
12. **Plot**: Rechtsklick/Menu auf Formel-Item → "Als Graph darstellen". Dialog: Intervall X/Y,
    Schrittweite, Stil (Linie/Punkte). Erzeugt `plotters`-PNG → Bild-Item auf Canvas.

### Phase 8 — Extras
13. **OpenWebUI-Integration**: Verbindung zu lokaler OpenWebUI-Instanz, ausgewählten Text/Zeichnung
    als Prompt senden, Antwort als Text-Item einfügen. Konfiguration in Preferences (URL+API-Key).
14. **Symbole/Pfeile**: Bibliothek einfügbarer Symbole (Pfeile in diverse Richtungen, Formen) als
    SVG-Items, skalierbar/drehbar.

---

## Wichtige Regeln / Designentscheidungen

- **HIG-konform** (https://developer.gnome.org/hig/): Headerbar mit primärer Aktion rechts,
  destructive-action Style für Löschen, `AdwToastOverlay` für Feedback, Dunkel-/Hell-Theme folgen
  automatisch (`color-scheme`).
- **Kein Word-Klon**: bewusst reduziertes Feature-Set, das Ausreichend zum Hefteintrag ist.
- **Performance**: Canvas mit Tausenden Items → Culling (nur sichtbare Items rendern), Dirty-Regions
  beim Repaint, Strokes als GSK `path_node` statt viele Linien-Drawcalls.
- **Dateiformat versioniert** (`"format": 1`), damit Migration möglich ist.
- **Offline-first**: Keine Cloud; alles lokal. OpenWebUI optional.
- **Benutzereingriff nur wenn nötig**: Wenn eine Bibliothek fehlt, **fragen** statt Workaround.

## Entwicklungsumgebung

Vorhanden (überprüft):
- GTK 4.22.4, libadwaita 1.9.2, graphene, Meson, Ninja, Rust + Cargo, Python3, Vala.

Vermutlich nachzuinstallieren (frage ich bei Bedarf):
- `poppler-glib` (PDF) → poppler-rs crate
- `webkit6gtk` falls wir WebKit für LaTeX/Websitelinks brauchen
- `gtksourceview-5` falls Code-Highlight in Formeln gewünscht

## Nächster Schritt

Ich beginne jetzt mit **Phase 1**: Cargo-Projekt + lauffähige libadwaita-App mit leerem Canvas.
Danach kannst du testen, ob die App startet und GNOME-like aussieht.
