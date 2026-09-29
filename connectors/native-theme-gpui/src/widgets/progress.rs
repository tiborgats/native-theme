//! `ProgressBar` (spec §2.5): gpui-base's headless progress indicator,
//! painted from `ProgressBarTheme`.

use gpui::{
    AnyElement, App, ElementId, Hsla, InteractiveElement as _, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
    px, relative,
};
use gpui_base::{
    Progress as BaseProgress, ProgressIndicator, ProgressTrack, Transition, transition,
};
use gpui_component::{ActiveTheme as _, StyledExt as _};
use native_theme::theme::ResolvedTheme;

use super::{Part, PartBounds, color, length, native, part_bounds};

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
                    .child(
                        ProgressIndicator::new()
                            .relative()
                            .h_full()
                            .w(relative((shown / 100.).clamp(0., 1.)))
                            .rounded(look.radius)
                            .bg(look.fill)
                            .children(
                                self.observer
                                    .as_ref()
                                    .map(|o| part_bounds(Part::Fill, o, px(0.))),
                            ),
                    ),
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
