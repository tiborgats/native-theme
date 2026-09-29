//! [`ComboBox`]: egui's own drop-down, as tall as its text and padding make it.

use native_theme_egui::egui;
use native_theme_egui::{Role, RoleVariant, ThemeAtlas};

use crate::parts::Parts;
use crate::scope;

/// egui's default combo-box arrow is a downward triangle `0.7` of its box wide and `0.45` of it
/// tall, centred in it (`egui/src/containers/combo_box.rs:472-486`, `paint_default_icon`); the
/// chevron spans the same rectangle.
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
/// **The arrow is an open chevron.** KDE, GNOME and Windows draw the drop-down's arrow as a
/// chevron (`docs/platform-facts.md` §2.24, `arrow_icon_size`), where egui fills a triangle
/// (`paint_default_icon`, `:472-486`); the drop-down paints, through `ComboBox::icon`
/// (`:155`), a chevron over the rectangle egui's triangle spans — `0.7` of its box wide and
/// `0.45` tall — stroked in the interaction state's `fg_stroke`, colour and width.
///
/// egui makes the arrow's box square, `Vec2::splat(icon_width)` (`:342`), and the content as
/// tall as the taller of the text and that box (`:361`), so an `arrow_icon_size` taller than
/// the text would raise the drop-down above its text and padding, and above `min_height`
/// where those meet it. A platform's arrow sits in its own column the drop-down's full height
/// and sets no height of its own: Breeze's `SC_ComboBoxArrow` is a `MenuButton_IndicatorWidth`
/// column, WinUI's a 38px column (`docs/platform-facts.md` §2.24). So where `icon_width` is
/// taller than the selected text's line, this scope's `icon_width` is that line's height and
/// its `icon_spacing` grows by the difference, which keeps the arrow's column as wide as the
/// theme states, and the chevron is painted in a box of the stated `icon_width`, centred on
/// its height, so it is the size the theme states. Where the text is as tall as the arrow's box
/// or taller, egui's own layout stands.
///
/// **The sides are the theme's.** egui pads its button by one `button_padding.x` on both sides
/// (`combo_box_dyn`), where a platform pads the text and places the arrow's column apart:
/// Breeze pads the text 6 and puts its 20px column flush at the frame, the arrow centred in it
/// (`docs/platform-facts.md` §2.24). So where the theme states `border.padding.left`, the
/// scope's `button_padding.x` is that side plus the border's width, which starts the text
/// there; and where it states `border.padding.right` and `arrow_area_width`, the chevron is
/// centred in a column that wide ending that padding and the border inside the frame's right
/// edge.
///
/// Everything else is egui's: layout, interaction, the popup, keyboard and accessibility
/// (`WidgetType::ComboBox`). `Ui::add` takes an `egui::Widget`, and a drop-down's contents are
/// a closure, so this, like egui's own, is shown with [`ComboBox::show_ui`].
///
/// `.enabled(false)`, or a disabled calling `Ui`, opens the role's `Disabled` scope, whose
/// cell carries `combo_box.disabled_background` and `disabled_text_color`, and disables the
/// drop-down, which egui then fades by the cell's `disabled_alpha`,
/// `combo_box.disabled_opacity`.
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

    /// `false` shows the platform's disabled drop-down, its disabled colours faded by its
    /// `disabled_opacity`, and opens no popup.
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
        let variant = if enabled && ui.is_enabled() {
            RoleVariant::Normal
        } else {
            RoleVariant::Disabled
        };
        let popup_style =
            popup_style.unwrap_or_else(|| atlas.role_modifier(theme, Role::ComboBox, variant));
        let c = &atlas.resolved_for(theme).combo_box;
        // Where the border's width is not finite, egui's own padding and arrow place stand.
        let sides = scope::length(c.border.line_width).map(|border| Sides {
            left: c.border.padding.left.and_then(scope::length),
            right: c.border.padding.right.and_then(scope::length),
            border,
            column: c.arrow_area_width.and_then(scope::length),
        });
        scope::open(ui, Role::ComboBox, enabled, |ui| {
            // The text `combo_box.border.padding.left` inside the border: egui pads its button by
            // one `button_padding.x` on both sides (`combo_box_dyn`), which the scope holds as
            // the pair's single value.
            if let Some((left, sides)) = sides.and_then(|s| Some((s.left?, s))) {
                ui.spacing_mut().button_padding.x = left + sides.border;
            }
            let text_size = selected_text
                .clone()
                .into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    egui::TextStyle::Button,
                )
                .size();
            let line = text_size.y;
            let padding = ui.spacing().button_padding;
            let arrow = ui.spacing().icon_width;
            let mut combo = egui::ComboBox::from_id_salt(id_salt)
                .selected_text(selected_text)
                .popup_style(popup_style);
            if let Some(width) = width {
                combo = combo.width(width);
            }
            let grow = if line.is_finite() && line < arrow {
                let spacing = ui.spacing_mut();
                spacing.icon_spacing += arrow - line;
                spacing.icon_width = line;
                arrow - line
            } else {
                0.0
            };
            let arrow_box = std::rc::Rc::new(std::cell::Cell::new(None));
            let painted = std::rc::Rc::clone(&arrow_box);
            let margin = padding.x;
            combo = combo.icon(move |ui, rect, visuals, _open| {
                let centre = sides.and_then(|s| s.column_centre(rect, margin));
                painted.set(Some(paint_chevron(ui, rect, visuals, grow, centre)));
            });
            let out = combo.show_ui(ui, contents);
            // egui lays the selected text out flush left in the button's padding, centred on
            // the row (`egui/src/containers/combo_box.rs`, `combo_box_dyn`).
            let button = out.response.rect;
            let mut parts = Parts::default();
            parts.push(
                "text",
                egui::Rect::from_min_size(
                    egui::pos2(
                        button.left() + padding.x,
                        button.center().y - 0.5 * text_size.y,
                    ),
                    text_size,
                ),
            );
            if let Some(arrow) = arrow_box.get() {
                parts.push("arrow", arrow);
            }
            parts.store(ui.ctx(), out.response.id);
            out
        })
    }
}

/// The drop-down's stated sides, from `combo_box`: its `border.padding` left and right, its
/// border's width, and its arrow column's width (`arrow_area_width`).
#[derive(Clone, Copy)]
struct Sides {
    left: Option<f32>,
    right: Option<f32>,
    border: f32,
    column: Option<f32>,
}

impl Sides {
    /// Where the arrow's column is centred: the column `arrow_area_width` wide, at the button's
    /// right edge inside its border and `border.padding.right` — Breeze centres its arrow in
    /// its `MenuButton_IndicatorWidth` column (`docs/platform-facts.md` §2.24). `icon` is the
    /// box egui hands the icon, flush right in the content, which ends `margin`
    /// (`button_padding.x`) inside the button's edge (`combo_box_dyn`). `None` where the theme
    /// states no right padding or column: egui's own place stands.
    fn column_centre(self, icon: egui::Rect, margin: f32) -> Option<f32> {
        let (right, column) = (self.right?, self.column?);
        Some(icon.right() + margin - self.border - right - 0.5 * column)
    }
}

/// An open chevron in egui's arrow box (`egui/src/containers/combo_box.rs:472-486`, the box of
/// its `paint_default_icon`) of the stated arrow size, stroked in the state's `fg_stroke`:
/// the glyph KDE, GNOME and Windows draw (`docs/platform-facts.md` §2.24, `arrow_icon_size`),
/// where egui fills a triangle. `rect` is the box egui hands the icon, `grow` shorter than
/// that size, so the box it stood for is `grow` wider and taller, sharing its right edge and
/// centre line — or, where the theme places the arrow's column (`Sides::column_centre`), centred
/// in that column. Returns that box, the arrow's `Parts` rectangle.
fn paint_chevron(
    ui: &egui::Ui,
    rect: egui::Rect,
    visuals: &egui::style::WidgetVisuals,
    grow: f32,
    column_centre: Option<f32>,
) -> egui::Rect {
    let side = egui::vec2(rect.width() + grow, rect.height() + grow);
    let centre = egui::pos2(
        column_centre.unwrap_or(rect.right() - 0.5 * side.x),
        rect.center().y,
    );
    let arrow = egui::Rect::from_center_size(
        centre,
        egui::vec2(side.x * ARROW_WIDTH, side.y * ARROW_HEIGHT),
    );
    ui.painter().add(egui::Shape::line(
        vec![arrow.left_top(), arrow.center_bottom(), arrow.right_top()],
        visuals.fg_stroke,
    ));
    egui::Rect::from_center_size(centre, side)
}
