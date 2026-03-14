//! macOS theme reader: reads semantic NSColor values with P3-to-sRGB conversion,
//! resolves both light and dark appearance variants via NSAppearance, and reads
//! system and monospace fonts via NSFont.

#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_app_kit::{NSAppearance, NSColor, NSColorSpace, NSFont, NSFontWeight};
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_foundation::NSString;

/// Convert an NSColor to sRGB and extract RGBA components.
///
/// Converts the color to the sRGB color space via `colorUsingColorSpace`,
/// handling P3-to-sRGB conversion automatically. Returns `None` if the
/// color cannot be converted (e.g., pattern colors).
#[cfg(all(target_os = "macos", feature = "macos"))]
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
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_semantic_colors() -> crate::ThemeColors {
    let srgb = unsafe { NSColorSpace::sRGBColorSpace() };

    // Bind all NSColor temporaries before borrowing them.
    // Rust 2024 edition drops temporaries inside `unsafe {}` blocks at the
    // end of the block, so `nscolor_to_rgba(unsafe { &NSColor::foo() }, …)`
    // would reference a dropped value.
    let label_c = unsafe { NSColor::labelColor() };
    let control_accent = unsafe { NSColor::controlAccentColor() };
    let window_bg = unsafe { NSColor::windowBackgroundColor() };
    let control_bg = unsafe { NSColor::controlBackgroundColor() };
    let separator_c = unsafe { NSColor::separatorColor() };
    let secondary_label = unsafe { NSColor::secondaryLabelColor() };
    let shadow_c = unsafe { NSColor::shadowColor() };
    let alt_sel_text = unsafe { NSColor::alternateSelectedControlTextColor() };
    let control_c = unsafe { NSColor::controlColor() };
    let control_text = unsafe { NSColor::controlTextColor() };
    let system_red = unsafe { NSColor::systemRedColor() };
    let system_orange = unsafe { NSColor::systemOrangeColor() };
    let system_green = unsafe { NSColor::systemGreenColor() };
    let system_blue = unsafe { NSColor::systemBlueColor() };
    let sel_content_bg = unsafe { NSColor::selectedContentBackgroundColor() };
    let sel_text = unsafe { NSColor::selectedTextColor() };
    let link_c = unsafe { NSColor::linkColor() };
    let focus_c = unsafe { NSColor::keyboardFocusIndicatorColor() };
    let under_page_bg = unsafe { NSColor::underPageBackgroundColor() };
    let text_bg = unsafe { NSColor::textBackgroundColor() };
    let text_c = unsafe { NSColor::textColor() };
    let disabled_text = unsafe { NSColor::disabledControlTextColor() };

    let label = nscolor_to_rgba(&label_c, &srgb);

    crate::ThemeColors {
        // Core (7)
        accent: nscolor_to_rgba(&control_accent, &srgb),
        background: nscolor_to_rgba(&window_bg, &srgb),
        foreground: label,
        surface: nscolor_to_rgba(&control_bg, &srgb),
        border: nscolor_to_rgba(&separator_c, &srgb),
        muted: nscolor_to_rgba(&secondary_label, &srgb),
        shadow: nscolor_to_rgba(&shadow_c, &srgb),
        // Primary (2)
        primary_background: nscolor_to_rgba(&control_accent, &srgb),
        primary_foreground: nscolor_to_rgba(&alt_sel_text, &srgb),
        // Secondary (2)
        secondary_background: nscolor_to_rgba(&control_c, &srgb),
        secondary_foreground: nscolor_to_rgba(&control_text, &srgb),
        // Status (8)
        danger: nscolor_to_rgba(&system_red, &srgb),
        danger_foreground: label,
        warning: nscolor_to_rgba(&system_orange, &srgb),
        warning_foreground: label,
        success: nscolor_to_rgba(&system_green, &srgb),
        success_foreground: label,
        info: nscolor_to_rgba(&system_blue, &srgb),
        info_foreground: label,
        // Interactive (4)
        selection: nscolor_to_rgba(&sel_content_bg, &srgb),
        selection_foreground: nscolor_to_rgba(&sel_text, &srgb),
        link: nscolor_to_rgba(&link_c, &srgb),
        focus_ring: nscolor_to_rgba(&focus_c, &srgb),
        // Panel (6)
        sidebar: nscolor_to_rgba(&under_page_bg, &srgb),
        sidebar_foreground: label,
        tooltip: nscolor_to_rgba(&window_bg, &srgb),
        tooltip_foreground: label,
        popover: nscolor_to_rgba(&window_bg, &srgb),
        popover_foreground: label,
        // Component (7)
        button: nscolor_to_rgba(&control_c, &srgb),
        button_foreground: nscolor_to_rgba(&control_text, &srgb),
        input: nscolor_to_rgba(&text_bg, &srgb),
        input_foreground: nscolor_to_rgba(&text_c, &srgb),
        disabled: nscolor_to_rgba(&disabled_text, &srgb),
        separator: nscolor_to_rgba(&separator_c, &srgb),
        alternate_row: nscolor_to_rgba(&control_bg, &srgb),
    }
}

/// Read system and monospace font information via NSFont.
///
/// Fonts are appearance-independent, so this only needs to be called once
/// (not per-appearance).
#[cfg(all(target_os = "macos", feature = "macos"))]
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

/// Return widget metrics populated from macOS HIG defaults.
///
/// Values based on AppKit intrinsic content sizes and Apple Human Interface
/// Guidelines for standard control dimensions.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn macos_widget_metrics() -> crate::model::widget_metrics::WidgetMetrics {
    use crate::model::widget_metrics::*;

    WidgetMetrics {
        button: ButtonMetrics {
            min_height: Some(22.0), // NSButton regular control size
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(14.0), // NSButton switch type
            spacing: Some(4.0),
            ..Default::default()
        },
        input: InputMetrics {
            min_height: Some(22.0), // NSTextField regular
            padding_horizontal: Some(4.0),
            ..Default::default()
        },
        scrollbar: ScrollbarMetrics {
            width: Some(15.0),       // NSScroller legacy style
            slider_width: Some(7.0), // Overlay style
            ..Default::default()
        },
        slider: SliderMetrics {
            track_height: Some(4.0), // NSSlider circular knob
            thumb_size: Some(21.0),
            ..Default::default()
        },
        progress_bar: ProgressBarMetrics {
            height: Some(6.0), // NSProgressIndicator regular
            ..Default::default()
        },
        tab: TabMetrics {
            min_height: Some(24.0), // NSTabView
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        menu_item: MenuItemMetrics {
            height: Some(22.0), // Standard menu item
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        tooltip: TooltipMetrics {
            padding: Some(4.0),
            ..Default::default()
        },
        list_item: ListItemMetrics {
            height: Some(24.0), // NSTableView row
            padding_horizontal: Some(4.0),
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            height: Some(38.0), // NSToolbar standard
            item_spacing: Some(8.0),
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(9.0), // NSSplitView divider
        },
    }
}

/// Testable core: assemble a NativeTheme from pre-read color and font data.
///
/// Takes pre-resolved colors for both light and dark variants and fonts,
/// then constructs the complete NativeTheme. Both variants are always
/// populated (unlike KDE/GNOME/Windows which populate only the active one),
/// since macOS can resolve colors for both appearances.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn build_theme(
    light_colors: crate::ThemeColors,
    dark_colors: crate::ThemeColors,
    fonts: crate::ThemeFonts,
) -> crate::NativeTheme {
    let wm = macos_widget_metrics();

    crate::NativeTheme {
        name: "macOS".to_string(),
        light: Some(crate::ThemeVariant {
            colors: light_colors,
            fonts: fonts.clone(),
            geometry: Default::default(),
            spacing: Default::default(),
            widget_metrics: Some(wm.clone()),
            icon_set: None,
        }),
        dark: Some(crate::ThemeVariant {
            colors: dark_colors,
            fonts,
            geometry: Default::default(),
            spacing: Default::default(),
            widget_metrics: Some(wm),
            icon_set: None,
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
#[cfg(all(target_os = "macos", feature = "macos"))]
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

        let theme = build_theme(
            crate::ThemeColors::default(),
            crate::ThemeColors::default(),
            fonts,
        );

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
        assert!(
            light.geometry.is_empty(),
            "light geometry should be default"
        );
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

    #[test]
    fn macos_widget_metrics_spot_check() {
        let wm = macos_widget_metrics();
        assert_eq!(
            wm.button.min_height,
            Some(22.0),
            "NSButton regular control size"
        );
        assert_eq!(wm.scrollbar.width, Some(15.0), "NSScroller legacy style");
        assert_eq!(
            wm.checkbox.indicator_size,
            Some(14.0),
            "NSButton switch type"
        );
        assert_eq!(wm.slider.thumb_size, Some(21.0), "NSSlider circular knob");
    }

    #[test]
    fn build_theme_populates_widget_metrics() {
        let theme = build_theme(sample_light_colors(), sample_dark_colors(), sample_fonts());

        let light = theme.light.as_ref().unwrap();
        assert!(
            light.widget_metrics.is_some(),
            "light widget_metrics should be Some"
        );

        let dark = theme.dark.as_ref().unwrap();
        assert!(
            dark.widget_metrics.is_some(),
            "dark widget_metrics should be Some"
        );
    }
}
