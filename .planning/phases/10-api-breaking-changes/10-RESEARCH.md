# Phase 10: API Breaking Changes - Research

**Researched:** 2026-03-08
**Domain:** Rust struct refactoring, serde TOML schema migration, API surface redesign
**Confidence:** HIGH

## Summary

Phase 10 transforms the public API from its v0.1 shape to its final v0.2 shape. There are three major workstreams: (1) flattening `ThemeColors` from 6 nested sub-structs to 36 direct `Option<Rgba>` fields, (2) moving free functions (`preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()`) to `impl NativeTheme` associated functions, and (3) adding `radius_lg` and `shadow` fields to `ThemeGeometry`. All 17 preset TOML files must be migrated from nested `[light.colors.core]`, `[light.colors.status]`, etc. tables to flat `[light.colors]` / `[dark.colors]` tables.

This is a mechanically intensive but conceptually simple refactoring phase. The main risk is not in design decisions but in ensuring every reference site is updated consistently: the 6 sub-structs are used in 3 platform readers (KDE colors.rs, GNOME mod.rs, Windows windows.rs), 17 preset TOML files, all tests (140 total), the `impl_merge!` macro, lib.rs re-exports, README.md code examples, and doc comments. The serde schema change means every field in the flat struct must have unique names (they already do -- `primary_background` vs `button` vs `sidebar` -- because the sub-struct grouping provided namespace isolation, so the flattened names need prefixing like `primary_background` instead of just `background`).

**Primary recommendation:** Execute as three ordered waves -- (1) flatten ThemeColors + update TOML presets, (2) move functions to `impl NativeTheme` + remove old exports, (3) add geometry fields. Each wave is independently compilable and testable.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| API-02 | ThemeColors flattened to 36 direct `Option<Rgba>` fields (no nested sub-structs) | Full field inventory documented; naming scheme resolved; serde/merge/is_empty patterns documented |
| API-03 | All presets updated to flat `[light.colors]` / `[dark.colors]` TOML format | Current nested format analyzed; flat format designed; all 17 files identified |
| API-04 | Platform readers updated for flat ThemeColors field access | KDE colors.rs, GNOME mod.rs, Windows windows.rs analyzed; field mapping documented |
| API-05 | Preset functions moved to `impl NativeTheme` associated functions | Current free-function signatures documented; new method signatures designed |
| API-06 | Old free functions removed (no deprecation period, pre-1.0) | All export sites identified in lib.rs; README references catalogued |
| API-07 | ThemeGeometry gains `radius_lg` and `shadow` fields | Current ThemeGeometry analyzed; new fields specified |
| API-08 | Presets updated with radius_lg and shadow data | Reasonable default values researched per platform |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.0.228 | Derive Serialize/Deserialize on flattened struct | Already in use, workspace dep |
| serde_with | 3.17.0 | `#[skip_serializing_none]` on ThemeColors | Already in use, workspace dep |
| toml | 1.0.6 | TOML ser/de for presets | Already in use, workspace dep |

### Supporting
No new dependencies required. This is a pure refactoring phase using existing stack.

## Architecture Patterns

### Current ThemeColors Structure (BEFORE)
```
ThemeColors
  +-- core: CoreColors (7 fields: accent, background, foreground, surface, border, muted, shadow)
  +-- primary: ActionColors (2 fields: background, foreground)
  +-- secondary: ActionColors (2 fields: background, foreground)
  +-- status: StatusColors (8 fields: danger, danger_foreground, warning, warning_foreground, success, success_foreground, info, info_foreground)
  +-- interactive: InteractiveColors (4 fields: selection, selection_foreground, link, focus_ring)
  +-- panel: PanelColors (6 fields: sidebar, sidebar_foreground, tooltip, tooltip_foreground, popover, popover_foreground)
  +-- component: ComponentColors (7 fields: button, button_foreground, input, input_foreground, disabled, separator, alternate_row)
```
Total: 36 fields across 6 sub-structs (ActionColors reused for primary+secondary).

### Target ThemeColors Structure (AFTER -- Flat)

All 36 fields become direct `Option<Rgba>` fields on `ThemeColors`. Naming must be globally unique since namespace isolation from sub-structs is gone. The naming convention adds the old group name as a prefix where needed to disambiguate:

```rust
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeColors {
    // Core (7) -- no prefix needed, names already unique
    pub accent: Option<Rgba>,
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
    pub surface: Option<Rgba>,
    pub border: Option<Rgba>,
    pub muted: Option<Rgba>,
    pub shadow: Option<Rgba>,

    // Primary action (2)
    pub primary_background: Option<Rgba>,
    pub primary_foreground: Option<Rgba>,

    // Secondary action (2)
    pub secondary_background: Option<Rgba>,
    pub secondary_foreground: Option<Rgba>,

    // Status (8) -- names already unique
    pub danger: Option<Rgba>,
    pub danger_foreground: Option<Rgba>,
    pub warning: Option<Rgba>,
    pub warning_foreground: Option<Rgba>,
    pub success: Option<Rgba>,
    pub success_foreground: Option<Rgba>,
    pub info: Option<Rgba>,
    pub info_foreground: Option<Rgba>,

    // Interactive (4) -- names already unique
    pub selection: Option<Rgba>,
    pub selection_foreground: Option<Rgba>,
    pub link: Option<Rgba>,
    pub focus_ring: Option<Rgba>,

    // Panel (6) -- names already unique
    pub sidebar: Option<Rgba>,
    pub sidebar_foreground: Option<Rgba>,
    pub tooltip: Option<Rgba>,
    pub tooltip_foreground: Option<Rgba>,
    pub popover: Option<Rgba>,
    pub popover_foreground: Option<Rgba>,

    // Component (7) -- names already unique
    pub button: Option<Rgba>,
    pub button_foreground: Option<Rgba>,
    pub input: Option<Rgba>,
    pub input_foreground: Option<Rgba>,
    pub disabled: Option<Rgba>,
    pub separator: Option<Rgba>,
    pub alternate_row: Option<Rgba>,
}
```

### Naming Collision Analysis (HIGH confidence)

Checked all 36 field names for collisions:
- `background` and `foreground` appear in CoreColors, ActionColors (x2), StatusColors (as suffixed), PanelColors (as suffixed), ComponentColors (as suffixed)
- Only CoreColors uses bare `background`/`foreground` -- action colors get `primary_` and `secondary_` prefixes
- All status/panel/component fields already have unique names like `danger`, `sidebar`, `button`, etc.
- The `_foreground` suffixed fields in status/panel/component are already unique (`danger_foreground`, `sidebar_foreground`, `button_foreground`)

### Pattern: impl_merge! Macro Update

The current macro supports `option {}` and `nested {}` categories. After flattening, ThemeColors will use only `option {}` with all 36 fields:

```rust
impl_merge!(ThemeColors {
    option {
        accent, background, foreground, surface, border, muted, shadow,
        primary_background, primary_foreground,
        secondary_background, secondary_foreground,
        danger, danger_foreground, warning, warning_foreground,
        success, success_foreground, info, info_foreground,
        selection, selection_foreground, link, focus_ring,
        sidebar, sidebar_foreground, tooltip, tooltip_foreground,
        popover, popover_foreground,
        button, button_foreground, input, input_foreground,
        disabled, separator, alternate_row
    }
});
```

### Pattern: TOML Format Migration

**Before (nested):**
```toml
[light.colors.core]
accent = "#3584e4"
background = "#fafafb"

[light.colors.primary]
background = "#3584e4"
foreground = "#ffffff"

[light.colors.status]
danger = "#e01b24"
```

**After (flat):**
```toml
[light.colors]
accent = "#3584e4"
background = "#fafafb"
primary_background = "#3584e4"
primary_foreground = "#ffffff"
danger = "#e01b24"
```

### Pattern: Moving Free Functions to impl NativeTheme

**Before (free functions):**
```rust
// In presets.rs
pub fn preset(name: &str) -> Result<NativeTheme> { ... }
pub fn from_toml(toml_str: &str) -> Result<NativeTheme> { ... }
pub fn from_file(path: impl AsRef<Path>) -> Result<NativeTheme> { ... }
pub fn list_presets() -> &'static [&'static str] { ... }
pub fn to_toml(theme: &NativeTheme) -> Result<String> { ... }

// In lib.rs
pub use presets::{from_file, from_toml, list_presets, preset, to_toml};
```

**After (associated functions on NativeTheme):**
```rust
// In model/mod.rs (or a new file)
impl NativeTheme {
    pub fn preset(name: &str) -> Result<Self> { ... }
    pub fn from_toml(toml_str: &str) -> Result<Self> { ... }
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> { ... }
    pub fn list_presets() -> &'static [&'static str] { ... }
    pub fn to_toml(&self) -> Result<String> { ... }
}
```

Key difference: `to_toml()` becomes `&self` method instead of `to_toml(theme: &NativeTheme)`.

### Pattern: ThemeGeometry Extension

**Before:**
```rust
pub struct ThemeGeometry {
    pub radius: Option<f32>,
    pub frame_width: Option<f32>,
    pub disabled_opacity: Option<f32>,
    pub border_opacity: Option<f32>,
    pub scroll_width: Option<f32>,
}
```

**After (two new fields):**
```rust
pub struct ThemeGeometry {
    pub radius: Option<f32>,
    pub radius_lg: Option<f32>,      // NEW: larger radius for dialogs/cards
    pub frame_width: Option<f32>,
    pub disabled_opacity: Option<f32>,
    pub border_opacity: Option<f32>,
    pub scroll_width: Option<f32>,
    pub shadow: Option<bool>,         // NEW: whether platform uses drop shadows
}
```

The `impl_merge!` invocation must add both new fields to the `option {}` list.

### Anti-Patterns to Avoid
- **Partial migration:** Do NOT leave some code using `theme.colors.core.accent` and other code using `theme.colors.accent`. Every reference must be updated atomically per wave.
- **Name collision via `#[serde(flatten)]`:** Do NOT use serde flatten to combine sub-structs -- it creates confusing error messages and prevents `#[serde(deny_unknown_fields)]`. Use explicit flat fields.
- **Keeping sub-struct types public:** After flattening, remove `CoreColors`, `ActionColors`, etc. from public exports. They serve no purpose in the flat model.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| TOML field skipping for None | Custom serialize logic | `#[serde_with::skip_serializing_none]` | Already proven in codebase, handles all Option fields |
| Merge implementation | Manual field-by-field merge | `impl_merge!` macro with `option {}` | Macro already handles this pattern, just expand the field list |
| TOML preset migration | Manual editing of 17 files | Script or systematic find-replace | 17 files x ~36 fields = mechanical, error-prone if done by hand |

**Key insight:** The `impl_merge!` macro already handles the flat `option {}` pattern perfectly. No new infrastructure needed -- just change `nested {}` to `option {}` with all 36 fields listed.

## Common Pitfalls

### Pitfall 1: Forgetting a Reference Site
**What goes wrong:** Compilation fails because some file still references `colors.core.accent` instead of `colors.accent`.
**Why it happens:** 36 fields x multiple reference sites (3 platform readers, tests, presets, README) = hundreds of individual field accesses to update.
**How to avoid:** After changing the struct definition, let the compiler guide you -- `cargo check` will flag every broken reference. Fix systematically by file.
**Warning signs:** Trying to update references manually without compiling between changes.

### Pitfall 2: TOML Key Name Mismatch
**What goes wrong:** Preset TOML files use `background` under `[light.colors]` but the struct field is `primary_background`. Serde silently ignores the mismatch because `#[serde(default)]` fills in `None`.
**Why it happens:** Serde's default behavior ignores unknown keys. A renamed field in the struct doesn't cause a parse error -- it just silently becomes `None`.
**How to avoid:** After migration, add a test that asserts all 36 color fields are `Some(...)` for a preset known to have all fields populated (e.g., "default"). The existing `all_presets_have_nonempty_core_colors` test should be expanded to check ALL 36 fields.
**Warning signs:** Tests pass but preset colors look wrong or missing at runtime.

### Pitfall 3: Breaking the impl_merge! Macro
**What goes wrong:** Adding `primary_background` to the `option {}` list but forgetting `primary_foreground`, causing merge to skip it silently.
**Why it happens:** The macro field list is a plain comma-separated list with no compile-time check against the struct definition.
**How to avoid:** The field list in `impl_merge!()` must match the struct fields exactly. Add a test that merges two fully-populated ThemeColors and verifies all 36 fields propagate.
**Warning signs:** Merge produces `None` for a field that should have been set from overlay.

### Pitfall 4: README Doc Examples Breaking
**What goes wrong:** README.md has doc-tested code examples (```` ```rust ````) that reference `colors.core.accent`, `colors.panel.sidebar`, etc. These are tested via `#[doc = include_str!("../README.md")]` with `#[cfg(doctest)]`.
**Why it happens:** README examples are easy to forget when refactoring internal structs.
**How to avoid:** Update README examples to use flat field access (`colors.accent`, `colors.sidebar`). Run `cargo test --doc` to verify.
**Warning signs:** CI doc tests fail after the struct change.

### Pitfall 5: from_system() and Platform Reader Dispatch
**What goes wrong:** `from_system()` in lib.rs calls `crate::preset("adwaita")` which used to be a free function. After moving to `NativeTheme::preset()`, this call site breaks.
**Why it happens:** Internal code also uses the free functions, not just external callers.
**How to avoid:** Grep for all internal uses of `preset(`, `from_toml(`, `from_file(`, `to_toml(` and update them to `NativeTheme::preset(` etc.
**Warning signs:** `cargo check` catches this immediately.

### Pitfall 6: Serialization Field Order Changes
**What goes wrong:** `to_toml()` round-trip tests fail because the flat struct serializes fields in a different order than the nested version, even though the data is semantically identical.
**Why it happens:** TOML serialization follows struct field declaration order, not nested table order.
**How to avoid:** Round-trip tests should compare parsed values, not raw TOML strings. The existing test `to_toml_produces_valid_round_trip` already does this correctly (compares `name` and `colors.core.accent` values, not strings).
**Warning signs:** String equality assertions on serialized TOML.

## Code Examples

### Flat ThemeColors with serde and merge

```rust
// Source: derived from existing codebase pattern
use serde::{Deserialize, Serialize};
use crate::Rgba;

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
    // Primary (2)
    pub primary_background: Option<Rgba>,
    pub primary_foreground: Option<Rgba>,
    // Secondary (2)
    pub secondary_background: Option<Rgba>,
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
        primary_background, primary_foreground,
        secondary_background, secondary_foreground,
        danger, danger_foreground, warning, warning_foreground,
        success, success_foreground, info, info_foreground,
        selection, selection_foreground, link, focus_ring,
        sidebar, sidebar_foreground, tooltip, tooltip_foreground,
        popover, popover_foreground,
        button, button_foreground, input, input_foreground,
        disabled, separator, alternate_row
    }
});
```

### NativeTheme Associated Functions

```rust
// Source: derived from existing presets.rs
impl NativeTheme {
    /// Load a bundled theme preset by name.
    pub fn preset(name: &str) -> crate::Result<Self> {
        let toml_str = match name {
            "default" => DEFAULT_TOML,
            // ... all 17 presets
            _ => return Err(Error::Unavailable(format!("unknown preset: {name}"))),
        };
        Self::from_toml(toml_str)
    }

    /// Parse a TOML string into a NativeTheme.
    pub fn from_toml(toml_str: &str) -> crate::Result<Self> {
        let theme: NativeTheme = toml::from_str(toml_str)?;
        Ok(theme)
    }

    /// Load a NativeTheme from a TOML file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Self::from_toml(&contents)
    }

    /// List all available bundled preset names.
    pub fn list_presets() -> &'static [&'static str] {
        PRESET_NAMES
    }

    /// Serialize this theme to a TOML string.
    pub fn to_toml(&self) -> crate::Result<String> {
        let s = toml::to_string_pretty(self)?;
        Ok(s)
    }
}
```

### KDE Reader Field Access Migration

```rust
// BEFORE:
ThemeColors {
    core: CoreColors {
        accent: get_color(ini, "Colors:View", "DecorationFocus"),
        background: get_color(ini, "Colors:Window", "BackgroundNormal"),
        // ...
    },
    primary: ActionColors {
        background: get_color(ini, "Colors:Selection", "BackgroundNormal"),
        foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
    },
    // ...
}

// AFTER:
ThemeColors {
    accent: get_color(ini, "Colors:View", "DecorationFocus"),
    background: get_color(ini, "Colors:Window", "BackgroundNormal"),
    // ...
    primary_background: get_color(ini, "Colors:Selection", "BackgroundNormal"),
    primary_foreground: get_color(ini, "Colors:Selection", "ForegroundNormal"),
    // ...
    ..Default::default()
}
```

### Windows Reader Field Access Migration

```rust
// BEFORE:
let mut colors = crate::ThemeColors::default();
colors.core.accent = Some(accent);
colors.core.foreground = Some(fg);
colors.core.background = Some(bg);
colors.interactive.selection = Some(accent);
colors.interactive.focus_ring = Some(accent);
colors.primary.background = Some(accent);

// AFTER:
let mut colors = crate::ThemeColors::default();
colors.accent = Some(accent);
colors.foreground = Some(fg);
colors.background = Some(bg);
colors.selection = Some(accent);
colors.focus_ring = Some(accent);
colors.primary_background = Some(accent);
```

### GNOME Reader Field Access Migration

```rust
// BEFORE:
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    variant.colors.core.accent = Some(*accent);
    variant.colors.interactive.selection = Some(*accent);
    variant.colors.interactive.focus_ring = Some(*accent);
    variant.colors.primary.background = Some(*accent);
}

// AFTER:
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    variant.colors.accent = Some(*accent);
    variant.colors.selection = Some(*accent);
    variant.colors.focus_ring = Some(*accent);
    variant.colors.primary_background = Some(*accent);
}
```

### Flat TOML Preset Format (full example)

```toml
name = "Default"

[light.colors]
accent = "#4a90d9"
background = "#fafafa"
foreground = "#2e3436"
surface = "#ffffff"
border = "#c0c0c0"
muted = "#929292"
shadow = "#00000018"
primary_background = "#4a90d9"
primary_foreground = "#ffffff"
secondary_background = "#6c757d"
secondary_foreground = "#ffffff"
danger = "#dc3545"
danger_foreground = "#ffffff"
warning = "#f0ad4e"
warning_foreground = "#2e3436"
success = "#28a745"
success_foreground = "#ffffff"
info = "#4a90d9"
info_foreground = "#ffffff"
selection = "#4a90d9"
selection_foreground = "#ffffff"
link = "#2a6cb6"
focus_ring = "#4a90d9"
sidebar = "#f0f0f0"
sidebar_foreground = "#2e3436"
tooltip = "#2e3436"
tooltip_foreground = "#f0f0f0"
popover = "#ffffff"
popover_foreground = "#2e3436"
button = "#e8e8e8"
button_foreground = "#2e3436"
input = "#ffffff"
input_foreground = "#2e3436"
disabled = "#c0c0c0"
separator = "#d0d0d0"
alternate_row = "#f5f5f5"

[light.fonts]
# unchanged

[light.geometry]
radius = 6.0
radius_lg = 12.0
frame_width = 1.0
disabled_opacity = 0.5
border_opacity = 0.15
scroll_width = 8.0
shadow = true

[light.spacing]
# unchanged
```

## State of the Art

| Old Approach (v0.1) | Current Approach (v0.2) | Impact |
|---------------------|------------------------|--------|
| `theme.colors.core.accent` | `theme.colors.accent` | Shorter, flatter access path |
| `theme.colors.primary.background` | `theme.colors.primary_background` | No ambiguity, single level |
| `native_theme::preset("x")` | `NativeTheme::preset("x")` | More idiomatic Rust |
| `native_theme::from_toml(s)` | `NativeTheme::from_toml(s)` | Constructor pattern |
| `native_theme::to_toml(&theme)` | `theme.to_toml()` | Method on self, more natural |
| `[light.colors.core]` TOML table | `[light.colors]` flat table | Simpler file format |

## Inventory of Files to Modify

### Struct Definitions
| File | What Changes |
|------|-------------|
| `native-theme/src/model/colors.rs` | Remove 6 sub-structs, replace `ThemeColors` with 36 flat fields |
| `native-theme/src/model/geometry.rs` | Add `radius_lg: Option<f32>` and `shadow: Option<bool>` |
| `native-theme/src/model/mod.rs` | Remove sub-struct re-exports from `pub use colors::*` |
| `native-theme/src/lib.rs` | Remove sub-struct re-exports, remove free function re-exports, update `from_system()` / `from_linux()` |

### Platform Readers
| File | What Changes |
|------|-------------|
| `native-theme/src/kde/colors.rs` | `parse_colors()` returns flat `ThemeColors` |
| `native-theme/src/gnome/mod.rs` | `apply_accent()` uses flat field paths |
| `native-theme/src/windows.rs` | `build_theme()` uses flat field paths |

### Preset Functions
| File | What Changes |
|------|-------------|
| `native-theme/src/presets.rs` | Functions move to `impl NativeTheme` block; module may remain for constants (TOML strings, PRESET_NAMES) |

### TOML Presets (17 files)
All files in `native-theme/src/presets/*.toml` -- convert from nested `[variant.colors.group]` tables to flat `[variant.colors]` tables. Also add `radius_lg` and `shadow` to `[variant.geometry]` sections.

### Documentation
| File | What Changes |
|------|-------------|
| `native-theme/README.md` | Update all code examples to use flat field access and `NativeTheme::preset()` |
| `README.md` (repo root) | Same content, symlink or copy |

### Tests
All tests in `colors.rs`, `mod.rs`, `presets.rs`, `windows.rs`, `kde/colors.rs`, `kde/mod.rs`, `gnome/mod.rs`, and `lib.rs` that reference `colors.core.X`, `colors.primary.X`, `colors.status.X`, etc. must be updated to `colors.X` or `colors.primary_X`.

## ThemeGeometry Values for Presets (API-08)

Reasonable `radius_lg` and `shadow` values per preset, based on platform design guidelines:

| Preset | radius_lg | shadow | Rationale |
|--------|----------|--------|-----------|
| default | 12.0 | true | 2x standard radius; generic shadow |
| kde-breeze | 8.0 | true | Breeze uses moderate radii; has shadows |
| adwaita | 14.0 | true | Adwaita uses large radii for dialogs; has shadows |
| windows-11 | 8.0 | true | WinUI3 uses 8px for flyouts; has shadows |
| macos-sonoma | 10.0 | true | macOS uses generous radii; heavy shadows |
| material | 16.0 | true | Material 3 uses 16dp for large components |
| ios | 13.0 | true | iOS uses 13pt for large elements |
| catppuccin-* | 12.0 | true | Community theme, follows default |
| nord | 12.0 | true | Community theme, follows default |
| dracula | 12.0 | true | Community theme, follows default |
| gruvbox | 8.0 | true | Retro aesthetic, moderate radii |
| solarized | 8.0 | true | Clean aesthetic, moderate radii |
| tokyo-night | 12.0 | true | Modern aesthetic, generous radii |
| one-dark | 8.0 | true | Atom-inspired, moderate radii |

## Open Questions

1. **Where should `impl NativeTheme` preset methods live physically?**
   - What we know: Currently presets.rs contains the functions plus the embedded TOML constants. The `impl NativeTheme` block needs access to those constants.
   - What's unclear: Should the constants stay in presets.rs (which NativeTheme imports), or move to model/mod.rs alongside the impl block?
   - Recommendation: Keep the TOML constants and `PRESET_NAMES` in presets.rs as module-level items, and add the `impl NativeTheme` methods in model/mod.rs using `use crate::presets::*` internally. This keeps the preset data organized separately from the model. Alternatively, the impl block can live in presets.rs -- Rust allows `impl` blocks in any module that has access to the type.

2. **Should `from_system()` become `NativeTheme::from_system()`?**
   - What we know: Requirements API-05 lists `preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()` but NOT `from_system()`.
   - What's unclear: Whether `from_system()` should also move for consistency.
   - Recommendation: Move `from_system()` to `NativeTheme::from_system()` for consistency, even though it's not explicitly required. It follows the same constructor pattern. Also move `from_kde()`, `from_gnome()`, `from_windows()` to `NativeTheme::from_kde()`, etc. But the requirements only mandate the 5 functions listed -- if scope is strict, leave platform readers as free functions for now (Phase 11 will touch them anyway).

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Built-in `#[test]` with cargo test |
| Config file | Workspace Cargo.toml |
| Quick run command | `cargo test -p native-theme` |
| Full suite command | `cargo test` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| API-02 | ThemeColors has 36 flat fields | unit | `cargo test -p native-theme colors::tests` | Exists (update needed) |
| API-03 | All presets parse with flat format | unit | `cargo test -p native-theme presets::tests::all_presets_loadable` | Exists (update needed) |
| API-04 | Platform readers use flat fields | unit | `cargo test -p native-theme --features kde` | Exists (update needed) |
| API-05 | NativeTheme:: methods work | unit | `cargo test -p native-theme presets::tests` | Exists (update needed) |
| API-06 | Old free functions removed | unit | Compile check -- old imports fail | Implicit via compilation |
| API-07 | ThemeGeometry has radius_lg + shadow | unit | `cargo test -p native-theme geometry::tests` | Exists (update needed) |
| API-08 | Presets include new geometry fields | unit | `cargo test -p native-theme presets::tests` | New test needed |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme`
- **Per wave merge:** `cargo test` (full workspace)
- **Phase gate:** Full suite green before verify

### Wave 0 Gaps
- [ ] Expand `all_presets_have_nonempty_core_colors` to check all 36 color fields, not just 3
- [ ] Add test asserting `ThemeGeometry { radius_lg, shadow }` round-trips through TOML
- [ ] Add test asserting `NativeTheme::preset("default")` works (method form)

## Sources

### Primary (HIGH confidence)
- Codebase analysis: `native-theme/src/model/colors.rs` -- current 6 sub-struct definitions (36 fields verified by counting)
- Codebase analysis: `native-theme/src/presets.rs` -- current free function signatures
- Codebase analysis: `native-theme/src/model/geometry.rs` -- current ThemeGeometry (5 fields)
- Codebase analysis: `native-theme/src/kde/colors.rs` -- KDE reader field access patterns
- Codebase analysis: `native-theme/src/gnome/mod.rs` -- GNOME reader field access patterns
- Codebase analysis: `native-theme/src/windows.rs` -- Windows reader field access patterns
- Codebase analysis: All 17 TOML preset files in `native-theme/src/presets/`
- `cargo test` output: 140 tests, all passing

### Secondary (MEDIUM confidence)
- `radius_lg` and `shadow` default values are based on general knowledge of platform design guidelines (Apple HIG, Material Design 3, WinUI3, GNOME HIG)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new dependencies, pure refactoring of existing code
- Architecture: HIGH - complete field inventory done, naming scheme verified collision-free
- Pitfalls: HIGH - all identified from direct codebase analysis, specific file references provided
- Geometry values: MEDIUM - reasonable defaults but could be refined with deeper platform research

**Research date:** 2026-03-08
**Valid until:** 2026-04-08 (stable -- internal refactoring, no external dependencies changing)
