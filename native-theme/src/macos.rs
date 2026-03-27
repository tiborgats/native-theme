//! macOS theme reader: reads semantic NSColor values with P3-to-sRGB conversion,
//! resolves both light and dark appearance variants via NSAppearance, and reads
//! system and monospace fonts via NSFont.

// Objective-C FFI via objc2 -- no safe alternative
#![allow(unsafe_code)]

#[cfg(all(target_os = "macos", feature = "macos"))]
use block2::RcBlock;
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_app_kit::{NSAppearance, NSColor, NSColorSpace, NSFont, NSFontWeightRegular};
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

/// Read all semantic NSColor values for the current appearance context into ThemeDefaults.
///
/// Must be called within an `NSAppearance::performAsCurrentDrawingAppearance`
/// block so that dynamic colors resolve to the correct appearance.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_semantic_colors() -> crate::ThemeDefaults {
    let srgb = unsafe { NSColorSpace::sRGBColorSpace() };

    // Bind all NSColor temporaries before borrowing them.
    let label_c = unsafe { NSColor::labelColor() };
    let control_accent = unsafe { NSColor::controlAccentColor() };
    let window_bg = unsafe { NSColor::windowBackgroundColor() };
    let control_bg = unsafe { NSColor::controlBackgroundColor() };
    let separator_c = unsafe { NSColor::separatorColor() };
    let secondary_label = unsafe { NSColor::secondaryLabelColor() };
    let shadow_c = unsafe { NSColor::shadowColor() };
    let alt_sel_text = unsafe { NSColor::alternateSelectedControlTextColor() };
    let control_c = unsafe { NSColor::controlColor() };
    let system_red = unsafe { NSColor::systemRedColor() };
    let system_orange = unsafe { NSColor::systemOrangeColor() };
    let system_green = unsafe { NSColor::systemGreenColor() };
    let system_blue = unsafe { NSColor::systemBlueColor() };
    let sel_content_bg = unsafe { NSColor::selectedContentBackgroundColor() };
    let sel_text = unsafe { NSColor::selectedTextColor() };
    let link_c = unsafe { NSColor::linkColor() };
    let focus_c = unsafe { NSColor::keyboardFocusIndicatorColor() };
    let text_bg = unsafe { NSColor::textBackgroundColor() };
    let text_c = unsafe { NSColor::textColor() };
    let disabled_text = unsafe { NSColor::disabledControlTextColor() };

    let label = nscolor_to_rgba(&label_c, &srgb);

    crate::ThemeDefaults {
        accent: nscolor_to_rgba(&control_accent, &srgb),
        accent_foreground: nscolor_to_rgba(&alt_sel_text, &srgb),
        background: nscolor_to_rgba(&window_bg, &srgb),
        foreground: label,
        surface: nscolor_to_rgba(&control_bg, &srgb),
        border: nscolor_to_rgba(&separator_c, &srgb),
        muted: nscolor_to_rgba(&secondary_label, &srgb),
        shadow: nscolor_to_rgba(&shadow_c, &srgb),
        danger: nscolor_to_rgba(&system_red, &srgb),
        danger_foreground: label,
        warning: nscolor_to_rgba(&system_orange, &srgb),
        warning_foreground: label,
        success: nscolor_to_rgba(&system_green, &srgb),
        success_foreground: label,
        info: nscolor_to_rgba(&system_blue, &srgb),
        info_foreground: label,
        selection: nscolor_to_rgba(&sel_content_bg, &srgb),
        selection_foreground: nscolor_to_rgba(&sel_text, &srgb),
        link: nscolor_to_rgba(&link_c, &srgb),
        focus_ring_color: nscolor_to_rgba(&focus_c, &srgb),
        disabled_foreground: nscolor_to_rgba(&disabled_text, &srgb),
        ..Default::default()
    }
}

/// Read system and monospace font information via NSFont.
///
/// Fonts are appearance-independent, so this only needs to be called once
/// (not per-appearance).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_fonts() -> (crate::FontSpec, crate::FontSpec) {
    let system_size = unsafe { NSFont::systemFontSize() };
    let system_font = unsafe { NSFont::systemFontOfSize(system_size) };
    let mono_font =
        unsafe { NSFont::monospacedSystemFontOfSize_weight(system_size, NSFontWeightRegular) };

    let font = crate::FontSpec {
        family: system_font.familyName().map(|n| n.to_string()),
        size: Some(unsafe { system_font.pointSize() } as f32),
        weight: None,
    };
    let mono = crate::FontSpec {
        family: mono_font.familyName().map(|n| n.to_string()),
        size: Some(unsafe { mono_font.pointSize() } as f32),
        weight: None,
    };
    (font, mono)
}

/// Return per-widget defaults populated from macOS HIG sizes.
///
/// Values based on AppKit intrinsic content sizes and Apple Human Interface
/// Guidelines for standard control dimensions.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn macos_widget_defaults() -> crate::ThemeVariant {
    crate::ThemeVariant {
        button: crate::ButtonTheme {
            min_height: Some(22.0), // NSButton regular control size
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        checkbox: crate::CheckboxTheme {
            indicator_size: Some(14.0), // NSButton switch type
            spacing: Some(4.0),
            ..Default::default()
        },
        input: crate::InputTheme {
            min_height: Some(22.0), // NSTextField regular
            padding_horizontal: Some(4.0),
            ..Default::default()
        },
        scrollbar: crate::ScrollbarTheme {
            width: Some(15.0),       // NSScroller legacy style
            slider_width: Some(7.0), // Overlay style
            ..Default::default()
        },
        slider: crate::SliderTheme {
            track_height: Some(4.0), // NSSlider circular knob
            thumb_size: Some(21.0),
            ..Default::default()
        },
        progress_bar: crate::ProgressBarTheme {
            height: Some(6.0), // NSProgressIndicator regular
            ..Default::default()
        },
        tab: crate::TabTheme {
            min_height: Some(24.0), // NSTabView
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        menu: crate::MenuTheme {
            item_height: Some(22.0), // Standard menu item
            padding_horizontal: Some(12.0),
            ..Default::default()
        },
        tooltip: crate::TooltipTheme {
            padding_horizontal: Some(4.0),
            padding_vertical: Some(4.0),
            ..Default::default()
        },
        list: crate::ListTheme {
            item_height: Some(24.0), // NSTableView row
            padding_horizontal: Some(4.0),
            ..Default::default()
        },
        toolbar: crate::ToolbarTheme {
            height: Some(38.0), // NSToolbar standard
            item_spacing: Some(8.0),
            ..Default::default()
        },
        splitter: crate::SplitterTheme {
            width: Some(9.0), // NSSplitView divider
        },
        ..Default::default()
    }
}

/// Testable core: assemble a NativeTheme from pre-read color and font data.
///
/// Takes pre-resolved ThemeDefaults for both light and dark variants,
/// then constructs the complete NativeTheme. Both variants are always
/// populated (unlike KDE/GNOME/Windows which populate only the active one),
/// since macOS can resolve colors for both appearances.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn build_theme(
    light_defaults: crate::ThemeDefaults,
    dark_defaults: crate::ThemeDefaults,
) -> crate::NativeTheme {
    let widget_defaults = macos_widget_defaults();

    let mut light_variant = widget_defaults.clone();
    light_variant.defaults = light_defaults;
    light_variant.icon_set = Some("sf-symbols".to_string());

    let mut dark_variant = widget_defaults;
    dark_variant.defaults = dark_defaults;
    dark_variant.icon_set = Some("sf-symbols".to_string());

    crate::NativeTheme {
        name: "macOS".to_string(),
        light: Some(light_variant),
        dark: Some(dark_variant),
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

    let (font, mono_font) = read_fonts();

    let light_defaults = if let Some(app) = &light_appearance {
        let defaults = std::cell::RefCell::new(crate::ThemeDefaults::default());
        {
            let block = RcBlock::new(|| {
                *defaults.borrow_mut() = read_semantic_colors();
            });
            app.performAsCurrentDrawingAppearance(&block);
        }
        let mut d = defaults.into_inner();
        d.font = font.clone();
        d.mono_font = mono_font.clone();
        d
    } else {
        crate::ThemeDefaults::default()
    };

    let dark_defaults = if let Some(app) = &dark_appearance {
        let defaults = std::cell::RefCell::new(crate::ThemeDefaults::default());
        {
            let block = RcBlock::new(|| {
                *defaults.borrow_mut() = read_semantic_colors();
            });
            app.performAsCurrentDrawingAppearance(&block);
        }
        let mut d = defaults.into_inner();
        d.font = font;
        d.mono_font = mono_font;
        d
    } else {
        crate::ThemeDefaults::default()
    };

    Ok(build_theme(light_defaults, dark_defaults))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_light_defaults() -> crate::ThemeDefaults {
        crate::ThemeDefaults {
            accent: Some(crate::Rgba::rgb(0, 122, 255)),
            background: Some(crate::Rgba::rgb(246, 246, 246)),
            foreground: Some(crate::Rgba::rgb(0, 0, 0)),
            surface: Some(crate::Rgba::rgb(255, 255, 255)),
            border: Some(crate::Rgba::rgb(200, 200, 200)),
            font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: None,
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".to_string()),
                size: Some(13.0),
                weight: None,
            },
            ..Default::default()
        }
    }

    fn sample_dark_defaults() -> crate::ThemeDefaults {
        crate::ThemeDefaults {
            accent: Some(crate::Rgba::rgb(10, 132, 255)),
            background: Some(crate::Rgba::rgb(30, 30, 30)),
            foreground: Some(crate::Rgba::rgb(255, 255, 255)),
            surface: Some(crate::Rgba::rgb(44, 44, 46)),
            border: Some(crate::Rgba::rgb(56, 56, 58)),
            font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: None,
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".to_string()),
                size: Some(13.0),
                weight: None,
            },
            ..Default::default()
        }
    }

    #[test]
    fn build_theme_populates_both_variants() {
        let theme = build_theme(sample_light_defaults(), sample_dark_defaults());

        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_some(), "dark variant should be Some");

        // Colors should differ between variants
        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();
        assert_ne!(light.defaults.accent, dark.defaults.accent);
        assert_ne!(light.defaults.background, dark.defaults.background);

        // Fonts should be identical in both
        assert_eq!(light.defaults.font, dark.defaults.font);
    }

    #[test]
    fn build_theme_name_is_macos() {
        let theme = build_theme(sample_light_defaults(), sample_dark_defaults());
        assert_eq!(theme.name, "macOS");
    }

    #[test]
    fn build_theme_fonts_populated() {
        let defaults = crate::ThemeDefaults {
            font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: None,
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".to_string()),
                size: Some(13.0),
                weight: None,
            },
            ..Default::default()
        };

        let theme = build_theme(defaults.clone(), defaults);

        let light = theme.light.as_ref().unwrap();
        assert_eq!(light.defaults.font.family.as_deref(), Some("SF Pro"));
        assert_eq!(light.defaults.font.size, Some(13.0));
        assert_eq!(light.defaults.mono_font.family.as_deref(), Some("SF Mono"));
        assert_eq!(light.defaults.mono_font.size, Some(13.0));

        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(dark.defaults.font.family.as_deref(), Some("SF Pro"));
        assert_eq!(dark.defaults.font.size, Some(13.0));
    }

    #[test]
    fn build_theme_defaults_empty_produces_nonempty_variant() {
        let theme = build_theme(crate::ThemeDefaults::default(), crate::ThemeDefaults::default());

        let light = theme.light.as_ref().unwrap();
        // Variant should not be empty because widget defaults are populated
        assert!(!light.is_empty(), "light variant should have widget defaults");

        let dark = theme.dark.as_ref().unwrap();
        assert!(!dark.is_empty(), "dark variant should have widget defaults");
    }

    #[test]
    fn build_theme_colors_propagated_correctly() {
        let blue = crate::Rgba::rgb(0, 122, 255);
        let red = crate::Rgba::rgb(255, 59, 48);

        let light_defaults = crate::ThemeDefaults {
            accent: Some(blue),
            ..Default::default()
        };
        let dark_defaults = crate::ThemeDefaults {
            accent: Some(red),
            ..Default::default()
        };

        let theme = build_theme(light_defaults, dark_defaults);

        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert_eq!(light.defaults.accent, Some(blue));
        assert_eq!(dark.defaults.accent, Some(red));
    }

    #[test]
    fn macos_widget_defaults_spot_check() {
        let wv = macos_widget_defaults();
        assert_eq!(
            wv.button.min_height,
            Some(22.0),
            "NSButton regular control size"
        );
        assert_eq!(wv.scrollbar.width, Some(15.0), "NSScroller legacy style");
        assert_eq!(
            wv.checkbox.indicator_size,
            Some(14.0),
            "NSButton switch type"
        );
        assert_eq!(wv.slider.thumb_size, Some(21.0), "NSSlider circular knob");
    }

    #[test]
    fn build_theme_has_icon_set_sf_symbols() {
        let theme = build_theme(sample_light_defaults(), sample_dark_defaults());

        let light = theme.light.as_ref().unwrap();
        assert_eq!(light.icon_set.as_deref(), Some("sf-symbols"));

        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(dark.icon_set.as_deref(), Some("sf-symbols"));
    }
}
