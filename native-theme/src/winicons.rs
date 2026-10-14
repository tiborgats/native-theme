// Windows icon loader
//
// Resolves Segoe icon names (the names `icon_name` gives each IconRole in
// IconSet::SegoeFluent) to RGBA pixel data via two pipelines:
// 1. Stock icons via SHGetStockIconInfo (13 SIID_ prefixed roles), and
//    IDI_QUESTION via LoadIconW -- full-colour
// 2. Font glyphs via GetGlyphOutlineW from Segoe Fluent Icons (27 glyph
//    roles) -- monochrome, drawn in the requested colour or white on the
//    font's size x size em box
//
// Both pipelines produce IconData::Rgba with correct RGBA byte order and
// straight (non-premultiplied) alpha. Returns None when the name is not
// one of them or the icon cannot be loaded on this system.
//
// This module is compiled on all platforms (gated by feature = "system-icons")
// so that platform-independent logic like `parse_hex_codepoint` can be tested
// everywhere. Windows-specific code is behind `#[cfg(target_os = "windows")]`.

// Win32 GDI FFI -- no safe alternative
#![allow(unsafe_code)]
use crate::IconData;

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

/// Default icon size in pixels for font glyph rendering: the em height the
/// font is created with.
const DEFAULT_ICON_SIZE: u32 = 32;

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
        // Names and codepoints as the Segoe Fluent Icons reference lists them:
        // https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font
        "Search" => 0xE721,
        "Settings" => 0xE713,
        "Delete" => 0xE74D,
        "Print" => 0xE749,
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
    for pixel in pixels.as_chunks_mut::<4>().0 {
        pixel.swap(0, 2);
    }
}

/// Convert a GGO_GRAY8_BITMAP grayscale value (0-64) to an RGBA pixel.
///
/// Produces white foreground (255,255,255) with alpha scaled from the
/// 65-level grayscale: `alpha = min(255, gray * 255 / 64)`.
fn gray8_to_rgba(gray: u8) -> [u8; 4] {
    // gray is 0..=64 per GGO_GRAY8 contract. `u32::from(gray) * 255` max is 16320,
    // no overflow risk. Division by literal 64 is arithmetically safe (non-zero
    // constant); `.min(255)` caps the result so the `as u8` cast is exact.
    #[allow(clippy::integer_division, clippy::arithmetic_side_effects)]
    let alpha = (u32::from(gray).saturating_mul(255) / 64).min(255) as u8;
    [255, 255, 255, alpha]
}

/// Extract RGBA pixels from an HICON handle.
///
/// Uses GetIconInfo to obtain the color bitmap, then GetDIBits with a
/// top-down 32-bit BITMAPINFOHEADER to extract BGRA pixels. Converts
/// to RGBA and applies unpremultiply. `None` when a GDI call fails or the
/// bitmap is over the decode bound
/// ([`raster_bytes_in_bound`](crate::icons::raster_bytes_in_bound)).
#[cfg(target_os = "windows")]
// SAFETY: the caller passes a live icon handle and keeps it alive for the call;
// it is not destroyed here.
unsafe fn hicon_to_rgba(hicon: HICON) -> Option<IconData> {
    let mut icon_info = ICONINFO::default();
    // SAFETY: `hicon` is an icon handle (the caller's contract) and
    // `icon_info` is a live ICONINFO for GetIconInfo to fill.
    unsafe { GetIconInfo(hicon, &mut icon_info) }.ok()?;

    // SAFETY: GetIconInfo's colour bitmap belongs to this function and is
    // selected into no device context.
    let bitmap = unsafe { bitmap_bgra(icon_info.hbmColor) };

    // SAFETY: GetIconInfo created both bitmaps for the caller to delete, and
    // neither is used after this. A failed delete has no recovery.
    unsafe {
        let _ = DeleteObject(icon_info.hbmColor.into());
        let _ = DeleteObject(icon_info.hbmMask.into());
    }

    let (width, height, mut pixels) = bitmap?;
    // Convert BGRA to RGBA and fix premultiplied alpha
    bgra_to_rgba(&mut pixels);
    crate::color::unpremultiply_alpha(&mut pixels);

    IconData::rgba(width, height, pixels)
}

/// The width, height and top-down 32-bit BGRA pixels of a bitmap, read
/// with GetDIBits; `None` when a GDI call fails, when GetDIBits copies
/// fewer than all the rows, or for a bitmap that is empty or over the
/// decode bound.
///
/// # Safety
///
/// `hbm` is a bitmap handle selected into no device context, as GetDIBits
/// requires.
#[cfg(target_os = "windows")]
// SAFETY: the caller passes a bitmap handle selected into no device context
// (`# Safety` above), as GetDIBits requires.
unsafe fn bitmap_bgra(hbm: HBITMAP) -> Option<(u32, u32, Vec<u8>)> {
    let mut bmp = BITMAP::default();
    // SAFETY: `bmp` is a live BITMAP of the size passed, for GetObjectW to
    // fill; it returns 0 on failure.
    let written = unsafe {
        GetObjectW(
            hbm.into(),
            mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        )
    };
    if written == 0 {
        return None;
    }

    let width = u32::try_from(bmp.bmWidth).ok()?;
    let height = u32::try_from(bmp.bmHeight).ok()?;
    let bytes = crate::icons::raster_bytes_in_bound(width, height)?;

    // Set up BITMAPINFOHEADER for 32-bit top-down BGRA
    let mut bmi = BITMAPINFO::default();
    bmi.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = bmp.bmWidth;
    bmi.bmiHeader.biHeight = bmp.bmHeight.checked_neg()?; // negative = top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB.0;

    let mut pixels = vec![0u8; bytes];

    // SAFETY: a memory DC compatible with the screen, deleted below.
    let hdc = unsafe { CreateCompatibleDC(None) };
    if hdc.is_invalid() {
        return None;
    }
    // SAFETY: `pixels` holds `height` rows of `width` 32-bit pixels, the
    // layout `bmi` asks for, so GetDIBits writes inside it; `hdc` is live and
    // `hbm` is selected into no DC (the caller's contract).
    let lines = unsafe {
        GetDIBits(
            hdc,
            hbm,
            0,
            height,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    };
    // SAFETY: `hdc` was created above and is not used after this.
    let _ = unsafe { DeleteDC(hdc) };

    // GetDIBits returns the number of scan lines copied, 0 on failure.
    if u32::try_from(lines).ok() != Some(height) {
        return None;
    }
    Some((width, height, pixels))
}

/// Held for each stock icon load, so that one runs at a time.
///
/// On GitHub's `windows-latest` image, of the test threads that loaded
/// `SIID_WARNING` at the same time, one got `None` and another, with the same
/// arguments, the icon (CI run 38478340750). Microsoft documents no thread
/// rule for SHGetStockIconInfo. Mozilla found that the shell's SHGetFileInfo,
/// which like it gives the icons of the system image list
/// (`SHGSI_SYSICONINDEX`), cannot be called from more than one thread at a
/// time, and calls it from one thread
/// (<https://bugzilla.mozilla.org/show_bug.cgi?id=1530190>, comments 11-12):
/// a developer's finding, not Microsoft's.
#[cfg(target_os = "windows")]
static STOCK_ICON_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Load a stock system icon by its SIID_ name.
///
/// Maps the name to a SHSTOCKICONID, calls SHGetStockIconInfo to get
/// an HICON, extracts RGBA pixels, and cleans up the icon handle, one load
/// at a time ([`STOCK_ICON_LOCK`]).
#[cfg(target_os = "windows")]
fn load_stock_icon(name: &str) -> Option<IconData> {
    let siid = siid_from_name(name)?;
    // The lock guards no data, so a load that panicked leaves nothing to
    // repair.
    let _one_at_a_time = STOCK_ICON_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    // SAFETY: `sii` is a live SHSTOCKICONINFO with `cbSize` set to its size, for
    // SHGetStockIconInfo to fill; on success `hIcon` is an icon this function owns,
    // live for `hicon_to_rgba` and destroyed once afterwards, after its last use.
    unsafe {
        let mut sii = SHSTOCKICONINFO {
            cbSize: mem::size_of::<SHSTOCKICONINFO>() as u32,
            ..Default::default()
        };

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
    // SAFETY: LoadIconW with no module loads a predefined system icon, a shared
    // handle that stays live and is never destroyed here; `hicon_to_rgba` only
    // reads it.
    unsafe {
        let hicon = LoadIconW(None, IDI_QUESTION).ok()?;
        hicon_to_rgba(hicon)
        // Do NOT call DestroyIcon -- shared system resource
    }
}

/// Try to create a font and verify it was actually loaded (not substituted).
///
/// `em_height` is the font's em height in pixels: CreateFontW matches a
/// negative height against the character (em) height, a positive one
/// against the cell height (LOGFONTW `lfHeight`,
/// <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw>).
///
/// GDI silently substitutes a different font if the requested one is not
/// available. We detect this by comparing GetTextFaceW output against the
/// requested name.
#[cfg(target_os = "windows")]
// SAFETY: the caller passes a live device context; the font created is
// returned only if not substituted, and deleted on every other path.
unsafe fn try_create_font(hdc: HDC, face_name: &str, em_height: u32) -> Option<HFONT> {
    let height = i32::try_from(em_height).ok()?.checked_neg()?;
    // SAFETY: `hdc` is a live DC (the caller's contract); `wide_name` is
    // NUL-terminated and outlives CreateFontW; the font is deselected again
    // before it is returned, and deleted on every path that does not.
    unsafe {
        let wide_name: Vec<u16> = face_name.encode_utf16().chain(std::iter::once(0)).collect();

        let font = CreateFontW(
            height,
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

        let actual = usize::try_from(len)
            .ok()
            .filter(|&len| len > 0)
            .and_then(|len| actual_name.get(..len))
            .map(String::from_utf16_lossy);
        let loaded = actual
            .as_deref()
            .map(|actual| actual.trim_end_matches('\0'));
        if loaded != Some(face_name) {
            // Font was substituted -- not actually available
            let _ = DeleteObject(font.into());
            return None;
        }

        Some(font)
    }
}

/// Where GetGlyphOutlineW puts a glyph's black box: its size, and its
/// upper left corner relative to the character cell's origin, which is at
/// the left of the cell on the baseline (GLYPHMETRICS,
/// <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-glyphmetrics>,
/// and GetGlyphOutlineW's remarks,
/// <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getglyphoutlinew>);
/// `origin_y` counts up from the baseline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GlyphBox {
    /// `gmBlackBoxX`
    width: u32,
    /// `gmBlackBoxY`
    height: u32,
    /// `gmptGlyphOrigin.x`
    origin_x: i32,
    /// `gmptGlyphOrigin.y`
    origin_y: i32,
}

/// Draw a GGO_GRAY8_BITMAP black box onto the `size × size` em box of the
/// font created at `size` pixels em height, whose em box's top is `ascent`
/// above the baseline ([`em_ascent`]).
///
/// The box's upper left corner lands at `(origin_x, ascent − origin_y)`,
/// ink outside the em box is clipped, and every pixel is white with the
/// glyph's coverage as alpha ([`gray8_to_rgba`]). `gray` holds the box's
/// rows, each DWORD-aligned (GetGlyphOutlineW's remarks).
///
/// Returns `None` for an empty box, a `gray` shorter than its rows, and a
/// `size` of 0 or over the decode bound.
fn glyph_canvas(gray: &[u8], glyph: GlyphBox, ascent: i32, size: u32) -> Option<Vec<u8>> {
    if !crate::icons::raster_size_in_bound(size) || glyph.width == 0 || glyph.height == 0 {
        return None;
    }
    let glyph_w = usize::try_from(glyph.width).ok()?;
    let glyph_h = usize::try_from(glyph.height).ok()?;
    let pitch = glyph_w.checked_add(3)? & !3;
    if gray.len() < pitch.checked_mul(glyph_h)? {
        return None;
    }
    let rows = gray
        .chunks(pitch)
        .take(glyph_h)
        .map(|row| row.get(..glyph_w))
        .collect::<Option<Vec<_>>>()?;

    let top = i64::from(ascent).checked_sub(i64::from(glyph.origin_y))?;
    let left = i64::from(glyph.origin_x);
    let size = usize::try_from(size).ok()?;
    let mut canvas = vec![gray8_to_rgba(0); size.checked_mul(size)?];
    for (y, out_row) in canvas.chunks_exact_mut(size).enumerate() {
        let Some(row) = box_offset(y, top).and_then(|y| rows.get(y)) else {
            continue;
        };
        for (x, pixel) in out_row.iter_mut().enumerate() {
            if let Some(&gray) = box_offset(x, left).and_then(|x| row.get(x)) {
                *pixel = gray8_to_rgba(gray);
            }
        }
    }
    Some(canvas.into_flattened())
}

/// The black box's row or column under canvas position `at`, for a box
/// that starts at `start`; `None` before the box.
fn box_offset(at: usize, start: i64) -> Option<usize> {
    usize::try_from(i64::try_from(at).ok()?.checked_sub(start)?).ok()
}

/// How far above the baseline the em box's top is, for a font created with
/// a negative height: its ascent less its internal leading.
///
/// The em height is the cell height less the internal leading (LOGFONTW
/// `lfHeight`,
/// <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw>),
/// and the internal leading lies inside the cell, `tmHeight` = ascent +
/// descent, where accent marks may occur (TEXTMETRICW `tmInternalLeading`,
/// <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-textmetricw>).
fn em_ascent(tm_ascent: i32, tm_internal_leading: i32) -> Option<i32> {
    tm_ascent.checked_sub(tm_internal_leading)
}

/// Render a font glyph to RGBA pixels via GetGlyphOutlineW, on the
/// `size × size` em box of the font created at `size` pixels em height
/// ([`glyph_canvas`]).
///
/// Tries Segoe Fluent Icons first (Win11), then Segoe MDL2 Assets (Win10).
/// Returns None if neither font is available (caller falls back to bundled)
/// or a GDI call fails.
///
/// Uses GGO_GRAY8_BITMAP which produces a 65-level grayscale alpha mask
/// (values 0-64), converted to RGBA with scaled alpha and a white
/// foreground, or `fg_color` where it is given.
#[cfg(target_os = "windows")]
fn load_glyph_icon(codepoint: u32, size: u32, fg_color: Option<[u8; 3]>) -> Option<IconData> {
    if !crate::icons::raster_size_in_bound(size) {
        return None;
    }
    // SAFETY: a memory DC compatible with the screen, deleted below.
    let hdc = unsafe { CreateCompatibleDC(None) };
    if hdc.is_invalid() {
        return None;
    }

    // Try Segoe Fluent Icons first (Win11), then Segoe MDL2 Assets (Win10)
    // SAFETY: `hdc` is the live DC created above.
    let font = unsafe {
        try_create_font(hdc, "Segoe Fluent Icons", size)
            .or_else(|| try_create_font(hdc, "Segoe MDL2 Assets", size))
    };

    let rgba = font.and_then(|font| {
        // SAFETY: `hdc` and `font` are live; render_glyph selects the font
        // back out before it returns.
        let rgba = unsafe { render_glyph(hdc, font, codepoint, size) };
        // SAFETY: `font` was created by try_create_font, is selected into no
        // DC and is not used after this.
        let _ = unsafe { DeleteObject(font.into()) };
        rgba
    });
    // SAFETY: `hdc` was created above and is not used after this.
    let _ = unsafe { DeleteDC(hdc) };

    let mut rgba = rgba?;
    crate::color::tint_monochrome(&mut rgba, fg_color);
    IconData::rgba(size, size, rgba)
}

/// Draw `codepoint` of `font` onto the `size × size` em box
/// ([`glyph_canvas`]), with `font` selected into `hdc` meanwhile.
///
/// # Safety
///
/// `hdc` is a live device context and `font` a live font.
#[cfg(target_os = "windows")]
unsafe fn render_glyph(hdc: HDC, font: HFONT, codepoint: u32, size: u32) -> Option<Vec<u8>> {
    // SAFETY: both handles are live (the caller's contract); the previous
    // font is selected back before this function returns.
    let old_font = unsafe { SelectObject(hdc, font.into()) };
    // SAFETY: `hdc` is live with `font` selected.
    let outline = unsafe { glyph_outline(hdc, codepoint) };
    // SAFETY: `old_font` is the object SelectObject replaced in `hdc`.
    unsafe { SelectObject(hdc, old_font) };
    let (gray, glyph, ascent) = outline?;
    glyph_canvas(&gray, glyph, ascent, size)
}

/// The GGO_GRAY8_BITMAP black box of `codepoint` in the font selected into
/// `hdc`, where GetGlyphOutlineW puts it, and the font's em box's top above
/// the baseline ([`em_ascent`]). `None` when a call fails, for a glyph with no
/// bitmap, and for a buffer over the decode bound.
///
/// # Safety
///
/// `hdc` is a live device context.
#[cfg(target_os = "windows")]
// SAFETY: the caller passes a live device context (`# Safety` above).
unsafe fn glyph_outline(hdc: HDC, codepoint: u32) -> Option<(Vec<u8>, GlyphBox, i32)> {
    let mut tm = TEXTMETRICW::default();
    // SAFETY: `hdc` is live (the caller's contract) and `tm` is a live
    // TEXTMETRICW for GetTextMetricsW to fill.
    if !unsafe { GetTextMetricsW(hdc, &mut tm) }.as_bool() {
        return None;
    }

    // Identity matrix (no transform)
    let mat2 = MAT2 {
        eM11: FIXED { fract: 0, value: 1 },
        eM12: FIXED { fract: 0, value: 0 },
        eM21: FIXED { fract: 0, value: 0 },
        eM22: FIXED { fract: 0, value: 1 },
    };

    let mut gm = GLYPHMETRICS::default();

    // First call: get buffer size
    // SAFETY: with no buffer GetGlyphOutlineW only fills `gm`, a live
    // GLYPHMETRICS, and reads `mat2`.
    let buf_size =
        unsafe { GetGlyphOutlineW(hdc, codepoint, GGO_GRAY8_BITMAP, &mut gm, 0, None, &mat2) };
    if buf_size == GDI_ERROR as u32 || buf_size == 0 {
        return None;
    }
    let len = usize::try_from(buf_size)
        .ok()
        .filter(|&len| len <= crate::icons::MAX_ICON_DECODE_BYTES)?;

    // Second call: get the grayscale bitmap
    let mut gray = vec![0u8; len];
    // SAFETY: `gray` holds the `buf_size` bytes passed as the buffer's size,
    // so GetGlyphOutlineW writes inside it.
    let written = unsafe {
        GetGlyphOutlineW(
            hdc,
            codepoint,
            GGO_GRAY8_BITMAP,
            &mut gm,
            buf_size,
            Some(gray.as_mut_ptr() as *mut _),
            &mat2,
        )
    };
    if written == GDI_ERROR as u32 {
        return None;
    }

    let glyph = GlyphBox {
        width: gm.gmBlackBoxX,
        height: gm.gmBlackBoxY,
        origin_x: gm.gmptGlyphOrigin.x,
        origin_y: gm.gmptGlyphOrigin.y,
    };
    Some((gray, glyph, em_ascent(tm.tmAscent, tm.tmInternalLeading)?))
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
/// A glyph is drawn on its font's `size × size` em box, at `size` pixels
/// em height, [`DEFAULT_ICON_SIZE`] where it is `None`, and in `fg_color`
/// where it is given, white otherwise; the stock and dialog icons are
/// full-colour at the system's large icon size and ignore both.
///
/// Returns `None` if the name doesn't match any known format or
/// the icon cannot be loaded on this system, and for a glyph whose `size`
/// is 0 or whose `size² × 4` bytes exceed the decode bound
/// ([`raster_size_in_bound`](crate::icons::raster_size_in_bound)).
#[must_use]
pub(crate) fn load_windows_icon_by_name(
    name: &str,
    size: Option<u32>,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    #[cfg(target_os = "windows")]
    {
        if name.starts_with("SIID_") {
            return load_stock_icon(name);
        }
        if name == "IDI_QUESTION" {
            return load_idi_icon();
        }
        let size = size.unwrap_or(DEFAULT_ICON_SIZE);
        // Try hex codepoint (e.g., "0xE8BB")
        if let Some(cp) = parse_hex_codepoint(name) {
            return load_glyph_icon(cp, size, fg_color);
        }
        // Try named glyph (e.g., "ChromeClose")
        if let Some(cp) = glyph_codepoint(name) {
            return load_glyph_icon(cp, size, fg_color);
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = (name, size, fg_color);
        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::{IconRole, IconSet, icon_name};

    /// Load a role's icon the way `IconLoader::load` does: by the name
    /// `icon_name` gives it in the Segoe set.
    fn load_role(role: IconRole) -> Option<IconData> {
        load_windows_icon_by_name(icon_name(role, IconSet::SegoeFluent)?, None, None)
    }

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

    /// A GGO_GRAY8_BITMAP buffer of `rows`, each padded to `pitch` bytes.
    fn gray_rows<R: AsRef<[u8]>>(rows: &[R], pitch: usize) -> Vec<u8> {
        rows.iter()
            .flat_map(|row| {
                let mut padded = row.as_ref().to_vec();
                padded.resize(pitch, 0);
                padded
            })
            .collect()
    }

    fn alpha_at(canvas: &[u8], size: usize, x: usize, y: usize) -> u8 {
        canvas[(y * size + x) * 4 + 3]
    }

    /// A bar and a square, glyphs of different ink, both fill the
    /// `size × size` em box.
    #[test]
    fn glyph_canvas_is_the_em_box_whatever_the_ink() {
        let bar = GlyphBox {
            width: 6,
            height: 1,
            origin_x: 1,
            origin_y: 4,
        };
        let square = GlyphBox {
            width: 4,
            height: 4,
            origin_x: 2,
            origin_y: 6,
        };
        let bar = glyph_canvas(&gray_rows(&[&[64; 6]], 8), bar, 7, 8).unwrap();
        let square = glyph_canvas(&gray_rows(&[&[64; 4]; 4], 4), square, 7, 8).unwrap();
        assert_eq!(bar.len(), 8 * 8 * 4);
        assert_eq!(square.len(), 8 * 8 * 4);
    }

    /// The black box lands at `(origin_x, ascent - origin_y)`, and the rest
    /// of the em box is transparent white.
    #[test]
    fn glyph_canvas_places_the_black_box_at_its_origin() {
        let glyph = GlyphBox {
            width: 2,
            height: 2,
            origin_x: 2,
            origin_y: 5,
        };
        let canvas = glyph_canvas(&gray_rows(&[&[64, 32], &[16, 0]], 4), glyph, 7, 8).unwrap();
        assert_eq!(alpha_at(&canvas, 8, 2, 2), 255);
        assert_eq!(alpha_at(&canvas, 8, 3, 2), 127);
        assert_eq!(alpha_at(&canvas, 8, 2, 3), 63);
        let ink = canvas.chunks(4).filter(|pixel| pixel[3] != 0).count();
        assert_eq!(ink, 3, "only the black box is drawn");
        assert!(canvas.chunks(4).all(|pixel| pixel[..3] == [255, 255, 255]));
    }

    /// The em box's top is the ascent less the internal leading: a glyph
    /// that fills the em of a font with internal leading fills the canvas,
    /// its top row on row 0 and its bottom row, on the descent, on the last.
    #[test]
    fn glyph_canvas_em_box_leaves_out_the_internal_leading() {
        // An 8 px em: ascent 9 with 2 of internal leading, descent 1, so the
        // cell is 10 high and the em box spans 7 above to 1 below the
        // baseline.
        let ascent = em_ascent(9, 2).unwrap();
        assert_eq!(ascent, 7);
        let glyph = GlyphBox {
            width: 8,
            height: 8,
            origin_x: 0,
            origin_y: 7,
        };
        let canvas = glyph_canvas(&gray_rows(&[&[64; 8]; 8], 8), glyph, ascent, 8).unwrap();
        assert!((0..8).all(|y| (0..8).all(|x| alpha_at(&canvas, 8, x, y) == 255)));
        assert_eq!(em_ascent(9, 0), Some(9), "no leading, the ascent");
        assert_eq!(em_ascent(i32::MIN, 1), None);
    }

    /// Ink outside the em box is clipped, on every side.
    #[test]
    fn glyph_canvas_clips_the_black_box() {
        let glyph = GlyphBox {
            width: 6,
            height: 7,
            origin_x: -1,
            origin_y: 5,
        };
        let canvas = glyph_canvas(&gray_rows(&[&[64; 6]; 7], 8), glyph, 3, 4).unwrap();
        assert_eq!(canvas.len(), 4 * 4 * 4);
        // Rows 2 to 5 and columns 1 to 4 of the box fill the canvas; rows 0,
        // 1 and 6 and columns 0 and 5 fall outside it.
        assert!((0..4).all(|y| (0..4).all(|x| alpha_at(&canvas, 4, x, y) == 255)));
        let far = GlyphBox {
            width: 2,
            height: 2,
            origin_x: i32::MAX,
            origin_y: i32::MIN,
        };
        let canvas = glyph_canvas(&gray_rows(&[&[64; 2]; 2], 4), far, i32::MAX, 4).unwrap();
        assert!(canvas.chunks(4).all(|pixel| pixel[3] == 0));
    }

    /// A buffer shorter than `pitch × height`, an empty black box and a size
    /// outside the decode bound give no canvas.
    #[test]
    fn glyph_canvas_rejects_what_it_cannot_draw() {
        let glyph = GlyphBox {
            width: 5,
            height: 2,
            origin_x: 0,
            origin_y: 0,
        };
        // pitch 8, so two rows need 16 bytes
        assert!(glyph_canvas(&[64; 15], glyph, 0, 8).is_none());
        assert!(glyph_canvas(&[64; 16], glyph, 0, 8).is_some());
        let empty = GlyphBox { width: 0, ..glyph };
        assert!(glyph_canvas(&[], empty, 0, 8).is_none());
        assert!(glyph_canvas(&[64; 16], glyph, 0, 0).is_none());
        assert!(glyph_canvas(&[64; 16], glyph, 0, 4097).is_none());
        let wide = GlyphBox {
            width: u32::MAX,
            ..glyph
        };
        assert!(glyph_canvas(&[64; 16], wide, 0, 8).is_none());
    }

    #[test]
    fn glyph_codepoint_lookup() {
        assert_eq!(glyph_codepoint("ChromeClose"), Some(0xE8BB));
        assert_eq!(glyph_codepoint("Unknown"), None);
    }

    /// Every action role is a Segoe Fluent glyph, drawn in the loader's
    /// colour, not a full-colour stock icon.
    #[test]
    fn action_roles_are_glyphs() {
        for (role, name, codepoint) in [
            (IconRole::ActionSearch, "Search", 0xE721),
            (IconRole::ActionSettings, "Settings", 0xE713),
            (IconRole::ActionDelete, "Delete", 0xE74D),
            (IconRole::ActionPrint, "Print", 0xE749),
        ] {
            assert_eq!(icon_name(role, IconSet::SegoeFluent), Some(name));
            assert_eq!(glyph_codepoint(name), Some(codepoint));
        }
    }

    #[test]
    fn unmapped_role_returns_none() {
        // StatusBusy has no Segoe mapping (known gap), should return None
        let result = load_role(IconRole::StatusBusy);
        assert!(
            result.is_none(),
            "StatusBusy should return None (no Segoe mapping, no fallback)"
        );
    }

    // === Windows-only integration tests ===

    #[cfg(target_os = "windows")]
    #[test]
    fn stock_icon_returns_rgba() {
        let result = load_role(IconRole::DialogWarning);
        assert!(result.is_some(), "DialogWarning should return an icon");
        if let Some(IconData::Rgba(_)) = result {
            // Stock icons should return RGBA
        } else {
            panic!("Expected IconData::Rgba for stock icon");
        }
    }

    /// Every stock icon role loads while every other one loads on another
    /// thread at the same moment, as tests and applications load them.
    #[cfg(target_os = "windows")]
    #[test]
    fn stock_icons_load_from_threads_at_once() {
        let roles: Vec<IconRole> = IconRole::ALL
            .iter()
            .copied()
            .filter(|&role| {
                icon_name(role, IconSet::SegoeFluent).is_some_and(|n| n.starts_with("SIID_"))
            })
            .collect();
        assert!(!roles.is_empty());
        let start = std::sync::Barrier::new(roles.len());
        std::thread::scope(|scope| {
            let loads: Vec<_> = roles
                .iter()
                .map(|&role| {
                    let start = &start;
                    scope.spawn(move || {
                        start.wait();
                        (role, load_role(role))
                    })
                })
                .collect();
            for load in loads {
                let (role, icon) = load.join().unwrap();
                assert!(
                    matches!(icon, Some(IconData::Rgba(_))),
                    "{role:?} should load as RGBA"
                );
            }
        });
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn glyph_icon_returns_rgba() {
        let result = load_role(IconRole::ActionCopy);
        assert!(result.is_some(), "ActionCopy should return an icon");
        if let Some(IconData::Rgba(_)) = result {
            // Font glyph icons should return RGBA
        } else {
            panic!("Expected IconData::Rgba for glyph icon");
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn rgba_dimensions_correct() {
        if let Some(IconData::Rgba(rgba)) = load_role(IconRole::DialogWarning) {
            assert_eq!(
                (rgba.width() * rgba.height() * 4) as usize,
                rgba.data().len(),
                "RGBA buffer size must equal width * height * 4"
            );
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn idi_question_returns_some() {
        let result = load_role(IconRole::DialogQuestion);
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
        let result = load_windows_icon_by_name("0xE8C8", None, None);
        assert!(
            result.is_some(),
            "hex codepoint 0xE8C8 should load Copy glyph"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_named_glyph() {
        let result = load_windows_icon_by_name("Copy", None, None);
        assert!(result.is_some(), "named glyph Copy should load");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_glyph_size_scales_the_raster() {
        let small = load_windows_icon_by_name("Copy", Some(16), None);
        let large = load_windows_icon_by_name("Copy", Some(64), None);
        let (Some(IconData::Rgba(small)), Some(IconData::Rgba(large))) = (small, large) else {
            panic!("Copy should load as RGBA at 16 and 64 px");
        };
        assert!(
            large.height() > small.height(),
            "64 px ({}) should be taller than 16 px ({})",
            large.height(),
            small.height()
        );
    }

    /// "Remove" (U+E738, a bar) and "ChromeMaximize" (U+E922, a square)
    /// have different ink; both come back as the 32 × 32 em box, each glyph
    /// where the font places it: the bar's inked rows are fewer than the
    /// square's and lie within the square's.
    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_glyphs_of_different_ink_share_the_em_box() {
        let inked_rows = |name: &str| {
            let Some(IconData::Rgba(rgba)) = load_windows_icon_by_name(name, Some(32), None) else {
                panic!("{name} should load as RGBA");
            };
            assert_eq!((rgba.width(), rgba.height()), (32, 32), "{name}");
            let rows: Vec<usize> = rgba
                .data()
                .chunks(32 * 4)
                .enumerate()
                .filter(|(_, row)| row.chunks(4).any(|pixel| pixel[3] != 0))
                .map(|(y, _)| y)
                .collect();
            eprintln!("{name}: inked rows {rows:?}");
            rows
        };
        let bar = inked_rows("Remove");
        let square = inked_rows("ChromeMaximize");
        assert!(!bar.is_empty(), "Remove has ink");
        assert!(
            bar.len() < square.len(),
            "the bar is shorter than the square"
        );
        let (Some(bar_top), Some(bar_bottom)) = (bar.first(), bar.last()) else {
            panic!("Remove has ink");
        };
        let (Some(square_top), Some(square_bottom)) = (square.first(), square.last()) else {
            panic!("ChromeMaximize has ink");
        };
        assert!(
            square_top <= bar_top && bar_bottom <= square_bottom,
            "the bar ({bar_top}..={bar_bottom}) lies within the square \
             ({square_top}..={square_bottom})"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_glyph_size_out_of_bound_returns_none() {
        assert!(load_windows_icon_by_name("Copy", Some(0), None).is_none());
        assert!(load_windows_icon_by_name("Copy", Some(u32::MAX), None).is_none());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_by_name_stock_icon() {
        let result = load_windows_icon_by_name("SIID_WARNING", None, None);
        assert!(result.is_some(), "SIID_WARNING stock icon should load");
    }
}
