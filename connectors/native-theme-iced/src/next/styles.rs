//! [`crate::styles`] on the 0.8.0 model: each function reads what its 0.7.2
//! namesake reads, at its home in the table of homes, and hands it to the
//! function that namesake hands its own reads to, so both return the same
//! style function from the same values.
//!
//! Requires the `widgets` feature (on by default).

use iced_core::theme::Theme;
use native_theme::next::{
    ButtonQuery, IconStated, Paint, PartStated, PartStyle, ResolvedTheme, Stroke, WidgetState,
    WindowState,
};

use super::borders::{
    line, line_or_none, outline, outline_or_iced, radius, state_line, stroke_width,
};
use super::{Mapped, Unmapped, fill, state_fill, state_text_color, stated, text_color, to_color};
use crate::styles::{
    self as old, ButtonSource, CheckFills, CheckboxSource, InputSource, Layer, Line, LinkSource,
    Outline, PickListSource, SwitchSource, ToolbarButtonSource,
};

/// A part's fill, where stated, as a layer the style lays over the fill
/// under it: its colour as [`fill`] reads it, or, for a stated
/// `Paint::None`, no fill, which paints nothing in that state.
fn layer(part: &PartStyle, home: &'static str) -> Mapped<Layer> {
    match part.fill {
        Paint::None if part.stated.contains(PartStated::FILL) => Ok(Layer::Nothing),
        _ => fill(part, home).map(Layer::Color),
    }
}

/// [`layer`] where a rule states the fill for the queried state (its
/// `from_state` bit); `None` where none does.
pub(crate) fn state_layer(part: &PartStyle, home: &'static str) -> Mapped<Option<Layer>> {
    if part.from_state.contains(PartStated::FILL) {
        layer(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// [`ButtonSource`] of the 0.8.0 model. A state option no rule states for
/// its state (its `from_state` bit clear) is `None`, and the state is then
/// iced's own for that status (spec §2, `from_state`). A button that states
/// no disabled opacity takes iced's own disabled look for each colour the
/// theme leaves to iced (spec §10, rule 0).
fn button_source(r: &ResolvedTheme) -> Mapped<ButtonSource> {
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let hover = b.body(WidgetState::HOVER);
    let pressed = b.body(WidgetState::PRESSED);
    let default = b.body(WidgetState::DEFAULT);
    let primary = b.body(ButtonQuery::primary());
    let c = &r.colors;
    Ok(ButtonSource {
        background: fill(&rest, "button.body(NONE).fill")?,
        hover_background: state_layer(&hover, "button.body(HOVER).fill")?,
        active_background: state_layer(&pressed, "button.body(PRESSED).fill")?,
        disabled_background: state_fill(
            &b.body(WidgetState::DISABLED),
            "button.body(DISABLED).fill",
        )?,
        checked_background: state_fill(&b.body(WidgetState::CHECKED), "button.body(CHECKED).fill")?,
        label: text_color(&b.label(WidgetState::NONE), "button.label(NONE).color")?,
        hover_label: state_text_color(&b.label(WidgetState::HOVER), "button.label(HOVER).color")?,
        active_label: state_text_color(
            &b.label(WidgetState::PRESSED),
            "button.label(PRESSED).color",
        )?,
        disabled_label: state_text_color(
            &b.label(WidgetState::DISABLED),
            "button.label(DISABLED).color",
        )?,
        checked_label: state_text_color(
            &b.label(WidgetState::CHECKED),
            "button.label(CHECKED).color",
        )?,
        disabled_opacity: b.disabled_opacity,
        border: outline!(rest, "button.body(NONE)")?,
        primary_border: outline!(primary, "button.body(NONE; variant=Primary)")?,
        hover_border_color: state_line(&hover, "button.body(HOVER).stroke.color")?,
        pressed_border_color: state_line(&pressed, "button.body(PRESSED).stroke.color")?,
        primary_hover_border_color: state_line(
            &b.body(ButtonQuery::primary().state(WidgetState::HOVER)),
            "button.body(HOVER; variant=Primary).stroke.color",
        )?,
        primary_pressed_border_color: state_line(
            &b.body(ButtonQuery::primary().state(WidgetState::PRESSED)),
            "button.body(PRESSED; variant=Primary).stroke.color",
        )?,
        primary_background: fill(&primary, "button.body(NONE; variant=Primary).fill")?,
        primary_label: text_color(
            &b.label(ButtonQuery::primary()),
            "button.label(NONE; variant=Primary).color",
        )?,
        default_background: state_fill(&default, "button.body(DEFAULT).fill")?,
        default_label: state_text_color(
            &b.label(WidgetState::DEFAULT),
            "button.label(DEFAULT).color",
        )?,
        default_border_color: state_line(&default, "button.body(DEFAULT).stroke.color")?,
        window_background: to_color(stated(c.window.background, "colors.window.background")?),
        window_text: to_color(stated(c.window.foreground, "colors.window.foreground")?),
    })
}

/// [`ToolbarButtonSource`] of the 0.8.0 model.
fn toolbar_button_source(r: &ResolvedTheme) -> Mapped<ToolbarButtonSource> {
    let t = &r.toolbar;
    let hover = t.button(WidgetState::HOVER);
    let checked = t.button(WidgetState::CHECKED);
    Ok(ToolbarButtonSource {
        hover_background: state_layer(&hover, "toolbar.button(HOVER).fill")?,
        active_background: state_layer(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).fill",
        )?,
        checked_background: state_fill(&checked, "toolbar.button(CHECKED).fill")?,
        hover_border_color: state_line(&hover, "toolbar.button(HOVER).stroke.color")?,
        checked_border_color: state_line(&checked, "toolbar.button(CHECKED).stroke.color")?,
        pressed_border_color: state_line(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).stroke.color",
        )?,
        checked_hover_border_color: state_line(
            &t.button(WidgetState::HOVER | WidgetState::CHECKED),
            "toolbar.button(HOVER|CHECKED).stroke.color",
        )?,
        checked_pressed_border_color: state_line(
            &t.button(WidgetState::PRESSED | WidgetState::HOVER | WidgetState::CHECKED),
            "toolbar.button(PRESSED|HOVER|CHECKED).stroke.color",
        )?,
    })
}

/// [`crate::styles::button`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_from(&button_source(r)?))
}

/// [`crate::styles::button_toggle`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_toggle(
    r: &ResolvedTheme,
    checked: bool,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_toggle_from(&button_source(r)?, checked))
}

/// [`crate::styles::toolbar_button`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn toolbar_button(
    r: &ResolvedTheme,
    checked: bool,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::toolbar_button_from(
        &button_source(r)?,
        &toolbar_button_source(r)?,
        checked,
    ))
}

/// [`crate::styles::button_primary`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_primary(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_primary_from(&button_source(r)?))
}

/// [`crate::styles::button_default`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_default(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_default_from(&button_source(r)?))
}

/// [`crate::styles::button_danger`]: the error role's fill and foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_danger(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.error;
    Ok(old::class_button(
        &button_source(r)?,
        to_color(role.fill),
        to_color(role.foreground),
        iced_widget::button::danger,
    ))
}

/// [`crate::styles::button_success`]: the success role's fill and
/// foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_success(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.success;
    Ok(old::class_button(
        &button_source(r)?,
        to_color(role.fill),
        to_color(role.foreground),
        iced_widget::button::success,
    ))
}

/// [`crate::styles::button_warning`]: the warning role's fill and
/// foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_warning(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.warning;
    Ok(old::class_button(
        &button_source(r)?,
        to_color(role.fill),
        to_color(role.foreground),
        iced_widget::button::warning,
    ))
}

/// [`crate::styles::button_link`]: faded when disabled by the theme's
/// `colors.disabled_opacity`, the link stating none of its own. A hover
/// fill no `HOVER` rule states is iced's own text button's.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the link needs.
pub fn button_link(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let l = &r.link;
    let idle = fill(&l.container(WidgetState::NONE), "link.container(NONE).fill")?;
    Ok(old::button_link_from(&LinkSource {
        background: idle,
        hover_background: state_layer(
            &l.container(WidgetState::HOVER),
            "link.container(HOVER).fill",
        )?,
        pressed_background: state_layer(
            &l.container(WidgetState::PRESSED),
            "link.container(PRESSED).fill",
        )?
        .map(|pressed| pressed.over(idle)),
        label: text_color(&l.label(WidgetState::NONE), "link.label(NONE).color")?,
        hover_label: state_text_color(&l.label(WidgetState::HOVER), "link.label(HOVER).color")?,
        active_label: state_text_color(
            &l.label(WidgetState::PRESSED),
            "link.label(PRESSED).color",
        )?,
        disabled_label: text_color(
            &l.label(WidgetState::DISABLED),
            "link.label(DISABLED).color",
        )?,
        disabled_opacity: r.colors.disabled_opacity,
    }))
}

/// [`InputSource`] of the 0.8.0 model, of the field `$w` of `$r` (`input`
/// or `text_area`, each its own widget), every home under `$home`, and the
/// focus indicator.
macro_rules! input_source {
    ($r:expr, $w:ident, $home:literal) => {{
        let r: &ResolvedTheme = $r;
        let i = &r.$w;
        let ring = &r.focus_ring;
        let rest = i.field(WidgetState::NONE);
        Ok::<InputSource, Unmapped>(InputSource {
            background: fill(&rest, concat!($home, ".field(NONE).fill"))?,
            disabled_background: state_fill(
                &i.field(WidgetState::DISABLED),
                concat!($home, ".field(DISABLED).fill"),
            )?,
            text: text_color(
                &i.text(WidgetState::NONE),
                concat!($home, ".text(NONE).color"),
            )?,
            disabled_text: text_color(
                &i.text(WidgetState::DISABLED),
                concat!($home, ".text(DISABLED).color"),
            )?,
            disabled_opacity: i.disabled_opacity,
            border: outline!(rest, concat!($home, ".field(NONE)"))?,
            hover_border_color: state_line(
                &i.field(WidgetState::HOVER),
                concat!($home, ".field(HOVER).stroke.color"),
            )?,
            focus_border_color: state_line(
                &i.field(WidgetState::FOCUSED),
                concat!($home, ".field(FOCUSED).stroke.color"),
            )?,
            focus_ring_color: to_color(r.colors.focus_ring),
            focus_ring_width: match &ring.stroke {
                Stroke::None => 0.0,
                Stroke::Uniform { width, .. } => *width,
                _ => return Err(Unmapped("focus_ring.stroke.width")),
            },
            focus_ring_offset: ring.offset,
            placeholder: text_color(
                &i.placeholder(WidgetState::NONE),
                concat!($home, ".placeholder(NONE).color"),
            )?,
            selection: to_color(r.colors.text_selection.background),
        })
    }};
}

/// [`crate::styles::text_input`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the field needs.
pub fn text_input(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::text_input::Status) -> iced_widget::text_input::Style + Clone + use<>,
> {
    Ok(old::text_input_from(&input_source!(r, input, "input")?))
}

/// [`crate::styles::text_editor`]: the text area's own colours and border.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the field needs.
pub fn text_editor(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::text_editor::Status) -> iced_widget::text_editor::Style + Clone + use<>,
> {
    Ok(old::text_editor_from(&input_source!(
        r,
        text_area,
        "text_area"
    )?))
}

/// A part's fill where a rule states it for `$flag` (the query with that
/// flag alone, spec §2, `from_state`), read at `$query`; `None` where none
/// does. A value a rule for another flag of `$query` states (a checked box's
/// fill under the pointer) is not this state's.
macro_rules! flag_fill {
    ($w:expr, $part:ident, $flag:expr, $query:expr, $home:expr) => {
        if $w.$part($flag).from_state.contains(PartStated::FILL) {
            fill(&$w.$part($query), $home).map(Some)
        } else {
            Ok(None)
        }
    };
}

/// [`flag_fill`] read as a [`layer`].
macro_rules! flag_layer {
    ($w:expr, $part:ident, $flag:expr, $query:expr, $home:expr) => {
        if $w.$part($flag).from_state.contains(PartStated::FILL) {
            layer(&$w.$part($query), $home).map(Some)
        } else {
            Ok(None)
        }
    };
}

/// [`flag_fill`] of the stroke.
macro_rules! flag_line {
    ($w:expr, $part:ident, $flag:expr, $query:expr, $home:expr) => {
        if $w.$part($flag).from_state.contains(PartStated::STROKE) {
            line(&$w.$part($query), $home).map(Some)
        } else {
            Ok(None)
        }
    };
}

/// [`CheckFills`] of the 0.8.0 model, of the indicator of `$w` (the check
/// box's or the radio's), every home under `$home`.
macro_rules! check_fills {
    ($w:expr, $home:literal) => {{
        let w = $w;
        let unchecked = fill(
            &w.indicator(WidgetState::NONE),
            concat!($home, ".indicator(NONE).fill"),
        )?;
        Ok::<CheckFills, Unmapped>(CheckFills {
            checked: fill(
                &w.indicator(WidgetState::CHECKED),
                concat!($home, ".indicator(CHECKED).fill"),
            )?,
            unchecked,
            hover_fill: state_layer(
                &w.indicator(WidgetState::HOVER),
                concat!($home, ".indicator(HOVER).fill"),
            )?
            .map(|hover| hover.over(unchecked)),
            checked_border: line_or_none(
                &w.indicator(WidgetState::CHECKED),
                concat!($home, ".indicator(CHECKED).stroke.color"),
            )?,
            unchecked_border: line_or_none(
                &w.indicator(WidgetState::NONE),
                concat!($home, ".indicator(NONE).stroke.color"),
            )?,
            hover_border: state_line(
                &w.indicator(WidgetState::HOVER),
                concat!($home, ".indicator(HOVER).stroke.color"),
            )?,
            checked_hover_border: flag_line!(
                w,
                indicator,
                WidgetState::HOVER,
                WidgetState::HOVER | WidgetState::CHECKED,
                concat!($home, ".indicator(HOVER|CHECKED).stroke.color")
            )?,
            hover_label: state_text_color(
                &w.label(WidgetState::HOVER),
                concat!($home, ".label(HOVER).color"),
            )?,
        })
    }};
}

/// [`CheckboxSource`] of the 0.8.0 model.
fn checkbox_source(r: &ResolvedTheme) -> Mapped<CheckboxSource> {
    let c = &r.checkbox;
    let rest = c.indicator(WidgetState::NONE);
    let mark = c.checkmark(WidgetState::CHECKED);
    if !mark.stated.contains(IconStated::COLOR) {
        return Err(Unmapped("checkbox.checkmark(CHECKED).color"));
    }
    Ok(CheckboxSource {
        fills: check_fills!(c, "checkbox")?,
        disabled_background: state_fill(
            &c.indicator(WidgetState::DISABLED),
            "checkbox.indicator(DISABLED).fill",
        )?,
        checked_disabled_background: flag_fill!(
            c,
            indicator,
            WidgetState::DISABLED,
            WidgetState::DISABLED | WidgetState::CHECKED,
            "checkbox.indicator(DISABLED|CHECKED).fill"
        )?,
        disabled_opacity: c.disabled_opacity,
        mark: to_color(mark.color),
        label: text_color(&c.label(WidgetState::NONE), "checkbox.label(NONE).color")?,
        disabled_label: text_color(
            &c.label(WidgetState::DISABLED),
            "checkbox.label(DISABLED).color",
        )?,
        border_width: stroke_width(&rest, "checkbox.indicator(NONE).stroke.width")?,
        border_radius: radius(&rest, "checkbox.indicator(NONE).corners")?,
    })
}

/// [`crate::styles::checkbox`]: a disabled box faded as one widget over the
/// window.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the box needs.
pub fn checkbox(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + Clone + use<>,
> {
    let window = stated(r.colors.window.background, "colors.window.background")?;
    checkbox_over(r, to_color(window))
}

/// [`crate::styles::checkbox_over`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the box needs.
pub fn checkbox_over(
    r: &ResolvedTheme,
    backdrop: iced_core::Color,
) -> Mapped<
    impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + Clone + use<>,
> {
    Ok(old::checkbox_from(&checkbox_source(r)?, backdrop))
}

/// [`crate::styles::radio`]: the radio's own indicator, dot and label.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the radio needs.
pub fn radio(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, iced_widget::radio::Status) -> iced_widget::radio::Style + Clone + use<>>
{
    let w = &r.radio;
    Ok(old::radio_from(
        check_fills!(w, "radio")?,
        fill(&w.dot(WidgetState::CHECKED), "radio.dot(CHECKED).fill")?,
        text_color(&w.label(WidgetState::NONE), "radio.label(NONE).color")?,
        stroke_width(
            &w.indicator(WidgetState::NONE),
            "radio.indicator(NONE).stroke.width",
        )?,
    ))
}

/// A part's outline as the switch takes it, its width and its stroke, where
/// the theme states a stroke; `None` where it states none or no stroke,
/// iced's own (none) then drawn.
fn edge(part: &PartStyle, home: &'static str) -> Mapped<Option<(f32, Line)>> {
    if !part.stated.contains(PartStated::STROKE) || matches!(part.stroke, Stroke::None) {
        return Ok(None);
    }
    Ok(Some((stroke_width(part, home)?, line(part, home)?)))
}

/// [`edge`] where a rule states the stroke for `$flag`, as [`flag_fill`]
/// reads a fill.
macro_rules! flag_edge {
    ($w:expr, $part:ident, $flag:expr, $query:expr, $home:expr) => {
        if $w.$part($flag).from_state.contains(PartStated::STROKE) {
            edge(&$w.$part($query), $home)
        } else {
            Ok(None)
        }
    };
}

/// A part's corner radius where stated; `None`, iced's own, where not.
fn radius_or_iced(part: &PartStyle, home: &'static str) -> Mapped<Option<f32>> {
    if part.stated.contains(PartStated::CORNERS) {
        radius(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// [`SwitchSource`] of the 0.8.0 model.
fn switch_source(r: &ResolvedTheme) -> Mapped<SwitchSource> {
    let s = &r.switch;
    let off = s.track(WidgetState::NONE);
    let on = s.track(WidgetState::CHECKED);
    let thumb_on = s.thumb(WidgetState::CHECKED);
    let thumb_off = s.thumb(WidgetState::NONE);
    let checked = fill(&on, "switch.track(CHECKED).fill")?;
    let unchecked = fill(&off, "switch.track(NONE).fill")?;
    let [_, track_height] = off.min_size;
    let [thumb_diameter, _] = thumb_on.min_size;
    let [unchecked_thumb_diameter, _] = thumb_off.min_size;
    Ok(SwitchSource {
        disabled_opacity: s.disabled_opacity,
        checked_background: checked,
        unchecked_background: unchecked,
        hovered_checked_background: flag_layer!(
            s,
            track,
            WidgetState::HOVER,
            WidgetState::HOVER | WidgetState::CHECKED,
            "switch.track(HOVER|CHECKED).fill"
        )?
        .map(|hover| hover.over(checked)),
        hovered_unchecked_background: state_layer(
            &s.track(WidgetState::HOVER),
            "switch.track(HOVER).fill",
        )?
        .map(|hover| hover.over(unchecked)),
        disabled_checked_background: flag_fill!(
            s,
            track,
            WidgetState::DISABLED,
            WidgetState::DISABLED | WidgetState::CHECKED,
            "switch.track(DISABLED|CHECKED).fill"
        )?,
        disabled_unchecked_background: state_fill(
            &s.track(WidgetState::DISABLED),
            "switch.track(DISABLED).fill",
        )?,
        thumb_background: fill(&thumb_on, "switch.thumb(CHECKED).fill")?,
        unchecked_thumb_background: fill(&thumb_off, "switch.thumb(NONE).fill")?,
        disabled_thumb_color: flag_fill!(
            s,
            thumb,
            WidgetState::DISABLED,
            WidgetState::DISABLED | WidgetState::CHECKED,
            "switch.thumb(DISABLED|CHECKED).fill"
        )?,
        disabled_unchecked_thumb_color: state_fill(
            &s.thumb(WidgetState::DISABLED),
            "switch.thumb(DISABLED).fill",
        )?,
        checked_track_border: edge(&on, "switch.track(CHECKED).stroke")?,
        unchecked_track_border: edge(&off, "switch.track(NONE).stroke")?,
        disabled_checked_track_border: flag_edge!(
            s,
            track,
            WidgetState::DISABLED,
            WidgetState::DISABLED | WidgetState::CHECKED,
            "switch.track(DISABLED|CHECKED).stroke"
        )?,
        disabled_unchecked_track_border: flag_edge!(
            s,
            track,
            WidgetState::DISABLED,
            WidgetState::DISABLED,
            "switch.track(DISABLED).stroke"
        )?,
        thumb_border: edge(&thumb_off, "switch.thumb(NONE).stroke")?,
        track_radius: radius_or_iced(&off, "switch.track(NONE).corners")?,
        track_height,
        thumb_diameter,
        unchecked_thumb_diameter,
    })
}

/// [`crate::styles::toggler`]: a disabled switch faded as one widget over
/// the window.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the switch needs.
pub fn toggler(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::toggler::Status) -> iced_widget::toggler::Style + Clone + use<>,
> {
    let window = stated(r.colors.window.background, "colors.window.background")?;
    toggler_over(r, to_color(window))
}

/// [`crate::styles::toggler_over`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the switch needs.
pub fn toggler_over(
    r: &ResolvedTheme,
    backdrop: iced_core::Color,
) -> Mapped<
    impl Fn(&Theme, iced_widget::toggler::Status) -> iced_widget::toggler::Style + Clone + use<>,
> {
    Ok(old::toggler_from(&switch_source(r)?, backdrop))
}

/// [`crate::styles::pick_list`]: the combo box's field, its label and the
/// input's placeholder.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the field needs.
pub fn pick_list(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::pick_list::Status) -> iced_widget::pick_list::Style + Clone + use<>,
> {
    let c = &r.combo_box;
    let rest = c.field(WidgetState::NONE);
    let hover = c.field(WidgetState::HOVER);
    let open = c.field(WidgetState::OPEN);
    let open_hover = c.field(WidgetState::OPEN | WidgetState::HOVER);
    let idle = fill(&rest, "combo_box.field(NONE).fill")?;
    Ok(old::pick_list_from(&PickListSource {
        background: idle,
        hovered_background: state_layer(&hover, "combo_box.field(HOVER).fill")?
            .map(|hover| hover.over(idle)),
        label: text_color(&c.label(WidgetState::NONE), "combo_box.label(NONE).color")?,
        placeholder: text_color(
            &r.input.placeholder(WidgetState::NONE),
            "input.placeholder(NONE).color",
        )?,
        open_background: state_layer(&open, "combo_box.field(OPEN).fill")?,
        open_hover_background: state_layer(&open_hover, "combo_box.field(OPEN|HOVER).fill")?,
        border: outline!(rest, "combo_box.field(NONE)")?,
        hover_border_color: state_line(&hover, "combo_box.field(HOVER).stroke.color")?,
        open_border_color: state_line(&open, "combo_box.field(OPEN).stroke.color")?,
        open_hover_border_color: state_line(
            &open_hover,
            "combo_box.field(OPEN|HOVER).stroke.color",
        )?,
    }))
}

/// [`crate::styles::progress_bar`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no track or fill colour.
pub fn progress_bar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::progress_bar::Style + Clone + use<>> {
    let p = &r.progress_bar;
    let track = p.track(WidgetState::NONE);
    Ok(old::progress_bar_from(
        fill(&track, "progress_bar.track(NONE).fill")?,
        fill(&p.fill(WidgetState::NONE), "progress_bar.fill(NONE).fill")?,
        outline!(track, "progress_bar.track(NONE)")?,
    ))
}

/// [`crate::styles::rule`]: the separator's colour.
#[must_use = "this returns the style function; it does not apply it"]
pub fn rule(r: &ResolvedTheme) -> impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<> {
    old::rule_from(to_color(r.separator.color))
}

/// [`crate::styles::toolbar_rule`]: the toolbar's stroke colour as stated.
/// The rule is a line of its own under the bar, not a border over the bar's
/// fill, so no fill is composited under it.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no toolbar stroke of one
/// colour.
pub fn toolbar_rule(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<>> {
    let bar = r.toolbar.bar(WidgetState::NONE);
    let line = match &bar.stroke {
        Stroke::Uniform { color, .. } if bar.stated.contains(PartStated::STROKE) => *color,
        _ => return Err(Unmapped("toolbar.bar(NONE).stroke.color")),
    };
    Ok(old::rule_from(to_color(line)))
}

/// [`crate::styles::pane_grid`]: the splitter's handle under the pointer
/// and its divider width.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no hovered handle fill.
pub fn pane_grid(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::pane_grid::Style + Clone + use<>> {
    let s = &r.splitter;
    Ok(old::pane_grid_from(
        fill(&s.handle(WidgetState::HOVER), "splitter.handle(HOVER).fill")?,
        s.divider_width,
    ))
}

/// [`crate::styles::tooltip`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no panel fill or text colour.
pub fn tooltip(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let t = &r.tooltip;
    Ok(old::panel_from(
        fill(&t.panel, "tooltip.panel.fill")?,
        Some(text_color(&t.text, "tooltip.text.color")?),
        Some(outline!(t.panel, "tooltip.panel")?),
    ))
}

/// [`crate::styles::container_card`]. The card's border is iced's own where
/// the theme states none (the table of homes, `card.border.*`:
/// `unstated_if_unset`).
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no card fill.
pub fn container_card(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let card = r.card.container(WidgetState::NONE);
    Ok(old::panel_from(
        fill(&card, "card.container(NONE).fill")?,
        None,
        Some(outline_or_iced!(card, "card.container(NONE)")?),
    ))
}

/// [`crate::styles::dialog`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no panel fill or body colour.
pub fn dialog(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let d = &r.dialog;
    Ok(old::panel_from(
        fill(&d.panel, "dialog.panel.fill")?,
        Some(text_color(&d.body, "dialog.body.color")?),
        Some(outline!(d.panel, "dialog.panel")?),
    ))
}

/// [`crate::styles::toolbar`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no bar fill or button label
/// colour.
pub fn toolbar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let t = &r.toolbar;
    Ok(old::panel_from(
        fill(&t.bar(WidgetState::NONE), "toolbar.bar(NONE).fill")?,
        Some(text_color(
            &t.button_label(WidgetState::NONE),
            "toolbar.button_label(NONE).color",
        )?),
        None::<Outline>,
    ))
}

/// [`crate::styles::application`]: the fill of a normal window's frame.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no frame fill.
pub fn application(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_core::theme::Style + Clone + use<>> {
    Ok(old::application_from(fill(
        &r.window.frame(WindowState::default()),
        "window.frame(normal).fill",
    )?))
}

#[cfg(test)]
mod tests;
