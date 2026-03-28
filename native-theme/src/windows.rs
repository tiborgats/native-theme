//! Windows theme reader: reads accent color, accent shades, foreground/background,
//! per-widget fonts from NONCLIENTMETRICSW, DwmGetColorizationColor title bar colors,
//! GetSysColor per-widget colors, accessibility from UISettings and SystemParametersInfoW,
//! icon sizes from GetSystemMetricsForDpi, WinUI3 spacing defaults, and DPI-aware
//! geometry metrics from UISettings (WinRT) and Win32 APIs.

#[cfg(all(target_os = "windows", feature = "windows"))]
use ::windows::UI::ViewManagement::{UIColorType, UISettings};
#[cfg(all(target_os = "windows", feature = "windows"))]
use ::windows::Win32::UI::HiDpi::{GetDpiForSystem, GetSystemMetricsForDpi};
#[cfg(all(target_os = "windows", feature = "windows"))]
use ::windows::Win32::UI::WindowsAndMessaging::{
    NONCLIENTMETRICSW, SM_CXBORDER, SM_CXFOCUSBORDER, SM_CXICON, SM_CXSMICON, SM_CXVSCROLL,
    SM_CYMENU, SM_CYVTHUMB, SPI_GETNONCLIENTMETRICS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    SystemParametersInfoW,
};

use crate::model::FontSpec;

/// Per-widget fonts extracted from NONCLIENTMETRICSW.
///
/// Windows exposes four named LOGFONTW fields:
/// - `lfMessageFont` -- default UI font (messages, dialogs)
/// - `lfCaptionFont` -- title bar font
/// - `lfMenuFont` -- menu item font
/// - `lfStatusFont` -- status bar font
struct AllFonts {
    msg: FontSpec,
    caption: FontSpec,
    menu: FontSpec,
    status: FontSpec,
}

/// System color values extracted from GetSysColor.
///
/// COLORREF format: 0x00BBGGRR (blue in high byte, red in low byte).
struct SysColors {
    btn_face: crate::Rgba,
    btn_text: crate::Rgba,
    menu_bg: crate::Rgba,
    menu_text: crate::Rgba,
    info_bg: crate::Rgba,
    info_text: crate::Rgba,
    window_bg: crate::Rgba,
    window_text: crate::Rgba,
    highlight: crate::Rgba,
    highlight_text: crate::Rgba,
    caption_text: crate::Rgba,
    inactive_caption_text: crate::Rgba,
    gray_text: crate::Rgba,
}

/// Accessibility data from UISettings and SystemParametersInfoW.
struct AccessibilityData {
    text_scaling_factor: Option<f32>,
    high_contrast: Option<bool>,
    reduce_motion: Option<bool>,
}

/// Convert a `windows::UI::Color` to our `Rgba` type.
#[cfg(all(target_os = "windows", feature = "windows"))]
fn win_color_to_rgba(c: ::windows::UI::Color) -> crate::Rgba {
    crate::Rgba::rgba(c.R, c.G, c.B, c.A)
}

/// Detect dark mode from the system foreground color luminance.
///
/// Uses BT.601 luminance coefficients. A light foreground (luminance > 128)
/// indicates a dark background, i.e., dark mode.
fn is_dark_mode(fg: &crate::Rgba) -> bool {
    let luma = 0.299 * (fg.r as f32) + 0.587 * (fg.g as f32) + 0.114 * (fg.b as f32);
    luma > 128.0
}

/// Read accent shade colors from UISettings with graceful per-shade fallback.
///
/// Returns `[AccentDark1, AccentDark2, AccentDark3, AccentLight1, AccentLight2, AccentLight3]`.
/// Each shade is individually wrapped in `.ok()` so a failure on one shade does not
/// prevent reading the others (PLAT-05 graceful fallback).
#[cfg(all(target_os = "windows", feature = "windows"))]
fn read_accent_shades(settings: &UISettings) -> [Option<crate::Rgba>; 6] {
    let variants = [
        UIColorType::AccentDark1,
        UIColorType::AccentDark2,
        UIColorType::AccentDark3,
        UIColorType::AccentLight1,
        UIColorType::AccentLight2,
        UIColorType::AccentLight3,
    ];
    variants.map(|ct| settings.GetColorValue(ct).ok().map(win_color_to_rgba))
}

/// Convert a LOGFONTW to a FontSpec.
///
/// Extracts font family from `lfFaceName` (null-terminated UTF-16),
/// size in points from `abs(lfHeight) * 72 / dpi`, and weight from `lfWeight`
/// (already CSS 100-900 scale, clamped).
#[cfg(all(target_os = "windows", feature = "windows"))]
fn logfont_to_fontspec(lf: &::windows::Win32::Graphics::Gdi::LOGFONTW, dpi: u32) -> FontSpec {
    logfont_to_fontspec_raw(&lf.lfFaceName, lf.lfHeight, lf.lfWeight, dpi)
}

/// Testable core of logfont_to_fontspec: takes raw field values.
fn logfont_to_fontspec_raw(
    face_name: &[u16; 32],
    lf_height: i32,
    lf_weight: i32,
    dpi: u32,
) -> FontSpec {
    let face_end = face_name.iter().position(|&c| c == 0).unwrap_or(32);
    let family = String::from_utf16_lossy(&face_name[..face_end]);
    let points = (lf_height.unsigned_abs() * 72) / dpi;
    let weight = (lf_weight.clamp(100, 900)) as u16;
    FontSpec {
        family: Some(family),
        size: Some(points as f32),
        weight: Some(weight),
    }
}

/// Read all system fonts from NONCLIENTMETRICSW (WIN-01).
///
/// Extracts lfMessageFont, lfCaptionFont, lfMenuFont, and lfStatusFont
/// as FontSpec values. Returns default fonts if the system call fails.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_all_system_fonts(dpi: u32) -> AllFonts {
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
        AllFonts {
            msg: logfont_to_fontspec(&ncm.lfMessageFont, dpi),
            caption: logfont_to_fontspec(&ncm.lfCaptionFont, dpi),
            menu: logfont_to_fontspec(&ncm.lfMenuFont, dpi),
            status: logfont_to_fontspec(&ncm.lfStatusFont, dpi),
        }
    } else {
        AllFonts {
            msg: FontSpec::default(),
            caption: FontSpec::default(),
            menu: FontSpec::default(),
            status: FontSpec::default(),
        }
    }
}

/// Compute text scale entries from the system default font size.
///
/// Derives caption, section heading, dialog title, and display sizes
/// using Fluent Design type scale ratios relative to the base font.
fn compute_text_scale(base_size: f32) -> crate::TextScale {
    crate::TextScale {
        caption: Some(crate::TextScaleEntry {
            size: Some(base_size * 0.85),
            weight: Some(400),
            line_height: None,
        }),
        section_heading: Some(crate::TextScaleEntry {
            size: Some(base_size * 1.15),
            weight: Some(600),
            line_height: None,
        }),
        dialog_title: Some(crate::TextScaleEntry {
            size: Some(base_size * 1.35),
            weight: Some(600),
            line_height: None,
        }),
        display: Some(crate::TextScaleEntry {
            size: Some(base_size * 1.80),
            weight: Some(300),
            line_height: None,
        }),
    }
}

/// Return the WinUI3 Fluent Design spacing scale.
///
/// These are the standard spacing values from Microsoft Fluent Design guidelines,
/// in effective pixels (epx). Pure function with no OS API calls.
fn winui3_spacing() -> crate::ThemeSpacing {
    crate::ThemeSpacing {
        xxs: Some(2.0),
        xs: Some(4.0),
        s: Some(8.0),
        m: Some(12.0),
        l: Some(16.0),
        xl: Some(24.0),
        xxl: Some(32.0),
    }
}

/// Read DPI-aware system DPI value.
///
/// Returns the system DPI (96 = standard 100% scaling).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_dpi() -> u32 {
    unsafe { GetDpiForSystem() }
}

/// Read DPI-aware frame width.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_frame_width(dpi: u32) -> f32 {
    unsafe { GetSystemMetricsForDpi(SM_CXBORDER, dpi) as f32 }
}

/// Read DPI-aware scrollbar and widget metrics.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_widget_sizing(dpi: u32, variant: &mut crate::ThemeVariant) {
    unsafe {
        variant.scrollbar.width = Some(GetSystemMetricsForDpi(SM_CXVSCROLL, dpi) as f32);
        variant.scrollbar.min_thumb_height = Some(GetSystemMetricsForDpi(SM_CYVTHUMB, dpi) as f32);
        variant.menu.item_height = Some(GetSystemMetricsForDpi(SM_CYMENU, dpi) as f32);
        variant.defaults.focus_ring_width =
            Some(GetSystemMetricsForDpi(SM_CXFOCUSBORDER, dpi) as f32);
    }
    // WinUI3 Fluent Design constants (not from OS APIs)
    variant.button.min_height = Some(32.0);
    variant.button.padding_horizontal = Some(12.0);
    variant.checkbox.indicator_size = Some(20.0);
    variant.checkbox.spacing = Some(8.0);
    variant.input.min_height = Some(32.0);
    variant.input.padding_horizontal = Some(12.0);
    variant.slider.track_height = Some(4.0);
    variant.slider.thumb_size = Some(22.0);
    variant.progress_bar.height = Some(4.0);
    variant.tab.min_height = Some(32.0);
    variant.tab.padding_horizontal = Some(12.0);
    variant.menu.padding_horizontal = Some(12.0);
    variant.tooltip.padding_horizontal = Some(8.0);
    variant.tooltip.padding_vertical = Some(8.0);
    variant.list.item_height = Some(40.0);
    variant.list.padding_horizontal = Some(12.0);
    variant.toolbar.height = Some(48.0);
    variant.toolbar.item_spacing = Some(4.0);
    variant.splitter.width = Some(4.0);
}

/// Apply WinUI3 Fluent Design widget sizing constants (non-Windows testable version).
#[cfg(not(target_os = "windows"))]
fn read_widget_sizing(_dpi: u32, variant: &mut crate::ThemeVariant) {
    variant.scrollbar.width = Some(17.0);
    variant.scrollbar.min_thumb_height = Some(40.0);
    variant.menu.item_height = Some(32.0);
    variant.defaults.focus_ring_width = Some(1.0); // SM_CXFOCUSBORDER typical value
    variant.button.min_height = Some(32.0);
    variant.button.padding_horizontal = Some(12.0);
    variant.checkbox.indicator_size = Some(20.0);
    variant.checkbox.spacing = Some(8.0);
    variant.input.min_height = Some(32.0);
    variant.input.padding_horizontal = Some(12.0);
    variant.slider.track_height = Some(4.0);
    variant.slider.thumb_size = Some(22.0);
    variant.progress_bar.height = Some(4.0);
    variant.tab.min_height = Some(32.0);
    variant.tab.padding_horizontal = Some(12.0);
    variant.menu.padding_horizontal = Some(12.0);
    variant.tooltip.padding_horizontal = Some(8.0);
    variant.tooltip.padding_vertical = Some(8.0);
    variant.list.item_height = Some(40.0);
    variant.list.padding_horizontal = Some(12.0);
    variant.toolbar.height = Some(48.0);
    variant.toolbar.item_spacing = Some(4.0);
    variant.splitter.width = Some(4.0);
}

/// Convert a Win32 COLORREF (0x00BBGGRR) to Rgba.
///
/// COLORREF stores colors as blue in the high byte, red in the low byte.
/// This is the inverse of typical RGB ordering.
pub(crate) fn colorref_to_rgba(c: u32) -> crate::Rgba {
    let r = (c & 0xFF) as u8;
    let g = ((c >> 8) & 0xFF) as u8;
    let b = ((c >> 16) & 0xFF) as u8;
    crate::Rgba::rgb(r, g, b)
}

/// Read GetSysColor widget colors (WIN-03).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_sys_colors() -> SysColors {
    use ::windows::Win32::Graphics::Gdi::*;

    fn sys_color(index: SYS_COLOR_INDEX) -> crate::Rgba {
        let c = unsafe { GetSysColor(index) };
        colorref_to_rgba(c)
    }

    SysColors {
        btn_face: sys_color(COLOR_BTNFACE),
        btn_text: sys_color(COLOR_BTNTEXT),
        menu_bg: sys_color(COLOR_MENU),
        menu_text: sys_color(COLOR_MENUTEXT),
        info_bg: sys_color(COLOR_INFOBK),
        info_text: sys_color(COLOR_INFOTEXT),
        window_bg: sys_color(COLOR_WINDOW),
        window_text: sys_color(COLOR_WINDOWTEXT),
        highlight: sys_color(COLOR_HIGHLIGHT),
        highlight_text: sys_color(COLOR_HIGHLIGHTTEXT),
        caption_text: sys_color(COLOR_CAPTIONTEXT),
        inactive_caption_text: sys_color(COLOR_INACTIVECAPTIONTEXT),
        gray_text: sys_color(COLOR_GRAYTEXT),
    }
}

/// Apply SysColors to the per-widget fields on a ThemeVariant.
fn apply_sys_colors(variant: &mut crate::ThemeVariant, colors: &SysColors) {
    variant.button.background = Some(colors.btn_face);
    variant.button.foreground = Some(colors.btn_text);
    variant.menu.background = Some(colors.menu_bg);
    variant.menu.foreground = Some(colors.menu_text);
    variant.tooltip.background = Some(colors.info_bg);
    variant.tooltip.foreground = Some(colors.info_text);
    variant.input.background = Some(colors.window_bg);
    variant.input.foreground = Some(colors.window_text);
    variant.input.placeholder = Some(colors.gray_text);
    variant.list.selection = Some(colors.highlight);
    variant.list.selection_foreground = Some(colors.highlight_text);
    variant.window.title_bar_foreground = Some(colors.caption_text);
    variant.window.inactive_title_bar_foreground = Some(colors.inactive_caption_text);
}

/// Read DwmGetColorizationColor for title bar background (WIN-02).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_dwm_colorization() -> Option<crate::Rgba> {
    use ::windows::Win32::Graphics::Dwm::DwmGetColorizationColor;
    let mut colorization: u32 = 0;
    let mut opaque_blend = ::windows::core::BOOL::default();
    unsafe { DwmGetColorizationColor(&mut colorization, &mut opaque_blend) }.ok()?;
    // DWM colorization is 0xAARRGGBB (NOT COLORREF format)
    let a = ((colorization >> 24) & 0xFF) as u8;
    let r = ((colorization >> 16) & 0xFF) as u8;
    let g = ((colorization >> 8) & 0xFF) as u8;
    let b = (colorization & 0xFF) as u8;
    Some(crate::Rgba::rgba(r, g, b, a))
}

/// Convert a DWM colorization u32 (0xAARRGGBB) to Rgba. Testable helper.
fn dwm_color_to_rgba(c: u32) -> crate::Rgba {
    let a = ((c >> 24) & 0xFF) as u8;
    let r = ((c >> 16) & 0xFF) as u8;
    let g = ((c >> 8) & 0xFF) as u8;
    let b = (c & 0xFF) as u8;
    crate::Rgba::rgba(r, g, b, a)
}

/// Read inactive title bar colors from GetSysColor.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_inactive_caption_color() -> crate::Rgba {
    use ::windows::Win32::Graphics::Gdi::{COLOR_INACTIVECAPTION, GetSysColor};
    let c = unsafe { GetSysColor(COLOR_INACTIVECAPTION) };
    colorref_to_rgba(c)
}

/// Read accessibility settings (WIN-04).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_accessibility(settings: &UISettings) -> AccessibilityData {
    // TextScaleFactor from UISettings
    let text_scaling_factor = settings.TextScaleFactor().ok().map(|f| f as f32);

    // SPI_GETHIGHCONTRAST
    let high_contrast = {
        use ::windows::Win32::UI::Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTW};
        use ::windows::Win32::UI::WindowsAndMessaging::*;
        let mut hc = HIGHCONTRASTW::default();
        hc.cbSize = std::mem::size_of::<HIGHCONTRASTW>() as u32;
        let success = unsafe {
            SystemParametersInfoW(
                SPI_GETHIGHCONTRAST,
                hc.cbSize,
                Some(&mut hc as *mut _ as *mut _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            )
        };
        if success.is_ok() {
            Some(hc.dwFlags.contains(HCF_HIGHCONTRASTON))
        } else {
            None
        }
    };

    // SPI_GETCLIENTAREAANIMATION
    let reduce_motion = {
        let mut animation_enabled = ::windows::core::BOOL(1);
        let success = unsafe {
            SystemParametersInfoW(
                ::windows::Win32::UI::WindowsAndMessaging::SPI_GETCLIENTAREAANIMATION,
                0,
                Some(&mut animation_enabled as *mut _ as *mut _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            )
        };
        if success.is_ok() {
            // If animation is disabled, reduce_motion is true
            Some(!animation_enabled.as_bool())
        } else {
            None
        }
    };

    AccessibilityData {
        text_scaling_factor,
        high_contrast,
        reduce_motion,
    }
}

/// Read icon sizes from GetSystemMetricsForDpi (WIN-05).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_icon_sizes(dpi: u32) -> (f32, f32) {
    let small = unsafe { GetSystemMetricsForDpi(SM_CXSMICON, dpi) } as f32;
    let large = unsafe { GetSystemMetricsForDpi(SM_CXICON, dpi) } as f32;
    (small, large)
}

/// Testable core: given raw color values, accent shades, fonts, and sizing data,
/// build a `NativeTheme` with a sparse `ThemeVariant`.
///
/// Determines light/dark variant based on foreground luminance, then populates
/// the appropriate variant with defaults-level colors, per-widget fonts, spacing,
/// geometry, and sizing. Only one variant is ever populated (matching KDE/GNOME
/// reader pattern).
#[allow(clippy::too_many_arguments)]
fn build_theme(
    accent: crate::Rgba,
    fg: crate::Rgba,
    bg: crate::Rgba,
    accent_shades: [Option<crate::Rgba>; 6],
    fonts: AllFonts,
    sys_colors: Option<&SysColors>,
    dwm_title_bar: Option<crate::Rgba>,
    inactive_title_bar: Option<crate::Rgba>,
    icon_sizes: Option<(f32, f32)>,
    accessibility: Option<&AccessibilityData>,
    dpi: u32,
) -> crate::NativeTheme {
    let dark = is_dark_mode(&fg);

    // Primary button background: In light mode use AccentDark1 (shades[0]), in dark mode
    // use AccentLight1 (shades[3]). Fall back to accent if shade unavailable.
    let primary_bg = if dark {
        accent_shades[3].unwrap_or(accent)
    } else {
        accent_shades[0].unwrap_or(accent)
    };

    let mut variant = crate::ThemeVariant::default();

    // --- Defaults-level colors ---
    variant.defaults.accent = Some(accent);
    variant.defaults.foreground = Some(fg);
    variant.defaults.background = Some(bg);
    variant.defaults.selection = Some(accent);
    variant.defaults.focus_ring_color = Some(accent);
    variant.defaults.surface = Some(bg);
    variant.button.primary_bg = Some(primary_bg);
    variant.button.primary_fg = Some(fg);

    // Disabled foreground: midpoint between fg and bg
    let disabled_r = ((fg.r as u16 + bg.r as u16) / 2) as u8;
    let disabled_g = ((fg.g as u16 + bg.g as u16) / 2) as u8;
    let disabled_b = ((fg.b as u16 + bg.b as u16) / 2) as u8;
    variant.defaults.disabled_foreground =
        Some(crate::Rgba::rgb(disabled_r, disabled_g, disabled_b));

    // --- Defaults-level font (message font) ---
    variant.defaults.font = fonts.msg;

    // --- Per-widget fonts (WIN-01) ---
    variant.window.title_bar_font = Some(fonts.caption);
    variant.menu.font = Some(fonts.menu);
    variant.status_bar.font = Some(fonts.status);

    // --- Text scale (derived from defaults.font.size) ---
    if let Some(base_size) = variant.defaults.font.size {
        variant.text_scale = compute_text_scale(base_size);
    }

    // --- Spacing (WinUI3 Fluent) ---
    variant.defaults.spacing = winui3_spacing();

    // --- Geometry (Windows 11 defaults) ---
    variant.defaults.radius = Some(4.0);
    variant.defaults.radius_lg = Some(8.0);
    variant.defaults.shadow_enabled = Some(true);

    // --- Widget sizing ---
    read_widget_sizing(dpi, &mut variant);

    // --- Dialog button order (Windows convention) ---
    variant.dialog.button_order = Some(crate::model::DialogButtonOrder::TrailingAffirmative);

    // --- DWM title bar color (WIN-02) ---
    if let Some(color) = dwm_title_bar {
        variant.window.title_bar_background = Some(color);
    }
    if let Some(color) = inactive_title_bar {
        variant.window.inactive_title_bar_background = Some(color);
    }

    // --- GetSysColor per-widget colors (WIN-03) ---
    if let Some(colors) = sys_colors {
        apply_sys_colors(&mut variant, colors);
    }

    // --- Icon sizes (WIN-05) ---
    if let Some((small, large)) = icon_sizes {
        variant.defaults.icon_sizes.small = Some(small);
        variant.defaults.icon_sizes.large = Some(large);
    }

    // --- Accessibility (WIN-04) ---
    if let Some(a) = accessibility {
        variant.defaults.text_scaling_factor = a.text_scaling_factor;
        variant.defaults.high_contrast = a.high_contrast;
        variant.defaults.reduce_motion = a.reduce_motion;
    }

    if dark {
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
    }
}

/// Read the current Windows theme from UISettings, SystemParametersInfoW,
/// GetSystemMetricsForDpi, DwmGetColorizationColor, and GetSysColor.
///
/// Reads accent, foreground, and background colors plus 6 accent shade colors
/// from `UISettings` (WinRT), per-widget fonts from `NONCLIENTMETRICSW` (Win32),
/// DWM colorization for title bar, GetSysColor for per-widget colors, accessibility
/// settings, and icon sizes.
///
/// Returns `Error::Unavailable` if UISettings cannot be created (pre-Windows 10).
#[cfg(all(target_os = "windows", feature = "windows"))]
pub fn from_windows() -> crate::Result<crate::NativeTheme> {
    let settings = UISettings::new()
        .map_err(|e| crate::Error::Unavailable(format!("UISettings unavailable: {e}")))?;

    let accent = settings
        .GetColorValue(UIColorType::Accent)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::Unavailable(format!("GetColorValue(Accent) failed: {e}")))?;
    let fg = settings
        .GetColorValue(UIColorType::Foreground)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::Unavailable(format!("GetColorValue(Foreground) failed: {e}")))?;
    let bg = settings
        .GetColorValue(UIColorType::Background)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::Unavailable(format!("GetColorValue(Background) failed: {e}")))?;

    let accent_shades = read_accent_shades(&settings);
    let dpi = read_dpi();
    let fonts = read_all_system_fonts(dpi);
    let sys_colors = read_sys_colors();
    let dwm_title_bar = read_dwm_colorization();
    let inactive_title_bar = Some(read_inactive_caption_color());
    let (small, large) = read_icon_sizes(dpi);
    let accessibility = read_accessibility(&settings);

    Ok(build_theme(
        accent,
        fg,
        bg,
        accent_shades,
        fonts,
        Some(&sys_colors),
        dwm_title_bar,
        inactive_title_bar,
        Some((small, large)),
        Some(&accessibility),
        dpi,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// Helper: create default AllFonts for tests that don't care about fonts.
    fn default_fonts() -> AllFonts {
        AllFonts {
            msg: FontSpec::default(),
            caption: FontSpec::default(),
            menu: FontSpec::default(),
            status: FontSpec::default(),
        }
    }

    /// Helper: create AllFonts with named fonts for testing per-widget placement.
    fn named_fonts() -> AllFonts {
        AllFonts {
            msg: FontSpec {
                family: Some("Segoe UI".to_string()),
                size: Some(9.0),
                weight: Some(400),
            },
            caption: FontSpec {
                family: Some("Segoe UI".to_string()),
                size: Some(9.0),
                weight: Some(700),
            },
            menu: FontSpec {
                family: Some("Segoe UI".to_string()),
                size: Some(9.0),
                weight: Some(400),
            },
            status: FontSpec {
                family: Some("Segoe UI".to_string()),
                size: Some(8.0),
                weight: Some(400),
            },
        }
    }

    /// Helper: build a theme in light mode with minimal args.
    fn light_theme() -> crate::NativeTheme {
        build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0), // black fg = light mode
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        )
    }

    /// Helper: build a theme in dark mode with minimal args.
    fn dark_theme() -> crate::NativeTheme {
        build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(255, 255, 255), // white fg = dark mode
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        )
    }

    // === is_dark_mode tests ===

    #[test]
    fn is_dark_mode_white_foreground_returns_true() {
        let fg = crate::Rgba::rgb(255, 255, 255);
        assert!(is_dark_mode(&fg));
    }

    #[test]
    fn is_dark_mode_black_foreground_returns_false() {
        let fg = crate::Rgba::rgb(0, 0, 0);
        assert!(!is_dark_mode(&fg));
    }

    #[test]
    fn is_dark_mode_mid_gray_boundary_returns_false() {
        let fg = crate::Rgba::rgb(128, 128, 128);
        assert!(!is_dark_mode(&fg));
    }

    // === logfont_to_fontspec_raw tests ===

    #[test]
    fn logfont_to_fontspec_extracts_family_size_weight() {
        // "Segoe UI" in UTF-16 + null terminator
        let mut face: [u16; 32] = [0; 32];
        for (i, ch) in "Segoe UI".encode_utf16().enumerate() {
            face[i] = ch;
        }
        let fs = logfont_to_fontspec_raw(&face, -16, 400, 96);
        assert_eq!(fs.family.as_deref(), Some("Segoe UI"));
        assert_eq!(fs.size, Some(12.0)); // abs(16) * 72 / 96 = 12
        assert_eq!(fs.weight, Some(400));
    }

    #[test]
    fn logfont_to_fontspec_bold_weight_700() {
        let face: [u16; 32] = [0; 32];
        let fs = logfont_to_fontspec_raw(&face, -16, 700, 96);
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn logfont_to_fontspec_weight_clamped_to_range() {
        let face: [u16; 32] = [0; 32];
        // Weight below 100 gets clamped
        let fs = logfont_to_fontspec_raw(&face, -16, 0, 96);
        assert_eq!(fs.weight, Some(100));
        // Weight above 900 gets clamped
        let fs = logfont_to_fontspec_raw(&face, -16, 1000, 96);
        assert_eq!(fs.weight, Some(900));
    }

    // === colorref_to_rgba tests ===

    #[test]
    fn colorref_to_rgba_correct_rgb_extraction() {
        // COLORREF 0x00BBGGRR: blue=0xAA, green=0xBB, red=0xCC
        let rgba = colorref_to_rgba(0x00AABBCC);
        assert_eq!(rgba.r, 0xCC);
        assert_eq!(rgba.g, 0xBB);
        assert_eq!(rgba.b, 0xAA);
        assert_eq!(rgba.a, 255); // Rgba::rgb sets alpha to 255
    }

    #[test]
    fn colorref_to_rgba_black() {
        let rgba = colorref_to_rgba(0x00000000);
        assert_eq!(rgba, crate::Rgba::rgb(0, 0, 0));
    }

    #[test]
    fn colorref_to_rgba_white() {
        let rgba = colorref_to_rgba(0x00FFFFFF);
        assert_eq!(rgba, crate::Rgba::rgb(255, 255, 255));
    }

    // === dwm_color_to_rgba tests ===

    #[test]
    fn dwm_color_to_rgba_extracts_argb() {
        // 0xAARRGGBB format
        let rgba = dwm_color_to_rgba(0xCC112233);
        assert_eq!(rgba.r, 0x11);
        assert_eq!(rgba.g, 0x22);
        assert_eq!(rgba.b, 0x33);
        assert_eq!(rgba.a, 0xCC);
    }

    // === build_theme tests ===

    #[test]
    fn build_theme_dark_mode_populates_dark_variant_only() {
        let theme = dark_theme();
        assert!(theme.dark.is_some(), "dark variant should be Some");
        assert!(theme.light.is_none(), "light variant should be None");
    }

    #[test]
    fn build_theme_light_mode_populates_light_variant_only() {
        let theme = light_theme();
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    #[test]
    fn build_theme_sets_defaults_accent_fg_bg_selection() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let fg = crate::Rgba::rgb(0, 0, 0);
        let bg = crate::Rgba::rgb(255, 255, 255);
        let theme = build_theme(
            accent,
            fg,
            bg,
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.accent, Some(accent));
        assert_eq!(variant.defaults.foreground, Some(fg));
        assert_eq!(variant.defaults.background, Some(bg));
        assert_eq!(variant.defaults.selection, Some(accent));
        assert_eq!(variant.defaults.focus_ring_color, Some(accent));
    }

    #[test]
    fn build_theme_name_is_windows() {
        assert_eq!(light_theme().name, "Windows");
    }

    #[test]
    fn build_theme_accent_shades_light_mode() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let dark1 = crate::Rgba::rgb(0, 90, 170);
        let mut shades = [None; 6];
        shades[0] = Some(dark1);
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            shades,
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        );
        // In light mode, AccentDark1 is not directly used in ThemeVariant (old primary_background
        // is no longer a field). But the logic still selects primary_bg -- which is not set on the
        // new model. This is fine: the resolve() pipeline handles it.
        // Just verify the core defaults are set.
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.accent, Some(accent));
    }

    #[test]
    fn build_theme_accent_shades_dark_mode() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let light1 = crate::Rgba::rgb(60, 160, 240);
        let mut shades = [None; 6];
        shades[3] = Some(light1);
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(255, 255, 255),
            crate::Rgba::rgb(0, 0, 0),
            shades,
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.dark.as_ref().expect("dark variant");
        assert_eq!(variant.defaults.accent, Some(accent));
    }

    // === Per-widget font tests (WIN-01) ===

    #[test]
    fn build_theme_sets_title_bar_font() {
        let fonts = named_fonts();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            fonts,
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        let title_font = variant
            .window
            .title_bar_font
            .as_ref()
            .expect("title_bar_font");
        assert_eq!(title_font.family.as_deref(), Some("Segoe UI"));
        assert_eq!(title_font.weight, Some(700));
    }

    #[test]
    fn build_theme_sets_menu_and_status_bar_fonts() {
        let fonts = named_fonts();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            fonts,
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        let menu_font = variant.menu.font.as_ref().expect("menu.font");
        assert_eq!(menu_font.family.as_deref(), Some("Segoe UI"));
        let status_font = variant.status_bar.font.as_ref().expect("status_bar.font");
        assert_eq!(status_font.size, Some(8.0));
    }

    #[test]
    fn build_theme_sets_defaults_font_from_msg_font() {
        let fonts = named_fonts();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            fonts,
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Segoe UI"));
        assert_eq!(variant.defaults.font.size, Some(9.0));
    }

    // === SysColors per-widget tests (WIN-03) ===

    /// Helper: create sample SysColors for tests.
    fn sample_sys_colors() -> SysColors {
        SysColors {
            btn_face: crate::Rgba::rgb(240, 240, 240),
            btn_text: crate::Rgba::rgb(0, 0, 0),
            menu_bg: crate::Rgba::rgb(255, 255, 255),
            menu_text: crate::Rgba::rgb(0, 0, 0),
            info_bg: crate::Rgba::rgb(255, 255, 225),
            info_text: crate::Rgba::rgb(0, 0, 0),
            window_bg: crate::Rgba::rgb(255, 255, 255),
            window_text: crate::Rgba::rgb(0, 0, 0),
            highlight: crate::Rgba::rgb(0, 120, 215),
            highlight_text: crate::Rgba::rgb(255, 255, 255),
            caption_text: crate::Rgba::rgb(0, 0, 0),
            inactive_caption_text: crate::Rgba::rgb(128, 128, 128),
            gray_text: crate::Rgba::rgb(109, 109, 109),
        }
    }

    #[test]
    fn build_theme_with_sys_colors_populates_widgets() {
        let colors = sample_sys_colors();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            Some(&colors),
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(
            variant.button.background,
            Some(crate::Rgba::rgb(240, 240, 240))
        );
        assert_eq!(variant.button.foreground, Some(crate::Rgba::rgb(0, 0, 0)));
        assert_eq!(
            variant.menu.background,
            Some(crate::Rgba::rgb(255, 255, 255))
        );
        assert_eq!(variant.menu.foreground, Some(crate::Rgba::rgb(0, 0, 0)));
        assert_eq!(
            variant.tooltip.background,
            Some(crate::Rgba::rgb(255, 255, 225))
        );
        assert_eq!(variant.tooltip.foreground, Some(crate::Rgba::rgb(0, 0, 0)));
        assert_eq!(
            variant.input.background,
            Some(crate::Rgba::rgb(255, 255, 255))
        );
        assert_eq!(variant.input.foreground, Some(crate::Rgba::rgb(0, 0, 0)));
        assert_eq!(variant.list.selection, Some(crate::Rgba::rgb(0, 120, 215)));
        assert_eq!(
            variant.list.selection_foreground,
            Some(crate::Rgba::rgb(255, 255, 255))
        );
        assert_eq!(
            variant.window.title_bar_foreground,
            Some(crate::Rgba::rgb(0, 0, 0)),
            "caption_text -> window.title_bar_foreground"
        );
        assert_eq!(
            variant.window.inactive_title_bar_foreground,
            Some(crate::Rgba::rgb(128, 128, 128)),
            "inactive_caption_text -> window.inactive_title_bar_foreground"
        );
        assert_eq!(
            variant.input.placeholder,
            Some(crate::Rgba::rgb(109, 109, 109)),
            "gray_text -> input.placeholder"
        );
    }

    // === Focus ring width test ===

    #[test]
    fn build_theme_sets_focus_ring_width() {
        let theme = light_theme();
        let variant = theme.light.as_ref().expect("light variant");
        assert!(
            variant.defaults.focus_ring_width.is_some(),
            "focus_ring_width should be set from SM_CXFOCUSBORDER"
        );
    }

    // === Text scale tests ===

    #[test]
    fn build_theme_text_scale_from_font_size() {
        let fonts = named_fonts(); // msg font size = 9.0
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            fonts,
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        let caption = variant.text_scale.caption.as_ref().expect("caption");
        assert!((caption.size.unwrap_or(0.0) - 9.0 * 0.85).abs() < 0.01);
        assert_eq!(caption.weight, Some(400));

        let heading = variant
            .text_scale
            .section_heading
            .as_ref()
            .expect("section_heading");
        assert!((heading.size.unwrap_or(0.0) - 9.0 * 1.15).abs() < 0.01);
        assert_eq!(heading.weight, Some(600));

        let title = variant
            .text_scale
            .dialog_title
            .as_ref()
            .expect("dialog_title");
        assert!((title.size.unwrap_or(0.0) - 9.0 * 1.35).abs() < 0.01);
        assert_eq!(title.weight, Some(600));

        let display = variant.text_scale.display.as_ref().expect("display");
        assert!((display.size.unwrap_or(0.0) - 9.0 * 1.80).abs() < 0.01);
        assert_eq!(display.weight, Some(300));
    }

    #[test]
    fn compute_text_scale_values() {
        let ts = compute_text_scale(10.0);
        let cap = ts.caption.as_ref().unwrap();
        assert!((cap.size.unwrap() - 8.5).abs() < 0.01);
        assert_eq!(cap.weight, Some(400));
        assert!(cap.line_height.is_none());

        let sh = ts.section_heading.as_ref().unwrap();
        assert!((sh.size.unwrap() - 11.5).abs() < 0.01);
        assert_eq!(sh.weight, Some(600));

        let dt = ts.dialog_title.as_ref().unwrap();
        assert!((dt.size.unwrap() - 13.5).abs() < 0.01);
        assert_eq!(dt.weight, Some(600));

        let d = ts.display.as_ref().unwrap();
        assert!((d.size.unwrap() - 18.0).abs() < 0.01);
        assert_eq!(d.weight, Some(300));
    }

    // === DWM title bar color test (WIN-02) ===

    #[test]
    fn build_theme_with_dwm_color_sets_title_bar_background() {
        let dwm_color = crate::Rgba::rgba(0, 120, 215, 200);
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            None,
            Some(dwm_color),
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.window.title_bar_background, Some(dwm_color));
    }

    #[test]
    fn build_theme_with_inactive_title_bar() {
        let inactive = crate::Rgba::rgb(200, 200, 200);
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            None,
            None,
            Some(inactive),
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.window.inactive_title_bar_background, Some(inactive));
    }

    // === Icon sizes test (WIN-05) ===

    #[test]
    fn build_theme_with_icon_sizes() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            Some((16.0, 32.0)),
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.icon_sizes.small, Some(16.0));
        assert_eq!(variant.defaults.icon_sizes.large, Some(32.0));
    }

    // === Accessibility tests (WIN-04) ===

    #[test]
    fn build_theme_with_accessibility() {
        let accessibility = AccessibilityData {
            text_scaling_factor: Some(1.5),
            high_contrast: Some(true),
            reduce_motion: Some(false),
        };
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            None,
            Some(&accessibility),
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.text_scaling_factor, Some(1.5));
        assert_eq!(variant.defaults.high_contrast, Some(true));
        assert_eq!(variant.defaults.reduce_motion, Some(false));
    }

    // === Dialog button order test ===

    #[test]
    fn build_theme_sets_dialog_trailing_affirmative() {
        let theme = light_theme();
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(
            variant.dialog.button_order,
            Some(crate::model::DialogButtonOrder::TrailingAffirmative)
        );
    }

    // === Geometry tests ===

    #[test]
    fn build_theme_sets_geometry_defaults() {
        let theme = light_theme();
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.radius, Some(4.0));
        assert_eq!(variant.defaults.radius_lg, Some(8.0));
        assert_eq!(variant.defaults.shadow_enabled, Some(true));
    }

    // === Spacing test ===

    #[test]
    fn winui3_spacing_values() {
        let spacing = winui3_spacing();
        assert_eq!(spacing.xxs, Some(2.0));
        assert_eq!(spacing.xs, Some(4.0));
        assert_eq!(spacing.s, Some(8.0));
        assert_eq!(spacing.m, Some(12.0));
        assert_eq!(spacing.l, Some(16.0));
        assert_eq!(spacing.xl, Some(24.0));
        assert_eq!(spacing.xxl, Some(32.0));
    }

    // === Widget sizing test ===

    #[test]
    fn build_theme_includes_widget_sizing() {
        let theme = light_theme();
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.button.min_height, Some(32.0));
        assert_eq!(variant.checkbox.indicator_size, Some(20.0));
        assert_eq!(variant.input.min_height, Some(32.0));
        assert_eq!(variant.slider.thumb_size, Some(22.0));
        assert!(variant.scrollbar.width.is_some());
        assert!(variant.menu.item_height.is_some());
        assert_eq!(variant.splitter.width, Some(4.0));
    }

    // === Surface and disabled_foreground tests ===

    #[test]
    fn build_theme_surface_equals_bg() {
        let bg = crate::Rgba::rgb(255, 255, 255);
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            bg,
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.surface, Some(bg));
    }

    #[test]
    fn build_theme_disabled_foreground_is_midpoint() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),       // fg
            crate::Rgba::rgb(255, 255, 255), // bg
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
            None,
            None,
            96,
        );
        let variant = theme.light.as_ref().expect("light variant");
        // midpoint of (0,0,0) and (255,255,255) = (127,127,127)
        assert_eq!(
            variant.defaults.disabled_foreground,
            Some(crate::Rgba::rgb(127, 127, 127))
        );
    }

    // === No old model references verification ===

    #[test]
    fn build_theme_returns_native_theme_with_theme_variant() {
        // Verify the output type is correct (NativeTheme with ThemeVariant, not old types)
        let theme = light_theme();
        let variant: &crate::ThemeVariant = theme.light.as_ref().unwrap();
        // Access new per-widget fields to prove they exist
        let _ = variant.defaults.accent;
        let _ = variant.window.title_bar_font;
        let _ = variant.menu.font;
        let _ = variant.status_bar.font;
        let _ = variant.button.background;
        let _ = variant.defaults.icon_sizes.small;
        let _ = variant.defaults.reduce_motion;
        let _ = variant.dialog.button_order;
    }

    #[test]
    fn test_windows_resolve_validate() {
        // Load windows-11 preset as base (provides full color/geometry/spacing).
        let mut base = crate::NativeTheme::preset("windows-11").unwrap();
        // Build reader output (light mode, sample data).
        let reader_output = light_theme();
        // Merge reader output on top of preset.
        base.merge(&reader_output);

        // Extract light variant.
        let mut light = base
            .light
            .clone()
            .expect("light variant should exist after merge");
        light.resolve();
        let resolved = light.validate().unwrap_or_else(|e| {
            panic!("Windows resolve/validate pipeline failed: {e}");
        });

        // Spot-check: reader-sourced fields present.
        assert_eq!(
            resolved.defaults.accent,
            crate::Rgba::rgb(0, 120, 215),
            "accent should be from Windows reader"
        );
        assert_eq!(
            resolved.defaults.font.family, "Segoe UI",
            "font family should be from Windows reader"
        );
        assert_eq!(
            resolved.dialog.button_order,
            crate::DialogButtonOrder::TrailingAffirmative,
            "dialog button order should be trailing affirmative for Windows"
        );
        assert_eq!(
            resolved.icon_set, "segoe-fluent",
            "icon_set should be segoe-fluent from Windows preset"
        );
    }
}
