//! The GNOME reader (spec §8, "The GNOME part"): [`read`] reads what the
//! 0.7.2 reader reads, through its own reading function
//! (`gnome::read_gnome_data`: the XDG portal, gsettings and the session
//! type); the rest of this module takes the settings of such a read
//! ([`GnomePortalData`], as 0.7.2's `build_gnome_spec_pure` takes them), each
//! as read and parsed by the 0.7.2 reader's functions, but the fonts by the
//! clean-room Pango parser ([`pango_font_description`]) (plan, Appendix B).
//! Nothing but [`read`] reads D-Bus, gsettings or the environment, and
//! nothing is derived for a widget: what the 0.7.2 reader derives from these
//! settings the `adwaita` builder derives.

use std::sync::Arc;

use super::pango_font_description::{PangoDescription, pango_font_description};
use super::{DiagnosticEntry, OsFont, SystemOverrides, SystemSnapshot};
use crate::gnome::{
    GnomePortalData, caret_blink, effective_high_contrast, toolkit_scale_carries_text_scale,
};
use crate::next::color::Color;
use crate::next::error::Error;
use crate::next::preset::Preset;
use crate::next::query::{Contrast, MotionPreference, Scheme, ThemeContext};
use crate::next::style::{FontSize, FontStyle};

/// The theme's name, as the 0.7.2 reader names it (`gnome::build_gnome_spec_pure`).
const THEME_NAME: &str = "GNOME";

/// The source a diagnostic of this reader names.
const SOURCE: &str = "portal";

/// A Pango size's units per point, or per pixel where the size is absolute:
/// the parser's `size` is in 1/1024 of either
/// ([`PangoDescription::size`]).
const PANGO_UNITS: f64 = 1024.0;

/// The GNOME system's settings, read as 0.7.2's `GnomeReader` reads them
/// (`gnome::read_gnome_data`: a setting the portal does not give within its
/// deadline is gsettings', and what neither gives is not stated), as
/// [`snapshot_from_gnome_data`] takes them. No read fails.
pub(crate) fn read() -> Result<SystemSnapshot, Error> {
    Ok(snapshot_from_gnome_data(&pollster::block_on(
        crate::gnome::read_gnome_data(),
    )))
}

/// The snapshot of the settings `data` states: the `adwaita` preset,
/// [`context_from_gnome_data`] and [`overrides_from_gnome_data`], with a
/// diagnostic where the text scale is not carried.
pub(crate) fn snapshot_from_gnome_data(data: &GnomePortalData) -> SystemSnapshot {
    let (context, diagnostic) = context_from_gnome_data(data);
    let mut overrides = overrides_from_gnome_data(data);
    overrides.diagnostics.extend(diagnostic);
    SystemSnapshot::new(Preset::Adwaita, context, overrides)
}

/// The context the system is in by the settings `data` states (spec §8,
/// "The GNOME part", *Context*): the scheme read; `Contrast::High` where the
/// portal's `contrast` is high or gsettings' `high-contrast` true
/// (`gnome::effective_high_contrast`); `MotionPreference::Reduced` where the
/// portal's `reduced-motion`, else gsettings' `enable-animations`, asks for
/// it; the text scale `text-scaling-factor`, and 1.0 on an X11 session,
/// whose toolkit scale factor carries it already
/// (`gnome::toolkit_scale_carries_text_scale`). A factor that is not a finite
/// number above 0 is not carried: the text scale stays 1.0, and the
/// diagnostic returned says so. Every other field is the default, as the
/// 0.7.2 reader leaves `reduce_transparency`.
pub(crate) fn context_from_gnome_data(
    data: &GnomePortalData,
) -> (ThemeContext, Option<DiagnosticEntry>) {
    let mut context = ThemeContext::default().with_scheme(scheme(data));
    if effective_high_contrast(data) {
        context.contrast = Contrast::High;
    }
    if data.reduce_motion == Some(true) {
        context.motion = MotionPreference::Reduced;
    }
    let factor = data
        .text_scaling_factor
        .filter(|_| !toolkit_scale_carries_text_scale(data.session_type.as_deref()));
    let mut diagnostic = None;
    match factor {
        Some(factor) if factor.is_finite() && factor > 0.0 => {
            context = context.with_text_scale(factor);
        }
        Some(factor) => {
            diagnostic = Some(DiagnosticEntry::new(
                SOURCE,
                format!(
                    "text-scaling-factor {factor} is not a finite number above 0; the text scale stays 1.0"
                ),
            ));
        }
        None => {}
    }
    (context, diagnostic)
}

/// The settings `data` states (spec §8, "The GNOME part", *Settings*), each
/// `Some` only where read and parsed, except the three the 0.7.2 reader
/// always sets: `scheme`, `theme_name` ([`THEME_NAME`]) and `font_dpi` (96).
/// The reader makes no entry of `color_schemes`; `icon_sizes` stays `None`.
pub(crate) fn overrides_from_gnome_data(data: &GnomePortalData) -> SystemOverrides {
    let mut caret = crate::model::InputTheme::default();
    caret_blink(data, &mut caret);
    SystemOverrides {
        scheme: Some(scheme(data)),
        theme_name: Some(Arc::from(THEME_NAME)),
        accent: data.accent().map(|c| Color::from_srgb8(c.r, c.g, c.b, c.a)),
        ui_font: font(data.font_name.as_deref()),
        monospace_font: font(data.monospace_font_name.as_deref()),
        title_bar_font: font(data.titlebar_font.as_deref()),
        font_dpi: Some(crate::detect::LINUX_FONT_DPI),
        icon_theme: data.icon_theme.as_deref().map(Arc::from),
        caret_blink_on_ms: caret.caret_blink_on,
        caret_blink_off_ms: caret.caret_blink_off,
        caret_blink_timeout_ms: caret.caret_blink_timeout,
        overlay_scrollbars: data.overlay_scrolling,
        ..SystemOverrides::default()
    }
}

/// The scheme of `data`: dark where the portal's, else gsettings',
/// `color-scheme` prefers dark, else light (`detect::color_scheme_is_dark`,
/// read by `gnome::gnome_data`).
fn scheme(data: &GnomePortalData) -> Scheme {
    if data.is_dark {
        Scheme::Dark
    } else {
        Scheme::Light
    }
}

/// The font of a GNOME font string `value`, as Pango reads it
/// ([`pango_font_description`], [`font_of`]).
pub(crate) fn font(value: Option<&str>) -> Option<OsFont> {
    font_of(&pango_font_description(value?))
}

/// The font of a Pango description: the family list's first entry that is
/// not empty (the font holds one name, where Pango keeps the list for
/// fontconfig); the size above 0 in Pango's units ([`PANGO_UNITS`]),
/// `FontSize::Px` where absolute, else `FontSize::Pt`; the weight bounded to
/// CSS 100..=900, as 0.7.2's resolution bounds a font's (`check_font`); the
/// style (`PangoStyle`: 0 normal, 1 oblique, 2 italic; any other `style=N`
/// value `None`). `None` where the description has no family or no size
/// above 0: it names no font, so the preset's stays, as 0.7.2's
/// `gnome::parse_gnome_font_to_fontspec` returns none for it.
fn font_of(desc: &PangoDescription) -> Option<OsFont> {
    let family = desc
        .family
        .as_deref()?
        .split(',')
        .find(|entry| !entry.is_empty())?;
    let units = desc
        .size
        .map(|size| (f64::from(size) / PANGO_UNITS) as f32)
        .filter(|size| *size > 0.0)?;
    Some(OsFont {
        family: Some(Arc::from(family)),
        size: Some(if desc.absolute {
            FontSize::Px(units)
        } else {
            FontSize::Pt(units)
        }),
        weight: u16::try_from(desc.weight.clamp(100, 900)).ok(),
        style: match desc.style {
            0 => Some(FontStyle::Normal),
            1 => Some(FontStyle::Oblique),
            2 => Some(FontStyle::Italic),
            _ => None,
        },
        width: None,
    })
}

#[cfg(test)]
mod tests;
