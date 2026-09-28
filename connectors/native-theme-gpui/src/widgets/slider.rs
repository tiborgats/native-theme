//! `Slider` (spec §2.4): gpui-base's headless slider state, rail, travel and
//! thumb, painted from `SliderTheme`, and made focusable with keyboard steps.

use gpui::{
    AccessibleAction, AnyElement, App, ElementId, Entity, Hsla, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, Orientation, ParentElement, Pixels, RenderOnce, Role,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};
use gpui_base::slider::{SliderEvent, SliderState};
use gpui_base::{SliderIndicator, SliderThumb, SliderTrack};
use gpui_component::{ActiveTheme as _, StyledExt as _, ThemeStyled as _};
use native_theme::theme::ResolvedTheme;

use super::{color, length, native, over};

/// The alpha of the thumb's outline, which `SliderTheme` does not state:
/// gpui-component's own, its fill colour at half alpha round the thumb
/// (slider.rs:216-219).
const THUMB_OUTLINE_ALPHA: f32 = 0.5;

/// The outline's width inside the thumb: gpui-component's `p(px(1.))`
/// (slider.rs:219).
const THUMB_OUTLINE_WIDTH: Pixels = gpui::px(1.);

/// The group name the thumb's hover style listens to.
const THUMB_GROUP: &str = "native-theme-slider-thumb";

/// What a slider paints, per state, from `SliderTheme` (spec §2.4).
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// where the slider is enabled.
    pub hover_thumb: Option<Hsla>,
    /// The thumb's outline, which the model does not state: gpui-component's
    /// fill colour at half alpha (gpui-component's, slider.rs:216-219).
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
            over(
                color(s.thumb_color),
                color(s.thumb_hover_color.unwrap_or(s.thumb_color)),
            )
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
}

/// A horizontal slider over a gpui-base `SliderState` whose rail, fill and
/// thumb are `SliderTheme`'s (spec §2.4).
///
/// gpui-base's parts own the pointer: a press on the rail sets the value, a
/// drag of the rail or the thumb moves it, and `SliderEvent::Change` /
/// `Release` are emitted as for gpui-component's slider. It reports itself as
/// a slider with its value, range and step, and answers AccessKit's
/// Increment and Decrement. Unlike gpui-component's, it takes keyboard focus:
/// Left and Down step it down, Right and Up up, Home and End to the ends,
/// each emitting `SliderEvent::Change`. The thumb's centre travels the width
/// less one thumb radius at each end, so the thumb stays inside the widget.
///
/// Its width is the caller's (`Styled`). Without a native theme it renders
/// gpui-component's `Slider`.
pub struct Slider {
    state: Entity<SliderState>,
    disabled: bool,
    style: StyleRefinement,
}

impl Slider {
    /// A slider over `state`.
    #[must_use]
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            disabled: false,
            style: StyleRefinement::default(),
        }
    }

    /// Whether it is disabled: inert, unfocusable, in the disabled colours.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn fallback(self) -> AnyElement {
        gpui_component::slider::Slider::new(&self.state)
            .disabled(self.disabled)
            .refine_style(&self.style)
            .into_any_element()
    }
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// The value a key moves `state` to, if the key is one the slider answers.
fn stepped(state: &SliderState, key: &str) -> Option<f32> {
    let (min, max) = (state.min_value(), state.max_value());
    let value = state.value().end();
    let step = state.step_value();
    let target = match key {
        "left" | "down" => value - step,
        "right" | "up" => value + step,
        "home" => min,
        "end" => max,
        _ => return None,
    };
    // `max` then `min`, never `clamp`: a state built with `min > max` must
    // not panic here.
    Some(target.max(min).min(max))
}

/// Sets `state` to the value `step` gives it and emits the change, as a
/// pointer does. Returns whether it answered.
fn step_by(
    state: &Entity<SliderState>,
    step: impl FnOnce(&SliderState) -> Option<f32>,
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

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(n) = native(cx) else {
            return self.fallback();
        };
        let Some(look) = SliderLook::of(n.resolved, self.disabled) else {
            return self.fallback();
        };
        let disabled = self.disabled;
        let state = self.state.clone();
        let entity_id = state.entity_id();
        let (value, min, max, step, percentage) = {
            let s = state.read(cx);
            (
                s.value().end(),
                s.min_value(),
                s.max_value(),
                s.step_value(),
                s.percentage().end.clamp(0., 1.),
            )
        };
        let id = ElementId::from(("native-theme-slider", entity_id));
        let focus_handle = window
            .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = !disabled && focus_handle.is_focused(window);
        let radius = px(f32::from(look.thumb) / 2.);
        // The rail is a pill unless the theme squares its corners
        // (slider.rs:174); `SliderTheme` states no rail radius.
        let pill = cx.theme().radius_full();

        let thumb = SliderThumb::new(&state)
            .disabled(disabled)
            .group(THUMB_GROUP)
            .absolute()
            .top(px(
                (f32::from(look.track_height) - f32::from(look.thumb)) / 2.
            ))
            .left(relative(percentage))
            .ml(px(-f32::from(radius)))
            .size(look.thumb)
            .rounded_full()
            .bg(look.outline)
            .p(THUMB_OUTLINE_WIDTH)
            .debug_selector(|| "native-slider-thumb".into())
            .when(focused, |thumb| thumb.focus_ring_style(window, cx))
            .child(
                div()
                    .size_full()
                    .rounded_full()
                    .bg(look.thumb_color)
                    .when_some(look.hover_thumb, |inner, hover| {
                        inner.group_hover(THUMB_GROUP, move |style| style.bg(hover))
                    }),
            );
        // The travel the thumb's centre covers, and the pointer maps onto
        // (gpui-base records its bounds, slider.rs `SliderIndicator`): the
        // rail and the fill reach one thumb radius past it at each end.
        let travel = SliderIndicator::new(&state)
            .relative()
            .w_full()
            .h(look.track_height)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(-f32::from(radius)))
                    .right(px(-f32::from(radius)))
                    .rounded(pill)
                    .bg(look.track)
                    .debug_selector(|| "native-slider-rail".into()),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(-f32::from(radius)))
                    .right(relative(1. - percentage))
                    .rounded(pill)
                    .bg(look.fill)
                    .debug_selector(|| "native-slider-fill".into()),
            )
            .child(thumb);
        let track = SliderTrack::new(&state)
            .disabled(disabled)
            .flex()
            .items_center()
            .size_full()
            .px(radius)
            .child(travel);

        div()
            .id(id)
            .role(Role::Slider)
            .aria_numeric_value(f64::from(value))
            .aria_min_numeric_value(f64::from(min))
            .aria_max_numeric_value(f64::from(max))
            .aria_numeric_value_step(f64::from(step))
            .aria_orientation(Orientation::Horizontal)
            .flex()
            .items_center()
            .h(look.height)
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
            .child(track)
            .into_any_element()
    }
}
