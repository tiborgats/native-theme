//! [`Spinner`]: the icon set's loading indicator, or an arc at the theme's stroke.

use std::f32::consts::TAU;
use std::time::Duration;

use iced_core::{Color, Element, Length, Radians, Rectangle, Rotation, Theme, mouse};
use iced_widget::canvas::{self, Path, Stroke};
use iced_widget::graphics::geometry;
use native_theme::icons::{FreedesktopLoader, load_icon_indicator};
use native_theme::theme::{AnimatedIcon, IconData, IconSet, ResolvedTheme, TransformAnimation};

use crate::icons::{spin_rotation_radians, to_image_handle, to_svg_handle};
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
/// indicator -- `native_theme::icons::load_icon_indicator`, and for a
/// freedesktop set `FreedesktopLoader::load_indicator` of the icon theme
/// given (the system's where `None`) -- that indicator is drawn at
/// `spinner.diameter`, recoloured `spinner.fill_color` for the monochrome
/// bundled sets (Material, Lucide) and in its own colours otherwise, one
/// frame per `frame_duration_ms` or turned once per `TransformAnimation::Spin`
/// period, and still on its first frame under reduced motion.
///
/// **An arc otherwise.** Where the set has none (`SfSymbols`, `SegoeIcons`,
/// or its icon feature is off), or a frame is neither SVG nor RGBA, it draws
/// an arc on a canvas: `spinner.diameter` across its outer edge, in
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
    Frames { frames: Vec<Frame>, frame_ms: u32 },
    /// One image turned once every `period_ms`.
    Spin { frame: Frame, period_ms: u32 },
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
    /// `data` as a handle, recoloured `tint` where it is an SVG and `tint` is
    /// given; `None` where it is neither SVG nor RGBA.
    fn of(data: &IconData, tint: Option<Color>) -> Option<Self> {
        to_svg_handle(data, tint)
            .map(Self::Svg)
            .or_else(|| to_image_handle(data).map(Self::Image))
    }
}

impl Spinner {
    /// The spinner of `icon_set` under `resolved`: for
    /// [`IconSet::Freedesktop`], the indicator of `icon_theme` (the system's
    /// icon theme where `None`), never another theme's.
    #[must_use]
    pub fn new(resolved: &ResolvedTheme, icon_set: IconSet, icon_theme: Option<&str>) -> Self {
        let s = &resolved.spinner;
        let color = to_color(s.fill_color);
        let indicator = match icon_set {
            IconSet::Freedesktop => FreedesktopLoader::load_indicator(icon_theme),
            other => load_icon_indicator(other),
        };
        let tint = matches!(icon_set, IconSet::Material | IconSet::Lucide).then_some(color);
        Self {
            drawn: indicator
                .and_then(|anim| drawn(&anim, tint))
                .unwrap_or(Drawn::Arc),
            diameter: s.diameter,
            stroke_width: s.stroke_width,
            color,
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
                    elapsed.as_secs_f32().fract() * TAU
                },
            })
            .width(size)
            .height(size)
            .into(),
        }
    }
}

/// What draws `anim`, its SVG frames recoloured `tint` where given; `None`
/// where a frame cannot be drawn or the animation is of a kind this crate
/// does not know.
fn drawn(anim: &AnimatedIcon, tint: Option<Color>) -> Option<Drawn> {
    if let (Some(list), Some(frame_ms)) = (anim.frame_list(), anim.frame_duration_ms()) {
        let frames = list
            .iter()
            .map(|data| Frame::of(data, tint))
            .collect::<Option<Vec<_>>>()?;
        return Some(Drawn::Frames {
            frames,
            frame_ms: frame_ms.get(),
        });
    }
    match (anim.icon(), anim.animation()) {
        (Some(icon), Some(TransformAnimation::Spin { duration_ms })) => Some(Drawn::Spin {
            frame: Frame::of(icon, tint)?,
            period_ms: duration_ms.get(),
        }),
        _ => None,
    }
}

/// The frame shown `elapsed` into an animation of `count` frames, each
/// `frame_ms` long, looping.
fn frame_index(elapsed: Duration, frame_ms: u32, count: usize) -> usize {
    let Ok(count) = u128::try_from(count) else {
        return 0;
    };
    let cycle = u128::from(frame_ms).saturating_mul(count);
    elapsed
        .as_millis()
        .checked_rem(cycle)
        .and_then(|position| position.checked_div(u128::from(frame_ms)))
        .and_then(|index| usize::try_from(index).ok())
        .unwrap_or(0)
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

    #[test]
    fn frames_loop_one_per_frame_duration() {
        let at = |ms| frame_index(Duration::from_millis(ms), 42, 24);
        assert_eq!(at(0), 0);
        assert_eq!(at(41), 0);
        assert_eq!(at(42), 1);
        assert_eq!(at(42 * 24 - 1), 23);
        assert_eq!(at(42 * 24), 0);
        assert_eq!(frame_index(Duration::from_millis(5), 0, 24), 0);
        assert_eq!(frame_index(Duration::from_millis(5), 42, 0), 0);
    }

    #[test]
    fn a_set_without_an_indicator_draws_the_arc() -> native_theme::Result<()> {
        let (_, resolved) = crate::from_preset("kde-breeze", false)?;
        for set in [IconSet::SfSymbols, IconSet::SegoeIcons] {
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

    #[cfg(feature = "material-icons")]
    #[test]
    fn material_draws_its_frames_in_the_spinner_colour() -> native_theme::Result<()> {
        let (_, resolved) = crate::from_preset("material", true)?;
        let spinner = Spinner::new(&resolved, IconSet::Material, None);
        let Some(anim) = load_icon_indicator(IconSet::Material) else {
            panic!("material-icons is on, yet Material has no indicator");
        };
        let tint = to_color(resolved.spinner.fill_color);
        let expected = to_svg_handle(anim.first_frame(), Some(tint));
        match &spinner.drawn {
            Drawn::Frames { frames, frame_ms } => {
                assert_eq!(frames.len(), anim.frame_list().map_or(0, |l| l.len()));
                assert_eq!(Some(*frame_ms), anim.frame_duration_ms().map(|d| d.get()));
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
