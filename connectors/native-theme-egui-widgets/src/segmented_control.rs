//! [`SegmentedControl`]: one exclusive choice among a row of segments.

use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::{NativeThemeUiExt as _, Role, RoleVariant, ThemeAtlas};

/// A row of segments, one of them selected: `segmented_control.*`.
///
/// **Tier C.** One horizontal row of `Button::new(label).selected(i == *selected)`
/// (`egui/src/widgets/button.rs:270`) in one `Role::SegmentedControl` scope for the whole row.
/// The scope's `Normal` cell carries `segmented_control.background_color`, `hover_background`,
/// `font` and `border.*` for every segment, `active_background` and `active_text_color` in
/// `selection.*`, which egui paints only on the selected button
/// (`egui/src/widget_style.rs:150-155`), `segment_height` and the segments' padding, and
/// `separator_width` as the gap between the segments (`spacing.item_spacing.x`). Not
/// `Button::selectable`, whose unselected button paints no frame at rest
/// (`egui/src/widgets/button.rs:78-83`) and would lose the background colour. A click on a
/// segment selects it.
///
/// `segment_height` is a button's minimum height there (`egui/src/widgets/button.rs:307-309`),
/// which the text and its padding can exceed. Where `segmented_control.border.padding` states
/// neither its top nor its bottom, the scope's vertical padding is one the theme does not
/// state for this widget, so it is narrowed, never widened, until a one-line segment is
/// `segment_height` tall: `0.5 · (segment_height − the tallest label's height)`, since a
/// button's frame adds its padding exactly on each side — `button_padding + expansion − stroke`
/// inside, the stroke, `−expansion` outside (`egui/src/widget_style.rs:158-165`) — and its
/// text is in the style's `override_font_id`, else its Body font (`:137`). A stated padding is
/// kept as stated.
///
/// **What is not drawn**, because no source states it: the divider's line (no leaf states its
/// colour, so what lies behind the row shows through the gap), and a joined outline (nothing
/// states whether a segment's corners that face a divider are rounded, so every segment keeps
/// `segmented_control.border.corner_radius` on all four corners).
///
/// It returns the row's `Response`, marked changed when a click moved the selection. The row
/// is a `RadioGroup` for AccessKit and each segment a `RadioButton`, toggled when selected.
///
/// It has no `enabled`: `segmented_control` states no disabled colour, so
/// `ui.add_enabled(false, ..)` — egui's fade at the calling `Ui`'s `disabled_alpha` — is its
/// disabled appearance.
///
/// With no atlas installed, it adds the same buttons in the calling `Ui`'s style.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct SegmentedControl<'a> {
    selected: &'a mut usize,
    segments: Vec<egui::WidgetText>,
}

impl<'a> SegmentedControl<'a> {
    /// A row of `segments`, the one at index `selected` selected; a click sets `selected` to
    /// the index of the segment clicked.
    pub fn new(
        selected: &'a mut usize,
        segments: impl IntoIterator<Item = impl Into<egui::WidgetText>>,
    ) -> Self {
        Self {
            selected,
            segments: segments.into_iter().map(Into::into).collect(),
        }
    }
}

impl egui::Widget for SegmentedControl<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let SegmentedControl { selected, segments } = self;
        if let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) {
            let padding = &atlas
                .resolved_for(ui.ctx().theme())
                .segmented_control
                .border
                .padding;
            let unstated = padding.top.is_none() && padding.bottom.is_none();
            ui.native_scope(Role::SegmentedControl, RoleVariant::Normal, |ui| {
                if unstated {
                    fit_height(ui, &segments);
                }
                row(ui, selected, segments)
            })
            .inner
        } else {
            row(ui, selected, segments)
        }
    }
}

/// Narrow the scope's vertical button padding until the tallest one-line segment is the
/// scope's `interact_size.y` (`segment_height`) tall; never widen it. Each label is laid out
/// as the button lays it out (`egui/src/atomics/atom_kind.rs:134-135`): unwrapped, in the
/// style's `override_font_id`, else its Body font.
fn fit_height(ui: &mut egui::Ui, segments: &[egui::WidgetText]) {
    let font = ui
        .style()
        .override_font_id
        .clone()
        .unwrap_or_else(|| egui::TextStyle::Body.resolve(ui.style()));
    let text = segments
        .iter()
        .map(|label| {
            label
                .clone()
                .into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    font.clone(),
                )
                .size()
                .y
        })
        .fold(0.0, f32::max);
    let fit = (0.5 * (ui.spacing().interact_size.y - text)).max(0.0);
    let padding = &mut ui.spacing_mut().button_padding.y;
    if fit < *padding {
        *padding = fit;
    }
}

fn row(ui: &mut egui::Ui, selected: &mut usize, segments: Vec<egui::WidgetText>) -> egui::Response {
    let mut changed = false;
    let mut row = ui.horizontal(|ui| {
        ui.ctx().accesskit_node_builder(ui.unique_id(), |node| {
            node.set_role(accesskit::Role::RadioGroup);
        });
        for (i, label) in segments.into_iter().enumerate() {
            let response = ui.add(egui::Button::new(label).selected(i == *selected));
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(accesskit::Role::RadioButton);
            });
            if response.clicked() && i != *selected {
                *selected = i;
                changed = true;
            }
        }
    });
    if changed {
        row.response.mark_changed();
    }
    row.response
}
