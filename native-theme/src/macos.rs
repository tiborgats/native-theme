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

/// The first macOS major version with `NSColor.textInsertionPointColor`
/// ("introducedAt": "14.0" in Apple's documentation;
/// docs/platform-facts.md:1209). Below it the selector does not exist, so
/// the reader does not send it and the preset's caret stands.
const TEXT_INSERTION_POINT_MAJOR: u32 = 14;

/// Whether `version` has `NSColor.textInsertionPointColor`; the version is
/// `os_version`'s, the one the generation table reads.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn has_text_insertion_point_color(version: crate::pipeline::OsVersion) -> bool {
    version.major >= TEXT_INSERTION_POINT_MAJOR
}

/// The alpha a disabled on switch's track draws its accent at: macOS draws
/// the enabled on track at one half in macos-monterey's look and at about
/// 0.45 in macos-tahoe's (docs/platform-facts.md:1529). The generation is
/// `os_version`'s, as the generation table reads it, and each preset states
/// `switch.disabled_checked_background` as its accent at its generation's
/// alpha.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn disabled_checked_alpha(version: crate::pipeline::OsVersion) -> u8 {
    if crate::pipeline::macos_generation(version).0.name == "macos-tahoe" {
        0x73
    } else {
        0x80
    }
}

/// Write what macOS fades from the accent: a disabled on switch's track,
/// the variant's accent at `disabled_checked_alpha`. A variant whose accent
/// was not read keeps the preset's.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn apply_accent_fades(v: &mut crate::ThemeMode, disabled_checked_alpha: u8) {
    v.switch.disabled_checked_background = v
        .defaults
        .accent_color
        .map(|a| crate::Rgba::new(a.r, a.g, a.b, disabled_checked_alpha));
}

/// The semantic `NSColor`s read under one appearance, each converted by
/// `nscolor_to_rgba` (`None` where AppKit cannot convert it): what
/// [`defaults_of`] and [`apply_per_widget`] write into the reader's variant,
/// and what the 0.8.0 reader carries as that appearance's entry of
/// `color_schemes` (`next::reader::macos`).
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct AppearanceColors {
    /// `labelColor`.
    pub(crate) label: Option<crate::Rgba>,
    /// `controlAccentColor`.
    pub(crate) control_accent: Option<crate::Rgba>,
    /// `windowBackgroundColor`.
    pub(crate) window_background: Option<crate::Rgba>,
    /// `controlBackgroundColor`.
    pub(crate) control_background: Option<crate::Rgba>,
    /// `separatorColor`.
    pub(crate) separator: Option<crate::Rgba>,
    /// `secondaryLabelColor`.
    pub(crate) secondary_label: Option<crate::Rgba>,
    /// `alternateSelectedControlTextColor`.
    pub(crate) alternate_selected_control_text: Option<crate::Rgba>,
    /// `systemRedColor`.
    pub(crate) system_red: Option<crate::Rgba>,
    /// `systemOrangeColor`.
    pub(crate) system_orange: Option<crate::Rgba>,
    /// `systemGreenColor`.
    pub(crate) system_green: Option<crate::Rgba>,
    /// `systemBlueColor`.
    pub(crate) system_blue: Option<crate::Rgba>,
    /// `selectedContentBackgroundColor`.
    pub(crate) selected_content_background: Option<crate::Rgba>,
    /// `unemphasizedSelectedContentBackgroundColor`.
    pub(crate) unemphasized_selected_content_background: Option<crate::Rgba>,
    /// `selectedTextBackgroundColor`.
    pub(crate) selected_text_background: Option<crate::Rgba>,
    /// `selectedTextColor`.
    pub(crate) selected_text: Option<crate::Rgba>,
    /// `linkColor`.
    pub(crate) link: Option<crate::Rgba>,
    /// `keyboardFocusIndicatorColor`.
    pub(crate) keyboard_focus_indicator: Option<crate::Rgba>,
    /// `disabledControlTextColor`.
    pub(crate) disabled_control_text: Option<crate::Rgba>,
    /// `placeholderTextColor`.
    pub(crate) placeholder_text: Option<crate::Rgba>,
    /// `textInsertionPointColor`, read on macOS 14 and later only
    /// ([`has_text_insertion_point_color`]).
    pub(crate) text_insertion_point: Option<crate::Rgba>,
    /// `alternatingContentBackgroundColors[1]` (index 0 is the normal
    /// background), where the array has two colours.
    pub(crate) alternate_row: Option<crate::Rgba>,
    /// `headerTextColor`.
    pub(crate) header_text: Option<crate::Rgba>,
    /// `gridColor`.
    pub(crate) grid: Option<crate::Rgba>,
    /// `windowFrameTextColor`.
    pub(crate) window_frame_text: Option<crate::Rgba>,
}

/// The variant's `defaults` of the colours read under one appearance.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn defaults_of(colors: &AppearanceColors) -> crate::ThemeDefaults {
    let label = colors.label;
    crate::ThemeDefaults {
        accent_color: colors.control_accent,
        accent_text_color: colors.alternate_selected_control_text,
        background_color: colors.window_background,
        text_color: label,
        surface_color: colors.control_background,
        // platform-facts.md:1055: border.color is separatorColor on macOS.
        border: crate::model::border::DefaultsBorderSpec {
            color: colors.separator,
            ..Default::default()
        },
        muted_color: colors.secondary_label,
        // shadow_color is not read: NSColor.shadowColor is an opaque black, while the
        // shadow AppKit draws is NSShadow's default, black at 1/3, which the preset
        // states (platform-facts.md:1057).
        danger_color: colors.system_red,
        danger_text_color: label,
        warning_color: colors.system_orange,
        warning_text_color: label,
        success_color: colors.system_green,
        success_text_color: label,
        info_color: colors.system_blue,
        info_text_color: label,
        selection_background: colors.selected_content_background,
        // platform-facts.md §2.1.3 (selection_text_color,
        // selection_inactive_text_color, text_selection_*):
        // alternateSelectedControlTextColor on a selection, labelColor on an
        // unfocused one, which does not adapt;
        // selectedTextBackgroundColor and selectedTextColor for selected text
        selection_text_color: colors.alternate_selected_control_text,
        selection_inactive_background: colors.unemphasized_selected_content_background,
        selection_inactive_text_color: label,
        text_selection_background: colors.selected_text_background,
        text_selection_color: colors.selected_text,
        link_color: colors.link,
        focus_ring_color: colors.keyboard_focus_indicator,
        disabled_text_color: colors.disabled_control_text,
        ..Default::default()
    }
}

/// Read all semantic NSColor values for the current appearance context.
///
/// Must be called within an `NSAppearance::performAsCurrentDrawingAppearance`
/// block so that dynamic colors resolve to the correct appearance.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_appearance_colors() -> AppearanceColors {
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
    // platform-facts.md:1209 (§2.4): the insertion point's own colour, on
    // macOS 14 and later only.
    let insertion_point_c =
        has_text_insertion_point_color(os_version()).then(NSColor::textInsertionPointColor);
    let unemph_sel_bg = NSColor::unemphasizedSelectedContentBackgroundColor();
    let alt_bg_colors = NSColor::alternatingContentBackgroundColors();
    let header_text_c = NSColor::headerTextColor();
    let grid_c = NSColor::gridColor();
    let frame_text_c = NSColor::windowFrameTextColor();

    // Alternate row: index 1 of alternatingContentBackgroundColors (index 0 is normal).
    let alternate_row = if alt_bg_colors.count() >= 2 {
        nscolor_to_rgba(&alt_bg_colors.objectAtIndex(1), &srgb)
    } else {
        None
    };

    AppearanceColors {
        label: nscolor_to_rgba(&label_c, &srgb),
        control_accent: nscolor_to_rgba(&control_accent, &srgb),
        window_background: nscolor_to_rgba(&window_bg, &srgb),
        control_background: nscolor_to_rgba(&control_bg, &srgb),
        separator: nscolor_to_rgba(&separator_c, &srgb),
        secondary_label: nscolor_to_rgba(&secondary_label, &srgb),
        alternate_selected_control_text: nscolor_to_rgba(&alt_sel_text, &srgb),
        system_red: nscolor_to_rgba(&system_red, &srgb),
        system_orange: nscolor_to_rgba(&system_orange, &srgb),
        system_green: nscolor_to_rgba(&system_green, &srgb),
        system_blue: nscolor_to_rgba(&system_blue, &srgb),
        selected_content_background: nscolor_to_rgba(&sel_content_bg, &srgb),
        unemphasized_selected_content_background: nscolor_to_rgba(&unemph_sel_bg, &srgb),
        selected_text_background: nscolor_to_rgba(&sel_text_bg, &srgb),
        selected_text: nscolor_to_rgba(&sel_text, &srgb),
        link: nscolor_to_rgba(&link_c, &srgb),
        keyboard_focus_indicator: nscolor_to_rgba(&focus_c, &srgb),
        disabled_control_text: nscolor_to_rgba(&disabled_text, &srgb),
        placeholder_text: nscolor_to_rgba(&placeholder_c, &srgb),
        text_insertion_point: insertion_point_c
            .as_ref()
            .and_then(|c| nscolor_to_rgba(c, &srgb)),
        alternate_row,
        header_text: nscolor_to_rgba(&header_text_c, &srgb),
        grid: nscolor_to_rgba(&grid_c, &srgb),
        window_frame_text: nscolor_to_rgba(&frame_text_c, &srgb),
    }
}

/// Read one appearance's colours: [`read_appearance_colors`] inside
/// `performAsCurrentDrawingAppearance`, so that dynamic colours resolve to
/// `appearance`.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn read_variant(appearance: &NSAppearance) -> AppearanceColors {
    let data = std::cell::Cell::new(None);
    {
        let block = RcBlock::new(|| data.set(Some(read_appearance_colors())));
        appearance.performAsCurrentDrawingAppearance(&block);
    }
    data.into_inner().unwrap_or_default()
}

/// Apply one appearance's per-widget colours to its variant.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
fn apply_per_widget(v: &mut crate::ThemeMode, colors: &AppearanceColors) {
    v.input.placeholder_color = colors.placeholder_text;
    v.input.caret_color = colors.text_insertion_point;
    v.list.alternate_row_background = colors.alternate_row;
    if let Some(color) = colors.header_text {
        v.list.header_font.get_or_insert_default().color = Some(color);
    }
    v.list.grid_color = colors.grid;
    if let Some(color) = colors.window_frame_text {
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

/// The font DPI the reader states: macOS's coordinate system uses 72 DPI as
/// its base. `NSFont.pointSize()` returns Apple points where 1pt = 1 logical
/// pixel, so the formula pt * 72/72 = pt (identity) is correct.
pub(crate) const FONT_DPI: f32 = 72.0;

/// Per-widget font and text scale data passed to [`build_theme`].
///
/// Collected once (appearance-independent) and applied to both variants.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
#[derive(Clone, Debug)]
pub(crate) struct WidgetFontData {
    /// `NSFont.menuFontOfSize(0)`.
    pub(crate) menu_font: crate::FontSpec,
    /// `NSFont.toolTipsFontOfSize(0)`.
    pub(crate) tooltip_font: crate::FontSpec,
    /// `NSFont.titleBarFontOfSize(0)`.
    pub(crate) title_bar_font: crate::FontSpec,
}

/// Everything the reader reads, each as read: what [`reader_result`] builds
/// the 0.7.2 reader's result from, and the 0.8.0 reader its snapshot
/// (`next::reader::macos`).
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
#[derive(Clone, Debug)]
pub(crate) struct MacosSettings {
    /// The colours under Aqua, `None` where AppKit gives no such appearance.
    pub(crate) light: Option<AppearanceColors>,
    /// The colours under Dark Aqua, `None` where AppKit gives no such
    /// appearance.
    pub(crate) dark: Option<AppearanceColors>,
    /// The system font (`read_fonts`).
    pub(crate) font: crate::FontSpec,
    /// The monospaced system font (`read_fonts`).
    pub(crate) mono_font: crate::FontSpec,
    /// The menu, tool-tip and title-bar fonts (`read_per_widget_fonts`).
    pub(crate) widget_fonts: WidgetFontData,
    /// The scroll bar of the user's scroller style
    /// ([`scrollbar_for_style`]), `None` where not read on the main thread.
    pub(crate) scrollbar: Option<crate::ScrollbarTheme>,
    /// `accessibilityDisplayShouldReduceMotion`.
    pub(crate) reduce_motion: Option<bool>,
    /// `accessibilityDisplayShouldIncreaseContrast`.
    pub(crate) increase_contrast: Option<bool>,
    /// `accessibilityDisplayShouldReduceTransparency`.
    pub(crate) reduce_transparency: Option<bool>,
    /// `AppleInterfaceStyle` is dark (`detect::detect_is_dark`).
    pub(crate) is_dark: bool,
    /// The running macOS version (`os_version`).
    pub(crate) version: crate::pipeline::OsVersion,
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
    read_macos_settings().map(|settings| reader_result(&settings))
}

/// Read every setting the reader reads ([`MacosSettings`]): what
/// [`read_macos`] builds its result from, and the 0.8.0 reader its snapshot
/// (`next::reader::macos::read`).
///
/// Returns `Error::ReaderFailed` if neither light nor dark appearance can be
/// created.
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) fn read_macos_settings() -> crate::Result<MacosSettings> {
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

    let light = light_appearance.as_deref().map(read_variant);
    let dark = dark_appearance.as_deref().map(read_variant);

    // The scroll bar is appearance-independent and requires the main thread.
    let scrollbar = objc2::MainThreadMarker::new().map(read_scrollbar);

    // Accessibility flags (appearance-independent).
    let (reduce_motion, increase_contrast, reduce_transparency) = read_accessibility();

    Ok(MacosSettings {
        light,
        dark,
        font,
        mono_font,
        widget_fonts,
        scrollbar,
        reduce_motion,
        increase_contrast,
        reduce_transparency,
        is_dark: crate::detect::detect_is_dark(),
        version: os_version(),
    })
}

/// The 0.7.2 reader's result of the settings `settings`: both variants
/// ([`build_theme`]), each with its appearance's colours, the fonts, the
/// scroll bar and the disabled on switch's fade, and the accessibility
/// flags. An appearance that was not read leaves its variant without colours
/// and without the system and monospaced fonts.
#[cfg_attr(not(all(target_os = "macos", feature = "macos")), allow(dead_code))]
pub(crate) fn reader_result(settings: &MacosSettings) -> crate::ReaderResult {
    let variant_defaults = |colors: Option<&AppearanceColors>| match colors {
        Some(colors) => {
            let mut defaults = defaults_of(colors);
            defaults.font = settings.font.clone();
            defaults.mono_font = settings.mono_font.clone();
            defaults
        }
        None => crate::ThemeDefaults::default(),
    };
    let mut theme = build_theme(
        variant_defaults(settings.light.as_ref()),
        variant_defaults(settings.dark.as_ref()),
        &settings.widget_fonts,
    );

    let disabled_on_alpha = disabled_checked_alpha(settings.version);
    for (variant, colors) in [
        (&mut theme.light, settings.light.as_ref()),
        (&mut theme.dark, settings.dark.as_ref()),
    ] {
        let Some(v) = variant else { continue };
        if let Some(colors) = colors {
            apply_per_widget(v, colors);
        }
        apply_accent_fades(v, disabled_on_alpha);
        if let Some(scrollbar) = &settings.scrollbar {
            v.scrollbar.merge(scrollbar);
        }
    }

    let mut acc = crate::AccessibilityPreferences::default();
    if let Some(rm) = settings.reduce_motion {
        acc.reduce_motion = rm;
    }
    if let Some(hc) = settings.increase_contrast {
        acc.high_contrast = hc;
    }
    if let Some(rt) = settings.reduce_transparency {
        acc.reduce_transparency = rt;
    }

    let output = crate::ReaderOutput::Dual {
        light: Box::new(theme.light.unwrap_or_default()),
        dark: Box::new(theme.dark.unwrap_or_default()),
        is_dark: settings.is_dark,
    };

    crate::ReaderResult {
        output,
        name: theme.name,
        icon_set: theme.icon_set,
        layout: theme.layout,
        font_dpi: Some(FONT_DPI),
        accessibility: acc,
    }
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
    /// none of them, and the reader writes the accent, not the fills.
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

    /// D12: each preset states its disabled on track as its accent at the
    /// alpha the reader fades the user's accent by on that preset's
    /// generation (docs/platform-facts.md:1529): 0x80 on macOS 12 to 15, and
    /// on the 16 early macOS 26 betas report to an older SDK's binary, which
    /// the generation table gives macos-monterey; 0x73 from 26.
    #[test]
    fn a_disabled_on_switch_is_the_accent_at_its_generations_alpha() {
        use crate::pipeline::OsVersion;
        for (version, preset) in [
            (OsVersion::new(12, 0, 0), "macos-monterey"),
            (OsVersion::new(15, 7, 9), "macos-monterey"),
            (OsVersion::new(16, 0, 0), "macos-monterey"),
            (OsVersion::new(26, 6, 2), "macos-tahoe"),
            (OsVersion::new(27, 0, 1), "macos-tahoe"),
        ] {
            let theme = crate::Theme::preset(preset).unwrap();
            for mode in [theme.light.as_ref().unwrap(), theme.dark.as_ref().unwrap()] {
                let mut read = crate::ThemeMode {
                    defaults: crate::ThemeDefaults {
                        accent_color: mode.defaults.accent_color,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                apply_accent_fades(&mut read, disabled_checked_alpha(version));
                assert_eq!(
                    read.switch.disabled_checked_background,
                    mode.switch.disabled_checked_background,
                    "{preset} on {version}"
                );
            }
        }
    }

    /// D12: under another accent a disabled on switch is that accent at the
    /// generation's alpha, and macos-tahoe's selected segment is the accent,
    /// as macOS 26 draws it (docs/platform-facts.md:1529, :1636), while
    /// macos-monterey's stays the raised neutral segment.
    #[test]
    fn a_custom_accent_reaches_the_disabled_on_switch_and_tahoes_segment() {
        use crate::pipeline::OsVersion;
        let red = crate::Rgba::rgb(224, 56, 62);
        for (version, preset) in [
            (OsVersion::new(15, 7, 9), "macos-monterey"),
            (OsVersion::new(26, 6, 2), "macos-tahoe"),
        ] {
            let alpha = disabled_checked_alpha(version);
            let mut read = crate::ThemeMode {
                defaults: crate::ThemeDefaults {
                    accent_color: Some(red),
                    ..Default::default()
                },
                ..Default::default()
            };
            apply_accent_fades(&mut read, alpha);
            let mut mode = crate::Theme::preset(preset).unwrap().light.unwrap();
            mode.merge(&read);
            let r = mode
                .into_resolved(&crate::ResolutionContext::for_tests())
                .unwrap();
            assert_eq!(
                r.switch.disabled_checked_background,
                Some(crate::Rgba::new(224, 56, 62, alpha)),
                "{preset}"
            );
            assert_eq!(
                r.segmented_control.active_background == red,
                preset == "macos-tahoe",
                "{preset}"
            );
        }
    }

    /// D7: the reader asks for `textInsertionPointColor` from macOS 14 on,
    /// macOS 26's early betas' 16 included, and not below, where the selector
    /// does not exist.
    #[test]
    fn the_insertion_point_colour_is_read_from_macos_14() {
        use crate::pipeline::OsVersion;
        assert!(!has_text_insertion_point_color(OsVersion::new(13, 7, 8)));
        assert!(!has_text_insertion_point_color(OsVersion::new(12, 0, 0)));
        assert!(has_text_insertion_point_color(OsVersion::new(14, 0, 0)));
        assert!(has_text_insertion_point_color(OsVersion::new(15, 7, 9)));
        assert!(has_text_insertion_point_color(OsVersion::new(16, 0, 0)));
        assert!(has_text_insertion_point_color(OsVersion::new(26, 6, 2)));
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
        // Accessibility fields live on AccessibilityPreferences (constructed
        // by the pipeline), not on ThemeDefaults.
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
        let defaults = super::defaults_of(&super::read_appearance_colors());
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
        let defaults = super::defaults_of(&super::read_appearance_colors());
        assert_eq!(
            defaults.shadow_color, None,
            "NSColor.shadowColor is not the drawn shadow (platform-facts.md:1057)"
        );
        assert_eq!(
            defaults.selection_text_color,
            nscolor_to_rgba(&NSColor::alternateSelectedControlTextColor(), &srgb)
        );
        assert_eq!(
            defaults.selection_inactive_text_color,
            nscolor_to_rgba(&NSColor::labelColor(), &srgb)
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
