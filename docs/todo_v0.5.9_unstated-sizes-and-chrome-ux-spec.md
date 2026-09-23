# v0.5.9: unstated sizes, and the showcase's chrome UX — spec

Rationale: `todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md` (decisions D1–D8).
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. Model (native-theme)

### 1.1 Which fields may be unstated

A resolved sizing field becomes `Option<f32>` when the platform-facts table marks at least one of the four native platforms **(none)** for it, or when a documented value is asymmetric (D3). Two field groups are known to qualify now:

- `border.padding_horizontal` and `border.padding_vertical` on every widget that has them. §2.2, §2.5, §2.16, §2.26 and §2.27 mark them (none), and §2.14 has no row at all.
- `toolbar.bar_height`. KDE's value is "(none) — sizes to content" (§2.13).

The audit (plan Task 1) lists every other sizing field that some native platform marks (none). The controller's ruling on that list decides which fields join. Only a field a native platform really leaves unstated joins.

### 1.2 Semantics

- `None`: the platform states no value. `Some(0.0)`: it states zero.
- TOML is unchanged. An absent key means `None`.
- The resolver no longer fills a missing padding with `0.0`: the `unwrap_or_default` at `resolve/validate_helpers.rs:283, :337, :587` and `model/resolved.rs:297` go. Validation that uses these fields handles `None` without inventing a number.
- `docs/property-registry.toml` and `docs/inheritance-rules.toml` state the new optionality. The note at `inheritance-rules.toml:58-59` ("0.0 when border.line_width or corner_radius present") is corrected to describe what the code does.

### 1.3 Native presets state what their platform documents (D2)

The native presets are `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, each with its `-live` twin. Each one states every sizing value platform-facts documents for its platform, using the documented number. Each value must also be symmetric per axis, or it stays absent (D3).

Known now:

| Preset | Field | Value | Source |
|---|---|---|---|
| kde-breeze(-live) | `dialog.border.padding_{h,v}` | 10 | §2.22, `Layout_TopLevelMarginWidth` ✅ |
| kde-breeze(-live) | `toolbar.border.padding_horizontal` / `_vertical` | 6 / 0 | §2.13, `ToolBar_ItemMargin` |
| kde-breeze(-live) | `toolbar.bar_height` | absent | §2.13, "(none), sizes to content" |
| adwaita(-live) | `toolbar.border.padding_{h,v}` | 6 / 0 | §2.13 |
| macos-sonoma(-live) | `toolbar.border.padding_{h,v}` | 8 / 0 | §2.13, measured |
| macos-sonoma(-live) | `dialog.border.padding_{h,v}` | 20 / 20 | §2.22, "~20px measured" |
| windows-11(-live) | `dialog.border.padding_{h,v}` | 24 / 24 | §2.22, `ContentDialogPadding` ✅ |
| windows-11(-live) | `toolbar.border.padding_horizontal` | absent | 4 left / 0 right, asymmetric (D3) |
| adwaita(-live) | `dialog.border.padding_vertical` | absent | 32 top / 24 bottom, asymmetric (D3) |

About the macOS dialog row: platform-facts writes "~20", meaning a measured value. The preset states 20 and its comment says "measured".

The audit (Task 1) adds every other documented-but-missing and stated-but-(none) mismatch. The controller rules on each row before Task 5 writes it.

### 1.4 Gate

A test in `native-theme` holds one table row per (native preset, variant, widget, sizing field) for every field in §1.1. Each row carries the expected `Some(v)` or `None` and the platform-facts line it comes from. The test asserts the resolved values against the table.

Discrimination proof: remove `kde-breeze`'s dialog padding, and the test fails naming preset, variant, field and citation.

### 1.5 Status-bar padding (D4)

Research each platform's native status bar, using upstream sources only:

- Qt `QStatusBar` together with Breeze, for KDE;
- GTK3 Adwaita's `statusbar` CSS, for GNOME;
- the Win32 status bar and WinUI, for Windows;
- AppKit, for macOS, which has no status-bar widget.

Append a `border.padding_*` row to platform-facts §2.14, citing each source. A value with no source is **(none)**, with the reason stated. Where the platform does state a symmetric value, the native preset states it.

## 2. Connectors

### 2.1 gpui (`native-theme-gpui`)

- Every `geometry::*` builder that sets padding does so only when the value is `Some`.
- `geometry::toolbar` sets `min_h` only when `bar_height` is `Some`.
- `control_height` adds `2 × padding_vertical` only when that value is stated. The doc says so.
- The builders' doc comments and the tests in `geometry.rs` cover both `Some` and `None`.
- The seams test and `lib.rs:~472` are updated.
- There is no new public API. The one visible difference is that fewer properties are refined.
- The README's builder table and the showcase's `GEOMETRY_NOTES` describe the conditional.

### 2.2 iced (`native-theme-iced`)

- `button_padding` and `input_padding` (lib.rs:273-289) return `Option<Padding>`. `None` means the application keeps iced's own default.
- Their docs say so, and the iced showcase follows.

## 3. Showcase chrome (gpui example)

### 3.1 Status bar (D5)

Status-bar layout, left to right:

- the **left-panel toggle**;
- the environment text;
- (the space between);
- the title of the shown info;
- the **inspector toggle**.

The version leaves the status bar (D7).

Each toggle:

- is built by a `demo::` helper, reports itself, and is a ghost icon Button;
- takes its icon from the chosen set through `chrome_icon`: `PanelLeft…` for the left toggle, `PanelRight…` for the inspector. Use the upstream `IconName` variants that exist; where the set has none, the toggle shows no icon and keeps its tooltip label, per the Task 3 final-fix rule;
- is `selected` while its panel is open;
- dispatches `ToggleSidebar` or `ToggleInspector`, with the tooltip naming the key (Ctrl+B, Ctrl+I).

`ToggleSidebar` keeps collapsing the Sidebar to its icon rail.

### 3.2 Toolbar (D6)

- It holds Command Palette, Reload Theme and Preferences.
- The preset Combobox, the colour-mode ToggleGroup, the icon-set Select, the SidebarToggleButton and the inspector button all leave it.
- `SidebarToggleButton` goes to `docs/showcase-exceptions.toml`, with the reason from rationale §6.

### 3.3 Sidebar header (D6)

`Sidebar::header` (sidebar/mod.rs:276) holds three labelled rows:

- **Theme**: the preset Combobox;
- **Mode**: the colour-mode ToggleGroup;
- **Icons**: the icon-set Select.

The rows follow these rules:

- Each label is page text built by `demo::label`, and reports itself.
- The rows stack vertically at the panel's width. Spacing comes from `geometry::widget_gap` and `container_margin`.
- The header is not rendered while collapsed to the rail.

### 3.4 Title (D7)

The title bar label is `concat!(CARGO_PKG_NAME, " ", CARGO_PKG_VERSION, " showcase")`.

### 3.5 Sidebar icon overlap

The icon and the label of a Sidebar item must not intersect, expanded or collapsed. A windowed test measures their bounds for every page item under two icon sets (gpui built-in and one freedesktop set where available) and asserts no overlap. The fix addresses the cause found, whether that is icon sizing, gap or wrapping.

### 3.6 Info and tests

Every moved or new chrome element carries true info, following the existing rules: claims cite read lines, painted values are shown, notes hold under reduced motion, and ids are unique.

These tests change or are added:

- The toolbar tests.
- The chrome-bar hover test.
- The status bar tests.
- `the_toolbar_is_the_models_toolbar`, which must hold with `bar_height = None`: no `min_h`, and it sizes to its content.

## 4. Docs

- CHANGELOG `[Unreleased]` records:
  - the resolver no longer inventing 0 paddings;
  - `Option` sizing fields;
  - the preset values added;
  - the iced padding functions;
  - the showcase chrome changes.
- `docs/todo.md` gets appended items for:
  - per-side padding (D3);
  - any audit row left open.
- Archive these three documents when done.
