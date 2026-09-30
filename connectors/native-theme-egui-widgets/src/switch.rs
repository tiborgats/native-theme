//! [`Switch`]: a two-state switch, a thumb on a track.

use native_theme_egui::convert::{composite_over, to_color32, to_corner_radius};
use native_theme_egui::egui::widget_style::Classes;
use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::native_theme::color::Rgba;
use native_theme_egui::native_theme::theme::ResolvedSwitchTheme;
use native_theme_egui::{Role, ThemeAtlas, register_focus_shape};

use crate::parts::Parts;
use crate::scope;

/// A two-state switch: `switch.*`'s track and thumb, the label after it.
///
/// **Tier P.** `egui/src/widgets/` has no switch or toggle module (`button`, `checkbox`,
/// `color_picker`, `drag_value`, `hyperlink`, `image`, `label`, `progress_bar`,
/// `radio_button`, `separator`, `slider`, `spinner`, `text_edit`), and the connector's
/// substitute, `Button::new(..).selected(checked)` (`egui/src/widgets/button.rs:270`),
/// shows the track colours as a button's fill and no thumb.
///
/// Built as egui's own `Checkbox` is (`egui/src/widgets/checkbox.rs:92-104`): an
/// `AtomLayout` whose first atom is the track, the label after it at egui's own
/// `Spacing::icon_spacing`, sensing `Sense::click()`; a click flips the value and marks the
/// response changed.
///
/// * The track is `switch.track_width` × `switch.track_height` at `switch.track_radius`, in
///   `checked_background` or `unchecked_background`, with `hover_checked_background` or
///   `hover_unchecked_background` composited over it while hovered.
/// * The thumb is a `switch.thumb_diameter` circle in `thumb_background`, centred on the
///   track's axis and inset `0.5 · (track_height − thumb_diameter)` from its ends; a thumb
///   larger than its track overhangs it, as those numbers state. While off it is
///   `unchecked_thumb_diameter` across in `unchecked_thumb_background` where the theme
///   states them (Material's 16 in `outline`, against 24 in `on-primary`). It moves, and
///   grows or shrinks, over the scope's `Style::animation_time`, which the connector sets to
///   `0.0` under reduced motion.
/// * The label is text of no widget font of its own, so in `defaults.font.color`: the switch
///   role's cell carries no text colour of the switch's.
/// * `.enabled(false)` paints `disabled_checked_background`, `disabled_unchecked_background`
///   and `disabled_thumb_color` (a `None` copies the colour it stands for) and the label in
///   `defaults.disabled_text_color`, faded by `switch.disabled_opacity` as one widget over
///   the backdrop (the crate's *Disabled*): the thumb on the track, the pair over
///   [`Switch::backdrop`]; under `ui.add_enabled(false, ..)` the fade is the calling `Ui`'s
///   `disabled_alpha`.
///
/// The focus ring surrounds the track (`register_focus_shape`). AccessKit sees a
/// `Role::Switch`, toggled as the value is.
///
/// With no atlas installed, or a switch size that is not finite, it adds `egui::Checkbox`,
/// egui's own two-state control.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Switch<'a> {
    on: &'a mut bool,
    label: Option<egui::WidgetText>,
    enabled: bool,
    backdrop: Option<egui::Color32>,
}

impl<'a> Switch<'a> {
    /// A switch showing, and flipping, `on`.
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            label: None,
            enabled: true,
            backdrop: None,
        }
    }

    /// The text after the track, which also names the switch for assistive technology.
    pub fn label(mut self, text: impl Into<egui::WidgetText>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// `false` shows the platform's disabled switch, its disabled colours faded by its
    /// `disabled_opacity` as one widget over its backdrop ([`Self::backdrop`]), and takes no
    /// input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The colour under the switch, which a disabled one is faded over as one widget: the thumb
    /// on its track, the pair over the backdrop ([`crate::fade::fade_as_one`]); the window's,
    /// `defaults.background_color`, where not given.
    pub fn backdrop(mut self, backdrop: egui::Color32) -> Self {
        self.backdrop = Some(backdrop);
        self
    }
}

/// The track and thumb sizes, every one finite.
#[derive(Clone, Copy)]
struct Geometry {
    track: egui::Vec2,
    radius: f32,
    thumb: f32,
    /// The thumb while off: `unchecked_thumb_diameter`, or `thumb`.
    unchecked_thumb: f32,
}

impl Geometry {
    fn of(sw: &ResolvedSwitchTheme) -> Option<Self> {
        let thumb = scope::length(sw.thumb_diameter)?;
        let unchecked_thumb = match sw.unchecked_thumb_diameter {
            Some(d) => scope::length(d)?,
            None => thumb,
        };
        Some(Self {
            track: egui::vec2(
                scope::length(sw.track_width)?,
                scope::length(sw.track_height)?,
            ),
            radius: scope::length(sw.track_radius)?,
            thumb,
            unchecked_thumb,
        })
    }
}

/// What a switch paints with, read from the atlas before its scope opens.
struct Paint {
    geometry: Geometry,
    sw: ResolvedSwitchTheme,
    text: Rgba,
    disabled_text: Rgba,
    /// The colour a switch disabled by its own `.enabled(false)` is faded over as one widget;
    /// `None` where it is enabled, or its calling `Ui` faded it already.
    ground: Option<egui::Color32>,
}

impl egui::Widget for Switch<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Switch {
            on,
            label,
            enabled,
            backdrop,
        } = self;
        // Faded here, as one widget, only where the calling `Ui` has not faded it already.
        let own_fade = !enabled && ui.is_enabled();
        let paint = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let t = atlas.resolved_for(ui.ctx().theme());
            Geometry::of(&t.switch).map(|geometry| Paint {
                geometry,
                sw: t.switch.clone(),
                text: t.defaults.font.color,
                disabled_text: t.defaults.disabled_text_color,
                ground: own_fade
                    .then(|| backdrop.unwrap_or_else(|| to_color32(t.defaults.background_color))),
            })
        });
        match paint {
            Some(paint) => {
                scope::open_faded(ui, Role::Switch, false, enabled, paint.ground, |ui| {
                    switch_ui(ui, on, label, &paint)
                })
            }
            None => {
                let checkbox = match label {
                    Some(label) => egui::Checkbox::new(on, label),
                    None => egui::Checkbox::without_text(on),
                };
                ui.add_enabled(enabled, checkbox)
            }
        }
    }
}

fn switch_ui(
    ui: &mut egui::Ui,
    on: &mut bool,
    label: Option<egui::WidgetText>,
    paint: &Paint,
) -> egui::Response {
    let Paint {
        geometry: g,
        sw,
        text: label_text,
        disabled_text,
        ground,
    } = paint;
    // Faded as one over `ground` (the painter's opacity kept), or not at all here: where the
    // platform dims by colour alone (`disabled_alpha` 1) the colours are as stated.
    let alpha = ui.visuals().disabled_alpha();
    let ground = &ground.filter(|_| alpha < 1.0);
    let as_one = |under: egui::Color32, colour: egui::Color32| match ground {
        Some(ground) => crate::fade::faded(*ground, under.blend(colour), alpha),
        None => colour,
    };
    // The state of the previous pass, as `Checkbox` reads it (`egui/src/widgets/checkbox.rs:72-74`).
    let id = ui.next_auto_id();
    let state = ui
        .ctx()
        .read_response(id)
        .map(|r| r.widget_state())
        .unwrap_or_default();
    let style = ui.style().checkbox_style(&Classes::default(), state);

    // The atom holds the track and a thumb that overhangs it.
    let largest = g.thumb.max(g.unchecked_thumb);
    let overhang = (largest - g.track.y).max(0.0);
    let atom = egui::vec2(g.track.x + overhang, g.track.y.max(largest));
    let mut min_size = egui::Vec2::splat(ui.spacing().interact_size.y);
    min_size.y = min_size.y.max(atom.y);
    let text = label.as_ref().map(|l| l.text().to_string());
    // The label as the layout sizes it (`egui/src/atomics/atom_kind.rs:134-135`), for `Parts`.
    let label_size = label.as_ref().map(|l| {
        l.clone()
            .into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::FontSelection::Default,
            )
            .size()
    });
    let mut atoms = match label {
        Some(label) => egui::Atoms::new(label),
        None => egui::Atoms::new(()),
    };
    let rect_id = egui::Id::new("native-theme-egui-widgets::switch");
    atoms.push_left(egui::Atom::custom(rect_id, atom));

    let mut prepared = egui::AtomLayout::new(atoms)
        .sense(egui::Sense::click())
        .min_size(min_size)
        .frame(style.frame)
        .allocate(ui);
    if prepared.response.clicked() {
        *on = !*on;
        prepared.response.mark_changed();
    }
    let enabled = ui.is_enabled();
    let checked = *on;
    prepared.response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            enabled,
            checked,
            text.as_deref().unwrap_or_default(),
        )
    });
    ui.ctx()
        .accesskit_node_builder(prepared.response.id, |node| {
            node.set_role(accesskit::Role::Switch);
            node.set_toggled(if checked {
                accesskit::Toggled::True
            } else {
                accesskit::Toggled::False
            });
        });

    let hovered = enabled && prepared.response.hovered();
    let time = ui.style().animation_time;
    let position = ui
        .ctx()
        .animate_bool_with_time(prepared.response.id, checked, time);
    let text = to_color32(if enabled { *label_text } else { *disabled_text });
    prepared.fallback_text_color = ground.map_or(text, |ground| as_one(ground, text));
    let laid = prepared.paint(ui);
    let Some(atom_rect) = laid.rect(rect_id) else {
        return laid.response;
    };
    // From the atom's left edge, as `Checkbox` places its box (`egui/src/widgets/checkbox.rs:129-132`).
    let track = egui::Rect::from_center_size(
        egui::pos2(atom_rect.left() + 0.5 * atom.x, atom_rect.center().y),
        g.track,
    );
    let radius = to_corner_radius(egui::CornerRadius::ZERO, g.radius);
    register_focus_shape(ui.ctx(), laid.response.id, track, radius);

    let (fill, hover) = match (checked, enabled) {
        (true, true) => (sw.checked_background, sw.hover_checked_background),
        (false, true) => (sw.unchecked_background, sw.hover_unchecked_background),
        (true, false) => (
            sw.disabled_checked_background
                .unwrap_or(sw.checked_background),
            None,
        ),
        (false, false) => (
            sw.disabled_unchecked_background
                .unwrap_or(sw.unchecked_background),
            None,
        ),
    };
    let fill = match hover {
        Some(layer) if hovered => composite_over(layer, fill),
        _ => to_color32(fill),
    };
    let rest_thumb = if checked {
        sw.thumb_background
    } else {
        sw.unchecked_thumb_background.unwrap_or(sw.thumb_background)
    };
    let thumb = if enabled {
        rest_thumb
    } else {
        sw.disabled_thumb_color.unwrap_or(rest_thumb)
    };
    // The thumb grows from its off diameter to its on one as it travels.
    let diameter = egui::lerp(g.unchecked_thumb..=g.thumb, position);
    // The thumb's centre travels between the two ends of the track's axis, each half the
    // track's height in from its end: the thumb inset `0.5 · (track_height − thumb_diameter)`.
    let half = 0.5 * g.track.y;
    let ends = (track.left() + half)..=(track.right() - half);
    let centre = egui::pos2(egui::lerp(ends, position), track.center().y);
    // Faded as one: the track over the ground, the thumb over the track.
    let (fill, thumb) = match ground {
        Some(ground) => {
            let on_ground = ground.blend(fill);
            (as_one(*ground, fill), as_one(on_ground, to_color32(thumb)))
        }
        None => (fill, to_color32(thumb)),
    };
    let painter = ui.painter();
    painter.rect_filled(track, radius, fill);
    painter.circle_filled(centre, 0.5 * diameter, thumb);
    let mut parts = Parts::default();
    parts.push("track", track);
    parts.push(
        "thumb",
        egui::Rect::from_center_size(centre, egui::Vec2::splat(diameter)),
    );
    if let Some(size) = label_size {
        parts.push(
            "label",
            crate::parts::text_after(atom_rect, ui.spacing().icon_spacing, size),
        );
    }
    parts.store(ui.ctx(), laid.response.id);
    laid.response
}
