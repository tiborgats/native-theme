//! [`Spinner`]: the icon set's loading indicator, or an arc at the theme's stroke.

use std::f32::consts::TAU;
use std::time::Duration;

use iced_core::{Color, Element, Length, Radians, Rectangle, Rotation, Theme, mouse};
use iced_widget::canvas::{self, Path, Stroke};
use iced_widget::graphics::geometry;
use native_theme::Rgba;
use native_theme::icons::{
    AnimatedIcon, IconData, TransformAnimation, fill_current_color, load_icon_indicator,
};
use native_theme::theme::{IconSet, ResolvedTheme};

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

/// How an indicator's SVG frames are coloured.
#[derive(Debug, Clone, Copy)]
enum Paint {
    /// Recoloured through `colorize_monochrome_svg` (Material, Lucide).
    Tint(Color),
    /// `currentColor` filled in where the document sets no `color` of its
    /// own (freedesktop).
    CurrentColor(Rgba),
    /// As they are.
    AsIs,
}

impl Frame {
    /// `data` as a handle, its SVG coloured as `paint` says; `None` where it
    /// is neither SVG nor RGBA.
    fn of(data: &IconData, paint: Paint) -> Option<Self> {
        let svg = match (paint, data) {
            (Paint::Tint(tint), _) => to_svg_handle(data, Some(tint)),
            (Paint::CurrentColor(color), IconData::Svg(bytes)) => Some(
                iced_core::svg::Handle::from_memory(fill_current_color(bytes, color).into_owned()),
            ),
            _ => to_svg_handle(data, None),
        };
        svg.map(Self::Svg)
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
        let paint = match icon_set {
            IconSet::Material | IconSet::Lucide => Paint::Tint(color),
            IconSet::Freedesktop => Paint::CurrentColor(resolved.defaults.text_color),
            _ => Paint::AsIs,
        };
        Self {
            drawn: load_icon_indicator(icon_set, icon_theme)
                .and_then(|anim| drawn(&anim, paint))
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
                    arc_start(elapsed)
                },
            })
            .width(size)
            .height(size)
            .into(),
        }
    }
}

/// What draws `anim`, its SVG frames coloured as `paint` says; `None` where
/// a frame cannot be drawn or the animation is of a kind this crate does not
/// know.
fn drawn(anim: &AnimatedIcon, paint: Paint) -> Option<Drawn> {
    if let (Some(list), Some(frame_ms)) = (anim.frame_list(), anim.frame_duration_ms()) {
        let frames = list
            .iter()
            .map(|data| Frame::of(data, paint))
            .collect::<Option<Vec<_>>>()?;
        return Some(Drawn::Frames {
            frames,
            frame_ms: frame_ms.get(),
        });
    }
    match (anim.icon(), anim.animation()) {
        (Some(icon), Some(TransformAnimation::Spin { duration_ms })) => Some(Drawn::Spin {
            frame: Frame::of(icon, paint)?,
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

    /// Names the fixture's root in the child process that
    /// [`a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour`] runs.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    const SYMBOLIC_FIXTURE_ROOT: &str = "NATIVE_THEME_ICED_SYMBOLIC_SPINNER_FIXTURE";

    /// A theme whose spinner is only `process-working-symbolic`, as GNOME's
    /// (platform-facts.md §2.23), draws it in `defaults.text_color`, not the
    /// black usvg gives an unset `currentColor`. The icon dirs are read once
    /// per process, so the test runs itself again in a child process whose
    /// `HOME`, `XDG_DATA_HOME` and `XDG_DATA_DIRS` all lie in a temp dir that
    /// holds the theme: no system dir is searched.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour() -> native_theme::Result<()> {
        const NAME: &str =
            "spinner::tests::a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour";
        if std::env::var_os(SYMBOLIC_FIXTURE_ROOT).is_some() {
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
            let svg = std::str::from_utf8(bytes).unwrap();
            let c = resolved.defaults.text_color;
            let hex = format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b);
            assert!(
                !svg.contains("currentColor") && svg.contains(&hex),
                "{hex}: {svg}"
            );
            return Ok(());
        }
        let root = std::env::temp_dir().join(format!(
            "native-theme-iced-symbolic-spinner-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let theme = root.join("share/icons/symbolic-only");
        std::fs::create_dir_all(theme.join("actions")).unwrap();
        std::fs::create_dir_all(root.join("home")).unwrap();
        std::fs::write(
            theme.join("index.theme"),
            "[Icon Theme]\nName=symbolic-only\nDirectories=actions\n\n\
             [actions]\nSize=16\nType=Scalable\nMinSize=8\nMaxSize=512\n",
        )
        .unwrap();
        std::fs::write(
            theme.join("actions/process-working-symbolic.svg"),
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="#2e3436" d="M0 0h16v16H0z"/></svg>"##,
        )
        .unwrap();
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", NAME, "--nocapture", "--test-threads=1"])
            .env(SYMBOLIC_FIXTURE_ROOT, &root)
            .env("HOME", root.join("home"))
            .env("XDG_DATA_HOME", root.join("home/.local/share"))
            .env("XDG_DATA_DIRS", root.join("share"))
            .output()
            .unwrap();
        let _ = std::fs::remove_dir_all(&root);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success() && stdout.contains("1 passed"),
            "{stdout}{stderr}"
        );
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
