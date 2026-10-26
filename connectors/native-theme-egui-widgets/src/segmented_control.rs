//! [`SegmentedControl`]: one exclusive choice among a row of segments.

use native_theme_egui::convert::{to_color32, u8_from_f32_saturating};
use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::native_theme::color::Rgba;
use native_theme_egui::{NativeThemeUiExt as _, ResolvedTheme, Role, RoleVariant, ThemeAtlas};

use crate::parts::Parts;

/// One control of joined segments, one of them selected: `segmented_control.*`.
///
/// **Tier C.** One horizontal row of `Button::new(label).selected(i == *selected)`
/// (`egui/src/widgets/button.rs:270`) in one `Role::SegmentedControl` scope for the whole row.
/// The scope's `Normal` cell carries `segmented_control.background_color`, `hover_background`,
/// `font` and `border.*`, `active_background` and `active_text_color` in `selection.*`, which
/// egui paints only on the selected button (`egui/src/widget_style.rs:150-155`),
/// `segment_height` and the segments' padding, and `separator_width` as the gap between the
/// segments (`spacing.item_spacing.x`). Each segment is a framed button whose resting fill
/// the widget clears, so the group fill shows through it (*Joined*). A click on a segment
/// selects it.
///
/// **Joined.** A segmented control is one control parted by dividers on every platform the
/// presets model (`docs/platform-facts.md` §2.25: `separator_width` is "the width of the
/// divider line between segments"). The row sits in one `egui::Frame` — the non-interactive
/// decoration of a composed widget — filled with the group fill,
/// `segmented_control.background_color`, rounded to the scope's `border.corner_radius` and
/// stroked inside its edge with the outline, the border's colour as wide as the border in
/// whole points, which insets the row by as much; over the group the widget paints a divider
/// of that colour in each `separator_width` gap between two segments of one row, the one thing
/// it paints itself: the model states the dividers' width and no colour of
/// their own, so they are the one line colour the control states. A segment lays only its own
/// fill over the group: none at rest, `hover_background` as given under the pointer and
/// pressed, `active_background` while selected, so a translucent group fill, as windows-11
/// states, shows once, over what lies under the control, as gpui's showcase draws it. The
/// segments paint no stroke of their own (`Button::stroke`,
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
/// It has no `enabled`: `segmented_control` states no disabled colour, so under
/// `ui.add_enabled(false, ..)` its segments are their enabled selves faded by the calling
/// `Ui`'s `disabled_alpha` over the group, which does not fade
/// (`docs/platform-facts.md` §2.25 `disabled_opacity`): the selected segment's fill and each
/// label at that alpha times
/// its own, the group fill, the outline and the dividers as stated, all painted at the opacity
/// the calling `Ui` had before egui faded it (`Ui::disable` multiplies the painter's opacity by
/// `disabled_alpha`, `egui/src/ui.rs:497-502`, `:1588-1592`). Only that one level of disabling
/// is taken back: a `Ui` disabled twice (a nested `add_enabled_ui(false, ..)`), or restyled
/// after it was disabled, keeps the rest of its fade. Where `disabled_alpha` is 1 the colours
/// are as stated.
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
    ///
    /// A segment rounds the corners it shares with the outline in the rows as placed: the
    /// first row's first and last segments the outline's top corners, the last row's its
    /// bottom ones, where they reach them. egui places the segments one by one, so the rows
    /// are known only after a pass: each pass takes them from the one before, and where they
    /// came out otherwise (the first pass, a changed segment list, a resized `Ui`) it asks egui
    /// to discard the pass and run it again with them known (`Context::request_discard`), and
    /// to repaint, so a pass egui declines to run again is corrected in the next; a pass that
    /// is not shown (an invisible `Ui`, a sizing pass) asks for neither.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
}

/// The colours a segmented control reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `segmented_control.hover_background`: what a hovered unselected segment lays over the
    /// group fill, as given; `None`, nothing.
    pub(crate) hover: Option<Rgba>,
}

impl Look {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        Self {
            hover: t.segmented_control.hover_background,
        }
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
            let t = atlas.resolved_for(ui.ctx().theme());
            let padding = &t.segmented_control.border.padding;
            let unstated = padding.top.is_none() && padding.bottom.is_none();
            let hover = Look::of(t).hover.map(to_color32);
            // The fade a disabled calling `Ui` gave the painter, which the control takes back to
            // fade its segments alone.
            let fade = crate::fade::outer_fade(ui);
            ui.native_scope(Role::SegmentedControl, RoleVariant::Normal, |ui| {
                if unstated {
                    fit_height(ui, &segments);
                }
                let outline = Outline::of(ui);
                lay_over_group(ui.visuals_mut(), hover);
                if let Some((alpha, opacity)) = fade {
                    fade_segments(ui.visuals_mut(), alpha);
                    ui.set_opacity(opacity);
                }
                let contents = |ui: &mut egui::Ui| {
                    let count = segments.len();
                    let shared = if wrap {
                        // The rows the last pass placed the segments in, kept under the id the
                        // row's response answers with: its `Ui`'s (`egui/src/ui.rs:2203-2215`).
                        Parts::of_id(ui.ctx(), ui.unique_id())
                            .and_then(|parts| segment_rects(&parts, count))
                            .map_or_else(|| by_index(count), |rects| by_place(&rects))
                    } else {
                        by_index(count)
                    };
                    let framed = outline.frame().show(ui, |ui| {
                        row(ui, selected, segments, Some((&outline, shared.as_slice())))
                    });
                    let (changed, rects) = framed.inner;
                    outline.paint_dividers(ui.painter(), &rects);
                    // egui's way with a guess that layout proves wrong, as its `Grid` does it:
                    // run the pass again where it is shown (`egui/src/grid.rs:466-468`,
                    // `egui/src/context.rs:1980-1992`), and repaint (`egui/src/grid.rs:290-294`),
                    // so a pass egui does not run again (`Options::max_passes`) is corrected in
                    // the next one.
                    let shown = ui.is_visible() && !ui.is_sizing_pass();
                    if wrap && shown && by_place(&rects) != shared {
                        ui.ctx().request_discard("segmented control wrap");
                        ui.ctx().request_repaint();
                    }
                    (changed, rects)
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
/// changed when it did, with the row's segments as its `Parts`, and a divider between each
/// two neighbours in one row.
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
        let dividers = dividers(&segments);
        for (i, segment) in segments.iter().enumerate() {
            parts.push(format!("segment_{i}"), *segment);
            if let Some((_, divider)) = dividers.iter().find(|(after, _)| *after == i) {
                parts.push(format!("divider_{i}"), *divider);
            }
        }
        parts.store(&self.response.ctx, self.response.id);
        self.response
    }
}

/// The dividers between the segments at `segments`, each with the index of the segment it
/// follows: between two segments of one row, side by side and overlapping in height, of one
/// height or two, the gap the row leaves. A wrapped row's next segment starts the next row.
fn dividers(segments: &[egui::Rect]) -> Vec<(usize, egui::Rect)> {
    let mut later = segments.iter();
    later.next();
    segments
        .iter()
        .zip(later)
        .enumerate()
        .filter(|(_, (segment, next))| {
            next.left() >= segment.right()
                && next.top() < segment.bottom()
                && segment.top() < next.bottom()
        })
        .map(|(i, (segment, next))| {
            (
                i,
                egui::Rect::from_x_y_ranges(segment.right()..=next.left(), segment.y_range()),
            )
        })
        .collect()
}

/// The joined control, from the segmented control scope's own resting cell: its group fill,
/// `segmented_control.background_color`, its outline's colour and width, and its corner radius.
struct Outline {
    group: egui::Color32,
    stroke: egui::Stroke,
    radius: egui::CornerRadius,
}

impl Outline {
    fn of(ui: &egui::Ui) -> Self {
        let idle = &ui.visuals().widgets.inactive;
        Self {
            group: idle.weak_bg_fill,
            stroke: idle.bg_stroke,
            radius: idle.corner_radius,
        }
    }

    /// The outline's width in whole points, the frame's stroke and the row's inset on every
    /// side.
    fn inset(&self) -> u8 {
        u8_from_f32_saturating(self.stroke.width)
    }

    /// The frame round the row: the group fill, inside the outline, the border's colour as
    /// wide as the border in whole points, which the frame strokes inside its rect and lays the
    /// row inside (`egui/src/containers/frame.rs`, `Frame::paint`, `Frame::widget_rect`), and the
    /// border's radius on its corners.
    fn frame(&self) -> egui::Frame {
        egui::Frame::NONE
            .fill(self.group)
            .stroke(egui::Stroke::new(
                f32::from(self.inset()),
                self.stroke.color,
            ))
            .corner_radius(self.radius)
    }

    /// The dividers, over the group fill: the border's colour between each two neighbouring
    /// segments of one row (`rects`), the `separator_width` gap the row leaves. The one thing
    /// the control paints itself: egui's `Button` has no stroke on one side.
    fn paint_dividers(&self, painter: &egui::Painter, rects: &[egui::Rect]) {
        for (_, divider) in dividers(rects) {
            painter.rect_filled(divider, egui::CornerRadius::ZERO, self.stroke.color);
        }
    }

    /// The corners of a segment: the outline's inner radius on those `shared` with the
    /// outline, square where it meets a divider or the outline's straight edge.
    fn corners(&self, shared: Shared) -> egui::CornerRadius {
        let inset = u8_from_f32_saturating(self.stroke.width);
        let inner = |on: bool, r: u8| if on { r.saturating_sub(inset) } else { 0 };
        let r = self.radius;
        egui::CornerRadius {
            nw: inner(shared.nw, r.nw),
            ne: inner(shared.ne, r.ne),
            sw: inner(shared.sw, r.sw),
            se: inner(shared.se, r.se),
        }
    }
}

/// Which corners of a segment lie on the outline's corners.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Shared {
    nw: bool,
    ne: bool,
    sw: bool,
    se: bool,
}

/// How far from the outline's corner a segment's corner may lie and still be on it: a point,
/// the plan's ruling (M7-19), for the rounding of egui's layout.
const ON_CORNER: f32 = 1.0;

impl Shared {
    /// The corners of `segment` that lie on a corner of `outline`, within [`ON_CORNER`].
    fn by_place(segment: egui::Rect, outline: egui::Rect) -> Self {
        let on = |a: f32, b: f32| (a - b).abs() <= ON_CORNER;
        let (left, right) = (
            on(segment.left(), outline.left()),
            on(segment.right(), outline.right()),
        );
        let (top, bottom) = (
            on(segment.top(), outline.top()),
            on(segment.bottom(), outline.bottom()),
        );
        Self {
            nw: left && top,
            ne: right && top,
            sw: left && bottom,
            se: right && bottom,
        }
    }
}

/// The shared corners of `count` segments in one row: the first one's left corners and the
/// last one's right.
fn by_index(count: usize) -> Vec<Shared> {
    let mut shared: Vec<Shared> = (0..count)
        .map(|i| Shared {
            nw: i == 0,
            sw: i == 0,
            ..Shared::default()
        })
        .collect();
    if let Some(last) = shared.last_mut() {
        last.ne = true;
        last.se = true;
    }
    shared
}

/// The shared corners of segments placed at `rects`, whose rows the outline holds: the
/// corners on the corners of the rectangle round them all.
fn by_place(rects: &[egui::Rect]) -> Vec<Shared> {
    let outline = rects
        .iter()
        .fold(egui::Rect::NOTHING, |outline, rect| outline.union(*rect));
    rects
        .iter()
        .map(|rect| Shared::by_place(*rect, outline))
        .collect()
}

/// The rectangles of the `count` segments `parts` records, where it records that many.
fn segment_rects(parts: &Parts, count: usize) -> Option<Vec<egui::Rect>> {
    let recorded = parts
        .iter()
        .filter(|(name, _)| name.starts_with("segment_"))
        .count();
    if recorded != count {
        return None;
    }
    (0..count)
        .map(|i| parts.get(&format!("segment_{i}")))
        .collect()
}

/// The scope's segment colours as what a segment lays over the group fill: an unselected
/// segment paints no fill at rest (`inactive` and `open`, egui's resting cells for a button) and
/// `hover`, `segmented_control.hover_background` as given, hovered and pressed (the model states
/// no pressed segment), nothing where that soft option is `None`; the selected one paints
/// `selection.bg_fill`, `active_background`, which already lies over the group
/// (`egui/src/widget_style.rs:150-155`). The group fill itself is the frame's ([`Outline`]).
fn lay_over_group(v: &mut egui::Visuals, hover: Option<egui::Color32>) {
    let w = &mut v.widgets;
    w.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    w.open.weak_bg_fill = egui::Color32::TRANSPARENT;
    let hovered = hover.unwrap_or(egui::Color32::TRANSPARENT);
    w.hovered.weak_bg_fill = hovered;
    w.active.weak_bg_fill = hovered;
}

/// A disabled control's segments faded by `alpha` over the group fill, which does not fade
/// (`docs/platform-facts.md` §2.25 `disabled_opacity`): the selected segment's fill and its
/// label, and an unselected one's label, each at `alpha` times its own alpha. A disabled
/// segment is in its `inactive` state, as egui hovers no disabled widget
/// (`egui/src/context.rs:1496-1500`).
fn fade_segments(v: &mut egui::Visuals, alpha: f32) {
    let fade = |colour: egui::Color32| colour.gamma_multiply(alpha);
    let idle = &mut v.widgets.inactive;
    idle.fg_stroke.color = fade(idle.fg_stroke.color);
    v.override_text_color = v.override_text_color.map(fade);
    v.selection.bg_fill = fade(v.selection.bg_fill);
    v.selection.stroke.color = fade(v.selection.stroke.color);
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

/// The segments, laid left to right in `ui`, which is the radio group; joined inside the
/// outline where one is given, each segment rounding the corners it shares with it. Returns
/// whether a click moved the selection, and where each segment lies.
fn row(
    ui: &mut egui::Ui,
    selected: &mut usize,
    segments: Vec<egui::WidgetText>,
    joined: Option<(&Outline, &[Shared])>,
) -> (bool, Vec<egui::Rect>) {
    let mut changed = false;
    let mut rects = Vec::new();
    ui.ctx().accesskit_node_builder(ui.unique_id(), |node| {
        node.set_role(accesskit::Role::RadioGroup);
    });
    for (i, label) in segments.into_iter().enumerate() {
        let mut button = egui::Button::new(label).selected(i == *selected);
        if let Some((outline, shared)) = joined {
            let shared = shared.get(i).copied().unwrap_or_default();
            button = button
                .stroke(egui::Stroke::NONE)
                .corner_radius(outline.corners(shared));
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
