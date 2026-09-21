//! Layer 1 of the theme contracts: every `ThemeColor` field this connector
//! writes, the native value it must equal or the derivation that produced it,
//! and the contrast invariant of spec section 7.
//!
//! Test-only. The rows are asserted over the sixteen presets in both modes --
//! the four `*-live.toml` files are geometry-only merge bases and are not in
//! `Theme::list_presets()`. A row that must differ on some preset carries the
//! reason in `exceptions`, and an exception that no longer describes a real
//! difference fails just as loudly as a broken row.
//!
//! The coverage tripwire (section 5.2) closes the table: every one of the 138
//! `ThemeColor` fields -- enumerated through serde, the same mechanism
//! `colors::tests::no_theme_color_field_is_left_at_default` uses, so a field
//! added upstream appears here without anyone listing it -- must appear in
//! exactly one of the rows and `DERIVED`.

use gpui::Hsla;
use gpui_component::theme::ThemeColor;

use crate::colors::{rgba_to_hsla, to_theme_color};
use crate::derive::{active_color, contrast_ratio};
use crate::{ColorMode, ResolvedTheme, Rgba};

/// One row of the mapping contract: a toolkit slot, the native field it must
/// equal, and the presets where it may legitimately differ.
struct Row {
    slot: &'static str,
    native: fn(&ResolvedTheme) -> Rgba,
    get: fn(&ThemeColor) -> Hsla,
    /// Preset keys where this row does not hold, each with its reason.
    exceptions: &'static [(&'static str, &'static str)],
}

/// One row whose expected value is more than a single native field.
///
/// Two kinds need it, and both are still native values rather than house
/// style: a state layer composited over the fill it is drawn on (C17), and a
/// soft option whose `None` the connector answers with a named derivation.
/// `source` says which, for the failure message; `is_dark` is the mode, which
/// a derivation needs and a native field does not.
struct ComputedRow {
    slot: &'static str,
    source: &'static str,
    native: fn(&ResolvedTheme, bool) -> Hsla,
    get: fn(&ThemeColor) -> Hsla,
    exceptions: &'static [(&'static str, &'static str)],
}

/// Every `ThemeColor` field that equals one named native field.
///
/// The fields whose value is composited or derived are in `COMPUTED_ROWS` and
/// `DERIVED`; the three lists partition the 138 fields, which
/// `every_theme_color_field_has_a_declared_source` asserts.
const ROWS: &[Row] = &[
    Row {
        slot: "background",
        native: |r| r.defaults.background_color,
        get: |tc| tc.background,
        exceptions: &[],
    },
    Row {
        slot: "foreground",
        native: |r| r.defaults.text_color,
        get: |tc| tc.foreground,
        exceptions: &[],
    },
    // Upstream's `accent` is the item highlight, not the platform's accent
    // colour (`theme/schema.rs:254-255`), so it takes the menu hover pair.
    Row {
        slot: "accent",
        native: |r| r.menu.hover_background,
        get: |tc| tc.accent,
        exceptions: &[],
    },
    Row {
        slot: "accent_foreground",
        native: |r| r.menu.hover_text_color,
        get: |tc| tc.accent_foreground,
        exceptions: &[],
    },
    Row {
        slot: "border",
        native: |r| r.defaults.border.color,
        get: |tc| tc.border,
        exceptions: &[],
    },
    Row {
        slot: "muted_foreground",
        native: |r| r.defaults.muted_color,
        get: |tc| tc.muted_foreground,
        exceptions: &[],
    },
    Row {
        slot: "input",
        native: |r| r.input.border.color,
        get: |tc| tc.input,
        exceptions: &[],
    },
    Row {
        slot: "ring",
        native: |r| r.defaults.focus_ring_color,
        get: |tc| tc.ring,
        exceptions: &[],
    },
    Row {
        slot: "selection",
        native: |r| r.defaults.selection_background,
        get: |tc| tc.selection,
        exceptions: &[],
    },
    Row {
        slot: "link",
        native: |r| r.defaults.link_color,
        get: |tc| tc.link,
        exceptions: &[],
    },
    Row {
        slot: "link_hover",
        native: |r| r.link.hover_background,
        get: |tc| tc.link_hover,
        exceptions: &[],
    },
    Row {
        slot: "primary",
        native: |r| r.button.primary_background,
        get: |tc| tc.primary,
        exceptions: &[],
    },
    Row {
        slot: "primary_foreground",
        native: |r| r.button.primary_text_color,
        get: |tc| tc.primary_foreground,
        exceptions: &[],
    },
    Row {
        slot: "secondary",
        native: |r| r.button.background_color,
        get: |tc| tc.secondary,
        exceptions: &[],
    },
    Row {
        slot: "secondary_foreground",
        native: |r| r.button.font.color,
        get: |tc| tc.secondary_foreground,
        exceptions: &[],
    },
    // Raw, not composited: the readers of `secondary_hover` are
    // transparent-idle -- title bar, stepper trigger, tab, calendar and
    // `variants::ghost_button` -- where the platform's layer over the backdrop
    // is exactly right (C17).
    Row {
        slot: "secondary_hover",
        native: |r| r.button.hover_background,
        get: |tc| tc.secondary_hover,
        exceptions: &[],
    },
    Row {
        slot: "danger",
        native: |r| r.defaults.danger_color,
        get: |tc| tc.danger,
        exceptions: &[],
    },
    // The status labels are the platform's own, never a colour of ours: a
    // connector that replaced a sub-AA label would be overriding the
    // platform's choice, and on the real platforms the replacement was the
    // colour the platform already had (C19).
    Row {
        slot: "danger_foreground",
        native: |r| r.defaults.danger_text_color,
        get: |tc| tc.danger_foreground,
        exceptions: &[],
    },
    Row {
        slot: "success",
        native: |r| r.defaults.success_color,
        get: |tc| tc.success,
        exceptions: &[],
    },
    Row {
        slot: "success_foreground",
        native: |r| r.defaults.success_text_color,
        get: |tc| tc.success_foreground,
        exceptions: &[],
    },
    Row {
        slot: "warning",
        native: |r| r.defaults.warning_color,
        get: |tc| tc.warning,
        exceptions: &[],
    },
    Row {
        slot: "warning_foreground",
        native: |r| r.defaults.warning_text_color,
        get: |tc| tc.warning_foreground,
        exceptions: &[],
    },
    Row {
        slot: "info",
        native: |r| r.defaults.info_color,
        get: |tc| tc.info,
        exceptions: &[],
    },
    Row {
        slot: "info_foreground",
        native: |r| r.defaults.info_text_color,
        get: |tc| tc.info_foreground,
        exceptions: &[],
    },
    Row {
        slot: "chart_bullish",
        native: |r| r.defaults.success_color,
        get: |tc| tc.chart_bullish,
        exceptions: &[],
    },
    Row {
        slot: "chart_bearish",
        native: |r| r.defaults.danger_color,
        get: |tc| tc.chart_bearish,
        exceptions: &[],
    },
    // The 28 `button_*` fields take the values the semantic fields their
    // variant used in 0.5.1, so the ones that are a plain copy of a native
    // field are rows of their own rather than a claim about the copy.
    Row {
        slot: "button",
        native: |r| r.button.background_color,
        get: |tc| tc.button,
        exceptions: &[],
    },
    Row {
        slot: "button_foreground",
        native: |r| r.button.font.color,
        get: |tc| tc.button_foreground,
        exceptions: &[],
    },
    Row {
        slot: "button_secondary",
        native: |r| r.button.background_color,
        get: |tc| tc.button_secondary,
        exceptions: &[],
    },
    Row {
        slot: "button_secondary_foreground",
        native: |r| r.button.font.color,
        get: |tc| tc.button_secondary_foreground,
        exceptions: &[],
    },
    Row {
        slot: "button_primary",
        native: |r| r.button.primary_background,
        get: |tc| tc.button_primary,
        exceptions: &[],
    },
    Row {
        slot: "button_primary_foreground",
        native: |r| r.button.primary_text_color,
        get: |tc| tc.button_primary_foreground,
        exceptions: &[],
    },
    Row {
        slot: "button_danger",
        native: |r| r.defaults.danger_color,
        get: |tc| tc.button_danger,
        exceptions: &[],
    },
    Row {
        slot: "button_danger_foreground",
        native: |r| r.defaults.danger_text_color,
        get: |tc| tc.button_danger_foreground,
        exceptions: &[],
    },
    Row {
        slot: "button_info",
        native: |r| r.defaults.info_color,
        get: |tc| tc.button_info,
        exceptions: &[],
    },
    Row {
        slot: "button_info_foreground",
        native: |r| r.defaults.info_text_color,
        get: |tc| tc.button_info_foreground,
        exceptions: &[],
    },
    Row {
        slot: "button_success",
        native: |r| r.defaults.success_color,
        get: |tc| tc.button_success,
        exceptions: &[],
    },
    Row {
        slot: "button_success_foreground",
        native: |r| r.defaults.success_text_color,
        get: |tc| tc.button_success_foreground,
        exceptions: &[],
    },
    Row {
        slot: "button_warning",
        native: |r| r.defaults.warning_color,
        get: |tc| tc.button_warning,
        exceptions: &[],
    },
    Row {
        slot: "button_warning_foreground",
        native: |r| r.defaults.warning_text_color,
        get: |tc| tc.button_warning_foreground,
        exceptions: &[],
    },
    Row {
        slot: "list",
        native: |r| r.defaults.background_color,
        get: |tc| tc.list,
        exceptions: &[],
    },
    Row {
        slot: "list_hover",
        native: |r| r.list.hover_background,
        get: |tc| tc.list_hover,
        exceptions: &[],
    },
    Row {
        slot: "list_active",
        native: |r| r.list.selection_background,
        get: |tc| tc.list_active,
        exceptions: &[],
    },
    Row {
        slot: "list_even",
        native: |r| r.list.alternate_row_background,
        get: |tc| tc.list_even,
        exceptions: &[],
    },
    Row {
        slot: "list_head",
        native: |r| r.defaults.background_color,
        get: |tc| tc.list_head,
        exceptions: &[],
    },
    Row {
        slot: "table",
        native: |r| r.defaults.background_color,
        get: |tc| tc.table,
        exceptions: &[],
    },
    Row {
        slot: "table_hover",
        native: |r| r.list.hover_background,
        get: |tc| tc.table_hover,
        exceptions: &[],
    },
    Row {
        slot: "table_active",
        native: |r| r.list.selection_background,
        get: |tc| tc.table_active,
        exceptions: &[],
    },
    Row {
        slot: "table_even",
        native: |r| r.list.alternate_row_background,
        get: |tc| tc.table_even,
        exceptions: &[],
    },
    Row {
        slot: "table_head",
        native: |r| r.defaults.background_color,
        get: |tc| tc.table_head,
        exceptions: &[],
    },
    Row {
        slot: "table_head_foreground",
        native: |r| r.defaults.muted_color,
        get: |tc| tc.table_head_foreground,
        exceptions: &[],
    },
    Row {
        slot: "table_row_border",
        native: |r| r.defaults.border.color,
        get: |tc| tc.table_row_border,
        exceptions: &[],
    },
    // The footer mirrors the header (`ListTheme` has no footer field), so it
    // lands on the same two native values rather than on a derivation.
    Row {
        slot: "table_foot",
        native: |r| r.defaults.background_color,
        get: |tc| tc.table_foot,
        exceptions: &[],
    },
    Row {
        slot: "table_foot_foreground",
        native: |r| r.defaults.muted_color,
        get: |tc| tc.table_foot_foreground,
        exceptions: &[],
    },
    Row {
        slot: "tab",
        native: |r| r.tab.background_color,
        get: |tc| tc.tab,
        exceptions: &[],
    },
    Row {
        slot: "tab_active",
        native: |r| r.tab.active_background,
        get: |tc| tc.tab_active,
        exceptions: &[],
    },
    Row {
        slot: "tab_active_foreground",
        native: |r| r.tab.active_text_color,
        get: |tc| tc.tab_active_foreground,
        exceptions: &[],
    },
    Row {
        slot: "tab_bar",
        native: |r| r.tab.bar_background,
        get: |tc| tc.tab_bar,
        exceptions: &[],
    },
    // `ResolvedTabTheme` has no segmented-specific colour; the ordinary button
    // fill is the segmented indicator's nearest native source (Issue 42).
    Row {
        slot: "tab_bar_segmented",
        native: |r| r.button.background_color,
        get: |tc| tc.tab_bar_segmented,
        exceptions: &[],
    },
    Row {
        slot: "tab_foreground",
        native: |r| r.tab.font.color,
        get: |tc| tc.tab_foreground,
        exceptions: &[],
    },
    Row {
        slot: "sidebar",
        native: |r| r.sidebar.background_color,
        get: |tc| tc.sidebar,
        exceptions: &[],
    },
    Row {
        slot: "sidebar_foreground",
        native: |r| r.sidebar.font.color,
        get: |tc| tc.sidebar_foreground,
        exceptions: &[],
    },
    Row {
        slot: "sidebar_accent",
        native: |r| r.sidebar.selection_background,
        get: |tc| tc.sidebar_accent,
        exceptions: &[],
    },
    Row {
        slot: "sidebar_accent_foreground",
        native: |r| r.sidebar.selection_text_color,
        get: |tc| tc.sidebar_accent_foreground,
        exceptions: &[],
    },
    Row {
        slot: "sidebar_border",
        native: |r| r.window.border.color,
        get: |tc| tc.sidebar_border,
        exceptions: &[],
    },
    Row {
        slot: "sidebar_primary",
        native: |r| r.button.primary_background,
        get: |tc| tc.sidebar_primary,
        exceptions: &[],
    },
    Row {
        slot: "sidebar_primary_foreground",
        native: |r| r.button.primary_text_color,
        get: |tc| tc.sidebar_primary_foreground,
        exceptions: &[],
    },
    Row {
        slot: "title_bar",
        native: |r| r.window.title_bar_background,
        get: |tc| tc.title_bar,
        exceptions: &[],
    },
    Row {
        slot: "title_bar_border",
        native: |r| r.window.border.color,
        get: |tc| tc.title_bar_border,
        exceptions: &[],
    },
    Row {
        slot: "window_border",
        native: |r| r.window.border.color,
        get: |tc| tc.window_border,
        exceptions: &[],
    },
    Row {
        slot: "chart_1",
        native: |r| r.defaults.accent_color,
        get: |tc| tc.chart_1,
        exceptions: &[],
    },
    Row {
        slot: "popover",
        native: |r| r.popover.background_color,
        get: |tc| tc.popover,
        exceptions: &[],
    },
    Row {
        slot: "popover_foreground",
        native: |r| r.popover.font.color,
        get: |tc| tc.popover_foreground,
        exceptions: &[],
    },
    Row {
        slot: "accordion",
        native: |r| r.defaults.background_color,
        get: |tc| tc.accordion,
        exceptions: &[],
    },
    Row {
        slot: "group_box_foreground",
        native: |r| r.defaults.text_color,
        get: |tc| tc.group_box_foreground,
        exceptions: &[],
    },
    Row {
        slot: "description_list_label_foreground",
        native: |r| r.defaults.muted_color,
        get: |tc| tc.description_list_label_foreground,
        exceptions: &[],
    },
    Row {
        slot: "scrollbar",
        native: |r| r.scrollbar.track_color,
        get: |tc| tc.scrollbar,
        exceptions: &[],
    },
    Row {
        slot: "scrollbar_thumb",
        native: |r| r.scrollbar.thumb_color,
        get: |tc| tc.scrollbar_thumb,
        exceptions: &[],
    },
    Row {
        slot: "scrollbar_thumb_hover",
        native: |r| r.scrollbar.thumb_hover_color,
        get: |tc| tc.scrollbar_thumb_hover,
        exceptions: &[],
    },
    Row {
        slot: "slider_bar",
        native: |r| r.slider.fill_color,
        get: |tc| tc.slider_bar,
        exceptions: &[],
    },
    Row {
        slot: "slider_thumb",
        native: |r| r.slider.thumb_color,
        get: |tc| tc.slider_thumb,
        exceptions: &[],
    },
    Row {
        slot: "switch",
        native: |r| r.switch.unchecked_background,
        get: |tc| tc.switch,
        exceptions: &[],
    },
    Row {
        slot: "switch_thumb",
        native: |r| r.switch.thumb_background,
        get: |tc| tc.switch_thumb,
        exceptions: &[],
    },
    Row {
        slot: "progress_bar",
        native: |r| r.progress_bar.fill_color,
        get: |tc| tc.progress_bar,
        exceptions: &[],
    },
    Row {
        slot: "caret",
        native: |r| r.input.caret_color,
        get: |tc| tc.caret,
        exceptions: &[],
    },
    Row {
        slot: "skeleton",
        native: |r| r.button.background_color,
        get: |tc| tc.skeleton,
        exceptions: &[],
    },
    Row {
        slot: "status_bar",
        native: |r| r.status_bar.background_color,
        get: |tc| tc.status_bar,
        exceptions: &[],
    },
    Row {
        slot: "status_bar_border",
        native: |r| r.status_bar.border.color,
        get: |tc| tc.status_bar_border,
        exceptions: &[],
    },
    // The four base-palette colours the connector maps directly
    // (`colors.rs:601-607`); their `_light` siblings are derived.
    Row {
        slot: "red",
        native: |r| r.defaults.danger_color,
        get: |tc| tc.red,
        exceptions: &[],
    },
    Row {
        slot: "green",
        native: |r| r.defaults.success_color,
        get: |tc| tc.green,
        exceptions: &[],
    },
    Row {
        slot: "blue",
        native: |r| r.defaults.info_color,
        get: |tc| tc.blue,
        exceptions: &[],
    },
    Row {
        slot: "yellow",
        native: |r| r.defaults.warning_color,
        get: |tc| tc.yellow,
        exceptions: &[],
    },
];

/// Composite `layer` over `base`: straight-alpha source-over, so a row's
/// expected value is what the screen shows rather than the layer alone.
///
/// Written here rather than taken from `Hsla::blend`, which the connector
/// calls: an expectation computed with the code under test would agree with
/// anything. The two agree bit for bit where `base` is opaque -- the same two
/// products, added in the same order -- and
/// `every_preset_states_an_opaque_button_fill` shows that is every
/// combination, so the translucent-base arm is the general rule rather than a
/// tolerance introduced to make a row pass.
fn over(layer: Hsla, base: Hsla) -> Hsla {
    if layer.a >= 1.0 {
        return layer;
    }
    if layer.a <= 0.0 {
        return base;
    }
    let (l, b) = (gpui::Rgba::from(layer), gpui::Rgba::from(base));
    if b.a >= 1.0 {
        let mix = |l: f32, b: f32| (b * (1.0 - layer.a)) + (l * layer.a);
        return gpui::Rgba {
            r: mix(l.r, b.r),
            g: mix(l.g, b.g),
            b: mix(l.b, b.b),
            a: b.a,
        }
        .into();
    }
    let under = b.a * (1.0 - layer.a);
    let alpha = layer.a + under;
    if alpha <= 0.0 {
        return gpui::Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
        .into();
    }
    let mix = |l: f32, b: f32| ((l * layer.a) + (b * under)) / alpha;
    gpui::Rgba {
        r: mix(l.r, b.r),
        g: mix(l.g, b.g),
        b: mix(l.b, b.b),
        a: alpha,
    }
    .into()
}

/// The idle fill a filled button paints, which its state layers sit on.
fn native_button_fill(r: &ResolvedTheme) -> Hsla {
    rgba_to_hsla(r.button.background_color)
}

/// The pressed fill the platform gives an ordinary button.
///
/// `button.active_background` is a soft option: where the platform states
/// none, the connector darkens (or, near black, lightens) its own idle fill
/// instead. That fallback is the connector's derivation rather than a native
/// value, so it is named here and computed with the same helper
/// `colors::assign_secondary` uses -- there is no second way to state "the
/// connector's existing derivation" without copying it.
fn native_pressed_fill(r: &ResolvedTheme, is_dark: bool) -> Hsla {
    match r.button.active_background {
        Some(active) => rgba_to_hsla(active),
        None => active_color(rgba_to_hsla(r.button.background_color), is_dark),
    }
}

/// The `ThemeColor` fields whose native value is composited or soft-optional.
const COMPUTED_ROWS: &[ComputedRow] = &[
    ComputedRow {
        slot: "secondary_active",
        source: "button.active_background, or the connector's darkening of \
                 button.background_color where the platform states none",
        native: native_pressed_fill,
        get: |tc| tc.secondary_active,
        exceptions: &[],
    },
    // The four filled-button tokens: a `Default` or `Secondary` button paints
    // its own fill and upstream replaces that fill per state, so the
    // platform's hover and pressed layers are composited over it (C17).
    // Windows 11 is the one preset where the layer is translucent and the
    // compositing is therefore visible; everywhere else it is the identity.
    ComputedRow {
        slot: "button_hover",
        source: "button.hover_background over button.background_color",
        native: |r, _| {
            over(
                rgba_to_hsla(r.button.hover_background),
                native_button_fill(r),
            )
        },
        get: |tc| tc.button_hover,
        exceptions: &[],
    },
    ComputedRow {
        slot: "button_secondary_hover",
        source: "button.hover_background over button.background_color",
        native: |r, _| {
            over(
                rgba_to_hsla(r.button.hover_background),
                native_button_fill(r),
            )
        },
        get: |tc| tc.button_secondary_hover,
        exceptions: &[],
    },
    ComputedRow {
        slot: "button_active",
        source: "the pressed fill of `secondary_active` over button.background_color",
        native: |r, is_dark| over(native_pressed_fill(r, is_dark), native_button_fill(r)),
        get: |tc| tc.button_active,
        exceptions: &[],
    },
    ComputedRow {
        slot: "button_secondary_active",
        source: "the pressed fill of `secondary_active` over button.background_color",
        native: |r, is_dark| over(native_pressed_fill(r, is_dark), native_button_fill(r)),
        get: |tc| tc.button_secondary_active,
        exceptions: &[],
    },
];

/// Every `ThemeColor` field the connector derives, with the derivation named.
///
/// A derivation, not a native value: arithmetic on one, a copy of another
/// field that is itself derived, or a hue the platform does not state.
const DERIVED: &[(&str, &str)] = &[
    (
        "muted",
        "the foreground at a tenth alpha over the background",
    ),
    ("link_active", "active_color(link, is_dark)"),
    ("primary_hover", "hover_color(primary, background)"),
    ("primary_active", "active_color(primary, is_dark)"),
    ("danger_hover", "hover_color(danger, background)"),
    ("danger_active", "active_color(danger, is_dark)"),
    ("success_hover", "hover_color(success, background)"),
    ("success_active", "active_color(success, is_dark)"),
    ("warning_hover", "hover_color(warning, background)"),
    ("warning_active", "active_color(warning, is_dark)"),
    ("info_hover", "hover_color(info, background)"),
    ("info_active", "active_color(info, is_dark)"),
    ("button_primary_hover", "a copy of primary_hover"),
    ("button_primary_active", "a copy of primary_active"),
    ("button_danger_hover", "a copy of danger_hover"),
    ("button_danger_active", "a copy of danger_active"),
    ("button_info_hover", "a copy of info_hover"),
    ("button_info_active", "a copy of info_active"),
    ("button_success_hover", "a copy of success_hover"),
    ("button_success_active", "a copy of success_active"),
    ("button_warning_hover", "a copy of warning_hover"),
    ("button_warning_active", "a copy of warning_active"),
    (
        "list_active_border",
        "the primary colour at 0.6 alpha over the background",
    ),
    ("table_active_border", "a copy of list_active_border"),
    ("chart_2", "accent rotated 0.2 around the hue wheel"),
    ("chart_3", "accent rotated 0.4 around the hue wheel"),
    ("chart_4", "accent rotated 0.6 around the hue wheel"),
    ("chart_5", "accent rotated 0.8 around the hue wheel"),
    (
        "group_box",
        "the secondary fill over the background, at the mode's group-box alpha",
    ),
    (
        "description_list_label",
        "the border colour at a fifth alpha over the background",
    ),
    (
        "overlay",
        "defaults.shadow_color at the mode's overlay alpha",
    ),
    ("drag_border", "primary faded to 0.65"),
    ("drop_target", "primary faded to 0.2"),
    ("red_light", "light_variant(background, danger, is_dark)"),
    ("green_light", "light_variant(background, success, is_dark)"),
    ("blue_light", "light_variant(background, info, is_dark)"),
    (
        "yellow_light",
        "light_variant(background, warning, is_dark)",
    ),
    (
        "magenta",
        "a fixed hue carrying the accent's saturation and lightness",
    ),
    (
        "magenta_light",
        "light_variant(background, magenta, is_dark)",
    ),
    (
        "cyan",
        "a fixed hue carrying the info colour's saturation and lightness",
    ),
    ("cyan_light", "light_variant(background, cyan, is_dark)"),
];

/// One preset in one mode: the native values and the `ThemeColor` built from
/// them.
struct Combination {
    key: &'static str,
    is_dark: bool,
    resolved: ResolvedTheme,
    colors: ThemeColor,
}

impl Combination {
    fn label(&self) -> String {
        let mode = if self.is_dark { "dark" } else { "light" };
        format!("{}/{mode}", self.key)
    }
}

/// Resolve every bundled preset in both modes and map it.
///
/// A preset that fails to load is an error the caller propagates, never a
/// combination quietly left out.
fn combinations() -> crate::Result<Vec<Combination>> {
    let mut out = Vec::new();
    for info in crate::Theme::list_presets() {
        for is_dark in [false, true] {
            let mode = if is_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            };
            let resolved = crate::Theme::preset(info.key)?
                .into_variant(mode)?
                .into_resolved(&native_theme::ResolutionContext::for_tests())?;
            let colors = to_theme_color(&resolved, is_dark, false);
            out.push(Combination {
                key: info.key,
                is_dark,
                resolved,
                colors,
            });
        }
    }
    Ok(out)
}

fn show(c: Hsla) -> String {
    format!("({:.4}, {:.4}, {:.4}, {:.4})", c.h, c.s, c.l, c.a)
}

/// What a row list actually compared, so a declared row cannot assert nothing.
///
/// A row whose `exceptions` happen to name every preset is skipped in every
/// combination and would otherwise leave no trace: the failure count is simply
/// smaller. Each checker records the slots it compared here, and `covers`
/// requires that set to be exactly the declared one.
#[derive(Default)]
struct Checked {
    slots: Vec<&'static str>,
    checks: usize,
}

impl Checked {
    fn ran(&mut self, slot: &'static str) {
        self.slots.push(slot);
        self.checks += 1;
    }

    /// Require that the slots which ran are exactly the ones `declared` names.
    fn covers(&self, declared: &[&'static str], what: &str) {
        let mut ran: Vec<&str> = self.slots.clone();
        ran.sort_unstable();
        ran.dedup();
        let mut declared: Vec<&str> = declared.to_vec();
        declared.sort_unstable();
        let duplicated: Vec<&str> = declared
            .windows(2)
            .filter(|pair| pair[0] == pair[1])
            .map(|pair| pair[0])
            .collect();
        declared.dedup();

        let missing: Vec<&&str> = declared.iter().filter(|n| !ran.contains(n)).collect();
        let unexpected: Vec<&&str> = ran.iter().filter(|n| !declared.contains(n)).collect();
        assert!(
            missing.is_empty() && unexpected.is_empty() && duplicated.is_empty(),
            "what ran and what {what} declares differ, so a declared row may be \
             asserting nothing:\n  declared but never checked: {missing:?}\n  \
             checked but not declared: {unexpected:?}\n  declared twice: \
             {duplicated:?}"
        );
    }
}

#[test]
fn the_contract_covers_sixteen_presets_in_both_modes() -> crate::Result<()> {
    assert_eq!(
        combinations()?.len(),
        32,
        "the contract asserts over 16 presets x 2 modes; a shrinking preset \
         list must not silently narrow it"
    );
    Ok(())
}

/// Every preset paints its buttons on an opaque fill.
///
/// `over` keeps the base's alpha where the base is opaque, which is the arm
/// the four filled-button rows take and the only one that agrees bit for bit
/// with the `Hsla::blend` the connector calls. A translucent idle fill would
/// move those rows onto the general arm, where agreement is no longer exact,
/// so the assumption is asserted rather than remembered.
#[test]
fn every_preset_states_an_opaque_button_fill() -> crate::Result<()> {
    let combinations = combinations()?;
    let translucent: Vec<String> = combinations
        .iter()
        .filter(|c| native_button_fill(&c.resolved).a < 1.0)
        .map(|c| format!("{}: {}", c.label(), show(native_button_fill(&c.resolved))))
        .collect();
    assert!(
        translucent.is_empty(),
        "{} combination(s) state a translucent button fill, which the \
         filled-button rows composite onto:\n{}",
        translucent.len(),
        translucent.join("\n")
    );
    Ok(())
}

#[test]
fn every_mapped_field_equals_its_native_source() -> crate::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut stale = Vec::new();
    let mut ran = Checked::default();

    for c in &combinations {
        for row in ROWS {
            let expected = rgba_to_hsla((row.native)(&c.resolved));
            let actual = (row.get)(&c.colors);
            match row.exceptions.iter().find(|(key, _)| *key == c.key) {
                // An exception is a claim that the row does not hold here, so
                // it is checked too: one that has become true again is stale.
                Some((_, why)) => {
                    if actual == expected {
                        stale.push(format!(
                            "{}: {} is excepted ({why}) yet equals its native value",
                            c.label(),
                            row.slot
                        ));
                    }
                }
                None => {
                    ran.ran(row.slot);
                    if actual != expected {
                        failures.push(format!(
                            "{}: {} is {}, native gives {}",
                            c.label(),
                            row.slot,
                            show(actual),
                            show(expected)
                        ));
                    }
                }
            }
        }
        for row in COMPUTED_ROWS {
            let expected = (row.native)(&c.resolved, c.is_dark);
            let actual = (row.get)(&c.colors);
            match row.exceptions.iter().find(|(key, _)| *key == c.key) {
                Some((_, why)) => {
                    if actual == expected {
                        stale.push(format!(
                            "{}: {} is excepted ({why}) yet equals its native value",
                            c.label(),
                            row.slot
                        ));
                    }
                }
                None => {
                    ran.ran(row.slot);
                    if actual != expected {
                        failures.push(format!(
                            "{}: {} is {}, {} gives {}",
                            c.label(),
                            row.slot,
                            show(actual),
                            row.source,
                            show(expected)
                        ));
                    }
                }
            }
        }
    }

    let declared: Vec<&'static str> = ROWS
        .iter()
        .map(|row| row.slot)
        .chain(COMPUTED_ROWS.iter().map(|row| row.slot))
        .collect();
    ran.covers(&declared, "the contract table");

    println!(
        "--- gpui mapping contract: {} rows x {} combinations = {} comparisons ---",
        declared.len(),
        combinations.len(),
        ran.checks
    );
    assert!(
        stale.is_empty(),
        "{} exception(s) no longer describe a difference:\n{}",
        stale.len(),
        stale.join("\n")
    );
    assert!(
        failures.is_empty(),
        "{} of {} contract checks failed:\n{}",
        failures.len(),
        ran.checks,
        failures.join("\n")
    );
    Ok(())
}

/// Every one of the 138 `ThemeColor` fields is in exactly one of the contract
/// table and `DERIVED` (spec section 5.2).
///
/// The field names come from serde rather than from a hand-written list, so a
/// field added upstream arrives here unclassified and fails by name. There is
/// no third list: `colors.rs` assigns all twelve base-palette fields from
/// native values, and `no_theme_color_field_is_left_at_default` already proves
/// nothing is left at upstream's default.
#[test]
fn every_theme_color_field_has_a_declared_source() -> crate::Result<()> {
    let combinations = combinations()?;
    // Through serde, so a field added upstream is walked without anyone
    // listing it; an empty list means the walk itself broke and fails here.
    let fields: Vec<String> = combinations
        .first()
        .and_then(|c| serde_json::to_value(c.colors).ok())
        .and_then(|value| {
            value
                .as_object()
                .map(|fields| fields.keys().cloned().collect())
        })
        .unwrap_or_default();
    assert_eq!(
        fields.len(),
        138,
        "serde sees a different field count than the tripwire in colors.rs"
    );

    let mut failures = Vec::new();
    for field in &fields {
        let rows = ROWS.iter().filter(|row| row.slot == field).count()
            + COMPUTED_ROWS.iter().filter(|row| row.slot == field).count();
        let derived = DERIVED.iter().filter(|(name, _)| name == field).count();
        match (rows, derived) {
            (1, 0) | (0, 1) => {}
            (0, 0) => failures.push(format!(
                "{field}: in neither the contract table nor DERIVED"
            )),
            (r, d) if r > 0 && d > 0 => failures.push(format!(
                "{field}: claimed by {r} contract row(s) and by {d} DERIVED entries"
            )),
            (r, _) if r > 1 => failures.push(format!("{field}: {r} contract rows claim it")),
            (_, d) => failures.push(format!("{field}: {d} DERIVED entries")),
        }
    }

    assert!(
        failures.is_empty(),
        "{} field(s) are not classified:\n{}",
        failures.len(),
        failures.join("\n")
    );

    let rows = ROWS.len() + COMPUTED_ROWS.len();
    println!(
        "--- gpui coverage: {rows} contract rows + {} derived = {} of 138 fields ---",
        DERIVED.len(),
        rows + DERIVED.len()
    );
    assert_eq!(
        rows + DERIVED.len(),
        fields.len(),
        "the two lists no longer partition ThemeColor: {rows} rows + {} derived \
         against {} fields",
        DERIVED.len(),
        fields.len()
    );
    Ok(())
}

// ---------- Layer 3: the contrast invariant (spec section 7) ----------

/// WCAG AA for normal text. Nothing is asserted against it: it is the
/// threshold the printed list uses, so a pair the platform itself puts below
/// AA stays visible for `preset-validator` to judge.
const AA: f32 = 4.5;

/// One foreground-on-background pair `ThemeColor` carries, with the platform's
/// own pair beside it.
///
/// Both sides come as three colours -- the text, the fill it sits on, and the
/// surface that fill is painted over -- because a fill with alpha below 1
/// shows the surface through, and a ratio measured before compositing is a
/// ratio of something no one sees.
struct Pair {
    what: &'static str,
    /// The pair the platform's own fields give. `is_dark` is the mode, which
    /// a soft option's fallback needs.
    native: fn(&ResolvedTheme, bool) -> (Hsla, Hsla, Hsla),
    emitted: fn(&ThemeColor) -> (Hsla, Hsla, Hsla),
    /// Combinations -- `preset/mode`, because the two this release knows are
    /// one mode of their preset -- where the emitted pair is legitimately
    /// worse than the platform's, each with its reason.
    exceptions: &'static [(&'static str, &'static str)],
}

/// Upstream renders every popup menu on the `popover` token, so a menu's hover
/// layer lands on a surface the platform did not put it on.
///
/// `PopupMenu::render` calls `.popover_style(cx)`, which is
/// `bg(theme.popover)` (`gpui-component-0.6.4/src/menu/popup_menu.rs:1476`,
/// `styled.rs:193-199`), and upstream has no menu-surface token. The connector
/// feeds `popover` from `popover.background_color`, the token's documented
/// meaning, but `menu.background_color` differs from it in 30 of the 32
/// combinations. The call is hardcoded inside upstream's `render`, so there is
/// no seam: it is on the Tier U list. It becomes measurable only where the
/// hover layer is translucent, and both remaining pairs stay above 9:1.
const MENU_SURFACE: &str = "upstream paints menus on the popover token \
                            (menu/popup_menu.rs:1476) and the platform's menu \
                            background differs from its popover background";

/// Every pair the connector controls both colours of.
const PAIRS: &[Pair] = &[
    Pair {
        what: "window text on the window",
        native: |r, _| {
            let bg = rgba_to_hsla(r.defaults.background_color);
            (rgba_to_hsla(r.defaults.text_color), bg, bg)
        },
        emitted: |tc| (tc.foreground, tc.background, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "muted text on the window",
        native: |r, _| {
            let bg = rgba_to_hsla(r.defaults.background_color);
            (rgba_to_hsla(r.defaults.muted_color), bg, bg)
        },
        emitted: |tc| (tc.muted_foreground, tc.background, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "primary button label",
        native: |r, _| {
            (
                rgba_to_hsla(r.button.primary_text_color),
                rgba_to_hsla(r.button.primary_background),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.primary_foreground, tc.primary, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "ordinary button label",
        native: |r, _| {
            (
                rgba_to_hsla(r.button.font.color),
                native_button_fill(r),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.secondary_foreground, tc.secondary, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "hovered button label",
        native: |r, _| {
            (
                rgba_to_hsla(r.button.hover_text_color),
                over(
                    rgba_to_hsla(r.button.hover_background),
                    native_button_fill(r),
                ),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.button_foreground, tc.button_hover, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "pressed button label",
        native: |r, is_dark| {
            (
                rgba_to_hsla(r.button.active_text_color),
                over(native_pressed_fill(r, is_dark), native_button_fill(r)),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.button_foreground, tc.button_active, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "hovered menu row",
        native: |r, _| {
            (
                rgba_to_hsla(r.menu.hover_text_color),
                rgba_to_hsla(r.menu.hover_background),
                rgba_to_hsla(r.menu.background_color),
            )
        },
        emitted: |tc| (tc.accent_foreground, tc.accent, tc.popover),
        exceptions: &[
            ("windows-11/dark", MENU_SURFACE),
            ("material/dark", MENU_SURFACE),
        ],
    },
    Pair {
        what: "popover text",
        native: |r, _| {
            (
                rgba_to_hsla(r.popover.font.color),
                rgba_to_hsla(r.popover.background_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.popover_foreground, tc.popover, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "danger label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.danger_text_color),
                rgba_to_hsla(r.defaults.danger_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.danger_foreground, tc.danger, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "success label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.success_text_color),
                rgba_to_hsla(r.defaults.success_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.success_foreground, tc.success, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "warning label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.warning_text_color),
                rgba_to_hsla(r.defaults.warning_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.warning_foreground, tc.warning, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "info label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.info_text_color),
                rgba_to_hsla(r.defaults.info_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.info_foreground, tc.info, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "sidebar text",
        native: |r, _| {
            (
                rgba_to_hsla(r.sidebar.font.color),
                rgba_to_hsla(r.sidebar.background_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.sidebar_foreground, tc.sidebar, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "selected sidebar row",
        native: |r, _| {
            (
                rgba_to_hsla(r.sidebar.selection_text_color),
                rgba_to_hsla(r.sidebar.selection_background),
                rgba_to_hsla(r.sidebar.background_color),
            )
        },
        emitted: |tc| (tc.sidebar_accent_foreground, tc.sidebar_accent, tc.sidebar),
        exceptions: &[],
    },
    Pair {
        what: "sidebar primary button label",
        native: |r, _| {
            (
                rgba_to_hsla(r.button.primary_text_color),
                rgba_to_hsla(r.button.primary_background),
                rgba_to_hsla(r.sidebar.background_color),
            )
        },
        emitted: |tc| {
            (
                tc.sidebar_primary_foreground,
                tc.sidebar_primary,
                tc.sidebar,
            )
        },
        exceptions: &[],
    },
    Pair {
        what: "tab label",
        native: |r, _| {
            (
                rgba_to_hsla(r.tab.font.color),
                rgba_to_hsla(r.tab.background_color),
                rgba_to_hsla(r.tab.bar_background),
            )
        },
        emitted: |tc| (tc.tab_foreground, tc.tab, tc.tab_bar),
        exceptions: &[],
    },
    Pair {
        what: "active tab label",
        native: |r, _| {
            (
                rgba_to_hsla(r.tab.active_text_color),
                rgba_to_hsla(r.tab.active_background),
                rgba_to_hsla(r.tab.bar_background),
            )
        },
        emitted: |tc| (tc.tab_active_foreground, tc.tab_active, tc.tab_bar),
        exceptions: &[],
    },
    // The model carries no title-bar or status-bar text colour of its own, so
    // the platform's pair reads the window foreground on that bar -- exactly
    // what upstream inherits into both.
    Pair {
        what: "title bar text",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.text_color),
                rgba_to_hsla(r.window.title_bar_background),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.foreground, tc.title_bar, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "status bar text",
        native: |r, _| {
            (
                rgba_to_hsla(r.status_bar.font.color),
                rgba_to_hsla(r.status_bar.background_color),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.foreground, tc.status_bar, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "table header text",
        native: |r, _| {
            let bg = rgba_to_hsla(r.defaults.background_color);
            (rgba_to_hsla(r.defaults.muted_color), bg, bg)
        },
        emitted: |tc| (tc.table_head_foreground, tc.table_head, tc.background),
        exceptions: &[],
    },
    // A list row keeps the inherited text colour whatever its state
    // (`list/list_item.rs:209, 237`), so the emitted foreground is the
    // window's; for an idle and a hovered row the platform agrees.
    Pair {
        what: "list row text",
        native: |r, _| {
            let bg = rgba_to_hsla(r.list.background_color);
            (rgba_to_hsla(r.list.item_font.color), bg, bg)
        },
        emitted: |tc| (tc.foreground, tc.list, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "hovered list row",
        native: |r, _| {
            (
                rgba_to_hsla(r.list.hover_text_color),
                rgba_to_hsla(r.list.hover_background),
                rgba_to_hsla(r.list.background_color),
            )
        },
        emitted: |tc| (tc.foreground, tc.list_hover, tc.list),
        exceptions: &[],
    },
    Pair {
        what: "link on the window",
        native: |r, _| {
            let bg = rgba_to_hsla(r.defaults.background_color);
            (rgba_to_hsla(r.defaults.link_color), bg, bg)
        },
        emitted: |tc| (tc.link, tc.background, tc.background),
        exceptions: &[],
    },
];

/// A pair whose two ratios are printed and not asserted, with the reason.
///
/// Spec section 7 asserts where the connector controls both colours and
/// reports where it does not. Two of these are upstream gaps -- a `ThemeColor`
/// with no foreground token for a surface the platform pairs one with -- and
/// the third is a finding this test made, recorded here rather than silenced:
/// see `LINK_HOVER_IS_A_FILL`.
struct Reported {
    what: &'static str,
    why: &'static str,
    native: fn(&ResolvedTheme) -> (Hsla, Hsla, Hsla),
    emitted: fn(&ThemeColor) -> (Hsla, Hsla, Hsla),
}

/// **A finding, not an accepted difference.**
///
/// `ThemeColor::link_hover` is upstream's hovered-link *text* colour
/// (`theme_color.rs:178`, and its only reader takes it as `fg` at
/// `button/button.rs:1139`, over the transparent fill of `:1133`). The
/// connector feeds it `link.hover_background`, a fill, which every preset
/// states at about 9% alpha -- so a hovered link button's label is very nearly
/// invisible, and the pair measures around 1.1:1 against the platform's 1.9:1
/// to 10.3:1, in all 32 combinations. It is the defect class of spec section
/// 1.2 in a field this release did not look at, and the fix is a mapping
/// change with an iced sibling and a decision behind it, so it is reported
/// here for the maintainer rather than made quietly.
const LINK_HOVER_IS_A_FILL: &str = "FINDING: link_hover is upstream's hovered-link text colour \
                                    and the connector maps a fill into it \
                                    (link.hover_background, ~9% alpha); reported, not accepted";

/// Pairs measured and printed, never asserted.
const REPORTED: &[Reported] = &[
    Reported {
        what: "selected list row",
        why: "ThemeColor has no foreground for a selected row: upstream paints \
              list_active and lets the window foreground through \
              (list/list_item.rs:237), while the platform pairs \
              list.selection_background with list.selection_text_color",
        native: |r| {
            (
                rgba_to_hsla(r.list.selection_text_color),
                rgba_to_hsla(r.list.selection_background),
                rgba_to_hsla(r.list.background_color),
            )
        },
        emitted: |tc| (tc.foreground, tc.list_active, tc.list),
    },
    Reported {
        what: "selected text",
        why: "ThemeColor has no foreground for selected text: `selection` is a \
              highlight upstream paints under text that keeps its own colour \
              (input/input.rs:502), while the platform pairs \
              selection_background with selection_text_color",
        native: |r| {
            (
                rgba_to_hsla(r.defaults.selection_text_color),
                rgba_to_hsla(r.defaults.selection_background),
                rgba_to_hsla(r.defaults.background_color),
            )
        },
        emitted: |tc| (tc.foreground, tc.selection, tc.background),
    },
    Reported {
        what: "hovered link text",
        why: LINK_HOVER_IS_A_FILL,
        native: |r| {
            let bg = rgba_to_hsla(r.defaults.background_color);
            (rgba_to_hsla(r.link.hover_text_color), bg, bg)
        },
        emitted: |tc| (tc.link_hover, tc.background, tc.background),
    },
];

/// Contrast of `fg` on `bg`, with every layer composited first: `bg` over the
/// `surface` it is painted on, `fg` over the result.
fn pair_ratio(fg: Hsla, bg: Hsla, surface: Hsla) -> f32 {
    let bg = over(bg, surface);
    contrast_ratio(over(fg, bg), bg)
}

/// Spec section 7: for every pair the connector controls both colours of, the
/// ratio it emits is not worse than the ratio the platform's own fields give.
///
/// Not AA: 174 of the 512 pairs measured for this release sit below it, and
/// they are not all errors -- macOS ships `#34c759` with white text at about
/// 2.2:1 and Apple uses it. A connector that asserted AA would be asserting
/// that every platform meets AA, and "fixing" the ones that do not would mean
/// inventing values the platform did not give. Those pairs are printed
/// instead, so they stay visible without being overridden.
#[test]
fn no_pair_contrasts_worse_than_the_platforms_own() -> crate::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut stale = Vec::new();
    let mut below_aa = Vec::new();
    let mut ran = Checked::default();

    for c in &combinations {
        let label = c.label();
        for pair in PAIRS {
            let (native_fg, native_bg, native_surface) = (pair.native)(&c.resolved, c.is_dark);
            let native = pair_ratio(native_fg, native_bg, native_surface);
            let (fg, bg, surface) = (pair.emitted)(&c.colors);
            let emitted = pair_ratio(fg, bg, surface);
            match pair.exceptions.iter().find(|(key, _)| *key == label) {
                // An exception claims the pair degrades here, so it is checked
                // too: one that no longer degrades is stale and says so.
                Some((_, why)) => {
                    if emitted >= native {
                        stale.push(format!(
                            "{label}: {} is excepted ({why}) yet emits {emitted:.4} \
                             against the platform's {native:.4}",
                            pair.what
                        ));
                    }
                }
                None => {
                    ran.ran(pair.what);
                    if emitted < native {
                        failures.push(format!(
                            "{label}: {} emitted {emitted:.4}, native {native:.4}; \
                             emitted {} on {}, native {} on {}",
                            pair.what,
                            show(fg),
                            show(bg),
                            show(native_fg),
                            show(native_bg)
                        ));
                    }
                }
            }
            if emitted < AA {
                below_aa.push(format!(
                    "{label}: {} {emitted:.2} (native {native:.2})",
                    pair.what
                ));
            }
        }
    }

    let declared: Vec<&'static str> = PAIRS.iter().map(|pair| pair.what).collect();
    ran.covers(&declared, "the contrast pair list");

    // The pairs section 7 reports rather than asserts: printed with both
    // ratios, and with the count of combinations where ours is the worse, so
    // the record says how large each one is rather than only that it exists.
    let mut reported = Vec::new();
    for pair in REPORTED {
        let mut worse = 0usize;
        let mut lines = Vec::new();
        for c in &combinations {
            let (native_fg, native_bg, native_surface) = (pair.native)(&c.resolved);
            let native = pair_ratio(native_fg, native_bg, native_surface);
            let (fg, bg, surface) = (pair.emitted)(&c.colors);
            let emitted = pair_ratio(fg, bg, surface);
            if emitted < native {
                worse += 1;
            }
            if emitted < AA {
                below_aa.push(format!(
                    "{}: {} {emitted:.2} (native {native:.2})  [reported]",
                    c.label(),
                    pair.what
                ));
            }
            lines.push(format!(
                "  {}: {emitted:.2} (native {native:.2})",
                c.label()
            ));
        }
        reported.push(format!(
            "{} -- {}\n  worse than the platform's own in {worse} of {} \
             combinations\n{}",
            pair.what,
            pair.why,
            combinations.len(),
            lines.join("\n")
        ));
    }

    println!(
        "--- gpui contrast: {} asserted pairs x {} combinations = {} \
         comparisons, {} below AA ---\n{}\n--- reported, not asserted ---\n{}",
        PAIRS.len(),
        combinations.len(),
        ran.checks,
        below_aa.len(),
        below_aa.join("\n"),
        reported.join("\n")
    );
    assert!(
        stale.is_empty(),
        "{} contrast exception(s) no longer describe a degradation:\n{}",
        stale.len(),
        stale.join("\n")
    );
    assert!(
        failures.is_empty(),
        "{} of {} pairs contrast worse than the platform's own:\n{}",
        failures.len(),
        ran.checks,
        failures.join("\n")
    );
    Ok(())
}
