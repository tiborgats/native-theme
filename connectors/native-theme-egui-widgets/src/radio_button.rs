//! [`RadioButton`]: egui's radio button, its dot the theme's size.

use native_theme_egui::convert::clamp_length;
use native_theme_egui::egui;
use native_theme_egui::{Role, ThemeAtlas};

use crate::scope;

/// egui's radio dot is a circle of radius `small_icon_rect.width() / 3.0`
/// (`egui/src/widgets/radio_button.rs:101`), `small_icon_rect` being `Spacing::icon_width_inner`
/// square (`egui/src/style.rs:477-478`): the divisor, egui's own.
const EGUI_DOT_DIVISOR: f32 = 3.0;

/// A radio button: egui's own, in the `Role::Checkbox` scope, its dot
/// `checkbox.radio_dot_diameter` across.
///
/// **Tier W.** One `egui::RadioButton` in the checkbox role's scope — the `Selected` variant
/// while it is checked, whose cell fills the circle with `checkbox.checked_background` and
/// outlines it in `checkbox.border.color`; the `Disabled` variant and `Ui::disable` for
/// `.enabled(false)` — with one per-instance change: egui paints the dot at a third of
/// `Spacing::icon_width_inner` in radius (`egui/src/widgets/radio_button.rs:101`), a sink the
/// checkbox's check mark shares (`egui/src/widgets/checkbox.rs:132-133`), so where the theme
/// states `checkbox.radio_dot_diameter` the scope's `icon_width_inner` becomes
/// `EGUI_DOT_DIVISOR · 0.5 · radio_dot_diameter` for this radio alone, and the dot is that
/// many pixels across, in the cell's `fg_stroke`, `checkbox.indicator_color`. Where the theme
/// states none (macOS publishes none, `docs/platform-facts.md:1220`) or a size that is not
/// finite, the dot is egui's own. Everything else is egui's: layout, interaction and
/// accessibility (`WidgetType::RadioButton`).
///
/// Like egui's, it holds no value: a click is `Response::clicked`, and the caller selects it.
///
/// With no atlas installed it adds `egui::RadioButton`.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct RadioButton {
    checked: bool,
    text: egui::WidgetText,
    enabled: bool,
}

impl RadioButton {
    /// A radio button, `checked` or not, labelled `text`.
    pub fn new(checked: bool, text: impl Into<egui::WidgetText>) -> Self {
        Self {
            checked,
            text: text.into(),
            enabled: true,
        }
    }

    /// `false` shows the platform's disabled radio button, its disabled colours faded by
    /// `checkbox.disabled_opacity`, and takes no input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl egui::Widget for RadioButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let RadioButton {
            checked,
            text,
            enabled,
        } = self;
        let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) else {
            return ui.add_enabled(enabled, egui::RadioButton::new(checked, text));
        };
        let dot = atlas
            .resolved_for(ui.ctx().theme())
            .checkbox
            .radio_dot_diameter
            .and_then(scope::length);
        scope::open_selected(ui, Role::Checkbox, checked, enabled, |ui| {
            if let Some(dot) = dot {
                ui.spacing_mut().icon_width_inner = clamp_length(EGUI_DOT_DIVISOR * 0.5 * dot);
            }
            ui.add(egui::RadioButton::new(checked, text))
        })
    }
}
