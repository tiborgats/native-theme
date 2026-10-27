# v0.7.1 — Model completeness: the gap map

Status: first pass done (2026-10-28, branch `v0.7.1-rc1`, model at caeddd6b); the
cross-check of §6 is open, so the map is not complete yet.

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

**Platforms**, one column each, as `platform-facts.md` has them:

| Column | Generation | Preset |
|---|---|---|
| macOS | 12–15 (Aqua) | `macos-monterey` |
| macOS 26 | 26 and 27 (Liquid Glass) | `macos-tahoe` |
| Windows | 11 (WinUI 3, Fluent 2) | `windows-11` |
| KDE | Plasma 6, Breeze (Qt Widgets and Kirigami / Qt Quick) | `kde-breeze` |
| GNOME | GNOME 47+, libadwaita / GTK 4 | `adwaita` |
| Material | Material 3 (and 3 Expressive) | `material-3` |
| iOS | 13–18, and 26 (Liquid Glass) | `ios-13`, `ios-26` (not yet written) |

The community presets (Catppuccin, Dracula, Nord, …) are not platforms: they take their
structure from the model and need no column.

**The model** is everything `docs/property-registry.toml` lists (`[defaults]`,
`[text_scale]`, the window and 29 widget sections, the `Font`, `Border`, `IconSizes` and
`TextScaleEntry` structures), the icon sets, and what the platform readers return.

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
corner radius, border and shadow), the 29 widget sections the model has, **every standard
control a platform has and the model does not** (§4.2), and the properties that cross
widgets (§4.3).

## 3. Sources and measurement

Each gap names, per platform, where its value comes from, in this order of preference:

1. **Read at run time** — an API or setting the reader can query (`UISettings`,
   `NSColor`, `kdeglobals`, the settings portal, `GetCaretBlinkTime`, …).
2. **Published** — the platform's own source or documentation, cited at a commit or with
   a read date: WinUI 3's XAML theme resources, Breeze's `breezemetrics.h` and its
   C++, libadwaita's SCSS, Material's tokens, Apple's documentation.
3. **Measured** — on the platform, when 1 and 2 give nothing:
   - **macOS and Windows:** the OS probe (`tools/os-probe`, CI workflow
     `os-probe.yml`) renders controls through the platform's own API on each OS
     version and reads its metrics; it gains a render or a query per gap;
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

The research ran per platform, one survey each, against `docs/property-registry.toml` at
caeddd6b. Each platform's rows, with their sources, are in its own file (§8); this
section is the cross-platform view built from them. Every row was then tagged with the
capabilities (§4.3) it needs the model to have.

| Platform | §4.1 | §4.2 | §4.3 | Total |
|---|---:|---:|---:|---:|
| macOS | 171 | 26 | 29 | 226 |
| Windows | 147 | 39 | 43 | 229 |
| KDE | 278 | 73 | 80 | 431 |
| GNOME | 276 | 50 | 58 | 384 |
| Material | 164 | 29 | 27 | 220 |
| iOS | 205 | 32 | 75 | 312 |
| **All** | 1241 | 249 | 312 | 1802 |

The columns are the platform files' own sections: §4.1 an element the model has, §4.2 a
control it lacks, §4.3 a property that crosses widgets. A row counts once. Not counted are
42 rows that only point to another row or record "no gap" (macOS 2, Windows 1, KDE 13,
GNOME 9, Material 7, iOS 10). A row ID is unique within its file.

### 4.1 Elements the model has: gaps per section

The rows of §4.1 of each platform file (parts, states and properties of an element the
model has), by the model section they belong to. Their capabilities are in §4.3.

| Model section | macOS | Windows | KDE | GNOME | Material | iOS |
|---|---:|---:|---:|---:|---:|---:|
| `[button]` | 10 | 8 | 14 | 19 | 14 | 17 |
| `[card]` | 2 | 1 | 8 | 5 | 3 | 1 |
| `[checkbox]` | 7 | 7 | 11 | 11 | 9 | 1 |
| `[combo_box]` | 6 | 7 | 12 | 6 | 3 | 4 |
| `[defaults]` | 8 | 0 | 2 | 9 | 0 | 20 |
| `[dialog]` | 4 | 5 | 19 | 18 | 9 | 12 |
| `[expander]` | 4 | 3 | 2 | 7 | 1 | 4 |
| `[input]` | 6 | 10 | 15 | 9 | 15 | 7 |
| `[layout]` | 1 | 1 | 4 | 3 | 1 | 4 |
| `[link]` | 3 | 3 | 3 | 4 | 0 | 1 |
| `[list]` | 16 | 8 | 27 | 21 | 14 | 19 |
| `[menu]` | 16 | 12 | 24 | 15 | 11 | 16 |
| `[popover]` | 5 | 4 | 4 | 7 | 1 | 6 |
| `[progress_bar]` | 4 | 5 | 7 | 7 | 7 | 3 |
| `[scrollbar]` | 8 | 7 | 12 | 10 | 2 | 6 |
| `[segmented_control]` | 9 | 1 | 0 | 13 | 4 | 7 |
| `[separator]` | 0 | 1 | 3 | 2 | 1 | 2 |
| `[sidebar]` | 8 | 10 | 17 | 13 | 9 | 8 |
| `[slider]` | 7 | 8 | 11 | 15 | 12 | 9 |
| `[spinner]` | 4 | 1 | 4 | 7 | 6 | 5 |
| `[splitter]` | 4 | 1 | 4 | 1 | 3 | 3 |
| `[status_bar]` | 1 | 1 | 1 | 1 | 0 | 0 |
| `[switch]` | 6 | 5 | 5 | 10 | 9 | 7 |
| `[tab]` | 5 | 9 | 16 | 20 | 8 | 16 |
| `[text_area]` | 2 | 1 | 0 | 3 | 1 | 5 |
| `[text_scale]` | 1 | 1 | 6 | 6 | 5 | 4 |
| `[toolbar]` | 9 | 6 | 18 | 8 | 7 | 8 |
| `[tooltip]` | 5 | 4 | 9 | 3 | 7 | 2 |
| `[window]` | 10 | 17 | 20 | 23 | 2 | 8 |

### 4.2 Standard controls the model does not have

The rows of §4.2 of each platform file, sorted by how many platforms list the control. A
"–" means that platform's survey did not list the control as missing: the platform may lack it, or may draw it inside a section the
model has (a tree view inside `[list]`). Settling every "–" is the cross-check of §6.

| Control | macOS | Windows | KDE | GNOME | Material | iOS |
|---|---|---|---|---|---|---|
| badge | MAC-N11 | W2-08 | Q2.25, Q2.26, Q2.27 | GN-2.36, GN-2.37 | MAT-N10 | C-15 |
| icon button | – | W2-19 | Q2.43 | GN-2.44 | MAT-N4, MAT-N5 | C-29 |
| navigation/app bar | – | W2-11 | K2.14, Q2.14, Q2.15 | GN-2.18, GN-2.20 | MAT-N11, MAT-N25 | C-1, C-28 |
| search field | MAC-N02 | W2-15 | – | GN-2.21, GN-2.22, GN-2.45 | MAT-N14, MAT-N15 | C-3 |
| stepper/spin box | MAC-N01 | W2-01 | K2.3, Q2.1 | GN-2.05, GN-2.24 | – | C-4 |
| banner/inline message | – | W2-07 | K2.13, Q2.16, Q2.17, Q2.18, Q2.19, Q2.20, Q2.21 | GN-2.11, GN-2.31 | MAT-N23 | – |
| bottom tab bar | MAC-N23 | – | Q2.33, Q2.34, Q2.35, Q2.36 | – | MAT-N12 | C-22 |
| calendar | – | W2-02 | K2.4 | GN-2.25 | – | C-6 |
| date picker | MAC-N04 | W2-03 | – | – | MAT-N16 | C-5 |
| level indicator/gauge | MAC-N06, MAC-N22 | – | K2.6, K2.18 | GN-2.26 | – | C-30, C-31 |
| menu bar | MAC-N20 | W2-22 | K2.1, Q2.9, Q2.10 | GN-2.30 | – | – |
| page control/pager | – | W2-20, W2-21 | Q2.11 | GN-2.38 | – | C-9 |
| preference rows/boxed list | MAC-N21 | W2-25, W2-38 | Q2.41, Q2.42, Q2.45, Q2.46, Q2.47 | GN-2.01, GN-2.02, GN-2.03, GN-2.04, GN-2.06, GN-2.07, GN-2.08, GN-2.09, GN-2.10 | – | – |
| pull-to-refresh | MAC-N24 | W2-26 | – | – | MAT-N28 | C-10 |
| sheet/bottom sheet | – | – | Q2.37, Q2.38, Q2.39, Q2.40 | GN-2.17 | MAT-N19, MAT-N20, MAT-N21 | C-12 |
| toast/snackbar | MAC-N25 | – | Q2.49, Q2.50, Q2.51, Q2.52 | GN-2.12, GN-2.50 | MAT-N22 | – |
| avatar | – | W2-24 | Q2.28, Q2.29 | GN-2.14 | – | – |
| breadcrumb/path control | MAC-N07 | W2-10 | Q2.44 | – | – | – |
| button group/toggle group | – | – | K2.7, K2.17, Q2.3 | GN-2.46 | MAT-N6 | – |
| chip | – | – | Q2.22, Q2.23, Q2.24 | GN-2.43 | MAT-N9 | – |
| colour button | MAC-N05 | – | K2.20 | GN-2.27 | – | – |
| group box/frame | – | – | K2.8, Q2.12, Q2.13 | GN-2.32 | – | C-32 |
| help button | MAC-N10 | – | Q2.48 | GN-2.40 | – | – |
| image frame | MAC-N12 | – | K2.21 | GN-2.49 | – | – |
| split button | – | W2-17 | – | GN-2.29 | MAT-N7 | – |
| status page/placeholder | – | – | K2.15, Q2.30, Q2.31, Q2.32 | GN-2.13 | – | C-16 |
| about dialog | – | – | K2.22 | GN-2.39, GN-2.41 | – | – |
| carousel | – | W2-31 | – | – | MAT-N18 | – |
| colour picker | – | W2-05 | – | – | – | C-8 |
| dial/circular slider | MAC-N15 | – | K2.5, Q2.2 | – | – | – |
| fab | – | – | Q2.4, Q2.5, Q2.6, Q2.7, Q2.8 | – | MAT-N1, MAT-N2, MAT-N3 | – |
| find bar | MAC-N18 | – | – | – | – | C-25 |
| menu button | – | W2-18 | – | – | MAT-N8 | – |
| navigation drawer/pane | – | W2-30 | – | GN-2.19, GN-2.34, GN-2.51 | – | – |
| navigation rail | – | – | K2.16 | – | MAT-N13 | – |
| password field | – | W2-16 | – | GN-2.23 | – | – |
| rating | – | W2-06 | K2.19 | – | – | – |
| time picker | – | W2-04 | – | – | MAT-N17 | – |
| token field | MAC-N03 | W2-39 | – | – | – | – |
| view switcher | – | W2-27, W2-32 | – | GN-2.15, GN-2.16, GN-2.33, GN-2.35 | – | – |
| access key badge | – | W2-37 | – | – | – | – |
| action sheet | – | – | – | – | – | C-11 |
| autocomplete | – | – | – | – | MAT-N27 | – |
| back button | – | – | – | – | – | C-2 |
| background extension | – | – | – | – | – | C-19 |
| bottom accessory | – | – | – | – | – | C-21 |
| collection view | MAC-N13 | – | – | – | – | – |
| column browser | MAC-N08 | – | – | – | – | – |
| context menu preview | – | – | – | – | – | C-13 |
| dock/panel title bar | – | – | K2.10, K2.11 | – | – | – |
| drag and drop | – | – | – | – | – | C-24 |
| edit menu | – | – | – | – | – | C-14 |
| file chooser | – | – | – | GN-2.48 | – | – |
| font chooser | – | – | – | GN-2.28 | – | – |
| full-screen dialog | – | – | – | – | MAT-N26 | – |
| glass container | – | – | – | – | – | C-20 |
| ink toolbar | – | W2-36 | – | – | – | – |
| inspector/utility panel | MAC-N14, MAC-N16, MAC-N19 | – | – | – | – | – |
| keyboard | – | – | – | – | – | C-23 |
| large content viewer hud | – | – | – | – | – | C-26 |
| list box | – | W2-33 | – | – | – | – |
| loading indicator | – | – | – | – | MAT-N24 | – |
| media transport controls | – | W2-35 | – | – | – | – |
| paste control | – | – | – | – | – | C-17 |
| picker wheel | – | – | – | – | – | C-7 |
| rubber band | – | – | K2.12 | – | – | – |
| rule editor | MAC-N09 | – | – | – | – | – |
| scrim | – | – | – | – | MAT-N29 | – |
| scroll edge effect | – | – | – | – | – | C-18 |
| scrollbar annotation | – | W2-28 | – | – | – | – |
| swipe actions | – | W2-29 | – | – | – | – |
| teaching tip | – | W2-09 | – | – | – | – |
| text ruler | MAC-N17 | – | – | – | – | – |
| tool box | – | – | K2.9 | – | – | – |
| tool button | – | – | – | GN-2.42 | – | – |
| toolbar flyout | – | W2-23 | – | – | – | – |
| touch bar | MAC-N26 | – | – | – | – | – |
| tree view | – | W2-12, W2-13, W2-14 | – | – | – | – |
| writing tools | – | – | – | – | – | C-33 |
| zoom button | – | W2-34 | – | – | – | – |

### 4.3 Capabilities the model lacks, across all rows

One row can need several capabilities (the checked-and-hovered switch's border colour is
C07 and C12), so a column's counts add up to more than the platform's total. A count is how
many of that platform's rows need the capability; it shows where a capability matters, not
how hard it is. The row IDs per capability are in Appendix A.

| ID | Capability the model lacks | macOS | Windows | KDE | GNOME | Material | iOS |
|---|---|---:|---:|---:|---:|---:|---:|
| C01 | Shadow geometry: layers, each with offset, blur, spread, colour, inset; per surface | 12 | 16 | 29 | 29 | 3 | 16 |
| C02 | Elevation: a named elevation ramp/levels per element, elevation per state, surface tint | 1 | 9 | 0 | 0 | 35 | 0 |
| C03 | Materials: translucency, backdrop blur, vibrancy, Mica/Acrylic, Liquid Glass, their tint/noise/luminosity and opaque fallbacks | 20 | 17 | 6 | 3 | 0 | 38 |
| C04 | Colour composition: a colour stated relative to what is behind it (alpha fill, mix of current text colour, overlay) rather than one resolved opaque colour | 4 | 7 | 75 | 66 | 6 | 5 |
| C05 | Colour roles missing from defaults (accent ramp, standalone accent, status background/foreground, tertiary, container/surface ladder, inverse, header/complementary colour sets, shade roles) | 8 | 20 | 57 | 53 | 11 | 10 |
| C06 | Colour variant axes beyond light/dark: increased/high contrast values, inactive/non-key window, elevated level, contrast levels | 20 | 11 | 19 | 39 | 1 | 12 |
| C07 | States and state combinations: pressed, focused, dragged, drop target, indeterminate, read-only, invalid/error, default, open, checked/selected × hover/pressed/disabled, per-state colours of a part | 51 | 85 | 95 | 80 | 77 | 36 |
| C08 | State-layer model: hover/pressed/focus/drag overlay opacities, ripple | 0 | 0 | 2 | 0 | 29 | 0 |
| C09 | Widget variants/styles: flat, outlined, filled, tonal, destructive, pill, circular, bezel styles, field styles, segmented styles, glass styles | 35 | 5 | 43 | 50 | 31 | 31 |
| C10 | Size variants and density: control sizes (mini…extraLarge), compact density, touch vs mouse sizing | 13 | 18 | 34 | 56 | 31 | 14 |
| C11 | Shape: per-corner radii, continuous corner curve, capsule/fully rounded, concentric radii, shape change between states | 22 | 26 | 32 | 49 | 46 | 28 |
| C12 | Border/outline model: per-side widths and colours, gradient strokes, inner+outer outlines, outline drawn outside the box, state-dependent border thickness | 18 | 27 | 58 | 54 | 21 | 13 |
| C13 | Parts missing from a widget the model has: icons, glyphs, labels, helper text, counters, headers, footers, close buttons, indicators/pills, ticks, grabbers, arrows, their geometry and colours | 71 | 85 | 122 | 84 | 103 | 76 |
| C14 | Motion: durations, easing curves, springs, delays per transition, a speed factor, reduced-motion replacements | 13 | 54 | 75 | 51 | 37 | 56 |
| C15 | Focus indication: style (ring stroked/filled, inner/outer, underline, rounded rect), width, offset, keyboard-only rule, per-widget focus style, animation | 5 | 8 | 18 | 28 | 6 | 3 |
| C16 | Text caret: colour, width, blink period / on-off / timeout, style | 2 | 2 | 5 | 10 | 2 | 3 |
| C17 | Selection and highlights: inactive/unemphasised selection, its text colour, find highlight, drag-and-drop highlight | 8 | 2 | 8 | 6 | 0 | 8 |
| C18 | Typography: letter spacing, line height per role, optical size / variable axes, font width, missing text styles, per-size text tables (Dynamic Type), font features, title fonts | 19 | 18 | 30 | 26 | 37 | 30 |
| C19 | Icons: state effects (disabled/hover/active), symbolic recolouring, size groups per context, rendering modes, symbol effects | 10 | 4 | 46 | 11 | 3 | 14 |
| C20 | Window chrome: title bar geometry and fills, caption/traffic-light buttons and their states, button layout, window corner radius, window border/outline, window shadow, maximised/tiled/snapped/fullscreen states, unfocused-window appearance | 12 | 17 | 21 | 23 | 2 | 8 |
| C21 | Visible timings: double-click, drag threshold, submenu/hover-open delays, tooltip show/hide/reshow, scrollbar auto-hide, toast durations | 4 | 8 | 20 | 17 | 6 | 5 |
| C22 | Pointer: cursor theme and size, pointer hover effects (iPad) | 3 | 1 | 2 | 1 | 1 | 3 |
| C23 | Accessibility variants as values: what high contrast, reduce transparency, bold text, button shapes, differentiate without colour, larger text change | 9 | 9 | 7 | 43 | 8 | 28 |
| C24 | A whole control the model lacks (the row is in §4.2) | 26 | 39 | 73 | 50 | 29 | 32 |
| C25 | Reader coverage: a value readable at run time that no reader reads today | 10 | 11 | 68 | 19 | 9 | 7 |
| C26 | Layout and placement: alignment, ordering, positions, insets, spacing rules not tied to one widget part | 28 | 39 | 102 | 85 | 36 | 29 |
| C27 | Gradient paint: linear/radial gradient fills and strokes, bevels and highlights drawn as gradients | 4 | 6 | 4 | 6 | 0 | 1 |
| C28 | Colour space and precision: wide gamut (Display P3), HDR, more than 8 bits per channel | 2 | 0 | 0 | 0 | 0 | 2 |

C27 and C28 were added after the tagging and applied from each row's own text (the words
*gradient*, *bevel*; *P3*, *HDR*, *wide gamut*, *extended range*), then checked by hand.
C02 and C08 are Material's and Windows' own mechanisms (elevation levels, state layers);
the other platforms' shadow and hover rows are tagged C01 and C07 (KDE has two C08 rows).

What the counts show, in brief:

- **Every platform** draws layered shadows (C01) the model cannot state, animates almost
  everything (C14), draws parts the model has no field for (C13), and draws states and
  state combinations the model lacks (C07): pressed, focused, checked or selected with hover
  and pressed, indeterminate, drop target, the default button.
- **Translucency** (C03) is the core of macOS's and iOS's look (14 named macOS materials,
  Liquid Glass on macOS 26 and iOS 26) and of Windows' (Mica, Mica Alt, Acrylic); the model
  stores a solid colour where these draw a material.
- **Colours stated over what lies behind** (C04): KDE and GNOME derive most fills as a
  blend or a share of the text colour over the surface below; the model stores one opaque
  colour, resolved over one background, which is wrong on any other surface.
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
  the model has no slot for (macOS has 11, Material 15, the model 4), per-size Dynamic Type.
- **Window chrome** (C20): title-bar geometry, the caption and traffic-light buttons and
  their states, the button layout, window corners, the window's own outline and shadow,
  maximised and tiled states.
- **Focus, caret, selection** (C15–C17): Breeze's filled focus ring shown only after
  keyboard focus, WinUI's inner and outer rings, the caret's width and blink timing (read at
  run time on Windows, KDE and GNOME), unemphasised selection, the find highlight.
- **Reader coverage** (C25): values the platforms expose at run time that no reader reads,
  most on KDE (seven colour sets with Active, Inactive and Disabled groups, the colour
  effects, `breezerc`, `kwinrc`); iOS has no reader at all.

## 5. Seeds

The gaps the repository had already written down, gathered before the platform research:
`todo_v0.7.1_model-completeness-gap-map-seeds.md` (37 groups, each with every file:line
that records it, at caeddd6b), and in its second part the 222 lines of
`platform-facts.md` with a ❓ cell, a value no source gave. Under this milestone's rule a ❓
is a value to measure. The cross-check (§6) confirms that every seed group has a row in a
platform file.

## 6. Open before the rationale

1. **Cross-check across platforms.** A capability or control one survey found is a
   question for the other five: does that platform draw it too? Each "–" of §4.2 and each
   low count of §4.3 gets an answer, a new row with its source, or the platform's reason
   it does not draw it.
2. **Seeds.** Every seed group of §5 is matched to its rows, or becomes a row.
3. **`platform-facts.md`'s 366 "(none)" cells** (at caeddd6b) are classified: the platform
   does not draw it (the cell stays, with the reason), or it draws it and no document
   states it (a value to measure, a row here).
4. **Citations.** The KDE file cites KDE's GitHub mirrors at `master`; each citation is
   re-pinned to the release tags `platform-facts.md` uses (Breeze and KWin 6.7.x, KF
   6.31.0) before any value is collected. The rows marked "to verify" (macOS 17,
   Windows 37, KDE 22, GNOME 8, Material 30, iOS 17) are settled.
5. **Measuring, as the surveys found it:**
   - the GitHub macOS runner images have Reduce Motion and Reduce Transparency **on**, so
     motion and materials need a probe pass with both off;
   - the probe's offscreen renders draw the non-key look, so key-window states need the
     controls in the key-window capture;
   - the window captures use `screencapture -o`, which leaves shadows out;
   - the probe opens no menu and does not render the extraLarge control size;
   - the Windows probe only queries; it renders nothing (`tools/os-probe/src/windows.rs`);
   - no KDE (Qt Widgets, Kirigami), GNOME (GTK 4 / libadwaita) or iOS (UIKit, simulator)
     probe app exists yet, nor a Material one (material-web, Compose); whether the macOS
     CI image has the iOS 13–18 simulator runtimes is unchecked;
   - an animation is measured from a screen recording at the display's refresh rate, or
     from the toolkit's own animation trace.

## 7. Values found wrong on the way

The surveys read sources that disagree with values native-theme already ships (KDE 9,
GNOME 12, Material 1, Windows 2). They are not gaps of the model; they are recorded in
`docs/todo.md`, *Values the v0.7.1 gap map found against their sources*.

## 8. The platform files

| File | Platform | Row IDs |
|---|---|---|
| `todo_v0.7.1_model-completeness-gap-map-macos.md` | macOS 12–15 and 26–27 | `MAC-…` |
| `todo_v0.7.1_model-completeness-gap-map-windows.md` | Windows 11 | `W1-…`, `W2-…`, `W3-…` |
| `todo_v0.7.1_model-completeness-gap-map-kde.md` | KDE Plasma 6, Breeze | `K…` (Qt Widgets, KDecoration, KWin), `Q…` (Kirigami, qqc2-desktop-style) |
| `todo_v0.7.1_model-completeness-gap-map-gnome.md` | GNOME, libadwaita 1.10.0, GTK 4.24.1 | `GN-…` |
| `todo_v0.7.1_model-completeness-gap-map-material.md` | Material 3 and Expressive | `MAT-…` |
| `todo_v0.7.1_model-completeness-gap-map-ios.md` | iOS 13–18 and 26 | per section (`B-…`, `X-MT-…`, `C-…` for the controls of §4.2) |
| `todo_v0.7.1_model-completeness-gap-map-seeds.md` | the repository's own records | `G01`–`G37` |

Each file states its sources and their refs at its top, and no file states a value it did
not read in a cited source.

## Appendix A. Rows per capability

#### C01 — Shadow geometry

- macOS: MAC-B04, MAC-D03, MAC-MN14, MAC-PO03, MAC-SB06, MAC-SG02, MAC-SL02, MAC-SW02, MAC-T01, MAC-W04, MAC-W07, MAC-X01
- Windows: W1-003, W1-014, W1-053, W1-057, W1-096, W1-102, W1-115, W1-129, W1-133, W1-139, W2-01, W2-09, W2-15, W2-23, W3-01, W3-05
- KDE: K1.17, K1.18, K1.23, K1.81, K1.101, K1.190, K1.244, K2.5, K2.18, K3.1, K3.2, K3.3, K3.4, Q1.16, Q1.23, Q1.33, Q1.47, Q1.56, Q1.89, Q1.90, Q1.106, Q1.112, Q1.125, Q1.127, Q1.128, Q2.7, Q2.49, Q3.20, Q3.21
- GNOME: GN-1.01, GN-1.02, GN-1.05, GN-1.07, GN-1.09, GN-1.14, GN-1.97, GN-1.133, GN-1.153, GN-1.156, GN-1.166, GN-1.178, GN-1.179, GN-1.183, GN-1.201, GN-1.208, GN-1.240, GN-1.252, GN-1.256, GN-2.12, GN-2.17, GN-2.18, GN-2.19, GN-3.01, GN-3.02, GN-3.03, GN-3.05, GN-3.06, GN-3.07
- Material: MAT-PO1, MAT-X3, MAT-X5
- iOS: C-13, C-24, CD-1, MN-2, PO-3, SD-6, SG-1, SL-1, SW-1, T-9, TB-5, TT-1, W-2, X-SH-1, X-SH-2, X-SH-3

#### C02 — Elevation

- macOS: MAC-X01
- Windows: W1-003, W1-057, W1-115, W1-129, W1-139, W3-01, W3-02, W3-03, W3-04
- Material: MAT-B1, MAT-B5, MAT-B6, MAT-B13, MAT-CA1, MAT-CA2, MAT-L4, MAT-N1, MAT-N2, MAT-N9, MAT-N11, MAT-N13, MAT-N14, MAT-N15, MAT-N18, MAT-N19, MAT-N20, MAT-N21, MAT-N22, MAT-N23, MAT-N25, MAT-N26, MAT-N28, MAT-PO1, MAT-S9, MAT-SB6, MAT-SP2, MAT-SW7, MAT-T1, MAT-TA1, MAT-X2, MAT-X4, MAT-X6, MAT-X7, MAT-X29

#### C03 — Materials

- macOS: MAC-D03, MAC-L09, MAC-MN14, MAC-N14, MAC-N19, MAC-N20, MAC-PO02, MAC-SB06, MAC-SB07, MAC-T02, MAC-TL03, MAC-TL09, MAC-W08, MAC-W10, MAC-X02, MAC-X03, MAC-X04, MAC-X05, MAC-X06, MAC-X23
- Windows: W1-005, W1-014, W1-053, W1-056, W1-062, W1-088, W1-096, W1-102, W1-114, W1-133, W3-05, W3-06, W3-07, W3-08, W3-09, W3-10, W3-38
- KDE: K1.64, K1.65, K3.11, K3.12, K3.13, K3.92
- GNOME: GN-2.17, GN-3.04, GN-3.09
- iOS: B-2, C-1, C-9, C-11, C-13, C-14, C-18, C-19, C-20, C-23, C-26, CD-1, DL-1, MN-1, PO-2, SB-6, SD-1, SL-3, SW-2, T-1, T-8, TB-1, TB-5, TT-1, X-A11-2, X-MT-1, X-MT-2, X-MT-3, X-MT-4, X-MT-5, X-MT-6, X-MT-7, X-MT-8, X-MT-9, X-MT-10, X-MT-11, X-MT-12, X-SC-1

#### C04 — Colour composition

- macOS: MAC-B05, MAC-C05, MAC-X03, MAC-X09
- Windows: W1-018, W1-055, W1-088, W1-125, W3-09, W3-11, W3-16
- KDE: K1.7, K1.14, K1.22, K1.24, K1.25, K1.60, K1.65, K1.66, K1.67, K1.90, K1.91, K1.93, K1.111, K1.123, K1.124, K1.126, K1.152, K1.171, K1.177, K1.200, K1.242, K2.1, K2.12, K2.13, K2.21, K3.12, K3.13, K3.40, K3.50, K3.52, Q1.22, Q1.26, Q1.38, Q1.41, Q1.45, Q1.56, Q1.57, Q1.62, Q1.66, Q1.87, Q1.89, Q1.95, Q1.100, Q1.103, Q1.104, Q1.112, Q1.126, Q1.128, Q1.131, Q2.3, Q2.5, Q2.6, Q2.10, Q2.11, Q2.12, Q2.17, Q2.22, Q2.25, Q2.26, Q2.29, Q2.31, Q2.35, Q2.36, Q2.38, Q2.39, Q2.41, Q2.42, Q2.43, Q2.44, Q2.49, Q3.14, Q3.15, Q3.16, Q3.22, Q3.33
- GNOME: GN-1.03, GN-1.10, GN-1.16, GN-1.24, GN-1.25, GN-1.29, GN-1.30, GN-1.31, GN-1.32, GN-1.34, GN-1.35, GN-1.52, GN-1.55, GN-1.57, GN-1.72, GN-1.86, GN-1.87, GN-1.98, GN-1.100, GN-1.102, GN-1.110, GN-1.120, GN-1.124, GN-1.141, GN-1.146, GN-1.155, GN-1.158, GN-1.159, GN-1.167, GN-1.174, GN-1.175, GN-1.176, GN-1.184, GN-1.198, GN-1.199, GN-1.205, GN-1.210, GN-1.226, GN-1.241, GN-1.242, GN-1.246, GN-1.253, GN-1.256, GN-1.260, GN-1.265, GN-1.271, GN-1.273, GN-2.01, GN-2.02, GN-2.11, GN-2.23, GN-2.29, GN-2.31, GN-2.40, GN-2.50, GN-2.51, GN-3.04, GN-3.08, GN-3.09, GN-3.12, GN-3.40, GN-3.41, GN-3.42, GN-3.43, GN-3.44, GN-3.60
- Material: MAT-DG5, MAT-N29, MAT-X6, MAT-X8, MAT-X23, MAT-X24
- iOS: DL-2, SE-2, X-MT-4, X-MT-10, X-SC-1

#### C05 — Colour roles missing from defaults (accent ramp, standalone accent, status background/foreground, tertiary, container/surface ladder, inverse, header/complementary colour sets, shade roles)

- macOS: MAC-B07, MAC-DF01, MAC-DF02, MAC-DF03, MAC-DF04, MAC-N06, MAC-TL04, MAC-X27
- Windows: W1-013, W1-055, W1-116, W1-125, W1-126, W1-139, W1-141, W2-07, W2-08, W2-30, W3-08, W3-09, W3-11, W3-12, W3-13, W3-14, W3-15, W3-16, W3-17, W3-18
- KDE: K1.122, K1.183, K2.1, K2.4, K2.6, K2.13, K2.15, K2.18, K2.20, K2.21, K3.23, K3.50, K3.51, K3.52, K3.53, K3.54, K3.61, Q1.2, Q1.13, Q1.37, Q1.52, Q1.53, Q1.54, Q1.68, Q1.71, Q1.81, Q1.93, Q1.96, Q1.112, Q1.121, Q1.123, Q1.129, Q1.134, Q2.1, Q2.8, Q2.9, Q2.16, Q2.22, Q2.25, Q2.28, Q2.33, Q2.34, Q2.37, Q2.49, Q3.1, Q3.2, Q3.4, Q3.5, Q3.6, Q3.7, Q3.8, Q3.9, Q3.10, Q3.11, Q3.12, Q3.13, Q3.17
- GNOME: GN-1.15, GN-1.31, GN-1.45, GN-1.52, GN-1.88, GN-1.122, GN-1.125, GN-1.127, GN-1.136, GN-1.137, GN-1.138, GN-1.149, GN-1.153, GN-1.157, GN-1.171, GN-1.182, GN-1.205, GN-1.225, GN-1.247, GN-1.255, GN-1.257, GN-1.260, GN-1.265, GN-1.268, GN-1.269, GN-1.270, GN-1.271, GN-1.272, GN-1.274, GN-1.275, GN-1.276, GN-2.11, GN-2.14, GN-2.19, GN-2.20, GN-2.21, GN-2.26, GN-2.31, GN-2.35, GN-2.36, GN-2.39, GN-2.45, GN-2.49, GN-2.51, GN-3.09, GN-3.12, GN-3.13, GN-3.14, GN-3.15, GN-3.16, GN-3.17, GN-3.43, GN-3.58
- Material: MAT-M10, MAT-N1, MAT-N3, MAT-N11, MAT-N29, MAT-P6, MAT-SN5, MAT-X1, MAT-X5, MAT-X17, MAT-X23
- iOS: CB-4, D-9, D-10, D-11, D-12, D-13, SE-2, X-CO-2, X-CO-5, X-MT-6

#### C06 — Colour variant axes beyond light/dark

- macOS: MAC-B02, MAC-CB02, MAC-CB06, MAC-L03, MAC-P03, MAC-SB05, MAC-SB07, MAC-SG06, MAC-ST01, MAC-SW04, MAC-TB04, MAC-W02, MAC-W06, MAC-W07, MAC-X02, MAC-X03, MAC-X16, MAC-X22, MAC-X27, MAC-X29
- Windows: W1-001, W1-003, W1-005, W1-008, W1-012, W1-013, W3-01, W3-06, W3-10, W3-35, W3-44
- KDE: K1.9, K1.12, K1.13, K1.18, K1.22, K1.23, K1.45, K1.92, K1.150, K1.173, K1.222, K2.1, K2.11, K3.40, K3.51, K3.91, Q1.37, Q1.71, Q3.3
- GNOME: GN-1.02, GN-1.03, GN-1.04, GN-1.07, GN-1.13, GN-1.14, GN-1.16, GN-1.38, GN-1.51, GN-1.65, GN-1.73, GN-1.74, GN-1.80, GN-1.86, GN-1.93, GN-1.96, GN-1.108, GN-1.115, GN-1.125, GN-1.135, GN-1.136, GN-1.148, GN-1.153, GN-1.155, GN-1.207, GN-1.208, GN-1.209, GN-1.251, GN-1.252, GN-1.273, GN-2.11, GN-2.12, GN-2.20, GN-2.21, GN-2.26, GN-2.27, GN-3.31, GN-3.44, GN-3.61
- Material: MAT-X18
- iOS: D-14, D-15, D-16, DL-12, PO-6, SD-3, W-4, X-A11-1, X-CO-1, X-CO-3, X-MT-12, X-SH-3

#### C07 — States and state combinations

- macOS: MAC-B02, MAC-B03, MAC-B09, MAC-B10, MAC-C01, MAC-C02, MAC-C03, MAC-C04, MAC-C07, MAC-CB02, MAC-CB03, MAC-CB04, MAC-EX03, MAC-I02, MAC-L04, MAC-L05, MAC-L11, MAC-L13, MAC-L15, MAC-MN04, MAC-MN05, MAC-MN06, MAC-MN10, MAC-N01, MAC-N03, MAC-N04, MAC-N07, MAC-N08, MAC-N10, MAC-N13, MAC-P03, MAC-S01, MAC-S02, MAC-S08, MAC-SB02, MAC-SG03, MAC-SG05, MAC-SG06, MAC-SL05, MAC-SL06, MAC-SL07, MAC-SP03, MAC-SW03, MAC-TB04, MAC-TL02, MAC-W02, MAC-W09, MAC-X16, MAC-X19, MAC-X21, MAC-X29
- Windows: W1-008, W1-011, W1-019, W1-020, W1-021, W1-022, W1-026, W1-027, W1-028, W1-029, W1-030, W1-033, W1-037, W1-038, W1-041, W1-042, W1-044, W1-048, W1-049, W1-050, W1-061, W1-062, W1-063, W1-067, W1-068, W1-070, W1-074, W1-076, W1-080, W1-081, W1-082, W1-083, W1-087, W1-089, W1-091, W1-099, W1-100, W1-103, W1-106, W1-107, W1-109, W1-110, W1-111, W1-112, W1-118, W1-120, W1-121, W1-122, W1-130, W1-131, W1-132, W1-134, W1-138, W1-139, W1-140, W1-143, W1-144, W2-01, W2-02, W2-03, W2-04, W2-06, W2-10, W2-11, W2-12, W2-13, W2-14, W2-15, W2-16, W2-17, W2-18, W2-19, W2-20, W2-21, W2-22, W2-23, W2-27, W2-28, W2-29, W2-31, W2-32, W2-33, W2-34, W2-38, W3-02
- KDE: K1.7, K1.16, K1.20, K1.21, K1.22, K1.24, K1.25, K1.29, K1.40, K1.43, K1.44, K1.50, K1.51, K1.52, K1.54, K1.56, K1.60, K1.61, K1.91, K1.92, K1.100, K1.101, K1.102, K1.105, K1.112, K1.121, K1.123, K1.124, K1.132, K1.140, K1.152, K1.155, K1.172, K1.174, K1.177, K1.182, K1.240, K1.242, K1.243, K1.244, K1.246, K1.247, K1.250, K2.1, K2.3, K2.4, K2.5, K2.8, K2.9, K2.16, K2.19, K2.21, K3.4, K3.20, K3.23, K3.50, Q1.8, Q1.9, Q1.10, Q1.20, Q1.22, Q1.26, Q1.27, Q1.31, Q1.40, Q1.46, Q1.51, Q1.53, Q1.54, Q1.62, Q1.77, Q1.78, Q1.81, Q1.83, Q1.86, Q1.101, Q1.125, Q1.126, Q1.128, Q1.134, Q2.1, Q2.3, Q2.5, Q2.7, Q2.8, Q2.10, Q2.11, Q2.22, Q2.23, Q2.35, Q3.4, Q3.13, Q3.16, Q3.30, Q3.33
- GNOME: GN-1.05, GN-1.22, GN-1.24, GN-1.25, GN-1.26, GN-1.29, GN-1.30, GN-1.31, GN-1.32, GN-1.34, GN-1.35, GN-1.41, GN-1.44, GN-1.45, GN-1.46, GN-1.53, GN-1.55, GN-1.56, GN-1.57, GN-1.58, GN-1.66, GN-1.67, GN-1.73, GN-1.78, GN-1.79, GN-1.85, GN-1.87, GN-1.88, GN-1.89, GN-1.97, GN-1.98, GN-1.99, GN-1.105, GN-1.114, GN-1.117, GN-1.120, GN-1.123, GN-1.124, GN-1.127, GN-1.133, GN-1.141, GN-1.158, GN-1.159, GN-1.160, GN-1.165, GN-1.166, GN-1.167, GN-1.171, GN-1.172, GN-1.174, GN-1.176, GN-1.198, GN-1.199, GN-1.200, GN-1.201, GN-1.231, GN-1.234, GN-1.237, GN-1.240, GN-1.241, GN-1.242, GN-1.246, GN-1.253, GN-1.257, GN-1.261, GN-1.263, GN-1.265, GN-1.266, GN-1.272, GN-2.03, GN-2.08, GN-2.22, GN-2.25, GN-2.29, GN-2.30, GN-2.31, GN-2.45, GN-3.39, GN-3.42, GN-3.60
- Material: MAT-B1, MAT-B2, MAT-B3, MAT-B4, MAT-B5, MAT-B6, MAT-B7, MAT-B10, MAT-B12, MAT-C1, MAT-C3, MAT-C4, MAT-C5, MAT-C6, MAT-C7, MAT-CA2, MAT-CB1, MAT-CB2, MAT-CB3, MAT-DG3, MAT-EX1, MAT-I1, MAT-I2, MAT-I3, MAT-I5, MAT-I7, MAT-I9, MAT-I10, MAT-I11, MAT-L3, MAT-L4, MAT-L5, MAT-L8, MAT-L9, MAT-L10, MAT-L12, MAT-L13, MAT-M1, MAT-M2, MAT-M3, MAT-N1, MAT-N2, MAT-N4, MAT-N8, MAT-N9, MAT-N12, MAT-N13, MAT-N16, MAT-N17, MAT-N26, MAT-N27, MAT-S1, MAT-S4, MAT-S5, MAT-S8, MAT-SB2, MAT-SB3, MAT-SB8, MAT-SG2, MAT-SG4, MAT-SP2, MAT-SW1, MAT-SW2, MAT-SW3, MAT-SW4, MAT-SW5, MAT-SW6, MAT-T1, MAT-TA1, MAT-TB1, MAT-TB3, MAT-TB4, MAT-X4, MAT-X8, MAT-X12, MAT-X24, MAT-X29
- iOS: B-5, B-6, B-7, B-10, B-14, C-2, C-4, C-6, C-9, C-11, C-12, CB-3, D-20, L-10, LK-1, MN-6, MN-8, SB-4, SD-2, SD-3, SG-2, SG-4, SG-7, SL-3, SL-7, SP-2, SW-2, T-1, T-4, T-5, T-6, T-10, TB-1, TB-6, X-IC-4, X-SE-3

#### C08 — State-layer model

- KDE: K3.31, Q3.16
- Material: MAT-B2, MAT-B5, MAT-B12, MAT-C2, MAT-C5, MAT-CA2, MAT-DG3, MAT-I11, MAT-L4, MAT-L5, MAT-M2, MAT-N1, MAT-N2, MAT-N4, MAT-N9, MAT-N16, MAT-N18, MAT-N20, MAT-N22, MAT-N26, MAT-S7, MAT-SB2, MAT-SG2, MAT-SP2, MAT-SW4, MAT-TB3, MAT-X8, MAT-X9, MAT-X29

#### C09 — Widget variants/styles

- macOS: MAC-B01, MAC-B06, MAC-B07, MAC-B08, MAC-BX02, MAC-CB01, MAC-CB05, MAC-D02, MAC-EX01, MAC-I01, MAC-I08, MAC-L01, MAC-L06, MAC-L12, MAC-MN13, MAC-N03, MAC-N04, MAC-N05, MAC-N06, MAC-N07, MAC-N11, MAC-N12, MAC-N23, MAC-P01, MAC-S03, MAC-SG04, MAC-SG05, MAC-SN04, MAC-SP01, MAC-TB03, MAC-TL03, MAC-TL04, MAC-TL07, MAC-X04, MAC-X05
- Windows: W1-020, W1-021, W1-060, W1-137, W3-43
- KDE: K1.14, K1.24, K1.25, K1.26, K1.41, K1.42, K1.56, K1.62, K1.94, K1.105, K1.120, K1.122, K1.123, K1.152, K1.155, K1.180, K1.240, K1.241, K1.247, K2.6, K2.8, K2.18, K2.21, K3.20, K3.21, K3.44, Q1.8, Q1.9, Q1.11, Q1.41, Q1.43, Q1.45, Q1.76, Q1.95, Q1.107, Q1.115, Q1.118, Q1.123, Q2.4, Q2.5, Q2.6, Q2.18, Q2.20
- GNOME: GN-1.17, GN-1.28, GN-1.29, GN-1.30, GN-1.31, GN-1.32, GN-1.33, GN-1.34, GN-1.35, GN-1.42, GN-1.47, GN-1.54, GN-1.62, GN-1.63, GN-1.112, GN-1.125, GN-1.149, GN-1.151, GN-1.154, GN-1.162, GN-1.164, GN-1.165, GN-1.166, GN-1.170, GN-1.184, GN-1.188, GN-1.205, GN-1.211, GN-1.212, GN-1.213, GN-1.215, GN-1.222, GN-1.224, GN-1.236, GN-1.245, GN-1.246, GN-1.247, GN-1.256, GN-1.266, GN-2.05, GN-2.08, GN-2.20, GN-2.21, GN-2.24, GN-2.29, GN-2.33, GN-2.50, GN-3.09, GN-3.11, GN-3.17
- Material: MAT-B1, MAT-B4, MAT-B10, MAT-C1, MAT-CA1, MAT-CB1, MAT-D1, MAT-I11, MAT-L8, MAT-M10, MAT-N1, MAT-N2, MAT-N3, MAT-N4, MAT-N8, MAT-N9, MAT-N23, MAT-N24, MAT-N25, MAT-N27, MAT-P1, MAT-P4, MAT-S12, MAT-SB6, MAT-SN2, MAT-SN3, MAT-T1, MAT-TA3, MAT-TA6, MAT-TB1, MAT-TS4
- iOS: B-1, B-2, B-6, B-11, B-12, B-13, B-14, C-3, C-5, C-17, C-20, C-22, CB-2, DL-7, EX-3, I-1, L-1, L-4, L-13, MN-7, MN-12, MN-16, PB-1, SB-1, SL-8, T-3, T-13, T-16, TA-1, TB-4, X-A11-5

#### C10 — Size variants and density

- macOS: MAC-B08, MAC-L08, MAC-LY01, MAC-N01, MAC-N10, MAC-N16, MAC-S07, MAC-SB01, MAC-SN03, MAC-SW05, MAC-TB05, MAC-TL01, MAC-X17
- Windows: W1-006, W1-040, W1-047, W1-060, W1-066, W1-085, W1-092, W1-094, W1-099, W1-103, W1-117, W1-122, W1-127, W1-134, W2-20, W3-36, W3-39, W3-40
- KDE: K1.4, K1.28, K1.123, K1.130, K1.154, K1.158, K1.180, K1.181, K3.63, K3.83, Q1.1, Q1.12, Q1.25, Q1.28, Q1.29, Q1.52, Q1.59, Q1.60, Q1.69, Q1.72, Q1.80, Q1.97, Q1.99, Q1.117, Q1.120, Q1.124, Q2.2, Q2.9, Q2.28, Q2.33, Q2.34, Q2.50, Q3.29, Q3.31
- GNOME: GN-1.11, GN-1.21, GN-1.28, GN-1.34, GN-1.70, GN-1.78, GN-1.85, GN-1.92, GN-1.111, GN-1.112, GN-1.119, GN-1.120, GN-1.124, GN-1.128, GN-1.129, GN-1.130, GN-1.140, GN-1.147, GN-1.150, GN-1.164, GN-1.180, GN-1.185, GN-1.188, GN-1.195, GN-1.197, GN-1.212, GN-1.216, GN-1.217, GN-1.218, GN-1.219, GN-1.224, GN-1.227, GN-1.228, GN-1.232, GN-1.238, GN-1.244, GN-1.248, GN-2.05, GN-2.06, GN-2.10, GN-2.11, GN-2.13, GN-2.14, GN-2.15, GN-2.19, GN-2.24, GN-2.26, GN-2.33, GN-2.35, GN-2.36, GN-2.37, GN-2.39, GN-2.41, GN-2.44, GN-3.59, GN-3.63
- Material: MAT-B9, MAT-C9, MAT-L1, MAT-M8, MAT-M10, MAT-N1, MAT-N2, MAT-N5, MAT-N6, MAT-N7, MAT-N9, MAT-N10, MAT-N11, MAT-N13, MAT-N16, MAT-N17, MAT-N20, MAT-N22, MAT-N23, MAT-N25, MAT-N28, MAT-P4, MAT-S1, MAT-S3, MAT-S11, MAT-SN3, MAT-SW3, MAT-T5, MAT-TA3, MAT-X22, MAT-X31
- iOS: B-3, C-1, C-3, DL-8, L-9, LY-1, MN-15, SD-8, SP-3a, T-14, TB-3, X-DT-2, X-SV-1, X-SV-2

#### C11 — Shape

- macOS: MAC-B08, MAC-D03, MAC-L01, MAC-L02, MAC-L11, MAC-MN01, MAC-N02, MAC-N03, MAC-N13, MAC-N21, MAC-P02, MAC-S04, MAC-SB04, MAC-SB06, MAC-SG08, MAC-SL01, MAC-SL03, MAC-SL05, MAC-SW01, MAC-W05, MAC-X04, MAC-X07
- Windows: W1-004, W1-014, W1-045, W1-063, W1-069, W1-077, W1-089, W1-092, W1-096, W1-097, W1-099, W1-107, W1-108, W1-109, W1-120, W1-133, W1-134, W1-141, W2-05, W2-08, W2-09, W2-23, W2-24, W2-28, W2-30, W3-20
- KDE: K1.10, K1.26, K1.53, K1.64, K1.82, K1.90, K1.104, K1.121, K1.128, K1.170, K1.191, K2.11, K2.12, K2.13, Q1.16, Q1.26, Q1.45, Q1.79, Q1.107, Q1.129, Q2.4, Q2.12, Q2.17, Q2.18, Q2.23, Q2.27, Q2.29, Q2.35, Q2.39, Q2.44, Q2.49, Q3.20
- GNOME: GN-1.05, GN-1.06, GN-1.22, GN-1.28, GN-1.42, GN-1.47, GN-1.60, GN-1.61, GN-1.62, GN-1.63, GN-1.68, GN-1.79, GN-1.84, GN-1.94, GN-1.101, GN-1.109, GN-1.124, GN-1.130, GN-1.140, GN-1.143, GN-1.149, GN-1.165, GN-1.174, GN-1.177, GN-1.183, GN-1.211, GN-1.212, GN-1.223, GN-1.224, GN-1.239, GN-1.245, GN-1.261, GN-2.12, GN-2.17, GN-2.25, GN-2.26, GN-2.27, GN-2.29, GN-2.30, GN-2.32, GN-2.35, GN-2.39, GN-2.40, GN-2.41, GN-2.42, GN-2.44, GN-2.50, GN-2.51, GN-3.29
- Material: MAT-B10, MAT-B11, MAT-C2, MAT-EX1, MAT-I11, MAT-L8, MAT-L13, MAT-M10, MAT-N1, MAT-N3, MAT-N5, MAT-N6, MAT-N7, MAT-N8, MAT-N9, MAT-N10, MAT-N11, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N16, MAT-N17, MAT-N18, MAT-N19, MAT-N20, MAT-N21, MAT-N22, MAT-N23, MAT-N24, MAT-N25, MAT-N26, MAT-P5, MAT-S1, MAT-S3, MAT-S11, MAT-SB1, MAT-SB7, MAT-SG3, MAT-SN4, MAT-SP1, MAT-T1, MAT-TB2, MAT-X10, MAT-X11, MAT-X12
- iOS: B-4, C-3, C-11, C-12, C-13, C-21, C-24, C-26, C-30, C-32, L-2, PB-2, SB-5, SD-2, SD-6, SG-1, SL-2, SL-4, SW-3, T-9, TA-1, TB-5, TT-1, W-6, X-CR-1, X-CR-2, X-CR-3, X-CR-4

#### C12 — Border/outline model

- macOS: MAC-B05, MAC-C05, MAC-I04, MAC-I08, MAC-L05, MAC-L06, MAC-L11, MAC-L16, MAC-MN14, MAC-N12, MAC-S03, MAC-S05, MAC-SL02, MAC-ST01, MAC-TB01, MAC-W04, MAC-W06, MAC-X08
- Windows: W1-001, W1-002, W1-018, W1-019, W1-020, W1-026, W1-027, W1-042, W1-046, W1-062, W1-069, W1-079, W1-086, W1-091, W1-097, W1-103, W1-126, W1-131, W1-133, W1-138, W1-141, W1-144, W2-13, W2-30, W2-33, W3-04, W3-43
- KDE: K1.9, K1.11, K1.20, K1.22, K1.40, K1.42, K1.60, K1.90, K1.100, K1.111, K1.128, K1.140, K1.151, K1.160, K1.171, K1.178, K1.222, K1.243, K2.8, K2.9, K2.10, K2.12, K2.13, K2.16, K2.21, Q1.16, Q1.26, Q1.38, Q1.46, Q1.54, Q1.55, Q1.67, Q1.68, Q1.71, Q1.73, Q1.93, Q1.96, Q1.101, Q1.107, Q1.109, Q1.112, Q1.114, Q1.115, Q1.121, Q1.129, Q2.5, Q2.12, Q2.13, Q2.16, Q2.18, Q2.23, Q2.26, Q2.29, Q2.35, Q3.11, Q3.14, Q3.15, Q3.20
- GNOME: GN-1.03, GN-1.04, GN-1.05, GN-1.07, GN-1.08, GN-1.13, GN-1.25, GN-1.38, GN-1.42, GN-1.46, GN-1.51, GN-1.56, GN-1.59, GN-1.65, GN-1.73, GN-1.80, GN-1.86, GN-1.96, GN-1.105, GN-1.108, GN-1.115, GN-1.116, GN-1.133, GN-1.135, GN-1.139, GN-1.146, GN-1.148, GN-1.154, GN-1.157, GN-1.162, GN-1.165, GN-1.169, GN-1.172, GN-1.175, GN-1.180, GN-1.183, GN-1.184, GN-1.186, GN-1.209, GN-1.222, GN-1.251, GN-1.252, GN-2.17, GN-2.20, GN-2.21, GN-2.24, GN-2.25, GN-2.27, GN-2.30, GN-2.32, GN-2.40, GN-3.02, GN-3.03, GN-3.61
- Material: MAT-B1, MAT-B3, MAT-C3, MAT-C4, MAT-CA1, MAT-CA2, MAT-DG4, MAT-I1, MAT-I2, MAT-I4, MAT-I11, MAT-L13, MAT-N4, MAT-N9, MAT-N15, MAT-N18, MAT-N26, MAT-S10, MAT-SG4, MAT-SW5, MAT-TB5
- iOS: CD-1, DL-5, I-1, SB-5, SE-1, SG-1, SG-3, SP-1, SW-6, T-2, TA-1, TB-2, X-HL-1

#### C13 — Parts missing from a widget the model has

- macOS: MAC-B09, MAC-BX01, MAC-C01, MAC-C06, MAC-C07, MAC-CB01, MAC-CB04, MAC-D01, MAC-D02, MAC-EX01, MAC-EX02, MAC-I03, MAC-I06, MAC-L05, MAC-L09, MAC-L10, MAC-L12, MAC-LK01, MAC-MN01, MAC-MN03, MAC-MN04, MAC-MN05, MAC-MN06, MAC-MN07, MAC-MN08, MAC-MN09, MAC-MN12, MAC-MN13, MAC-N01, MAC-N02, MAC-N03, MAC-N04, MAC-N05, MAC-N06, MAC-N07, MAC-N08, MAC-N09, MAC-N10, MAC-N14, MAC-N15, MAC-N17, MAC-N18, MAC-N19, MAC-N21, MAC-N24, MAC-P01, MAC-PO01, MAC-S02, MAC-S05, MAC-S06, MAC-SB03, MAC-SB08, MAC-SG01, MAC-SG07, MAC-SL01, MAC-SL04, MAC-SN01, MAC-SP02, MAC-SP04, MAC-SW01, MAC-SW06, MAC-TB01, MAC-TL02, MAC-TL05, MAC-TL06, MAC-TL08, MAC-W03, MAC-W09, MAC-X14, MAC-X21, MAC-X28
- Windows: W1-007, W1-010, W1-011, W1-015, W1-024, W1-030, W1-031, W1-034, W1-036, W1-038, W1-039, W1-041, W1-043, W1-048, W1-049, W1-050, W1-051, W1-060, W1-061, W1-065, W1-067, W1-071, W1-072, W1-073, W1-075, W1-077, W1-081, W1-082, W1-083, W1-084, W1-089, W1-092, W1-093, W1-095, W1-099, W1-100, W1-101, W1-102, W1-105, W1-107, W1-109, W1-110, W1-118, W1-124, W1-130, W1-132, W1-135, W1-137, W1-140, W1-142, W2-01, W2-02, W2-03, W2-04, W2-05, W2-06, W2-07, W2-08, W2-09, W2-10, W2-11, W2-12, W2-13, W2-14, W2-15, W2-16, W2-17, W2-18, W2-20, W2-21, W2-23, W2-24, W2-25, W2-26, W2-27, W2-28, W2-31, W2-34, W2-35, W2-36, W2-37, W2-38, W2-39, W3-26, W3-27
- KDE: K1.4, K1.6, K1.7, K1.27, K1.43, K1.50, K1.52, K1.55, K1.61, K1.66, K1.67, K1.68, K1.69, K1.72, K1.93, K1.94, K1.96, K1.103, K1.110, K1.112, K1.113, K1.120, K1.124, K1.125, K1.126, K1.127, K1.151, K1.153, K1.155, K1.156, K1.157, K1.160, K1.176, K1.178, K1.179, K1.182, K1.200, K1.201, K1.211, K1.221, K1.222, K1.245, K1.246, K1.250, K2.3, K2.4, K2.5, K2.7, K2.8, K2.9, K2.10, K2.11, K2.14, K2.16, K2.17, K2.19, K2.20, K2.22, K3.44, Q1.9, Q1.10, Q1.14, Q1.15, Q1.16, Q1.17, Q1.18, Q1.19, Q1.20, Q1.29, Q1.30, Q1.31, Q1.32, Q1.39, Q1.44, Q1.45, Q1.48, Q1.50, Q1.61, Q1.62, Q1.63, Q1.65, Q1.70, Q1.73, Q1.75, Q1.76, Q1.77, Q1.82, Q1.83, Q1.85, Q1.86, Q1.88, Q1.92, Q1.99, Q1.100, Q1.109, Q1.110, Q1.114, Q1.116, Q1.118, Q1.119, Q1.121, Q1.122, Q1.131, Q1.132, Q1.136, Q2.1, Q2.11, Q2.13, Q2.19, Q2.20, Q2.24, Q2.29, Q2.30, Q2.32, Q2.34, Q2.38, Q2.39, Q2.41, Q2.42, Q2.43, Q2.44, Q2.48
- GNOME: GN-1.10, GN-1.18, GN-1.27, GN-1.39, GN-1.40, GN-1.41, GN-1.43, GN-1.44, GN-1.48, GN-1.58, GN-1.61, GN-1.72, GN-1.73, GN-1.74, GN-1.75, GN-1.78, GN-1.81, GN-1.100, GN-1.101, GN-1.102, GN-1.103, GN-1.110, GN-1.116, GN-1.117, GN-1.118, GN-1.119, GN-1.120, GN-1.125, GN-1.126, GN-1.127, GN-1.128, GN-1.129, GN-1.130, GN-1.142, GN-1.144, GN-1.146, GN-1.156, GN-1.163, GN-1.168, GN-1.173, GN-1.178, GN-1.180, GN-1.186, GN-1.202, GN-1.226, GN-1.227, GN-1.230, GN-1.232, GN-1.233, GN-1.234, GN-1.235, GN-1.243, GN-1.261, GN-1.267, GN-2.01, GN-2.03, GN-2.06, GN-2.09, GN-2.13, GN-2.15, GN-2.16, GN-2.17, GN-2.21, GN-2.22, GN-2.23, GN-2.24, GN-2.25, GN-2.26, GN-2.28, GN-2.29, GN-2.31, GN-2.32, GN-2.33, GN-2.34, GN-2.35, GN-2.36, GN-2.37, GN-2.38, GN-2.39, GN-2.42, GN-2.43, GN-2.48, GN-3.06, GN-3.62
- Material: MAT-B7, MAT-B9, MAT-B12, MAT-C1, MAT-C2, MAT-C3, MAT-C5, MAT-C6, MAT-CA3, MAT-CB1, MAT-CB2, MAT-CB3, MAT-DG2, MAT-DG3, MAT-DG4, MAT-DG5, MAT-DG7, MAT-EX1, MAT-I3, MAT-I4, MAT-I5, MAT-I6, MAT-I7, MAT-I8, MAT-I9, MAT-I11, MAT-I13, MAT-L1, MAT-L2, MAT-L3, MAT-L6, MAT-L9, MAT-L10, MAT-L11, MAT-L12, MAT-M3, MAT-M4, MAT-M5, MAT-M6, MAT-M10, MAT-N2, MAT-N3, MAT-N4, MAT-N7, MAT-N8, MAT-N9, MAT-N10, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N16, MAT-N17, MAT-N19, MAT-N20, MAT-N22, MAT-N23, MAT-N24, MAT-N25, MAT-N26, MAT-N27, MAT-N28, MAT-P1, MAT-P2, MAT-P3, MAT-P5, MAT-P7, MAT-S1, MAT-S2, MAT-S3, MAT-S4, MAT-S5, MAT-S6, MAT-S7, MAT-S10, MAT-S11, MAT-SB1, MAT-SB3, MAT-SB4, MAT-SB5, MAT-SB6, MAT-SG1, MAT-SN1, MAT-SN2, MAT-SN4, MAT-SN5, MAT-SP1, MAT-SW1, MAT-SW2, MAT-SW3, MAT-SW4, MAT-SW7, MAT-T1, MAT-T4, MAT-TA2, MAT-TA4, MAT-TA5, MAT-TA6, MAT-TB1, MAT-TB2, MAT-TB4, MAT-TB5, MAT-X27
- iOS: B-8, B-9, B-10, B-12, C-1, C-2, C-3, C-4, C-6, C-7, C-9, C-10, C-11, C-12, C-14, C-15, C-16, C-24, C-25, C-26, C-28, C-29, C-30, CB-1, CB-1, DL-2, DL-5, DL-9, EX-1, EX-4, I-2, I-3, I-6, L-3, L-5, L-7, L-8, L-12, L-13, L-15, L-17, L-18, MN-4, MN-5, MN-6, MN-9, MN-10, MN-11, MN-12, MN-13, PO-1, SB-4, SD-4, SD-5, SG-1, SG-3, SG-6, SL-2, SL-5, SL-6, SP-1a, SW-3, SW-4, SW-7, T-3, T-5, T-7, T-8, T-11, T-12, TA-5, TB-5, TB-7, X-A11-6, X-IC-5, X-SE-2

#### C14 — Motion

- macOS: MAC-D04, MAC-EX04, MAC-N02, MAC-P01, MAC-P04, MAC-PO05, MAC-SG09, MAC-SN02, MAC-T04, MAC-TL07, MAC-X04, MAC-X11, MAC-X25
- Windows: W1-009, W1-014, W1-017, W1-023, W1-024, W1-035, W1-039, W1-041, W1-054, W1-058, W1-061, W1-064, W1-065, W1-068, W1-075, W1-078, W1-087, W1-090, W1-093, W1-098, W1-104, W1-107, W1-110, W1-113, W1-116, W1-120, W1-123, W1-128, W1-130, W1-132, W1-135, W1-136, W1-138, W1-142, W2-06, W2-09, W2-12, W2-18, W2-23, W2-26, W2-27, W2-29, W2-30, W2-32, W2-34, W3-07, W3-10, W3-28, W3-30, W3-31, W3-32, W3-33, W3-34, W3-37
- KDE: K1.8, K1.17, K1.30, K1.44, K1.52, K1.55, K1.81, K1.95, K1.106, K1.110, K1.129, K1.132, K1.177, K1.223, K1.230, K1.240, K1.251, K2.3, K2.9, K2.17, K3.14, K3.24, K3.30, K3.31, K3.33, K3.34, K3.35, K3.36, K3.37, K3.90, Q1.18, Q1.24, Q1.34, Q1.40, Q1.41, Q1.48, Q1.55, Q1.56, Q1.57, Q1.58, Q1.59, Q1.61, Q1.64, Q1.67, Q1.74, Q1.84, Q1.88, Q1.89, Q1.91, Q1.94, Q1.102, Q1.104, Q1.105, Q1.111, Q1.119, Q1.120, Q1.125, Q2.3, Q2.10, Q2.11, Q2.14, Q2.15, Q2.21, Q2.35, Q2.38, Q2.40, Q2.43, Q2.52, Q3.19, Q3.22, Q3.23, Q3.24, Q3.25, Q3.26, Q3.27
- GNOME: GN-1.02, GN-1.16, GN-1.22, GN-1.36, GN-1.41, GN-1.50, GN-1.64, GN-1.76, GN-1.90, GN-1.91, GN-1.106, GN-1.113, GN-1.126, GN-1.128, GN-1.130, GN-1.131, GN-1.136, GN-1.155, GN-1.165, GN-1.203, GN-1.204, GN-1.220, GN-1.221, GN-1.229, GN-1.230, GN-1.243, GN-1.249, GN-1.253, GN-1.258, GN-1.259, GN-2.03, GN-2.04, GN-2.11, GN-2.12, GN-2.15, GN-2.17, GN-2.18, GN-2.19, GN-2.35, GN-2.38, GN-3.18, GN-3.19, GN-3.20, GN-3.21, GN-3.22, GN-3.23, GN-3.24, GN-3.25, GN-3.28, GN-3.35, GN-3.44
- Material: MAT-B11, MAT-B13, MAT-C8, MAT-CB3, MAT-DG6, MAT-I14, MAT-M9, MAT-N3, MAT-N5, MAT-N6, MAT-N7, MAT-N12, MAT-N14, MAT-N18, MAT-N19, MAT-N22, MAT-N24, MAT-N25, MAT-N28, MAT-P6, MAT-S6, MAT-SB9, MAT-SC2, MAT-SG1, MAT-SN6, MAT-SW8, MAT-T3, MAT-TA1, MAT-TB6, MAT-W2, MAT-X7, MAT-X9, MAT-X10, MAT-X12, MAT-X13, MAT-X14, MAT-X20
- iOS: B-16, C-1, C-2, C-7, C-9, C-10, C-11, C-12, C-13, C-21, C-22, C-23, C-24, C-33, DL-10, DL-11, EX-2, L-11, L-14, L-16, MN-11, MN-14, PB-3, PO-5, SB-3, SD-7, SG-4, SG-5, SL-9, SP-2a, SP-3, SP-5a, SW-5, T-8, T-10, T-15, TA-2, TA-4, TB-6, TT-2, X-A11-3, X-A11-9, X-FO-2, X-IC-6, X-MO-1, X-MO-2, X-MO-3, X-MO-4, X-MO-5, X-MO-6, X-MO-7, X-MO-8, X-MO-9, X-MO-10, X-MT-8, X-MT-9

#### C15 — Focus indication

- macOS: MAC-L13, MAC-L14, MAC-LK03, MAC-N02, MAC-X13
- Windows: W1-025, W1-026, W1-035, W1-111, W1-145, W3-19, W3-20, W3-21
- KDE: K1.21, K1.57, K1.62, K1.106, K1.129, K1.175, K2.8, K3.20, K3.21, K3.22, K3.23, K3.24, Q1.21, Q1.51, Q1.135, Q2.36, Q3.12, Q3.30
- GNOME: GN-1.30, GN-1.34, GN-1.35, GN-1.37, GN-1.45, GN-1.50, GN-1.61, GN-1.104, GN-1.134, GN-1.161, GN-1.165, GN-1.172, GN-1.174, GN-1.206, GN-1.247, GN-1.250, GN-1.254, GN-1.264, GN-2.03, GN-2.08, GN-2.25, GN-2.27, GN-2.40, GN-3.27, GN-3.28, GN-3.29, GN-3.30, GN-3.31
- Material: MAT-N1, MAT-N4, MAT-N9, MAT-N19, MAT-SW9, MAT-X10
- iOS: D-18, X-FO-1, X-FO-2

#### C16 — Text caret

- macOS: MAC-X12, MAC-X14
- Windows: W1-034, W3-24
- KDE: K3.41, K3.42, K3.43, Q1.14, Q3.31
- GNOME: GN-1.49, GN-1.53, GN-3.32, GN-3.33, GN-3.34, GN-3.35, GN-3.36, GN-3.37, GN-3.38, GN-3.39
- Material: MAT-I9, MAT-I12
- iOS: X-CA-1, X-CA-2, X-CA-3

#### C17 — Selection and highlights

- macOS: MAC-I05, MAC-L03, MAC-L04, MAC-N13, MAC-SB05, MAC-X15, MAC-X21, MAC-X28
- Windows: W1-032, W3-25
- KDE: K1.45, K1.173, K1.174, K3.40, K3.43, K3.62, Q1.13, Q1.14
- GNOME: GN-1.175, GN-1.177, GN-3.40, GN-3.41, GN-3.42, GN-3.43
- iOS: C-25, I-4, I-5, TA-2, TA-3, X-SE-1, X-SE-2, X-SE-3

#### C18 — Typography

- macOS: MAC-BX01, MAC-D01, MAC-I07, MAC-MN04, MAC-MN07, MAC-MN09, MAC-MN15, MAC-N16, MAC-N17, MAC-N20, MAC-N21, MAC-SB01, MAC-SB03, MAC-SG03, MAC-TB05, MAC-TS01, MAC-TS02, MAC-W03, MAC-X18
- Windows: W1-031, W1-048, W1-072, W1-085, W1-093, W1-124, W1-147, W2-02, W2-07, W2-10, W2-24, W2-25, W2-32, W2-37, W3-22, W3-23, W3-26, W3-36
- KDE: K1.66, K1.270, K1.271, K1.272, K2.7, K2.8, K2.15, K3.45, K3.46, K3.47, Q1.2, Q1.3, Q1.4, Q1.5, Q1.62, Q1.63, Q1.66, Q1.72, Q1.87, Q1.114, Q1.131, Q1.135, Q2.27, Q2.30, Q2.34, Q2.38, Q2.41, Q2.42, Q2.47, Q3.32
- GNOME: GN-1.18, GN-1.19, GN-1.39, GN-1.75, GN-1.102, GN-1.110, GN-1.163, GN-1.167, GN-1.189, GN-1.190, GN-1.191, GN-1.192, GN-1.193, GN-1.194, GN-2.01, GN-2.02, GN-2.13, GN-2.14, GN-2.24, GN-2.25, GN-2.28, GN-2.36, GN-2.37, GN-2.40, GN-2.41, GN-3.56
- Material: MAT-B9, MAT-B15, MAT-DG3, MAT-DG7, MAT-DG9, MAT-I3, MAT-I5, MAT-I15, MAT-L1, MAT-L14, MAT-M4, MAT-M11, MAT-N2, MAT-N10, MAT-N12, MAT-N14, MAT-N16, MAT-N17, MAT-N20, MAT-N22, MAT-N23, MAT-S6, MAT-SB4, MAT-SB5, MAT-SB8, MAT-T7, MAT-TA3, MAT-TA4, MAT-TA7, MAT-TB8, MAT-TS1, MAT-TS2, MAT-TS3, MAT-TS4, MAT-TS5, MAT-X16, MAT-X19
- iOS: B-13, C-1, C-11, C-14, C-16, C-32, D-1, D-2, D-3, D-4, D-5, D-6, D-7, D-8, DL-7, I-7, L-4, L-5, SD-4, SG-2, SW-7, TS-1, TS-2, TS-3, TS-4, X-A11-4, X-A11-10, X-DT-1, X-DT-4, X-DT-5

#### C19 — Icons

- macOS: MAC-B09, MAC-IC01, MAC-IC02, MAC-IC03, MAC-MN10, MAC-N20, MAC-SB01, MAC-SB02, MAC-TL02, MAC-X19
- Windows: W1-024, W1-039, W3-27, W3-28
- KDE: K1.6, K1.28, K1.43, K1.70, K1.130, K1.157, K1.158, K1.181, K1.221, K1.230, K2.10, K2.13, K2.14, K2.15, K2.19, K3.60, K3.61, K3.62, K3.63, Q1.1, Q1.15, Q1.18, Q1.19, Q1.27, Q1.73, Q1.75, Q1.80, Q1.87, Q1.88, Q1.110, Q1.114, Q1.116, Q1.119, Q1.136, Q2.4, Q2.19, Q2.24, Q2.27, Q2.29, Q2.30, Q2.43, Q2.44, Q2.48, Q3.17, Q3.18, Q3.19
- GNOME: GN-1.40, GN-1.74, GN-1.119, GN-1.168, GN-1.173, GN-1.202, GN-3.07, GN-3.57, GN-3.58, GN-3.59, GN-3.60
- Material: MAT-B7, MAT-M3, MAT-X27
- iOS: B-16, C-29, CB-1, D-19, SD-5, SP-4a, T-5, X-IC-1, X-IC-2, X-IC-3, X-IC-4, X-IC-5, X-IC-6, X-MO-9

#### C20 — Window chrome

- macOS: MAC-N14, MAC-N16, MAC-PO04, MAC-W01, MAC-W02, MAC-W03, MAC-W04, MAC-W05, MAC-W06, MAC-W07, MAC-W09, MAC-W10
- Windows: W1-001, W1-002, W1-003, W1-004, W1-006, W1-007, W1-008, W1-009, W1-010, W1-011, W1-012, W1-013, W1-014, W1-015, W1-016, W1-017, W3-44
- KDE: K1.1, K1.2, K1.3, K1.4, K1.5, K1.6, K1.7, K1.8, K1.9, K1.10, K1.11, K1.12, K1.13, K1.14, K1.15, K1.16, K1.17, K1.18, K2.11, K3.1, K3.33
- GNOME: GN-1.01, GN-1.02, GN-1.03, GN-1.04, GN-1.05, GN-1.06, GN-1.07, GN-1.08, GN-1.09, GN-1.10, GN-1.11, GN-1.12, GN-1.13, GN-1.14, GN-1.15, GN-1.16, GN-1.17, GN-1.19, GN-1.20, GN-1.21, GN-1.22, GN-1.23, GN-3.44
- Material: MAT-W1, MAT-W2
- iOS: W-1, W-2, W-3, W-4, W-5, W-6, W-9, X-CR-4

#### C21 — Visible timings

- macOS: MAC-N01, MAC-T04, MAC-T05, MAC-X12
- Windows: W1-014, W1-054, W1-059, W1-064, W2-19, W2-26, W2-35, W3-41
- KDE: K1.73, K1.80, K3.36, K3.70, K3.71, K3.72, K3.73, Q1.17, Q1.36, Q1.41, Q1.42, Q1.65, Q1.82, Q2.40, Q2.43, Q2.48, Q2.51, Q3.27, Q3.28, Q3.31
- GNOME: GN-1.77, GN-1.82, GN-1.91, GN-1.132, GN-1.145, GN-1.262, GN-2.12, GN-2.15, GN-2.16, GN-2.38, GN-3.30, GN-3.47, GN-3.48, GN-3.49, GN-3.50, GN-3.51, GN-3.55
- Material: MAT-M5, MAT-N22, MAT-SC1, MAT-T2, MAT-T6, MAT-X25
- iOS: C-4, C-24, SB-3, TT-2, X-BT-1

#### C22 — Pointer

- macOS: MAC-LK02, MAC-X20, MAC-X26
- Windows: W3-29
- KDE: K3.64, Q2.48
- GNOME: GN-3.46
- Material: MAT-X31
- iOS: B-17, X-PH-1, X-PH-2

#### C23 — Accessibility variants as values

- macOS: MAC-SW06, MAC-W08, MAC-X02, MAC-X11, MAC-X22, MAC-X23, MAC-X24, MAC-X25, MAC-X26
- Windows: W1-017, W1-079, W3-05, W3-07, W3-24, W3-35, W3-36, W3-37, W3-38
- KDE: K3.30, K3.90, K3.91, K3.92, K3.93, Q3.25, Q3.31
- GNOME: GN-1.04, GN-1.08, GN-1.22, GN-1.29, GN-1.34, GN-1.38, GN-1.44, GN-1.51, GN-1.65, GN-1.80, GN-1.86, GN-1.87, GN-1.88, GN-1.93, GN-1.105, GN-1.108, GN-1.115, GN-1.127, GN-1.133, GN-1.135, GN-1.148, GN-1.153, GN-1.167, GN-1.202, GN-1.207, GN-1.208, GN-1.209, GN-1.220, GN-1.230, GN-1.251, GN-1.273, GN-2.12, GN-2.18, GN-2.23, GN-2.30, GN-2.35, GN-3.06, GN-3.25, GN-3.26, GN-3.31, GN-3.61, GN-3.62, GN-3.63
- Material: MAT-S6, MAT-TB6, MAT-W1, MAT-X10, MAT-X18, MAT-X19, MAT-X20, MAT-X21
- iOS: B-15, D-15, L-9, L-19, LY-3, LY-4, MN-15, SW-4, SW-6, TS-2, TS-3, X-A11-1, X-A11-2, X-A11-3, X-A11-4, X-A11-5, X-A11-6, X-A11-7, X-A11-8, X-A11-9, X-A11-10, X-DT-2, X-DT-3, X-DT-4, X-MO-10, X-MT-3, X-MT-11, X-MT-12

#### C24 — A whole control the model lacks (the row is in §4.2)

- macOS: MAC-N01, MAC-N02, MAC-N03, MAC-N04, MAC-N05, MAC-N06, MAC-N07, MAC-N08, MAC-N09, MAC-N10, MAC-N11, MAC-N12, MAC-N13, MAC-N14, MAC-N15, MAC-N16, MAC-N17, MAC-N18, MAC-N19, MAC-N20, MAC-N21, MAC-N22, MAC-N23, MAC-N24, MAC-N25, MAC-N26
- Windows: W2-01, W2-02, W2-03, W2-04, W2-05, W2-06, W2-07, W2-08, W2-09, W2-10, W2-11, W2-12, W2-13, W2-14, W2-15, W2-16, W2-17, W2-18, W2-19, W2-20, W2-21, W2-22, W2-23, W2-24, W2-25, W2-26, W2-27, W2-28, W2-29, W2-30, W2-31, W2-32, W2-33, W2-34, W2-35, W2-36, W2-37, W2-38, W2-39
- KDE: K2.1, K2.3, K2.4, K2.5, K2.6, K2.7, K2.8, K2.9, K2.10, K2.11, K2.12, K2.13, K2.14, K2.15, K2.16, K2.17, K2.18, K2.19, K2.20, K2.21, K2.22, Q2.1, Q2.2, Q2.3, Q2.4, Q2.5, Q2.6, Q2.7, Q2.8, Q2.9, Q2.10, Q2.11, Q2.12, Q2.13, Q2.14, Q2.15, Q2.16, Q2.17, Q2.18, Q2.19, Q2.20, Q2.21, Q2.22, Q2.23, Q2.24, Q2.25, Q2.26, Q2.27, Q2.28, Q2.29, Q2.30, Q2.31, Q2.32, Q2.33, Q2.34, Q2.35, Q2.36, Q2.37, Q2.38, Q2.39, Q2.40, Q2.41, Q2.42, Q2.43, Q2.44, Q2.45, Q2.46, Q2.47, Q2.48, Q2.49, Q2.50, Q2.51, Q2.52
- GNOME: GN-2.01, GN-2.02, GN-2.03, GN-2.04, GN-2.05, GN-2.06, GN-2.07, GN-2.08, GN-2.09, GN-2.10, GN-2.11, GN-2.12, GN-2.13, GN-2.14, GN-2.15, GN-2.16, GN-2.17, GN-2.18, GN-2.19, GN-2.20, GN-2.21, GN-2.22, GN-2.23, GN-2.24, GN-2.25, GN-2.26, GN-2.27, GN-2.28, GN-2.29, GN-2.30, GN-2.31, GN-2.32, GN-2.33, GN-2.34, GN-2.35, GN-2.36, GN-2.37, GN-2.38, GN-2.39, GN-2.40, GN-2.41, GN-2.42, GN-2.43, GN-2.44, GN-2.45, GN-2.46, GN-2.48, GN-2.49, GN-2.50, GN-2.51
- Material: MAT-N1, MAT-N2, MAT-N3, MAT-N4, MAT-N5, MAT-N6, MAT-N7, MAT-N8, MAT-N9, MAT-N10, MAT-N11, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N16, MAT-N17, MAT-N18, MAT-N19, MAT-N20, MAT-N21, MAT-N22, MAT-N23, MAT-N24, MAT-N25, MAT-N26, MAT-N27, MAT-N28, MAT-N29
- iOS: C-1, C-2, C-3, C-4, C-5, C-6, C-7, C-8, C-9, C-10, C-11, C-12, C-13, C-14, C-15, C-16, C-17, C-18, C-19, C-20, C-21, C-22, C-23, C-24, C-25, C-26, C-28, C-29, C-30, C-31, C-32, C-33

#### C25 — Reader coverage

- macOS: MAC-I07, MAC-IC03, MAC-T05, MAC-TS02, MAC-X10, MAC-X12, MAC-X13, MAC-X14, MAC-X24, MAC-X26
- Windows: W1-002, W1-007, W1-016, W1-034, W1-054, W1-066, W1-105, W3-01, W3-24, W3-41, W3-43
- KDE: K1.3, K1.5, K1.9, K1.11, K1.13, K1.15, K1.28, K1.62, K1.65, K1.70, K1.94, K1.103, K1.110, K1.131, K1.140, K1.150, K1.153, K1.156, K1.158, K1.175, K1.176, K1.179, K1.183, K1.220, K1.221, K2.10, K3.1, K3.2, K3.11, K3.12, K3.13, K3.14, K3.30, K3.31, K3.33, K3.34, K3.35, K3.36, K3.40, K3.41, K3.42, K3.44, K3.45, K3.46, K3.47, K3.50, K3.51, K3.53, K3.54, K3.63, K3.64, K3.70, K3.71, K3.72, K3.73, K3.83, K3.90, K3.91, K3.92, K3.93, Q1.8, Q1.39, Q1.43, Q1.44, Q1.110, Q3.1, Q3.14, Q3.24
- GNOME: GN-1.19, GN-1.20, GN-1.191, GN-1.193, GN-1.202, GN-3.15, GN-3.16, GN-3.25, GN-3.26, GN-3.30, GN-3.32, GN-3.33, GN-3.34, GN-3.38, GN-3.46, GN-3.47, GN-3.48, GN-3.56, GN-3.62
- Material: MAT-SC1, MAT-T6, MAT-W1, MAT-X3, MAT-X17, MAT-X18, MAT-X19, MAT-X20, MAT-X25
- iOS: D-17, X-CO-4, X-IC-7, X-MT-11, X-MT-13, X-RD-1, X-VR-1

#### C26 — Layout and placement

- macOS: MAC-D01, MAC-L01, MAC-L02, MAC-L07, MAC-L10, MAC-LY01, MAC-MN01, MAC-MN02, MAC-MN03, MAC-MN05, MAC-MN11, MAC-MN15, MAC-MN16, MAC-N09, MAC-N20, MAC-S04, MAC-SB04, MAC-SB06, MAC-SB08, MAC-SL04, MAC-SP04, MAC-ST01, MAC-T03, MAC-TB01, MAC-TB02, MAC-TB03, MAC-TL01, MAC-TL03
- Windows: W1-010, W1-016, W1-036, W1-040, W1-045, W1-046, W1-047, W1-052, W1-059, W1-065, W1-072, W1-085, W1-088, W1-092, W1-094, W1-095, W1-097, W1-099, W1-101, W1-102, W1-108, W1-116, W1-117, W1-119, W1-124, W1-127, W1-133, W1-134, W1-136, W1-141, W1-146, W2-04, W2-11, W2-12, W2-22, W2-25, W2-30, W2-33, W2-37
- KDE: K1.1, K1.2, K1.3, K1.4, K1.5, K1.53, K1.63, K1.67, K1.69, K1.70, K1.71, K1.114, K1.128, K1.130, K1.131, K1.141, K1.154, K1.170, K1.176, K1.179, K1.180, K1.210, K1.211, K1.220, K1.245, K2.1, K2.7, K2.14, K2.22, Q1.5, Q1.6, Q1.7, Q1.12, Q1.15, Q1.16, Q1.20, Q1.21, Q1.25, Q1.28, Q1.29, Q1.30, Q1.32, Q1.35, Q1.49, Q1.50, Q1.52, Q1.55, Q1.59, Q1.60, Q1.61, Q1.66, Q1.67, Q1.69, Q1.70, Q1.72, Q1.74, Q1.75, Q1.79, Q1.85, Q1.87, Q1.90, Q1.92, Q1.93, Q1.94, Q1.97, Q1.98, Q1.99, Q1.103, Q1.108, Q1.109, Q1.113, Q1.115, Q1.116, Q1.117, Q1.118, Q1.124, Q1.130, Q1.131, Q1.133, Q1.136, Q2.1, Q2.4, Q2.9, Q2.12, Q2.13, Q2.19, Q2.20, Q2.24, Q2.27, Q2.30, Q2.32, Q2.33, Q2.34, Q2.38, Q2.40, Q2.41, Q2.42, Q2.45, Q2.46, Q2.47, Q2.50, Q3.29
- GNOME: GN-1.12, GN-1.18, GN-1.20, GN-1.21, GN-1.27, GN-1.39, GN-1.43, GN-1.47, GN-1.61, GN-1.62, GN-1.69, GN-1.71, GN-1.72, GN-1.75, GN-1.79, GN-1.81, GN-1.83, GN-1.85, GN-1.95, GN-1.107, GN-1.111, GN-1.118, GN-1.121, GN-1.125, GN-1.139, GN-1.140, GN-1.142, GN-1.143, GN-1.144, GN-1.147, GN-1.149, GN-1.150, GN-1.152, GN-1.157, GN-1.163, GN-1.164, GN-1.166, GN-1.167, GN-1.170, GN-1.173, GN-1.174, GN-1.181, GN-1.187, GN-1.195, GN-1.196, GN-1.197, GN-1.214, GN-1.215, GN-1.216, GN-1.218, GN-1.222, GN-1.223, GN-1.224, GN-1.233, GN-1.234, GN-1.235, GN-1.238, GN-1.239, GN-1.243, GN-1.245, GN-1.248, GN-2.01, GN-2.10, GN-2.13, GN-2.15, GN-2.16, GN-2.17, GN-2.20, GN-2.21, GN-2.28, GN-2.29, GN-2.30, GN-2.32, GN-2.33, GN-2.34, GN-2.35, GN-2.39, GN-2.40, GN-2.41, GN-2.42, GN-2.43, GN-2.46, GN-2.48, GN-2.50, GN-2.51
- Material: MAT-B4, MAT-B8, MAT-C9, MAT-D1, MAT-DG1, MAT-DG2, MAT-DG8, MAT-I16, MAT-L6, MAT-L7, MAT-LA1, MAT-M6, MAT-M7, MAT-M8, MAT-M10, MAT-N5, MAT-N6, MAT-N7, MAT-N10, MAT-N12, MAT-N13, MAT-N14, MAT-N15, MAT-N18, MAT-N23, MAT-N25, MAT-P2, MAT-S2, MAT-S12, MAT-SB7, MAT-SP3, MAT-T4, MAT-TA3, MAT-TB4, MAT-TB7, MAT-X22
- iOS: B-9, C-12, C-16, C-21, C-32, CD-1, DL-3, DL-4, DL-6, I-3, L-2, L-3, L-6, L-7, L-19, LY-1, LY-2, LY-3, LY-4, MN-3, MN-5, PO-4, SB-2, SD-6, T-9, TB-8, W-7, X-DT-3, X-SV-3

#### C27 — Gradient paint

- macOS: MAC-B05, MAC-C05, MAC-X08, MAC-X09
- Windows: W1-018, W1-020, W1-026, W1-086, W1-131, W3-04
- KDE: K1.14, Q1.56, Q2.6, Q3.21
- GNOME: GN-1.178, GN-2.14, GN-2.18, GN-2.50, GN-3.05, GN-3.06
- iOS: C-30

#### C28 — Colour space and precision

- macOS: MAC-N05, MAC-X10
- iOS: D-17, X-CO-4
