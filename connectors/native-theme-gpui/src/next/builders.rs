//! The `geometry` builders, and the values the connector feeds its
//! `variants` with, from the 0.8.0 model: each
//! reads what its 0.7.2 builder reads, at its home in the table of homes, and
//! hands it to the function that builder hands its own reads to.
//!
//! The text sizes are the model's, which the theme's text scale has already
//! scaled; the control-height rule takes that scale from the theme's
//! context, where the 0.7.2 builders take it from the accessibility
//! preferences.

use gpui::{Hsla, Pixels, StyleRefinement, Styled as _, px};
use gpui_component::Size;
use gpui_component::dialog::DialogEntrance;
use native_theme::next::{
    ButtonQuery, DecorationKind, DialogEntranceStyle, IconStated, IconStyle, PartStated, PartStyle,
    ResolvedTheme, Stroke, TextStated, WidgetState,
};

use super::{
    Mapped, Unmapped, fill, fill_of, flat_button_states, font, hsla, normal_window, padding_of,
    radius, scrollbar_geometry, state_fill_of, state_stroke_color, state_text_color, stated,
    stroke_color, stroke_width, stroke_width_of, text_color, text_size, uncoloured_font,
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
        padding: padding_of(&body),
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

/// [`crate::geometry::button_disabled`], and the border colour
/// `button.body(DISABLED).stroke.color` where a rule states it for
/// `DISABLED` (module doc of `next`), else unset, so [`button`]'s border at
/// rest, which upstream replays over its disabled style, stands (spec §10,
/// rule 0).
pub(crate) fn button_disabled(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let b = &r.button;
    let disabled = b.body(WidgetState::DISABLED);
    let style = geometry::button_disabled_from(
        Some(fill(&disabled, "button.body(DISABLED).fill")?),
        fill(&b.body(WidgetState::NONE), "button.body(NONE).fill")?,
        text_color(
            &b.label(WidgetState::DISABLED),
            "button.label(DISABLED).color",
        )?,
        stated(b.disabled_opacity, "button.disabled_opacity")?,
    );
    Ok(
        match state_stroke_color(&disabled, "button.body(DISABLED).stroke.color")? {
            Some(pen) => style.border_color(pen),
            None => style,
        },
    )
}

/// [`crate::geometry::button_checked`]: `button.checked_text_color` and
/// `button.checked_background`, soft options, where a rule states them for
/// `CHECKED` (module doc of `next`), else unset, so gpui-component's own
/// selected label and fill stand (gpui-component 0.10.0 `button/button.rs`,
/// `ButtonVariant::selected`): never the pressed pair the 0.7.2 path stands
/// in with, `button.active_text_color` and `button.active_background`, since
/// which state looks like which is the theme's to state (spec §10, rule 0a).
/// The border colour is `button.body(CHECKED).stroke.color` where a rule
/// states it for `CHECKED` (no line, transparent, where it states none),
/// at [`button`]'s width, else unset, so [`button`]'s border at rest, which
/// upstream replays over its selected style, stands (spec §10, rule 0).
/// Where that stroke's stated width is not [`button`]'s, which gpui-component keeps for a
/// selected Button, `button.body(CHECKED).stroke.width` is not mapped (spec §10, rule 4): gpui's
/// border, unlike iced's, takes layout space (gpui-fast 0.1.16 `src/taffy.rs`, `to_taffy`).
pub(crate) fn button_checked(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let b = &r.button;
    let checked = b.body(WidgetState::CHECKED);
    if checked.from_state.contains(PartStated::STROKE)
        && let Stroke::Uniform { width, .. } = checked.stroke
        && stroke_width_of(&b.body(WidgetState::NONE)) != Some(width)
    {
        return Err(Unmapped("button.body(CHECKED).stroke.width"));
    }
    let style = geometry::button_checked_from(
        state_text_color(
            &b.label(WidgetState::CHECKED),
            "button.label(CHECKED).color",
        )?,
        None,
        state_fill_of(&checked, "button.body(CHECKED).fill")?,
        None,
    );
    Ok(
        match state_stroke_color(&checked, "button.body(CHECKED).stroke.color")? {
            Some(pen) => style.border_color(pen),
            None => style,
        },
    )
}

/// [`crate::geometry::default_button_rest`].
pub(crate) fn default_button_rest(r: &ResolvedTheme) -> Mapped<Option<StyleRefinement>> {
    let b = &r.button;
    let default = b.body(WidgetState::DEFAULT);
    Ok(geometry::default_button_rest_from(
        state_fill_of(&default, "button.body(DEFAULT).fill")?,
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
    let [pt, pr, pb, pl] = padding_of(row);
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
        padding: padding_of(panel),
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
    let [_, right, _, left] = padding_of(panel);
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
            padding: padding_of(&field),
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
            padding: padding_of(&field),
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
            disabled_fill: state_fill_of(
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
            padding: padding_of(&field),
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
            disabled_fill: state_fill_of(
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
            padding: padding_of(&item),
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
        padding_of(panel),
        radius(panel, "popover.panel.corners")?,
        stroke_width(panel, "popover.panel.stroke.width")?,
        stroke_color(panel, "popover.panel.stroke.color")?,
    ))
}

/// [`crate::geometry::dialog`]: the panel's sizes where stated, and its
/// edge's colour and width (spec §10, rule 2: none is 0 wide), on every
/// side, where upstream draws one (`src/dialog/dialog.rs:783`, `border_1`).
pub(crate) fn dialog(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let panel = &r.dialog.panel;
    Ok(geometry::dialog_from(&geometry::DialogFrame {
        padding: padding_of(panel),
        min_width: panel.min_size[0],
        min_height: panel.min_size[1],
        max_height: panel.max_size[1],
        radius: radius(panel, "dialog.panel.corners")?,
        background: fill(panel, "dialog.panel.fill")?,
        border_color: stroke_color(panel, "dialog.panel.stroke.color")?,
        border_width: Some(stroke_width(panel, "dialog.panel.stroke.width")?),
    }))
}

/// 0 where `part`'s stroke is stated as none, which draws no line (spec
/// §10, rule 2): a builder that sets only the edge's colour sets its width
/// there too; `None` elsewhere, where it keeps upstream's.
fn no_line_width(part: &PartStyle) -> Option<f32> {
    (part.stated.contains(PartStated::STROKE) && matches!(part.stroke, Stroke::None)).then_some(0.0)
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

/// [`crate::geometry::dialog_max_width`]; `None` where the theme states no
/// maximum width (`dialog.panel.max_size[0]`), and the dialog keeps
/// gpui-component's own.
pub(crate) fn dialog_max_width(r: &ResolvedTheme) -> Mapped<Option<Pixels>> {
    Ok(r.dialog.panel.max_size[0].map(px))
}

/// [`crate::geometry::status_bar`].
pub(crate) fn status_bar(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let s = &r.status_bar;
    Ok(geometry::status_bar_from(
        padding_of(&s.bar),
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
            padding: padding_of(&bar),
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

/// [`crate::geometry::toolbar_button_rest`]: `toolbar.button_hover_border_color` at the
/// hovered toolbar button's stroke, a soft option, where a rule states it for the state (module
/// doc of `next`); the push button's stroke width and padding; in [`toolbar_capsule`].
pub(crate) fn toolbar_button_rest(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let body = r.button.body(WidgetState::NONE);
    let style = geometry::toolbar_button_rest_from(
        state_stroke_color(
            &r.toolbar.button(WidgetState::HOVER),
            "toolbar.button(HOVER).stroke.color",
        )?
        .is_some(),
        stroke_width(&body, "button.body(NONE).stroke.width")?,
        padding_of(&body),
    );
    toolbar_capsule(r, style)
}

/// [`crate::geometry::toolbar_button_checked`]: each soft option where a rule
/// states it for its state (module doc of `next`), else unset; an unstated
/// checked fill is gpui-component's own selected ghost's, `secondary_active`
/// (gpui-component 0.10.0 `button/button.rs:1346`), from [`super::apply`], and
/// the label `toolbar.button_label(CHECKED)`'s, else `button.label(CHECKED)`'s,
/// else the variant's own: never another state's (spec §10, rule 0a).
pub(crate) fn toolbar_button_checked(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let checked = r.toolbar.button(WidgetState::CHECKED);
    let (_, own_selected) = flat_button_states(r)?;
    let checked_text_color = match state_text_color(
        &r.toolbar.button_label(WidgetState::CHECKED),
        "toolbar.button_label(CHECKED).color",
    )? {
        Some(color) => Some(color),
        None => state_text_color(
            &r.button.label(WidgetState::CHECKED),
            "button.label(CHECKED).color",
        )?,
    };
    Ok(geometry::toolbar_button_checked_from(
        &geometry::ToolbarChecked {
            button_checked_background: state_fill_of(&checked, "toolbar.button(CHECKED).fill")?,
            button_active_background: None,
            button_checked_border_color: state_stroke_color(
                &checked,
                "toolbar.button(CHECKED).stroke.color",
            )?,
            checked_text_color,
            active_text_color: None,
            active_background: Some(own_selected),
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
        hover: state_fill_of(&hovered, "toolbar.button(HOVER).fill")?,
        hover_border: state_stroke_color(&hovered, "toolbar.button(HOVER).stroke.color")?,
        active: state_fill_of(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).fill",
        )?,
    })
}

/// [`crate::geometry::progress`]: `progress_bar.track_height` at the track's
/// minimum height, an exact height being stated as its minimum and maximum.
pub(crate) fn progress(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let track = r.progress_bar.track(WidgetState::NONE);
    Ok(geometry::progress_from(
        stated(track.min_size[1], "progress_bar.track(NONE).min_size[1]")?,
        radius(&track, "progress_bar.track(NONE).corners")?,
        stated(track.min_size[0], "progress_bar.track(NONE).min_size[0]")?,
    ))
}

/// [`crate::geometry::group_box_content`]. A card whose stroke or corners
/// the theme leaves unstated gets no border or no radius from the
/// refinement, and the group box keeps gpui-component's own.
pub(crate) fn group_box_content(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let container = r.card.container(WidgetState::NONE);
    let stroked = container.stated.contains(PartStated::STROKE);
    let rounded = container.stated.contains(PartStated::CORNERS);
    Ok(geometry::group_box_content_from(&geometry::CardFrame {
        padding: padding_of(&container),
        radius: rounded
            .then(|| radius(&container, "card.container(NONE).corners"))
            .transpose()?,
        border_width: stroked
            .then(|| stroke_width(&container, "card.container(NONE).stroke.width"))
            .transpose()?,
        border_color: stroked
            .then(|| stroke_color(&container, "card.container(NONE).stroke.color"))
            .transpose()?,
        background: fill(&container, "card.container(NONE).fill")?,
    }))
}

/// [`crate::geometry::accordion_title`].
pub(crate) fn accordion_title(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let e = &r.expander;
    Ok(geometry::accordion_title_from(
        stated(
            e.header(WidgetState::NONE).min_size[1],
            "expander.header(NONE).min_size[1]",
        )?,
        text_color(
            &e.header_label(WidgetState::NONE),
            "expander.header_label(NONE).color",
        )?,
    ))
}

/// [`crate::geometry::accordion`]: the edge's colour, and its width only
/// where the stroke is stated as none, 0 (spec §10, rule 2). A stated
/// visible width is not set: the one refinement lands on the `Accordion`,
/// edged on every side, and on each `AccordionItem`, edged at the bottom
/// but the last (gpui-component 0.10.0 `src/accordion.rs:108-114`,
/// `:380-384`), so a width on every side would frame each item, and upstream's
/// `border_1` and `border_b_1` stand.
pub(crate) fn accordion(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let container = r.expander.container(WidgetState::NONE);
    Ok(geometry::accordion_from(
        stroke_color(&container, "expander.container(NONE).stroke.color")?,
        no_line_width(&container),
    ))
}

/// [`crate::geometry::accordion_hover`]: `expander.hover_background`, a soft
/// option, where a rule states it for `HOVER` (module doc of `next`), else
/// unset.
pub(crate) fn accordion_hover(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    Ok(geometry::accordion_hover_from(state_fill_of(
        &r.expander.header(WidgetState::HOVER),
        "expander.header(HOVER).fill",
    )?))
}

/// [`crate::geometry::spinner_size`].
pub(crate) fn spinner_size(r: &ResolvedTheme) -> Mapped<Size> {
    Ok(geometry::spinner_size_from(r.spinner.diameter))
}

/// [`crate::geometry::link`]: `link.underline_enabled` at the label's
/// decoration, an underline or none, stated.
pub(crate) fn link(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let l = &r.link;
    let label = l.label(WidgetState::NONE);
    if !label.stated.contains(TextStated::DECORATION) {
        return Err(Unmapped("link.label(NONE).decoration"));
    }
    let underlined = match label.decoration.kind {
        DecorationKind::Underline => true,
        DecorationKind::None => false,
        _ => return Err(Unmapped("link.label(NONE).decoration")),
    };
    Ok(geometry::link_from(
        &uncoloured_font(&label, "link.label(NONE)")?,
        line_height(r),
        fill(&l.container(WidgetState::NONE), "link.container(NONE).fill")?,
        underlined,
    ))
}

/// [`crate::variants::switch_color`].
pub(crate) fn switch_color(r: &ResolvedTheme) -> Mapped<Hsla> {
    fill(
        &r.switch.track(WidgetState::CHECKED),
        "switch.track(CHECKED).fill",
    )
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
    let Some(hover) = state_fill_of(&default, "button.body(DEFAULT).fill")? else {
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

/// The flat button's hovered outline, the edge of
/// [`super::variants::ghost_button`]'s variant:
/// `button.body(HOVER; variant=Flat).stroke.color`, a soft option, where a
/// rule states it for the state (module doc of `next`), else `None`. A rule
/// that names no variant is the flat button's too (spec §1, "Axis values
/// without data"), so a theme that states no flat rule gives it the push
/// button's hovered stroke.
pub(crate) fn ghost_hover_border(r: &ResolvedTheme) -> Mapped<Option<Hsla>> {
    state_stroke_color(
        &r.button.body(ButtonQuery::flat().state(WidgetState::HOVER)),
        "button.body(HOVER; variant=Flat).stroke.color",
    )
}

/// [`super::geometry::ghost_button_rest`]: where [`ghost_hover_border`]
/// states the outline, a transparent fill and a transparent border of the
/// hovered flat body's stroke width, each stated side of the push button's
/// padding, which [`button`] gives the Button, less that width
/// ([`geometry::toolbar_button_rest_from`]); nothing where it states none.
pub(crate) fn ghost_button_rest(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    ghost_rest_within(r, padding_of(&r.button.body(WidgetState::NONE)))
}

/// The builder of [`super::geometry::ghost_button_rest_own_padding`]: where
/// [`ghost_hover_border`] states the outline, a transparent fill and a
/// transparent border of the hovered flat body's stroke width, and no
/// padding side, so the Button's own stands; nothing where it states none.
pub(crate) fn ghost_button_rest_own_padding(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    ghost_rest_within(r, [None; 4])
}

/// The rest of [`ghost_button_rest`] and [`ghost_button_rest_own_padding`],
/// the padding sides `padding` less the hovered flat body's stroke width
/// ([`geometry::toolbar_button_rest_from`]).
fn ghost_rest_within(r: &ResolvedTheme, padding: [Option<f32>; 4]) -> Mapped<StyleRefinement> {
    if ghost_hover_border(r)?.is_none() {
        return Ok(StyleRefinement::default());
    }
    let hovered = r.button.body(ButtonQuery::flat().state(WidgetState::HOVER));
    Ok(geometry::toolbar_button_rest_from(
        true,
        stroke_width(&hovered, "button.body(HOVER; variant=Flat).stroke.width")?,
        padding,
    ))
}

/// [`super::geometry::ghost_button_checked`]: where a rule states the
/// checked flat body's stroke for `CHECKED`,
/// `button.body(CHECKED; variant=Flat).stroke`, a border of its width in its
/// colour, each stated side of the push button's padding less that width
/// ([`geometry::toolbar_button_rest_from`]), and no fill, so the selected
/// style's stands; nothing where none does.
pub(crate) fn ghost_button_checked(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let checked = r
        .button
        .body(ButtonQuery::flat().state(WidgetState::CHECKED));
    let Some(color) =
        state_stroke_color(&checked, "button.body(CHECKED; variant=Flat).stroke.color")?
    else {
        return Ok(StyleRefinement::default());
    };
    let mut style = geometry::toolbar_button_rest_from(
        true,
        stroke_width(&checked, "button.body(CHECKED; variant=Flat).stroke.width")?,
        padding_of(&r.button.body(WidgetState::NONE)),
    );
    style.background = None;
    Ok(style.border_color(color))
}

/// `style`, [`toolbar_button_rest`]'s, laid in the toolbar button's own fill
/// and corners at rest, `toolbar.button(NONE)`'s, each where stated (spec
/// §10, rule 0), the fill over the outline's transparent one. Upstream
/// replays the refinement over the selected and disabled styles
/// (gpui-component 0.10.0 `button/button.rs:834-850`), and its hover and
/// press styles lie over it.
fn toolbar_capsule(r: &ResolvedTheme, style: StyleRefinement) -> Mapped<StyleRefinement> {
    let rest = r.toolbar.button(WidgetState::NONE);
    let style = match fill_of(&rest, "toolbar.button(NONE).fill")? {
        Some(fill) => style.bg(fill),
        None => style,
    };
    Ok(if rest.stated.contains(PartStated::CORNERS) {
        style.rounded(px(radius(&rest, "toolbar.button(NONE).corners")?.max(0.0)))
    } else {
        style
    })
}

/// The disabled state of a toolbar `Button` built on
/// [`crate::variants::toolbar_button`], applied after
/// [`toolbar_button_rest`]: `toolbar.button(DISABLED)`'s fill and corners,
/// each where a rule states it for `DISABLED` (module doc of `next`), else
/// unset, so the rest refinement upstream replays over the disabled style
/// (gpui-component 0.10.0 `button/button.rs:843-850`) stands, the theme's
/// value at rest (spec §10, rule 0); and the text colour
/// `toolbar.button_label(DISABLED).color` where a rule states it for
/// `DISABLED`, else unset, and the disabled style's own text colour stands
/// (gpui-component 0.10.0 `button/button.rs:1374-1385`,
/// `ButtonVariant::disabled`). An untinted `Icon` takes the Button's text
/// colour (gpui-component 0.10.0 `src/icon.rs`, `into_svg`), so it follows
/// the label. A disabled Button only: upstream replays the refinement at
/// rest too.
pub(crate) fn toolbar_button_disabled(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    let disabled = r.toolbar.button(WidgetState::DISABLED);
    let style = match state_fill_of(&disabled, "toolbar.button(DISABLED).fill")? {
        Some(fill) => StyleRefinement::default().bg(fill),
        None => StyleRefinement::default(),
    };
    let style = match state_text_color(
        &r.toolbar.button_label(WidgetState::DISABLED),
        "toolbar.button_label(DISABLED).color",
    )? {
        Some(color) => style.text_color(color),
        None => style,
    };
    Ok(if disabled.from_state.contains(PartStated::CORNERS) {
        style.rounded(px(
            radius(&disabled, "toolbar.button(DISABLED).corners")?.max(0.0)
        ))
    } else {
        style
    })
}

/// The colour a toolbar Button's icon is tinted in, in `state`:
/// `toolbar.button_icon(NONE).color` at rest where stated; in another state
/// `toolbar.button_icon(state).color` where a rule states it for the state
/// (module doc of `next`); else `None`, and the icon keeps the colour it is
/// drawn in: an untinted `Icon` takes the Button's text colour
/// (gpui-component 0.10.0 `src/icon.rs`, `into_svg`), so a checked Button's
/// icon follows its checked label, the toolkit's own treatment (spec §10,
/// rule 0). An `Icon` takes one
/// colour (gpui-component 0.10.0 `src/icon.rs`, `Styled for Icon`), so the
/// tint of the state a Button is built in stays under the pointer and
/// pressed.
pub(crate) fn toolbar_button_icon_color(r: &ResolvedTheme, state: WidgetState) -> Option<Hsla> {
    icon_color_in(&r.toolbar.button_icon(state), state)
}

/// [`toolbar_button_icon_color`] of a flat Button built on
/// [`super::variants::ghost_button`]: `button.icon(NONE; variant=Flat).color`
/// at rest where stated; in another state
/// `button.icon(state; variant=Flat).color` where a rule states it for the
/// state; else `None`, and the icon keeps the Button's text colour.
pub(crate) fn ghost_button_icon_color(r: &ResolvedTheme, state: WidgetState) -> Option<Hsla> {
    icon_color_in(&r.button.icon(ButtonQuery::flat().state(state)), state)
}

/// The colour of `icon`, queried in `state`, as
/// [`toolbar_button_icon_color`] takes it: stated at rest, else stated by a
/// rule for the state.
fn icon_color_in(icon: &IconStyle, state: WidgetState) -> Option<Hsla> {
    let bits = if state == WidgetState::NONE {
        icon.stated
    } else {
        icon.from_state
    };
    bits.contains(IconStated::COLOR).then(|| hsla(icon.color))
}

#[cfg(test)]
mod tests {
    use gpui::{AbsoluteLength, TestAppContext};
    use native_theme::next::{
        ButtonRule, CardRule, Color, DialogPatch, ExpanderRule, ListRule, PartPatch, PopoverPatch,
        Preset, Scheme, Stroke, Theme, ThemeContext, ThemePatch, ToolbarRule, TooltipPatch,
        WidgetPatchMap,
    };

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

    /// A part patch that states the stroke as none.
    fn none() -> PartPatch {
        PartPatch::new().stroke(Stroke::None)
    }

    /// kde-breeze light with `patch` laid over it, installed through
    /// `next::apply`: the theme the installed `NativeTheme` holds.
    fn installed(cx: &mut TestAppContext, patch: &ThemePatch) -> Mapped<ResolvedTheme> {
        let mut r = Theme::preset(Preset::KdeBreeze)
            .resolve(&ThemeContext::default().with_scheme(Scheme::Light));
        r.apply_patch(patch);
        cx.update(|cx| {
            super::super::apply(&r, cx)?;
            cx.try_global::<crate::NativeTheme>()
                .and_then(|theme| theme.next_resolved(cx))
                .cloned()
                .ok_or(Unmapped("the installed 0.8.0 theme"))
        })
    }

    /// Whether `style` draws no line (spec §10, rule 2): its border colour
    /// transparent, and every border width it sets 0.
    fn draws_no_line(style: &StyleRefinement) -> bool {
        let w = &style.border_widths;
        let zero = Some(AbsoluteLength::Pixels(px(0.0)));
        style.border_color == Some(hsla(Color::TRANSPARENT))
            && [&w.top, &w.right, &w.bottom, &w.left]
                .into_iter()
                .all(|side| side.is_none() || *side == zero)
    }

    /// `patch`, installed, builds each refinement `build` gives with no line.
    fn built_with_no_line(
        cx: &mut TestAppContext,
        patch: &ThemePatch,
        build: impl Fn(&ResolvedTheme) -> Mapped<Vec<StyleRefinement>>,
    ) {
        let built = installed(cx, patch).and_then(|r| build(&r));
        assert!(
            built
                .as_ref()
                .is_ok_and(|styles| !styles.is_empty() && styles.iter().all(draws_no_line)),
            "{built:?}"
        );
    }

    /// A button body stroked as none in every state: the push button and the
    /// default button at rest are built with no line, the default button's
    /// hover frame is transparent, and so is the bordered button's hover
    /// edge where a rule states one (kde-breeze states none).
    #[gpui::test]
    fn a_button_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new().button(ButtonRule::new().body(none()));
        built_with_no_line(cx, &patch, |r| {
            let mut styles = vec![button(r)?];
            styles.extend(default_button_rest(r)?);
            Ok(styles)
        });
        let clear = hsla(Color::TRANSPARENT);
        let frames = installed(cx, &patch).and_then(|r| {
            Ok((
                hover_border(&r)?,
                default_button_states(&r)?.map(|states| states.hover_frame),
            ))
        });
        assert!(
            frames.as_ref().is_ok_and(
                |&(hover, frame)| hover.is_none_or(|h| h == clear) && frame == Some(clear)
            ),
            "{frames:?}"
        );
        let hovered =
            ThemePatch::new().button(ButtonRule::new().states(WidgetState::HOVER).body(none()));
        let edge = installed(cx, &hovered).and_then(|r| hover_border(&r));
        assert_eq!(edge, Ok(Some(clear)));
    }

    /// A list container stroked as none.
    #[gpui::test]
    fn a_list_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new().list(ListRule::new().container(none()));
        built_with_no_line(cx, &patch, |r| Ok(vec![list(r)?]));
    }

    /// A tooltip panel stroked as none.
    #[gpui::test]
    fn a_tooltip_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new()
            .widget_rules(WidgetPatchMap::new().tooltip(TooltipPatch::new().panel(none())));
        built_with_no_line(cx, &patch, |r| Ok(vec![tooltip(r)?]));
    }

    /// A popover panel stroked as none.
    #[gpui::test]
    fn a_popover_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new()
            .widget_rules(WidgetPatchMap::new().popover(PopoverPatch::new().panel(none())));
        built_with_no_line(cx, &patch, |r| Ok(vec![popover(r)?]));
    }

    /// Whether `style` sets every border width to 0, as a builder that sets
    /// only an edge's colour must where the stroke is none: upstream's width
    /// would otherwise stand.
    fn sets_every_width_to_0(style: &StyleRefinement) -> bool {
        let w = &style.border_widths;
        let zero = Some(AbsoluteLength::Pixels(px(0.0)));
        [&w.top, &w.right, &w.bottom, &w.left]
            .into_iter()
            .all(|side| *side == zero)
    }

    /// A dialog panel stroked as none: its edge transparent and 0 wide.
    #[gpui::test]
    fn a_dialog_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new()
            .widget_rules(WidgetPatchMap::new().dialog(DialogPatch::new().panel(none())));
        built_with_no_line(cx, &patch, |r| Ok(vec![dialog(r)?]));
        let built = installed(cx, &patch).and_then(|r| dialog(&r));
        assert!(built.as_ref().is_ok_and(sets_every_width_to_0), "{built:?}");
    }

    /// A dialog panel stroked 3 px wide, unlike upstream's `border_1`: its
    /// edge is the stated colour at the stated width on every side.
    #[gpui::test]
    fn a_dialog_is_built_with_its_stated_stroke_width(cx: &mut TestAppContext) {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let patch =
            ThemePatch::new().widget_rules(WidgetPatchMap::new().dialog(
                DialogPatch::new().panel(PartPatch::new().stroke(Stroke::uniform(3.0, red))),
            ));
        let built = installed(cx, &patch).and_then(|r| dialog(&r));
        let three = Some(AbsoluteLength::Pixels(px(3.0)));
        assert!(
            built.as_ref().is_ok_and(|style| {
                let w = &style.border_widths;
                style.border_color == Some(hsla(red))
                    && [&w.top, &w.right, &w.bottom, &w.left]
                        .into_iter()
                        .all(|side| *side == three)
            }),
            "{built:?}"
        );
    }

    /// A toolbar bar stroked as none.
    #[gpui::test]
    fn a_toolbar_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new().toolbar(ToolbarRule::new().bar(none()));
        built_with_no_line(cx, &patch, |r| Ok(vec![toolbar(r)?]));
    }

    /// A card container stroked as none.
    #[gpui::test]
    fn a_card_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new().card(CardRule::new().container(none()));
        built_with_no_line(cx, &patch, |r| Ok(vec![group_box_content(r)?]));
    }

    /// An expander container stroked as none: the accordion's edge
    /// transparent and 0 wide; a stroked one keeps upstream's width.
    #[gpui::test]
    fn an_accordion_stroked_as_none_is_built_with_no_line(cx: &mut TestAppContext) {
        let patch = ThemePatch::new().expander(ExpanderRule::new().container(none()));
        built_with_no_line(cx, &patch, |r| Ok(vec![accordion(r)?]));
        let built = installed(cx, &patch).and_then(|r| accordion(&r));
        assert!(built.as_ref().is_ok_and(sets_every_width_to_0), "{built:?}");
        let stroked = installed(cx, &ThemePatch::new()).and_then(|r| accordion(&r));
        assert!(
            stroked
                .as_ref()
                .is_ok_and(|style| style.border_widths.top.is_none()),
            "{stroked:?}"
        );
    }
}
