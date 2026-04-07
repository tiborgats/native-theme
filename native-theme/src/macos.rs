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
    NSAppearance, NSColor, NSColorSpace, NSFont, NSFontTraitsAttribute, NSFontWeightRegular,
    NSFontWeightTrait, NSScroller, NSScrollerStyle, NSWorkspace,
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
    let srgb_color = color.colorUsingColorSpace(srgb)?;
    let r = srgb_color.redComponent() as f32;
    let g = srgb_color.greenComponent() as f32;
    let b = srgb_color.blueComponent() as f32;
    let a = srgb_color.alphaComponent() as f32;
    Some(crate::Rgba::from_f32(r, g, b, a))
}

/// Per-widget colors read from appearance-dependent NSColor APIs.
#[cfg(all(target_os = "macos", feature = "macos"))]
#[derive(Default)]
struct PerWidgetColors {
    placeholder: Option<crate::Rgba>,
    selection_inactive: Option<crate::Rgba>,
    alternate_row: Option<crate::Rgba>,
    header_foreground: Option<crate::Rgba>,
    grid_color: Option<crate::Rgba>,
    title_bar_foreground: Option<crate::Rgba>,
}

/// Read all semantic NSColor values for the current appearance context.
///
/// Returns both the ThemeDefaults and per-widget color data. Must be called
/// within an `NSAppearance::performAsCurrentDrawingAppearance` block so that
/// dynamic colors resolve to the correct appearance.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_appearance_colors() -> (crate::ThemeDefaults, PerWidgetColors) {
    let srgb = NSColorSpace::sRGBColorSpace();

    // Bind all NSColor temporaries before borrowing them.
    let label_c = NSColor::labelColor();
    let control_accent = NSColor::controlAccentColor();
    let window_bg = NSColor::windowBackgroundColor();
    let control_bg = NSColor::controlBackgroundColor();
    let separator_c = NSColor::separatorColor();
    let secondary_label = NSColor::secondaryLabelColor();
    let shadow_c = NSColor::shadowColor();
    let alt_sel_text = NSColor::alternateSelectedControlTextColor();
    let system_red = NSColor::systemRedColor();
    let system_orange = NSColor::systemOrangeColor();
    let system_green = NSColor::systemGreenColor();
    let system_blue = NSColor::systemBlueColor();
    let sel_content_bg = NSColor::selectedContentBackgroundColor();
    let sel_text = NSColor::selectedTextColor();
    let link_c = NSColor::linkColor();
    let focus_c = NSColor::keyboardFocusIndicatorColor();
    let disabled_text = NSColor::disabledControlTextColor();

    // Additional per-widget color bindings (MACOS-03).
    let placeholder_c = NSColor::placeholderTextColor();
    let unemph_sel_bg = NSColor::unemphasizedSelectedContentBackgroundColor();
    let alt_bg_colors = NSColor::alternatingContentBackgroundColors();
    let header_text_c = NSColor::headerTextColor();
    let grid_c = NSColor::gridColor();
    let frame_text_c = NSColor::windowFrameTextColor();

    let label = nscolor_to_rgba(&label_c, &srgb);

    let defaults = crate::ThemeDefaults {
        accent_color: nscolor_to_rgba(&control_accent, &srgb),
        accent_text_color: nscolor_to_rgba(&alt_sel_text, &srgb),
        background_color: nscolor_to_rgba(&window_bg, &srgb),
        text_color: label,
        surface_color: nscolor_to_rgba(&control_bg, &srgb),
        muted_color: nscolor_to_rgba(&secondary_label, &srgb),
        shadow_color: nscolor_to_rgba(&shadow_c, &srgb),
        danger_color: nscolor_to_rgba(&system_red, &srgb),
        danger_text_color: label,
        warning_color: nscolor_to_rgba(&system_orange, &srgb),
        warning_text_color: label,
        success_color: nscolor_to_rgba(&system_green, &srgb),
        success_text_color: label,
        info_color: nscolor_to_rgba(&system_blue, &srgb),
        info_text_color: label,
        selection_background: nscolor_to_rgba(&sel_content_bg, &srgb),
        selection_text_color: nscolor_to_rgba(&sel_text, &srgb),
        selection_inactive_background: nscolor_to_rgba(&unemph_sel_bg, &srgb),
        link_color: nscolor_to_rgba(&link_c, &srgb),
        focus_ring_color: nscolor_to_rgba(&focus_c, &srgb),
        disabled_text_color: nscolor_to_rgba(&disabled_text, &srgb),
        ..Default::default()
    };

    // Alternate row: index 1 of alternatingContentBackgroundColors (index 0 is normal).
    let alternate_row = if alt_bg_colors.count() >= 2 {
        nscolor_to_rgba(&alt_bg_colors.objectAtIndex(1), &srgb)
    } else {
        None
    };

    let per_widget = PerWidgetColors {
        placeholder: nscolor_to_rgba(&placeholder_c, &srgb),
        selection_inactive: nscolor_to_rgba(&unemph_sel_bg, &srgb),
        alternate_row,
        header_foreground: nscolor_to_rgba(&header_text_c, &srgb),
        grid_color: nscolor_to_rgba(&grid_c, &srgb),
        title_bar_foreground: nscolor_to_rgba(&frame_text_c, &srgb),
    };

    (defaults, per_widget)
}

/// Read scrollbar overlay mode from NSScroller.preferredScrollerStyle.
///
/// Returns `true` if the preferred scroller style is overlay, `false` for legacy.
/// Requires main thread (MainThreadMarker).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_scrollbar_style(mtm: objc2::MainThreadMarker) -> Option<bool> {
    Some(NSScroller::preferredScrollerStyle(mtm) == NSScrollerStyle::Overlay)
}

/// Read accessibility flags from NSWorkspace.
///
/// Returns (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor).
/// text_scaling_factor is derived by comparing the system font size to the default (13pt).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_accessibility() -> (Option<bool>, Option<bool>, Option<bool>, Option<f32>) {
    let workspace = NSWorkspace::sharedWorkspace();
    let reduce_motion = Some(workspace.accessibilityDisplayShouldReduceMotion());
    let high_contrast = Some(workspace.accessibilityDisplayShouldIncreaseContrast());
    let reduce_transparency = Some(workspace.accessibilityDisplayShouldReduceTransparency());

    // Derive text scaling factor from system font size vs default 13pt.
    let system_size = NSFont::systemFontSize() as f32;
    let text_scaling_factor = if (system_size - 13.0).abs() > 0.01 {
        Some(system_size / 13.0)
    } else {
        None
    };

    (
        reduce_motion,
        high_contrast,
        reduce_transparency,
        text_scaling_factor,
    )
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
    let traits_obj = descriptor.objectForKey(traits_key)?;
    // Safety: the traits attribute is an NSDictionary<NSFontDescriptorTraitKey, id>
    let traits_dict: &NSDictionary<NSString, objc2::runtime::AnyObject> =
        unsafe { &*(&*traits_obj as *const _ as *const _) };
    let weight_key: &NSString = unsafe { NSFontWeightTrait };
    let weight_obj = traits_dict.objectForKey(weight_key)?;
    // Safety: the weight trait value is an NSNumber wrapping a CGFloat.
    let weight_num: &NSNumber = unsafe { &*(&*weight_obj as *const _ as *const NSNumber) };
    let w = weight_num.doubleValue();

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
        size: Some(font.pointSize() as f32),
        weight: nsfont_weight_to_css(font),
        ..Default::default()
    }
}

/// Read system and monospace font information via NSFont.
///
/// Fonts are appearance-independent, so this only needs to be called once
/// (not per-appearance). Includes CSS weight extraction from the font descriptor.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_fonts() -> (crate::FontSpec, crate::FontSpec) {
    let system_size = NSFont::systemFontSize();
    let system_font = NSFont::systemFontOfSize(system_size);
    let mono_font =
        unsafe { NSFont::monospacedSystemFontOfSize_weight(system_size, NSFontWeightRegular) };

    (
        fontspec_from_nsfont(&system_font),
        fontspec_from_nsfont(&mono_font),
    )
}

/// Read per-widget fonts: menu, tooltip, and title bar.
///
/// Appearance-independent -- called once. Returns (menu, tooltip, title_bar).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_per_widget_fonts() -> (crate::FontSpec, crate::FontSpec, crate::FontSpec) {
    let menu_font = NSFont::menuFontOfSize(0.0);
    let tooltip_font = NSFont::toolTipsFontOfSize(0.0);
    let title_bar_font = NSFont::titleBarFontOfSize(0.0);

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
    let system_size = NSFont::systemFontSize() as f32;
    compute_text_scale(system_size)
}

/// Return per-widget defaults populated from macOS HIG sizes.
///
/// Values based on AppKit intrinsic content sizes and Apple Human Interface
/// Guidelines for standard control dimensions.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn macos_widget_defaults() -> crate::ThemeVariant {
    use crate::model::border::BorderSpec;
    crate::ThemeVariant {
        button: crate::ButtonTheme {
            min_height: Some(22.0), // NSButton regular control size
            border: Some(BorderSpec {
                padding_horizontal: Some(12.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        checkbox: crate::CheckboxTheme {
            indicator_width: Some(14.0), // NSButton switch type
            label_gap: Some(4.0),
            ..Default::default()
        },
        input: crate::InputTheme {
            min_height: Some(22.0), // NSTextField regular
            border: Some(BorderSpec {
                padding_horizontal: Some(4.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        scrollbar: crate::ScrollbarTheme {
            groove_width: Some(15.0), // NSScroller legacy style
            thumb_width: Some(7.0),   // Overlay style
            ..Default::default()
        },
        slider: crate::SliderTheme {
            track_height: Some(4.0), // NSSlider circular knob
            thumb_diameter: Some(21.0),
            ..Default::default()
        },
        progress_bar: crate::ProgressBarTheme {
            track_height: Some(6.0), // NSProgressIndicator regular
            ..Default::default()
        },
        tab: crate::TabTheme {
            min_height: Some(24.0), // NSTabView
            border: Some(BorderSpec {
                padding_horizontal: Some(12.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        menu: crate::MenuTheme {
            row_height: Some(22.0), // Standard menu item
            border: Some(BorderSpec {
                padding_horizontal: Some(12.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        tooltip: crate::TooltipTheme {
            border: Some(BorderSpec {
                padding_horizontal: Some(4.0),
                padding_vertical: Some(4.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        list: crate::ListTheme {
            row_height: Some(24.0), // NSTableView row
            border: Some(BorderSpec {
                padding_horizontal: Some(4.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        toolbar: crate::ToolbarTheme {
            bar_height: Some(38.0), // NSToolbar standard
            item_gap: Some(8.0),
            ..Default::default()
        },
        splitter: crate::SplitterTheme {
            divider_width: Some(9.0), // NSSplitView divider
            ..Default::default()
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

/// Testable core: assemble a ThemeSpec from pre-read color and font data.
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
) -> crate::ThemeSpec {
    let widget_defaults = macos_widget_defaults();

    let mut light_variant = widget_defaults.clone();
    light_variant.defaults = light_defaults;
    light_variant.icon_set = Some(crate::IconSet::SfSymbols);
    light_variant.menu.font = Some(widget_fonts.menu_font.clone());
    light_variant.tooltip.font = Some(widget_fonts.tooltip_font.clone());
    light_variant.window.title_bar_font = Some(widget_fonts.title_bar_font.clone());
    light_variant.text_scale = widget_fonts.text_scale.clone();

    let mut dark_variant = widget_defaults;
    dark_variant.defaults = dark_defaults;
    dark_variant.icon_set = Some(crate::IconSet::SfSymbols);
    dark_variant.menu.font = Some(widget_fonts.menu_font.clone());
    dark_variant.tooltip.font = Some(widget_fonts.tooltip_font.clone());
    dark_variant.window.title_bar_font = Some(widget_fonts.title_bar_font.clone());
    dark_variant.text_scale = widget_fonts.text_scale.clone();

    crate::ThemeSpec {
        name: "macOS".to_string(),
        light: Some(light_variant),
        dark: Some(dark_variant),
        layout: crate::LayoutTheme::default(),
    }
}

/// Read the current macOS theme, resolving both light and dark appearance variants.
///
/// Uses `NSAppearance::performAsCurrentDrawingAppearance` (macOS 11+) to scope
/// semantic color resolution to each appearance. Reads ~25 NSColor semantic
/// colors per variant with P3-to-sRGB conversion, per-widget fonts with weight,
/// text scale entries, scrollbar overlay mode, and accessibility flags.
///
/// # Errors
///
/// Returns `Error::Unavailable` if neither light nor dark appearance can be created
/// (extremely unlikely on any macOS version that supports these APIs).
#[cfg(all(target_os = "macos", feature = "macos"))]
#[must_use = "this returns the detected macOS theme; it does not apply it"]
pub fn from_macos() -> crate::Result<crate::ThemeSpec> {
    let light_name = NSString::from_str("NSAppearanceNameAqua");
    let dark_name = NSString::from_str("NSAppearanceNameDarkAqua");

    let light_appearance = NSAppearance::appearanceNamed(&light_name);
    let dark_appearance = NSAppearance::appearanceNamed(&dark_name);

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

    // Type alias for the appearance-block data.
    type AppearanceData = (crate::ThemeDefaults, PerWidgetColors);

    let (light_defaults, light_pw) = if let Some(app) = &light_appearance {
        let data = std::cell::RefCell::new(None::<AppearanceData>);
        {
            let block = RcBlock::new(|| {
                *data.borrow_mut() = Some(read_appearance_colors());
            });
            app.performAsCurrentDrawingAppearance(&block);
        }
        let (mut d, pw) = data.into_inner().unwrap_or_default();
        d.font = font.clone();
        d.mono_font = mono_font.clone();
        (d, Some(pw))
    } else {
        (crate::ThemeDefaults::default(), None)
    };

    let (dark_defaults, dark_pw) = if let Some(app) = &dark_appearance {
        let data = std::cell::RefCell::new(None::<AppearanceData>);
        {
            let block = RcBlock::new(|| {
                *data.borrow_mut() = Some(read_appearance_colors());
            });
            app.performAsCurrentDrawingAppearance(&block);
        }
        let (mut d, pw) = data.into_inner().unwrap_or_default();
        d.font = font;
        d.mono_font = mono_font;
        (d, Some(pw))
    } else {
        (crate::ThemeDefaults::default(), None)
    };

    let mut theme = build_theme(light_defaults, dark_defaults, &widget_fonts);

    // Apply per-widget colors (appearance-dependent, per variant).
    if let (Some(v), Some(pw)) = (&mut theme.light, light_pw) {
        v.input.placeholder_color = pw.placeholder;
        v.input.selection_background = pw.selection_inactive;
        v.list.alternate_row_background = pw.alternate_row;
        if let Some(color) = pw.header_foreground {
            v.list.header_font.get_or_insert_default().color = Some(color);
        }
        v.list.grid_color = pw.grid_color;
        if let Some(color) = pw.title_bar_foreground {
            v.window.title_bar_font.get_or_insert_default().color = Some(color);
        }
    }
    if let (Some(v), Some(pw)) = (&mut theme.dark, dark_pw) {
        v.input.placeholder_color = pw.placeholder;
        v.input.selection_background = pw.selection_inactive;
        v.list.alternate_row_background = pw.alternate_row;
        if let Some(color) = pw.header_foreground {
            v.list.header_font.get_or_insert_default().color = Some(color);
        }
        v.list.grid_color = pw.grid_color;
        if let Some(color) = pw.title_bar_foreground {
            v.window.title_bar_font.get_or_insert_default().color = Some(color);
        }
    }

    // Scrollbar overlay mode (appearance-independent, requires main thread).
    let overlay_mode = objc2::MainThreadMarker::new().and_then(read_scrollbar_style);
    if let Some(v) = &mut theme.light {
        v.scrollbar.overlay_mode = overlay_mode;
    }
    if let Some(v) = &mut theme.dark {
        v.scrollbar.overlay_mode = overlay_mode;
    }

    // Accessibility flags (appearance-independent).
    let (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor) =
        read_accessibility();
    for variant in [&mut theme.light, &mut theme.dark] {
        if let Some(v) = variant {
            v.defaults.reduce_motion = reduce_motion;
            v.defaults.high_contrast = high_contrast;
            v.defaults.reduce_transparency = reduce_transparency;
            v.defaults.text_scaling_factor = text_scaling_factor;
            // macOS uses leading affirmative (OK/Cancel) dialog button order.
            v.dialog.button_order = Some(crate::DialogButtonOrder::PrimaryLeft);
        }
    }

    Ok(theme)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn sample_widget_fonts() -> WidgetFontData {
        WidgetFontData {
            menu_font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(14.0),
                weight: Some(400),
                ..Default::default()
            },
            tooltip_font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(11.0),
                weight: Some(400),
                ..Default::default()
            },
            title_bar_font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: Some(700),
                ..Default::default()
            },
            text_scale: compute_text_scale(13.0),
        }
    }

    fn sample_light_defaults() -> crate::ThemeDefaults {
        crate::ThemeDefaults {
            accent_color: Some(crate::Rgba::rgb(0, 122, 255)),
            background_color: Some(crate::Rgba::rgb(246, 246, 246)),
            text_color: Some(crate::Rgba::rgb(0, 0, 0)),
            surface_color: Some(crate::Rgba::rgb(255, 255, 255)),
            border: crate::model::border::BorderSpec {
                color: Some(crate::Rgba::rgb(200, 200, 200)),
                ..Default::default()
            },
            font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: None,
                ..Default::default()
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".to_string()),
                size: Some(13.0),
                weight: None,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn sample_dark_defaults() -> crate::ThemeDefaults {
        crate::ThemeDefaults {
            accent_color: Some(crate::Rgba::rgb(10, 132, 255)),
            background_color: Some(crate::Rgba::rgb(30, 30, 30)),
            text_color: Some(crate::Rgba::rgb(255, 255, 255)),
            surface_color: Some(crate::Rgba::rgb(44, 44, 46)),
            border: crate::model::border::BorderSpec {
                color: Some(crate::Rgba::rgb(56, 56, 58)),
                ..Default::default()
            },
            font: crate::FontSpec {
                family: Some("SF Pro".to_string()),
                size: Some(13.0),
                weight: None,
                ..Default::default()
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
        assert_ne!(light.defaults.accent_color, dark.defaults.accent_color);
        assert_ne!(
            light.defaults.background_color,
            dark.defaults.background_color
        );

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
                ..Default::default()
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".to_string()),
                size: Some(13.0),
                weight: None,
                ..Default::default()
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
        assert!(
            !light.is_empty(),
            "light variant should have widget defaults"
        );

        let dark = theme.dark.as_ref().unwrap();
        assert!(!dark.is_empty(), "dark variant should have widget defaults");
    }

    #[test]
    fn build_theme_colors_propagated_correctly() {
        let blue = crate::Rgba::rgb(0, 122, 255);
        let red = crate::Rgba::rgb(255, 59, 48);

        let light_defaults = crate::ThemeDefaults {
            accent_color: Some(blue),
            ..Default::default()
        };
        let dark_defaults = crate::ThemeDefaults {
            accent_color: Some(red),
            ..Default::default()
        };

        let theme = build_theme(light_defaults, dark_defaults, &sample_widget_fonts());

        let light = theme.light.as_ref().unwrap();
        let dark = theme.dark.as_ref().unwrap();

        assert_eq!(light.defaults.accent_color, Some(blue));
        assert_eq!(dark.defaults.accent_color, Some(red));
    }

    #[test]
    fn macos_widget_defaults_spot_check() {
        let wv = macos_widget_defaults();
        assert_eq!(
            wv.button.min_height,
            Some(22.0),
            "NSButton regular control size"
        );
        assert_eq!(
            wv.scrollbar.groove_width,
            Some(15.0),
            "NSScroller legacy style"
        );
        assert_eq!(
            wv.checkbox.indicator_width,
            Some(14.0),
            "NSButton switch type"
        );
        assert_eq!(
            wv.slider.thumb_diameter,
            Some(21.0),
            "NSSlider circular knob"
        );
    }

    #[test]
    fn build_theme_has_icon_set_sf_symbols() {
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );

        let light = theme.light.as_ref().unwrap();
        assert_eq!(light.icon_set, Some(crate::IconSet::SfSymbols));

        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(dark.icon_set, Some(crate::IconSet::SfSymbols));
    }

    #[test]
    fn build_theme_per_widget_fonts_populated() {
        let wf = sample_widget_fonts();
        let theme = build_theme(sample_light_defaults(), sample_dark_defaults(), &wf);

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

    #[test]
    fn build_theme_per_widget_colors_not_populated_by_build() {
        // build_theme does not populate per-widget colors -- that's done by
        // from_macos() after build_theme returns. Verify they start as None.
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
        let light = theme.light.as_ref().unwrap();
        assert!(
            light.input.placeholder_color.is_none(),
            "placeholder_color starts None (set by from_macos)"
        );
        assert!(
            light.list.alternate_row_background.is_none(),
            "alternate_row_background starts None"
        );
        assert!(light.list.header_font.is_none(), "header_font starts None");
        assert!(light.list.grid_color.is_none(), "grid_color starts None");
    }

    #[test]
    fn build_theme_scrollbar_overlay_not_set_by_build() {
        // Scrollbar overlay mode is set after build_theme by from_macos().
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
        let light = theme.light.as_ref().unwrap();
        assert!(
            light.scrollbar.overlay_mode.is_none(),
            "overlay_mode starts None (set by from_macos)"
        );
    }

    #[test]
    fn build_theme_dialog_button_order_not_set_by_build() {
        // Dialog button order is set after build_theme by from_macos().
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
        let light = theme.light.as_ref().unwrap();
        assert!(
            light.dialog.button_order.is_none(),
            "button_order starts None (set by from_macos)"
        );
    }

    #[test]
    fn build_theme_accessibility_not_set_by_build() {
        // Accessibility flags are set after build_theme by from_macos().
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
        let light = theme.light.as_ref().unwrap();
        assert!(light.defaults.reduce_motion.is_none());
        assert!(light.defaults.high_contrast.is_none());
        assert!(light.defaults.reduce_transparency.is_none());
        assert!(light.defaults.text_scaling_factor.is_none());
    }

    #[test]
    fn test_macos_resolve_validate() {
        // Load macOS-sonoma preset as base (provides full color/geometry/spacing).
        let mut base = crate::ThemeSpec::preset("macos-sonoma").unwrap();
        // Build reader output with sample data (simulates from_macos() on real hardware).
        let reader_output = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
        // Merge reader output on top of preset.
        base.merge(&reader_output);

        // Test light variant.
        let mut light = base
            .light
            .clone()
            .expect("light variant should exist after merge");
        light.resolve_all();
        let resolved = light.validate().unwrap_or_else(|e| {
            panic!("macOS resolve/validate pipeline failed (light): {e}");
        });

        // Spot-check: reader-sourced fields present.
        assert_eq!(
            resolved.defaults.accent_color,
            crate::Rgba::rgb(0, 122, 255),
            "accent should be from macOS reader"
        );
        assert_eq!(
            resolved.defaults.font.family, "SF Pro",
            "font family should be from macOS reader"
        );
        assert_eq!(
            resolved.icon_set,
            crate::IconSet::SfSymbols,
            "icon_set should be SfSymbols from macOS reader"
        );

        // Test dark variant too.
        let mut dark = base
            .dark
            .clone()
            .expect("dark variant should exist after merge");
        dark.resolve_all();
        let resolved_dark = dark.validate().unwrap_or_else(|e| {
            panic!("macOS resolve/validate pipeline failed (dark): {e}");
        });
        assert_eq!(
            resolved_dark.defaults.accent_color,
            crate::Rgba::rgb(10, 132, 255),
            "dark accent should be from macOS reader"
        );
    }
}
