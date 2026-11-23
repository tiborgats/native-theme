//! The `geometry` builders of the slice's widgets, and the values the
//! connector feeds the slice's `variants` with, from the 0.8.0 model: each
//! reads what its 0.7.2 builder reads, at its home in the table of homes, and
//! hands it to the function that builder hands its own reads to.
//!
//! The text sizes are the model's, which the theme's text scale has already
//! scaled; the control-height rule takes that scale from the theme's
//! context, where the 0.7.2 builders take it from the accessibility
//! preferences.

use gpui::{Hsla, Pixels, StyleRefinement, px};
use gpui_component::Size;
use gpui_component::dialog::DialogEntrance;
use native_theme::next::{
    DialogEntranceStyle, IconStated, PartStated, PartStyle, ResolvedTheme, TextStated, WidgetState,
};

use super::{
    Mapped, Unmapped, fill, font, hsla, normal_window, padding, radius, scrollbar_geometry,
    state_fill, state_stroke_color, state_text_color, stated, stroke_color, stroke_width,
    text_color, text_size, uncoloured_font,
};
use crate::geometry;
use crate::variants::{DefaultButtonStates, ToolbarButtonStates};

/// `defaults.line_height`'s home.
fn line_height(r: &ResolvedTheme) -> f32 {
    r.typography.line_height_multiplier
}

/// [`crate::geometry::button`]. `button.min_height` is the body's minimum
/// height, which a theme may leave unstated (issue #4, Z1); the button then
/// keeps the toolkit's own height, as the builder does for every control
/// whose height the theme does not state.
pub(crate) fn button(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let body = r.button.body(WidgetState::NONE);
    let label = r.button.label(WidgetState::NONE);
    if !label.stated.contains(TextStated::WEIGHT) {
        return Err(Unmapped("button.label(NONE).weight"));
    }
    Ok(geometry::button_from(&geometry::ButtonFrame {
        line_height: line_height(r),
        text_scale: r.context.text_scale,
        min_width: stated(body.min_size[0], "button.body(NONE).min_size[0]")?,
        min_height: body.min_size[1],
        padding: padding(&body),
        radius: radius(&body, "button.body(NONE).corners")?,
        border_width: stroke_width(&body, "button.body(NONE).stroke.width")?,
        border_color: stroke_color(&body, "button.body(NONE).stroke.color")?,
        weight: label.weight,
    }))
}

/// [`crate::geometry::button_label`].
pub(crate) fn button_label(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let label = r.button.label(WidgetState::NONE);
    Ok(geometry::text(
        StyleRefinement::default(),
        &uncoloured_font(&label, "button.label(NONE)")?,
        line_height(r),
    ))
}

/// [`crate::geometry::toggle`].
pub(crate) fn toggle(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    button_label(r)
}

/// [`crate::geometry::button_disabled`].
pub(crate) fn button_disabled(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let b = &r.button;
    Ok(geometry::button_disabled_from(
        Some(fill(
            &b.body(WidgetState::DISABLED),
            "button.body(DISABLED).fill",
        )?),
        fill(&b.body(WidgetState::NONE), "button.body(NONE).fill")?,
        text_color(
            &b.label(WidgetState::DISABLED),
            "button.label(DISABLED).color",
        )?,
        stated(b.disabled_opacity, "button.disabled_opacity")?,
    ))
}

/// [`crate::geometry::button_checked`]: `button.checked_text_color`,
/// `button.checked_background` and `button.active_background`, soft options,
/// where a rule states them for their state (module doc of `next`), else
/// unset, so the pressed pair stands in as on the 0.7.2 path.
pub(crate) fn button_checked(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let b = &r.button;
    Ok(geometry::button_checked_from(
        state_text_color(
            &b.label(WidgetState::CHECKED),
            "button.label(CHECKED).color",
        )?,
        text_color(
            &b.label(WidgetState::PRESSED),
            "button.label(PRESSED).color",
        )?,
        state_fill(&b.body(WidgetState::CHECKED), "button.body(CHECKED).fill")?,
        state_fill(&b.body(WidgetState::PRESSED), "button.body(PRESSED).fill")?,
    ))
}

/// [`crate::geometry::default_button_rest`].
pub(crate) fn default_button_rest(r: &ResolvedTheme) -> Mapped<Option<StyleRefinement>> {
    let b = &r.button;
    let default = b.body(WidgetState::DEFAULT);
    Ok(geometry::default_button_rest_from(
        state_fill(&default, "button.body(DEFAULT).fill")?,
        state_stroke_color(&default, "button.body(DEFAULT).stroke.color")?,
        stroke_color(&b.body(WidgetState::NONE), "button.body(NONE).stroke.color")?,
    ))
}

/// [`crate::geometry::checkbox`].
pub(crate) fn checkbox(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let label = r.checkbox.label(WidgetState::NONE);
    Ok(geometry::text(
        geometry::label_gap(r.checkbox.label_gap),
        &uncoloured_font(&label, "checkbox.label(NONE)")?,
        line_height(r),
    ))
}

/// [`crate::geometry::radio`]: the 0.7.2 builder reads `checkbox.*`, which
/// serves the checkbox and the radio, so this reads the checkbox's homes too.
pub(crate) fn radio(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let label = r.checkbox.label(WidgetState::NONE);
    Ok(geometry::coloured_text(
        geometry::label_gap(r.checkbox.label_gap),
        &font(&label, "checkbox.label(NONE)")?,
        line_height(r),
    ))
}

/// [`crate::geometry::list`].
pub(crate) fn list(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let container = r.list.container(WidgetState::NONE);
    Ok(geometry::list_from(
        stroke_width(&container, "list.container(NONE).stroke.width")?,
        stroke_color(&container, "list.container(NONE).stroke.color")?,
        radius(&container, "list.container(NONE).corners")?,
        fill(&container, "list.container(NONE).fill")?,
    ))
}

/// [`crate::geometry::table`].
pub(crate) fn table(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let label = r.list.item_label(WidgetState::NONE);
    Ok(geometry::text(
        StyleRefinement::default(),
        &uncoloured_font(&label, "list.item_label(NONE)")?,
        line_height(r),
    ))
}

/// The row's inset that 0.7.2's `list.border.padding` holds: per side, the
/// highlight's stated margin plus the row's stated padding, an unstated one
/// counting as none, and unset where neither is stated on that side. The
/// table of homes places the field at the highlight's margin on kde-breeze
/// (Breeze's highlight margins, spec §5, L4) and at the row's padding
/// elsewhere, so the sum is 0.7.2's value on every preset without naming
/// one; whether a margin is drawn as a margin rather than as padding is
/// decided with captures in a later milestone.
fn row_inset(row: &PartStyle, highlight: &PartStyle) -> [Option<f32>; 4] {
    let side = |bit: PartStated, v: f32| highlight.stated.contains(bit).then_some(v);
    let m = &highlight.margin;
    let [mt, mr, mb, ml] = [
        side(PartStated::MARGIN_TOP, m.top),
        side(PartStated::MARGIN_RIGHT, m.right),
        side(PartStated::MARGIN_BOTTOM, m.bottom),
        side(PartStated::MARGIN_LEFT, m.left),
    ];
    let [pt, pr, pb, pl] = padding(row);
    let sum = |m: Option<f32>, p: Option<f32>| match (m, p) {
        (Some(m), Some(p)) => Some(m + p),
        (Some(v), None) | (None, Some(v)) => Some(v),
        (None, None) => None,
    };
    [sum(mt, pt), sum(mr, pr), sum(mb, pb), sum(ml, pl)]
}

/// [`crate::geometry::list_item`]: `list.row_height` at the row's height,
/// `list.border.padding` as [`row_inset`] reads it, `list.item_font` at the
/// item label.
pub(crate) fn list_item(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let l = &r.list;
    let row = l.row(WidgetState::NONE);
    Ok(geometry::list_item_from(
        row.min_size[1],
        row_inset(&row, &l.highlight(WidgetState::NONE)),
        &font(&l.item_label(WidgetState::NONE), "list.item_label(NONE)")?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::list_item_selected`].
pub(crate) fn list_item_selected(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::list_item_selected_from(text_color(
        &r.list.item_label(WidgetState::SELECTED),
        "list.item_label(SELECTED).color",
    )?))
}

/// [`crate::geometry::data_table_size`].
pub(crate) fn data_table_size(r: &ResolvedTheme) -> Mapped<Size> {
    let l = &r.list;
    Ok(geometry::data_table_size_from(
        l.row(WidgetState::NONE).min_size[1],
        text_size(
            &l.item_label(WidgetState::NONE),
            "list.item_label(NONE).size_px",
        )?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::list_header_height`].
pub(crate) fn list_header_height(r: &ResolvedTheme) -> Mapped<Option<Pixels>> {
    let l = &r.list;
    Ok(geometry::list_header_height_from(
        l.header(WidgetState::NONE).min_size[1],
        text_size(
            &l.item_label(WidgetState::NONE),
            "list.item_label(NONE).size_px",
        )?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::tooltip`]. `tooltip.border.shadow_enabled` is the
/// panel's `SHADOWS` bit unstated (true) or stated with no shadows (false);
/// stated shadow layers are a geometry the builder has no form for.
pub(crate) fn tooltip(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let t = &r.tooltip;
    let panel = &t.panel;
    let shadow = match (panel.stated.contains(PartStated::SHADOWS), &panel.shadows) {
        (false, _) => Some(hsla(r.colors.shadow)),
        (true, None) => None,
        (true, Some(_)) => return Err(Unmapped("tooltip.panel.shadows")),
    };
    let frame = geometry::tooltip_frame(&geometry::TooltipFrame {
        padding: padding(panel),
        radius: radius(panel, "tooltip.panel.corners")?,
        background: fill(panel, "tooltip.panel.fill")?,
        border_width: stroke_width(panel, "tooltip.panel.stroke.width")?,
        border_color: stroke_color(panel, "tooltip.panel.stroke.color")?,
    });
    Ok(geometry::with_tooltip_shadow(
        geometry::coloured_text(frame, &font(&t.text, "tooltip.text")?, line_height(r)),
        shadow,
    ))
}

/// [`crate::geometry::tooltip_content`].
pub(crate) fn tooltip_content(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let panel = &r.tooltip.panel;
    let [_, right, _, left] = padding(panel);
    Ok(geometry::tooltip_content_from(
        stated(panel.max_size[0], "tooltip.panel.max_size[0]")?,
        [left, right],
        stroke_width(panel, "tooltip.panel.stroke.width")?,
        text_size(&r.typography.body, "typography.body.size_px")?,
    ))
}

/// [`crate::geometry::icon_size_small`].
pub(crate) fn icon_size_small(r: &ResolvedTheme) -> Mapped<Size> {
    Ok(geometry::icon_size(stated(
        r.icons.sizes.small,
        "icons.sizes.small",
    )?))
}

/// [`crate::geometry::icon_size_large`].
pub(crate) fn icon_size_large(r: &ResolvedTheme) -> Mapped<Size> {
    Ok(geometry::icon_size(stated(
        r.icons.sizes.large,
        "icons.sizes.large",
    )?))
}

/// [`crate::geometry::icon_size_dialog`].
pub(crate) fn icon_size_dialog(r: &ResolvedTheme) -> Mapped<Size> {
    Ok(geometry::icon_size(stated(
        r.icons.sizes.dialog,
        "icons.sizes.dialog",
    )?))
}

/// [`crate::geometry::icon_size_panel`].
pub(crate) fn icon_size_panel(r: &ResolvedTheme) -> Mapped<Size> {
    Ok(geometry::icon_size(stated(
        r.icons.sizes.panel,
        "icons.sizes.panel",
    )?))
}

/// [`crate::geometry::scrollbar_gutter`].
pub(crate) fn scrollbar_gutter(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::scrollbar_gutter_from(
        r.scrollbar.overlay_mode,
        scrollbar_geometry(r)?.track_width,
    ))
}

/// [`crate::geometry::input_group_button`]: `button.border.corner_radius`.
pub(crate) fn input_group_button(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::input_group_button_from(radius(
        &r.button.body(WidgetState::NONE),
        "button.body(NONE).corners",
    )?))
}

/// [`crate::geometry::editor`]: `defaults.line_height` alone.
pub(crate) fn editor(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::line_height_of(
        StyleRefinement::default(),
        line_height(r),
    ))
}

/// [`crate::geometry::widget_gap`].
pub(crate) fn widget_gap(r: &ResolvedTheme) -> Mapped<Option<Pixels>> {
    Ok(geometry::layout_metric(r.layout.widget_gap))
}

/// [`crate::geometry::container_margin`].
pub(crate) fn container_margin(r: &ResolvedTheme) -> Mapped<Option<Pixels>> {
    Ok(geometry::layout_metric(r.layout.container_margin))
}

/// [`crate::geometry::window_margin`].
pub(crate) fn window_margin(r: &ResolvedTheme) -> Mapped<Option<Pixels>> {
    Ok(geometry::layout_metric(r.layout.window_margin))
}

/// [`crate::geometry::section_gap`].
pub(crate) fn section_gap(r: &ResolvedTheme) -> Mapped<Option<Pixels>> {
    Ok(geometry::layout_metric(r.layout.section_gap))
}

/// [`crate::geometry::input`].
pub(crate) fn input(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let field = r.input.field(WidgetState::NONE);
    Ok(geometry::input_from(
        &geometry::FieldFrame {
            min_height: field.min_size[1],
            padding: padding(&field),
            radius: radius(&field, "input.field(NONE).corners")?,
            border_width: stroke_width(&field, "input.field(NONE).stroke.width")?,
        },
        &font(&r.input.text(WidgetState::NONE), "input.text(NONE)")?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::text_area`]: the multi-line field's frame, the
/// single-line field's font, as the 0.7.2 builder reads them.
pub(crate) fn text_area(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let field = r.text_area.field(WidgetState::NONE);
    Ok(geometry::text_area_from(
        &geometry::FieldFrame {
            min_height: None,
            padding: padding(&field),
            radius: radius(&field, "text_area.field(NONE).corners")?,
            border_width: stroke_width(&field, "text_area.field(NONE).stroke.width")?,
        },
        &font(&r.input.text(WidgetState::NONE), "input.text(NONE)")?,
        line_height(r),
    ))
}

/// [`crate::geometry::input_height`].
pub(crate) fn input_height(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::input_height_from(
        r.input.field(WidgetState::NONE).min_size[1],
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::input_fill`]: `input.disabled_background`, a soft
/// option, where a rule states it for `DISABLED` (module doc of `next`), else
/// unset, so the fill at rest stands in as on the 0.7.2 path.
pub(crate) fn input_fill(r: &ResolvedTheme, disabled: bool) -> Mapped<StyleRefinement> {
    let i = &r.input;
    Ok(geometry::field_fill(
        &geometry::FieldFill {
            rest: fill(&i.field(WidgetState::NONE), "input.field(NONE).fill")?,
            disabled_fill: state_fill(
                &i.field(WidgetState::DISABLED),
                "input.field(DISABLED).fill",
            )?,
            disabled_text: text_color(
                &i.text(WidgetState::DISABLED),
                "input.text(DISABLED).color",
            )?,
            disabled_opacity: stated(i.disabled_opacity, "input.disabled_opacity")?,
        },
        disabled,
    ))
}

/// The combo box trigger's frame, as [`crate::geometry::select`] and
/// [`crate::geometry::combobox`] read it.
fn combo_frame(r: &ResolvedTheme) -> Mapped<geometry::ComboFrame> {
    let c = &r.combo_box;
    let field = c.field(WidgetState::NONE);
    Ok(geometry::ComboFrame {
        field: geometry::FieldFrame {
            min_height: field.min_size[1],
            padding: padding(&field),
            radius: radius(&field, "combo_box.field(NONE).corners")?,
            border_width: stroke_width(&field, "combo_box.field(NONE).stroke.width")?,
        },
        arrow_area: c.arrow_area_width.is_some(),
        min_width: stated(field.min_size[0], "combo_box.field(NONE).min_size[0]")?,
    })
}

/// [`crate::geometry::select`].
pub(crate) fn select(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::select_from(
        &combo_frame(r)?,
        &font(
            &r.combo_box.label(WidgetState::NONE),
            "combo_box.label(NONE)",
        )?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::combobox`].
pub(crate) fn combobox(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::combobox_from(
        &combo_frame(r)?,
        &uncoloured_font(
            &r.combo_box.label(WidgetState::NONE),
            "combo_box.label(NONE)",
        )?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::combo_box_fill`]: `combo_box.disabled_background` as
/// [`input_fill`] reads the input's.
pub(crate) fn combo_box_fill(r: &ResolvedTheme, disabled: bool) -> Mapped<StyleRefinement> {
    let c = &r.combo_box;
    Ok(geometry::field_fill(
        &geometry::FieldFill {
            rest: fill(&c.field(WidgetState::NONE), "combo_box.field(NONE).fill")?,
            disabled_fill: state_fill(
                &c.field(WidgetState::DISABLED),
                "combo_box.field(DISABLED).fill",
            )?,
            disabled_text: text_color(
                &c.label(WidgetState::DISABLED),
                "combo_box.label(DISABLED).color",
            )?,
            disabled_opacity: stated(c.disabled_opacity, "combo_box.disabled_opacity")?,
        },
        disabled,
    ))
}

/// [`crate::geometry::menu_item`]: `menu.row_height` at the item's maximum
/// height, an exact height being stated as both, and `menu.min_row_height`
/// at its minimum, which the height rule takes only where no exact height is
/// stated, as 0.7.2's builder does.
pub(crate) fn menu_item(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let m = &r.menu;
    let item = m.item(WidgetState::NONE);
    Ok(geometry::menu_item_from(
        &geometry::MenuRow {
            row_height: item.max_size[1],
            min_row_height: item.min_size[1],
            padding: padding(&item),
            icon_text_gap: m.icon_text_gap,
        },
        &uncoloured_font(&m.item_label(WidgetState::NONE), "menu.item_label(NONE)")?,
        line_height(r),
        r.context.text_scale,
    ))
}

/// [`crate::geometry::popover`].
pub(crate) fn popover(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let panel = &r.popover.panel;
    Ok(geometry::popover_from(
        padding(panel),
        radius(panel, "popover.panel.corners")?,
        stroke_width(panel, "popover.panel.stroke.width")?,
        stroke_color(panel, "popover.panel.stroke.color")?,
    ))
}

/// [`crate::geometry::dialog`]: the panel's sizes where stated.
pub(crate) fn dialog(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let panel = &r.dialog.panel;
    Ok(geometry::dialog_from(&geometry::DialogFrame {
        padding: padding(panel),
        min_width: panel.min_size[0],
        min_height: panel.min_size[1],
        max_height: panel.max_size[1],
        radius: radius(panel, "dialog.panel.corners")?,
        background: fill(panel, "dialog.panel.fill")?,
        border_color: stroke_color(panel, "dialog.panel.stroke.color")?,
    }))
}

/// [`crate::geometry::dialog_entrance`]: the two styles 0.7.2 has as its
/// builder maps them, and the two it lacks as the gpui-component entrances
/// that draw them (gpui-component 0.10.0 `src/dialog/entrance.rs:14-27`):
/// `Fade`, a fade in place, as `DialogEntrance::Fade`, and `None`, no
/// motion, as `DialogEntrance::None`.
pub(crate) fn dialog_entrance(r: &ResolvedTheme) -> Mapped<Option<DialogEntrance>> {
    let Some(style) = r.dialog.entrance else {
        return Ok(None);
    };
    match style {
        DialogEntranceStyle::ScaleFade
        | DialogEntranceStyle::FadeRise
        | DialogEntranceStyle::Fade => Ok(Some(DialogEntrance::Fade)),
        DialogEntranceStyle::None => Ok(Some(DialogEntrance::None)),
        _ => Err(Unmapped("dialog.entrance")),
    }
}

/// [`crate::geometry::dialog_footer`].
pub(crate) fn dialog_footer(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::dialog_footer_from(r.dialog.button_gap))
}

/// [`crate::geometry::dialog_title`].
pub(crate) fn dialog_title(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::coloured_text(
        StyleRefinement::default(),
        &font(&r.dialog.title, "dialog.title")?,
        line_height(r),
    ))
}

/// [`crate::geometry::dialog_description`].
pub(crate) fn dialog_description(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::coloured_text(
        StyleRefinement::default(),
        &font(&r.dialog.body, "dialog.body")?,
        line_height(r),
    ))
}

/// [`crate::geometry::dialog_max_width`].
pub(crate) fn dialog_max_width(r: &ResolvedTheme) -> Mapped<Pixels> {
    Ok(px(stated(
        r.dialog.panel.max_size[0],
        "dialog.panel.max_size[0]",
    )?))
}

/// [`crate::geometry::status_bar`].
pub(crate) fn status_bar(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let s = &r.status_bar;
    Ok(geometry::status_bar_from(
        padding(&s.bar),
        &font(&s.label, "status_bar.label")?,
        line_height(r),
    ))
}

/// [`crate::geometry::title_bar`].
pub(crate) fn title_bar(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::coloured_text(
        StyleRefinement::default(),
        &font(&r.window.title(normal_window()), "window.title(normal)")?,
        line_height(r),
    ))
}

/// [`crate::geometry::title_bar_inactive`]: the title bar's fill and the
/// title's colour of `inactive`, the theme resolved for an inactive window
/// (`ThemeContext::window_active` false, spec §3), over the window's fill of
/// `r`, the theme resolved for the context the connector shows.
pub(crate) fn title_bar_inactive(
    r: &ResolvedTheme,
    inactive: &ResolvedTheme,
) -> Mapped<StyleRefinement> {
    let w = &inactive.window;
    Ok(geometry::title_bar_inactive_from(
        fill(
            &w.title_bar(normal_window()),
            "window.title_bar(normal).fill, inactive",
        )?,
        fill(
            &r.window.frame(normal_window()),
            "window.frame(normal).fill",
        )?,
        text_color(
            &w.title(normal_window()),
            "window.title(normal).color, inactive",
        )?,
    ))
}

/// [`crate::geometry::toolbar`]: the bar's frame, the toolbar button's label.
pub(crate) fn toolbar(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let t = &r.toolbar;
    let bar = t.bar(WidgetState::NONE);
    Ok(geometry::toolbar_from(
        &geometry::ToolbarFrame {
            bar_height: bar.min_size[1],
            item_gap: t.item_gap,
            padding: padding(&bar),
            background: fill(&bar, "toolbar.bar(NONE).fill")?,
            border_width: stroke_width(&bar, "toolbar.bar(NONE).stroke.width")?,
            border_color: stroke_color(&bar, "toolbar.bar(NONE).stroke.color")?,
        },
        &font(
            &t.button_label(WidgetState::NONE),
            "toolbar.button_label(NONE)",
        )?,
        line_height(r),
    ))
}

/// [`crate::geometry::icon_size_toolbar`].
pub(crate) fn icon_size_toolbar(r: &ResolvedTheme) -> Mapped<Size> {
    let icon = r.toolbar.button_icon(WidgetState::NONE);
    if !icon.stated.contains(IconStated::SIZE) {
        return Err(Unmapped("toolbar.button_icon(NONE).size"));
    }
    Ok(geometry::icon_size(icon.size))
}

/// [`crate::geometry::toolbar_button_rest`]: `toolbar.button_hover_border_color`
/// at the hovered toolbar button's stroke, a soft option, where a rule states
/// it for the state (module doc of `next`); the push button's stroke width
/// and padding.
pub(crate) fn toolbar_button_rest(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let body = r.button.body(WidgetState::NONE);
    Ok(geometry::toolbar_button_rest_from(
        state_stroke_color(
            &r.toolbar.button(WidgetState::HOVER),
            "toolbar.button(HOVER).stroke.color",
        )?
        .is_some(),
        stroke_width(&body, "button.body(NONE).stroke.width")?,
        padding(&body),
    ))
}

/// [`crate::geometry::toolbar_button_checked`]: each soft option where a rule
/// states it for its state (module doc of `next`), else unset, so the 0.7.2
/// path's fallbacks are taken.
pub(crate) fn toolbar_button_checked(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let t = &r.toolbar;
    let b = &r.button;
    let checked = t.button(WidgetState::CHECKED);
    Ok(geometry::toolbar_button_checked_from(
        &geometry::ToolbarChecked {
            button_checked_background: state_fill(&checked, "toolbar.button(CHECKED).fill")?,
            button_active_background: state_fill(
                &t.button(WidgetState::PRESSED),
                "toolbar.button(PRESSED).fill",
            )?,
            button_checked_border_color: state_stroke_color(
                &checked,
                "toolbar.button(CHECKED).stroke.color",
            )?,
            checked_text_color: state_text_color(
                &b.label(WidgetState::CHECKED),
                "button.label(CHECKED).color",
            )?,
            active_text_color: text_color(
                &b.label(WidgetState::PRESSED),
                "button.label(PRESSED).color",
            )?,
            active_background: state_fill(
                &b.body(WidgetState::PRESSED),
                "button.body(PRESSED).fill",
            )?,
        },
    ))
}

/// [`crate::variants::toolbar_button_states`], what
/// [`crate::variants::toolbar_button`] gives its variant: the toolbar
/// button's hovered fill and stroke and pressed fill, soft options, each
/// where a rule states it for its state (module doc of `next`), else `None`.
pub(crate) fn toolbar_button_states(r: &ResolvedTheme) -> Mapped<ToolbarButtonStates> {
    let t = &r.toolbar;
    let hovered = t.button(WidgetState::HOVER);
    Ok(ToolbarButtonStates {
        hover: state_fill(&hovered, "toolbar.button(HOVER).fill")?,
        hover_border: state_stroke_color(&hovered, "toolbar.button(HOVER).stroke.color")?,
        active: state_fill(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).fill",
        )?,
    })
}

/// [`crate::variants::hover_border`], the colour of
/// [`crate::variants::bordered_button`]'s variant: `button.hover_border_color`
/// at the hovered body's stroke, a soft option, where a rule states it for
/// the state (module doc of `next`), else `None`, as 0.7.2's on a preset that
/// leaves the field unset.
pub(crate) fn hover_border(r: &ResolvedTheme) -> Mapped<Option<Hsla>> {
    state_stroke_color(
        &r.button.body(WidgetState::HOVER),
        "button.body(HOVER).stroke.color",
    )
}

/// The default button's frame hovered and pressed: the hovered body's stroke
/// where a rule states it for the state (`button.hover_border_color`), else
/// the default body's stroke (`button.default_button_border_color`, else
/// `button.border.color`, 0.7.2's fallback).
fn default_hover_frame(hovered: &PartStyle, default: &PartStyle) -> Mapped<Hsla> {
    match state_stroke_color(hovered, "button.body(HOVER).stroke.color")? {
        Some(frame) => Ok(frame),
        None => stroke_color(default, "button.body(DEFAULT).stroke.color"),
    }
}

/// [`crate::variants::default_button_states`], what
/// [`crate::variants::default_button`] gives its variant:
/// `button.default_button_background` at the default body's fill,
/// `button.default_button_text_color` at the default label's colour (a theme
/// with no rule for the state returns `button.font.color`, 0.7.2's fallback)
/// and the frame [`default_hover_frame`] gives. `DEFAULT` is a state, so
/// `button.default_button_background` is unset, and the states `None` as in
/// 0.7.2, where no rule states the default body's fill (its `from_state` bit).
pub(crate) fn default_button_states(r: &ResolvedTheme) -> Mapped<Option<DefaultButtonStates>> {
    let b = &r.button;
    let default = b.body(WidgetState::DEFAULT);
    let Some(hover) = state_fill(&default, "button.body(DEFAULT).fill")? else {
        return Ok(None);
    };
    Ok(Some(DefaultButtonStates {
        hover,
        foreground: text_color(
            &b.label(WidgetState::DEFAULT),
            "button.label(DEFAULT).color",
        )?,
        hover_frame: default_hover_frame(&b.body(WidgetState::HOVER), &default)?,
    }))
}

#[cfg(test)]
mod tests {
    use native_theme::next::{Color, Stroke};

    use super::*;

    fn stroked(c: Color, from_state: PartStated) -> PartStyle {
        let mut part = PartStyle::default();
        part.stroke = Stroke::uniform(1.0, c);
        part.stated = PartStated::STROKE;
        part.from_state = from_state;
        part
    }

    /// `button.hover_border_color` in the default button's states: without a
    /// rule for `HOVER` the frame is the default body's, 0.7.2's fallback.
    #[test]
    fn a_default_hover_frame_without_a_state_rule_is_the_default_frame() -> Mapped<()> {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let blue = Color::rgb(0.0, 0.0, 1.0);
        let default = stroked(blue, PartStated::default());
        assert_eq!(
            default_hover_frame(&stroked(red, PartStated::default()), &default)?,
            hsla(blue)
        );
        assert_eq!(
            default_hover_frame(&stroked(red, PartStated::STROKE), &default)?,
            hsla(red)
        );
        Ok(())
    }
}
