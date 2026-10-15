//! `Spinner` (spec §2.6): the icon set's own loading indicator, or, for a set
//! without one, an arc painted from `SpinnerTheme`, on gpui-base's headless
//! progress indicator.

use std::collections::HashMap;
use std::f32::consts::TAU;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, App, Bounds, ElementId, Global, Hsla, ImageSource,
    InteractiveElement as _, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled,
    Window, canvas, div, img, prelude::FluentBuilder as _, px, svg,
};
use gpui_base::Progress as BaseProgress;
use gpui_component::plot::shape::{Arc, ArcData};
use gpui_component::{Sizable, Size};
use native_theme::color::Rgba;
use native_theme::detect::{system_icon_set, system_icon_theme};
use native_theme::icons::{AnimatedIcon, IconData, TransformAnimation, load_icon_indicator};
use native_theme::theme::{IconSet, ResolvedTheme};

use super::{color, length};
use crate::icons::{MAX_ICON_SIZE, animated_frames_to_image_sources_for, with_spin_animation};
use crate::{ActiveNativeTheme, NativeTheme};

/// gpui-component's spinner sizes its icon `size_3`, `size_3p5`, `size_4`
/// and `size_6` for XSmall, Small, Medium and Large: 0.75, 0.875, 1 and 1.5
/// rem (gpui-component 0.7.3 `src/icon.rs:188-192`, `Icon::with_size`). A
/// size other than Medium keeps that ratio to Medium, anchored at
/// `spinner.diameter`.
const XSMALL_RATIO: f32 = 0.75;
/// Small to Medium: `size_3p5` to `size_4` (gpui-component 0.7.3
/// `src/icon.rs:190-191`).
const SMALL_RATIO: f32 = 0.875;
/// Large to Medium: `size_6` to `size_4` (gpui-component 0.7.3
/// `src/icon.rs:191-192`).
const LARGE_RATIO: f32 = 1.5;

/// What a spinner paints, from `SpinnerTheme` (spec §2.6).
///
/// The diameter depends on the [`Size`]: Medium is `spinner.diameter`, the
/// platform's spinner; XSmall, Small and Large are `spinner.diameter` × 0.75,
/// × 0.875 and × 1.5, gpui-component's own spinner ladder (`size_3`,
/// `size_3p5`, `size_4`, `size_6`; gpui-component 0.7.3 `src/icon.rs:188-192`)
/// re-anchored at the platform's Medium, so the sizes keep upstream's
/// proportions and the platform's size; `Size::Size(px)` is `px`. None is
/// less than `spinner.min_diameter`. The stroke is `spinner.stroke_width` at
/// every size.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct SpinnerLook {
    /// The indicator's size: `spinner.diameter` for Medium, scaled for the
    /// other sizes ([`SpinnerLook::of_size`]), and no less than
    /// `min_diameter`.
    pub diameter: Pixels,
    /// `spinner.stroke_width`: the arc's stroke.
    pub stroke: Pixels,
    /// `spinner.fill_color`: the arc, and the tint of a monochrome bundled
    /// set's indicator.
    pub color: Hsla,
}

impl SpinnerLook {
    /// The look of a Medium spinner, or `None` when a length the theme gives
    /// is not finite.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme) -> Option<Self> {
        Self::of_size(resolved, Size::Medium)
    }

    /// The look of a spinner of `size`, or `None` when a length the theme
    /// gives, or a `Size::Size`, is not finite and non-negative.
    #[must_use]
    pub fn of_size(resolved: &ResolvedTheme, size: Size) -> Option<Self> {
        let s = &resolved.spinner;
        let medium = length(s.diameter)?;
        let min = length(s.min_diameter)?;
        let diameter = match size {
            Size::Size(own) => length(f32::from(own))?,
            Size::XSmall => px(f32::from(medium) * XSMALL_RATIO),
            Size::Small => px(f32::from(medium) * SMALL_RATIO),
            Size::Medium => medium,
            Size::Large => px(f32::from(medium) * LARGE_RATIO),
        };
        Some(Self {
            diameter: diameter.max(min),
            stroke: length(s.stroke_width)?,
            color: color(s.fill_color),
        })
    }

    /// The radius of the stroke's centre line: the model gives the diameter
    /// as the ring's outer size, and the stroke is centred on its path. No
    /// less than 0: a stroke wider than the diameter fills the disc, as
    /// `ring` clamps its inner radius at 0. Nothing draws with it; the tests
    /// check the look against the model through it.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn path_radius(&self) -> Pixels {
        px(((f32::from(self.diameter) - f32::from(self.stroke)) / 2.).max(0.))
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
    /// its text colour: it rasterizes the SVG to an alpha mask and draws that
    /// in the colour (gpui-pre 0.3.9 `src/window.rs:5007`, `:5032`).
    Spin {
        svg: Rc<[u8]>,
        duration_ms: NonZeroU32,
    },
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

impl IndicatorKey {
    /// The key of `set`'s indicator in `colour` at `size`. A freedesktop
    /// set's icon theme is `icon_theme`, or where that is `None` the system's,
    /// `system` (asked only then; [`system_theme`]), so a key names the theme
    /// its indicator comes from and a changed system theme is a key of its
    /// own; another set has no theme. The size is at most the 512 an SVG is
    /// rasterized at (`icons::MAX_ICON_SIZE`), so the sizes above it share
    /// their one image.
    fn new(
        set: IconSet,
        icon_theme: Option<SharedString>,
        system: impl FnOnce() -> Option<SharedString>,
        colour: Rgba,
        size: u32,
    ) -> Self {
        let icon_theme = match set {
            IconSet::Freedesktop => icon_theme.or_else(system),
            _ => None,
        };
        Self {
            set,
            icon_theme,
            colour: [colour.r, colour.g, colour.b, colour.a],
            size: size.clamp(1, MAX_ICON_SIZE),
        }
    }
}

/// The indicators loaded so far, one per key: loading reads the file system
/// for a freedesktop set and rasterizes every frame, so it is done once.
/// Each key's images stay for the life of the application; a key is a set,
/// an icon theme (the system's, where none is named), a colour and a size,
/// so there are as many as the combinations drawn.
#[derive(Default)]
struct Indicators {
    loaded: HashMap<IndicatorKey, Option<Rc<Indicator>>>,
    /// `system_icon_theme()`'s answer since the native theme was last
    /// installed ([`system_theme`]); `None` until it is asked.
    system_theme: Option<Option<SharedString>>,
    /// Whether an observer of [`NativeTheme`] forgets `system_theme`.
    observing: bool,
}

impl Global for Indicators {}

/// The system's icon theme, `system_icon_theme()`, asked once and kept until
/// the native theme is installed again: a spinner is drawn again every frame
/// it turns, and asking each time would take the detection cache's lock and
/// copy its answer each frame. Every install of the native theme ([`apply`],
/// [`apply_system_theme`], [`apply_accessibility`], an upstream mode switch)
/// writes the [`NativeTheme`] global, whose observer forgets the answer, so
/// a changed system theme, read again after `invalidate_caches()` and
/// installed as the runtime theme change recipe does, is asked anew and
/// makes a key of its own. `None` where the theme cannot be detected.
///
/// [`apply`]: crate::apply
/// [`apply_system_theme`]: crate::apply_system_theme
/// [`apply_accessibility`]: crate::apply_accessibility
fn system_theme(cx: &mut App) -> Option<SharedString> {
    if let Some(known) = cx
        .try_global::<Indicators>()
        .and_then(|i| i.system_theme.clone())
    {
        return known;
    }
    let theme = system_icon_theme().ok().map(SharedString::from);
    let indicators = cx.default_global::<Indicators>();
    indicators.system_theme = Some(theme.clone());
    if !std::mem::replace(&mut indicators.observing, true) {
        cx.observe_global::<NativeTheme>(|cx| {
            if cx.has_global::<Indicators>() {
                cx.global_mut::<Indicators>().system_theme = None;
            }
        })
        .detach();
    }
    theme
}

/// Load `key`'s indicator: `None` where its set has none, or it cannot be
/// made an image. Frames are coloured as their set is, in `colour`
/// ([`animated_frames_to_image_sources_for`]); a spinning SVG is kept as it
/// is, as the SVG element draws it as a mask in its text colour.
fn load(key: &IndicatorKey, colour: Rgba) -> Option<Indicator> {
    let anim = load_icon_indicator(key.set, key.icon_theme.as_deref())?;
    match &anim {
        AnimatedIcon::Frames(_) => {
            let frames = animated_frames_to_image_sources_for(
                key.set,
                &anim,
                Some(color(colour)),
                Some(key.size),
            )?;
            Some(Indicator::Frames {
                frames: frames.sources.into(),
                frame_ms: frames.frame_duration_ms.get(),
            })
        }
        AnimatedIcon::Transform(data) => match (data.icon(), data.animation()) {
            (IconData::Svg(bytes), TransformAnimation::Spin { duration_ms, .. }) => {
                Some(Indicator::Spin {
                    svg: bytes.as_ref().into(),
                    duration_ms: *duration_ms,
                })
            }
            _ => None,
        },
        _ => None,
    }
}

/// `key`'s indicator, loaded once per application.
fn indicator(key: IndicatorKey, colour: Rgba, cx: &mut App) -> Option<Rc<Indicator>> {
    if let Some(found) = cx
        .try_global::<Indicators>()
        .and_then(|c| c.loaded.get(&key))
    {
        return found.clone();
    }
    let loaded = load(&key, colour).map(Rc::new);
    cx.default_global::<Indicators>()
        .loaded
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
                            &ArcData::new(&(), 0, end - start, start * TAU, end * TAU),
                            look.color,
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
    /// `native_theme::detect::system_icon_set()`, its icon theme detected.
    System,
    /// A set, or none; for a freedesktop set, the icon theme.
    Set(Option<IconSet>, Option<SharedString>),
}

/// An indeterminate spinner, `spinner.diameter` across at `Size::Medium`, on
/// gpui-base's headless `Progress` (a progress indicator with no value).
///
/// **Every size is the platform's.** The spinner is [`Sizable`]: Medium, the
/// default, is `spinner.diameter`; XSmall, Small and Large are
/// `spinner.diameter` × 0.75, × 0.875 and × 1.5, the ratios of
/// gpui-component's own spinner sizes to its Medium (gpui-component 0.7.3
/// `src/icon.rs:188-192`, `Icon::with_size`), so `.small()` and `.large()`
/// draw the same indicator smaller and larger; `Size::Size(px)` is `px`.
/// None is less than `spinner.min_diameter` ([`SpinnerLook::of_size`]). The
/// arc's stroke stays `spinner.stroke_width` at every size.
///
/// **The icon set's indicator first.** Where the application's icon set has
/// an animated indicator -- `native_theme::icons::load_icon_indicator`, and
/// for a freedesktop set `FreedesktopLoader::load_indicator` of the icon
/// theme the icons come from (Breeze's is `process-working`) -- that
/// indicator is drawn, at the Size's diameter: the monochrome bundled sets
/// (Material, Lucide) tinted `spinner.fill_color`; a freedesktop sprite
/// sheet (Breeze's) as images in its own colours (its `currentColor`, where
/// it sets no colour of its own, `defaults.text_color`); a freedesktop icon
/// that spins (a theme whose indicator is only `process-working-symbolic`)
/// in `defaults.text_color` alone. gpui turns an SVG element but not an
/// image, whose sprite carries no transformation (gpui-pre 0.3.9
/// `src/scene.rs:749-758`), and the SVG element draws the icon's alpha
/// mask in its text colour (`src/svg_renderer.rs:241-250`,
/// `src/window.rs:5007`, `:5032`), so a spinning icon's own colours are
/// lost. Frames play at the indicator's own frame duration, a spinning icon
/// turns at its own speed, and under reduced motion the first frame stands
/// still. Name the set with [`icon_set`](Self::icon_set) and, for a
/// freedesktop set, [`icon_theme`](Self::icon_theme); without them it is
/// `native_theme::detect::system_icon_set()` with the system's icon theme,
/// `native_theme::detect::system_icon_theme()`. That theme is asked once
/// and kept until the native theme is installed again, so a change of it is
/// drawn once `native_theme::detect::invalidate_caches()` has cleared the
/// detection and the theme is installed again ([`apply_system_theme`]), as
/// the runtime theme change recipe does.
///
/// **An arc for a set without one** (`SfSymbols`, `SegoeFluent`, an icon
/// feature that is off, or `icon_set(None)`): `spinner.fill_color` at
/// `spinner.stroke_width`, its outer edge the Size's diameter across,
/// sweeping 240° and turning a turn a second; under reduced motion it stands
/// still.
///
/// At Medium the widget is drawn the same with the
/// `native-theme-egui-widgets` and iced connectors' spinners. Without a
/// native theme it renders gpui-component's `Spinner` at the same [`Size`].
///
/// [`apply_system_theme`]: crate::apply_system_theme
pub struct Spinner {
    id: ElementId,
    label: Option<SharedString>,
    icons: IconChoice,
    size: Size,
}

impl Spinner {
    /// A new spinner.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icons: IconChoice::System,
            size: Size::Medium,
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

impl Sizable for Spinner {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for Spinner {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some((look, fill, text)) = cx.native().and_then(|n| {
            let r = n.resolved;
            Some((
                SpinnerLook::of_size(r, self.size)?,
                r.spinner.fill_color,
                r.defaults.text_color,
            ))
        }) else {
            return div()
                .child(gpui_component::spinner::Spinner::new().with_size(self.size))
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
            // non-negative (`SpinnerLook::of_size`), and the cast saturates.
            let size = (f32::from(look.diameter) * window.scale_factor())
                .ceil()
                .max(1.) as u32;
            let key = IndicatorKey::new(set, icon_theme, || system_theme(cx), colour, size);
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
    fn a_set_without_an_indicator_has_none() {
        let black = Rgba::rgb(0, 0, 0);
        let key = |set| IndicatorKey::new(set, None, || None, black, 16);
        assert!(load(&key(IconSet::SfSymbols), black).is_none());
        assert!(load(&key(IconSet::SegoeFluent), black).is_none());
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

    /// A stroke wider than the diameter, which `of_size` does not bound, has
    /// its centre line at the centre, as `ring` clamps the inner radius at
    /// 0, not at a negative radius.
    #[test]
    fn the_path_radius_is_never_negative() {
        let look = |diameter: f32, stroke: f32| SpinnerLook {
            diameter: px(diameter),
            stroke: px(stroke),
            color: gpui::black(),
        };
        assert_eq!(look(4., 6.).path_radius(), px(0.));
        assert_eq!(look(16., 2.).path_radius(), px(7.));
    }

    /// A size above the 512 an SVG is rasterized at most
    /// (`icons::MAX_ICON_SIZE`) is one key, as it is one image.
    #[test]
    fn a_key_s_size_is_at_most_the_largest_raster() {
        let black = Rgba::rgb(0, 0, 0);
        let key = |size| IndicatorKey::new(IconSet::Material, None, || None, black, size);
        assert_eq!(key(4096).size, 512);
        assert!(key(4096) == key(513));
        assert_eq!(key(22).size, 22);
    }

    /// The system's icon theme is in the key of a freedesktop indicator that
    /// names none. It is asked once, not every frame: a changed system theme
    /// whose detection was cleared is not seen until the native theme is
    /// installed again, and then the spinner loads the new theme's indicator
    /// instead of finding the old one. A named theme stays, and another set
    /// has no theme. The themes are fixtures in a child process
    /// ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[gpui::test]
    fn a_changed_system_icon_theme_is_a_new_indicator(cx: &mut gpui::TestAppContext) {
        const NAME: &str =
            "widgets::spinner::tests::a_changed_system_icon_theme_is_a_new_indicator";
        const TWO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 22 44"><path fill="currentColor" d="M2 2h18v18H2zM6 28h10v10H6z"/></svg>"#;
        const THREE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 22 66"><path fill="currentColor" d="M2 2h18v18H2zM6 28h10v10H6zM8 52h6v6H8z"/></svg>"#;
        if crate::icon_fixture::in_child() {
            let black = Rgba::rgb(0, 0, 0);
            let frames = |key: &IndicatorKey| match load(key, black) {
                Some(Indicator::Frames { frames, .. }) => frames.len(),
                _ => 0,
            };
            let key = |cx: &mut App| {
                IndicatorKey::new(IconSet::Freedesktop, None, || system_theme(cx), black, 22)
            };
            let first = cx.update(|cx| key(cx));
            assert_eq!(first.icon_theme.as_deref(), Some("first"));
            assert_eq!(frames(&first), 2);

            let written = std::env::var_os("XDG_CONFIG_HOME").map(|config| {
                std::fs::write(
                    std::path::Path::new(&config).join("kdeglobals"),
                    "[Icons]\nTheme=second\n",
                )
            });
            assert!(matches!(written, Some(Ok(()))), "{written:?}");
            native_theme::detect::invalidate_caches();
            let kept = cx.update(|cx| key(cx));
            assert!(
                kept == first,
                "the answer is kept until the theme is installed"
            );

            cx.update(|cx| {
                let resolved = native_theme::theme::Theme::preset("kde-breeze")
                    .ok()
                    .and_then(|theme| {
                        theme
                            .into_variant(native_theme::theme::ColorMode::Light)
                            .ok()
                    })
                    .and_then(|variant| {
                        variant
                            .into_resolved(&native_theme::ResolutionContext::for_tests())
                            .ok()
                    });
                assert!(resolved.is_some(), "kde-breeze resolves");
                if let Some(r) = resolved {
                    let prefs = native_theme::AccessibilityPreferences::default();
                    crate::apply(
                        crate::to_theme(&r, "kde-breeze", false, &prefs),
                        &r,
                        &prefs,
                        cx,
                    );
                }
            });
            let second = cx.update(|cx| key(cx));
            assert_eq!(second.icon_theme.as_deref(), Some("second"));
            assert!(first != second, "a changed theme is a new key");
            assert_eq!(frames(&second), 3);

            let named = IndicatorKey::new(
                IconSet::Freedesktop,
                Some("first".into()),
                || None,
                black,
                22,
            );
            assert_eq!(named.icon_theme.as_deref(), Some("first"));
            let other =
                IndicatorKey::new(IconSet::Material, Some("first".into()), || None, black, 22);
            assert_eq!(other.icon_theme, None);
            return;
        }
        let ran = crate::icon_fixture::run(NAME, "first", |icons| {
            crate::icon_fixture::write_indicator_theme(icons, "first", TWO)?;
            crate::icon_fixture::write_indicator_theme(icons, "second", THREE)
        });
        assert!(ran.is_ok(), "{ran:?}");
    }
}
