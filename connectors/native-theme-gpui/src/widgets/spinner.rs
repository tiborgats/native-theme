//! `Spinner` (spec §2.6): the icon set's own loading indicator, or, for a set
//! without one, an arc painted from `SpinnerTheme`, on gpui-base's headless
//! progress indicator.

use std::borrow::Cow;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, App, Bounds, ElementId, Global, Hsla, ImageSource,
    InteractiveElement as _, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled,
    Window, canvas, div, img, prelude::FluentBuilder as _, px, svg,
};
use gpui_base::Progress as BaseProgress;
use gpui_component::plot::shape::{Arc, ArcData};
use native_theme::color::Rgba;
use native_theme::icons::{FreedesktopLoader, colorize_monochrome_svg, load_icon_indicator};
use native_theme::theme::{
    AnimatedIcon, IconData, IconSet, ResolvedTheme, TransformAnimation, system_icon_set,
};

use super::{color, length, native};
use crate::icons::{to_image_source, with_spin_animation};

/// What a spinner paints, from `SpinnerTheme` (spec §2.6).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerLook {
    /// The indicator's size: `spinner.diameter`, and no less than
    /// `min_diameter`.
    pub diameter: Pixels,
    /// `spinner.stroke_width`: the arc's stroke.
    pub stroke: Pixels,
    /// `spinner.fill_color`: the arc, and the tint of a monochrome bundled
    /// set's indicator.
    pub color: Hsla,
}

impl SpinnerLook {
    /// The look of a spinner, or `None` when a length the theme gives is not
    /// finite.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme) -> Option<Self> {
        let s = &resolved.spinner;
        let diameter = length(s.diameter)?;
        let min = length(s.min_diameter)?;
        Some(Self {
            diameter: diameter.max(min),
            stroke: length(s.stroke_width)?,
            color: color(s.fill_color),
        })
    }

    /// The radius of the stroke's centre line: the model gives the diameter
    /// as the ring's outer size, and the stroke is centred on its path.
    #[must_use]
    pub fn path_radius(&self) -> Pixels {
        px((f32::from(self.diameter) - f32::from(self.stroke)) / 2.)
    }
}

/// The arc's turn: a turn a second, gpui-component's indeterminate period
/// (progress/progress_circle.rs:201) and egui's `Spinner`'s (egui 0.36.2
/// `src/widgets/spinner.rs:48`).
const ARC_PERIOD: Duration = Duration::from_secs(1);

/// The arc's sweep, as a fraction of the circle: 240°, the widest egui's
/// `Spinner` draws (egui 0.36.2 `src/widgets/spinner.rs:49`), which the iced
/// connector's arc keeps too. The arc keeps it while it turns: a sweep that
/// grows and shrinks passes through nothing, and a capture taken then shows
/// no spinner. The model states the arc's diameter, stroke and colour, not
/// its sweep.
const ARC_SWEEP: f32 = 240. / 360.;

/// The arc at `delta` (0–1) of its turn, as the fractions of the circle its
/// two ends have reached.
fn arc_at(delta: f32) -> (f32, f32) {
    (delta, delta + ARC_SWEEP)
}

/// The frame of `count` shown at `delta` (0–1) of the animation's cycle.
fn frame_at(delta: f32, count: usize) -> usize {
    // `count` is a frame count; the cast to f32 loses nothing an index can show.
    let ix = (delta.clamp(0., 1.) * count as f32).floor() as usize;
    ix.min(count.saturating_sub(1))
}

/// Whether an SVG sets a `color` of its own (a `color` property or
/// attribute): egui's `native_theme_egui::icons::sets_color`, so a
/// freedesktop icon is coloured as the egui connector colours it.
fn sets_color(svg: &str) -> bool {
    let bytes = svg.as_bytes();
    svg.match_indices("color").any(|(at, _)| {
        let preceded = at
            .checked_sub(1)
            .and_then(|i| bytes.get(i))
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'_');
        if preceded {
            return false;
        }
        at.checked_add("color".len())
            .and_then(|i| bytes.get(i..))
            .and_then(|rest| rest.iter().find(|b| !b.is_ascii_whitespace()))
            .is_some_and(|b| *b == b':' || *b == b'=')
    })
}

/// An indicator frame's bytes in `colour`: a monochrome bundled set's
/// coloured through `colorize_monochrome_svg`; a freedesktop icon's
/// `currentColor` replaced where the document sets no `color` of its own
/// (Breeze's stylesheet sets one per scheme); any other as it is.
fn paint_svg(set: IconSet, bytes: &[u8], colour: Rgba) -> Vec<u8> {
    match set {
        IconSet::Material | IconSet::Lucide => colorize_monochrome_svg(bytes, colour),
        IconSet::Freedesktop => match std::str::from_utf8(bytes) {
            Ok(text) if text.contains("currentColor") && !sets_color(text) => text
                .replace(
                    "currentColor",
                    &format!("#{:02x}{:02x}{:02x}", colour.r, colour.g, colour.b),
                )
                .into_bytes(),
            _ => bytes.to_vec(),
        },
        _ => bytes.to_vec(),
    }
}

/// The colour an indicator of `set` is drawn in: `fill`
/// (`spinner.fill_color`) for the monochrome bundled sets, `text`
/// (`defaults.text_color`, the colour `currentColor` stands for) for the
/// others.
fn indicator_colour(set: IconSet, fill: Rgba, text: Rgba) -> Rgba {
    match set {
        IconSet::Material | IconSet::Lucide => fill,
        _ => text,
    }
}

/// An icon set's indicator, ready to draw.
enum Indicator {
    /// Frames shown in turn, `frame_ms` each.
    Frames {
        frames: Rc<[ImageSource]>,
        frame_ms: u32,
    },
    /// One SVG turned a full turn every `duration_ms`. gpui turns an SVG
    /// element but not an image (gpui-pre elements/svg.rs,
    /// `Svg::with_transformation`), and an SVG element paints every shape in
    /// its text colour.
    Spin { svg: Rc<[u8]>, duration_ms: u32 },
}

/// Which indicator, drawn how.
#[derive(Clone, PartialEq, Eq, Hash)]
struct IndicatorKey {
    set: IconSet,
    icon_theme: Option<SharedString>,
    colour: [u8; 4],
    /// The raster size, in device pixels.
    size: u32,
}

/// The indicators loaded so far, one per key: loading reads the file system
/// for a freedesktop set and rasterizes every frame, so it is done once.
/// Each key's images stay for the life of the application; a key is a set,
/// an icon theme, a colour and a size, so there are as many as the
/// combinations drawn.
#[derive(Default)]
struct Indicators(HashMap<IndicatorKey, Option<Rc<Indicator>>>);

impl Global for Indicators {}

/// Load `key`'s indicator: `None` where its set has none, or it cannot be
/// made an image.
fn load(key: &IndicatorKey, colour: Rgba) -> Option<Indicator> {
    let anim = match key.set {
        IconSet::Freedesktop => FreedesktopLoader::load_indicator(key.icon_theme.as_deref()),
        other => load_icon_indicator(other),
    }?;
    match &anim {
        AnimatedIcon::Frames(data) => {
            let frames: Option<Vec<ImageSource>> = data
                .frames()
                .iter()
                .map(|frame| match frame {
                    IconData::Svg(bytes) => to_image_source(
                        &IconData::Svg(Cow::Owned(paint_svg(key.set, bytes, colour))),
                        None,
                        Some(key.size),
                    ),
                    other => to_image_source(other, None, Some(key.size)),
                })
                .collect();
            Some(Indicator::Frames {
                frames: frames?.into(),
                frame_ms: data.frame_duration_ms().get(),
            })
        }
        AnimatedIcon::Transform(data) => match (data.icon(), data.animation()) {
            (IconData::Svg(bytes), TransformAnimation::Spin { duration_ms, .. }) => {
                Some(Indicator::Spin {
                    svg: paint_svg(key.set, bytes, colour).into(),
                    duration_ms: duration_ms.get(),
                })
            }
            _ => None,
        },
        _ => None,
    }
}

/// `key`'s indicator, loaded once per application.
fn indicator(key: IndicatorKey, colour: Rgba, cx: &mut App) -> Option<Rc<Indicator>> {
    if let Some(found) = cx.try_global::<Indicators>().and_then(|c| c.0.get(&key)) {
        return found.clone();
    }
    let loaded = load(&key, colour).map(Rc::new);
    cx.default_global::<Indicators>()
        .0
        .insert(key, loaded.clone());
    loaded
}

/// The arc of `look` from `start` to `end` (fractions of the circle).
fn ring(look: SpinnerLook, start: f32, end: f32) -> impl IntoElement {
    div()
        .absolute()
        .size_full()
        .debug_selector(|| "native-spinner-arc".into())
        .child(
            canvas(
                move |bounds: Bounds<Pixels>, _, _| bounds,
                move |_, bounds, window, _| {
                    let outer = f32::from(look.diameter) / 2.;
                    let inner = outer - f32::from(look.stroke);
                    Arc::new()
                        .inner_radius(inner.max(0.))
                        .outer_radius(outer)
                        .paint(
                            &ArcData {
                                data: &(),
                                index: 0,
                                value: end - start,
                                start_angle: start * TAU,
                                end_angle: end * TAU,
                                pad_angle: 0.,
                            },
                            look.color,
                            None,
                            None,
                            &bounds,
                            window,
                        );
                },
            )
            .size_full(),
        )
}

/// Frame `ix` of `frames` (the first where there is no such frame), filling
/// the spinner.
fn frame(frames: &[ImageSource], ix: usize) -> impl IntoElement {
    div()
        .absolute()
        .size_full()
        .debug_selector(|| "native-spinner-indicator".into())
        .children(
            frames
                .get(ix)
                .or(frames.first())
                .map(|source| img(source.clone()).size_full()),
        )
}

/// Which icon set's indicator a spinner draws.
#[derive(Clone)]
enum IconChoice {
    /// `native_theme::theme::system_icon_set()`, its icon theme detected.
    System,
    /// A set, or none; for a freedesktop set, the icon theme.
    Set(Option<IconSet>, Option<SharedString>),
}

/// An indeterminate spinner `spinner.diameter` across, on gpui-base's
/// headless `Progress` (a progress indicator with no value).
///
/// **The icon set's indicator first.** Where the application's icon set has
/// an animated indicator -- `native_theme::icons::load_icon_indicator`, and
/// for a freedesktop set `FreedesktopLoader::load_indicator` of the icon
/// theme the icons come from (Breeze's is `process-working`) -- that
/// indicator is drawn, at `spinner.diameter`: the monochrome bundled sets
/// (Material, Lucide) tinted `spinner.fill_color`, a freedesktop icon in its
/// own colours (its `currentColor`, where it sets no colour of its own,
/// `defaults.text_color`). Frames play at the indicator's own frame
/// duration, a spinning icon turns at its own speed, and under reduced
/// motion the first frame stands still. Name the set with
/// [`icon_set`](Self::icon_set) and, for a freedesktop set,
/// [`icon_theme`](Self::icon_theme); without them it is
/// `native_theme::theme::system_icon_set()` with the system's icon theme.
///
/// **An arc for a set without one** (`SfSymbols`, `SegoeIcons`, an icon
/// feature that is off, or `icon_set(None)`): `spinner.fill_color` at
/// `spinner.stroke_width`, its outer edge `spinner.diameter` across, sweeping
/// 240° and turning a turn a second; under reduced motion it stands still.
///
/// The widget is drawn the same with the `native-theme-egui-widgets` and
/// iced connectors' spinners. Without a native theme it renders
/// gpui-component's `Spinner`.
pub struct Spinner {
    id: ElementId,
    label: Option<SharedString>,
    icons: IconChoice,
}

impl Spinner {
    /// A new spinner.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icons: IconChoice::System,
        }
    }

    /// The name a screen reader announces.
    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// The icon set whose indicator is drawn: the set the application's
    /// icons come from. `None` where they come from no native-theme set,
    /// which draws the arc.
    #[must_use]
    pub fn icon_set(mut self, set: impl Into<Option<IconSet>>) -> Self {
        let theme = match self.icons {
            IconChoice::Set(_, theme) => theme,
            IconChoice::System => None,
        };
        self.icons = IconChoice::Set(set.into(), theme);
        self
    }

    /// For a freedesktop set, the icon theme the indicator comes from;
    /// `None` is the system's. Without [`icon_set`](Self::icon_set) the set
    /// is the system's.
    #[must_use]
    pub fn icon_theme(mut self, theme: impl Into<Option<SharedString>>) -> Self {
        let set = match self.icons {
            IconChoice::Set(set, _) => set,
            IconChoice::System => Some(system_icon_set()),
        };
        self.icons = IconChoice::Set(set, theme.into());
        self
    }
}

impl RenderOnce for Spinner {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some((look, fill, text)) = native(cx).and_then(|n| {
            let r = n.resolved;
            Some((
                SpinnerLook::of(r)?,
                r.spinner.fill_color,
                r.defaults.text_color,
            ))
        }) else {
            return div()
                .child(gpui_component::spinner::Spinner::new())
                .into_any_element();
        };
        let reduce = cx.reduce_motion();
        let (set, icon_theme) = match self.icons {
            IconChoice::System => (Some(system_icon_set()), None),
            IconChoice::Set(set, theme) => (set, theme),
        };
        let found = set.and_then(|set| {
            let colour = indicator_colour(set, fill, text);
            // Rasterized for the window's pixels: the size is finite and
            // non-negative (`SpinnerLook::of`), and the cast saturates.
            let size = (f32::from(look.diameter) * window.scale_factor())
                .ceil()
                .max(1.) as u32;
            let key = IndicatorKey {
                set,
                icon_theme: icon_theme.filter(|_| set == IconSet::Freedesktop),
                colour: [colour.r, colour.g, colour.b, colour.a],
                size,
            };
            Some((indicator(key, colour, cx)?, colour))
        });
        let root = BaseProgress::new(self.id)
            .indeterminate(true)
            .when_some(self.label, |spinner, label| {
                spinner.accessibility_label(label)
            })
            .relative()
            .flex_none()
            .size(look.diameter)
            .debug_selector(|| "native-spinner".into());
        match found.as_ref().map(|(ind, colour)| (ind.as_ref(), *colour)) {
            Some((Indicator::Frames { frames, frame_ms }, _)) => {
                let count = frames.len();
                if reduce || count < 2 {
                    return root.child(frame(frames, 0)).into_any_element();
                }
                let frames = Rc::clone(frames);
                let cycle = u64::from(*frame_ms).saturating_mul(count as u64);
                root.with_animation(
                    "native-theme-spinner-frames",
                    Animation::new(Duration::from_millis(cycle)).repeat(),
                    move |spinner, delta| spinner.child(frame(&frames, frame_at(delta, count))),
                )
                .into_any_element()
            }
            Some((
                Indicator::Spin {
                    svg: bytes,
                    duration_ms,
                },
                colour,
            )) => {
                let icon = svg().data(bytes).size_full().text_color(color(colour));
                let holder = div()
                    .absolute()
                    .size_full()
                    .debug_selector(|| "native-spinner-indicator".into());
                if reduce {
                    return root.child(holder.child(icon)).into_any_element();
                }
                root.child(holder.child(with_spin_animation(
                    icon,
                    "native-theme-spinner-spin",
                    *duration_ms,
                )))
                .into_any_element()
            }
            None => {
                if reduce {
                    let (start, end) = arc_at(0.);
                    return root.child(ring(look, start, end)).into_any_element();
                }
                root.with_animation(
                    "native-theme-spinner",
                    Animation::new(ARC_PERIOD).repeat(),
                    move |spinner, delta| {
                        let (start, end) = arc_at(delta);
                        spinner.child(ring(look, start, end))
                    },
                )
                .into_any_element()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arc_keeps_its_sweep_at_every_moment_of_its_turn() {
        for step in 0..=100u8 {
            let (start, end) = arc_at(f32::from(step) / 100.);
            assert!(
                ((end - start) - 240. / 360.).abs() < 1e-6,
                "at {step}%: {start}..{end}"
            );
        }
    }

    #[test]
    fn every_frame_is_shown_in_turn() {
        assert_eq!(frame_at(0., 24), 0);
        assert_eq!(frame_at(0.5, 24), 12);
        assert_eq!(frame_at(1., 24), 23);
        assert_eq!(frame_at(-1., 24), 0);
        assert_eq!(frame_at(f32::NAN, 24), 0);
        assert_eq!(frame_at(0.5, 0), 0);
    }

    #[test]
    fn a_freedesktop_icon_keeps_its_own_colour() {
        let red = Rgba::rgb(255, 0, 0);
        let breeze = br#"<svg><style>.ColorScheme-Text { color:#232629; }</style><path class="ColorScheme-Text" fill="currentColor"/></svg>"#;
        assert_eq!(
            paint_svg(IconSet::Freedesktop, breeze, red),
            breeze.to_vec()
        );
        let plain = br#"<svg><path fill="currentColor"/></svg>"#;
        assert_eq!(
            paint_svg(IconSet::Freedesktop, plain, red),
            br##"<svg><path fill="#ff0000"/></svg>"##.to_vec()
        );
    }

    #[test]
    fn a_set_without_an_indicator_has_none() {
        let key = |set| IndicatorKey {
            set,
            icon_theme: None,
            colour: [0, 0, 0, 255],
            size: 16,
        };
        let black = Rgba::rgb(0, 0, 0);
        assert!(load(&key(IconSet::SfSymbols), black).is_none());
        assert!(load(&key(IconSet::SegoeIcons), black).is_none());
        #[cfg(feature = "lucide-icons")]
        assert!(matches!(
            load(&key(IconSet::Lucide), black),
            Some(Indicator::Frames { .. })
        ));
        #[cfg(feature = "material-icons")]
        assert!(matches!(
            load(&key(IconSet::Material), black),
            Some(Indicator::Frames { .. })
        ));
    }
}
