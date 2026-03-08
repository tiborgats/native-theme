//! macOS theme reader: reads semantic NSColor values with P3-to-sRGB conversion,
//! resolves both light and dark appearance variants via NSAppearance, and reads
//! system and monospace fonts via NSFont.

#[cfg(feature = "macos")]
use objc2_app_kit::{NSAppearance, NSColor, NSColorSpace, NSFont, NSFontWeight};
#[cfg(feature = "macos")]
use objc2_foundation::NSString;

/// Convert an NSColor to sRGB and extract RGBA components.
///
/// Converts the color to the sRGB color space via `colorUsingColorSpace`,
/// handling P3-to-sRGB conversion automatically. Returns `None` if the
/// color cannot be converted (e.g., pattern colors).
#[cfg(feature = "macos")]
fn nscolor_to_rgba(color: &NSColor, srgb: &NSColorSpace) -> Option<crate::Rgba> {
    let srgb_color = unsafe { color.colorUsingColorSpace(srgb) }?;
    let r = unsafe { srgb_color.redComponent() } as f32;
    let g = unsafe { srgb_color.greenComponent() } as f32;
    let b = unsafe { srgb_color.blueComponent() } as f32;
    let a = unsafe { srgb_color.alphaComponent() } as f32;
    Some(crate::Rgba::from_f32(r, g, b, a))
}

/// Read all semantic NSColor values for the current appearance context.
///
/// Must be called within an `NSAppearance::performAsCurrentDrawingAppearance`
/// block so that dynamic colors resolve to the correct appearance.
#[cfg(feature = "macos")]
fn read_semantic_colors() -> crate::ThemeColors {
    let srgb = unsafe { NSColorSpace::sRGBColorSpace() };

    let label = nscolor_to_rgba(unsafe { &NSColor::labelColor() }, &srgb);

    crate::ThemeColors {
        // Core (7)
        accent: nscolor_to_rgba(unsafe { &NSColor::controlAccentColor() }, &srgb),
        background: nscolor_to_rgba(unsafe { &NSColor::windowBackgroundColor() }, &srgb),
        foreground: label,
        surface: nscolor_to_rgba(unsafe { &NSColor::controlBackgroundColor() }, &srgb),
        border: nscolor_to_rgba(unsafe { &NSColor::separatorColor() }, &srgb),
        muted: nscolor_to_rgba(unsafe { &NSColor::secondaryLabelColor() }, &srgb),
        shadow: nscolor_to_rgba(unsafe { &NSColor::shadowColor() }, &srgb),
        // Primary (2)
        primary_background: nscolor_to_rgba(unsafe { &NSColor::controlAccentColor() }, &srgb),
        primary_foreground: nscolor_to_rgba(
            unsafe { &NSColor::alternateSelectedControlTextColor() },
            &srgb,
        ),
        // Secondary (2)
        secondary_background: nscolor_to_rgba(unsafe { &NSColor::controlColor() }, &srgb),
        secondary_foreground: nscolor_to_rgba(unsafe { &NSColor::controlTextColor() }, &srgb),
        // Status (8)
        danger: nscolor_to_rgba(unsafe { &NSColor::systemRedColor() }, &srgb),
        danger_foreground: label,
        warning: nscolor_to_rgba(unsafe { &NSColor::systemOrangeColor() }, &srgb),
        warning_foreground: label,
        success: nscolor_to_rgba(unsafe { &NSColor::systemGreenColor() }, &srgb),
        success_foreground: label,
        info: nscolor_to_rgba(unsafe { &NSColor::systemBlueColor() }, &srgb),
        info_foreground: label,
        // Interactive (4)
        selection: nscolor_to_rgba(
            unsafe { &NSColor::selectedContentBackgroundColor() },
            &srgb,
        ),
        selection_foreground: nscolor_to_rgba(unsafe { &NSColor::selectedTextColor() }, &srgb),
        link: nscolor_to_rgba(unsafe { &NSColor::linkColor() }, &srgb),
        focus_ring: nscolor_to_rgba(
            unsafe { &NSColor::keyboardFocusIndicatorColor() },
            &srgb,
        ),
        // Panel (6)
        sidebar: nscolor_to_rgba(unsafe { &NSColor::underPageBackgroundColor() }, &srgb),
        sidebar_foreground: label,
        tooltip: nscolor_to_rgba(unsafe { &NSColor::windowBackgroundColor() }, &srgb),
        tooltip_foreground: label,
        popover: nscolor_to_rgba(unsafe { &NSColor::windowBackgroundColor() }, &srgb),
        popover_foreground: label,
        // Component (7)
        button: nscolor_to_rgba(unsafe { &NSColor::controlColor() }, &srgb),
        button_foreground: nscolor_to_rgba(unsafe { &NSColor::controlTextColor() }, &srgb),
        input: nscolor_to_rgba(unsafe { &NSColor::textBackgroundColor() }, &srgb),
        input_foreground: nscolor_to_rgba(unsafe { &NSColor::textColor() }, &srgb),
        disabled: nscolor_to_rgba(unsafe { &NSColor::disabledControlTextColor() }, &srgb),
        separator: nscolor_to_rgba(unsafe { &NSColor::separatorColor() }, &srgb),
        alternate_row: nscolor_to_rgba(unsafe { &NSColor::controlBackgroundColor() }, &srgb),
    }
}

/// Read system and monospace font information via NSFont.
///
/// Fonts are appearance-independent, so this only needs to be called once
/// (not per-appearance).
#[cfg(feature = "macos")]
fn read_fonts() -> crate::ThemeFonts {
    let system_size = unsafe { NSFont::systemFontSize() };
    let system_font = unsafe { NSFont::systemFontOfSize(system_size) };
    let mono_font =
        unsafe { NSFont::monospacedSystemFontOfSize_weight(system_size, NSFontWeight::Regular) };

    crate::ThemeFonts {
        family: system_font.familyName().map(|n| n.to_string()),
        size: Some(unsafe { system_font.pointSize() } as f32),
        mono_family: mono_font.familyName().map(|n| n.to_string()),
        mono_size: Some(unsafe { mono_font.pointSize() } as f32),
    }
}

/// Testable core: assemble a NativeTheme from pre-read color and font data.
///
/// Takes pre-resolved colors for both light and dark variants and fonts,
/// then constructs the complete NativeTheme. Both variants are always
/// populated (unlike KDE/GNOME/Windows which populate only the active one),
/// since macOS can resolve colors for both appearances.
#[cfg_attr(not(feature = "macos"), allow(dead_code))]
fn build_theme(
    light_colors: crate::ThemeColors,
    dark_colors: crate::ThemeColors,
    fonts: crate::ThemeFonts,
) -> crate::NativeTheme {
    crate::NativeTheme {
        name: "macOS".to_string(),
        light: Some(crate::ThemeVariant {
            colors: light_colors,
            fonts: fonts.clone(),
            geometry: Default::default(),
            spacing: Default::default(),
            widget_metrics: None,
        }),
        dark: Some(crate::ThemeVariant {
            colors: dark_colors,
            fonts,
            geometry: Default::default(),
            spacing: Default::default(),
            widget_metrics: None,
        }),
    }
}

/// Read the current macOS theme, resolving both light and dark appearance variants.
///
/// Uses `NSAppearance::performAsCurrentDrawingAppearance` (macOS 11+) to scope
/// semantic color resolution to each appearance. Reads ~20 NSColor semantic
/// colors per variant with P3-to-sRGB conversion, plus system and monospace fonts.
///
/// # Errors
///
/// Returns `Error::Unavailable` if neither light nor dark appearance can be created
/// (extremely unlikely on any macOS version that supports these APIs).
#[cfg(feature = "macos")]
pub fn from_macos() -> crate::Result<crate::NativeTheme> {
    let light_name = NSString::from_str("NSAppearanceNameAqua");
    let dark_name = NSString::from_str("NSAppearanceNameDarkAqua");

    let light_appearance = unsafe { NSAppearance::appearanceNamed(&light_name) };
    let dark_appearance = unsafe { NSAppearance::appearanceNamed(&dark_name) };

    if light_appearance.is_none() && dark_appearance.is_none() {
        return Err(crate::Error::Unavailable(
            "neither light nor dark NSAppearance could be created".to_string(),
        ));
    }

    let light_colors = if let Some(app) = &light_appearance {
        let mut colors = crate::ThemeColors::default();
        app.performAsCurrentDrawingAppearance(|| {
            colors = read_semantic_colors();
        });
        colors
    } else {
        crate::ThemeColors::default()
    };

    let dark_colors = if let Some(app) = &dark_appearance {
        let mut colors = crate::ThemeColors::default();
        app.performAsCurrentDrawingAppearance(|| {
            colors = read_semantic_colors();
        });
        colors
    } else {
        crate::ThemeColors::default()
    };

    let fonts = read_fonts();

    Ok(build_theme(light_colors, dark_colors, fonts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_light_colors() -> crate::ThemeColors {
        crate::ThemeColors {
            accent: Some(crate::Rgba::rgb(0, 122, 255)),
            background: Some(crate::Rgba::rgb(246, 246, 246)),
            foreground: Some(crate::Rgba::rgb(0, 0, 0)),
            surface: Some(crate::Rgba::rgb(255, 255, 255)),
            border: Some(crate::Rgba::rgb(200, 200, 200)),
            ..Default::default()
        }
    }

    fn sample_dark_colors() -> crate::ThemeColors {
        crate::ThemeColors {
            accent: Some(crate::Rgba::rgb(10, 132, 255)),
            background: Some(crate::Rgba::rgb(30, 30, 30)),
            foreground: Some(crate::Rgba::rgb(255, 255, 255)),
            surface: Some(crate::Rgba::rgb(44, 44, 46)),
            border: Some(crate::Rgba::rgb(56, 56, 58)),
            ..Default::default()
        }
    }

    fn sample_fonts() -> crate::ThemeFonts {
        crate::ThemeFonts {
            family: Some("SF Pro".to_string()),
            size: Some(13.0),
            mono_family: Some("SF Mono".to_string()),
            mono_size: Some(13.0),
        }
    }

    #[test]
    fn build_theme_populates_both_variants() {
        let theme = build_theme(sample_light_colors(), sample_dark_colors(), sample_fonts());

        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_some(), "dark variant should be Some");

        // Colors should differ between variants
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();
        assert_ne!(light.colors.accent, dark.colors.accent);
        assert_ne!(light.colors.background, dark.colors.background);

        // Fonts should be identical in both
        assert_eq!(light.fonts, dark.fonts);
    }

    #[test]
    fn build_theme_name_is_macos() {
        let theme = build_theme(sample_light_colors(), sample_dark_colors(), sample_fonts());
        assert_eq!(theme.name, "macOS");
    }

    #[test]
    fn build_theme_fonts_populated() {
        let fonts = crate::ThemeFonts {
            family: Some("SF Pro".to_string()),
            size: Some(13.0),
            mono_family: Some("SF Mono".to_string()),
            mono_size: Some(13.0),
        };

        let theme =
            build_theme(crate::ThemeColors::default(), crate::ThemeColors::default(), fonts);

        let light = theme.light.as_ref().unwrap();
        assert_eq!(light.fonts.family.as_deref(), Some("SF Pro"));
        assert_eq!(light.fonts.size, Some(13.0));
        assert_eq!(light.fonts.mono_family.as_deref(), Some("SF Mono"));
        assert_eq!(light.fonts.mono_size, Some(13.0));

        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(dark.fonts.family.as_deref(), Some("SF Pro"));
        assert_eq!(dark.fonts.size, Some(13.0));
    }

    #[test]
    fn build_theme_geometry_and_spacing_default() {
        let theme = build_theme(sample_light_colors(), sample_dark_colors(), sample_fonts());

        let light = theme.light.as_ref().unwrap();
        assert!(light.geometry.is_empty(), "light geometry should be default");
        assert!(light.spacing.is_empty(), "light spacing should be default");

        let dark = theme.dark.as_ref().unwrap();
        assert!(dark.geometry.is_empty(), "dark geometry should be default");
        assert!(dark.spacing.is_empty(), "dark spacing should be default");
    }

    #[test]
    fn build_theme_colors_propagated_correctly() {
        let blue = crate::Rgba::rgb(0, 122, 255);
        let red = crate::Rgba::rgb(255, 59, 48);

        let light_colors = crate::ThemeColors {
            accent: Some(blue),
            ..Default::default()
        };
        let dark_colors = crate::ThemeColors {
            accent: Some(red),
            ..Default::default()
        };

        let theme = build_theme(light_colors, dark_colors, crate::ThemeFonts::default());

        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert_eq!(light.colors.accent, Some(blue));
        assert_eq!(dark.colors.accent, Some(red));
    }
}
