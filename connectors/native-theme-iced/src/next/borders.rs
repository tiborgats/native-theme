//! A part's border as the shared style functions take it: the stroke as
//! stated, with its alignment ([`Line`]), its width and the corners
//! ([`Outline`]); the style function composites the stroke over the fill it
//! paints ([`Line::over`], the parent module's doc).

use iced_core::Border;
use iced_core::border::Radius;
use native_theme::next::{
    CornerCurve, CornerRadius, Corners, PartStated, PartStyle, Stroke, StrokeAlign,
};

use super::{Mapped, Unmapped, radius_of, stated, stroke_width_of, to_color};
use crate::styles::{Line, LineAlign, Outline};

/// A stroke's alignment as the shared style functions take it.
fn line_align(align: StrokeAlign, home: &'static str) -> Mapped<LineAlign> {
    match align {
        StrokeAlign::Outside => Ok(LineAlign::Outside),
        StrokeAlign::Inside => Ok(LineAlign::Inside),
        StrokeAlign::Center => Ok(LineAlign::Center),
        _ => Err(Unmapped(home)),
    }
}

/// A part's stroke, where stated as one stroke of one colour: its colour as
/// stated and where it lies, which the style function composites over the
/// fill it paints.
pub(crate) fn line(part: &PartStyle, home: &'static str) -> Mapped<Line> {
    match &part.stroke {
        Stroke::Uniform { color, align, .. } if part.stated.contains(PartStated::STROKE) => {
            Ok(Line {
                align: line_align(*align, home)?,
                ..Line::outside(to_color(*color))
            })
        }
        _ => Err(Unmapped(home)),
    }
}

/// [`line()`], or, where the part states no stroke (`Stroke::None`), no
/// stroke ([`Line::none`]), which iced draws 0 wide.
pub(crate) fn line_or_none(part: &PartStyle, home: &'static str) -> Mapped<Line> {
    match part.stroke {
        Stroke::None if part.stated.contains(PartStated::STROKE) => Ok(Line::none()),
        _ => line(part, home),
    }
}

/// A part's stroke where a rule states it for the queried state, as
/// [`line_or_none`] reads it; `None` where none does.
pub(crate) fn state_line(part: &PartStyle, home: &'static str) -> Mapped<Option<Line>> {
    if part.from_state.contains(PartStated::STROKE) {
        line_or_none(part, home).map(Some)
    } else {
        Ok(None)
    }
}

/// `read` of the part `at` gives, where a rule states its `property` for
/// the state of the query `flag` gives, queried with that flag alone (spec
/// §2, `from_state`); `None` where none does. A value a rule for another
/// flag of `at`'s query states (a checked box's fill under the pointer) is
/// not this state's.
pub(crate) fn flagged<T>(
    flag: &PartStyle,
    at: &PartStyle,
    property: PartStated,
    read: impl FnOnce(&PartStyle) -> Mapped<T>,
) -> Mapped<Option<T>> {
    if flag.from_state.contains(property) {
        read(at).map(Some)
    } else {
        Ok(None)
    }
}

/// A part's stroke width, where stated as one stroke, in any alignment, or
/// as none, which is no width.
pub(crate) fn stroke_width(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    stated(stroke_width_of(part), home)
}

/// A part's corner radius, where stated as one circular radius, as square
/// corners, which are no radius, or as a pill, which iced rounds by a radius
/// its renderers clamp to half the shorter side ([`crate::styles::pill`]).
pub(crate) fn radius(part: &PartStyle, home: &'static str) -> Mapped<f32> {
    stated(radius_of(part), home)
}

/// Stated `corners` as iced rounds a quad, corner by corner: square corners
/// are no radius, a pill a radius iced clamps to half the shorter side
/// ([`crate::styles::pill`]).
///
/// # Errors
///
/// [`Unmapped`] at `home` for a squircle (`CornerCurve::Continuous`) or a
/// radius of a kind iced has no form for.
pub(crate) fn corner_radii(corners: &Corners, home: &'static str) -> Mapped<Radius> {
    match (corners.radius, corners.curve) {
        (CornerRadius::None, _) => Ok(Radius::from(0.0)),
        (CornerRadius::Uniform(r), CornerCurve::Circular) => Ok(Radius::from(r)),
        (
            CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            },
            CornerCurve::Circular,
        ) => Ok(Radius {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }),
        (CornerRadius::Pill, CornerCurve::Circular) => Ok(crate::styles::pill()),
        _ => Err(Unmapped(home)),
    }
}

/// A part's border as iced outlines a widget, of a part a resolved theme
/// always states it for: the stroke ([`line()`]) and its width, a stated
/// `Stroke::None` being no width in iced's own colour, and the corner
/// radius.
pub(crate) fn outline_of(
    part: &PartStyle,
    color_home: &'static str,
    width_home: &'static str,
    corners_home: &'static str,
) -> Mapped<Outline> {
    let width = stroke_width(part, width_home)?;
    Ok(Outline {
        line: line_or_none(part, color_home)?,
        width,
        radius: Radius::new(radius(part, corners_home)?),
    })
}

/// [`outline_of`] for a part whose border the table of homes allows to be
/// unstated (`unstated_if_unset`): the stroke and its width where the stroke
/// is stated, the radius where the corners are, and iced's own
/// (`Border::default()`, no line and square corners) for each the theme
/// leaves unstated (spec §10, rule 0).
pub(crate) fn outline_or_iced_of(
    part: &PartStyle,
    color_home: &'static str,
    width_home: &'static str,
    corners_home: &'static str,
) -> Mapped<Outline> {
    let iced = Border::default();
    let mut outline = Outline {
        line: Line::outside(iced.color),
        width: iced.width,
        radius: iced.radius,
    };
    if part.stated.contains(PartStated::STROKE) {
        outline.width = stroke_width(part, width_home)?;
        outline.line = line_or_none(part, color_home)?;
    }
    if part.stated.contains(PartStated::CORNERS) {
        outline.radius = Radius::new(radius(part, corners_home)?);
    }
    Ok(outline)
}

/// [`outline_of`] the part at `$home`, each value's home under it.
macro_rules! outline {
    ($part:expr, $home:expr) => {
        $crate::next::borders::outline_of(
            &$part,
            concat!($home, ".stroke.color"),
            concat!($home, ".stroke.width"),
            concat!($home, ".corners"),
        )
    };
}
pub(crate) use outline;

/// [`outline_or_iced_of`] the part at `$home`, each value's home under it.
macro_rules! outline_or_iced {
    ($part:expr, $home:expr) => {
        $crate::next::borders::outline_or_iced_of(
            &$part,
            concat!($home, ".stroke.color"),
            concat!($home, ".stroke.width"),
            concat!($home, ".corners"),
        )
    };
}
pub(crate) use outline_or_iced;

#[cfg(test)]
mod tests {
    use iced_core::Color as IcedColor;
    use native_theme::next::{Color, Paint};

    use super::*;
    use crate::next::fill;

    /// A part that states its fill and a uniform stroke at `align`.
    fn part(fill: Paint, stroke: Color, align: StrokeAlign) -> PartStyle {
        let mut part = PartStyle::default();
        part.fill = fill;
        part.stroke = Stroke::uniform_aligned(1.0, stroke, align);
        part.stated = PartStated::FILL | PartStated::STROKE;
        part
    }

    const FILL: Color = Color::new(0.2, 0.4, 0.6, 1.0);
    const OPAQUE: Color = Color::new(0.9, 0.1, 0.1, 1.0);
    const TRANSLUCENT: Color = Color::new(0.0, 0.0, 0.0, 0.25);

    /// The colour iced is handed for a part's stroke over its own fill.
    fn on_its_fill(p: &PartStyle) -> Mapped<IcedColor> {
        let painted = match p.fill {
            Paint::None => IcedColor::TRANSPARENT,
            _ => fill(p, "fill")?,
        };
        Ok(line(p, "stroke")?.over(painted))
    }

    #[test]
    fn an_outside_stroke_is_handed_as_stated() -> Mapped<()> {
        let p = part(Paint::Solid(FILL), TRANSLUCENT, StrokeAlign::Outside);
        assert_eq!(on_its_fill(&p)?, to_color(TRANSLUCENT));
        Ok(())
    }

    #[test]
    fn an_inside_stroke_is_composited_over_the_fill() -> Mapped<()> {
        let p = part(Paint::Solid(FILL), TRANSLUCENT, StrokeAlign::Inside);
        let expected = crate::styles::composite_over(to_color(TRANSLUCENT), to_color(FILL));
        assert_eq!(on_its_fill(&p)?, expected);
        assert!(
            expected.a >= 1.0,
            "an opaque fill under it gives an opaque band"
        );
        Ok(())
    }

    #[test]
    fn a_center_stroke_is_composited_over_the_fill_at_half_its_alpha() -> Mapped<()> {
        let p = part(Paint::Solid(FILL), TRANSLUCENT, StrokeAlign::Center);
        let half = IcedColor {
            a: FILL.a / 2.0,
            ..to_color(FILL)
        };
        assert_eq!(
            on_its_fill(&p)?,
            crate::styles::composite_over(to_color(TRANSLUCENT), half)
        );
        // Source-over of (0, 0, 0, 0.25) on (0.2, 0.4, 0.6, 0.5): alpha
        // 0.25 + 0.5 × 0.75 = 0.625, red 0.2 × 0.5 × 0.75 / 0.625 = 0.12.
        let band = on_its_fill(&p)?;
        assert!((band.a - 0.625).abs() < 1e-6, "{band:?}");
        assert!((band.r - 0.12).abs() < 1e-6, "{band:?}");
        Ok(())
    }

    #[test]
    fn an_opaque_stroke_is_handed_as_stated_in_every_alignment() -> Mapped<()> {
        for align in [
            StrokeAlign::Outside,
            StrokeAlign::Inside,
            StrokeAlign::Center,
        ] {
            let p = part(Paint::Solid(FILL), OPAQUE, align);
            assert_eq!(on_its_fill(&p)?, to_color(OPAQUE), "{align:?}");
        }
        Ok(())
    }

    #[test]
    fn a_stroke_over_no_fill_is_handed_as_stated() -> Mapped<()> {
        for align in [StrokeAlign::Inside, StrokeAlign::Center] {
            let p = part(Paint::None, TRANSLUCENT, align);
            assert_eq!(on_its_fill(&p)?, to_color(TRANSLUCENT), "{align:?}");
        }
        Ok(())
    }

    #[test]
    fn a_missing_stroke_or_fill_names_its_own_home() {
        let mut p = part(Paint::Solid(FILL), TRANSLUCENT, StrokeAlign::Inside);
        p.stated = PartStated::STROKE;
        assert_eq!(fill(&p, "x.fill"), Err(Unmapped("x.fill")));
        p.stated = PartStated::FILL;
        assert_eq!(line(&p, "x.stroke.color"), Err(Unmapped("x.stroke.color")));
    }

    #[test]
    fn an_outline_is_strict_and_the_unstated_one_is_iceds_own() -> Mapped<()> {
        let mut p = part(Paint::Solid(FILL), OPAQUE, StrokeAlign::Inside);
        assert_eq!(
            outline_of(&p, "c", "w", "r"),
            Err(Unmapped("r")),
            "corners unstated"
        );
        p.stated = PartStated::FILL;
        let iced = Border::default();
        assert_eq!(
            outline_or_iced_of(&p, "c", "w", "r")?.over(to_color(FILL)),
            iced
        );
        p.stroke = Stroke::None;
        p.stated = PartStated::STROKE | PartStated::CORNERS;
        assert_eq!(outline_of(&p, "c", "w", "r")?.over(to_color(FILL)), iced);
        Ok(())
    }
}
