//! The `geometry` builders of the slice's widgets, and the values the
//! connector feeds the slice's `variants` with, from the 0.8.0 model: each
//! reads what its 0.7.2 builder reads, at its home in the table of homes, and
//! hands it to the function that builder hands its own reads to.
//!
//! The text sizes are the model's, which the theme's text scale has already
//! scaled; the control-height rule takes that scale from the theme's
//! context, where the 0.7.2 builders take it from the accessibility
//! preferences.

use gpui::{Hsla, Pixels, StyleRefinement};
use gpui_component::Size;
use native_theme::next::{PartStated, PartStyle, ResolvedTheme, TextStated, WidgetState};

use super::{
    Mapped, Unmapped, fill, font, hsla, padding, radius, scrollbar_geometry, state_fill,
    state_stroke_color, state_text_color, stated, stroke_color, stroke_width, text_color,
    text_size, uncoloured_font,
};
use crate::geometry;
use crate::variants::DefaultButtonStates;

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
/// row's stated margin plus its stated padding, an unstated one counting as
/// none, and unset where neither is stated on that side. The table of homes
/// places the field at the row's margin on kde-breeze (Breeze's highlight
/// margins, spec §5, L4) and at its padding elsewhere, so the sum is 0.7.2's
/// value on every preset without naming one; whether a margin is drawn as a
/// margin rather than as padding is decided with captures in a later
/// milestone.
fn row_inset(row: &PartStyle) -> [Option<f32>; 4] {
    let side = |bit: PartStated, v: f32| row.stated.contains(bit).then_some(v);
    let m = &row.margin;
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
        row_inset(&row),
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
