//! Windows theme reader: reads light or dark, the accent color and its shades,
//! per-widget fonts from NONCLIENTMETRICSW, accessibility from UISettings and
//! SystemParametersInfoW, icon sizes from GetSystemMetricsForDpi, and the
//! scrollbar and border metrics. Outside high contrast it writes no other
//! colour, as the `windows-11` preset states what WinUI 3 draws; under high
//! contrast it writes the GetSysColor colours WinUI 3's HighContrast theme
//! dictionaries draw with (see `apply_sys_colors`). Every system metric is read
//! in logical pixels (see `logical_system_metric`), and the fonts are read at
//! 96 DPI and reported with a `font_dpi` of 96, so their points resolve to
//! logical pixels too (see `LOGICAL_DPI`).
//!
//! # Sources
//!
//! What the reader reads that the user's settings change and a WinRT event
//! reports; the watcher subscribes to each event:
//!
//! - `UISettings.GetColorValue`: the accent colour and its shades, and the
//!   foreground, which tells light from dark (`UISettings::ColorValuesChanged`).
//! - `UISettings.TextScaleFactor`: the text scale
//!   (`UISettings::TextScaleFactorChanged`).
//! - `SPI_GETHIGHCONTRAST`: high contrast
//!   (`AccessibilitySettings::HighContrastChanged`, "the system high
//!   contrast feature turns on or off").
//! - `SPI_GETCLIENTAREAANIMATION`: reduced motion
//!   (`UISettings::AnimationsEnabledChanged`, Windows 10 2004 and later).
//!   No Microsoft page says `UISettings.AnimationsEnabled` is the same
//!   setting; it read the same as `SPI_GETCLIENTAREAANIMATION` on the three
//!   Windows probe images (`docs/platform-facts.md`, `reduce_motion`).
//! - `UISettings.AdvancedEffectsEnabled`: reduced transparency, the
//!   *Transparency effects* setting turned off
//!   (`UISettings::AdvancedEffectsEnabledChanged`, Windows 10 1703 and later,
//!   <https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.advancedeffectsenabled>).
//!
//! # Sources not watched
//!
//! Each is reported by a message Windows sends to top-level windows, which
//! the watcher, without a window, does not receive: `WM_SETTINGCHANGE` with
//! the `SPI_SET*` action in `wParam` for the `SystemParametersInfo` one
//! (<https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settingchange>)
//! and `WM_SYSCOLORCHANGE`
//! (<https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syscolorchange>).
//!
//! - `SPI_GETNONCLIENTMETRICS`: the fonts.
//! - `GetSysColor`: the high-contrast colours (a change of high contrast
//!   itself is watched).

#[cfg(all(target_os = "windows", feature = "windows"))]
use ::windows::UI::ViewManagement::{UIColorType, UISettings};
#[cfg(all(target_os = "windows", feature = "windows"))]
use ::windows::Win32::UI::HiDpi::{GetSystemMetricsForDpi, SystemParametersInfoForDpi};
#[cfg(all(target_os = "windows", feature = "windows"))]
use ::windows::Win32::UI::WindowsAndMessaging::{
    NONCLIENTMETRICSW, SM_CXBORDER, SM_CXICON, SM_CXSMICON, SM_CXVSCROLL, SM_CYVTHUMB,
    SPI_GETNONCLIENTMETRICS, SYSTEM_METRICS_INDEX, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    SystemParametersInfoW, USER_DEFAULT_SCREEN_DPI,
};

use crate::model::FontSpec;

/// The running Windows version, from the `windows-version` crate's safe
/// `OsVersion::current()` (`RtlGetVersion`), for the generation table:
/// Windows 11 still reports major version 10, and its build tells the
/// generations apart.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) fn os_version() -> crate::pipeline::OsVersion {
    let v = ::windows_version::OsVersion::current();
    crate::pipeline::OsVersion::new(v.major, v.minor, v.build)
}

/// Per-widget fonts extracted from NONCLIENTMETRICSW.
///
/// Windows exposes four named LOGFONTW fields:
/// - `lfMessageFont` -- default UI font (messages, dialogs)
/// - `lfCaptionFont` -- title bar font
/// - `lfMenuFont` -- menu item font
/// - `lfStatusFont` -- status bar and tooltip font
struct AllFonts {
    msg: FontSpec,
    caption: FontSpec,
    menu: FontSpec,
    status: FontSpec,
}

/// The GetSysColor colours WinUI 3's HighContrast dictionaries draw with,
/// each the `SystemColor*Color` resource of its index (`COLOR_WINDOW` is
/// `SystemColorWindowColor`, see `apply_sys_colors`).
///
/// COLORREF format: 0x00BBGGRR (blue in high byte, red in low byte).
struct SysColors {
    window_bg: crate::Rgba,
    window_text: crate::Rgba,
    btn_face: crate::Rgba,
    btn_text: crate::Rgba,
    highlight: crate::Rgba,
    highlight_text: crate::Rgba,
    hotlight: crate::Rgba,
    gray_text: crate::Rgba,
}

/// Accessibility data from UISettings and SystemParametersInfoW.
struct AccessibilityData {
    text_scaling_factor: Option<f32>,
    high_contrast: Option<bool>,
    reduce_motion: Option<bool>,
    /// `UISettings.AdvancedEffectsEnabled`: the *Transparency effects* setting.
    advanced_effects_enabled: Option<bool>,
}

/// Convert a `windows::UI::Color` to our `Rgba` type.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) fn win_color_to_rgba(c: ::windows::UI::Color) -> crate::Rgba {
    crate::Rgba::new(c.R, c.G, c.B, c.A)
}

/// Detect dark mode from the system foreground color luminance.
///
/// Uses BT.601 luminance coefficients. A light foreground (luminance > 128)
/// indicates a dark background, i.e., dark mode.
pub(crate) fn is_dark_mode(fg: &crate::Rgba) -> bool {
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

/// The reader's `font_dpi`: 96, `USER_DEFAULT_SCREEN_DPI` (100 % scaling).
///
/// The model's sizes are logical pixels, and WinUI's effective pixels are
/// logical pixels, at 96 per inch: a point is 96/72 of them at any display
/// scale. The fonts are read at this DPI too, so their points are the
/// unscaled ones.
pub(crate) const LOGICAL_DPI: u32 = 96;

/// Convert a LOGFONTW read at [`LOGICAL_DPI`] to a FontSpec.
///
/// Extracts font family from `lfFaceName` (null-terminated UTF-16),
/// size in points from `abs(lfHeight) * 72 / 96`, and weight from `lfWeight`
/// (already CSS 100-900 scale, clamped), with `FW_DONTCARE` (0) no weight.
#[cfg(all(target_os = "windows", feature = "windows"))]
fn logfont_to_fontspec(lf: &::windows::Win32::Graphics::Gdi::LOGFONTW) -> FontSpec {
    logfont_to_fontspec_raw(&lf.lfFaceName, lf.lfHeight, lf.lfWeight)
}

/// Testable core of logfont_to_fontspec: takes raw field values.
fn logfont_to_fontspec_raw(face_name: &[u16; 32], lf_height: i32, lf_weight: i32) -> FontSpec {
    // Truncate at the NUL terminator without panic-prone slicing: `slice::split`
    // on the sentinel yields the pre-NUL prefix as its first item, fallback to
    // the full name if there is no NUL (per LOGFONTW's 32-char fixed buffer).
    let trimmed = face_name.split(|&c| c == 0).next().unwrap_or(face_name);
    let family = String::from_utf16_lossy(trimmed);
    let points = lf_height.unsigned_abs() as f32 * 72.0 / LOGICAL_DPI as f32;
    // FW_DONTCARE is 0 (windows 0.62.2 Win32/Graphics/Gdi/mod.rs:4998): the
    // default weight, which the preset states.
    let weight = (lf_weight > 0).then(|| lf_weight.clamp(100, 900) as u16);
    FontSpec {
        family: Some(family.into()),
        size: Some(crate::model::font::FontSize::Pt(points)),
        weight,
        ..Default::default()
    }
}

/// Read all system fonts from NONCLIENTMETRICSW (WIN-01), at [`LOGICAL_DPI`].
///
/// Extracts lfMessageFont, lfCaptionFont, lfMenuFont, and lfStatusFont
/// as FontSpec values. `SystemParametersInfoForDpi` at 96 DPI gives the
/// unscaled fonts; `SystemParametersInfoW` in a DPI-aware process would give
/// them at the system DPI. Returns default fonts if the system call fails.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_all_system_fonts() -> AllFonts {
    let mut ncm = NONCLIENTMETRICSW {
        cbSize: std::mem::size_of::<NONCLIENTMETRICSW>() as u32,
        ..Default::default()
    };

    // SAFETY: `ncm` is a live NONCLIENTMETRICSW whose `cbSize` is its size, as
    // SPI_GETNONCLIENTMETRICS requires of `uiParam`, and `pvParam` points to it; the
    // DPI is passed explicitly, so no process awareness is read
    // (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfofordpi).
    let success = unsafe {
        SystemParametersInfoForDpi(
            SPI_GETNONCLIENTMETRICS.0,
            ncm.cbSize,
            Some(&mut ncm as *mut _ as *mut _),
            0,
            USER_DEFAULT_SCREEN_DPI,
        )
    };

    if success.is_ok() {
        AllFonts {
            msg: logfont_to_fontspec(&ncm.lfMessageFont),
            caption: logfont_to_fontspec(&ncm.lfCaptionFont),
            menu: logfont_to_fontspec(&ncm.lfMenuFont),
            status: logfont_to_fontspec(&ncm.lfStatusFont),
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

// REMOVED(spacing): // WinUI3 Fluent Design spacing scale removed -- ThemeSpacing_DELETED deleted in Plan 01.

/// Read a system metric in logical pixels, the unit of every length in the
/// model.
///
/// `GetSystemMetricsForDpi` scales a metric to the DPI it is given. At
/// `USER_DEFAULT_SCREEN_DPI` (96, 100 % scaling) that is the unscaled metric,
/// in logical pixels; at the system DPI of a DPI-aware process it would be
/// device pixels (26 instead of 17 for `SM_CXVSCROLL` at 150 %).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn logical_system_metric(index: SYSTEM_METRICS_INDEX) -> f32 {
    // SAFETY: GetSystemMetricsForDpi takes the index and the DPI by value, with
    // no pointer; 96 is a valid DPI and an unknown index returns 0
    // (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi).
    unsafe { GetSystemMetricsForDpi(index, USER_DEFAULT_SCREEN_DPI) as f32 }
}

/// Read the scrollbar and border metrics, in logical pixels.
///
/// `SM_CXBORDER` is the Windows source of `defaults.border.line_width`
/// (`docs/platform-facts.md:1114`). The focus ring's width is not read:
/// `SM_CXFOCUSBORDER` is 1, and WinUI 3 draws a 2 px ring, which the preset
/// states.
#[cfg(all(target_os = "windows", feature = "windows"))]
fn read_widget_sizing(variant: &mut crate::ThemeMode) {
    variant.scrollbar.groove_width = Some(logical_system_metric(SM_CXVSCROLL));
    variant.scrollbar.min_thumb_length = Some(logical_system_metric(SM_CYVTHUMB));
    variant.defaults.border.line_width = Some(logical_system_metric(SM_CXBORDER));
}

/// The scrollbar and border metrics at 96 DPI (non-Windows testable version).
#[cfg(not(all(target_os = "windows", feature = "windows")))]
fn read_widget_sizing(variant: &mut crate::ThemeMode) {
    variant.scrollbar.groove_width = Some(17.0);
    // SM_CYVTHUMB at 96 DPI on the OS probe's three Windows images (platform-facts.md:382)
    variant.scrollbar.min_thumb_length = Some(17.0);
    variant.defaults.border.line_width = Some(1.0); // SM_CXBORDER default, platform-facts.md:378
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

/// Read the GetSysColor colours WinUI 3's HighContrast dictionaries use.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
fn read_sys_colors() -> SysColors {
    use ::windows::Win32::Graphics::Gdi::*;

    fn sys_color(index: SYS_COLOR_INDEX) -> crate::Rgba {
        // SAFETY: GetSysColor takes a colour index by value and touches no
        // memory of the caller's.
        let c = unsafe { GetSysColor(index) };
        colorref_to_rgba(c)
    }

    SysColors {
        window_bg: sys_color(COLOR_WINDOW),
        window_text: sys_color(COLOR_WINDOWTEXT),
        btn_face: sys_color(COLOR_BTNFACE),
        btn_text: sys_color(COLOR_BTNTEXT),
        highlight: sys_color(COLOR_HIGHLIGHT),
        highlight_text: sys_color(COLOR_HIGHLIGHTTEXT),
        hotlight: sys_color(COLOR_HOTLIGHT),
        gray_text: sys_color(COLOR_GRAYTEXT),
    }
}

/// Apply the high-contrast colours: each field takes the system colour that
/// WinUI 3's `HighContrast` theme dictionary gives the brush drawing it.
///
/// Lines are microsoft-ui-xaml 8463f45's: `controls/dev/CommonStyles/*.xaml`,
/// and `dxaml/xcp/dxaml/themes/generic.xaml` (its `HighContrast` dictionary is
/// :1970-3930) for the `SystemControl*` brushes and the keys a control's
/// dictionary leaves out, which the lookup reaches last, as the platform
/// resources ("Lookup behavior for XAML resource references",
/// <https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-resource-dictionary>,
/// updated 2026-08-28). A `SystemColor*Color` resource is GetSysColor of its
/// index (`dxaml/xcp/components/theming/FrameworkTheming.cpp:145-157`,
/// `:219-231`; `theminginterop/SystemThemingInterop.cpp:202`). WinUI 3 does
/// not draw the window caption, so the title bar colours stay the preset's.
fn apply_sys_colors(variant: &mut crate::ThemeMode, colors: &SysColors) {
    // TextFillColorPrimaryBrush (Common_themeresources_any.xaml:416).
    variant.defaults.text_color = Some(colors.window_text);
    // SolidBackgroundFillColorBaseBrush, Mica's fallback (Common_themeresources_any.xaml:486).
    variant.defaults.background_color = Some(colors.window_bg);
    // CardBackgroundFillColorDefaultBrush (Common_themeresources_any.xaml:474).
    variant.defaults.surface_color = Some(colors.window_bg);
    // TextFillColorDisabledBrush (Common_themeresources_any.xaml:419).
    variant.defaults.disabled_text_color = Some(colors.gray_text);
    // TextControlSelectionHighlightColor = SystemControlHighlightAccentBrush
    // (generic.xaml:2666, :2055); COLOR_HIGHLIGHT, docs/platform-facts.md:1055.
    variant.defaults.selection_background = Some(colors.highlight);
    // COLOR_HIGHLIGHTTEXT, docs/platform-facts.md:1056; no style in the theme resources or
    // generic.xaml sets a selected-text foreground (a text box sets only
    // SelectionHighlightColor, TextBox_themeresources.xaml:183).
    variant.defaults.selection_text_color = Some(colors.highlight_text);
    // SystemControlHighlightAccentBrush (generic.xaml:2055), the accent button's and the
    // checked ToggleButton's fill below; the fills that inherit the accent follow it.
    variant.defaults.accent_color = Some(colors.highlight);
    // The text on that accent: AccentButtonForeground = SystemControlHighlightAltAccentBrush
    // (generic.xaml:2056). TextOnAccentFillColorPrimaryBrush is WindowText under high contrast
    // (Common_themeresources_any.xaml:426), but there it is text on the AccentFill brushes,
    // which are COLOR_WINDOW (:453), not on the COLOR_HIGHLIGHT accent above.
    variant.defaults.accent_text_color = Some(colors.highlight_text);
    // HyperlinkForeground = SystemControlHyperlinkTextBrush (Hyperlink_themeresources.xaml:10;
    // generic.xaml:2083), SystemColorHotlightColor.
    variant.defaults.link_color = Some(colors.hotlight);
    // SystemFillColorAttentionBrush (Common_themeresources_any.xaml:493).
    variant.defaults.info_color = Some(colors.window_text);
    // SystemControlFocusVisualPrimaryBrush = FocusStrokeColorOuterBrush
    // (Common_themeresources.xaml:43; Common_themeresources_any.xaml:472).
    variant.defaults.focus_ring_color = Some(colors.window_text);
    // ButtonBackground = SystemControlBackgroundBaseLowBrush (Button_themeresources.xaml:78;
    // generic.xaml:2010), ButtonForeground = SystemColorButtonTextColorBrush
    // (Button_themeresources.xaml:82; Common_themeresources_any.xaml:512).
    variant.button.background_color = Some(colors.btn_face);
    variant.button.font.get_or_insert_default().color = Some(colors.btn_text);
    // AccentButtonBackground = SystemControlHighlightAccentBrush, AccentButtonForeground =
    // SystemControlHighlightAltAccentBrush (Button_themeresources.xaml:54, :58;
    // generic.xaml:2055, :2056).
    variant.button.primary_background = Some(colors.highlight);
    variant.button.primary_text_color = Some(colors.highlight_text);
    // ToggleButtonBackgroundChecked = SystemControlHighlightAccentBrush,
    // ToggleButtonForegroundChecked = SystemControlHighlightAltChromeWhiteBrush
    // (ToggleButton_themeresources.xaml:69, :81; generic.xaml:2055, :2067).
    variant.button.checked_background = Some(colors.highlight);
    variant.button.checked_text_color = Some(colors.highlight_text);
    // TextBox_themeresources.xaml's HighContrast dictionary leaves out the three keys:
    // TextControlBackground = SystemControlBackgroundAltMediumLowBrush (generic.xaml:2652,
    // :2008), TextControlForeground = SystemControlForegroundBaseHighBrush (:2648, :2039),
    // TextControlPlaceholderForeground = SystemControlPageTextBaseMediumBrush (:2660, :2097).
    variant.input.background_color = Some(colors.btn_face);
    variant.input.font.get_or_insert_default().color = Some(colors.btn_text);
    variant.input.placeholder_color = Some(colors.window_text);
    // MenuFlyoutPresenterBackground = SystemColorWindowColorBrush, MenuFlyoutItemForeground =
    // SystemColorButtonTextColorBrush (MenuFlyout_themeresources.xaml:123, :94).
    variant.menu.background_color = Some(colors.window_bg);
    variant.menu.font.get_or_insert_default().color = Some(colors.btn_text);
    // DefaultToolTipStyle's ToolTipBackgroundBrush = SystemColorWindowColorBrush and
    // ToolTipForegroundBrush = SystemColorWindowTextColorBrush (ToolTip_themeresources.xaml:43-44,
    // :26, :25).
    variant.tooltip.background_color = Some(colors.window_bg);
    variant.tooltip.font.get_or_insert_default().color = Some(colors.window_text);
    // ListViewItemBackgroundSelected, ListViewItemForegroundSelected
    // (ListViewItem_themeresources.xaml:85, :91).
    variant.list.selection_background = Some(colors.highlight);
    variant.list.selection_text_color = Some(colors.highlight_text);
    // ProgressBarBorderThemeThickness is 1 in HighContrast (ProgressBar_themeresources.xaml:13;
    // docs/platform-facts.md:1308).
    variant
        .progress_bar
        .border
        .get_or_insert_default()
        .line_width = Some(1.0);
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
        let mut hc = HIGHCONTRASTW {
            cbSize: std::mem::size_of::<HIGHCONTRASTW>() as u32,
            ..Default::default()
        };
        // SAFETY: `hc` is a live HIGHCONTRASTW whose `cbSize` is its size, as
        // SPI_GETHIGHCONTRAST requires of `uiParam`, and `pvParam` points to it; only
        // `dwFlags` is read afterwards
        // (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow).
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

    AccessibilityData {
        text_scaling_factor,
        high_contrast,
        reduce_motion: read_reduce_motion(),
        advanced_effects_enabled: settings.AdvancedEffectsEnabled().ok(),
    }
}

/// Whether the user asks for reduced motion: client-area animations are off
/// (`SPI_GETCLIENTAREAANIMATION`), as the reader reads it, for
/// [`crate::detect::detect_reduced_motion`]; `None` when the call fails.
#[cfg(all(target_os = "windows", feature = "windows"))]
#[allow(unsafe_code)]
pub(crate) fn read_reduce_motion() -> Option<bool> {
    use ::windows::Win32::UI::WindowsAndMessaging::SPI_GETCLIENTAREAANIMATION;
    let mut animation_enabled = ::windows::core::BOOL(1);
    // SAFETY: SPI_GETCLIENTAREAANIMATION writes one BOOL through `pvParam`,
    // which points to `animation_enabled`, a live BOOL for the whole call.
    let success = unsafe {
        SystemParametersInfoW(
            SPI_GETCLIENTAREAANIMATION,
            0,
            Some(&mut animation_enabled as *mut _ as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };
    success.is_ok().then(|| !animation_enabled.as_bool())
}

/// Read icon sizes from GetSystemMetricsForDpi, in logical pixels (WIN-05).
#[cfg(all(target_os = "windows", feature = "windows"))]
fn read_icon_sizes() -> (f32, f32) {
    (
        logical_system_metric(SM_CXSMICON),
        logical_system_metric(SM_CXICON),
    )
}

/// Testable core: given the accent and its shades, the foreground, fonts,
/// system colours and sizing data, build a `Theme` with a sparse `ThemeMode`.
///
/// Determines light/dark variant based on foreground luminance, then populates
/// the appropriate variant with the accent colours, per-widget fonts and
/// sizing, and, under high contrast only, the system colours
/// (`apply_sys_colors`). Only one variant is ever populated (matching KDE/GNOME
/// reader pattern).
fn build_theme(
    accent: crate::Rgba,
    fg: crate::Rgba,
    accent_shades: [Option<crate::Rgba>; 6],
    fonts: AllFonts,
    sys_colors: Option<&SysColors>,
    icon_sizes: Option<(f32, f32)>,
    accessibility: Option<&AccessibilityData>,
) -> crate::ReaderResult {
    let dark = is_dark_mode(&fg);

    // `accent_shades` is [Dark1, Dark2, Dark3, Light1, Light2, Light3]; WinUI 3's
    // SystemAccentColor* resources are those UISettings shades
    // (SystemThemingInterop.cpp:262-291, microsoft-ui-xaml 8463f45).
    let [dark1, dark2, _, _, light2, light3] = accent_shades;

    let mut variant = crate::ThemeMode::default();

    // --- The accent colours, which the user sets ---
    variant.defaults.accent_color = Some(accent);
    // The accent button and a checked ToggleButton are filled with
    // `AccentFillColorDefault`: AccentDark1 light, AccentLight2 dark
    // (docs/platform-facts.md:1175, :1181); the accent if the shade is unavailable.
    let primary_bg = if dark { light2 } else { dark1 }.unwrap_or(accent);
    variant.button.primary_background = Some(primary_bg);
    variant.button.checked_background = Some(primary_bg);
    // `HyperlinkForeground` = `AccentTextFillColorPrimary`: AccentDark2 light,
    // AccentLight3 dark (docs/platform-facts.md:1054).
    variant.defaults.link_color = if dark { light3 } else { dark2 };
    // `SystemFillColorAttention`: the accent light, AccentLight2 dark
    // (docs/platform-facts.md:1086).
    variant.defaults.info_color = if dark { light2 } else { Some(accent) };

    // --- Defaults-level font (message font) ---
    variant.defaults.font = fonts.msg;

    // --- Per-widget fonts (WIN-01) ---
    variant.window.title_bar_font = Some(fonts.caption);
    variant.menu.font = Some(fonts.menu);
    // lfStatusFont is "the font used in status bars and tooltips" (NONCLIENTMETRICSW).
    variant.tooltip.font = Some(fonts.status.clone());
    variant.status_bar.font = Some(fonts.status);

    // --- Widget sizing ---
    read_widget_sizing(&mut variant);

    // --- The high-contrast colours (WIN-03); outside high contrast the
    // GetSysColor indices are classic Win32 values WinUI 3 does not draw ---
    let high_contrast = accessibility.and_then(|a| a.high_contrast) == Some(true);
    if let (true, Some(colors)) = (high_contrast, sys_colors) {
        apply_sys_colors(&mut variant, colors);
    }

    // --- Icon sizes (WIN-05) ---
    if let Some((small, large)) = icon_sizes {
        variant.defaults.icon_sizes.small = Some(small);
        variant.defaults.icon_sizes.large = Some(large);
    }

    // Build AccessibilityPreferences from Windows AccessibilityData
    let acc = if let Some(a) = accessibility {
        let mut ap = crate::AccessibilityPreferences::default();
        if let Some(tsf) = a.text_scaling_factor {
            ap.text_scaling_factor = tsf;
        }
        if let Some(rm) = a.reduce_motion {
            ap.reduce_motion = rm;
        }
        if let Some(hc) = a.high_contrast {
            ap.high_contrast = hc;
        }
        if let Some(enabled) = a.advanced_effects_enabled {
            ap.reduce_transparency = !enabled;
        }
        ap
    } else {
        crate::AccessibilityPreferences::default()
    };
    let font_dpi = Some(LOGICAL_DPI as f32);

    let output = crate::ReaderOutput::Single {
        mode: Box::new(variant),
        is_dark: dark,
    };

    crate::ReaderResult {
        output,
        name: "Windows".into(),
        icon_set: Some(crate::IconSet::SegoeFluent),
        layout: crate::LayoutTheme::default(),
        font_dpi,
        accessibility: acc,
    }
}

/// Read the current Windows theme from UISettings, SystemParametersInfoW,
/// GetSystemMetricsForDpi, and, under high contrast, GetSysColor.
///
/// Reads the accent and foreground colors plus 6 accent shade colors from
/// `UISettings` (WinRT), per-widget fonts from `NONCLIENTMETRICSW` (Win32),
/// GetSysColor for the high-contrast colors, accessibility settings, and icon
/// sizes.
///
/// Returns `Error::ReaderFailed` if UISettings cannot be created (pre-Windows 10).
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`pipeline::select_reader`]; external consumers should use
/// [`SystemTheme::from_system()`](crate::SystemTheme::from_system).
///
/// The body contains no `.await` points — wrapping a sync implementation in
/// an async trait method produces a future that resolves immediately.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) struct WindowsReader;

#[cfg(all(target_os = "windows", feature = "windows"))]
#[async_trait::async_trait]
impl crate::reader::ThemeReader for WindowsReader {
    async fn read(&self) -> crate::Result<crate::ReaderResult> {
        read_windows()
    }
}

#[cfg(all(target_os = "windows", feature = "windows"))]
fn read_windows() -> crate::Result<crate::ReaderResult> {
    let settings = UISettings::new().map_err(|e| crate::Error::ReaderFailed {
        reader: crate::error::reader_names::WINDOWS,
        source: format!("UISettings unavailable: {e}").into(),
    })?;

    let accent = settings
        .GetColorValue(UIColorType::Accent)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::ReaderFailed {
            reader: crate::error::reader_names::WINDOWS,
            source: format!("GetColorValue(Accent) failed: {e}").into(),
        })?;
    let fg = settings
        .GetColorValue(UIColorType::Foreground)
        .map(win_color_to_rgba)
        .map_err(|e| crate::Error::ReaderFailed {
            reader: crate::error::reader_names::WINDOWS,
            source: format!("GetColorValue(Foreground) failed: {e}").into(),
        })?;

    let accent_shades = read_accent_shades(&settings);
    let fonts = read_all_system_fonts();
    let (small, large) = read_icon_sizes();
    let accessibility = read_accessibility(&settings);
    let sys_colors = (accessibility.high_contrast == Some(true)).then(read_sys_colors);

    Ok(build_theme(
        accent,
        fg,
        accent_shades,
        fonts,
        sys_colors.as_ref(),
        Some((small, large)),
        Some(&accessibility),
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::model::font::FontSize;

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
        use crate::model::font::FontSize;
        AllFonts {
            msg: FontSpec {
                family: Some("Segoe UI".into()),
                size: Some(FontSize::Pt(9.0)),
                weight: Some(400),
                ..Default::default()
            },
            caption: FontSpec {
                family: Some("Segoe UI".into()),
                size: Some(FontSize::Pt(9.0)),
                weight: Some(700),
                ..Default::default()
            },
            menu: FontSpec {
                family: Some("Segoe UI".into()),
                size: Some(FontSize::Pt(9.0)),
                weight: Some(400),
                ..Default::default()
            },
            status: FontSpec {
                family: Some("Segoe UI".into()),
                size: Some(FontSize::Pt(8.0)),
                weight: Some(400),
                ..Default::default()
            },
        }
    }

    /// Helper: extract the ThemeMode from a ReaderResult for test assertions.
    fn reader_mode(result: &crate::ReaderResult) -> &crate::ThemeMode {
        match &result.output {
            crate::ReaderOutput::Single { mode, .. } => mode,
            crate::ReaderOutput::Dual { light, .. } => light,
        }
    }

    /// Helper: build a ReaderResult in light mode with minimal args.
    fn light_reader() -> crate::ReaderResult {
        build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0), // black fg = light mode
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
        )
    }

    /// Helper: reconstruct a Theme from light_reader for tests needing Theme fields.
    fn light_theme() -> crate::Theme {
        let r = light_reader();
        r.output.to_theme(&r.name, r.icon_set, &r.layout)
    }

    /// Helper: build a ReaderResult in dark mode with minimal args.
    fn dark_reader() -> crate::ReaderResult {
        build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(255, 255, 255), // white fg = dark mode
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
        )
    }

    /// Helper: reconstruct a Theme from dark_reader for tests needing Theme fields.
    fn dark_theme() -> crate::Theme {
        let r = dark_reader();
        r.output.to_theme(&r.name, r.icon_set, &r.layout)
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
        let fs = logfont_to_fontspec_raw(&face, -16, 400);
        assert_eq!(fs.family.as_deref(), Some("Segoe UI"));
        assert_eq!(fs.size, Some(FontSize::Pt(12.0))); // abs(16) * 72 / 96 = 12
        assert_eq!(fs.weight, Some(400));
    }

    #[test]
    fn logfont_to_fontspec_bold_weight_700() {
        let face: [u16; 32] = [0; 32];
        let fs = logfont_to_fontspec_raw(&face, -16, 700);
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn logfont_to_fontspec_weight_clamped_to_range() {
        let face: [u16; 32] = [0; 32];
        // Weight below 100 gets clamped
        let fs = logfont_to_fontspec_raw(&face, -16, 50);
        assert_eq!(fs.weight, Some(100));
        // Weight above 900 gets clamped
        let fs = logfont_to_fontspec_raw(&face, -16, 1000);
        assert_eq!(fs.weight, Some(900));
    }

    /// `FW_DONTCARE` (0) states no weight, so the preset's stands.
    #[test]
    fn logfont_to_fontspec_weight_dontcare_is_no_weight() {
        let face: [u16; 32] = [0; 32];
        let fs = logfont_to_fontspec_raw(&face, -16, 0);
        assert_eq!(fs.weight, None);
        let fs = logfont_to_fontspec_raw(&face, -16, -400);
        assert_eq!(fs.weight, None);
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
    fn build_theme_name_is_windows() {
        assert_eq!(light_theme().name, "Windows");
    }

    /// The shades in `read_accent_shades`' order: Dark1, Dark2, Dark3,
    /// Light1, Light2, Light3, a distinct value each.
    const SHADES: [Option<crate::Rgba>; 6] = [
        Some(crate::Rgba::rgb(0, 0, 1)),
        Some(crate::Rgba::rgb(0, 0, 2)),
        Some(crate::Rgba::rgb(0, 0, 3)),
        Some(crate::Rgba::rgb(0, 0, 4)),
        Some(crate::Rgba::rgb(0, 0, 5)),
        Some(crate::Rgba::rgb(0, 0, 6)),
    ];

    /// Light mode: the accent fill is AccentDark1, the link AccentDark2, the
    /// informational fill the accent (docs/platform-facts.md:1175, :1181,
    /// :1054, :1086).
    #[test]
    fn build_theme_accent_shades_light_mode() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(0, 0, 0),
            SHADES,
            default_fonts(),
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
        assert_eq!(variant.defaults.accent_color, Some(accent));
        assert_eq!(variant.button.primary_background, SHADES[0]);
        assert_eq!(variant.button.checked_background, SHADES[0]);
        assert_eq!(variant.defaults.link_color, SHADES[1]);
        assert_eq!(variant.defaults.info_color, Some(accent));
    }

    /// Dark mode: the accent fill is AccentLight2, the link AccentLight3, the
    /// informational fill AccentLight2.
    #[test]
    fn build_theme_accent_shades_dark_mode() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(255, 255, 255),
            SHADES,
            default_fonts(),
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
        assert_eq!(variant.defaults.accent_color, Some(accent));
        assert_eq!(variant.button.primary_background, SHADES[4]);
        assert_eq!(variant.button.checked_background, SHADES[4]);
        assert_eq!(variant.defaults.link_color, SHADES[5]);
        assert_eq!(variant.defaults.info_color, SHADES[4]);
    }

    /// A shade that fails to read leaves the link and informational colours
    /// to the preset; the accent fill falls back to the accent
    /// (docs/platform-facts.md:1175).
    #[test]
    fn build_theme_without_shades_leaves_link_to_the_preset() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            default_fonts(),
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
        assert_eq!(variant.button.primary_background, Some(accent));
        assert_eq!(variant.defaults.link_color, None);
        assert_eq!(variant.defaults.info_color, None);
    }

    // === Per-widget font tests (WIN-01) ===

    #[test]
    fn build_theme_sets_title_bar_font() {
        let fonts = named_fonts();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            fonts,
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
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
            [None; 6],
            fonts,
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
        let menu_font = variant.menu.font.as_ref().expect("menu.font");
        assert_eq!(menu_font.family.as_deref(), Some("Segoe UI"));
        let status_font = variant.status_bar.font.as_ref().expect("status_bar.font");
        assert_eq!(status_font.size, Some(FontSize::Pt(8.0)));
    }

    /// The tooltip text is `lfStatusFont`'s, "the font used in status bars and
    /// tooltips" (NONCLIENTMETRICSW).
    #[test]
    fn build_theme_sets_tooltip_font_from_status_font() {
        let fonts = named_fonts();
        let status = fonts.status.clone();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            fonts,
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
        assert_eq!(variant.tooltip.font, Some(status));
    }

    #[test]
    fn build_theme_sets_defaults_font_from_msg_font() {
        let fonts = named_fonts();
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            fonts,
            None,
            None,
            None,
        );
        let variant = reader_mode(&theme);
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Segoe UI"));
        assert_eq!(variant.defaults.font.size, Some(FontSize::Pt(9.0)));
    }

    // === High contrast (the WinUI 3 HighContrast dictionaries) ===

    /// A distinct value per system colour, so each field shows which one it took.
    fn distinct_sys_colors() -> SysColors {
        SysColors {
            window_bg: crate::Rgba::rgb(1, 1, 1),
            window_text: crate::Rgba::rgb(2, 2, 2),
            btn_face: crate::Rgba::rgb(3, 3, 3),
            btn_text: crate::Rgba::rgb(4, 4, 4),
            highlight: crate::Rgba::rgb(5, 5, 5),
            highlight_text: crate::Rgba::rgb(6, 6, 6),
            hotlight: crate::Rgba::rgb(8, 8, 8),
            gray_text: crate::Rgba::rgb(7, 7, 7),
        }
    }

    fn reader_with_high_contrast(high_contrast: Option<bool>) -> crate::ReaderResult {
        let accessibility = AccessibilityData {
            text_scaling_factor: None,
            high_contrast,
            reduce_motion: None,
            advanced_effects_enabled: None,
        };
        build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [Some(crate::Rgba::rgb(0, 90, 170)); 6],
            default_fonts(),
            Some(&distinct_sys_colors()),
            None,
            Some(&accessibility),
        )
    }

    /// Outside high contrast the reader writes no Win32 colour, no derived
    /// colour and no focus ring: the windows-11 preset's WinUI 3 values stand.
    #[test]
    fn outside_high_contrast_the_reader_writes_only_the_accent_colours() {
        for high_contrast in [Some(false), None] {
            let result = reader_with_high_contrast(high_contrast);
            let v = reader_mode(&result);
            let font_color = |f: &Option<FontSpec>| f.as_ref().and_then(|f| f.color);
            assert_eq!(v.defaults.accent_color, Some(crate::Rgba::rgb(0, 120, 215)));
            assert_eq!(
                v.button.primary_background,
                Some(crate::Rgba::rgb(0, 90, 170))
            );
            assert_eq!(v.defaults.text_color, None);
            assert_eq!(v.defaults.background_color, None);
            assert_eq!(v.defaults.surface_color, None);
            assert_eq!(v.defaults.disabled_text_color, None);
            assert_eq!(v.defaults.selection_background, None);
            assert_eq!(v.defaults.focus_ring_color, None);
            assert_eq!(v.defaults.focus_ring_width, None);
            assert_eq!(v.button.background_color, None);
            assert_eq!(font_color(&v.button.font), None);
            assert_eq!(v.button.primary_text_color, None);
            assert_eq!(v.button.checked_text_color, None);
            assert_eq!(v.menu.background_color, None);
            assert_eq!(font_color(&v.menu.font), None);
            assert_eq!(v.tooltip.background_color, None);
            assert_eq!(font_color(&v.tooltip.font), None);
            assert_eq!(v.input.background_color, None);
            assert_eq!(font_color(&v.input.font), None);
            assert_eq!(v.input.placeholder_color, None);
            assert_eq!(v.list.selection_background, None);
            assert_eq!(v.list.selection_text_color, None);
            assert_eq!(v.window.title_bar_background, None);
            assert_eq!(font_color(&v.window.title_bar_font), None);
            assert_eq!(v.window.inactive_title_bar_background, None);
            assert_eq!(v.window.inactive_title_bar_text_color, None);
        }
    }

    /// Under high contrast each field takes the system colour WinUI 3's
    /// HighContrast dictionary gives its brush (see `apply_sys_colors`).
    #[test]
    fn under_high_contrast_the_reader_maps_the_system_colours() {
        let result = reader_with_high_contrast(Some(true));
        let v = reader_mode(&result);
        let c = distinct_sys_colors();
        let font_color = |f: &Option<FontSpec>| f.as_ref().and_then(|f| f.color);
        assert_eq!(v.defaults.text_color, Some(c.window_text));
        assert_eq!(v.defaults.background_color, Some(c.window_bg));
        assert_eq!(v.defaults.surface_color, Some(c.window_bg));
        assert_eq!(v.defaults.disabled_text_color, Some(c.gray_text));
        assert_eq!(v.defaults.selection_background, Some(c.highlight));
        assert_eq!(v.defaults.selection_text_color, Some(c.highlight_text));
        assert_eq!(v.defaults.accent_color, Some(c.highlight));
        assert_eq!(v.defaults.accent_text_color, Some(c.highlight_text));
        assert_eq!(v.defaults.link_color, Some(c.hotlight));
        assert_eq!(v.defaults.info_color, Some(c.window_text));
        assert_eq!(
            v.progress_bar.border.as_ref().and_then(|b| b.line_width),
            Some(1.0)
        );
        assert_eq!(v.defaults.focus_ring_color, Some(c.window_text));
        assert_eq!(v.button.background_color, Some(c.btn_face));
        assert_eq!(font_color(&v.button.font), Some(c.btn_text));
        assert_eq!(v.button.primary_background, Some(c.highlight));
        assert_eq!(v.button.primary_text_color, Some(c.highlight_text));
        assert_eq!(v.button.checked_background, Some(c.highlight));
        assert_eq!(v.button.checked_text_color, Some(c.highlight_text));
        assert_eq!(v.input.background_color, Some(c.btn_face));
        assert_eq!(font_color(&v.input.font), Some(c.btn_text));
        assert_eq!(v.input.placeholder_color, Some(c.window_text));
        assert_eq!(v.menu.background_color, Some(c.window_bg));
        assert_eq!(font_color(&v.menu.font), Some(c.btn_text));
        assert_eq!(v.tooltip.background_color, Some(c.window_bg));
        assert_eq!(font_color(&v.tooltip.font), Some(c.window_text));
        assert_eq!(v.list.selection_background, Some(c.highlight));
        assert_eq!(v.list.selection_text_color, Some(c.highlight_text));
        // WinUI 3 does not draw the window caption: no HighContrast brush.
        assert_eq!(v.window.title_bar_background, None);
        assert_eq!(font_color(&v.window.title_bar_font), None);
        assert_eq!(v.window.inactive_title_bar_background, None);
        assert_eq!(v.window.inactive_title_bar_text_color, None);
        assert_eq!(v.defaults.focus_ring_width, None);
    }

    // === Logical system metrics ===

    /// The model states lengths in logical pixels. `GetSystemMetricsForDpi`
    /// at 96 DPI (100 % scaling) is the unscaled metric; at the system DPI it
    /// would be device pixels, twice as large at 200 % scaling.
    #[cfg(all(target_os = "windows", feature = "windows"))]
    #[allow(unsafe_code)]
    #[test]
    fn system_metrics_are_logical_at_any_system_dpi() {
        let at_96 =
            |index| unsafe { GetSystemMetricsForDpi(index, USER_DEFAULT_SCREEN_DPI) } as f32;

        // A per-monitor-aware process is told the real system DPI; an unaware
        // one is told 96 and would pass whatever the reader does.
        //
        // The awareness is the process's, so it holds for every test in this
        // binary from here on, and it cannot be undone. No other test depends
        // on it: the reader asks for its metrics and fonts at an explicit
        // 96 DPI (`GetSystemMetricsForDpi`, `SystemParametersInfoForDpi`),
        // which awareness does not change, its `SystemParametersInfoW` calls
        // read accessibility flags, which have no size, and the icon tests in
        // `winicons.rs` check that an icon loads, not its pixel size.
        let _ = unsafe {
            ::windows::Win32::UI::HiDpi::SetProcessDpiAwarenessContext(
                ::windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            )
        };

        let mut variant = crate::ThemeMode::default();
        read_widget_sizing(&mut variant);
        assert_eq!(variant.scrollbar.groove_width, Some(at_96(SM_CXVSCROLL)));
        assert_eq!(variant.scrollbar.min_thumb_length, Some(at_96(SM_CYVTHUMB)));
        assert_eq!(variant.defaults.border.line_width, Some(at_96(SM_CXBORDER)));

        let (small, large) = read_icon_sizes();
        assert_eq!(small, at_96(SM_CXSMICON));
        assert_eq!(large, at_96(SM_CXICON));
    }

    /// The fonts' point sizes resolve, at the reader's `font_dpi`, to logical
    /// pixels: the message font's `lfHeight` at 96 DPI (100 % scaling). At the
    /// system DPI they would be device pixels, 1.5 times as large at 150 %.
    #[cfg(all(target_os = "windows", feature = "windows"))]
    #[allow(unsafe_code)]
    #[test]
    fn fonts_resolve_to_logical_pixels_at_any_system_dpi() {
        // Per-monitor aware, as in `system_metrics_are_logical_at_any_system_dpi`,
        // which says why setting it for the whole test binary is safe.
        let _ = unsafe {
            ::windows::Win32::UI::HiDpi::SetProcessDpiAwarenessContext(
                ::windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            )
        };
        let mut ncm = NONCLIENTMETRICSW {
            cbSize: std::mem::size_of::<NONCLIENTMETRICSW>() as u32,
            ..Default::default()
        };
        unsafe {
            ::windows::Win32::UI::HiDpi::SystemParametersInfoForDpi(
                SPI_GETNONCLIENTMETRICS.0,
                ncm.cbSize,
                Some(&mut ncm as *mut _ as *mut _),
                0,
                USER_DEFAULT_SCREEN_DPI,
            )
        }
        .unwrap();
        let logical = ncm.lfMessageFont.lfHeight.unsigned_abs() as f32;
        assert!(logical > 0.0, "no message font at 96 DPI");
        assert_eq!(LOGICAL_DPI, USER_DEFAULT_SCREEN_DPI);

        let result = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            read_all_system_fonts(),
            None,
            None,
            None,
        );
        let font_dpi = result.font_dpi.unwrap();
        let size = reader_mode(&result)
            .defaults
            .font
            .size
            .unwrap()
            .to_logical_px(font_dpi);
        assert!(
            (size - logical).abs() < 1e-3,
            "message font resolves to {size}px at font_dpi {font_dpi}; logical is {logical}px"
        );
    }

    /// The reader states points that resolve to logical pixels: `font_dpi` 96.
    #[test]
    fn build_theme_reports_logical_font_dpi() {
        assert_eq!(light_reader().font_dpi, Some(96.0));
    }

    // === Border line width test ===

    #[test]
    fn build_theme_sets_border_line_width() {
        let result = light_reader();
        let variant = reader_mode(&result);
        assert!(
            variant.defaults.border.line_width.is_some(),
            "defaults.border.line_width should be set from SM_CXBORDER"
        );
    }

    // === Icon sizes test (WIN-05) ===

    #[test]
    fn build_theme_with_icon_sizes() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            default_fonts(),
            None,
            Some((16.0, 32.0)),
            None,
        );
        let variant = reader_mode(&theme);
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
            advanced_effects_enabled: None,
        };
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            default_fonts(),
            None,
            None,
            Some(&accessibility),
        );
        // Accessibility fields are no longer on ThemeDefaults;
        // they live on AccessibilityPreferences (constructed by pipeline).
        assert!(matches!(
            theme.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
    }

    /// `reduce_transparency` is the *Transparency effects* setting turned off:
    /// `UISettings.AdvancedEffectsEnabled` false.
    #[test]
    fn build_theme_reduce_transparency_is_advanced_effects_off() {
        for (enabled, reduce) in [(Some(true), false), (Some(false), true), (None, false)] {
            let accessibility = AccessibilityData {
                text_scaling_factor: None,
                high_contrast: None,
                reduce_motion: None,
                advanced_effects_enabled: enabled,
            };
            let result = build_theme(
                crate::Rgba::rgb(0, 120, 215),
                crate::Rgba::rgb(0, 0, 0),
                [None; 6],
                default_fonts(),
                None,
                None,
                Some(&accessibility),
            );
            assert_eq!(
                result.accessibility.reduce_transparency, reduce,
                "AdvancedEffectsEnabled {enabled:?}"
            );
        }
    }

    // === Widget sizing test ===

    #[test]
    fn build_theme_includes_widget_sizing() {
        let result = light_reader();
        let variant = reader_mode(&result);
        assert_eq!(variant.scrollbar.groove_width, Some(17.0));
        assert_eq!(variant.scrollbar.min_thumb_length, Some(17.0));
        assert_eq!(variant.defaults.focus_ring_width, None);
        assert_eq!(variant.defaults.border.line_width, Some(1.0));
        // WinUI 3's design sizes come from the windows-11 preset alone.
        assert_eq!(variant.button.min_height, None);
        assert_eq!(variant.splitter.divider_width, None);
        assert_eq!(variant.menu.row_height, None);
        assert_eq!(variant.defaults.border.corner_radius, None);
    }

    // === What the preset states, not the reader ===

    #[test]
    fn build_theme_sets_the_accent_but_not_text_background_selection_focus() {
        let result = light_reader();
        let variant = reader_mode(&result);
        assert_eq!(
            variant.defaults.accent_color,
            Some(crate::Rgba::rgb(0, 120, 215))
        );
        assert_eq!(variant.defaults.text_color, None);
        assert_eq!(variant.defaults.background_color, None);
        assert_eq!(variant.defaults.selection_background, None);
        assert_eq!(variant.defaults.focus_ring_color, None);
    }

    /// `SM_CXFOCUSBORDER` is 1; WinUI 3's 2 px ring comes from the preset.
    #[test]
    fn build_theme_leaves_focus_ring_width_to_the_preset() {
        let result = light_reader();
        assert_eq!(reader_mode(&result).defaults.focus_ring_width, None);
    }

    /// `CardBackgroundFillColorDefault` comes from the preset
    /// (docs/platform-facts.md:1050).
    #[test]
    fn build_theme_leaves_surface_to_the_preset() {
        let result = light_reader();
        assert_eq!(reader_mode(&result).defaults.surface_color, None);
    }

    /// `TextFillColorDisabled` comes from the preset
    /// (docs/platform-facts.md:1060).
    #[test]
    fn build_theme_leaves_disabled_text_to_the_preset() {
        let result = light_reader();
        assert_eq!(reader_mode(&result).defaults.disabled_text_color, None);
    }

    // === No old model references verification ===

    #[test]
    fn build_theme_returns_native_theme_with_theme_variant() {
        // Verify the output type is correct (Theme with ThemeMode, not old types)
        let theme = light_theme();
        let variant: &crate::ThemeMode = theme.light.as_ref().unwrap();
        // Access new per-widget fields to prove they exist
        let _ = variant.defaults.accent_color;
        let _ = variant.window.title_bar_font;
        let _ = variant.menu.font;
        let _ = variant.status_bar.font;
        let _ = variant.button.background_color;
        let _ = variant.defaults.icon_sizes.small;
        let _ = variant.defaults.icon_sizes.toolbar;
        let _ = variant.dialog.button_order;
    }

    #[test]
    fn test_windows_resolve_validate() {
        // Load windows-11 preset as base (provides full color/geometry/spacing).
        let mut base = crate::Theme::preset("windows-11").unwrap();
        // Build reader output (light mode, sample data).
        let reader_output = light_theme();
        // Merge reader output on top of preset.
        base.merge(&reader_output);

        // Extract light variant.
        let mut light = base
            .light
            .clone()
            .expect("light variant should exist after merge");
        light.resolve_all();
        let resolved = light.validate().unwrap_or_else(|e| {
            panic!("Windows resolve/validate pipeline failed: {e}");
        });

        // Spot-check: reader-sourced fields present.
        assert_eq!(
            resolved.defaults.accent_color,
            crate::Rgba::rgb(0, 120, 215),
            "accent should be from Windows reader"
        );
        assert_eq!(
            resolved.defaults.font.family.as_ref(),
            "Segoe UI",
            "font family should be from Windows reader"
        );
        assert_eq!(
            resolved.dialog.button_order,
            crate::DialogButtonOrder::PrimaryLeft,
            "dialog button order should be leading-affirmative (primary leftmost) for Windows per MS Common Buttons / WinUI 3 guidance"
        );
        // icon_set is now on Theme/SystemTheme, not on ResolvedTheme
    }

    /// A high-contrast read over the windows-11 preset resolves and validates,
    /// with the system colours where the reader writes them.
    #[test]
    fn high_contrast_reader_over_the_preset_resolves() {
        let mut base = crate::Theme::preset("windows-11").unwrap();
        let r = reader_with_high_contrast(Some(true));
        base.merge(&r.output.to_theme(&r.name, r.icon_set, &r.layout));
        let mut light = base.light.clone().unwrap();
        light.resolve_all();
        let resolved = light.validate().unwrap_or_else(|e| {
            panic!("high-contrast resolve/validate failed: {e}");
        });
        let c = distinct_sys_colors();
        assert_eq!(resolved.button.background_color, c.btn_face);
        assert_eq!(resolved.button.font.color, c.btn_text);
        assert_eq!(resolved.list.selection_background, c.highlight);
    }

    // === Plan 94-03 (G8): WindowsReader ThemeReader impl ===
    //
    // Locks the G8 contract on the Windows side: a unit struct `WindowsReader`
    // exists at the module root and implements `crate::reader::ThemeReader`.
    // Before Task 2 lands, this fails to compile (`no struct WindowsReader` /
    // `unresolved module crate::reader`).
    //
    // The trait object coercion is the structural object-safety probe. Body
    // is gated on `cfg(all(target_os = "windows", feature = "windows"))` so
    // that the non-Windows build only compile-checks the trait path reference.
    #[test]
    fn windows_reader_exists() {
        // Reference the trait path once so name resolution fires on all
        // targets regardless of whether the WindowsReader struct is compiled.
        let _: Option<&dyn crate::reader::ThemeReader> = None;

        #[cfg(all(target_os = "windows", feature = "windows"))]
        {
            let r: Box<dyn crate::reader::ThemeReader> = Box::new(super::WindowsReader);
            let _ = &r;
        }
    }
}
