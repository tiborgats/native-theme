//! [`SegmentedControl`]: one exclusive choice among a row of segments.

use native_theme_egui::convert::{to_color32, u8_from_f32_saturating};
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
/// It has no `enabled`: `segmented_control` states no disabled colour, so under
/// `ui.add_enabled(false, ..)` it is its enabled self faded by the calling `Ui`'s
/// `disabled_alpha`, as one widget over its backdrop ([`Self::backdrop`]), as libadwaita fades
/// the whole widget (the crate's *Disabled*): the outline's fill over the backdrop, each
/// segment's fill over the outline's, each label over its segment, each faded over the
/// backdrop and painted at the opacity the calling `Ui` had before egui faded it
/// (`Ui::disable` multiplies the painter's opacity by `disabled_alpha`,
/// `egui/src/ui.rs:497-502`, `:1588-1592`), so the outline's colour does not show through a
/// segment. Only that one level of disabling is taken back: a `Ui` disabled twice (a nested
/// `add_enabled_ui(false, ..)`), or restyled after it was disabled, keeps the rest of its
/// fade. Where `disabled_alpha` is 1 the colours are as stated.
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
    /// to discard the pass and run it again with them known (`Context::request_discard`).
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
            // The fade a disabled calling `Ui` gave the painter, which the control takes back to
            // fade itself as one.
            let fade = crate::fade::outer_fade(ui).map(|under| {
                let ground = backdrop.unwrap_or_else(|| to_color32(t.defaults.background_color));
                (ground, under)
            });
            ui.native_scope(Role::SegmentedControl, RoleVariant::Normal, |ui| {
                if unstated {
                    fit_height(ui, &segments);
                }
                if let Some((ground, (alpha, opacity))) = fade {
                    fade_as_one(ui.visuals_mut(), ground, alpha);
                    ui.set_opacity(opacity);
                }
                let outline = Outline::of(ui);
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
                    let (changed, rects) = outline
                        .frame()
                        .show(ui, |ui| {
                            row(ui, selected, segments, Some((&outline, shared.as_slice())))
                        })
                        .inner;
                    // egui's way with a guess that layout proves wrong: run the pass again
                    // (`egui/src/context.rs:1980-1992`, `Options::max_passes`).
                    if wrap && by_place(&rects) != shared {
                        ui.ctx().request_discard("segmented control wrap");
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
        let mut later = segments.iter();
        later.next();
        for (i, segment) in segments.iter().enumerate() {
            parts.push(format!("segment_{i}"), *segment);
            // The outline's colour shows between two segments of one row: the gap the row
            // leaves. A wrapped row's next segment starts the next row.
            if let Some(next) = later.next().filter(|next| {
                next.left() >= segment.right() && next.y_range() == segment.y_range()
            }) {
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

/// The segmented control scope's colours faded as one widget over `ground` by `alpha`: the
/// outline's fill, the border colour, over `ground`; a segment's fill, `inactive` at rest and
/// `selection` while selected (`egui/src/widget_style.rs:150-159`), over the outline's; its
/// label over its segment; each faded over `ground`. A disabled segment is in its `inactive`
/// state, as egui hovers no disabled widget (`egui/src/context.rs:1496-1500`).
fn fade_as_one(v: &mut egui::Visuals, ground: egui::Color32, alpha: f32) {
    let fade = |under: egui::Color32, colour: egui::Color32| {
        crate::fade::faded(ground, under.blend(colour), alpha)
    };
    let outline = ground.blend(v.widgets.inactive.bg_stroke.color);
    let idle = &mut v.widgets.inactive;
    let segment = outline.blend(idle.weak_bg_fill);
    let label = v.override_text_color.unwrap_or(idle.fg_stroke.color);
    idle.bg_stroke.color = fade(ground, outline);
    idle.weak_bg_fill = fade(outline, idle.weak_bg_fill);
    idle.fg_stroke.color = fade(segment, idle.fg_stroke.color);
    v.override_text_color = v.override_text_color.map(|_| fade(segment, label));
    let selected = outline.blend(v.selection.bg_fill);
    v.selection.bg_fill = fade(outline, v.selection.bg_fill);
    v.selection.stroke.color = fade(selected, v.selection.stroke.color);
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
