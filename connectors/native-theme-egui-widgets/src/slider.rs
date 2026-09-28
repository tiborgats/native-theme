//! [`Slider`]: a horizontal slider whose knob is not its rail.

use std::ops::RangeInclusive;

use native_theme_egui::convert::{composite_over, to_color32};
use native_theme_egui::egui::emath::Numeric as _;
use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::native_theme::theme::ResolvedSliderTheme;
use native_theme_egui::{Role, ThemeAtlas};

use crate::scope;

/// How far, in points, one arrow-key press or AccessKit increment moves the knob: egui's own
/// slider step (`ui_point_per_step`, `egui/src/widgets/slider.rs:722`).
const KEY_STEP: f32 = 1.0;

/// A horizontal slider over a linear range: `slider.*`'s rail, trailing fill and knob.
///
/// **Tier P.** egui paints the rail in `widgets.inactive.bg_fill`
/// (`egui/src/widgets/slider.rs:774-775`) and the resting knob in the fill of the widget's
/// interaction state (`:766`, `:813-818`), which at rest is that same `inactive.bg_fill`, so
/// in any `Style` knob and rail are one colour at rest, while the platform presets state a
/// knob colour (`slider.thumb_color`) distinct from the rail's (`slider.track_color`).
///
/// * The rail is `slider.track_height` tall across the widget in `track_color`, at the
///   slider scope's `widgets.inactive.corner_radius` (egui's own rail radius, `:772`; the
///   theme states none); the trailing fill runs from its start to the knob's centre in
///   `fill_color`.
/// * The knob is a `slider.thumb_diameter` circle, its outline included, in `thumb_color`,
///   with `thumb_hover_color` composited over it while hovered; its outline is egui's, the
///   interaction state's `fg_stroke` (`:817`), as the theme states none. Its centre travels
///   the width less one knob radius at each end, as egui's `position_range` does
///   (`:853-865`).
/// * The width is egui's `Spacing::slider_width`, the height the larger of `thumb_diameter`
///   and `track_height`. No tick marks and no value field are drawn.
/// * `.enabled(false)` paints `disabled_track_color`, `disabled_fill_color` and
///   `disabled_thumb_color` (a `None` copies the colour it stands for), unfaded;
///   `ui.add_enabled(false, ..)` fades them at the calling `Ui`'s `disabled_alpha` on top.
///
/// Interaction is egui's on a linear range: it senses `Sense::drag()` (`:655`); a drag sets
/// the value from the pointer (`:666-678`); while focused, the arrow keys along the rail move
/// it one point per press (`:722`) and do not move focus (`:684-698`); AccessKit's
/// `Increment`, `Decrement` and `SetValue` act as in egui (`:713-717`, `:753-760`). The value
/// is kept inside the range. `label` is shown after the rail and names the slider
/// (`WidgetInfo::slider`, `:967`).
///
/// With no atlas installed, or a slider size that is not finite, it adds `egui::Slider`
/// without its value field.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Slider<'a> {
    value: &'a mut f64,
    range: RangeInclusive<f64>,
    label: Option<egui::WidgetText>,
    enabled: bool,
}

impl<'a> Slider<'a> {
    /// A slider showing, and setting, `value` within `range`.
    pub fn new(value: &'a mut f64, range: RangeInclusive<f64>) -> Self {
        Self {
            value,
            range,
            label: None,
            enabled: true,
        }
    }

    /// The text after the rail, which also names the slider for assistive technology.
    pub fn label(mut self, text: impl Into<egui::WidgetText>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// `false` shows the platform's disabled slider, unfaded, and takes no input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// The knob and rail sizes, every one finite.
#[derive(Clone, Copy)]
struct Geometry {
    thumb: f32,
    rail: f32,
}

impl Geometry {
    fn of(sl: &ResolvedSliderTheme) -> Option<Self> {
        Some(Self {
            thumb: scope::length(sl.thumb_diameter)?,
            rail: scope::length(sl.track_height)?,
        })
    }
}

impl egui::Widget for Slider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let paint = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let sl = &atlas.resolved_for(ui.ctx().theme()).slider;
            Geometry::of(sl).map(|g| (g, sl.clone()))
        });
        let Slider {
            value,
            range,
            label,
            enabled,
        } = self;
        let Some((geometry, sl)) = paint else {
            let mut slider = egui::Slider::new(value, range).show_value(false);
            if let Some(label) = label {
                slider = slider.text(label);
            }
            return ui.add_enabled(enabled, slider);
        };
        scope::open(ui, Role::Slider, enabled, |ui| match label {
            Some(label) => {
                let row = ui.horizontal(|ui| {
                    let slider = slider_ui(ui, value, &range, Some(label.text()), geometry, &sl);
                    ui.label(label);
                    slider
                });
                row.inner.union(row.response)
            }
            None => slider_ui(ui, value, &range, None, geometry, &sl),
        })
    }
}

/// `v` inside `range`, whichever way round it is; `NaN` is its start.
fn clamp_to(v: f64, range: &RangeInclusive<f64>) -> f64 {
    let (lo, hi) = (
        range.start().min(*range.end()),
        range.start().max(*range.end()),
    );
    if v.is_nan() { lo } else { v.max(lo).min(hi) }
}

/// Where along `range` the value sits, from `0.0` at its start to `1.0` at its end; `0.0`
/// where the range is empty or not finite.
#[allow(
    clippy::manual_clamp,
    reason = "f64::clamp is not used on theme or input values: `max` then `min`, as the connector's `convert`"
)]
fn fraction(v: f64, range: &RangeInclusive<f64>) -> f32 {
    let (start, end) = (*range.start(), *range.end());
    let span = end - start;
    if !span.is_finite() || span == 0.0 {
        return 0.0;
    }
    let t = (clamp_to(v, range) - start) / span;
    f32::from_f64(t.max(0.0).min(1.0))
}

/// The value at `t` along `range`, `t` from `0.0` to `1.0`.
fn value_at(t: f32, range: &RangeInclusive<f64>) -> f64 {
    let (start, end) = (*range.start(), *range.end());
    clamp_to(start + f64::from(t) * (end - start), range)
}

fn slider_ui(
    ui: &mut egui::Ui,
    value: &mut f64,
    range: &RangeInclusive<f64>,
    label: Option<&str>,
    g: Geometry,
    sl: &ResolvedSliderTheme,
) -> egui::Response {
    let old = *value;
    *value = clamp_to(*value, range);
    let size = egui::vec2(ui.spacing().slider_width, g.thumb.max(g.rail));
    let mut response = ui.allocate_response(size, egui::Sense::drag());
    let rect = response.rect;
    let knob_radius = 0.5 * g.thumb;
    let travel = rect.x_range().shrink(knob_radius);
    let position_of = |v: f64| egui::lerp(travel.min..=travel.max, fraction(v, range));
    let value_of = |x: f32| {
        if travel.span() > 0.0 {
            value_at(egui::remap_clamp(x, travel, 0.0..=1.0), range)
        } else {
            *range.start()
        }
    };

    if let Some(pointer) = response.interact_pointer_pos()
        && pointer.x.is_finite()
    {
        *value = value_of(pointer.x);
    }
    let (mut decrement, mut increment) = (0, 0);
    if response.has_focus() {
        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                response.id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    ..Default::default()
                },
            );
        });
        ui.input(|i| {
            decrement += i.num_presses(egui::Key::ArrowLeft);
            increment += i.num_presses(egui::Key::ArrowRight);
        });
    }
    ui.input(|i| {
        decrement += i.num_accesskit_action_requests(response.id, accesskit::Action::Decrement);
        increment += i.num_accesskit_action_requests(response.id, accesskit::Action::Increment);
    });
    if increment != decrement {
        let mut x = position_of(*value);
        for _ in 0..increment {
            x += KEY_STEP;
        }
        for _ in 0..decrement {
            x -= KEY_STEP;
        }
        *value = value_of(x);
    }
    ui.input(|i| {
        for request in i.accesskit_action_requests(response.id, accesskit::Action::SetValue) {
            if let Some(accesskit::ActionData::NumericValue(v)) = request.data {
                *value = clamp_to(v, range);
            }
        }
    });
    if *value != old {
        response.mark_changed();
    }

    let enabled = ui.is_enabled();
    let current = *value;
    response.widget_info(|| egui::WidgetInfo::slider(enabled, current, label.unwrap_or_default()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_min_numeric_value(*range.start());
        node.set_max_numeric_value(*range.end());
        node.add_action(accesskit::Action::SetValue);
        if current < range.start().max(*range.end()) {
            node.add_action(accesskit::Action::Increment);
        }
        if current > range.start().min(*range.end()) {
            node.add_action(accesskit::Action::Decrement);
        }
    });

    if ui.is_rect_visible(rect) {
        let (track, fill, thumb) = if enabled {
            let thumb = match sl.thumb_hover_color {
                Some(layer) if response.hovered() => composite_over(layer, sl.thumb_color),
                _ => to_color32(sl.thumb_color),
            };
            (sl.track_color, sl.fill_color, thumb)
        } else {
            (
                sl.disabled_track_color.unwrap_or(sl.track_color),
                sl.disabled_fill_color.unwrap_or(sl.fill_color),
                to_color32(sl.disabled_thumb_color.unwrap_or(sl.thumb_color)),
            )
        };
        let rail = egui::Rect::from_center_size(rect.center(), egui::vec2(rect.width(), g.rail));
        let radius = ui.visuals().widgets.inactive.corner_radius;
        let centre = egui::pos2(position_of(current), rail.center().y);
        let mut trailing = rail;
        trailing.max.x = centre.x;
        let outline = ui.style().interact(&response).fg_stroke;
        let painter = ui.painter();
        painter.rect_filled(rail, radius, to_color32(track));
        painter.rect_filled(trailing, radius, to_color32(fill));
        // epaint strokes a circle outside its radius (`epaint/src/tessellator.rs:1531`), so
        // the disc is the knob less its outline.
        painter.add(egui::epaint::CircleShape {
            center: centre,
            radius: (knob_radius - outline.width).max(0.0),
            fill: thumb,
            stroke: outline,
        });
    }
    response
}
