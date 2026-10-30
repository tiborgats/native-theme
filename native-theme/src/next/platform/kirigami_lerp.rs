//! `kirigami_lerp(a, b, balance)` (S2.2): Kirigami's
//! `ColorUtils::linearInterpolation`, `kirigami@6.31.0
//! src/platform/colorutils.cpp:61-80`, over Qt's `QColor`.

use super::qcolor::{Hsv16, Rgb16, unit};
use crate::next::color::Color;

/// Kirigami's `ColorUtils::linearInterpolation(one, two, balance)`: hue,
/// saturation, value and alpha each interpolated linearly in `QColor`'s HSV,
/// the last three clamped to `0..=1`; a hue that is not above zero (QColor's
/// −1 for a colour without one, and pure red's 0) taken from the other
/// colour, else 0; the hue's `fmod` by 1 not wrapped further.
///
/// `balance` is clamped to `0..=1` (S2.2), which Kirigami does not do: outside
/// that range the hue can leave `0..1`, where `QColor::fromHsvF` returns an
/// invalid colour the model has no value for; S12 refuses a stated balance
/// outside it.
pub(crate) fn kirigami_lerp(a: Color, b: Color, balance: f32) -> Color {
    let balance = if balance.is_finite() {
        f64::from(balance.clamp(0.0, 1.0))
    } else {
        0.0
    };
    let one = Rgb16::from_color(a).to_hsv();
    let two = Rgb16::from_color(b).to_hsv();
    // `linearlyInterpolateDouble`, over doubles.
    let lerp = |from: f32, to: f32| {
        let from = f64::from(from);
        from + (f64::from(to) - from) * balance
    };
    // `std::max(one.hueF() > 0.0 ? one.hueF() : two.hueF(), 0.0f)`.
    let hue_of = |own: Hsv16, other: Hsv16| {
        let hue = if own.hue_f() > 0.0 {
            own.hue_f()
        } else {
            other.hue_f()
        };
        hue.max(0.0)
    };
    // `std::fmod(hue, 1.0)`: `rem_euclid` for the hue, which is not negative.
    let hue = lerp(hue_of(one, two), hue_of(two, one)).rem_euclid(1.0);
    let clamped = |from: u16, to: u16| lerp(unit(from), unit(to)).clamp(0.0, 1.0);
    let saturation = clamped(one.saturation, two.saturation);
    let value = clamped(one.value, two.value);
    let alpha = clamped(one.alpha, two.alpha);
    Hsv16::from_hsv_f(hue as f32, saturation as f32, value as f32, alpha as f32)
        .to_rgb()
        .to_color()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb8(r: u8, g: u8, b: u8) -> Color {
        let f = |c: u8| f32::from(c) / 255.0;
        Color {
            r: f(r),
            g: f(g),
            b: f(b),
            a: 1.0,
        }
    }

    fn sixteen(c: Color) -> [u16; 4] {
        let Rgb16 { r, g, b, a } = Rgb16::from_color(c);
        [r, g, b, a]
    }

    /// One row: two opaque 8-bit colours, the balance, and the 16-bit
    /// colour of the result.
    type Row = ([u8; 3], [u8; 3], f32, [u16; 4]);

    /// Rows `kirigami_lerp` gives as Qt's `QColor` gives them: today's KDE
    /// reader's vectors, which `scripts/generate_qt_color_vectors.py` made
    /// with Qt's own `QColor` (Breeze's frame colours, then the hue edges:
    /// a grey and pure red), each the 16-bit colour of the result.
    #[test]
    fn linear_interpolation_is_kirigamis() {
        let rows: [Row; 9] = [
            (
                [222, 224, 226],
                [35, 38, 41],
                0.2,
                [46464, 47518, 48573, 65535],
            ),
            (
                [239, 240, 241],
                [35, 38, 41],
                0.2,
                [49802, 50730, 51657, 65535],
            ),
            (
                [255, 255, 255],
                [35, 38, 41],
                0.2,
                [52939, 53737, 54535, 65535],
            ),
            (
                [41, 44, 48],
                [252, 252, 252],
                0.2,
                [20159, 21300, 22822, 65535],
            ),
            (
                [128, 128, 128],
                [61, 174, 233],
                0.2,
                [32639, 36354, 38293, 65535],
            ),
            (
                [61, 174, 233],
                [128, 128, 128],
                0.2,
                [22308, 43448, 54484, 65535],
            ),
            (
                [128, 128, 128],
                [200, 200, 200],
                0.5,
                [42148, 42148, 42148, 65535],
            ),
            (
                [255, 0, 0],
                [61, 174, 233],
                0.3,
                [5014, 43662, 63839, 65535],
            ),
            ([255, 0, 0], [0, 0, 0], 0.7, [19661, 13763, 13763, 65535]),
        ];
        for ([r1, g1, b1], [r2, g2, b2], balance, expected) in rows {
            let out = kirigami_lerp(rgb8(r1, g1, b1), rgb8(r2, g2, b2), balance);
            assert_eq!(sixteen(out), expected, "{r1},{g1},{b1} by {balance}");
        }
    }

    #[test]
    fn alpha_interpolated_and_balance_clamped() {
        let clear = Color {
            a: 0.0,
            ..rgb8(255, 0, 0)
        };
        let red = rgb8(255, 0, 0);
        // Alpha moves linearly: 0.25 of the way, qRound(0.25 * 65535).
        assert_eq!(sixteen(kirigami_lerp(clear, red, 0.25))[3], 16384);
        // A balance outside 0..=1 is clamped; a non-finite one is zero.
        assert_eq!(
            kirigami_lerp(clear, red, 1.5),
            kirigami_lerp(clear, red, 1.0)
        );
        assert_eq!(
            kirigami_lerp(clear, red, -1.0),
            kirigami_lerp(clear, red, 0.0)
        );
        assert_eq!(
            kirigami_lerp(clear, red, f32::NAN),
            kirigami_lerp(clear, red, 0.0)
        );
    }
}
