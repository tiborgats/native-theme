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
/// whole points, which insets the row by as much; over the group the widget paints the line
/// colour wherever no segment covers the inside -- a divider in each `separator_width` gap
/// between two segments of one row, and, wrapped, the gap between the rows and the room a
/// shorter row leaves, as the frame filled with that colour showed there -- the one thing it
/// paints itself: the model states the dividers' width and no colour of
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
/// `ui.add_enabled(false, ..)` it is its enabled self faded as one widget by its own
/// `segmented_control.disabled_opacity` (the scope's `disabled_alpha`) over its backdrop
/// ([`Self::backdrop`]), as the model defines the opacity, for the entire widget
/// (`docs/platform-facts.md` §2.1.6): the group fill over the backdrop, the outline and the
/// lines over the group, the selected segment's fill over the group, each label over its
/// segment, each faded over the backdrop and painted at the opacity the calling `Ui` had before
/// egui faded it by its own `disabled_alpha` (`Ui::disable` multiplies the painter's opacity by
/// it, `egui/src/ui.rs:497-502`, `:1588-1592`). Only that one level of disabling is taken back:
/// a `Ui` disabled twice (a nested `add_enabled_ui(false, ..)`), or restyled after it was
/// disabled, keeps the rest of its fade. Where the opacity is 1 the colours are as stated.
///
/// With no atlas installed, it adds the same buttons in the calling `Ui`'s style, in a row
/// with no outline.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct SegmentedControl<'a> {
    selected: &'a mut usize,
    segments: Vec<egui::WidgetText>,
    wrap: bool,
    backdrop: Option<egui::Color32>,
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
            backdrop: None,
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

    /// The colour under the control, which a disabled one is faded over as one widget; the
    /// window's, `defaults.background_color`, where not given.
    pub fn backdrop(mut self, backdrop: egui::Color32) -> Self {
        self.backdrop = Some(backdrop);
        self
    }
}

/// The colours a segmented control reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `defaults.background_color`: the colour faded over where no backdrop is given.
    pub(crate) background: Rgba,
    /// `segmented_control.hover_background`: what a hovered unselected segment lays over the
    /// group fill, as given; `None`, nothing.
    pub(crate) hover: Option<Rgba>,
}

impl Look {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        Self {
            background: t.defaults.background_color,
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
            backdrop,
        } = self;
        if let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) {
            let t = atlas.resolved_for(ui.ctx().theme());
            let padding = &t.segmented_control.border.padding;
            let unstated = padding.top.is_none() && padding.bottom.is_none();
            let look = Look::of(t);
            let hover = look.hover.map(to_color32);
            // A disabled calling `Ui`: the opacity its painter had before egui faded it by that
            // `Ui`'s `disabled_alpha`, which the control takes back to fade itself as one by its
            // own `disabled_opacity`, over the backdrop.
            let disabled = (!ui.is_enabled()).then(|| {
                let before = crate::fade::outer_fade(ui).map_or(ui.opacity(), |(_, before)| before);
                let ground = backdrop.unwrap_or_else(|| to_color32(look.background));
                (ground, before)
            });
            ui.native_scope(Role::SegmentedControl, RoleVariant::Normal, |ui| {
                if unstated {
                    fit_height(ui, &segments);
                }
                let mut outline = Outline::of(ui);
                lay_over_group(ui.visuals_mut(), hover);
                if let Some((ground, before)) = disabled {
                    let alpha = ui.visuals().disabled_alpha();
                    if alpha < 1.0 {
                        fade_as_one(&mut outline, ui.visuals_mut(), ground, alpha);
                    }
                    ui.set_opacity(before);
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
                    let inside = framed.response.rect.shrink(f32::from(outline.inset()));
                    outline.paint_lines(ui.painter(), inside, &rects);
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
/// `segmented_control.background_color`, its outline's colour and width, the colour of its
/// dividers and of whatever of the frame's inside no segment covers (the outline's, at rest),
/// and its corner radius.
struct Outline {
    group: egui::Color32,
    stroke: egui::Stroke,
    lines: egui::Color32,
    radius: egui::CornerRadius,
}

impl Outline {
    fn of(ui: &egui::Ui) -> Self {
        let idle = &ui.visuals().widgets.inactive;
        Self {
            group: idle.weak_bg_fill,
            stroke: idle.bg_stroke,
            lines: idle.bg_stroke.color,
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

    /// The lines, over the group fill: the border's colour wherever no segment covers the
    /// frame's inside, `inside` ([`uncovered`]): the `separator_width` gap between two
    /// neighbouring segments of one row (`rects`), the gap between two wrapped rows, the room a
    /// shorter row or segment leaves; the line colour the outline is, as the model states no
    /// colour of its own for the dividers. The one thing the control paints itself: egui's
    /// `Button` has no stroke on one side.
    fn paint_lines(&self, painter: &egui::Painter, inside: egui::Rect, rects: &[egui::Rect]) {
        for line in uncovered(inside, rects) {
            painter.rect_filled(line, egui::CornerRadius::ZERO, self.lines);
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

/// A disabled control faded as one widget by `alpha`, the scope's `disabled_alpha`
/// (`segmented_control.disabled_opacity`), over `ground`, as the model defines the opacity, for
/// the entire widget (`docs/platform-facts.md` §2.1.6): each colour composited over what it lies
/// on and faded over `ground` -- the group fill over `ground`, the outline and the lines over the
/// group, the selected segment's fill over the group, each label over its segment -- so a part
/// keeps its colour on the part under it and the whole fades. A disabled segment is in its
/// `inactive` state, as egui hovers no disabled widget (`egui/src/context.rs:1496-1500`).
fn fade_as_one(outline: &mut Outline, v: &mut egui::Visuals, ground: egui::Color32, alpha: f32) {
    let fade = |colour: egui::Color32| crate::fade::faded(ground, colour, alpha);
    let group = ground.blend(outline.group);
    let line = group.blend(outline.stroke.color);
    outline.group = fade(group);
    outline.stroke.color = fade(line);
    outline.lines = fade(line);
    let idle = &mut v.widgets.inactive;
    let label = v.override_text_color.unwrap_or(idle.fg_stroke.color);
    idle.fg_stroke.color = fade(group.blend(idle.fg_stroke.color));
    v.override_text_color = v.override_text_color.map(|_| fade(group.blend(label)));
    let selected = group.blend(v.selection.bg_fill);
    v.selection.bg_fill = fade(selected);
    v.selection.stroke.color = fade(selected.blend(v.selection.stroke.color));
}

/// What of `inside`, the frame's inside, the segments at `rects` leave uncovered, in rows as
/// they lie (a row's next segment lies right of the one before it and overlaps its height):
/// between two neighbours of a row, the `separator_width` gap, across the row's height; above or
/// below a segment shorter than its row; before a row's first segment and after its last; and
/// between the rows, and above the first and below the last, across `inside`'s width. Empty
/// rectangles are left out. With the segments of one height on one row, it is the dividers.
fn uncovered(inside: egui::Rect, rects: &[egui::Rect]) -> Vec<egui::Rect> {
    let mut rows: Vec<Vec<egui::Rect>> = Vec::new();
    for rect in rects {
        match rows.last_mut() {
            Some(row)
                if row.last().is_some_and(|last| {
                    rect.left() >= last.right()
                        && rect.top() < last.bottom()
                        && last.top() < rect.bottom()
                }) =>
            {
                row.push(*rect);
            }
            _ => rows.push(vec![*rect]),
        }
    }
    let mut out = Vec::new();
    let mut push = |x: egui::Rangef, y: egui::Rangef| {
        if x.min < x.max && y.min < y.max {
            out.push(egui::Rect::from_x_y_ranges(x, y));
        }
    };
    let mut top = inside.top();
    for row in &rows {
        let band = row.iter().fold(egui::Rangef::NOTHING, |band, r| {
            egui::Rangef::new(band.min.min(r.top()), band.max.max(r.bottom()))
        });
        push(inside.x_range(), egui::Rangef::new(top, band.min));
        let mut left = inside.left();
        for r in row {
            push(egui::Rangef::new(left, r.left()), band);
            push(r.x_range(), egui::Rangef::new(band.min, r.top()));
            push(r.x_range(), egui::Rangef::new(r.bottom(), band.max));
            left = r.right();
        }
        push(egui::Rangef::new(left, inside.right()), band);
        top = band.max;
    }
    push(inside.x_range(), egui::Rangef::new(top, inside.bottom()));
    out
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
