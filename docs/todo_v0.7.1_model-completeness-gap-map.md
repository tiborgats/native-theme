# v0.7.1 — Model completeness: the gap map

Status: surveyed, cross-checked and followed up (2026-10-27/28, branch `v0.7.1-rc1`, model
at caeddd6b). §6 lists the open work: the scope questions the rationale decides, and what
comes before any value is collected.

The first of v0.7.1's three documents: this one lists what native-theme does not carry,
the rationale (`todo_v0.7.1_model-completeness-rationale.md`) decides how the model
covers each gap, and the specification (`todo_v0.7.1_model-completeness-spec.md`) states
the model changes exactly. The connectors are not in scope: they follow the model once it
changes.

## The rule

native-theme covers **everything a platform draws visibly**. A visible property the
model cannot express is a defect of the model, whatever a connector or a toolkit does with
it. A value no document states is not a reason to leave it out: it is measured on the
platform (§3). No gap is closed by "the toolkit's default".

## 1. Scope

**Platforms.** `platform-facts.md` has a column for macOS (its 26 values inside it), Windows,
KDE, GNOME and Material, and none for iOS. macOS and iOS each have two generations whose
looks differ; each keeps one file, with a column saying where the generations differ.

| Platform | Generations and versions read | Preset |
|---|---|---|
| macOS | 12–15 (Aqua) and 26–27 (Liquid Glass) | `macos-monterey`, `macos-tahoe` |
| Windows | 11 (WinUI 3, Fluent 2; microsoft-ui-xaml 8463f45) | `windows-11` |
| KDE | Plasma 6, Breeze (Qt Widgets, and Kirigami / Qt Quick; read at `master`, to re-pin, §6.2) | `kde-breeze` |
| GNOME | libadwaita 1.10.0, GTK 4.24.1, gnome-shell 51.0 | `adwaita` |
| Material | Material 3 and 3 Expressive (material-web, Compose Material 3) | `material-3` |
| iOS | 13–18 and 26 (Liquid Glass) | `ios-13`; `ios-26` is not yet written |

The community presets (Catppuccin, Dracula, Nord, …) are not platforms: they take their
structure from the model and need no column.

**The model** is everything `docs/property-registry.toml` lists (`[defaults]`,
`[text_scale]`, `[window]`, `[layout]` and 25 widget sections, the 29 of
`platform-facts.md` §2.1–§2.29; the `Font`, `Border`, `IconSizes` and `TextScaleEntry`
structures), the icon sets, and what the platform readers return.

## 2. What counts as visible

For every element a platform draws, each of these is checked against the model:

1. **Parts.** Every sub-element: frame, fill, indicator, glyph, thumb, track, groove,
   handle, separator, header, footer, arrow, close button, badge, caret.
2. **States, and their combinations.** Rest, hover, pressed, focused (keyboard and
   pointer), disabled, checked / selected / on, indeterminate, mixed, open / expanded,
   dragged, drop target, read-only, invalid / error, default (the dialog's default
   button), the key and the non-key window, and each combination a platform draws
   differently (checked + hover, selected + unfocused list, disabled + checked).
3. **Paint.** Fill (solid, gradient, translucent material), border colour and width per
   side, corner radius per corner, inner and outer outline, **shadow** (every layer's
   offset, blur, spread, colour, inset or not), opacity.
4. **Geometry.** Size, minimum and maximum sizes, padding per side, gaps, alignment,
   the size variants a platform offers (macOS control sizes, WinUI density, Material
   density, Breeze's tool-button sizes).
5. **Text and icons.** Font per role (family, size, weight, style, line height, letter
   spacing, features), text colour per state, the glyph or icon and its size and colour.
6. **Motion.** Every animated transition: what changes, its duration, its easing curve
   (or spring), its delay, and what reduced motion changes.
7. **Behaviour that shows.** Timings a user sees: the caret's blink, tooltip delays,
   scrollbar auto-hide, hover-to-open delays, double-click time, drag thresholds.
8. **Accessibility variants.** What high contrast, increased contrast, reduced
   transparency, larger text and reduced motion change in each of the above.

The elements are: the window and its chrome (title bar, its buttons, the window's own
corner radius, border and shadow), the widget sections the model has, **every standard
control a platform has and the model does not** (§4.2), and the properties that cross
widgets (§4.3). The surveys found more than this list names (the pointer, placement,
right-to-left mirroring, colour space, the emoji and fallback fonts); §4.3's capabilities hold all of it.

## 3. Sources and measurement

Each gap names, per platform, where its value comes from, in this order of preference:

1. **Read at run time** — an API or setting the reader can query (`UISettings`,
   `NSColor`, `kdeglobals`, the settings portal, `GetCaretBlinkTime`, …).
2. **Published** — the platform's own source or documentation, cited at a commit or with
   a read date: WinUI 3's XAML theme resources, Breeze's `breezemetrics.h` and its
   C++, libadwaita's SCSS, Material's tokens, Apple's documentation.
3. **Measured** — on the platform, when 1 and 2 give nothing:
   - **macOS and Windows:** the OS probe (`tools/os-probe`, CI workflow
     `os-probe.yml`) runs on each OS version; on macOS it renders controls through
     AppKit and reads their metrics, on Windows it only queries settings so far; it
     gains a render or a query per gap;
   - **KDE:** on the maintainer's Plasma 6 desktop, a Qt Widgets and a Kirigami probe
     app, captured with Spectacle or KWin's screenshot interface;
   - **GNOME:** a GTK 4 / libadwaita probe app; libadwaita draws its own stylesheet on
     any desktop, so a GNOME session is needed only for what GNOME Shell draws;
   - **Material:** material-web in a headless browser (computed styles, element
     screenshots) and Compose Material 3 on an Android emulator;
   - **iOS:** a UIKit probe app in the Xcode simulator on the macOS CI runner;
   - **animations:** a screen recording at the display's refresh rate, or the
     toolkit's own animation trace, from which the duration and curve are fitted;
     where the platform's source states the curve, the recording confirms it.

The map records which of the three each value will come from; the values themselves are
found when the specification's plan implements them, never before, and never guessed.

## 4. The gaps

Every step ran per platform, against `docs/property-registry.toml` at caeddd6b:

- **pass 1** surveyed each platform on its own;
- **pass 2** checked each platform against what the other five found: every control another
  survey listed, every capability, and every "(none)" cell of the platform's column in
  `platform-facts.md` (for iOS, which has no column there, every `ios-13` preset value with
  no iOS source);
- **the follow-up** wrote rows for the parts of the seed groups no row covered (§5), asked
  the other platforms about 18 controls that pass 2 had put under a wrong control name,
  and, from a check of this map's completeness, wrote rows for the emoji font and the
  fallback chain for scripts the UI font lacks (no platform had one) and for right-to-left
  layout where a platform had no general row.

Rows added after pass 1 carry a trailing "†". Each platform's rows, with their sources,
are in its own file (§8); this section is the cross-platform view built from them, each row
tagged with the capabilities (§4.3) it needs the model to have.

| Platform | §4.1 | §4.2 | §4.3 | Total |
|---|---:|---:|---:|---:|
| macOS | 205 | 42 | 40 | 287 |
| Windows | 166 | 52 | 57 | 275 |
| KDE | 300 | 111 | 95 | 506 |
| GNOME | 290 | 74 | 69 | 433 |
| Material | 175 | 33 | 39 | 247 |
| iOS | 232 | 52 | 80 | 364 |
| **All** | 1368 | 364 | 380 | 2112 |

The columns are the platform files' own sections: §4.1 an element the model has, §4.2 a
control it lacks, §4.3 a property that crosses widgets. A row counts once. Not counted are
42 pass-1 rows that only point to another row or record "no gap" (macOS 2, Windows 1, KDE
13, GNOME 9, Material 7, iOS 10). A row ID is unique within its file.

Pass 2, per platform (detail in each file's *Cross-check* section):

| Platform | Rows added (§4.1 / §4.2 / §4.3) | Controls others list: own control / inside an element / not on the platform | Capabilities: gained rows / no further row (with reason) | "(none)" cells: right / drawn, now a row |
|---|---|---|---|---|
| macOS | 27 / 11 / 9 | 7 / 29 / 21 | 19 / 9 | 32 / 29, and 2 that differ by generation |
| Windows | 12 / 4 / 12 | 8 / 11 / 27 | 19 / 9 | 43 / 13 |
| KDE | 15 / 16 / 11 | 12 / 19 / 19 | 23 / 5 | 38 / 20 |
| GNOME | 9 / 7 / 8 | 8 / 12 / 31 | 12 / 16 | 44 / 19 |
| Material | 9 / 2 / 7 | 2 / 17 / 39 | 19 / 9 | 98 / 27 |
| iOS | 21 / 14 / 3 | 12 / 18 / 20 | 22 / 6 | `ios-13` values with no source: 17 for things iOS does not draw / 62 to measure |

The follow-up, per platform (each file's *Seed follow-up*, *Relabelled controls* and
*Fonts and mirroring* sections; IDs `…SF…†`, `…RL…†` and `…FM…†`):

| Platform | Seed rows | Relabelled-control rows | Font and mirroring rows |
|---|---:|---:|---:|
| macOS | 10 | 2 | 2 |
| Windows | 13 | 3 | 2 |
| KDE | 23 | 7 | 3 |
| GNOME | 21 | 1 | 3 |
| Material | 4 | 1 | 4 |
| iOS | 9 | 3 | 2 |

### 4.1 Elements the model has: gaps per section

The rows of §4.1 of each platform file (parts, states and properties of an element the
model has), by the model section they belong to. Their capabilities are in §4.3.

| Model section | macOS | Windows | KDE | GNOME | Material | iOS |
|---|---:|---:|---:|---:|---:|---:|
| `[button]` | 11 | 9 | 15 | 19 | 14 | 18 |
| `[card]` | 2 | 1 | 8 | 5 | 3 | 1 |
| `[checkbox]` | 8 | 7 | 12 | 11 | 9 | 2 |
| `[combo_box]` | 6 | 7 | 12 | 6 | 3 | 4 |
| `[defaults]` | 11 | 0 | 2 | 9 | 0 | 20 |
| `[dialog]` | 7 | 11 | 24 | 24 | 12 | 16 |
| `[expander]` | 5 | 3 | 2 | 7 | 1 | 4 |
| `[input]` | 7 | 12 | 19 | 10 | 18 | 9 |
| `[layout]` | 3 | 1 | 4 | 3 | 1 | 5 |
| `[link]` | 3 | 3 | 4 | 4 | 0 | 1 |
| `[list]` | 19 | 10 | 31 | 23 | 14 | 22 |
| `[menu]` | 17 | 12 | 24 | 16 | 11 | 18 |
| `[popover]` | 5 | 4 | 4 | 8 | 1 | 7 |
| `[progress_bar]` | 5 | 5 | 8 | 7 | 7 | 4 |
| `[scrollbar]` | 8 | 8 | 12 | 10 | 4 | 7 |
| `[segmented_control]` | 10 | 2 | 0 | 13 | 4 | 9 |
| `[separator]` | 0 | 1 | 3 | 2 | 1 | 2 |
| `[sidebar]` | 9 | 10 | 18 | 13 | 9 | 10 |
| `[slider]` | 7 | 9 | 11 | 15 | 12 | 9 |
| `[spinner]` | 5 | 1 | 4 | 7 | 6 | 5 |
| `[splitter]` | 4 | 2 | 4 | 1 | 3 | 3 |
| `[status_bar]` | 1 | 2 | 1 | 1 | 0 | 0 |
| `[switch]` | 6 | 5 | 5 | 10 | 9 | 7 |
| `[tab]` | 7 | 10 | 17 | 20 | 8 | 17 |
| `[text_area]` | 5 | 1 | 0 | 3 | 1 | 7 |
| `[text_scale]` | 2 | 1 | 6 | 6 | 5 | 4 |
| `[toolbar]` | 11 | 6 | 18 | 8 | 8 | 8 |
| `[tooltip]` | 6 | 4 | 11 | 4 | 7 | 3 |
| `[window]` | 15 | 19 | 21 | 25 | 4 | 10 |

### 4.2 Standard controls the model does not have

The rows of §4.2 of each platform file, sorted by how many platforms list the control. A
"–" is settled in that platform's *Cross-check*, table A, or in its *Relabelled controls*
section: the platform has no such control (with the source that shows it), or draws it
inside an element the model has, whose rows then cover it (a tree view inside `[list]`).

| Control | macOS | Windows | KDE | GNOME | Material | iOS |
|---|---|---|---|---|---|---|
| badge | MAC-N11 | W2-08 | Q2.25, Q2.26, Q2.27 | GN-2.36, GN-2.37 | MAT-N10 | C-15 |
| date picker | MAC-N04 | W2-03 | K2.23† | – | MAT-N16 | C-5 |
| level indicator/gauge | MAC-N06, MAC-N22 | W2-41† | K2.6, K2.18 | GN-2.26 | – | C-30, C-31 |
| menu bar | MAC-N20, MAC-N33†, MAC-N34† | W2-22 | K2.1, Q2.9, Q2.10 | GN-2.30 | – | C-37† |
| navigation/app bar | – | W2-11 | K2.14, Q2.14, Q2.15 | GN-2.18, GN-2.20 | MAT-N11, MAT-N25 | C-1, C-28, C-43† |
| pull-to-refresh | MAC-N24 | W2-26 | Q2.54† | – | MAT-N28 | C-10 |
| search field | MAC-N02 | W2-15 | – | GN-2.21, GN-2.22 | MAT-N14, MAT-N15 | C-3 |
| stepper/spin box | MAC-N01 | W2-01 | K2.3, Q2.1 | GN-2.05, GN-2.24 | – | C-4 |
| autocomplete | – | – | K2.29† | GN-2.56† | MAT-N27 | C-38† |
| banner/inline message | – | W2-07 | K2.13, Q2.16, Q2.17, Q2.18, Q2.19, Q2.20, Q2.21 | GN-2.11, GN-2.31 | MAT-N23 | – |
| bottom tab bar | MAC-N23 | – | Q2.33, Q2.34, Q2.35, Q2.36 | – | MAT-N12 | C-22 |
| breadcrumb/path control | MAC-N07 | W2-10 | K2.27†, Q2.44 | GN-2.52† | – | – |
| calendar | – | W2-02 | K2.4 | GN-2.25 | – | C-6 |
| carousel | – | W2-31 | – | GN-2.55† | MAT-N18 | C-48† |
| colour button | MAC-N05 | W2-42† | K2.20 | GN-2.27 | – | – |
| colour picker | – | W2-05 | K2.25† | GN-2.53† | – | C-8 |
| media transport controls | MAC-N30† | W2-35 | – | GN-2.54† | – | C-42† |
| page control/pager | – | W2-20, W2-21 | Q2.11 | GN-2.38 | – | C-9 |
| preference rows/boxed list | MAC-N21 | W2-38, W2-40† | Q2.41, Q2.42, Q2.45, Q2.46, Q2.47, Q2.55†, Q2.56† | GN-2.01, GN-2.02, GN-2.03, GN-2.04, GN-2.06, GN-2.07, GN-2.08, GN-2.09, GN-2.10 | – | – |
| rich text view | MAC-SF02†, MAC-SF03†, MAC-SF04†, MAC-SF06† | W-SF01†, W-SF02†, W-SF03†, W-SF04†, W-SF05†, W-SF06†, W-SF07†, W-SF08† | K-SF12†, K-SF13†, K-SF14†, K-SF15†, K-SF16†, K-SF17† | – | – | SF-2†, SF-3†, SF-4†, SF-6† |
| sheet/bottom sheet | – | – | K-SF23†, Q2.37, Q2.38, Q2.39, Q2.40 | GN-2.17 | MAT-N19, MAT-N20, MAT-N21, MAT-SF03† | C-12 |
| split button | MAC-N28† | W2-17 | – | GN-2.29 | MAT-N7 | – |
| status page/placeholder | MAC-N35† | – | Q2.30, Q2.31, Q2.32 | GN-2.13 | – | C-16 |
| teaching tip | MAC-N36† | W2-09 | K2.31† | – | – | C-36† |
| about dialog | MAC-N32† | – | Q2.57† | GN-2.39 | – | – |
| avatar | – | W2-24 | Q2.28, Q2.29 | GN-2.14 | – | – |
| chip | – | W2-43† | Q2.22, Q2.23, Q2.24 | – | MAT-N9 | – |
| edit menu | – | – | Q2.53† | – | MAT-N30† | C-14 |
| file chooser | – | – | K2.26† | GN-2.48 | – | C-39† |
| find bar | MAC-N18 | – | K2.30† | – | – | C-25 |
| group box/frame | – | – | K2.8, Q2.12, Q2.13 | GN-2.32 | – | C-32 |
| menu button | MAC-N29† | W2-18 | – | – | MAT-N8 | – |
| rubber band | MAC-N31† | – | K2.12 | – | – | C-47† |
| shortcuts dialog | – | – | K-RL02† | GN-2.41 | – | RL-1† |
| time picker | – | W2-04 | K2.24† | – | MAT-N17 | – |
| toast/snackbar | – | – | Q2.49, Q2.50, Q2.51, Q2.52 | GN-2.12, GN-2.50 | MAT-N22 | – |
| wizard | – | W-RL03† | K2.22 | GN-RL01† | – | – |
| button group/toggle group | – | – | Q2.3 | – | MAT-N6 | – |
| chart | MAC-N37† | – | – | – | – | C-45† |
| code editor | – | – | K-SF01†, K-SF02†, K-SF03†, K-SF04†, K-SF05†, K-SF06†, K-SF07†, K-SF08†, K-SF09†, K-SF10†, K-SF11† | GN-SF01†, GN-SF02†, GN-SF03†, GN-SF04†, GN-SF05†, GN-SF06†, GN-SF07†, GN-SF08†, GN-SF09†, GN-SF10†, GN-SF11†, GN-SF12†, GN-SF13† | – | – |
| dial/circular slider | MAC-N15 | – | K2.5, Q2.2 | – | – | – |
| editable label | – | – | – | GN-2.45 | – | RL-2† |
| emoji chooser | – | – | K-RL03† | GN-2.43 | – | – |
| FAB | – | – | Q2.4, Q2.5, Q2.6, Q2.7, Q2.8 | – | MAT-N1, MAT-N2, MAT-N3 | – |
| font chooser | – | – | K2.28† | GN-2.28 | – | – |
| full-screen dialog | – | – | Q2.58† | – | MAT-N26 | – |
| help button | MAC-N10 | – | Q2.48 | – | – | – |
| icon button | – | – | – | – | MAT-N4, MAT-N5 | C-29 |
| image frame | MAC-N12 | – | K2.21 | – | – | – |
| ink toolbar | – | W2-36 | – | – | – | C-40† |
| inspector/utility panel | MAC-N14, MAC-N16, MAC-N19 | – | – | – | – | C-41† |
| navigation drawer/pane | – | W2-30 | – | GN-2.19, GN-2.34, GN-2.51 | – | – |
| password field | – | W2-16 | – | GN-2.23, GN-2.58† | – | – |
| picker wheel | – | – | – | – | MAT-N31† | C-7 |
| rating | – | W2-06 | K2.19 | – | – | – |
| text ruler | MAC-N17 | – | K2.32† | – | – | – |
| token field | MAC-N03 | W2-39 | – | – | – | – |
| view switcher | – | W2-27, W2-32 | – | GN-2.15, GN-2.16, GN-2.33, GN-2.35 | – | – |
| wrap box | – | – | – | GN-2.46 | MAT-RL01† | – |
| access key badge | – | W2-37 | – | – | – | – |
| action sheet | – | – | – | – | – | C-11 |
| activity rings | – | – | – | – | – | C-46† |
| back button | – | – | – | – | – | C-2 |
| background extension | – | – | – | – | – | C-19 |
| bottom accessory | – | – | – | – | – | C-21 |
| character selector | – | – | K-RL04† | – | – | – |
| collection view | MAC-N13 | – | – | – | – | – |
| column browser | MAC-N08 | – | – | – | – | – |
| command bar | – | – | K-RL01† | – | – | – |
| command link button | – | – | K2.7 | – | – | – |
| composite dialogs | – | – | – | GN-2.57† | – | – |
| context menu preview | – | – | – | – | – | C-13 |
| dock frame (libpanel) | – | – | – | GN-SF14†, GN-SF15†, GN-SF16† | – | – |
| dock/panel title bar | – | – | K2.10, K2.11 | – | – | – |
| document launch view | – | – | – | – | – | C-44† |
| drag and drop | – | – | – | – | – | C-24 |
| drag handle | – | – | Q2.43 | – | – | – |
| drag icon | – | – | – | GN-2.49 | – | – |
| editing HUD | – | – | – | – | – | C-49† |
| glass container | – | – | – | – | – | C-20 |
| grid view | – | W2-13 | – | – | – | – |
| items view | – | W2-14 | – | – | – | – |
| keyboard | – | – | – | – | – | C-23 |
| keyboard shortcut keycap | – | – | – | GN-2.40 | – | – |
| large content viewer HUD | – | – | – | – | – | C-26 |
| list box | – | W2-33 | – | – | – | – |
| loading indicator | – | – | – | – | MAT-N24 | – |
| navigation rail | – | – | – | – | MAT-N13 | – |
| paste control | – | – | – | – | – | C-17 |
| radio group | – | W2-25 | – | – | – | – |
| repeat button | – | W2-19 | – | – | – | – |
| rule editor | MAC-N09 | – | – | – | – | – |
| scrim | – | – | – | – | MAT-N29 | – |
| scroll edge effect | – | – | – | – | – | C-18 |
| scrollbar annotation | – | W2-28 | – | – | – | – |
| side tab bar | – | – | K2.16 | – | – | – |
| splitter collapser | – | – | K2.17 | – | – | – |
| swipe actions | – | W2-29 | – | – | – | – |
| system-drawn panels | MAC-N25 | – | – | – | – | – |
| text input system UI | MAC-N27† | – | – | – | – | – |
| text selection handles | – | – | – | GN-2.42 | – | – |
| title widget | – | – | K2.15 | – | – | – |
| tool box | – | – | K2.9 | – | – | – |
| toolbar flyout | – | W2-23 | – | – | – | – |
| touch bar | MAC-N26 | – | – | – | – | – |
| tree view | – | W2-12 | – | – | – | – |
| video play overlay | – | – | – | GN-2.44 | – | – |
| window tab bar | MAC-SF07† | – | – | – | – | – |
| writing tools | – | – | – | – | – | C-33 |
| zoom button | – | W2-34 | – | – | – | – |

### 4.3 Capabilities the model lacks, across all rows

One row can need several capabilities (the checked-and-hovered switch's border colour is
C07 and C12), so a column's counts add up to more than the platform's total. A count is how
many of that platform's rows need the capability; it shows where a capability matters, not
how hard it is. Where a platform's count is low or zero, its *Cross-check*, table B, says
why (Windows states its hover colours per state, so it needs no state-layer model, C08).
Two IDs are not capabilities of the model's types: C24 marks a §4.2 row (a whole control
the model lacks), and C25 a value the platform exposes at run time that no reader reads.
The row IDs per capability are in Appendix A.

| ID | Capability | macOS | Windows | KDE | GNOME | Material | iOS |
|---|---|---:|---:|---:|---:|---:|---:|
| C01 | Shadow geometry: layers, each with offset, blur, spread, colour, inset; per surface | 14 | 17 | 31 | 31 | 6 | 21 |
| C02 | Elevation: a named elevation ramp/levels per element, elevation per state, surface tint | 1 | 9 | 0 | 0 | 35 | 0 |
| C03 | Materials: translucency, backdrop blur, vibrancy, Mica/Acrylic, Liquid Glass, their tint/noise/luminosity and opaque fallbacks | 24 | 17 | 7 | 3 | 0 | 45 |
| C04 | Colour composition: a colour stated relative to what is behind it (alpha fill, mix of current text colour, overlay) rather than one resolved opaque colour | 5 | 9 | 84 | 71 | 7 | 6 |
| C05 | Colour roles missing from defaults (accent ramp, standalone accent, status background/foreground, tertiary, container/surface ladder, inverse, header/complementary colour sets, shade roles) | 10 | 21 | 70 | 57 | 11 | 11 |
| C06 | Colour variant axes beyond light/dark: increased/high contrast values, inactive/non-key window, elevated level, contrast levels | 22 | 12 | 21 | 40 | 2 | 12 |
| C07 | States and state combinations: pressed, focused, dragged, drop target, indeterminate, read-only, invalid/error, default, open, checked/selected × hover/pressed/disabled, per-state colours of a part | 66 | 98 | 106 | 86 | 80 | 46 |
| C08 | State-layer model: hover/pressed/focus/drag overlay opacities, ripple | 1 | 0 | 2 | 0 | 30 | 0 |
| C09 | Widget variants/styles: flat, outlined, filled, tonal, destructive, pill, circular, bezel styles, field styles, segmented styles, glass styles | 41 | 9 | 47 | 52 | 32 | 35 |
| C10 | Size variants and density: control sizes (mini…extraLarge), compact density, touch vs mouse sizing | 17 | 19 | 39 | 57 | 32 | 23 |
| C11 | Shape: per-corner radii, continuous corner curve, capsule/fully rounded, concentric radii, shape change between states | 26 | 31 | 35 | 50 | 52 | 34 |
| C12 | Border/outline model: per-side widths and colours, gradient strokes, inner+outer outlines, outline drawn outside the box, state-dependent border thickness | 23 | 31 | 66 | 57 | 22 | 14 |
| C13 | Parts missing from a widget the model has: icons, glyphs, labels, helper text, counters, headers, footers, close buttons, indicators/pills, ticks, grabbers, arrows, their geometry and colours | 92 | 104 | 152 | 100 | 112 | 95 |
| C14 | Motion: durations, easing curves, springs, delays per transition, a speed factor, reduced-motion replacements | 18 | 59 | 90 | 55 | 40 | 70 |
| C15 | Focus indication: style (ring stroked/filled, inner/outer, underline, rounded rect), width, offset, keyboard-only rule, per-widget focus style, animation | 6 | 8 | 18 | 29 | 7 | 4 |
| C16 | Text caret: colour, width, blink period / on-off / timeout, style | 2 | 2 | 7 | 12 | 2 | 3 |
| C17 | Selection and highlights: inactive/unemphasised selection, its text colour, find highlight, drag-and-drop highlight | 11 | 6 | 17 | 15 | 2 | 11 |
| C18 | Typography: letter spacing, line height per role, optical size / variable axes, font width, missing text styles, per-size text tables (Dynamic Type), font features, title fonts | 29 | 29 | 42 | 32 | 41 | 39 |
| C19 | Icons: state effects (disabled/hover/active), symbolic recolouring, size groups per context, rendering modes, symbol effects | 13 | 7 | 54 | 15 | 4 | 14 |
| C20 | Window chrome: title bar geometry and fills, caption/traffic-light buttons and their states, button layout, window corner radius, window border/outline, window shadow, maximised/tiled/snapped/fullscreen states, unfocused-window appearance | 19 | 21 | 25 | 27 | 4 | 11 |
| C21 | Visible timings: double-click, drag threshold, submenu/hover-open delays, tooltip show/hide/reshow, scrollbar auto-hide, toast durations | 6 | 11 | 30 | 22 | 9 | 9 |
| C22 | Pointer: cursor theme and size, pointer hover effects (iPad) | 3 | 2 | 6 | 4 | 2 | 4 |
| C23 | Accessibility variants as values: what high contrast, reduce transparency, bold text, button shapes, differentiate without colour, larger text change | 10 | 13 | 10 | 45 | 9 | 29 |
| C24 | A whole control the model lacks (the row is in §4.2) | 42 | 52 | 111 | 74 | 33 | 52 |
| C25 | Reader coverage: a value readable at run time that no reader reads today | 21 | 24 | 81 | 25 | 20 | 12 |
| C26 | Layout and placement: alignment, ordering, positions, insets, spacing rules not tied to one widget part | 45 | 54 | 129 | 99 | 48 | 46 |
| C27 | Gradient paint: linear/radial gradient fills and strokes, bevels and highlights drawn as gradients | 4 | 6 | 5 | 7 | 1 | 3 |
| C28 | Colour space and precision: wide gamut (Display P3), HDR, more than 8 bits per channel | 2 | 0 | 0 | 0 | 0 | 2 |

C27 and C28 were added after pass 1's tagging and applied from each row's own text (the
words *gradient*, *bevel*; *P3*, *HDR*, *wide gamut*, *extended range*), then checked by
hand; the later rows were tagged with all 28.

What the counts show, in brief:

- **Every platform** draws layered shadows (C01) the model cannot state, animates almost
  everything (C14), draws parts the model has no field for (C13), and draws states and
  state combinations the model lacks (C07): pressed, focused, checked or selected with hover
  and pressed, indeterminate, drop target, the default button.
- **Translucency** (C03) is the core of macOS's and iOS's look (14 named macOS materials,
  Liquid Glass on macOS 26 and iOS 26) and of Windows' (Mica, Mica Alt, Acrylic); the model
  stores a solid colour where these draw a material.
- **Colours stated over what lies behind** (C04): KDE and GNOME derive most fills as a
  blend or a share of the text colour over the surface below, and WinUI's hover and pressed
  fills are translucent too; the model stores one opaque colour, resolved over one
  background, which is wrong on any other surface.
- **Variant axes** (C06, C23): increased contrast and the inactive (non-key) window
  change colours on every platform, and iOS's elevated level on iOS; the model has an
  accessibility flag for contrast with no colours behind it, and for the inactive window
  only the title-bar and selection colours.
- **Variants and sizes** (C09, C10): bezel styles, flat, destructive, pill, outlined,
  tonal, glass; macOS's five control sizes, WinUI's compact density, Material's density.
- **Shape and borders** (C11, C12, C27): per-corner radii, the continuous corner curve,
  capsules, concentric radii; per-side and gradient strokes, inner and outer outlines,
  outlines drawn as spread shadows, the focused field's thicker accent edge.
- **Typography** (C18): letter spacing, line height per role, optical size, the text styles
  the model has no slot for (macOS has 11, Material 15; the model has 4 besides the body
  font, `[text_scale]`), per-size Dynamic Type, the emoji font and the fallback chain for
  scripts the UI font lacks (the model has one family name).
- **Window chrome** (C20): title-bar geometry, the caption and traffic-light buttons and
  their states, the button layout, window corners, the window's own outline and shadow,
  maximised, tiled and full-screen states, the dimmed parent of a modal dialog.
- **Focus, caret, selection** (C15–C17): Breeze's filled focus ring shown only after
  keyboard focus, WinUI's inner and outer rings, Full Keyboard Access on macOS and iOS, the
  caret's width and blink timing (read at run time on Windows, KDE and GNOME),
  unemphasised selection, the find highlight.
- **Placement** (C26): where a dialog or a sheet sits and the window sizes at which it
  turns into a bottom sheet or a full-screen dialog, alignment and order of buttons, insets,
  gaps a platform states beside a widget's own padding.
- **Whole controls** (C24, §4.2): from the badge, date picker, menu bar and search field,
  which nearly every platform has, to those of one or two (the code editors of KDE and
  GNOME, macOS's rule editor, Windows' annotated scrollbar, KDE's side tab bar, iOS's
  action sheet).
- **Reader coverage** (C25): values the platforms expose at run time that no reader reads,
  most on KDE (seven colour sets with Active, Inactive and Disabled groups, the colour
  effects, `breezerc`, `kwinrc`); iOS has no reader at all.

## 5. Seeds

The gaps the repository had already written down, gathered before the platform research,
are in `todo_v0.7.1_model-completeness-gap-map-seeds.md`: 37 groups, each with every
file:line that records it (at caeddd6b, as are the citations in the table below), and in
its second part the 222 lines of `platform-facts.md` with a ❓ cell, a value no source gave,
which under this milestone's rule is a value to measure.

Each group was matched to the platform rows after pass 2 (G37, the "(none)" cells, was
settled by pass 2 itself): 24 were covered, 5 partly, and 7 are not gaps of the model. The
follow-up then wrote the rows the partly covered groups lacked, on every platform: a code
editor's colours and parts (G12; KDE's KTextEditor and GNOME's GtkSourceView, the other
platforms ship none), the drop zone under a dragged pane or dock (G19), where a modal
dialog is placed (G24), and rich text and Markdown (G27); G35's keycap is GN-2.40, asked
of every platform in the relabelled-control check. Of the 7 that are not model gaps, G03
(iced's renderer `snap`), G29–G31 (fields the model has) and G32 (the toolkits' own
fallbacks) are connector matters; G34 and G36 are presets that state no value or one
without a source, which is value work (§6.4). The table is the matching as it was before
the follow-up.

| Group | Verdict | Rows | Note |
|---|---|---|---|
| G01 shadow geometry | COVERED | GNOME GN-3.01, GN-1.01, GN-1.179, GN-1.208, GN-1.252; KDE K3.1, K3.2, Q3.20, Q3.21; macOS MAC-X01, MAC-W07; Windows W3-01, W3-02, W1-003; Material MAT-X2, MAT-X3, MAT-X5; iOS X-SH-1, X-SH-2 | Registry has only defaults.shadow_color and border.shadow_enabled; every platform has a layer-list row. Windows' two-layer vs one colour is W3-01 and W1-003. |
| G02 shadow on a specific widget | COVERED | slider thumb: GN-1.97, K1.101, Q1.47, MAC-SL02, SL-1, MAT-S9; popup/menu/tooltip: GN-1.179, K1.190, Q1.23, MAC-PO03, MAC-MN14, W1-115, W1-053, PO-3, MN-2, MAT-PO1; card: GN-1.252, Q1.127, W1-139, CD-1; switch thumb: GN-1.201, MAC-SW02, SW-1, MAT-SW7 | Progress bar, expander and scrollbar shadow have no row because no platform draws one (todo.md progress-bar entry: platform-facts gives no shadow on any platform); that is a preset-source question (border.shadow_enabled unstated), not a missing property. |
| G03 snap | NOT A MODEL GAP | none | Seed lines (derived.rs:84,96,...,554,705): "a renderer setting, cfg!(feature = \"crisp\")" - iced renderer pixel snapping, no platform property. |
| G04 checked checkbox hover/pressed | COVERED | GN-1.57, K1.54, MAC-C04, MAC-C02, W1-037, MAT-C3 | Registry [checkbox] has hover_background only (the unchecked box), while [switch] has hover_checked_background; gap stands for the checkbox. |
| G05 checkbox disabled checked fill | COVERED | K1.54, MAC-C03, W1-037, MAT-C7, Q1.22 | [checkbox] has disabled_background only; [switch] has disabled_checked_background. The "disabled_opacity has no receiver" half (iced styles.rs:885) is a connector matter. |
| G06 check mark glyph/size, radio dot | COVERED | glyph/size: MAC-C06, W1-039, W1-038, MAC-C01, GN-1.58, K1.50, MAT-C6; radio dot: W1-041, W1-042, W1-043, MAC-C07, MAT-C1, MAT-C2, CK-1 | Registry has check_mark_stroke_width_px, radio_dot_diameter_px, radio_indicator_width_px; what is missing is the glyph shape/extent and per-state dot, which the rows record. |
| G07 pressed/hovered primary and status button fills | COVERED | GN-1.30, GN-1.31, MAC-B03, MAC-B02, MAC-B06, W1-020, MAT-B2, B-6, B-11 | [button] has primary_background but no primary hover/pressed. |
| G08 pressed/hovered selected segment, segment text on hover | COVERED | GN-1.241, GN-1.242, MAC-SG06, MAC-SG01, W1-138, SG-4, MAT-SG2 | [segmented_control] has hover_background and active_background; no hover text colour or pressed (todo.md "a hovered segment's text colour"; W1-138 is the HC-hover case). |
| G09 dragged thumb; slider handle/rail border | COVERED | dragged: GN-1.99, K1.102, MAC-SL07, SL-3, MAT-S8; border: GN-1.96, GN-1.105, K1.100, Q1.46, W1-069, SL-1, MAT-S10 | [slider] has thumb_hover_color only; tick_mark_length_px exists (no receiver is a connector matter: GN-1.100, K1.103, MAC-SL04, W1-071 carry tick looks). |
| G10 progress bar motion, min width, diameter | COVERED | motion: W1-075, GN-1.113, K1.110, MAC-P01, MAT-P6, PB-3; min width: K1.114; circular diameter: MAT-SN3, MAC-SN04, W1-130 | [progress_bar] already has min_width_px; iced "no receiver at all" is the connector. Motion is the whole motion category (G26). |
| G11 spinner geometry | COVERED | stroke: GN-1.227, MAT-SN1, MAT-SN3, Q1.121; fins/arc: MAC-SN01, SP-1a, GN-1.229, MAT-SN6; min/named size: MAC-SN03, SP-3a, GN-1.228, Q1.120; track: GN-1.226, W1-130 | [spinner] has stroke_width_px and min_diameter_px but no fins/arc geometry. Container background/border (derived.rs:924-969) is NOT a model gap: a spinner is painted on the consumer's surface. |
| G12 scrollbar | PARTLY | container/rails/borders: GN-1.84, GN-1.86, MAC-S03, MAC-S04, K1.90, W1-063; gap/corner: K1.96, MAC-S06, W1-065; margins/anchor: GN-1.85, Q1.39, SB-2, MAC-S01; min thumb: GN-1.92; arrows: K1.94, W1-061 | Not covered by any row: config.rs:101 - code-editor line numbers, current line, invisibles and syntax colours (no platform has a system syntax scheme; no row). The iced "auto-scroll overlay has no native counterpart" and "ScrollbarTheme states nothing about the scrolled container" lines are NOT model gaps. Registry [scrollbar] has min_thumb_length_px and thumb_active_color. |
| G13 text input icon colour, placeholder, open pick-list | COVERED | icon colour: GN-1.44, MAT-I7, Q1.17, K1.43, W1-030; placeholder: W1-028, MAT-I3; open/arrow: CB-3, MAC-CB04, K1.242, GN-1.237, W1-131, MAT-CB3; arrow colour: K1.246, MAC-CB03, W1-132, MAT-CB2 | [input] has placeholder_color (one for all states). |
| G14 switch font, label gap, thumb radius | COVERED | font: SW-7; label gap: Q1.103, W1-124; thumb radius/shape: SW-3, MAC-SW01, W1-122, MAT-SW1 | [switch] has track_radius_px and thumb_diameter_px, no font, label gap or thumb radius. |
| G15 link border, pressed/disabled fill, darkening | COVERED | W1-144, LK-1, Q1.134, MAC-LK01, GN-1.266, GN-1.267, W1-143 | [link] has no border, pressed fill or disabled opacity; Kirigami's pressed darkening is Q1.134. |
| G16 card text colour, font, fill | COVERED | GN-1.255, W1-139, MAT-CA1, MAT-CA2, Q1.129, CD-1, MAC-BX01 | [card] has background_color and border only. |
| G17 expander frame fill, label, hover, pressed header | COVERED | W1-140, W1-141, MAT-EX1, GN-1.260, GN-1.263, K1.250, MAC-EX03, EX-3, EX-4 | [expander] has hover_background and frame_enabled; no resting fill, label colour, pressed header or disabled opacity. |
| G18 rule/separator radius, fill mode, toolbar rule | COVERED | separator geometry: W1-119, W1-101, MAT-D1, Q1.95, GN-1.187, SE-1; toolbar rule: K1.151, Q1.68, Q1.71, MAC-TL08, TB-2, W1-101 | Rule `fill_mode` is iced's own enum; the platform facts are the insets (W1-119, MAT-D1, GN-1.187). "A line holds no text", "the bar is square" lines are NOT model gaps. |
| G19 pane grid hovered_region / drop target | PARTLY | dragged divider: MAC-SP03, SP-2, W1-118, MAT-SP2; other drop targets: K1.182, MAC-L11, W1-110, L-10, GN-1.166, GN-1.127, MAT-L10 | No row covers the highlight under a pane dragged over another pane (dock / pane-grid drop target). Rows exist only for list rows, tabs and boxes. |
| G20 menu bar fill, text colour, geometry | COVERED | GN-2.30, K2.1, Q2.9, Q2.10, MAC-N20, W2-22 | Registry has no [menu_bar] section. |
| G21 sidebar unselected fill, hovered label, radius | COVERED | W1-091, W1-092, GN-1.140, GN-1.141, MAC-SB02, MAC-SB04, MAC-SB05, SD-2, MAT-SB2, MAT-SB1 | [sidebar] has hover_background and selection_*; no item radius, hover label or text. |
| G22 tab: unselected border, min size, selected flag, separators, flares | COVERED | border/lines: W1-086, W1-088, K1.121, K1.127, GN-1.135; min size: K1.123, W1-085, GN-1.129, Q1.52, T-14; separators: W1-084, GN-1.126; selected+hover: GN-1.124, K1.122, MAC-TB04; flares/cut corners: K1.128 | [tab] has min_width_px, min_height_px, active_outward_corner_radius_px. "no receiver" and the iced `selected` flag are connector matters. |
| G23 list row / table header / grid / footer | COVERED | disabled: GN-1.176, K1.174, W1-112, MAT-L4; radius: K1.170, W1-108, GN-1.177; grid: MAC-L06, GN-1.169; header: GN-1.167, MAC-L05, K1.177, MAT-L12; footer: L-5, MAT-L12; striping: L-18 | [list] already has alternate_row_background, grid_color, header_*, disabled_text_color; no footer. |
| G24 title bar, chrome, toolbar height, dialog placement | PARTLY | title bar: GN-1.11, K1.1, MAC-W01, W1-006; chrome: GN-1.06, K1.10, MAC-W05, W1-004; accent title bar: W1-013, W1-001; toolbar height: MAC-TL01, Q1.69, W1-103, TB-3; window border/shadow: K3.1, W1-003, MAC-W07 | Not covered: where a platform places an in-window modal dialog vertically (todo.md "Dialog: the platform's vertical placement"). Nearest rows are margins/size only: GN-1.218, GN-1.219, Q1.113, MAT-DG8, MAC-D03. |
| G25 focus ring | COVERED | GN-3.27, GN-3.29, K3.20, Q3.30, MAC-X13, MAT-X10, X-FO-1, W1-025 | Registry already has defaults.focus_ring_color/width/offset; the iced "no receiver" half is a connector matter. Which part is ringed: GN-3.29, K3.21, Q1.21, MAC-L14. |
| G26 motion, reduced motion, HC, reduced transparency | COVERED | motion: GN-3.18, K3.30, K3.31, MAC-X11, MAT-X13, W3-30, X-MO-1; reduced motion: GN-3.25, K3.90, MAC-X25, MAT-X20, W3-37, X-A11-3; HC: GN-3.61, K3.91, MAC-X22, MAT-X18, W3-35, X-A11-1; transparency: K3.92, MAC-X23, W3-38, X-A11-2 | No registry section for motion or accessibility. The iced README "no receiver" is a connector matter. |
| G27 text colours and fonts | PARTLY | status/on-fill text colours: GN-1.269, K3.52, Q3.10, W3-17; selection text: MAC-X15, GN-3.41, K3.40, W3-25, X-SE-1; weight/width axes: D-3, TS-3, K3.46, GN-1.193 | No row: markdown inline code, list markers and blockquotes (todo.md "Markdown ... model states none of them"); no row for a window label's own font. |
| G28 tooltip delay, sidebar width, overlay delay | COVERED | sidebar width: GN-1.147, W1-094, Q1.60, MAT-SB7; tooltip delay: GN-1.82, K1.80, Q3.28, MAC-T04, W1-059, MAT-T6; submenu/overlay delay: K1.73, GN-1.77, W1-054, MAT-M5, W1-116 | Registry already has tooltip.show_delay_ms; iced lib.rs:1591 "states none" is a preset gap (ios-13 leaves it unset). Sidebar width is not in [sidebar]. |
| G29 table row/header height | NOT A MODEL GAP | none | Registry [list] has row_height_px and header_height_px. Seeds: iced "table has no row height of its own" (lib.rs:1604-1608) is the toolkit; gpui "None where the theme states none" (todo_gpui-full-theme.md:720-722) is a preset value. |
| G30 button min vs exact height, padding sides, active/checked | NOT A MODEL GAP | none | Registry [button] has min_height_px, active_background, checked_background, and border.padding_* sides. The seed is gpui drawing a minimum as an exact height (todo.md:3994) and "no receiver" for stated sides; presets state some sides none. |
| G31 combo box right padding, column, arrow geometry | NOT A MODEL GAP | none | combo_box_dyn egui comment: "None where the theme states no right padding or column: egui's own place stands"; registry has border.padding_right_px and [combo_box] arrow_area_width_px, arrow_icon_size_px. WinUI's separate arrow column (todo.md:3396) is a preset/field-extension question, no platform row exists. |
| G32 generic "egui's/gpui's own" fallbacks | NOT A MODEL GAP | none | egui mapping.toml:19 "[unwritten]: sink path -> why egui's own value stands"; each is a toolkit sink the model does not feed, or a value the model carries and a preset leaves unstated. |
| G33 icon / icon-set gaps | COVERED | icon sizes: GN-3.59, Q1.1, MAC-IC01, X-IC-2, D-19, K1.158, K1.181; stroke/weight/rendering: GN-3.57, MAC-IC02, X-IC-1; icon states: K3.60, GN-3.60 | `system_icon_set()` returning Material on other platforms (todo.md:3886) is API behaviour, not a visible property. |
| G34 windows-11 values with no field | NOT A MODEL GAP | none (related: W3-25, W3-35, W1-053, W3-01) | The seed's items are values with no Microsoft source (dark selection_text_color, menu acrylic fallback, HC mappings, one shadow colour) - preset sourcing. The hovered-segment text colour is G08, the single shadow colour G01. |
| G35 gnome/macOS presets mentioning no field | PARTLY | Pango stretch/variant/gravity: GN-1.193; widgets with no model section: GN-2.11, Q2.18, Q2.19 (alert), Q2.22-Q2.24, MAT-N9 (tag/chip), W2-10, Q2.44, MAC-N07 (breadcrumb), GN-2.36, Q2.25-Q2.27, MAC-N11, MAT-N10 (badge); size variants: GN-3.11, MAC-X17, X-SV-2, K3.83, Q3.31 | No row for Kbd (keycap) as a widget; border.line_width 0.5 / shadow_enabled "no source" lines (todo.md:3259-3263) are preset sourcing, not model gaps. |
| G36 unstated/unsourced presets | NOT A MODEL GAP | none (related: MAC-C04, MAC-B03, MAC-B02) | Seed header: "the model has a row, no preset value" (checkbox.hover macOS, macos-tahoe button.checked_background and active_background; registry [button] has both). |

## 6. Open work

1. **Scope, decided in the rationale.** Surfaces the OS draws outside the app's window,
   which an application neither draws nor styles (GNOME's on-screen keyboard, iOS widgets,
   Live Activities, notifications, Control Center, the shell's tiling previews, the macOS
   menu-bar extras' system part). The platform files decide this differently today: the
   Windows and Material files leave shell-drawn UI out, the macOS and KDE files have rows
   for some of it (MAC-N25, MAC-N27†, K-RL03†); the rationale settles it once for all six.
   Libraries a platform's own applications use beside its
   toolkit: KDE's kirigami-addons (FormCard, date and time pickers) and KDDockWidgets
   (Kdenlive), GNOME's GtkSourceView (GNOME Text Editor, Builder) and libpanel (Builder),
   the Windows Community Toolkit, whose rows pass 2 included, and its experimental Labs
   (`MarkdownTextBlock`). SwiftUI and Mac Catalyst where they draw differently from AppKit
   and UIKit.
2. **Citations, before any value is collected.** The KDE file cites KDE's GitHub mirrors at
   `master`: each citation is re-pinned to the release tags `platform-facts.md` uses
   (Breeze and KWin 6.7.x, KF 6.31.0); qtbase is cited at v6.10.1 in some rows and v6.11.2
   in others, and the GNOME Text Editor scheme at `main`. The rows marked "to verify"
   (macOS 46, Windows 52, KDE 37, GNOME 11, Material 44, iOS 27) are settled.
3. **Measuring, as the surveys found it:**
   - the GitHub macOS runner images have Reduce Motion and Reduce Transparency **on**, so
     motion and materials need a probe pass with both off;
   - the probe's offscreen renders draw the non-key look, so key-window states need the
     controls in the key-window capture;
   - the window captures use `screencapture -o`, which leaves shadows out;
   - the probe opens no menu and does not render the extraLarge control size;
   - the Windows probe only queries; it renders nothing (`tools/os-probe/src/windows.rs`);
   - no KDE (Qt Widgets, Kirigami), GNOME (GTK 4 / libadwaita) or iOS (UIKit, simulator)
     probe app exists yet, nor a Material one (material-web, Compose); whether the macOS
     CI image has the iOS 13–18 simulator runtimes is unchecked.
4. **Value work the map found** (not model gaps, done with the values): the 222 ❓ lines of
   `platform-facts.md`; the "(none)" cells pass 2 classified as drawn; the 62 `ios-13`
   values iOS draws with no source; the seed groups G34 and G36; and the values found
   wrong (§7).

## 7. Values found wrong on the way

The research read sources that disagree with values native-theme already ships: wrong
cells of `platform-facts.md`, readers that read the wrong key or none, and presets that
state values for things a platform does not draw. They are not gaps of the model; they are
recorded in `docs/todo.md`, *Values the v0.7.1 gap map found against their sources*. Rows
of the platform files that a later step found wrong are corrected in place, and each
correction names what corrected it (pass 2 or the follow-up, 2026-10-28).

## 8. The platform files

| File | Platform | Row IDs |
|---|---|---|
| `todo_v0.7.1_model-completeness-gap-map-macos.md` | macOS 12–15 and 26–27 | `MAC-…` |
| `todo_v0.7.1_model-completeness-gap-map-windows.md` | Windows 11 | `W1-…`, `W2-…`, `W3-…`, `W-SF…`, `W-RL…`, `W-FM…` |
| `todo_v0.7.1_model-completeness-gap-map-kde.md` | KDE Plasma 6, Breeze | `K…` (Qt Widgets, KDecoration, KWin), `Q…` (Kirigami, qqc2-desktop-style), `K-SF…`, `K-RL…`, `K-FM…` |
| `todo_v0.7.1_model-completeness-gap-map-gnome.md` | GNOME, libadwaita 1.10.0, GTK 4.24.1 | `GN-…` |
| `todo_v0.7.1_model-completeness-gap-map-material.md` | Material 3 and Expressive | `MAT-…` |
| `todo_v0.7.1_model-completeness-gap-map-ios.md` | iOS 13–18 and 26 | per section (`B-…`, `CK-…`, `X-MT-…`; `C-…` for the controls of §4.2; `SF-…`, `RL-…`, `FM-…`) |
| `todo_v0.7.1_model-completeness-gap-map-seeds.md` | the repository's own records, and the follow-up's sources | `G01`–`G37` |

Each file states its sources and their refs at its top (later sources in the section that
added them), and no file states a value it did not read in a cited source. A platform
file's own *Counts* section, where it has one, is pass 1's.

## Appendix A. Rows per capability

#### C01 — Shadow geometry

- macOS: MAC-B04, MAC-D03, MAC-MN14, MAC-N34†, MAC-PO03, MAC-SB06, MAC-SG02, MAC-SL02, MAC-SW02, MAC-T01, MAC-T06†, MAC-W04, MAC-W07, MAC-X01
- Windows: W1-003, W1-014, W1-053, W1-057, W1-096, W1-102, W1-115, W1-129, W1-133, W1-139, W2-01, W2-09, W2-15, W2-23, W3-01, W3-05, W3-53†
- KDE: K1.17, K1.18, K1.23, K1.81, K1.101, K1.190, K1.244, K2.5, K2.18, K2.31†, K3.1, K3.2, K3.3, K3.4, K-RL01†, Q1.16, Q1.23, Q1.33, Q1.47, Q1.56, Q1.89, Q1.90, Q1.106, Q1.112, Q1.125, Q1.127, Q1.128, Q2.7, Q2.49, Q3.20, Q3.21
- GNOME: GN-1.01, GN-1.02, GN-1.05, GN-1.07, GN-1.09, GN-1.14, GN-1.97, GN-1.133, GN-1.153, GN-1.156, GN-1.166, GN-1.178, GN-1.179, GN-1.183, GN-1.201, GN-1.208, GN-1.240, GN-1.252, GN-1.256, GN-2.12, GN-2.17, GN-2.18, GN-2.19, GN-3.01, GN-3.02, GN-3.03, GN-3.05, GN-3.06, GN-3.07, GN-SF13†, GN-SF15†
- Material: MAT-I17†, MAT-N30†, MAT-PO1, MAT-W4†, MAT-X3, MAT-X5
- iOS: C-13, C-24, C-40†, CD-1, MN-2, PO-3, RL-1†, SD-6, SD-9†, SG-1, SL-1, SW-1, T-9, TB-5, TT-1, W-2, W-10†, W-11†, X-SH-1, X-SH-2, X-SH-3

#### C02 — Elevation

- macOS: MAC-X01
- Windows: W1-003, W1-057, W1-115, W1-129, W1-139, W3-01, W3-02, W3-03, W3-04
- Material: MAT-B1, MAT-B5, MAT-B6, MAT-B13, MAT-CA1, MAT-CA2, MAT-L4, MAT-N1, MAT-N2, MAT-N9, MAT-N11, MAT-N13, MAT-N14, MAT-N15, MAT-N18, MAT-N19, MAT-N20, MAT-N21, MAT-N22, MAT-N23, MAT-N25, MAT-N26, MAT-N28, MAT-PO1, MAT-S9, MAT-SB6, MAT-SP2, MAT-SW7, MAT-T1, MAT-TA1, MAT-X2, MAT-X4, MAT-X6, MAT-X7, MAT-X29

#### C03 — Materials

- macOS: MAC-D03, MAC-L09, MAC-MN14, MAC-N14, MAC-N19, MAC-N20, MAC-N27†, MAC-N30†, MAC-N34†, MAC-PO02, MAC-SB06, MAC-SB07, MAC-T02, MAC-TL03, MAC-TL09, MAC-W08, MAC-W10, MAC-W12†, MAC-X02, MAC-X03, MAC-X04, MAC-X05, MAC-X06, MAC-X23
- Windows: W1-005, W1-014, W1-053, W1-056, W1-062, W1-088, W1-096, W1-102, W1-114, W1-133, W3-05, W3-06, W3-07, W3-08, W3-09, W3-10, W3-38
- KDE: K1.19†, K1.64, K1.65, K3.11, K3.12, K3.13, K3.92
- GNOME: GN-2.17, GN-3.04, GN-3.09
- iOS: B-2, C-1, C-9, C-11, C-13, C-14, C-18, C-19, C-20, C-23, C-26, C-36†, C-37†, C-40†, C-41†, C-42†, C-49†, CD-1, DL-1, MN-1, PO-2, RL-1†, SB-6, SD-1, SL-3, SW-2, T-1, T-8, TB-1, TB-5, TT-1, X-A11-2, X-MT-1, X-MT-2, X-MT-3, X-MT-4, X-MT-5, X-MT-6, X-MT-7, X-MT-8, X-MT-9, X-MT-10, X-MT-11, X-MT-12, X-SC-1

#### C04 — Colour composition

- macOS: MAC-B05, MAC-C05, MAC-N31†, MAC-X03, MAC-X09
- Windows: W1-018, W1-055, W1-088, W1-125, W1-159†, W3-09, W3-11, W3-16, W3-45†
- KDE: K1.7, K1.14, K1.22, K1.24, K1.25, K1.60, K1.65, K1.66, K1.67, K1.90, K1.91, K1.93, K1.111, K1.123, K1.124, K1.126, K1.142†, K1.152, K1.171, K1.177, K1.200, K1.242, K2.1, K2.12, K2.13, K2.21, K2.27†, K3.12, K3.13, K3.15†, K3.16†, K3.40, K3.50, K3.52, K-SF03†, K-SF05†, K-SF17†, K-SF18†, Q1.22, Q1.26, Q1.38, Q1.41, Q1.45, Q1.56, Q1.57, Q1.62, Q1.66, Q1.87, Q1.89, Q1.95, Q1.100, Q1.103, Q1.104, Q1.112, Q1.126, Q1.128, Q1.131, Q2.3, Q2.5, Q2.6, Q2.10, Q2.11, Q2.12, Q2.17, Q2.22, Q2.25, Q2.26, Q2.29, Q2.31, Q2.35, Q2.36, Q2.38, Q2.39, Q2.41, Q2.42, Q2.43, Q2.44, Q2.49, Q2.55†, Q3.14, Q3.15, Q3.16, Q3.22, Q3.33
- GNOME: GN-1.03, GN-1.10, GN-1.16, GN-1.24, GN-1.25, GN-1.29, GN-1.30, GN-1.31, GN-1.32, GN-1.34, GN-1.35, GN-1.52, GN-1.55, GN-1.57, GN-1.72, GN-1.86, GN-1.87, GN-1.98, GN-1.100, GN-1.102, GN-1.110, GN-1.120, GN-1.124, GN-1.141, GN-1.146, GN-1.155, GN-1.158, GN-1.159, GN-1.167, GN-1.174, GN-1.175, GN-1.176, GN-1.184, GN-1.198, GN-1.199, GN-1.205, GN-1.210, GN-1.226, GN-1.241, GN-1.242, GN-1.246, GN-1.253, GN-1.256, GN-1.260, GN-1.265, GN-1.271, GN-1.273, GN-1.279†, GN-2.01, GN-2.02, GN-2.11, GN-2.23, GN-2.29, GN-2.31, GN-2.40, GN-2.50, GN-2.51, GN-2.55†, GN-3.04, GN-3.08, GN-3.09, GN-3.12, GN-3.40, GN-3.41, GN-3.42, GN-3.43, GN-3.44, GN-3.60, GN-SF08†, GN-SF09†, GN-SF14†
- Material: MAT-DG5, MAT-N29, MAT-SC3†, MAT-X6, MAT-X8, MAT-X23, MAT-X24
- iOS: DL-2, SD-9†, SE-2, X-MT-4, X-MT-10, X-SC-1

#### C05 — Colour roles missing from defaults (accent ramp, standalone accent, status background/foreground, tertiary, container/surface ladder, inverse, header/complementary colour sets, shade roles)

- macOS: MAC-B07, MAC-DF01, MAC-DF02, MAC-DF03, MAC-DF04, MAC-DF05†, MAC-N06, MAC-N37†, MAC-TL04, MAC-X27
- Windows: W1-013, W1-055, W1-116, W1-125, W1-126, W1-139, W1-141, W2-07, W2-08, W2-30, W3-08, W3-09, W3-11, W3-12, W3-13, W3-14, W3-15, W3-16, W3-17, W3-18, W3-46†
- KDE: K1.49†, K1.122, K1.142†, K1.183, K1.184†, K1.260†, K2.1, K2.4, K2.6, K2.13, K2.15, K2.18, K2.20, K2.21, K2.23†, K2.30†, K2.31†, K3.23, K3.50, K3.51, K3.52, K3.53, K3.54, K3.61, K-RL01†, K-SF01†, K-SF02†, K-SF04†, K-SF10†, K-SF11†, Q1.2, Q1.13, Q1.37, Q1.52, Q1.53, Q1.54, Q1.68, Q1.71, Q1.81, Q1.93, Q1.96, Q1.112, Q1.121, Q1.123, Q1.129, Q1.134, Q2.1, Q2.8, Q2.9, Q2.16, Q2.22, Q2.25, Q2.28, Q2.33, Q2.34, Q2.37, Q2.49, Q3.1, Q3.2, Q3.4, Q3.5, Q3.6, Q3.7, Q3.8, Q3.9, Q3.10, Q3.11, Q3.12, Q3.13, Q3.17
- GNOME: GN-1.15, GN-1.31, GN-1.45, GN-1.52, GN-1.88, GN-1.122, GN-1.125, GN-1.127, GN-1.136, GN-1.137, GN-1.138, GN-1.149, GN-1.153, GN-1.157, GN-1.171, GN-1.182, GN-1.205, GN-1.225, GN-1.247, GN-1.255, GN-1.257, GN-1.260, GN-1.265, GN-1.268, GN-1.269, GN-1.270, GN-1.271, GN-1.272, GN-1.274, GN-1.275, GN-1.276, GN-2.11, GN-2.14, GN-2.19, GN-2.20, GN-2.21, GN-2.26, GN-2.31, GN-2.35, GN-2.36, GN-2.39, GN-2.45, GN-2.49, GN-2.51, GN-3.09, GN-3.12, GN-3.13, GN-3.14, GN-3.15, GN-3.16, GN-3.17, GN-3.43, GN-3.58, GN-SF01†, GN-SF02†, GN-SF10†, GN-SF12†
- Material: MAT-M10, MAT-N1, MAT-N3, MAT-N11, MAT-N29, MAT-P6, MAT-SN5, MAT-X1, MAT-X5, MAT-X17, MAT-X23
- iOS: C-45†, CB-4, D-9, D-10, D-11, D-12, D-13, SE-2, X-CO-2, X-CO-5, X-MT-6

#### C06 — Colour variant axes beyond light/dark

- macOS: MAC-B02, MAC-CB02, MAC-CB06, MAC-L03, MAC-P03, MAC-SB05, MAC-SB07, MAC-SG06, MAC-ST01, MAC-SW04, MAC-TB04, MAC-TL11†, MAC-W02, MAC-W06, MAC-W07, MAC-W15†, MAC-X02, MAC-X03, MAC-X16, MAC-X22, MAC-X27, MAC-X29
- Windows: W1-001, W1-003, W1-005, W1-008, W1-012, W1-013, W3-01, W3-06, W3-10, W3-35, W3-44, W-RL03†
- KDE: K1.9, K1.12, K1.13, K1.18, K1.22, K1.23, K1.45, K1.92, K1.150, K1.173, K1.222, K2.1, K2.11, K3.15†, K3.16†, K3.40, K3.51, K3.91, Q1.37, Q1.71, Q3.3
- GNOME: GN-1.02, GN-1.03, GN-1.04, GN-1.07, GN-1.13, GN-1.14, GN-1.16, GN-1.38, GN-1.51, GN-1.65, GN-1.73, GN-1.74, GN-1.80, GN-1.86, GN-1.93, GN-1.96, GN-1.108, GN-1.115, GN-1.125, GN-1.135, GN-1.136, GN-1.148, GN-1.153, GN-1.155, GN-1.207, GN-1.208, GN-1.209, GN-1.251, GN-1.252, GN-1.273, GN-2.11, GN-2.12, GN-2.20, GN-2.21, GN-2.26, GN-2.27, GN-3.31, GN-3.44, GN-3.61, GN-3.72†
- Material: MAT-X18, MAT-X35†
- iOS: D-14, D-15, D-16, DL-12, PO-6, SD-3, W-4, X-A11-1, X-CO-1, X-CO-3, X-MT-12, X-SH-3

#### C07 — States and state combinations

- macOS: MAC-B02, MAC-B03, MAC-B09, MAC-B10, MAC-C01, MAC-C02, MAC-C03, MAC-C04, MAC-C07, MAC-CB02, MAC-CB03, MAC-CB04, MAC-EX03, MAC-I02, MAC-I09†, MAC-L04, MAC-L05, MAC-L11, MAC-L13, MAC-L15, MAC-L17†, MAC-L18†, MAC-L19†, MAC-MN04, MAC-MN05, MAC-MN06, MAC-MN10, MAC-MN17†, MAC-N01, MAC-N03, MAC-N04, MAC-N07, MAC-N08, MAC-N10, MAC-N13, MAC-N28†, MAC-N29†, MAC-P03, MAC-S01, MAC-S02, MAC-S08, MAC-SB02, MAC-SF04†, MAC-SF07†, MAC-SG03, MAC-SG05, MAC-SG06, MAC-SG10†, MAC-SL05, MAC-SL06, MAC-SL07, MAC-SP03, MAC-SW03, MAC-TB04, MAC-TB06†, MAC-TL02, MAC-TL11†, MAC-W02, MAC-W09, MAC-W11†, MAC-W13†, MAC-X16, MAC-X19, MAC-X21, MAC-X29, MAC-X30†
- Windows: W1-008, W1-011, W1-019, W1-020, W1-021, W1-022, W1-026, W1-027, W1-028, W1-029, W1-030, W1-033, W1-037, W1-038, W1-041, W1-042, W1-044, W1-048, W1-049, W1-050, W1-061, W1-062, W1-063, W1-067, W1-068, W1-070, W1-074, W1-076, W1-080, W1-081, W1-082, W1-083, W1-087, W1-089, W1-091, W1-099, W1-100, W1-103, W1-106, W1-107, W1-109, W1-110, W1-111, W1-112, W1-118, W1-120, W1-121, W1-122, W1-130, W1-131, W1-132, W1-134, W1-138, W1-139, W1-140, W1-143, W1-144, W1-148†, W1-149†, W1-153†, W1-154†, W1-157†, W2-01, W2-02, W2-03, W2-04, W2-06, W2-10, W2-11, W2-12, W2-13, W2-14, W2-15, W2-16, W2-17, W2-18, W2-19, W2-20, W2-21, W2-22, W2-23, W2-27, W2-28, W2-29, W2-31, W2-32, W2-33, W2-34, W2-38, W2-40†, W2-42†, W2-43†, W3-02, W3-51†, W3-53†, W-RL01†, W-RL02†, W-SF09†
- KDE: K1.7, K1.16, K1.20, K1.21, K1.22, K1.24, K1.25, K1.29, K1.40, K1.43, K1.44, K1.50, K1.51, K1.52, K1.54, K1.56, K1.60, K1.61, K1.91, K1.92, K1.100, K1.101, K1.102, K1.105, K1.112, K1.121, K1.123, K1.124, K1.132, K1.133†, K1.140, K1.142†, K1.152, K1.155, K1.172, K1.174, K1.177, K1.182, K1.184†, K1.240, K1.242, K1.243, K1.244, K1.246, K1.247, K1.250, K1.260†, K2.1, K2.3, K2.4, K2.5, K2.8, K2.9, K2.16, K2.19, K2.21, K2.23†, K2.27†, K2.30†, K3.4, K3.20, K3.23, K3.50, K-RL02†, K-RL06†, K-SF18†, Q1.8, Q1.9, Q1.10, Q1.20, Q1.22, Q1.26, Q1.27, Q1.31, Q1.40, Q1.46, Q1.51, Q1.53, Q1.54, Q1.62, Q1.77, Q1.78, Q1.81, Q1.83, Q1.86, Q1.101, Q1.125, Q1.126, Q1.128, Q1.134, Q2.1, Q2.3, Q2.5, Q2.7, Q2.8, Q2.10, Q2.11, Q2.22, Q2.23, Q2.35, Q2.55†, Q3.4, Q3.13, Q3.16, Q3.30, Q3.33
- GNOME: GN-1.05, GN-1.22, GN-1.24, GN-1.25, GN-1.26, GN-1.29, GN-1.30, GN-1.31, GN-1.32, GN-1.34, GN-1.35, GN-1.41, GN-1.44, GN-1.45, GN-1.46, GN-1.53, GN-1.55, GN-1.56, GN-1.57, GN-1.58, GN-1.66, GN-1.67, GN-1.73, GN-1.78, GN-1.79, GN-1.85, GN-1.87, GN-1.88, GN-1.89, GN-1.97, GN-1.98, GN-1.99, GN-1.105, GN-1.114, GN-1.117, GN-1.120, GN-1.123, GN-1.124, GN-1.127, GN-1.133, GN-1.141, GN-1.158, GN-1.159, GN-1.160, GN-1.165, GN-1.166, GN-1.167, GN-1.171, GN-1.172, GN-1.174, GN-1.176, GN-1.198, GN-1.199, GN-1.200, GN-1.201, GN-1.231, GN-1.234, GN-1.237, GN-1.240, GN-1.241, GN-1.242, GN-1.246, GN-1.253, GN-1.257, GN-1.261, GN-1.263, GN-1.265, GN-1.266, GN-1.272, GN-1.279†, GN-1.280†, GN-2.03, GN-2.08, GN-2.22, GN-2.25, GN-2.29, GN-2.30, GN-2.31, GN-2.45, GN-2.52†, GN-3.39, GN-3.42, GN-3.60, GN-3.66†, GN-SF14†, GN-SF15†
- Material: MAT-B1, MAT-B2, MAT-B3, MAT-B4, MAT-B5, MAT-B6, MAT-B7, MAT-B10, MAT-B12, MAT-C1, MAT-C3, MAT-C4, MAT-C5, MAT-C6, MAT-C7, MAT-CA2, MAT-CB1, MAT-CB2, MAT-CB3, MAT-DG3, MAT-EX1, MAT-I1, MAT-I2, MAT-I3, MAT-I5, MAT-I7, MAT-I9, MAT-I10, MAT-I11, MAT-L3, MAT-L4, MAT-L5, MAT-L8, MAT-L9, MAT-L10, MAT-L12, MAT-L13, MAT-M1, MAT-M2, MAT-M3, MAT-N1, MAT-N2, MAT-N4, MAT-N8, MAT-N9, MAT-N12, MAT-N13, MAT-N16, MAT-N17, MAT-N26, MAT-N27, MAT-N31†, MAT-S1, MAT-S4, MAT-S5, MAT-S8, MAT-SB2, MAT-SB3, MAT-SB8, MAT-SG2, MAT-SG4, MAT-SP2, MAT-SW1, MAT-SW2, MAT-SW3, MAT-SW4, MAT-SW5, MAT-SW6, MAT-T1, MAT-TA1, MAT-TB1, MAT-TB3, MAT-TB4, MAT-W4†, MAT-X4, MAT-X8, MAT-X12, MAT-X24, MAT-X29, MAT-X33†
- iOS: B-5, B-6, B-7, B-10, B-14, C-2, C-4, C-6, C-9, C-11, C-12, C-42†, CB-3, CK-2†, D-20, I-9†, I-10†, L-10, LK-1, MN-6, MN-8, MN-18†, RL-2†, SB-4, SD-2, SD-3, SF-4†, SG-2, SG-4, SG-7, SG-8†, SG-9†, SL-3, SL-7, SP-2, SW-2, T-1, T-4, T-5, T-6, T-10, T-17†, TB-1, TB-6, X-IC-4, X-SE-3

#### C08 — State-layer model

- macOS: MAC-X30†
- KDE: K3.31, Q3.16
- Material: MAT-B2, MAT-B5, MAT-B12, MAT-C2, MAT-C5, MAT-CA2, MAT-DG3, MAT-I11, MAT-L4, MAT-L5, MAT-M2, MAT-N1, MAT-N2, MAT-N4, MAT-N9, MAT-N16, MAT-N18, MAT-N20, MAT-N22, MAT-N26, MAT-S7, MAT-SB2, MAT-SG2, MAT-SP2, MAT-SW4, MAT-TB3, MAT-X8, MAT-X9, MAT-X29, MAT-X34†

#### C09 — Widget variants/styles

- macOS: MAC-B01, MAC-B06, MAC-B07, MAC-B08, MAC-BX02, MAC-CB01, MAC-CB05, MAC-D02, MAC-EX01, MAC-I01, MAC-I08, MAC-L01, MAC-L06, MAC-L12, MAC-MN13, MAC-MN17†, MAC-N03, MAC-N04, MAC-N05, MAC-N06, MAC-N07, MAC-N11, MAC-N12, MAC-N23, MAC-N28†, MAC-N29†, MAC-P01, MAC-S03, MAC-SG04, MAC-SG05, MAC-SN04, MAC-SP01, MAC-TB03, MAC-TL03, MAC-TL04, MAC-TL07, MAC-W14†, MAC-X04, MAC-X05, MAC-X35†, MAC-X36†
- Windows: W1-020, W1-021, W1-060, W1-137, W1-153†, W1-154†, W1-155†, W3-43, W-RL02†
- KDE: K1.14, K1.24, K1.25, K1.26, K1.41, K1.42, K1.47†, K1.56, K1.62, K1.84†, K1.94, K1.105, K1.115†, K1.120, K1.122, K1.123, K1.152, K1.155, K1.180, K1.240, K1.241, K1.247, K2.6, K2.8, K2.18, K2.21, K3.20, K3.21, K3.44, Q1.8, Q1.9, Q1.11, Q1.41, Q1.43, Q1.45, Q1.76, Q1.95, Q1.107, Q1.115, Q1.118, Q1.123, Q2.4, Q2.5, Q2.6, Q2.18, Q2.20, Q2.56†
- GNOME: GN-1.17, GN-1.28, GN-1.29, GN-1.30, GN-1.31, GN-1.32, GN-1.33, GN-1.34, GN-1.35, GN-1.42, GN-1.47, GN-1.54, GN-1.62, GN-1.63, GN-1.112, GN-1.125, GN-1.149, GN-1.151, GN-1.154, GN-1.162, GN-1.164, GN-1.165, GN-1.166, GN-1.170, GN-1.184, GN-1.188, GN-1.205, GN-1.211, GN-1.212, GN-1.213, GN-1.215, GN-1.222, GN-1.224, GN-1.236, GN-1.245, GN-1.246, GN-1.247, GN-1.256, GN-1.266, GN-2.05, GN-2.08, GN-2.20, GN-2.21, GN-2.24, GN-2.29, GN-2.33, GN-2.50, GN-2.57†, GN-3.09, GN-3.11, GN-3.17, GN-RL01†
- Material: MAT-B1, MAT-B4, MAT-B10, MAT-C1, MAT-CA1, MAT-CB1, MAT-D1, MAT-I11, MAT-L8, MAT-M10, MAT-N1, MAT-N2, MAT-N3, MAT-N4, MAT-N8, MAT-N9, MAT-N23, MAT-N24, MAT-N25, MAT-N27, MAT-P1, MAT-P4, MAT-S12, MAT-SB6, MAT-SN2, MAT-SN3, MAT-T1, MAT-TA3, MAT-TA6, MAT-TB1, MAT-TS4, MAT-X34†
- iOS: B-1, B-2, B-6, B-11, B-12, B-13, B-14, B-18†, C-3, C-5, C-17, C-20, C-22, C-43†, CB-2, DL-7, EX-3, I-1, L-1, L-4, L-13, L-23†, MN-7, MN-12, MN-16, PB-1, SB-1, SG-8†, SL-8, T-3, T-13, T-16, TA-1, TB-4, X-A11-5

#### C10 — Size variants and density

- macOS: MAC-B08, MAC-B11†, MAC-EX05†, MAC-L08, MAC-LY01, MAC-N01, MAC-N10, MAC-N16, MAC-RL02†, MAC-S07, MAC-SB01, MAC-SB09†, MAC-SN03, MAC-SW05, MAC-TB05, MAC-TL01, MAC-X17
- Windows: W1-006, W1-040, W1-047, W1-060, W1-066, W1-085, W1-092, W1-094, W1-099, W1-103, W1-117, W1-122, W1-127, W1-134, W1-148†, W2-20, W3-36, W3-39, W3-40
- KDE: K1.4, K1.28, K1.115†, K1.123, K1.130, K1.154, K1.158, K1.180, K1.181, K1.224†, K3.63, K3.83, K3.84†, Q1.1, Q1.12, Q1.25, Q1.28, Q1.29, Q1.52, Q1.59, Q1.60, Q1.69, Q1.72, Q1.80, Q1.97, Q1.99, Q1.117, Q1.120, Q1.124, Q2.2, Q2.9, Q2.28, Q2.33, Q2.34, Q2.50, Q2.53†, Q2.58†, Q3.29, Q3.31
- GNOME: GN-1.11, GN-1.21, GN-1.28, GN-1.34, GN-1.70, GN-1.78, GN-1.85, GN-1.92, GN-1.111, GN-1.112, GN-1.119, GN-1.120, GN-1.124, GN-1.128, GN-1.129, GN-1.130, GN-1.140, GN-1.147, GN-1.150, GN-1.164, GN-1.180, GN-1.185, GN-1.188, GN-1.195, GN-1.197, GN-1.212, GN-1.216, GN-1.217, GN-1.218, GN-1.219, GN-1.224, GN-1.227, GN-1.228, GN-1.232, GN-1.238, GN-1.244, GN-1.248, GN-2.05, GN-2.06, GN-2.10, GN-2.11, GN-2.13, GN-2.14, GN-2.15, GN-2.19, GN-2.24, GN-2.26, GN-2.33, GN-2.35, GN-2.36, GN-2.37, GN-2.39, GN-2.41, GN-2.44, GN-3.59, GN-3.63, GN-SF17†
- Material: MAT-B9, MAT-C9, MAT-L1, MAT-M8, MAT-M10, MAT-N1, MAT-N2, MAT-N5, MAT-N6, MAT-N7, MAT-N9, MAT-N10, MAT-N11, MAT-N13, MAT-N16, MAT-N17, MAT-N20, MAT-N22, MAT-N23, MAT-N25, MAT-N28, MAT-P4, MAT-S1, MAT-S3, MAT-S11, MAT-SF02†, MAT-SN3, MAT-SW3, MAT-T5, MAT-TA3, MAT-X22, MAT-X31
- iOS: B-3, C-1, C-3, DL-8, DL-13†, I-10†, L-9, L-22†, LY-1, MN-15, PO-7†, SB-7†, SD-8, SD-10†, SF-7†, SF-8†, SG-9†, SP-3a, T-14, TB-3, X-DT-2, X-SV-1, X-SV-2

#### C11 — Shape

- macOS: MAC-B08, MAC-D03, MAC-L01, MAC-L02, MAC-L11, MAC-L19†, MAC-MN01, MAC-N02, MAC-N03, MAC-N13, MAC-N21, MAC-N31†, MAC-N34†, MAC-N36†, MAC-P02, MAC-S04, MAC-SB04, MAC-SB06, MAC-SG08, MAC-SL01, MAC-SL03, MAC-SL05, MAC-SW01, MAC-W05, MAC-X04, MAC-X07
- Windows: W1-004, W1-014, W1-045, W1-063, W1-069, W1-077, W1-089, W1-092, W1-096, W1-097, W1-099, W1-107, W1-108, W1-109, W1-120, W1-133, W1-134, W1-141, W1-157†, W2-05, W2-08, W2-09, W2-23, W2-24, W2-28, W2-30, W2-42†, W2-43†, W3-20, W-SF02†, W-SF03†
- KDE: K1.10, K1.26, K1.53, K1.64, K1.82, K1.90, K1.104, K1.121, K1.128, K1.142†, K1.170, K1.191, K2.11, K2.12, K2.13, K-RL01†, Q1.16, Q1.26, Q1.45, Q1.79, Q1.107, Q1.129, Q2.4, Q2.12, Q2.17, Q2.18, Q2.23, Q2.27, Q2.29, Q2.35, Q2.39, Q2.44, Q2.49, Q2.55†, Q3.20
- GNOME: GN-1.05, GN-1.06, GN-1.22, GN-1.28, GN-1.42, GN-1.47, GN-1.60, GN-1.61, GN-1.62, GN-1.63, GN-1.68, GN-1.79, GN-1.84, GN-1.94, GN-1.101, GN-1.109, GN-1.124, GN-1.130, GN-1.140, GN-1.143, GN-1.149, GN-1.165, GN-1.174, GN-1.177, GN-1.183, GN-1.211, GN-1.212, GN-1.223, GN-1.224, GN-1.239, GN-1.245, GN-1.261, GN-2.12, GN-2.17, GN-2.25, GN-2.26, GN-2.27, GN-2.29, GN-2.30, GN-2.32, GN-2.35, GN-2.39, GN-2.40, GN-2.41, GN-2.42, GN-2.44, GN-2.50, GN-2.51, GN-3.29, GN-SF13†
- Material: MAT-B10, MAT-B11, MAT-C2, MAT-EX1, MAT-FM03†, MAT-I11, MAT-I17†, MAT-L8, MAT-L13, MAT-M10, MAT-N1, MAT-N3, MAT-N5, MAT-N6, MAT-N7, MAT-N8, MAT-N9, MAT-N10, MAT-N11, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N16, MAT-N17, MAT-N18, MAT-N19, MAT-N20, MAT-N21, MAT-N22, MAT-N23, MAT-N24, MAT-N25, MAT-N26, MAT-N30†, MAT-N31†, MAT-P5, MAT-S1, MAT-S3, MAT-S11, MAT-SB1, MAT-SB7, MAT-SG3, MAT-SN4, MAT-SP1, MAT-T1, MAT-TB2, MAT-W5†, MAT-X10, MAT-X11, MAT-X12, MAT-X37†
- iOS: B-4, C-3, C-11, C-12, C-13, C-21, C-24, C-26, C-30, C-32, C-36†, CK-2†, L-2, MN-17†, PB-2, RL-1†, SB-5, SD-2, SD-6, SG-1, SL-2, SL-4, SW-3, T-9, TA-1, TB-5, TT-1, W-6, W-10†, W-11†, X-CR-1, X-CR-2, X-CR-3, X-CR-4

#### C12 — Border/outline model

- macOS: MAC-B05, MAC-C05, MAC-I04, MAC-I08, MAC-L05, MAC-L06, MAC-L11, MAC-L16, MAC-L19†, MAC-MN14, MAC-N12, MAC-N31†, MAC-P05†, MAC-S03, MAC-S05, MAC-SL02, MAC-ST01, MAC-TB01, MAC-TB06†, MAC-W04, MAC-W06, MAC-X08, MAC-X38†
- Windows: W1-001, W1-002, W1-018, W1-019, W1-020, W1-026, W1-027, W1-042, W1-046, W1-062, W1-069, W1-079, W1-086, W1-091, W1-097, W1-103, W1-126, W1-131, W1-133, W1-138, W1-141, W1-144, W2-13, W2-30, W2-33, W3-04, W3-43, W-SF02†, W-SF03†, W-SF04†, W-SF06†
- KDE: K1.9, K1.11, K1.20, K1.22, K1.40, K1.42, K1.58†, K1.60, K1.90, K1.100, K1.111, K1.128, K1.140, K1.151, K1.160, K1.171, K1.178, K1.222, K1.243, K2.8, K2.9, K2.10, K2.12, K2.13, K2.16, K2.21, K2.25†, K2.28†, K2.29†, K2.31†, K-RL05†, K-SF16†, K-SF18†, Q1.16, Q1.26, Q1.38, Q1.46, Q1.54, Q1.55, Q1.67, Q1.68, Q1.71, Q1.73, Q1.93, Q1.96, Q1.101, Q1.107, Q1.109, Q1.112, Q1.114, Q1.115, Q1.121, Q1.129, Q2.5, Q2.12, Q2.13, Q2.16, Q2.18, Q2.23, Q2.26, Q2.29, Q2.35, Q3.11, Q3.14, Q3.15, Q3.20
- GNOME: GN-1.03, GN-1.04, GN-1.05, GN-1.07, GN-1.08, GN-1.13, GN-1.25, GN-1.38, GN-1.42, GN-1.46, GN-1.51, GN-1.56, GN-1.59, GN-1.65, GN-1.73, GN-1.80, GN-1.86, GN-1.96, GN-1.105, GN-1.108, GN-1.115, GN-1.116, GN-1.133, GN-1.135, GN-1.139, GN-1.146, GN-1.148, GN-1.154, GN-1.157, GN-1.162, GN-1.165, GN-1.169, GN-1.172, GN-1.175, GN-1.180, GN-1.183, GN-1.184, GN-1.186, GN-1.209, GN-1.222, GN-1.251, GN-1.252, GN-1.279†, GN-1.280†, GN-2.17, GN-2.20, GN-2.21, GN-2.24, GN-2.25, GN-2.27, GN-2.30, GN-2.32, GN-2.40, GN-3.02, GN-3.03, GN-3.61, GN-3.66†
- Material: MAT-B1, MAT-B3, MAT-C3, MAT-C4, MAT-CA1, MAT-CA2, MAT-DG4, MAT-I1, MAT-I2, MAT-I4, MAT-I11, MAT-L13, MAT-N4, MAT-N9, MAT-N15, MAT-N18, MAT-N26, MAT-S10, MAT-SG4, MAT-SW5, MAT-TB5, MAT-X34†
- iOS: C-47†, CD-1, DL-5, I-1, SB-5, SE-1, SG-1, SG-3, SP-1, SW-6, T-2, TA-1, TB-2, X-HL-1

#### C13 — Parts missing from a widget the model has

- macOS: MAC-B09, MAC-BX01, MAC-C01, MAC-C06, MAC-C07, MAC-CB01, MAC-CB04, MAC-D01, MAC-D02, MAC-EX01, MAC-EX02, MAC-EX05†, MAC-I03, MAC-I06, MAC-I09†, MAC-IC04†, MAC-L05, MAC-L09, MAC-L10, MAC-L12, MAC-L18†, MAC-LK01, MAC-MN01, MAC-MN03, MAC-MN04, MAC-MN05, MAC-MN06, MAC-MN07, MAC-MN08, MAC-MN09, MAC-MN12, MAC-MN13, MAC-N01, MAC-N02, MAC-N03, MAC-N04, MAC-N05, MAC-N06, MAC-N07, MAC-N08, MAC-N09, MAC-N10, MAC-N14, MAC-N15, MAC-N17, MAC-N18, MAC-N19, MAC-N21, MAC-N24, MAC-N27†, MAC-N28†, MAC-N29†, MAC-N30†, MAC-N32†, MAC-N33†, MAC-N35†, MAC-N36†, MAC-N37†, MAC-P01, MAC-PO01, MAC-RL02†, MAC-S02, MAC-S05, MAC-S06, MAC-SB03, MAC-SB08, MAC-SF01†, MAC-SG01, MAC-SG07, MAC-SG10†, MAC-SL01, MAC-SL04, MAC-SN01, MAC-SP02, MAC-SP04, MAC-SW01, MAC-SW06, MAC-T06†, MAC-TB01, MAC-TB06†, MAC-TB07†, MAC-TL02, MAC-TL05, MAC-TL06, MAC-TL08, MAC-TL11†, MAC-W03, MAC-W09, MAC-W11†, MAC-X14, MAC-X21, MAC-X28
- Windows: W1-007, W1-010, W1-011, W1-015, W1-024, W1-030, W1-031, W1-034, W1-036, W1-038, W1-039, W1-041, W1-043, W1-048, W1-049, W1-050, W1-051, W1-060, W1-061, W1-065, W1-067, W1-071, W1-072, W1-073, W1-075, W1-077, W1-081, W1-082, W1-083, W1-084, W1-089, W1-092, W1-093, W1-095, W1-099, W1-100, W1-101, W1-102, W1-105, W1-107, W1-109, W1-110, W1-118, W1-124, W1-130, W1-132, W1-135, W1-137, W1-140, W1-142, W1-148†, W1-150†, W1-151†, W1-153†, W1-154†, W1-156†, W1-157†, W1-159†, W2-01, W2-02, W2-03, W2-04, W2-05, W2-06, W2-07, W2-08, W2-09, W2-10, W2-11, W2-12, W2-13, W2-14, W2-15, W2-16, W2-17, W2-18, W2-20, W2-21, W2-23, W2-24, W2-25, W2-26, W2-27, W2-28, W2-31, W2-34, W2-35, W2-36, W2-37, W2-38, W2-39, W2-40†, W2-41†, W2-42†, W2-43†, W3-26, W3-27, W3-48†, W3-49†, W-RL01†, W-RL02†, W-RL03†, W-SF04†, W-SF05†
- KDE: K1.4, K1.6, K1.7, K1.27, K1.43, K1.46†, K1.48†, K1.49†, K1.50, K1.52, K1.55, K1.58†, K1.61, K1.66, K1.67, K1.68, K1.69, K1.72, K1.84†, K1.93, K1.94, K1.96, K1.103, K1.110, K1.112, K1.113, K1.115†, K1.120, K1.124, K1.125, K1.126, K1.127, K1.142†, K1.151, K1.153, K1.155, K1.156, K1.157, K1.160, K1.176, K1.178, K1.179, K1.182, K1.184†, K1.200, K1.201, K1.211, K1.221, K1.222, K1.224†, K1.245, K1.246, K1.250, K2.3, K2.4, K2.5, K2.7, K2.8, K2.9, K2.10, K2.11, K2.14, K2.16, K2.17, K2.19, K2.20, K2.22, K2.23†, K2.24†, K2.25†, K2.26†, K2.27†, K2.28†, K2.29†, K2.32†, K3.44, K-RL02†, K-RL05†, K-SF04†, K-SF05†, K-SF06†, K-SF09†, K-SF15†, Q1.9, Q1.10, Q1.14, Q1.15, Q1.16, Q1.17, Q1.18, Q1.19, Q1.20, Q1.29, Q1.30, Q1.31, Q1.32, Q1.39, Q1.44, Q1.45, Q1.48, Q1.50, Q1.61, Q1.62, Q1.63, Q1.65, Q1.70, Q1.73, Q1.75, Q1.76, Q1.77, Q1.82, Q1.83, Q1.85, Q1.86, Q1.88, Q1.92, Q1.99, Q1.100, Q1.109, Q1.110, Q1.114, Q1.116, Q1.118, Q1.119, Q1.121, Q1.122, Q1.131, Q1.132, Q1.136, Q2.1, Q2.11, Q2.13, Q2.19, Q2.20, Q2.24, Q2.29, Q2.30, Q2.32, Q2.34, Q2.38, Q2.39, Q2.41, Q2.42, Q2.43, Q2.44, Q2.48, Q2.53†, Q2.54†, Q2.55†, Q2.56†, Q2.57†, Q2.58†
- GNOME: GN-1.10, GN-1.18, GN-1.27, GN-1.39, GN-1.40, GN-1.41, GN-1.43, GN-1.44, GN-1.48, GN-1.58, GN-1.61, GN-1.72, GN-1.73, GN-1.74, GN-1.75, GN-1.78, GN-1.81, GN-1.100, GN-1.101, GN-1.102, GN-1.103, GN-1.110, GN-1.116, GN-1.117, GN-1.118, GN-1.119, GN-1.120, GN-1.125, GN-1.126, GN-1.127, GN-1.128, GN-1.129, GN-1.130, GN-1.142, GN-1.144, GN-1.146, GN-1.156, GN-1.163, GN-1.168, GN-1.173, GN-1.178, GN-1.180, GN-1.186, GN-1.202, GN-1.226, GN-1.227, GN-1.230, GN-1.232, GN-1.233, GN-1.234, GN-1.235, GN-1.243, GN-1.261, GN-1.267, GN-1.278†, GN-1.279†, GN-1.281†, GN-1.285†, GN-1.286†, GN-2.01, GN-2.03, GN-2.06, GN-2.09, GN-2.13, GN-2.15, GN-2.16, GN-2.17, GN-2.21, GN-2.22, GN-2.23, GN-2.24, GN-2.25, GN-2.26, GN-2.28, GN-2.29, GN-2.31, GN-2.32, GN-2.33, GN-2.34, GN-2.35, GN-2.36, GN-2.37, GN-2.38, GN-2.39, GN-2.42, GN-2.43, GN-2.48, GN-2.52†, GN-2.53†, GN-2.54†, GN-2.55†, GN-2.58†, GN-3.06, GN-3.62, GN-SF05†, GN-SF06†, GN-SF09†, GN-SF11†, GN-SF13†, GN-SF15†
- Material: MAT-B7, MAT-B9, MAT-B12, MAT-C1, MAT-C2, MAT-C3, MAT-C5, MAT-C6, MAT-CA3, MAT-CB1, MAT-CB2, MAT-CB3, MAT-DG2, MAT-DG3, MAT-DG4, MAT-DG5, MAT-DG7, MAT-EX1, MAT-I3, MAT-I4, MAT-I5, MAT-I6, MAT-I7, MAT-I8, MAT-I9, MAT-I11, MAT-I13, MAT-I17†, MAT-I18†, MAT-I19†, MAT-L1, MAT-L2, MAT-L3, MAT-L6, MAT-L9, MAT-L10, MAT-L11, MAT-L12, MAT-M3, MAT-M4, MAT-M5, MAT-M6, MAT-M10, MAT-N2, MAT-N3, MAT-N4, MAT-N7, MAT-N8, MAT-N9, MAT-N10, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N16, MAT-N17, MAT-N19, MAT-N20, MAT-N22, MAT-N23, MAT-N24, MAT-N25, MAT-N26, MAT-N27, MAT-N28, MAT-N30†, MAT-N31†, MAT-P1, MAT-P2, MAT-P3, MAT-P5, MAT-P7, MAT-S1, MAT-S2, MAT-S3, MAT-S4, MAT-S5, MAT-S6, MAT-S7, MAT-S10, MAT-S11, MAT-SB1, MAT-SB3, MAT-SB4, MAT-SB5, MAT-SB6, MAT-SC3†, MAT-SC4†, MAT-SG1, MAT-SN1, MAT-SN2, MAT-SN4, MAT-SN5, MAT-SP1, MAT-SW1, MAT-SW2, MAT-SW3, MAT-SW4, MAT-SW7, MAT-T1, MAT-T4, MAT-TA2, MAT-TA4, MAT-TA5, MAT-TA6, MAT-TA8†, MAT-TB1, MAT-TB2, MAT-TB4, MAT-TB5, MAT-W4†, MAT-X27
- iOS: B-8, B-9, B-10, B-12, B-18†, C-1, C-2, C-3, C-4, C-6, C-7, C-9, C-10, C-11, C-12, C-14, C-15, C-16, C-24, C-25, C-26, C-28, C-29, C-30, C-36†, C-37†, C-38†, C-42†, C-44†, C-45†, C-46†, CB-1, CK-1, CK-2†, DL-2, DL-5, DL-9, EX-1, EX-4, I-2, I-3, I-6, I-9†, L-3, L-5, L-7, L-8, L-12, L-13, L-15, L-17, L-18, L-21†, L-22†, L-23†, MN-4, MN-5, MN-6, MN-9, MN-10, MN-11, MN-12, MN-13, MN-17†, PB-4†, PO-1, RL-2†, SB-4, SD-4, SD-5, SF-1†, SG-1, SG-3, SG-6, SL-2, SL-5, SL-6, SP-1a, SW-3, SW-4, SW-7, T-3, T-5, T-7, T-8, T-11, T-12, T-17†, TA-5, TB-5, TB-7, W-10†, X-A11-6, X-IC-5, X-SE-2

#### C14 — Motion

- macOS: MAC-D04, MAC-EX04, MAC-IC05†, MAC-N02, MAC-P01, MAC-P04, MAC-PO05, MAC-SF07†, MAC-SG09, MAC-SN02, MAC-SN05†, MAC-T04, MAC-TL07, MAC-W12†, MAC-X04, MAC-X11, MAC-X25, MAC-X32†
- Windows: W1-009, W1-014, W1-017, W1-023, W1-024, W1-035, W1-039, W1-041, W1-054, W1-058, W1-061, W1-064, W1-065, W1-068, W1-075, W1-078, W1-087, W1-090, W1-093, W1-098, W1-104, W1-107, W1-110, W1-113, W1-116, W1-120, W1-123, W1-128, W1-130, W1-132, W1-135, W1-136, W1-138, W1-142, W2-06, W2-09, W2-12, W2-18, W2-23, W2-26, W2-27, W2-29, W2-30, W2-32, W2-34, W3-07, W3-10, W3-28, W3-30, W3-31, W3-32, W3-33, W3-34, W3-37, W3-49†, W3-52†, W3-53†, W-RL02†, W-SF09†
- KDE: K1.8, K1.17, K1.19†, K1.30, K1.44, K1.46†, K1.52, K1.55, K1.81, K1.95, K1.106, K1.110, K1.129, K1.132, K1.133†, K1.142†, K1.177, K1.185†, K1.223, K1.230, K1.240, K1.251, K2.3, K2.9, K2.17, K3.14, K3.15†, K3.16†, K3.24, K3.30, K3.31, K3.33, K3.34, K3.35, K3.36, K3.37, K3.38†, K3.39†, K3.66†, K3.90, K3.96†, K-SF07†, Q1.18, Q1.24, Q1.34, Q1.40, Q1.41, Q1.48, Q1.55, Q1.56, Q1.57, Q1.58, Q1.59, Q1.61, Q1.64, Q1.67, Q1.74, Q1.84, Q1.88, Q1.89, Q1.91, Q1.94, Q1.102, Q1.104, Q1.105, Q1.111, Q1.119, Q1.120, Q1.125, Q2.3, Q2.10, Q2.11, Q2.14, Q2.15, Q2.21, Q2.35, Q2.38, Q2.40, Q2.43, Q2.52, Q2.54†, Q2.55†, Q2.56†, Q3.19, Q3.22, Q3.23, Q3.24, Q3.25, Q3.26, Q3.27
- GNOME: GN-1.02, GN-1.16, GN-1.22, GN-1.36, GN-1.41, GN-1.50, GN-1.64, GN-1.76, GN-1.90, GN-1.91, GN-1.106, GN-1.113, GN-1.126, GN-1.128, GN-1.130, GN-1.131, GN-1.136, GN-1.155, GN-1.165, GN-1.203, GN-1.204, GN-1.220, GN-1.221, GN-1.229, GN-1.230, GN-1.243, GN-1.249, GN-1.253, GN-1.258, GN-1.259, GN-2.03, GN-2.04, GN-2.11, GN-2.12, GN-2.15, GN-2.17, GN-2.18, GN-2.19, GN-2.35, GN-2.38, GN-2.54†, GN-3.18, GN-3.19, GN-3.20, GN-3.21, GN-3.22, GN-3.23, GN-3.24, GN-3.25, GN-3.28, GN-3.35, GN-3.44, GN-3.71†, GN-3.72†, GN-SF14†
- Material: MAT-B11, MAT-B13, MAT-C8, MAT-CB3, MAT-DG6, MAT-I14, MAT-M9, MAT-N3, MAT-N5, MAT-N6, MAT-N7, MAT-N12, MAT-N14, MAT-N18, MAT-N19, MAT-N22, MAT-N24, MAT-N25, MAT-N28, MAT-N31†, MAT-P6, MAT-S6, MAT-SB9, MAT-SC2, MAT-SC3†, MAT-SC4†, MAT-SG1, MAT-SN6, MAT-SW8, MAT-T3, MAT-TA1, MAT-TB6, MAT-W2, MAT-X7, MAT-X9, MAT-X10, MAT-X12, MAT-X13, MAT-X14, MAT-X20
- iOS: B-16, C-1, C-2, C-7, C-9, C-10, C-11, C-12, C-13, C-21, C-22, C-23, C-24, C-33, C-37†, C-41†, C-42†, C-48†, C-49†, CK-2†, DL-10, DL-11, EX-2, L-11, L-14, L-16, MN-11, MN-14, PB-3, PO-5, RL-1†, RL-2†, SB-3, SB-7†, SD-7, SD-9†, SG-4, SG-5, SG-8†, SL-9, SP-2a, SP-3, SP-5a, SW-5, T-8, T-10, T-15, T-17†, TA-2, TA-4, TB-6, TT-2, W-11†, X-A11-3, X-A11-9, X-FO-2, X-IC-6, X-MO-1, X-MO-2, X-MO-3, X-MO-4, X-MO-5, X-MO-6, X-MO-7, X-MO-8, X-MO-9, X-MO-10, X-MO-11†, X-MT-8, X-MT-9

#### C15 — Focus indication

- macOS: MAC-L13, MAC-L14, MAC-LK03, MAC-N02, MAC-X13, MAC-X31†
- Windows: W1-025, W1-026, W1-035, W1-111, W1-145, W3-19, W3-20, W3-21
- KDE: K1.21, K1.57, K1.62, K1.106, K1.129, K1.175, K2.8, K3.20, K3.21, K3.22, K3.23, K3.24, Q1.21, Q1.51, Q1.135, Q2.36, Q3.12, Q3.30
- GNOME: GN-1.30, GN-1.34, GN-1.35, GN-1.37, GN-1.45, GN-1.50, GN-1.61, GN-1.104, GN-1.134, GN-1.161, GN-1.165, GN-1.172, GN-1.174, GN-1.206, GN-1.247, GN-1.250, GN-1.254, GN-1.264, GN-2.03, GN-2.08, GN-2.25, GN-2.27, GN-2.40, GN-2.53†, GN-3.27, GN-3.28, GN-3.29, GN-3.30, GN-3.31
- Material: MAT-N1, MAT-N4, MAT-N9, MAT-N19, MAT-SW9, MAT-X10, MAT-X34†
- iOS: D-18, X-FO-1, X-FO-2, X-FO-3†

#### C16 — Text caret

- macOS: MAC-X12, MAC-X14
- Windows: W1-034, W3-24
- KDE: K1.47†, K3.41, K3.42, K3.43, K3.48†, Q1.14, Q3.31
- GNOME: GN-1.49, GN-1.53, GN-3.32, GN-3.33, GN-3.34, GN-3.35, GN-3.36, GN-3.37, GN-3.38, GN-3.39, GN-3.73†, GN-SF03†
- Material: MAT-I9, MAT-I12
- iOS: X-CA-1, X-CA-2, X-CA-3

#### C17 — Selection and highlights

- macOS: MAC-I05, MAC-L03, MAC-L04, MAC-L19†, MAC-N13, MAC-SB05, MAC-SF01†, MAC-SF07†, MAC-X15, MAC-X21, MAC-X28
- Windows: W1-032, W1-151†, W1-159†, W3-25, W3-49†, W-SF09†
- KDE: K1.45, K1.49†, K1.173, K1.174, K2.30†, K3.40, K3.43, K3.48†, K3.62, K-SF02†, K-SF03†, K-SF07†, K-SF08†, K-SF11†, K-SF18†, Q1.13, Q1.14
- GNOME: GN-1.175, GN-1.177, GN-1.279†, GN-1.281†, GN-3.40, GN-3.41, GN-3.42, GN-3.43, GN-3.66†, GN-SF02†, GN-SF04†, GN-SF07†, GN-SF08†, GN-SF14†, GN-SF16†
- Material: MAT-I17†, MAT-I18†
- iOS: C-25, C-47†, I-4, I-5, SF-1†, T-17†, TA-2, TA-3, X-SE-1, X-SE-2, X-SE-3

#### C18 — Typography

- macOS: MAC-BX01, MAC-D01, MAC-FM01†, MAC-FM02†, MAC-I07, MAC-MN04, MAC-MN07, MAC-MN09, MAC-MN15, MAC-N16, MAC-N17, MAC-N20, MAC-N21, MAC-N32†, MAC-N35†, MAC-N36†, MAC-N37†, MAC-SB01, MAC-SB03, MAC-SF03†, MAC-SF05†, MAC-SF06†, MAC-SG03, MAC-TB05, MAC-TS01, MAC-TS02, MAC-TS03†, MAC-W03, MAC-X18
- Windows: W1-031, W1-048, W1-072, W1-085, W1-093, W1-124, W1-147, W1-150†, W1-156†, W2-02, W2-07, W2-10, W2-24, W2-25, W2-32, W2-37, W3-22, W3-23, W3-26, W3-36, W3-47†, W-FM01†, W-FM02†, W-RL02†, W-RL03†, W-SF01†, W-SF02†, W-SF03†, W-SF07†
- KDE: K1.66, K1.270, K1.271, K1.272, K2.7, K2.8, K2.15, K2.23†, K2.27†, K2.32†, K3.45, K3.46, K3.47, K-FM01†, K-FM02†, K-RL03†, K-SF10†, K-SF12†, K-SF13†, Q1.2, Q1.3, Q1.4, Q1.5, Q1.62, Q1.63, Q1.66, Q1.72, Q1.87, Q1.114, Q1.131, Q1.135, Q2.27, Q2.30, Q2.34, Q2.38, Q2.41, Q2.42, Q2.47, Q2.55†, Q2.56†, Q2.57†, Q3.32
- GNOME: GN-1.18, GN-1.19, GN-1.39, GN-1.75, GN-1.102, GN-1.110, GN-1.163, GN-1.167, GN-1.189, GN-1.190, GN-1.191, GN-1.192, GN-1.193, GN-1.194, GN-1.283†, GN-2.01, GN-2.02, GN-2.13, GN-2.14, GN-2.24, GN-2.25, GN-2.28, GN-2.36, GN-2.37, GN-2.40, GN-2.41, GN-3.56, GN-FM01†, GN-FM02†, GN-SF05†, GN-SF07†, GN-SF10†
- Material: MAT-B9, MAT-B15, MAT-DG3, MAT-DG7, MAT-DG9, MAT-FM01†, MAT-FM02†, MAT-I3, MAT-I5, MAT-I15, MAT-L1, MAT-L14, MAT-M4, MAT-M11, MAT-N2, MAT-N10, MAT-N12, MAT-N14, MAT-N16, MAT-N17, MAT-N20, MAT-N22, MAT-N23, MAT-N31†, MAT-S6, MAT-SB4, MAT-SB5, MAT-SB8, MAT-T7, MAT-TA3, MAT-TA4, MAT-TA7, MAT-TB8, MAT-TS1, MAT-TS2, MAT-TS3, MAT-TS4, MAT-TS5, MAT-X16, MAT-X19, MAT-X36†
- iOS: B-13, C-1, C-11, C-14, C-16, C-32, C-36†, C-45†, D-1, D-2, D-3, D-4, D-5, D-6, D-7, D-8, DL-7, FM-1†, FM-2†, I-7, L-4, L-5, PB-4†, RL-1†, SD-4, SF-3†, SF-5†, SF-6†, SG-2, SW-7, TS-1, TS-2, TS-3, TS-4, X-A11-4, X-A11-10, X-DT-1, X-DT-4, X-DT-5

#### C19 — Icons

- macOS: MAC-B09, MAC-IC01, MAC-IC02, MAC-IC03, MAC-IC04†, MAC-IC05†, MAC-MN10, MAC-N20, MAC-SB01, MAC-SB02, MAC-SG10†, MAC-TL02, MAC-X19
- Windows: W1-024, W1-039, W2-40†, W3-27, W3-28, W3-51†, W3-54†
- KDE: K1.6, K1.28, K1.43, K1.46†, K1.48†, K1.70, K1.130, K1.142†, K1.157, K1.158, K1.181, K1.221, K1.224†, K1.230, K2.10, K2.13, K2.14, K2.15, K2.19, K3.60, K3.61, K3.62, K3.63, K-FM03†, K-RL02†, Q1.1, Q1.15, Q1.18, Q1.19, Q1.27, Q1.73, Q1.75, Q1.80, Q1.87, Q1.88, Q1.110, Q1.114, Q1.116, Q1.119, Q1.136, Q2.4, Q2.19, Q2.24, Q2.27, Q2.29, Q2.30, Q2.43, Q2.44, Q2.48, Q2.53†, Q2.57†, Q3.17, Q3.18, Q3.19
- GNOME: GN-1.40, GN-1.74, GN-1.119, GN-1.168, GN-1.173, GN-1.202, GN-1.278†, GN-1.285†, GN-3.07, GN-3.57, GN-3.58, GN-3.59, GN-3.60, GN-FM03†, GN-SF15†
- Material: MAT-B7, MAT-FM04†, MAT-M3, MAT-X27
- iOS: B-16, C-29, CB-1, D-19, SD-5, SP-4a, T-5, X-IC-1, X-IC-2, X-IC-3, X-IC-4, X-IC-5, X-IC-6, X-MO-9

#### C20 — Window chrome

- macOS: MAC-N14, MAC-N16, MAC-PO04, MAC-W01, MAC-W02, MAC-W03, MAC-W04, MAC-W05, MAC-W06, MAC-W07, MAC-W09, MAC-W10, MAC-W11†, MAC-W12†, MAC-W13†, MAC-W14†, MAC-W15†, MAC-X32†, MAC-X33†
- Windows: W1-001, W1-002, W1-003, W1-004, W1-006, W1-007, W1-008, W1-009, W1-010, W1-011, W1-012, W1-013, W1-014, W1-015, W1-016, W1-017, W1-148†, W1-149†, W3-44, W-RL03†, W-SF11†
- KDE: K1.1, K1.2, K1.3, K1.4, K1.5, K1.6, K1.7, K1.8, K1.9, K1.10, K1.11, K1.12, K1.13, K1.14, K1.15, K1.16, K1.17, K1.18, K1.19†, K2.11, K3.1, K3.15†, K3.16†, K3.33, K3.39†
- GNOME: GN-1.01, GN-1.02, GN-1.03, GN-1.04, GN-1.05, GN-1.06, GN-1.07, GN-1.08, GN-1.09, GN-1.10, GN-1.11, GN-1.12, GN-1.13, GN-1.14, GN-1.15, GN-1.16, GN-1.17, GN-1.19, GN-1.20, GN-1.21, GN-1.22, GN-1.23, GN-1.278†, GN-1.282†, GN-3.44, GN-3.71†, GN-3.72†
- Material: MAT-W1, MAT-W2, MAT-W4†, MAT-W5†
- iOS: C-37†, W-1, W-2, W-3, W-4, W-5, W-6, W-9, W-10†, W-11†, X-CR-4

#### C21 — Visible timings

- macOS: MAC-N01, MAC-N30†, MAC-T04, MAC-T05, MAC-T06†, MAC-X12
- Windows: W1-014, W1-054, W1-059, W1-064, W1-149†, W1-152†, W2-19, W2-26, W2-35, W3-41, W3-52†
- KDE: K1.48†, K1.73, K1.80, K1.83†, K1.84†, K1.133†, K3.36, K3.65†, K3.66†, K3.70, K3.71, K3.72, K3.73, K3.76†, K3.95†, K-RL07†, Q1.17, Q1.36, Q1.41, Q1.42, Q1.65, Q1.82, Q2.40, Q2.43, Q2.48, Q2.51, Q2.54†, Q3.27, Q3.28, Q3.31
- GNOME: GN-1.77, GN-1.82, GN-1.91, GN-1.132, GN-1.145, GN-1.262, GN-1.279†, GN-2.12, GN-2.15, GN-2.16, GN-2.38, GN-2.54†, GN-2.56†, GN-3.30, GN-3.47, GN-3.48, GN-3.49, GN-3.50, GN-3.51, GN-3.55, GN-3.68†, GN-3.69†
- Material: MAT-I19†, MAT-M5, MAT-N22, MAT-SC1, MAT-SC3†, MAT-T2, MAT-T6, MAT-X25, MAT-X32†
- iOS: C-4, C-24, C-37†, I-9†, SB-3, TT-2, TT-3†, X-BT-1, X-BT-2†

#### C22 — Pointer

- macOS: MAC-LK02, MAC-X20, MAC-X26
- Windows: W3-29, W3-50†
- KDE: K1.260†, K3.64, K3.65†, K3.66†, K3.96†, Q2.48
- GNOME: GN-1.282†, GN-2.53†, GN-3.46, GN-3.67†
- Material: MAT-X31, MAT-X33†
- iOS: B-17, SB-7†, X-PH-1, X-PH-2

#### C23 — Accessibility variants as values

- macOS: MAC-SW06, MAC-W08, MAC-X02, MAC-X11, MAC-X22, MAC-X23, MAC-X24, MAC-X25, MAC-X26, MAC-X31†
- Windows: W1-017, W1-079, W1-152†, W3-05, W3-07, W3-24, W3-35, W3-36, W3-37, W3-38, W3-48†, W3-50†, W3-56†
- KDE: K2.23†, K3.30, K3.90, K3.91, K3.92, K3.93, K3.95†, K3.96†, Q3.25, Q3.31
- GNOME: GN-1.04, GN-1.08, GN-1.22, GN-1.29, GN-1.34, GN-1.38, GN-1.44, GN-1.51, GN-1.65, GN-1.80, GN-1.86, GN-1.87, GN-1.88, GN-1.93, GN-1.105, GN-1.108, GN-1.115, GN-1.127, GN-1.133, GN-1.135, GN-1.148, GN-1.153, GN-1.167, GN-1.202, GN-1.207, GN-1.208, GN-1.209, GN-1.220, GN-1.230, GN-1.251, GN-1.273, GN-2.12, GN-2.18, GN-2.23, GN-2.30, GN-2.35, GN-3.06, GN-3.25, GN-3.26, GN-3.31, GN-3.61, GN-3.62, GN-3.63, GN-3.70†, GN-3.71†
- Material: MAT-S6, MAT-TB6, MAT-W1, MAT-X10, MAT-X18, MAT-X19, MAT-X20, MAT-X21, MAT-X35†
- iOS: B-15, D-15, L-9, L-19, LY-3, LY-4, MN-15, SW-4, SW-6, TS-2, TS-3, X-A11-1, X-A11-2, X-A11-3, X-A11-4, X-A11-5, X-A11-6, X-A11-7, X-A11-8, X-A11-9, X-A11-10, X-DT-2, X-DT-3, X-DT-4, X-FO-3†, X-MO-10, X-MT-3, X-MT-11, X-MT-12

#### C24 — A whole control the model lacks (the row is in §4.2)

- macOS: MAC-N01, MAC-N02, MAC-N03, MAC-N04, MAC-N05, MAC-N06, MAC-N07, MAC-N08, MAC-N09, MAC-N10, MAC-N11, MAC-N12, MAC-N13, MAC-N14, MAC-N15, MAC-N16, MAC-N17, MAC-N18, MAC-N19, MAC-N20, MAC-N21, MAC-N22, MAC-N23, MAC-N24, MAC-N25, MAC-N26, MAC-N27†, MAC-N28†, MAC-N29†, MAC-N30†, MAC-N31†, MAC-N32†, MAC-N33†, MAC-N34†, MAC-N35†, MAC-N36†, MAC-N37†, MAC-SF02†, MAC-SF03†, MAC-SF04†, MAC-SF06†, MAC-SF07†
- Windows: W2-01, W2-02, W2-03, W2-04, W2-05, W2-06, W2-07, W2-08, W2-09, W2-10, W2-11, W2-12, W2-13, W2-14, W2-15, W2-16, W2-17, W2-18, W2-19, W2-20, W2-21, W2-22, W2-23, W2-24, W2-25, W2-26, W2-27, W2-28, W2-29, W2-30, W2-31, W2-32, W2-33, W2-34, W2-35, W2-36, W2-37, W2-38, W2-39, W2-40†, W2-41†, W2-42†, W2-43†, W-RL03†, W-SF01†, W-SF02†, W-SF03†, W-SF04†, W-SF05†, W-SF06†, W-SF07†, W-SF08†
- KDE: K2.1, K2.3, K2.4, K2.5, K2.6, K2.7, K2.8, K2.9, K2.10, K2.11, K2.12, K2.13, K2.14, K2.15, K2.16, K2.17, K2.18, K2.19, K2.20, K2.21, K2.22, K2.23†, K2.24†, K2.25†, K2.26†, K2.27†, K2.28†, K2.29†, K2.30†, K2.31†, K2.32†, K-RL01†, K-RL02†, K-RL03†, K-RL04†, K-SF01†, K-SF02†, K-SF03†, K-SF04†, K-SF05†, K-SF06†, K-SF07†, K-SF08†, K-SF09†, K-SF10†, K-SF11†, K-SF12†, K-SF13†, K-SF14†, K-SF15†, K-SF16†, K-SF17†, K-SF23†, Q2.1, Q2.2, Q2.3, Q2.4, Q2.5, Q2.6, Q2.7, Q2.8, Q2.9, Q2.10, Q2.11, Q2.12, Q2.13, Q2.14, Q2.15, Q2.16, Q2.17, Q2.18, Q2.19, Q2.20, Q2.21, Q2.22, Q2.23, Q2.24, Q2.25, Q2.26, Q2.27, Q2.28, Q2.29, Q2.30, Q2.31, Q2.32, Q2.33, Q2.34, Q2.35, Q2.36, Q2.37, Q2.38, Q2.39, Q2.40, Q2.41, Q2.42, Q2.43, Q2.44, Q2.45, Q2.46, Q2.47, Q2.48, Q2.49, Q2.50, Q2.51, Q2.52, Q2.53†, Q2.54†, Q2.55†, Q2.56†, Q2.57†, Q2.58†
- GNOME: GN-2.01, GN-2.02, GN-2.03, GN-2.04, GN-2.05, GN-2.06, GN-2.07, GN-2.08, GN-2.09, GN-2.10, GN-2.11, GN-2.12, GN-2.13, GN-2.14, GN-2.15, GN-2.16, GN-2.17, GN-2.18, GN-2.19, GN-2.20, GN-2.21, GN-2.22, GN-2.23, GN-2.24, GN-2.25, GN-2.26, GN-2.27, GN-2.28, GN-2.29, GN-2.30, GN-2.31, GN-2.32, GN-2.33, GN-2.34, GN-2.35, GN-2.36, GN-2.37, GN-2.38, GN-2.39, GN-2.40, GN-2.41, GN-2.42, GN-2.43, GN-2.44, GN-2.45, GN-2.46, GN-2.48, GN-2.49, GN-2.50, GN-2.51, GN-2.52†, GN-2.53†, GN-2.54†, GN-2.55†, GN-2.56†, GN-2.57†, GN-2.58†, GN-RL01†, GN-SF01†, GN-SF02†, GN-SF03†, GN-SF04†, GN-SF05†, GN-SF06†, GN-SF07†, GN-SF08†, GN-SF09†, GN-SF10†, GN-SF11†, GN-SF12†, GN-SF13†, GN-SF14†, GN-SF15†, GN-SF16†
- Material: MAT-N1, MAT-N2, MAT-N3, MAT-N4, MAT-N5, MAT-N6, MAT-N7, MAT-N8, MAT-N9, MAT-N10, MAT-N11, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N16, MAT-N17, MAT-N18, MAT-N19, MAT-N20, MAT-N21, MAT-N22, MAT-N23, MAT-N24, MAT-N25, MAT-N26, MAT-N27, MAT-N28, MAT-N29, MAT-N30†, MAT-N31†, MAT-RL01†, MAT-SF03†
- iOS: C-1, C-2, C-3, C-4, C-5, C-6, C-7, C-8, C-9, C-10, C-11, C-12, C-13, C-14, C-15, C-16, C-17, C-18, C-19, C-20, C-21, C-22, C-23, C-24, C-25, C-26, C-28, C-29, C-30, C-31, C-32, C-33, C-36†, C-37†, C-38†, C-39†, C-40†, C-41†, C-42†, C-43†, C-44†, C-45†, C-46†, C-47†, C-48†, C-49†, RL-1†, RL-2†, SF-2†, SF-3†, SF-4†, SF-6†

#### C25 — Reader coverage

- macOS: MAC-DF05†, MAC-FM01†, MAC-FM02†, MAC-I07, MAC-I10†, MAC-IC03, MAC-SB09†, MAC-SF08†, MAC-T05, MAC-TS02, MAC-X10, MAC-X12, MAC-X13, MAC-X14, MAC-X24, MAC-X26, MAC-X30†, MAC-X33†, MAC-X35†, MAC-X36†, MAC-X37†
- Windows: W1-002, W1-007, W1-016, W1-034, W1-054, W1-066, W1-105, W1-148†, W1-149†, W1-152†, W1-156†, W1-158†, W3-01, W3-24, W3-41, W3-43, W3-47†, W3-48†, W3-50†, W3-53†, W3-55†, W3-56†, W-FM01†, W-FM02†
- KDE: K1.3, K1.5, K1.9, K1.11, K1.13, K1.15, K1.19†, K1.28, K1.62, K1.65, K1.70, K1.94, K1.103, K1.110, K1.131, K1.140, K1.150, K1.153, K1.156, K1.158, K1.175, K1.176, K1.179, K1.183, K1.220, K1.221, K2.10, K3.1, K3.2, K3.11, K3.12, K3.13, K3.14, K3.15†, K3.16†, K3.30, K3.31, K3.33, K3.34, K3.35, K3.36, K3.38†, K3.40, K3.41, K3.42, K3.44, K3.45, K3.46, K3.47, K3.50, K3.51, K3.53, K3.54, K3.63, K3.64, K3.66†, K3.70, K3.71, K3.72, K3.73, K3.83, K3.84†, K3.90, K3.91, K3.92, K3.93, K3.95†, K3.96†, K-FM01†, K-FM02†, K-FM03†, K-SF01†, K-SF20†, Q1.8, Q1.39, Q1.43, Q1.44, Q1.110, Q3.1, Q3.14, Q3.24
- GNOME: GN-1.19, GN-1.20, GN-1.191, GN-1.193, GN-1.202, GN-3.15, GN-3.16, GN-3.25, GN-3.26, GN-3.30, GN-3.32, GN-3.33, GN-3.34, GN-3.38, GN-3.46, GN-3.47, GN-3.48, GN-3.56, GN-3.62, GN-3.68†, GN-3.70†, GN-FM01†, GN-FM02†, GN-FM03†, GN-SF21†
- Material: MAT-FM01†, MAT-FM02†, MAT-FM03†, MAT-I19†, MAT-SC1, MAT-SC4†, MAT-T6, MAT-W1, MAT-W4†, MAT-W5†, MAT-X3, MAT-X17, MAT-X18, MAT-X19, MAT-X20, MAT-X25, MAT-X32†, MAT-X33†, MAT-X35†, MAT-X36†
- iOS: D-17, FM-1†, FM-2†, SD-10†, SF-7†, X-CO-4, X-FO-3†, X-IC-7, X-MT-11, X-MT-13, X-RD-1, X-VR-1

#### C26 — Layout and placement

- macOS: MAC-B11†, MAC-D01, MAC-EX05†, MAC-I10†, MAC-L01, MAC-L02, MAC-L07, MAC-L10, MAC-LY01, MAC-LY02†, MAC-MN01, MAC-MN02, MAC-MN03, MAC-MN05, MAC-MN11, MAC-MN15, MAC-MN16, MAC-N09, MAC-N20, MAC-N32†, MAC-N35†, MAC-RL01†, MAC-RL02†, MAC-S04, MAC-SB04, MAC-SB06, MAC-SB08, MAC-SB09†, MAC-SF08†, MAC-SF09†, MAC-SF10†, MAC-SL04, MAC-SP04, MAC-ST01, MAC-T03, MAC-TB01, MAC-TB02, MAC-TB03, MAC-TB07†, MAC-TL01, MAC-TL03, MAC-TL10†, MAC-W13†, MAC-X33†, MAC-X34†
- Windows: W1-010, W1-016, W1-036, W1-040, W1-045, W1-046, W1-047, W1-052, W1-059, W1-065, W1-072, W1-085, W1-088, W1-092, W1-094, W1-095, W1-097, W1-099, W1-101, W1-102, W1-108, W1-116, W1-117, W1-119, W1-124, W1-127, W1-133, W1-134, W1-136, W1-141, W1-146, W1-155†, W1-158†, W2-04, W2-11, W2-12, W2-22, W2-25, W2-30, W2-33, W2-37, W2-40†, W3-54†, W3-55†, W-RL03†, W-SF01†, W-SF04†, W-SF05†, W-SF07†, W-SF08†, W-SF10†, W-SF11†, W-SF12†, W-SF13†
- KDE: K1.1, K1.2, K1.3, K1.4, K1.5, K1.46†, K1.53, K1.58†, K1.63, K1.67, K1.69, K1.70, K1.71, K1.83†, K1.84†, K1.114, K1.115†, K1.128, K1.130, K1.131, K1.141, K1.154, K1.170, K1.176, K1.179, K1.180, K1.210, K1.211, K1.220, K1.245, K2.1, K2.7, K2.14, K2.22, K2.26†, K2.29†, K2.31†, K-FM03†, K-RL01†, K-RL03†, K-RL04†, K-RL05†, K-RL06†, K-SF14†, K-SF17†, K-SF19†, K-SF20†, K-SF21†, K-SF22†, K-SF23†, Q1.5, Q1.6, Q1.7, Q1.12, Q1.15, Q1.16, Q1.20, Q1.21, Q1.25, Q1.28, Q1.29, Q1.30, Q1.32, Q1.35, Q1.49, Q1.50, Q1.52, Q1.55, Q1.59, Q1.60, Q1.61, Q1.66, Q1.67, Q1.69, Q1.70, Q1.72, Q1.74, Q1.75, Q1.79, Q1.85, Q1.87, Q1.90, Q1.92, Q1.93, Q1.94, Q1.97, Q1.98, Q1.99, Q1.103, Q1.108, Q1.109, Q1.113, Q1.115, Q1.116, Q1.117, Q1.118, Q1.124, Q1.130, Q1.131, Q1.133, Q1.136, Q2.1, Q2.4, Q2.9, Q2.12, Q2.13, Q2.19, Q2.20, Q2.24, Q2.27, Q2.30, Q2.32, Q2.33, Q2.34, Q2.38, Q2.40, Q2.41, Q2.42, Q2.45, Q2.46, Q2.47, Q2.50, Q2.53†, Q2.54†, Q2.55†, Q2.56†, Q2.57†, Q2.58†, Q3.29
- GNOME: GN-1.12, GN-1.18, GN-1.20, GN-1.21, GN-1.27, GN-1.39, GN-1.43, GN-1.47, GN-1.61, GN-1.62, GN-1.69, GN-1.71, GN-1.72, GN-1.75, GN-1.79, GN-1.81, GN-1.83, GN-1.85, GN-1.95, GN-1.107, GN-1.111, GN-1.118, GN-1.121, GN-1.125, GN-1.139, GN-1.140, GN-1.142, GN-1.143, GN-1.144, GN-1.147, GN-1.149, GN-1.150, GN-1.152, GN-1.157, GN-1.163, GN-1.164, GN-1.166, GN-1.167, GN-1.170, GN-1.173, GN-1.174, GN-1.181, GN-1.187, GN-1.195, GN-1.196, GN-1.197, GN-1.214, GN-1.215, GN-1.216, GN-1.218, GN-1.222, GN-1.223, GN-1.224, GN-1.233, GN-1.234, GN-1.235, GN-1.238, GN-1.239, GN-1.243, GN-1.245, GN-1.248, GN-1.279†, GN-1.280†, GN-1.282†, GN-1.283†, GN-1.284†, GN-1.286†, GN-2.01, GN-2.10, GN-2.13, GN-2.15, GN-2.16, GN-2.17, GN-2.20, GN-2.21, GN-2.28, GN-2.29, GN-2.30, GN-2.32, GN-2.33, GN-2.34, GN-2.35, GN-2.39, GN-2.40, GN-2.41, GN-2.42, GN-2.43, GN-2.46, GN-2.48, GN-2.50, GN-2.51, GN-2.56†, GN-FM03†, GN-RL01†, GN-SF17†, GN-SF18†, GN-SF19†, GN-SF20†, GN-SF21†
- Material: MAT-B4, MAT-B8, MAT-C9, MAT-D1, MAT-DG1, MAT-DG2, MAT-DG8, MAT-DG10†, MAT-FM03†, MAT-FM04†, MAT-I16, MAT-L6, MAT-L7, MAT-LA1, MAT-M6, MAT-M7, MAT-M8, MAT-M10, MAT-N5, MAT-N6, MAT-N7, MAT-N10, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N18, MAT-N23, MAT-N25, MAT-P2, MAT-RL01†, MAT-S2, MAT-S12, MAT-SB7, MAT-SC3†, MAT-SF01†, MAT-SF02†, MAT-SF03†, MAT-SF04†, MAT-SP3, MAT-T4, MAT-TA3, MAT-TA8†, MAT-TB4, MAT-TB7, MAT-X22, MAT-X37†, MAT-X38†
- iOS: B-9, C-12, C-16, C-21, C-32, C-41†, C-43†, CD-1, DL-3, DL-4, DL-6, DL-13†, I-3, L-2, L-3, L-6, L-7, L-19, L-21†, L-22†, L-23†, LY-1, LY-2, LY-3, LY-4, MN-3, MN-5, MN-17†, PB-4†, PO-4, PO-7†, RL-3†, SB-2, SD-6, SD-9†, SD-10†, SF-7†, SF-8†, SF-9†, SG-9†, T-9, TB-8, TT-3†, W-7, X-DT-3, X-SV-3

#### C27 — Gradient paint

- macOS: MAC-B05, MAC-C05, MAC-X08, MAC-X09
- Windows: W1-018, W1-020, W1-026, W1-086, W1-131, W3-04
- KDE: K1.14, K2.27†, Q1.56, Q2.6, Q3.21
- GNOME: GN-1.178, GN-2.14, GN-2.18, GN-2.50, GN-2.53†, GN-3.05, GN-3.06
- Material: MAT-X9
- iOS: C-30, C-46†, C-48†

#### C28 — Colour space and precision

- macOS: MAC-N05, MAC-X10
- iOS: D-17, X-CO-4
