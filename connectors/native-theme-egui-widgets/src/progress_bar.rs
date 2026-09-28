//! [`ProgressBar`]: egui's progress bar, outlined as the theme states.

use native_theme_egui::convert::{to_color32, to_corner_radius};
use native_theme_egui::egui;
use native_theme_egui::{Role, ThemeAtlas};

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
/// As wide as the `Ui`'s available width, or `desired_width`, as egui's own.
///
/// With no atlas installed, or a radius or width that is not finite, it adds
/// `egui::ProgressBar` with no change but the width.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct ProgressBar {
    progress: f32,
    desired_width: Option<f32>,
}

impl ProgressBar {
    /// A bar showing `progress`, from `0.0` to `1.0`.
    pub fn new(progress: f32) -> Self {
        Self {
            progress,
            desired_width: None,
        }
    }

    /// The bar's width, as `egui::ProgressBar::desired_width`.
    pub fn desired_width(mut self, width: f32) -> Self {
        self.desired_width = Some(width);
        self
    }
}

impl egui::Widget for ProgressBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let ProgressBar {
            progress,
            desired_width,
        } = self;
        let mut bar = egui::ProgressBar::new(progress);
        if let Some(width) = desired_width {
            bar = bar.desired_width(width);
        }
        let border = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let b = &atlas.resolved_for(ui.ctx().theme()).progress_bar.border;
            let radius = b.corner_radius.is_finite().then_some(b.corner_radius)?;
            let width = scope::length(b.line_width)?;
            Some((radius, width, b.color))
        });
        let Some((radius, width, color)) = border else {
            return ui.add(bar);
        };
        let radius = to_corner_radius(egui::CornerRadius::default(), radius);
        scope::open(ui, Role::ProgressBar, true, |ui| {
            let response = ui.add(bar.corner_radius(radius));
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
