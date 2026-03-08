# Phase 11: Platform Readers - Research

**Researched:** 2026-03-08
**Domain:** Platform-specific OS theme reading (macOS, Windows, Linux)
**Confidence:** HIGH

## Summary

Phase 11 adds a new macOS reader (PLAT-01 through PLAT-04), enhances the existing Windows reader with accent shades, system font, spacing, DPI-aware geometry, and capability checks (PLAT-05 through PLAT-09), and enhances the existing Linux readers with portal/KDE overlay merging, D-Bus backend detection for DE heuristic, GNOME font reading, and a kdeglobals fallback for non-KDE desktops (PLAT-10 through PLAT-13).

The macOS reader is the largest new work. It requires the `objc2-app-kit` crate (v0.3.2) for NSColor, NSAppearance, and NSFont bindings. The pattern is: set NSAppearance.current to light or dark, read semantic NSColor values, convert from Display P3 to sRGB via `colorUsingColorSpace(NSColorSpace::sRGBColorSpace())`, extract RGBA components, and populate both ThemeVariant light and dark. Windows enhancements use the existing `windows` crate but add new feature dependencies (`Foundation_Metadata` for ApiInformation, `Win32_UI_HiDpi` for GetSystemMetricsForDpi, `Win32_Graphics_Gdi` for LOGFONTW). Linux enhancements build on the existing `ashpd` portal and `configparser` KDE readers.

**Primary recommendation:** Implement platform readers as three independent plan waves (macOS, Windows, Linux) since they share no code paths and touch separate feature-gated modules.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-01 | macOS reader reads ~20 NSColor semantic colors with P3-to-sRGB conversion | objc2-app-kit NSColor semantic methods + colorUsingColorSpace conversion documented below |
| PLAT-02 | macOS reader resolves both light and dark variants via NSAppearance | NSAppearance.performAsCurrentDrawingAppearance with block2 feature documented below |
| PLAT-03 | macOS reader reads NSFont system and monospace fonts | NSFont::systemFontOfSize and monospacedSystemFontOfSize_weight documented below |
| PLAT-04 | macOS reader wired into from_system() dispatch | Existing from_system() pattern in lib.rs; add cfg(target_os = "macos") arm |
| PLAT-05 | Windows reader adds ApiInformation::IsMethodPresent capability checks | windows::Foundation::Metadata::ApiInformation documented below |
| PLAT-06 | Windows reader reads AccentDark1-3 and AccentLight1-3 accent shades | UIColorType enum values (2-8) documented below |
| PLAT-07 | Windows reader reads system font via SystemParametersInfo(SPI_GETNONCLIENTMETRICS) | NONCLIENTMETRICSW struct with lfMessageFont LOGFONTW documented below |
| PLAT-08 | Windows reader populates spacing from WinUI3 defaults and derives primary_foreground | WinUI3 spacing scale (4/8/12/16/24/32) documented below |
| PLAT-09 | Windows reader uses DPI-aware GetSystemMetricsForDpi for geometry | windows::Win32::UI::HiDpi::GetSystemMetricsForDpi documented below |
| PLAT-10 | Linux from_kde_with_portal() async overlay of portal accent on kdeglobals palette | ashpd Settings + NativeTheme::merge() pattern documented below |
| PLAT-11 | Linux D-Bus portal backend detection for DE heuristic | D-Bus ListNames + well-known backend bus names documented below |
| PLAT-12 | GNOME font reading from gsettings/dconf | std::process::Command gsettings approach documented below |
| PLAT-13 | from_linux() fallback: try kdeglobals if file exists on non-KDE desktops | File existence check + from_kde() fallback pattern documented below |
</phase_requirements>

## Standard Stack

### Core (macOS reader)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `objc2` | 0.6.x | ObjC runtime bindings | Official Rust-ObjC interop, maintained by madsmtm |
| `objc2-foundation` | 0.3.x | NSString, NSArray types | Required for objc2-app-kit |
| `objc2-app-kit` | 0.3.2 | NSColor, NSAppearance, NSFont | Only crate providing typed AppKit bindings for Rust |
| `block2` | 0.6-0.7 | ObjC block support | Required for performAsCurrentDrawingAppearance |

### Core (Windows enhancements)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `windows` | >=0.59, <=0.62 | WinRT + Win32 APIs | Already used; add new feature flags |

### Core (Linux enhancements)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `ashpd` | 0.13.4 | XDG portal D-Bus access | Already used for GNOME portal reader |
| `configparser` | 3.1.0 | kdeglobals INI parsing | Already used for KDE reader |

### No New Dependencies
The GNOME font reading (PLAT-12) and D-Bus backend detection (PLAT-11) should NOT introduce new crate dependencies. Instead:
- Font reading: use `std::process::Command` to call `gsettings get org.gnome.desktop.interface font-name` and parse the output
- Backend detection: use `zbus` (already a transitive dependency via ashpd) or check for well-known D-Bus bus names

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| objc2-app-kit | Raw objc2 msg_send! | Type safety vs flexibility; objc2-app-kit is generated from Apple headers |
| gsettings subprocess | gio crate (gtk-rs) | gio pulls in glib/gobject C dependencies; subprocess is zero-dep |
| zbus for backend detection | dbus-rs crate | zbus is already a transitive dep via ashpd; dbus-rs requires libdbus-1 |

**Installation (Cargo.toml additions):**
```toml
# macOS feature + dependencies
[features]
macos = ["dep:objc2", "dep:objc2-foundation", "dep:objc2-app-kit", "dep:block2"]

[dependencies]
objc2 = { version = "0.6", optional = true }
objc2-foundation = { version = "0.3", optional = true, features = ["NSString", "NSArray"] }
objc2-app-kit = { version = "0.3", optional = true, features = [
    "NSColor", "NSColorSpace", "NSAppearance", "NSFont", "NSFontDescriptor",
    "objc2-core-foundation",  # needed for CGFloat, pointSize, systemFontSize
] }
block2 = { version = ">=0.6.1, <0.8.0", optional = true }

# Windows feature additions (extend existing)
[dependencies.windows]
features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_HiDpi",           # NEW: GetSystemMetricsForDpi, GetDpiForSystem
    "Win32_Graphics_Gdi",       # NEW: LOGFONTW for font extraction
    "Foundation_Metadata",      # NEW: ApiInformation
]
```

## Architecture Patterns

### Recommended Module Structure
```
native-theme/src/
  lib.rs            # from_system() dispatch, from_linux(), detect_linux_de()
  macos.rs          # NEW: from_macos() + build_theme() + color mapping
  windows.rs        # ENHANCED: accent shades, font, spacing, DPI, capability checks
  gnome/
    mod.rs          # ENHANCED: font reading, from_kde_with_portal overlay
  kde/
    mod.rs          # ENHANCED: public from_kde() for fallback reuse
    colors.rs       # (unchanged)
    fonts.rs        # (unchanged)
```

### Pattern 1: Appearance-Resolved Color Reading (macOS)
**What:** Read semantic NSColor values under a specific NSAppearance context
**When to use:** For PLAT-01 and PLAT-02 (macOS semantic colors in both light and dark)
**Example:**
```rust
// Source: Apple Developer Documentation + objc2-app-kit 0.3.2 API
use objc2_app_kit::{NSAppearance, NSColor, NSColorSpace};
use objc2_foundation::NSString;

fn read_colors_for_appearance(appearance_name: &str) -> ThemeColors {
    let name = NSString::from_str(appearance_name);
    let appearance = unsafe { NSAppearance::appearanceNamed(&name) };
    if let Some(appearance) = appearance {
        // performAsCurrentDrawingAppearance sets NSAppearance.current
        // for the duration of the block, allowing semantic colors to resolve
        appearance.performAsCurrentDrawingAppearance(&|| {
            read_semantic_colors()
        });
    }
    // ...
}

fn nscolor_to_rgba(color: &NSColor) -> Option<Rgba> {
    // Convert to sRGB color space (handles P3 -> sRGB)
    let srgb_space = unsafe { NSColorSpace::sRGBColorSpace() };
    let srgb_color = unsafe { color.colorUsingColorSpace(&srgb_space) }?;
    // Extract components
    let r = unsafe { srgb_color.redComponent() } as f32;
    let g = unsafe { srgb_color.greenComponent() } as f32;
    let b = unsafe { srgb_color.blueComponent() } as f32;
    let a = unsafe { srgb_color.alphaComponent() } as f32;
    Some(Rgba::from_f32(r, g, b, a))
}
```

### Pattern 2: Capability-Checked API Calls (Windows)
**What:** Check if a WinRT API exists before calling it, preventing crashes on older Windows
**When to use:** For PLAT-05 (capability checks on accent shades, which may not exist on older Win10)
**Example:**
```rust
// Source: Microsoft Learn UIColorType + ApiInformation docs
use windows::Foundation::Metadata::ApiInformation;
use windows::UI::ViewManagement::{UIColorType, UISettings};

fn read_accent_shades(settings: &UISettings) -> [Option<Rgba>; 6] {
    // AccentDark1-3 and AccentLight1-3 are available since Windows 10 10240
    // but may fail on some SKUs -- wrap each in a Result handler
    let variants = [
        UIColorType::AccentDark1,   // value 4
        UIColorType::AccentDark2,   // value 3
        UIColorType::AccentDark3,   // value 2
        UIColorType::AccentLight1,  // value 6
        UIColorType::AccentLight2,  // value 7
        UIColorType::AccentLight3,  // value 8
    ];
    variants.map(|ct| {
        settings.GetColorValue(ct).ok().map(win_color_to_rgba)
    })
}
```

### Pattern 3: Merge Overlay (Linux portal + KDE)
**What:** Read KDE kdeglobals as base, then overlay portal accent color
**When to use:** For PLAT-10 (from_kde_with_portal)
**Example:**
```rust
// Uses existing NativeTheme::merge() infrastructure
pub async fn from_kde_with_portal() -> crate::Result<NativeTheme> {
    let base = crate::kde::from_kde()?;  // sync KDE read
    // Attempt async portal read for accent overlay
    match portal_accent().await {
        Ok(accent_overlay) => {
            let mut result = base;
            result.merge(&accent_overlay);
            Ok(result)
        }
        Err(_) => Ok(base),  // portal unavailable, return KDE-only
    }
}
```

### Pattern 4: Testable Core with Build Function
**What:** Separate platform API calls from theme construction logic
**When to use:** ALL platform readers -- already established pattern in windows.rs and gnome/mod.rs
**Why:** Enables unit testing without OS APIs (critical for cross-platform CI)

### Anti-Patterns to Avoid
- **Reading colors without setting NSAppearance.current:** NSColor semantic colors resolve to the *current* appearance. Without explicitly setting it, you get whatever appearance the process happens to have, which is unpredictable in non-GUI contexts.
- **Calling WinRT APIs without capability checks:** AccentDark/Light shades exist on Windows 10 10240+ but calling GetColorValue can return E_INVALIDARG on some Windows versions or SKUs. Always handle errors gracefully.
- **Blocking on D-Bus in synchronous contexts:** The portal reader is async. from_kde_with_portal must be async too. Don't use block_on() inside library code.
- **Linking gio/glib for a single gsettings read:** The GNOME font read is a single key lookup. Using std::process::Command to call `gsettings` avoids pulling in the entire GIO/GLib C dependency chain.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| P3 to sRGB conversion | Manual matrix math | NSColor::colorUsingColorSpace(sRGBColorSpace) | Apple's implementation handles ICC profiles, gamut mapping, rendering intent |
| DPI-aware metrics | GetSystemMetrics + manual DPI multiply | GetSystemMetricsForDpi(metric, dpi) | Avoids rounding errors, handles per-monitor DPI correctly |
| D-Bus connection management | Raw socket + protocol handling | zbus (via ashpd) | D-Bus protocol is complex; zbus handles auth, marshaling, async |
| Qt font string parsing | New parser for macOS | Reuse nothing -- macOS uses NSFont API, not Qt strings | Platform-specific: KDE has Qt format, macOS has NSFont, Windows has LOGFONTW |
| ObjC memory management | Manual retain/release | objc2 Retained<T> | Rust ownership maps to ObjC ARC via Retained<T> |

**Key insight:** Each platform has its own native API for colors, fonts, and metrics. The value of this crate is normalizing them into the same ThemeVariant structure, not reimplementing what the OS already provides.

## Common Pitfalls

### Pitfall 1: NSColor Semantic Colors Are Appearance-Dependent
**What goes wrong:** Reading NSColor::controlAccentColor() without setting NSAppearance returns the color for the *process's* current appearance, not necessarily the one you want.
**Why it happens:** macOS resolves dynamic colors lazily against NSAppearance.current.
**How to avoid:** Use `NSAppearance::performAsCurrentDrawingAppearance()` (macOS 11+) to scope color resolution. For macOS 10.14-10.15, manually set `NSAppearance::setCurrentAppearance()` then restore.
**Warning signs:** Light and dark variants returning identical color values.

### Pitfall 2: NSColor colorUsingColorSpace Returns Optional
**What goes wrong:** Some NSColor types (pattern colors, catalog colors without a backing representation) cannot be converted to sRGB. `colorUsingColorSpace()` returns `nil`.
**Why it happens:** Not all NSColor subclasses support color space conversion.
**How to avoid:** Always handle the `None` case. For semantic colors this is rare but possible. Map to `Option<Rgba>`.
**Warning signs:** Panics on unwrap() when running on macOS with custom system themes.

### Pitfall 3: Windows UIColorType::Complement is Not Supported
**What goes wrong:** Requesting `UIColorType::Complement` (value 9) throws an exception.
**Why it happens:** Microsoft docs say "Not supported. Do not use."
**How to avoid:** Only use values 0-8 (Background, Foreground, AccentDark3-1, Accent, AccentLight1-3).
**Warning signs:** HRESULT errors from GetColorValue.

### Pitfall 4: GetSystemMetricsForDpi Requires Windows 10 1607+
**What goes wrong:** Calling GetSystemMetricsForDpi on Windows 10 pre-Anniversary Update (build < 14393) causes a linker error or crash.
**Why it happens:** The function was added in Windows 10 1607.
**How to avoid:** Use ApiInformation::IsApiContractPresentByMajor or GetProcAddress for runtime detection. Alternatively, fall back to GetSystemMetrics if GetSystemMetricsForDpi fails.
**Warning signs:** Application crashes on older Windows 10 builds.

### Pitfall 5: gsettings Command May Not Exist
**What goes wrong:** Calling `gsettings` via subprocess fails on non-GNOME Linux or minimal installations.
**Why it happens:** gsettings is part of glib and may not be installed.
**How to avoid:** Check the Command result for Err or non-zero exit code. Return `ThemeFonts::default()` on failure.
**Warning signs:** Err(NotFound) from std::process::Command.

### Pitfall 6: kdeglobals May Exist on Non-KDE Desktops
**What goes wrong:** Reading kdeglobals on a GNOME system returns stale KDE config from a previous desktop installation.
**Why it happens:** KDE config files persist in ~/.config even when the user switches DEs.
**How to avoid:** For the from_linux() fallback (PLAT-13), only use kdeglobals as a *fallback* when the portal is unavailable. Check file freshness is not a requirement -- the file's existence is sufficient as a heuristic.
**Warning signs:** Non-KDE desktops returning KDE color schemes that don't match the active theme.

## Code Examples

### macOS: Reading Semantic Colors for Both Appearances
```rust
// Source: Apple Developer Documentation NSColor, NSAppearance
use objc2_app_kit::{NSAppearance, NSColor, NSColorSpace, NSFont};
use objc2_foundation::NSString;

const LIGHT_APPEARANCE: &str = "NSAppearanceNameAqua";
const DARK_APPEARANCE: &str = "NSAppearanceNameDarkAqua";

fn read_semantic_colors() -> ThemeColors {
    let srgb = unsafe { NSColorSpace::sRGBColorSpace() };

    let mut colors = ThemeColors::default();

    // Each NSColor class method returns a dynamic color that resolves
    // against NSAppearance.current
    colors.accent = nscolor_to_rgba(unsafe { &NSColor::controlAccentColor() }, &srgb);
    colors.background = nscolor_to_rgba(unsafe { &NSColor::windowBackgroundColor() }, &srgb);
    colors.foreground = nscolor_to_rgba(unsafe { &NSColor::labelColor() }, &srgb);
    colors.surface = nscolor_to_rgba(unsafe { &NSColor::controlBackgroundColor() }, &srgb);
    colors.muted = nscolor_to_rgba(unsafe { &NSColor::secondaryLabelColor() }, &srgb);
    colors.border = nscolor_to_rgba(unsafe { &NSColor::separatorColor() }, &srgb);
    colors.shadow = nscolor_to_rgba(unsafe { &NSColor::shadowColor() }, &srgb);
    colors.selection = nscolor_to_rgba(unsafe { &NSColor::selectedContentBackgroundColor() }, &srgb);
    colors.selection_foreground = nscolor_to_rgba(unsafe { &NSColor::selectedTextColor() }, &srgb);
    colors.link = nscolor_to_rgba(unsafe { &NSColor::linkColor() }, &srgb);
    colors.focus_ring = nscolor_to_rgba(unsafe { &NSColor::keyboardFocusIndicatorColor() }, &srgb);
    // ... etc for ~20 colors

    colors
}

fn nscolor_to_rgba(color: &NSColor, srgb: &NSColorSpace) -> Option<Rgba> {
    let srgb_color = unsafe { color.colorUsingColorSpace(srgb) }?;
    let r = unsafe { srgb_color.redComponent() } as f32;
    let g = unsafe { srgb_color.greenComponent() } as f32;
    let b = unsafe { srgb_color.blueComponent() } as f32;
    let a = unsafe { srgb_color.alphaComponent() } as f32;
    Some(Rgba::from_f32(r, g, b, a))
}
```

### macOS: Resolving Both Light and Dark Variants
```rust
// Source: Apple Developer Documentation NSAppearance
fn build_macos_theme() -> NativeTheme {
    let light_name = unsafe { NSString::from_str(LIGHT_APPEARANCE) };
    let dark_name = unsafe { NSString::from_str(DARK_APPEARANCE) };

    let light_appearance = unsafe { NSAppearance::appearanceNamed(&light_name) };
    let dark_appearance = unsafe { NSAppearance::appearanceNamed(&dark_name) };

    let light_colors = if let Some(app) = &light_appearance {
        let mut colors = ThemeColors::default();
        app.performAsCurrentDrawingAppearance(|| {
            colors = read_semantic_colors();
        });
        colors
    } else {
        ThemeColors::default()
    };

    let dark_colors = if let Some(app) = &dark_appearance {
        let mut colors = ThemeColors::default();
        app.performAsCurrentDrawingAppearance(|| {
            colors = read_semantic_colors();
        });
        colors
    } else {
        ThemeColors::default()
    };

    let fonts = read_fonts();

    NativeTheme {
        name: "macOS".to_string(),
        light: Some(ThemeVariant {
            colors: light_colors,
            fonts: fonts.clone(),
            geometry: Default::default(),
            spacing: Default::default(),
        }),
        dark: Some(ThemeVariant {
            colors: dark_colors,
            fonts,
            geometry: Default::default(),
            spacing: Default::default(),
        }),
    }
}
```

### macOS: Reading System Fonts
```rust
// Source: objc2-app-kit 0.3.2 NSFont docs
use objc2_app_kit::{NSFont, NSFontWeight};

fn read_fonts() -> ThemeFonts {
    let system_size = unsafe { NSFont::systemFontSize() };
    let system_font = unsafe { NSFont::systemFontOfSize(system_size) };
    let mono_font = unsafe {
        NSFont::monospacedSystemFontOfSize_weight(system_size, NSFontWeight::Regular)
    };

    ThemeFonts {
        family: system_font.familyName().map(|n| n.to_string()),
        size: Some(system_font.pointSize() as f32),
        mono_family: mono_font.familyName().map(|n| n.to_string()),
        mono_size: Some(mono_font.pointSize() as f32),
    }
}
```

### Windows: Reading Accent Shades with Capability Check
```rust
// Source: Microsoft Learn UIColorType enum + windows crate docs
use windows::UI::ViewManagement::{UIColorType, UISettings};

fn read_all_colors(settings: &UISettings) -> (Rgba, Rgba, Rgba, [Option<Rgba>; 6]) {
    let accent = settings.GetColorValue(UIColorType::Accent)
        .map(win_color_to_rgba)
        .expect("Accent always available");
    let fg = settings.GetColorValue(UIColorType::Foreground)
        .map(win_color_to_rgba)
        .expect("Foreground always available");
    let bg = settings.GetColorValue(UIColorType::Background)
        .map(win_color_to_rgba)
        .expect("Background always available");

    // Accent shades -- handle errors gracefully per PLAT-05
    let shades = [
        UIColorType::AccentDark1,
        UIColorType::AccentDark2,
        UIColorType::AccentDark3,
        UIColorType::AccentLight1,
        UIColorType::AccentLight2,
        UIColorType::AccentLight3,
    ].map(|ct| settings.GetColorValue(ct).ok().map(win_color_to_rgba));

    (accent, fg, bg, shades)
}
```

### Windows: Reading System Font
```rust
// Source: Microsoft Learn NONCLIENTMETRICSW + SystemParametersInfoW
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, NONCLIENTMETRICSW, SPI_GETNONCLIENTMETRICS,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

fn read_system_font() -> ThemeFonts {
    let mut ncm = NONCLIENTMETRICSW::default();
    ncm.cbSize = std::mem::size_of::<NONCLIENTMETRICSW>() as u32;

    let success = unsafe {
        SystemParametersInfoW(
            SPI_GETNONCLIENTMETRICS,
            ncm.cbSize,
            Some(&mut ncm as *mut _ as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };

    if success.is_ok() {
        let lf = &ncm.lfMessageFont;
        // LOGFONTW.lfFaceName is [u16; 32] -- null-terminated UTF-16
        let face_end = lf.lfFaceName.iter().position(|&c| c == 0).unwrap_or(32);
        let family = String::from_utf16_lossy(&lf.lfFaceName[..face_end]);
        // lfHeight is negative for character height in logical units
        let size = if lf.lfHeight < 0 { -lf.lfHeight } else { lf.lfHeight };

        ThemeFonts {
            family: Some(family),
            size: Some(size as f32),
            mono_family: None,  // Windows doesn't have a system mono font setting
            mono_size: None,
        }
    } else {
        ThemeFonts::default()
    }
}
```

### Windows: DPI-Aware Geometry
```rust
// Source: Microsoft Learn GetSystemMetricsForDpi + GetDpiForSystem
use windows::Win32::UI::HiDpi::{GetSystemMetricsForDpi, GetDpiForSystem};
use windows::Win32::UI::WindowsAndMessaging::{SM_CXBORDER, SM_CXVSCROLL};

fn read_geometry_dpi_aware() -> ThemeGeometry {
    let dpi = unsafe { GetDpiForSystem() };

    ThemeGeometry {
        frame_width: Some(unsafe { GetSystemMetricsForDpi(SM_CXBORDER, dpi) } as f32),
        scroll_width: Some(unsafe { GetSystemMetricsForDpi(SM_CXVSCROLL, dpi) } as f32),
        ..Default::default()
    }
}
```

### Linux: D-Bus Portal Backend Detection
```rust
// Detect which portal backend is running by checking D-Bus bus names
// Well-known names: org.freedesktop.impl.portal.desktop.kde
//                   org.freedesktop.impl.portal.desktop.gnome
async fn detect_portal_backend() -> Option<LinuxDesktop> {
    // ashpd uses zbus internally; we can access the connection
    let connection = zbus::Connection::session().await.ok()?;
    let dbus_proxy = zbus::fdo::DBusProxy::new(&connection).await.ok()?;
    let names = dbus_proxy.list_activatable_names().await.ok()?;

    for name in &names {
        if name.as_str().contains("portal.desktop.kde") {
            return Some(LinuxDesktop::Kde);
        }
        if name.as_str().contains("portal.desktop.gnome") {
            return Some(LinuxDesktop::Gnome);
        }
    }
    None
}
```

### Linux: GNOME Font Reading via gsettings
```rust
fn read_gnome_fonts() -> ThemeFonts {
    let mut fonts = ThemeFonts::default();

    // Read system UI font
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "font-name"])
        .output()
    {
        if output.status.success() {
            let raw = String::from_utf8_lossy(&output.stdout);
            // gsettings outputs: 'Cantarell 11' (with quotes)
            if let Some((family, size)) = parse_gnome_font_string(raw.trim()) {
                fonts.family = Some(family);
                fonts.size = Some(size);
            }
        }
    }

    // Read monospace font
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "monospace-font-name"])
        .output()
    {
        if output.status.success() {
            let raw = String::from_utf8_lossy(&output.stdout);
            if let Some((family, size)) = parse_gnome_font_string(raw.trim()) {
                fonts.mono_family = Some(family);
                fonts.mono_size = Some(size);
            }
        }
    }

    fonts
}

/// Parse GNOME font string format: "'Family Name Size'" or "Family Name Size"
fn parse_gnome_font_string(s: &str) -> Option<(String, f32)> {
    let s = s.trim_matches('\'');
    let last_space = s.rfind(' ')?;
    let family = s[..last_space].to_string();
    let size: f32 = s[last_space + 1..].parse().ok()?;
    if family.is_empty() || size <= 0.0 {
        return None;
    }
    Some((family, size))
}
```

### Linux: from_kde_with_portal() Overlay
```rust
// Reads KDE kdeglobals, then overlays portal accent if available
pub async fn from_kde_with_portal() -> crate::Result<NativeTheme> {
    let mut base = crate::kde::from_kde()?;

    // Try to get portal accent overlay
    if let Ok(settings) = ashpd::desktop::settings::Settings::new().await {
        if let Ok(color) = settings.accent_color().await {
            if let Some(rgba) = portal_color_to_rgba(&color) {
                // Build an overlay theme with just the accent
                let mut overlay_colors = ThemeColors::default();
                overlay_colors.accent = Some(rgba);
                overlay_colors.selection = Some(rgba);
                overlay_colors.focus_ring = Some(rgba);
                overlay_colors.primary_background = Some(rgba);

                // Apply to whichever variant exists
                let overlay_variant = ThemeVariant {
                    colors: overlay_colors,
                    ..Default::default()
                };
                let overlay = NativeTheme {
                    name: String::new(),
                    light: base.light.as_ref().map(|_| overlay_variant.clone()),
                    dark: base.dark.as_ref().map(|_| overlay_variant),
                };
                base.merge(&overlay);
            }
        }
    }

    Ok(base)
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| NSAppearance.setCurrentAppearance() | performAsCurrentDrawingAppearance() | macOS 11 (2020) | Thread-safe appearance scoping |
| GetSystemMetrics | GetSystemMetricsForDpi | Windows 10 1607 (2016) | Correct DPI-aware metrics |
| XDG_CURRENT_DESKTOP only | portals.conf + D-Bus backend names | xdg-desktop-portal 1.15+ | More accurate DE detection |
| objc (0.x) crate | objc2 (0.6) + framework crates | 2023-2024 | Type-safe ObjC bindings |

**Deprecated/outdated:**
- `NSAppearance::setCurrentAppearance()` / `NSAppearance::currentAppearance()`: Deprecated by Apple; use `performAsCurrentDrawingAppearance` instead
- `UIColorType::Complement` (value 9): Microsoft docs say "Not supported. Do not use"
- XDG portal UseIn key in .portal files: Deprecated in favor of portals.conf

## NSColor-to-ThemeColors Mapping (macOS)

This is the recommended mapping of ~20 NSColor semantic methods to the 36 ThemeColors fields:

| ThemeColors field | NSColor method | Notes |
|---|---|---|
| accent | controlAccentColor | User's system accent |
| background | windowBackgroundColor | Main window bg |
| foreground | labelColor | Primary text |
| surface | controlBackgroundColor | Content area bg |
| border | separatorColor | Divider lines |
| muted | secondaryLabelColor | Secondary text |
| shadow | shadowColor | Drop shadow color |
| primary_background | controlAccentColor | Same as accent |
| primary_foreground | alternateSelectedControlTextColor | Text on accent bg |
| secondary_background | controlColor | Standard button/control bg |
| secondary_foreground | controlTextColor | Text on controls |
| danger | systemRedColor | Error/destructive |
| danger_foreground | labelColor | Text on danger |
| warning | systemOrangeColor | Warning state |
| warning_foreground | labelColor | Text on warning |
| success | systemGreenColor | Success state |
| success_foreground | labelColor | Text on success |
| info | systemBlueColor | Informational |
| info_foreground | labelColor | Text on info |
| selection | selectedContentBackgroundColor | Selection highlight |
| selection_foreground | selectedTextColor | Text in selection |
| link | linkColor | Hyperlinks |
| focus_ring | keyboardFocusIndicatorColor | Focus indicator |
| sidebar | underPageBackgroundColor | Sidebar/source list bg |
| sidebar_foreground | labelColor | Sidebar text |
| tooltip | windowBackgroundColor | Tooltip bg (no dedicated API) |
| tooltip_foreground | labelColor | Tooltip text |
| popover | windowBackgroundColor | Popover bg |
| popover_foreground | labelColor | Popover text |
| button | controlColor | Button bg |
| button_foreground | controlTextColor | Button text |
| input | textBackgroundColor | Text field bg |
| input_foreground | textColor | Text field text |
| disabled | disabledControlTextColor | Disabled state |
| separator | separatorColor | Dividers |
| alternate_row | controlBackgroundColor | Alternating row bg |

## WinUI3 Spacing Scale (Windows)

Based on Microsoft Fluent Design spacing guidelines, the standard spacing scale in effective pixels (epx):

| ThemeSpacing field | Value (epx) | WinUI3 Usage |
|---|---|---|
| xxs | 2.0 | Hairline separators |
| xs | 4.0 | Compact control padding |
| s | 8.0 | Between buttons, control-to-flyout |
| m | 12.0 | Control-to-label, between content areas |
| l | 16.0 | Surface-to-edge text margin |
| xl | 24.0 | Section spacing |
| xxl | 32.0 | Page margins |

## Windows Accent Shades Mapping

| UIColorType | Value | Maps to ThemeColors | Notes |
|---|---|---|---|
| Accent | 5 | accent, selection, focus_ring | Main accent color |
| AccentDark1 | 4 | primary_background | Hover state on light bg |
| AccentDark2 | 3 | (informational) | Pressed state on light bg |
| AccentDark3 | 2 | (informational) | Not directly mapped |
| AccentLight1 | 6 | primary_background (dark) | Hover state on dark bg |
| AccentLight2 | 7 | (informational) | Pressed state on dark bg |
| AccentLight3 | 8 | (informational) | Not directly mapped |

The dark accent shades are used in light mode (darker variants stand out against light backgrounds), and light shades are used in dark mode (lighter variants stand out against dark backgrounds). The primary_background mapping should use AccentDark1 for light variant and AccentLight1 for dark variant.

## Open Questions

1. **macOS minimum version target**
   - What we know: performAsCurrentDrawingAppearance requires macOS 11+. Dark mode requires 10.14+. controlAccentColor requires 10.14+.
   - What's unclear: Should we support macOS 10.14-10.15 with the older setCurrentAppearance approach, or target macOS 11+ only?
   - Recommendation: Target macOS 11+ (Big Sur). This is 6 years old and covers the vast majority of active Macs. Use performAsCurrentDrawingAppearance exclusively.

2. **objc2-app-kit feature flag granularity**
   - What we know: objc2-app-kit requires enabling individual feature flags for each AppKit class used (NSColor, NSFont, NSAppearance, NSColorSpace, etc.)
   - What's unclear: The exact minimal set of features needed. The code examples above list the likely set but may need adjustment during implementation.
   - Recommendation: Start with the features listed in Standard Stack; add more during implementation if compilation errors indicate missing features.

3. **D-Bus backend detection accuracy**
   - What we know: Well-known bus names like `org.freedesktop.impl.portal.desktop.kde` can be checked. ListActivatableNames works without activating services.
   - What's unclear: How reliable this is on non-standard setups (Flatpak, Snap, custom portal configs).
   - Recommendation: Use D-Bus detection as a *supplement* to XDG_CURRENT_DESKTOP, not a replacement. The heuristic should be: (1) check XDG_CURRENT_DESKTOP, (2) if Unknown, check D-Bus portal backends, (3) if still Unknown, try kdeglobals file fallback.

4. **Windows font size: logical units vs points**
   - What we know: LOGFONTW.lfHeight is in logical units (negative = character height, positive = cell height). ThemeFonts.size is in points.
   - What's unclear: Whether direct negative-to-positive conversion is sufficient or if we need to account for DPI (lfHeight = -MulDiv(PointSize, DPI, 72)).
   - Recommendation: Convert lfHeight to points using: `points = abs(lfHeight) * 72 / dpi`. Use GetDpiForSystem() for the DPI value.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | native-theme/Cargo.toml (features) |
| Quick run command | `cargo test -p native-theme` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-01 | macOS ~20 semantic colors with P3-to-sRGB | unit (build_theme mock) | `cargo test -p native-theme --features macos macos::tests` | Wave 0 |
| PLAT-02 | Both light/dark variants resolved | unit (build_theme mock) | `cargo test -p native-theme --features macos macos::tests::both_variants` | Wave 0 |
| PLAT-03 | NSFont system + mono fonts | unit (build_theme mock) | `cargo test -p native-theme --features macos macos::tests::fonts` | Wave 0 |
| PLAT-04 | from_system() dispatches to from_macos() | unit (cfg test) | `cargo test -p native-theme dispatch` | Exists (lib.rs) |
| PLAT-05 | ApiInformation capability checks | unit (build_theme with shades) | `cargo test -p native-theme --features windows windows::tests::capability` | Wave 0 |
| PLAT-06 | AccentDark1-3 and AccentLight1-3 | unit (build_theme mock) | `cargo test -p native-theme --features windows windows::tests::accent_shades` | Wave 0 |
| PLAT-07 | System font from NONCLIENTMETRICS | unit (font parsing) | `cargo test -p native-theme --features windows windows::tests::font` | Wave 0 |
| PLAT-08 | WinUI3 spacing defaults | unit (verify constants) | `cargo test -p native-theme --features windows windows::tests::spacing` | Wave 0 |
| PLAT-09 | DPI-aware geometry | unit (build_theme with DPI values) | `cargo test -p native-theme --features windows windows::tests::dpi_geometry` | Wave 0 |
| PLAT-10 | KDE + portal overlay | unit (merge test) | `cargo test -p native-theme --features kde,portal kde_portal_overlay` | Wave 0 |
| PLAT-11 | D-Bus backend detection | unit (mock names) | `cargo test -p native-theme --features portal gnome::tests::backend_detect` | Wave 0 |
| PLAT-12 | GNOME font from gsettings | unit (parse_gnome_font_string) | `cargo test -p native-theme --features portal gnome::tests::font_parse` | Wave 0 |
| PLAT-13 | from_linux() kdeglobals fallback | unit (with env var) | `cargo test -p native-theme --features kde dispatch::tests::fallback` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before /gsd:verify-work

### Wave 0 Gaps
- [ ] `native-theme/src/macos.rs` -- new file, needs build_theme tests with mock color/font data
- [ ] Update `native-theme/src/gnome/mod.rs` tests for font parsing and portal overlay
- [ ] Update `native-theme/src/windows.rs` tests for accent shades, font, spacing, DPI
- [ ] Update `native-theme/src/lib.rs` tests for from_linux() fallback with kdeglobals

## Sources

### Primary (HIGH confidence)
- [objc2-app-kit 0.3.2 docs](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/) - NSColor, NSAppearance, NSFont APIs verified
- [Apple NSColor docs](https://developer.apple.com/documentation/appkit/nscolor) - Semantic color method list
- [Apple NSAppearance docs](https://developer.apple.com/documentation/appkit/nsappearance) - performAsCurrentDrawingAppearance
- [Apple NSColorSpace docs](https://developer.apple.com/documentation/appkit/nscolorspace/1412071-srgbcolorspace) - sRGBColorSpace method
- [Microsoft UIColorType enum](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype?view=winrt-26100) - All 10 enum values with numeric IDs
- [Microsoft NONCLIENTMETRICSW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw) - System font structure
- [Microsoft GetSystemMetricsForDpi](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/HiDpi/fn.GetSystemMetricsForDpi.html) - DPI-aware metrics
- [Microsoft ApiInformation](https://microsoft.github.io/windows-docs-rs/doc/windows/Foundation/Metadata/struct.ApiInformation.html) - Runtime capability checks
- [XDG Portal Settings spec](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html) - Settings interface v2
- [ashpd Settings source](https://bilelmoussaoui.github.io/ashpd/src/ashpd/desktop/settings.rs.html) - read_all, version(), accent_color methods

### Secondary (MEDIUM confidence)
- [WinUI3 Spacing docs](https://learn.microsoft.com/en-us/windows/apps/design/style/spacing) - 4/8/12/16/24/32/48 epx scale
- [XDG Desktop Portal ArchWiki](https://wiki.archlinux.org/title/XDG_Desktop_Portal) - Backend detection, portals.conf
- [NSAppearance resolution blog](https://christiantietze.de/posts/2021/10/nscolor-performAsCurrentDrawingAppearance-resolve-current-appearance/) - Practical P3-to-sRGB pattern

### Tertiary (LOW confidence)
- GNOME font gsettings key names (org.gnome.desktop.interface font-name / monospace-font-name) -- verified against gsettings schema documentation but not tested in code

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries verified against docs.rs and official docs
- Architecture: HIGH - Follows established patterns already in codebase (build_theme, testable core)
- macOS color mapping: MEDIUM - NSColor method names verified, but exact runtime behavior needs testing on macOS
- Windows enhancements: HIGH - All Win32/WinRT APIs verified against Microsoft docs
- Linux enhancements: HIGH - ashpd API verified, gsettings approach is well-known pattern
- D-Bus backend detection: MEDIUM - Bus names verified from error reports/wiki but not from spec

**Research date:** 2026-03-08
**Valid until:** 2026-04-08 (30 days -- stable APIs, no fast-moving dependencies)
