//! [`ProgressBar`]: egui's progress bar, outlined as the theme states.

use native_theme_egui::convert::{to_color32, to_corner_radius};
use native_theme_egui::egui;
use native_theme_egui::{Role, ThemeAtlas};

use crate::parts::Parts;
use crate::scope;

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
/// is egui's: layout, the fill, and accessibility (`WidgetType::ProgressIndicator`).
///
/// As wide as the `Ui`'s available width, at least `progress_bar.min_width`, or
/// `desired_width`: egui's own rule, the theme's floor in place of egui's 96
/// (`egui/src/widgets/progress_bar.rs:113-114`).
///
/// With no atlas installed, or a radius or width that is not finite, it adds
/// `egui::ProgressBar` with no change but the width; with no atlas, or a `min_width` that is
/// not finite, egui's floor stands.
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

impl egui::Widget for ProgressBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let ProgressBar {
            progress,
            desired_width,
            animate,
        } = self;
        let atlas = ThemeAtlas::from_ctx(ui.ctx());
        let theme = atlas
            .as_ref()
            .map(|a| &a.resolved_for(ui.ctx().theme()).progress_bar);
        let min_width = theme.and_then(|p| scope::length(p.min_width));
        let still = atlas
            .as_ref()
            .is_some_and(|a| a.accessibility().reduce_motion);
        let bar = egui::ProgressBar::new(progress).animate(animate && !still);
        let border = theme.and_then(|p| {
            let b = &p.border;
            let radius = b.corner_radius.is_finite().then_some(b.corner_radius)?;
            let width = scope::length(b.line_width)?;
            Some((radius, width, b.color))
        });
        let Some((radius, width, color)) = border else {
            let bar = sized(bar, ui, desired_width, min_width);
            return ui.add(bar);
        };
        let radius = to_corner_radius(egui::CornerRadius::default(), radius);
        scope::open(ui, Role::ProgressBar, true, |ui| {
            let bar = sized(bar, ui, desired_width, min_width);
            let response = ui.add(bar.corner_radius(radius));
            // egui's fill: from the bar's start, `progress` of its width, at least as wide as
            // its rounded ends (`egui/src/widgets/progress_bar.rs`).
            let outer = response.rect;
            let rounded = f32::from(radius.sw.max(radius.nw)).min(0.5 * outer.height());
            let filled = (outer.width() * progress).max(rounded + rounded);
            let mut parts = Parts::default();
            parts.push(
                "fill",
                egui::Rect::from_min_size(outer.min, egui::vec2(filled, outer.height())),
            );
            parts.store(ui.ctx(), response.id);
            if width > 0.0 && ui.is_rect_visible(response.rect) {
                ui.painter().rect_stroke(
                    response.rect,
                    radius,
                    egui::Stroke::new(width, to_color32(color)),
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
