//! Windows theme reader: reads accent color, foreground/background, and system
//! geometry metrics from UISettings (WinRT) and GetSystemMetrics (Win32).

use ::windows::UI::ViewManagement::{UIColorType, UISettings};
use ::windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXBORDER, SM_CXVSCROLL};

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

/// Read system geometry metrics from GetSystemMetrics.
fn read_geometry() -> crate::ThemeGeometry {
    // SAFETY: GetSystemMetrics is always safe to call with valid SM_* constants.
    // It returns 0 on failure, which is a valid (if unlikely) metric value.
    unsafe {
        crate::ThemeGeometry {
            frame_width: Some(GetSystemMetrics(SM_CXBORDER) as f32),
            scroll_width: Some(GetSystemMetrics(SM_CXVSCROLL) as f32),
            ..Default::default()
        }
    }
}

/// Testable core: given raw color values and geometry, build a `NativeTheme`.
///
/// Determines light/dark variant based on foreground luminance, then populates
/// the appropriate variant with colors and geometry. Only one variant is ever
/// populated (matching KDE/GNOME reader pattern).
fn build_theme(
    accent: crate::Rgba,
    fg: crate::Rgba,
    bg: crate::Rgba,
    geometry: crate::ThemeGeometry,
) -> crate::NativeTheme {
    let dark = is_dark_mode(&fg);

    let mut colors = crate::ThemeColors::default();
    colors.accent = Some(accent);
    colors.foreground = Some(fg);
    colors.background = Some(bg);
    colors.selection = Some(accent);
    colors.focus_ring = Some(accent);
    colors.primary_background = Some(accent);

    let variant = crate::ThemeVariant {
        colors,
        geometry,
        fonts: Default::default(),
        spacing: Default::default(),
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

/// Read the current Windows theme from UISettings and GetSystemMetrics.
///
/// Reads accent, foreground, and background colors from `UISettings` (WinRT),
/// and border/scrollbar widths from `GetSystemMetrics` (Win32).
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

    let geometry = read_geometry();

    Ok(build_theme(accent, fg, bg, geometry))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // === build_theme tests ===

    #[test]
    fn build_theme_dark_mode_populates_dark_variant_only() {
        let theme = build_theme(
            crate::Rgba::rgb(0, 120, 215),  // Windows blue accent
            crate::Rgba::rgb(255, 255, 255), // white fg = dark mode
            crate::Rgba::rgb(0, 0, 0),       // black bg
            crate::ThemeGeometry::default(),
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
            crate::ThemeGeometry::default(),
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
            crate::ThemeGeometry::default(),
        );

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.colors.accent, Some(accent));
        assert_eq!(variant.colors.selection, Some(accent));
        assert_eq!(variant.colors.focus_ring, Some(accent));
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
            geometry,
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
            crate::ThemeGeometry::default(),
        );
        assert_eq!(theme.name, "Windows");
    }
}
