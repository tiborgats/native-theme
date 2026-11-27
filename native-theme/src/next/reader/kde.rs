//! The KDE reader (spec §8, "The KDE part"): [`read`] reads what the 0.7.2
//! reader reads, through its own reading functions (`crate::kde`, and
//! `crate::gnome` for the portal's accent); the rest of this module takes
//! the settings of a parsed `kdeglobals`, each as read, parsed by the 0.7.2
//! reader's functions, but the fonts by the clean-room `QFont` parser
//! ([`qfont_from_string`]) and the scheme by the clean-room dark/light rule
//! ([`kde_color_scheme()`]) (plan, Appendix B). Nothing but [`read`] reads a
//! file or D-Bus, and nothing is derived: what the 0.7.2 reader derives from
//! these settings the `kde-breeze` builder derives.

use std::sync::Arc;

use configparser::ini::Ini;

use super::kde_color_scheme::{self, kde_color_scheme};
use super::qfont_from_string::{QtFontFields, qfont_from_string};
use super::{
    KdeColorEffects, OsColors, OsFont, OsSelectionColors, OsSurfaceOverrides, SystemOverrides,
    SystemSnapshot,
};
use crate::kde::colors::{get_color, kconfig_bool};
use crate::kde::{
    caret_blink, create_kde_parser, icon_theme, parse_icon_sizes_from_index_theme, read_kdeglobals,
};
use crate::next::color::Color;
use crate::next::error::Error;
use crate::next::icons::IconSizes;
use crate::next::preset::Preset;
use crate::next::query::{Contrast, MotionPreference, Scheme, ThemeContext};
use crate::next::roles::SurfaceColors;
use crate::next::style::{FontSize, FontStyle};

/// The theme's name where kdeglobals states no `[General] ColorScheme`, as
/// the 0.7.2 reader names it (`kde::read_kde_ini`).
const UNNAMED_SCHEME: &str = "KDE";

/// The KDE system's settings: kdeglobals as KConfig cascades it
/// (`kde::read_kdeglobals`; no file reads as an empty one, as in 0.7.2's
/// `KdeReader`), the named icon theme's default sizes from its `index.theme`
/// (`kde::parse_icon_sizes_from_index_theme`) and, with `portal`, the XDG
/// portal's accent (`gnome::portal_accent`, 0.7.2's
/// `GnomePortalKdeReader`), as [`snapshot_from_kdeglobals`] takes them. An
/// icon theme whose `index.theme` is not found states no sizes. No read
/// fails: what cannot be read is not stated.
pub(crate) fn read() -> Result<SystemSnapshot, Error> {
    let ini = read_kdeglobals().unwrap_or_else(create_kde_parser);
    let icon_sizes = icon_theme(&ini)
        .map(|theme| parse_icon_sizes_from_index_theme(&theme))
        .filter(|sizes| *sizes != crate::model::IconSizes::default());
    #[cfg(feature = "portal")]
    let portal_accent = pollster::block_on(crate::gnome::portal_accent());
    #[cfg(not(feature = "portal"))]
    let portal_accent = None;
    Ok(snapshot_from_kdeglobals(&ini, icon_sizes, portal_accent))
}

/// The snapshot of the settings `ini` states: the `kde-breeze` preset,
/// [`context_from_kdeglobals`] and [`overrides_from_kdeglobals`] with
/// `icon_sizes` and `portal_accent`.
// `allow`, not `expect`, for the reason given on `context_from_kdeglobals`.
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "called by the KDE reader's `read()` and the reader parity test"
    )
)]
pub(crate) fn snapshot_from_kdeglobals(
    ini: &Ini,
    icon_sizes: Option<crate::model::IconSizes>,
    portal_accent: Option<crate::Rgba>,
) -> SystemSnapshot {
    SystemSnapshot::new(
        Preset::KdeBreeze,
        context_from_kdeglobals(ini),
        overrides_from_kdeglobals(ini, icon_sizes, portal_accent),
    )
}

/// The context the system is in by the settings `ini` states: the scheme of
/// [`overrides_from_kdeglobals`], and `MotionPreference::None` where
/// `[KDE] AnimationDurationFactor` is 0, the 0.7.2 reader's
/// `accessibility.reduce_motion` (`kde::reduce_motion`). Every other field is
/// the default, as the 0.7.2 reader leaves the rest of its `accessibility`
/// block: a text scale of 1.0, no high contrast, no reduced transparency.
// `allow`, not `expect`: rustc 1.88 (the MSRV) does not report a function
// that only an unused function calls, so an expectation here is unfulfilled
// there and fulfilled on the current toolchain.
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "called by the KDE reader's `read()` (plan, M2) and the reader parity test"
    )
)]
pub(crate) fn context_from_kdeglobals(ini: &Ini) -> ThemeContext {
    let mut context = ThemeContext::default().with_scheme(scheme(ini));
    if animation_duration_factor(ini) == Some(0.0) {
        context.motion = MotionPreference::None;
    }
    context
}

/// The settings `ini` states, with the icon theme's default sizes
/// `icon_sizes` (read from its `index.theme` by the caller) and the XDG
/// portal's accent `portal_accent`, each where given.
///
/// Every slot is `Some` only where the file states its setting in a form its
/// parser accepts, except the three the 0.7.2 reader always sets: `scheme`
/// (by the clean-room rule of [`scheme`]),
/// `font_dpi` (96) and `theme_name` (`[General] ColorScheme`, else
/// [`UNNAMED_SCHEME`]). The colours, the two contrast scalars and the colour
/// effects form the entry of `color_schemes` for `(scheme, Contrast::Normal)`,
/// which is left out where the file states none of them. The motion
/// preference that `[KDE] AnimationDurationFactor = 0` means is the
/// snapshot's context ([`context_from_kdeglobals`]).
// `allow`, not `expect`, for the reason given on `context_from_kdeglobals`.
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "called by the KDE reader's `read()` (plan, M2) and the kde-breeze builder's tests"
    )
)]
pub(crate) fn overrides_from_kdeglobals(
    ini: &Ini,
    icon_sizes: Option<crate::model::IconSizes>,
    portal_accent: Option<crate::Rgba>,
) -> SystemOverrides {
    let scheme = scheme(ini);
    let colors = os_colors(ini);
    let mut caret = crate::model::InputTheme::default();
    caret_blink(ini, &mut caret);
    let icon_theme = icon_theme(ini).map(Arc::from);
    // The sizes are the named theme's: without a theme they are dropped with it.
    let icon_sizes = icon_sizes
        .filter(|_| icon_theme.is_some())
        .map(|s| IconSizes::new(s.toolbar, s.small, s.large, s.dialog, s.panel));
    SystemOverrides {
        color_schemes: stated(colors)
            .map(|colors| ((scheme, Contrast::Normal), colors))
            .into_iter()
            .collect(),
        scheme: Some(scheme),
        theme_name: Some(Arc::from(
            ini.get("General", "ColorScheme")
                .as_deref()
                .unwrap_or(UNNAMED_SCHEME),
        )),
        accent: portal_accent.map(color_of),
        ui_font: font(ini, "General", "font"),
        monospace_font: font(ini, "General", "fixed"),
        caption_font: font(ini, "General", "smallestReadableFont"),
        menu_font: font(ini, "General", "menuFont"),
        toolbar_font: font(ini, "General", "toolBarFont"),
        title_bar_font: font(ini, "WM", "activeFont"),
        font_dpi: Some(crate::detect::LINUX_FONT_DPI),
        icon_theme,
        icon_sizes,
        // A non-finite factor refused.
        animation_scale: animation_duration_factor(ini).filter(|value| value.is_finite()),
        caret_blink_on_ms: caret.caret_blink_on,
        caret_blink_off_ms: caret.caret_blink_off,
        ..SystemOverrides::default()
    }
}

/// The scheme Plasma reports for `ini` by the clean-room rule
/// ([`kde_color_scheme()`], its table `tests/vectors/reader.kde_color_scheme.tsv`):
/// dark where the grey of `[Colors:Window] BackgroundNormal` is below 192,
/// else light, Breeze Light's window colour standing in where the entry is
/// absent or not a colour KConfig reads. It replaces 0.7.2's
/// `kde::is_dark_theme` (plan, Appendix B).
fn scheme(ini: &Ini) -> Scheme {
    match kde_color_scheme(ini.get("Colors:Window", "BackgroundNormal").as_deref()) {
        kde_color_scheme::Scheme::Dark => Scheme::Dark,
        kde_color_scheme::Scheme::Light => Scheme::Light,
    }
}

/// `[KDE] AnimationDurationFactor` as the 0.7.2 reader parses it
/// (`kde::reduce_motion`).
fn animation_duration_factor(ini: &Ini) -> Option<f32> {
    ini.get("KDE", "AnimationDurationFactor")
        .and_then(|value| value.trim().parse::<f32>().ok())
}

/// The colours, contrast scalars and colour effects `ini` states.
fn os_colors(ini: &Ini) -> OsColors {
    let view = |key: &str| color(ini, "Colors:View", key);
    let status = [
        view("ForegroundNegative"),
        view("ForegroundNeutral"),
        view("ForegroundPositive"),
        view("ForegroundActive"),
    ];
    let surfaces = OsSurfaceOverrides {
        window: set(ini, "Colors:Window", true),
        view: set(ini, "Colors:View", true),
        header: set(ini, "Colors:Header", false),
        tooltip: set(ini, "Colors:Tooltip", false),
        title_bar: surface(
            color(ini, "WM", "activeBackground"),
            color(ini, "WM", "activeForeground"),
            None,
        ),
        title_bar_inactive: surface(
            color(ini, "WM", "inactiveBackground"),
            color(ini, "WM", "inactiveForeground"),
            None,
        ),
        button: set(ini, "Colors:Button", false),
        ..OsSurfaceOverrides::default()
    };
    let selection = OsSelectionColors {
        background: color(ini, "Colors:Selection", "BackgroundNormal"),
        foreground: color(ini, "Colors:Selection", "ForegroundNormal"),
        ..OsSelectionColors::default()
    };
    OsColors {
        surfaces: stated(surfaces),
        selection: stated(selection),
        link: view("ForegroundLink"),
        link_visited: view("ForegroundVisited"),
        status: status.iter().any(Option::is_some).then_some(status),
        alternate_row_background: view("BackgroundAlternate"),
        kde_decoration_focus: view("DecorationFocus"),
        kde_decoration_hover: view("DecorationHover"),
        // Parsed as `kde::colors::frame_contrast` and `contrast` parse them,
        // without the defaults those functions put where the setting is
        // unset or not a number (the builder's, spec §8).
        kde_frame_contrast: ini
            .get("KDE", "frameContrast")
            .and_then(|v| v.trim().parse::<f64>().ok())
            .filter(|v| v.is_finite())
            .map(|v| v.clamp(0.0, 1.0)),
        kde_contrast: ini
            .get("KDE", "contrast")
            .and_then(|v| v.trim().parse::<i32>().ok()),
        // As `kde::colors::inactive_change_selection_color` resolves it, but
        // `None` where neither key is stated.
        kde_effects: kconfig_bool(ini.get("ColorEffects:Inactive", "ChangeSelectionColor"))
            .or_else(|| kconfig_bool(ini.get("ColorEffects:Inactive", "Enable")))
            .map(|change| KdeColorEffects::new(None, None, change)),
        ..OsColors::default()
    }
}

/// `value`, or `None` where it states nothing (equals its default).
fn stated<T: Default + PartialEq>(value: T) -> Option<T> {
    (value != T::default()).then_some(value)
}

/// The colour `ini` states at `section`/`key`.
fn color(ini: &Ini, section: &str, key: &str) -> Option<Color> {
    get_color(ini, section, key).map(color_of)
}

/// A 0.7.2 colour as a 0.8.0 one.
fn color_of(c: crate::Rgba) -> Color {
    Color::from_srgb8(c.r, c.g, c.b, c.a)
}

/// The surface of these colours, or `None` where none is stated.
fn surface(
    background: Option<Color>,
    foreground: Option<Color>,
    foreground_muted: Option<Color>,
) -> Option<SurfaceColors> {
    stated(SurfaceColors::new(
        background,
        foreground,
        foreground_muted,
        None,
        None,
    ))
}

/// A colour set's `BackgroundNormal` and `ForegroundNormal`, and where
/// `muted`, its `ForegroundInactive`.
fn set(ini: &Ini, section: &str, muted: bool) -> Option<SurfaceColors> {
    surface(
        color(ini, section, "BackgroundNormal"),
        color(ini, section, "ForegroundNormal"),
        if muted {
            color(ini, section, "ForegroundInactive")
        } else {
            None
        },
    )
}

/// The font `ini` states at `section`/`key`, as Qt reads its `QFont` string
/// ([`qfont_from_string`], [`font_of`]).
fn font(ini: &Ini, section: &str, key: &str) -> Option<OsFont> {
    font_of(&qfont_from_string(&ini.get(section, key)?)?)
}

/// The font of the fields `QFont::fromString` set, `None` where it states
/// none: the family where it is not empty; the size, a pixel size above 0
/// as `FontSize::Px`, else a point size that is finite and above 0 as
/// `FontSize::Pt`; the weight bounded to CSS 100..=900, as 0.7.2's
/// `kde::fonts::parse_qt_font_with_weight` bounds Qt's; the style
/// (`QFont::Style`: 0 normal, 1 italic, 2 oblique; 3, no member, `None`).
fn font_of(fields: &QtFontFields) -> Option<OsFont> {
    let size = fields
        .pixel_size
        .filter(|px| *px > 0)
        .map(|px| FontSize::Px(px as f32))
        .or_else(|| {
            fields
                .point_size
                .map(|pt| pt as f32)
                .filter(|pt| pt.is_finite() && *pt > 0.0)
                .map(FontSize::Pt)
        });
    stated(OsFont {
        family: fields
            .family
            .as_deref()
            .filter(|family| !family.is_empty())
            .map(Arc::from),
        size,
        weight: fields
            .weight
            .and_then(|weight| u16::try_from(weight.clamp(100, 900)).ok()),
        style: fields.style.and_then(|style| match style {
            0 => Some(FontStyle::Normal),
            1 => Some(FontStyle::Italic),
            2 => Some(FontStyle::Oblique),
            _ => None,
        }),
        width: None,
    })
}

#[cfg(test)]
mod tests;
