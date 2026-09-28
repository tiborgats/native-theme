//! `ProgressBar` and `Spinner` (spec §2.5, §2.6): gpui-base's headless
//! progress indicator, painted from `ProgressBarTheme` and `SpinnerTheme`.

use std::f32::consts::TAU;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, AnyElement, App, Bounds, ElementId, Hsla,
    InteractiveElement as _, IntoElement, ParentElement, Pixels, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, canvas, div, ease_in_out, prelude::FluentBuilder as _, px,
    relative,
};
use gpui_base::{
    Progress as BaseProgress, ProgressIndicator, ProgressTrack, Transition, transition,
};
use gpui_component::plot::shape::{Arc, ArcData};
use gpui_component::{ActiveTheme as _, StyledExt as _};
use native_theme::theme::ResolvedTheme;

use super::{color, length, native};

/// What a progress bar paints, from `ProgressBarTheme` (spec §2.5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressBarLook {
    /// `progress_bar.track_height`: the bar's height, frame included.
    pub height: Pixels,
    /// `progress_bar.min_width`.
    pub min_width: Pixels,
    /// `progress_bar.border.corner_radius`.
    pub radius: Pixels,
    /// `progress_bar.border.line_width`.
    pub border_width: Pixels,
    /// `progress_bar.border.color`.
    pub border: Hsla,
    /// `progress_bar.track_color`.
    pub track: Hsla,
    /// `progress_bar.fill_color`.
    pub fill: Hsla,
}

impl ProgressBarLook {
    /// The look of a progress bar, or `None` when a length the theme gives
    /// is not finite.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme) -> Option<Self> {
        let p = &resolved.progress_bar;
        Some(Self {
            height: length(p.track_height)?,
            min_width: length(p.min_width)?,
            radius: length(p.border.corner_radius)?,
            border_width: length(p.border.line_width)?,
            border: color(p.border.color),
            track: color(p.track_color),
            fill: color(p.fill_color),
        })
    }
}

/// A determinate progress bar whose track, fill and frame are
/// `ProgressBarTheme`'s (spec §2.5), on gpui-base's headless `Progress`: it
/// reports itself as a progress indicator with its value out of 100. The
/// fill follows a new value on gpui-component's eased transition
/// (progress/progress.rs:111-118).
///
/// As wide as its container, and at least `min_width`. Without a native
/// theme it renders gpui-component's `Progress`.
pub struct ProgressBar {
    id: ElementId,
    value: f32,
    label: Option<SharedString>,
    style: StyleRefinement,
}

impl ProgressBar {
    /// A new bar at 0.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.,
            label: None,
            style: StyleRefinement::default(),
        }
    }

    /// The value, in percent, clamped to 0–100.
    #[must_use]
    pub fn value(mut self, value: f32) -> Self {
        self.value = if value.is_finite() {
            value.clamp(0., 100.)
        } else {
            0.
        };
        self
    }

    /// The name a screen reader announces.
    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn fallback(self) -> AnyElement {
        gpui_component::progress::Progress::new(self.id)
            .value(self.value)
            .when_some(self.label, |bar, label| bar.accessibility_label(label))
            .refine_style(&self.style)
            .into_any_element()
    }
}

impl Styled for ProgressBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ProgressBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(n) = native(cx) else {
            return self.fallback();
        };
        let Some(look) = ProgressBarLook::of(n.resolved) else {
            return self.fallback();
        };
        let motion = cx.theme().motion_tokens();
        let shown = transition(
            (self.id.clone(), "indicator"),
            self.value,
            Transition::new(motion.duration_normal).easing(motion.easing_move.clone()),
            window,
            cx,
        );
        BaseProgress::new(self.id)
            .value(self.value)
            .when_some(self.label, |bar, label| bar.accessibility_label(label))
            .relative()
            .w_full()
            .min_w(look.min_width)
            .h(look.height)
            .refine_style(&self.style)
            .debug_selector(|| "native-progress".into())
            .child(
                ProgressTrack::new()
                    .absolute()
                    .size_full()
                    .rounded(look.radius)
                    .border(look.border_width)
                    .border_color(look.border)
                    .bg(look.track)
                    .child(
                        ProgressIndicator::new()
                            .h_full()
                            .w(relative((shown / 100.).clamp(0., 1.)))
                            .rounded(look.radius)
                            .bg(look.fill),
                    ),
            )
            .into_any_element()
    }
}

/// The indeterminate arc's period: gpui-component's (progress/progress_circle.rs:201).
const SPINNER_PERIOD: Duration = Duration::from_secs(1);

/// The point of the motion drawn still under reduced motion: half-way, where
/// the head has swept half the circle and the tail has not yet moved.
const REDUCED_MOTION_PHASE: f32 = 0.5;

/// What a spinner paints, from `SpinnerTheme` (spec §2.6).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerLook {
    /// The ring's outer size: `spinner.diameter`, and no less than
    /// `min_diameter`.
    pub diameter: Pixels,
    /// `spinner.stroke_width`.
    pub stroke: Pixels,
    /// `spinner.fill_color`.
    pub color: Hsla,
}

impl SpinnerLook {
    /// The look of a spinner, or `None` when a length the theme gives is not
    /// finite.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme) -> Option<Self> {
        let s = &resolved.spinner;
        let diameter = length(s.diameter)?;
        let min = length(s.min_diameter)?;
        Some(Self {
            diameter: diameter.max(min),
            stroke: length(s.stroke_width)?,
            color: color(s.fill_color),
        })
    }

    /// The radius of the stroke's centre line: the model gives the diameter
    /// as the ring's outer size, and the stroke is centred on its path.
    #[must_use]
    pub fn path_radius(&self) -> Pixels {
        px((f32::from(self.diameter) - f32::from(self.stroke)) / 2.)
    }
}

/// The arc of gpui-component's indeterminate circle at `delta` (0–1) of its
/// period, as the fraction of the circle each end has reached
/// (progress/progress_circle.rs:203-204): the head eased over the whole
/// period, the tail over its second half.
fn arc_at(delta: f32) -> (f32, f32) {
    let head = ease_in_out(delta);
    let tail = ease_in_out(((delta - 0.5) / 0.5).clamp(0., 1.));
    (tail, head)
}

/// The ring `look` from `start` to `end` (fractions of the circle).
fn ring(look: SpinnerLook, start: f32, end: f32) -> impl IntoElement {
    canvas(
        move |bounds: Bounds<Pixels>, _, _| bounds,
        move |_, bounds, window, _| {
            let outer = f32::from(look.diameter) / 2.;
            let inner = outer - f32::from(look.stroke);
            Arc::new()
                .inner_radius(inner.max(0.))
                .outer_radius(outer)
                .paint(
                    &ArcData {
                        data: &(),
                        index: 0,
                        value: end - start,
                        start_angle: start * TAU,
                        end_angle: end * TAU,
                        pad_angle: 0.,
                    },
                    look.color,
                    None,
                    None,
                    &bounds,
                    window,
                );
        },
    )
    .absolute()
    .size_full()
}

/// An indeterminate spinner: an arc whose size, stroke and colour are
/// `SpinnerTheme`'s (spec §2.6), in gpui-component's own indeterminate-arc
/// motion, on gpui-base's headless `Progress` (a progress indicator with no
/// value). Under reduced motion the arc stands still at the motion's
/// half-way frame. Without a native theme it renders gpui-component's
/// `Spinner`.
pub struct Spinner {
    id: ElementId,
    label: Option<SharedString>,
}

impl Spinner {
    /// A new spinner.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
        }
    }

    /// The name a screen reader announces.
    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for Spinner {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let look = native(cx).and_then(|n| SpinnerLook::of(n.resolved));
        let Some(look) = look else {
            return div()
                .child(gpui_component::spinner::Spinner::new())
                .into_any_element();
        };
        let root = BaseProgress::new(self.id)
            .indeterminate(true)
            .when_some(self.label, |spinner, label| {
                spinner.accessibility_label(label)
            })
            .relative()
            .flex_none()
            .size(look.diameter)
            .debug_selector(|| "native-spinner".into());
        if cx.reduce_motion() {
            let (start, end) = arc_at(REDUCED_MOTION_PHASE);
            return root.child(ring(look, start, end)).into_any_element();
        }
        root.with_animation(
            "native-theme-spinner",
            Animation::new(SPINNER_PERIOD).repeat(),
            move |spinner, delta| {
                let (start, end) = arc_at(delta);
                spinner.child(ring(look, start, end))
            },
        )
        .into_any_element()
    }
}
