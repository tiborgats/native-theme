# Architecture Patterns

**Domain:** v0.2 feature integration into existing cross-platform Rust theme crate
**Researched:** 2026-03-08
**Confidence:** HIGH -- based on full source code analysis of the existing v0.1 codebase

## Executive Summary

Six v0.2 features integrate into an existing ~7,000 LOC crate with 140+ tests, 17 TOML presets, and 3 platform readers. The core architectural challenge is sequencing changes so that breaking API refactors (ThemeColors flattening, API migration to methods) land first, new data model additions (widget metrics) build on the stabilized model, platform additions (macOS reader) follow the proven reader pattern, and workspace restructuring happens last when the core crate is stable and ready for connector crates to depend on it.

Every change touches the `impl_merge!` macro, the TOML preset files, or both. The merge macro is the single most important integration point -- it generates `merge()` and `is_empty()` for every model struct and must be updated atomically with any struct change.

---

## Recommended Architecture

### v0.2 System Overview (after all changes)

```
                      Consumer (egui / iced / gpui / slint / ...)
                                    |
                 +------------------+------------------+
                 |                  |                  |
        native-theme-gpui   native-theme-iced    (direct use)
        (connector crate)   (connector crate)         |
                 |                  |                  |
                 +--------+---------+------------------+
                          |
               +=========+=========+
               |    native-theme   |  (core crate)
               |   Public API      |
               +=========+=========+
                          |
          +-------+-------+-------+-------+
          |       |       |       |       |
       model/  presets  kde/   gnome/  macos/  windows/
       (flat    (17     (sync) (async) (sync)  (sync)
       colors,  TOML)
       widget
       metrics)
```

### Component Boundaries

| Component | Responsibility | Communicates With | v0.2 Changes |
|-----------|---------------|-------------------|--------------|
| `model/colors.rs` | 36 semantic color roles | All readers, presets, merge macro | **REWRITE**: flatten from 7 nested sub-structs to 36 direct `Option<Rgba>` fields on `ThemeColors` |
| `model/mod.rs` | `NativeTheme`, `ThemeVariant` | Everything | **MODIFY**: add `NativeTheme` associated functions (preset, from_toml, etc.), add `widget_metrics` field to `ThemeVariant` |
| `model/metrics.rs` | NEW: `WidgetMetrics` + 12 per-widget sub-structs | Presets, readers, merge macro | **NEW FILE** |
| `src/macos.rs` | NEW: macOS reader via `objc2-app-kit` | model, error, `from_system()` dispatch | **NEW FILE** |
| `src/lib.rs` | Public API, re-exports, dispatch | All modules | **MODIFY**: add `macos` feature gate, update re-exports for flat colors, remove free function re-exports |
| `src/presets.rs` | Preset loading, TOML API | model | **MODIFY**: move functions to `impl NativeTheme` |
| `presets/*.toml` (17 files) | Bundled theme data | model schema | **REWRITE**: flatten `[light.colors.core]` etc. to `[light.colors]`, add `[light.widget_metrics.*]` sections |
| `src/kde/colors.rs` | KDE color parsing | model/colors.rs | **MODIFY**: update field access from `colors.core.accent` to `colors.accent` |
| `src/gnome/mod.rs` | GNOME portal reader | model/colors.rs | **MODIFY**: update field access for flat colors |
| `src/windows.rs` | Windows reader | model/colors.rs | **MODIFY**: update field access for flat colors |
| Workspace root `Cargo.toml` | NEW: workspace manifest | core crate, connector crates | **NEW FILE** (current `Cargo.toml` moves into `native-theme/`) |
| `native-theme-gpui/` | NEW: gpui connector crate | core crate (path dep), gpui-component (git dep) | **NEW CRATE** |
| `native-theme-iced/` | NEW: iced connector crate | core crate (path dep), iced (crates.io dep) | **NEW CRATE** |

### Data Flow

**Before v0.2 (nested colors):**
```
KDE reader: colors.core.accent = get_color(ini, "Colors:View", "DecorationFocus")
TOML preset: [light.colors.core]\n accent = "#4a90d9"
Test access: variant.colors.core.accent
Merge chain: ThemeColors -> core.merge() -> CoreColors field-by-field
```

**After v0.2 (flat colors):**
```
KDE reader: colors.accent = get_color(ini, "Colors:View", "DecorationFocus")
TOML preset: [light.colors]\n accent = "#4a90d9"
Test access: variant.colors.accent
Merge chain: ThemeColors -> field-by-field (single level, 36 fields)
```

**New widget metrics flow:**
```
Preset TOML:  [light.widget_metrics.button]  ->  serde  ->  ThemeVariant.widget_metrics.button.height
KDE reader:   Hardcoded Breeze constants     ->  direct  ->  ThemeVariant.widget_metrics.button.height
Windows reader: GetSystemMetrics(SM_*)       ->  direct  ->  ThemeVariant.widget_metrics.scrollbar.width
Merge:        base.widget_metrics.merge(&overlay.widget_metrics) -- recursive via impl_merge!
```

---

## Feature 1: Flatten ThemeColors

### Current Structure (v0.1)

```
ThemeColors
  +-- core: CoreColors (7 fields)
  +-- primary: ActionColors (2 fields)
  +-- secondary: ActionColors (2 fields)
  +-- status: StatusColors (8 fields)
  +-- interactive: InteractiveColors (4 fields)
  +-- panel: PanelColors (6 fields)
  +-- component: ComponentColors (7 fields)

impl_merge!(ThemeColors { nested { core, primary, secondary, status, interactive, panel, component } })
```

7 sub-structs, each with its own `impl_merge!` invocation. `ThemeColors` uses `nested {}` variant. TOML has 7 sub-tables per variant: `[light.colors.core]`, `[light.colors.primary]`, etc.

### Target Structure (v0.2)

```rust
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeColors {
    // Core (7)
    pub accent: Option<Rgba>,
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
    pub surface: Option<Rgba>,
    pub border: Option<Rgba>,
    pub muted: Option<Rgba>,
    pub shadow: Option<Rgba>,

    // Actions (4)
    pub primary: Option<Rgba>,
    pub primary_foreground: Option<Rgba>,
    pub secondary: Option<Rgba>,
    pub secondary_foreground: Option<Rgba>,

    // Status (8)
    pub danger: Option<Rgba>,
    pub danger_foreground: Option<Rgba>,
    pub warning: Option<Rgba>,
    pub warning_foreground: Option<Rgba>,
    pub success: Option<Rgba>,
    pub success_foreground: Option<Rgba>,
    pub info: Option<Rgba>,
    pub info_foreground: Option<Rgba>,

    // Interactive (4)
    pub selection: Option<Rgba>,
    pub selection_foreground: Option<Rgba>,
    pub link: Option<Rgba>,
    pub focus_ring: Option<Rgba>,

    // Panel (6)
    pub sidebar: Option<Rgba>,
    pub sidebar_foreground: Option<Rgba>,
    pub tooltip: Option<Rgba>,
    pub tooltip_foreground: Option<Rgba>,
    pub popover: Option<Rgba>,
    pub popover_foreground: Option<Rgba>,

    // Component (7)
    pub button: Option<Rgba>,
    pub button_foreground: Option<Rgba>,
    pub input: Option<Rgba>,
    pub input_foreground: Option<Rgba>,
    pub disabled: Option<Rgba>,
    pub separator: Option<Rgba>,
    pub alternate_row: Option<Rgba>,
}

impl_merge!(ThemeColors {
    option {
        accent, background, foreground, surface, border, muted, shadow,
        primary, primary_foreground, secondary, secondary_foreground,
        danger, danger_foreground, warning, warning_foreground,
        success, success_foreground, info, info_foreground,
        selection, selection_foreground, link, focus_ring,
        sidebar, sidebar_foreground, tooltip, tooltip_foreground,
        popover, popover_foreground,
        button, button_foreground, input, input_foreground,
        disabled, separator, alternate_row,
    }
});
```

### Impact Analysis

**Merge macro:** Changes from `nested { core, primary, ... }` to `option { accent, background, ... }` (36 fields). The macro itself does not change -- only the invocation changes. The 6 sub-struct `impl_merge!` invocations are deleted entirely.

**Naming collision for `primary`/`secondary`:** In v0.1, `colors.primary` is an `ActionColors` struct with `.background` and `.foreground`. In v0.2 flat model, the field names become `primary` (was `primary.background`) and `primary_foreground` (was `primary.foreground`). This is consistent with the pattern already used for `danger`/`danger_foreground`, `selection`/`selection_foreground`, etc.

**ThemeVariant merge:** Currently `ThemeVariant { nested { colors, fonts, geometry, spacing } }`. `colors` stays as `nested` because `ThemeColors` still has its own `merge()` -- it just uses `option` fields instead of `nested` sub-structs internally. Wait -- actually, after flattening, `ThemeColors` only has `option` fields. So `ThemeVariant` still uses `nested { colors, fonts, geometry, spacing }` and calls `colors.merge()`, which now does 36 option field merges directly. This is correct.

**Presets (17 files):** Every TOML file changes. Example transformation:

```toml
# BEFORE (v0.1)
[light.colors.core]
accent = "#4a90d9"
background = "#fafafa"
foreground = "#2e3436"

[light.colors.primary]
background = "#4a90d9"
foreground = "#ffffff"

# AFTER (v0.2)
[light.colors]
accent = "#4a90d9"
background = "#fafafa"
foreground = "#2e3436"
primary = "#4a90d9"
primary_foreground = "#ffffff"
```

**Readers (3 files):**

KDE `kde/colors.rs` -- changes from constructing 7 sub-structs to setting flat fields:
```rust
// BEFORE
ThemeColors {
    core: CoreColors { accent: get_color(...), background: get_color(...), ... },
    primary: ActionColors { background: get_color(...), foreground: get_color(...) },
    ...
}
// AFTER
ThemeColors {
    accent: get_color(ini, "Colors:View", "DecorationFocus"),
    background: get_color(ini, "Colors:Window", "BackgroundNormal"),
    primary: get_color(ini, "Colors:Selection", "BackgroundNormal"),
    primary_foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
    ...
}
```

GNOME `gnome/mod.rs` -- `apply_accent()` changes from `variant.colors.core.accent` to `variant.colors.accent`, etc.

Windows `windows.rs` -- `build_theme()` changes from `colors.core.accent` to `colors.accent`, etc.

**Tests:** Every test that accesses color fields through nested paths must update. In the existing codebase, the pattern `variant.colors.core.accent` appears extensively. Mechanical search-and-replace handles most cases:
- `colors.core.accent` -> `colors.accent`
- `colors.core.background` -> `colors.background`
- `colors.primary.background` -> `colors.primary`
- `colors.primary.foreground` -> `colors.primary_foreground`
- `colors.status.danger` -> `colors.danger`
- `colors.interactive.selection` -> `colors.selection`
- `colors.panel.sidebar` -> `colors.sidebar`
- `colors.component.button` -> `colors.button`

**lib.rs re-exports:** Remove re-exports of `ActionColors`, `ComponentColors`, `CoreColors`, `InteractiveColors`, `PanelColors`, `StatusColors`. Keep re-export of `ThemeColors`.

**Deleted files/code:** `model/colors.rs` is rewritten (sub-struct definitions removed). The 6 sub-struct types and their 6 `impl_merge!` invocations are deleted. Net LOC reduction.

### Serde Considerations

The flat struct with `#[serde_with::skip_serializing_none]` on `ThemeColors` already works -- this is the same pattern used by `ThemeFonts`, `ThemeGeometry`, and `ThemeSpacing` in the existing codebase. The TOML output will be a single `[light.colors]` section with 36 keys, which is significantly simpler for human editing.

No `#[serde(flatten)]` is used. This is intentional -- `serde(flatten)` has known compatibility issues with TOML (alias/default interactions break, error reporting degrades). The flat struct with direct fields is the cleanest approach.

---

## Feature 2: Move Free Functions to `impl NativeTheme`

### Current API (v0.1)

```rust
// src/presets.rs -- free functions
pub fn preset(name: &str) -> Result<NativeTheme> { ... }
pub fn list_presets() -> &'static [&'static str] { ... }
pub fn from_toml(toml_str: &str) -> Result<NativeTheme> { ... }
pub fn from_file(path: impl AsRef<Path>) -> Result<NativeTheme> { ... }
pub fn to_toml(theme: &NativeTheme) -> Result<String> { ... }

// src/lib.rs -- re-exports
pub use presets::{from_file, from_toml, list_presets, preset, to_toml};
```

### Target API (v0.2)

```rust
// src/model/mod.rs -- associated functions on NativeTheme
impl NativeTheme {
    pub fn preset(name: &str) -> Result<NativeTheme> { ... }
    pub fn list_presets() -> &'static [&'static str] { ... }
    pub fn from_toml(toml_str: &str) -> Result<NativeTheme> { ... }
    pub fn from_file(path: impl AsRef<Path>) -> Result<NativeTheme> { ... }
    pub fn to_toml(&self) -> Result<String> { ... }  // now &self method
}
```

### Impact Analysis

**Key change:** `to_toml(&theme)` becomes `theme.to_toml()`. All other functions keep the same signatures but become associated functions.

**`src/presets.rs`:** The module still exists (it holds `include_str!()` constants and the TOML embedding). But the public functions move to `impl NativeTheme` in `model/mod.rs`. The presets module becomes `pub(crate)` with helper functions.

Alternatively, the functions can stay in `presets.rs` physically but be exposed via `impl NativeTheme` using a delegation pattern:

```rust
// model/mod.rs
impl NativeTheme {
    pub fn preset(name: &str) -> crate::Result<NativeTheme> {
        crate::presets::preset_impl(name)  // delegate to presets module
    }
}
```

This keeps the preset TOML data and parsing logic co-located with the TOML files (important for `include_str!()` relative paths), while exposing the API through `NativeTheme`. This is the recommended approach because `include_str!()` paths are relative to the source file, so the TOML loading logic must remain in `src/presets.rs`.

**lib.rs re-exports:** Remove `pub use presets::{from_file, from_toml, list_presets, preset, to_toml}`. The functions are now accessible via `NativeTheme::preset()`, `NativeTheme::from_toml()`, etc. Since `NativeTheme` is already re-exported, no additional re-exports needed.

**Breaking change:** All call sites change from `native_theme::preset("name")` to `NativeTheme::preset("name")`. This is a pre-1.0 breaking change -- acceptable.

**Internal call sites in the crate itself:** `crate::preset("adwaita")` in `gnome/mod.rs` becomes `crate::NativeTheme::preset("adwaita")` or `NativeTheme::preset("adwaita")`.

**from_system():** Currently in `lib.rs`. Could remain as a free function (it dispatches across platforms) or also become `NativeTheme::from_system()`. The todo.md spec implies `NativeTheme::from_system()` based on the usage example `NativeTheme::from_system()?`. Making it an associated function is consistent.

**Tests:** Mechanical changes. Replace `preset("name")` with `NativeTheme::preset("name")`, `from_toml(s)` with `NativeTheme::from_toml(s)`, `to_toml(&theme)` with `theme.to_toml()`.

---

## Feature 3: macOS Reader Module

### Architecture Pattern

Follow the exact same pattern as existing readers: a module at `src/macos.rs` (or `src/macos/mod.rs` if sub-modules needed), feature-gated behind `macos` feature flag, producing a `NativeTheme` from platform APIs.

### Module Structure

```rust
// src/macos.rs (or src/macos/mod.rs)
//! macOS theme reader: reads NSColor semantic colors, NSFont system fonts,
//! and NSAppearance light/dark state from AppKit.

/// Read the current macOS theme from AppKit.
///
/// Reads ~20 NSColor semantic properties, system/monospace fonts,
/// and appearance (light/dark) to produce a NativeTheme.
pub fn from_macos() -> crate::Result<crate::NativeTheme> {
    // 1. Resolve NSAppearance to determine light/dark
    // 2. Read NSColor semantic colors -> ThemeColors
    // 3. Read NSFont.systemFont + monospacedSystemFont -> ThemeFonts
    // 4. Hardcoded AppKit geometry (radius ~5px, etc.)
    // 5. Construct NativeTheme with appropriate variant
    ...
}
```

### NSColor Mapping to Flat ThemeColors

Based on `objc2-app-kit` NSColor availability (HIGH confidence from docs.rs):

| ThemeColors field | NSColor source |
|-------------------|----------------|
| `accent` | `controlAccentColor()` |
| `background` | `windowBackgroundColor()` |
| `foreground` | `labelColor()` |
| `surface` | `controlBackgroundColor()` |
| `border` | `separatorColor()` |
| `muted` | `secondaryLabelColor()` |
| `shadow` | None (NSColor has no shadow color) |
| `primary` | `controlAccentColor()` |
| `primary_foreground` | white/black based on accent luminance |
| `secondary` | `controlColor()` |
| `secondary_foreground` | `controlTextColor()` |
| `danger` | `systemRedColor()` |
| `warning` | `systemOrangeColor()` |
| `success` | `systemGreenColor()` |
| `info` | `systemBlueColor()` |
| `selection` | `selectedContentBackgroundColor()` |
| `selection_foreground` | `selectedControlTextColor()` |
| `link` | `linkColor()` |
| `focus_ring` | `keyboardFocusIndicatorColor()` |
| `sidebar` | `underPageBackgroundColor()` |
| `tooltip` | derive from `windowBackgroundColor()` |
| `button` | `controlColor()` |
| `button_foreground` | `controlTextColor()` |
| `input` | `textBackgroundColor()` |
| `input_foreground` | `textColor()` |
| `disabled` | `disabledControlTextColor()` |
| `separator` | `separatorColor()` |
| `alternate_row` | `alternatingContentBackgroundColors()` index 1 |

### Color Space Conversion

NSColor often returns colors in Display P3 or extended sRGB. The reader must convert to sRGB via `colorUsingColorSpace(_:)` before extracting RGB components. Pattern:

```rust
fn nscolor_to_rgba(color: &NSColor) -> Option<Rgba> {
    // Convert to sRGB color space
    let srgb = unsafe { NSColorSpace::sRGBColorSpace() };
    let converted = unsafe { color.colorUsingColorSpace(&srgb) }?;

    // Extract components
    let r = unsafe { converted.redComponent() };
    let g = unsafe { converted.greenComponent() };
    let b = unsafe { converted.blueComponent() };
    let a = unsafe { converted.alphaComponent() };

    Some(Rgba::from_f32(r as f32, g as f32, b as f32, a as f32))
}
```

### NSAppearance Resolution

Semantic NSColors return different values depending on the active appearance. Before reading colors, the reader must resolve the current effective appearance:

```rust
fn is_dark_appearance() -> bool {
    // NSApp.effectiveAppearance or NSAppearance.currentDrawing
    // Check bestMatchFromAppearancesWithNames for "NSAppearanceNameDarkAqua"
    ...
}
```

### Feature Flag Integration

```toml
# Cargo.toml
[features]
macos = ["dep:objc2-app-kit", "dep:objc2-foundation"]

[target.'cfg(target_os = "macos")'.dependencies]
objc2-app-kit = { version = "0.3", optional = true, default-features = false, features = [
    "NSColor", "NSColorSpace", "NSFont", "NSAppearance", "NSApplication"
] }
objc2-foundation = { version = "0.3", optional = true }
```

### Dispatch Integration

```rust
// src/lib.rs -- update from_system()
pub fn from_system() -> crate::Result<NativeTheme> {
    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "macos")]
        return crate::macos::from_macos();

        #[cfg(not(feature = "macos"))]
        return Err(crate::Error::Unsupported);
    }
    // ... existing linux/windows dispatch
}
```

This follows the exact pattern already used for the `windows` reader dispatch.

### Testable Core Pattern

Like `kde/mod.rs::from_kde_content()`, `gnome/mod.rs::build_theme()`, and `windows.rs::build_theme()`, the macOS reader should have a testable core that takes raw color values and produces a `NativeTheme`, separate from the Objective-C API calls:

```rust
/// Testable core: given extracted color values, build a NativeTheme.
fn build_theme(
    colors: ThemeColors,
    fonts: ThemeFonts,
    is_dark: bool,
) -> NativeTheme {
    let variant = ThemeVariant { colors, fonts, ..Default::default() };
    if is_dark {
        NativeTheme { name: "macOS".into(), light: None, dark: Some(variant) }
    } else {
        NativeTheme { name: "macOS".into(), light: Some(variant), dark: None }
    }
}
```

---

## Feature 4: Widget Metrics Data Model

### Design Principles

1. **Same pattern as existing model structs:** all `Option<f32>` fields, `#[non_exhaustive]`, `#[serde(default)]`, `#[serde_with::skip_serializing_none]`, `impl_merge!`.
2. **`WidgetMetrics` is nested in `ThemeVariant`** as `Option<WidgetMetrics>` (not required).
3. **Each widget gets its own sub-struct** for clean TOML sections and targeted merging.
4. **`impl_merge!` uses `nested {}` for `WidgetMetrics`** (delegates to each sub-struct's `merge()`), and `option {}` for each widget sub-struct.

### Struct Hierarchy

```
ThemeVariant
  +-- colors: ThemeColors          (existing, flattened)
  +-- fonts: ThemeFonts            (existing)
  +-- geometry: ThemeGeometry      (existing)
  +-- spacing: ThemeSpacing        (existing)
  +-- widget_metrics: WidgetMetrics  (NEW)
        +-- button: ButtonMetrics
        +-- checkbox: CheckboxMetrics
        +-- input: InputMetrics
        +-- scrollbar: ScrollbarMetrics
        +-- slider: SliderMetrics
        +-- progress: ProgressBarMetrics
        +-- tab: TabMetrics
        +-- menu_item: MenuItemMetrics
        +-- tooltip: TooltipMetrics
        +-- list_item: ListItemMetrics
        +-- toolbar: ToolbarMetrics
        +-- splitter: SplitterMetrics
```

### Example Sub-Struct

```rust
// src/model/metrics.rs

/// Per-button sizing and spacing metrics.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ButtonMetrics {
    /// Minimum button height in logical pixels.
    pub height: Option<f32>,
    /// Minimum button width in logical pixels.
    pub min_width: Option<f32>,
    /// Horizontal padding inside the button.
    pub padding_h: Option<f32>,
    /// Vertical padding inside the button.
    pub padding_v: Option<f32>,
    /// Spacing between icon and text.
    pub icon_spacing: Option<f32>,
    /// Corner radius (overrides ThemeGeometry.radius for buttons).
    pub radius: Option<f32>,
}

impl_merge!(ButtonMetrics {
    option { height, min_width, padding_h, padding_v, icon_spacing, radius }
});
```

### WidgetMetrics Container

```rust
/// Per-widget sizing metrics organized by widget type.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WidgetMetrics {
    #[serde(default, skip_serializing_if = "ButtonMetrics::is_empty")]
    pub button: ButtonMetrics,
    #[serde(default, skip_serializing_if = "CheckboxMetrics::is_empty")]
    pub checkbox: CheckboxMetrics,
    // ... 10 more widget sub-structs
}

impl_merge!(WidgetMetrics {
    nested { button, checkbox, input, scrollbar, slider, progress,
             tab, menu_item, tooltip, list_item, toolbar, splitter }
});
```

### Integration with ThemeVariant

```rust
// model/mod.rs -- ThemeVariant gains one field
pub struct ThemeVariant {
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub geometry: ThemeGeometry,
    pub spacing: ThemeSpacing,
    #[serde(default, skip_serializing_if = "WidgetMetrics::is_empty")]
    pub widget_metrics: WidgetMetrics,  // NEW
}

impl_merge!(ThemeVariant {
    nested { colors, fonts, geometry, spacing, widget_metrics }  // add widget_metrics
});
```

### TOML Serialization Format

```toml
[light.widget_metrics.button]
height = 32.0
min_width = 80.0
padding_h = 16.0
padding_v = 6.0
icon_spacing = 8.0
radius = 6.0

[light.widget_metrics.scrollbar]
width = 12.0
min_thumb = 20.0
track_radius = 6.0
```

This naturally produces nested TOML tables. Unlike the ThemeColors flattening decision, nesting is appropriate here because widget metrics are logically independent units -- you never merge "all button properties" with "all scrollbar properties." Each widget type IS a meaningful unit.

### Merge Behavior

The merge chain for widget metrics is: `ThemeVariant::merge()` -> `WidgetMetrics::merge()` -> `ButtonMetrics::merge()` (per-field Option replacement). This means a preset can define button metrics, and a platform reader can overlay scrollbar metrics without clobbering the button values. The existing merge architecture handles this naturally.

---

## Feature 5: Cargo Workspace Restructuring

### Target Layout

```
native-theme/                     # workspace root
  Cargo.toml                      # [workspace] manifest (virtual or root)
  native-theme/                   # core crate (what is currently the entire repo)
    Cargo.toml                    # [package] manifest -- published to crates.io
    src/
      lib.rs
      model/
      presets.rs
      presets/                    # TOML files
      kde/
      gnome/
      macos.rs
      windows.rs
      ...
  native-theme-gpui/              # connector crate
    Cargo.toml                    # path dep: native-theme = { path = "../native-theme" }
    src/lib.rs
    examples/showcase.rs
  native-theme-iced/              # connector crate
    Cargo.toml                    # path dep: native-theme = { path = "../native-theme" }
    src/lib.rs
    examples/demo.rs
```

### Migration Steps

1. Create workspace root `Cargo.toml` with `[workspace]` section.
2. Move current crate contents into `native-theme/` subdirectory.
3. Update all `include_str!()` relative paths (they are relative to the source file, so they should NOT need changes if `src/presets.rs` stays in the same relative position to `src/presets/*.toml`).
4. Update `.gitignore`, CI config, etc. for new layout.
5. Create connector crate directories with their own `Cargo.toml`.

### Workspace Manifest Options

**Option A: Virtual manifest (recommended)**

```toml
# workspace root Cargo.toml
[workspace]
members = [
    "native-theme",
    "native-theme-gpui",
    "native-theme-iced",
]
resolver = "3"

[workspace.dependencies]
native-theme = { path = "native-theme" }
serde = { version = "1.0", features = ["derive"] }
```

Virtual manifest is preferred because there is no "root package" -- the core crate and connectors are peers. The workspace root is only for workspace config, shared dependencies, and workspace-level commands (`cargo test --workspace`).

**Option B: Root package manifest**

The workspace root could also be the core crate (`[package]` + `[workspace]` in the same file). This avoids moving files but creates ambiguity about which crate is "the" package. For a crate that will be published to crates.io, having it in its own subdirectory is cleaner.

### crates.io Publishing

Only `native-theme` (core) publishes initially. Connector crates depend on `native-theme` via `path` locally and `version` on crates.io:

```toml
# native-theme-iced/Cargo.toml
[dependencies]
native-theme = { path = "../native-theme", version = "0.2" }
```

The `path` is used for local development; `version` is used when publishing (Cargo strips `path` from published manifests). For `native-theme-gpui`, which depends on `gpui-component` via git, it cannot be published to crates.io until gpui-component is.

### Shared Workspace Dependencies

```toml
# workspace root Cargo.toml
[workspace.dependencies]
native-theme = { path = "native-theme" }
serde = { version = "1.0", features = ["derive"] }
toml = "1.0"
```

Connector crates reference these with `dep.workspace = true`:
```toml
# native-theme-iced/Cargo.toml
[dependencies]
native-theme = { workspace = true }
```

---

## Feature 6: Connector Crate Architecture

### Design Principle: Thin Mapping Layers

Connector crates are pure mapping code -- they translate `NativeTheme` fields to toolkit-specific types. They contain:
1. A mapping function (e.g., `apply()`)
2. No business logic, no data model extensions, no platform code
3. Examples demonstrating the integration

### gpui-component Connector

```rust
// native-theme-gpui/src/lib.rs

use native_theme::{NativeTheme, ThemeVariant};

/// Apply a NativeTheme to the gpui-component theme system.
pub fn apply(theme: &NativeTheme, cx: &mut gpui::AppContext) {
    let variant = pick_variant(theme, cx);
    if let Some(v) = variant {
        apply_colors(v, cx);
        apply_fonts(v, cx);
        apply_geometry(v, cx);
        if let Some(metrics) = &v.widget_metrics {
            apply_widget_metrics(metrics, cx);
        }
    }
}

fn pick_variant<'a>(theme: &'a NativeTheme, cx: &gpui::AppContext) -> Option<&'a ThemeVariant> {
    // Use gpui's appearance detection to pick light/dark variant
    let is_dark = cx.window_appearance().is_dark();
    if is_dark { theme.dark.as_ref() } else { theme.light.as_ref() }
}

fn apply_colors(variant: &ThemeVariant, cx: &mut gpui::AppContext) {
    // Map ThemeColors flat fields -> gpui-component ThemeColor tokens
    // e.g., variant.colors.accent -> theme_color.accent (via hsl conversion if needed)
    // variant.colors.background -> theme_color.background
    // variant.colors.primary -> theme_color.primary
    ...
}
```

gpui-component uses an `ActiveTheme` trait with `ThemeColor` struct containing color tokens accessed via `cx.theme()`. The connector maps flat `ThemeColors` fields to `ThemeColor` properties. Key challenge: gpui-component colors use HSL internally (Tailwind-style color scales), while `native-theme` uses sRGB hex. The connector must convert.

**Dependency:**
```toml
# native-theme-gpui/Cargo.toml
[dependencies]
native-theme = { workspace = true }
gpui = { git = "https://github.com/zed-industries/zed", package = "gpui" }
gpui-component = { git = "https://github.com/longbridge/gpui-component" }
```

### iced Connector

```rust
// native-theme-iced/src/lib.rs

use native_theme::{NativeTheme, ThemeVariant, Rgba};

/// Create an iced Theme from a NativeTheme variant.
pub fn to_iced_theme(variant: &ThemeVariant) -> iced::Theme {
    let palette = to_palette(variant);
    iced::Theme::Custom(Arc::new(iced::theme::Custom::new("native".into(), palette)))
}

fn to_palette(variant: &ThemeVariant) -> iced::theme::Palette {
    iced::theme::Palette {
        background: rgba_to_iced(variant.colors.background),
        text: rgba_to_iced(variant.colors.foreground),
        primary: rgba_to_iced(variant.colors.accent),
        success: rgba_to_iced(variant.colors.success),
        danger: rgba_to_iced(variant.colors.danger),
    }
}

fn rgba_to_iced(color: Option<Rgba>) -> iced::Color {
    match color {
        Some(c) => {
            let [r, g, b, a] = c.to_f32_array();
            iced::Color { r, g, b, a }
        }
        None => iced::Color::BLACK,
    }
}
```

iced uses a `Palette` with 5 core colors (`background`, `text`, `primary`, `success`, `danger`) that an `Extended` palette derives from. The `Custom::new(name, palette)` constructor auto-generates the extended palette. For finer control, `Custom::with_fn(name, palette, generator)` allows overriding extended palette generation.

**Dependency:**
```toml
# native-theme-iced/Cargo.toml
[dependencies]
native-theme = { workspace = true }
iced = "0.13"
```

### Connector Testing Strategy

Connectors are thin enough that integration tests with real toolkit types are the right testing level. Example tests verify:
1. All `ThemeColors` fields map to toolkit colors (no field forgotten)
2. `None` values produce sensible defaults
3. Light/dark variant selection works
4. Round-trip: `NativeTheme` -> toolkit theme -> verify key colors match

---

## Patterns to Follow

### Pattern: Atomic Struct + Macro Changes

**What:** When modifying a struct that has an `impl_merge!` invocation, always update the struct fields and the macro invocation in the same commit.

**Why:** If the struct gains a field but the macro invocation is not updated, the new field silently gets no merge/is_empty behavior. Tests may pass because `Default` fills in `None`, but merge overlays silently ignore the new field.

**Example:**
```rust
// BAD: add field to struct but forget macro
pub struct ThemeGeometry {
    pub radius: Option<f32>,
    pub radius_lg: Option<f32>,  // NEW -- but not in impl_merge! below
    ...
}
impl_merge!(ThemeGeometry { option { radius, frame_width, ... } }); // Missing radius_lg!

// GOOD: always update both together
pub struct ThemeGeometry {
    pub radius: Option<f32>,
    pub radius_lg: Option<f32>,  // NEW
    ...
}
impl_merge!(ThemeGeometry { option { radius, radius_lg, frame_width, ... } }); // Included
```

### Pattern: Testable Core for Platform Readers

**What:** Every platform reader has a `build_theme()` or equivalent function that takes raw values (not platform API types) and produces a `NativeTheme`.

**Why:** Platform API calls are untestable in CI (no KDE session, no D-Bus, no macOS, no Windows). The testable core validates the mapping logic with fixture data.

**Existing examples:** `kde::from_kde_content()`, `gnome::build_theme()`, `windows::build_theme()`. The macOS reader must follow this pattern.

### Pattern: include_str! Co-location

**What:** TOML preset files live adjacent to the Rust code that loads them via `include_str!()`.

**Why:** `include_str!()` paths are relative to the source file. Moving the Rust file (e.g., during workspace restructure) without moving the TOML files breaks compilation. Keeping them co-located prevents this.

**Implication for workspace restructure:** When moving `src/presets.rs` to `native-theme/src/presets.rs`, the `src/presets/*.toml` files must move to `native-theme/src/presets/*.toml` in the same commit.

---

## Anti-Patterns to Avoid

### Anti-Pattern: Nested ThemeColors for Widget Metrics Mapping

**What:** Following the v0.1 ThemeColors sub-struct pattern for WidgetMetrics.

**Why bad:** ThemeColors flattening is driven by the human-editable TOML argument (one section vs. 7). Widget metrics are different: each widget IS a meaningful group, and TOML sections like `[light.widget_metrics.button]` are natural. The nesting is appropriate here.

**Instead:** Use nested sub-structs for WidgetMetrics (each widget type is a struct), flat fields for ThemeColors (all 36 colors are peers).

### Anti-Pattern: Workspace Restructure Before API Stabilization

**What:** Restructuring to a workspace first, then doing API-breaking changes.

**Why bad:** Every workspace member's tests must pass after each change. Breaking changes to the core crate's API force simultaneous updates to connector crates. If connectors are created before the API is stable, every API change creates cascading fixes.

**Instead:** Do all API-breaking work (flatten colors, move to methods) in the single-crate phase. Only restructure to workspace when the core crate's public API is stable for v0.2.

### Anti-Pattern: Connector Crates with Their Own Data Model

**What:** Having connector crates define intermediate types between `NativeTheme` and toolkit types.

**Why bad:** Adds a third data model layer with its own conversion bugs. The whole point of `native-theme` is one canonical model.

**Instead:** Connectors map directly from `NativeTheme`/`ThemeVariant` fields to toolkit types. Zero intermediate types. If a toolkit needs a type that doesn't exist in `native-theme`, the connector uses a local constant or derives it from existing fields.

---

## Scalability Considerations

| Concern | Current (v0.1) | v0.2 | Future |
|---------|---------------|------|--------|
| Color field count | 36 (nested in 7 structs) | 36 (flat) | Add to flat struct, update macro. Non-breaking due to `#[non_exhaustive]` |
| Widget metric types | 0 | 12 sub-structs | Add new sub-struct, add to `WidgetMetrics` nested list. Non-breaking |
| Platform readers | 3 (KDE, GNOME, Windows) | 4 (+macOS) | Add new `src/[platform].rs` + feature flag. Independent of existing readers |
| Connector crates | 0 | 2 (gpui, iced) | Add new workspace member directory. Independent of existing connectors |
| Preset files | 17 | 17 (with widget_metrics) | Add new .toml, add to `PRESET_NAMES` array + match arm. Mechanical |
| Binary size per preset | ~2-4 KB | ~3-6 KB (widget_metrics) | Negligible even at 50 presets |

---

## Build Order (Dependency-Driven)

This is the critical integration order. Each step builds on the previous.

### Phase A: API Breaking Changes (do first, single crate)

**Step A1: Flatten ThemeColors**
- Modify: `model/colors.rs` (rewrite), `model/mod.rs` (ThemeVariant merge stays `nested`), `lib.rs` (remove sub-struct re-exports)
- Modify: all 17 preset TOML files
- Modify: `kde/colors.rs`, `gnome/mod.rs`, `windows.rs` (field access paths)
- Modify: all tests touching color fields
- This is the largest single change in v0.2 by diff size

**Step A2: Move Free Functions to impl NativeTheme**
- Modify: `model/mod.rs` (add associated functions), `presets.rs` (make internals `pub(crate)`)
- Modify: `lib.rs` (remove free function re-exports)
- Modify: `gnome/mod.rs` (internal `crate::preset()` -> `NativeTheme::preset()`)
- Modify: `lib.rs` dispatch (make `from_system()` an associated function)
- Modify: all tests and doctests using free functions

**Step A3: ThemeGeometry Additions**
- Modify: `model/geometry.rs` (add `radius_lg`, `shadow` fields)
- Modify: `impl_merge!` invocation for `ThemeGeometry`
- Modify: presets that have large-radius or shadow data
- Small, mechanical change

### Phase B: New Data Model (builds on stable API)

**Step B1: Widget Metrics Structs**
- New: `model/metrics.rs` with 12 sub-structs + `WidgetMetrics` container
- Modify: `model/mod.rs` (add `widget_metrics` to `ThemeVariant`, update merge macro)
- Modify: `lib.rs` (add re-exports for metric types)
- Does NOT require preset updates yet (empty metrics are skipped in serialization)

**Step B2: Widget Metrics in Presets**
- Modify: preset TOML files to include `[light.widget_metrics.*]` sections
- Research required: extract actual values from Breeze, Adwaita, Windows, macOS sources

### Phase C: Platform Addition

**Step C1: macOS Reader**
- New: `src/macos.rs` (or `src/macos/mod.rs`)
- Modify: `Cargo.toml` (add `macos` feature + `objc2-app-kit` dep)
- Modify: `lib.rs` (add `macos` module declaration + dispatch)
- Independent of A/B changes (only depends on the flat ThemeColors from A1)

**Step C2: Windows Reader Enhancements**
- Modify: `src/windows.rs` (add accent shades, system font, spacing)
- Modify: `Cargo.toml` (add `Win32_System_SystemInformation` feature to windows dep)

**Step C3: Linux Reader Enhancements**
- Modify: `src/kde/mod.rs` (portal overlay), `src/gnome/mod.rs` (gsettings fonts)
- Modify: `src/lib.rs` (smarter fallback in `from_linux()`)

### Phase D: CI Pipeline

**Step D1: GitHub Actions**
- New: `.github/workflows/ci.yml`
- Cross-platform matrix: Linux, Windows, macOS
- Feature flag matrix: no-features, kde, portal-tokio, windows, macos

### Phase E: Workspace + Connectors (last -- needs stable core)

**Step E1: Workspace Restructuring**
- Move entire crate into `native-theme/` subdirectory
- Create workspace root `Cargo.toml`
- Update CI paths
- Verify `cargo test --workspace` passes

**Step E2: gpui Connector Crate**
- New: `native-theme-gpui/` directory with `Cargo.toml`, `src/lib.rs`, examples
- Depends on: stable core API, gpui-component git dep

**Step E3: iced Connector Crate**
- New: `native-theme-iced/` directory with `Cargo.toml`, `src/lib.rs`, examples
- Depends on: stable core API, iced crates.io dep

### Phase F: Publishing Prep

**Step F1: Cargo.toml Metadata**
- `rust-version`, `repository`, `homepage`, `keywords`, `categories`
- License files, CHANGELOG.md

**Build Order Rationale:**
- A before B: API-breaking changes must land before new features build on the API.
- A1 before A2: Flattening colors is the largest refactor; moving functions is smaller and cleaner when done on the already-flat API.
- B before C: Widget metrics struct must exist before readers can populate it.
- C independent: Each platform reader can be developed in parallel once the model is stable.
- D after C: CI needs all platform features to exist to test them.
- E last: Workspace restructure is a repo organization change, not a functionality change. Doing it last means all code changes are done in the simpler single-crate layout.

---

## Sources

- Existing codebase analysis: full source read of all 14 `.rs` files and 17 `.toml` presets (HIGH confidence)
- [docs/todo.md](/home/tibi/Rust/native-theme/docs/todo.md) -- v0.2 feature specifications (HIGH confidence)
- [docs/IMPLEMENTATION.md](/home/tibi/Rust/native-theme/docs/IMPLEMENTATION.md) -- original design spec (HIGH confidence)
- [serde field attributes](https://serde.rs/field-attrs.html) -- flatten/skip_serializing_if behavior (HIGH confidence)
- [serde_with skip_serializing_none](https://docs.rs/serde_with/latest/serde_with/attr.skip_serializing_none.html) -- attribute used by existing codebase (HIGH confidence)
- [serde flatten + TOML limitations](https://github.com/serde-rs/serde/issues/1879) -- default/flatten incompatibility (HIGH confidence)
- [Cargo workspaces reference](https://doc.rust-lang.org/cargo/reference/workspaces.html) -- workspace member config, virtual manifests (HIGH confidence)
- [objc2-app-kit NSColor docs](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSColor.html) -- available semantic color methods (HIGH confidence)
- [gpui-component theme documentation](https://longbridge.github.io/gpui-component/docs/theme) -- ThemeColor/ActiveTheme system (MEDIUM confidence)
- [iced Custom theme](https://docs.iced.rs/iced/widget/theme/struct.Custom.html) -- Palette + Custom theme creation (MEDIUM confidence)

---
*Architecture research for: native-theme v0.2 feature integration*
*Researched: 2026-03-08*
