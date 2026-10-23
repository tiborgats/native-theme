//! [`Spinner`]: the icon set's loading indicator, or an arc at the theme's stroke.

use std::collections::HashMap;
use std::f64::consts::TAU;
use std::sync::Arc;

use native_theme_egui::convert::{to_color32, u8_from_f32_saturating};
use native_theme_egui::egui;
use native_theme_egui::egui::emath::Numeric as _;
use native_theme_egui::egui::mutex::Mutex;
use native_theme_egui::icons::{self, IconKey};
use native_theme_egui::native_theme::color::Rgba;
use native_theme_egui::native_theme::icons::{
    AnimatedIcon, FreedesktopLoader, load_icon_indicator,
};
use native_theme_egui::native_theme::theme::IconSet;
use native_theme_egui::{ResolvedTheme, Role, ThemeAtlas};

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
/// tinted `spinner.fill_color` for the monochrome bundled sets (Material, Lucide) and, for a
/// freedesktop set, with its `currentColor` filled in with `defaults.text_color` where the
/// document sets no `color` of its own (`native_theme::icons::fill_current_color`), animated by
/// [`icons::animated_frame_index`] or [`icons::spin_angle`] and drawn still on its first frame
/// under reduced motion. It is an SVG or RGBA image: an application draws SVGs with the
/// image loaders it installs (`egui_extras::install_image_loaders`), as for every icon of the
/// connector.
///
/// **Tier P.** Where the set has none (`SfSymbols`, `SegoeFluent`, or its icon feature is
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

/// The colours a spinner reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `spinner.fill_color`: the arc, and a Material or Lucide indicator's tint.
    pub(crate) fill_color: Rgba,
    /// `defaults.text_color`: a freedesktop indicator's `currentColor`.
    pub(crate) text_color: Rgba,
}

impl Look {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        Self {
            fill_color: t.spinner.fill_color,
            text_color: t.defaults.text_color,
        }
    }
}

/// What the spinner draws, read from the atlas before its scope opens.
struct Paint {
    diameter: f32,
    stroke_width: f32,
    colour: egui::Color32,
    /// A freedesktop indicator's `currentColor`.
    current_colour: egui::Color32,
    set: IconSet,
    icon_theme: Option<String>,
    reduce_motion: bool,
}

impl egui::Widget for Spinner {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let paint = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let theme = ui.ctx().theme();
            let t = atlas.resolved_for(theme);
            let s = &t.spinner;
            let look = Look::of(t);
            Some(Paint {
                diameter: scope::length(s.diameter)?,
                stroke_width: scope::length(s.stroke_width)?,
                colour: to_color32(look.fill_color),
                current_colour: to_color32(look.text_color),
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

/// An icon set's indicator, and the image source of each of its frames drawn so far under one
/// key and scheme: `icons::to_image_source` copies, colours and hashes a frame's SVG and
/// formats its URI on every call, and a spinner draws a frame in every pass.
struct Indicator {
    icon: Option<AnimatedIcon>,
    sources: Mutex<Sources>,
}

/// The image sources of an indicator's frames, by frame index, for the key and scheme they
/// were made under: one key at a time, as every spinner of a `Context` draws the indicator of
/// its entry under the one key the installed atlas and egui's scheme give in a pass (set, icon
/// theme, `spinner.diameter`, `spinner.fill_color`). A new key, from a scheme switch or another
/// atlas, drops the frames kept for the old one, and its frames are made once each.
#[derive(Default)]
struct Sources {
    made_for: Option<(IconKey, egui::Theme)>,
    frames: HashMap<usize, egui::ImageSource<'static>>,
}

impl Sources {
    /// The source of frame `index` made under `key` in `scheme`, while egui still holds its
    /// bytes: `icons::forget_icons` (the install of another atlas, `ThemeAtlas::clear`) forgets
    /// them, and a source egui loads again then is one the connector no longer registers.
    fn get(
        &self,
        ctx: &egui::Context,
        key: &IconKey,
        scheme: egui::Theme,
        index: usize,
    ) -> Option<egui::ImageSource<'static>> {
        let (made_key, made_scheme) = self.made_for.as_ref()?;
        if made_key != key || *made_scheme != scheme {
            return None;
        }
        let source = self.frames.get(&index)?;
        if let egui::ImageSource::Bytes { uri, .. } = source
            && ctx.try_load_bytes(uri).is_err()
        {
            return None;
        }
        Some(source.clone())
    }

    /// Keep `source` as frame `index` made under `key` in `scheme`, dropping the sources made
    /// under another. A raster frame's `ImageSource::Texture` is not kept: `forget_icons` frees
    /// the texture it names, and `to_image_source` reuses its stored handle anyway.
    fn put(
        &mut self,
        key: &IconKey,
        scheme: egui::Theme,
        index: usize,
        source: egui::ImageSource<'static>,
    ) {
        if !matches!(source, egui::ImageSource::Bytes { .. }) {
            return;
        }
        let same = self
            .made_for
            .as_ref()
            .is_some_and(|(k, s)| k == key && *s == scheme);
        if !same {
            self.made_for = Some((key.clone(), scheme));
            self.frames.clear();
        }
        self.frames.insert(index, source);
    }
}

/// The icon set's indicator for `set` and, for a freedesktop set, `icon_theme`, loaded once per
/// `Context` and kept in its data.
fn indicator(ctx: &egui::Context, set: IconSet, icon_theme: Option<&str>) -> Arc<Indicator> {
    let key = egui::Id::new(("native-theme-egui-widgets::spinner", set, icon_theme));
    if let Some(found) = ctx.data(|d| d.get_temp::<Arc<Indicator>>(key)) {
        return found;
    }
    // Outside every accessor closure: a freedesktop lookup reads the file system.
    let loaded = Arc::new(Indicator {
        icon: match set {
            IconSet::Freedesktop => FreedesktopLoader::load_indicator(icon_theme),
            other => load_icon_indicator(other, None),
        },
        sources: Mutex::default(),
    });
    ctx.data_mut(|d| d.insert_temp(key, Arc::clone(&loaded)));
    loaded
}

/// Draw the icon set's indicator into `rect`; `false` where the set has none or it cannot be
/// made an image.
fn paint_indicator(ui: &egui::Ui, rect: egui::Rect, paint: &Paint) -> bool {
    let ctx = ui.ctx();
    let loaded = indicator(ctx, paint.set, paint.icon_theme.as_deref());
    let Some(anim) = loaded.icon.as_ref() else {
        return false;
    };
    let mut key = IconKey::name("indicator", paint.set).size(paint.diameter);
    if paint.set == IconSet::Freedesktop {
        if let Some(theme) = paint.icon_theme.as_deref() {
            key = key.icon_theme(theme);
        }
        key = key.tint(paint.current_colour);
    } else if matches!(paint.set, IconSet::Material | IconSet::Lucide) {
        key = key.tint(paint.colour);
    }
    // Frame 0 is the first frame (`FrameList::first`), and a turning indicator's one frame.
    let (index, frame) = icons::animated_frame_index(ctx, anim, paint.reduce_motion)
        .and_then(|i| Some((i, anim.frame_list()?.get(i)?)))
        .unwrap_or_else(|| (0, anim.first_frame()));
    // The scheme keys the cached frame; a freedesktop frame's `currentColor` is the key's tint.
    let scheme = ctx.theme();
    let cached = loaded.sources.lock().get(ctx, &key, scheme, index);
    let Some(source) = cached.or_else(|| {
        // Made with the lock released, as `get` released it: no other draw waits on the copy,
        // colouring and hash. The lock is taken before the `Context`'s, never inside it: `get`
        // asks egui's bytes loaders with it held, and nothing here takes it from inside an
        // accessor.
        let made = icons::to_image_source(ctx, &key, frame)?;
        loaded.sources.lock().put(&key, scheme, index, made.clone());
        Some(made)
    }) else {
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

/// The bytes of every frame source the indicator entry of `set` (with no icon theme) keeps.
#[cfg(all(
    test,
    any(
        feature = "material-icons",
        all(target_os = "linux", feature = "system-icons")
    )
))]
pub(crate) fn kept_frames(ctx: &egui::Context, set: IconSet) -> Vec<egui::load::Bytes> {
    indicator(ctx, set, None)
        .sources
        .lock()
        .frames
        .values()
        .filter_map(|source| match source {
            egui::ImageSource::Bytes { bytes, .. } => Some(bytes.clone()),
            _ => None,
        })
        .collect()
}
