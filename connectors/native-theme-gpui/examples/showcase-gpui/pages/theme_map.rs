//! The Theme Map page.

use gpui::{
    Context, Hsla, IntoElement, ParentElement, StyleRefinement, Styled, div, prelude::*, px,
};
use gpui_component::{ActiveTheme, v_flex};

use native_theme::theme::{ResolvedBorderSpec, ResolvedFontSpec};
use native_theme_gpui::{Native, geometry};

use crate::app::Showcase;
use crate::support::{NativeStyled, color_swatch, format_font_info, native_value, section};

/// What `geometry::control_height` computes for one widget of the installed
/// theme, with the inputs it took.
fn control_height_line(
    widget: &str,
    min_height: f32,
    font: &ResolvedFontSpec,
    border: &ResolvedBorderSpec,
    n: Native<'_>,
) -> String {
    let height = geometry::control_height(min_height, font, border, n);
    format!(
        "{widget}: {}px from geometry::control_height -- the larger of \
         {widget}.min_height {min_height}px and ceil({widget}.font.size {}px × text scale × \
         line_height {}) + 2 × {widget}.border.padding_vertical {}px",
        height.as_f32(),
        font.size,
        n.resolved.defaults.line_height,
        border.padding_vertical,
    )
}

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Theme Map
    // -----------------------------------------------------------------------
    pub(crate) fn render_theme_map_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let _fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        // Built once and shadowed onto the name the swatches below already
        // call, so every swatch sits in the showcase's one frame without the
        // theme being reached for a hundred and forty times.
        let swatch_frame = StyleRefinement::default().demo_frame(cx);
        let color_swatch = |name: &str, color: Hsla| color_swatch(name, color, &swatch_frame);
        // Derived, not stored: the height the connector gives a control at the
        // current text scale. Nothing is shown in its place before a native
        // theme is installed.
        let control_heights = [
            native_value(cx, |n| {
                let b = &n.resolved.button;
                control_height_line("button", b.min_height, &b.font, &b.border, n)
            }),
            native_value(cx, |n| {
                let i = &n.resolved.input;
                control_height_line("input", i.min_height, &i.font, &i.border, n)
            }),
        ]
        .map(|line| div().child(line.unwrap_or_else(|| "no native theme installed".into())));

        v_flex()
            .gap_4()
            .p_4()
            .flex_1()
            .child(section("All ThemeColor Fields"))
            // Core
            .child(section("Core"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("background", t.background))
                    .child(color_swatch("foreground", t.foreground))
                    .child(color_swatch("accent", t.accent))
                    .child(color_swatch("accent_foreground", t.accent_foreground))
                    .child(color_swatch("border", t.border))
                    .child(color_swatch("muted", t.muted))
                    .child(color_swatch("muted_foreground", t.muted_foreground))
                    .child(color_swatch("input", t.input))
                    .child(color_swatch("ring", t.ring))
                    .child(color_swatch("selection", t.selection))
                    .child(color_swatch("caret", t.caret))
                    .child(color_swatch("link", t.link))
                    .child(color_swatch("link_hover", t.link_hover))
                    .child(color_swatch("link_active", t.link_active))
                    .child(color_swatch("overlay", t.overlay)),
            )
            // Primary
            .child(section("Primary"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("primary", t.primary))
                    .child(color_swatch("primary_foreground", t.primary_foreground))
                    .child(color_swatch("primary_hover", t.primary_hover))
                    .child(color_swatch("primary_active", t.primary_active)),
            )
            // Secondary
            .child(section("Secondary"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("secondary", t.secondary))
                    .child(color_swatch("secondary_foreground", t.secondary_foreground))
                    .child(color_swatch("secondary_hover", t.secondary_hover))
                    .child(color_swatch("secondary_active", t.secondary_active)),
            )
            // Button (0.6.0): button*/button_secondary* ← secondary*, button_primary*
            // ← primary*, button_{danger,info,success,warning}* ← the status
            // fields (spec §6.2) — solid native surfaces, not upstream's tint.
            .child(section(
                "Button (28 fields, copies of secondary/primary/status)",
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("button", t.button))
                    .child(color_swatch("button_hover", t.button_hover))
                    .child(color_swatch("button_active", t.button_active))
                    .child(color_swatch("button_foreground", t.button_foreground))
                    .child(color_swatch("button_secondary", t.button_secondary))
                    .child(color_swatch(
                        "button_secondary_hover",
                        t.button_secondary_hover,
                    ))
                    .child(color_swatch(
                        "button_secondary_active",
                        t.button_secondary_active,
                    ))
                    .child(color_swatch(
                        "button_secondary_foreground",
                        t.button_secondary_foreground,
                    ))
                    .child(color_swatch("button_primary", t.button_primary))
                    .child(color_swatch("button_primary_hover", t.button_primary_hover))
                    .child(color_swatch(
                        "button_primary_active",
                        t.button_primary_active,
                    ))
                    .child(color_swatch(
                        "button_primary_foreground",
                        t.button_primary_foreground,
                    ))
                    .child(color_swatch("button_danger", t.button_danger))
                    .child(color_swatch("button_danger_hover", t.button_danger_hover))
                    .child(color_swatch("button_danger_active", t.button_danger_active))
                    .child(color_swatch(
                        "button_danger_foreground",
                        t.button_danger_foreground,
                    ))
                    .child(color_swatch("button_info", t.button_info))
                    .child(color_swatch("button_info_hover", t.button_info_hover))
                    .child(color_swatch("button_info_active", t.button_info_active))
                    .child(color_swatch(
                        "button_info_foreground",
                        t.button_info_foreground,
                    ))
                    .child(color_swatch("button_success", t.button_success))
                    .child(color_swatch("button_success_hover", t.button_success_hover))
                    .child(color_swatch(
                        "button_success_active",
                        t.button_success_active,
                    ))
                    .child(color_swatch(
                        "button_success_foreground",
                        t.button_success_foreground,
                    ))
                    .child(color_swatch("button_warning", t.button_warning))
                    .child(color_swatch("button_warning_hover", t.button_warning_hover))
                    .child(color_swatch(
                        "button_warning_active",
                        t.button_warning_active,
                    ))
                    .child(color_swatch(
                        "button_warning_foreground",
                        t.button_warning_foreground,
                    )),
            )
            // Danger
            .child(section("Danger"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("danger", t.danger))
                    .child(color_swatch("danger_foreground", t.danger_foreground))
                    .child(color_swatch("danger_hover", t.danger_hover))
                    .child(color_swatch("danger_active", t.danger_active))
                    .child(color_swatch("red", t.red))
                    .child(color_swatch("red_light", t.red_light)),
            )
            // Success
            .child(section("Success"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("success", t.success))
                    .child(color_swatch("success_foreground", t.success_foreground))
                    .child(color_swatch("success_hover", t.success_hover))
                    .child(color_swatch("success_active", t.success_active))
                    .child(color_swatch("green", t.green))
                    .child(color_swatch("green_light", t.green_light)),
            )
            // Warning
            .child(section("Warning"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("warning", t.warning))
                    .child(color_swatch("warning_foreground", t.warning_foreground))
                    .child(color_swatch("warning_hover", t.warning_hover))
                    .child(color_swatch("warning_active", t.warning_active))
                    .child(color_swatch("yellow", t.yellow))
                    .child(color_swatch("yellow_light", t.yellow_light)),
            )
            // Info
            .child(section("Info"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("info", t.info))
                    .child(color_swatch("info_foreground", t.info_foreground))
                    .child(color_swatch("info_hover", t.info_hover))
                    .child(color_swatch("info_active", t.info_active))
                    .child(color_swatch("blue", t.blue))
                    .child(color_swatch("blue_light", t.blue_light)),
            )
            // List
            .child(section("List"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("list", t.colors.list))
                    .child(color_swatch("list_active", t.list_active))
                    .child(color_swatch("list_active_border", t.list_active_border))
                    .child(color_swatch("list_even", t.list_even))
                    .child(color_swatch("list_head", t.list_head))
                    .child(color_swatch("list_hover", t.list_hover)),
            )
            // Table
            .child(section("Table"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("table", t.table))
                    .child(color_swatch("table_active", t.table_active))
                    .child(color_swatch("table_active_border", t.table_active_border))
                    .child(color_swatch("table_even", t.table_even))
                    .child(color_swatch("table_head", t.table_head))
                    .child(color_swatch(
                        "table_head_foreground",
                        t.table_head_foreground,
                    ))
                    // table_foot* mirror table_head* (spec §6.2 derivation)
                    .child(color_swatch("table_foot", t.table_foot))
                    .child(color_swatch(
                        "table_foot_foreground",
                        t.table_foot_foreground,
                    ))
                    .child(color_swatch("table_hover", t.table_hover))
                    .child(color_swatch("table_row_border", t.table_row_border)),
            )
            // Tab
            .child(section("Tab"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("tab", t.tab))
                    .child(color_swatch("tab_active", t.tab_active))
                    .child(color_swatch(
                        "tab_active_foreground",
                        t.tab_active_foreground,
                    ))
                    .child(color_swatch("tab_bar", t.tab_bar))
                    .child(color_swatch("tab_bar_segmented", t.tab_bar_segmented))
                    .child(color_swatch("tab_foreground", t.tab_foreground)),
            )
            // Sidebar
            .child(section("Sidebar"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("sidebar", t.sidebar))
                    .child(color_swatch("sidebar_foreground", t.sidebar_foreground))
                    .child(color_swatch("sidebar_accent", t.sidebar_accent))
                    .child(color_swatch(
                        "sidebar_accent_foreground",
                        t.sidebar_accent_foreground,
                    ))
                    .child(color_swatch("sidebar_border", t.sidebar_border))
                    .child(color_swatch("sidebar_primary", t.sidebar_primary))
                    .child(color_swatch(
                        "sidebar_primary_foreground",
                        t.sidebar_primary_foreground,
                    )),
            )
            // Scrollbar
            .child(section("Scrollbar"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("scrollbar", t.scrollbar))
                    .child(color_swatch("scrollbar_thumb", t.scrollbar_thumb))
                    .child(color_swatch(
                        "scrollbar_thumb_hover",
                        t.scrollbar_thumb_hover,
                    )),
            )
            // Accordion
            .child(section("Accordion"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("accordion", t.accordion)),
            )
            // GroupBox
            .child(section("GroupBox"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("group_box", t.group_box))
                    .child(color_swatch("group_box_foreground", t.group_box_foreground)),
            )
            // Chart
            .child(section("Chart"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("chart_1", t.chart_1))
                    .child(color_swatch("chart_2", t.chart_2))
                    .child(color_swatch("chart_3", t.chart_3))
                    .child(color_swatch("chart_4", t.chart_4))
                    .child(color_swatch("chart_5", t.chart_5))
                    .child(color_swatch("chart_bullish", t.chart_bullish))
                    .child(color_swatch("chart_bearish", t.chart_bearish)),
            )
            // Misc
            .child(section("Misc"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch(
                        "description_list_label",
                        t.description_list_label,
                    ))
                    .child(color_swatch(
                        "description_list_label_foreground",
                        t.description_list_label_foreground,
                    ))
                    .child(color_swatch("drag_border", t.drag_border))
                    .child(color_swatch("drop_target", t.drop_target))
                    .child(color_swatch("popover", t.popover))
                    .child(color_swatch("popover_foreground", t.popover_foreground))
                    .child(color_swatch("progress_bar", t.progress_bar))
                    .child(color_swatch("skeleton", t.skeleton))
                    .child(color_swatch("slider_bar", t.slider_bar))
                    .child(color_swatch("slider_thumb", t.slider_thumb))
                    .child(color_swatch("switch", t.switch))
                    .child(color_swatch("switch_thumb", t.switch_thumb))
                    // status_bar* ← status_bar.background_color / .border.color (spec §6.2)
                    .child(color_swatch("status_bar", t.status_bar))
                    .child(color_swatch("status_bar_border", t.status_bar_border))
                    .child(color_swatch("title_bar", t.title_bar))
                    .child(color_swatch("title_bar_border", t.title_bar_border))
                    .child(color_swatch("window_border", t.window_border)),
            )
            // Base colors
            .child(section("Base"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(px(16.0))
                    .gap_y(px(4.0))
                    .child(color_swatch("magenta", t.magenta))
                    .child(color_swatch("magenta_light", t.magenta_light))
                    .child(color_swatch("cyan", t.cyan))
                    .child(color_swatch("cyan_light", t.cyan_light)),
            )
            .child(section("Control height (derived by the connector)"))
            .child(v_flex().children(control_heights))
    }
}
