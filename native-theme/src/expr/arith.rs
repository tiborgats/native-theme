//! The model's own arithmetic (S2.2): `alpha`, `with_alpha`, `mix`, `mul`,
//! `div`, `add`, `sub`, `min`, `max`, `floor`, `round` and `pick`, total as
//! S2.2's *Totality* says.

use super::ty::{BinOp, Bound};
use crate::color::Color;

/// `x`, or zero where it is not finite: a non-finite result, or non-finite
/// element data, is zero.
pub(super) fn finite(x: f32) -> f32 {
    if x.is_finite() { x } else { 0.0 }
}

/// `c` with each non-finite channel zero.
pub(super) fn finite_color(c: Color) -> Color {
    Color {
        r: finite(c.r),
        g: finite(c.g),
        b: finite(c.b),
        a: finite(c.a),
    }
}

/// `alpha(c, a)`: `c` with its alpha multiplied by `a`, the result clamped
/// to `0..=1` as CSS Color 5's relative colours clamp it (a factor above one
/// is valid).
pub(super) fn alpha(c: Color, a: f32) -> Color {
    Color {
        a: finite(c.a * a).clamp(0.0, 1.0),
        ..c
    }
}

/// `with_alpha(c, a)`: `c` with its alpha set to `a`, clamped to `0..=1` as
/// `QColor::setAlphaF` and CSS's relative colours clamp it; `alpha`
/// multiplies instead, the two agreeing only on an opaque `c`.
pub(super) fn with_alpha(c: Color, a: f32) -> Color {
    Color {
        a: finite(a).clamp(0.0, 1.0),
        ..c
    }
}

/// `mix(a, b, t)`: CSS `color-mix(in srgb, a (1−t)·100%, b t·100%)`, `t`
/// clamped to `0..=1`.
///
/// The two percentages sum to 100%, so color-mix's alpha multiplier is one
/// and it interpolates `a` and `b` with progress `t` (CSS Color 5,
/// "Calculating the Result of color-mix"), in premultiplied alpha (CSS Color
/// 4, "Interpolating with Alpha"): each colour's channels multiplied by its
/// alpha, channels and alpha interpolated linearly, and the channels divided
/// by the interpolated alpha, or kept premultiplied where that alpha is zero.
pub(super) fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = finite(t).clamp(0.0, 1.0);
    let s = 1.0 - t;
    let alpha = a.a * s + b.a * t;
    let channel = |x: f32, y: f32| {
        let premultiplied = x * a.a * s + y * b.a * t;
        finite(if alpha > 0.0 {
            premultiplied / alpha
        } else {
            premultiplied
        })
    };
    Color {
        r: channel(a.r, b.r),
        g: channel(a.g, b.g),
        b: channel(a.b, b.b),
        a: finite(alpha),
    }
}

/// `add`, `sub`, `min`, `max`, `mul` and `div` over two magnitudes in one
/// unit; a division by zero is not finite, so zero.
pub(super) fn binary(op: BinOp, x: f32, y: f32) -> f32 {
    finite(match op {
        BinOp::Add => x + y,
        BinOp::Sub => x - y,
        BinOp::Min => x.min(y),
        BinOp::Max => x.max(y),
        BinOp::Mul => x * y,
        BinOp::Div => x / y,
    })
}

/// `floor(x)`: toward negative infinity.
pub(super) fn floor(x: f32) -> f32 {
    finite(x.floor())
}

/// `round(x)`: to the nearest integer, a half away from zero, as Qt's
/// `qRound` and `f32::round` do (S2.2).
pub(super) fn round(x: f32) -> f32 {
    finite(x.round())
}

/// `round_pt(x)`: the font size `x`, in px, rounded to whole points by
/// `dpi` and converted back, `round(x · 72 / dpi) · dpi / 72`, computed in
/// f64; a point size within a relative 2^-21 of a half point is taken as
/// that half, so a size stored with six significant digits rounds as Qt's
/// `qRound` rounds the stored point size (S2.2). A `dpi` that is not
/// finite and above zero gives zero (S2.2's *Totality*).
pub(super) fn round_pt(x: f32, dpi: f32) -> f32 {
    let (x, dpi) = (f64::from(x), f64::from(dpi));
    let points = x * 72.0 / dpi;
    let half = points.floor() + 0.5;
    let points = if (points - half).abs() <= HALF_POINT_TOLERANCE * half.abs() {
        half
    } else {
        points
    };
    finite((points.round() * dpi / 72.0) as f32)
}

/// `round_pt`'s relative tolerance around a half point, 2^-21 (S2.2).
const HALF_POINT_TOLERANCE: f64 = 1.0 / 2_097_152.0;

/// `pick(index, c0, c1, …)`: the colour at the floored index's Euclidean
/// remainder by the number of colours; `None` only where there is no colour,
/// which typing refuses.
pub(super) fn pick(index: f32, colors: &[Color]) -> Option<Color> {
    let count = colors.len() as f32;
    let position = finite(index).floor().rem_euclid(count);
    colors.get(position as usize).copied()
}

/// A field's number limited as its bound says.
pub(super) fn bound(x: f32, bound: Bound) -> f32 {
    let x = finite(x);
    match bound {
        Bound::Unbounded => x,
        Bound::NonNegative => x.max(0.0),
        Bound::Fraction => x.clamp(0.0, 1.0),
        Bound::Integer { max } => x.floor().clamp(0.0, max as f32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    fn assert_close(actual: Color, expected: Color) {
        let pairs = [
            (actual.r, expected.r),
            (actual.g, expected.g),
            (actual.b, expected.b),
            (actual.a, expected.a),
        ];
        assert!(
            pairs.iter().all(|(a, e)| (a - e).abs() < 1e-6),
            "{actual:?} != {expected:?}"
        );
    }

    #[test]
    fn mix_matches_color_mix_premultiplied() {
        // CSS Color 5's color-mix(in srgb, A (1−t)·100%, B t·100%): the percentages sum to
        // 100%, so the alpha multiplier is 1 and A and B are interpolated with progress t, in
        // premultiplied alpha (CSS Color 4 §13.4): premultiply, interpolate, divide by the
        // interpolated alpha (kept premultiplied where that alpha is 0).
        let half_red = rgba(1.0, 0.0, 0.0, 0.5);
        let blue = rgba(0.0, 0.0, 1.0, 1.0);
        // color-mix(in srgb, srgb(1 0 0 / 0.5) 75%, srgb(0 0 1) 25%):
        // premultiplied A = (0.5, 0, 0, 0.5), B = (0, 0, 1, 1);
        // interpolated = (0.375, 0, 0.25, 0.625); divided by 0.625 = (0.6, 0, 0.4).
        // (Straight alpha would give red 0.75, blue 0.25.)
        assert_close(mix(half_red, blue, 0.25), rgba(0.6, 0.0, 0.4, 0.625));
        // color-mix(in srgb, transparent 50%, srgb(0 0.5 1) 50%): A premultiplied is all
        // zero, interpolated = (0, 0.25, 0.5, 0.5), divided by 0.5 = (0, 0.5, 1): the
        // transparent end takes the other's colour, not black.
        let clear = rgba(0.0, 0.0, 0.0, 0.0);
        let sky = rgba(0.0, 0.5, 1.0, 1.0);
        assert_close(mix(clear, sky, 0.5), rgba(0.0, 0.5, 1.0, 0.5));
        // Both alphas 0: the interpolated alpha is 0, so the result is the premultiplied
        // value, all zero.
        let clear_red = rgba(1.0, 0.0, 0.0, 0.0);
        let clear_blue = rgba(0.0, 0.0, 1.0, 0.0);
        assert_close(mix(clear_red, clear_blue, 0.5), clear);
        // t = 0 is A, t = 1 is B; t outside 0..=1 is clamped first.
        assert_close(mix(half_red, blue, 0.0), half_red);
        assert_close(mix(half_red, blue, 1.0), blue);
        assert_close(mix(half_red, blue, 1.5), blue);
        assert_close(mix(half_red, blue, -1.0), half_red);
        assert_close(mix(half_red, blue, f32::NAN), half_red);
        // Extended sRGB channels are interpolated as they are, unclamped.
        let wide = rgba(1.25, -0.25, 0.0, 1.0);
        assert_close(
            mix(wide, rgba(0.25, 0.75, 0.0, 1.0), 0.5),
            rgba(0.75, 0.25, 0.0, 1.0),
        );
    }

    /// S2.2's unit tests: a point size reaches `round_pt` converted to px
    /// in single precision (S9.4), and rounds as `qRound` on the stored
    /// point size.
    #[test]
    fn round_pt_rounds_whole_points() {
        let px = |points: f32, dpi: f32| points * dpi / 72.0;
        let close = |actual: f32, expected: f32| (actual - expected).abs() < 1e-4;
        let half = round_pt(px(10.5, 97.0), 97.0);
        assert!(close(half, px(11.0, 97.0)), "{half}");
        let below = round_pt(px(10.4999, 96.0), 96.0);
        assert!(close(below, px(10.0, 96.0)), "{below}");
        // The half point lies within the tolerance only: the single
        // precision conversion leaves it below 10.5 points at 97 dpi.
        let stored = f64::from(px(10.5, 97.0)) * 72.0 / 97.0;
        assert!(stored != 10.5, "{stored}");
        assert!(close(round_pt(px(12.0, 96.0), 96.0), 16.0));
        assert_eq!(round_pt(px(10.0, 96.0), 0.0), 0.0);
        assert_eq!(round_pt(px(10.0, 96.0), f32::NAN), 0.0);
    }

    #[test]
    fn alpha_multiplies_and_clamps() {
        let c = rgba(0.2, 0.4, 0.6, 0.8);
        assert_close(alpha(c, 0.5), rgba(0.2, 0.4, 0.6, 0.4));
        assert_close(alpha(c, 1.5), rgba(0.2, 0.4, 0.6, 1.0));
        assert_close(alpha(c, -1.0), rgba(0.2, 0.4, 0.6, 0.0));
        assert_close(alpha(c, f32::INFINITY), rgba(0.2, 0.4, 0.6, 0.0));
    }

    #[test]
    fn with_alpha_sets_and_clamps() {
        let c = rgba(0.2, 0.4, 0.6, 0.5);
        // Set, not multiplied: `alpha` gives 0.4 here.
        assert_close(with_alpha(c, 0.8), rgba(0.2, 0.4, 0.6, 0.8));
        assert_close(alpha(c, 0.8), rgba(0.2, 0.4, 0.6, 0.4));
        // The two agree on an opaque colour.
        let opaque = rgba(0.2, 0.4, 0.6, 1.0);
        assert_close(with_alpha(opaque, 0.8), alpha(opaque, 0.8));
        assert_close(with_alpha(c, 1.5), rgba(0.2, 0.4, 0.6, 1.0));
        assert_close(with_alpha(c, -1.0), rgba(0.2, 0.4, 0.6, 0.0));
        assert_close(with_alpha(c, f32::NAN), rgba(0.2, 0.4, 0.6, 0.0));
    }

    #[test]
    fn pick_euclidean() {
        let colors = [
            rgba(1.0, 0.0, 0.0, 1.0),
            rgba(0.0, 1.0, 0.0, 1.0),
            rgba(0.0, 0.0, 1.0, 1.0),
        ];
        let [c0, c1, c2] = colors;
        for (index, expected) in [
            (0.0, c0),
            (1.0, c1),
            (2.0, c2),
            (3.0, c0),
            (7.0, c1),
            (-1.0, c2),
            (-3.0, c0),
            (-4.0, c2),
            (2.7, c2),
            (-0.5, c2),
            (f32::NAN, c0),
            (f32::NEG_INFINITY, c0),
        ] {
            assert_eq!(pick(index, &colors), Some(expected), "{index}");
        }
        assert_eq!(pick(5.0, &[c1]), Some(c1));
        assert_eq!(pick(0.0, &[]), None);
    }

    #[test]
    fn binary_and_floor() {
        assert_eq!(binary(BinOp::Add, 1.5, 2.0), 3.5);
        assert_eq!(binary(BinOp::Sub, 1.5, 2.0), -0.5);
        assert_eq!(binary(BinOp::Min, 1.5, 2.0), 1.5);
        assert_eq!(binary(BinOp::Max, 1.5, 2.0), 2.0);
        assert_eq!(binary(BinOp::Mul, 1.5, 2.0), 3.0);
        assert_eq!(binary(BinOp::Div, 3.0, 2.0), 1.5);
        assert_eq!(floor(-1.5), -2.0);
        assert_eq!(floor(2.9), 2.0);
        assert_eq!(round(2.5), 3.0);
        assert_eq!(round(-2.5), -3.0);
        assert_eq!(round(2.4), 2.0);
        assert_eq!(round(-1.5), -2.0);
    }

    #[test]
    fn non_finite_is_zero() {
        assert_eq!(binary(BinOp::Div, 1.0, 0.0), 0.0);
        assert_eq!(binary(BinOp::Div, 0.0, 0.0), 0.0);
        assert_eq!(binary(BinOp::Mul, f32::MAX, f32::MAX), 0.0);
        assert_eq!(binary(BinOp::Add, f32::NAN, 1.0), 0.0);
        assert_eq!(floor(f32::INFINITY), 0.0);
        assert_eq!(round(f32::NAN), 0.0);
        assert_eq!(finite(f32::NAN), 0.0);
        assert_eq!(
            finite_color(rgba(f32::NAN, f32::INFINITY, 0.5, f32::NEG_INFINITY)),
            rgba(0.0, 0.0, 0.5, 0.0)
        );
        assert_eq!(bound(f32::NAN, Bound::Unbounded), 0.0);
    }

    #[test]
    fn fraction_clamped() {
        assert_eq!(bound(1.5, Bound::Fraction), 1.0);
        assert_eq!(bound(-0.25, Bound::Fraction), 0.0);
        assert_eq!(bound(0.25, Bound::Fraction), 0.25);
        assert_eq!(bound(f32::INFINITY, Bound::Fraction), 0.0);
    }

    #[test]
    fn bounds_zero_negatives_and_floor_integers() {
        assert_eq!(bound(-3.0, Bound::NonNegative), 0.0);
        assert_eq!(bound(3.0, Bound::NonNegative), 3.0);
        assert_eq!(bound(-3.0, Bound::Unbounded), -3.0);
        assert_eq!(bound(400.9, Bound::Integer { max: 1000 }), 400.0);
        assert_eq!(bound(-1.0, Bound::Integer { max: 1000 }), 0.0);
        assert_eq!(
            bound(
                70000.0,
                Bound::Integer {
                    max: u32::from(u16::MAX)
                }
            ),
            65535.0
        );
    }
}
