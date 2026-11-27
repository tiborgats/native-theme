//! [`ProgressBar`]: egui's progress bar, outlined as the theme states.

use native_theme_egui::convert::{to_color32, to_corner_radius};
use native_theme_egui::egui;
use native_theme_egui::native_theme::next::{ResolvedTheme as NextResolvedTheme, WidgetState};
use native_theme_egui::{ResolvedTheme, Role, ThemeAtlas};

use crate::parts::Parts;
use crate::{next, scope};

/// A determinate progress bar: egui's own, in the `Role::ProgressBar` scope, rounded
/// `progress_bar.border.corner_radius` and outlined `progress_bar.border.line_width` wide in
/// `progress_bar.border.color`.
///
/// **Tier W.** One `egui::ProgressBar` in the progress bar role's scope, whose cell carries
/// `progress_bar.fill_color`, `.track_color` and `.track_height` (the connector's
/// `Role::ProgressBar`), with two per-instance calls: `ProgressBar::corner_radius` takes the
/// stated radius, and the stated outline is stroked inside the bar's rectangle over what egui
/// painted. `ProgressBar::ui` paints only filled rectangles and a galley, no outline
/// (`egui/src/widgets/progress_bar.rs:130-204`), and has no builder for one. KDE's Breeze
/// strokes its groove 1px in the window text at 20 %; GNOME and Windows draw none and state a
/// width of 0 (`docs/platform-facts.md` §2.10), so nothing is stroked there. Everything else
/// is egui's: layout, the fill, and accessibility (`Role::ProgressIndicator`).
///
/// As wide as the `Ui`'s available width, at least `progress_bar.min_width`, or
/// `desired_width`: egui's own rule, the theme's floor in place of egui's 96
/// (`egui/src/widgets/progress_bar.rs:113-114`).
///
/// With no atlas installed, or a radius or width that is not finite, it adds
/// `egui::ProgressBar` with no change but the width; with no atlas, or a `min_width` that is
/// not finite, egui's floor stands. Under an atlas of the 0.8.0 model each of
/// `progress_bar.track(NONE)`'s corners and stroke the theme leaves unstated, or states in a
/// form egui cannot draw, is egui's own: half the bar's height, no outline.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct ProgressBar {
    progress: f32,
    desired_width: Option<f32>,
    animate: bool,
}

impl ProgressBar {
    /// A bar showing `progress`, from `0.0` to `1.0`.
    pub fn new(progress: f32) -> Self {
        Self {
            progress,
            desired_width: None,
            animate: false,
        }
    }

    /// The bar's width, as `egui::ProgressBar::desired_width`.
    pub fn desired_width(mut self, width: f32) -> Self {
        self.desired_width = Some(width);
        self
    }

    /// Whether the bar shows egui's busy animation while `progress` is below `1.0`, as
    /// `egui::ProgressBar::animate`: an indeterminate bar, repainted every frame. Under the
    /// stated corner radius egui pulses the fill and draws no spinning arc, its own rule for a
    /// bar given a radius (`egui/src/widgets/progress_bar.rs:78-80`, `:163`). Under reduced
    /// motion (the atlas's `AccessibilityPreferences::reduce_motion`) the bar holds still, as
    /// the crate's [`Spinner`](crate::spinner::Spinner) does.
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
}

/// The colours a progress bar reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `progress_bar.border.color`: the outline.
    pub(crate) border_color: egui::Color32,
}

impl Look {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        Self {
            border_color: to_color32(t.progress_bar.border.color),
        }
    }
}

/// What a progress bar reads of a theme: its least width, its corner radius and its outline,
/// each `None` where the theme states none egui can draw and egui's own stays.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Bar {
    /// `progress_bar.min_width`.
    pub(crate) min_width: Option<f32>,
    /// `progress_bar.border.corner_radius`; `None`, egui's own, half the bar's height.
    pub(crate) radius: Option<f32>,
    /// The outline's width and colour where it has one wider than nothing; `None`, egui's own
    /// bar, which has no outline.
    pub(crate) outline: Option<(f32, egui::Color32)>,
}

impl Bar {
    /// A radius or width that is not finite leaves both egui's own.
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        let p = &t.progress_bar;
        let frame = || {
            let b = &p.border;
            let radius = b.corner_radius.is_finite().then_some(b.corner_radius)?;
            let width = scope::length(b.line_width)?;
            let outline = (width > 0.0).then_some((width, Look::of(t).border_color));
            Some((radius, outline))
        };
        let frame = frame();
        Self {
            min_width: scope::length(p.min_width),
            radius: frame.map(|(radius, _)| radius),
            outline: frame.and_then(|(_, outline)| outline),
        }
    }

    /// What a progress bar of a 0.8.0 theme reads: `progress_bar.track(NONE)`'s minimum width,
    /// its corners (a pill's on its minimum height) and its stroke, each where stated; a
    /// stroke stated as none is no outline.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Self {
        let track = r.progress_bar.track(WidgetState::NONE);
        let [width, height] = track.min_size;
        let outline = next::stroke_width_of(&track)
            .and_then(scope::length)
            .filter(|w| *w > 0.0)
            .zip(next::stroke_color_of(&track));
        Self {
            min_width: width.and_then(scope::length),
            radius: next::radius(&track, width, height).filter(|r| r.is_finite()),
            outline,
        }
    }
}

impl egui::Widget for ProgressBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let ProgressBar {
            progress,
            desired_width,
            animate,
        } = self;
        let atlas = ThemeAtlas::from_ctx(ui.ctx());
        let theme = ui.ctx().theme();
        let read = atlas.as_ref().and_then(|a| match a.resolved_for(theme) {
            Some(t) => Some(Bar::of(t)),
            None => a.next_resolved_for(theme).map(Bar::next_of),
        });
        let min_width = read.and_then(|b| b.min_width);
        let still = atlas
            .as_ref()
            .is_some_and(|a| a.accessibility().reduce_motion);
        let bar = egui::ProgressBar::new(progress).animate(animate && !still);
        let Some(Bar {
            radius, outline, ..
        }) = read.filter(|b| b.radius.is_some() || b.outline.is_some())
        else {
            let bar = sized(bar, ui, desired_width, min_width);
            return ui.add(bar);
        };
        let stated = radius.map(|r| to_corner_radius(egui::CornerRadius::default(), r));
        scope::open(ui, Role::ProgressBar, true, |ui| {
            let bar = sized(bar, ui, desired_width, min_width);
            let response = match stated {
                Some(radius) => ui.add(bar.corner_radius(radius)),
                None => ui.add(bar),
            };
            let outer = response.rect;
            // egui's own radius where the theme states none: half the bar's height
            // (`egui-0.37.0/src/widgets/progress_bar.rs`, `ProgressBar::ui`).
            let radius = stated.unwrap_or_else(|| (0.5 * outer.height()).into());
            // egui's fill: from the bar's start, `progress` of its width, at least as wide as
            // its rounded ends (`egui/src/widgets/progress_bar.rs`).
            let rounded = f32::from(radius.sw.max(radius.nw)).min(0.5 * outer.height());
            let filled = (outer.width() * progress).max(rounded + rounded);
            let mut parts = Parts::default();
            parts.push(
                "fill",
                egui::Rect::from_min_size(outer.min, egui::vec2(filled, outer.height())),
            );
            parts.store(ui.ctx(), response.id);
            if let Some((width, color)) = outline
                && ui.is_rect_visible(response.rect)
            {
                ui.painter().rect_stroke(
                    response.rect,
                    radius,
                    egui::Stroke::new(width, color),
                    egui::StrokeKind::Inside,
                );
            }
            response
        })
    }
}

/// `bar` as wide as `desired_width`, else as egui sizes it with `min_width` for its floor:
/// the available width, at least that (`egui/src/widgets/progress_bar.rs:113-114`).
fn sized(
    bar: egui::ProgressBar,
    ui: &egui::Ui,
    desired_width: Option<f32>,
    min_width: Option<f32>,
) -> egui::ProgressBar {
    match (desired_width, min_width) {
        (Some(width), _) => bar.desired_width(width),
        (None, Some(min)) => bar.desired_width(ui.available_size_before_wrap().x.max(min)),
        (None, None) => bar,
    }
}
