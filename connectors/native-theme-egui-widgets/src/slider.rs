//! [`Slider`]: a horizontal slider whose knob is not its rail.

use std::ops::RangeInclusive;

use native_theme_egui::convert::{composite_over, to_color32};
use native_theme_egui::egui::emath::Numeric as _;
use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::native_theme::color::Rgba;
use native_theme_egui::native_theme::next::{ResolvedTheme as NextResolvedTheme, WidgetState};
use native_theme_egui::native_theme::theme::ResolvedSliderTheme;
use native_theme_egui::next::stroke_width_of;
use native_theme_egui::{NativeThemeUiExt as _, ResolvedTheme, Role, RoleVariant, ThemeAtlas};

use crate::parts::Parts;
use crate::{next, scope};

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
///   with `thumb_hover_color` composited over it while hovered. Where the theme states
///   `slider.thumb_border_line_width`, the outline is that wide inside the knob: at rest in
///   `thumb_border_color`, with the pointer on the knob or while it is dragged in
///   `thumb_hover_border_color`, while the slider has the focus in `thumb_focus_border_color`
///   (hover wins over focus; each `None` is the rest colour), a disabled knob in the rest
///   colour, faded as the slider is; a stated 0, or a width with no rest colour at rest, draws
///   no line, the knob whole at its diameter. egui gives a slider the focus by keyboard only,
///   so the focus colour shows after Tab, not after a click. Where the width is unstated the
///   outline is egui's, the interaction state's `fg_stroke` (`:817`). Its centre travels
///   the width less one knob radius at each end, as egui's `position_range` does
///   (`:853-865`).
/// * The width is egui's `Spacing::slider_width` (`egui/src/style.rs:412`) of the `Ui` it is
///   added to, read before the slider scope opens, so an application sizes it as it sizes
///   egui's own; the height is the larger of `thumb_diameter` and `track_height`. No tick marks
///   and no value field are drawn.
/// * `.enabled(false)` paints `disabled_track_color`, `disabled_fill_color` and
///   `disabled_thumb_color` (a `None` copies the colour it stands for), faded by
///   `slider.disabled_opacity` as one widget over its backdrop ([`Self::backdrop`]), as
///   libadwaita fades the whole widget (the crate's *Disabled*): the rail over the backdrop,
///   the trailing fill over the rail, the knob and its outline over the backdrop, each faded
///   over the backdrop and painted at the opacity the `Ui` had, so no part shows another
///   through it. The knob is taken over the backdrop, where a knob taller than the rail
///   mostly lies; where it overlaps the rail and the fill, a translucent knob
///   (macos-monterey's dark `thumb_color` `#ffffff80`, macos-tahoe's dark `#ffffffd9`,
///   material-3's `disabled_thumb_color` at alpha `0x61`) shows the backdrop's composite
///   there, not the rail's or the fill's; an opaque knob is exact everywhere. Under
///   `ui.add_enabled(false, ..)` it is faded as one by the calling `Ui`'s `disabled_alpha`,
///   the fade egui gave the painter taken back.
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
    backdrop: Option<egui::Color32>,
}

impl<'a> Slider<'a> {
    /// A slider showing, and setting, `value` within `range`.
    pub fn new(value: &'a mut f64, range: RangeInclusive<f64>) -> Self {
        Self {
            value,
            range,
            label: None,
            enabled: true,
            backdrop: None,
        }
    }

    /// The text after the rail, which also names the slider for assistive technology.
    pub fn label(mut self, text: impl Into<egui::WidgetText>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// `false` shows the platform's disabled slider, its disabled colours faded by its
    /// `disabled_opacity` as one widget over its backdrop ([`Self::backdrop`]), and takes no
    /// input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The colour under the slider, which a disabled one is faded over as one widget; the
    /// window's, `defaults.background_color`, where not given.
    pub fn backdrop(mut self, backdrop: egui::Color32) -> Self {
        self.backdrop = Some(backdrop);
        self
    }
}

/// How a disabled slider is faded as one widget: over `ground`, by the scope's
/// `disabled_alpha` once faded, below 1, its knob's outline the `Disabled` cell's before the
/// fade, the stroke of a disabled response's `inactive` state (egui hovers no disabled widget,
/// `egui/src/context.rs:1496-1500`; `egui/src/style.rs:1273-1283`).
#[derive(Clone, Copy)]
struct Fade {
    ground: egui::Color32,
    alpha: f32,
    outline: egui::Stroke,
}

impl Fade {
    /// The fade of the faded slider scope `ui` over `ground`, its knob's `outline` read before
    /// the fade; `None` where `disabled_alpha` is 1, a platform that dims by colour alone,
    /// whose colours are painted as stated.
    fn of(ui: &egui::Ui, ground: egui::Color32, outline: egui::Stroke) -> Option<Self> {
        let alpha = ui.visuals().disabled_alpha();
        (alpha < 1.0).then_some(Self {
            ground,
            alpha,
            outline,
        })
    }

    /// `colour` over `under`, faded over the ground: exact for an opaque `colour`, and for a
    /// translucent one where `under` is what lies beneath it.
    fn over(&self, under: egui::Color32, colour: egui::Color32) -> egui::Color32 {
        crate::fade::faded(self.ground, under.blend(colour), self.alpha)
    }
}

/// The colours a slider reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    pub(crate) track_color: Rgba,
    pub(crate) fill_color: Rgba,
    pub(crate) thumb_color: Rgba,
    pub(crate) thumb_hover_color: Option<Rgba>,
    pub(crate) disabled_track_color: Option<Rgba>,
    pub(crate) disabled_fill_color: Option<Rgba>,
    pub(crate) disabled_thumb_color: Option<Rgba>,
    pub(crate) thumb_border_color: Option<Rgba>,
    pub(crate) thumb_hover_border_color: Option<Rgba>,
    pub(crate) thumb_focus_border_color: Option<Rgba>,
    /// `defaults.background_color`: the colour faded over where no backdrop is given; `None`,
    /// egui's own fade of a disabled widget.
    pub(crate) background: Option<Rgba>,
}

impl Look {
    /// The colours of a slider of a 0.8.0 theme, at the homes of the 0.7.2 fields `of` reads:
    /// `slider.track`, `slider.active_track` and `slider.thumb`'s fills at `NONE`, and at
    /// `DISABLED` where a rule for it states them; the thumb's hover layer
    /// (`slider.thumb(HOVER)`'s fill) where a rule for `HOVER` states it (spec §2,
    /// `from_state`); the thumb's outline colour at `NONE`, and at `HOVER` and `FOCUSED` where
    /// a rule states it; the window's background as the ground. `None` where the theme states
    /// no track, filled track or thumb fill, which sends the slider to egui's own.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Option<Self> {
        let sl = &r.slider;
        let off = WidgetState::DISABLED;
        let rest = sl.thumb(WidgetState::NONE);
        let (hover, focus) = (sl.thumb(WidgetState::HOVER), sl.thumb(WidgetState::FOCUSED));
        Some(Self {
            track_color: next::fill(&sl.track(WidgetState::NONE))?,
            fill_color: next::fill(&sl.active_track(WidgetState::NONE))?,
            thumb_color: next::fill(&rest)?,
            thumb_hover_color: next::ruled_fill(&hover, &hover),
            disabled_track_color: next::ruled_fill(&sl.track(off), &sl.track(off)),
            disabled_fill_color: next::ruled_fill(&sl.active_track(off), &sl.active_track(off)),
            disabled_thumb_color: next::ruled_fill(&sl.thumb(off), &sl.thumb(off)),
            thumb_border_color: next::stroke(&rest),
            thumb_hover_border_color: next::ruled_stroke(&hover, &hover),
            thumb_focus_border_color: next::ruled_stroke(&focus, &focus),
            background: next::window_background(r),
        })
    }

    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        let sl = &t.slider;
        Self {
            track_color: sl.track_color,
            fill_color: sl.fill_color,
            thumb_color: sl.thumb_color,
            thumb_hover_color: sl.thumb_hover_color,
            disabled_track_color: sl.disabled_track_color,
            disabled_fill_color: sl.disabled_fill_color,
            disabled_thumb_color: sl.disabled_thumb_color,
            thumb_border_color: sl.thumb_border_color,
            thumb_hover_border_color: sl.thumb_hover_border_color,
            thumb_focus_border_color: sl.thumb_focus_border_color,
            background: Some(t.defaults.background_color),
        }
    }
}

/// The knob and rail sizes, every one finite, the slider's width, and the knob's outline width
/// where the theme states one (`slider.thumb_border_line_width`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Geometry {
    pub(crate) thumb: f32,
    pub(crate) rail: f32,
    pub(crate) width: f32,
    pub(crate) outline: Option<f32>,
}

impl Geometry {
    pub(crate) fn of(sl: &ResolvedSliderTheme, width: f32) -> Option<Self> {
        Self::checked(
            sl.thumb_diameter,
            sl.track_height,
            width,
            sl.thumb_border_line_width,
        )
    }

    /// The sizes of a slider of a 0.8.0 theme, `width` wide: `slider.thumb(NONE)`'s minimum
    /// width, `slider.track(NONE)`'s minimum height and the thumb's outline width where it is
    /// stated as one stroke, or as none, a width of 0 (spec §10, rule 2). `None` where the theme states no thumb or track size, which sends
    /// the slider to egui's own.
    pub(crate) fn next_of(r: &NextResolvedTheme, width: f32) -> Option<Self> {
        let sl = &r.slider;
        let thumb = sl.thumb(WidgetState::NONE);
        let [diameter, _] = thumb.min_size;
        let [_, rail] = sl.track(WidgetState::NONE).min_size;
        Self::checked(diameter?, rail?, width, stroke_width_of(&thumb))
    }

    /// The sizes, `None` where one is not finite.
    fn checked(thumb: f32, rail: f32, width: f32, outline: Option<f32>) -> Option<Self> {
        let outline = match outline {
            Some(w) => Some(scope::length(w)?),
            None => None,
        };
        Some(Self {
            thumb: scope::length(thumb)?,
            rail: scope::length(rail)?,
            width: scope::length(width)?,
            outline,
        })
    }
}

impl egui::Widget for Slider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let width = ui.spacing().slider_width;
        let Slider {
            value,
            range,
            label,
            enabled,
            backdrop,
        } = self;
        // Faded as one where disabled here, or under a disabled calling `Ui` whose fade it
        // takes back.
        let under = crate::fade::outer_fade(ui);
        let as_one = (!enabled && ui.is_enabled()) || under.is_some();
        let paint = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let theme = ui.ctx().theme();
            let (look, geometry) = match atlas.resolved_for(theme) {
                Some(t) => (Look::of(t), Geometry::of(&t.slider, width)?),
                None => {
                    let r = atlas.next_resolved_for(theme)?;
                    (Look::next_of(r)?, Geometry::next_of(r, width)?)
                }
            };
            let ground = as_one
                .then(|| backdrop.or_else(|| look.background.map(to_color32)))
                .flatten();
            Some((geometry, look, ground))
        });
        let Some((geometry, sl, ground)) = paint else {
            let mut slider = egui::Slider::new(value, range).show_value(false);
            if let Some(label) = label {
                slider = slider.text(label);
            }
            return ui.add_enabled(enabled, slider);
        };
        let add = |ui: &mut egui::Ui, fade: Option<Fade>| match label {
            Some(label) => {
                let row = ui.horizontal(|ui| {
                    let text = Some(label.text());
                    let slider = slider_ui(ui, value, &range, text, geometry, &sl, fade);
                    ui.label(label);
                    slider
                });
                row.inner.union(row.response)
            }
            None => slider_ui(ui, value, &range, None, geometry, &sl, fade),
        };
        match ground {
            // `scope::open_faded`'s fading arms, with the knob's outline read before the fade.
            Some(ground) => {
                ui.native_scope(Role::Slider, RoleVariant::Disabled, |ui| {
                    let outline = ui.visuals().widgets.inactive.fg_stroke;
                    match under {
                        Some(under) => crate::fade::fade_as_one_under(ui, ground, under),
                        None => crate::fade::fade_as_one(ui, ground),
                    }
                    let fade = Fade::of(ui, ground, outline);
                    add(ui, fade)
                })
                .inner
            }
            None => scope::open(ui, Role::Slider, enabled, |ui| add(ui, None)),
        }
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
    sl: &Look,
    fade: Option<Fade>,
) -> egui::Response {
    let old = *value;
    *value = clamp_to(*value, range);
    let size = egui::vec2(g.width, g.thumb.max(g.rail));
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
    let (mut decrement, mut increment): (usize, usize) = (0, 0);
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
            decrement = decrement.saturating_add(i.num_presses(egui::Key::ArrowLeft));
            increment = increment.saturating_add(i.num_presses(egui::Key::ArrowRight));
        });
    }
    ui.input(|i| {
        decrement = decrement.saturating_add(
            i.num_accesskit_action_requests(response.id, accesskit::Action::Decrement),
        );
        increment = increment.saturating_add(
            i.num_accesskit_action_requests(response.id, accesskit::Action::Increment),
        );
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

    let rail = egui::Rect::from_center_size(rect.center(), egui::vec2(rect.width(), g.rail));
    let centre = egui::pos2(position_of(current), rail.center().y);
    let mut trailing = rail;
    trailing.max.x = centre.x;
    let mut parts = Parts::default();
    parts.push("track", rail);
    parts.push("fill", trailing);
    parts.push(
        "thumb",
        egui::Rect::from_center_size(centre, egui::Vec2::splat(g.thumb)),
    );
    parts.store(ui.ctx(), response.id);
    if ui.is_rect_visible(rect) {
        let (track, fill, thumb) = if enabled {
            let thumb = match sl.thumb_hover_color {
                Some(layer) if response.hovered() => composite_over(layer, sl.thumb_color),
                _ => to_color32(sl.thumb_color),
            };
            (to_color32(sl.track_color), to_color32(sl.fill_color), thumb)
        } else {
            (
                to_color32(sl.disabled_track_color.unwrap_or(sl.track_color)),
                to_color32(sl.disabled_fill_color.unwrap_or(sl.fill_color)),
                to_color32(sl.disabled_thumb_color.unwrap_or(sl.thumb_color)),
            )
        };
        let radius = ui.visuals().widgets.inactive.corner_radius;
        // E8: the knob's own line where the theme states its width (0: none); else egui's, as
        // without it (the class doc)
        let on_knob = response.dragged()
            || response
                .hover_pos()
                .is_some_and(|p| p.distance(centre) <= knob_radius);
        let mut outline = match g.outline {
            None => ui.style().interact(&response).fg_stroke,
            Some(width) => {
                let rest = sl.thumb_border_color; // a disabled knob's too
                let colour = if enabled && on_knob {
                    sl.thumb_hover_border_color.or(rest) // hover wins over focus
                } else if enabled && response.has_focus() {
                    sl.thumb_focus_border_color.or(rest)
                } else {
                    rest
                };
                colour
                    .filter(|_| width > 0.0)
                    .map_or(egui::Stroke::NONE, |c| {
                        egui::Stroke::new(width, to_color32(c))
                    })
            }
        };
        // Faded as one: the rail over the ground, the fill over the rail, the knob and its
        // outline over the ground, where most of the knob lies (the class doc); a stated
        // outline from its own colour, egui's from the `Disabled` cell's.
        let (track, fill, thumb) = match fade {
            Some(f) => {
                if g.outline.is_none() {
                    outline.color = f.over(f.ground, f.outline.color);
                } else if !outline.is_empty() {
                    outline.color = f.over(f.ground, outline.color);
                }
                (
                    f.over(f.ground, track),
                    f.over(f.ground.blend(track), fill),
                    f.over(f.ground, thumb),
                )
            }
            None => (track, fill, thumb),
        };
        let painter = ui.painter();
        painter.rect_filled(rail, radius, track);
        painter.rect_filled(trailing, radius, fill);
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
