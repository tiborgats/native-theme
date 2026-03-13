//! ThemeColors -> gpui_component::theme::ThemeColor mapping (108 fields).
//!
//! Maps native-theme's 36 semantic color fields to gpui-component's 108-field
//! ThemeColor struct. Direct mappings cover ~30 fields; the remaining ~78 are
//! derived via shade generation, blending, or fallback logic that mirrors
//! gpui-component's own `apply_config` derivation.

use gpui::Hsla;
use gpui_component::{Colorize, theme::ThemeColor};
use native_theme::ThemeVariant;

use crate::derive::{active_color, hover_color};

/// Convert an `Option<native_theme::Rgba>` to `gpui::Hsla`, falling back to
/// `fallback` when `None`.
fn rgba_to_hsla(rgba: Option<native_theme::Rgba>, fallback: Hsla) -> Hsla {
    match rgba {
        Some(c) => {
            let [r, g, b, a] = c.to_f32_array();
            let gpui_rgba = gpui::Rgba { r, g, b, a };
            gpui_rgba.into()
        }
        None => fallback,
    }
}

/// Returns true if the background color indicates a dark theme (lightness < 0.5).
fn is_dark_background(bg: Hsla) -> bool {
    bg.l < 0.5
}

// -- Defaults (white and black) --

fn white() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 1.0,
    }
}

fn black() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    }
}

/// Default accent blue: roughly #0078d7 in HSL.
fn default_accent() -> Hsla {
    gpui::Rgba {
        r: 0.0,
        g: 0.471,
        b: 0.843,
        a: 1.0,
    }
    .into()
}

/// Default red for danger/error states.
fn default_red() -> Hsla {
    Hsla {
        h: 0.0,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    }
}

/// Default green for success states.
fn default_green() -> Hsla {
    Hsla {
        h: 0.333,
        s: 1.0,
        l: 0.25,
        a: 1.0,
    }
}

/// Default yellow for warning states.
fn default_yellow() -> Hsla {
    Hsla {
        h: 0.167,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    }
}

/// Default cyan for info states.
fn default_cyan() -> Hsla {
    Hsla {
        h: 0.5,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    }
}

/// Default blue for ring/chart.
fn default_blue() -> Hsla {
    Hsla {
        h: 0.667,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    }
}

/// Default magenta.
fn default_magenta() -> Hsla {
    Hsla {
        h: 0.833,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    }
}

/// Build a complete [`ThemeColor`] from a [`ThemeVariant`].
///
/// Maps all 108 fields: ~30 directly from ThemeColors, the rest derived
/// via shade generation following gpui-component's own fallback logic.
pub fn to_theme_color(variant: &ThemeVariant) -> ThemeColor {
    let colors = &variant.colors;

    // -- Resolve core colors with defaults --
    let bg = rgba_to_hsla(colors.background, white());
    let fg = rgba_to_hsla(colors.foreground, black());
    let is_dark = is_dark_background(bg);
    let accent = rgba_to_hsla(colors.accent, default_accent());
    let surface = rgba_to_hsla(
        colors.surface,
        if is_dark {
            bg.lighten(0.1)
        } else {
            bg.darken(0.05)
        },
    );
    let border = rgba_to_hsla(
        colors.border,
        if is_dark {
            fg.opacity(0.2)
        } else {
            fg.opacity(0.15)
        },
    );
    let muted = rgba_to_hsla(
        colors.muted,
        if is_dark {
            bg.lighten(0.15)
        } else {
            bg.darken(0.06)
        },
    );

    // -- Resolve semantic colors --
    let primary = rgba_to_hsla(colors.primary_background, accent);
    let primary_fg = rgba_to_hsla(
        colors.primary_foreground,
        if is_dark { black() } else { white() },
    );
    let secondary = rgba_to_hsla(colors.secondary_background, muted);
    let secondary_fg = rgba_to_hsla(colors.secondary_foreground, fg);
    let danger = rgba_to_hsla(colors.danger, default_red());
    let danger_fg = rgba_to_hsla(colors.danger_foreground, primary_fg);
    let success = rgba_to_hsla(colors.success, default_green());
    let success_fg = rgba_to_hsla(colors.success_foreground, primary_fg);
    let warning = rgba_to_hsla(colors.warning, default_yellow());
    let warning_fg = rgba_to_hsla(colors.warning_foreground, primary_fg);
    let info = rgba_to_hsla(colors.info, default_cyan());
    let info_fg = rgba_to_hsla(colors.info_foreground, primary_fg);
    let selection = rgba_to_hsla(colors.selection, primary);
    let link = rgba_to_hsla(colors.link, primary);
    let ring = rgba_to_hsla(colors.focus_ring, default_blue());
    let input = rgba_to_hsla(colors.input, border);

    let sidebar = rgba_to_hsla(colors.sidebar, bg);
    let sidebar_fg = rgba_to_hsla(colors.sidebar_foreground, fg);
    let popover = rgba_to_hsla(colors.popover, bg);
    let popover_fg = rgba_to_hsla(colors.popover_foreground, fg);
    let muted_fg = rgba_to_hsla(colors.muted, muted).blend(fg.opacity(0.7));
    let alternate_row = rgba_to_hsla(colors.alternate_row, bg);

    // -- Build ThemeColor from defaults and override everything --
    let mut tc = ThemeColor::default();

    // Core
    assign_core(
        &mut tc, bg, fg, accent, border, muted, muted_fg, input, ring, selection, link,
    );

    // Primary/Secondary
    assign_primary(&mut tc, primary, primary_fg, bg, is_dark);
    assign_secondary(&mut tc, secondary, secondary_fg, bg, is_dark);

    // Status colors (danger, success, warning, info)
    assign_status(
        &mut tc, danger, danger_fg, success, success_fg, warning, warning_fg, info, info_fg, bg,
        is_dark, primary_fg,
    );

    // List/table
    assign_list_table(
        &mut tc,
        bg,
        alternate_row,
        primary,
        secondary,
        border,
        muted_fg,
    );

    // Tab/sidebar
    assign_tab_sidebar(
        &mut tc, bg, fg, surface, secondary, sidebar, sidebar_fg, accent, primary, primary_fg,
        border,
    );

    // Chart colors
    assign_charts(&mut tc, accent);

    // Misc (overlay, scrollbar, slider, switch, etc.)
    assign_misc(
        &mut tc, bg, fg, accent, muted, primary, primary_fg, secondary, border, surface, is_dark,
        link, popover, popover_fg,
    );

    // Base named colors
    assign_base_colors(&mut tc, danger, success, info, warning, bg);

    tc
}

// ---------- helper assignment functions ----------

#[allow(clippy::too_many_arguments)]
fn assign_core(
    tc: &mut ThemeColor,
    bg: Hsla,
    fg: Hsla,
    accent: Hsla,
    border: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    input: Hsla,
    ring: Hsla,
    selection: Hsla,
    link: Hsla,
) {
    tc.background = bg;
    tc.foreground = fg;
    tc.accent = accent;
    tc.accent_foreground = fg;
    tc.border = border;
    tc.muted = muted;
    tc.muted_foreground = muted_fg;
    tc.input = input;
    tc.ring = ring;
    tc.selection = selection.alpha(selection.a.min(0.3));
    tc.link = link;
    tc.link_hover = link;
    tc.link_active = link;
    tc.caret = fg;
}

fn assign_primary(tc: &mut ThemeColor, primary: Hsla, primary_fg: Hsla, bg: Hsla, is_dark: bool) {
    tc.primary = primary;
    tc.primary_foreground = primary_fg;
    tc.primary_hover = hover_color(primary, bg);
    tc.primary_active = active_color(primary, is_dark);
}

fn assign_secondary(
    tc: &mut ThemeColor,
    secondary: Hsla,
    secondary_fg: Hsla,
    bg: Hsla,
    is_dark: bool,
) {
    tc.secondary = secondary;
    tc.secondary_foreground = secondary_fg;
    tc.secondary_hover = hover_color(secondary, bg);
    tc.secondary_active = active_color(secondary, is_dark);
}

#[allow(clippy::too_many_arguments)]
fn assign_status(
    tc: &mut ThemeColor,
    danger: Hsla,
    danger_fg: Hsla,
    success: Hsla,
    success_fg: Hsla,
    warning: Hsla,
    warning_fg: Hsla,
    info: Hsla,
    info_fg: Hsla,
    bg: Hsla,
    is_dark: bool,
    _primary_fg: Hsla,
) {
    tc.danger = danger;
    tc.danger_foreground = danger_fg;
    tc.danger_hover = hover_color(danger, bg);
    tc.danger_active = active_color(danger, is_dark);

    tc.success = success;
    tc.success_foreground = success_fg;
    tc.success_hover = hover_color(success, bg);
    tc.success_active = active_color(success, is_dark);

    tc.warning = warning;
    tc.warning_foreground = warning_fg;
    tc.warning_hover = hover_color(warning, bg);
    tc.warning_active = active_color(warning, is_dark);

    tc.info = info;
    tc.info_foreground = info_fg;
    tc.info_hover = hover_color(info, bg);
    tc.info_active = active_color(info, is_dark);

    tc.bullish = success;
    tc.bearish = danger;
}

fn assign_list_table(
    tc: &mut ThemeColor,
    bg: Hsla,
    alternate_row: Hsla,
    primary: Hsla,
    secondary: Hsla,
    border: Hsla,
    muted_fg: Hsla,
) {
    tc.list = bg;
    tc.list_hover = hover_color(secondary, bg);
    tc.list_active = bg.blend(primary.opacity(0.1)).alpha(0.2);
    tc.list_active_border = bg.blend(primary.opacity(0.6));
    tc.list_even = alternate_row;
    tc.list_head = bg;

    tc.table = bg;
    tc.table_hover = tc.list_hover;
    tc.table_active = tc.list_active;
    tc.table_active_border = tc.list_active_border;
    tc.table_even = tc.list_even;
    tc.table_head = bg;
    tc.table_head_foreground = muted_fg;
    tc.table_row_border = border;
}

#[allow(clippy::too_many_arguments)]
fn assign_tab_sidebar(
    tc: &mut ThemeColor,
    bg: Hsla,
    fg: Hsla,
    surface: Hsla,
    secondary: Hsla,
    sidebar: Hsla,
    sidebar_fg: Hsla,
    accent: Hsla,
    primary: Hsla,
    primary_fg: Hsla,
    border: Hsla,
) {
    tc.tab = bg;
    tc.tab_active = bg;
    tc.tab_active_foreground = fg;
    tc.tab_bar = bg;
    tc.tab_bar_segmented = secondary;
    tc.tab_foreground = fg;

    tc.sidebar = sidebar;
    tc.sidebar_foreground = sidebar_fg;
    tc.sidebar_accent = accent;
    tc.sidebar_accent_foreground = fg;
    tc.sidebar_border = border;
    tc.sidebar_primary = primary;
    tc.sidebar_primary_foreground = primary_fg;

    tc.title_bar = surface;
    tc.title_bar_border = border;
    tc.window_border = border;
}

fn assign_charts(tc: &mut ThemeColor, accent: Hsla) {
    // Distribute chart colors around the hue wheel from the accent
    tc.chart_1 = accent.lighten(0.4);
    tc.chart_2 = accent.lighten(0.2);
    tc.chart_3 = accent;
    tc.chart_4 = accent.darken(0.2);
    tc.chart_5 = accent.darken(0.4);
}

#[allow(clippy::too_many_arguments)]
fn assign_misc(
    tc: &mut ThemeColor,
    bg: Hsla,
    fg: Hsla,
    accent: Hsla,
    _muted: Hsla,
    primary: Hsla,
    primary_fg: Hsla,
    secondary: Hsla,
    border: Hsla,
    _surface: Hsla,
    is_dark: bool,
    link: Hsla,
    popover: Hsla,
    popover_fg: Hsla,
) {
    tc.popover = popover;
    tc.popover_foreground = popover_fg;

    tc.accordion = bg;
    tc.accordion_hover = accent.opacity(0.8);

    tc.group_box = bg.blend(secondary.opacity(if is_dark { 0.3 } else { 0.4 }));
    tc.group_box_foreground = fg;

    tc.description_list_label = bg.blend(border.opacity(0.2));
    tc.description_list_label_foreground = tc.muted_foreground;

    tc.overlay = if is_dark {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.5,
        }
    } else {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.4,
        }
    };

    tc.scrollbar = bg;
    tc.scrollbar_thumb = accent;
    tc.scrollbar_thumb_hover = accent;

    tc.slider_bar = primary;
    tc.slider_thumb = primary_fg;

    tc.switch = secondary;
    tc.switch_thumb = bg;

    tc.progress_bar = primary;

    tc.skeleton = secondary;

    tc.tiles = bg;

    tc.drag_border = primary.opacity(0.65);
    tc.drop_target = primary.opacity(0.2);

    tc.link_hover = link;
    tc.link_active = link;
}

fn assign_base_colors(
    tc: &mut ThemeColor,
    danger: Hsla,
    success: Hsla,
    info: Hsla,
    warning: Hsla,
    bg: Hsla,
) {
    tc.red = danger;
    tc.red_light = bg.blend(danger.opacity(0.8));
    tc.green = success;
    tc.green_light = bg.blend(success.opacity(0.8));
    tc.blue = info;
    tc.blue_light = bg.blend(info.opacity(0.8));
    tc.yellow = warning;
    tc.yellow_light = bg.blend(warning.opacity(0.8));
    tc.magenta = default_magenta();
    tc.magenta_light = bg.blend(default_magenta().opacity(0.8));
    tc.cyan = info;
    tc.cyan_light = bg.blend(info.opacity(0.8));
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::Rgba;

    /// Create a ThemeVariant with fully populated colors for testing.
    fn populated_variant() -> ThemeVariant {
        let mut v = ThemeVariant::default();
        v.colors.background = Some(Rgba::rgb(255, 255, 255));
        v.colors.foreground = Some(Rgba::rgb(0, 0, 0));
        v.colors.accent = Some(Rgba::rgb(0, 120, 215));
        v.colors.surface = Some(Rgba::rgb(240, 240, 240));
        v.colors.border = Some(Rgba::rgb(200, 200, 200));
        v.colors.muted = Some(Rgba::rgb(150, 150, 150));
        v.colors.primary_background = Some(Rgba::rgb(0, 120, 215));
        v.colors.primary_foreground = Some(Rgba::rgb(255, 255, 255));
        v.colors.secondary_background = Some(Rgba::rgb(230, 230, 230));
        v.colors.secondary_foreground = Some(Rgba::rgb(50, 50, 50));
        v.colors.danger = Some(Rgba::rgb(209, 52, 56));
        v.colors.danger_foreground = Some(Rgba::rgb(255, 255, 255));
        v.colors.warning = Some(Rgba::rgb(255, 140, 0));
        v.colors.warning_foreground = Some(Rgba::rgb(0, 0, 0));
        v.colors.success = Some(Rgba::rgb(16, 124, 16));
        v.colors.success_foreground = Some(Rgba::rgb(255, 255, 255));
        v.colors.info = Some(Rgba::rgb(0, 120, 212));
        v.colors.info_foreground = Some(Rgba::rgb(255, 255, 255));
        v.colors.selection = Some(Rgba::rgb(0, 120, 215));
        v.colors.link = Some(Rgba::rgb(0, 100, 200));
        v.colors.focus_ring = Some(Rgba::rgb(0, 90, 200));
        v.colors.input = Some(Rgba::rgb(220, 220, 220));
        v.colors.sidebar = Some(Rgba::rgb(245, 245, 245));
        v.colors.sidebar_foreground = Some(Rgba::rgb(30, 30, 30));
        v.colors.popover = Some(Rgba::rgb(255, 255, 255));
        v.colors.popover_foreground = Some(Rgba::rgb(0, 0, 0));
        v.colors.alternate_row = Some(Rgba::rgb(248, 248, 248));
        v
    }

    #[test]
    fn rgba_to_hsla_converts_red() {
        let red = Some(Rgba::rgb(255, 0, 0));
        let result = rgba_to_hsla(red, white());
        // Red should have hue ~0 (or near 0/1), high saturation
        assert!(
            result.h < 0.05 || result.h > 0.95,
            "red hue={} should be near 0",
            result.h
        );
        assert!(result.s > 0.9, "red saturation={} should be high", result.s);
    }

    #[test]
    fn rgba_to_hsla_converts_green() {
        let green = Some(Rgba::rgb(0, 255, 0));
        let result = rgba_to_hsla(green, white());
        // Green hue ~0.333
        assert!(
            (result.h - 0.333).abs() < 0.05,
            "green hue={} should be near 0.333",
            result.h
        );
    }

    #[test]
    fn rgba_to_hsla_converts_blue() {
        let blue = Some(Rgba::rgb(0, 0, 255));
        let result = rgba_to_hsla(blue, white());
        // Blue hue ~0.667
        assert!(
            (result.h - 0.667).abs() < 0.05,
            "blue hue={} should be near 0.667",
            result.h
        );
    }

    #[test]
    fn rgba_to_hsla_none_returns_fallback() {
        let fallback = Hsla {
            h: 0.5,
            s: 0.5,
            l: 0.5,
            a: 1.0,
        };
        let result = rgba_to_hsla(None, fallback);
        assert_eq!(result, fallback);
    }

    #[test]
    fn to_theme_color_populated_produces_nondefault() {
        let variant = populated_variant();
        let tc = to_theme_color(&variant);
        let default = ThemeColor::default();

        // Direct-mapped fields should differ from default
        assert_ne!(
            tc.background, default.background,
            "background should be set"
        );
        assert_ne!(
            tc.foreground, default.foreground,
            "foreground should be set"
        );
        assert_ne!(tc.primary, default.primary, "primary should be set");
        assert_ne!(tc.danger, default.danger, "danger should be set");
        assert_ne!(tc.border, default.border, "border should be set");
    }

    #[test]
    fn to_theme_color_empty_produces_reasonable_defaults() {
        let variant = ThemeVariant::default();
        let tc = to_theme_color(&variant);

        // Background should be white (default), lightness = 1.0
        assert!(
            (tc.background.l - 1.0).abs() < 0.01,
            "default bg l={} should be ~1.0",
            tc.background.l
        );
        // Foreground should be black, lightness = 0.0
        assert!(
            tc.foreground.l < 0.01,
            "default fg l={} should be ~0.0",
            tc.foreground.l
        );
        // Should have some accent color (not zero lightness black)
        assert!(
            tc.accent.s > 0.0 || tc.accent.l > 0.0,
            "accent should not be all-zero"
        );
    }

    #[test]
    fn is_dark_detects_dark_background() {
        assert!(is_dark_background(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.1,
            a: 1.0
        }));
        assert!(!is_dark_background(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0
        }));
    }

    #[test]
    fn hover_states_differ_from_base() {
        let variant = populated_variant();
        let tc = to_theme_color(&variant);

        assert_ne!(
            tc.primary_hover, tc.primary,
            "primary_hover should differ from primary"
        );
        assert_ne!(
            tc.danger_hover, tc.danger,
            "danger_hover should differ from danger"
        );
    }
}
