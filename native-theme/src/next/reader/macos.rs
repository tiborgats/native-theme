//! The macOS reader (spec §8, "The macOS part"): `read` reads what the 0.7.2
//! reader reads, through its own reading function
//! (`macos::read_macos_settings`: the semantic `NSColor`s under Aqua and Dark
//! Aqua, the five `NSFont`s, the scroller's style and width, the three
//! accessibility flags, `AppleInterfaceStyle` and the running version); the
//! rest of this module takes the settings of such a read (`MacosSettings`, as
//! 0.7.2's `reader_result` takes them), each as read and parsed by the 0.7.2
//! reader's functions. Nothing but `read` calls AppKit, and nothing is
//! derived for a widget: what the 0.7.2 reader derives from these settings
//! (the copies of `labelColor` and `alternateSelectedControlTextColor`, the
//! disabled on switch's fade) the macOS builders derive.

use std::sync::Arc;

use super::{
    OsColors, OsFont, OsSelectionColors, OsSurfaceOverrides, SystemOverrides, SystemSnapshot,
};
use crate::macos::{AppearanceColors, MacosSettings};
use crate::next::color::Color;
#[cfg(all(target_os = "macos", feature = "macos"))]
use crate::next::error::Error;
use crate::next::preset::Preset;
use crate::next::query::{Contrast, MotionPreference, Scheme, ThemeContext};
use crate::next::roles::SurfaceColors;
use crate::next::style::FontSize;
use crate::pipeline::OsVersion;

/// The theme's name, as the 0.7.2 reader names it (`macos::build_theme`).
const THEME_NAME: &str = "macOS";

/// The source a diagnostic of this reader names.
#[cfg(all(target_os = "macos", feature = "macos"))]
const SOURCE: &str = "macos";

/// The macOS system's settings, read as 0.7.2's `MacosReader` reads them
/// (`macos::read_macos_settings`), as [`snapshot_from_settings`] takes them;
/// the read's error where neither appearance can be created, as 0.7.2's
/// reader fails.
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) fn read() -> Result<SystemSnapshot, Error> {
    crate::macos::read_macos_settings()
        .map(|settings| snapshot_from_settings(&settings))
        .map_err(|error| Error::ReaderFailed {
            reader: SOURCE,
            message: error.to_string(),
        })
}

/// The preset of the running version `version`: `MacosTahoe` where 0.7.2's
/// generation table (`pipeline::macos_generation`) gives `macos-tahoe`, else
/// `MacosMonterey`.
pub(crate) fn preset_of(version: OsVersion) -> Preset {
    if crate::pipeline::macos_generation(version).0.name == Preset::MacosTahoe.as_str() {
        Preset::MacosTahoe
    } else {
        Preset::MacosMonterey
    }
}

/// The snapshot of the settings `settings` states: the preset of the
/// running version ([`preset_of`]), [`context_from_settings`] and
/// [`overrides_from_settings`].
pub(crate) fn snapshot_from_settings(settings: &MacosSettings) -> SystemSnapshot {
    SystemSnapshot::new(
        preset_of(settings.version),
        context_from_settings(settings),
        overrides_from_settings(settings),
    )
}

/// The context the system is in by the settings `settings` states (spec §8,
/// "The macOS part", *Context*): the scheme of `AppleInterfaceStyle`;
/// `Contrast::Increased` where `accessibilityDisplayShouldIncreaseContrast`,
/// `MotionPreference::Reduced` where `accessibilityDisplayShouldReduceMotion`,
/// `reduced_transparency` from `accessibilityDisplayShouldReduceTransparency`;
/// the text scale 1.0, as the 0.7.2 reader leaves it. A flag that was not
/// read leaves its field at the default.
pub(crate) fn context_from_settings(settings: &MacosSettings) -> ThemeContext {
    let mut context = ThemeContext::default().with_scheme(scheme(settings));
    if settings.increase_contrast == Some(true) {
        context.contrast = Contrast::Increased;
    }
    if settings.reduce_motion == Some(true) {
        context.motion = MotionPreference::Reduced;
    }
    if settings.reduce_transparency == Some(true) {
        context.reduced_transparency = true;
    }
    context
}

/// The settings `settings` states (spec §8, "The macOS part", *Colours* and
/// *Scheme-independent*): each appearance read as its entry of
/// `color_schemes` at `Contrast::Normal` ([`appearance_colors`]); the five
/// fonts; the scroller's style as `overlay_scrollbars` and its width as
/// `scrollbar_width_px`, where read; and the three the 0.7.2 reader always
/// sets: `scheme`, `theme_name` ([`THEME_NAME`]) and `font_dpi`
/// (`macos::FONT_DPI`).
pub(crate) fn overrides_from_settings(settings: &MacosSettings) -> SystemOverrides {
    let color_schemes = [
        (Scheme::Light, settings.light.as_ref()),
        (Scheme::Dark, settings.dark.as_ref()),
    ]
    .into_iter()
    .filter_map(|(scheme, colors)| {
        colors.map(|colors| ((scheme, Contrast::Normal), appearance_colors(colors)))
    })
    .collect();
    let scrollbar = settings.scrollbar.as_ref();
    SystemOverrides {
        color_schemes,
        scheme: Some(scheme(settings)),
        theme_name: Some(Arc::from(THEME_NAME)),
        ui_font: font(&settings.font),
        monospace_font: font(&settings.mono_font),
        menu_font: font(&settings.widget_fonts.menu_font),
        tooltip_font: font(&settings.widget_fonts.tooltip_font),
        title_bar_font: font(&settings.widget_fonts.title_bar_font),
        font_dpi: Some(crate::macos::FONT_DPI),
        overlay_scrollbars: scrollbar.and_then(|s| s.overlay_mode),
        scrollbar_width_px: scrollbar.and_then(|s| s.groove_width),
        ..SystemOverrides::default()
    }
}

/// The entry of the colours read under one appearance (spec §8, "The macOS
/// part", *Colours*): `labelColor`, `windowBackgroundColor`,
/// `secondaryLabelColor` and `separatorColor` as the window's foreground,
/// background, muted text and border; `controlBackgroundColor` as the
/// view's background; `windowFrameTextColor` as the title bar's foreground;
/// `controlAccentColor` as the accent; `selectedContentBackgroundColor`,
/// `alternateSelectedControlTextColor` and
/// `unemphasizedSelectedContentBackgroundColor` as the selection's
/// background, text and unfocused background; `selectedTextBackgroundColor`
/// and `selectedTextColor` as the text selection's; `linkColor`,
/// `keyboardFocusIndicatorColor`, the four system colours, the disabled
/// text, the placeholder, the caret, the alternate row, the table header's
/// text and the grid each in its slot.
fn appearance_colors(colors: &AppearanceColors) -> OsColors {
    let c = |rgba: Option<crate::Rgba>| rgba.map(color);
    OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors::new(
                c(colors.window_background),
                c(colors.label),
                c(colors.secondary_label),
                c(colors.separator),
                None,
            )),
            view: Some(SurfaceColors::new(
                c(colors.control_background),
                None,
                None,
                None,
                None,
            )),
            title_bar: Some(SurfaceColors::new(
                None,
                c(colors.window_frame_text),
                None,
                None,
                None,
            )),
            ..OsSurfaceOverrides::default()
        }),
        selection: Some(OsSelectionColors::new(
            c(colors.selected_content_background),
            c(colors.alternate_selected_control_text),
            c(colors.unemphasized_selected_content_background),
            None,
        )),
        text_selection: Some(OsSelectionColors::new(
            c(colors.selected_text_background),
            c(colors.selected_text),
            None,
            None,
        )),
        accent: c(colors.control_accent),
        focus_ring: c(colors.keyboard_focus_indicator),
        link: c(colors.link),
        status: Some([
            c(colors.system_red),
            c(colors.system_orange),
            c(colors.system_green),
            c(colors.system_blue),
        ]),
        disabled_text: c(colors.disabled_control_text),
        placeholder: c(colors.placeholder_text),
        caret: c(colors.text_insertion_point),
        list_header_foreground: c(colors.header_text),
        grid: c(colors.grid),
        alternate_row_background: c(colors.alternate_row),
        ..OsColors::default()
    }
}

/// The scheme of `settings`: dark where `AppleInterfaceStyle` is dark, else
/// light.
fn scheme(settings: &MacosSettings) -> Scheme {
    if settings.is_dark {
        Scheme::Dark
    } else {
        Scheme::Light
    }
}

/// The colour of a read `Rgba`, each 8-bit channel as `nscolor_to_rgba`
/// rounds it.
fn color(c: crate::Rgba) -> Color {
    Color::from_srgb8(c.r, c.g, c.b, c.a)
}

/// The system font of an `NSFont` as 0.7.2 reads it
/// (`macos::fontspec_from_nsfont`): its family, its size in points, its
/// weight by the reader's buckets of AppKit's weight, and no style; `None`
/// where the read states none of them.
fn font(spec: &crate::model::FontSpec) -> Option<OsFont> {
    let size = spec.size.map(|size| match size {
        crate::model::font::FontSize::Pt(pt) => FontSize::Pt(pt),
        crate::model::font::FontSize::Px(px) => FontSize::Px(px),
    });
    if spec.family.is_none() && size.is_none() && spec.weight.is_none() {
        return None;
    }
    Some(OsFont::new(
        spec.family.clone(),
        size,
        spec.weight,
        None,
        None,
    ))
}

#[cfg(test)]
mod tests;
