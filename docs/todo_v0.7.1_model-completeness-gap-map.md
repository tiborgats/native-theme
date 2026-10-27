# v0.7.1 — Model completeness: the gap map

Status: in progress (started 2026-10-27, branch `v0.7.1-rc1`, model at caeddd6b).

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

Row format: **ID** · element · what is missing · platforms that draw it (and how each
states it: *read*, *published* with the source, or *measure* with the method) · what the
model has today (the nearest field, or none) · seed (where a connector or document
already recorded the gap).

### 4.1 Elements the model has, with parts, states or properties it lacks

(filled per platform)

### 4.2 Standard controls the model does not have

(filled per platform)

### 4.3 Properties that cross widgets

(filled per platform: elevation and shadows, translucent materials, motion, focus, the
caret, selection, window chrome, size variants, accessibility variants)

## 5. Seeds

Gaps already written down somewhere, each to be confirmed and given a row in §4: the
connectors' "the model states no …" notes, their mapping contracts, `docs/todo.md`, and
the archived specs.

(filled)
