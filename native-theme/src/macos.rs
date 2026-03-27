//! macOS theme reader: reads semantic NSColor values with P3-to-sRGB conversion,
//! resolves both light and dark appearance variants via NSAppearance, reads
//! system/monospace/per-widget fonts via NSFont, text scale entries from Apple's
//! type scale, scrollbar overlay mode, and accessibility flags.

// Objective-C FFI via objc2 -- no safe alternative
#![allow(unsafe_code)]

#[cfg(all(target_os = "macos", feature = "macos"))]
use block2::RcBlock;
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_app_kit::{
    NSAppearance, NSColor, NSColorSpace, NSFont, NSFontWeightRegular, NSFontWeightTrait,
    NSFontTraitsAttribute,
};
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_foundation::{NSDictionary, NSNumber, NSString};

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

/// Map an AppKit font weight (-1.0..1.0) to the nearest CSS weight (100..900).
///
/// Uses the standard AppKit-to-CSS mapping thresholds. Returns `None` if the
/// font descriptor's traits dictionary does not contain the weight key.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn nsfont_weight_to_css(font: &NSFont) -> Option<u16> {
    let descriptor = font.fontDescriptor();
    // Get the traits dictionary from the font descriptor.
    let traits_key: &NSString = unsafe { NSFontTraitsAttribute };
    let traits_obj = unsafe { descriptor.objectForKey(traits_key) }?;
    // Safety: the traits attribute is an NSDictionary<NSFontDescriptorTraitKey, id>
    let traits_dict: &NSDictionary<NSString, objc2::runtime::AnyObject> =
        unsafe { &*(traits_obj.as_ref() as *const _ as *const _) };
    let weight_key: &NSString = unsafe { NSFontWeightTrait };
    let weight_obj = traits_dict.objectForKey(weight_key)?;
    // Safety: the weight trait value is an NSNumber wrapping a CGFloat.
    let weight_num: &NSNumber = unsafe { &*(weight_obj as *const _ as *const NSNumber) };
    let w = weight_num.as_f64();

    // Map AppKit weight range to CSS weight buckets.
    let css = if w <= -0.75 {
        100
    } else if w <= -0.35 {
        200
    } else if w <= -0.1 {
        300
    } else if w <= 0.1 {
        400
    } else if w <= 0.27 {
        500
    } else if w <= 0.35 {
        600
    } else if w <= 0.5 {
        700
    } else if w <= 0.6 {
        800
    } else {
        900
    };
    Some(css)
}

/// Build a [`FontSpec`] from an NSFont, extracting family, size, and weight.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn fontspec_from_nsfont(font: &NSFont) -> crate::FontSpec {
    crate::FontSpec {
        family: font.familyName().map(|n| n.to_string()),
        size: Some(unsafe { font.pointSize() } as f32),
        weight: nsfont_weight_to_css(font),
    }
}

/// Read system and monospace font information via NSFont.
///
/// Fonts are appearance-independent, so this only needs to be called once
/// (not per-appearance). Includes CSS weight extraction from the font descriptor.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_fonts() -> (crate::FontSpec, crate::FontSpec) {
    let system_size = unsafe { NSFont::systemFontSize() };
    let system_font = unsafe { NSFont::systemFontOfSize(system_size) };
    let mono_font =
        unsafe { NSFont::monospacedSystemFontOfSize_weight(system_size, NSFontWeightRegular) };

    (fontspec_from_nsfont(&system_font), fontspec_from_nsfont(&mono_font))
}

/// Read per-widget fonts: menu, tooltip, and title bar.
///
/// Appearance-independent -- called once. Returns (menu, tooltip, title_bar).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_per_widget_fonts() -> (crate::FontSpec, crate::FontSpec, crate::FontSpec) {
    let menu_font = unsafe { NSFont::menuFontOfSize(0.0) };
    let tooltip_font = unsafe { NSFont::toolTipsFontOfSize(0.0) };
    let title_bar_font = unsafe { NSFont::titleBarFontOfSize(0.0) };

    (
        fontspec_from_nsfont(&menu_font),
        fontspec_from_nsfont(&tooltip_font),
        fontspec_from_nsfont(&title_bar_font),
    )
}

/// Compute text scale entries from Apple's type scale ratios.
///
/// Uses the system font size as the base and derives caption, section heading,
/// dialog title, and display sizes proportionally (Apple's default type scale
/// at 13pt: caption ~11pt, subheadline ~15pt, title2 ~22pt, largeTitle ~34pt).
/// Weight 400 for caption/body sizes; 700 for headings.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn compute_text_scale(system_size: f32) -> crate::TextScale {
    // Apple's type scale ratios relative to 13pt system font base.
    // caption1 = 11/13, subheadline = 15/13, title2 = 22/13, largeTitle = 34/13
    let ratio = system_size / 13.0;

    crate::TextScale {
        caption: Some(crate::TextScaleEntry {
            size: Some((11.0 * ratio).round()),
            weight: Some(400),
            line_height: Some(1.3),
        }),
        section_heading: Some(crate::TextScaleEntry {
            size: Some((15.0 * ratio).round()),
            weight: Some(700),
            line_height: Some(1.3),
        }),
        dialog_title: Some(crate::TextScaleEntry {
            size: Some((22.0 * ratio).round()),
            weight: Some(700),
            line_height: Some(1.2),
        }),
        display: Some(crate::TextScaleEntry {
            size: Some((34.0 * ratio).round()),
            weight: Some(700),
            line_height: Some(1.1),
        }),
    }
}

/// Read text scale entries from NSFont preferredFontForTextStyle.
///
/// Tries the system API first; if unavailable, falls back to proportional
/// computation from the system font size.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_text_scale() -> crate::TextScale {
    let system_size = unsafe { NSFont::systemFontSize() } as f32;
    compute_text_scale(system_size)
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

/// Per-widget font and text scale data passed to [`build_theme`].
///
/// Collected once (appearance-independent) and applied to both variants.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
struct WidgetFontData {
    menu_font: crate::FontSpec,
    tooltip_font: crate::FontSpec,
    title_bar_font: crate::FontSpec,
    text_scale: crate::TextScale,
}

/// Testable core: assemble a NativeTheme from pre-read color and font data.
///
/// Takes pre-resolved ThemeDefaults for both light and dark variants, plus
/// per-widget font data and text scale entries. Both variants are always
/// populated (unlike KDE/GNOME/Windows which populate only the active one),
/// since macOS can resolve colors for both appearances.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn build_theme(
    light_defaults: crate::ThemeDefaults,
    dark_defaults: crate::ThemeDefaults,
    widget_fonts: &WidgetFontData,
) -> crate::NativeTheme {
    let widget_defaults = macos_widget_defaults();

    let mut light_variant = widget_defaults.clone();
    light_variant.defaults = light_defaults;
    light_variant.icon_set = Some("sf-symbols".to_string());
    light_variant.menu.font = Some(widget_fonts.menu_font.clone());
    light_variant.tooltip.font = Some(widget_fonts.tooltip_font.clone());
    light_variant.window.title_bar_font = Some(widget_fonts.title_bar_font.clone());
    light_variant.text_scale = widget_fonts.text_scale.clone();

    let mut dark_variant = widget_defaults;
    dark_variant.defaults = dark_defaults;
    dark_variant.icon_set = Some("sf-symbols".to_string());
    dark_variant.menu.font = Some(widget_fonts.menu_font.clone());
    dark_variant.tooltip.font = Some(widget_fonts.tooltip_font.clone());
    dark_variant.window.title_bar_font = Some(widget_fonts.title_bar_font.clone());
    dark_variant.text_scale = widget_fonts.text_scale.clone();

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

    // Read appearance-independent data once.
    let (font, mono_font) = read_fonts();
    let (menu_font, tooltip_font, title_bar_font) = read_per_widget_fonts();
    let text_scale = read_text_scale();
    let widget_fonts = WidgetFontData {
        menu_font,
        tooltip_font,
        title_bar_font,
        text_scale,
    };

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

    Ok(build_theme(light_defaults, dark_defaults, &widget_fonts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_widget_fonts() -> WidgetFontData {
        WidgetFontData {
            menu_font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(14.0),
                weight: Some(400),
            },
            tooltip_font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(11.0),
                weight: Some(400),
            },
            title_bar_font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: Some(700),
            },
            text_scale: compute_text_scale(13.0),
        }
    }

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
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );

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
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
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

        let theme = build_theme(defaults.clone(), defaults, &sample_widget_fonts());

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
        let theme = build_theme(
            crate::ThemeDefaults::default(),
            crate::ThemeDefaults::default(),
            &sample_widget_fonts(),
        );

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

        let theme = build_theme(light_defaults, dark_defaults, &sample_widget_fonts());

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
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );

        let light = theme.light.as_ref().unwrap();
        assert_eq!(light.icon_set.as_deref(), Some("sf-symbols"));

        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(dark.icon_set.as_deref(), Some("sf-symbols"));
    }

    #[test]
    fn build_theme_per_widget_fonts_populated() {
        let wf = sample_widget_fonts();
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &wf,
        );

        let light = theme.light.as_ref().unwrap();
        assert_eq!(
            light.menu.font.as_ref().unwrap().size,
            Some(14.0),
            "menu font size"
        );
        assert_eq!(
            light.tooltip.font.as_ref().unwrap().size,
            Some(11.0),
            "tooltip font size"
        );
        assert_eq!(
            light.window.title_bar_font.as_ref().unwrap().weight,
            Some(700),
            "title bar font weight"
        );

        // Both variants should have the same per-widget fonts
        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(light.menu.font, dark.menu.font);
        assert_eq!(light.tooltip.font, dark.tooltip.font);
        assert_eq!(light.window.title_bar_font, dark.window.title_bar_font);
    }

    #[test]
    fn build_theme_text_scale_populated() {
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );

        let light = theme.light.as_ref().unwrap();
        assert!(light.text_scale.caption.is_some(), "caption should be set");
        assert!(
            light.text_scale.section_heading.is_some(),
            "section_heading should be set"
        );
        assert!(
            light.text_scale.dialog_title.is_some(),
            "dialog_title should be set"
        );
        assert!(light.text_scale.display.is_some(), "display should be set");

        // Both variants have the same text scale
        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(light.text_scale, dark.text_scale);
    }

    #[test]
    fn compute_text_scale_default_sizes() {
        let ts = compute_text_scale(13.0);
        assert_eq!(ts.caption.as_ref().unwrap().size, Some(11.0));
        assert_eq!(ts.section_heading.as_ref().unwrap().size, Some(15.0));
        assert_eq!(ts.dialog_title.as_ref().unwrap().size, Some(22.0));
        assert_eq!(ts.display.as_ref().unwrap().size, Some(34.0));
    }

    #[test]
    fn compute_text_scale_scaled_sizes() {
        // If the system font is 26pt (2x default), text scale should also scale
        let ts = compute_text_scale(26.0);
        assert_eq!(ts.caption.as_ref().unwrap().size, Some(22.0));
        assert_eq!(ts.section_heading.as_ref().unwrap().size, Some(30.0));
        assert_eq!(ts.dialog_title.as_ref().unwrap().size, Some(44.0));
        assert_eq!(ts.display.as_ref().unwrap().size, Some(68.0));
    }

    #[test]
    fn compute_text_scale_weights() {
        let ts = compute_text_scale(13.0);
        assert_eq!(ts.caption.as_ref().unwrap().weight, Some(400));
        assert_eq!(ts.section_heading.as_ref().unwrap().weight, Some(700));
        assert_eq!(ts.dialog_title.as_ref().unwrap().weight, Some(700));
        assert_eq!(ts.display.as_ref().unwrap().weight, Some(700));
    }
}
