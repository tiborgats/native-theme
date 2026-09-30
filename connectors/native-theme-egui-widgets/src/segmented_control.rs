//! [`SegmentedControl`]: one exclusive choice among a row of segments.

use native_theme_egui::convert::u8_from_f32_saturating;
use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::{NativeThemeUiExt as _, Role, RoleVariant, ThemeAtlas};

use crate::parts::Parts;

/// One control of joined segments, one of them selected: `segmented_control.*`.
///
/// **Tier C.** One horizontal row of `Button::new(label).selected(i == *selected)`
/// (`egui/src/widgets/button.rs:270`) in one `Role::SegmentedControl` scope for the whole row.
/// The scope's `Normal` cell carries `segmented_control.background_color`, `hover_background`,
/// `font` and `border.*`, `active_background` and `active_text_color` in `selection.*`, which
/// egui paints only on the selected button (`egui/src/widget_style.rs:150-155`),
/// `segment_height` and the segments' padding, and `separator_width` as the gap between the
/// segments (`spacing.item_spacing.x`). Not `Button::selectable`, whose unselected button
/// paints no frame at rest (`egui/src/widgets/button.rs:78-83`) and would lose the background
/// colour. A click on a segment selects it.
///
/// **Joined.** A segmented control is one control parted by dividers on every platform the
/// presets model (`docs/platform-facts.md` §2.25: `separator_width` is "the width of the
/// divider line between segments"). The row sits in one `egui::Frame` — the non-interactive
/// decoration of a composed widget — filled with the scope's border colour, rounded to the
/// scope's `border.corner_radius` and inset by the border's width all round, so what shows of
/// that fill is the control's outline and, through the `separator_width` gaps, its dividers:
/// the model states the dividers' width and no colour of their own, so they are the one line
/// colour the control states. The segments paint no stroke of their own (`Button::stroke`,
/// `egui/src/widgets/button.rs:151`), and each rounds only the corners it shares with the
/// outline, to the outline's inner radius, `border.corner_radius` less the border's width.
///
/// `segment_height` is each segment's height (`docs/platform-facts.md`, "height of each
/// segment button"), a button's minimum height there (`egui/src/widgets/button.rs:307-309`),
/// which the text and its padding can exceed; the outline adds its width above and below.
/// A segment's padding is `segmented_control.border.padding` inside the outline: the scope's
/// `button_padding` holds the stated side plus the border's width, which the button takes off
/// again for the stroke it no longer paints (`egui/src/widget_style.rs:163-165`). Where
/// `segmented_control.border.padding` states neither its top nor its bottom, the scope's
/// vertical padding is one the theme does not state for this widget, so it is narrowed, never
/// widened, until a one-line segment is `segment_height` tall: `0.5 · (segment_height − the
/// tallest label's height)`, its text in the style's `override_font_id`, else its Body font
/// (`:137`). A stated padding is kept as stated.
///
/// It returns the control's `Response`, the outline's rect, marked changed when a click moved
/// the selection. The row inside the outline is a `RadioGroup` for AccessKit and each segment
/// a `RadioButton`, toggled when selected.
///
/// It has no `enabled`: `segmented_control` states no disabled colour, so
/// `ui.add_enabled(false, ..)` — egui's fade at the calling `Ui`'s `disabled_alpha` — is its
/// disabled appearance.
///
/// With no atlas installed, it adds the same buttons in the calling `Ui`'s style, in a row
/// with no outline.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct SegmentedControl<'a> {
    selected: &'a mut usize,
    segments: Vec<egui::WidgetText>,
    wrap: bool,
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
            wrap: false,
        }
    }

    /// Wraps the segments onto a further line, inside the one outline, where the `Ui` is too
    /// narrow for them (`Ui::horizontal_wrapped`), instead of running past its edge.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
}

impl egui::Widget for SegmentedControl<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let SegmentedControl {
            selected,
            segments,
            wrap,
        } = self;
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
                let outline = Outline::of(ui);
                let contents = |ui: &mut egui::Ui| {
                    outline
                        .frame()
                        .show(ui, |ui| row(ui, selected, segments, Some(&outline)))
                        .inner
                };
                if wrap {
                    ui.horizontal_wrapped(contents).map_changed()
                } else {
                    ui.horizontal(contents).map_changed()
                }
            })
            .inner
        } else {
            ui.horizontal(|ui| row(ui, selected, segments, None))
                .map_changed()
        }
    }
}

/// The response of a row whose `inner` says whether a click moved the selection, marked
/// changed when it did, with the row's segments as its `Parts`.
trait MapChanged {
    fn map_changed(self) -> egui::Response;
}

impl MapChanged for egui::InnerResponse<(bool, Vec<egui::Rect>)> {
    fn map_changed(mut self) -> egui::Response {
        let (changed, segments) = self.inner;
        if changed {
            self.response.mark_changed();
        }
        let mut parts = Parts::default();
        let mut later = segments.iter();
        later.next();
        for (i, segment) in segments.iter().enumerate() {
            parts.push(format!("segment_{i}"), *segment);
            if let Some(next) = later.next() {
                // The outline's colour shows between two segments: the gap the row leaves.
                parts.push(
                    format!("divider_{i}"),
                    egui::Rect::from_x_y_ranges(segment.right()..=next.left(), segment.y_range()),
                );
            }
        }
        parts.store(&self.response.ctx, self.response.id);
        self.response
    }
}

/// The joined control's outline, from the segmented control scope's own border: its stroke's
/// colour and width, and its corner radius.
struct Outline {
    stroke: egui::Stroke,
    radius: egui::CornerRadius,
}

impl Outline {
    fn of(ui: &egui::Ui) -> Self {
        let idle = &ui.visuals().widgets.inactive;
        Self {
            stroke: idle.bg_stroke,
            radius: idle.corner_radius,
        }
    }

    /// The frame round the row: the border's colour as its fill, the border's width as its
    /// inset on every side, the border's radius on its corners.
    fn frame(&self) -> egui::Frame {
        let width = i8::try_from(u8_from_f32_saturating(self.stroke.width)).unwrap_or(i8::MAX);
        egui::Frame::NONE
            .fill(self.stroke.color)
            .inner_margin(egui::Margin::same(width))
            .corner_radius(self.radius)
    }

    /// The corners of a segment, the `first` and/or the `last` of its row or neither: the
    /// outline's inner radius on those it shares with the outline, square where it meets a
    /// divider.
    fn corners(&self, first: bool, last: bool) -> egui::CornerRadius {
        let inset = u8_from_f32_saturating(self.stroke.width);
        let inner = |r: u8| r.saturating_sub(inset);
        let r = self.radius;
        egui::CornerRadius {
            nw: if first { inner(r.nw) } else { 0 },
            sw: if first { inner(r.sw) } else { 0 },
            ne: if last { inner(r.ne) } else { 0 },
            se: if last { inner(r.se) } else { 0 },
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

/// The segments, laid left to right in `ui`, which is the radio group; joined inside
/// `outline` where one is given. Returns whether a click moved the selection.
fn row(
    ui: &mut egui::Ui,
    selected: &mut usize,
    segments: Vec<egui::WidgetText>,
    outline: Option<&Outline>,
) -> (bool, Vec<egui::Rect>) {
    let mut changed = false;
    let mut rects = Vec::new();
    ui.ctx().accesskit_node_builder(ui.unique_id(), |node| {
        node.set_role(accesskit::Role::RadioGroup);
    });
    let mut segments = segments.into_iter().enumerate().peekable();
    while let Some((i, label)) = segments.next() {
        let mut button = egui::Button::new(label).selected(i == *selected);
        if let Some(outline) = outline {
            let last = segments.peek().is_none();
            button = button
                .stroke(egui::Stroke::NONE)
                .corner_radius(outline.corners(i == 0, last));
        }
        let response = ui.add(button);
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(accesskit::Role::RadioButton);
        });
        if response.clicked() && i != *selected {
            *selected = i;
            changed = true;
        }
        rects.push(response.rect);
    }
    (changed, rects)
}
