//! [`Spinner`]: the icon set's loading indicator, or an arc at the theme's stroke.

use std::f64::consts::TAU;
use std::sync::Arc;

use native_theme_egui::convert::{to_color32, u8_from_f32_saturating};
use native_theme_egui::egui;
use native_theme_egui::egui::emath::Numeric as _;
use native_theme_egui::icons::{self, IconKey};
use native_theme_egui::native_theme::icons::{FreedesktopLoader, load_icon_indicator};
use native_theme_egui::native_theme::theme::{AnimatedIcon, IconSet};
use native_theme_egui::{Role, ThemeAtlas};

use crate::scope;

/// The painted arc's widest sweep, egui's own (`egui/src/widgets/spinner.rs:49`).
const SWEEP_DEGREES: f64 = 240.0;
/// The painted arc's fewest and most points, egui's own (`egui/src/widgets/spinner.rs:46`).
const MIN_POINTS: u8 = 8;
const MAX_POINTS: u8 = 128;

/// A loading indicator `spinner.diameter` across.
///
/// **The icon set's indicator first.** Where the atlas's icon set
/// ([`ThemeAtlas::icon_set`]) has an animated indicator —
/// `native_theme::icons::load_icon_indicator`, and for a freedesktop set
/// `FreedesktopLoader::load_indicator` of the icon theme the atlas names for the scheme being
/// drawn ([`ThemeAtlas::icon_theme`]) — that indicator is drawn, at `spinner.diameter`,
/// tinted `spinner.fill_color` for the monochrome bundled sets (Material, Lucide), animated by
/// [`icons::animated_frame_index`] or [`icons::spin_angle`] and drawn still on its first frame
/// under reduced motion. It is an SVG or RGBA image: an application draws SVGs with the
/// image loaders it installs (`egui_extras::install_image_loaders`), as for every icon of the
/// connector.
///
/// **Tier P.** Where the set has none (`SfSymbols`, `SegoeIcons`, or its icon feature is
/// off), it paints an arc, because `egui::Spinner` exposes only `.size()` and `.color()`
/// (`egui/src/widgets/spinner.rs:25`, `:32`): its stroke is hardcoded
/// `Stroke::new(3.0, color)` (`:58`) and its radius inset by a literal `- 2.0` (`:45`), so
/// `spinner.stroke_width` cannot reach it. The arc is `spinner.diameter` across its outer
/// edge, in `spinner.fill_color` at `spinner.stroke_width`, the stroke centred on a path of
/// radius `0.5 · (diameter − stroke_width)`. It turns as egui's does, from `time · TAU`
/// (`:48`), and always sweeps egui's widest arc, 240° (`:49`): egui's `240° · sin(time)` shrinks
/// it to nothing twice a cycle, which a still frame catches as a dot. Under reduced motion it
/// is drawn still from angle 0.
///
/// `spinner.min_diameter` is not read: the spinner offers no size of its own for it to bound.
///
/// With no atlas installed, or a spinner size that is not finite, it adds `egui::Spinner`.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Clone, Copy, Debug, Default)]
pub struct Spinner {}

impl Spinner {
    /// A spinner.
    pub fn new() -> Self {
        Self {}
    }
}

/// What the spinner draws, read from the atlas before its scope opens.
struct Paint {
    diameter: f32,
    stroke_width: f32,
    colour: egui::Color32,
    set: IconSet,
    icon_theme: Option<String>,
    reduce_motion: bool,
}

impl egui::Widget for Spinner {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let paint = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let theme = ui.ctx().theme();
            let s = &atlas.resolved_for(theme).spinner;
            Some(Paint {
                diameter: scope::length(s.diameter)?,
                stroke_width: scope::length(s.stroke_width)?,
                colour: to_color32(s.fill_color),
                set: atlas.icon_set(),
                icon_theme: atlas.icon_theme(theme).map(str::to_string),
                reduce_motion: atlas.accessibility().reduce_motion,
            })
        });
        match paint {
            Some(paint) => scope::open(ui, Role::Spinner, true, |ui| spinner_ui(ui, &paint)),
            None => ui.add(egui::Spinner::new()),
        }
    }
}

fn spinner_ui(ui: &mut egui::Ui, paint: &Paint) -> egui::Response {
    let size = egui::Vec2::splat(paint.diameter);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::ProgressIndicator));
    if !ui.is_rect_visible(rect) {
        return response;
    }
    if !paint_indicator(ui, rect, paint) {
        paint_arc(ui, rect, paint);
    }
    response
}

/// The icon set's indicator for `set` and, for a freedesktop set, `icon_theme`, loaded once per
/// `Context` and kept in its data.
fn indicator(
    ctx: &egui::Context,
    set: IconSet,
    icon_theme: Option<&str>,
) -> Arc<Option<AnimatedIcon>> {
    let key = egui::Id::new(("native-theme-egui-widgets::spinner", set, icon_theme));
    if let Some(found) = ctx.data(|d| d.get_temp::<Arc<Option<AnimatedIcon>>>(key)) {
        return found;
    }
    // Outside every accessor closure: a freedesktop lookup reads the file system.
    let loaded = Arc::new(match set {
        IconSet::Freedesktop => FreedesktopLoader::load_indicator(icon_theme),
        other => load_icon_indicator(other),
    });
    ctx.data_mut(|d| d.insert_temp(key, Arc::clone(&loaded)));
    loaded
}

/// Draw the icon set's indicator into `rect`; `false` where the set has none or it cannot be
/// made an image.
fn paint_indicator(ui: &egui::Ui, rect: egui::Rect, paint: &Paint) -> bool {
    let ctx = ui.ctx();
    let loaded = indicator(ctx, paint.set, paint.icon_theme.as_deref());
    let Some(anim) = loaded.as_ref() else {
        return false;
    };
    let mut key = IconKey::name("indicator", paint.set).size(paint.diameter);
    if paint.set == IconSet::Freedesktop {
        if let Some(theme) = paint.icon_theme.as_deref() {
            key = key.icon_theme(theme);
        }
    } else if matches!(paint.set, IconSet::Material | IconSet::Lucide) {
        key = key.tint(paint.colour);
    }
    let frame = icons::animated_frame_index(ctx, anim, paint.reduce_motion)
        .and_then(|i| anim.frame_list()?.get(i))
        .unwrap_or_else(|| anim.first_frame());
    let Some(source) = icons::to_image_source(ctx, &key, frame) else {
        return false;
    };
    let mut image = egui::Image::new(source);
    if let Some(angle) = icons::spin_angle(ctx, anim, paint.reduce_motion) {
        image = image.rotate(angle, egui::Vec2::splat(0.5));
    }
    image.paint_at(ui, rect);
    true
}

/// egui's spinner arc (`egui/src/widgets/spinner.rs:38-59`) at the theme's diameter and stroke.
fn paint_arc(ui: &egui::Ui, rect: egui::Rect, paint: &Paint) {
    let radius = (0.5 * (paint.diameter - paint.stroke_width)).max(0.0);
    let points = u8_from_f32_saturating(radius).clamp(MIN_POINTS, MAX_POINTS);
    let sweep = SWEEP_DEGREES.to_radians();
    let start = if paint.reduce_motion {
        0.0
    } else {
        ui.ctx().request_repaint(); // animated, as egui's (`:39-40`)
        ui.input(|i| i.time) * TAU
    };
    let path: Vec<egui::Pos2> = (0..points)
        .map(|i| {
            let angle = start + sweep * f64::from(i) / f64::from(points);
            let (sin, cos) = angle.sin_cos();
            let centre = rect.center();
            egui::pos2(
                centre.x + radius * f32::from_f64(cos),
                centre.y + radius * f32::from_f64(sin),
            )
        })
        .collect();
    ui.painter().add(egui::Shape::line(
        path,
        egui::Stroke::new(paint.stroke_width, paint.colour),
    ));
}
