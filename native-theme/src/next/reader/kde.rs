//! The KDE part of [`SystemOverrides`] (spec §8, "The KDE part"): the
//! settings of a parsed `kdeglobals`, each as read, parsed by the 0.7.2
//! reader's own functions (`crate::kde`). Nothing here reads a file or
//! D-Bus, and nothing is derived: what the 0.7.2 reader derives from these
//! settings the `kde-breeze` builder derives.

use std::sync::Arc;

use configparser::ini::Ini;

use super::{
    KdeColorEffects, OsColors, OsFont, OsSelectionColors, OsSurfaceOverrides, SystemOverrides,
};
use crate::kde::colors::{get_color, kconfig_bool};
use crate::kde::fonts::parse_qt_font_with_weight;
use crate::kde::{caret_blink, icon_theme, is_dark_theme};
use crate::next::color::Color;
use crate::next::icons::IconSizes;
use crate::next::query::{Contrast, Scheme};
use crate::next::roles::SurfaceColors;
use crate::next::style::{FontSize, FontStyle};

/// The settings `ini` states, with the icon theme's default sizes
/// `icon_sizes` (read from its `index.theme` by the caller) and the XDG
/// portal's accent `portal_accent`, each where given.
///
/// Every slot is `Some` only where the file states its setting in a form its
/// parser accepts, except the two the 0.7.2 reader always sets: `scheme`
/// (dark where `[Colors:Window] BackgroundNormal` is dark, else light) and
/// `font_dpi` (96). The colours, the two contrast scalars and the colour
/// effects form the entry of `color_schemes` for `(scheme, Contrast::Normal)`,
/// which is left out where the file states none of them. The motion
/// preference that `[KDE] AnimationDurationFactor = 0` means is the
/// snapshot's context, which the caller sets.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "called by the KDE reader's `read()` (plan, M2) and the kde-breeze builder's tests"
    )
)]
pub(crate) fn overrides_from_kdeglobals(
    ini: &Ini,
    icon_sizes: Option<crate::model::IconSizes>,
    portal_accent: Option<crate::Rgba>,
) -> SystemOverrides {
    let scheme = if is_dark_theme(ini) {
        Scheme::Dark
    } else {
        Scheme::Light
    };
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
        theme_name: ini.get("General", "ColorScheme").map(Arc::from),
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
        // The value as the 0.7.2 reader parses it (`kde::reduce_motion`), a
        // non-finite one refused.
        animation_scale: ini
            .get("KDE", "AnimationDurationFactor")
            .and_then(|value| value.trim().parse::<f32>().ok())
            .filter(|value| value.is_finite()),
        caret_blink_on_ms: caret.caret_blink_on,
        caret_blink_off_ms: caret.caret_blink_off,
        ..SystemOverrides::default()
    }
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
        kde_button: set(ini, "Colors:Button", false),
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

/// The font `ini` states at `section`/`key`, as its `QFont` string gives it.
fn font(ini: &Ini, section: &str, key: &str) -> Option<OsFont> {
    let spec = parse_qt_font_with_weight(&ini.get(section, key)?)?;
    Some(OsFont {
        family: spec.family,
        size: spec.size.map(|size| match size {
            crate::model::font::FontSize::Pt(v) => FontSize::Pt(v),
            crate::model::font::FontSize::Px(v) => FontSize::Px(v),
        }),
        weight: spec.weight,
        style: spec.style.map(|style| match style {
            crate::model::font::FontStyle::Normal => FontStyle::Normal,
            crate::model::font::FontStyle::Italic => FontStyle::Italic,
            crate::model::font::FontStyle::Oblique => FontStyle::Oblique,
        }),
        width: None,
    })
}

#[cfg(test)]
mod tests;
