//! `Slider` (spec §2.4): gpui-base's headless slider state, rail, travel and
//! thumb, painted from `SliderTheme`, and made focusable with keyboard steps.

use gpui::{
    AccessibleAction, AnyElement, App, Axis, Div, ElementId, Entity, Hsla, InteractiveElement,
    IntoElement, KeyDownEvent, Length, MouseButton, Orientation, ParentElement, Pixels, RenderOnce,
    Role, SharedString, Stateful, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px, relative,
};
use gpui_base::slider::{SliderEvent, SliderState, SliderValue};
use gpui_base::{SliderIndicator, SliderThumb, SliderTrack};
use gpui_component::{ActiveTheme as _, StyledExt as _, ThemeStyled as _};
use native_theme::theme::ResolvedTheme;

use super::{A11yDisabled, Part, PartBounds, color, faded, length, over, part_bounds};
use crate::ActiveNativeTheme;

/// The alpha of the thumb's outline, which `SliderTheme` does not state:
/// gpui-component's own, its fill colour at half alpha round the thumb
/// (slider.rs:224-227).
const THUMB_OUTLINE_ALPHA: f32 = 0.5;

/// The outline's width inside the thumb: gpui-component's `p(px(1.))`
/// (slider.rs:227).
const THUMB_OUTLINE_WIDTH: Pixels = gpui::px(1.);

/// The length of a vertical slider, which `SliderTheme` does not state:
/// gpui-component's own, `h(px(120.))` (slider.rs:279).
const VERTICAL_LENGTH: Pixels = gpui::px(120.);

/// The group name the thumb's hover style listens to.
const THUMB_GROUP: &str = "native-theme-slider-thumb";

/// What a slider paints, per state, from `SliderTheme`
/// (`docs/archive/todo_gpui-widgets-spec.md` §2.4).
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct SliderLook {
    /// `slider.track_height`: the rail's height.
    pub track_height: Pixels,
    /// `slider.thumb_diameter`.
    pub thumb: Pixels,
    /// The widget's height: the larger of the thumb and the rail.
    pub height: Pixels,
    /// The rail: `track_color`, or `disabled_track_color` disabled.
    pub track: Hsla,
    /// The filled part: `fill_color`, or `disabled_fill_color` disabled.
    pub fill: Hsla,
    /// The thumb: `thumb_color`, or `disabled_thumb_color` disabled.
    pub thumb_color: Hsla,
    /// The thumb under the pointer: `thumb_hover_color` over `thumb_color`,
    /// or the thumb at rest where no hover is stated, where the slider is
    /// enabled.
    pub hover_thumb: Option<Hsla>,
    /// The thumb's outline, which the model does not state: gpui-component's
    /// fill colour at half alpha (gpui-component's, slider.rs:224-227).
    pub outline: Hsla,
    /// The whole control's opacity: `slider.disabled_opacity` when disabled,
    /// on top of the disabled colours (docs/platform-facts.md §2.1.6), else 1.
    pub opacity: f32,
}

impl SliderLook {
    /// The look of a slider, `disabled` or not, or `None` when a length the
    /// theme gives is not finite. Each disabled colour is a soft option whose
    /// `None` copies the enabled colour (C16).
    #[must_use]
    pub fn of(resolved: &ResolvedTheme, disabled: bool) -> Option<Self> {
        let s = &resolved.slider;
        let (track, fill, thumb) = if disabled {
            (
                s.disabled_track_color.unwrap_or(s.track_color),
                s.disabled_fill_color.unwrap_or(s.fill_color),
                s.disabled_thumb_color.unwrap_or(s.thumb_color),
            )
        } else {
            (s.track_color, s.fill_color, s.thumb_color)
        };
        let hover_thumb = (!disabled).then(|| {
            s.thumb_hover_color.map_or(color(s.thumb_color), |h| {
                over(color(s.thumb_color), color(h))
            })
        });
        let track_height = length(s.track_height)?;
        let thumb_diameter = length(s.thumb_diameter)?;
        Some(Self {
            track_height,
            thumb: thumb_diameter,
            height: thumb_diameter.max(track_height),
            track: color(track),
            fill: color(fill),
            thumb_color: color(thumb),
            hover_thumb,
            outline: color(fill).opacity(THUMB_OUTLINE_ALPHA),
            opacity: super::disabled_opacity(disabled, s.disabled_opacity),
        })
    }

    /// This look faded as one control over `ground`, the colour it is drawn
    /// on: where [`opacity`](Self::opacity) is below 1, the rail composited
    /// over `ground`, the fill over the rail, the outline and the thumb over
    /// the fill, each faded by it over `ground`, the opacity then 1. As
    /// libadwaita fades a disabled scale, `filter: Opacity(..)` on the whole
    /// widget (docs/platform-facts.md §2.1.6), so the rail does not show
    /// through the fill nor the fill through the thumb, as fading each part
    /// on its own would show them.
    ///
    /// The outline is translucent (the fill at half alpha), and its 1 px
    /// ring lies partly over the fill and partly over the rail, so taking it
    /// over the fill is exact only where the ring covers the fill; the thumb,
    /// inside it, is taken over the fill too.
    #[must_use]
    pub fn faded_over(self, ground: Hsla) -> Self {
        if self.opacity >= 1. {
            return self;
        }
        let a = self.opacity;
        let rail = over(ground, self.track);
        let fill = over(rail, self.fill);
        Self {
            track: faded(ground, ground, self.track, a),
            fill: faded(ground, rail, self.fill, a),
            outline: faded(ground, fill, self.outline, a),
            thumb_color: faded(ground, fill, self.thumb_color, a),
            opacity: 1.,
            ..self
        }
    }
}

/// A slider over a gpui-base `SliderState` whose rail, fill and thumbs are
/// `SliderTheme`'s (`docs/archive/todo_gpui-widgets-spec.md` §2.4),
/// horizontal or vertical ([`Slider::axis`]).
///
/// gpui-base's parts own the pointer: a press on the rail sets the value, a
/// drag of the rail or the thumb moves it, and `SliderEvent::Change` /
/// `Release` are emitted as for gpui-component's slider. It reports itself as
/// a slider with its value, range and step, its
/// [`accessibility_label`](Slider::accessibility_label) and, while disabled,
/// its disabled state, as gpui-component's does, and answers AccessKit's
/// Increment and Decrement while enabled. Unlike gpui-component's, it takes
/// keyboard focus: Left and Down step it down, Right and Up up, Home and End
/// to the ends, each emitting `SliderEvent::Change`. The thumb's centre
/// travels the widget's length less one thumb radius at each end, so the
/// thumb stays inside the widget.
///
/// A range state (`SliderState::default_value(20.0..80.0)`) draws a thumb at
/// each end and the fill between them, as gpui-component's slider does
/// (slider.rs:157-164, :316-319): a press moves the nearer thumb, as
/// gpui-base picks it, and the keys move the end, which stops at the start.
/// The start moves by pointer only: a screen reader is told the range's end
/// alone, as gpui-base's slider reports it (slider.rs:516), and AccessKit's
/// Increment and Decrement step the end.
///
/// Its length is the caller's (`Styled`): a horizontal slider's width, a
/// vertical one's height, gpui-component's 120 px where not given. Without a
/// native theme it renders gpui-component's `Slider`.
pub struct Slider {
    state: Entity<SliderState>,
    axis: Axis,
    disabled: bool,
    style: StyleRefinement,
    observer: Option<PartBounds>,
    backdrop: Option<Hsla>,
    accessibility_label: Option<SharedString>,
}

impl Slider {
    /// A horizontal slider over `state`.
    #[must_use]
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            axis: Axis::Horizontal,
            disabled: false,
            style: StyleRefinement::default(),
            observer: None,
            backdrop: None,
            accessibility_label: None,
        }
    }

    /// The axis it lies along: `Axis::Horizontal`, its minimum at the left,
    /// or `Axis::Vertical`, its minimum at the bottom, as gpui-base maps the
    /// pointer (slider.rs:367-371).
    #[must_use]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Whether it is disabled: inert, unfocusable, in the disabled colours.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The colour under the slider, which a disabled one is faded over as
    /// one control ([`SliderLook::faded_over`]); the window's,
    /// `defaults.background_color`, where not given.
    #[must_use]
    pub fn backdrop(mut self, backdrop: Hsla) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    /// The name a screen reader announces: a slider draws no text of its
    /// own to be named by.
    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    /// Hands `observer` the bounds of the rail, its fill and the thumbs as
    /// a frame lays them out ([`PartBounds`]).
    #[must_use]
    pub fn on_part_bounds(mut self, observer: PartBounds) -> Self {
        self.observer = Some(observer);
        self
    }

    fn fallback(self) -> AnyElement {
        gpui_component::slider::Slider::new(&self.state)
            .when(self.axis == Axis::Vertical, |slider| slider.vertical())
            .disabled(self.disabled)
            .when_some(self.accessibility_label, |slider, label| {
                slider.accessibility_label(label)
            })
            .refine_style(&self.style)
            .into_any_element()
    }
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// The value a key moves `state` to, if the key is one the slider answers: a
/// range's end, with its start as it was, the end kept from passing the
/// start, as a pointer keeps it (gpui-base slider.rs:378-382).
fn stepped(state: &SliderState, key: &str) -> Option<SliderValue> {
    let (min, max) = (state.min_value(), state.max_value());
    let value = state.value();
    let step = state.step_value();
    let target = match key {
        "left" | "down" => value.end() - step,
        "right" | "up" => value.end() + step,
        "home" => min,
        "end" => max,
        _ => return None,
    };
    // `max` then `min`, never `clamp`: a state built with `min > max` must
    // not panic here.
    Some(match value {
        SliderValue::Range(start, _) => {
            SliderValue::from((start, target.max(min).max(start).min(max)))
        }
        SliderValue::Single(_) => SliderValue::from(target.max(min).min(max)),
    })
}

/// Sets `state` to the value `step` gives it and emits the change, as a
/// pointer does. Returns whether it answered.
fn step_by(
    state: &Entity<SliderState>,
    step: impl FnOnce(&SliderState) -> Option<SliderValue>,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    state.update(cx, |state, cx| {
        let Some(value) = step(state) else {
            return false;
        };
        state.set_value(value, window, cx);
        cx.emit(SliderEvent::Change(state.value()));
        true
    })
}

/// A thumb of `state` at `position` of the travel along `axis`, the range's
/// start thumb if `start`: the thumb's colour inside its outline, centred
/// across the rail.
fn thumb(
    state: &Entity<SliderState>,
    look: &SliderLook,
    axis: Axis,
    position: f32,
    start: bool,
    disabled: bool,
) -> SliderThumb {
    let across = px((f32::from(look.track_height) - f32::from(look.thumb)) / 2.);
    let back = px(-f32::from(look.thumb) / 2.);
    SliderThumb::new(state)
        .axis(axis)
        .start(start)
        .disabled(disabled)
        .group(THUMB_GROUP)
        .absolute()
        .map(|thumb| match axis {
            Axis::Vertical => thumb.left(across).bottom(relative(position)).mb(back),
            Axis::Horizontal => thumb.top(across).left(relative(position)).ml(back),
        })
        .size(look.thumb)
        .rounded_full()
        .bg(look.outline)
        .p(THUMB_OUTLINE_WIDTH)
        .child(
            div()
                .size_full()
                .rounded_full()
                .bg(look.thumb_color)
                .when_some(look.hover_thumb, |inner, hover| {
                    inner.group_hover(THUMB_GROUP, move |style| style.bg(hover))
                }),
        )
}

/// The theme-drawn slider's root, its node marked disabled while the slider
/// is.
pub(super) type SliderRoot = A11yDisabled<Stateful<Div>>;

impl Slider {
    /// The theme-drawn root, or the slider back where gpui-component's
    /// slider is drawn instead.
    pub(super) fn themed_root(
        mut self,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<SliderRoot, Box<Self>> {
        let Some(n) = cx.native() else {
            return Err(Box::new(self));
        };
        let Some(look) = SliderLook::of(n.resolved, self.disabled) else {
            return Err(Box::new(self));
        };
        let look = look.faded_over(
            self.backdrop
                .unwrap_or_else(|| color(n.resolved.defaults.background_color)),
        );
        let axis = self.axis;
        let vertical = axis == Axis::Vertical;
        let disabled = self.disabled;
        let label = self.accessibility_label.take();
        let observer = self.observer.as_ref();
        let state = self.state.clone();
        let entity_id = state.entity_id();
        let (value, range, min, max, step, start, end) = {
            let s = state.read(cx);
            let percentage = s.percentage();
            (
                s.value().end(),
                s.value().is_range(),
                s.min_value(),
                s.max_value(),
                s.step_value(),
                percentage.start.clamp(0., 1.),
                percentage.end.clamp(0., 1.),
            )
        };
        let id = ElementId::from(("native-theme-slider", entity_id));
        let focus_handle = window
            .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = !disabled && focus_handle.is_focused(window);
        let radius = px(f32::from(look.thumb) / 2.);
        let beyond = px(-f32::from(radius));
        // The rail is a pill unless the theme squares its corners
        // (slider.rs:182); `SliderTheme` states no rail radius.
        let pill = cx.theme().radius_full();

        // The end thumb, the one the keys move, draws the focus ring.
        let end_thumb = thumb(&state, &look, axis, end, false, disabled)
            .debug_selector(|| "native-slider-thumb".into())
            .when(focused, |thumb| thumb.focus_ring_style(window, cx))
            .children(observer.map(|o| part_bounds(Part::Thumb, o, px(0.))));
        let start_thumb = range.then(|| {
            thumb(&state, &look, axis, start, true, disabled)
                .debug_selector(|| "native-slider-start-thumb".into())
                .children(observer.map(|o| part_bounds(Part::StartThumb, o, px(0.))))
        });
        // The fill from the rail's end, or a range's start thumb, to the end
        // thumb.
        let fill_from: Length = if range {
            relative(start).into()
        } else {
            beyond.into()
        };
        let fill_to = relative(1. - end);
        // The travel the thumb's centre covers, and the pointer maps onto
        // (gpui-base records its bounds, slider.rs `SliderIndicator`): the
        // rail and the fill reach one thumb radius past it at each end.
        let travel = SliderIndicator::new(&state)
            .relative()
            .map(|travel| {
                if vertical {
                    travel.h_full().w(look.track_height)
                } else {
                    travel.w_full().h(look.track_height)
                }
            })
            .child(
                div()
                    .absolute()
                    .map(|rail| {
                        if vertical {
                            rail.left_0().right_0().top(beyond).bottom(beyond)
                        } else {
                            rail.top_0().bottom_0().left(beyond).right(beyond)
                        }
                    })
                    .rounded(pill)
                    .bg(look.track)
                    .debug_selector(|| "native-slider-rail".into())
                    .children(observer.map(|o| part_bounds(Part::Track, o, px(0.)))),
            )
            .child(
                div()
                    .absolute()
                    .map(|fill| {
                        if vertical {
                            fill.left_0().right_0().bottom(fill_from).top(fill_to)
                        } else {
                            fill.top_0().bottom_0().left(fill_from).right(fill_to)
                        }
                    })
                    .rounded(pill)
                    .bg(look.fill)
                    .debug_selector(|| "native-slider-fill".into())
                    .children(observer.map(|o| part_bounds(Part::Fill, o, px(0.)))),
            )
            .children(start_thumb)
            .child(end_thumb);
        let track = SliderTrack::new(&state)
            .axis(axis)
            .disabled(disabled)
            .flex()
            .size_full()
            .map(|track| {
                if vertical {
                    track.justify_center().py(radius)
                } else {
                    track.items_center().px(radius)
                }
            })
            .child(travel);

        let root = div()
            .id(id)
            .role(Role::Slider)
            .when_some(label, |root, label| root.aria_label(label))
            .aria_numeric_value(f64::from(value))
            .aria_min_numeric_value(f64::from(min))
            .aria_max_numeric_value(f64::from(max))
            .aria_numeric_value_step(f64::from(step))
            .aria_orientation(if vertical {
                Orientation::Vertical
            } else {
                Orientation::Horizontal
            })
            .flex()
            .map(|root| {
                if vertical {
                    root.justify_center().w(look.height).h(VERTICAL_LENGTH)
                } else {
                    root.items_center().h(look.height)
                }
            })
            .opacity(look.opacity)
            .refine_style(&self.style)
            .debug_selector(|| "native-slider".into())
            .when(!disabled, |root| {
                let keys = state.clone();
                let increment = state.clone();
                let decrement = state.clone();
                root.track_focus(&focus_handle.clone().tab_stop(true))
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let key = event.keystroke.key.clone();
                        if step_by(&keys, |s| stepped(s, &key), window, cx) {
                            cx.stop_propagation();
                        }
                    })
                    .on_a11y_action(AccessibleAction::Increment, move |_, window, cx| {
                        step_by(&increment, |s| stepped(s, "right"), window, cx);
                    })
                    .on_a11y_action(AccessibleAction::Decrement, move |_, window, cx| {
                        step_by(&decrement, |s| stepped(s, "left"), window, cx);
                    })
                    .on_mouse_up(
                        MouseButton::Left,
                        window.listener_for(&state, |state, _, _, cx| state.handle_release(cx)),
                    )
                    .on_mouse_up_out(
                        MouseButton::Left,
                        window.listener_for(&state, |state, _, _, cx| state.handle_release(cx)),
                    )
            })
            .child(track);
        Ok(A11yDisabled::new(root, disabled))
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.themed_root(window, cx) {
            Ok(root) => root.into_any_element(),
            Err(slider) => slider.fallback(),
        }
    }
}
