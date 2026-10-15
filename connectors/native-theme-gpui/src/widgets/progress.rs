//! `ProgressBar` (spec §2.5): gpui-base's headless progress indicator,
//! painted from `ProgressBarTheme`.

use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, AnyElement, App, ElementId, Hsla, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, RenderOnce, SharedString, StyleRefinement, Styled, Window,
    div, ease_in_out, prelude::FluentBuilder as _, px, relative,
};
use gpui_base::{
    Progress as BaseProgress, ProgressIndicator, ProgressTrack, Transition, transition,
};
use gpui_component::{ActiveTheme as _, StyledExt as _};
use native_theme::theme::ResolvedTheme;

use super::{Part, PartBounds, color, length, part_bounds};
use crate::ActiveNativeTheme;

/// The period of an indeterminate bar's sweep, which the model does not
/// state: gpui-component's `Animation::new(Duration::from_secs(1)).repeat()`
/// (progress/progress.rs:152).
const LOADING_PERIOD: Duration = Duration::from_secs(1);

/// How far in from either end of the track a still indeterminate fill
/// stands under reduced motion: gpui-component's `relative(0.325)`
/// (progress/progress.rs:162-163).
const LOADING_STILL_INSET: f32 = 0.325;

/// Where an indeterminate bar's fill lies at `delta` of its sweep, as the
/// fractions of the track left before its start and after its end:
/// gpui-component's `progress-loading` motion (progress/progress.rs:153-157),
/// the end running out over the whole turn, `ease_in_out(1 − delta)` short of
/// the track's end, and the start following over its second half,
/// `ease_in_out((delta − 0.5) / 0.5)` from the track's start.
fn loading_span(delta: f32) -> (f32, f32) {
    let start = if delta > 0.5 {
        ease_in_out(((delta - 0.5) / 0.5).clamp(0., 1.))
    } else {
        0.
    };
    (start, ease_in_out(1. - delta))
}

/// What a progress bar paints, from `ProgressBarTheme` (spec §2.5).
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
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

/// A progress bar whose track, fill and frame are `ProgressBarTheme`'s
/// (spec §2.5), on gpui-base's headless `Progress`: it reports itself as a
/// progress indicator with its value out of 100. The fill follows a new
/// value on gpui-component's eased transition (progress/progress.rs:111-118).
/// A [`loading`](Self::loading) bar is indeterminate: its fill sweeps across
/// the track on gpui-component's motion.
///
/// As wide as its container, and at least `min_width`. Without a native
/// theme it renders gpui-component's `Progress`.
pub struct ProgressBar {
    id: ElementId,
    value: f32,
    label: Option<SharedString>,
    loading: bool,
    style: StyleRefinement,
    observer: Option<PartBounds>,
}

impl ProgressBar {
    /// A new bar at 0.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.,
            label: None,
            loading: false,
            style: StyleRefinement::default(),
            observer: None,
        }
    }

    /// Hands `observer` the bounds of the fill as each frame lays it out
    /// ([`PartBounds`]).
    #[must_use]
    pub fn on_part_bounds(mut self, observer: PartBounds) -> Self {
        self.observer = Some(observer);
        self
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

    /// Whether the bar is indeterminate. The value is then ignored: the fill,
    /// in the theme's colours and radius, sweeps across the track once a
    /// second on gpui-component's own motion (progress/progress.rs:149-160;
    /// the model states no motion), and stands still in the middle of the
    /// track under reduced motion (`App::reduce_motion`, :161-164). It reports
    /// itself as an indeterminate progress indicator, with no value.
    #[must_use]
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
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
            .loading(self.loading)
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
        let Some(n) = cx.native() else {
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
        let loading = self.loading;
        let indicator = ProgressIndicator::new()
            .h_full()
            .rounded(look.radius)
            .bg(look.fill)
            .child(
                div()
                    .absolute()
                    .size_full()
                    .debug_selector(|| "native-progress-fill".into()),
            )
            .children(
                self.observer
                    .as_ref()
                    .map(|o| part_bounds(Part::Fill, o, px(0.))),
            );
        // As gpui-component lays its indicator out (progress/progress.rs:148-168):
        // sweeping, still in the middle under reduced motion, or as wide as
        // the value.
        let indicator = if loading && !cx.reduce_motion() {
            indicator
                .absolute()
                .top_0()
                .with_animation(
                    "native-theme-progress-loading",
                    Animation::new(LOADING_PERIOD).repeat(),
                    |bar, delta| {
                        let (start, end) = loading_span(delta);
                        bar.left(relative(start)).right(relative(end))
                    },
                )
                .into_any_element()
        } else if loading {
            indicator
                .absolute()
                .top_0()
                .left(relative(LOADING_STILL_INSET))
                .right(relative(LOADING_STILL_INSET))
                .into_any_element()
        } else {
            indicator
                .relative()
                .w(relative((shown / 100.).clamp(0., 1.)))
                .into_any_element()
        };
        BaseProgress::new(self.id)
            .value(self.value)
            .indeterminate(loading)
            .when_some(self.label, |bar, label| bar.accessibility_label(label))
            .relative()
            .w_full()
            .min_w(look.min_width)
            .h(look.height)
            .refine_style(&self.style)
            .debug_selector(|| "native-progress".into())
            // The track, the fill `track_height` tall across it from its start,
            // and the frame drawn over both: `progress_bar.border` edges the
            // bar, not a box the fill sits inside.
            .child(
                ProgressTrack::new()
                    .absolute()
                    .size_full()
                    .rounded(look.radius)
                    .overflow_hidden()
                    .bg(look.track)
                    .child(indicator),
            )
            .child(
                div()
                    .absolute()
                    .size_full()
                    .rounded(look.radius)
                    .border(look.border_width)
                    .border_color(look.border),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// gpui-component's sweep (progress/progress.rs:153-157): the fill grows
    /// from the track's start over the first half turn, then its start runs
    /// after its end, and the turn ends with the fill gone at the far end.
    #[test]
    fn the_loading_fill_sweeps_from_the_start_to_the_end() {
        assert_eq!(loading_span(0.), (0., 1.));
        assert_eq!(loading_span(0.5), (0., 0.5));
        assert_eq!(loading_span(1.), (1., 0.));
        let (start, end) = loading_span(0.75);
        assert_eq!(start, ease_in_out(0.5));
        assert_eq!(end, ease_in_out(0.25));
    }
}
