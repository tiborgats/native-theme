//! [`ComboBox`]: egui's own drop-down, as tall as its text and padding make it.

use native_theme_egui::egui;
use native_theme_egui::{Role, RoleVariant, ThemeAtlas};

use crate::scope;

/// egui's default combo-box arrow is a downward triangle `0.7` of its box wide and `0.45` of it
/// tall, centred in it (`egui/src/containers/combo_box.rs:472-486`, `paint_default_icon`).
const ARROW_WIDTH: f32 = 0.7;
const ARROW_HEIGHT: f32 = 0.45;

/// A drop-down: egui's own `ComboBox` in the combo-box scope, as tall as its text and padding
/// make it — `combo_box.*`.
///
/// **Tier W.** One `egui::ComboBox` (`egui/src/containers/combo_box.rs`) in the
/// `Role::ComboBox` scope, whose cell carries `combo_box.*` — its fill, hover fill, font,
/// border, padding, `min_height` as `interact_size.y`, `arrow_icon_size` as `icon_width` and
/// `arrow_area_width` as `icon_width + icon_spacing` — and its popup in that role's style
/// (`ComboBox::popup_style`, `:199`), with one per-instance change to the scope:
///
/// egui makes the arrow's box square, `Vec2::splat(icon_width)` (`:342`), and the content as
/// tall as the taller of the text and that box (`:361`), so an `arrow_icon_size` taller than
/// the text raises the drop-down above its text and padding, and above `min_height` where
/// those meet it: `kde-breeze`'s 20px arrow over an 18px line gives 34px where the theme
/// states 32. A platform's arrow sits in its own column the drop-down's full height and sets
/// no height of its own: Breeze's `SC_ComboBoxArrow` is a `MenuButton_IndicatorWidth` column,
/// WinUI's a 38px column (`docs/platform-facts.md` §2.24). So where `icon_width` is taller than
/// the selected text's line, this scope's `icon_width` is that line's height and its
/// `icon_spacing` grows by the difference, which keeps the arrow's column as wide as the theme
/// states; the arrow is then painted as egui paints its own — a triangle `0.7` of its box wide
/// and `0.45` tall, in the state's `fg_stroke` colour (`:472-486`) — in a box of the stated
/// `icon_width`, right-aligned in its column and centred on its height, so the arrow is the
/// size it was. Where the text is as tall as the arrow's box or taller, egui's own layout and
/// arrow stand unchanged.
///
/// Everything else is egui's: layout, interaction, the popup, keyboard and accessibility
/// (`WidgetType::ComboBox`). `Ui::add` takes an `egui::Widget`, and a drop-down's contents are
/// a closure, so this, like egui's own, is shown with [`ComboBox::show_ui`].
///
/// `.enabled(false)` opens the role's `Disabled` scope, whose cell carries
/// `combo_box.disabled_background` and `disabled_text_color`, and disables the drop-down.
///
/// With no atlas installed, it is `egui::ComboBox` with no change.
#[must_use = "You should call .show_ui()"]
pub struct ComboBox {
    id_salt: egui::IdSalt,
    selected_text: egui::WidgetText,
    width: Option<f32>,
    popup_style: Option<egui::style::StyleModifier>,
    enabled: bool,
}

impl ComboBox {
    /// A drop-down whose state egui keeps under `id_salt`, as `egui::ComboBox::from_id_salt`.
    pub fn from_id_salt(id_salt: impl egui::AsIdSalt) -> Self {
        Self {
            id_salt: egui::IdSalt::new(id_salt),
            selected_text: egui::WidgetText::default(),
            width: None,
            popup_style: None,
            enabled: true,
        }
    }

    /// The text shown in the drop-down, the selected row's.
    pub fn selected_text(mut self, text: impl Into<egui::WidgetText>) -> Self {
        self.selected_text = text.into();
        self
    }

    /// The drop-down's minimum width, as `egui::ComboBox::width`.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// The popup's style, in place of the combo-box role's, as `egui::ComboBox::popup_style`.
    pub fn popup_style(mut self, style: egui::style::StyleModifier) -> Self {
        self.popup_style = Some(style);
        self
    }

    /// `false` shows the platform's disabled drop-down, unfaded, and opens no popup.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Show the drop-down, `contents` filling its popup while it is open; as
    /// `egui::ComboBox::show_ui`, `inner` is `None` while it is closed.
    pub fn show_ui<R>(
        self,
        ui: &mut egui::Ui,
        contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<Option<R>> {
        let ComboBox {
            id_salt,
            selected_text,
            width,
            popup_style,
            enabled,
        } = self;
        let theme = ui.ctx().theme();
        let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) else {
            let mut combo = egui::ComboBox::from_id_salt(id_salt).selected_text(selected_text);
            if let Some(width) = width {
                combo = combo.width(width);
            }
            if let Some(style) = popup_style {
                combo = combo.popup_style(style);
            }
            return if enabled {
                combo.show_ui(ui, contents)
            } else {
                ui.add_enabled_ui(false, |ui| combo.show_ui(ui, contents))
                    .inner
            };
        };
        let variant = if enabled {
            RoleVariant::Normal
        } else {
            RoleVariant::Disabled
        };
        let popup_style =
            popup_style.unwrap_or_else(|| atlas.role_modifier(theme, Role::ComboBox, variant));
        scope::open(ui, Role::ComboBox, enabled, |ui| {
            let line = selected_text
                .clone()
                .into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    egui::TextStyle::Button,
                )
                .size()
                .y;
            let arrow = ui.spacing().icon_width;
            let mut combo = egui::ComboBox::from_id_salt(id_salt)
                .selected_text(selected_text)
                .popup_style(popup_style);
            if let Some(width) = width {
                combo = combo.width(width);
            }
            if line.is_finite() && line < arrow {
                let spacing = ui.spacing_mut();
                spacing.icon_spacing += arrow - line;
                spacing.icon_width = line;
                combo = combo.icon(move |ui, rect, visuals, _open| {
                    paint_arrow(ui, rect, visuals, arrow - line);
                });
            }
            combo.show_ui(ui, contents)
        })
    }
}

/// egui's default arrow (`egui/src/containers/combo_box.rs:472-486`) in the box of the stated
/// arrow size: `rect` is the box egui hands the icon, `grow` shorter than that size, so the
/// box it stood for is `grow` wider and taller, sharing its right edge and centre line.
fn paint_arrow(ui: &egui::Ui, rect: egui::Rect, visuals: &egui::style::WidgetVisuals, grow: f32) {
    let side = egui::vec2(rect.width() + grow, rect.height() + grow);
    let centre = egui::pos2(rect.right() - 0.5 * side.x, rect.center().y);
    let arrow = egui::Rect::from_center_size(
        centre,
        egui::vec2(side.x * ARROW_WIDTH, side.y * ARROW_HEIGHT),
    );
    ui.painter().add(egui::Shape::convex_polygon(
        vec![arrow.left_top(), arrow.right_top(), arrow.center_bottom()],
        visuals.fg_stroke.color,
        egui::Stroke::NONE,
    ));
}
