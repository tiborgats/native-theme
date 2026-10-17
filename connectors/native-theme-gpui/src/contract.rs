//! Layer 1 of the theme contracts: every `ThemeColor` field this connector
//! writes, the native value it must equal or the derivation that produced it,
//! and the contrast invariant of spec section 7.
//!
//! Test-only. The rows are asserted over the seventeen presets of
//! `Theme::list_presets()` in both modes. A row that must differ on some preset carries the
//! reason in `exceptions`, and an exception that no longer describes a real
//! difference fails just as loudly as a broken row.
//!
//! The coverage tripwire (section 5.2) closes the table: every one of the 139
//! `ThemeColor` fields -- enumerated through serde, the same mechanism
//! `colors::tests::no_theme_color_field_is_left_at_default` uses, so a field
//! added upstream appears here without anyone listing it -- must appear in
//! exactly one of the rows and `DERIVED`. Its converse walks the model rather
//! than the output: with `widgets`,
//! `every_colour_leaf_reaches_a_sink_or_is_named` splices every colour leaf of
//! `ResolvedTheme` and requires each to move one of the connector's outputs,
//! or to be named by `UNREAD_COLOURS` or `NO_RECEIVER`.

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
/// `DERIVED`; the three lists partition the 139 fields, which
/// `every_theme_color_field_has_a_declared_source` asserts.
const ROWS: &[Row] = &[
    // The model states the window's own fill, which inherits
    // `defaults.background_color` and which no bundled preset overrides; the
    // row names the field the token means rather than the one it happens to
    // equal.
    Row {
        slot: "background",
        native: |r| r.window.background_color,
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
    // Upstream documents `selection` as the *input* selection background
    // (`theme_color.rs:228`), and its text readers are the input editor style
    // (`input/input.rs:562`), the text view (`text/mod.rs:52`), the touch
    // handle (`touch_selection/handle.rs:96`) and the time field's selected
    // segment (`time/time_field.rs:135`). It also fills a token input's chips
    // (`input/token.rs:50-51`) and outlines a table's or list's
    // secondary-selected row (`table/state.rs:2295`, `list/list_item.rs:272`),
    // so it is not only a text selection. The documented meaning decides the
    // model's counterpart: `input.selection_background`, which inherits
    // `defaults.text_selection_background` and in turn
    // `defaults.selection_background` -- the row-selection colour the
    // `sidebar_accent` and `list_active` rows read.
    Row {
        slot: "selection",
        native: |r| r.input.selection_background,
        get: |tc| tc.selection,
        exceptions: &[],
    },
    // The link's own text (`link.rs:76`, `button/button.rs:1053`), which the
    // model states as `link.font.color` -- the one font colour that inherits
    // `defaults.link_color` instead of `defaults.font.color`.
    Row {
        slot: "link",
        native: |r| r.link.font.color,
        get: |tc| tc.link,
        exceptions: &[],
    },
    // `link_hover` and `link_active` are upstream's hovered and pressed link
    // *text* colours ("Hover link text color", `theme_color.rs:179`, `:181`), and
    // their only reader takes each as the `fg` of a `Button::link`
    // (`button/button.rs:1199, 1275, 1317`) over the transparent fill that
    // variant paints in every state (`:1193, 1270, 1310`). Neither native
    // field is a soft option, so both are plain rows.
    Row {
        slot: "link_hover",
        native: |r| r.link.hover_text_color,
        get: |tc| tc.link_hover,
        exceptions: &[],
    },
    Row {
        slot: "link_active",
        native: |r| r.link.active_text_color,
        get: |tc| tc.link_active,
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
    Row {
        slot: "chart_grid",
        native: |r| r.list.grid_color,
        get: |tc| tc.chart_grid,
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
        native: |r| r.list.background_color,
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
    // The header row's own fill: the model states it
    // (`list.header_background`, inheriting `defaults.surface_color`), and it
    // differs from the window background in nearly every preset. Upstream
    // paints `table_head` as the header row (`table/column.rs:278`,
    // `table/state.rs:1608, :1852`, `table/table.rs:199`); `list_head` has no
    // reader in 0.7.3 beyond a schema fallback (`theme/schema.rs:972, 1010`),
    // but the mapping is the truth for the day it gets one.
    Row {
        slot: "list_head",
        native: |r| r.list.header_background,
        get: |tc| tc.list_head,
        exceptions: &[],
    },
    Row {
        slot: "table",
        native: |r| r.list.background_color,
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
        native: |r| r.list.header_background,
        get: |tc| tc.table_head,
        exceptions: &[],
    },
    // The header row's own text (`table/table.rs:200`,
    // `table/state.rs:1853`), which the model states as `list.header_font`.
    Row {
        slot: "table_head_foreground",
        native: |r| r.list.header_font.color,
        get: |tc| tc.table_head_foreground,
        exceptions: &[],
    },
    // The line between rows and columns (`table/table.rs:203, 343, 415`,
    // `table/state.rs:1504, 1607, 2063, 2323`), which the model states as
    // `list.grid_color` rather than as the generic border.
    Row {
        slot: "table_row_border",
        native: |r| r.list.grid_color,
        get: |tc| tc.table_row_border,
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
    // The track a segmented tab bar paints itself with
    // (`tab/tab_bar.rs:393`, the `TabVariant::Segmented` arm). The model
    // states it in `SegmentedControlTheme`, a struct of its own -- the
    // justification this row used to carry was about `ResolvedTabTheme`,
    // which indeed has no segmented field, and stopped there.
    Row {
        slot: "tab_bar_segmented",
        native: |r| r.segmented_control.background_color,
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
        native: |r| r.sidebar.border.color,
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
    // The model has no accordion or expander fill -- `ExpanderTheme` states
    // geometry and a soft hover only -- so the connector gives the panel the
    // window's own fill, which is what this row names.
    Row {
        slot: "accordion",
        native: |r| r.window.background_color,
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
    // (`colors.rs:686-693`); their `_light` siblings are derived.
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
/// Written here rather than taken from `colors::over`, which the connector
/// calls: an expectation computed with the code under test would agree with
/// anything. The two are the same rule, the same products added in the same
/// order, so they agree bit for bit; over an opaque `base` the result's alpha
/// is exactly 1 and this is `Hsla::blend`, which the connector called before
/// it composited a translucent layer over a translucent fill.
fn over(layer: Hsla, base: Hsla) -> Hsla {
    if layer.a >= 1.0 {
        return layer;
    }
    if layer.a <= 0.0 {
        return base;
    }
    let (l, b) = (gpui::Rgba::from(layer), gpui::Rgba::from(base));
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

/// The window's own fill: the surface everything else is painted over, and the
/// native field the `background` row pins `tokens.background` to.
fn native_window(r: &ResolvedTheme) -> Hsla {
    rgba_to_hsla(r.window.background_color)
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
    // Every stated layer is opaque and composites to itself; macos-tahoe's
    // derived pressed fill is the one translucent layer, over a translucent
    // fill, so the compositing is visible there (`DERIVED_PRESSED_FILL`).
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
    // Upstream really does paint a table footer (`table/table.rs:340-341`),
    // but the model carries no footer colour of any kind -- `ListTheme` states
    // a header and rows and nothing below them -- so neither field can name a
    // native source without inventing one.
    (
        "table_foot",
        "no native footer colour; the window background",
    ),
    (
        "table_foot_foreground",
        "no native footer text colour; the muted foreground",
    ),
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
        "defaults.shadow_color at the mode's overlay alpha; transparent under reduce_transparency",
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

/// Native colours `ThemeColor` has no field to receive: the ones this release
/// examined, with the evidence. Not an exhaustive census of the model.
///
/// A claim about a *native* field rather than an emitted one, so the coverage
/// tripwire cannot see it: that walks what the connector produces. What is
/// checked instead is that each entry is about something real -- the field is
/// read here, so removing it from the model breaks this file, and
/// `every_unreachable_native_colour_is_stated_by_every_preset` requires every
/// preset to state it, so the record says what is dropped rather than only
/// that something is.
struct NoReceiver {
    field: &'static str,
    native: fn(&ResolvedTheme) -> Rgba,
    evidence: &'static str,
    /// Presets whose platform states no value, which they express as fully
    /// transparent, with the source that says so: nothing is dropped there,
    /// and a preset listed here that states a colour fails as loudly as a
    /// missing one.
    stated_none: &'static [(&'static str, &'static str)],
}

/// Why kde-breeze, adwaita, macos-monterey, macos-tahoe and windows-11 leave
/// `link.hover_background` transparent.
const LINK_HOVER_NONE: &str = "docs/platform-facts.md:1685 (§2.28) states no \
                               link hover background for macOS, KDE and GNOME, \
                               nor for WinUI's inline Hyperlink, which has no \
                               background";

const NO_RECEIVER: &[NoReceiver] = &[
    NoReceiver {
        field: "link.hover_background",
        native: |r| r.link.hover_background,
        evidence: "`ButtonVariant::Link` paints `theme.transparent` as its \
                   background in every state (`button/button.rs:1193` hovered, \
                   `:1270` pressed, `:1310` selected) and the `Link` widget \
                   paints none at all (`link.rs:70-90`); `transparent` lives \
                   on `Theme`, not on `ThemeColor`, and is not one of the 139. \
                   `link_hover` and `link_active` are the link's hover and \
                   pressed *text* (`theme_color.rs:179`, `:181`), which is what \
                   their rows give them.",
        stated_none: &[
            ("kde-breeze", LINK_HOVER_NONE),
            ("adwaita", LINK_HOVER_NONE),
            ("macos-monterey", LINK_HOVER_NONE),
            ("macos-tahoe", LINK_HOVER_NONE),
            ("windows-11", LINK_HOVER_NONE),
        ],
    },
    NoReceiver {
        field: "segmented_control.active_background",
        stated_none: &[],
        native: |r| r.segmented_control.active_background,
        evidence: "the selected segment of a `TabVariant::Segmented` bar is \
                   filled with `tokens.background` (`tab/tab.rs:249`, and \
                   `:201` while hovered) -- the window token, not one of its \
                   own -- so a platform that tints the active segment has \
                   nowhere to put the tint. `tab_bar_segmented` takes the \
                   track around it (`tab/tab_bar.rs:393`).",
    },
    NoReceiver {
        field: "segmented_control.active_text_color",
        stated_none: &[],
        native: |r| r.segmented_control.active_text_color,
        evidence: "the selected segment's label is `tab_active_foreground` \
                   (`tab/tab.rs:247`), the token the `tab_active_foreground` \
                   row gives `tab.active_text_color`; upstream has no \
                   segmented-specific text token, so the segmented control's \
                   own label colour cannot reach it.",
    },
    NoReceiver {
        field: "segmented_control.font.color",
        stated_none: &[],
        native: |r| r.segmented_control.font.color,
        evidence: "a `TabBar` labels every segment from the tab family -- \
                   `tab_foreground` on the bar (`tab/tab_bar.rs:541`) and on \
                   an idle segment (`tab/tab.rs:154`), `tab_active_foreground` \
                   on the selected one (`:247`) -- and those two tokens are \
                   `tab.font.color` and `tab.active_text_color` by their rows. \
                   The segmented control's own label colour has nowhere to go.",
    },
    NoReceiver {
        field: "segmented_control.border.color",
        stated_none: &[],
        native: |r| r.segmented_control.border.color,
        evidence: "a segmented `TabBar` paints no border at all: the bottom \
                   rule is drawn only for the `Underline` and `Tab` variants \
                   and takes the shared `border` token \
                   (`tab/tab_bar.rs:542-556`), which the `border` row gives \
                   `defaults.border.color`.",
    },
    // With `widgets` the connector's own switch draws it (`SwitchLook`), and
    // the colour-leaf coverage test, which runs only there, sees it reached.
    #[cfg(not(feature = "widgets"))]
    NoReceiver {
        field: "switch.checked_background",
        stated_none: &[],
        native: |r| r.switch.checked_background,
        evidence: "`ThemeColor` has no checked-state field for a switch -- \
                   only `switch` and `switch_thumb` -- and upstream paints a \
                   checked one with `tokens.primary` (`switch.rs:180-183`). \
                   `Switch::color` is a per-instance receiver (`switch.rs:102`), \
                   which an application can feed and the connector does not. \
                   Recorded at `colors.rs`'s Issue 51 note as well.",
    },
];

/// Why a colour leaf of `ResolvedTheme` reaches no sink: the classes of
/// `UNREAD_COLOURS`, each with its evidence. iced's `Unread`
/// (`native-theme-iced/src/contract/derived.rs`) has the same shape.
#[cfg(feature = "widgets")]
enum Unread {
    /// gpui-component and gpui-base 0.7.3 have nowhere to draw the value as
    /// the model states it, or only a receiver that would also paint a
    /// variant upstream leaves bare. The evidence cites the upstream source,
    /// `file.rs:line`.
    NoReceiver(&'static str),
    /// The value reaches the connector through the widget leaves that inherit
    /// it, which sinks read. The evidence cites
    /// `docs/inheritance-rules.toml:line`.
    Inherited(&'static str),
    /// The value is a soft option's fallback, which the connector draws where
    /// the option is unstated, and every bundled preset states the option.
    /// The evidence cites the connector's `file.rs:line` that falls back, and
    /// the walk proves it: with `option` cleared in a base, splicing the leaf
    /// must move a sink.
    Fallback {
        /// The soft option's dotted path, which the leaf stands in for.
        option: &'static str,
        evidence: &'static str,
    },
}

/// One colour leaf no sink reads, and why.
#[cfg(feature = "widgets")]
struct UnreadColour {
    /// The leaf's dotted path in the serialised `ResolvedTheme`.
    leaf: &'static str,
    why: Unread,
}

/// gpui-component's text fields draw the selected text in the field's text
/// colour: the selection is a fill behind it.
#[cfg(feature = "widgets")]
const SELECTED_TEXT: &str = "`InputEditorStyle` has a selection fill and no \
     selected-text colour (`input/input.rs:556-563`), so a selected run is \
     drawn in the field's text colour";

/// The colour leaves of `ResolvedTheme` that reach no sink in any of the 34
/// bundled combinations and that `NO_RECEIVER` does not name, each with its
/// reason.
///
/// `every_colour_leaf_reaches_a_sink_or_is_named` holds the list to the
/// model: a colour leaf that reaches no sink and is named nowhere fails it,
/// and so does an entry that names a leaf a sink now reads, or no colour leaf
/// at all. It is a table of its own rather than more of `NO_RECEIVER`, whose
/// entries are about `ThemeColor` and must be stated by every preset: several
/// entries here reach gpui through the leaves that inherit them, and five
/// have a receiver the connector leaves unfilled by a ruling: the trigger's
/// and the text area's edge, which would paint a frameless variant, and the
/// three most presets inherit rather than state.
#[cfg(feature = "widgets")]
const UNREAD_COLOURS: &[UnreadColour] = &[
    UnreadColour {
        leaf: "defaults.accent_text_color",
        why: Unread::Inherited(
            "the primary button's label and the check mark inherit it, \
             `button.primary_text_color` and `checkbox.indicator_color` \
             (`docs/inheritance-rules.toml:157`, `:181`), and they reach \
             `primary_foreground` and `CheckboxLook`",
        ),
    },
    UnreadColour {
        leaf: "defaults.font.color",
        why: Unread::Inherited(
            "every widget's font colour inherits it \
             (`docs/inheritance-rules.toml:109`), and the widgets' own reach \
             their sinks: `checkbox.font.color`, `input.font.color`, \
             `list.item_font.color`, `popover.font.color`, \
             `status_bar.font.color` and `window.title_bar_font.color` among \
             them",
        ),
    },
    UnreadColour {
        leaf: "defaults.link_color",
        why: Unread::Inherited(
            "`link.font.color` inherits it (`docs/inheritance-rules.toml:286`), \
             and the `link` row reads that",
        ),
    },
    UnreadColour {
        leaf: "defaults.selection_background",
        why: Unread::Inherited(
            "the selections that sinks read inherit it: \
             `menu.hover_background`, `sidebar.selection_background` and \
             `list.selection_background` (`docs/inheritance-rules.toml:189`, \
             `:218`, `:234`), and `input.selection_background` through \
             `defaults.text_selection_background` (`:54`, `:167`)",
        ),
    },
    UnreadColour {
        leaf: "defaults.selection_text_color",
        why: Unread::Inherited(
            "the selected-item labels that sinks read inherit it: \
             `menu.hover_text_color`, `sidebar.selection_text_color` and \
             `list.selection_text_color` (`docs/inheritance-rules.toml:190`, \
             `:219`, `:235`)",
        ),
    },
    UnreadColour {
        leaf: "defaults.surface_color",
        why: Unread::Inherited(
            "the surfaces that sinks read inherit it: \
             `window.title_bar_background`, `slider.thumb_color`, \
             `list.header_background` and `switch.thumb_background` \
             (`docs/inheritance-rules.toml:150`, `:203`, `:236`, `:250`); \
             `card.background_color` inherits it too (`:266`), which \
             `geometry::group_box_content` reads",
        ),
    },
    UnreadColour {
        leaf: "defaults.text_selection_background",
        why: Unread::Inherited(
            "`input.selection_background` inherits it \
             (`docs/inheritance-rules.toml:167`), and the `selection` row \
             reads that",
        ),
    },
    UnreadColour {
        leaf: "defaults.mono_font.color",
        why: Unread::NoReceiver(
            "`Theme` has `mono_font_family` and `mono_font_size` and no mono \
             colour (`theme/mod.rs:181-183`); an editor's text is the root's \
             text colour or the highlight theme's",
        ),
    },
    UnreadColour {
        leaf: "defaults.selection_inactive_background",
        why: Unread::NoReceiver(
            "none of the 139 `ThemeColor` fields is an inactive selection \
             (`theme/theme_color.rs:59-343`), and a selected `ListItem` fills \
             `list_active` or `accent` whatever the window's state \
             (`list/list_item.rs:257-264`)",
        ),
    },
    UnreadColour {
        leaf: "defaults.text_selection_color",
        why: Unread::NoReceiver(SELECTED_TEXT),
    },
    UnreadColour {
        leaf: "input.selection_text_color",
        why: Unread::NoReceiver(SELECTED_TEXT),
    },
    UnreadColour {
        leaf: "button.hover_text_color",
        why: Unread::NoReceiver(
            "a hovered Default, Primary or Secondary button keeps its \
             `text_color` (`button/button.rs:1198-1203`), and a \
             `ButtonCustomVariant` has one foreground for every state \
             (`:40-45`)",
        ),
    },
    UnreadColour {
        leaf: "combo_box.hover_background",
        why: Unread::NoReceiver(
            "neither trigger has a hover style (`select.rs:536-555`, \
             `combobox.rs:981-998`), and a `StyleRefinement` has no hover \
             state",
        ),
    },
    UnreadColour {
        leaf: "combo_box.border.color",
        why: Unread::NoReceiver(
            "the trigger's edge is `border_color(input)` (`select.rs:548`, \
             `combobox.rs:993`) before the refinement (`select.rs:553`, \
             `combobox.rs:998`), the only receiver, and a trigger built with \
             `appearance(false)` keeps `border_1()` in `transparent` \
             (`select.rs:542-543`, `combobox.rs:987-988`), which an edge \
             colour in the refinement would paint. The leaf equals \
             `input.border.color`, which the `input` row reads, in all 34 \
             combinations",
        ),
    },
    UnreadColour {
        leaf: "text_area.border.color",
        why: Unread::NoReceiver(
            "a text area's edge is `border_color(input)` \
             (`input/input.rs:798`) before the refinement (`:803`), the only \
             receiver, and a field built with `bordered(false)` or \
             `appearance(false)` draws no edge (`input/input.rs:795-799`), \
             which an edge colour in the refinement would paint over the \
             width `geometry::text_area` sets. The leaf inherits \
             `input.border.color` (`docs/inheritance-rules.toml:173`), which \
             the `input` row reads, and equals it in all 34 combinations",
        ),
    },
    UnreadColour {
        leaf: "expander.arrow_color",
        why: Unread::NoReceiver(
            "an Accordion's chevron is `muted_foreground`, with no style hook \
             (`accordion.rs:327-331`)",
        ),
    },
    UnreadColour {
        leaf: "input.focus_border_color",
        why: Unread::NoReceiver(
            "a focused field's edge is `ring` (`input/input.rs:761`, `:806`), \
             the token every focus ring shares, which the `ring` row gives \
             `defaults.focus_ring_color`",
        ),
    },
    UnreadColour {
        leaf: "input.hover_border_color",
        why: Unread::NoReceiver(
            "an `Input` has no hover style: its render \
             (`input/input.rs:707-803`) sets no `.hover(`",
        ),
    },
    UnreadColour {
        leaf: "input.placeholder_color",
        why: Unread::NoReceiver(
            "the placeholder is `editor_style.muted_foreground` (gpui-base \
             `input/base/element.rs:2543-2547`), which is \
             `theme.muted_foreground` (`input/input.rs:559`), the token all \
             muted text shares",
        ),
    },
    UnreadColour {
        leaf: "link.disabled_text_color",
        why: Unread::NoReceiver("`Link` has no disabled state (`link.rs:69-90`)"),
    },
    UnreadColour {
        leaf: "link.visited_text_color",
        why: Unread::NoReceiver("`Link` has no visited state (`link.rs:69-90`)"),
    },
    UnreadColour {
        leaf: "list.disabled_text_color",
        why: Unread::NoReceiver(
            "a non-selectable `ListItem` takes `muted_foreground` \
             (`list/list_item.rs:235-237`) after the refinement (`:209`)",
        ),
    },
    UnreadColour {
        leaf: "list.hover_text_color",
        why: Unread::NoReceiver(
            "a hovered `ListItem` changes only its fill \
             (`list/list_item.rs:227-233`)",
        ),
    },
    UnreadColour {
        leaf: "menu.background_color",
        why: Unread::NoReceiver(
            "a `PopupMenu` paints `popover_style` (`menu/popup_menu.rs:1499` \
             -> `styled.rs:190`) and is not `Styled`: its fill is the \
             `popover` token, which the `popover` row gives \
             `popover.background_color`",
        ),
    },
    UnreadColour {
        leaf: "menu.border.color",
        why: Unread::NoReceiver(
            "the menu's edge is the popover ring, `foreground` at a fixed ink \
             (`styled.rs:36-38`, `:192`), not a border",
        ),
    },
    UnreadColour {
        leaf: "menu.disabled_text_color",
        why: Unread::NoReceiver(
            "a disabled `MenuItem` takes `muted_foreground` \
             (`menu/menu_item.rs:131-133`), the token all muted text shares",
        ),
    },
    UnreadColour {
        leaf: "menu.font.color",
        why: Unread::NoReceiver(
            "menu text is `popover_foreground` (`menu/popup_menu.rs:1500`), \
             which the `popover_foreground` row gives `popover.font.color`",
        ),
    },
    UnreadColour {
        leaf: "menu.separator_color",
        why: Unread::NoReceiver(
            "the separator is a hairline in `border` \
             (`menu/popup_menu.rs:1270-1276`), the token every edge shares",
        ),
    },
    UnreadColour {
        leaf: "segmented_control.hover_background",
        why: Unread::NoReceiver(
            "a segment's fill is `tokens.background` or transparent \
             (`tab/tab.rs:197-204`), the site of the segmented control's \
             `NO_RECEIVER` entries",
        ),
    },
    UnreadColour {
        leaf: "sidebar.hover_background",
        why: Unread::NoReceiver(
            "a hovered sidebar item takes `sidebar_accent` at 0.8, the \
             selection's token (`sidebar/menu.rs:305-308`)",
        ),
    },
    UnreadColour {
        leaf: "checkbox.background_color",
        why: Unread::Fallback {
            option: "checkbox.unchecked_background",
            evidence: "`CheckboxLook` fills an unchecked box with it where \
                       `checkbox.unchecked_background` is unset \
                       (`widgets/checkbox.rs:123`), and every bundled preset \
                       states that",
        },
    },
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
            .filter_map(|pair| match pair {
                [a, b] if a == b => Some(*a),
                _ => None,
            })
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
fn the_contract_covers_seventeen_presets_in_both_modes() -> crate::Result<()> {
    assert_eq!(
        combinations()?.len(),
        34,
        "the contract asserts over 17 presets x 2 modes; a shrinking preset \
         list must not silently narrow it"
    );
    Ok(())
}

/// Each colour `NO_RECEIVER` records is one the presets really state, but
/// where the entry's `stated_none` says the platform states none.
///
/// A note about a value nothing receives is worth only as much as the value:
/// if every preset left it empty there would be nothing to record. The values
/// are printed so the record says what is dropped.
#[test]
fn every_unreachable_native_colour_is_stated_by_every_preset() -> crate::Result<()> {
    let combinations = combinations()?;
    let mut missing = Vec::new();

    for entry in NO_RECEIVER {
        let mut stated = Vec::new();
        for c in &combinations {
            let value = rgba_to_hsla((entry.native)(&c.resolved));
            let none = entry.stated_none.iter().find(|(key, _)| *key == c.key);
            if let Some((_, why)) = none {
                if value.a > 0.0 {
                    missing.push(format!(
                        "{}: {} states {} where it should state none ({why})",
                        c.label(),
                        entry.field,
                        show(value)
                    ));
                }
            } else if value.a <= 0.0 {
                missing.push(format!("{}: {}", c.label(), entry.field));
            } else {
                stated.push(format!("{}: {}", c.label(), show(value)));
            }
        }
        println!(
            "--- {}, which ThemeColor cannot take, stated by {} of {} \
             combinations ---\n{}\n{}",
            entry.field,
            stated.len(),
            combinations.len(),
            entry.evidence,
            stated.join("\n")
        );
    }

    assert!(
        missing.is_empty(),
        "{} combination(s) leave a NO_RECEIVER colour fully transparent, so \
         the note no longer describes a value the platform gives:\n{}",
        missing.len(),
        missing.join("\n")
    );
    Ok(())
}

/// The combinations whose platform states a translucent button fill:
/// `controlColor` is `#ffffff3f` in darkAqua
/// (docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/probe.toml:408),
/// macOS 26 draws the push button in a translucent fill, `#00000014` / `#ffffff14`
/// (docs/platform-facts.md:1164, the probe's render), and WinUI 3's `ButtonBackground` is
/// `ControlFillColorDefault`, `#B3FFFFFF` light and `#0FFFFFFF` dark (docs/platform-facts.md:1164).
const TRANSLUCENT_BUTTON_FILL: [&str; 5] = [
    "macos-monterey/dark",
    "macos-tahoe/light",
    "macos-tahoe/dark",
    "windows-11/light",
    "windows-11/dark",
];

/// The [`TRANSLUCENT_BUTTON_FILL`] combinations whose pressed fill gpui derives
/// (`derive::active_color` of the translucent idle fill), with no platform
/// source: no source gives macOS 26's pressed fill (docs/platform-facts.md §2.3
/// has no pressed row, and its hover row, :1179, is ❓), so the layer is as
/// translucent as the fill it is drawn over.
const DERIVED_PRESSED_FILL: [&str; 2] = ["macos-tahoe/light", "macos-tahoe/dark"];

/// Every preset paints its buttons on an opaque fill, except the combinations
/// in [`TRANSLUCENT_BUTTON_FILL`].
///
/// Over an opaque fill `over` is `Hsla::blend`, which the connector called
/// before it composited source-over; only a translucent layer over a
/// translucent fill tells the two apart, so where that happens is asserted
/// rather than remembered. A declared combination's hover and pressed layers
/// must be opaque, so they composite to themselves. The one exception is a
/// [`DERIVED_PRESSED_FILL`] combination's pressed fill, which must still be
/// derived (no `button.active_background`) and translucent, so the entry fails
/// once it no longer describes one.
#[test]
fn button_fill_is_opaque_outside_translucent_button_fill() -> crate::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    for c in &combinations {
        let r = &c.resolved;
        let fill = native_button_fill(r);
        let declared = TRANSLUCENT_BUTTON_FILL.contains(&c.label().as_str());
        if declared != (fill.a < 1.0) {
            failures.push(format!(
                "{}: the button fill is {}, and TRANSLUCENT_BUTTON_FILL {} it",
                c.label(),
                show(fill),
                if declared { "lists" } else { "does not list" }
            ));
        }
        let derived_pressed = DERIVED_PRESSED_FILL.contains(&c.label().as_str());
        if derived_pressed && !declared {
            failures.push(format!(
                "{}: DERIVED_PRESSED_FILL lists it, and TRANSLUCENT_BUTTON_FILL does not",
                c.label()
            ));
        }
        if derived_pressed {
            let pressed = native_pressed_fill(r, c.is_dark);
            if r.button.active_background.is_some() || pressed.a >= 1.0 {
                failures.push(format!(
                    "{}: DERIVED_PRESSED_FILL lists it, yet the pressed fill is {} \
                     and the platform {} one",
                    c.label(),
                    show(pressed),
                    if r.button.active_background.is_some() {
                        "states"
                    } else {
                        "states no"
                    }
                ));
            }
        }
        if declared {
            let pressed =
                (!derived_pressed).then(|| ("the pressed fill", native_pressed_fill(r, c.is_dark)));
            let hover = (
                "button.hover_background",
                rgba_to_hsla(r.button.hover_background),
            );
            for (what, layer) in std::iter::once(hover).chain(pressed) {
                if layer.a < 1.0 {
                    failures.push(format!(
                        "{}: {what} is {}, a translucent layer over the translucent \
                         fill, which takes `over`'s general arm",
                        c.label(),
                        show(layer)
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} button-fill assumption(s) broken:\n{}",
        failures.len(),
        failures.join("\n")
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

/// Every one of the 139 `ThemeColor` fields is in exactly one of the contract
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
        139,
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
        "--- gpui coverage: {rows} contract rows + {} derived = {} of 139 fields ---",
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
    /// What the connector draws: out of the tokens it writes, or, for a
    /// surface a [`widgets`](crate::widgets) control draws, out of the
    /// resolved theme the control paints from.
    emitted: fn(&ThemeColor, &ResolvedTheme) -> (Hsla, Hsla, Hsla),
    /// Combinations -- `preset/mode`, because the two this release knows are
    /// one mode of their preset -- where the emitted pair is legitimately
    /// worse than the platform's, each with its reason.
    exceptions: &'static [(&'static str, &'static str)],
}

/// Upstream paints an idle tab `transparent` in every variant
/// (`gpui-component-0.7.3/src/tab/tab.rs:132, 143, 150, 155, 160`), so its
/// label lands on the bar, and `tokens.tab` has no reader to carry the
/// platform's idle-tab fill. Breeze fills an unselected tab with
/// `Window.darker(120)` (`docs/platform-facts.md:1333`, §2.11), which under
/// Breeze Dark (`#1b1d20` on the `#202326` window) lifts the label's contrast
/// above the bar's; the emitted pair stays above 15:1. Without the `widgets`
/// feature only: `widgets::TabBar` paints the idle fill.
#[cfg(not(feature = "widgets"))]
const IDLE_TAB_SURFACE: &str = "upstream paints an idle tab transparent \
                                (tab/tab.rs:132) and Breeze Dark's idle tab fill, \
                                Window.darker(120), is darker than the bar";

/// Where the `tab label` pair may be worse than the platform's.
#[cfg(not(feature = "widgets"))]
const IDLE_TAB_EXCEPTIONS: &[(&str, &str)] = &[("kde-breeze/dark", IDLE_TAB_SURFACE)];
#[cfg(feature = "widgets")]
const IDLE_TAB_EXCEPTIONS: &[(&str, &str)] = &[];

/// The idle tab label as the connector draws it: `widgets::TabBar`'s idle
/// tab on its bar.
#[cfg(feature = "widgets")]
fn idle_tab_drawn(tc: &ThemeColor, r: &ResolvedTheme) -> (Hsla, Hsla, Hsla) {
    crate::widgets::TabLook::of(r).map_or((tc.tab_foreground, tc.tab_bar, tc.tab_bar), |t| {
        (t.idle_text, t.idle, t.bar)
    })
}

/// The idle tab label as the connector draws it: gpui-component's, on the
/// bar.
#[cfg(not(feature = "widgets"))]
fn idle_tab_drawn(tc: &ThemeColor, _: &ResolvedTheme) -> (Hsla, Hsla, Hsla) {
    (tc.tab_foreground, tc.tab_bar, tc.tab_bar)
}

/// The selected tab label as the connector draws it: `widgets::TabBar`'s
/// selected tab on its bar.
#[cfg(feature = "widgets")]
fn active_tab_drawn(tc: &ThemeColor, r: &ResolvedTheme) -> (Hsla, Hsla, Hsla) {
    crate::widgets::TabLook::of(r)
        .map_or((tc.tab_active_foreground, tc.tab_active, tc.tab_bar), |t| {
            (t.active_text, t.active, t.bar)
        })
}

/// The selected tab label as the connector draws it: gpui-component's.
#[cfg(not(feature = "widgets"))]
fn active_tab_drawn(tc: &ThemeColor, _: &ResolvedTheme) -> (Hsla, Hsla, Hsla) {
    (tc.tab_active_foreground, tc.tab_active, tc.tab_bar)
}

/// The asserted pairs.
///
/// Every pair here and in `REPORTED` is built the same way: the emitted side
/// is what upstream really renders for that surface, out of the tokens this
/// connector writes, and the native side is the platform's own pair for the
/// same surface. Where the emitted surface is not the plain reading of its
/// tokens -- a label that lands on a fill other than its own, a fill upstream
/// composes itself -- the pair carries a comment with the upstream line; not
/// every pair can, because some tokens have many readers (`muted_foreground`
/// alone has 95 sites outside `theme/` in 0.7.3) and the surfaces
/// `INERT_SURFACES` names have no reader at all. The two need not read the
/// same native fields -- `hovered button label` keeps `button_foreground` on
/// `button_hover` where the platform states `hover_text_color` on
/// `hover_background` -- and where they differ is exactly where a degradation
/// could appear, which is why such a pair is asserted rather than excused.
///
/// A pair leaves this list only for the reason `REPORTED` states.
const PAIRS: &[Pair] = &[
    Pair {
        what: "window text on the window",
        native: |r, _| {
            let bg = native_window(r);
            (rgba_to_hsla(r.defaults.text_color), bg, bg)
        },
        emitted: |tc, _| (tc.foreground, tc.background, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "muted text on the window",
        native: |r, _| {
            let bg = native_window(r);
            (rgba_to_hsla(r.defaults.muted_color), bg, bg)
        },
        emitted: |tc, _| (tc.muted_foreground, tc.background, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "primary button label",
        native: |r, _| {
            (
                rgba_to_hsla(r.button.primary_text_color),
                rgba_to_hsla(r.button.primary_background),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.primary_foreground, tc.primary, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "ordinary button label",
        native: |r, _| {
            (
                rgba_to_hsla(r.button.font.color),
                native_button_fill(r),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.secondary_foreground, tc.secondary, tc.background),
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
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.button_foreground, tc.button_hover, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "pressed button label",
        native: |r, is_dark| {
            (
                rgba_to_hsla(r.button.active_text_color),
                over(native_pressed_fill(r, is_dark), native_button_fill(r)),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.button_foreground, tc.button_active, tc.background),
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
        emitted: |tc, _| (tc.accent_foreground, tc.accent, tc.popover),
        exceptions: &[],
    },
    Pair {
        what: "popover text",
        native: |r, _| {
            (
                rgba_to_hsla(r.popover.font.color),
                rgba_to_hsla(r.popover.background_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.popover_foreground, tc.popover, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "danger label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.danger_text_color),
                rgba_to_hsla(r.defaults.danger_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.danger_foreground, tc.danger, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "success label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.success_text_color),
                rgba_to_hsla(r.defaults.success_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.success_foreground, tc.success, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "warning label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.warning_text_color),
                rgba_to_hsla(r.defaults.warning_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.warning_foreground, tc.warning, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "info label",
        native: |r, _| {
            (
                rgba_to_hsla(r.defaults.info_text_color),
                rgba_to_hsla(r.defaults.info_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.info_foreground, tc.info, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "sidebar text",
        native: |r, _| {
            (
                rgba_to_hsla(r.sidebar.font.color),
                rgba_to_hsla(r.sidebar.background_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.sidebar_foreground, tc.sidebar, tc.background),
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
        emitted: |tc, _| (tc.sidebar_accent_foreground, tc.sidebar_accent, tc.sidebar),
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
        emitted: |tc, _| {
            (
                tc.sidebar_primary_foreground,
                tc.sidebar_primary,
                tc.sidebar,
            )
        },
        exceptions: &[],
    },
    // With the `widgets` feature a tab row is `widgets::TabBar`, and the
    // emitted side is what it paints (`TabLook`). Without it, gpui-component's
    // idle tab paints `transparent` in every variant (`tab/tab.rs:132, 143,
    // 150, 155, 160`), so its label lands on the bar -- a live surface
    // (`tab/tab_bar.rs:371`, `tab/tab.rs:309`), which is what the emitted
    // side measures then. The platform puts the same label on its own tab
    // fill, and the two surfaces differing is exactly what this pair is for.
    // `tokens.tab`, which nothing in 0.7.3 reads, keeps its row but takes no
    // part here.
    Pair {
        what: "tab label",
        native: |r, _| {
            (
                rgba_to_hsla(r.tab.font.color),
                rgba_to_hsla(r.tab.background_color),
                rgba_to_hsla(r.tab.bar_background),
            )
        },
        emitted: idle_tab_drawn,
        exceptions: IDLE_TAB_EXCEPTIONS,
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
        emitted: active_tab_drawn,
        exceptions: &[],
    },
    // The header row upstream paints: `table_head_foreground` on
    // `tokens.table_head` (`table/table.rs:199-200`,
    // `table/state.rs:1852-1853`), over the window behind the table.
    Pair {
        what: "table head text",
        native: |r, _| {
            (
                rgba_to_hsla(r.list.header_font.color),
                rgba_to_hsla(r.list.header_background),
                rgba_to_hsla(r.window.background_color),
            )
        },
        emitted: |tc, _| (tc.table_head_foreground, tc.table_head, tc.background),
        exceptions: &[],
    },
    // A list row sets `foreground` and no state changes it
    // (`list/list_item.rs:205`; the hover and selected branches set only a
    // fill, `:227-233, 257-265`), so the emitted foreground is the window's; for an idle and a hovered row the platform agrees.
    Pair {
        what: "list row text",
        native: |r, _| {
            let bg = rgba_to_hsla(r.list.background_color);
            (rgba_to_hsla(r.list.item_font.color), bg, bg)
        },
        emitted: |tc, _| (tc.foreground, tc.list, tc.background),
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
        emitted: |tc, _| (tc.foreground, tc.list_hover, tc.list),
        exceptions: &[],
    },
    Pair {
        what: "link on the window",
        native: |r, _| {
            let bg = native_window(r);
            (rgba_to_hsla(r.link.font.color), bg, bg)
        },
        emitted: |tc, _| (tc.link, tc.background, tc.background),
        exceptions: &[],
    },
    // A `Button::link` paints no fill in any state
    // (`button/button.rs:1193, 1270, 1310`) and takes `link_hover` and
    // `link_active` as its text, so both pairs are that colour on the window.
    Pair {
        what: "hovered link text",
        native: |r, _| {
            let bg = native_window(r);
            (rgba_to_hsla(r.link.hover_text_color), bg, bg)
        },
        emitted: |tc, _| (tc.link_hover, tc.background, tc.background),
        exceptions: &[],
    },
    Pair {
        what: "pressed link text",
        native: |r, _| {
            let bg = native_window(r);
            (rgba_to_hsla(r.link.active_text_color), bg, bg)
        },
        emitted: |tc, _| (tc.link_active, tc.background, tc.background),
        exceptions: &[],
    },
];

/// Geometry a theme states that gpui-component draws its own way where no
/// `geometry` refinement reaches, with the upstream lines. A documented limit:
/// no test can observe it, as gpui's test support exposes laid-out bounds
/// (`TestAppContext::debug_bounds`) and no painted scene (gpui-pre 0.3.9
/// `src/window.rs:1250`, `rendered_frame` is crate-private), so neither a
/// painted edge nor the ring outside it can be asserted. Recheck it on every
/// gpui-component bump.
#[expect(
    dead_code,
    reason = "a documented limit no test can observe: gpui exposes no painted scene"
)]
struct UpstreamLimit {
    what: &'static str,
    evidence: &'static str,
}

#[expect(
    dead_code,
    reason = "a documented limit no test can observe: gpui exposes no painted scene"
)]
const UPSTREAM_LIMITS: &[UpstreamLimit] = &[UpstreamLimit {
    what: "a focused text field's edge and focus ring",
    evidence: "gpui-component 0.7.3 draws a focused Input in its own way: \
               `border_1()` in the `ring` colour as a focus-state style \
               (`src/input/input.rs:758-762`), which a `StyleRefinement` such \
               as `geometry::input` cannot reach, so a field the theme leaves \
               without a line (adwaita's 0) gains a 1px edge and its text moves \
               in a pixel; and its own ring, `FOCUS_RING_WIDTH` 3 outside that \
               edge (`src/styled.rs:11`, `:257-262`, `:301-327`, called at \
               `src/input/input.rs:806`). It cannot draw the theme's inset ring \
               in a focus state. The 1px edge is not forced: \
               `Input::focus_bordered(false)` (`src/input/input.rs:311-314`, \
               also `FocusableExt::focus_ring(false)`, `:164-167`) turns off \
               both the edge (`:760`) and the ring (`:805`), but leaves a \
               focused field with no focus display at all; the connector keeps \
               upstream's display and the 1px edge with it",
}];

/// A pair whose two ratios are printed and not asserted, with the reason.
///
/// This is a pair-wide named exception, not a second kind of pair: both sides
/// are built exactly as `PAIRS`'s are. An entry belongs here only when the
/// emitted pair is worse than the platform's somewhere *and* the connector
/// cannot fix it, because `ThemeColor` has no token for the colour the
/// platform states -- Tier U material, with the upstream evidence in `why`.
///
/// Like the two combination-level exceptions, each entry is checked in both
/// directions: one that is no longer worse anywhere has stopped being an
/// exception and fails the test, which says to promote it to `PAIRS`. Each is
/// measured on every run and printed with both ratios and the count of
/// combinations where ours is the worse, so none of them can go quiet.
struct Reported {
    what: &'static str,
    why: &'static str,
    native: fn(&ResolvedTheme) -> (Hsla, Hsla, Hsla),
    emitted: fn(&ThemeColor, &ResolvedTheme) -> (Hsla, Hsla, Hsla),
}

/// Pairs measured and printed, never asserted.
const REPORTED: &[Reported] = &[
    Reported {
        what: "selected list row",
        why: "ThemeColor has no foreground for a selected row: upstream paints \
              list_active and keeps the window foreground \
              (list/list_item.rs:205, 259), while the platform pairs \
              list.selection_background with list.selection_text_color",
        native: |r| {
            (
                rgba_to_hsla(r.list.selection_text_color),
                rgba_to_hsla(r.list.selection_background),
                rgba_to_hsla(r.list.background_color),
            )
        },
        emitted: |tc, _| (tc.foreground, tc.list_active, tc.list),
    },
    Reported {
        what: "selected text",
        why: "ThemeColor has no foreground for selected text: `selection` is a \
              highlight upstream paints under text that keeps its own colour \
              (input/input.rs:562), while the platform pairs \
              input.selection_background with input.selection_text_color",
        native: |r| {
            (
                rgba_to_hsla(r.input.selection_text_color),
                rgba_to_hsla(r.input.selection_background),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.foreground, tc.selection, tc.background),
    },
    // Upstream paints a status bar's text with `muted_foreground`
    // (`status_bar.rs:95`) on `tokens.status_bar` (`:93`), and refines one line
    // later (`:96`), so `geometry::status_bar` -- which does carry
    // `status_bar.font.color` -- wins for an application that applies it. The
    // pair is measured over the tokens regardless, because that is what this
    // contract is about: an application that installs the theme and no builder
    // gets the ratios below.
    Reported {
        what: "status bar text",
        why: "ThemeColor has no status-bar foreground: upstream labels the bar \
              with muted_foreground (status_bar.rs:95), which the connector \
              feeds from defaults.muted_color, while the platform states \
              status_bar.font.color -- a colour no token carries, so it reaches \
              the bar only through geometry::status_bar, whose refinement \
              upstream applies after its own (:96)",
        native: |r| {
            (
                rgba_to_hsla(r.status_bar.font.color),
                rgba_to_hsla(r.status_bar.background_color),
                native_window(r),
            )
        },
        emitted: |tc, _| (tc.muted_foreground, tc.status_bar, tc.background),
    },
    // Upstream's title bar has no text colour of its own: its children
    // inherit, and the one thing it paints itself -- the window controls --
    // takes `foreground` on its own element (`title_bar.rs:219`), where a
    // refinement on the bar cannot reach it. Its fill is a gradient, not a
    // flat `title_bar` (`:22-36`, applied at `:342`), so the emitted side is
    // measured against whichever end is the worse of the two. The bar's own
    // label is not what is reported here: `geometry::title_bar` carries
    // `window.title_bar_font.color` onto the bar, and upstream sets nothing
    // there to displace it (`:331-346`).
    Reported {
        what: "title bar text",
        why: "ThemeColor has no title-bar foreground: upstream paints the \
              window controls with `foreground` set on their own element \
              (title_bar.rs:219), out of reach of a refinement on the bar, \
              over a gradient from a 55/45 mix of title_bar and background to \
              title_bar (:22-36, :342) -- measured at the worse end. The bar's \
              own label does reach the platform's colour: geometry::title_bar \
              carries window.title_bar_font.color and upstream sets nothing \
              there to displace it (:331-346)",
        native: |r| {
            (
                rgba_to_hsla(r.window.title_bar_font.color),
                rgba_to_hsla(r.window.title_bar_background),
                native_window(r),
            )
        },
        emitted: |tc, _| {
            (
                tc.foreground,
                worse_title_bar_end(tc.foreground, tc.title_bar, tc.background),
                tc.background,
            )
        },
    },
];

/// The end of upstream's title-bar gradient that contrasts worse with `fg`.
///
/// `TitleBar` fills itself with `default_title_bar_background(title_bar,
/// background)` (`title_bar.rs:342`), a vertical gradient from a 55/45 mix of
/// the two tokens to `title_bar` alone (`:22-36`). The mix is upstream's
/// arithmetic, reproduced here with its own factors so the reported ratio is
/// the one a user can actually read at the worse end of the bar.
fn worse_title_bar_end(fg: Hsla, title_bar: Hsla, background: Hsla) -> Hsla {
    let (t, b) = (gpui::Rgba::from(title_bar), gpui::Rgba::from(background));
    let mix = |t: f32, b: f32| (t * 0.55) + (b * 0.45);
    let mixed: Hsla = gpui::Rgba {
        r: mix(t.r, b.r),
        g: mix(t.g, b.g),
        b: mix(t.b, b.b),
        a: mix(t.a, b.a),
    }
    .into();
    if pair_ratio(fg, mixed, background) <= pair_ratio(fg, title_bar, background) {
        mixed
    } else {
        title_bar
    }
}

/// Asserted pairs whose emitted side rests on a token 0.7.3 paints nowhere.
///
/// The row that maps such a token stays -- it is the truth about the mapping
/// for the day upstream reads it -- but a contrast pair over it is a claim
/// about a surface nothing draws, so it is named here with the evidence,
/// printed, and left out of the coverage the summary counts. The names are
/// checked against the pair list, so a renamed pair cannot leave a note
/// pointing at nothing.
const INERT_SURFACES: &[(&str, &str)] = &[
    (
        "list row text",
        "nothing reads tokens.list: a ListItem paints no idle background \
         (list/list_item.rs:227-233, 257-275) and the token survives only as a \
         schema fallback (theme/schema.rs:971-972, 1006)",
    ),
    (
        "hovered list row",
        "the hover fill itself is painted (list/list_item.rs:231), but the \
         surface under it is tokens.list, which nothing paints",
    ),
    (
        "sidebar primary button label",
        "neither sidebar_primary nor sidebar_primary_foreground has a reader \
         outside theme/ in 0.7.3",
    ),
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
/// Not AA: a good many of the measured pairs sit below it -- the run prints
/// how many and which -- and they are not all errors: the `ios-13` preset states
/// `#34c759` with white text (`ios-13.toml:52-53`), about 2.2:1. A connector that
/// asserted AA would be asserting
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
            let (fg, bg, surface) = (pair.emitted)(&c.colors, &c.resolved);
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

    // The pairs that emit exactly the platform's own ratio in every
    // combination, and so cannot fail as the presets stand. The test is ratio
    // equality, not a structural property: a pair whose two sides are pinned
    // to the same native fields lands here by construction, and one whose
    // sides read *different* fields lands here too whenever the two ratios
    // come out equal in every combination -- `tab label`, with
    // `tab.background_color` on the native side and `tab.bar_background` on
    // the emitted one, was the case until kde-breeze stated Breeze's idle-tab
    // fill. Derived here rather than claimed, so the figure cannot drift from
    // the pairs.
    let unbiting: Vec<&'static str> = PAIRS
        .iter()
        .filter(|pair| {
            combinations.iter().all(|c| {
                let (native_fg, native_bg, native_surface) = (pair.native)(&c.resolved, c.is_dark);
                let (fg, bg, surface) = (pair.emitted)(&c.colors, &c.resolved);
                pair_ratio(fg, bg, surface) == pair_ratio(native_fg, native_bg, native_surface)
            })
        })
        .map(|pair| pair.what)
        .collect();
    let dangling: Vec<&str> = INERT_SURFACES
        .iter()
        .map(|(what, _)| *what)
        .filter(|what| !declared.contains(what))
        .collect();
    assert!(
        dangling.is_empty(),
        "{} inert-surface note(s) name a pair that is not in the list: {:?}",
        dangling.len(),
        dangling
    );
    let inert: Vec<String> = INERT_SURFACES
        .iter()
        .map(|(what, why)| format!("  {what} -- {why}"))
        .collect();

    // The pairs section 7 reports rather than asserts: printed with both
    // ratios, and with the count of combinations where ours is the worse, so
    // the record says how large each one is rather than only that it exists.
    // Each is a pair-wide exception, so it is checked the way the
    // combination-level ones are: one that is worse nowhere is no longer an
    // exception and belongs in `PAIRS`.
    let mut reported = Vec::new();
    for pair in REPORTED {
        let mut worse = 0usize;
        let mut lines = Vec::new();
        for c in &combinations {
            let (native_fg, native_bg, native_surface) = (pair.native)(&c.resolved);
            let native = pair_ratio(native_fg, native_bg, native_surface);
            let (fg, bg, surface) = (pair.emitted)(&c.colors, &c.resolved);
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
        if worse == 0 {
            stale.push(format!(
                "{} is reported rather than asserted ({}) yet is worse than \
                 the platform's own in 0 of {} combinations -- it is no longer \
                 an exception: move it from REPORTED to PAIRS",
                pair.what,
                pair.why,
                combinations.len()
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
        "--- gpui contrast: {} asserted pairs ({} of them over a token 0.7.3 \
         paints nowhere, not counted as coverage: {} surfaces covered) x {} \
         combinations = {} comparisons, {} below AA ---\n{}\n--- of the \
         asserted pairs, {} emit the platform's own ratio in all {} \
         combinations and so cannot fail as the presets stand -- a value \
         coincidence of today's {} presets, not a structural guarantee, and a \
         preset that states one of the two fields differently would make them \
         bite ({:?}); the other {} can bite ---\n--- inert surfaces \
         ---\n{}\n--- reported, not asserted ---\n{}",
        PAIRS.len(),
        inert.len(),
        PAIRS.len() - inert.len(),
        combinations.len(),
        ran.checks,
        below_aa.len(),
        below_aa.join("\n"),
        unbiting.len(),
        combinations.len(),
        crate::Theme::list_presets().len(),
        unbiting,
        PAIRS.len() - unbiting.len(),
        inert.join("\n"),
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

/// Every leaf of a serialised `ResolvedTheme`, under its dotted path, with its
/// value: the walk descends into objects and keeps everything else, a `null`
/// (an unstated soft option) included. iced's `leaf_values`
/// (`native-theme-iced/src/contract.rs`) is the same walk.
#[cfg(feature = "widgets")]
fn leaf_values(value: &serde_json::Value) -> Vec<(String, serde_json::Value)> {
    fn walk(prefix: &str, value: &serde_json::Value, out: &mut Vec<(String, serde_json::Value)>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, child) in map {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    walk(&path, child, out);
                }
            }
            _ => out.push((prefix.to_owned(), value.clone())),
        }
    }
    let mut out = Vec::new();
    walk("", value, &mut out);
    out
}

/// Why an `Unread::Fallback` entry does not hold, if it does not: in the first
/// base whose `option` is stated, `option` cleared, splicing `leaf` with the
/// base's sentinels must move a sink -- the fallback is drawn. `None` when it is.
#[cfg(feature = "widgets")]
fn fallback_unproved(
    leaf: &str,
    option: &str,
    combinations: &[Combination],
    jsons: &[serde_json::Value],
) -> Option<String> {
    let pointer = format!("/{}", option.replace('.', "/"));
    let Some((c, json)) = combinations
        .iter()
        .zip(jsons)
        .find(|(_, json)| json.pointer(&pointer).is_some_and(|v| !v.is_null()))
    else {
        return Some(format!(
            "UNREAD_COLOURS {leaf}: no base states its option {option}"
        ));
    };
    let mut cleared = json.clone();
    if let Some(value) = cleared.pointer_mut(&pointer) {
        *value = serde_json::Value::Null;
    }
    let Ok(base) = serde_json::from_value::<ResolvedTheme>(cleared.clone()) else {
        return Some(format!(
            "UNREAD_COLOURS {leaf}: {option} cleared does not deserialise"
        ));
    };
    let base_sinks = sinks(&base, c.is_dark);
    let (red, green, blue) = sentinel_rgb(&cleared);
    let reached = [
        format!("#{red:02x}{green:02x}{blue:02x}"),
        format!("#{red:02x}{green:02x}{blue:02x}80"),
    ]
    .iter()
    .filter_map(|hex| spliced_colour(&cleared, leaf, hex))
    .any(|r| !moved(&base_sinks, &sinks(&r, c.is_dark)).is_empty());
    (!reached).then(|| {
        format!(
            "UNREAD_COLOURS {leaf}: with {option} cleared it still moves no sink, \
             so it is not that option's fallback"
        )
    })
}

/// `json` with the leaf at `leaf` replaced by the colour `hex`, read back as a
/// theme; `None` where the leaf already holds it or does not take a colour.
#[cfg(feature = "widgets")]
fn spliced_colour(json: &serde_json::Value, leaf: &str, hex: &str) -> Option<ResolvedTheme> {
    let pointer = format!("/{}", leaf.replace('.', "/"));
    let value = serde_json::Value::String(hex.to_owned());
    if json.pointer(&pointer)? == &value {
        return None;
    }
    let mut b = json.clone();
    *b.pointer_mut(&pointer)? = value;
    serde_json::from_value(b).ok()
}

/// The colour leaves of one serialised theme: a leaf whose value is a colour,
/// or a `null` a colour splices into -- an `Option<Rgba>` the theme leaves
/// unstated.
#[cfg(feature = "widgets")]
fn colour_leaves(json: &serde_json::Value) -> Vec<String> {
    leaf_values(json)
        .into_iter()
        .filter(|(leaf, value)| match value {
            serde_json::Value::String(s) => s.starts_with('#') && s.parse::<Rgba>().is_ok(),
            serde_json::Value::Null => spliced_colour(json, leaf, "#123456").is_some(),
            _ => false,
        })
        .map(|(leaf, _)| leaf)
        .collect()
}

/// An opaque colour whose red, green and blue appear in no colour of `json`,
/// so that no sink can show it by passing another leaf through: counting up
/// from `#13579b` in the red channel.
#[cfg(feature = "widgets")]
fn sentinel_rgb(json: &serde_json::Value) -> (u8, u8, u8) {
    let taken: std::collections::BTreeSet<(u8, u8, u8)> = leaf_values(json)
        .into_iter()
        .filter_map(|(_, value)| value.as_str().and_then(|s| s.parse::<Rgba>().ok()))
        .map(|c| (c.r, c.g, c.b))
        .collect();
    (0x13..=u8::MAX)
        .map(|red| (red, 0x57, 0x9b))
        .find(|rgb| !taken.contains(rgb))
        .unwrap_or((0x13, 0x57, 0x9b))
}

/// A `geometry` builder that returns a refinement.
#[cfg(feature = "widgets")]
type Refinement = fn(crate::Native<'_>) -> gpui::StyleRefinement;

/// What the connector builds from one theme, without a window, as text keyed
/// by sink: every `ThemeColor` field `to_theme_color` writes, every field of
/// the `base_palette` `apply`'s observer copies back, every
/// `ScrollbarGeometry` field and the `ResizableTheme` the base layer
/// installs, every `geometry` builder that takes a `Native`, and each of the
/// `widgets` Looks in each of its states, as built and as drawn over the
/// default backdrop, `defaults.background_color`.
#[cfg(feature = "widgets")]
fn sinks(r: &ResolvedTheme, is_dark: bool) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    let mut fields = |prefix: &str, tc: ThemeColor| {
        if let Some(map) = serde_json::to_value(tc)
            .ok()
            .as_ref()
            .and_then(serde_json::Value::as_object)
        {
            for (field, value) in map {
                out.insert(format!("{prefix}.{field}"), value.to_string());
            }
        }
    };
    fields("ThemeColor", to_theme_color(r, is_dark, false));
    fields("base_palette", crate::colors::base_palette(r, is_dark));

    let g = crate::base_layer::scrollbar_geometry(r);
    let show = |v: &dyn std::fmt::Debug| format!("{v:?}");
    for (field, value) in [
        ("track_width", show(&g.track_width)),
        ("thumb_width", show(&g.thumb_width)),
        ("thumb_inset", show(&g.thumb_inset)),
        ("thumb_radius", show(&g.thumb_radius)),
        ("min_thumb_length", show(&g.min_thumb_length)),
        ("track", show(&g.track)),
        ("track_active_border", show(&g.track_active_border)),
        ("thumb", show(&g.thumb)),
        ("thumb_hover", show(&g.thumb_hover)),
        ("thumb_active", show(&g.thumb_active)),
    ] {
        out.insert(format!("ScrollbarGeometry.{field}"), value);
    }
    let resizable = crate::base_layer::resizable_theme(r);
    out.insert(
        "ResizableTheme.handle".to_owned(),
        format!("{:?}", resizable.handle),
    );
    out.insert(
        "ResizableTheme.active_handle".to_owned(),
        format!("{:?}", resizable.active_handle),
    );

    use crate::geometry as gm;
    let n = crate::Native::unscaled(r);
    let refinements: &[(&str, Refinement)] = &[
        ("button", gm::button),
        ("button_label", gm::button_label),
        ("toggle", gm::toggle),
        ("input", gm::input),
        ("text_area", gm::text_area),
        ("menu_item", gm::menu_item),
        ("list_item", gm::list_item),
        ("tooltip", gm::tooltip),
        ("tooltip_content", gm::tooltip_content),
        ("popover", gm::popover),
        ("status_bar", gm::status_bar),
        ("dialog", gm::dialog),
        ("input_group_button", gm::input_group_button),
        ("dialog_footer", gm::dialog_footer),
        ("dialog_title", gm::dialog_title),
        ("dialog_description", gm::dialog_description),
        ("list", gm::list),
        ("table", gm::table),
        ("progress", gm::progress),
        ("group_box_content", gm::group_box_content),
        ("accordion_title", gm::accordion_title),
        ("accordion", gm::accordion),
        ("accordion_hover", gm::accordion_hover),
        ("checkbox", gm::checkbox),
        ("radio", gm::radio),
        ("select", gm::select),
        ("combobox", gm::combobox),
        ("title_bar", gm::title_bar),
        ("title_bar_inactive", gm::title_bar_inactive),
        ("toolbar", gm::toolbar),
        ("scrollbar_gutter", gm::scrollbar_gutter),
        ("input_height", gm::input_height),
        ("button_disabled", gm::button_disabled),
        ("button_checked", gm::button_checked),
        ("list_item_selected", gm::list_item_selected),
        ("link", gm::link),
        ("input_fill(false)", |n| gm::input_fill(n, false)),
        ("input_fill(true)", |n| gm::input_fill(n, true)),
        ("combo_box_fill(false)", |n| gm::combo_box_fill(n, false)),
        ("combo_box_fill(true)", |n| gm::combo_box_fill(n, true)),
    ];
    for (name, builder) in refinements {
        out.insert(format!("geometry::{name}"), format!("{:?}", builder(n)));
    }
    for (name, value) in [
        ("dialog_entrance", show(&gm::dialog_entrance(n))),
        ("spinner_size", show(&gm::spinner_size(n))),
        ("data_table_size", show(&gm::data_table_size(n))),
        ("list_header_height", show(&gm::list_header_height(n))),
        ("icon_size_toolbar", show(&gm::icon_size_toolbar(n))),
        ("icon_size_small", show(&gm::icon_size_small(n))),
        ("icon_size_large", show(&gm::icon_size_large(n))),
        ("icon_size_dialog", show(&gm::icon_size_dialog(n))),
        ("icon_size_panel", show(&gm::icon_size_panel(n))),
        ("dialog_max_width", show(&gm::dialog_max_width(n))),
    ] {
        out.insert(format!("geometry::{name}"), value);
    }

    use crate::widgets::{
        CheckboxLook, ProgressBarLook, SeparatorLook, SliderLook, SpinnerLook, SwitchLook, TabLook,
    };
    let ground = rgba_to_hsla(r.defaults.background_color);
    for checked in [false, true] {
        for disabled in [false, true] {
            let state = format!("({checked}, {disabled})");
            let checkbox = CheckboxLook::of(r, checked, disabled);
            let radio = CheckboxLook::radio(r, checked, disabled);
            let switch = SwitchLook::of(r, checked, disabled);
            out.insert(format!("CheckboxLook::of{state}"), format!("{checkbox:?}"));
            out.insert(
                format!("CheckboxLook::of{state} drawn"),
                format!("{:?}", checkbox.map(|l| l.faded_over(ground))),
            );
            out.insert(format!("CheckboxLook::radio{state}"), format!("{radio:?}"));
            out.insert(
                format!("CheckboxLook::radio{state} drawn"),
                format!("{:?}", radio.map(|l| l.faded_over(ground))),
            );
            out.insert(format!("SwitchLook::of{state}"), format!("{switch:?}"));
            out.insert(
                format!("SwitchLook::of{state} drawn"),
                format!("{:?}", switch.map(|l| l.faded_over(ground))),
            );
        }
    }
    for disabled in [false, true] {
        let slider = SliderLook::of(r, disabled);
        out.insert(format!("SliderLook::of({disabled})"), format!("{slider:?}"));
        out.insert(
            format!("SliderLook::of({disabled}) drawn"),
            format!("{:?}", slider.map(|l| l.faded_over(ground))),
        );
    }
    out.insert(
        "ProgressBarLook::of".to_owned(),
        format!("{:?}", ProgressBarLook::of(r)),
    );
    out.insert(
        "SeparatorLook::of".to_owned(),
        format!("{:?}", SeparatorLook::of(r)),
    );
    out.insert(
        "SpinnerLook::of".to_owned(),
        format!("{:?}", SpinnerLook::of(r)),
    );
    out.insert("TabLook::of".to_owned(), format!("{:?}", TabLook::of(r)));
    out
}

/// The sinks whose text differs between two readings.
#[cfg(feature = "widgets")]
fn moved(
    before: &std::collections::BTreeMap<String, String>,
    after: &std::collections::BTreeMap<String, String>,
) -> Vec<String> {
    before
        .iter()
        .filter(|(name, value)| after.get(*name) != Some(*value))
        .map(|(name, _)| name.clone())
        .collect()
}

/// M1-T7-03, gpui's half: every colour leaf of `ResolvedTheme` reaches one of
/// the connector's sinks, or a table says why it does not.
///
/// The tripwire (`every_theme_color_field_has_a_declared_source`) walks what
/// the connector emits into `ThemeColor`, so it cannot see a model value
/// nothing reads. This walks the model instead, by iced's method
/// (`native-theme-iced/src/contract.rs`,
/// `every_colour_leaf_reaches_a_row_or_is_named`) over gpui's outputs
/// ([`sinks`]). Each of the 34 preset/mode combinations is a base, and every
/// colour leaf of the serialised theme -- a colour, or an unstated
/// `Option<Rgba>` a colour splices into -- is spliced in turn with the leaf's
/// value in another preset of the same mode and with an opaque and a
/// half-transparent sentinel no colour of the base shares its channels with,
/// so neither a passthrough of another leaf nor a composite under an opaque
/// layer hides the change. A leaf *reaches* a sink when the sink's text
/// moves, in at least one base: a soft option's fallback moves its sink only
/// where the option is unstated, which is why every base is read, and one
/// that no base leaves unstated is named by `UNREAD_COLOURS` as a fallback.
/// A leaf that reaches a sink is not spliced in the bases after.
///
/// Every leaf that reaches no sink must be named once, by `UNREAD_COLOURS` or
/// `NO_RECEIVER`. An entry of either that names a leaf a sink reads is stale,
/// and so is an `UNREAD_COLOURS` entry that names no colour leaf.
///
/// The Looks are sinks only with `widgets`, the feature that builds them, and
/// without it the leaves only they read would reach nothing; so the test runs
/// with it, as iced's runs with `iced_aw`.
#[cfg(feature = "widgets")]
#[test]
fn every_colour_leaf_reaches_a_sink_or_is_named() -> crate::Result<()> {
    use std::collections::{BTreeMap, BTreeSet};

    let started = std::time::Instant::now();
    let combinations = combinations()?;
    let jsons: Vec<serde_json::Value> = combinations
        .iter()
        .filter_map(|c| serde_json::to_value(&c.resolved).ok())
        .collect();
    assert_eq!(
        jsons.len(),
        combinations.len(),
        "every ResolvedTheme serialises"
    );

    let mut leaves: BTreeSet<String> = BTreeSet::new();
    for json in &jsons {
        leaves.extend(colour_leaves(json));
    }

    // The sinks each leaf first moved, in the first base it moved one in.
    let mut reached: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let n = combinations.len();
    for (i, (c, json)) in combinations.iter().zip(&jsons).enumerate() {
        let base = sinks(&c.resolved, c.is_dark);
        let donor = (1..n)
            .map(|k| (i + 2 * k) % n)
            .find(|&j| combinations.get(j).is_some_and(|d| d.is_dark == c.is_dark))
            .and_then(|j| jsons.get(j));
        let (red, green, blue) = sentinel_rgb(json);
        let sentinels = [
            format!("#{red:02x}{green:02x}{blue:02x}"),
            format!("#{red:02x}{green:02x}{blue:02x}80"),
        ];
        for leaf in &leaves {
            if reached.contains_key(leaf) {
                continue;
            }
            let pointer = format!("/{}", leaf.replace('.', "/"));
            let from_donor = donor
                .and_then(|d| d.pointer(&pointer))
                .and_then(|v| v.as_str())
                .map(str::to_owned);
            for hex in from_donor.iter().chain(&sentinels) {
                let Some(r) = spliced_colour(json, leaf, hex) else {
                    continue;
                };
                let moved = moved(&base, &sinks(&r, c.is_dark));
                if !moved.is_empty() {
                    reached.insert(leaf.clone(), moved);
                    break;
                }
            }
        }
    }

    let mut failures = Vec::new();
    let (mut by_sinks, mut by_unread, mut by_no_receiver) = (0, 0, 0);
    for leaf in &leaves {
        let unread = UNREAD_COLOURS.iter().filter(|e| e.leaf == leaf).count();
        let no_receiver = NO_RECEIVER.iter().filter(|u| u.field == leaf).count();
        match reached.get(leaf) {
            Some(moved) => {
                by_sinks += 1;
                if unread + no_receiver > 0 {
                    failures.push(format!(
                        "{leaf}: reaches {} yet is named unread ({unread} by \
                         UNREAD_COLOURS, {no_receiver} by NO_RECEIVER)",
                        moved.join(", ")
                    ));
                }
            }
            None => match (unread, no_receiver) {
                (1, 0) => by_unread += 1,
                (0, 1) => by_no_receiver += 1,
                (0, 0) => failures.push(format!(
                    "{leaf}: reaches no sink, and neither UNREAD_COLOURS nor \
                     NO_RECEIVER names it"
                )),
                _ => failures.push(format!(
                    "{leaf}: named {unread} time(s) by UNREAD_COLOURS and \
                     {no_receiver} by NO_RECEIVER"
                )),
            },
        }
    }

    for entry in UNREAD_COLOURS {
        if !leaves.contains(entry.leaf) {
            failures.push(format!(
                "UNREAD_COLOURS names {}, which is no colour leaf",
                entry.leaf
            ));
        }
        let (cited, needs) = match entry.why {
            Unread::NoReceiver(evidence) => (evidence.contains(".rs:"), "an upstream file.rs:line"),
            Unread::Inherited(evidence) => (
                evidence.contains("inheritance-rules.toml:"),
                "a docs/inheritance-rules.toml line",
            ),
            Unread::Fallback { evidence, .. } => {
                (evidence.contains(".rs:"), "the connector's file.rs:line")
            }
        };
        if !cited {
            failures.push(format!("UNREAD_COLOURS {}: needs {needs}", entry.leaf));
        }
        if let Unread::Fallback { option, .. } = entry.why
            && let Some(failure) = fallback_unproved(entry.leaf, option, &combinations, &jsons)
        {
            failures.push(failure);
        }
    }

    println!(
        "{} colour leaves: {by_sinks} reach a sink, {by_unread} named by \
         UNREAD_COLOURS, {by_no_receiver} by NO_RECEIVER; {:.2?}",
        leaves.len(),
        started.elapsed()
    );
    assert!(
        failures.is_empty(),
        "{} colour-leaf classification(s) do not hold:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
