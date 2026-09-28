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
        if ThemeAtlas::from_ctx(ui.ctx()).is_some() {
            ui.native_scope(Role::SegmentedControl, RoleVariant::Normal, |ui| {
                row(ui, selected, segments)
            })
            .inner
        } else {
            row(ui, selected, segments)
        }
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
