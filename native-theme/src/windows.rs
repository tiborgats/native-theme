//! Windows theme reader: reads accent color, accent shades, foreground/background,
//! system font, WinUI3 spacing defaults, and DPI-aware geometry metrics from
//! UISettings (WinRT), SystemParametersInfoW, and GetSystemMetricsForDpi (Win32).

use ::windows::UI::ViewManagement::{UIColorType, UISettings};
use ::windows::Win32::UI::HiDpi::{GetDpiForSystem, GetSystemMetricsForDpi};
use ::windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, NONCLIENTMETRICSW, SM_CXBORDER, SM_CXVSCROLL,
    SPI_GETNONCLIENTMETRICS, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

/// Convert a `windows::UI::Color` to our `Rgba` type.
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

/// Read the system message font from NONCLIENTMETRICSW.
///
/// Uses `SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, ...)` to read the system
/// font configuration, then extracts `lfMessageFont` (the font used for message boxes
/// and dialog text, which is the standard system UI font on Windows).
///
/// Converts `lfHeight` to points using `points = abs(lfHeight) * 72 / dpi`.
/// Returns `ThemeFonts::default()` if the system call fails.
fn read_system_font(dpi: u32) -> crate::ThemeFonts {
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
        // Convert to points: points = abs(lfHeight) * 72 / dpi
        let points = (lf.lfHeight.unsigned_abs() * 72) / dpi;

        crate::ThemeFonts {
            family: Some(family),
            size: Some(points as f32),
            mono_family: None, // Windows has no system monospace font setting
            mono_size: None,
        }
    } else {
        crate::ThemeFonts::default()
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

/// Read DPI-aware system geometry metrics.
///
/// Uses `GetDpiForSystem()` to get the system DPI, then `GetSystemMetricsForDpi`
/// for DPI-correct border and scrollbar widths. Also populates Windows 11 standard
/// corner radius values (4.0 / 8.0) and shadow setting.
///
/// Returns `(ThemeGeometry, dpi)` -- the DPI value is also needed for font conversion.
fn read_geometry_dpi_aware() -> (crate::ThemeGeometry, u32) {
    // SAFETY: GetDpiForSystem and GetSystemMetricsForDpi are always safe to call.
    // GetDpiForSystem returns 96 on failure (standard DPI).
    // GetSystemMetricsForDpi returns 0 on failure.
    let dpi = unsafe { GetDpiForSystem() };

    let geometry = unsafe {
        crate::ThemeGeometry {
            frame_width: Some(GetSystemMetricsForDpi(SM_CXBORDER, dpi) as f32),
            scroll_width: Some(GetSystemMetricsForDpi(SM_CXVSCROLL, dpi) as f32),
            radius: Some(4.0),
            radius_lg: Some(8.0),
            shadow: Some(true),
            ..Default::default()
        }
    };

    (geometry, dpi)
}

/// Read widget metrics for Windows, combining WinUI3 Fluent Design defaults
/// with system metrics for scrollbar and menu dimensions.
///
/// On Windows, uses `GetSystemMetricsForDpi` for scrollbar width (SM_CXVSCROLL),
/// scrollbar thumb height (SM_CYVTHUMB), and menu item height (SM_CYMENU).
/// On non-Windows platforms (for testability), uses WinUI3 Fluent defaults for all widgets.
fn read_widget_metrics(_dpi: u32) -> crate::model::widget_metrics::WidgetMetrics {
    use crate::model::widget_metrics::*;

    #[cfg(target_os = "windows")]
    let scrollbar = unsafe {
        use ::windows::Win32::UI::WindowsAndMessaging::*;
        ScrollbarMetrics {
            width: Some(GetSystemMetricsForDpi(SM_CXVSCROLL, _dpi) as f32),
            min_thumb_height: Some(GetSystemMetricsForDpi(SM_CYVTHUMB, _dpi) as f32),
            ..Default::default()
        }
    };

    #[cfg(not(target_os = "windows"))]
    let scrollbar = ScrollbarMetrics {
        width: Some(17.0),            // WinUI3 Fluent default
        min_thumb_height: Some(40.0), // WinUI3 Fluent default
        ..Default::default()
    };

    #[cfg(target_os = "windows")]
    let menu_item = unsafe {
        use ::windows::Win32::UI::WindowsAndMessaging::*;
        MenuItemMetrics {
            height: Some(GetSystemMetricsForDpi(SM_CYMENU, _dpi) as f32),
            padding_horizontal: Some(12.0),
            ..Default::default()
        }
    };

    #[cfg(not(target_os = "windows"))]
    let menu_item = MenuItemMetrics {
        height: Some(32.0),           // WinUI3 Fluent default
        padding_horizontal: Some(12.0),
        ..Default::default()
    };

    WidgetMetrics {
        button: ButtonMetrics {
            min_height: Some(32.0),          // WinUI3 default
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(20.0), // WinUI3 default
            spacing: Some(8.0),
            ..Default::default()
        },
        input: InputMetrics {
            min_height: Some(32.0),          // WinUI3 default
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        scrollbar,
        slider: SliderMetrics {
            track_height: Some(4.0),    // WinUI3 Fluent
            thumb_size: Some(22.0),     // WinUI3 Fluent
            ..Default::default()
        },
        progress_bar: ProgressBarMetrics {
            height: Some(4.0),  // WinUI3 Fluent
            ..Default::default()
        },
        tab: TabMetrics {
            min_height: Some(32.0),   // WinUI3 Fluent
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        menu_item,
        tooltip: TooltipMetrics {
            padding: Some(8.0),  // WinUI3 Fluent
            ..Default::default()
        },
        list_item: ListItemMetrics {
            height: Some(40.0),              // WinUI3 Fluent
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            height: Some(48.0),     // WinUI3 Fluent
            item_spacing: Some(4.0),
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(4.0), // WinUI3 Fluent
        },
    }
}

/// Testable core: given raw color values, accent shades, fonts, spacing,
/// geometry, and widget metrics, build a `NativeTheme`.
///
/// Determines light/dark variant based on foreground luminance, then populates
/// the appropriate variant with colors and geometry. Only one variant is ever
/// populated (matching KDE/GNOME reader pattern).
fn build_theme(
    accent: crate::Rgba,
    fg: crate::Rgba,
    bg: crate::Rgba,
    accent_shades: [Option<crate::Rgba>; 6],
    fonts: crate::ThemeFonts,
    spacing: crate::ThemeSpacing,
    geometry: crate::ThemeGeometry,
    widget_metrics: crate::model::widget_metrics::WidgetMetrics,
) -> crate::NativeTheme {
    let dark = is_dark_mode(&fg);

    // primary_background: In light mode use AccentDark1 (shades[0]), in dark mode
    // use AccentLight1 (shades[3]). Fall back to accent if shade unavailable.
    let primary_bg = if dark {
        accent_shades[3].unwrap_or(accent)
    } else {
        accent_shades[0].unwrap_or(accent)
    };

    // Border: mid-gray default since Windows doesn't expose a semantic border color
    let border = if dark {
        crate::Rgba::rgb(60, 60, 60)
    } else {
        crate::Rgba::rgb(200, 200, 200)
    };

    let mut colors = crate::ThemeColors::default();
    colors.accent = Some(accent);
    colors.foreground = Some(fg);
    colors.background = Some(bg);
    colors.selection = Some(accent);
    colors.focus_ring = Some(accent);
    colors.primary_background = Some(primary_bg);
    colors.primary_foreground = Some(fg);
    colors.surface = Some(bg);
    colors.secondary_background = Some(bg);
    colors.secondary_foreground = Some(fg);
    colors.border = Some(border);

    let variant = crate::ThemeVariant {
        colors,
        geometry,
        fonts,
        spacing,
        widget_metrics: Some(widget_metrics),
    };

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
/// and GetSystemMetricsForDpi.
///
/// Reads accent, foreground, and background colors plus 6 accent shade colors
/// from `UISettings` (WinRT), system font from `NONCLIENTMETRICSW` (Win32),
/// WinUI3 spacing defaults, and DPI-aware border/scrollbar widths from
/// `GetSystemMetricsForDpi` (Win32).
///
/// Returns `Error::Unavailable` if UISettings cannot be created (pre-Windows 10).
pub fn from_windows() -> crate::Result<crate::NativeTheme> {
    let settings = UISettings::new().map_err(|e| {
        crate::Error::Unavailable(format!("UISettings unavailable: {e}"))
    })?;

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
    let (geometry, dpi) = read_geometry_dpi_aware();
    let fonts = read_system_font(dpi);
    let spacing = winui3_spacing();
    let widget_metrics = read_widget_metrics(dpi);

    Ok(build_theme(accent, fg, bg, accent_shades, fonts, spacing, geometry, widget_metrics))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Default widget metrics for tests that don't care about metrics values.
    fn default_wm() -> crate::model::widget_metrics::WidgetMetrics {
        crate::model::widget_metrics::WidgetMetrics::default()
    }

    // === is_dark_mode tests ===

    #[test]
    fn is_dark_mode_white_foreground_returns_true() {
        // White foreground = dark background = dark mode
        let fg = crate::Rgba::rgb(255, 255, 255);
        assert!(is_dark_mode(&fg));
    }

    #[test]
    fn is_dark_mode_black_foreground_returns_false() {
        // Black foreground = light background = light mode
        let fg = crate::Rgba::rgb(0, 0, 0);
        assert!(!is_dark_mode(&fg));
    }

    #[test]
    fn is_dark_mode_mid_gray_boundary_returns_false() {
        // Mid-gray (128,128,128): luminance = 0.299*128 + 0.587*128 + 0.114*128 = 128.0
        // 128.0 is NOT > 128.0, so this should return false
        let fg = crate::Rgba::rgb(128, 128, 128);
        assert!(!is_dark_mode(&fg));
    }

    // === build_theme tests (updated with widget_metrics parameter) ===

    #[test]
    fn build_theme_dark_mode_populates_dark_variant_only() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),  // Windows blue accent
            crate::Rgba::rgb(255, 255, 255), // white fg = dark mode
            crate::Rgba::rgb(0, 0, 0),       // black bg
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        assert!(theme.dark.is_some(), "dark variant should be Some");
        assert!(theme.light.is_none(), "light variant should be None");
    }

    #[test]
    fn build_theme_light_mode_populates_light_variant_only() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),  // Windows blue accent
            crate::Rgba::rgb(0, 0, 0),       // black fg = light mode
            crate::Rgba::rgb(255, 255, 255), // white bg
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    #[test]
    fn accent_propagates_to_four_semantic_roles() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(0, 0, 0),       // light mode
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.colors.accent, Some(accent));
        assert_eq!(variant.colors.selection, Some(accent));
        assert_eq!(variant.colors.focus_ring, Some(accent));
        // With no accent shades, primary_background falls back to accent
        assert_eq!(variant.colors.primary_background, Some(accent));
    }

    #[test]
    fn geometry_values_preserved_in_output() {
        let geometry = crate::ThemeGeometry {
            frame_width: Some(1.0),
            scroll_width: Some(17.0),
            ..Default::default()
        };
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),       // light mode
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            geometry,
            default_wm(),
        );

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.geometry.frame_width, Some(1.0));
        assert_eq!(variant.geometry.scroll_width, Some(17.0));
    }

    #[test]
    fn theme_name_is_windows() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        assert_eq!(theme.name, "Windows");
    }

    // === New tests for accent shades ===

    #[test]
    fn accent_shades_applied_in_light_mode() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let dark1 = crate::Rgba::rgb(0, 90, 170); // AccentDark1
        let mut shades = [None; 6];
        shades[0] = Some(dark1); // AccentDark1 at index 0

        let theme = build_theme(
            accent,
            crate::Rgba::rgb(0, 0, 0), // light mode
            crate::Rgba::rgb(255, 255, 255),
            shades,
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(
            variant.colors.primary_background,
            Some(dark1),
            "light mode should use AccentDark1 for primary_background"
        );
    }

    #[test]
    fn accent_shades_applied_in_dark_mode() {
        let accent = crate::Rgba::rgb(0, 120, 215);
        let light1 = crate::Rgba::rgb(60, 160, 240); // AccentLight1
        let mut shades = [None; 6];
        shades[3] = Some(light1); // AccentLight1 at index 3

        let theme = build_theme(
            accent,
            crate::Rgba::rgb(255, 255, 255), // dark mode
            crate::Rgba::rgb(0, 0, 0),
            shades,
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );

        let variant = theme.dark.as_ref().expect("dark variant");
        assert_eq!(
            variant.colors.primary_background,
            Some(light1),
            "dark mode should use AccentLight1 for primary_background"
        );
    }

    #[test]
    fn accent_shades_fallback_when_none() {
        let accent = crate::Rgba::rgb(0, 120, 215);

        // Light mode: all shades None
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(
            variant.colors.primary_background,
            Some(accent),
            "should fall back to accent when shades are None"
        );

        // Dark mode: all shades None
        let theme = build_theme(
            accent,
            crate::Rgba::rgb(255, 255, 255),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        let variant = theme.dark.as_ref().expect("dark variant");
        assert_eq!(
            variant.colors.primary_background,
            Some(accent),
            "should fall back to accent when shades are None"
        );
    }

    // === primary_foreground test ===

    #[test]
    fn primary_foreground_is_fg() {
        let fg_light = crate::Rgba::rgb(0, 0, 0);
        let fg_dark = crate::Rgba::rgb(255, 255, 255);

        // Light mode
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            fg_light,
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.colors.primary_foreground, Some(fg_light));

        // Dark mode
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            fg_dark,
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        let variant = theme.dark.as_ref().expect("dark variant");
        assert_eq!(variant.colors.primary_foreground, Some(fg_dark));
    }

    // === winui3_spacing test ===

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

    // === fonts preserved test ===

    #[test]
    fn fonts_preserved_in_output() {
        let fonts = crate::ThemeFonts {
            family: Some("Segoe UI".to_string()),
            size: Some(12.0),
            mono_family: None,
            mono_size: None,
        };

        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            fonts.clone(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.fonts.family.as_deref(), Some("Segoe UI"));
        assert_eq!(variant.fonts.size, Some(12.0));
    }

    // === surface and secondary populated test ===

    #[test]
    fn surface_and_secondary_populated() {
        let fg = crate::Rgba::rgb(0, 0, 0);
        let bg = crate::Rgba::rgb(255, 255, 255);

        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            fg,
            bg,
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.colors.surface, Some(bg), "surface should equal bg");
        assert_eq!(
            variant.colors.secondary_foreground,
            Some(fg),
            "secondary_foreground should equal fg"
        );
        assert_eq!(
            variant.colors.secondary_background,
            Some(bg),
            "secondary_background should equal bg"
        );
    }

    // === border color test ===

    #[test]
    fn border_color_populated() {
        // Light mode
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(0, 0, 0),
            crate::Rgba::rgb(255, 255, 255),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        let variant = theme.light.as_ref().expect("light variant");
        assert!(variant.colors.border.is_some(), "light mode border should be Some");
        assert_eq!(variant.colors.border, Some(crate::Rgba::rgb(200, 200, 200)));

        // Dark mode
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(255, 255, 255),
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            default_wm(),
        );
        let variant = theme.dark.as_ref().expect("dark variant");
        assert!(variant.colors.border.is_some(), "dark mode border should be Some");
        assert_eq!(variant.colors.border, Some(crate::Rgba::rgb(60, 60, 60)));
    }

    // === widget metrics tests ===

    #[test]
    fn widget_metrics_fluent_defaults() {
        let wm = read_widget_metrics(96);
        assert_eq!(wm.button.min_height, Some(32.0), "WinUI3 button min_height");
        assert_eq!(wm.checkbox.indicator_size, Some(20.0), "WinUI3 checkbox indicator_size");
        assert_eq!(wm.input.min_height, Some(32.0), "WinUI3 input min_height");
        assert_eq!(wm.slider.thumb_size, Some(22.0), "WinUI3 slider thumb_size");
        assert!(wm.scrollbar.width.is_some(), "scrollbar width should be set");
        assert!(wm.menu_item.height.is_some(), "menu_item height should be set");
    }

    #[test]
    fn build_theme_dark_mode_includes_widget_metrics() {
        let wm = read_widget_metrics(96);
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),
            crate::Rgba::rgb(255, 255, 255), // dark mode
            crate::Rgba::rgb(0, 0, 0),
            [None; 6],
            crate::ThemeFonts::default(),
            crate::ThemeSpacing::default(),
            crate::ThemeGeometry::default(),
            wm,
        );
        let variant = theme.dark.as_ref().expect("dark variant");
        assert!(variant.widget_metrics.is_some(), "widget_metrics should be Some");
        let wm = variant.widget_metrics.as_ref().unwrap();
        assert_eq!(wm.button.min_height, Some(32.0));
    }
}
