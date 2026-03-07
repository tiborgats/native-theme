# Phase 2: Core Presets - Research

**Researched:** 2026-03-07
**Domain:** Rust compile-time asset embedding, TOML preset files, preset registry API
**Confidence:** HIGH

## Summary

Phase 2 builds on the complete Phase 1 data model to add three bundled theme presets (default, kde-breeze, adwaita) embedded at compile time via `include_str!()`, plus a public API for loading presets by name, listing available presets, and performing TOML serialization/deserialization from strings and files.

The technical domain is straightforward: `include_str!()` embeds TOML files as `&'static str` at compile time, the existing `toml` crate (already a dependency) handles deserialization into `NativeTheme`, and a simple module with static lookup provides the preset registry. The main authoring work is creating accurate, complete TOML preset files for the three themes with both light and dark variants. All infrastructure needed (Rgba hex serde, NativeTheme with light/dark variants, TOML round-trip) is already proven by Phase 1.

The preset TOML files should use the `r##"..."##` double-hash raw string pattern established in Phase 1 for hex color literals. Each preset TOML file lives under `src/presets/` and is `include_str!()`-ed into the binary. The API surface is five functions: `preset()`, `list_presets()`, `from_toml()`, `from_file()`, `to_toml()`.

**Primary recommendation:** Create a `src/presets.rs` module with `include_str!()` for three TOML files under `src/presets/`, expose five public functions, and write comprehensive tests that every bundled preset parses correctly with non-empty color sets and valid font sizes.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PRESET-01 | Bundled core presets embedded via include_str!(): default, kde-breeze, adwaita (light + dark each) | Authoring three TOML files with accurate platform colors; include_str!() embedding pattern; preset directory layout |
| PRESET-02 | Preset loading API: preset(), list_presets(), from_toml(), from_file(), to_toml() | Function signatures and implementations detailed in Architecture Patterns; uses existing toml crate + std::fs |
| TEST-02 | Preset loading tests (all presets parse correctly) | Validation Architecture section with test map; integration test file structure |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| toml | 1.x (already in Cargo.toml) | TOML deserialization of preset files and user files | Already used in Phase 1; handles NativeTheme round-trip |
| serde | 1.x (already in Cargo.toml) | Derive Serialize/Deserialize for all types | Already used in Phase 1 |
| std::include_str!() | stable (1.38+) | Compile-time embedding of TOML preset files | Zero-cost, no runtime file I/O, no extra dependencies |
| std::fs | stable | Reading TOML files from disk (from_file API) | Standard library, no extra dependencies |

### Supporting
No additional dependencies needed. All required functionality is covered by std and existing Cargo.toml dependencies.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| include_str!() | rust-embed crate | Over-engineered for 3 small TOML files; adds proc-macro dep |
| include_str!() | static-toml crate | Generates typed structs at compile time; we already have NativeTheme types and want runtime deserialization |
| Manual TOML file per preset | Single Rust file with inline TOML strings | Separate .toml files are easier to read, edit, validate independently, and reuse outside Rust |

**Installation:**
```bash
# No new dependencies needed
```

## Architecture Patterns

### Recommended Project Structure
```
src/
    presets/
        default.toml         # Default balanced theme (light + dark)
        kde-breeze.toml      # KDE Breeze colors (light + dark)
        adwaita.toml         # GNOME Adwaita colors (light + dark)
    presets.rs               # Preset registry module + public API
    lib.rs                   # Updated with `pub mod presets;` + re-exports
    color.rs                 # (unchanged)
    error.rs                 # (unchanged)
    model/                   # (unchanged)
```

### Pattern 1: Static Preset Registry with include_str!()
**What:** Each TOML file is embedded at compile time. A `preset()` function matches on name and deserializes the embedded string. A static array holds the known preset names.
**When to use:** When preset count is small and known at compile time.
**Example:**
```rust
// src/presets.rs

use crate::{Error, NativeTheme, Result};
use std::path::Path;

// Embed preset TOML files at compile time
const DEFAULT_TOML: &str = include_str!("presets/default.toml");
const KDE_BREEZE_TOML: &str = include_str!("presets/kde-breeze.toml");
const ADWAITA_TOML: &str = include_str!("presets/adwaita.toml");

/// All available preset names.
const PRESET_NAMES: &[&str] = &["default", "kde-breeze", "adwaita"];

/// Load a bundled theme preset by name.
///
/// Returns the preset as a fully populated `NativeTheme` with both
/// light and dark variants.
///
/// # Errors
///
/// Returns `Error::Unavailable` if the preset name is not recognized.
pub fn preset(name: &str) -> Result<NativeTheme> {
    let toml_str = match name {
        "default" => DEFAULT_TOML,
        "kde-breeze" => KDE_BREEZE_TOML,
        "adwaita" => ADWAITA_TOML,
        _ => return Err(Error::Unavailable(
            format!("unknown preset: {name}")
        )),
    };
    from_toml(toml_str)
}

/// List all available bundled preset names.
pub fn list_presets() -> &'static [&'static str] {
    PRESET_NAMES
}

/// Parse a TOML string into a `NativeTheme`.
///
/// # Errors
///
/// Returns `Error::Format` if the TOML is invalid or doesn't
/// match the `NativeTheme` schema.
pub fn from_toml(toml_str: &str) -> Result<NativeTheme> {
    let theme: NativeTheme = toml::from_str(toml_str)?;
    Ok(theme)
}

/// Load a `NativeTheme` from a TOML file at the given path.
///
/// # Errors
///
/// Returns `Error::Unavailable` if the file cannot be read, or
/// `Error::Format` if the contents are not valid theme TOML.
pub fn from_file(path: impl AsRef<Path>) -> Result<NativeTheme> {
    let contents = std::fs::read_to_string(path)?;
    from_toml(&contents)
}

/// Serialize a `NativeTheme` to a TOML string.
///
/// # Errors
///
/// Returns `Error::Format` if serialization fails.
pub fn to_toml(theme: &NativeTheme) -> Result<String> {
    let s = toml::to_string_pretty(theme)?;
    Ok(s)
}
```

### Pattern 2: TOML Preset File Structure
**What:** Each preset TOML file follows the NativeTheme schema with name, light variant, and dark variant.
**When to use:** For every bundled preset file.
**Example:**
```toml
# src/presets/default.toml
name = "Default"

[light.colors.core]
accent = "#3584e4"
background = "#fafafa"
foreground = "#2e3436"
surface = "#f0f0f0"
border = "#c0c0c0"
muted = "#929292"
shadow = "#00000018"

[light.colors.primary]
background = "#3584e4"
foreground = "#ffffff"

# ... (all sections populated)

[light.fonts]
family = "sans-serif"
size = 10.0
mono_family = "monospace"
mono_size = 10.0

[light.geometry]
radius = 6.0
frame_width = 1.0
disabled_opacity = 0.5
border_opacity = 0.15
scroll_width = 8.0

[light.spacing]
xxs = 2.0
xs = 4.0
s = 6.0
m = 12.0
l = 18.0
xl = 24.0
xxl = 36.0

[dark.colors.core]
# ... dark variant colors
```

### Pattern 3: Re-export from lib.rs
**What:** The preset API functions are re-exported from the crate root for ergonomic access.
**When to use:** Always -- users should be able to call `native_theme::preset("default")`.
**Example:**
```rust
// In src/lib.rs, add:
pub mod presets;

// Re-export the five public functions at crate root
pub use presets::{from_file, from_toml, list_presets, preset, to_toml};
```

### Anti-Patterns to Avoid
- **Parsing at compile time with proc macros:** Don't use `static-toml` or custom proc macros. The presets are small; runtime deserialization with `toml::from_str()` on `&'static str` is negligible cost and keeps the build simple.
- **Lazy static / once_cell for parsed presets:** Don't cache parsed NativeTheme in a global static. Each `preset()` call should return a fresh owned `NativeTheme` so callers can freely mutate it (e.g., merge user overrides). The deserialization cost for ~100 lines of TOML is microseconds.
- **HashMap-based registry:** Don't use a HashMap for 3 entries. A match statement is clearer, faster, and catches missing cases at compile time.
- **Putting TOML content inline in Rust source:** Keep preset TOML in separate `.toml` files for readability, independent validation, and potential reuse by external tools.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| TOML parsing | Custom TOML parser | `toml` crate (already dep) | Spec-complete, handles edge cases (multiline strings, dates, escapes) |
| File reading | Custom buffered reader | `std::fs::read_to_string()` | Handles encoding, errors, OS differences |
| Color hex format | Custom hex-to-RGB | Existing `Rgba::from_str()` via serde | Already handles #RGB, #RGBA, #RRGGBB, #RRGGBBAA |
| Error conversion | Manual error mapping | Existing `From<toml::de::Error>` and `From<std::io::Error>` impls | Phase 1 already implemented these conversions |

**Key insight:** Phase 1 already built all the infrastructure. Phase 2 is purely about authoring content (TOML files) and wiring up a thin API layer. Zero new dependencies, zero new types.

## Common Pitfalls

### Pitfall 1: Incorrect Color Values in Preset Files
**What goes wrong:** Preset TOML files contain wrong hex values that don't match the actual platform theme, making the library look amateurish.
**Why it happens:** Colors are transcribed from various sources with different formats (RGB triplets vs hex, different alpha conventions).
**How to avoid:** Source colors from official theme files (KDE Breeze .colors files, libadwaita CSS variables). Double-check with round-trip: parse the TOML, serialize back, compare visually.
**Warning signs:** Colors look obviously wrong when rendered (e.g., dark mode background is light, accent is gray).

### Pitfall 2: Missing Light or Dark Variant
**What goes wrong:** A preset has only light or only dark, violating the success criteria that both variants must be populated.
**Why it happens:** Forgetting to add the second variant, or copy-paste errors where dark uses light colors.
**How to avoid:** Tests that assert `theme.light.is_some() && theme.dark.is_some()` for every preset. Tests that verify dark background is actually darker than light background.
**Warning signs:** `preset("x").unwrap().dark.is_none()` returns true.

### Pitfall 3: include_str!() Path Resolution
**What goes wrong:** `include_str!("presets/default.toml")` fails to compile because the path is resolved relative to the source file, not the crate root.
**Why it happens:** `include_str!()` resolves paths relative to the file containing the macro invocation.
**How to avoid:** Since `presets.rs` is at `src/presets.rs`, the TOML files at `src/presets/default.toml` are referenced as `include_str!("presets/default.toml")` -- this is correct because both are in `src/`.
**Warning signs:** Compilation error "couldn't read file".

### Pitfall 4: Forgetting to Add PRESET_NAMES Entry
**What goes wrong:** A new preset's TOML is embedded with `include_str!()` and matched in `preset()`, but its name is not added to the `PRESET_NAMES` array, so `list_presets()` does not return it.
**Why it happens:** Two separate locations must be updated (match arm + names array).
**How to avoid:** Write a test that verifies every name in `list_presets()` is loadable via `preset()`, and vice versa (every match arm name appears in the list).
**Warning signs:** `list_presets()` count != number of match arms in `preset()`.

### Pitfall 5: TOML Table Naming Mismatch
**What goes wrong:** TOML section headers don't match the struct field names (e.g., `[light.colors.core_colors]` instead of `[light.colors.core]`), causing silent deserialization to defaults.
**Why it happens:** Struct field names don't always match intuitive TOML section names.
**How to avoid:** Verify field names against the actual struct definitions. Use `#[serde(default)]` behavior: if a section is misspelled, all its values silently default to None. Tests must assert that specific color fields are `Some(...)` after loading.
**Warning signs:** A loaded preset has all-None colors despite the TOML file having values.

### Pitfall 6: Foreground Color with Alpha in Adwaita
**What goes wrong:** Adwaita uses `rgb(0 0 6 / 80%)` for foreground colors (CSS format with alpha), which is not a hex string our Rgba parser can handle.
**Why it happens:** Adwaita defines some colors in CSS rgba() format rather than hex.
**How to avoid:** Convert Adwaita's CSS colors to hex when authoring the TOML file. `rgb(0 0 6 / 80%)` is approximately `#000006cc` (0,0,6 with alpha 204/255). Pre-compute these values.
**Warning signs:** TOML parse errors on non-hex color strings.

## Code Examples

Verified patterns from the existing codebase and official sources:

### Loading a Preset and Applying User Overrides
```rust
// Usage pattern that downstream consumers will follow
use native_theme::{preset, from_file, NativeTheme};

fn load_theme() -> native_theme::Result<NativeTheme> {
    // Start with a bundled preset
    let mut theme = preset("kde-breeze")?;

    // Optionally merge user overrides from a file
    if let Ok(user_theme) = from_file("~/.config/my-app/theme.toml") {
        theme.merge(&user_theme);
    }

    Ok(theme)
}
```

### TOML File for KDE Breeze (Key Sections)
```toml
# Sourced from KDE/breeze repository colors/BreezeLight.colors and BreezeDark.colors
name = "KDE Breeze"

[light.colors.core]
accent = "#3daee9"
background = "#eff0f1"
foreground = "#232629"
surface = "#fcfcfc"
border = "#bcc0bf"
muted = "#7f8c8d"
shadow = "#00000040"

[light.colors.status]
danger = "#da4453"
danger_foreground = "#fcfcfc"
warning = "#f67400"
warning_foreground = "#232629"
success = "#27ae60"
success_foreground = "#fcfcfc"
info = "#3daee9"
info_foreground = "#fcfcfc"

[light.colors.interactive]
selection = "#3daee9"
selection_foreground = "#fcfcfc"
link = "#2980b9"
focus_ring = "#3daee9"

# ... remaining sections
```

### Test Pattern: Verify All Presets Parse Correctly
```rust
#[test]
fn all_presets_parse_and_have_both_variants() {
    for name in native_theme::list_presets() {
        let theme = native_theme::preset(name)
            .unwrap_or_else(|e| panic!("preset '{name}' failed to parse: {e}"));

        assert!(
            theme.light.is_some(),
            "preset '{name}' missing light variant"
        );
        assert!(
            theme.dark.is_some(),
            "preset '{name}' missing dark variant"
        );
    }
}

#[test]
fn all_presets_have_nonempty_colors() {
    for name in native_theme::list_presets() {
        let theme = native_theme::preset(name).unwrap();
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        // At minimum, core colors should be populated
        assert!(
            light.colors.core.accent.is_some(),
            "preset '{name}' light missing accent"
        );
        assert!(
            light.colors.core.background.is_some(),
            "preset '{name}' light missing background"
        );
        assert!(
            light.colors.core.foreground.is_some(),
            "preset '{name}' light missing foreground"
        );
        assert!(
            dark.colors.core.accent.is_some(),
            "preset '{name}' dark missing accent"
        );
        assert!(
            dark.colors.core.background.is_some(),
            "preset '{name}' dark missing background"
        );
        assert!(
            dark.colors.core.foreground.is_some(),
            "preset '{name}' dark missing foreground"
        );
    }
}
```

## Preset Color Reference

### KDE Breeze Colors (from official KDE/breeze repository)

**Light variant** (BreezeLight.colors):
| Role | Hex | RGB |
|------|-----|-----|
| accent / focus / selection bg | #3daee9 | 61,174,233 |
| background (window) | #eff0f1 | 239,240,241 |
| foreground | #232629 | 35,38,41 |
| surface (view) | #ffffff | 255,255,255 |
| button bg | #fcfcfc | 252,252,252 |
| border | #bcc0bf | ~188,192,191 |
| muted/inactive fg | #707d8a | 112,125,138 |
| sidebar (header) | #dee0e2 | 222,224,226 |
| tooltip bg | #f7f7f7 | 247,247,247 |
| link | #2980b9 | 41,128,185 |
| danger/negative | #da4453 | 218,68,83 |
| warning/neutral | #f67400 | 246,116,0 |
| success/positive | #27ae60 | 39,174,96 |

**Dark variant** (BreezeDark.colors):
| Role | Hex | RGB |
|------|-----|-----|
| accent / focus / selection bg | #3daee9 | 61,174,233 |
| background (window) | #232629 | ~32,35,38 |
| foreground | #fcfcfc | 252,252,252 |
| surface (view) | #1b1e20 | ~27,30,32 |
| button bg | #31363b | ~49,54,59 |
| muted/inactive fg | #a1a9b1 | 161,169,177 |
| sidebar | #272c31 | ~39,44,49 |
| link | #1d99f3 | 29,153,243 |
| danger | #da4453 | 218,68,83 |
| warning | #f67400 | 246,116,0 |
| success | #27ae60 | 39,174,96 |

### GNOME Adwaita Colors (from libadwaita CSS variables documentation)

**Light variant:**
| Role | Hex | Notes |
|------|-----|-------|
| accent bg | #3584e4 | Blue accent |
| window bg | #fafafb | |
| foreground | #000006cc | rgb(0,0,6) at 80% opacity -- use #2e3436 (GTK convention) or pre-computed |
| header bg | #ffffff | |
| card/surface | #ffffff | |
| sidebar | #ebebed | |
| popover | #ffffff | |
| view bg | #ffffff | |
| destructive/error | #e01b24 | |
| success | #2ec27e | |
| warning | #e5a50a | |
| border | ~15% opacity black | Pre-compute to solid: #d5d5d5 approx |

**Dark variant:**
| Role | Hex | Notes |
|------|-----|-------|
| accent bg | #3584e4 | Same blue accent |
| window bg | #222226 | |
| foreground | #ffffff | |
| header bg | #2e2e32 | |
| card/surface | ~8% white on dark | Pre-compute: #303034 approx |
| sidebar | #2e2e32 | |
| popover | #36363a | |
| view bg | #1d1d20 | |
| destructive/error | #c01c28 | |
| success | #26a269 | |
| warning | #cd9309 | |

### Default Theme (Balanced Neutral)

The "default" preset should be a neutral, toolkit-agnostic theme:
- Use a medium blue accent (#4a90d9 or similar)
- Light: white/near-white backgrounds, dark gray foreground
- Dark: dark gray backgrounds, white/near-white foreground
- Generic "sans-serif" / "monospace" font families
- Standard geometry (6px radius, 1px borders)
- Standard spacing scale (2, 4, 6, 12, 18, 24, 36)

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| embed!()/resource! macros | include_str!() | Stable since Rust 1.38 | No proc macro dependency for text embedding |
| Lazy deserialization with OnceLock | Fresh owned NativeTheme per call | N/A | Caller can mutate freely; deserialization cost is negligible for small TOML |
| GNOME Adwaita hardcoded colors | libadwaita CSS variables system | GNOME 42+ (2022) | Colors can change with accent color preferences; our preset uses defaults |

**Deprecated/outdated:**
- `lazy_static!` for cached parsed themes: Replaced by `std::sync::OnceLock` in Rust 1.70+, but neither is needed here since fresh-per-call is the correct pattern.

## Open Questions

1. **Font family names for presets**
   - What we know: KDE Breeze uses "Noto Sans" (default install), Adwaita now uses "Adwaita Sans" (based on Inter, since GNOME 48), monospace is "Adwaita Mono" (based on Iosevka).
   - What's unclear: Should presets use the actual platform font names (may not be installed cross-platform) or generic names like "sans-serif"?
   - Recommendation: Use actual platform font names in KDE/Adwaita presets (these are the correct values when running on those platforms). Use generic names in the "default" preset. Consuming apps handle font fallback.

2. **Adwaita foreground color with alpha**
   - What we know: libadwaita uses `rgb(0 0 6 / 80%)` for light-mode foreground, which is a near-black with 80% opacity.
   - What's unclear: Should the preset use the alpha-blended color or pre-compute against white background?
   - Recommendation: Pre-compute to a solid color for simplicity. 80% of rgb(0,0,6) on white = approximately #333336. This is the practical color that users see. Alternatively, use the traditional GTK foreground #2e3436 which is what most Adwaita implementations use.

3. **Spacing and geometry values for Adwaita/Breeze**
   - What we know: Exact spacing scales are not standardized in the color scheme files.
   - What's unclear: What are the "correct" spacing values for each platform?
   - Recommendation: Use sensible defaults that match visual inspection of each platform. KDE Breeze: typically 2,4,6,8,12,16,24. Adwaita: typically 2,4,6,12,18,24,36 (6px base grid). Document that these are approximations.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test (cargo test) |
| Config file | Cargo.toml (already configured) |
| Quick run command | `cargo test --lib presets` |
| Full suite command | `cargo test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PRESET-01 | Each preset parses into NativeTheme with light+dark | integration | `cargo test --test preset_loading` | No -- Wave 0 |
| PRESET-01 | All preset colors are non-empty, reasonable values | integration | `cargo test --test preset_loading` | No -- Wave 0 |
| PRESET-02 | preset() returns correct theme by name | unit | `cargo test --lib presets::tests` | No -- Wave 0 |
| PRESET-02 | list_presets() returns all names | unit | `cargo test --lib presets::tests` | No -- Wave 0 |
| PRESET-02 | from_toml() parses valid TOML string | unit | `cargo test --lib presets::tests` | No -- Wave 0 |
| PRESET-02 | from_file() loads from path | unit | `cargo test --lib presets::tests` | No -- Wave 0 |
| PRESET-02 | to_toml() produces valid TOML | unit | `cargo test --lib presets::tests` | No -- Wave 0 |
| PRESET-02 | preset() returns error for unknown name | unit | `cargo test --lib presets::tests` | No -- Wave 0 |
| TEST-02 | All presets round-trip TOML correctly | integration | `cargo test --test preset_loading` | No -- Wave 0 |
| TEST-02 | All presets have valid font sizes (> 0) | integration | `cargo test --test preset_loading` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `src/presets.rs` -- module with API functions and include_str!() constants
- [ ] `src/presets/default.toml` -- default balanced preset
- [ ] `src/presets/kde-breeze.toml` -- KDE Breeze preset with accurate colors
- [ ] `src/presets/adwaita.toml` -- GNOME Adwaita preset with accurate colors
- [ ] `tests/preset_loading.rs` -- integration tests for all presets

## Sources

### Primary (HIGH confidence)
- [KDE/breeze repository - BreezeLight.colors](https://github.com/KDE/breeze/blob/master/colors/BreezeLight.colors) - Official KDE Breeze light color scheme
- [KDE/breeze repository - BreezeDark.colors](https://github.com/KDE/breeze/blob/master/colors/BreezeDark.colors) - Official KDE Breeze dark color scheme
- [libadwaita CSS variables documentation](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/css-variables.html) - Official GNOME Adwaita color values
- [Rust include_str!() documentation](https://doc.rust-lang.org/std/macro.include_str.html) - Path resolution and usage

### Secondary (MEDIUM confidence)
- Existing Phase 1 codebase -- verified working TOML round-trip, Rgba hex serde, Error conversions
- KDE Breeze Light/Dark color files fetched and parsed for RGB values

### Tertiary (LOW confidence)
- Adwaita foreground color opacity calculations (pre-computed values are approximations)
- Spacing/geometry values for presets (visual approximations, not from official specs)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - No new dependencies; uses existing toml crate + std::include_str!()
- Architecture: HIGH - Straightforward module + include_str!() + match pattern; well-established Rust idiom
- Preset colors: HIGH for KDE Breeze (official .colors files), MEDIUM for Adwaita (CSS variables with some alpha/opacity conversion needed)
- Pitfalls: HIGH - Well-understood failure modes (path resolution, missing variants, color transcription errors)

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (stable domain, 30 days)
