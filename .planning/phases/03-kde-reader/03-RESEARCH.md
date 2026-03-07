# Phase 3: KDE Reader - Research

**Researched:** 2026-03-07
**Domain:** Linux KDE desktop theme reading (kdeglobals INI parsing, color/font mapping)
**Confidence:** HIGH

## Summary

This phase implements `from_kde()`, a synchronous function that reads the user's live KDE Plasma theme from `~/.config/kdeglobals` (respecting `XDG_CONFIG_HOME`) and maps KDE's INI-format color groups and font strings to the existing `NativeTheme` data model. The core challenge is parsing KDE's non-standard INI format (colons in section names like `[Colors:View]`, comma-separated RGB values, Qt font strings) and mapping KDE's 60+ color roles across 5 color groups down to the 36 semantic color roles in `ThemeColors`.

The `configparser` crate (v3.1.0) is the right tool for INI parsing, but requires specific configuration: case-sensitive mode (`new_cs()`) AND custom delimiters (`vec!['=']` only) to correctly handle KDE's case-sensitive key names (`BackgroundNormal` vs `backgroundnormal`) and avoid treating `:` as a key-value delimiter. This has been empirically verified against real kdeglobals files. KDE font strings follow the Qt `QFont::toString()` format with either 10 fields (Qt4 legacy) or 16+ fields (Qt5/Qt6); only fields 0 (family name) and 1 (point size) are needed for our `ThemeFonts`.

KDE kdeglobals contains a single color scheme (no separate light/dark variants). The `from_kde()` function should detect whether the active scheme is dark or light by examining the `[Colors:Window]` `BackgroundNormal` luminance, and populate the appropriate variant in `NativeTheme`. This is the first feature-gated module (`feature = "kde"`), requiring Cargo.toml changes and conditional compilation via `#[cfg(feature = "kde")]`.

**Primary recommendation:** Use `configparser` 3.1.0 with `Ini::new_from_defaults()` configured for case-sensitive + equals-only delimiter, parse the 5 standard color groups + `[WM]` + `[General]` sections, and produce a single-variant `NativeTheme` (light OR dark based on luminance detection).

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-01 | Linux KDE reader: from_kde() -- sync, parses ~/.config/kdeglobals via configparser (feature "kde") | configparser 3.1.0 API verified, INI parsing tested with real kdeglobals, color group mapping documented, font parsing format confirmed, feature gating pattern established |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| configparser | 3.1.0 | INI file parsing for kdeglobals | Zero dependencies, case-sensitive mode, custom delimiters, handles KDE section names with colons. Verified empirically. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| dirs | 6.0 (or `std::env`) | Resolve XDG_CONFIG_HOME | For finding kdeglobals path. Could also use `std::env::var("XDG_CONFIG_HOME")` with fallback to `$HOME/.config` to avoid the dependency. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| configparser | rust-ini | rust-ini is also zero-dep and handles INI files, but configparser's `IniDefault` system gives explicit control over delimiters and case sensitivity needed for KDE files |
| configparser | Manual parsing | KDE INI files have edge cases (comments with `#`, multiline values in some configs, empty values like `ChangeSelectionColor=`); a tested parser avoids subtle bugs |
| dirs crate | std::env::var | `std::env::var("XDG_CONFIG_HOME")` + fallback to `$HOME/.config` is 5 lines of code; avoids adding a dependency just for one path lookup |

**Installation (Cargo.toml):**
```toml
[features]
kde = ["dep:configparser"]

[dependencies]
configparser = { version = "3.1", optional = true }
```

## Architecture Patterns

### Recommended Project Structure
```
src/
├── lib.rs              # Add: #[cfg(feature = "kde")] pub mod kde;
├── kde/
│   ├── mod.rs          # pub fn from_kde() -> Result<NativeTheme>
│   ├── colors.rs       # KDE color group parsing + mapping to ThemeColors
│   └── fonts.rs        # Qt font string parsing -> ThemeFonts
├── color.rs            # Existing Rgba type
├── error.rs            # Existing Error enum
├── model/              # Existing theme model structs
└── presets/            # Existing preset system
```

### Pattern 1: Feature-Gated Module
**What:** The entire `kde` module is behind `#[cfg(feature = "kde")]` and only compiled when the feature is enabled.
**When to use:** Always for platform-specific readers.
**Example:**
```rust
// src/lib.rs
#[cfg(feature = "kde")]
pub mod kde;

// Re-export the main function at crate root when feature is enabled
#[cfg(feature = "kde")]
pub use kde::from_kde;
```

### Pattern 2: Graceful Degradation with Error Types
**What:** Return `Error::Unavailable` for missing files/sections, `Error::Format` for malformed data, never panic.
**When to use:** All error paths in `from_kde()`.
**Example:**
```rust
pub fn from_kde() -> crate::Result<NativeTheme> {
    let path = kdeglobals_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| Error::Unavailable(format!("cannot read {}: {e}", path.display())))?;

    let mut ini = create_kde_parser();
    ini.read(content)
        .map_err(|e| Error::Format(format!("cannot parse kdeglobals: {e}")))?;

    // Parse colors from each group, skipping missing groups gracefully
    let colors = parse_colors(&ini);
    let fonts = parse_fonts(&ini);
    // ... build NativeTheme
}
```

### Pattern 3: Helper for RGB Parsing
**What:** Parse "R,G,B" strings into `Rgba` with validation.
**When to use:** Every color value from kdeglobals.
**Example:**
```rust
fn parse_rgb(value: &str) -> Option<Rgba> {
    let parts: Vec<&str> = value.split(',').collect();
    if parts.len() != 3 {
        return None; // Malformed, skip gracefully
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some(Rgba::rgb(r, g, b))
}
```

### Pattern 4: Dark/Light Detection by Luminance
**What:** KDE has no explicit light/dark flag. Detect from `[Colors:Window] BackgroundNormal` luminance.
**When to use:** Deciding which variant (light/dark) to populate in `NativeTheme`.
**Example:**
```rust
fn is_dark(bg: &Rgba) -> bool {
    // Relative luminance using sRGB coefficients
    let luminance = 0.299 * (bg.r as f32) + 0.587 * (bg.g as f32) + 0.114 * (bg.b as f32);
    luminance < 128.0
}
```

### Anti-Patterns to Avoid
- **Case-insensitive parser for KDE files:** KDE key names are PascalCase (`BackgroundNormal`, `ForegroundActive`). Using `Ini::new()` lowercases everything, causing lookup failures when you try to `get("Colors:View", "BackgroundNormal")`. Use `Ini::new_cs()` or `new_from_defaults()` with `case_sensitive: true`.
- **Default delimiters with configparser:** The default includes `:` as a delimiter. While section names still parse correctly (text between `[` and `]`), lines like `another:value` would be misinterpreted as key-value pairs. Use `delimiters: vec!['=']` to be safe.
- **Assuming both light and dark are available:** kdeglobals contains ONE color scheme. Populate only the detected variant (light or dark), leave the other as `None`.
- **Hardcoding `~/.config/kdeglobals`:** Must respect `XDG_CONFIG_HOME` environment variable.
- **Parsing all 60 KDE color keys:** Only map the ones that have semantic equivalents in `ThemeColors`. Unmapped keys are simply not used.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| INI parsing | Custom line-by-line parser | `configparser` crate | Edge cases: empty values (`key=`), comment handling (`#`, `;`), whitespace trimming, multiline values |
| XDG path resolution | Manual `$HOME/.config` concatenation | `std::env::var("XDG_CONFIG_HOME")` with fallback | Need to handle: env var set but empty, var not set, non-absolute paths |
| Qt font string parsing | Full QFont parser | Extract fields [0] and [1] only | We only need family and size; the other 8-14 fields are rendering hints we don't use |

**Key insight:** The kdeglobals file is a well-defined INI variant. The complexity is in the mapping logic (KDE groups to semantic roles), not in the parsing itself. Let `configparser` handle the syntax; focus implementation effort on the color/font mapping.

## Common Pitfalls

### Pitfall 1: Case Sensitivity with configparser
**What goes wrong:** All section/key lookups fail because `configparser` lowercases everything by default, but code uses PascalCase keys like `BackgroundNormal`.
**Why it happens:** `Ini::new()` creates a case-insensitive parser. KDE key names are case-sensitive PascalCase.
**How to avoid:** Use custom defaults with `case_sensitive: true` and `delimiters: vec!['=']`.
**Warning signs:** All `ini.get()` calls return `None` even though the file has data.

### Pitfall 2: Colon in Delimiters
**What goes wrong:** If `:` is a delimiter (default), lines that happen to contain `:` outside `=` assignments could be misparsed as key-value pairs.
**Why it happens:** configparser's default delimiters are `['=', ':']`, matching Python's configparser behavior.
**How to avoid:** Set `delimiters: vec!['=']` in the `IniDefault` configuration.
**Warning signs:** Unexpected keys appearing in sections, or values being truncated.

### Pitfall 3: Missing Sections/Keys in kdeglobals
**What goes wrong:** Panic or crash when a color group or key doesn't exist in the file.
**Why it happens:** kdeglobals files vary widely -- some have all sections, some only have a few. Example: the test system's kdeglobals has NO `[General]` font entries and no accent color key.
**How to avoid:** Every `ini.get()` returns `Option<String>` -- always use `if let Some(val)` or `?` propagation. Return partial themes, never panic.
**Warning signs:** Tests that only work with fully-populated fixture files.

### Pitfall 4: Qt Font String Field Count
**What goes wrong:** Parser assumes exactly 10 or exactly 16 fields, crashes on unexpected format.
**Why it happens:** Qt4 used 10 fields, Qt5/6 use 16+ fields. Some distributions or older configs may have non-standard counts.
**How to avoid:** Split on `,`, take field [0] as family and field [1] as point size. Ignore remaining fields. Only fail if fewer than 2 fields.
**Warning signs:** Test failures with different Qt version font strings.

### Pitfall 5: Color Value Format Variations
**What goes wrong:** `parse::<u8>()` fails because value has extra whitespace or unexpected characters.
**Why it happens:** KDE INI files sometimes have trailing whitespace or comments after values.
**How to avoid:** Always `.trim()` each component after splitting on `,`. configparser handles trailing comments, but trim the values anyway.
**Warning signs:** Some color values parse fine, others silently fail.

### Pitfall 6: Feature Flag Not Wired Up
**What goes wrong:** `from_kde()` is not accessible from crate root, or `configparser` is always compiled.
**Why it happens:** Forgetting to add `optional = true` on the dependency, or not re-exporting the function.
**How to avoid:** Checklist: (1) `Cargo.toml` has `[features] kde = ["dep:configparser"]`, (2) dependency has `optional = true`, (3) `lib.rs` has `#[cfg(feature = "kde")] pub mod kde;`, (4) `lib.rs` has `#[cfg(feature = "kde")] pub use kde::from_kde;`.
**Warning signs:** `cargo build` compiles configparser even without `--features kde`.

## Code Examples

### Configuring configparser for KDE Files
```rust
// Verified by empirical test against real kdeglobals
fn create_kde_parser() -> configparser::ini::Ini {
    let tmp = configparser::ini::Ini::new_cs();
    let mut defaults = tmp.defaults();
    defaults.delimiters = vec!['='];  // Exclude ':' -- KDE uses colons in section names
    // case_sensitive is already true from new_cs()
    configparser::ini::Ini::new_from_defaults(defaults)
}
```

### Parsing KDE RGB Color Values
```rust
use crate::Rgba;

/// Parse a KDE "R,G,B" color string into an Rgba (opaque).
/// Returns None for malformed values (never panics).
fn parse_rgb(value: &str) -> Option<Rgba> {
    let parts: Vec<&str> = value.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some(Rgba::rgb(r, g, b))
}
```

### Parsing Qt Font Strings
```rust
use crate::ThemeFonts;

/// Parse a Qt QFont::toString() string into family and size.
/// Handles both Qt4 (10 fields) and Qt5/6 (16+ fields) formats.
/// Only extracts family (field 0) and point size (field 1).
fn parse_qt_font(font_str: &str) -> Option<(String, f32)> {
    let fields: Vec<&str> = font_str.split(',').collect();
    if fields.len() < 2 {
        return None;
    }
    let family = fields[0].trim().to_string();
    if family.is_empty() {
        return None;
    }
    let size = fields[1].trim().parse::<f32>().ok()?;
    if size <= 0.0 {
        return None;
    }
    Some((family, size))
}
```

### KDE Color Group to ThemeColors Mapping
```rust
// KDE has 5 standard color groups, each with 12 keys:
// [Colors:View]     -> content/view area colors
// [Colors:Window]   -> window chrome/background
// [Colors:Button]   -> button widget colors
// [Colors:Selection] -> selection/highlight colors
// [Colors:Tooltip]  -> tooltip popup colors
//
// Plus non-standard groups:
// [Colors:Header]        -> header bar (Plasma 5.25+)
// [Colors:Complementary] -> complementary/sidebar colors
// [WM]                   -> window manager title bar colors
//
// Mapping to ThemeColors (36 semantic roles):
//
// core.accent       <- Colors:View/DecorationFocus (or ForegroundActive)
// core.background   <- Colors:Window/BackgroundNormal
// core.foreground   <- Colors:Window/ForegroundNormal
// core.surface      <- Colors:View/BackgroundNormal
// core.border       <- Colors:Window/DecorationFocus (or computed)
// core.muted        <- Colors:Window/ForegroundInactive
// core.shadow       <- None (KDE doesn't expose shadow color in kdeglobals)
//
// primary.background <- Colors:Selection/BackgroundNormal (accent action)
// primary.foreground <- Colors:Selection/ForegroundNormal
// secondary.background <- Colors:Button/BackgroundNormal
// secondary.foreground <- Colors:Button/ForegroundNormal
//
// status.danger       <- Colors:View/ForegroundNegative
// status.danger_fg    <- Colors:Window/ForegroundNormal (text on danger)
// status.warning      <- Colors:View/ForegroundNeutral
// status.warning_fg   <- Colors:Window/ForegroundNormal
// status.success      <- Colors:View/ForegroundPositive
// status.success_fg   <- Colors:Window/ForegroundNormal
// status.info         <- Colors:View/ForegroundActive
// status.info_fg      <- Colors:Window/ForegroundNormal
//
// interactive.selection <- Colors:Selection/BackgroundNormal
// interactive.selection_foreground <- Colors:Selection/ForegroundNormal
// interactive.link     <- Colors:View/ForegroundLink
// interactive.focus_ring <- Colors:View/DecorationFocus
//
// panel.sidebar       <- Colors:Complementary/BackgroundNormal (if available)
// panel.sidebar_fg    <- Colors:Complementary/ForegroundNormal
// panel.tooltip       <- Colors:Tooltip/BackgroundNormal
// panel.tooltip_fg    <- Colors:Tooltip/ForegroundNormal
// panel.popover       <- Colors:View/BackgroundNormal (reuse surface)
// panel.popover_fg    <- Colors:View/ForegroundNormal
//
// component.button    <- Colors:Button/BackgroundNormal
// component.button_fg <- Colors:Button/ForegroundNormal
// component.input     <- Colors:View/BackgroundNormal
// component.input_fg  <- Colors:View/ForegroundNormal
// component.disabled  <- Colors:View/ForegroundInactive
// component.separator <- Colors:Window/ForegroundInactive (reuse muted)
// component.alternate_row <- Colors:View/BackgroundAlternate
```

### kdeglobals Path Resolution
```rust
fn kdeglobals_path() -> std::path::PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        if !config_home.is_empty() {
            return std::path::PathBuf::from(config_home).join("kdeglobals");
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home).join(".config").join("kdeglobals");
    }
    // Last resort fallback
    std::path::PathBuf::from("/etc/xdg/kdeglobals")
}
```

### Dark/Light Variant Detection
```rust
fn is_dark_theme(ini: &configparser::ini::Ini) -> bool {
    // Use Colors:Window/BackgroundNormal for luminance check
    if let Some(bg_str) = ini.get("Colors:Window", "BackgroundNormal") {
        if let Some(bg) = parse_rgb(&bg_str) {
            // ITU-R BT.601 luma coefficients
            let luma = 0.299 * (bg.r as f32) + 0.587 * (bg.g as f32) + 0.114 * (bg.b as f32);
            return luma < 128.0;
        }
    }
    // Default to dark if we can't determine
    false
}
```

## KDE kdeglobals File Format Reference

### File Location
- Primary: `$XDG_CONFIG_HOME/kdeglobals` (defaults to `~/.config/kdeglobals`)
- System fallback: `/etc/xdg/kdeglobals`

### Format
Standard INI with KDE-specific conventions:
- Section names contain colons: `[Colors:View]`, `[Colors:Window]`
- Color values are comma-separated RGB: `BackgroundNormal=20,22,24`
- Font values are Qt `QFont::toString()` format: `font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1`
- Some sections have "sub-sections": `[Colors:Header][Inactive]` (treated as flat section name)
- Empty values are valid: `ChangeSelectionColor=` (parsed as `Some("")`)

### Standard Color Groups (12 keys each)
Each group contains:
| Key | Description |
|-----|-------------|
| BackgroundNormal | Primary background |
| BackgroundAlternate | Alternating row background |
| ForegroundNormal | Primary text color |
| ForegroundInactive | Dimmed/disabled text |
| ForegroundActive | Active/accent text |
| ForegroundLink | Hyperlink text |
| ForegroundVisited | Visited link text |
| ForegroundNegative | Error/danger indicator |
| ForegroundNeutral | Warning indicator |
| ForegroundPositive | Success indicator |
| DecorationFocus | Focus ring/highlight decoration |
| DecorationHover | Hover state decoration |

### Font Keys in [General]
| Key | Maps To |
|-----|---------|
| `font` | ThemeFonts.family + ThemeFonts.size |
| `fixed` | ThemeFonts.mono_family + ThemeFonts.mono_size |
| `menuFont` | (not mapped -- no equivalent in ThemeFonts) |
| `toolBarFont` | (not mapped) |
| `smallestReadableFont` | (not mapped) |
| `desktopFont` | (not mapped) |

### Qt Font String Format
```
family,pointSize,pixelSize,styleHint,weight,style,underline,strikeOut,fixedPitch,reserved[,capitalization,letterSpacingType,letterSpacing,wordSpacing,stretch,styleStrategy]
```
- Qt4: 10 fields (ends at reserved/rawMode)
- Qt5/6: 16+ fields (adds capitalization through styleStrategy, may include styleName, features, variable axes)
- Fields 0 (family) and 1 (pointSize) are sufficient for ThemeFonts

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Qt4 10-field font strings | Qt5/6 16+ field font strings | Qt 5.0 (2012) | Parser must handle both lengths |
| No accent color support | Custom accent color in Plasma | Plasma 5.25 (2022) | `DecorationFocus` / `ForegroundActive` carry accent color |
| No Header color group | `[Colors:Header]` added | KF5/Plasma 5.x | Sidebar/header colors available but optional |
| `[Colors:Complementary]` optional | More widely populated | Plasma 5+ | Use for sidebar colors when available |

**Deprecated/outdated:**
- Qt4 10-field font format: Still found on old systems. Parser handles gracefully by only reading fields [0] and [1].
- `[ColorEffects:Disabled]` / `[ColorEffects:Inactive]`: Effect parameters -- not color values. Skip entirely.

## Open Questions

1. **Should from_kde() also read `/etc/xdg/kdeglobals` as a fallback?**
   - What we know: `$XDG_CONFIG_HOME/kdeglobals` is the per-user file. `/etc/xdg/kdeglobals` is the system default.
   - What's unclear: Whether we should merge system defaults with user overrides (KDE itself does this).
   - Recommendation: For v1, read only the user file. If missing, return `Error::Unavailable`. System-level fallback can be added later. Keep it simple.

2. **Should the theme name be extracted from kdeglobals?**
   - What we know: `[General] ColorScheme=BreezeDark` gives the scheme name. `[KDE] LookAndFeelPackage=org.kde.breezedark.desktop` gives the global theme.
   - What's unclear: Which one to use, or whether to hardcode "KDE" as the name.
   - Recommendation: Use `[General] ColorScheme` if available, fall back to "KDE" as the theme name.

3. **Should we populate both light AND dark variants?**
   - What we know: kdeglobals has ONE color scheme. KDE Breeze preset has both variants because we hand-authored them.
   - What's unclear: Whether users expect both variants from a live reader.
   - Recommendation: Populate only the detected variant (light or dark). Leave the other `None`. Users who want both can merge with the `kde-breeze` preset for the missing variant.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | none (built-in) |
| Quick run command | `cargo test --features kde` |
| Full suite command | `cargo test --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-01a | from_kde() parses color groups into semantic roles | unit | `cargo test --features kde kde::tests::parse_colors -- -x` | Wave 0 |
| PLAT-01b | from_kde() handles missing file gracefully | unit | `cargo test --features kde kde::tests::missing_file -- -x` | Wave 0 |
| PLAT-01c | from_kde() handles missing sections gracefully | unit | `cargo test --features kde kde::tests::missing_sections -- -x` | Wave 0 |
| PLAT-01d | from_kde() handles malformed color values | unit | `cargo test --features kde kde::tests::malformed_values -- -x` | Wave 0 |
| PLAT-01e | Qt4 (10-field) font string parses correctly | unit | `cargo test --features kde kde::tests::qt4_font -- -x` | Wave 0 |
| PLAT-01f | Qt5/6 (16-field) font string parses correctly | unit | `cargo test --features kde kde::tests::qt6_font -- -x` | Wave 0 |
| PLAT-01g | Feature flag compiles correctly (on/off) | integration | `cargo check && cargo check --features kde` | Wave 0 |
| PLAT-01h | Dark/light detection from background luminance | unit | `cargo test --features kde kde::tests::dark_light_detection -- -x` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test --features kde`
- **Per wave merge:** `cargo test --all-features`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `src/kde/mod.rs` -- main module with `from_kde()` function
- [ ] `src/kde/colors.rs` -- color parsing and mapping logic
- [ ] `src/kde/fonts.rs` -- Qt font string parser
- [ ] Test fixtures: embedded kdeglobals strings (dark + light + minimal + malformed variants)
- [ ] Cargo.toml feature flag setup: `kde = ["dep:configparser"]`

## Sources

### Primary (HIGH confidence)
- Empirical test: `/tmp/kde_test/` -- configparser 3.1.0 tested with real kdeglobals format, section names with colons, case sensitivity, delimiter behavior
- Real kdeglobals: `~/.config/kdeglobals` on test system (Arch Linux, Plasma, BreezeDark)
- System color scheme: `/usr/share/color-schemes/BreezeDark.colors` -- complete color group reference
- [configparser 3.1.0 API docs](https://docs.rs/configparser/3.1.0/configparser/ini/struct.Ini.html) -- Ini, IniDefault, sections, get methods
- [configparser IniDefault](https://docs.rs/configparser/3.1.0/configparser/ini/struct.IniDefault.html) -- delimiters, case_sensitive, default values

### Secondary (MEDIUM confidence)
- [Qt6 QFont::toString() docs](https://doc.qt.io/qt-6/qfont.html) -- field list for font string format (17+ fields)
- [Qt5 QFont docs](https://doc.qt.io/qt-5/qfont.html) -- backward compatibility
- [Qt Forum: QFont::fromString() Qt4 vs Qt5](https://forum.qt.io/topic/94958/qfont-fromstring-change-between-qt-4-and-qt-5) -- 10 vs 16 field difference confirmed
- [KDE TechBase: Oxygen/ColorSchemes](https://techbase.kde.org/Projects/Oxygen/ColorSchemes) -- 5 color groups, 12 keys each, 60 total
- [KColorScheme API (KDE 4.12)](https://api.kde.org/legacy/4.12-api/kdelibs-apidocs/kdeui/html/classKColorScheme.html) -- BackgroundRole, ForegroundRole, DecorationRole enums
- [Cargo features documentation](https://doc.rust-lang.org/cargo/reference/features.html) -- optional dependencies, dep: prefix, cfg(feature)
- [KDE accent color blog post](https://pointieststick.com/2022/04/22/this-week-in-kde-major-accent-color-and-global-theme-improvements/) -- accent color in DecorationFocus

### Tertiary (LOW confidence)
- [Arch Forums: KDE font in kdeglobals](https://bbs.archlinux.org/viewtopic.php?id=75570) -- font key examples (font=, fixed=, menuFont=)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - configparser 3.1.0 empirically tested with real KDE files, API verified, edge cases explored
- Architecture: HIGH - Existing codebase patterns (feature modules, error types) well understood; color mapping based on actual KDE source analysis
- Pitfalls: HIGH - Case sensitivity and delimiter issues discovered and verified through actual testing; font format confirmed through Qt documentation
- Color mapping: MEDIUM - Mapping from KDE's 60 roles to 36 semantic roles involves judgment calls; the proposed mapping is reasonable but some choices (e.g., what maps to `core.border`) are debatable

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (stable domain -- KDE config format changes slowly)
