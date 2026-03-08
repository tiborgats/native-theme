//! GNOME portal reader: reads accent color, color scheme, and contrast
//! from the XDG Desktop Portal Settings interface via ashpd.
//!
//! Uses the bundled Adwaita preset as base, then overlays portal-provided
//! accent color, color scheme (light/dark), and contrast preference.

use ashpd::desktop::Color;
use ashpd::desktop::settings::{ColorScheme, Contrast};

use crate::ThemeFonts;

/// Convert an ashpd portal Color to an Rgba, returning None if any
/// component is outside the [0.0, 1.0] range (per XDG spec: out-of-range
/// means "unset").
pub(crate) fn portal_color_to_rgba(color: &Color) -> Option<crate::Rgba> {
    let r = color.red();
    let g = color.green();
    let b = color.blue();

    // Per XDG spec: out-of-range means "accent color not set"
    if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
        return None;
    }

    Some(crate::Rgba::from_f32(r as f32, g as f32, b as f32, 1.0))
}

/// Apply a portal accent color across multiple semantic color roles.
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    variant.colors.accent = Some(*accent);
    variant.colors.selection = Some(*accent);
    variant.colors.focus_ring = Some(*accent);
    variant.colors.primary_background = Some(*accent);
}

/// Adjust theme variant for high contrast preference.
fn apply_high_contrast(variant: &mut crate::ThemeVariant) {
    variant.geometry.border_opacity = Some(1.0);
    variant.geometry.disabled_opacity = Some(0.7);
}

/// Parse a GNOME font string in the format `'Family Name Size'`.
///
/// gsettings outputs font names with single quotes (e.g., `'Cantarell 11'`).
/// The family is everything before the last space; the size is the number after it.
///
/// Returns `None` if the string is empty, has no space separator, or the size
/// is not a valid positive number.
pub(crate) fn parse_gnome_font_string(s: &str) -> Option<(String, f32)> {
    let trimmed = s.trim().trim_matches('\'');
    if trimmed.is_empty() {
        return None;
    }
    let last_space = trimmed.rfind(' ')?;
    let family = &trimmed[..last_space];
    let size_str = &trimmed[last_space + 1..];
    let size: f32 = size_str.parse().ok()?;
    if family.is_empty() || size <= 0.0 {
        return None;
    }
    Some((family.to_string(), size))
}

/// Read GNOME system fonts via gsettings.
///
/// Calls `gsettings get org.gnome.desktop.interface font-name` and
/// `gsettings get org.gnome.desktop.interface monospace-font-name` to
/// read the system UI and monospace fonts respectively.
///
/// Returns `ThemeFonts::default()` if gsettings is not available or fails.
pub(crate) fn read_gnome_fonts() -> ThemeFonts {
    let mut fonts = ThemeFonts::default();

    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "font-name"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some((family, size)) = parse_gnome_font_string(stdout.trim()) {
                fonts.family = Some(family);
                fonts.size = Some(size);
            }
        }
    }

    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "monospace-font-name"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some((family, size)) = parse_gnome_font_string(stdout.trim()) {
                fonts.mono_family = Some(family);
                fonts.mono_size = Some(size);
            }
        }
    }

    fonts
}

/// Return widget metrics populated from GNOME libadwaita CSS defaults.
///
/// Values based on libadwaita CSS defaults and Adwaita theme conventions
/// for standard GTK4 widget dimensions.
fn adwaita_widget_metrics() -> crate::model::widget_metrics::WidgetMetrics {
    use crate::model::widget_metrics::*;

    WidgetMetrics {
        button: ButtonMetrics {
            min_height: Some(34.0), // libadwaita default button
            padding_horizontal: Some(12.0),
            padding_vertical: Some(8.0),
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(20.0), // GtkCheckButton indicator
            spacing: Some(8.0),
            ..Default::default()
        },
        input: InputMetrics {
            min_height: Some(34.0), // GtkEntry
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        scrollbar: ScrollbarMetrics {
            width: Some(12.0), // Adwaita overlay scrollbar
            slider_width: Some(8.0),
            ..Default::default()
        },
        slider: SliderMetrics {
            track_height: Some(6.0), // GtkScale trough
            thumb_size: Some(20.0),
            ..Default::default()
        },
        progress_bar: ProgressBarMetrics {
            height: Some(6.0), // GtkProgressBar
            ..Default::default()
        },
        tab: TabMetrics {
            min_height: Some(34.0), // AdwTabBar
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        menu_item: MenuItemMetrics {
            height: Some(34.0), // GtkPopoverMenuBar
            padding_horizontal: Some(8.0),
            padding_vertical: Some(4.0),
            ..Default::default()
        },
        tooltip: TooltipMetrics {
            padding: Some(6.0), // GtkTooltip
            ..Default::default()
        },
        list_item: ListItemMetrics {
            padding_horizontal: Some(12.0),
            padding_vertical: Some(8.0),
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            height: Some(46.0), // AdwHeaderBar default
            item_spacing: Some(6.0),
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(6.0), // GtkPaned
        },
    }
}

/// Build a NativeTheme from an Adwaita base, applying portal-provided
/// color scheme, accent color, and contrast settings.
///
/// This is the testable core -- no D-Bus required.
pub(crate) fn build_theme(
    base: crate::NativeTheme,
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
) -> crate::Result<crate::NativeTheme> {
    let is_dark = matches!(scheme, ColorScheme::PreferDark);

    // Pick the appropriate variant from the Adwaita base
    let mut variant = if is_dark {
        base.dark.unwrap_or_default()
    } else {
        base.light.unwrap_or_default()
    };

    // Always set adwaita widget metrics on the variant
    variant.widget_metrics = Some(adwaita_widget_metrics());

    // Apply accent color if available and in range
    if let Some(color) = accent {
        if let Some(rgba) = portal_color_to_rgba(&color) {
            apply_accent(&mut variant, &rgba);
        }
    }

    // Apply high contrast adjustments
    if matches!(contrast, Contrast::High) {
        apply_high_contrast(&mut variant);
    }

    // Build NativeTheme with only the selected variant populated
    let theme = if is_dark {
        crate::NativeTheme {
            name: "GNOME".to_string(),
            light: None,
            dark: Some(variant),
        }
    } else {
        crate::NativeTheme {
            name: "GNOME".to_string(),
            light: Some(variant),
            dark: None,
        }
    };

    Ok(theme)
}

/// Read the current GNOME theme from the XDG Desktop Portal.
///
/// Reads color scheme (light/dark), accent color, and contrast preference
/// from the `org.freedesktop.appearance` portal namespace via ashpd.
///
/// Falls back to bundled Adwaita defaults when the portal is unavailable
/// (no D-Bus session, sandboxed environment, or old portal version).
pub async fn from_gnome() -> crate::Result<crate::NativeTheme> {
    let base = crate::NativeTheme::preset("adwaita").expect("adwaita preset must be bundled");

    // Try to connect to the portal. If unavailable, return Adwaita defaults.
    let settings = match ashpd::desktop::settings::Settings::new().await {
        Ok(s) => s,
        Err(_) => {
            return build_theme(
                base,
                ColorScheme::NoPreference,
                None,
                Contrast::NoPreference,
            );
        }
    };

    // Read the three appearance settings. Each can fail independently.
    let scheme = settings.color_scheme().await.unwrap_or_default();
    let accent = settings.accent_color().await.ok();
    let contrast = settings.contrast().await.unwrap_or_default();

    let mut theme = build_theme(base, scheme, accent, contrast)?;

    // Merge GNOME font data from gsettings into the theme variant
    let fonts = read_gnome_fonts();
    if !fonts.is_empty() {
        if let Some(ref mut variant) = theme.light {
            variant.fonts.merge(&fonts);
        }
        if let Some(ref mut variant) = theme.dark {
            variant.fonts.merge(&fonts);
        }
    }

    Ok(theme)
}

/// Read KDE theme from kdeglobals, then overlay portal accent color if available.
///
/// Reads the KDE kdeglobals file as the base theme via [`crate::kde::from_kde()`],
/// then attempts to read the accent color from the XDG Desktop Portal. If the
/// portal provides a valid accent color, it is applied to accent, selection,
/// focus_ring, and primary_background fields via [`NativeTheme::merge`].
///
/// Falls back to the KDE-only base if the portal is unavailable or provides
/// no accent color.
///
/// Requires both `kde` and `portal` features.
#[cfg(feature = "kde")]
pub async fn from_kde_with_portal() -> crate::Result<crate::NativeTheme> {
    let mut base = crate::kde::from_kde()?;

    // Try to get accent color from portal
    let settings = match ashpd::desktop::settings::Settings::new().await {
        Ok(s) => s,
        Err(_) => return Ok(base),
    };

    let accent = match settings.accent_color().await {
        Ok(color) => color,
        Err(_) => return Ok(base),
    };

    let rgba = match portal_color_to_rgba(&accent) {
        Some(r) => r,
        None => return Ok(base),
    };

    // Build overlay with accent applied to the same variant(s) as base
    let mut overlay = crate::NativeTheme::new("");

    if base.light.is_some() {
        let mut variant = crate::ThemeVariant::default();
        apply_accent(&mut variant, &rgba);
        overlay.light = Some(variant);
    }
    if base.dark.is_some() {
        let mut variant = crate::ThemeVariant::default();
        apply_accent(&mut variant, &rgba);
        overlay.dark = Some(variant);
    }

    base.merge(&overlay);
    Ok(base)
}

/// Detect which desktop portal backend is running via D-Bus activatable names.
///
/// Checks the session bus for activatable service names containing
/// `portal.desktop.kde` or `portal.desktop.gnome` to infer the desktop
/// environment when `XDG_CURRENT_DESKTOP` is ambiguous or unset.
///
/// Returns `None` if D-Bus is unavailable or no recognized portal backend
/// is found.
pub(crate) async fn detect_portal_backend() -> Option<super::LinuxDesktop> {
    let connection = ashpd::zbus::Connection::session().await.ok()?;
    let proxy = ashpd::zbus::fdo::DBusProxy::new(&connection).await.ok()?;
    let names = proxy.list_activatable_names().await.ok()?;

    for name in &names {
        let name_str = name.as_str();
        if name_str.contains("portal.desktop.kde") {
            return Some(super::LinuxDesktop::Kde);
        }
        if name_str.contains("portal.desktop.gnome") {
            return Some(super::LinuxDesktop::Gnome);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_gnome_font_string tests ===

    #[test]
    fn parse_gnome_font_string_standard() {
        assert_eq!(
            parse_gnome_font_string("'Cantarell 11'"),
            Some(("Cantarell".to_string(), 11.0))
        );
    }

    #[test]
    fn parse_gnome_font_string_multi_word() {
        assert_eq!(
            parse_gnome_font_string("'Noto Sans 10'"),
            Some(("Noto Sans".to_string(), 10.0))
        );
    }

    #[test]
    fn parse_gnome_font_string_no_quotes() {
        assert_eq!(
            parse_gnome_font_string("Ubuntu Mono 13"),
            Some(("Ubuntu Mono".to_string(), 13.0))
        );
    }

    #[test]
    fn parse_gnome_font_string_fractional_size() {
        assert_eq!(
            parse_gnome_font_string("'Inter 10.5'"),
            Some(("Inter".to_string(), 10.5))
        );
    }

    #[test]
    fn parse_gnome_font_string_empty() {
        assert_eq!(parse_gnome_font_string(""), None);
    }

    #[test]
    fn parse_gnome_font_string_only_quotes() {
        assert_eq!(parse_gnome_font_string("''"), None);
    }

    #[test]
    fn parse_gnome_font_string_no_size() {
        assert_eq!(parse_gnome_font_string("'Cantarell'"), None);
    }

    #[test]
    fn parse_gnome_font_string_zero_size() {
        assert_eq!(parse_gnome_font_string("'Font 0'"), None);
    }

    #[test]
    fn parse_gnome_font_string_negative_size() {
        assert_eq!(parse_gnome_font_string("'Font -1'"), None);
    }

    // === portal_color_to_rgba tests ===

    #[test]
    fn portal_color_valid_converts_to_rgba() {
        let color = Color::new(0.2, 0.4, 0.6);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        let rgba = result.unwrap();
        assert_eq!(rgba, crate::Rgba::from_f32(0.2, 0.4, 0.6, 1.0));
    }

    #[test]
    fn portal_color_out_of_range_high_returns_none() {
        let color = Color::new(1.5, 0.0, 0.0);
        assert!(portal_color_to_rgba(&color).is_none());
    }

    #[test]
    fn portal_color_out_of_range_negative_returns_none() {
        let color = Color::new(-0.1, 0.5, 0.5);
        assert!(portal_color_to_rgba(&color).is_none());
    }

    #[test]
    fn portal_color_to_rgba_boundary_zero() {
        let color = Color::new(0.0, 0.0, 0.0);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), crate::Rgba::from_f32(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn portal_color_to_rgba_boundary_one() {
        let color = Color::new(1.0, 1.0, 1.0);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), crate::Rgba::from_f32(1.0, 1.0, 1.0, 1.0));
    }

    // === build_theme color scheme tests ===

    fn adwaita_base() -> crate::NativeTheme {
        crate::NativeTheme::preset("adwaita").unwrap()
    }

    #[test]
    fn dark_scheme_produces_dark_variant_only() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::PreferDark,
            None,
            Contrast::NoPreference,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.dark.is_some(), "dark variant should be Some");
        assert!(theme.light.is_none(), "light variant should be None");
    }

    #[test]
    fn light_scheme_produces_light_variant_only() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::PreferLight,
            None,
            Contrast::NoPreference,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    #[test]
    fn no_preference_defaults_to_light() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    // === accent color tests ===

    #[test]
    fn valid_accent_propagates_to_four_fields() {
        let accent = Color::new(0.2, 0.4, 0.8);
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);

        assert_eq!(variant.colors.accent, Some(expected));
        assert_eq!(variant.colors.selection, Some(expected));
        assert_eq!(variant.colors.focus_ring, Some(expected));
        assert_eq!(variant.colors.primary_background, Some(expected));
    }

    // === high contrast tests ===

    #[test]
    fn high_contrast_sets_border_and_disabled_opacity() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::High,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.geometry.border_opacity, Some(1.0));
        assert_eq!(variant.geometry.disabled_opacity, Some(0.7));
    }

    #[test]
    fn normal_contrast_preserves_adwaita_geometry() {
        let base = adwaita_base();
        let base_light = base.light.as_ref().unwrap().clone();

        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.geometry, base_light.geometry);
    }

    // === widget metrics tests ===

    #[test]
    fn adwaita_widget_metrics_spot_check() {
        let wm = adwaita_widget_metrics();
        assert_eq!(
            wm.button.min_height,
            Some(34.0),
            "libadwaita default button"
        );
        assert_eq!(
            wm.checkbox.indicator_size,
            Some(20.0),
            "GtkCheckButton indicator"
        );
        assert_eq!(wm.scrollbar.width, Some(12.0), "Adwaita overlay scrollbar");
    }

    #[test]
    fn build_theme_includes_widget_metrics() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert!(
            variant.widget_metrics.is_some(),
            "widget_metrics should be Some"
        );
    }

    // === fallback test ===

    #[test]
    fn no_accent_no_preference_no_contrast_returns_adwaita_light() {
        let base = adwaita_base();
        let base_light = base.light.as_ref().unwrap().clone();

        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
        )
        .unwrap();

        assert_eq!(theme.name, "GNOME");
        let variant = theme.light.as_ref().expect("light variant");
        // Colors should match Adwaita light defaults exactly
        assert_eq!(variant.colors, base_light.colors);
        assert_eq!(variant.fonts, base_light.fonts);
        assert_eq!(variant.geometry, base_light.geometry);
        assert_eq!(variant.spacing, base_light.spacing);
    }
}
