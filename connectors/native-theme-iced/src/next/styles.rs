//! [`crate::styles`] on the 0.8.0 model: each function reads what its 0.7.2
//! namesake reads, at its home in the table of homes, and hands it to the
//! function that namesake hands its own reads to, so both return the same
//! style function from the same values.
//!
//! Requires the `widgets` feature (on by default).

use iced_core::border::Radius;
use iced_core::theme::Theme;
use iced_core::{Shadow, Vector};
use native_theme::next::{
    ButtonQuery, IconStated, Orientation, Paint, PartStated, PartStyle, ResolvedTheme, Stroke,
    StyleQuery, TextStated, TextStyle, WidgetState, WindowState,
};

use super::borders::{
    flagged, line, line_or_none, outline_or_iced, part_outline, radius, state_line, stroke_width,
};
use super::{
    Mapped, Unmapped, fill, fill_of, state_fill_of, state_text_color, stated, text_color_of,
    to_color,
};
use crate::styles::{
    self as old, ButtonSource, CheckFills, CheckboxSource, CheckedBorders, CheckedStates,
    ExpanderSource, InputSource, Layer, Line, LinkSource, MenuSource, Outline, PartOutline,
    PickListSource, ScrollbarSource, SegmentPosition, SegmentedSource, SliderSource, StateLine,
    SwitchSource, ToolbarButtonSource, ToolbarLabels, ToolbarOutline,
};

/// A part's fill, where stated, as the state's whole fill (spec §9, step
/// 2): its colour as [`fill`] reads it, or, for a stated `Paint::None`, no
/// fill, which paints nothing in that state.
fn layer(part: &PartStyle, home: &'static str) -> Mapped<Layer> {
    match part.fill {
        Paint::None if part.stated.contains(PartStated::FILL) => Ok(Layer::Nothing),
        _ => fill(part, home).map(Layer::Whole),
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

/// The query at `state` asking `flag` (`StyleQuery::asking`): the second
/// of the spec's two queries for a flag the connector keeps its own
/// treatment of, a part drawn at a state wider than the flag (spec §2,
/// `from_state`).
fn asking(state: WidgetState, flag: WidgetState) -> StyleQuery {
    StyleQuery::new(state).asking(flag)
}

/// [`ButtonSource`] of the 0.8.0 model. A state option no rule states for
/// its state (its `from_state` bit clear) is `None`, and the state is then
/// iced's own for that status (spec §2, `from_state`). A button that states
/// no disabled opacity takes iced's own disabled look for each colour the
/// theme leaves to iced (spec §10, rule 0). The fill, the label and each
/// border property at rest, of the plain and of the primary button, are
/// `None` where the theme leaves them unstated, iced's own class's at rest
/// there (spec §10, rule 0).
fn button_source(r: &ResolvedTheme) -> Mapped<ButtonSource> {
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let hover = b.body(WidgetState::HOVER);
    let pressed = b.body(WidgetState::PRESSED);
    let default = b.body(WidgetState::DEFAULT);
    let primary = b.body(ButtonQuery::primary());
    let c = &r.colors;
    let checked_background =
        state_fill_of(&b.body(WidgetState::CHECKED), "button.body(CHECKED).fill")?;
    let checked_label = state_text_color(
        &b.label(WidgetState::CHECKED),
        "button.label(CHECKED).color",
    )?;
    Ok(ButtonSource {
        background: fill_of(&rest, "button.body(NONE).fill")?,
        hover_background: state_layer(&hover, "button.body(HOVER).fill")?,
        active_background: state_layer(&pressed, "button.body(PRESSED).fill")?,
        disabled_background: state_fill_of(
            &b.body(WidgetState::DISABLED),
            "button.body(DISABLED).fill",
        )?,
        checked_disabled_background: flagged(
            &b.body(WidgetState::DISABLED),
            &b.body(
                ButtonQuery::new(WidgetState::DISABLED | WidgetState::CHECKED)
                    .asking(WidgetState::DISABLED),
            ),
            PartStated::FILL,
            |p| fill(p, "button.body(DISABLED|CHECKED).fill"),
        )?,
        checked_background,
        checked_primary: true,
        label: text_color_of(&b.label(WidgetState::NONE)),
        hover_label: state_text_color(&b.label(WidgetState::HOVER), "button.label(HOVER).color")?,
        active_label: state_text_color(
            &b.label(WidgetState::PRESSED),
            "button.label(PRESSED).color",
        )?,
        disabled_label: state_text_color(
            &b.label(WidgetState::DISABLED),
            "button.label(DISABLED).color",
        )?,
        checked_label,
        checked_states: checked_states(
            |query: ButtonQuery| b.body(query),
            |query: ButtonQuery| b.label(query),
            (checked_background, checked_label),
            (
                "button.body(HOVER|CHECKED).fill",
                "button.body(PRESSED|CHECKED).fill",
            ),
        )?,
        disabled_opacity: b.disabled_opacity,
        border: part_outline!(rest, "button.body(NONE)")?,
        primary_border: part_outline!(primary, "button.body(NONE; variant=Primary)")?,
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
        disabled_border_color: state_line(
            &b.body(WidgetState::DISABLED),
            "button.body(DISABLED).stroke.color",
        )?,
        primary_disabled_border_color: state_line(
            &b.body(ButtonQuery::primary().state(WidgetState::DISABLED)),
            "button.body(DISABLED; variant=Primary).stroke.color",
        )?,
        // A checked toggle's own strokes, which [`button_toggle`] reads alone.
        checked_borders: CheckedBorders::default(),
        primary_background: fill_of(&primary, "button.body(NONE; variant=Primary).fill")?,
        primary_label: text_color_of(&b.label(ButtonQuery::primary())),
        default_background: state_fill_of(&default, "button.body(DEFAULT).fill")?,
        default_label: state_text_color(
            &b.label(WidgetState::DEFAULT),
            "button.label(DEFAULT).color",
        )?,
        default_border_color: state_line(&default, "button.body(DEFAULT).stroke.color")?,
        window_background: to_color(stated(c.window.background, "colors.window.background")?),
        window_text: to_color(stated(c.window.foreground, "colors.window.foreground")?),
    })
}

/// [`CheckedBorders`] of the 0.8.0 model: a checked push button's stroke at
/// rest where a rule states `button.body(CHECKED)`'s; under the pointer and
/// pressed the theme's answer for `CHECKED | HOVER` and `CHECKED | PRESSED`
/// where it states the stroke for `CHECKED` by the spec's two queries
/// (spec §2, `from_state`: `CHECKED` alone, or the state's own query asking
/// `CHECKED`); disabled its answer for `DISABLED | CHECKED` where it states
/// the stroke so for `CHECKED` or for `DISABLED`, the two flags the style
/// keeps a treatment of there. Each `None` where it does not, and the
/// unchecked button's border stays in that status (spec §10, rule 0a).
/// Each is drawn at the width the theme states for it: iced draws a
/// button's border on its bounds and lays it out by its padding alone
/// ([`Outline::with_state`]), so the width moves no content.
fn checked_borders(r: &ResolvedTheme) -> Mapped<CheckedBorders> {
    let b = &r.button;
    let read = |part: &PartStyle, (color, width): (&'static str, &'static str)| {
        Ok(StateLine {
            line: line_or_none(part, color)?,
            width: stroke_width(part, width)?,
        })
    };
    let checked = b.body(WidgetState::CHECKED);
    let at = |state: WidgetState, flag: WidgetState| b.body(ButtonQuery::new(state).asking(flag));
    let on = |state: WidgetState, homes| {
        flagged(
            &checked,
            &at(WidgetState::CHECKED | state, WidgetState::CHECKED),
            PartStated::STROKE,
            |p| read(p, homes),
        )
    };
    let rest = checked
        .from_state
        .contains(PartStated::STROKE)
        .then(|| {
            read(
                &checked,
                (
                    "button.body(CHECKED).stroke.color",
                    "button.body(CHECKED).stroke.width",
                ),
            )
        })
        .transpose()?;
    let hover = on(
        WidgetState::HOVER,
        (
            "button.body(HOVER|CHECKED).stroke.color",
            "button.body(HOVER|CHECKED).stroke.width",
        ),
    )?;
    let pressed = on(
        WidgetState::PRESSED,
        (
            "button.body(PRESSED|CHECKED).stroke.color",
            "button.body(PRESSED|CHECKED).stroke.width",
        ),
    )?;
    let both = WidgetState::DISABLED | WidgetState::CHECKED;
    let homes = (
        "button.body(DISABLED|CHECKED).stroke.color",
        "button.body(DISABLED|CHECKED).stroke.width",
    );
    let disabled = match on(WidgetState::DISABLED, homes)? {
        Some(line) => Some(line),
        None => flagged(
            &b.body(WidgetState::DISABLED),
            &at(both, WidgetState::DISABLED),
            PartStated::STROKE,
            |p| read(p, homes),
        )?,
    };
    Ok(CheckedBorders {
        rest,
        hover,
        pressed,
        disabled,
    })
}

/// A part's stroke width where a rule states its stroke for the queried
/// state (its `from_state` bit), as [`stroke_width`] reads it; `None`
/// where none does.
fn state_stroke_width(part: &PartStyle, home: &'static str) -> Mapped<Option<f32>> {
    if part.from_state.contains(PartStated::STROKE) {
        stroke_width(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// [`ToolbarOutline`] of the 0.8.0 model: `toolbar.button(NONE)`'s stroke
/// and corners, the width of each state's stroke a rule states, a disabled
/// button's stroke where a rule states it for `DISABLED`
/// (`toolbar.button(DISABLED)`), and a disabled checked button's stroke, the
/// theme's answer for
/// `DISABLED | CHECKED` where it states the stroke for `CHECKED` or for
/// `DISABLED` by the spec's two queries (spec §2, `from_state`), as
/// [`checked_borders`] reads a checked push button's.
pub(super) fn toolbar_outline(r: &ResolvedTheme) -> Mapped<ToolbarOutline> {
    let t = &r.toolbar;
    let width = |state: WidgetState, home| state_stroke_width(&t.button(state), home);
    let (hover, pressed, checked) = (
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::CHECKED,
    );
    let both = WidgetState::DISABLED | checked;
    let read = |part: &PartStyle| {
        Ok(StateLine {
            line: line_or_none(part, "toolbar.button(DISABLED|CHECKED).stroke.color")?,
            width: stroke_width(part, "toolbar.button(DISABLED|CHECKED).stroke.width")?,
        })
    };
    let on = |flag: WidgetState| {
        flagged(
            &t.button(flag),
            &t.button(asking(both, flag)),
            PartStated::STROKE,
            read,
        )
    };
    let checked_disabled = match on(checked)? {
        Some(line) => Some(line),
        None => on(WidgetState::DISABLED)?,
    };
    let off = t.button(WidgetState::DISABLED);
    let disabled = if off.from_state.contains(PartStated::STROKE) {
        Some(StateLine {
            line: line_or_none(&off, "toolbar.button(DISABLED).stroke.color")?,
            width: stroke_width(&off, "toolbar.button(DISABLED).stroke.width")?,
        })
    } else {
        None
    };
    Ok(ToolbarOutline {
        rest: part_outline!(t.button(WidgetState::NONE), "toolbar.button(NONE)")?,
        hover_width: width(hover, "toolbar.button(HOVER).stroke.width")?,
        pressed_width: width(pressed, "toolbar.button(PRESSED).stroke.width")?,
        checked_width: width(checked, "toolbar.button(CHECKED).stroke.width")?,
        checked_hover_width: width(
            hover | checked,
            "toolbar.button(HOVER|CHECKED).stroke.width",
        )?,
        checked_pressed_width: width(
            pressed | hover | checked,
            "toolbar.button(PRESSED|HOVER|CHECKED).stroke.width",
        )?,
        disabled,
        checked_disabled,
    })
}

/// [`CheckedStates`] of the 0.8.0 model, of a button whose part in a state
/// is `body`'s and whose label is `label`'s: a checked button's fill under
/// the pointer and pressed is asked of the theme only where it states the
/// checked fill for `CHECKED`, by the spec's two queries (spec §2,
/// `from_state`): `CHECKED` alone (`checked`'s fill), or the state's own
/// query asking `CHECKED`; its label only where it states the checked label
/// so (`checked`'s label, or the state's own query); each then the theme's
/// answer for `CHECKED | HOVER` and `CHECKED | PRESSED` (spec §10, rule 0:
/// "what the theme states for the state is laid over"). Where it does not,
/// the checked button is iced's own way of showing the state in every
/// status (spec §10, rule 0a). `homes` are the hovered and the pressed
/// fill's.
fn checked_states<V: Default>(
    body: impl Fn(StyleQuery<V>) -> PartStyle,
    label: impl Fn(StyleQuery<V>) -> TextStyle,
    checked: (Option<iced_core::Color>, Option<iced_core::Color>),
    homes: (&'static str, &'static str),
) -> Mapped<CheckedStates> {
    let (checked_fill, checked_label) = checked;
    let at = |state: WidgetState| {
        StyleQuery::<V>::new(WidgetState::CHECKED | state).asking(WidgetState::CHECKED)
    };
    let fill = |state: WidgetState, home| {
        let part = body(at(state));
        if checked_fill.is_some() || part.from_state.contains(PartStated::FILL) {
            fill_of(&part, home)
        } else {
            Ok(None)
        }
    };
    let label_in = |state: WidgetState| {
        let text = label(at(state));
        (checked_label.is_some() || text.from_state.contains(TextStated::COLOR))
            .then(|| text_color_of(&text))
            .flatten()
    };
    Ok(CheckedStates {
        hover_background: fill(WidgetState::HOVER, homes.0)?,
        pressed_background: fill(WidgetState::PRESSED, homes.1)?,
        hover_label: label_in(WidgetState::HOVER),
        pressed_label: label_in(WidgetState::PRESSED),
    })
}

/// [`ToolbarButtonSource`] of the 0.8.0 model: a checked button's fill and
/// label under the pointer and pressed as [`checked_states`] reads them,
/// and a disabled one's fill the theme's answer for the state it is in,
/// `DISABLED | CHECKED` where checked.
fn toolbar_button_source(r: &ResolvedTheme) -> Mapped<ToolbarButtonSource> {
    let t = &r.toolbar;
    let hover = t.button(WidgetState::HOVER);
    let checked = t.button(WidgetState::CHECKED);
    let checked_background = state_fill_of(&checked, "toolbar.button(CHECKED).fill")?;
    let checked_label = state_text_color(
        &t.button_label(WidgetState::CHECKED),
        "toolbar.button_label(CHECKED).color",
    )?;
    Ok(ToolbarButtonSource {
        outline: Some(toolbar_outline(r)?),
        rest_background: fill_of(&t.button(WidgetState::NONE), "toolbar.button(NONE).fill")?,
        disabled_background: fill_of(
            &t.button(WidgetState::DISABLED),
            "toolbar.button(DISABLED).fill",
        )?,
        checked_disabled_background: fill_of(
            &t.button(WidgetState::DISABLED | WidgetState::CHECKED),
            "toolbar.button(DISABLED|CHECKED).fill",
        )?,
        hover_background: state_layer(&hover, "toolbar.button(HOVER).fill")?,
        active_background: state_layer(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).fill",
        )?,
        checked_background,
        checked_states: checked_states(
            |query: StyleQuery| t.button(query),
            |query: StyleQuery| t.button_label(query),
            (checked_background, checked_label),
            (
                "toolbar.button(HOVER|CHECKED).fill",
                "toolbar.button(PRESSED|CHECKED).fill",
            ),
        )?,
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
        labels: Some(ToolbarLabels {
            rest: text_color_of(&t.button_label(WidgetState::NONE)),
            hover: state_text_color(
                &t.button_label(WidgetState::HOVER),
                "toolbar.button_label(HOVER).color",
            )?,
            pressed: state_text_color(
                &t.button_label(WidgetState::PRESSED),
                "toolbar.button_label(PRESSED).color",
            )?,
            checked: checked_label,
            disabled: state_text_color(
                &t.button_label(WidgetState::DISABLED),
                "toolbar.button_label(DISABLED).color",
            )?,
        }),
    })
}

/// [`crate::styles::button`]; disabled, outlined in
/// `button.body(DISABLED)`'s stroke where a rule states it for `DISABLED`,
/// else in the one at rest, faded as the fill is.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
pub fn button(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_from(&button_source(r)?))
}

/// [`crate::styles::button_toggle`]: iced's button has no checked status,
/// so a checked toggle whose theme states no checked fill
/// (`button.body(CHECKED)`) is drawn in [`button_primary`]'s fill and label
/// in the same status, a stated `button.label(CHECKED)` laid over; its
/// corners and shadow are the unchecked button's, and so is its outline in a
/// status the theme states no checked stroke for: a checked stroke stated
/// for `CHECKED` is drawn at rest, and under the pointer, pressed and
/// disabled the theme's answer for `CHECKED | HOVER`, `CHECKED | PRESSED`
/// and `DISABLED | CHECKED` where it states it by the spec's two queries
/// for `CHECKED`, or for `DISABLED` there. A disabled toggle that is off
/// takes `button.body(DISABLED)`'s stroke where a rule states it for
/// `DISABLED`, as [`button`]. A stated checked
/// fill is lettered in `button.label(CHECKED)` where stated, else in the
/// label at rest (spec §10, rule 0a). Where the theme states the checked
/// fill, a checked button under the pointer and pressed is filled in
/// `button.body(CHECKED | HOVER)`'s and `button.body(CHECKED | PRESSED)`'s;
/// where it states the checked label, it is lettered in
/// `button.label(CHECKED | HOVER)`'s and `(CHECKED | PRESSED)`'s (spec §10,
/// rule 0), as [`toolbar_button`]. Where the theme states the disabled fill
/// (`button.body(DISABLED)`), a disabled checked toggle is filled in its
/// answer for `button.body(DISABLED | CHECKED)`, as a disabled checked
/// toolbar button is.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
pub fn button_toggle(
    r: &ResolvedTheme,
    checked: bool,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let source = ButtonSource {
        checked_borders: checked_borders(r)?,
        ..button_source(r)?
    };
    Ok(old::button_toggle_from(&source, checked))
}

/// [`crate::styles::toolbar_button`]: a checked button whose theme states
/// no checked fill (`toolbar.button(CHECKED)`) is drawn in
/// [`button_primary`]'s fill and label in the same status, a stated
/// `toolbar.button_label(CHECKED)` laid over, and a stated checked fill lettered in
/// it, else in the label at rest, as [`button_toggle`] (spec §10, rule 0a).
/// Where the theme states the checked fill, a checked button under the
/// pointer and pressed is filled in `toolbar.button(CHECKED | HOVER)`'s and
/// `toolbar.button(CHECKED | PRESSED)`'s; where it states the checked label,
/// it is lettered in `toolbar.button_label(CHECKED | HOVER)`'s and
/// `(CHECKED | PRESSED)`'s (spec §10, rule 0). A button that is off is filled
/// at rest in `toolbar.button(NONE)`'s fill, and a disabled one in
/// `toolbar.button(DISABLED)`'s, a disabled checked one in
/// `toolbar.button(DISABLED | CHECKED)`'s, each where stated, else in none.
/// A hovered or pressed fill is `toolbar.button(HOVER)`'s or
/// `toolbar.button(PRESSED)`'s where a rule states it, else iced's own
/// (spec §10, rule 0), never the push button's. The labels are
/// `toolbar.button_label(<state>)`'s: at rest where stated, else iced's
/// own; another state's where a rule states it, else as iced's own class
/// derives it from the label at rest, never the push button's. The outline
/// is the toolbar button's own (spec §5, the toolbar's `button` part):
/// `toolbar.button(NONE)`'s stroke and corners, each iced's own neutral
/// class's where unstated (spec §10, rule 0), and a state's stroke a rule
/// states drawn at that stroke's width, never the push button's
/// (`button.body(NONE)`); a disabled one's `toolbar.button(DISABLED)`'s
/// where a rule states the stroke for `DISABLED`, else the outline at rest;
/// a disabled checked one's `toolbar.button(DISABLED | CHECKED)`'s where the
/// theme states the stroke for `CHECKED` or for `DISABLED` by the spec's two
/// queries, else the outline at rest.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
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

/// [`crate::styles::button_primary`]; disabled, outlined in
/// `button.body(DISABLED; variant=Primary)`'s stroke where a rule states it
/// for `DISABLED` (windows-11: none), else in the one at rest, faded as the
/// fill is.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
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
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
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
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
pub fn button_danger(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.error;
    Ok(old::class_button(
        &button_source(r)?,
        Some(to_color(role.fill)),
        Some(to_color(role.foreground)),
        iced_widget::button::danger,
    ))
}

/// [`crate::styles::button_success`]: the success role's fill and
/// foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
pub fn button_success(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.success;
    Ok(old::class_button(
        &button_source(r)?,
        Some(to_color(role.fill)),
        Some(to_color(role.foreground)),
        iced_widget::button::success,
    ))
}

/// [`crate::styles::button_warning`]: the warning role's fill and
/// foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background or
/// foreground, or states a value in a form iced cannot draw (a per-side
/// stroke not reduced, a concentric radius); a value at rest it leaves
/// unstated is iced's own class's (spec §10, rule 0).
pub fn button_warning(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.warning;
    Ok(old::class_button(
        &button_source(r)?,
        Some(to_color(role.fill)),
        Some(to_color(role.foreground)),
        iced_widget::button::warning,
    ))
}

/// [`crate::styles::button_link`]: faded when disabled by the theme's
/// `colors.disabled_opacity`, the link stating none of its own. A hover
/// fill no `HOVER` rule states is iced's own text button's, and so is each
/// of the fill and the label at rest and the disabled label the theme
/// leaves unstated (spec §10, rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where a state's fill is stated in a form iced cannot
/// paint (a layered paint with no fallback colour).
pub fn button_link(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_link_from(&link_source(r)?))
}

/// [`LinkSource`] of the 0.8.0 model.
fn link_source(r: &ResolvedTheme) -> Mapped<LinkSource> {
    let l = &r.link;
    Ok(LinkSource {
        background: fill_of(&l.container(WidgetState::NONE), "link.container(NONE).fill")?,
        hover_background: state_layer(
            &l.container(WidgetState::HOVER),
            "link.container(HOVER).fill",
        )?,
        pressed_background: state_layer(
            &l.container(WidgetState::PRESSED),
            "link.container(PRESSED).fill",
        )?,
        label: text_color_of(&l.label(WidgetState::NONE)),
        hover_label: state_text_color(&l.label(WidgetState::HOVER), "link.label(HOVER).color")?,
        active_label: state_text_color(
            &l.label(WidgetState::PRESSED),
            "link.label(PRESSED).color",
        )?,
        disabled_label: text_color_of(&l.label(WidgetState::DISABLED)),
        disabled_opacity: r.colors.disabled_opacity,
    })
}

/// [`InputSource`] of the 0.8.0 model, of the field `$w` of `$r` (`input`
/// or `text_area`, each its own widget), every home under `$home`. The
/// field draws no focus ring: [`super::focus_ring`] paints the ring over
/// it, an inset one too (spec §2, `FocusIndicator`).
macro_rules! input_source {
    ($r:expr, $w:ident, $home:literal) => {{
        let r: &ResolvedTheme = $r;
        let i = &r.$w;
        let rest = i.field(WidgetState::NONE);
        Ok::<InputSource, Unmapped>(InputSource {
            background: fill_of(&rest, concat!($home, ".field(NONE).fill"))?,
            disabled_background: state_fill_of(
                &i.field(WidgetState::DISABLED),
                concat!($home, ".field(DISABLED).fill"),
            )?,
            text: text_color_of(&i.text(WidgetState::NONE)),
            disabled_text: state_text_color(
                &i.text(WidgetState::DISABLED),
                concat!($home, ".text(DISABLED).color"),
            )?,
            disabled_opacity: i.disabled_opacity,
            border: part_outline!(rest, concat!($home, ".field(NONE)"))?,
            hover_border_color: state_line(
                &i.field(WidgetState::HOVER),
                concat!($home, ".field(HOVER).stroke.color"),
            )?,
            focus_border_color: state_line(
                &i.field(WidgetState::FOCUSED),
                concat!($home, ".field(FOCUSED).stroke.color"),
            )?,
            focus_ring: None,
            placeholder: text_color_of(&i.placeholder(WidgetState::NONE)),
            selection: to_color(r.colors.text_selection.background),
        })
    }};
}

/// [`crate::styles::text_input`]: a hovered or focused field's stroke the
/// theme states nothing for is iced's own field's (`text_input::default`
/// recolours the border under the pointer and when focused), its width and
/// corners the theme's at rest (spec §2, `from_state`); so is a disabled
/// field's value no rule for `DISABLED` states (`input.text(DISABLED)`):
/// `text_input::default` recolours it when disabled. The field draws no
/// focus ring, an inset one included: [`super::focus_ring`] paints it over
/// the field.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced); a
/// value at rest it leaves unstated is iced's own field's (spec §10, rule
/// 0).
pub fn text_input(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::text_input::Status) -> iced_widget::text_input::Style + Clone + use<>,
> {
    Ok(old::text_input_from(&text_input_source(r)?))
}

/// [`InputSource`] of the 0.8.0 model's text field.
fn text_input_source(r: &ResolvedTheme) -> Mapped<InputSource> {
    input_source!(r, input, "input")
}

/// [`crate::styles::text_editor`]: the text area's own colours and border,
/// a hovered or focused stroke the theme states nothing for as
/// [`text_input`]'s, against iced's own `text_editor::default`; no focus
/// ring, as [`text_input`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced); a
/// value at rest it leaves unstated is iced's own text area's (spec §10,
/// rule 0).
pub fn text_editor(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::text_editor::Status) -> iced_widget::text_editor::Style + Clone + use<>,
> {
    Ok(old::text_editor_from(&text_area_source(r)?))
}

/// [`InputSource`] of the 0.8.0 model's text area.
fn text_area_source(r: &ResolvedTheme) -> Mapped<InputSource> {
    input_source!(r, text_area, "text_area")
}

/// [`CheckFills`] of the 0.8.0 model, of the indicator of `$w` (the check
/// box's or the radio's), every home under `$home`.
macro_rules! check_fills {
    ($w:expr, $home:literal) => {{
        let w = $w;
        let hover = w.indicator(WidgetState::HOVER);
        let checked_hover = w.indicator(asking(
            WidgetState::HOVER | WidgetState::CHECKED,
            WidgetState::HOVER,
        ));
        Ok::<CheckFills, Unmapped>(CheckFills {
            checked: fill_of(
                &w.indicator(WidgetState::CHECKED),
                concat!($home, ".indicator(CHECKED).fill"),
            )?,
            unchecked: fill_of(
                &w.indicator(WidgetState::NONE),
                concat!($home, ".indicator(NONE).fill"),
            )?,
            hover_fill: state_layer(&hover, concat!($home, ".indicator(HOVER).fill"))?
                .map(Layer::as_given),
            checked_hover_fill: flagged(&hover, &checked_hover, PartStated::FILL, |p| {
                layer(p, concat!($home, ".indicator(HOVER|CHECKED).fill"))
            })?
            .map(Layer::as_given),
            checked_border: edge(
                &w.indicator(WidgetState::CHECKED),
                concat!($home, ".indicator(CHECKED).stroke.color"),
            )?
            .map(|(_, line)| line),
            unchecked_border: edge(
                &w.indicator(WidgetState::NONE),
                concat!($home, ".indicator(NONE).stroke.color"),
            )?
            .map(|(_, line)| line),
            hover_border: state_line(&hover, concat!($home, ".indicator(HOVER).stroke.color"))?,
            checked_hover_border: flagged(&hover, &checked_hover, PartStated::STROKE, |p| {
                line_or_none(p, concat!($home, ".indicator(HOVER|CHECKED).stroke.color"))
            })?,
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
    let disabled = c.indicator(WidgetState::DISABLED);
    let disabled_on = asking(
        WidgetState::DISABLED | WidgetState::CHECKED,
        WidgetState::DISABLED,
    );
    Ok(CheckboxSource {
        fills: check_fills!(c, "checkbox")?,
        disabled_background: state_fill_of(&disabled, "checkbox.indicator(DISABLED).fill")?,
        checked_disabled_background: flagged(
            &disabled,
            &c.indicator(disabled_on),
            PartStated::FILL,
            |p| fill(p, "checkbox.indicator(DISABLED|CHECKED).fill"),
        )?,
        disabled_border: state_line(&disabled, "checkbox.indicator(DISABLED).stroke.color")?,
        checked_disabled_border: flagged(
            &disabled,
            &c.indicator(disabled_on),
            PartStated::STROKE,
            |p| line_or_none(p, "checkbox.indicator(DISABLED|CHECKED).stroke.color"),
        )?,
        disabled_opacity: c.disabled_opacity,
        mark: mark
            .stated
            .contains(IconStated::COLOR)
            .then(|| to_color(mark.color)),
        disabled_mark: {
            let checked = c.checkmark(disabled_on);
            (c.checkmark(WidgetState::DISABLED)
                .from_state
                .union(checked.from_state)
                .contains(IconStated::COLOR)
                && checked.stated.contains(IconStated::COLOR))
            .then(|| to_color(checked.color))
        },
        label: text_color_of(&c.label(WidgetState::NONE)),
        disabled_label: text_color_of(&c.label(WidgetState::DISABLED)),
        checked_disabled_label: {
            let off = c.label(WidgetState::DISABLED);
            let checked = c.label(disabled_on);
            if off
                .from_state
                .union(checked.from_state)
                .contains(TextStated::COLOR)
            {
                text_color_of(&checked)
            } else {
                text_color_of(&off)
            }
        },
        border_width: edge(&rest, "checkbox.indicator(NONE).stroke.width")?.map(|(width, _)| width),
        border_radius: radius_or_iced(&rest, "checkbox.indicator(NONE).corners")?,
    })
}

/// [`crate::styles::checkbox`]: a disabled box faded as one widget over the
/// window, stroked in what a rule for `DISABLED` states
/// (`checkbox.indicator(DISABLED)`, `indicator(DISABLED|CHECKED)` checked),
/// else in iced's own disabled border colour where iced's own box changes
/// it (spec §2, `from_state`; §10, rule 0); a disabled checked box
/// lettered in `checkbox.label(DISABLED|CHECKED)` where a rule for
/// `DISABLED` states it by the two queries (`label(DISABLED)`, or
/// `label(DISABLED|CHECKED)` asking `DISABLED`), else as the unchecked one,
/// in `checkbox.label(DISABLED)`.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background, or
/// states a value in a form iced cannot draw (a layered fill with no
/// fallback colour, a per-side stroke not reduced, a concentric radius); a value at
/// rest it leaves unstated is iced's own box's (spec §10, rule 0).
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
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced, a
/// concentric radius); a value at rest it leaves unstated is iced's own
/// box's (spec §10, rule 0).
pub fn checkbox_over(
    r: &ResolvedTheme,
    backdrop: iced_core::Color,
) -> Mapped<
    impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + Clone + use<>,
> {
    Ok(old::checkbox_from(&checkbox_source(r)?, backdrop))
}

/// [`crate::styles::radio`]: the radio's own indicator, dot and label.
/// iced's radio has no disabled status (iced_widget 0.14.2 `src/radio.rs`,
/// `Status`), so `radio.indicator(DISABLED)` has no receiver.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced); a
/// value at rest it leaves unstated is iced's own radio's (spec §10, rule
/// 0).
pub fn radio(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme, iced_widget::radio::Status) -> iced_widget::radio::Style + Clone + use<>>
{
    let (fills, dot, label, width) = radio_source(r)?;
    Ok(old::radio_from(fills, dot, label, width))
}

/// What [`radio`] hands [`old::radio_from`]: the circle's fills and
/// strokes, the dot, the label and the stroke's width, each at rest `None`
/// where the theme leaves it unstated.
type RadioSource = (
    CheckFills,
    Option<iced_core::Color>,
    Option<iced_core::Color>,
    Option<f32>,
);

/// [`RadioSource`] of the 0.8.0 model.
fn radio_source(r: &ResolvedTheme) -> Mapped<RadioSource> {
    let w = &r.radio;
    Ok((
        check_fills!(w, "radio")?,
        fill_of(&w.dot(WidgetState::CHECKED), "radio.dot(CHECKED).fill")?,
        text_color_of(&w.label(WidgetState::NONE)),
        edge(
            &w.indicator(WidgetState::NONE),
            "radio.indicator(NONE).stroke.width",
        )?
        .map(|(width, _)| width),
    ))
}

/// A part's outline as the switch takes it, its width and its stroke, where
/// the theme states a stroke, no stroke ([`Line::none`], 0 wide) where it
/// states none; `None` where it states nothing, iced's own (none) then
/// drawn.
fn edge(part: &PartStyle, home: &'static str) -> Mapped<Option<(f32, Line)>> {
    if !part.stated.contains(PartStated::STROKE) {
        return Ok(None);
    }
    Ok(Some((stroke_width(part, home)?, line_or_none(part, home)?)))
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
    let [_, track_height] = off.min_size;
    let [thumb_diameter, _] = thumb_on.min_size;
    let [unchecked_thumb_diameter, _] = thumb_off.min_size;
    let hover = WidgetState::HOVER;
    let hover_on = asking(hover | WidgetState::CHECKED, hover);
    let disabled = WidgetState::DISABLED;
    let disabled_on = asking(disabled | WidgetState::CHECKED, disabled);
    Ok(SwitchSource {
        disabled_opacity: s.disabled_opacity,
        checked_background: fill_of(&on, "switch.track(CHECKED).fill")?,
        unchecked_background: fill_of(&off, "switch.track(NONE).fill")?,
        hovered_checked_background: flagged(
            &s.track(hover),
            &s.track(hover_on),
            PartStated::FILL,
            |p| layer(p, "switch.track(HOVER|CHECKED).fill"),
        )?
        .map(Layer::as_given),
        hovered_unchecked_background: state_layer(&s.track(hover), "switch.track(HOVER).fill")?
            .map(Layer::as_given),
        disabled_checked_background: flagged(
            &s.track(disabled),
            &s.track(disabled_on),
            PartStated::FILL,
            |p| fill(p, "switch.track(DISABLED|CHECKED).fill"),
        )?,
        disabled_unchecked_background: state_fill_of(
            &s.track(disabled),
            "switch.track(DISABLED).fill",
        )?,
        thumb_background: fill_of(&thumb_on, "switch.thumb(CHECKED).fill")?,
        unchecked_thumb_background: fill_of(&thumb_off, "switch.thumb(NONE).fill")?,
        hovered_thumb: flagged(&s.thumb(hover), &s.thumb(hover_on), PartStated::FILL, |p| {
            layer(p, "switch.thumb(HOVER|CHECKED).fill")
        })?
        .map(Layer::as_given),
        hovered_unchecked_thumb: state_layer(&s.thumb(hover), "switch.thumb(HOVER).fill")?
            .map(Layer::as_given),
        disabled_thumb_color: flagged(
            &s.thumb(disabled),
            &s.thumb(disabled_on),
            PartStated::FILL,
            |p| fill(p, "switch.thumb(DISABLED|CHECKED).fill"),
        )?,
        disabled_unchecked_thumb_color: state_fill_of(
            &s.thumb(disabled),
            "switch.thumb(DISABLED).fill",
        )?,
        checked_track_border: edge(&on, "switch.track(CHECKED).stroke")?,
        unchecked_track_border: edge(&off, "switch.track(NONE).stroke")?,
        disabled_checked_track_border: flagged(
            &s.track(disabled),
            &s.track(disabled_on),
            PartStated::STROKE,
            |p| edge(p, "switch.track(DISABLED|CHECKED).stroke"),
        )?
        .flatten(),
        disabled_unchecked_track_border: {
            let track = s.track(disabled);
            if track.from_state.contains(PartStated::STROKE) {
                edge(&track, "switch.track(DISABLED).stroke")?
            } else {
                None
            }
        },
        thumb_border: edge(&thumb_on, "switch.thumb(CHECKED).stroke")?,
        unchecked_thumb_border: edge(&thumb_off, "switch.thumb(NONE).stroke")?,
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
/// [`super::Unmapped`] where the theme states no window background, or
/// states a value in a form iced cannot draw (a layered fill with no
/// fallback colour, a per-side stroke not reduced, a concentric radius); a fill at rest
/// it leaves unstated is iced's own switch's (spec §10, rule 0).
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
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced, a
/// concentric radius); a fill at rest it leaves unstated is iced's own
/// switch's (spec §10, rule 0).
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
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced); a
/// value at rest it leaves unstated is iced's own field's (spec §10, rule
/// 0).
pub fn pick_list(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::pick_list::Status) -> iced_widget::pick_list::Style + Clone + use<>,
> {
    Ok(old::pick_list_from(&pick_list_source(r)?))
}

/// [`PickListSource`] of the 0.8.0 model.
fn pick_list_source(r: &ResolvedTheme) -> Mapped<PickListSource> {
    let c = &r.combo_box;
    let rest = c.field(WidgetState::NONE);
    let hover = c.field(WidgetState::HOVER);
    let open = c.field(WidgetState::OPEN);
    let open_hover = c.field(WidgetState::OPEN | WidgetState::HOVER);
    Ok(PickListSource {
        background: fill_of(&rest, "combo_box.field(NONE).fill")?,
        hovered_background: state_layer(&hover, "combo_box.field(HOVER).fill")?
            .map(Layer::as_given),
        label: text_color_of(&c.label(WidgetState::NONE)),
        placeholder: text_color_of(&r.input.placeholder(WidgetState::NONE)),
        open_background: state_layer(&open, "combo_box.field(OPEN).fill")?,
        open_hover_background: state_layer(&open_hover, "combo_box.field(OPEN|HOVER).fill")?,
        border: part_outline!(rest, "combo_box.field(NONE)")?,
        hover_border_color: state_line(&hover, "combo_box.field(HOVER).stroke.color")?,
        open_border_color: state_line(&open, "combo_box.field(OPEN).stroke.color")?,
        open_hover_border_color: state_line(
            &open_hover,
            "combo_box.field(OPEN|HOVER).stroke.color",
        )?,
    })
}

/// A part's shadow as iced draws one, where the theme states the part's
/// shadows: iced's menu and container draw one outset shadow by its offset,
/// blur and colour, so it is the first outset layer, its spread and every
/// other layer dropped, and no shadow (`Shadow::default()`, transparent)
/// where the theme states none outset. `None` where the theme states no
/// shadow, iced's own then.
fn shadow(part: &PartStyle) -> Option<Shadow> {
    if !part.stated.contains(PartStated::SHADOWS) {
        return None;
    }
    let outset = part
        .shadows
        .as_deref()
        .and_then(|layers| layers.iter().find(|layer| !layer.inset));
    Some(outset.map_or_else(Shadow::default, |layer| Shadow {
        color: to_color(layer.color),
        offset: Vector::new(layer.offset_x, layer.offset_y),
        blur_radius: layer.blur,
    }))
}

/// [`crate::styles::menu`]: the menu's panel, its items and their labels,
/// framed by the menu's own stroke where the theme states one, else by the
/// popover's (spec §5, notes to the table, the menu's popup frame). iced
/// rounds the popup and the item highlight with one radius, which is
/// `menu.item`'s corners where the theme states them (spec §5,
/// `menu.border`), else `menu.panel`'s (material-3's container shape,
/// platform-facts.md §2.6); the panel's shadow is `shadow`'s. Each value
/// the theme leaves unstated is iced's own.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a frame, corners or highlight
/// iced cannot take; a panel fill or an item label colour it leaves
/// unstated is iced's own menu's (spec §10, rule 0).
pub fn menu(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::overlay::menu::Style + Clone + use<>> {
    Ok(old::menu_from(&menu_source(r)?))
}

/// [`MenuSource`] of the 0.8.0 model.
fn menu_source(r: &ResolvedTheme) -> Mapped<MenuSource> {
    let m = &r.menu;
    let panel = m.panel(WidgetState::NONE);
    let (frame, color_home, width_home) = if panel.stated.contains(PartStated::STROKE) {
        (
            &panel,
            "menu.panel(NONE).stroke.color",
            "menu.panel(NONE).stroke.width",
        )
    } else {
        (
            &r.popover.panel,
            "popover.panel.stroke.color",
            "popover.panel.stroke.width",
        )
    };
    let (line, width) = if frame.stated.contains(PartStated::STROKE) {
        let line = match frame.stroke.prevailing() {
            Stroke::None => None,
            _ => Some(line(frame, color_home)?),
        };
        (line, Some(stroke_width(frame, width_home)?))
    } else {
        (None, None)
    };
    Ok(MenuSource {
        background: fill_of(&panel, "menu.panel(NONE).fill")?,
        label: text_color_of(&m.item_label(WidgetState::NONE)),
        selected_label: state_text_color(
            &m.item_label(WidgetState::HOVER),
            "menu.item_label(HOVER).color",
        )?,
        selected_background: state_fill_of(&m.item(WidgetState::HOVER), "menu.item(HOVER).fill")?,
        line,
        width,
        radius: match radius_or_iced(&m.item(WidgetState::NONE), "menu.item(NONE).corners")? {
            Some(item) => Some(item),
            None => radius_or_iced(&panel, "menu.panel(NONE).corners")?,
        },
        shadow: shadow(&panel),
    })
}

/// A part's stroke at rest as the slider's thumb takes it: its width and
/// its stroke where stated, `None` for each where not or where the stroke
/// is none.
fn thumb_edge(part: &PartStyle) -> Mapped<(Option<f32>, Option<Line>)> {
    if !part.stated.contains(PartStated::STROKE) || matches!(part.stroke.prevailing(), Stroke::None)
    {
        return Ok((None, None));
    }
    Ok((
        Some(stroke_width(part, "slider.thumb(NONE).stroke.width")?),
        Some(line(part, "slider.thumb(NONE).stroke.color")?),
    ))
}

/// [`crate::styles::slider`]: the rail's two fills and its height, the
/// thumb's fill, diameter and strokes, each laid over the fill the thumb
/// paints in that status; under the pointer and dragged, the thumb's own
/// where the theme states it, else iced's handle fill and the stroke at
/// rest.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a fill or a stroke iced cannot
/// take; a rail or thumb fill at rest it leaves unstated is iced's own
/// slider's (spec §10, rule 0).
pub fn slider(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + Clone + use<>,
> {
    slider_across(r, Orientation::Horizontal)
}

/// [`slider`] for `vertical_slider(..).style(..)`, which takes the same
/// `slider::Style` (iced_widget 0.14.2 `src/vertical_slider.rs`): the rail
/// as wide as the vertical track (`slider.track(NONE; orientation=Vertical)
/// .min_size[0]`), where the theme states the track's thickness per
/// orientation.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a fill or a stroke iced cannot
/// take; a rail or thumb fill at rest it leaves unstated is iced's own
/// slider's (spec §10, rule 0).
pub fn vertical_slider(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + Clone + use<>,
> {
    slider_across(r, Orientation::Vertical)
}

/// [`slider`] of a slider laid out along `orientation`: the rail across it
/// the track's thickness that orientation's own query gives (its height
/// horizontal, its width vertical; iced's own where unstated).
fn slider_across(
    r: &ResolvedTheme,
    orientation: Orientation,
) -> Mapped<
    impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + Clone + use<>,
> {
    Ok(old::slider_from(&slider_source(r, orientation)?))
}

/// [`SliderSource`] of the 0.8.0 model, for a slider along `orientation`.
fn slider_source(r: &ResolvedTheme, orientation: Orientation) -> Mapped<SliderSource> {
    let s = &r.slider;
    let [width, height] = s
        .track(StyleQuery::new(WidgetState::NONE).orientation(orientation))
        .min_size;
    let track_height = match orientation {
        Orientation::Vertical => width,
        Orientation::Horizontal => height,
    };
    let thumb = s.thumb(WidgetState::NONE);
    let [thumb_diameter, _] = thumb.min_size;
    let hover = s.thumb(WidgetState::HOVER);
    let pressed = s.thumb(WidgetState::PRESSED);
    let (width, rest_line) = thumb_edge(&thumb)?;
    Ok(SliderSource {
        fill_color: fill_of(
            &s.active_track(WidgetState::NONE),
            "slider.active_track(NONE).fill",
        )?,
        hovered_fill_color: state_fill_of(
            &s.active_track(WidgetState::HOVER),
            "slider.active_track(HOVER).fill",
        )?,
        dragged_fill_color: state_fill_of(
            &s.active_track(WidgetState::PRESSED),
            "slider.active_track(PRESSED).fill",
        )?,
        track_color: fill_of(&s.track(WidgetState::NONE), "slider.track(NONE).fill")?,
        track_height,
        thumb_color: fill_of(&thumb, "slider.thumb(NONE).fill")?,
        hovered_thumb: state_layer(&hover, "slider.thumb(HOVER).fill")?.map(Layer::as_given),
        pressed_thumb: state_layer(&pressed, "slider.thumb(PRESSED).fill")?.map(Layer::as_given),
        thumb_diameter,
        thumb_border_line_width: width,
        thumb_border_color: rest_line,
        thumb_hover_border_color: state_line(&hover, "slider.thumb(HOVER).stroke.color")?,
        thumb_pressed_border_color: state_line(&pressed, "slider.thumb(PRESSED).stroke.color")?,
    })
}

/// [`crate::styles::scrollable`]: the rail's fill, the scroller at rest (the
/// handle of the view `view` says: `thumb(NONE)` in the view that has the
/// focus, `thumb(UNFOCUSED_VIEW)` where `view` holds `UNFOCUSED_VIEW`, as a
/// connector sets it only where the view lacks the focus, spec §5, S3;
/// spec §3, `WidgetState`; iced's scrollable reports no view focus, so the
/// caller passes it, `UNFOCUSED_VIEW` while the window is inactive; no other
/// bit of `view` is read), under the pointer and dragged (`thumb(PRESSED)`,
/// spec §5, `scrollbar.thumb_active_color`), with its outline and corners
/// where the theme states them (Breeze's pen round a pill), each value the
/// theme states nothing for iced's own, and whether the bar floats.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a fill, a stroke or corners
/// iced cannot take; a rail or scroller fill at rest it leaves unstated is
/// iced's own scrollable's (spec §10, rule 0).
pub fn scrollable(
    r: &ResolvedTheme,
    view: WidgetState,
) -> Mapped<
    impl Fn(&Theme, iced_widget::scrollable::Status) -> iced_widget::scrollable::Style + Clone + use<>,
> {
    Ok(old::scrollable_from(&scrollbar_source(r, view)?))
}

/// [`ScrollbarSource`] of the 0.8.0 model, in the view `view` says.
fn scrollbar_source(r: &ResolvedTheme, view: WidgetState) -> Mapped<ScrollbarSource> {
    let s = &r.scrollbar;
    let (rest, fill_home, stroke_home, corners_home) = if view.contains(WidgetState::UNFOCUSED_VIEW)
    {
        (
            WidgetState::UNFOCUSED_VIEW,
            "scrollbar.thumb(UNFOCUSED_VIEW).fill",
            "scrollbar.thumb(UNFOCUSED_VIEW).stroke",
            "scrollbar.thumb(UNFOCUSED_VIEW).corners",
        )
    } else {
        (
            WidgetState::NONE,
            "scrollbar.thumb(NONE).fill",
            "scrollbar.thumb(NONE).stroke",
            "scrollbar.thumb(NONE).corners",
        )
    };
    let thumb = s.thumb(rest);
    let hover = s.thumb(WidgetState::HOVER);
    let pressed = s.thumb(WidgetState::PRESSED);
    Ok(ScrollbarSource {
        track: fill_of(&s.track(WidgetState::NONE), "scrollbar.track(NONE).fill")?,
        thumb: fill_of(&thumb, fill_home)?,
        hovered_thumb: state_fill_of(&hover, "scrollbar.thumb(HOVER).fill")?,
        dragged_thumb: state_fill_of(&pressed, "scrollbar.thumb(PRESSED).fill")?,
        overlay_mode: s.overlay_mode,
        thumb_edge: edge(&thumb, stroke_home)?,
        thumb_radius: radius_or_iced(&thumb, corners_home)?.map(Radius::new),
        hovered_thumb_line: state_line(&hover, "scrollbar.thumb(HOVER).stroke.color")?,
        dragged_thumb_line: state_line(&pressed, "scrollbar.thumb(PRESSED).stroke.color")?,
    })
}

/// [`crate::styles::scrollbar`]: the groove's and the thumb's thickness
/// across a vertical bar (`min_size[0]`, which the model states equal to
/// `max_size[0]` and to a horizontal bar's `min_size[1]`, spec §5), iced's
/// own where unstated, and whether the bar floats.
#[must_use = "this returns the configured scrollbar; it does not apply it"]
pub fn scrollbar(r: &ResolvedTheme) -> iced_widget::scrollable::Scrollbar {
    let s = &r.scrollbar;
    let vertical = StyleQuery::new(WidgetState::NONE).orientation(Orientation::Vertical);
    let [groove_width, _] = s.track(vertical).min_size;
    let [thumb_width, _] = s.thumb(vertical).min_size;
    old::scrollbar_from(groove_width, thumb_width, s.overlay_mode)
}

/// [`SegmentedSource`] of the 0.8.0 model. The line between two segments
/// is the separator part's fill (spec §5, notes to the table: a separator
/// part is the line itself), and where the theme states none, the
/// outline's colour, the one line colour 0.7.2 states for the control (the
/// table of homes, `segmented_control.border.color`); no line where the
/// outline is stated as none, or left unstated, iced's own container's,
/// which draws none. The group fill, the labels, the outline and the
/// selected segment's fill at rest are each iced's own where the theme
/// leaves them unstated (spec §10, rule 0).
fn segmented_source(r: &ResolvedTheme) -> Mapped<SegmentedSource> {
    let s = &r.segmented_control;
    let track = s.track(WidgetState::NONE);
    let separator = s.separator(WidgetState::NONE);
    Ok(SegmentedSource {
        window_background: to_color(stated(
            r.colors.window.background,
            "colors.window.background",
        )?),
        background: fill_of(&track, "segmented_control.track(NONE).fill")?,
        label: text_color_of(&s.label(WidgetState::NONE)),
        border: part_outline!(track, "segmented_control.track(NONE)")?,
        separator: if separator.stated.contains(PartStated::FILL) {
            Some(fill(&separator, "segmented_control.separator(NONE).fill")?)
        } else if !track.stated.contains(PartStated::STROKE)
            || matches!(track.stroke.prevailing(), Stroke::None)
        {
            None
        } else {
            Some(line(&track, "segmented_control.track(NONE).stroke.color")?.color)
        },
        active_background: fill_of(
            &s.segment(WidgetState::SELECTED),
            "segmented_control.segment(SELECTED).fill",
        )?,
        active_text_color: text_color_of(&s.label(WidgetState::SELECTED)),
        hover_background: state_layer(
            &s.segment(WidgetState::HOVER),
            "segmented_control.segment(HOVER).fill",
        )?
        .map(Layer::as_given),
        pressed_background: state_layer(
            &s.segment(WidgetState::PRESSED),
            "segmented_control.segment(PRESSED).fill",
        )?
        .map(Layer::as_given),
        disabled_opacity: s.disabled_opacity,
        shadow: shadow(&track),
    })
}

/// [`crate::styles::segmented_control`]: the group fill and the outline,
/// its stroke laid over the group by its alignment, and the track's shadow
/// as `shadow` gives it.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background, or
/// states a value in a form iced cannot draw (a layered fill with no
/// fallback colour, a per-side stroke not reduced, a concentric radius); a value at
/// rest it leaves unstated is iced's own container's (spec §10, rule 0).
pub fn segmented_control(
    r: &ResolvedTheme,
    disabled: bool,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    Ok(old::segmented_control_from(&segmented_source(r)?, disabled))
}

/// [`crate::styles::segment_separator`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background, or
/// states a value in a form iced cannot draw (a layered fill with no
/// fallback colour, a per-side stroke not reduced, a concentric radius).
pub fn segment_separator(
    r: &ResolvedTheme,
    disabled: bool,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    Ok(old::segment_separator_from(&segmented_source(r)?, disabled))
}

/// [`crate::styles::segment`]: a hovered or pressed segment's fill where
/// the theme states one for the state, else no fill: iced's own for a
/// segment, `button::Style::default()`, changes nothing between statuses.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no window background, or
/// states a value in a form iced cannot draw (a layered fill with no
/// fallback colour, a per-side stroke not reduced, a concentric radius); a fill or a
/// label at rest it leaves unstated is iced's own segment's (spec §10, rule
/// 0).
pub fn segment(
    r: &ResolvedTheme,
    selected: bool,
    position: SegmentPosition,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::segment_from(&segmented_source(r)?, selected, position))
}

/// [`ExpanderSource`] of the 0.8.0 model.
fn expander_source(r: &ResolvedTheme) -> Mapped<ExpanderSource> {
    let x = &r.expander;
    let frame = x.container(WidgetState::NONE);
    Ok(ExpanderSource {
        hover_background: state_layer(
            &x.header(WidgetState::HOVER),
            "expander.header(HOVER).fill",
        )?
        .map(Layer::as_given),
        pressed_background: state_layer(
            &x.header(WidgetState::PRESSED),
            "expander.header(PRESSED).fill",
        )?
        .map(Layer::as_given),
        label: text_color_of(&x.header_label(WidgetState::NONE)),
        disabled_opacity: r.colors.disabled_opacity,
        border: part_outline!(frame, "expander.container(NONE)")?,
        frame_enabled: x.frame_enabled,
        shadow: shadow(&frame),
    })
}

/// [`crate::styles::expander`]: a hovered or pressed header's fill where
/// the theme states one for the state, else no fill.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced, a
/// concentric radius); a label or a frame property it leaves unstated is
/// iced's own (spec §10, rule 0).
pub fn expander(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::expander_from(&expander_source(r)?))
}

/// [`crate::styles::expander_frame`], with the container's shadow as
/// `shadow` gives it where the frame is drawn.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a value in a form iced cannot
/// draw (a layered fill with no fallback colour, a per-side stroke not reduced, a
/// concentric radius); a frame property it leaves unstated is iced's own
/// container's (spec §10, rule 0).
pub fn expander_frame(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    Ok(old::expander_frame_from(&expander_source(r)?))
}

/// [`crate::styles::progress_bar`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a stroke or corners iced
/// cannot take; a track or bar fill, a stroke or corners it leaves
/// unstated are iced's own bar's (spec §10, rule 0).
pub fn progress_bar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::progress_bar::Style + Clone + use<>> {
    let (track, filled, border) = progress_bar_source(r)?;
    Ok(old::progress_bar_from(track, filled, border))
}

/// What [`progress_bar`] hands [`old::progress_bar_from`]: the track's and
/// the bar's fills and the track's border, each `None` where unstated.
type ProgressBarSource = (
    Option<iced_core::Color>,
    Option<iced_core::Color>,
    PartOutline,
);

/// [`ProgressBarSource`] of the 0.8.0 model.
fn progress_bar_source(r: &ResolvedTheme) -> Mapped<ProgressBarSource> {
    let p = &r.progress_bar;
    let track = p.track(WidgetState::NONE);
    Ok((
        fill_of(&track, "progress_bar.track(NONE).fill")?,
        fill_of(&p.fill(WidgetState::NONE), "progress_bar.fill(NONE).fill")?,
        part_outline!(track, "progress_bar.track(NONE)")?,
    ))
}

/// [`crate::styles::rule`]: the separator's colour.
#[must_use = "this returns the style function; it does not apply it"]
pub fn rule(r: &ResolvedTheme) -> impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<> {
    old::rule_from(Some(to_color(r.separator.color)))
}

/// [`crate::styles::toolbar_rule`]: the toolbar's stroke colour as stated,
/// iced's own rule's where the theme leaves it unstated (spec §10, rule 0);
/// a stroke stated as none is no rule, drawn transparent. The rule is a
/// line of its own under the bar, not a border over the bar's fill, so no
/// fill is composited under it.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states a toolbar stroke per side and
/// [`Stroke::prevailing`] does not reduce it: iced's rule is one line (spec
/// §10, rule 4).
pub fn toolbar_rule(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<>> {
    let bar = r.toolbar.bar(WidgetState::NONE);
    let line = match &bar.stroke.prevailing() {
        _ if !bar.stated.contains(PartStated::STROKE) => None,
        Stroke::None => Some(iced_core::Color::TRANSPARENT),
        Stroke::Uniform { color, .. } => Some(to_color(*color)),
        _ => return Err(Unmapped("toolbar.bar(NONE).stroke.color")),
    };
    Ok(old::rule_from(line))
}

/// [`crate::styles::pane_grid`]: the splitter's handle under the pointer
/// and its divider width; a handle fill the theme leaves unstated is iced's
/// own split's (spec §10, rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states the hovered handle's fill in
/// a form iced cannot paint (a layered fill with no fallback colour).
pub fn pane_grid(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::pane_grid::Style + Clone + use<>> {
    let s = &r.splitter;
    let color = fill_of(&s.handle(WidgetState::HOVER), "splitter.handle(HOVER).fill")?;
    Ok(old::pane_grid_from(color, s.divider_width))
}

/// [`crate::styles::tooltip`]: the panel's fill, the text colour and the
/// panel's border, each iced's own container's (`container::Style::default()`:
/// no fill, the inherited text colour, no line and square corners) where the
/// theme leaves it unstated (spec §10, rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where the panel's fill or border is stated in a form
/// iced cannot draw (a layered fill with no fallback colour, a per-side
/// stroke not reduced, a concentric radius).
pub fn tooltip(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let t = &r.tooltip;
    Ok(old::panel_from(
        fill_of(&t.panel, "tooltip.panel.fill")?,
        text_color_of(&t.text),
        Some(outline_or_iced!(t.panel, "tooltip.panel")?),
    ))
}

/// [`crate::styles::container_card`]: the card's fill and border, each iced's
/// own container's where the theme states none (the table of homes,
/// `card.border.*`: `unstated_if_unset`; spec §10, rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where the card's fill or border is stated in a form
/// iced cannot draw.
pub fn container_card(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let card = r.card.container(WidgetState::NONE);
    Ok(old::panel_from(
        fill_of(&card, "card.container(NONE).fill")?,
        None,
        Some(outline_or_iced!(card, "card.container(NONE)")?),
    ))
}

/// [`crate::styles::dialog`]: the panel's fill, the body colour and the
/// panel's border, each iced's own container's where the theme leaves it
/// unstated (spec §10, rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where the panel's fill or border is stated in a form
/// iced cannot draw.
pub fn dialog(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let d = &r.dialog;
    Ok(old::panel_from(
        fill_of(&d.panel, "dialog.panel.fill")?,
        text_color_of(&d.body),
        Some(outline_or_iced!(d.panel, "dialog.panel")?),
    ))
}

/// [`crate::styles::toolbar`]: the bar's fill and the button label's colour,
/// each iced's own container's where the theme leaves it unstated (spec
/// §10, rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states the bar's fill in a form
/// iced cannot paint (a layered fill with no fallback colour).
pub fn toolbar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let t = &r.toolbar;
    Ok(old::panel_from(
        fill_of(&t.bar(WidgetState::NONE), "toolbar.bar(NONE).fill")?,
        text_color_of(&t.button_label(WidgetState::NONE)),
        None::<Outline>,
    ))
}

/// [`crate::styles::application`]: the fill of a normal window's frame,
/// the theme's base style's where the theme leaves it unstated (spec §10,
/// rule 0).
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states the frame's fill in a form
/// iced cannot paint (a layered fill with no fallback colour).
pub fn application(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_core::theme::Style + Clone + use<>> {
    let background = fill_of(
        &r.window.frame(WindowState::default()),
        "window.frame(normal).fill",
    )?;
    Ok(old::application_from(background))
}

#[cfg(feature = "iced_aw")]
pub mod aw;

#[cfg(test)]
mod tests;
