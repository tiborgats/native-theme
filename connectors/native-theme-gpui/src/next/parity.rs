//! Connector parity for the first slice of the 0.8.0 work
//! (docs/v0.8.0-plan.md, section 1, rule 2; M0, step 4): what the connector
//! hands to gpui, computed from the 0.7.2 model by the connector's public
//! mapping and from the 0.8.0 model by [`super`], is equal leaf by leaf, on
//! every leaf the slice (the globals and `button`, `checkbox`, `list`,
//! `scrollbar`, `tooltip`) feeds.
//!
//! Presets covered: kde-breeze, light and dark ([`PRESETS`]). The 0.7.2 side
//! is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), with no accessibility scaling; the
//! 0.8.0 side for the scheme's default context, whose text scale is 1.
//!
//! The form of comparison per toolkit type:
//!
//! - `gpui_component::Theme` has no `PartialEq`, and its `Serialize` skips
//!   `notification`, `motion` and `cursor_blink`; it is compared leaf by leaf,
//!   since the 0.8.0 side feeds only the slice's leaves, each leaf by the
//!   `PartialEq` of its own type, which every leaf compared here has: the
//!   `ThemeColor` fields (`Hsla`, equal when its four `f32` are, by
//!   `total_cmp`) and the `ThemeToken` gpui-component derives from each;
//!   `mode` (`ThemeMode`); the families (`SharedString`); the sizes and radii
//!   (`Pixels`); `shadow` and `focus_ring` (`bool`); `scrollbar_mode`
//!   (`ScrollbarMode`); and the `ThemeConfig` in the slot of the theme's
//!   mode, by its fields (`ThemeMode`, `Option` of `SharedString`, `f32`,
//!   `usize` and `bool`, its colours as hex strings). A difference is printed
//!   in the `Debug` form of both values.
//! - The scroll bar, which goes into gpui-base's global, a type with neither
//!   `Debug` nor `PartialEq`: through the connector's `ScrollbarGeometry`,
//!   field by field (`Pixels`, `Hsla`).
//! - The `geometry` builders: `StyleRefinement`, `gpui_component::Size` and
//!   `Option<Pixels>`, each by its `PartialEq`.
//! - The `variants` builders, whose `ButtonCustomVariant` has neither
//!   `Debug` nor `PartialEq` and needs an `App`: through the connector's own
//!   values that feed them, each by its `PartialEq` — `bordered_button` by
//!   `variants::hover_border` (`Option<Hsla>`), `default_button` by
//!   `variants::default_button_states` (`Option<DefaultButtonStates>`) and
//!   `ghost_button`, which reads only the theme, by the three `ThemeColor`
//!   fields it takes; the `ThemeColor` fields the first two take beside those
//!   are compared as colour leaves.
//!
//! A difference is allowed only as an entry of [`DIFFERENCES`], with its
//! reason, and is compared in the form its entry gives; an entry that no
//! longer differs fails. A leaf that compares only the connector's fallback
//! on a preset, the preset stating none of the field it reads, is named in
//! [`FALLBACK_ONLY`], which fails once the preset states the field. The list
//! of colour leaves ([`SLICE_SLOTS`]) is held against the contract tables:
//! every slot they name is in it, or in [`OUTSIDE_SLICE`] with the field
//! outside the slice it comes from, or in [`EXCLUDED`] with a reason. The
//! builders compared are held against the public functions of
//! `src/geometry.rs` and `src/variants.rs`: each is compared or in
//! [`BUILDERS_OUTSIDE_SLICE`] with a field outside the slice it reads.

use std::fmt::Debug;

use gpui::{Hsla, Length, SharedString, px};
use gpui_component::theme::{Theme as GpuiTheme, ThemeConfig, ThemeToken};
use native_theme::next::{Preset, ThemeContext};
use native_theme::theme::LayoutTheme;

use crate::{AccessibilityPreferences, ColorMode, Native, ResolvedTheme};

/// The presets the test covers: the 0.7.2 key and the 0.8.0 preset.
const PRESETS: &[(&str, Preset)] = &[("kde-breeze", Preset::KdeBreeze)];

/// One `ThemeColor` field the slice alone feeds: its colour, the token
/// gpui-component derives from it, and its hex string in the stored config,
/// where `ThemeConfigColors` has the field public.
struct Slot {
    name: &'static str,
    color: fn(&GpuiTheme) -> Hsla,
    token: fn(&GpuiTheme) -> ThemeToken,
    config: Option<fn(&ThemeConfig) -> Option<SharedString>>,
}

macro_rules! slot {
    ($slot:ident) => {
        Slot {
            name: stringify!($slot),
            color: |t| t.colors.$slot,
            token: |t| t.tokens.$slot,
            config: Some(|c| c.colors.$slot.clone()),
        }
    };
    // `ThemeConfigColors` keeps the 12 base-palette fields private
    // (`config::theme_color_to_config_colors`).
    ($slot:ident, base) => {
        Slot {
            name: stringify!($slot),
            color: |t| t.colors.$slot,
            token: |t| t.tokens.$slot,
            config: None,
        }
    };
}

/// The `ThemeColor` fields whose every source is a field of the slice
/// (`colors::assign_slice`).
const SLICE_SLOTS: &[Slot] = &[
    slot!(foreground),
    slot!(border),
    slot!(muted_foreground),
    slot!(ring),
    slot!(primary),
    slot!(primary_foreground),
    slot!(primary_active),
    slot!(sidebar_primary),
    slot!(sidebar_primary_foreground),
    slot!(secondary),
    slot!(secondary_foreground),
    slot!(secondary_hover),
    slot!(secondary_active),
    slot!(danger),
    slot!(danger_foreground),
    slot!(danger_active),
    slot!(success),
    slot!(success_foreground),
    slot!(success_active),
    slot!(warning),
    slot!(warning_foreground),
    slot!(warning_active),
    slot!(info),
    slot!(info_foreground),
    slot!(info_active),
    slot!(chart_bullish),
    slot!(chart_bearish),
    slot!(button),
    slot!(button_hover),
    slot!(button_active),
    slot!(button_foreground),
    slot!(button_secondary),
    slot!(button_secondary_hover),
    slot!(button_secondary_active),
    slot!(button_secondary_foreground),
    slot!(button_primary),
    slot!(button_primary_active),
    slot!(button_primary_foreground),
    slot!(button_danger),
    slot!(button_danger_active),
    slot!(button_danger_foreground),
    slot!(button_info),
    slot!(button_info_active),
    slot!(button_info_foreground),
    slot!(button_success),
    slot!(button_success_active),
    slot!(button_success_foreground),
    slot!(button_warning),
    slot!(button_warning_active),
    slot!(button_warning_foreground),
    slot!(list),
    slot!(list_hover),
    slot!(list_active),
    slot!(list_even),
    slot!(list_head),
    slot!(list_highlight_foreground),
    slot!(table),
    slot!(table_hover),
    slot!(table_active),
    slot!(table_even),
    slot!(table_head),
    slot!(table_head_foreground),
    slot!(table_row_border),
    slot!(table_foot_foreground),
    slot!(chart_1),
    slot!(chart_2),
    slot!(chart_3),
    slot!(chart_4),
    slot!(chart_5),
    slot!(group_box_foreground),
    slot!(description_list_label_foreground),
    slot!(overlay),
    slot!(scrollbar),
    slot!(scrollbar_thumb),
    slot!(scrollbar_thumb_hover),
    slot!(skeleton),
    slot!(drag_border),
    slot!(drop_target),
    slot!(red, base),
    slot!(green, base),
    slot!(blue, base),
    slot!(yellow, base),
    slot!(magenta, base),
    slot!(cyan, base),
];

/// The contract tables' slots with a source outside the slice, each with
/// that source: not mapped by the 0.8.0 side yet.
const OUTSIDE_SLICE: &[(&str, &str)] = &[
    ("background", "window.background_color"),
    ("accent", "menu.hover_background"),
    ("accent_foreground", "menu.hover_text_color"),
    ("input", "input.border.color"),
    ("selection", "input.selection_background"),
    ("link", "link.font.color"),
    ("link_hover", "link.hover_text_color"),
    ("link_active", "link.active_text_color"),
    ("chart_grid", "separator.line_color"),
    ("tab", "tab.background_color"),
    ("tab_active", "tab.active_background"),
    ("tab_active_foreground", "tab.active_text_color"),
    ("tab_bar", "tab.bar_background"),
    ("tab_bar_segmented", "segmented_control.background_color"),
    ("tab_foreground", "tab.font.color"),
    ("sidebar", "sidebar.background_color"),
    ("sidebar_foreground", "sidebar.font.color"),
    ("sidebar_accent", "sidebar.selection_background"),
    ("sidebar_accent_foreground", "sidebar.selection_text_color"),
    ("sidebar_border", "sidebar.border.color"),
    ("title_bar", "window.title_bar_background"),
    ("title_bar_border", "window.border.color"),
    ("window_border", "window.border.color"),
    ("popover", "popover.background_color"),
    ("popover_foreground", "popover.font.color"),
    ("accordion", "window.background_color"),
    ("slider_bar", "slider.fill_color"),
    ("slider_thumb", "slider.thumb_color"),
    ("switch", "switch.unchecked_background"),
    ("switch_thumb", "switch.thumb_background"),
    ("progress_bar", "progress_bar.fill_color"),
    ("caret", "input.caret_color"),
    ("status_bar", "status_bar.background_color"),
    ("status_bar_border", "status_bar.border.color"),
    ("muted", "window.background_color, under the foreground"),
    ("primary_hover", "window.background_color, under primary"),
    ("danger_hover", "window.background_color, under danger"),
    ("success_hover", "window.background_color, under success"),
    ("warning_hover", "window.background_color, under warning"),
    ("info_hover", "window.background_color, under info"),
    ("button_primary_hover", "a copy of primary_hover"),
    ("button_danger_hover", "a copy of danger_hover"),
    ("button_info_hover", "a copy of info_hover"),
    ("button_success_hover", "a copy of success_hover"),
    ("button_warning_hover", "a copy of warning_hover"),
    ("table_foot", "window.background_color"),
    (
        "list_active_border",
        "window.background_color, under primary",
    ),
    ("table_active_border", "a copy of list_active_border"),
    ("group_box", "window.background_color, under secondary"),
    (
        "description_list_label",
        "window.background_color, under the border",
    ),
    ("red_light", "window.background_color, under red"),
    ("green_light", "window.background_color, under green"),
    ("blue_light", "window.background_color, under blue"),
    ("yellow_light", "window.background_color, under yellow"),
    ("magenta_light", "window.background_color, under magenta"),
    ("cyan_light", "window.background_color, under cyan"),
];

/// The contract tables' slots whose sources are all in the slice and which
/// the test does not assert, each with the reason. None so far.
const EXCLUDED: &[(&str, &str)] = &[];

/// A listed difference (plan, section 1, rule 2): the leaf, the presets on
/// which it differs, and why.
struct Difference {
    leaf: &'static str,
    presets: &'static [&'static str],
    reason: &'static str,
}

/// The button height 0.7.2's kde-breeze states, `button.min_height` in
/// `native-theme/src/presets/kde-breeze.toml`, which [`DIFFERENCES`]' entry
/// names.
const KDE_BREEZE_BUTTON_HEIGHT_PX: f32 = 32.0;

/// The listed differences.
const DIFFERENCES: &[Difference] = &[Difference {
    leaf: "geometry::button",
    presets: &["kde-breeze"],
    reason: "0.7.2 states button.min_height 32, the height of a button at the default \
             font rather than a minimum (issue #4 Z1, \
             docs/issues/0004_kde-leftovers-rationale.md, sizes that are not minimums); \
             the 0.8.0 kde-breeze leaves button.body(NONE).min_size[1] unstated, so the \
             0.8.0 refinement sets no height (`size.height`, which 0.7.2 sets to 32 px \
             at a text scale of 1) and the button keeps gpui-component's own; every \
             other field of the refinement is equal",
}];

/// A leaf that, on the presets named, compares only the connector's own
/// fallback: the 0.7.2 field it reads is unset there, and the 0.8.0 home
/// unstated. The test asserts the field unset on each, so an entry fails, and
/// is to be removed, once such a preset states it.
struct FallbackOnly {
    leaf: &'static str,
    field: &'static str,
    presets: &'static [&'static str],
    unset: fn(&ResolvedTheme) -> bool,
}

/// The leaves compared on their fallback only.
const FALLBACK_ONLY: &[FallbackOnly] = &[
    FallbackOnly {
        leaf: "geometry::data_table_size",
        field: "list.row_height",
        presets: &["kde-breeze"],
        unset: |t| t.list.row_height.is_none(),
    },
    FallbackOnly {
        leaf: "geometry::list_header_height",
        field: "list.header_height",
        presets: &["kde-breeze"],
        unset: |t| t.list.header_height.is_none(),
    },
];

/// The public builders of `src/geometry.rs` and `src/variants.rs` that read a
/// field outside the slice, each with one such field: not mapped by the
/// 0.8.0 side yet.
const BUILDERS_OUTSIDE_SLICE: &[(&str, &str)] = &[
    ("geometry::input", "input.min_height"),
    ("geometry::text_area", "text_area.border.padding"),
    ("geometry::menu_item", "menu.row_height"),
    ("geometry::popover", "popover.border.padding"),
    ("geometry::status_bar", "status_bar.font"),
    ("geometry::dialog", "dialog.min_width"),
    ("geometry::dialog_entrance", "dialog.entrance"),
    ("geometry::dialog_footer", "dialog.button_gap"),
    ("geometry::dialog_title", "dialog.title_font"),
    ("geometry::dialog_description", "dialog.body_font"),
    ("geometry::progress", "progress_bar.track_height"),
    ("geometry::group_box_content", "card.border.padding"),
    ("geometry::accordion_title", "expander.header_height"),
    ("geometry::accordion", "expander.border.color"),
    ("geometry::accordion_hover", "expander.hover_background"),
    ("geometry::select", "combo_box.font"),
    ("geometry::combobox", "combo_box.font"),
    ("geometry::title_bar", "window.title_bar_font"),
    (
        "geometry::title_bar_inactive",
        "window.inactive_title_bar_background",
    ),
    ("geometry::toolbar", "toolbar.font"),
    ("geometry::spinner_size", "spinner.diameter"),
    ("geometry::icon_size_toolbar", "toolbar.icon_size"),
    ("geometry::dialog_max_width", "dialog.max_width"),
    ("geometry::input_height", "input.min_height"),
    ("geometry::input_fill", "input.background_color"),
    ("geometry::combo_box_fill", "combo_box.background_color"),
    (
        "geometry::toolbar_button_rest",
        "toolbar.button_hover_border_color",
    ),
    (
        "geometry::toolbar_button_checked",
        "toolbar.button_checked_background",
    ),
    ("geometry::link", "link.font"),
    ("variants::switch_color", "switch.checked_background"),
    (
        "variants::toolbar_button",
        "toolbar.button_hover_background",
    ),
];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    /// The leaves compared or found unmapped, by name.
    seen: Vec<String>,
    failures: Vec<String>,
}

impl Report {
    fn new(preset: &'static str, is_dark: bool) -> Self {
        let scheme = if is_dark { "dark" } else { "light" };
        Self {
            label: format!("{preset}/{scheme}"),
            preset,
            compared: 0,
            seen: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// Count `leaf` as compared.
    fn count(&mut self, leaf: &str) {
        self.compared += 1;
        self.seen.push(leaf.to_owned());
    }

    /// Compare one leaf; a listed difference fails here, being compared in
    /// its own form by the caller.
    fn leaf<T: PartialEq + Debug>(&mut self, leaf: &str, old: T, new: T) {
        self.count(leaf);
        if old != new {
            self.failures.push(format!(
                "{}: {leaf}: 0.7.2 {old:?}, 0.8.0 {new:?}",
                self.label
            ));
        }
    }

    /// A leaf of the 0.8.0 side that is unmapped is a failure too.
    fn mapped<T>(&mut self, leaf: &str, new: super::Mapped<T>) -> Option<T> {
        match new {
            Ok(v) => Some(v),
            Err(super::Unmapped(home)) => {
                self.seen.push(leaf.to_owned());
                self.failures
                    .push(format!("{}: {leaf}: 0.8.0 lacks {home}", self.label));
                None
            }
        }
    }

    fn listed(&self, leaf: &str) -> Option<&'static Difference> {
        DIFFERENCES
            .iter()
            .find(|d| d.leaf == leaf && d.presets.contains(&self.preset))
    }
}

/// The two resolved themes of `preset` for `is_dark`, and 0.7.2's layout,
/// which its theme holds beside the variants.
fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(
    ResolvedTheme,
    LayoutTheme,
    native_theme::next::ResolvedTheme,
)> {
    let (mode, ctx) = if is_dark {
        (ColorMode::Dark, ThemeContext::dark())
    } else {
        (ColorMode::Light, ThemeContext::light())
    };
    let theme = crate::Theme::preset(key)?;
    let layout = theme.layout.clone();
    let old = theme
        .into_variant(mode)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, layout, new))
}

/// The theme's leaves the slice feeds: the colours, their tokens and
/// config strings, the theme's values and its config's.
fn compare_theme(
    report: &mut Report,
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
    is_dark: bool,
) {
    let old_theme = crate::to_theme(old, "", is_dark, &AccessibilityPreferences::DEFAULT);
    let Some(new_theme) = report.mapped("to_theme", super::to_theme(new)) else {
        return;
    };
    let config = |t: &GpuiTheme| {
        if is_dark {
            t.dark_theme.clone()
        } else {
            t.light_theme.clone()
        }
    };
    let (old_config, new_config) = (config(&old_theme), config(&new_theme));
    for slot in SLICE_SLOTS {
        report.leaf(
            &format!("colors.{}", slot.name),
            (slot.color)(&old_theme),
            (slot.color)(&new_theme),
        );
        report.leaf(
            &format!("tokens.{}", slot.name),
            (slot.token)(&old_theme),
            (slot.token)(&new_theme),
        );
        if let Some(get) = slot.config {
            report.leaf(
                &format!("config.colors.{}", slot.name),
                get(&old_config),
                get(&new_config),
            );
        }
    }
    let (o, n) = (&old_theme, &new_theme);
    report.leaf("mode", o.mode, n.mode);
    report.leaf("font_family", &o.font_family, &n.font_family);
    report.leaf("font_size", o.font_size, n.font_size);
    report.leaf("mono_font_family", &o.mono_font_family, &n.mono_font_family);
    report.leaf("mono_font_size", o.mono_font_size, n.mono_font_size);
    report.leaf("radius", o.radius, n.radius);
    report.leaf("radius_lg", o.radius_lg, n.radius_lg);
    report.leaf("shadow", o.shadow, n.shadow);
    report.leaf("focus_ring", o.focus_ring, n.focus_ring);
    report.leaf("scrollbar_mode", o.scrollbar_mode, n.scrollbar_mode);
    // `variants::ghost_button` reads these three fields and nothing else.
    let ghost = |t: &GpuiTheme| {
        (
            t.colors.secondary_foreground,
            t.colors.secondary_hover,
            t.colors.secondary_active,
        )
    };
    report.leaf("variants::ghost_button", ghost(o), ghost(n));
    let (o, n) = (&old_config, &new_config);
    report.leaf("config.mode", o.mode, n.mode);
    report.leaf("config.font_family", &o.font_family, &n.font_family);
    report.leaf("config.font_size", o.font_size, n.font_size);
    report.leaf(
        "config.mono_font_family",
        &o.mono_font_family,
        &n.mono_font_family,
    );
    report.leaf("config.mono_font_size", o.mono_font_size, n.mono_font_size);
    report.leaf("config.radius", o.radius, n.radius);
    report.leaf("config.radius_lg", o.radius_lg, n.radius_lg);
    report.leaf("config.shadow", o.shadow, n.shadow);
}

/// The scroll bar's geometry, field by field.
fn compare_scrollbar(
    report: &mut Report,
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    let o = crate::base_layer::scrollbar_geometry(old);
    let Some(n) = report.mapped("ScrollbarGeometry", super::scrollbar_geometry(new)) else {
        return;
    };
    report.leaf(
        "ScrollbarGeometry.track_width",
        o.track_width,
        n.track_width,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_width",
        o.thumb_width,
        n.thumb_width,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_inset",
        o.thumb_inset,
        n.thumb_inset,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_radius",
        o.thumb_radius,
        n.thumb_radius,
    );
    report.leaf(
        "ScrollbarGeometry.min_thumb_length",
        o.min_thumb_length,
        n.min_thumb_length,
    );
    report.leaf("ScrollbarGeometry.track", o.track, n.track);
    report.leaf(
        "ScrollbarGeometry.track_active_border",
        o.track_active_border,
        n.track_active_border,
    );
    report.leaf("ScrollbarGeometry.thumb", o.thumb, n.thumb);
    report.leaf(
        "ScrollbarGeometry.thumb_hover",
        o.thumb_hover,
        n.thumb_hover,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_active",
        o.thumb_active,
        n.thumb_active,
    );
}

/// Compare a leaf's two values, the 0.8.0 one where mapped.
fn compare_mapped<T: PartialEq + Debug>(
    report: &mut Report,
    leaf: &str,
    old: T,
    new: super::Mapped<T>,
) {
    if let Some(n) = report.mapped(leaf, new) {
        report.leaf(leaf, old, n);
    }
}

/// Compare a builder's two outputs.
macro_rules! builder {
    ($report:expr, $old:expr, $new:expr, $name:ident) => {
        compare_mapped(
            $report,
            concat!("geometry::", stringify!($name)),
            crate::geometry::$name(Native::unscaled($old)),
            super::builders::$name($new),
        )
    };
}

/// Compare a layout metric's two values.
macro_rules! layout_metric {
    ($report:expr, $layout:expr, $new:expr, $name:ident) => {
        compare_mapped(
            $report,
            concat!("geometry::", stringify!($name)),
            crate::geometry::$name($layout),
            super::builders::$name($new),
        )
    };
}

/// The `geometry` builders of the slice's widgets, and the `variants`
/// builders through the values the connector feeds them with.
fn compare_builders(
    report: &mut Report,
    old: &ResolvedTheme,
    layout: &LayoutTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    let button = "geometry::button";
    if let Some(n) = report.mapped(button, super::builders::button(new)) {
        let o = crate::geometry::button(Native::unscaled(old));
        match report.listed(button) {
            Some(listed) => {
                report.count(button);
                let mut corrected = o.clone();
                corrected.size.height = None;
                let stated = Some(Length::from(px(KDE_BREEZE_BUTTON_HEIGHT_PX)));
                if o.size.height != stated || n.size.height.is_some() || corrected != n {
                    report.failures.push(format!(
                        "{}: {button}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
            None => report.leaf(button, o, n),
        }
    }
    builder!(report, old, new, button_label);
    builder!(report, old, new, toggle);
    builder!(report, old, new, button_disabled);
    builder!(report, old, new, button_checked);
    builder!(report, old, new, default_button_rest);
    builder!(report, old, new, checkbox);
    builder!(report, old, new, radio);
    builder!(report, old, new, list);
    builder!(report, old, new, table);
    builder!(report, old, new, list_item_selected);
    builder!(report, old, new, data_table_size);
    builder!(report, old, new, list_header_height);
    builder!(report, old, new, tooltip);
    builder!(report, old, new, tooltip_content);
    builder!(report, old, new, icon_size_small);
    builder!(report, old, new, icon_size_large);
    builder!(report, old, new, icon_size_dialog);
    builder!(report, old, new, icon_size_panel);
    builder!(report, old, new, scrollbar_gutter);
    builder!(report, old, new, input_group_button);
    builder!(report, old, new, editor);
    builder!(report, old, new, list_item);
    layout_metric!(report, layout, new, widget_gap);
    layout_metric!(report, layout, new, container_margin);
    layout_metric!(report, layout, new, window_margin);
    layout_metric!(report, layout, new, section_gap);
    compare_mapped(
        report,
        "variants::bordered_button",
        crate::variants::hover_border(Native::unscaled(old)),
        super::builders::hover_border(new),
    );
    compare_mapped(
        report,
        "variants::default_button",
        crate::variants::default_button_states(Native::unscaled(old)),
        super::builders::default_button_states(new),
    );
}

/// Every leaf of `preset` for `is_dark`, compared.
fn compare_all(key: &'static str, preset: Preset, is_dark: bool) -> crate::Result<Report> {
    let (old, layout, new) = resolve(key, preset, is_dark)?;
    let mut report = Report::new(key, is_dark);
    compare_theme(&mut report, &old, &new, is_dark);
    compare_scrollbar(&mut report, &old, &new);
    compare_builders(&mut report, &old, &layout, &new);
    for entry in FALLBACK_ONLY {
        if entry.presets.contains(&key) && !(entry.unset)(&old) {
            report.failures.push(format!(
                "{}: {}: {} is stated, so the leaf no longer compares only the fallback: \
                 remove the preset from its FALLBACK_ONLY entry",
                report.label, entry.leaf, entry.field
            ));
        }
    }
    Ok(report)
}

#[test]
fn the_slice_maps_to_the_same_leaves_from_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let report = compare_all(key, preset, is_dark)?;
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
        }
    }
    assert!(
        failures.is_empty(),
        "{} leaves differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// The names of the top-level `pub fn`s of the source `src`, each after
/// `module::`.
fn public_fns(module: &str, src: &str) -> Vec<String> {
    src.lines()
        .filter_map(|line| line.strip_prefix("pub fn "))
        .filter_map(|rest| rest.split(['(', '<']).next())
        .map(|name| format!("{module}::{name}"))
        .collect()
}

#[test]
fn every_builder_is_compared_or_accounted_for() -> crate::Result<()> {
    let public: Vec<String> = public_fns("geometry", include_str!("../geometry.rs"))
        .into_iter()
        .chain(public_fns("variants", include_str!("../variants.rs")))
        .collect();
    let mut compared = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            compared.extend(compare_all(key, preset, is_dark)?.seen);
        }
    }
    let outside = |b: &str| BUILDERS_OUTSIDE_SLICE.iter().any(|(name, _)| *name == b);
    let missing: Vec<&String> = public
        .iter()
        .filter(|b| !compared.contains(b) && !outside(b))
        .collect();
    let both: Vec<&String> = public
        .iter()
        .filter(|b| compared.contains(b) && outside(b))
        .collect();
    let unknown: Vec<&str> = BUILDERS_OUTSIDE_SLICE
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| !public.iter().any(|b| b == name))
        .collect();
    assert!(
        !public.is_empty() && missing.is_empty() && both.is_empty() && unknown.is_empty(),
        "public builders neither compared nor in BUILDERS_OUTSIDE_SLICE: {missing:?}; \
         compared and in BUILDERS_OUTSIDE_SLICE: {both:?}; in BUILDERS_OUTSIDE_SLICE but \
         no public builder: {unknown:?}"
    );
    Ok(())
}

#[test]
fn every_fallback_only_entry_names_a_compared_leaf_and_a_covered_preset() -> crate::Result<()> {
    for entry in FALLBACK_ONLY {
        for preset in entry.presets {
            let covered = PRESETS.iter().find(|(key, _)| key == preset);
            assert!(
                covered.is_some(),
                "{}: {preset} is not a preset the test covers",
                entry.leaf
            );
            let Some(&(key, p)) = covered else {
                continue;
            };
            let report = compare_all(key, p, false)?;
            assert!(
                report.seen.iter().any(|leaf| leaf == entry.leaf),
                "{}: not a leaf the test compares",
                entry.leaf
            );
        }
    }
    Ok(())
}

#[test]
fn every_contract_slot_is_compared_or_accounted_for() {
    let named: Vec<&str> = SLICE_SLOTS
        .iter()
        .map(|s| s.name)
        .chain(OUTSIDE_SLICE.iter().map(|(slot, _)| *slot))
        .chain(EXCLUDED.iter().map(|(slot, _)| *slot))
        .collect();
    let tables = crate::contract::table_slots();
    let missing: Vec<&&str> = tables.iter().filter(|s| !named.contains(s)).collect();
    let unknown: Vec<&&str> = named.iter().filter(|s| !tables.contains(s)).collect();
    let mut twice: Vec<&&str> = named
        .iter()
        .enumerate()
        .filter(|(i, s)| named.iter().skip(i + 1).any(|t| t == *s))
        .map(|(_, s)| s)
        .collect();
    twice.dedup();
    assert!(
        missing.is_empty() && unknown.is_empty() && twice.is_empty(),
        "contract slots in no list: {missing:?}; listed slots the contract tables do not \
         name: {unknown:?}; slots in more than one list: {twice:?}"
    );
}

#[test]
fn every_listed_difference_names_a_covered_preset() {
    for d in DIFFERENCES {
        for preset in d.presets {
            assert!(
                PRESETS.iter().any(|(key, _)| key == preset),
                "{}: {preset} is not a preset the test covers",
                d.leaf
            );
        }
    }
}
