//! [`Spinner`]: the icon set's loading indicator, or an arc at the theme's stroke.

use std::f32::consts::TAU;
use std::num::NonZeroU32;
use std::time::Duration;

use iced_core::{Color, Element, Length, Radians, Rectangle, Rotation, Theme, mouse};
use iced_widget::canvas::{self, Path, Stroke};
use iced_widget::graphics::geometry;
use native_theme::icons::{AnimatedIcon, IconData, TransformAnimation, load_icon_indicator};
use native_theme::theme::{IconSet, ResolvedTheme};

use crate::icons::{
    animated_frames_to_svg_handles_for, set_svg_handle, spin_rotation_radians, to_image_handle,
};
use crate::palette::to_color;

/// The painted arc's sweep, in degrees: the widest egui's `Spinner` draws
/// (egui 0.36.2 `src/widgets/spinner.rs:48-49`, `240°` times the sine of the
/// time), kept constant. A sweep of `240°` times the sine passes through
/// nothing every π seconds, so a snapshot taken then shows no arc at all. The
/// model states the arc's diameter, stroke width and colour, not its sweep.
const SWEEP_DEGREES: f32 = 240.0;

/// A loading indicator `spinner.diameter` across.
///
/// **The icon set's indicator first.** Where the icon set has an animated
/// indicator -- `native_theme::icons::load_icon_indicator`, for a freedesktop
/// set of the icon theme given (the system's where `None`) -- that indicator
/// is drawn at `spinner.diameter`, recoloured `spinner.fill_color` for the
/// monochrome bundled sets (Material, Lucide). A freedesktop indicator keeps
/// its own colours, except that its `currentColor` is filled in with
/// `defaults.text_color` where the document sets no `color` of its own
/// (`native_theme::icons::fill_current_color`): a symbolic
/// `process-working-symbolic`, GNOME's spinner, is drawn in the text colour,
/// and Breeze's, whose stylesheet states its colour, as Breeze colours it.
/// It is shown one frame per `frame_duration_ms` or turned once per
/// `TransformAnimation::Spin` period, and still on its first frame under
/// reduced motion.
///
/// **An arc otherwise.** Where the set has none (`SfSymbols`, `SegoeFluent`,
/// or its icon feature is off), where no frame of a frame sequence is SVG (its
/// frames go through `icons::animated_frames_to_svg_handles_for`, which
/// leaves RGBA frames out), or where a turned image is neither SVG nor RGBA,
/// it draws an arc on a canvas: `spinner.diameter` across its outer edge, in
/// `spinner.fill_color` at `spinner.stroke_width`, the stroke centred on a
/// circle of radius `0.5 · (diameter − stroke_width)`. It sweeps 240° and
/// turns a turn a second, egui's `Spinner`'s widest sweep and speed (egui
/// 0.36.2 `src/widgets/spinner.rs:47-49`); under reduced motion it stands
/// still from angle 0. iced has no spinner, and `iced_aw`'s paints one
/// orbiting dot (iced_aw 0.14.1 `src/widget/spinner.rs`), no arc.
///
/// `spinner.min_diameter` is not read: the spinner offers no size of its own
/// for it to bound.
///
/// **Build it once** per theme and icon set -- [`Spinner::new`] reads the
/// icon theme from the file system and recolours every frame -- and call
/// [`Spinner::view`] with the time since the animation started, redrawing on
/// a timer (`iced::time::every`) while it shows.
///
/// ```rust,ignore
/// // On a theme or icon-set change:
/// let spinner = native_theme_iced::Spinner::new(&resolved, IconSet::Freedesktop, Some("breeze"));
/// // In view():
/// spinner.view(start.elapsed(), prefs.reduce_motion)
/// ```
///
/// Requires the `spinner` feature (on by default).
#[derive(Debug, Clone)]
pub struct Spinner {
    drawn: Drawn,
    diameter: f32,
    stroke_width: f32,
    color: Color,
}

/// What a [`Spinner`] draws.
#[derive(Debug, Clone)]
enum Drawn {
    /// The indicator's frames, each `frame_ms` long.
    Frames {
        frames: Vec<Frame>,
        frame_ms: NonZeroU32,
    },
    /// One image turned once every `period_ms`.
    Spin { frame: Frame, period_ms: NonZeroU32 },
    /// No indicator: the arc.
    Arc,
}

/// One image of an indicator, in the widget that draws it.
#[derive(Debug, Clone)]
enum Frame {
    Svg(iced_core::svg::Handle),
    Image(iced_core::image::Handle),
}

impl Frame {
    /// `data`, an icon of `set`, as a handle, its SVG coloured in `color` as
    /// `set` colours it; `None` where it is neither SVG nor RGBA.
    fn of(data: &IconData, set: IconSet, color: Option<Color>) -> Option<Self> {
        set_svg_handle(set, data, color)
            .map(Self::Svg)
            .or_else(|| to_image_handle(data).map(Self::Image))
    }
}

/// What a [`Spinner`] reads of a theme, every read in one place, so the
/// contract's colour-leaf coverage test sees the widget's reads as it sees a
/// style's.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `spinner.diameter`: the spinner's size.
    pub(crate) diameter: f32,
    /// `spinner.stroke_width`: the arc's stroke.
    pub(crate) stroke_width: f32,
    /// `spinner.fill_color`: the arc, and a Material or Lucide indicator's
    /// tint.
    pub(crate) color: Color,
    /// `defaults.text_color`: a freedesktop indicator's `currentColor`.
    pub(crate) current_color: Color,
}

impl Look {
    /// The look of a spinner under `resolved`.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let s = &resolved.spinner;
        Self {
            diameter: s.diameter,
            stroke_width: s.stroke_width,
            color: to_color(s.fill_color),
            current_color: to_color(resolved.defaults.text_color),
        }
    }
}

impl Spinner {
    /// The spinner of `icon_set` under `resolved`: for
    /// [`IconSet::Freedesktop`], the indicator of `icon_theme` (the system's
    /// icon theme where `None`), never another theme's.
    #[must_use]
    pub fn new(resolved: &ResolvedTheme, icon_set: IconSet, icon_theme: Option<&str>) -> Self {
        let look = Look::of(resolved);
        let color = match icon_set {
            IconSet::Material | IconSet::Lucide => Some(look.color),
            IconSet::Freedesktop => Some(look.current_color),
            _ => None,
        };
        Self {
            drawn: load_icon_indicator(icon_set, icon_theme)
                .and_then(|anim| drawn(&anim, icon_set, color))
                .unwrap_or(Drawn::Arc),
            diameter: look.diameter,
            stroke_width: look.stroke_width,
            color: look.color,
        }
    }

    /// Whether it draws the icon set's indicator; `false` where it draws the
    /// arc.
    #[must_use]
    pub fn is_indicator(&self) -> bool {
        !matches!(self.drawn, Drawn::Arc)
    }

    /// The spinner `elapsed` into its animation, or still where
    /// `reduce_motion` is `true` (pass `AccessibilityPreferences::reduce_motion`).
    #[must_use]
    pub fn view<'a, Message, Renderer>(
        &self,
        elapsed: Duration,
        reduce_motion: bool,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Message: 'a,
        Renderer: iced_core::svg::Renderer
            + iced_core::image::Renderer<Handle = iced_core::image::Handle>
            + geometry::Renderer
            + 'a,
    {
        let size = Length::Fixed(self.diameter);
        let (frame, angle) = match &self.drawn {
            Drawn::Frames { frames, frame_ms } => {
                let index = if reduce_motion {
                    0
                } else {
                    frame_index(elapsed, *frame_ms, frames.len())
                };
                (frames.get(index), Radians(0.0))
            }
            Drawn::Spin { frame, period_ms } => {
                let angle = if reduce_motion {
                    Radians(0.0)
                } else {
                    spin_rotation_radians(elapsed, *period_ms)
                };
                (Some(frame), angle)
            }
            Drawn::Arc => (None, Radians(0.0)),
        };
        let rotation = Rotation::Floating(angle);
        match frame {
            Some(Frame::Svg(handle)) => iced_widget::svg(handle.clone())
                .width(size)
                .height(size)
                .rotation(rotation)
                .into(),
            Some(Frame::Image(handle)) => iced_widget::image(handle.clone())
                .width(size)
                .height(size)
                .rotation(rotation)
                .into(),
            None => iced_widget::canvas(Arc {
                color: self.color,
                stroke_width: self.stroke_width,
                start: if reduce_motion {
                    0.0
                } else {
                    arc_start(elapsed)
                },
            })
            .width(size)
            .height(size)
            .into(),
        }
    }
}

/// What draws `anim`, an indicator of `set`, its SVG frames coloured in
/// `color` as `set` colours them; `None` where no frame can be drawn or the
/// animation is of a kind this crate does not know.
fn drawn(anim: &AnimatedIcon, set: IconSet, color: Option<Color>) -> Option<Drawn> {
    if anim.frame_list().is_some() {
        let frames = animated_frames_to_svg_handles_for(set, anim, color)?;
        return Some(Drawn::Frames {
            frames: frames.handles.into_iter().map(Frame::Svg).collect(),
            frame_ms: frames.frame_duration_ms,
        });
    }
    match (anim.icon(), anim.animation()) {
        (Some(icon), Some(&TransformAnimation::Spin { duration_ms })) => Some(Drawn::Spin {
            frame: Frame::of(icon, set, color)?,
            period_ms: duration_ms,
        }),
        _ => None,
    }
}

/// The frame shown `elapsed` into an animation of `count` frames, each
/// `frame_ms` long, looping.
fn frame_index(elapsed: Duration, frame_ms: NonZeroU32, count: usize) -> usize {
    let Ok(count) = u128::try_from(count) else {
        return 0;
    };
    let frame_ms = u128::from(frame_ms.get());
    let cycle = frame_ms.saturating_mul(count);
    elapsed
        .as_millis()
        .checked_rem(cycle)
        .and_then(|position| position.checked_div(frame_ms))
        .and_then(|index| usize::try_from(index).ok())
        .unwrap_or(0)
}

/// The arc's start angle `elapsed` into its animation, in radians clockwise
/// from the right: a turn a second (see [`Spinner`]), from the sub-second
/// part alone, which an f32 of the whole seconds loses as they grow.
fn arc_start(elapsed: Duration) -> f32 {
    elapsed.subsec_nanos() as f32 / 1e9 * TAU
}

/// The arc a [`Spinner`] draws where the icon set has no indicator: `color`
/// at `stroke_width`, centred on the circle half a stroke inside the canvas,
/// from `start` (radians clockwise from the right) through
/// [`SWEEP_DEGREES`].
struct Arc {
    color: Color,
    stroke_width: f32,
    start: f32,
}

impl<Message, Renderer> canvas::Program<Message, Theme, Renderer> for Arc
where
    Renderer: geometry::Renderer,
{
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let radius = ((frame.width().min(frame.height()) - self.stroke_width) / 2.0).max(0.0);
        let arc = Path::new(|path| {
            path.arc(canvas::path::Arc {
                center: frame.center(),
                radius,
                start_angle: Radians(self.start),
                end_angle: Radians(self.start + SWEEP_DEGREES.to_radians()),
            });
        });
        frame.stroke(
            &arc,
            Stroke::default()
                .with_color(self.color)
                .with_width(self.stroke_width),
        );
        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    use crate::icon_fixture::Child;
    #[cfg(feature = "material-icons")]
    use crate::icons::to_svg_handle;

    #[test]
    fn frames_loop_one_per_frame_duration() {
        let frame_ms = NonZeroU32::new(42).unwrap();
        let at = |ms| frame_index(Duration::from_millis(ms), frame_ms, 24);
        assert_eq!(at(0), 0);
        assert_eq!(at(41), 0);
        assert_eq!(at(42), 1);
        assert_eq!(at(42 * 24 - 1), 23);
        assert_eq!(at(42 * 24), 0);
        assert_eq!(frame_index(Duration::from_millis(5), frame_ms, 0), 0);
    }

    /// Fourteen days and 50 ms in, the arc is a twentieth of a turn round:
    /// the phase comes from the sub-second nanoseconds, not an f32 of the
    /// seconds, whose step at 1.2e6 s is 0.125 s.
    #[test]
    fn the_arc_turns_exactly_two_weeks_in() {
        let start = arc_start(Duration::from_millis(1_209_600_050));
        assert!(
            (start - 0.05 * TAU).abs() < 1e-4,
            "expected 0.05·TAU, got {start}"
        );
    }

    #[test]
    fn a_set_without_an_indicator_draws_the_arc() -> native_theme::Result<()> {
        let (_, resolved) = crate::from_preset("kde-breeze", false)?;
        for set in [IconSet::SfSymbols, IconSet::SegoeFluent] {
            assert!(
                !Spinner::new(&resolved, set, None).is_indicator(),
                "{set:?}"
            );
        }
        assert!(
            !Spinner::new(&resolved, IconSet::Freedesktop, Some("no-such-icon-theme"))
                .is_indicator(),
            "a freedesktop theme without a spinner draws the arc, never the system's"
        );
        Ok(())
    }

    /// A theme whose spinner is only `process-working-symbolic`, as GNOME's
    /// (platform-facts.md §2.23), draws it in `defaults.text_color`, not the
    /// black usvg gives an unset `currentColor`. The theme is a fixture in a
    /// child process ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour()
    -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str =
            "spinner::tests::a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour";
        if crate::icon_fixture::in_child() {
            let (_, resolved) = crate::from_preset("adwaita", true)?;
            let spinner = Spinner::new(&resolved, IconSet::Freedesktop, Some("symbolic-only"));
            let Drawn::Spin {
                frame: Frame::Svg(handle),
                ..
            } = &spinner.drawn
            else {
                panic!(
                    "the symbolic indicator is not a turned SVG: {:?}",
                    spinner.drawn
                );
            };
            let iced_core::svg::Data::Bytes(bytes) = handle.data() else {
                panic!("the frame is not in memory");
            };
            let svg = std::str::from_utf8(bytes)?;
            let c = resolved.defaults.text_color;
            let hex = format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b);
            assert!(
                !svg.contains("currentColor") && svg.contains(&hex),
                "{hex}: {svg}"
            );
            return Ok(());
        }
        // The system's icon theme is one that does not exist, so only the
        // theme the test names holds the indicator.
        crate::icon_fixture::run(NAME, Child::kde("no-such-icon-theme"), |icons| {
            crate::icon_fixture::write_scalable_theme(
                icons,
                "symbolic-only",
                &[(
                    "process-working-symbolic",
                    r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#2e3436" d="M0 0h16v16H0z"/></svg>"##,
                )],
            )
        })?;
        Ok(())
    }

    #[cfg(feature = "material-icons")]
    #[test]
    fn material_draws_its_frames_in_the_spinner_colour() -> native_theme::Result<()> {
        let (_, resolved) = crate::from_preset("material-3", true)?;
        let spinner = Spinner::new(&resolved, IconSet::Material, None);
        let Some(anim) = load_icon_indicator(IconSet::Material, None) else {
            panic!("material-icons is on, yet Material has no indicator");
        };
        let tint = to_color(resolved.spinner.fill_color);
        let expected = to_svg_handle(anim.first_frame(), Some(tint));
        match &spinner.drawn {
            Drawn::Frames { frames, frame_ms } => {
                assert_eq!(frames.len(), anim.frame_list().map_or(0, |l| l.len()));
                assert_eq!(Some(*frame_ms), anim.frame_duration_ms());
                match (frames.first(), expected) {
                    (Some(Frame::Svg(first)), Some(expected)) => assert_eq!(first, &expected),
                    other => panic!("the first frame is not the recoloured SVG: {other:?}"),
                }
            }
            other => panic!("Material's indicator is frames, drawn as {other:?}"),
        }
        Ok(())
    }
}
