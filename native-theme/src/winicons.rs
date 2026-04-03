// Windows icon loader
//
// Resolves IconRole variants to RGBA pixel data via two pipelines:
// 1. Stock icons via SHGetStockIconInfo (18 SIID_ prefixed roles)
// 2. Font glyphs via GetGlyphOutlineW from Segoe Fluent Icons (22 glyph roles)
//
// Both pipelines produce IconData::Rgba with correct RGBA byte order and
// straight (non-premultiplied) alpha. Returns None when the role has no
// Segoe mapping or the icon cannot be loaded on this system.
//
// This module is compiled on all platforms (gated by feature = "system-icons")
// so that platform-independent logic like `parse_hex_codepoint` can be tested
// everywhere. Windows-specific code is behind `#[cfg(target_os = "windows")]`.

// Win32 GDI FFI -- no safe alternative
#![allow(unsafe_code)]
use crate::{IconData, IconRole, IconSet, icon_name};

#[cfg(target_os = "windows")]
use std::mem;
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::*;
#[cfg(target_os = "windows")]
use windows::Win32::UI::Shell::*;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;

/// Default icon size in pixels for font glyph rendering.
const DEFAULT_ICON_SIZE: i32 = 32;

/// Map a Segoe Fluent Icons glyph name to its Unicode PUA codepoint.
///
/// These codepoints are shared between Segoe Fluent Icons (Win11) and
/// Segoe MDL2 Assets (Win10).
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

/// Map a SIID_ string name to its SHSTOCKICONID constant.
#[cfg(target_os = "windows")]
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

/// Swap BGRA byte order to RGBA by exchanging bytes 0 and 2 in each pixel.
fn bgra_to_rgba(pixels: &mut [u8]) {
    for pixel in pixels.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
}

/// Convert a GGO_GRAY8_BITMAP grayscale value (0-64) to an RGBA pixel.
///
/// Produces white foreground (255,255,255) with alpha scaled from the
/// 65-level grayscale: `alpha = min(255, gray * 255 / 64)`.
fn gray8_to_rgba(gray: u8) -> [u8; 4] {
    let alpha = ((gray as u32 * 255) / 64).min(255) as u8;
    [255, 255, 255, alpha]
}

/// Extract RGBA pixels from an HICON handle.
///
/// Uses GetIconInfo to obtain the color bitmap, then GetDIBits with a
/// top-down 32-bit BITMAPINFOHEADER to extract BGRA pixels. Converts
/// to RGBA and applies unpremultiply.
#[cfg(target_os = "windows")]
unsafe fn hicon_to_rgba(hicon: HICON) -> Option<IconData> {
    unsafe {
        let mut icon_info = ICONINFO::default();
        GetIconInfo(hicon, &mut icon_info).ok()?;

        // Get bitmap dimensions
        let mut bmp = BITMAP::default();
        GetObjectW(
            icon_info.hbmColor.into(),
            mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        );

        let width = bmp.bmWidth as u32;
        let height = bmp.bmHeight as u32;

        if width == 0 || height == 0 {
            DeleteObject(icon_info.hbmColor.into());
            DeleteObject(icon_info.hbmMask.into());
            return None;
        }

        // Set up BITMAPINFOHEADER for 32-bit top-down BGRA
        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
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
        let _ = DeleteDC(hdc);

        // Cleanup bitmaps
        DeleteObject(icon_info.hbmColor.into());
        DeleteObject(icon_info.hbmMask.into());

        // Convert BGRA to RGBA and fix premultiplied alpha
        bgra_to_rgba(&mut pixels);
        crate::color::unpremultiply_alpha(&mut pixels);

        Some(IconData::Rgba {
            width,
            height,
            data: pixels,
        })
    }
}

/// Load a stock system icon by its SIID_ name.
///
/// Maps the name to a SHSTOCKICONID, calls SHGetStockIconInfo to get
/// an HICON, extracts RGBA pixels, and cleans up the icon handle.
#[cfg(target_os = "windows")]
fn load_stock_icon(name: &str) -> Option<IconData> {
    let siid = siid_from_name(name)?;

    unsafe {
        let mut sii = SHSTOCKICONINFO::default();
        sii.cbSize = mem::size_of::<SHSTOCKICONINFO>() as u32;

        SHGetStockIconInfo(siid, SHGSI_ICON | SHGSI_LARGEICON, &mut sii).ok()?;

        let result = hicon_to_rgba(sii.hIcon);

        // Clean up the icon handle
        let _ = DestroyIcon(sii.hIcon);

        result
    }
}

/// Load the system question dialog icon via LoadIconW.
///
/// DialogQuestion maps to IDI_QUESTION which requires LoadIconW instead
/// of SHGetStockIconInfo (there is no SIID_QUESTION constant).
/// Note: system icons from LoadIconW are shared resources and should
/// NOT be destroyed with DestroyIcon.
#[cfg(target_os = "windows")]
fn load_idi_icon() -> Option<IconData> {
    unsafe {
        let hicon = LoadIconW(None, IDI_QUESTION).ok()?;
        hicon_to_rgba(hicon)
        // Do NOT call DestroyIcon -- shared system resource
    }
}

/// Try to create a font and verify it was actually loaded (not substituted).
///
/// GDI silently substitutes a different font if the requested one is not
/// available. We detect this by comparing GetTextFaceW output against the
/// requested name.
#[cfg(target_os = "windows")]
unsafe fn try_create_font(hdc: HDC, face_name: &str, size: i32) -> Option<HFONT> {
    unsafe {
        let wide_name: Vec<u16> = face_name.encode_utf16().chain(std::iter::once(0)).collect();

        let font = CreateFontW(
            size,
            0,
            0,
            0,
            FW_NORMAL.0 as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_TT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY,
            (FF_DONTCARE.0 | DEFAULT_PITCH.0) as u32,
            PCWSTR(wide_name.as_ptr()),
        );

        if font.is_invalid() {
            return None;
        }

        // Verify the actual loaded font matches what we requested
        let old = SelectObject(hdc, font.into());
        let mut actual_name = [0u16; 64];
        let len = GetTextFaceW(hdc, Some(&mut actual_name));
        SelectObject(hdc, old);

        if len == 0 {
            let _ = DeleteObject(font.into());
            return None;
        }

        let actual = String::from_utf16_lossy(&actual_name[..len as usize]);
        if actual.trim_end_matches('\0') != face_name {
            // Font was substituted -- not actually available
            let _ = DeleteObject(font.into());
            return None;
        }

        Some(font)
    }
}

/// Render a font glyph to RGBA pixels via GetGlyphOutlineW.
///
/// Tries Segoe Fluent Icons first (Win11), then Segoe MDL2 Assets (Win10).
/// Returns None if neither font is available (caller falls back to bundled).
///
/// Uses GGO_GRAY8_BITMAP which produces a 65-level grayscale alpha mask
/// (values 0-64), converted to RGBA with white foreground and scaled alpha.
#[cfg(target_os = "windows")]
fn load_glyph_icon(codepoint: u32, size: i32) -> Option<IconData> {
    unsafe {
        let hdc = CreateCompatibleDC(None);

        // Try Segoe Fluent Icons first (Win11), then Segoe MDL2 Assets (Win10)
        let font = try_create_font(hdc, "Segoe Fluent Icons", size)
            .or_else(|| try_create_font(hdc, "Segoe MDL2 Assets", size));

        let font = match font {
            Some(f) => f,
            None => {
                let _ = DeleteDC(hdc);
                return None;
            }
        };

        let old_font = SelectObject(hdc, font.into());

        // Identity matrix (no transform)
        let mat2 = MAT2 {
            eM11: FIXED { fract: 0, value: 1 },
            eM12: FIXED { fract: 0, value: 0 },
            eM21: FIXED { fract: 0, value: 0 },
            eM22: FIXED { fract: 0, value: 1 },
        };

        let mut gm = GLYPHMETRICS::default();

        // First call: get buffer size
        let buf_size = GetGlyphOutlineW(hdc, codepoint, GGO_GRAY8_BITMAP, &mut gm, 0, None, &mat2);

        if buf_size == GDI_ERROR as u32 || buf_size == 0 {
            SelectObject(hdc, old_font);
            let _ = DeleteObject(font.into());
            let _ = DeleteDC(hdc);
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
        let _ = DeleteObject(font.into());
        let _ = DeleteDC(hdc);

        let glyph_w = gm.gmBlackBoxX;
        let glyph_h = gm.gmBlackBoxY;

        if glyph_w == 0 || glyph_h == 0 {
            return None;
        }

        // GGO_GRAY8_BITMAP rows are DWORD-aligned
        let pitch = ((glyph_w + 3) & !3) as usize;

        // Convert grayscale (0-64) to RGBA (white with scaled alpha)
        let mut rgba = Vec::with_capacity((glyph_w * glyph_h * 4) as usize);
        for y in 0..glyph_h as usize {
            for x in 0..glyph_w as usize {
                let gray = gray_buf[y * pitch + x];
                rgba.extend_from_slice(&gray8_to_rgba(gray));
            }
        }

        Some(IconData::Rgba {
            width: glyph_w,
            height: glyph_h,
            data: rgba,
        })
    }
}

/// Load a Windows icon for the given role as RGBA pixel data.
///
/// # Dispatch
///
/// - Names starting with `SIID_`: stock icon via SHGetStockIconInfo
/// - `IDI_QUESTION`: system dialog icon via LoadIconW
/// - Other names: font glyph via GetGlyphOutlineW (Segoe Fluent/MDL2)
///
/// Returns `None` if the role has no Segoe mapping or the icon cannot
/// be loaded on this system.
#[must_use]
pub fn load_windows_icon(role: IconRole) -> Option<IconData> {
    #[cfg(target_os = "windows")]
    if let Some(name) = icon_name(role, IconSet::SegoeIcons) {
        if name.starts_with("SIID_") {
            if let Some(data) = load_stock_icon(name) {
                return Some(data);
            }
        } else if name == "IDI_QUESTION" {
            if let Some(data) = load_idi_icon() {
                return Some(data);
            }
        } else if let Some(codepoint) = glyph_codepoint(name) {
            if let Some(data) = load_glyph_icon(codepoint, DEFAULT_ICON_SIZE) {
                return Some(data);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    let _ = role;

    None
}

/// Parse a hex codepoint string like "0xE8BB" or "0xe8bb" to a u32.
///
/// Requires the `0x` or `0X` prefix to avoid ambiguity with named glyphs
/// (e.g., "Add" is both a valid glyph name and valid hex).
///
/// Returns `None` for non-hex strings or strings without the 0x prefix.
fn parse_hex_codepoint(name: &str) -> Option<u32> {
    let hex = name
        .strip_prefix("0x")
        .or_else(|| name.strip_prefix("0X"))?;
    if hex.is_empty() {
        return None;
    }
    u32::from_str_radix(hex, 16).ok()
}

/// Load a Windows icon by its name string.
///
/// Accepts multiple name formats:
/// - `"SIID_*"` -- stock system icon via SHGetStockIconInfo
/// - `"IDI_QUESTION"` -- system dialog icon via LoadIconW
/// - `"0xE8BB"` -- hex codepoint for Segoe Fluent Icons glyph
/// - `"ChromeClose"` -- named Segoe Fluent Icons glyph
///
/// Returns `None` if the name doesn't match any known format or
/// the icon cannot be loaded on this system.
#[must_use]
pub fn load_windows_icon_by_name(name: &str) -> Option<IconData> {
    #[cfg(target_os = "windows")]
    {
        if name.starts_with("SIID_") {
            return load_stock_icon(name);
        }
        if name == "IDI_QUESTION" {
            return load_idi_icon();
        }
        // Try hex codepoint (e.g., "0xE8BB")
        if let Some(cp) = parse_hex_codepoint(name) {
            return load_glyph_icon(cp, DEFAULT_ICON_SIZE);
        }
        // Try named glyph (e.g., "ChromeClose")
        if let Some(cp) = glyph_codepoint(name) {
            return load_glyph_icon(cp, DEFAULT_ICON_SIZE);
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = name;
        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === Platform-independent unit tests ===

    #[test]
    fn bgra_to_rgba_conversion() {
        // BGRA blue [0, 0, 255, 255] -> RGBA red [255, 0, 0, 255]
        let mut buf = [0u8, 0, 255, 255];
        bgra_to_rgba(&mut buf);
        assert_eq!(buf, [255, 0, 0, 255]);
    }

    #[test]
    fn unpremultiply_correctness() {
        // Premultiplied [128, 0, 0, 128] -> straight [255, 0, 0, 128]
        let mut buf = [128u8, 0, 0, 128];
        crate::color::unpremultiply_alpha(&mut buf);
        assert_eq!(buf, [255, 0, 0, 128]);

        // Fully opaque pixels are unchanged
        let mut buf = [100u8, 200, 50, 255];
        crate::color::unpremultiply_alpha(&mut buf);
        assert_eq!(buf, [100, 200, 50, 255]);

        // Fully transparent pixels are unchanged
        let mut buf = [0u8, 0, 0, 0];
        crate::color::unpremultiply_alpha(&mut buf);
        assert_eq!(buf, [0, 0, 0, 0]);
    }

    #[test]
    fn gray8_alpha_scaling() {
        // gray=64 -> full alpha (255)
        let pixel = gray8_to_rgba(64);
        assert_eq!(pixel, [255, 255, 255, 255]);

        // gray=0 -> transparent (0)
        let pixel = gray8_to_rgba(0);
        assert_eq!(pixel, [255, 255, 255, 0]);

        // gray=32 -> half alpha: (32 * 255) / 64 = 127
        let pixel = gray8_to_rgba(32);
        assert_eq!(pixel, [255, 255, 255, 127]);
    }

    #[test]
    fn glyph_codepoint_lookup() {
        assert_eq!(glyph_codepoint("ChromeClose"), Some(0xE8BB));
        assert_eq!(glyph_codepoint("Unknown"), None);
    }

    #[test]
    fn unmapped_role_returns_none() {
        // StatusBusy has no Segoe mapping (known gap), should return None
        let result = load_windows_icon(IconRole::StatusBusy);
        assert!(
            result.is_none(),
            "StatusBusy should return None (no Segoe mapping, no fallback)"
        );
    }

    // === Windows-only integration tests ===

    #[cfg(target_os = "windows")]
    #[test]
    fn stock_icon_returns_rgba() {
        let result = load_windows_icon(IconRole::DialogWarning);
        assert!(result.is_some(), "DialogWarning should return an icon");
        if let Some(IconData::Rgba { .. }) = result {
            // Stock icons should return RGBA
        } else {
            panic!("Expected IconData::Rgba for stock icon");
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn glyph_icon_returns_rgba() {
        let result = load_windows_icon(IconRole::ActionCopy);
        assert!(result.is_some(), "ActionCopy should return an icon");
        if let Some(IconData::Rgba { .. }) = result {
            // Font glyph icons should return RGBA
        } else {
            panic!("Expected IconData::Rgba for glyph icon");
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn rgba_dimensions_correct() {
        if let Some(IconData::Rgba {
            width,
            height,
            data,
        }) = load_windows_icon(IconRole::DialogWarning)
        {
            assert_eq!(
                (width * height * 4) as usize,
                data.len(),
                "RGBA buffer size must equal width * height * 4"
            );
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn idi_question_returns_some() {
        let result = load_windows_icon(IconRole::DialogQuestion);
        assert!(
            result.is_some(),
            "DialogQuestion should return an icon via LoadIconW"
        );
    }

    // === parse_hex_codepoint tests ===

    #[test]
    fn parse_hex_codepoint_with_prefix() {
        assert_eq!(parse_hex_codepoint("0xE8BB"), Some(0xE8BB));
    }

    #[test]
    fn parse_hex_codepoint_lowercase() {
        assert_eq!(parse_hex_codepoint("0xe8bb"), Some(0xE8BB));
    }

    #[test]
    fn parse_hex_codepoint_no_prefix_returns_none() {
        // "E8BB" is valid hex but without 0x prefix, requires prefix per design
        assert_eq!(parse_hex_codepoint("E8BB"), None);
    }

    #[test]
    fn parse_hex_codepoint_named_glyph_not_hex() {
        assert_eq!(parse_hex_codepoint("ChromeClose"), None);
    }

    #[test]
    fn parse_hex_codepoint_add_not_hex() {
        // "Add" is both a valid glyph name and valid hex -- 0x prefix disambiguates
        assert_eq!(parse_hex_codepoint("Add"), None);
    }

    #[test]
    fn parse_hex_codepoint_invalid() {
        assert_eq!(parse_hex_codepoint("0xZZZZ"), None);
    }

    #[test]
    fn parse_hex_codepoint_empty() {
        assert_eq!(parse_hex_codepoint(""), None);
    }

    #[test]
    fn parse_hex_codepoint_bare_0x() {
        assert_eq!(parse_hex_codepoint("0x"), None);
    }

    // === Windows-only load_windows_icon_by_name tests ===

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_hex_codepoint() {
        // 0xE8C8 is the Copy glyph
        let result = load_windows_icon_by_name("0xE8C8");
        assert!(
            result.is_some(),
            "hex codepoint 0xE8C8 should load Copy glyph"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_named_glyph() {
        let result = load_windows_icon_by_name("Copy");
        assert!(result.is_some(), "named glyph Copy should load");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_stock_icon() {
        let result = load_windows_icon_by_name("SIID_WARNING");
        assert!(result.is_some(), "SIID_WARNING stock icon should load");
    }
}
