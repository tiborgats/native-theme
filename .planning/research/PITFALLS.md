# Pitfalls Research

**Domain:** Cross-platform Rust crate for OS theme data (colors, fonts, geometry, spacing)
**Researched:** 2026-03-07
**Confidence:** HIGH (most pitfalls verified through prior art analysis, official docs, and codebase examination)

## Critical Pitfalls

### Pitfall 1: configparser Case Sensitivity Silently Drops All KDE Color Data

**What goes wrong:**
The `configparser` crate's default constructor `Ini::new()` lowercases all section and key names. KDE's `kdeglobals` uses CamelCase everywhere: section names like `[Colors:Window]` and keys like `BackgroundNormal`, `ForegroundNormal`. Using the default constructor means every `config.get("Colors:Window", "BackgroundNormal")` call returns `None` because the keys were stored as `backgroundnormal`. The reader silently returns an empty theme with all fields `None` -- it "works" but produces no useful data.

**Why it happens:**
The `configparser` crate is modeled after Python's configparser which is case-insensitive by default. Developers use `Ini::new()` out of habit. The failure is silent -- no error, just missing data.

**How to avoid:**
Always use `Ini::new_cs()` (case-sensitive mode) for KDE kdeglobals parsing. Add an integration test that loads a real kdeglobals snippet and asserts that specific color values are `Some(...)`, not `None`. The test should fail loudly if case sensitivity is wrong.

**Warning signs:**
- `from_kde()` returns `Ok(theme)` but every color field is `None`
- Unit tests pass but integration tests with real kdeglobals data show empty results
- No compilation error, no runtime error -- pure silent data loss

**Phase to address:**
Phase 3 (Linux runtime readers). This must be caught by the first integration test for `from_kde()`.

---

### Pitfall 2: macOS NSColor Color Space Crash on Component Extraction

**What goes wrong:**
Calling `getRed:green:blue:alpha:` on an NSColor that is not in an RGB-compatible color space throws `NSInvalidArgumentException` and crashes the process. Many NSColor semantic colors (like `labelColor`, `controlAccentColor`) are "catalog colors" that live in a device-independent color space. Extracting RGB components without first converting to sRGB causes an unrecoverable Objective-C exception that bypasses Rust's panic infrastructure.

**Why it happens:**
macOS returns colors in P3 or catalog color spaces, not sRGB. Developers assume all NSColors have RGB components. The crash is an Objective-C exception, not a Rust panic, so `catch_unwind` does not help.

**How to avoid:**
Always convert to sRGB before extracting components:
```rust
let srgb = color.colorUsingColorSpace(&NSColorSpace::sRGBColorSpace());
// srgb can be None if conversion is impossible (rare but possible)
if let Some(srgb) = srgb {
    // Now safe to call getRed:green:blue:alpha:
}
```
Wrap every NSColor read in this conversion pattern. Never call component accessors on the raw catalog color.

**Warning signs:**
- Process crashes with `NSInvalidArgumentException` on macOS
- Crash happens only with certain system accent colors or on P3 displays
- Works in debug builds on Intel Macs, crashes on Apple Silicon with P3 displays

**Phase to address:**
Phase 5 (macOS reader). Every color extraction must go through a `nscolor_to_rgba()` helper that handles conversion. This is the single most dangerous platform-specific bug.

---

### Pitfall 3: ashpd Pulling tokio Into Sync-Only Consumers

**What goes wrong:**
Enabling the `portal` feature pulls in `ashpd`, which by default enables `tokio` via its default features. Downstream consumers who only want `from_kde()` (sync) but also enable `portal` for a different code path suddenly get tokio in their dependency tree. Worse, due to Cargo feature unification, if any crate in the dependency graph enables `portal`, ALL crates linking native-theme get tokio -- even if they never call async code.

**Why it happens:**
ashpd's default features include tokio. Feature unification means the union of all features is activated. Developers enable features thinking they are additive and isolated, but they affect the entire build graph.

**How to avoid:**
In Cargo.toml, disable ashpd's default features and let the consumer choose their runtime:
```toml
ashpd = { version = "0.13", optional = true, default-features = false }
```
Document that consumers must enable either `ashpd/tokio` or `ashpd/async-std` in their own Cargo.toml. Alternatively, provide `portal-tokio` and `portal-async-io` convenience features that pass through the runtime choice.

**Warning signs:**
- `cargo tree` shows tokio appearing even when only `kde` feature is enabled
- Compile times increase unexpectedly for sync-only consumers
- Runtime panics about missing tokio context when running async portal code under a different runtime

**Phase to address:**
Phase 3 (Linux runtime readers). Must be designed correctly from the start -- changing feature flag structure after publication is a breaking change.

---

### Pitfall 4: Merge Function Desynchronizes from Struct Fields After Adding New Fields

**What goes wrong:**
The `merge()` method on `ThemeColors` has 36 `if overlay.field.is_some() { self.field = overlay.field; }` lines. When a new field is added to `ThemeColors` (enabled by `#[non_exhaustive]`), the developer adds the field to the struct but forgets to add the corresponding line in `merge()`. The new field is silently ignored during theme layering. User overrides for the new field have no effect.

**Why it happens:**
Manual field-by-field merge implementations have no compile-time check that all fields are covered. The struct and the merge method are in different logical locations. `#[non_exhaustive]` specifically enables adding fields as non-breaking changes, creating a false sense that the compiler will catch issues.

**How to avoid:**
Use a declarative macro to define both the struct fields and the merge implementation from a single source of truth:
```rust
macro_rules! define_theme_colors {
    ($($field:ident),* $(,)?) => {
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        #[non_exhaustive]
        pub struct ThemeColors {
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<Rgba>,
            )*
        }
        impl ThemeColors {
            pub fn merge(&mut self, overlay: &ThemeColors) {
                $(
                    if overlay.$field.is_some() { self.$field = overlay.$field; }
                )*
            }
        }
    };
}
```
This guarantees merge() always covers every field. Apply the same pattern to `ThemeFonts`, `ThemeGeometry`, `ThemeSpacing`.

**Warning signs:**
- User-reported bug: "my override for field X doesn't work"
- Theme layering tests pass for old fields but nobody tests the new field in a layering scenario
- PR adds a field to the struct but not to merge -- review catches are unreliable for 36+ field structs

**Phase to address:**
Phase 1 (data model). The macro approach should be used from day one, not retrofitted.

---

### Pitfall 5: `#[serde(default)]` Missing on Nested Structs Breaks Forward Compatibility

**What goes wrong:**
If `ThemeVariant` does not have `#[serde(default)]` on its fields, a TOML file that omits an entire section (e.g., has `[light.colors]` but no `[light.fonts]` section) fails to deserialize instead of defaulting the missing section to `Default::default()`. This breaks the core promise of "specify only what you want to override."

**Why it happens:**
TOML treats a missing table as a missing field. Without `#[serde(default)]`, serde requires the field to be present. Developers test with complete TOML files and never test with partial files. The error message ("missing field `fonts`") is clear but appears only at runtime with real user files.

**How to avoid:**
Every struct field in the deserialization chain must have both:
- `#[serde(default)]` -- use Default::default() when the TOML section is absent
- All leaf fields as `Option<T>` -- individual missing keys become None

Add round-trip tests with minimal TOML files:
```rust
#[test]
fn minimal_toml_deserializes() {
    let toml = r#"
        name = "test"
        [light.colors]
        accent = "#ff0000"
    "#;
    let theme: NativeTheme = toml::from_str(toml).unwrap();
    assert!(theme.light.unwrap().colors.accent.is_some());
    assert!(theme.light.unwrap().fonts.family.is_none()); // defaulted, not error
}
```

**Warning signs:**
- Deserialization errors when users provide minimal TOML override files
- All example TOML files in tests are "complete" (every section present)
- Works with bundled presets (which are complete) but breaks with user overlays

**Phase to address:**
Phase 1 (data model + TOML serde). Must be correct from the first commit.

---

### Pitfall 6: KDE kdeglobals Font Parsing Assumes Fixed Field Count

**What goes wrong:**
KDE stores fonts in Qt's `QFont::toString()` format: `"Noto Sans,10,-1,5,50,0,0,0,0,0"` (Qt 4: 10 fields) or `"Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1"` (Qt 5/6: 16 fields). Parsing code that assumes exactly 10 or 16 fields, or that does `parts[1].parse::<f32>()` without bounds checking, will panic or return wrong results when encountering a different Qt version or a font family name containing commas.

**Why it happens:**
The format evolved across Qt versions. Font family names can contain commas (rare but legal). Developers test with their own system and don't encounter edge cases.

**How to avoid:**
Parse defensively: split by comma, take field 0 as family (but handle quoted names), take field 1 as point size, ignore all remaining fields. Return `None` for any field that fails to parse rather than panicking. Test with both Qt 4 (10-field) and Qt 6 (16-field) format strings, plus edge cases like empty strings and single-field strings.

**Warning signs:**
- Panics on systems running older KDE/Qt versions
- Wrong font sizes reported (parsing the wrong field index)
- Font family names with special characters cause parse failures

**Phase to address:**
Phase 3 (KDE reader). Include specific test vectors for Qt 4, Qt 5, and Qt 6 font string formats.

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Manual merge() instead of macro | Faster to write initially | Desync bugs when adding fields (Pitfall 4) | Never -- use the macro from day one |
| Hardcoding Adwaita color values | Avoids dconf/GSettings dependency | Stale colors when GNOME updates Adwaita | Acceptable for MVP; document staleness risk |
| Skip `#[serde(default)]` on nested structs | Slightly simpler code | Breaks partial TOML deserialization (Pitfall 5) | Never |
| Using `Ini::new()` instead of `Ini::new_cs()` | Compiles and runs fine | All kdeglobals data silently lost (Pitfall 1) | Never |
| Putting Linux deps under `[dependencies]` not `[target]` | Avoids feature unification issues | Compiles unused code on non-Linux; may confuse downstream audits | Acceptable trade-off given Cargo resolver 2 limitations |
| `include_str!()` for all presets at once | Simple implementation | Binary size grows linearly with preset count; 7+ presets at ~4KB each = ~28KB always embedded | Acceptable for < 15 presets; use lazy loading for community presets |

## Integration Gotchas

Common mistakes when connecting to platform-specific APIs.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| KDE kdeglobals | Using `~/.config/kdeglobals` hardcoded path | Use `dirs::config_dir()` which respects `$XDG_CONFIG_HOME`; also check system paths in `$XDG_CONFIG_DIRS` (defaults to `/etc/xdg`) |
| Freedesktop portal accent-color | Clamping out-of-range `(f64, f64, f64)` values to [0,1] | Out-of-range values mean "no accent color set" per the spec; treat as `None`, not as clamped color |
| Windows UISettings | Calling `GetColorValue` without API presence check | Call `ApiInformation::IsMethodPresent` first; gracefully degrade on Windows versions before 10 |
| macOS NSColor | Extracting RGB components from catalog colors directly | Always convert via `colorUsingColorSpace(&NSColorSpace::sRGBColorSpace())` first; handle `None` return |
| macOS NSAppearance | Reading semantic colors without resolving the current appearance | Use `performAsCurrentDrawingAppearance` (macOS 11+) to resolve dynamic colors before extraction |
| ashpd portal | Assuming the portal is always available on Linux | D-Bus may not be running (headless servers, minimal containers); return `Error::Unavailable`, don't panic |
| configparser KDE fonts | Splitting font string on commas naively | Qt font strings have fixed positional fields; handle variable field counts (Qt 4 = 10, Qt 5/6 = 16) |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Parsing all embedded presets at startup | Slow `list_presets()` if it parses TOML eagerly | Parse lazily on first access per preset; `list_presets()` returns names only (from a const array, not by parsing) | Noticeable with 15+ presets at ~2ms each |
| Creating a new D-Bus connection per `from_gnome()` call | High latency (~50-100ms per D-Bus handshake) on repeated calls | Cache the ashpd `Settings` object or let the caller manage connection lifetime | When theme is re-read on every frame or frequently |
| Repeated KDE INI parsing without caching | Disk I/O on every `from_kde()` call | Document that callers should cache the result; `from_kde()` is a snapshot, not a live view | Noticeable when called in hot paths (> 1 call/sec) |
| `to_toml()` allocating large strings for complete themes | Memory pressure with many serialization calls | Not a real issue -- TOML for 36 colors is < 2KB; only matters if serializing thousands of themes in a loop | Effectively never for this crate's use case |

## Security Mistakes

Domain-specific security issues beyond general concerns.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Reading arbitrary file paths via `from_file()` without validation | Path traversal if user input is passed directly as the path | Document that `from_file()` trusts its input; callers must validate paths. Do not resolve symlinks or `..` internally -- that is the caller's responsibility |
| Trusting kdeglobals color values without range validation | Malformed INI could produce `Rgba` with values that cause downstream overflow | `u8::from_str_radix` naturally constrains to 0-255; validate that parsed integers fit in u8 range before casting |
| D-Bus message parsing without size limits | Malicious D-Bus messages could cause excessive allocation | ashpd/zbus handle this internally; no action needed for this crate |

## UX Pitfalls

Common user experience mistakes in this domain.

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Returning `Err` when a single color field is unreadable | Entire theme read fails because one obscure field returned an error | Populate what you can, leave unreadable fields as `None`; only return `Err` for total failures (file not found, no D-Bus) |
| Preset names case-sensitive (`"kde-breeze"` vs `"KDE-Breeze"`) | Users type wrong case, get `None` back | Normalize preset names to lowercase internally; accept any case in `preset()` |
| No indication of which fields a preset/reader actually populated | Users don't know if `None` means "not supported on this platform" or "bug in the reader" | Document per-preset which fields are expected to be `Some`; consider a `coverage()` method returning field completeness stats |
| Opaque `Error::Platform(Box<dyn Error>)` messages | Users see "platform error" with no actionable detail | Include platform-specific context in the error message (e.g., "NSColor conversion failed for controlAccentColor") |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Rgba serde:** Often missing support for uppercase hex (`#FF0000` vs `#ff0000`) -- verify both parse correctly
- [ ] **Rgba serde:** Often missing 3-digit shorthand (`#F00`) -- decide whether to support it and document the decision
- [ ] **Preset TOML files:** Often missing the dark variant -- verify every preset has both `[light]` and `[dark]` sections
- [ ] **merge():** Often missing newly added fields -- verify field count in merge matches struct field count (macro solves this)
- [ ] **from_kde() dark mode detection:** Often returns wrong variant -- verify detection heuristic against actual KDE dark themes (check `[General] ColorScheme` key first, then luminance fallback)
- [ ] **from_system() on GNOME:** Often falls through to empty preset -- verify that the Adwaita preset actually loads when no portal/kde features are enabled
- [ ] **serde round-trip:** Often loses alpha channel (`#ff000080` -> `#ff0000`) -- verify 8-digit hex survives serialize/deserialize
- [ ] **ThemeSpacing in presets:** Often copied between presets without adjustment -- verify Material spacing (8dp grid) differs from Breeze (6px default) differs from Adwaita (8px default)
- [ ] **`skip_serializing_if`:** Often missing on some fields, producing verbose TOML with `accent = null` lines -- verify clean output

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| configparser case sensitivity (Pitfall 1) | LOW | Change `Ini::new()` to `Ini::new_cs()` -- single line fix, no API change |
| NSColor crash (Pitfall 2) | MEDIUM | Add `colorUsingColorSpace` wrapper; audit all NSColor call sites; requires macOS testing |
| tokio leaked into dependency tree (Pitfall 3) | HIGH | Changing ashpd feature configuration after publish is a semver-visible change; need a minor version bump with migration guide |
| merge() desync (Pitfall 4) | LOW | Add the missing field to merge(); add a test. If using macro approach, impossible to reach this state |
| Missing `#[serde(default)]` (Pitfall 5) | MEDIUM | Add the attribute; but users with broken TOML files may have already worked around it, creating legacy expectations |
| KDE font parsing (Pitfall 6) | LOW | Fix the parser to handle variable field counts; purely internal change |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| configparser case sensitivity | Phase 3 (KDE reader) | Integration test with real kdeglobals snippet asserts `accent.is_some()` |
| NSColor color space crash | Phase 5 (macOS reader) | Unit test mocking NSColor; manual test on macOS with P3 display |
| ashpd tokio dependency leak | Phase 3 (portal feature design) | `cargo tree` check in CI verifying tokio absent when only `kde` feature enabled |
| merge() desync from struct fields | Phase 1 (data model) | Use declarative macro; compile-time guarantee. Alternatively: test that counts struct fields matches merge coverage |
| `#[serde(default)]` on nested structs | Phase 1 (data model) | Round-trip test with minimal TOML (single field only) |
| KDE font string parsing | Phase 3 (KDE reader) | Test vectors for Qt 4, Qt 5, Qt 6 font string formats |
| Portal accent-color out-of-range | Phase 3 (portal reader) | Test with `(2.0, -1.0, 0.5)` values; assert returns `None` not clamped color |
| Preset name case sensitivity | Phase 1 (preset loading) | Test that `preset("KDE-Breeze")` and `preset("kde-breeze")` return the same result |
| from_system() fallback chain | Phase 3 (cross-platform dispatch) | Test on minimal Linux (no KDE, no portal) returns Adwaita preset, not error |
| Windows API availability check | Phase 5 (Windows reader) | Test on Windows 8.1 VM (or document minimum Windows 10 requirement) |

## Sources

- [configparser case sensitivity issue](https://github.com/QEDK/configparser-rs/issues/6) -- confirms `Ini::new()` lowercases keys by default
- [Apple NSColor getRed:green:blue:alpha: docs](https://developer.apple.com/documentation/appkit/nscolor/getred(_:green:blue:alpha:)) -- documents crash on wrong color space
- [objc2 MainThreadMarker docs](https://docs.rs/objc2-foundation/latest/objc2_foundation/struct.MainThreadMarker.html) -- thread safety requirements
- [XDG Desktop Portal Settings spec](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html) -- out-of-range accent color = unset
- [Effective Rust: Be wary of feature creep](https://effective-rust.com/features.html) -- feature unification pitfalls
- [Cargo feature unification docs](https://doc.rust-lang.org/cargo/reference/features.html) -- resolver v2 behavior
- [SemVer Compatibility - Cargo Book](https://doc.rust-lang.org/cargo/reference/semver.html) -- struct field additions and `#[non_exhaustive]`
- [ashpd GitHub](https://github.com/bilelmoussaoui/ashpd) -- default features include tokio
- [dark-light issue #69](https://github.com/frewsxcv/rust-dark-light/issues/69) -- COSMIC/tokio dependency conflict (same class of issue)
- [KDE bug 384950](https://bugs.kde.org/show_bug.cgi?id=384950) -- kdeglobals can be out of sync with active color scheme
- [system-theme 0.3.0](https://crates.io/crates/system-theme) -- prior art demonstrating the pitfalls of hard tokio dep, no kdeglobals fallback, minimal palette
- [rust-lang/api-guidelines #152](https://github.com/rust-lang/api-guidelines/issues/152) -- platform-specific crates should compile empty on wrong platform, not fail

---
*Pitfalls research for: cross-platform Rust OS theme data crate*
*Researched: 2026-03-07*
