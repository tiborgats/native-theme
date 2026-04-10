# Platform reader testing with fixture data

Status: Not started
Date: 2026-04-09

---

## Problem

The four platform readers (KDE, GNOME, macOS, Windows) are the highest-value
code in the crate — they are the reason users depend on native-theme. Yet
they have zero automated test coverage.

CI runs on Linux and cannot call macOS NSColor or Windows UISettings APIs.
Even on Linux, CI has no KDE session (no kdeglobals file) and no D-Bus
portal (no GNOME portal responses). The platform readers are tested only
manually by the maintainer.

This means:

- **Regressions in parsing logic go undetected.** A refactor of KDE color
  parsing could silently break, and the test suite would pass.
- **Edge cases are never exercised.** What happens when kdeglobals has a
  missing `[Colors:Window]` group? When a color value is `255,255` instead
  of `255,255,255`? When the portal returns an out-of-range accent color?
- **New field additions are untested.** Adding a new field to the KDE reader
  has no way to verify it extracts correctly without running on KDE.

### What IS tested today

The test suite thoroughly covers the pure data model:

- Merge semantics (604 lines)
- Resolution + validation (432 lines + 2,152 internal lines)
- Preset loading (378 lines)
- Serde roundtrip (475 lines)
- Property-based tests (614 lines)
- Platform-facts cross-reference (220 lines)

What's NOT tested: the code that reads OS configuration and populates
`ThemeSpec` from it.

---

## Options

### Option A: Trait-based mocking

Define a trait for each platform's data source (e.g., `trait KdeConfigSource`,
`trait PortalSource`) and inject a mock implementation in tests.

**Pros:**
- Standard Rust testing pattern. Familiar to contributors.
- Can mock error conditions (D-Bus timeout, file permission denied).

**Cons:**
- Requires restructuring every reader around a trait it doesn't naturally need.
  The KDE reader reads one file; the GNOME reader makes ~8 async calls.
  Wrapping each in a trait adds indirection for test-only purposes.
- Mock implementations must mirror the real API surface — a maintenance
  burden that grows with every new field or portal key.
- Doesn't help test the actual parsing logic in isolation — the mock returns
  already-parsed values, skipping the fragile string/INI/D-Bus parsing.

### Option B: Fixture-based unit tests (recommended)

Separate **data parsing** from **data acquisition** in each reader. The
parser is a pure function testable with fixture data; the acquisition layer
is a thin wrapper that reads from the OS and passes raw data to the parser.

**Pros:**
- Tests the actual parsing code — the fragile part (INI color extraction,
  Qt font string parsing, D-Bus value conversion).
- Fixtures are real data: captured from actual KDE/GNOME/Windows/macOS
  sessions, so they test realistic inputs.
- No new traits, no indirection. The refactor just moves the existing
  function boundary.
- Fixture files are self-documenting (read the INI, see what the test expects).

**Cons:**
- Cannot test the I/O layer itself (file reading, D-Bus connection).
  But that layer is thin (~5 lines per reader) and covered by platform CI.
- Fixtures can go stale if OS config formats change. Mitigated by capturing
  from current OS versions.
- GNOME reader: the `PortalData` intermediate struct adds a type that exists
  only for testability. Acceptable cost.

### Option C: Snapshot / golden-file testing

Run each reader on a real system, serialize the ThemeSpec output to JSON,
and commit it as a golden file. Tests re-run the reader and diff against
the golden file.

**Pros:**
- Tests the full pipeline end-to-end.
- Detects any output change, including regressions and intentional changes.

**Cons:**
- Requires a real KDE/GNOME/macOS/Windows session — cannot run in CI.
- Golden files must be updated manually after every intentional change.
- Only tests one configuration per golden file (the maintainer's).
  Edge cases (missing groups, malformed values) are never covered.

### Why Option B

Option A tests mocks, not code. Option C can't run in CI. Option B tests
the actual parsing logic with controlled inputs — the right layer to verify.

---

## Per-reader design

### KDE reader (`kde/`)

**Current architecture:**

```
from_kde() → read kdeglobals file → parse INI → extract colors/fonts/metrics → ThemeSpec
```

The INI parsing (`configparser`) and value extraction (color parsing, font
parsing, metric extraction) are interleaved with file I/O.

**Refactored architecture:**

```
from_kde() → read kdeglobals file → parse_kdeglobals(&str) → ThemeSpec
                                    ^^^^^^^^^^^^^^^^^^^^^^^^
                                    This function is testable with fixture data
```

Specifically, split the existing code into:

- `parse_kdeglobals(content: &str) -> Result<ThemeSpec>` — pure function,
  takes INI file content as string, returns ThemeSpec. This is the testable
  core.
- `from_kde() -> Result<ThemeSpec>` — reads `~/.config/kdeglobals`, calls
  `parse_kdeglobals()`.

**Fixture files to create:**

| Fixture | Purpose | Source |
|---------|---------|--------|
| `tests/fixtures/kde/breeze-light.ini` | Standard Breeze Light theme | Real kdeglobals from KDE Plasma 6 |
| `tests/fixtures/kde/breeze-dark.ini` | Standard Breeze Dark theme | Real kdeglobals from KDE Plasma 6 |
| `tests/fixtures/kde/custom-accent.ini` | Custom accent color | Modified kdeglobals with non-default accent |
| `tests/fixtures/kde/minimal.ini` | Only required groups present | Hand-crafted minimal file |
| `tests/fixtures/kde/missing-colors-window.ini` | Missing [Colors:Window] group | Tests graceful degradation |
| `tests/fixtures/kde/malformed-color.ini` | `BackgroundNormal=255,255` (2 values) | Tests error handling |
| `tests/fixtures/kde/high-dpi.ini` | `forceFontDPI=192` | Tests DPI extraction |

**Test assertions:**

```rust
#[test]
fn kde_breeze_light_fixture() {
    let content = include_str!("fixtures/kde/breeze-light.ini");
    let spec = parse_kdeglobals(content).unwrap();
    let variant = spec.light.unwrap();

    // Window background is Breeze Light's default
    assert_eq!(variant.window.background_color, Some(Rgba::from_hex("#eff0f1")));
    // Accent is Breeze blue
    assert_eq!(variant.defaults.accent_color, Some(Rgba::from_hex("#3daee9")));
    // Font was parsed
    assert_eq!(variant.defaults.font.family, Some("Noto Sans".to_string()));
}
```

### GNOME reader (`gnome/`)

**Current architecture:**

```
from_gnome() → async ashpd portal queries → parse responses → merge with adwaita → ThemeSpec
```

The portal queries (`ashpd::desktop::settings`) return strongly-typed Rust
values. The parsing and merging logic can be extracted.

**Refactored architecture:**

```
from_gnome() → query portal → build_gnome_spec(PortalData) → merge with adwaita → ThemeSpec
                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                               Testable with constructed PortalData
```

Define a `PortalData` struct using primitive types (not ashpd types) so
tests compile without the `portal` feature:

```rust
/// Collected portal + gsettings data, separated from D-Bus I/O.
/// Uses primitive types so tests don't depend on ashpd.
pub(crate) struct PortalData {
    pub color_scheme: Option<u32>,              // 0=none, 1=dark, 2=light (XDG spec)
    pub accent_color: Option<(f64, f64, f64)>,  // RGB 0.0-1.0
    pub contrast: Option<u32>,                  // 0=none, 1=more
    pub reduce_motion: Option<bool>,
    pub font_name: Option<String>,              // "Cantarell 11"
    pub monospace_font: Option<String>,         // "Source Code Pro 10"
    pub document_font: Option<String>,
    pub text_scaling_factor: Option<f64>,
    pub icon_theme: Option<String>,
}
```

Tests construct `PortalData` directly:

```rust
#[test]
fn gnome_dark_with_accent() {
    let data = PortalData {
        color_scheme: Some(ColorScheme::PreferDark),
        accent_color: Some((0.24, 0.68, 0.91)),
        ..Default::default()
    };
    let spec = build_gnome_spec(data).unwrap();
    assert!(spec.dark.is_some());
}
```

### Windows reader (`windows.rs`)

**Current architecture:**

```
from_windows() → call UISettings/GetSysColor/NONCLIENTMETRICSW/etc → extract → ThemeSpec
```

**Refactored architecture:**

```
from_windows() → read_windows_data() → build_windows_spec(WindowsData) → ThemeSpec
                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                                        Testable
```

Define `WindowsData`:

```rust
pub(crate) struct WindowsData {
    pub accent: Rgba,
    pub accent_light1: Rgba,
    pub accent_light2: Rgba,
    pub accent_light3: Rgba,
    pub accent_dark1: Rgba,
    pub accent_dark2: Rgba,
    pub accent_dark3: Rgba,
    pub sys_colors: HashMap<i32, u32>,  // GetSysColor results
    pub foreground: Rgba,               // UISettings foreground
    pub message_font: FontInfo,
    pub caption_font: FontInfo,
    pub menu_font: FontInfo,
    pub status_font: FontInfo,
    pub dpi: u32,
    pub text_scale: f64,
    pub high_contrast: bool,
    pub animations_enabled: bool,
    pub icon_sizes: [u32; 4],           // small/standard/large/extralarge
}
```

**Fixture approach:** Since there's no INI file to load, fixtures are
constructed `WindowsData` values representing known Windows configurations
(default light, default dark, high-contrast, custom accent).

### macOS reader (`macos.rs`)

Same pattern as Windows. Define `MacOSData` struct with the extracted
NSColor/NSFont values, separate `build_macos_spec(MacOSData)` from the
Objective-C FFI calls.

---

## What specifically to test per reader

### KDE (highest priority — INI parsing is most fragile)

1. **Color extraction:** All [Colors:*] groups → correct Rgba values
2. **Font parsing:** Qt font strings → family + weight + size
   - `"Noto Sans,10,-1,5,50,0,0,0,0,0"` (KDE 5 format)
   - `"Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1"` (KDE 6 format)
   - Bold weight: `"Noto Sans Bold,10,-1,5,75,0,0,0,0,0"` → weight 700
3. **DPI chain:** forceFontDPI → Xft.dpi → physical → 96 fallback
4. **Dark mode detection:** Background luminance calculation
5. **Missing groups:** Graceful fallback when optional groups absent
6. **Malformed values:** Two-component color, empty string, non-numeric

### GNOME (medium priority — portal types are well-structured)

1. **Accent color out-of-range:** XDG spec says values can exceed 1.0
2. **Color scheme variants:** light, dark, prefer-dark, no-preference
3. **Font name parsing:** `"Cantarell 11"` → family + size
4. **Missing portal data:** All fields None → falls back to Adwaita base
5. **Text scaling factor:** 1.0, 1.25, 2.0 → correct propagation

### Windows (medium priority — API types are strongly typed)

1. **Accent shade derivation:** Accent + 6 shade colors → correct mapping
2. **System color mapping:** GetSysColor results → widget theme colors
3. **Font weight mapping:** NONCLIENTMETRICSW weight → CSS weight
4. **DPI scaling:** 96, 120, 144, 192 → correct font size conversion
5. **Dark mode inference:** Foreground luminance > 128 → dark

### macOS (lower priority — smallest API surface)

1. **System color mapping:** NSColor → Rgba
2. **Font weight inference:** NSFont weight → CSS weight
3. **Appearance detection:** darkAqua → dark, aqua → light

---

## File layout

```
native-theme/
├── tests/
│   ├── fixtures/
│   │   └── kde/
│   │       ├── breeze-light.ini
│   │       ├── breeze-dark.ini
│   │       ├── custom-accent.ini
│   │       ├── minimal.ini
│   │       ├── missing-colors-window.ini
│   │       ├── malformed-color.ini
│   │       └── high-dpi.ini
│   └── reader_kde.rs          (integration tests)
├── src/
│   ├── kde/
│   │   ├── mod.rs             (from_kde unchanged; parse_kdeglobals extracted)
│   │   ├── colors.rs          (color parsing — already separate)
│   │   ├── fonts.rs           (font parsing — already separate)
│   │   └── metrics.rs         (if exists)
│   ├── gnome/
│   │   └── mod.rs             (from_gnome unchanged; build_gnome_spec extracted)
│   ├── windows.rs             (from_windows unchanged; build_windows_spec extracted)
│   └── macos.rs               (from_macos unchanged; build_macos_spec extracted)
```

---

## Implementation order

1. **KDE first** — largest attack surface (INI parsing, font strings, DPI chain),
   runs on the CI Linux runner, no async complications.
2. **GNOME second** — the `PortalData` struct simplifies testing; no async needed
   for the builder function.
3. **Windows third** — can test `build_windows_spec()` on Linux (it's a pure
   function), even though `read_windows_data()` only works on Windows.
4. **macOS last** — smallest reader, similar pattern to Windows.

## Risk

Low. This is additive (new tests + internal refactor of reader functions).
Public API does not change. The refactor separates parsing from I/O, which
is a strict improvement to testability without changing behavior.

## Verification

- All existing tests continue to pass
- New fixture tests pass on CI (Linux runner, no DE session needed)
- KDE fixture tests produce the same ThemeSpec as live `from_kde()` on a
  real KDE session (manual verification by maintainer, once)
