# Phase 63: KDE Reader Fixture Tests - Research

**Researched:** 2026-04-09
**Domain:** Rust testing patterns, KDE INI parsing, pure function extraction
**Confidence:** HIGH

## Summary

Phase 63 requires separating the KDE reader's pure parsing logic from its I/O dependencies, then testing the pure core with fixture INI files. The existing codebase is already 90% of the way there -- `from_kde_content(content: &str)` already accepts raw INI text, and the color/font/metrics submodules already have 113 inline tests using constant fixtures. However, two I/O calls leak into the "pure" path: `detect_font_dpi()` calls `read_kcmfontsrc_key()`, `xft_dpi()`, and `physical_dpi()`, and `parse_icon_sizes_from_index_theme()` reads filesystem icon theme files. These must be separated to achieve full testability.

The existing inline test constants (BREEZE_DARK, BREEZE_DARK_FULL, BREEZE_LIGHT_FULL, MINIMAL_FIXTURE) already cover several fixture scenarios but live as `const &str` inside test modules rather than as standalone `.ini` fixture files. The phase should extract these to files, add missing edge cases (custom accent, high DPI, malformed values), and ensure the pure parse function has zero I/O dependencies.

**Primary recommendation:** Split `detect_font_dpi()` into a pure `parse_force_font_dpi(ini) -> Option<f32>` and an I/O `detect_font_dpi_fallback() -> f32`, parameterize `from_kde_content` to accept an optional DPI override, and move fixture data to `tests/fixtures/kde/` files loaded via `include_str!`.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| TEST-01 | KDE reader separated into `parse_kdeglobals(content: &str)` pure function testable with fixture data | `from_kde_content` already exists as pub(crate) with signature matching `parse_kdeglobals`. Two I/O leaks must be removed: `detect_font_dpi` (calls xrdb/xrandr/kcmfontsrc) and `parse_icon_sizes_from_index_theme` (reads filesystem). See Architecture Patterns section. |
| TEST-02 | KDE fixture tests cover: Breeze light/dark, custom accent, minimal config, missing groups, malformed values, high DPI | Existing inline constants cover Breeze dark, Breeze light, minimal, empty, partial, malformed. Missing: custom accent fixture, high DPI fixture, dedicated missing-groups fixture. Real KDE color scheme files available at `/usr/share/color-schemes/` for reference. See Fixture Inventory section. |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

No CLAUDE.md exists in the project root. Constraints are derived from project memory:

- **No runtime panics, no unsafe** -- NEVER write panic-prone or unsafe Rust. Tests may use `#[allow(clippy::unwrap_used)]` [VERIFIED: existing test modules use this pattern]
- **No hardcoded theme values** -- Always read from resolved theme [VERIFIED: codebase pattern]
- **Run pre-release-check.sh after code changes** [VERIFIED: script exists at project root]
- **Fix root causes, not symptoms** [from project memory]
- **NEVER LIE, NEVER INVENT** -- Never fabricate data [from project memory]
- **Agents must append, never rewrite** existing docs [from project memory]

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| configparser | 3.1.0 | KDE INI parsing with case-sensitive keys and `=`-only delimiters | Already used; `create_kde_parser()` configures it correctly for KDE's PascalCase keys and colon section names |
| std::include_str! | stable | Load fixture .ini files at compile time | Zero-cost, no runtime I/O, standard Rust pattern for test fixtures |

[VERIFIED: Cargo.lock shows configparser 3.1.0]

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| proptest | 1.11 | Property-based testing | Already a dev-dependency; not needed for this phase (fixture tests are sufficient) |

[VERIFIED: Cargo.toml dev-dependencies]

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `include_str!` for fixtures | Runtime `std::fs::read_to_string` in tests | `include_str!` is simpler, catches missing files at compile time, works in CI without file path issues |
| Inline `const &str` fixtures | `.ini` fixture files | Files are more readable for large fixtures, but inline constants are already used for 113 tests. Phase should use files for new fixtures to match the design doc's file layout. |

## Architecture Patterns

### Current Architecture (I/O leaks identified)

```
from_kde()               -- reads kdeglobals from filesystem
  -> from_kde_content()   -- "pure" but leaks I/O via:
       -> populate_accessibility()
            -> detect_font_dpi()
                 -> read_kcmfontsrc_key()    [I/O: reads ~/.config/kcmfontsrc]
                 -> crate::detect::xft_dpi() [I/O: runs xrdb -query]
                 -> crate::detect::physical_dpi() [I/O: runs xrandr]
       -> parse_icon_sizes_from_index_theme()  [I/O: reads index.theme from disk]
```

[VERIFIED: source code inspection of kde/mod.rs lines 18-103]

### Target Architecture (pure parsing separated)

```
from_kde()                    -- reads kdeglobals, detects DPI, reads index.theme
  -> from_kde_content_pure()  -- ZERO I/O; takes &str content + Option<f32> DPI
       -> populate_colors()   -- already pure (takes &Ini)
       -> populate_fonts()    -- already pure (takes &Ini)
       -> populate_widget_sizing() -- already pure (takes &mut ThemeVariant)
       -> populate_accessibility_pure() -- pure: parses AnimationDurationFactor
       -> parse_force_font_dpi() -- pure: extracts forceFontDPI from INI
```

### Pattern 1: DPI Separation

**What:** Split `detect_font_dpi(ini)` into two functions:
1. `parse_force_font_dpi(ini: &Ini) -> Option<f32>` -- pure, extracts `[General] forceFontDPI` from the parsed INI
2. `detect_font_dpi(ini: &Ini) -> f32` -- the existing function, unchanged (calls I/O fallbacks)

**When to use:** In the pure parse path, call only `parse_force_font_dpi`. In the full `from_kde()` path, call `detect_font_dpi` which chains through I/O fallbacks.

**Implementation approach:**

The pure function receives an optional `font_dpi` parameter so tests can control it:

```rust
// Pure parsing -- no I/O
pub(crate) fn from_kde_content_pure(content: &str, font_dpi: Option<f32>) -> crate::Result<crate::ThemeSpec> {
    let mut ini = create_kde_parser();
    ini.read(content.to_string()).map_err(crate::Error::Format)?;

    let mut variant = crate::ThemeVariant::default();
    colors::populate_colors(&ini, &mut variant);
    fonts::populate_fonts(&ini, &mut variant);
    metrics::populate_widget_sizing(&mut variant);

    // Pure accessibility: AnimationDurationFactor parsing
    populate_accessibility_pure(&ini, &mut variant, font_dpi);

    // Icon theme name (without filesystem icon size lookup)
    if let Some(theme_name) = ini.get("Icons", "Theme") {
        if !theme_name.is_empty() {
            variant.icon_theme = Some(theme_name);
        }
    }
    // Icon sizes set by caller (from_kde) after filesystem lookup

    variant.dialog.button_order = Some(DialogButtonOrder::PrimaryLeft);
    // ... rest of theme construction
}

// I/O wrapper (existing behavior preserved)
pub(crate) fn from_kde_content(content: &str) -> crate::Result<crate::ThemeSpec> {
    let mut ini = create_kde_parser();
    ini.read(content.to_string()).map_err(crate::Error::Format)?;
    let font_dpi = detect_font_dpi(&ini);
    let mut theme = from_kde_content_pure(content, Some(font_dpi))?;

    // Icon sizes from filesystem (I/O)
    if let Some(variant) = theme.dark.as_mut().or(theme.light.as_mut()) {
        if let Some(ref theme_name) = variant.icon_theme {
            variant.defaults.icon_sizes = parse_icon_sizes_from_index_theme(theme_name);
        }
    }
    Ok(theme)
}
```

[ASSUMED -- exact API shape is Claude's discretion; the principle is verified from codebase analysis]

### Pattern 2: Fixture File Organization

**What:** Place fixture `.ini` files under `native-theme/tests/fixtures/kde/` and load them with `include_str!`.

```
native-theme/
  tests/
    fixtures/
      kde/
        breeze-dark.ini         -- Full Breeze Dark (from real KDE 6 system)
        breeze-light.ini        -- Full Breeze Light (from real color scheme)
        custom-accent.ini       -- Modified accent color (orange instead of blue)
        minimal.ini             -- Only [Colors:Window] BackgroundNormal
        missing-groups.ini      -- No [Colors:View], no [WM], etc.
        malformed-values.ini    -- Invalid RGB ("abc,def,ghi"), 2-component ("255,255")
        high-dpi.ini            -- forceFontDPI=192 in [General]
    reader_kde.rs               -- Integration tests using fixtures
```

[VERIFIED: matches layout from docs/todo_v0.5.6_platform-reader-testing.md]

### Pattern 3: Test Location Strategy

Two options for where tests live:

**Option A: Inline tests (current pattern)**
Tests live in `#[cfg(test)] mod tests` inside `kde/mod.rs`, `kde/colors.rs`, etc. This is already where 113 tests live. Fixture files are loaded with `include_str!("../../tests/fixtures/kde/breeze-dark.ini")` (relative to source file).

**Option B: Integration test file**
Tests live in `tests/reader_kde.rs`. Uses `include_str!("fixtures/kde/breeze-dark.ini")` (relative to tests/ directory). This is the pattern specified in the design doc.

**Recommendation:** Use integration tests in `tests/reader_kde.rs` for the new fixture-based tests. This keeps fixture tests separate from the existing inline unit tests, matches the design doc, and allows testing the public API surface (`kde::from_kde_content` or equivalent). The existing 113 inline tests should remain untouched.

**Key constraint:** The `from_kde_content` function is currently `pub(crate)`. For integration tests to call it, either:
- Make it `pub` (since `kde` module is already `pub`) -- this is the cleanest approach
- Or expose a test-only re-export

Since `kde` module is already `pub mod kde` (line 100 of lib.rs), making `from_kde_content` pub is safe and matches the doc's design intent. [VERIFIED: lib.rs line 100]

### Anti-Patterns to Avoid
- **I/O in "pure" test path:** Never call `detect_font_dpi` (with its I/O fallbacks) from the pure parsing function used in fixture tests. The whole point is to eliminate I/O.
- **Hardcoded expected values that differ from real KDE:** Fixture test expected values must match what real KDE Plasma 6 produces, not invented values.
- **Modifying existing test constants:** The existing `BREEZE_DARK_FULL` etc. inline constants in `kde/mod.rs` tests are battle-tested. Do not modify them; add fixture files alongside.
- **Fixture files with fabricated data:** Use real KDE color scheme data (from `/usr/share/color-schemes/` or actual `kdeglobals`), except for edge-case fixtures (malformed, minimal) which are hand-crafted by definition.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| INI parsing | Custom KDE INI parser | `configparser` 3.1.0 with `create_kde_parser()` | Already handles case-sensitivity, colon sections, equals-only delimiter |
| Qt font string parsing | New font parser | `fonts::parse_qt_font_with_weight()` | Already handles Qt5/Qt6 format detection, weight conversion, edge cases |
| Color RGB parsing | New color parser | `parse_rgb()` in `kde/mod.rs` | Already handles whitespace, validates 3 components, returns `Option<Rgba>` |

**Key insight:** The parsing infrastructure already exists and is well-tested. This phase's work is structural (separating I/O from pure logic) and additive (new fixture files + new test assertions), NOT reimplementation.

## Fixture Inventory

### Already Tested (as inline constants)

| Scenario | Current Location | Coverage |
|----------|-----------------|----------|
| Breeze Dark (full) | `kde/mod.rs::BREEZE_DARK_FULL` | Colors, fonts, accessibility, icon theme, metrics, dark detection |
| Breeze Dark (colors only) | `kde/colors.rs::BREEZE_DARK` | All color group mappings |
| Breeze Light (partial) | `kde/mod.rs::BREEZE_LIGHT_FULL` | Light detection, basic colors |
| Minimal (Window only) | `kde/mod.rs::MINIMAL_FIXTURE` | Dark detection with minimal config |
| Empty content | `kde/mod.rs::test_empty_content` | Empty string produces Ok |
| Partial sections | `kde/colors.rs::test_partial_sections` | Window-only, missing View/Button |
| Malformed values | `kde/colors.rs::test_malformed_values` | Non-numeric RGB values |
| Missing Complementary | `kde/colors.rs::test_sidebar_none_without_complementary` | Missing optional group |
| Empty INI | `kde/colors.rs::test_empty_ini` | No sections at all |

[VERIFIED: source code inspection of all test modules]

### Needs Fixture Files (per TEST-02 requirements)

| Fixture | Status | Source | Notes |
|---------|--------|--------|-------|
| `breeze-dark.ini` | Needs file | Real system data at `~/.config/kdeglobals` or `/usr/share/color-schemes/BreezeDark.colors` | Full fixture with all color groups, fonts, WM, KDE, Icons sections |
| `breeze-light.ini` | Needs file | `/usr/share/color-schemes/BreezeLight.colors` (available on system) | Full Breeze Light with light background colors |
| `custom-accent.ini` | Needs file | Hand-crafted based on Breeze Dark with modified `DecorationFocus`/`BackgroundNormal` in Selection | Orange accent (e.g., `246,116,0`) instead of default Breeze blue |
| `minimal.ini` | Needs file | Hand-crafted | Only `[Colors:Window]` with `BackgroundNormal` and `ForegroundNormal` |
| `missing-groups.ini` | Needs file | Hand-crafted | Has `[Colors:Window]` and `[Colors:View]` but no `[WM]`, `[Colors:Tooltip]`, `[Colors:Complementary]`, `[Colors:Header]`, `[Colors:Selection]` |
| `malformed-values.ini` | Needs file | Hand-crafted | Mix of valid and invalid: `BackgroundNormal=abc,def,ghi`, `ForegroundNormal=252,252` (2 components), normal `DecorationFocus` |
| `high-dpi.ini` | Needs file | Based on Breeze Dark with `[General] forceFontDPI=192` | Tests pure DPI extraction from INI without I/O fallbacks |

### Real KDE Data Available on System

The research machine has KDE Plasma 6 installed with the following reference data:

- `~/.config/kdeglobals` -- 170-line real active configuration (Breeze Dark variant with full color groups + WM section) [VERIFIED: file exists and was read]
- `/usr/share/color-schemes/BreezeDark.colors` -- 209-line official Breeze Dark color scheme [VERIFIED: file exists]
- `/usr/share/color-schemes/BreezeLight.colors` -- 202-line official Breeze Light color scheme [VERIFIED: file exists]

**Important difference:** The system `.colors` files include sections not present in kdeglobals: `[ColorEffects:Disabled]`, `[ColorEffects:Inactive]`, localized `Name[xx]` keys, and `shadeSortColumn`. The KDE reader's `configparser` with `create_kde_parser()` handles these correctly (unknown sections are ignored). However, the real `.colors` files do NOT include `[General]` font entries, `[Icons]`, or `[KDE]` sections -- those come from kdeglobals separately. Fixture files should combine both sources for realistic testing.

## Common Pitfalls

### Pitfall 1: DPI Fallback Chain Leaking Into Pure Tests
**What goes wrong:** Tests call `from_kde_content()` which internally calls `detect_font_dpi()` -> `xft_dpi()` -> `physical_dpi()`. On CI, xrdb and xrandr may not exist, causing different DPI values than expected.
**Why it happens:** The current `from_kde_content` is not fully pure -- it calls I/O for DPI detection.
**How to avoid:** The pure function must accept DPI as a parameter or only use the INI's `forceFontDPI`. Tests that care about DPI set `forceFontDPI` in the fixture. Tests that don't care assert `font_dpi.is_some()` without checking the exact value.
**Warning signs:** Tests that pass locally but fail in CI due to different DPI environments.

### Pitfall 2: Icon Sizes Depending on Installed Themes
**What goes wrong:** `parse_icon_sizes_from_index_theme("breeze-dark")` reads from the filesystem. On CI, breeze-dark may not be installed, returning all-None icon sizes.
**Why it happens:** Icon size parsing calls `find_index_theme_path()` which searches XDG icon directories.
**How to avoid:** The pure parse function should NOT call `parse_icon_sizes_from_index_theme()`. It should only extract the icon theme name. Icon size lookup is I/O and belongs in `from_kde()`.
**Warning signs:** Test `test_icon_sizes_populated_in_full_parse` already handles this with a conditional check.

### Pitfall 3: configparser Case Sensitivity Gotcha
**What goes wrong:** Using default `Ini::new()` instead of `create_kde_parser()` lowercases all keys, breaking lookups like `BackgroundNormal`.
**Why it happens:** configparser defaults to case-insensitive mode.
**How to avoid:** Always use `create_kde_parser()` which creates a case-sensitive INI parser with `=`-only delimiter.
**Warning signs:** All color/font lookups return None despite data being present in fixture.

[VERIFIED: `create_kde_parser()` uses `Ini::new_cs()` at kde/mod.rs line 321]

### Pitfall 4: Breeze Light vs Dark fixture - wrong section in `[Colors:Complementary]`
**What goes wrong:** Breeze Light's Complementary section has DARK colors (bg=42,46,50) because it represents the sidebar/panel which uses inverted colors on light themes.
**Why it happens:** KDE's Complementary group is meant for sidebar areas that use opposite contrast.
**How to avoid:** Don't assume Complementary background is light just because the theme is light. Verify expected values against real system color scheme files.
**Warning signs:** Test expecting light sidebar background on Breeze Light fixture.

[VERIFIED: BreezeLight.colors has Complementary BackgroundNormal=42,46,50 -- dark values on a light theme]

### Pitfall 5: Fixture Files Must Not Include SPDX Headers
**What goes wrong:** Copying `/usr/share/color-schemes/BreezeLight.colors` verbatim includes SPDX copyright headers as comments. The `configparser` handles `#` comments fine, but the fixtures should be clean test data, not copies of licensed files.
**Why it happens:** Real `.colors` files have LGPL headers.
**How to avoid:** Create fixture files with the relevant INI sections only. Use the real files as REFERENCE for correct color values, but craft fixtures containing only the sections the reader actually parses.
**Warning signs:** Large fixture files with localization entries and effect sections that the reader ignores.

## Code Examples

### Loading a fixture file in an integration test

```rust
// tests/reader_kde.rs
use native_theme::kde;

#[test]
fn breeze_dark_fixture_parses_accent_color() {
    let content = include_str!("fixtures/kde/breeze-dark.ini");
    let spec = kde::from_kde_content_pure(content, Some(96.0)).unwrap();
    let variant = spec.dark.as_ref().expect("should be dark theme");
    assert_eq!(
        variant.defaults.accent_color,
        Some(native_theme::Rgba::rgb(61, 174, 233)),
    );
}
```

[VERIFIED: pattern matches existing test style in kde/mod.rs]

### Pure DPI extraction test

```rust
#[test]
fn high_dpi_fixture_extracts_force_font_dpi() {
    let content = include_str!("fixtures/kde/high-dpi.ini");
    let spec = kde::from_kde_content_pure(content, None).unwrap();
    let variant = spec.dark.as_ref().unwrap();
    // forceFontDPI=192 in fixture -> font_dpi=192.0
    assert_eq!(variant.defaults.font_dpi, Some(192.0));
}
```

### Custom accent fixture expected values

```rust
#[test]
fn custom_accent_fixture() {
    let content = include_str!("fixtures/kde/custom-accent.ini");
    let spec = kde::from_kde_content_pure(content, Some(96.0)).unwrap();
    let variant = spec.dark.as_ref().unwrap();
    // Custom orange accent instead of default Breeze blue
    assert_eq!(
        variant.defaults.accent_color,
        Some(native_theme::Rgba::rgb(246, 116, 0)),
    );
    // Selection background should also use the custom accent
    assert_eq!(
        variant.defaults.selection_background,
        Some(native_theme::Rgba::rgb(246, 116, 0)),
    );
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No KDE test coverage | 113 inline unit tests with const fixtures | v0.5.5 (Phase 49-60) | Colors, fonts, metrics all tested but with I/O leaks in accessibility/icon paths |
| `from_kde_content` as sole entry point | Separate pure + I/O functions | This phase (v0.5.6) | Enables fixture-based integration tests without running KDE desktop |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `from_kde_content_pure` should take `Option<f32>` for DPI rather than a broader options struct | Architecture Patterns | Low -- if more I/O-dependent params emerge later, the signature can be extended |
| A2 | Integration tests should live in `tests/reader_kde.rs` rather than adding to existing inline tests | Architecture Patterns | Low -- either location works; design doc specifies integration test file |
| A3 | Fixture files should combine kdeglobals content (General, KDE, Icons) with color scheme sections, rather than being pure `.colors` files | Fixture Inventory | Medium -- if fixtures only contain color sections, font/DPI tests won't work |
| A4 | Making `from_kde_content_pure` (or equivalent) `pub` is acceptable since `kde` module is already `pub` | Architecture Patterns | Low -- doesn't expose anything that isn't conceptually public already |

## Open Questions

1. **Naming: `from_kde_content` vs `parse_kdeglobals`**
   - What we know: The design doc calls it `parse_kdeglobals`. The existing function is named `from_kde_content`. Both mean the same thing.
   - What's unclear: Whether to rename the existing function or add a new one alongside it.
   - Recommendation: Keep `from_kde_content` as the I/O wrapper (unchanged behavior), add `from_kde_content_pure` as the testable pure function. This avoids breaking existing callers. Alternatively, rename to `parse_kdeglobals` if preferred for clarity -- either works since it's `pub(crate)`.

2. **Icon sizes in pure path**
   - What we know: `parse_icon_sizes_from_index_theme()` does filesystem I/O. The pure function should not call it.
   - What's unclear: Should the pure function skip icon sizes entirely, or should there be a `parse_icon_sizes_from_content` that accepts pre-read content?
   - Recommendation: The pure function extracts the icon theme NAME but leaves `icon_sizes` as default. `from_kde()` handles the filesystem lookup afterward. This matches the existing `parse_icon_sizes_from_content()` function which already exists and is pure -- it just needs to be wired through the pure path if content is provided.

3. **`forceFontDPI` parsing in pure path**
   - What we know: The INI's `[General] forceFontDPI` is pure to extract. The kcmfontsrc fallback is I/O.
   - What's unclear: Should the pure function set `font_dpi` from forceFontDPI, or should it always use the caller-provided DPI?
   - Recommendation: The pure function should parse `forceFontDPI` from the INI content. If present and valid, use it. If not present, use the caller-provided fallback DPI (or None). This way, the high-dpi.ini fixture test works by including `forceFontDPI=192` in the INI.

## Environment Availability

Step 2.6: SKIPPED (no external dependencies identified -- this phase is purely code/config changes and test additions within the existing Rust project).

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in Rust test harness) |
| Config file | None needed (Cargo.toml `[dev-dependencies]` section) |
| Quick run command | `cargo test -p native-theme --features kde --lib -- kde::` |
| Full suite command | `cargo test -p native-theme --features kde` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| TEST-01 | Pure `parse_kdeglobals`/`from_kde_content_pure` function exists and returns ThemeVariant without I/O | unit + integration | `cargo test -p native-theme --features kde -- reader_kde` | Wave 0 |
| TEST-02a | Breeze light fixture asserts specific field values | integration | `cargo test -p native-theme --features kde -- reader_kde::breeze_light` | Wave 0 |
| TEST-02b | Breeze dark fixture asserts specific field values | integration | `cargo test -p native-theme --features kde -- reader_kde::breeze_dark` | Wave 0 |
| TEST-02c | Custom accent fixture | integration | `cargo test -p native-theme --features kde -- reader_kde::custom_accent` | Wave 0 |
| TEST-02d | Minimal config fixture | integration | `cargo test -p native-theme --features kde -- reader_kde::minimal` | Wave 0 |
| TEST-02e | Missing groups fixture | integration | `cargo test -p native-theme --features kde -- reader_kde::missing_groups` | Wave 0 |
| TEST-02f | Malformed values fixture | integration | `cargo test -p native-theme --features kde -- reader_kde::malformed` | Wave 0 |
| TEST-02g | High DPI fixture (forceFontDPI in INI) | integration | `cargo test -p native-theme --features kde -- reader_kde::high_dpi` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --features kde -- kde:: reader_kde`
- **Per wave merge:** `cargo test -p native-theme --features kde`
- **Phase gate:** Full suite green + `./pre-release-check.sh` before verification

### Wave 0 Gaps
- [ ] `tests/fixtures/kde/breeze-dark.ini` -- full fixture from real KDE data
- [ ] `tests/fixtures/kde/breeze-light.ini` -- full fixture from real KDE data
- [ ] `tests/fixtures/kde/custom-accent.ini` -- modified accent color fixture
- [ ] `tests/fixtures/kde/minimal.ini` -- only required groups
- [ ] `tests/fixtures/kde/missing-groups.ini` -- graceful degradation test
- [ ] `tests/fixtures/kde/malformed-values.ini` -- error handling test
- [ ] `tests/fixtures/kde/high-dpi.ini` -- DPI extraction test
- [ ] `tests/reader_kde.rs` -- integration test file

## Security Domain

Not applicable for this phase. The phase adds tests to an existing parser and does not introduce new I/O, network access, or security-sensitive operations.

## Sources

### Primary (HIGH confidence)
- Codebase inspection of `native-theme/src/kde/mod.rs` (1169 lines) -- full KDE reader implementation with `from_kde_content`, `detect_font_dpi`, `populate_accessibility`, `parse_icon_sizes_from_index_theme` [VERIFIED]
- Codebase inspection of `native-theme/src/kde/colors.rs` (600 lines) -- color mapping with 32 inline tests [VERIFIED]
- Codebase inspection of `native-theme/src/kde/fonts.rs` (339 lines) -- font parsing with 19 inline tests [VERIFIED]
- Codebase inspection of `native-theme/src/kde/metrics.rs` (158 lines) -- widget sizing with 6 inline tests [VERIFIED]
- `docs/todo_v0.5.6_platform-reader-testing.md` -- design document specifying fixture approach, file layout, per-reader design [VERIFIED]
- `native-theme/Cargo.toml` -- feature flags (`kde = ["dep:configparser"]`) and dev-dependencies [VERIFIED]
- System KDE files: `~/.config/kdeglobals`, `/usr/share/color-schemes/BreezeDark.colors`, `/usr/share/color-schemes/BreezeLight.colors` [VERIFIED: files read]

### Secondary (MEDIUM confidence)
- `docs/todo_v0.5.6_platform-reader-testing.md` design decisions (fixture-based approach, file layout) [VERIFIED: consistent with codebase]

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, all existing tools verified in Cargo.lock
- Architecture: HIGH -- codebase analyzed in detail, I/O leak points precisely identified, separation strategy is straightforward
- Pitfalls: HIGH -- all pitfalls verified against actual source code and real system files
- Fixture data: HIGH -- real KDE color scheme files available on the system for reference

**Research date:** 2026-04-09
**Valid until:** 2026-05-09 (30 days -- stable domain, no moving parts)
