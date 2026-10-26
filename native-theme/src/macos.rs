//! macOS theme reader: reads semantic NSColor values with P3-to-sRGB conversion,
//! resolves both light and dark appearance variants via NSAppearance, reads
//! system/monospace/per-widget fonts via NSFont, the scroll bar style and its
//! width, and accessibility flags.
//!
//! # Sources
//!
//! What the reader reads that the user's settings change; the watcher
//! observes each through the notification that reports it:
//!
//! - `AppleInterfaceStyle`: light or dark, the global user default.
//! - `NSColor`: the semantic colours, the accent and highlight colours among
//!   them, read under both appearances.
//! - `NSScroller.preferredScrollerStyle`: overlay scroll bars, and the groove
//!   width AppKit gives that style.
//! - `NSWorkspace.accessibilityDisplayOptions`: reduce motion, increase
//!   contrast and reduce transparency.
//!
//! # Sources not watched
//!
//! - `NSFont`: the system, monospaced, menu, tool-tip and title-bar fonts,
//!   which System Settings has no setting for.

// Objective-C FFI via objc2 -- no safe alternative
#![allow(unsafe_code)]

#[cfg(all(target_os = "macos", feature = "macos"))]
use block2::RcBlock;
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_app_kit::{
    NSAppearance, NSColor, NSColorSpace, NSControlSize, NSFont, NSFontTraitsAttribute,
    NSFontWeightRegular, NSFontWeightTrait, NSScroller, NSScrollerStyle, NSWorkspace,
};
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_foundation::{NSDictionary, NSNumber, NSProcessInfo, NSString};

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
    let alt_sel_text = NSColor::alternateSelectedControlTextColor();
    let system_red = NSColor::systemRedColor();
    let system_orange = NSColor::systemOrangeColor();
    let system_green = NSColor::systemGreenColor();
    let system_blue = NSColor::systemBlueColor();
    let sel_content_bg = NSColor::selectedContentBackgroundColor();
    let sel_text = NSColor::selectedTextColor();
    let sel_text_bg = NSColor::selectedTextBackgroundColor();
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
        // platform-facts.md:1055: border.color is separatorColor on macOS.
        border: crate::model::border::DefaultsBorderSpec {
            color: nscolor_to_rgba(&separator_c, &srgb),
            ..Default::default()
        },
        muted_color: nscolor_to_rgba(&secondary_label, &srgb),
        // shadow_color is not read: NSColor.shadowColor is an opaque black, while the
        // shadow AppKit draws is NSShadow's default, black at 1/3, which the preset
        // states (platform-facts.md:1057).
        danger_color: nscolor_to_rgba(&system_red, &srgb),
        danger_text_color: label,
        warning_color: nscolor_to_rgba(&system_orange, &srgb),
        warning_text_color: label,
        success_color: nscolor_to_rgba(&system_green, &srgb),
        success_text_color: label,
        info_color: nscolor_to_rgba(&system_blue, &srgb),
        info_text_color: label,
        selection_background: nscolor_to_rgba(&sel_content_bg, &srgb),
        // platform-facts.md:1060, :1062-1063: alternateSelectedControlTextColor on a
        // selection; selectedTextBackgroundColor and selectedTextColor for selected text
        selection_text_color: nscolor_to_rgba(&alt_sel_text, &srgb),
        selection_inactive_background: nscolor_to_rgba(&unemph_sel_bg, &srgb),
        text_selection_background: nscolor_to_rgba(&sel_text_bg, &srgb),
        text_selection_color: nscolor_to_rgba(&sel_text, &srgb),
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
        alternate_row,
        header_foreground: nscolor_to_rgba(&header_text_c, &srgb),
        grid_color: nscolor_to_rgba(&grid_c, &srgb),
        title_bar_foreground: nscolor_to_rgba(&frame_text_c, &srgb),
    };

    (defaults, per_widget)
}

/// Read one appearance's colours: [`read_appearance_colors`] inside
/// `performAsCurrentDrawingAppearance`, so that dynamic colours resolve to
/// `appearance`.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_variant(appearance: &NSAppearance) -> (crate::ThemeDefaults, PerWidgetColors) {
    let data = std::cell::Cell::new(None);
    {
        let block = RcBlock::new(|| data.set(Some(read_appearance_colors())));
        appearance.performAsCurrentDrawingAppearance(&block);
    }
    data.into_inner().unwrap_or_default()
}

/// Apply one appearance's per-widget colours to its variant.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn apply_per_widget(v: &mut crate::ThemeMode, pw: PerWidgetColors) {
    v.input.placeholder_color = pw.placeholder;
    v.list.alternate_row_background = pw.alternate_row;
    if let Some(color) = pw.header_foreground {
        v.list.header_font.get_or_insert_default().color = Some(color);
    }
    v.list.grid_color = pw.grid_color;
    if let Some(color) = pw.title_bar_foreground {
        v.window.title_bar_font.get_or_insert_default().color = Some(color);
    }
}

/// The scroll bar for the user's scroller style: `overlay` (else legacy)
/// and the groove width AppKit gives that style, `width_for(overlay)`.
/// The width differs per style and macOS version: legacy 15 and overlay 16
/// on macOS 14.8.9, 17 for both on macOS 26 (docs/platform-facts.md:1307).
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn scrollbar_for_style(overlay: bool, width_for: impl Fn(bool) -> f64) -> crate::ScrollbarTheme {
    crate::ScrollbarTheme {
        groove_width: Some(width_for(overlay) as f32),
        overlay_mode: Some(overlay),
        ..Default::default()
    }
}

/// Read the scroll bar from `NSScroller.preferredScrollerStyle` and
/// `scrollerWidth(for: .regular, scrollerStyle:)` for that style.
/// Requires main thread (MainThreadMarker).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_scrollbar(mtm: objc2::MainThreadMarker) -> crate::ScrollbarTheme {
    let overlay = NSScroller::preferredScrollerStyle(mtm) == NSScrollerStyle::Overlay;
    scrollbar_for_style(overlay, |overlay| {
        let style = if overlay {
            NSScrollerStyle::Overlay
        } else {
            NSScrollerStyle::Legacy
        };
        NSScroller::scrollerWidthForControlSize_scrollerStyle(NSControlSize::Regular, style, mtm)
    })
}

/// The running macOS version, from `NSProcessInfo.operatingSystemVersion`
/// (a safe binding in objc2-foundation 0.3.2), for the generation table.
/// AppKit's components are never negative; one that were would read as 0, and
/// a version of 0.0.0 gets `macos-monterey` with `VersionMatch::PresetNewer`.
/// Early macOS 26 betas reported 16 here to a binary built against the macOS
/// 15 SDK; `pipeline::MACOS_GENERATIONS` gives the sources and the mapping.
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) fn os_version() -> crate::pipeline::OsVersion {
    let v = NSProcessInfo::processInfo().operatingSystemVersion();
    let part = |n: isize| u32::try_from(n).unwrap_or_default();
    crate::pipeline::OsVersion::new(
        part(v.majorVersion),
        part(v.minorVersion),
        part(v.patchVersion),
    )
}

/// Whether the user asks for reduced motion
/// (`NSWorkspace.accessibilityDisplayShouldReduceMotion`), as the reader
/// reads it, for [`crate::detect::detect_reduced_motion`].
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) fn reduce_motion() -> bool {
    NSWorkspace::sharedWorkspace().accessibilityDisplayShouldReduceMotion()
}

/// Read accessibility flags from NSWorkspace.
///
/// Returns (reduce_motion, high_contrast, reduce_transparency). No text-scaling
/// factor: macOS's accessibility text size affects only a few Apple apps, and
/// `preferredFont(forTextStyle:)` still returns fixed sizes
/// (docs/platform-facts.md:1127), so the fonts the reader reads already carry
/// whatever size the system gives them.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_accessibility() -> (Option<bool>, Option<bool>, Option<bool>) {
    let workspace = NSWorkspace::sharedWorkspace();
    (
        Some(reduce_motion()),
        Some(workspace.accessibilityDisplayShouldIncreaseContrast()),
        Some(workspace.accessibilityDisplayShouldReduceTransparency()),
    )
}

/// Map an AppKit font weight (-1.0..1.0) to the nearest CSS weight (100..900).
///
/// Uses the standard AppKit-to-CSS mapping thresholds. Returns `None` if the
/// font descriptor's traits dictionary does not contain the weight key.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn nsfont_weight_to_css(font: &NSFont) -> Option<u16> {
    // SAFETY: the two keys are AppKit's own NSString statics, declared in
    // `extern` blocks; reading them is an FFI read of a constant.
    let (traits_key, weight_key): (&NSString, &NSString) =
        unsafe { (NSFontTraitsAttribute, NSFontWeightTrait) };
    let traits_obj = font.fontDescriptor().objectForKey(traits_key)?;
    let w = traits_obj
        .downcast_ref::<NSDictionary>()?
        .objectForKey(weight_key)?
        .downcast_ref::<NSNumber>()?
        .doubleValue();

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
        family: font
            .familyName()
            .map(|n| std::sync::Arc::from(n.to_string())),
        size: Some(crate::model::font::FontSize::Pt(font.pointSize() as f32)),
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
    // SAFETY: `NSFontWeightRegular` is an AppKit `extern` constant (a CGFloat,
    // valid for the process's lifetime); the class method takes no pointer.
    let mono_font =
        unsafe { NSFont::monospacedSystemFontOfSize_weight(system_size, NSFontWeightRegular) };

    (
        fontspec_from_nsfont(&system_font),
        fontspec_from_nsfont(&mono_font),
    )
}

/// The macOS system UI font as Core Text resolves it: its family name and
/// the file it lives in (egui connector spec §8.2).
#[cfg(all(target_os = "macos", feature = "system-fonts"))]
pub(crate) struct SystemUiFont {
    /// Core Text's family name for the font (`kCTFontFamilyNameAttribute`).
    pub(crate) family: String,
    /// The font file (`kCTFontURLAttribute` as a file path).
    pub(crate) path: std::path::PathBuf,
}

/// Ask Core Text which font is the system UI font — `CTFontUIFontType::System`
/// at the type's default size, no language — and, for `italic`, its italic
/// variant (the upright font where Core Text has none). `None` where Core
/// Text returns none, an attribute is missing or is not the type the
/// attribute documents, or the URL is not a file path. `CTFont::family_name`
/// is not used: its binding `expect`s a non-null result (objc2-core-text
/// 0.3.2 `src/generated/CTFont.rs` lines 977–983).
#[cfg(all(target_os = "macos", feature = "system-fonts"))]
pub(crate) fn system_ui_font(italic: bool) -> Option<SystemUiFont> {
    use objc2_core_foundation::{CFString, CFURL};
    use objc2_core_text::{
        CTFont, CTFontSymbolicTraits, CTFontUIFontType, kCTFontFamilyNameAttribute,
        kCTFontURLAttribute,
    };
    // SAFETY: `CTFontCreateUIFontForLanguage` takes a UI font type, a size
    // (0.0 = the type's default) and an optional language; a null language
    // is documented as allowed, and the binding maps a NULL result to None.
    let upright = unsafe { CTFont::new_ui_font_for_language(CTFontUIFontType::System, 0.0, None) }?;
    let font = if italic {
        // SAFETY: a null matrix is documented as allowed by the binding's
        // safety note; the value and mask both name the italic trait.
        let slanted = unsafe {
            upright.copy_with_symbolic_traits(
                0.0,
                std::ptr::null(),
                CTFontSymbolicTraits::ItalicTrait,
                CTFontSymbolicTraits::ItalicTrait,
            )
        };
        slanted.unwrap_or(upright)
    } else {
        upright
    };
    // SAFETY: the two keys are Core Text's own statics, declared in `extern`
    // blocks; reading them is an FFI read of a constant.
    let (family_key, url_key) = unsafe { (kCTFontFamilyNameAttribute, kCTFontURLAttribute) };
    // SAFETY: `CTFontCopyAttribute` returns a retained reference or NULL,
    // which the binding maps to None; the downcasts check the type id.
    let family = unsafe { font.attribute(family_key) }?
        .downcast::<CFString>()
        .ok()?
        .to_string();
    // SAFETY: as above: `url_key` is Core Text's own static, `font.attribute`
    // returns a retained reference or NULL (mapped to None), and the downcast
    // checks the type id.
    let path = unsafe { font.attribute(url_key) }?
        .downcast::<CFURL>()
        .ok()?
        .to_file_path()?;
    Some(SystemUiFont { family, path })
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

/// Per-widget font and text scale data passed to [`build_theme`].
///
/// Collected once (appearance-independent) and applied to both variants.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
struct WidgetFontData {
    menu_font: crate::FontSpec,
    tooltip_font: crate::FontSpec,
    title_bar_font: crate::FontSpec,
}

/// Testable core: assemble a Theme from pre-read color and font data.
///
/// Takes pre-resolved ThemeDefaults for both light and dark variants, plus
/// per-widget font data. Both variants are always
/// populated (unlike KDE/GNOME/Windows which populate only the active one),
/// since macOS can resolve colors for both appearances.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn build_theme(
    light_defaults: crate::ThemeDefaults,
    dark_defaults: crate::ThemeDefaults,
    widget_fonts: &WidgetFontData,
) -> crate::Theme {
    // The reader states only what macOS reports; the generation's design sizes
    // come from its preset (M1-T1-02).
    let variant = |defaults: crate::ThemeDefaults| crate::ThemeMode {
        defaults,
        menu: crate::MenuTheme {
            font: Some(widget_fonts.menu_font.clone()),
            ..Default::default()
        },
        tooltip: crate::TooltipTheme {
            font: Some(widget_fonts.tooltip_font.clone()),
            ..Default::default()
        },
        window: crate::WindowTheme {
            title_bar_font: Some(widget_fonts.title_bar_font.clone()),
            ..Default::default()
        },
        ..Default::default()
    };

    crate::Theme {
        name: "macOS".into(),
        light: Some(variant(light_defaults)),
        dark: Some(variant(dark_defaults)),
        layout: crate::LayoutTheme::default(),
        icon_set: Some(crate::IconSet::SfSymbols),
        // macOS uses the same icon theme ("sf-symbols") for both variants; the
        // Theme-level field is populated via the merged generation preset.
        icon_theme: None,
    }
}

/// Read the current macOS theme, resolving both light and dark appearance variants.
///
/// Uses `NSAppearance::performAsCurrentDrawingAppearance` (macOS 11+) to scope
/// semantic color resolution to each appearance. Reads ~25 NSColor semantic
/// colors per variant with P3-to-sRGB conversion, per-widget fonts with weight,
/// the scroll bar style and width, and accessibility flags.
///
/// # Errors
///
/// Returns `Error::ReaderFailed` if neither light nor dark appearance can be created
/// (extremely unlikely on any macOS version that supports these APIs).
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`crate::pipeline::select_reader`]; external consumers should use
/// [`SystemTheme::from_system()`](crate::SystemTheme::from_system).
///
/// The body contains no `.await` points — wrapping a sync implementation in
/// an async trait method produces a future that resolves immediately.
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) struct MacosReader;

#[cfg(all(target_os = "macos", feature = "macos"))]
#[async_trait::async_trait]
impl crate::reader::ThemeReader for MacosReader {
    async fn read(&self) -> crate::Result<crate::ReaderResult> {
        read_macos()
    }
}

#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_macos() -> crate::Result<crate::ReaderResult> {
    let light_name = NSString::from_str("NSAppearanceNameAqua");
    let dark_name = NSString::from_str("NSAppearanceNameDarkAqua");

    let light_appearance = NSAppearance::appearanceNamed(&light_name);
    let dark_appearance = NSAppearance::appearanceNamed(&dark_name);

    if light_appearance.is_none() && dark_appearance.is_none() {
        return Err(crate::Error::ReaderFailed {
            reader: crate::error::reader_names::MACOS,
            source: "neither light nor dark NSAppearance could be created".into(),
        });
    }

    // Read appearance-independent data once.
    let (font, mono_font) = read_fonts();
    let (menu_font, tooltip_font, title_bar_font) = read_per_widget_fonts();
    let widget_fonts = WidgetFontData {
        menu_font,
        tooltip_font,
        title_bar_font,
    };

    let read = |appearance: Option<&NSAppearance>| match appearance {
        Some(app) => {
            let (mut defaults, pw) = read_variant(app);
            defaults.font = font.clone();
            defaults.mono_font = mono_font.clone();
            (defaults, Some(pw))
        }
        None => (crate::ThemeDefaults::default(), None),
    };
    let (light_defaults, light_pw) = read(light_appearance.as_deref());
    let (dark_defaults, dark_pw) = read(dark_appearance.as_deref());

    let mut theme = build_theme(light_defaults, dark_defaults, &widget_fonts);

    // The scroll bar is appearance-independent and requires the main thread.
    let scrollbar = objc2::MainThreadMarker::new().map(read_scrollbar);
    for (variant, pw) in [(&mut theme.light, light_pw), (&mut theme.dark, dark_pw)] {
        let Some(v) = variant else { continue };
        if let Some(pw) = pw {
            apply_per_widget(v, pw);
        }
        if let Some(scrollbar) = &scrollbar {
            v.scrollbar.merge(scrollbar);
        }
    }

    // Accessibility flags (appearance-independent).
    let (reduce_motion, high_contrast, reduce_transparency) = read_accessibility();
    let mut acc = crate::AccessibilityPreferences::default();
    if let Some(rm) = reduce_motion {
        acc.reduce_motion = rm;
    }
    if let Some(hc) = high_contrast {
        acc.high_contrast = hc;
    }
    if let Some(rt) = reduce_transparency {
        acc.reduce_transparency = rt;
    }

    // macOS coordinate system uses 72 DPI as its base.
    // NSFont.pointSize() returns Apple points where 1pt = 1 logical pixel,
    // so the formula pt * 72/72 = pt (identity) is correct.
    let font_dpi = Some(72.0_f32);

    let output = crate::ReaderOutput::Dual {
        light: Box::new(theme.light.unwrap_or_default()),
        dark: Box::new(theme.dark.unwrap_or_default()),
        is_dark: crate::detect::detect_is_dark(),
    };

    Ok(crate::ReaderResult {
        output,
        name: theme.name,
        icon_set: theme.icon_set,
        layout: theme.layout,
        font_dpi,
        accessibility: acc,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::model::font::FontSize;

    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn from_system_mode_is_the_os_appearance() {
        let sys = crate::SystemTheme::from_system().unwrap();
        let expected = if crate::detect::detect_is_dark() {
            crate::ColorMode::Dark
        } else {
            crate::ColorMode::Light
        };
        assert_eq!(
            sys.mode, expected,
            "SystemTheme::mode must follow AppleInterfaceStyle"
        );
    }

    fn sample_widget_fonts() -> WidgetFontData {
        use crate::model::font::FontSize;
        WidgetFontData {
            menu_font: crate::FontSpec {
                family: Some("SF Pro".into()),
                size: Some(FontSize::Pt(14.0)),
                weight: Some(400),
                ..Default::default()
            },
            tooltip_font: crate::FontSpec {
                family: Some("SF Pro".into()),
                size: Some(FontSize::Pt(11.0)),
                weight: Some(400),
                ..Default::default()
            },
            title_bar_font: crate::FontSpec {
                family: Some("SF Pro".into()),
                size: Some(FontSize::Pt(13.0)),
                weight: Some(700),
                ..Default::default()
            },
        }
    }

    fn sample_light_defaults() -> crate::ThemeDefaults {
        use crate::model::font::FontSize;
        crate::ThemeDefaults {
            accent_color: Some(crate::Rgba::rgb(0, 122, 255)),
            background_color: Some(crate::Rgba::rgb(246, 246, 246)),
            text_color: Some(crate::Rgba::rgb(0, 0, 0)),
            surface_color: Some(crate::Rgba::rgb(255, 255, 255)),
            border: crate::model::border::DefaultsBorderSpec {
                color: Some(crate::Rgba::rgb(200, 200, 200)),
                ..Default::default()
            },
            font: crate::FontSpec {
                family: Some("SF Pro".into()),
                size: Some(FontSize::Pt(13.0)),
                weight: None,
                ..Default::default()
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".into()),
                size: Some(FontSize::Pt(13.0)),
                weight: None,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn sample_dark_defaults() -> crate::ThemeDefaults {
        use crate::model::font::FontSize;
        crate::ThemeDefaults {
            accent_color: Some(crate::Rgba::rgb(10, 132, 255)),
            background_color: Some(crate::Rgba::rgb(30, 30, 30)),
            text_color: Some(crate::Rgba::rgb(255, 255, 255)),
            surface_color: Some(crate::Rgba::rgb(44, 44, 46)),
            border: crate::model::border::DefaultsBorderSpec {
                color: Some(crate::Rgba::rgb(56, 56, 58)),
                ..Default::default()
            },
            font: crate::FontSpec {
                family: Some("SF Pro".into()),
                size: Some(FontSize::Pt(13.0)),
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
        use crate::model::font::FontSize;
        let defaults = crate::ThemeDefaults {
            font: crate::FontSpec {
                family: Some("SF Pro".into()),
                size: Some(FontSize::Pt(13.0)),
                weight: None,
                ..Default::default()
            },
            mono_font: crate::FontSpec {
                family: Some("SF Mono".into()),
                size: Some(FontSize::Pt(13.0)),
                weight: None,
                ..Default::default()
            },
            ..Default::default()
        };

        let theme = build_theme(defaults.clone(), defaults, &sample_widget_fonts());

        let light = theme.light.as_ref().unwrap();
        assert_eq!(light.defaults.font.family.as_deref(), Some("SF Pro"));
        assert_eq!(light.defaults.font.size, Some(FontSize::Pt(13.0)));
        assert_eq!(light.defaults.mono_font.family.as_deref(), Some("SF Mono"));
        assert_eq!(light.defaults.mono_font.size, Some(FontSize::Pt(13.0)));

        let dark = theme.dark.as_ref().unwrap();
        assert_eq!(dark.defaults.font.family.as_deref(), Some("SF Pro"));
        assert_eq!(dark.defaults.font.size, Some(FontSize::Pt(13.0)));
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

    /// A non-default accent reaches the four accent fills through inheritance:
    /// macOS draws a checked checkbox and radio, an on switch, a slider's value
    /// and a progress bar in `controlAccentColor` (docs/platform-facts.md:1239,
    /// :1521, :1317, :1333, ← `defaults.accent_color`), the macOS presets state
    /// none of them, and the reader writes only the accent.
    #[test]
    fn a_custom_accent_reaches_the_accent_fills() {
        let red = crate::Rgba::rgb(255, 59, 48);
        let defaults = |accent| crate::ThemeDefaults {
            accent_color: Some(accent),
            ..Default::default()
        };
        let theme = build_theme(defaults(red), defaults(red), &sample_widget_fonts());
        for preset in ["macos-monterey", "macos-tahoe"] {
            let mut mode = crate::Theme::preset(preset).unwrap().light.unwrap();
            assert_ne!(mode.defaults.accent_color, Some(red), "{preset}");
            mode.merge(theme.light.as_ref().unwrap());
            let r = mode
                .into_resolved(&crate::ResolutionContext::for_tests())
                .unwrap();
            assert_eq!(r.checkbox.checked_background, red, "{preset}");
            assert_eq!(r.switch.checked_background, red, "{preset}");
            assert_eq!(r.slider.fill_color, red, "{preset}");
            assert_eq!(r.progress_bar.fill_color, red, "{preset}");
        }
    }

    /// The reader states only what macOS reports (M1-T1-02): `build_theme`
    /// carries the read colours and fonts and no design constant.
    #[test]
    fn build_theme_states_no_design_constants() {
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );
        for variant in [theme.light.as_ref(), theme.dark.as_ref()] {
            let mut rest = variant.unwrap().clone();
            rest.defaults = crate::ThemeDefaults::default();
            rest.menu.font = None;
            rest.tooltip.font = None;
            rest.window.title_bar_font = None;
            assert_eq!(
                rest,
                crate::ThemeMode::default(),
                "build_theme states a value macOS did not report"
            );
        }
    }

    #[test]
    fn build_theme_has_icon_set_sf_symbols() {
        let theme = build_theme(
            sample_light_defaults(),
            sample_dark_defaults(),
            &sample_widget_fonts(),
        );

        // icon_set is now on Theme, shared across variants
        assert_eq!(theme.icon_set, Some(crate::IconSet::SfSymbols));
    }

    #[test]
    fn build_theme_per_widget_fonts_populated() {
        let wf = sample_widget_fonts();
        let theme = build_theme(sample_light_defaults(), sample_dark_defaults(), &wf);

        let light = theme.light.as_ref().unwrap();
        assert_eq!(
            light.menu.font.as_ref().unwrap().size,
            Some(FontSize::Pt(14.0)),
            "menu font size"
        );
        assert_eq!(
            light.tooltip.font.as_ref().unwrap().size,
            Some(FontSize::Pt(11.0)),
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

    /// The groove width is the one AppKit gives the user's scroller style,
    /// not the overlay style's always (docs/platform-facts.md:1307: legacy 15,
    /// overlay 16 on macOS 14.8.9).
    #[test]
    fn scrollbar_for_style_takes_the_styles_width() {
        let width_for = |overlay: bool| if overlay { 16.0 } else { 15.0 };
        let overlay = scrollbar_for_style(true, width_for);
        assert_eq!(overlay.overlay_mode, Some(true));
        assert_eq!(
            overlay.groove_width,
            Some(16.0),
            "the overlay style's width"
        );
        let legacy = scrollbar_for_style(false, width_for);
        assert_eq!(legacy.overlay_mode, Some(false));
        assert_eq!(legacy.groove_width, Some(15.0), "the legacy style's width");
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
        // Accessibility fields are no longer on ThemeDefaults;
        // they live on AccessibilityPreferences (constructed by pipeline).
        assert!(theme.light.is_some());
    }

    #[test]
    fn test_macos_resolve_validate() {
        // Load the macos-monterey preset as base (provides full color/geometry/spacing).
        let mut base = crate::Theme::preset("macos-monterey").unwrap();
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
            resolved.defaults.font.family.as_ref(),
            "SF Pro",
            "font family should be from macOS reader"
        );
        // icon_set is now on Theme/SystemTheme, not on ResolvedTheme

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

    // === Border colour (platform-facts.md:1055: separatorColor) ===

    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn read_appearance_colors_reads_border_color_from_separator_color() {
        let (defaults, _) = super::read_appearance_colors();
        assert!(
            defaults.border.color.is_some(),
            "defaults.border.color should be read from NSColor.separatorColor"
        );
    }

    // === The NSColor mapping (platform-facts.md:1057, :1060, :1062-1063, :1210) ===

    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn read_appearance_colors_maps_selection_and_shadow_as_platform_facts_names() {
        let srgb = NSColorSpace::sRGBColorSpace();
        let (defaults, _) = super::read_appearance_colors();
        assert_eq!(
            defaults.shadow_color, None,
            "NSColor.shadowColor is not the drawn shadow (platform-facts.md:1057)"
        );
        assert_eq!(
            defaults.selection_text_color,
            nscolor_to_rgba(&NSColor::alternateSelectedControlTextColor(), &srgb)
        );
        assert_eq!(
            defaults.text_selection_color,
            nscolor_to_rgba(&NSColor::selectedTextColor(), &srgb)
        );
        assert_eq!(
            defaults.text_selection_background,
            nscolor_to_rgba(&NSColor::selectedTextBackgroundColor(), &srgb)
        );
    }

    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn from_system_reports_the_running_generation() {
        let sys = crate::SystemTheme::from_system().unwrap();
        let (preset, matched) = crate::pipeline::macos_generation(super::os_version());
        assert_eq!(sys.preset, preset.name);
        assert_eq!(sys.version_match, Some(matched));
    }

    #[cfg(all(target_os = "macos", feature = "macos"))]
    #[test]
    fn read_macos_leaves_the_text_field_selection_to_inheritance() {
        let result = super::read_macos().unwrap();
        assert!(
            matches!(
                &result.output,
                crate::ReaderOutput::Dual { light, .. } if light.input.selection_background.is_none()
            ),
            "the reader should return both variants, light input.selection_background unset: it \
             inherits defaults.text_selection_background (platform-facts.md:1210)"
        );
        assert!(
            matches!(
                &result.output,
                crate::ReaderOutput::Dual { dark, .. } if dark.input.selection_background.is_none()
            ),
            "the reader should return both variants, dark input.selection_background unset: it \
             inherits defaults.text_selection_background (platform-facts.md:1210)"
        );
    }

    // === Plan 94-03 (G8): MacosReader ThemeReader impl ===
    //
    // Locks the G8 contract on the macOS side: a unit struct `MacosReader`
    // exists at the module root and implements `crate::reader::ThemeReader`.
    // Before Task 2 lands, this fails to compile (`no struct MacosReader` /
    // `unresolved module crate::reader`).
    //
    // The trait object coercion is the structural object-safety probe. Body
    // is gated on `cfg(all(target_os = "macos", feature = "macos"))` so that
    // the non-macOS build only compile-checks the trait path reference.
    #[test]
    fn macos_reader_exists() {
        // Reference the trait path once so name resolution fires on all
        // targets regardless of whether the MacosReader struct is compiled.
        let _: Option<&dyn crate::reader::ThemeReader> = None;

        #[cfg(all(target_os = "macos", feature = "macos"))]
        {
            let r: Box<dyn crate::reader::ThemeReader> = Box::new(super::MacosReader);
            let _ = &r;
        }
    }
}
