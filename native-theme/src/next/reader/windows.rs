//! The Windows reader (spec §8, "The Windows part"): `read` reads what the
//! 0.7.2 reader reads, through its own reading function
//! (`windows::read_windows_settings`: `UISettings`, `NONCLIENTMETRICSW`, the
//! system metrics, the accessibility settings, under high contrast the
//! system colours, and the caret's blink time); the rest of this module
//! takes the settings of such a read (`WindowsSettings`, as 0.7.2's
//! `build_theme` takes them), each as read and parsed by the 0.7.2 reader's
//! functions. Nothing but `read` calls Windows, and nothing is derived for
//! a widget: what the 0.7.2 reader derives from these settings (the accent's
//! fills, the link's shades, the high-contrast mapping) the `windows-11`
//! builder derives.

use std::sync::Arc;

use super::{
    DiagnosticEntry, OsColors, OsFont, OsSelectionColors, OsSurfaceOverrides, SystemOverrides,
    SystemSnapshot, WindowsAccentRamp,
};
use crate::next::color::Color;
#[cfg(all(target_os = "windows", feature = "windows"))]
use crate::next::error::Error;
use crate::next::icons::IconSizes;
use crate::next::preset::Preset;
use crate::next::query::{Contrast, MotionPreference, Scheme, ThemeContext};
use crate::next::roles::SurfaceColors;
use crate::next::style::FontSize;
use crate::windows::{SysColors, WindowsSettings};

/// The theme's name, as the 0.7.2 reader names it (`windows::build_theme`).
const THEME_NAME: &str = "Windows";

/// The source a diagnostic of this reader names.
const SOURCE: &str = "windows";

/// The Windows system's settings, read as 0.7.2's `WindowsReader` reads them
/// (`windows::read_windows_settings`), as [`snapshot_from_settings`] takes
/// them; the read's error where `UISettings`, the accent or the foreground
/// cannot be read, as 0.7.2's reader fails.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) fn read() -> Result<SystemSnapshot, Error> {
    crate::windows::read_windows_settings()
        .map(|settings| snapshot_from_settings(&settings))
        .map_err(|error| Error::ReaderFailed {
            reader: SOURCE,
            message: error.to_string(),
        })
}

/// The snapshot of the settings `settings` states: the `windows-11` preset,
/// [`context_from_settings`] and [`overrides_from_settings`], with a
/// diagnostic where the text scale is not carried.
pub(crate) fn snapshot_from_settings(settings: &WindowsSettings) -> SystemSnapshot {
    let (context, diagnostic) = context_from_settings(settings);
    let mut overrides = overrides_from_settings(settings);
    overrides.diagnostics.extend(diagnostic);
    SystemSnapshot::new(Preset::Windows11, context, overrides)
}

/// The context the system is in by the settings `settings` states (spec §8,
/// "The Windows part", *Context*): the scheme of [`scheme`];
/// `Contrast::High` under high contrast (`SPI_GETHIGHCONTRAST`);
/// `MotionPreference::None` where `SPI_GETCLIENTAREAANIMATION` is off;
/// `reduced_transparency` where `AdvancedEffectsEnabled` is false; the text
/// scale `UISettings.TextScaleFactor`. A factor that is not a finite number
/// above 0 is not carried: the text scale stays 1.0, and the diagnostic
/// returned says so. A setting that was not read leaves its field at the
/// default, as 0.7.2's `AccessibilityPreferences` (`windows::build_theme`).
pub(crate) fn context_from_settings(
    settings: &WindowsSettings,
) -> (ThemeContext, Option<DiagnosticEntry>) {
    let mut context = ThemeContext::default().with_scheme(scheme(settings));
    let Some(accessibility) = settings.accessibility else {
        return (context, None);
    };
    if accessibility.high_contrast == Some(true) {
        context.contrast = Contrast::High;
    }
    if accessibility.reduce_motion == Some(true) {
        context.motion = MotionPreference::None;
    }
    if accessibility.advanced_effects_enabled == Some(false) {
        context.reduced_transparency = true;
    }
    let mut diagnostic = None;
    match accessibility.text_scaling_factor {
        Some(factor) if factor.is_finite() && factor > 0.0 => {
            context = context.with_text_scale(factor);
        }
        Some(factor) => {
            diagnostic = Some(DiagnosticEntry::new(
                SOURCE,
                format!(
                    "TextScaleFactor {factor} is not a finite number above 0; the text scale stays 1.0"
                ),
            ));
        }
        None => {}
    }
    (context, diagnostic)
}

/// The settings `settings` states (spec §8, "The Windows part",
/// *Scheme-independent* and *High contrast*), each `Some` only where read,
/// except the three the 0.7.2 reader always sets: `scheme`, `theme_name`
/// ([`THEME_NAME`]) and `font_dpi` (96, `windows::LOGICAL_DPI`). The caret
/// times are 0.7.2's mapping of the blink time
/// (`windows::apply_caret_blink_time`). Only under high contrast, where the
/// system colours are read, the entry `(scheme, High)` of `color_schemes`
/// holds them ([`high_contrast_colors`]).
pub(crate) fn overrides_from_settings(settings: &WindowsSettings) -> SystemOverrides {
    let scheme = scheme(settings);
    let mut caret = crate::model::InputTheme::default();
    crate::windows::apply_caret_blink_time(&mut caret, settings.caret_blink_time);
    let [dark1, dark2, dark3, light1, light2, light3] =
        settings.accent_shades.map(|c| c.map(color));
    let high_contrast = settings
        .accessibility
        .is_some_and(|a| a.high_contrast == Some(true));
    let color_schemes = match (high_contrast, settings.sys_colors.as_ref()) {
        (true, Some(colors)) => vec![((scheme, Contrast::High), high_contrast_colors(colors))],
        _ => Vec::new(),
    };
    SystemOverrides {
        color_schemes,
        scheme: Some(scheme),
        theme_name: Some(Arc::from(THEME_NAME)),
        accent: Some(color(settings.accent)),
        windows_accent_ramp: Some(WindowsAccentRamp {
            light: [light1, light2, light3],
            dark: [dark1, dark2, dark3],
        }),
        ui_font: font(&settings.fonts.msg),
        title_bar_font: font(&settings.fonts.caption),
        menu_font: font(&settings.fonts.menu),
        status_bar_font: font(&settings.fonts.status),
        font_dpi: Some(crate::windows::LOGICAL_DPI as f32),
        icon_sizes: settings
            .icon_sizes
            .map(|(small, large)| IconSizes::new(None, Some(small), Some(large), None, None)),
        caret_blink_on_ms: caret.caret_blink_on,
        caret_blink_off_ms: caret.caret_blink_off,
        scrollbar_width_px: Some(settings.sizing.scrollbar_width),
        scrollbar_min_thumb_length_px: Some(settings.sizing.min_thumb_length),
        border_width_px: Some(settings.sizing.border_width),
        ..SystemOverrides::default()
    }
}

/// The high-contrast entry of the system colours `colors` (spec §8, "The
/// Windows part", *High contrast*): `COLOR_WINDOW` and `COLOR_WINDOWTEXT` as
/// the window's background and foreground, `COLOR_BTNFACE` and
/// `COLOR_BTNTEXT` as the button's, `COLOR_HIGHLIGHT` and
/// `COLOR_HIGHLIGHTTEXT` as the selection's, `COLOR_HOTLIGHT` as the link
/// and `COLOR_GRAYTEXT` as the disabled text.
fn high_contrast_colors(colors: &SysColors) -> OsColors {
    let pair = |background, foreground| {
        Some(SurfaceColors::new(
            Some(color(background)),
            Some(color(foreground)),
            None,
            None,
            None,
        ))
    };
    OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: pair(colors.window_bg, colors.window_text),
            button: pair(colors.btn_face, colors.btn_text),
            ..OsSurfaceOverrides::default()
        }),
        selection: Some(OsSelectionColors::new(
            Some(color(colors.highlight)),
            Some(color(colors.highlight_text)),
            None,
            None,
        )),
        link: Some(color(colors.hotlight)),
        disabled_text: Some(color(colors.gray_text)),
        ..OsColors::default()
    }
}

/// The scheme of `settings`: dark where the foreground is light, by the
/// 0.7.2 reader's luma test (`windows::is_dark_mode`), else light.
fn scheme(settings: &WindowsSettings) -> Scheme {
    if crate::windows::is_dark_mode(&settings.foreground) {
        Scheme::Dark
    } else {
        Scheme::Light
    }
}

/// The colour of a read `Rgba`, each 8-bit channel as read.
fn color(c: crate::Rgba) -> Color {
    Color::from_srgb8(c.r, c.g, c.b, c.a)
}

/// The system font of a `NONCLIENTMETRICSW` font as 0.7.2 parses it
/// (`windows::logfont_to_fontspec_raw`): its family, its size in points
/// (`|lfHeight| × 72 / 96`), its weight (`None` for `FW_DONTCARE`), and no
/// style; `None` where the parse states none of them (the call failed).
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
