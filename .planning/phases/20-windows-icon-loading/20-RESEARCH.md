# Phase 20: Windows Icon Loading - Research

**Researched:** 2026-03-09
**Domain:** Win32 Shell API (SHGetStockIconInfo), GDI (GetGlyphOutlineW, GetDIBits), Segoe Fluent Icons font, windows-rs crate
**Confidence:** HIGH

## Summary

Phase 20 implements Windows icon loading through two distinct pipelines that mirror the existing `segoe_name()` mapping in `icons.rs`. The mapping already divides 40 of 42 IconRoles into two categories: 18 roles use `SIID_*` prefixed names (stock icons via `SHGetStockIconInfo`) and 22 roles use Segoe Fluent Icons font glyph names (e.g., `"ChromeClose"`, `"Save"`, `"Copy"`). Two roles return `None` (DialogSuccess, StatusLoading) and fall back to bundled Material SVGs.

**Pipeline 1 -- Stock icons (PLAT-02):** Call `SHGetStockIconInfo` with `SHGSI_ICON | SHGSI_LARGEICON` to get an HICON, then extract BGRA pixel data using `GetIconInfo` + `GetDIBits`, swap B/R channels to produce RGBA, and return `IconData::Rgba`. The `SHSTOCKICONID` enum has constants for all 18 stock icon roles (SIID_WARNING=78, SIID_ERROR=80, SIID_INFO=79, etc.). One exception: `DialogQuestion` maps to `IDI_QUESTION` which uses `LoadIconW(None, IDI_QUESTION)` instead of `SHGetStockIconInfo`, since there is no `SIID_QUESTION` constant.

**Pipeline 2 -- Font glyphs (PLAT-03):** Load the Segoe Fluent Icons font (or Segoe MDL2 Assets on Windows 10 as fallback) via `CreateFontW`, render the Unicode glyph using `GetGlyphOutlineW` with `GGO_GRAY8_BITMAP` format which produces a 65-level grayscale alpha mask (values 0-64). Convert this to RGBA by treating the grayscale value as alpha (scaled from 0-64 to 0-255) with white RGB channels (255,255,255) for a standard monochrome icon appearance. The 22 glyph names map to known Unicode codepoints in the Private Use Area (e.g., ChromeClose=E8BB, Save=E74E, Copy=E8C8). Both Segoe Fluent Icons (Win11) and Segoe MDL2 Assets (Win10) share these same codepoints.

The critical alpha/color handling: stock icons from `SHGetStockIconInfo` come as BGRA with premultiplied alpha from Windows bitmaps; font glyphs from `GetGlyphOutlineW` come as a grayscale alpha mask. Both must be converted to RGBA with straight alpha.

**Primary recommendation:** Create a `winicons.rs` module (cfg-gated to `target_os = "windows"` + `system-icons` feature) with two internal functions: `load_stock_icon()` for SIID_ roles and `load_glyph_icon()` for font glyph roles. A router function inspects the `segoe_name()` prefix to dispatch to the correct pipeline. Add `Win32_UI_Shell` feature to the existing `windows` crate dependency.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-02 | Windows stock icon loading via `SHGetStockIconInfo` -> RGBA pixels (feature "system-icons") | `SHGetStockIconInfo` with `SHGSI_ICON` returns HICON; `GetIconInfo` extracts HBITMAP (hbmColor); `GetDIBits` with 32-bit BITMAPINFOHEADER dumps BGRA pixels; swap B/R to get RGBA; `DestroyIcon` cleanup. Covers 18 of 42 roles (SIID_ prefixed names). `DialogQuestion` uses `LoadIconW(None, IDI_QUESTION)` as special case. |
| PLAT-03 | Windows Segoe Fluent Icons font glyph rendering for action/navigation/window roles (feature "system-icons") | `CreateFontW("Segoe Fluent Icons")` with fallback to `"Segoe MDL2 Assets"`; `GetGlyphOutlineW(codepoint, GGO_GRAY8_BITMAP)` returns 65-level grayscale alpha mask; scale 0-64 -> 0-255 as alpha with white (255,255,255) RGB; covers 22 of 42 roles. Codepoints shared between both fonts. Bundled SVG fallback when neither font is present. |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| windows | >=0.59, <=0.62 | Win32 API bindings (SHGetStockIconInfo, GetDIBits, GetGlyphOutlineW, CreateFontW) | Already in project for Windows theme reader |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| (std) | stdlib | Vec<u8> pixel buffer management, mem::size_of for struct sizes | Always |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| GetGlyphOutlineW (GDI) | DirectWrite + D2D DrawGlyphRun | DirectWrite produces higher quality text but requires COM initialization, ID2D1Factory, IWICImagingFactory -- massive complexity for monochrome icon glyphs |
| GetGlyphOutlineW (GDI) | GDI ExtTextOut + CreateDIBSection | ExtTextOut destroys the alpha channel on 32-bit bitmaps -- unusable for transparent icon rendering |
| GetDIBits for HICON pixels | GetBitmapBits | GetBitmapBits is deprecated; GetDIBits with proper BITMAPINFOHEADER is the correct modern approach |

**Feature additions to Cargo.toml:**
```toml
# Add Win32_UI_Shell to the existing windows dependency features:
windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
    "UI_ViewManagement",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_HiDpi",
    "Win32_Graphics_Gdi",
    "Win32_UI_Shell",        # NEW: SHGetStockIconInfo, SHSTOCKICONID, SHGSI_FLAGS
    "Foundation_Metadata",
] }

# Update system-icons feature to include windows dep:
system-icons = ["dep:freedesktop-icons", "dep:objc2-core-graphics", "dep:windows", "material-icons"]
```

Note: The `windows` crate is NOT platform-gated in the dependency section (it already handles cross-platform via cfg internally), but it IS optional. Adding `"dep:windows"` to system-icons ensures the `windows` crate activates when `system-icons` is enabled. Since the `windows` crate itself only provides Windows APIs on Windows targets, this is safe.

Wait -- re-checking the existing Cargo.toml: `windows` is already in the top-level `[dependencies]` (not under `[target.'cfg(target_os = "windows")'.dependencies]`), and the `windows` feature already includes `"dep:windows"`. So `system-icons` does NOT need to add `"dep:windows"` -- the module just needs to be cfg-gated. The only change needed is adding `"Win32_UI_Shell"` to the windows crate features list.

## Architecture Patterns

### Recommended Module Structure
```
native-theme/src/
  winicons.rs             # New: Windows icon loader (cfg windows + system-icons)
  freedesktop.rs          # Existing: Linux freedesktop icon loader
  sficons.rs              # Existing: macOS SF Symbols icon loader
  lib.rs                  # Add: pub mod winicons (conditional) + re-export
  model/
    icons.rs              # Existing: segoe_name() maps 40/42 roles to SIID_/glyph names
    bundled.rs            # Existing: bundled_icon_svg() for fallback
```

### Pattern 1: Dispatch by Name Prefix
**What:** The `segoe_name()` function already returns names with two different conventions: `SIID_*` prefixed names for stock icons and plain names (e.g., `"ChromeClose"`) for font glyphs. The loader inspects this prefix to choose the pipeline.
**When to use:** In the main `load_windows_icon()` entry point.
**Example:**
```rust
pub fn load_windows_icon(role: IconRole) -> Option<IconData> {
    if let Some(name) = icon_name(IconSet::SegoeIcons, role) {
        if name.starts_with("SIID_") {
            // Stock icon pipeline
            if let Some(data) = load_stock_icon(name) {
                return Some(data);
            }
        } else if name == "IDI_QUESTION" {
            // Special case: LoadIconW for dialog question
            if let Some(data) = load_system_icon_idi(name) {
                return Some(data);
            }
        } else {
            // Font glyph pipeline
            if let Some(codepoint) = glyph_codepoint(name) {
                if let Some(data) = load_glyph_icon(codepoint) {
                    return Some(data);
                }
            }
        }
    }
    // Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role).map(|bytes| IconData::Svg(bytes.to_vec()))
}
```

### Pattern 2: HICON to RGBA Extraction
**What:** Extract RGBA pixel data from an HICON using GetIconInfo + GetDIBits.
**When to use:** After calling SHGetStockIconInfo or LoadIconW.
**Example:**
```rust
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

fn hicon_to_rgba(hicon: HICON) -> Option<IconData> {
    unsafe {
        let mut icon_info = ICONINFO::default();
        GetIconInfo(hicon, &mut icon_info).ok()?;

        // Get bitmap dimensions
        let mut bmp = BITMAP::default();
        GetObjectW(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        );

        let width = bmp.bmWidth as u32;
        let height = bmp.bmHeight as u32;

        // Set up BITMAPINFOHEADER for 32-bit top-down BGRA
        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = width as i32;
        bmi.bmiHeader.biHeight = -(height as i32); // negative = top-down
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB.0;

        let mut pixels = vec![0u8; (width * height * 4) as usize];

        let hdc = CreateCompatibleDC(None);
        GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );
        DeleteDC(hdc);

        // Swap BGRA -> RGBA
        for pixel in pixels.chunks_exact_mut(4) {
            pixel.swap(0, 2); // swap B and R
        }

        // Cleanup bitmaps
        DeleteObject(icon_info.hbmColor);
        DeleteObject(icon_info.hbmMask);

        Some(IconData::Rgba {
            width,
            height,
            data: pixels,
        })
    }
}
```

### Pattern 3: Font Glyph to RGBA via GetGlyphOutlineW
**What:** Render a single Unicode glyph from the Segoe Fluent Icons font as a grayscale alpha mask, then convert to RGBA.
**When to use:** For the 22 roles that map to Segoe Fluent Icons glyph names.
**Example:**
```rust
use windows::Win32::Graphics::Gdi::*;

/// Map glyph name to Unicode codepoint.
fn glyph_codepoint(name: &str) -> Option<u32> {
    Some(match name {
        "ChromeClose" => 0xE8BB,
        "ChromeMinimize" => 0xE921,
        "ChromeMaximize" => 0xE922,
        "ChromeRestore" => 0xE923,
        "Save" => 0xE74E,
        "Copy" => 0xE8C8,
        "Paste" => 0xE77F,
        "Cut" => 0xE8C6,
        "Undo" => 0xE7A7,
        "Redo" => 0xE7A6,
        "Edit" => 0xE70F,
        "Add" => 0xE710,
        "Remove" => 0xE738,
        "Refresh" => 0xE72C,
        "Back" => 0xE72B,
        "Forward" => 0xE72A,
        "Up" => 0xE74A,
        "Down" => 0xE74B,
        "Home" => 0xE80F,
        "GlobalNavigationButton" => 0xE700,
        "CheckMark" => 0xE73E,
        "Ringer" => 0xEA8F,
        _ => return None,
    })
}

fn load_glyph_icon(codepoint: u32, size: i32) -> Option<IconData> {
    unsafe {
        let hdc = CreateCompatibleDC(None);

        // Try Segoe Fluent Icons first (Win11), then Segoe MDL2 Assets (Win10)
        let font = try_create_font(hdc, "Segoe Fluent Icons", size)
            .or_else(|| try_create_font(hdc, "Segoe MDL2 Assets", size))?;

        let old_font = SelectObject(hdc, font);

        // Identity matrix (no transform)
        let mat2 = MAT2 {
            eM11: FIXED { fract: 0, value: 1 },
            eM12: FIXED { fract: 0, value: 0 },
            eM21: FIXED { fract: 0, value: 0 },
            eM22: FIXED { fract: 0, value: 1 },
        };

        let mut gm = GLYPHMETRICS::default();

        // First call: get buffer size
        let buf_size = GetGlyphOutlineW(
            hdc,
            codepoint,
            GGO_GRAY8_BITMAP,
            &mut gm,
            0,
            None,
            &mat2,
        );

        if buf_size == GDI_ERROR || buf_size == 0 {
            SelectObject(hdc, old_font);
            DeleteObject(font);
            DeleteDC(hdc);
            return None;
        }

        // Second call: get the grayscale bitmap
        let mut gray_buf = vec![0u8; buf_size as usize];
        GetGlyphOutlineW(
            hdc,
            codepoint,
            GGO_GRAY8_BITMAP,
            &mut gm,
            buf_size,
            Some(gray_buf.as_mut_ptr() as *mut _),
            &mat2,
        );

        SelectObject(hdc, old_font);
        DeleteObject(font);
        DeleteDC(hdc);

        // gm.gmBlackBoxX/Y are the actual glyph dimensions
        let glyph_w = gm.gmBlackBoxX;
        let glyph_h = gm.gmBlackBoxY;
        // GGO_GRAY8_BITMAP rows are DWORD-aligned
        let pitch = ((glyph_w + 3) & !3) as usize;

        // Convert grayscale (0-64) to RGBA (white with scaled alpha)
        let mut rgba = Vec::with_capacity((glyph_w * glyph_h * 4) as usize);
        for y in 0..glyph_h as usize {
            for x in 0..glyph_w as usize {
                let gray = gray_buf[y * pitch + x] as u32;
                let alpha = ((gray * 255) / 64).min(255) as u8;
                rgba.extend_from_slice(&[255, 255, 255, alpha]);
            }
        }

        Some(IconData::Rgba {
            width: glyph_w,
            height: glyph_h,
            data: rgba,
        })
    }
}
```

### Pattern 4: SIID_ Name to SHSTOCKICONID Dispatch
**What:** Map the SIID_ prefixed string names from `segoe_name()` to the actual `SHSTOCKICONID` enum values.
**When to use:** When dispatching stock icon requests.
**Example:**
```rust
use windows::Win32::UI::Shell::*;

fn siid_from_name(name: &str) -> Option<SHSTOCKICONID> {
    Some(match name {
        "SIID_WARNING" => SIID_WARNING,
        "SIID_ERROR" => SIID_ERROR,
        "SIID_INFO" => SIID_INFO,
        "SIID_SHIELD" => SIID_SHIELD,
        "SIID_DELETE" => SIID_DELETE,
        "SIID_FIND" => SIID_FIND,
        "SIID_SETTINGS" => SIID_SETTINGS,
        "SIID_PRINTER" => SIID_PRINTER,
        "SIID_DOCNOASSOC" => SIID_DOCNOASSOC,
        "SIID_FOLDER" => SIID_FOLDER,
        "SIID_FOLDEROPEN" => SIID_FOLDEROPEN,
        "SIID_RECYCLER" => SIID_RECYCLER,
        "SIID_RECYCLERFULL" => SIID_RECYCLERFULL,
        "SIID_USERS" => SIID_USERS,
        "SIID_HELP" => SIID_HELP,
        "SIID_LOCK" => SIID_LOCK,
        _ => return None,
    })
}

fn load_stock_icon(name: &str) -> Option<IconData> {
    let siid = siid_from_name(name)?;

    unsafe {
        let mut sii = SHSTOCKICONINFO::default();
        sii.cbSize = std::mem::size_of::<SHSTOCKICONINFO>() as u32;

        SHGetStockIconInfo(siid, SHGSI_ICON | SHGSI_LARGEICON, &mut sii).ok()?;

        let result = hicon_to_rgba(sii.hIcon);

        // Clean up the icon handle
        DestroyIcon(sii.hIcon).ok();

        result
    }
}
```

### Anti-Patterns to Avoid
- **Using ExtTextOut/DrawText for glyph rendering:** GDI text drawing functions destroy the alpha channel on 32-bit bitmaps, making them unusable for transparent icon rendering. Use `GetGlyphOutlineW` with `GGO_GRAY8_BITMAP` instead.
- **Assuming RGBA byte order from GetDIBits:** Windows GDI returns BGRA format. Failing to swap B/R channels produces icons with swapped red and blue channels.
- **Using GetBitmapBits instead of GetDIBits:** `GetBitmapBits` is deprecated since Win32. Use `GetDIBits` with a properly initialized `BITMAPINFOHEADER` (negative height for top-down, 32-bit, BI_RGB).
- **Forgetting to DestroyIcon after SHGetStockIconInfo:** The HICON returned must be freed with `DestroyIcon`. The `SHSTOCKICONINFO` docs explicitly state this responsibility.
- **Hardcoding "Segoe Fluent Icons" without fallback:** This font is only present on Windows 11 and newer Windows 10 installs. Always try "Segoe MDL2 Assets" as a secondary font, then bundled SVG as final fallback.
- **Skipping premultiplied alpha conversion for stock icons:** Stock icons from `SHGetStockIconInfo` may contain premultiplied alpha pixels. The same `unpremultiply_alpha()` pass from sficons.rs should be applied.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Stock icon loading | Manual resource extraction from system DLLs | `SHGetStockIconInfo` | The API handles icon theme changes, DPI scaling, and version-specific icon variants |
| HICON to pixels | Manual DC management with SelectObject chains | `GetIconInfo` + `GetDIBits` pattern | GetDIBits handles stride alignment, color depth conversion, and top-down/bottom-up orientation |
| Font glyph rendering | DirectWrite/D2D pipeline | `GetGlyphOutlineW(GGO_GRAY8_BITMAP)` | DirectWrite requires COM init, factory creation, multiple COM interfaces; GGO_GRAY8_BITMAP is a single function call |
| Segoe Fluent Icons codepoint table | Scrape from online docs at runtime | Compile-time lookup table | The codepoints are stable PUA values that don't change between font versions |

**Key insight:** The `GetGlyphOutlineW` with `GGO_GRAY8_BITMAP` approach produces clean alpha masks with 65 levels of grayscale, avoiding GDI's notorious alpha-channel destruction problem. It requires no COM initialization, no Direct2D factory, no render targets -- just a DC with a selected font.

## Common Pitfalls

### Pitfall 1: BGRA vs RGBA Byte Order
**What goes wrong:** Icons appear with swapped red and blue channels -- warning signs look blue, blue icons look red.
**Why it happens:** `GetDIBits` with 32-bit `BITMAPINFOHEADER` returns BGRA format (Windows native). The project's `IconData::Rgba` expects RGBA.
**How to avoid:** Swap bytes at indices 0 and 2 in each 4-byte pixel chunk after `GetDIBits`.
**Warning signs:** Colors look wrong in tests or visual inspection.

### Pitfall 2: Premultiplied Alpha in Stock Icons
**What goes wrong:** Semi-transparent edges of stock icons appear darker than expected.
**Why it happens:** Windows stock icon bitmaps use premultiplied alpha internally. After converting BGRA to RGBA, the premultiplied values persist.
**How to avoid:** Apply the same `unpremultiply_alpha()` pass used in sficons.rs after the BGRA->RGBA conversion.
**Warning signs:** Semi-transparent pixels around icon edges appear darker compared to the bundled SVG fallback.

### Pitfall 3: GetGlyphOutlineW Row Alignment
**What goes wrong:** Glyph rendering produces distorted or skewed icon images.
**Why it happens:** `GGO_GRAY8_BITMAP` rows are DWORD-aligned (padded to multiples of 4 bytes). If the glyph width is not a multiple of 4, each row has padding bytes that must be skipped.
**How to avoid:** Calculate pitch as `(glyph_width + 3) & !3` and use pitch (not width) as the row stride when reading the grayscale buffer.
**Warning signs:** Icons appear correct at certain sizes but distorted at others.

### Pitfall 4: Segoe Fluent Icons Not Present (Windows 10)
**What goes wrong:** `CreateFontW("Segoe Fluent Icons")` returns a handle to a fallback font (not the icon font), producing garbage glyphs.
**Why it happens:** Segoe Fluent Icons ships with Windows 11 but is not present on all Windows 10 installs. GDI silently falls back to a different font rather than failing.
**How to avoid:** After `CreateFontW`, verify the actual font by checking `GetTextFaceW` matches the requested name. If it does not match, try "Segoe MDL2 Assets" (always present on Windows 10). If neither is present, fall back to bundled SVGs.
**Warning signs:** Font glyph icons render as wrong characters or blank rectangles.

### Pitfall 5: Feature Flag Adds Windows Features on All Platforms
**What goes wrong:** Adding `Win32_UI_Shell` to the windows crate features causes the `windows` crate to be downloaded even on Linux/macOS when `system-icons` is enabled.
**Why it happens:** The `windows` crate is in the top-level `[dependencies]` section (not platform-gated), but it is optional and gated behind the `windows` feature. The `system-icons` feature does NOT currently include `"dep:windows"` -- the `windows` feature does. So enabling `system-icons` does NOT pull in the `windows` crate unless the `windows` feature is also enabled.
**How to avoid:** Keep the current Cargo.toml structure where `system-icons` does NOT include `"dep:windows"`. The winicons module is cfg-gated to `target_os = "windows"` AND `feature = "system-icons"`, but the actual `windows` crate dependency is activated by the existing `windows` feature flag. The new winicons module should require BOTH features: `#[cfg(all(target_os = "windows", feature = "system-icons", feature = "windows"))]`. Alternatively, simply use `#[cfg(all(target_os = "windows", feature = "system-icons"))]` and document that the `windows` Cargo feature must also be enabled.
**Warning signs:** Compilation errors about missing `windows::Win32::UI::Shell` types.

### Pitfall 6: Bottom-Up vs Top-Down Bitmap Orientation
**What goes wrong:** Icons appear vertically flipped.
**Why it happens:** `GetDIBits` with positive `biHeight` returns bottom-up rows. `IconData::Rgba` expects top-down (row-major from top-left).
**How to avoid:** Set `biHeight` to a negative value (e.g., `-(height as i32)`) in the `BITMAPINFOHEADER` to request top-down format.
**Warning signs:** Icons are upside down.

### Pitfall 7: DialogQuestion Has No SIID_ Constant
**What goes wrong:** Trying to look up `"IDI_QUESTION"` in the SIID_ dispatch table fails because it is not a stock icon ID.
**Why it happens:** The SHSTOCKICONID enumeration has no `SIID_QUESTION`. The `segoe_name()` mapping returns `"IDI_QUESTION"` for DialogQuestion, which requires the separate `LoadIconW(None, IDI_QUESTION)` API.
**How to avoid:** Handle `"IDI_QUESTION"` as a special case in the dispatch function, using `LoadIconW` instead of `SHGetStockIconInfo`.
**Warning signs:** DialogQuestion role returns None (falls back to bundled) even though it should load a system icon.

## Code Examples

Verified patterns from official sources and existing codebase:

### Segoe Fluent Icons Glyph Codepoint Table
```rust
// Source: https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font
// These codepoints are shared between Segoe Fluent Icons (Win11) and Segoe MDL2 Assets (Win10)
fn glyph_codepoint(name: &str) -> Option<u32> {
    Some(match name {
        // Window Controls
        "ChromeClose" => 0xE8BB,
        "ChromeMinimize" => 0xE921,
        "ChromeMaximize" => 0xE922,
        "ChromeRestore" => 0xE923,
        // Actions
        "Save" => 0xE74E,
        "Copy" => 0xE8C8,
        "Paste" => 0xE77F,
        "Cut" => 0xE8C6,
        "Undo" => 0xE7A7,
        "Redo" => 0xE7A6,
        "Edit" => 0xE70F,
        "Add" => 0xE710,
        "Remove" => 0xE738,
        "Refresh" => 0xE72C,
        // Navigation
        "Back" => 0xE72B,
        "Forward" => 0xE72A,
        "Up" => 0xE74A,
        "Down" => 0xE74B,
        "Home" => 0xE80F,
        "GlobalNavigationButton" => 0xE700,
        // Status
        "CheckMark" => 0xE73E,
        // System
        "Ringer" => 0xEA8F,
        _ => return None,
    })
}
```

### Font Availability Check
```rust
use windows::Win32::Graphics::Gdi::*;
use windows::core::PCWSTR;

/// Try to create a font and verify it was actually loaded (not substituted).
fn try_create_font(hdc: HDC, face_name: &str, size: i32) -> Option<HFONT> {
    let wide_name: Vec<u16> = face_name.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let font = CreateFontW(
            size,            // height
            0,               // width (auto)
            0, 0,            // escapement, orientation
            FW_NORMAL.0 as i32,
            0, 0, 0,         // italic, underline, strikeout
            DEFAULT_CHARSET.0 as u32,
            OUT_TT_PRECIS.0 as u32,
            CLIP_DEFAULT_PRECIS.0 as u32,
            CLEARTYPE_QUALITY.0 as u32,
            (FF_DONTCARE.0 | DEFAULT_PITCH.0) as u32,
            PCWSTR(wide_name.as_ptr()),
        );

        if font.is_invalid() {
            return None;
        }

        // Verify the actual loaded font matches what we requested
        let old = SelectObject(hdc, font);
        let mut actual_name = [0u16; 64];
        let len = GetTextFaceW(hdc, &mut actual_name);
        SelectObject(hdc, old);

        if len == 0 {
            DeleteObject(font);
            return None;
        }

        let actual = String::from_utf16_lossy(&actual_name[..len as usize]);
        if actual.trim_end_matches('\0') != face_name {
            // Font was substituted -- not actually available
            DeleteObject(font);
            return None;
        }

        Some(font)
    }
}
```

### Complete Module Skeleton
```rust
// native-theme/src/winicons.rs
// Windows icon loader: stock icons via SHGetStockIconInfo + font glyphs via GetGlyphOutlineW

use crate::{bundled_icon_svg, icon_name, IconData, IconRole, IconSet};

const DEFAULT_ICON_SIZE: i32 = 32; // pixels for font glyphs

pub fn load_windows_icon(role: IconRole) -> Option<IconData> {
    if let Some(name) = icon_name(IconSet::SegoeIcons, role) {
        if name.starts_with("SIID_") {
            if let Some(data) = load_stock_icon(name) {
                return Some(data);
            }
        } else if name == "IDI_QUESTION" {
            if let Some(data) = load_idi_icon() {
                return Some(data);
            }
        } else {
            if let Some(codepoint) = glyph_codepoint(name) {
                if let Some(data) = load_glyph_icon(codepoint, DEFAULT_ICON_SIZE) {
                    return Some(data);
                }
            }
        }
    }

    // Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role).map(|bytes| IconData::Svg(bytes.to_vec()))
}
```

### lib.rs Wiring
```rust
// In lib.rs, alongside the existing platform modules:
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub mod winicons;

#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub use winicons::load_windows_icon;
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Segoe MDL2 Assets icon font | Segoe Fluent Icons | Windows 11 (2021) | Same codepoints, updated visual style; MDL2 still works as fallback |
| ExtractIcon from system DLLs | SHGetStockIconInfo | Windows Vista (2006) | Clean API for stock system icons |
| GDI ExtTextOut for font rendering | GetGlyphOutlineW(GGO_GRAY8_BITMAP) | Always available | Only reliable approach for alpha-correct glyph bitmaps |
| GetBitmapBits for pixel extraction | GetDIBits with BITMAPINFOHEADER | Win32 (GetBitmapBits deprecated) | Format control, alignment handling |

**Deprecated/outdated:**
- `GetBitmapBits()`: Deprecated since Win32; use `GetDIBits()` instead
- `lockFocus`/`unlockFocus`: macOS only, not relevant here
- Segoe UI Symbol font: Predecessor to Segoe MDL2 Assets, very limited glyph set

## Open Questions

1. **Should the system-icons feature require the windows feature on Windows?**
   - What we know: The `windows` crate is optional and activated by the `windows` Cargo feature. The winicons module needs APIs from the `windows` crate. Currently `system-icons` does NOT include `"dep:windows"`.
   - What's unclear: Whether to add `"dep:windows"` to `system-icons` or require users to enable both features.
   - Recommendation: Add `"dep:windows"` to the `system-icons` feature list. This matches the pattern where `system-icons` already includes `"dep:freedesktop-icons"` and `"dep:objc2-core-graphics"` for their respective platforms. Cargo handles platform-gated optional deps correctly.

2. **What icon size should stock icons use?**
   - What we know: `SHGSI_LARGEICON` returns SM_CXICON size (typically 32x32 at 96 DPI). `SHGSI_SMALLICON` returns SM_CXSMICON (typically 16x16). There is no "exact size" flag.
   - What's unclear: Whether to use large (32px) or small (16px) icons.
   - Recommendation: Use `SHGSI_LARGEICON` for 32x32 icons. The actual pixel dimensions are read from the bitmap via `GetObjectW`, so the returned `IconData::Rgba` width/height will be correct regardless of DPI scaling.

3. **Should font glyph icons be rendered as white-on-transparent or black-on-transparent?**
   - What we know: The sficons.rs module renders SF Symbols as-is (the system provides colored/monochrome pixels). Segoe Fluent Icons are typically rendered in the foreground color. The `GetGlyphOutlineW` alpha mask has no inherent color.
   - What's unclear: Whether callers expect a specific foreground color.
   - Recommendation: Render as white (255,255,255) foreground with alpha from the grayscale mask. This matches the common convention for icon fonts and allows callers to apply their own tint/foreground color via simple multiplication.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | native-theme/Cargo.toml (features) |
| Quick run command | `cargo test -p native-theme --features system-icons,windows --lib` |
| Full suite command | `cargo test -p native-theme --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-02-a | load_windows_icon returns Rgba for SIID_ roles | integration | `cargo test -p native-theme --features system-icons,windows winicons::tests::stock_icon_returns_rgba -- -x` | Wave 0 |
| PLAT-02-b | RGBA has correct dimensions (width * height * 4 == data.len()) | unit | `cargo test -p native-theme --features system-icons,windows winicons::tests::rgba_dimensions_correct -- -x` | Wave 0 |
| PLAT-02-c | Pixels are RGBA (not BGRA) byte order | unit | `cargo test -p native-theme --features system-icons,windows winicons::tests::bgra_to_rgba_conversion -- -x` | Wave 0 |
| PLAT-02-d | Straight alpha (not premultiplied) | unit | `cargo test -p native-theme --features system-icons,windows winicons::tests::unpremultiply_correctness -- -x` | Wave 0 |
| PLAT-03-a | load_windows_icon returns Rgba for glyph roles when font present | integration | `cargo test -p native-theme --features system-icons,windows winicons::tests::glyph_icon_returns_rgba -- -x` | Wave 0 |
| PLAT-03-b | GGO_GRAY8_BITMAP alpha scaling (0-64 -> 0-255) correctness | unit | `cargo test -p native-theme --features system-icons,windows winicons::tests::gray8_alpha_scaling -- -x` | Wave 0 |
| PLAT-03-c | Falls back to Segoe MDL2 Assets when Fluent not available | integration | `cargo test -p native-theme --features system-icons,windows winicons::tests::font_fallback -- -x` | Wave 0 |
| PLAT-03-d | Falls back to bundled SVG when no font available | integration | `cargo test -p native-theme --features system-icons,windows winicons::tests::fallback_to_bundled -- -x` | Wave 0 |

Note: Integration tests for PLAT-02-a, PLAT-03-a, PLAT-03-c require running on Windows. The pure conversion tests (PLAT-02-c, PLAT-02-d, PLAT-03-b) can be platform-independent unit tests operating on synthetic data.

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --features system-icons,windows --lib`
- **Per wave merge:** `cargo test -p native-theme --all-features`
- **Phase gate:** Full suite green before verify-work

### Wave 0 Gaps
- [ ] `native-theme/src/winicons.rs` -- the main module (does not exist yet)
- [ ] Tests within the module covering PLAT-02 and PLAT-03 sub-behaviors
- [ ] `Win32_UI_Shell` feature added to windows crate dependency in Cargo.toml
- [ ] `"dep:windows"` potentially added to `system-icons` feature

## Sources

### Primary (HIGH confidence)
- [Microsoft Learn: SHGetStockIconInfo](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo) - Function signature, parameters, SHGSI flags, cleanup requirements
- [Microsoft Learn: SHSTOCKICONID](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-shstockiconid) - All 94 stock icon constants with numeric values and descriptions
- [Microsoft Learn: GetGlyphOutlineW](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getglyphoutlinew) - GGO_GRAY8_BITMAP format (65 levels, DWORD-aligned rows), GLYPHMETRICS output
- [Microsoft Learn: Segoe Fluent Icons font](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font) - Complete codepoint table for all glyphs, Win11 availability
- [microsoft.github.io/windows-docs-rs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/) - Rust API types for Win32::UI::Shell, Win32::Graphics::Gdi
- [Microsoft Q&A: Segoe Fluent vs MDL2 codepoints](https://learn.microsoft.com/en-us/answers/questions/1467934/difference-between-segoe-fluent-icons-and-segoe-md) - Confirms shared PUA codepoints between fonts
- Existing codebase: `sficons.rs` (CGBitmapContext rasterization pattern, unpremultiply_alpha), `freedesktop.rs` (icon loader pattern), `icons.rs` (segoe_name mapping with SIID_/glyph prefixes)

### Secondary (MEDIUM confidence)
- [Rust users forum: HICON to PNG](https://users.rust-lang.org/t/how-to-convert-hicon-to-png/90975) - GetIconInfo + GetDIBits/GetBitmapBits pattern for extracting HICON pixels
- [virtualdub.org: Drawing text in a 3D program](https://www.virtualdub.org/blog2/entry_379.html) - GDI alpha channel destruction with ExtTextOut, GGO_GRAY8_BITMAP as alternative
- [JUCE forum: HICON to ARGB](https://forum.juce.com/t/conversion-of-hicon-to-argb-image-icon2image/6565) - BGRA byte order from GetDIBits, swap B/R for RGBA
- [theartofdev.com: Transparent text rendering with GDI](https://theartofdev.com/2013/10/24/transparent-text-rendering-with-gdi/) - Confirmed GDI text APIs destroy alpha channel

### Tertiary (LOW confidence)
- None -- all critical claims verified against official Microsoft docs and existing codebase patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - windows crate already in project, only needs one additional feature (`Win32_UI_Shell`)
- Architecture: HIGH - Two-pipeline dispatch matches existing segoe_name() convention perfectly; GGO_GRAY8_BITMAP approach verified against multiple sources as the only GDI approach that preserves alpha
- Pitfalls: HIGH - BGRA byte order, premultiplied alpha, GDI alpha destruction, font fallback all verified against official docs and community experience
- Codepoints: HIGH - All 22 glyph codepoints verified against official Microsoft Segoe Fluent Icons documentation

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable Win32 APIs, Segoe Fluent Icons codepoints are stable PUA values)
