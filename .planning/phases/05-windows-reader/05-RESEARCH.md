# Phase 5: Windows Reader - Research

**Researched:** 2026-03-07
**Domain:** Windows theme detection (WinRT UISettings, Win32 GetSystemMetrics, `windows` crate, feature flags)
**Confidence:** HIGH

## Summary

This phase implements `from_windows()`, a sync function that reads the user's live accent colors, foreground/background, and system geometry metrics from Windows APIs and returns a `NativeTheme`. Two distinct Windows API surfaces are needed: WinRT `UISettings` (from `Windows.UI.ViewManagement`) for colors and UI metrics, and Win32 `GetSystemMetrics` (from `winuser.h`) for geometry values like border width, scroll bar width, and frame dimensions.

The `windows` crate (v0.59-0.62, current latest 0.62.2) is the only viable Rust binding because it supports both WinRT and Win32 APIs. The alternative `windows-sys` crate only provides raw bindings for C-style Win32 APIs and explicitly does **not** support WinRT (COM/WinRT). Since `UISettings` is a WinRT class, `windows-sys` cannot be used. The `windows` crate must be added as an optional dependency behind a `"windows"` feature flag, with only two feature gates enabled: `"UI_ViewManagement"` (for UISettings, UIColorType, Color) and `"Win32_UI_WindowsAndMessaging"` (for GetSystemMetrics and SM_* constants).

`UISettings` provides `GetColorValue(UIColorType)` which returns a `windows::UI::Color` struct with u8 fields `{A, R, G, B}` -- a direct match for this crate's `Rgba` type. The available `UIColorType` variants include `Accent`, `AccentDark1-3`, `AccentLight1-3`, `Background`, `Foreground`, and `Complement` (note: `Complement` throws an exception and must be avoided). Dark/light mode detection uses the foreground color luminance: if the foreground is light (luminance > 128), the system is in dark mode. `GetSystemMetrics` is a Win32 function available since Windows 2000 that returns i32 pixel values for system dimensions (border width, scroll bar width, icon size, etc.).

WinRT APIs (`UISettings`) require Windows 10 or later. On older Windows versions (7, 8, 8.1), `UISettings::new()` will fail. The function must degrade gracefully: attempt UISettings first, and if it fails, return `Error::Unavailable` with a descriptive message -- or return a partial theme with only GetSystemMetrics geometry data. Per the project's established pattern (KDE returns `Error::Unavailable`, GNOME falls back to Adwaita), returning `Error::Unavailable` on pre-Win10 systems is the correct behavior.

**Primary recommendation:** Use `windows` crate 0.62 with `default-features = false` and features `["UI_ViewManagement", "Win32_UI_WindowsAndMessaging"]`. Read accent + foreground + background colors from UISettings, geometry from GetSystemMetrics. Populate only the active variant (light or dark) based on foreground luminance. Return `Error::Unavailable` if UISettings creation fails (pre-Win10).

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-04 | Windows reader: from_windows() -- UISettings + GetSystemMetrics (feature "windows") | `windows` crate 0.62.2 verified as only option supporting both WinRT (UISettings) and Win32 (GetSystemMetrics). UISettings::new() + GetColorValue(UIColorType::Accent) API verified. GetSystemMetrics SM_* constants verified. Feature flags `UI_ViewManagement` + `Win32_UI_WindowsAndMessaging` identified as minimal set. Graceful degradation via Error::Unavailable on pre-Win10 follows KDE reader pattern. |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| windows | >=0.59, <=0.62 | WinRT UISettings + Win32 GetSystemMetrics bindings | Official Microsoft crate. Only Rust crate supporting both WinRT and Win32 APIs. Required for UISettings (WinRT) which windows-sys cannot provide. |

### Supporting
No additional dependencies. The `windows` crate pulls in `windows-core`, `windows-targets`, and link libraries as transitive dependencies, but no extra crates are needed.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| windows | windows-sys | windows-sys does NOT support WinRT APIs (UISettings). Only provides raw Win32 bindings. Would require a completely different approach for accent color (registry reading, which is fragile and undocumented). |
| windows | winapi (deprecated) | Unmaintained since 2021, does not support WinRT. Use windows crate instead. |
| windows (UISettings) | Registry HKCU\...\Personalize\AccentColor | Undocumented, fragile, does not track "automatic accent color" correctly. UISettings is the official API. |

**Installation (Cargo.toml changes):**
```toml
[features]
windows = ["dep:windows"]

[dependencies]
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
] }
```

## Architecture Patterns

### Recommended Project Structure
```
src/
  windows.rs            # pub fn from_windows() -> Result<NativeTheme> + internal helpers
  kde/                   # (existing) sync KDE reader
  gnome/                 # (existing) async GNOME reader
  model/                 # (existing) data model
  lib.rs                 # Add: #[cfg(feature = "windows")] pub mod windows; + re-export
```

Note: A single `windows.rs` file is sufficient (no submodules needed). The Windows reader is simpler than KDE (no INI parsing) and GNOME (no async/preset overlay). It directly calls APIs and maps results.

### Pattern 1: UISettings Color Extraction
**What:** Create a `UISettings` instance and call `GetColorValue()` with each `UIColorType` variant to extract accent, foreground, and background colors. Convert `windows::UI::Color {A, R, G, B}` to `crate::Rgba`.
**When to use:** Always -- this is the primary color source on Windows 10+.
**Example:**
```rust
// Source: https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UISettings.html
use windows::UI::ViewManagement::{UISettings, UIColorType};

fn read_colors() -> Option<(crate::Rgba, crate::Rgba, crate::Rgba)> {
    let settings = UISettings::new().ok()?;

    let accent = settings.GetColorValue(UIColorType::Accent).ok()?;
    let fg = settings.GetColorValue(UIColorType::Foreground).ok()?;
    let bg = settings.GetColorValue(UIColorType::Background).ok()?;

    Some((
        crate::Rgba::rgba(accent.R, accent.G, accent.B, accent.A),
        crate::Rgba::rgba(fg.R, fg.G, fg.B, fg.A),
        crate::Rgba::rgba(bg.R, bg.G, bg.B, bg.A),
    ))
}
```

### Pattern 2: GetSystemMetrics Geometry Extraction
**What:** Call `GetSystemMetrics` with relevant SM_* constants to populate `ThemeGeometry` fields.
**When to use:** Always available on Windows 2000+, complements UISettings.
**Example:**
```rust
// Source: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXBORDER, SM_CXVSCROLL};

fn read_geometry() -> crate::ThemeGeometry {
    // SAFETY: GetSystemMetrics is always safe to call, returns 0 on failure
    let border_width = unsafe { GetSystemMetrics(SM_CXBORDER) };
    let scroll_width = unsafe { GetSystemMetrics(SM_CXVSCROLL) };

    crate::ThemeGeometry {
        frame_width: Some(border_width as f32),
        scroll_width: Some(scroll_width as f32),
        ..Default::default()
    }
}
```

### Pattern 3: Dark Mode Detection via Foreground Luminance
**What:** Detect dark/light mode by checking if the system foreground color is light (dark mode) or dark (light mode). Use the same BT.601 luminance formula as the KDE reader.
**When to use:** Always -- Windows does not expose a direct "is dark mode" API through UISettings. The foreground color luminance is the established detection method used by winit, dark-light crate, and Microsoft's own documentation.
**Example:**
```rust
fn is_dark_mode(foreground: &crate::Rgba) -> bool {
    let luma = 0.299 * (foreground.r as f32)
             + 0.587 * (foreground.g as f32)
             + 0.114 * (foreground.b as f32);
    luma > 128.0  // Light foreground = dark background = dark mode
}
```

### Pattern 4: Single Active Variant (Matching KDE/GNOME Pattern)
**What:** Populate only `light` OR `dark` variant in the returned `NativeTheme`, never both. The detected mode determines which variant gets populated.
**When to use:** Always -- this is an established project convention from Phase 3 (KDE) and Phase 4 (GNOME).
**Example:**
```rust
let theme = if is_dark {
    crate::NativeTheme {
        name: "Windows".to_string(),
        light: None,
        dark: Some(variant),
    }
} else {
    crate::NativeTheme {
        name: "Windows".to_string(),
        light: Some(variant),
        dark: None,
    }
};
```

### Anti-Patterns to Avoid
- **Using UIColorType::Complement:** This variant throws an exception at runtime per Microsoft docs. Never pass it to `GetColorValue()`.
- **Reading registry for accent color:** The registry key `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize\AccentColor` is undocumented and does not track "automatic accent color" correctly. Always use UISettings.
- **Assuming UISettings always succeeds:** It requires WinRT, which requires Windows 10+. Always handle the `Err` case from `UISettings::new()`.
- **Naming the module `windows`:** The Rust module name `windows` would collide with the `windows` crate name in scope. Name the file/module `windows.rs` but use `#[path = "windows.rs"] mod windows_reader;` or simply name it `win.rs` to avoid ambiguity. Alternatively, use `mod windows` since Rust can disambiguate between the module and the external crate via the `::windows` (crate) vs `crate::windows` (module) paths, but be careful about confusion.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Accent color reading | Registry parsing or DWM color API | `UISettings::GetColorValue(UIColorType::Accent)` | Official API, handles "automatic accent color", returns all accent shades |
| System metric reading | P/Invoke-style raw FFI | `windows` crate's `GetSystemMetrics()` | Type-safe wrappers, correct linking, SM_* constants provided |
| WinRT activation | Manual COM initialization/RoActivateInstance | `UISettings::new()` | The `windows` crate handles COM initialization, IInspectable, and reference counting |
| Color type conversion | Manual struct layout matching | Direct field access on `windows::UI::Color` | Color struct has public `R, G, B, A: u8` fields, maps 1:1 to Rgba |

**Key insight:** The `windows` crate abstracts away all COM/WinRT initialization, reference counting, and error handling. A UISettings call that would be 20+ lines of raw COM code becomes `UISettings::new()?.GetColorValue(UIColorType::Accent)?`.

## Common Pitfalls

### Pitfall 1: Module Name Collision with `windows` Crate
**What goes wrong:** Creating `src/windows.rs` and `mod windows` creates ambiguity with the `windows` external crate dependency. Inside `src/windows.rs`, `use windows::...` could be interpreted as the module rather than the crate.
**Why it happens:** Rust 2024 edition uses the crate name for external crates, and `windows` is both the module name and the crate name.
**How to avoid:** Inside the module, use `use ::windows::...` (with leading `::`) to unambiguously refer to the external crate. Or name the module `win.rs` / `win/mod.rs` to avoid the collision entirely.
**Warning signs:** Compilation errors about "cannot find `UI` in `windows`" or similar path resolution failures.

### Pitfall 2: UIColorType::Complement Crashes
**What goes wrong:** Calling `GetColorValue(UIColorType::Complement)` throws a runtime exception (HRESULT error).
**Why it happens:** Microsoft documentation explicitly warns: "The UIColorType.Complement value is not supported and will cause an exception if used."
**How to avoid:** Never use `UIColorType::Complement`. Only use `Accent`, `AccentLight1-3`, `AccentDark1-3`, `Background`, `Foreground`.
**Warning signs:** `HRESULT` error / panic when calling GetColorValue.

### Pitfall 3: WinRT Unavailable on Pre-Windows 10
**What goes wrong:** `UISettings::new()` returns an error on Windows 7/8/8.1 because WinRT APIs are not available.
**Why it happens:** WinRT was introduced with Windows 8 for Store apps, but the UISettings desktop activation pathway requires Windows 10.
**How to avoid:** Handle the error from `UISettings::new()` and return `Error::Unavailable`. GetSystemMetrics works on all Windows versions (since Windows 2000) and can still provide geometry data.
**Warning signs:** `Err` from `UISettings::new()` in testing on older Windows.

### Pitfall 4: GetSystemMetrics Returns 0 on Failure
**What goes wrong:** GetSystemMetrics returns 0 both as a valid value (e.g., SM_CLEANBOOT = 0 means normal boot) and as an error indicator.
**Why it happens:** Win32 API design -- no separate error channel for this function.
**How to avoid:** For the metrics we care about (border width, scroll width, icon size), 0 is a valid but unlikely value. Treat all values as valid. The function essentially never fails for the SM_CX*/SM_CY* metrics we need.
**Warning signs:** Unexpected 0 values in ThemeGeometry fields.

### Pitfall 5: DPI Awareness
**What goes wrong:** GetSystemMetrics returns physical pixels, not logical pixels. On high-DPI displays, values are scaled.
**Why it happens:** GetSystemMetrics is not DPI-aware per Microsoft docs. The DPI-aware version is `GetSystemMetricsForDpi()`.
**How to avoid:** For this phase, document that values are in physical pixels. The ThemeGeometry fields describe "logical pixels" in the model but the Windows API returns physical pixels. Use GetSystemMetrics as-is and note the limitation. A future phase could use `GetSystemMetricsForDpi()` if needed.
**Warning signs:** Geometry values that seem too large on high-DPI displays.

### Pitfall 6: Unsafe Block for GetSystemMetrics
**What goes wrong:** GetSystemMetrics is marked `unsafe` in the windows crate, so calling it requires an `unsafe` block.
**Why it happens:** All Win32 FFI functions are marked unsafe in the windows crate, even when they are de facto safe.
**How to avoid:** Wrap in a minimal `unsafe` block with a SAFETY comment. The function is always safe to call with valid SM_* constants.
**Warning signs:** Compiler error about unsafe function called outside unsafe block.

## Code Examples

Verified patterns from official sources:

### Complete from_windows() Implementation Skeleton
```rust
// Source: Synthesized from verified API docs
// https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UISettings.html
// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.GetSystemMetrics.html

use ::windows::UI::ViewManagement::{UISettings, UIColorType};
use ::windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXBORDER, SM_CXVSCROLL};

/// Convert a windows::UI::Color to our Rgba type.
fn win_color_to_rgba(c: ::windows::UI::Color) -> crate::Rgba {
    crate::Rgba::rgba(c.R, c.G, c.B, c.A)
}

/// Detect dark mode from the system foreground color luminance.
fn is_dark_mode(fg: &crate::Rgba) -> bool {
    let luma = 0.299 * (fg.r as f32) + 0.587 * (fg.g as f32) + 0.114 * (fg.b as f32);
    luma > 128.0
}

/// Read system geometry metrics from GetSystemMetrics.
fn read_geometry() -> crate::ThemeGeometry {
    // SAFETY: GetSystemMetrics is always safe to call with valid SM_* constants.
    unsafe {
        crate::ThemeGeometry {
            frame_width: Some(GetSystemMetrics(SM_CXBORDER) as f32),
            scroll_width: Some(GetSystemMetrics(SM_CXVSCROLL) as f32),
            ..Default::default()
        }
    }
}

/// Read the current Windows theme from UISettings and GetSystemMetrics.
pub fn from_windows() -> crate::Result<crate::NativeTheme> {
    let settings = UISettings::new().map_err(|e| {
        crate::Error::Unavailable(format!("UISettings unavailable: {e}"))
    })?;

    let accent = settings.GetColorValue(UIColorType::Accent)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::Platform(Box::new(e)))?;
    let fg = settings.GetColorValue(UIColorType::Foreground)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::Platform(Box::new(e)))?;
    let bg = settings.GetColorValue(UIColorType::Background)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::Platform(Box::new(e)))?;

    let dark = is_dark_mode(&fg);
    let geometry = read_geometry();

    let mut colors = crate::ThemeColors::default();
    colors.core.accent = Some(accent);
    colors.core.foreground = Some(fg);
    colors.core.background = Some(bg);

    let variant = crate::ThemeVariant {
        colors,
        geometry,
        fonts: Default::default(),
        spacing: Default::default(),
    };

    let theme = if dark {
        crate::NativeTheme {
            name: "Windows".to_string(),
            light: None,
            dark: Some(variant),
        }
    } else {
        crate::NativeTheme {
            name: "Windows".to_string(),
            light: Some(variant),
            dark: None,
        }
    };

    Ok(theme)
}
```

### UIColorType Mapping to ThemeColors Fields
```rust
// Recommended mapping of UIColorType variants to NativeTheme color roles
// Source: https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UIColorType.html

// UIColorType::Accent       -> colors.core.accent
// UIColorType::Background   -> colors.core.background
// UIColorType::Foreground   -> colors.core.foreground
// UIColorType::AccentDark1  -> (available for future use, e.g., hover states)
// UIColorType::AccentLight1 -> (available for future use, e.g., pressed states)
//
// Additionally, accent can be propagated to:
// colors.interactive.selection     (accent as selection highlight)
// colors.interactive.focus_ring    (accent as focus indicator)
// colors.primary.background        (accent as primary button color)
```

### GetSystemMetrics Mapping to ThemeGeometry Fields
```rust
// Recommended mapping of SM_* constants to ThemeGeometry fields
// Source: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics

// SM_CXBORDER (5)   -> geometry.frame_width  (window border width)
// SM_CXVSCROLL (2)  -> geometry.scroll_width  (vertical scrollbar width)
// SM_CXEDGE (45)    -> (3-D border width, could inform border styling)
//
// These are available but not directly mapped to current ThemeGeometry fields:
// SM_CYCAPTION (4)  -> title bar height (no field currently)
// SM_CXICON (11)    -> icon width (no field currently)
// SM_CYICON (12)    -> icon height (no field currently)
// SM_CXSMICON (49)  -> small icon width (no field currently)
```

### Testing Pattern (Testable Core Without Windows APIs)
```rust
// Following the GNOME pattern: separate testable build_theme from live API calls

/// Testable core: given raw color values and metrics, build a NativeTheme.
fn build_theme(
    accent: crate::Rgba,
    fg: crate::Rgba,
    bg: crate::Rgba,
    geometry: crate::ThemeGeometry,
) -> crate::NativeTheme {
    let dark = is_dark_mode(&fg);
    // ... build variant and theme as above
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_mode_detected_from_light_foreground() {
        let fg = crate::Rgba::rgb(255, 255, 255); // white foreground = dark mode
        assert!(is_dark_mode(&fg));
    }

    #[test]
    fn light_mode_detected_from_dark_foreground() {
        let fg = crate::Rgba::rgb(0, 0, 0); // black foreground = light mode
        assert!(!is_dark_mode(&fg));
    }

    #[test]
    fn build_theme_dark_populates_dark_variant_only() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),   // Windows blue accent
            crate::Rgba::rgb(255, 255, 255),  // white fg = dark mode
            crate::Rgba::rgb(0, 0, 0),        // black bg
            crate::ThemeGeometry::default(),
        );
        assert!(theme.dark.is_some());
        assert!(theme.light.is_none());
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| winapi crate for Win32 FFI | `windows` crate (official Microsoft) | 2021+ | winapi is unmaintained; windows is the official, auto-generated binding |
| windows-sys for all Windows APIs | `windows` for WinRT, `windows-sys` for Win32-only | 2023+ | WinRT requires `windows` crate; windows-sys only does C-style APIs |
| Registry reading for accent color | UISettings::GetColorValue | Windows 10+ | Registry is undocumented and doesn't handle auto-accent correctly |
| Separate feature crates (windows-*) | Unified `windows` crate with feature flags | 0.59+ | Single dependency with granular feature selection |

**Deprecated/outdated:**
- `winapi` crate: Unmaintained since 2021, replaced by official `windows` crate
- `winrt` crate: Deprecated, capabilities merged into `windows` crate
- Reading `HKCU\...\AccentColor` registry key: Fragile, undocumented, does not track "automatic accent color"

## Open Questions

1. **Module naming: `windows.rs` vs `win.rs`**
   - What we know: Using `mod windows` creates a name collision with the `windows` external crate. The collision is resolvable with `::windows` for the crate, but may confuse developers.
   - What's unclear: Whether the Rust 2024 edition resolves this cleanly or if it causes subtle issues.
   - Recommendation: Name the file `windows.rs` and use `::windows::` prefix consistently inside the module for external crate references. This matches the feature flag name ("windows") and is the most intuitive name. If compilation issues arise, rename to `win.rs`.

2. **Whether to also read accent shade variants (AccentDark1-3, AccentLight1-3)**
   - What we know: UISettings provides 6 additional accent shades beyond the primary accent. These could map to hover/pressed states or secondary colors.
   - What's unclear: Whether these additional shades provide enough value to justify the extra complexity.
   - Recommendation: Read primary `Accent`, `Background`, `Foreground` only for v1. The accent shades can be read later if needed. The current ThemeColors model does not have explicit hover/pressed variants to map them to.

3. **DPI-aware metrics via GetSystemMetricsForDpi**
   - What we know: GetSystemMetrics returns physical pixels. GetSystemMetricsForDpi is the DPI-aware version but requires Windows 10 1607+.
   - What's unclear: Whether physical pixels are acceptable for the ThemeGeometry fields.
   - Recommendation: Use GetSystemMetrics for now. Document that values are in physical pixels. A future enhancement can switch to GetSystemMetricsForDpi if DPI awareness becomes a requirement.

4. **Cross-compilation testing**
   - What we know: This project is developed on Linux. The `windows` module behind `cfg(feature = "windows")` won't compile on Linux. Feature gating ensures it doesn't block Linux builds.
   - What's unclear: Whether CI/CD setup needs a Windows runner to test this module.
   - Recommendation: Write unit tests with a `build_theme()` testable core (following the GNOME pattern) that can be tested on any platform. Integration tests calling actual Windows APIs require a Windows environment.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | Cargo.toml (existing) |
| Quick run command | `cargo test --features windows` |
| Full suite command | `cargo test --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-04 | from_windows() returns NativeTheme with accent, fg, bg, geometry | unit | `cargo test --features windows windows::tests -x` | No -- Wave 0 |
| PLAT-04 | Dark mode detection from foreground luminance | unit | `cargo test --features windows windows::tests::dark_mode -x` | No -- Wave 0 |
| PLAT-04 | Graceful degradation: UISettings unavailable returns Error::Unavailable | unit | `cargo test --features windows windows::tests::unavailable -x` | No -- Wave 0 |
| PLAT-04 | Only active variant (light or dark) populated | unit | `cargo test --features windows windows::tests::single_variant -x` | No -- Wave 0 |
| PLAT-04 | Feature flag isolation: "windows" feature only pulls minimal crate features | unit | `cargo test --features windows` (compile check) | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test --features windows` (if on Windows, else `cargo check --features windows --target x86_64-pc-windows-msvc` for cross-check)
- **Per wave merge:** `cargo test --all-features`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `src/windows.rs` -- the entire module (new file)
- [ ] Unit tests for `is_dark_mode()`, `win_color_to_rgba()`, `build_theme()` testable core
- [ ] Integration test for `from_windows()` (requires Windows environment)

## Sources

### Primary (HIGH confidence)
- [windows crate docs - UISettings](https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UISettings.html) - UISettings methods, constructor, GetColorValue API
- [windows crate docs - UIColorType](https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UIColorType.html) - All 10 UIColorType variants (Accent, AccentDark1-3, AccentLight1-3, Background, Foreground, Complement)
- [windows crate docs - GetSystemMetrics](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.GetSystemMetrics.html) - Function signature, unsafe marker
- [windows crate docs - Color struct](https://microsoft.github.io/windows-docs-rs/doc/windows/UI/struct.Color.html) - Color {A: u8, R: u8, G: u8, B: u8} field layout
- [Microsoft Learn - GetSystemMetrics](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics) - Complete SM_* constant list with descriptions and values
- [Microsoft Learn - UISettings.GetColorValue](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.getcolorvalue) - API requirements (Windows 10+), Complement exception warning
- [Microsoft Learn - windows vs windows-sys](https://microsoft.github.io/windows-rs/book/rust-getting-started/windows-or-windows-sys.html) - windows-sys does NOT support WinRT/COM
- [docs.rs - windows 0.62.2](https://docs.rs/crate/windows/latest/source/readme.md) - Version 0.62.2 is latest, recommended range >=0.59 <=0.62

### Secondary (MEDIUM confidence)
- [Tauri discussion #5305](https://github.com/orgs/tauri-apps/discussions/5305) - Working Rust code example using UISettings::new() + GetColorValue(UIColorType::Accent)
- [dark-light crate](https://github.com/frewsxcv/rust-dark-light) - Established pattern for dark/light detection on Windows using foreground luminance
- [Microsoft Learn - dark theme support](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-windows-themes) - Foreground luminance method for dark mode detection

### Tertiary (LOW confidence)
- None -- all findings verified against official documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - `windows` crate is the only option for WinRT+Win32, verified via official docs and Microsoft's own comparison page
- Architecture: HIGH - Pattern follows established KDE/GNOME reader patterns in the codebase; API signatures verified against official docs
- Pitfalls: HIGH - Complement exception documented by Microsoft, DPI awareness documented, module naming is a known Rust pattern issue

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (30 days -- windows crate is stable and well-established)
