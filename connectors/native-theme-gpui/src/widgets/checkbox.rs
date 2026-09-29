//! `Checkbox`, `Radio` and `RadioGroup` (spec §2.1, §2.2): gpui-base's
//! headless checkbox and radio, painted from `CheckboxTheme`.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Axis, Bounds, ElementId, FontWeight, Hsla, InteractiveElement, IntoElement,
    MouseButton, ParentElement, PathBuilder, Pixels, RenderOnce, SharedString, StyleRefinement,
    Styled, Window, canvas, div, fill, point, prelude::FluentBuilder as _, px, relative, svg,
};
use gpui_base::{
    Checkbox as BaseCheckbox, Radio as BaseRadio, RadioGroup as BaseRadioGroup, spring,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, IconName, IconNamed as _, StyledExt as _, ThemeStyled as _,
};
use native_theme::theme::ResolvedTheme;

use super::{Part, PartBounds, color, length, native, over, part_bounds, text_size};

/// The group name the indicator's hover style listens to: the whole row
/// (indicator and label) is the control the pointer hovers, as on the
/// platforms.
const HOVER_GROUP: &str = "native-theme-checkbox";

/// What a checkbox or a radio paints, per state, from `CheckboxTheme`
/// (spec §2.1). The render reads only this; the tests check it leaf by leaf.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckboxLook {
    /// `checkbox.indicator_width`: the square's side, or the circle's
    /// diameter (docs/platform-facts.md:980).
    pub indicator: Pixels,
    /// `checkbox.radio_dot_diameter`: the dot a selected radio draws, or
    /// `None` where the theme states none or a length that is not finite --
    /// the radio then draws gpui-component's own mark.
    pub dot: Option<Pixels>,
    /// `checkbox.border.corner_radius` (a radio is round instead).
    pub radius: Pixels,
    /// `checkbox.border.line_width`.
    pub border_width: Pixels,
    /// The indicator's fill in this state.
    pub fill: Hsla,
    /// The fill while the pointer is over the control: `hover_background`
    /// composited over the unchecked fill, and only for an enabled, unchecked
    /// control -- the model states no hover for a checked one.
    pub hover_fill: Option<Hsla>,
    /// The indicator's border colour in this state.
    pub border: Hsla,
    /// The check mark's colour.
    pub mark: Hsla,
    /// `checkbox.check_mark_stroke_width`: the check mark's line, or `None`
    /// where the theme states none or a length that is not finite -- the
    /// checkbox then draws gpui-component's own `Check` icon, at its own
    /// stroke.
    pub mark_stroke: Option<Pixels>,
    /// The side of the square a checkbox's mark is drawn in: the indicator
    /// inside `checkbox.border.padding` where the theme states it, and
    /// inside its border on a side it leaves unstated -- "checkmark fills
    /// indicator" (docs/platform-facts.md:1216, §2.5).
    pub mark_size: Pixels,
    /// Whether the theme states any side of `checkbox.border.padding`: the
    /// mark's box is then that square, where it is otherwise the box its
    /// line paints.
    pub mark_padded: bool,
    /// The label's colour.
    pub label: Hsla,
    /// `checkbox.label_gap`.
    pub label_gap: Pixels,
    /// The whole control's opacity: `checkbox.disabled_opacity` when
    /// disabled, on top of the disabled colours, else 1.
    pub opacity: f32,
}

impl CheckboxLook {
    /// The look of a control `checked` or not, `disabled` or not, or `None`
    /// when a length the theme gives is not finite (the widget then falls
    /// back to gpui-component's).
    ///
    /// The reading of `CheckboxTheme` is the iced connector's
    /// (`native-theme-iced` `styles::checkbox`), so the two draw one control:
    /// both soft fills copy `background_color` when unstated; a checked box is
    /// bordered in `border.color`, an unchecked one in
    /// `unchecked_border_color`; `disabled_background` replaces the fill in
    /// either state, and there the mark takes `disabled_text_color`, because
    /// `indicator_color` is the colour stated for a foreground on the accent
    /// the disabled fill no longer shows. Where no disabled fill is stated the
    /// platform dims by opacity alone, and a disabled box is its enabled self.
    /// Either way a disabled control is faded by `checkbox.disabled_opacity`
    /// (docs/platform-facts.md §2.1.6).
    #[must_use]
    pub fn of(resolved: &ResolvedTheme, checked: bool, disabled: bool) -> Option<Self> {
        let c = &resolved.checkbox;
        let unchecked = color(c.unchecked_background.unwrap_or(c.background_color));
        let enabled_fill = if checked {
            color(c.checked_background)
        } else {
            unchecked
        };
        let fill = match (disabled, c.disabled_background) {
            (true, Some(fill)) => color(fill),
            _ => enabled_fill,
        };
        let hover_fill = (!checked && !disabled).then(|| {
            over(
                unchecked,
                color(c.hover_background.unwrap_or(c.background_color)),
            )
        });
        let border = if checked {
            color(c.border.color)
        } else {
            color(c.unchecked_border_color.unwrap_or(c.border.color))
        };
        let (mark, label) = match (disabled, c.disabled_background) {
            (true, Some(_)) => (color(c.disabled_text_color), color(c.disabled_text_color)),
            (true, None) => (color(c.indicator_color), color(c.disabled_text_color)),
            (false, _) => (color(c.indicator_color), color(c.font.color)),
        };
        let opacity = super::disabled_opacity(disabled, c.disabled_opacity);
        let indicator = length(c.indicator_width)?;
        let border_width = length(c.border.line_width)?;
        let p = &c.border.padding;
        let inset = |side: Option<f32>| {
            side.filter(|v| v.is_finite() && *v >= 0.0)
                .unwrap_or(c.border.line_width)
        };
        let across = c.indicator_width - inset(p.left) - inset(p.right);
        let down = c.indicator_width - inset(p.top) - inset(p.bottom);
        let mark_size = length(across.min(down).max(0.0))?;
        let mark_padded = [p.top, p.right, p.bottom, p.left]
            .iter()
            .any(Option::is_some);
        Some(Self {
            indicator,
            dot: c.radio_dot_diameter.and_then(length),
            radius: length(c.border.corner_radius)?,
            border_width,
            fill,
            hover_fill,
            border,
            mark,
            mark_stroke: c.check_mark_stroke_width.and_then(length),
            mark_size,
            mark_padded,
            label,
            label_gap: length(c.label_gap)?,
            opacity,
        })
    }
}

type ChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App)>;

/// The parts a checkbox and a radio share.
struct Parts {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    observer: Option<PartBounds>,
}

impl Parts {
    fn new(id: ElementId) -> Self {
        Self {
            id,
            label: None,
            checked: false,
            disabled: false,
            on_change: None,
            observer: None,
        }
    }
}

/// gpui-component's check mark, the Lucide `check` icon it draws as
/// `IconName::Check` (gpui-kit-assets 0.6.6 `assets/icons/check.svg`,
/// `M20 6 9 17l-5-5`): a polyline through these points of its view box,
/// round-capped and round-joined (`stroke-linecap`, `stroke-linejoin`).
const CHECK_POINTS: [(f32, f32); 3] = [(20., 6.), (9., 17.), (4., 12.)];

/// The side of that icon's view box (`viewBox="0 0 24 24"`).
const CHECK_VIEW_BOX: f32 = 24.;

/// gpui-component's check mark `size` square, its line `stroke` wide in
/// `colour`, faded to `opacity`: the icon's polyline scaled to the box and
/// stroked at the width the theme states, where the icon file strokes it at
/// its own two view-box units. Its round caps and join are discs as wide as
/// the line on the three points.
fn stroked_check(size: Pixels, stroke: Pixels, colour: Hsla, opacity: f32) -> AnyElement {
    let colour = Hsla {
        a: colour.a * opacity,
        ..colour
    };
    let mark = canvas(
        move |bounds: Bounds<Pixels>, _, _| bounds,
        move |_, bounds, window, _| {
            if opacity <= 0. {
                return;
            }
            let scale = f32::from(size) / CHECK_VIEW_BOX;
            let at = |(x, y): (f32, f32)| {
                point(
                    px(f32::from(bounds.origin.x) + x * scale),
                    px(f32::from(bounds.origin.y) + y * scale),
                )
            };
            let mut path = PathBuilder::stroke(stroke);
            let [first, rest @ ..] = CHECK_POINTS;
            path.move_to(at(first));
            for p in rest {
                path.line_to(at(p));
            }
            if let Ok(path) = path.build() {
                window.paint_path(path, colour);
            }
            let r = f32::from(stroke) / 2.;
            for p in CHECK_POINTS {
                let c = at(p);
                window.paint_quad(
                    fill(
                        Bounds::new(
                            point(px(f32::from(c.x) - r), px(f32::from(c.y) - r)),
                            gpui::size(stroke, stroke),
                        ),
                        colour,
                    )
                    .corner_radii(px(r)),
                );
            }
        },
    )
    .size_full();
    div()
        .size(size)
        .flex_none()
        .debug_selector(|| "native-checkbox-mark".into())
        .child(mark)
        .into_any_element()
}

/// The box `stroked_check` paints in a `size` square with a line `stroke`
/// wide, as (left, top, width, height) in that square: its points scaled
/// to the square, and half the line round them, where its round caps and
/// join reach.
fn stroked_check_bounds(size: f32, stroke: f32) -> (f32, f32, f32, f32) {
    let scale = size / CHECK_VIEW_BOX;
    let (mut left, mut top, mut right, mut bottom) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for (x, y) in CHECK_POINTS {
        left = left.min(x);
        top = top.min(y);
        right = right.max(x);
        bottom = bottom.max(y);
    }
    let r = stroke / 2.;
    (
        left * scale - r,
        top * scale - r,
        (right - left) * scale + stroke,
        (bottom - top) * scale + stroke,
    )
}

/// The indicator box, its mark and the label of a checkbox or radio in
/// `look`, round when `round`.
fn indicator_and_label(
    parts: &Parts,
    look: &CheckboxLook,
    round: bool,
    font: (Pixels, FontWeight, f32),
    window: &mut Window,
    cx: &mut App,
) -> (AnyElement, Option<AnyElement>) {
    // gpui-component fades the mark in and out on a spring rather than
    // swapping it (checkbox.rs:186-192); gpui holds it still under reduced
    // motion.
    let opacity = spring(
        (parts.id.clone(), "mark"),
        if parts.checked { 1. } else { 0. },
        cx.theme().motion_tokens().spring_control,
        window,
        cx,
    );
    let observer = parts.observer.as_ref();
    let mark = match look.dot.filter(|_| round) {
        // A radio's dot, `radio_dot_diameter` across, centred in the circle
        // (docs/platform-facts.md:1220, §2.5).
        Some(dot) => div()
            .size(dot)
            .flex_none()
            .rounded_full()
            .when(opacity > 0., |mark| mark.bg(look.mark).opacity(opacity))
            .debug_selector(|| "native-radio-dot".into())
            .into_any_element(),
        // The mark fills the indicator inside its padding, or its border:
        // "checkmark fills indicator" (docs/platform-facts.md:1216, §2.5).
        None => {
            let inner = look.mark_size;
            match look.mark_stroke {
                // gpui-component's own glyph, stroked as the theme states
                // (docs/platform-facts.md:1221, §2.5).
                Some(stroke) => stroked_check(inner, stroke, look.mark, opacity),
                None => svg()
                    .size(inner)
                    .flex_none()
                    .text_color(look.mark)
                    .when(opacity > 0., |mark| {
                        mark.path(IconName::Check.path()).opacity(opacity)
                    })
                    .into_any_element(),
            }
        }
    };
    // Where its bounds are asked for, the mark in a box of its own size
    // that reports them: the stroked check's the box its line paints, the
    // polyline's points and its round ends half the line out from them;
    // the dot's and the icon's their whole box.
    let mark = match observer {
        Some(observer) => {
            // The box its line paints; the whole square where the theme
            // states the padding round it.
            let stroked = look
                .mark_stroke
                .filter(|_| !(round && look.dot.is_some()) && !look.mark_padded)
                .map(|stroke| stroked_check_bounds(f32::from(look.mark_size), f32::from(stroke)));
            let reported = part_bounds(Part::Mark, observer, px(0.)).into_any_element();
            let reported = match stroked {
                Some((left, top, width, height)) => div()
                    .absolute()
                    .left(px(left))
                    .top(px(top))
                    .w(px(width))
                    .h(px(height))
                    .child(reported)
                    .into_any_element(),
                None => reported,
            };
            div()
                .flex_none()
                .relative()
                .child(mark)
                .child(reported)
                .into_any_element()
        }
        None => mark,
    };
    let rounded = |box_: gpui::Div, radius: Pixels| {
        if round {
            box_.rounded_full()
        } else {
            box_.rounded(radius)
        }
    };
    // Faded, the fill is a box of its own inside the edge rather than the
    // indicator's own background: gpui fades each quad it paints on its own
    // (gpui-pre src/window.rs:4513-4521, `paint_quad`) and draws a quad's
    // border over its own background (gpui-pre-wgpu src/shaders.wgsl:887,
    // `fs_quad`), so the faded fill would show through the faded edge, where
    // a platform fades the control as one. Unfaded, both are opaque and one
    // quad draws them.
    let inner_fill = (look.opacity < 1.).then(|| {
        let inset = look.border_width;
        rounded(
            div()
                .absolute()
                .top(inset)
                .left(inset)
                .right(inset)
                .bottom(inset)
                .bg(look.fill),
            px((f32::from(look.radius) - f32::from(inset)).max(0.)),
        )
        .into_any_element()
    });
    let indicator = rounded(
        div()
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(look.indicator)
            .border(look.border_width)
            .border_color(look.border)
            .when(inner_fill.is_none(), |box_| box_.bg(look.fill)),
        look.radius,
    )
    .when_some(look.hover_fill, |box_, hover| {
        box_.group_hover(HOVER_GROUP, move |style| style.bg(hover))
    })
    .debug_selector(|| "native-checkbox-indicator".into())
    .children(inner_fill)
    .child(mark)
    .children(observer.map(|o| part_bounds(Part::Indicator, o, look.border_width)))
    .into_any_element();
    let (size, weight, line_height) = font;
    let label = parts.label.clone().map(|label| {
        div()
            .relative()
            .text_size(size)
            .font_weight(weight)
            .line_height(relative(line_height))
            .text_color(look.label)
            .debug_selector(|| "native-checkbox-label".into())
            .child(label)
            .children(observer.map(|o| part_bounds(Part::Label, o, px(0.))))
            .into_any_element()
    });
    (indicator, label)
}

/// A checkbox whose indicator, mark, label and states are `CheckboxTheme`'s
/// (spec §2.1), on gpui-base's headless `Checkbox`: click, Enter and Space
/// toggle it, a pointer press does not move focus, a disabled one is inert,
/// and it reports itself as a checkbox with its toggled state.
///
/// Controlled, as gpui-component's: [`Checkbox::on_change`] receives the
/// requested value, and the owner writes it back. Without a native theme it
/// renders gpui-component's `Checkbox`.
pub struct Checkbox {
    parts: Parts,
}

impl Checkbox {
    /// A new, unchecked checkbox.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            parts: Parts::new(id.into()),
        }
    }

    /// The label drawn beside the indicator, and the name a screen reader
    /// announces.
    #[must_use]
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.parts.label = Some(label.into());
        self
    }

    /// Whether it is checked.
    #[must_use]
    pub fn checked(mut self, checked: bool) -> Self {
        self.parts.checked = checked;
        self
    }

    /// Whether it is disabled: inert, unfocusable, in the disabled colours.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.parts.disabled = disabled;
        self
    }

    /// Called with the requested checked value on activation.
    #[must_use]
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.parts.on_change = Some(Rc::new(handler));
        self
    }

    /// Hands `observer` the bounds of the box, the check mark and the label
    /// as each frame lays them out ([`PartBounds`]).
    #[must_use]
    pub fn on_part_bounds(mut self, observer: PartBounds) -> Self {
        self.parts.observer = Some(observer);
        self
    }

    fn fallback(self) -> AnyElement {
        let parts = self.parts;
        gpui_component::checkbox::Checkbox::new(parts.id)
            .when_some(parts.label, |checkbox, label| checkbox.label(label))
            .checked(parts.checked)
            .disabled(parts.disabled)
            .when_some(parts.on_change, |checkbox, on_change| {
                checkbox.on_click(move |checked, window, cx| on_change(checked, window, cx))
            })
            .into_any_element()
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(n) = native(cx) else {
            return self.fallback();
        };
        let Some(look) = CheckboxLook::of(n.resolved, self.parts.checked, self.parts.disabled)
        else {
            return self.fallback();
        };
        let font = &n.resolved.checkbox.font;
        let font = (
            text_size(font.size, n),
            FontWeight(f32::from(font.weight)),
            n.resolved.defaults.line_height,
        );
        let (indicator, label) = indicator_and_label(&self.parts, &look, false, font, window, cx);
        let Parts {
            id,
            label: name,
            checked,
            disabled,
            on_change,
            observer: _,
        } = self.parts;
        let focus_handle = window
            .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = focus_handle.is_focused(window);
        BaseCheckbox::new(id)
            .checked(checked)
            .disabled(disabled)
            .track_focus(&focus_handle)
            .when_some(name, |checkbox, name| checkbox.accessibility_label(name))
            .when_some(on_change, |checkbox, on_change| {
                checkbox.on_change(move |_, _, window, cx| {
                    window.prevent_default();
                    on_change(&!checked, window, cx);
                })
            })
            .group(HOVER_GROUP)
            .flex()
            .flex_row()
            .items_center()
            .gap(look.label_gap)
            // The row the focus ring follows, as gpui-component rounds it
            // (checkbox.rs:280).
            .rounded(px(f32::from(cx.theme().radius) / 2.))
            .when(focused, |row| row.focus_ring_style(window, cx))
            .on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
            .opacity(look.opacity)
            .child(indicator)
            .children(label)
            .into_any_element()
    }
}

/// A radio button: [`Checkbox`]'s look in a circle (docs/platform-facts.md
/// §2.5, :1223), on gpui-base's headless `Radio`. Activating an unchecked
/// one requests `true`; a checked one does nothing. The mark is a dot
/// `checkbox.radio_dot_diameter` across in `indicator_color`, centred in the
/// circle (:1220); where the theme states no dot size (macOS publishes none),
/// it is gpui-component's own check glyph (radio.rs:242).
pub struct Radio {
    parts: Parts,
    position: Option<(usize, usize)>,
}

impl Radio {
    /// A new, unchecked radio.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            parts: Parts::new(id.into()),
            position: None,
        }
    }

    /// The label drawn beside the indicator.
    #[must_use]
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.parts.label = Some(label.into());
        self
    }

    /// Whether it is the selected one of its group.
    #[must_use]
    pub fn checked(mut self, checked: bool) -> Self {
        self.parts.checked = checked;
        self
    }

    /// Whether it is disabled.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.parts.disabled = disabled;
        self
    }

    /// Its one-based position in a group of `size`, for assistive
    /// technology ("option 2 of 5").
    #[must_use]
    pub fn set_position(mut self, position: usize, size: usize) -> Self {
        self.position = Some((position, size));
        self
    }

    /// Called with `true` when an unchecked radio is activated.
    #[must_use]
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.parts.on_change = Some(Rc::new(handler));
        self
    }

    /// Hands `observer` the bounds of the circle, the dot and the label as
    /// each frame lays them out ([`PartBounds`]).
    #[must_use]
    pub fn on_part_bounds(mut self, observer: PartBounds) -> Self {
        self.parts.observer = Some(observer);
        self
    }

    fn fallback(self) -> AnyElement {
        let parts = self.parts;
        gpui_component::radio::Radio::new(parts.id)
            .when_some(parts.label, |radio, label| radio.label(label))
            .checked(parts.checked)
            .disabled(parts.disabled)
            .when_some(parts.on_change, |radio, on_change| {
                radio.on_click(move |checked, window, cx| on_change(checked, window, cx))
            })
            .into_any_element()
    }
}

impl RenderOnce for Radio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(n) = native(cx) else {
            return self.fallback();
        };
        let Some(look) = CheckboxLook::of(n.resolved, self.parts.checked, self.parts.disabled)
        else {
            return self.fallback();
        };
        let font = &n.resolved.checkbox.font;
        let font = (
            text_size(font.size, n),
            FontWeight(f32::from(font.weight)),
            n.resolved.defaults.line_height,
        );
        let (indicator, label) = indicator_and_label(&self.parts, &look, true, font, window, cx);
        let Parts {
            id,
            label: name,
            checked,
            disabled,
            on_change,
            observer: _,
        } = self.parts;
        let focus_handle = window
            .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = focus_handle.is_focused(window);
        BaseRadio::new(id)
            .checked(checked)
            .disabled(disabled)
            .track_focus(&focus_handle)
            .when_some(name, |radio, name| radio.accessibility_label(name))
            .when_some(self.position, |radio, (position, size)| {
                radio.set_position(position, size)
            })
            .when_some(on_change, |radio, on_change| {
                radio.on_change(move |next, _, window, cx| {
                    window.prevent_default();
                    on_change(&next, window, cx);
                })
            })
            .group(HOVER_GROUP)
            .flex()
            .flex_row()
            .items_center()
            .gap(look.label_gap)
            .rounded(px(f32::from(cx.theme().radius) / 2.))
            .when(focused, |row| row.focus_ring_style(window, cx))
            .on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
            .opacity(look.opacity)
            .child(indicator)
            .children(label)
            .into_any_element()
    }
}

/// A group of [`Radio`]s: gpui-base's `RadioGroup` (role `RadioGroup`, its
/// orientation), each radio told its position in the set. It lays its radios
/// out with the caller's style only: the model states no space between
/// them.
pub struct RadioGroup {
    id: ElementId,
    axis: Axis,
    style: StyleRefinement,
    radios: Vec<Radio>,
}

impl RadioGroup {
    /// A new, vertical group.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            axis: Axis::Vertical,
            style: StyleRefinement::default(),
            radios: Vec::new(),
        }
    }

    /// Lays the radios out along `axis`.
    #[must_use]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Adds the radios, in order.
    #[must_use]
    pub fn children(mut self, radios: impl IntoIterator<Item = Radio>) -> Self {
        self.radios.extend(radios);
        self
    }
}

impl Styled for RadioGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let total = self.radios.len();
        let flex = match self.axis {
            Axis::Vertical => div().flex().flex_col(),
            Axis::Horizontal => div().flex().flex_row(),
        };
        BaseRadioGroup::new(self.id).axis(self.axis).child(
            flex.refine_style(&self.style).children(
                self.radios
                    .into_iter()
                    .enumerate()
                    .map(|(ix, radio)| radio.set_position(ix.saturating_add(1), total)),
            ),
        )
    }
}
