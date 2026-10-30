//! Qt's `QColor` as the Qt functions compute with it: four 16-bit channels in
//! RGB or HSV, converted and rounded as `qtbase@6.12.0
//! src/gui/painting/qcolor.cpp` does, so a port that rounds otherwise between
//! steps does not give other colours than Qt.

use crate::next::color::Color;

/// `qRound(x)` for a float as Qt computes it on x86-64 (`x + copysign(0.5,
/// x)` truncated, `qtbase@6.12.0 src/corelib/global/qnumeric.h:540-544`),
/// held in the 16 bits a `QColor` component takes. Every caller passes a
/// value in `0.0..=65535.0`; the clamp keeps any other in the 16 bits.
fn q_round(x: f32) -> u16 {
    (x.clamp(0.0, 65535.0) + 0.5) as u16
}

/// `qfloat16(x)` read back as a float: `x` rounded to the nearest IEEE 754
/// half-precision value, ties to even, as an `ExtendedRgb` colour stores its
/// components (`qcolor.cpp:2611-2615`).
fn half(x: f32) -> f32 {
    if !x.is_finite() {
        return x;
    }
    let magnitude = x.abs();
    // Half precision's smallest normal is 2^-14, its spacing there 2^-24,
    // and above it 2^(e - 10) for a magnitude in 2^e..2^(e + 1).
    let biased = (magnitude.to_bits() >> 23) & 0xff;
    let quantum = if biased < 113 {
        f32::from_bits(103 << 23)
    } else {
        f32::from_bits(biased.saturating_sub(10).checked_shl(23).unwrap_or(0))
    };
    let rounded = (magnitude / quantum).round_ties_even() * quantum;
    let rounded = if rounded > 65504.0 {
        f32::INFINITY
    } else {
        rounded
    };
    rounded.copysign(x)
}

/// A colour as `QColor` stores it in its `Rgb` spec: four 16-bit channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Rgb16 {
    pub(super) r: u16,
    pub(super) g: u16,
    pub(super) b: u16,
    pub(super) a: u16,
}

/// A colour as `QColor` stores it in its `Hsv` spec: the hue in hundredths
/// of a degree, `u16::MAX` where it is undefined (a grey), the rest in 16
/// bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Hsv16 {
    pub(super) hue: u16,
    pub(super) saturation: u16,
    pub(super) value: u16,
    pub(super) alpha: u16,
}

/// A channel over 65535 as a float: `redF()` and the others
/// (`qcolor.cpp:1659-1664`, `:1777-1785`).
pub(super) fn unit(channel: u16) -> f32 {
    f32::from(channel) / 65535.0
}

impl Rgb16 {
    /// `QColor::fromRgbF(r, g, b, a)` (`qcolor.cpp:2601-2626`) followed by
    /// `toRgb()` (`:2346-2351`): a colour whose channels lie in `0..=1`
    /// is the `Rgb` spec, each channel `qRound(x * 65535)`; one with a
    /// channel outside it is `ExtendedRgb`, its channels held as half
    /// floats, which `toRgb` clamps to `0..=1` before rounding. The model's
    /// alpha is in `0..=1` (S2.1), Qt's valid range; a non-finite channel is
    /// zero, as S2.2's totality reads it.
    pub(super) fn from_color(c: Color) -> Rgb16 {
        let finite = |x: f32| if x.is_finite() { x } else { 0.0 };
        let (r, g, b) = (finite(c.r), finite(c.g), finite(c.b));
        let a = finite(c.a).clamp(0.0, 1.0);
        let in_range = |x: f32| (0.0..=1.0).contains(&x);
        if in_range(r) && in_range(g) && in_range(b) {
            let channel = |x: f32| q_round(x * 65535.0);
            return Rgb16 {
                r: channel(r),
                g: channel(g),
                b: channel(b),
                a: channel(a),
            };
        }
        let channel = |x: f32| q_round(65535.0 * half(x).clamp(0.0, 1.0));
        Rgb16 {
            r: channel(r),
            g: channel(g),
            b: channel(b),
            a: q_round(65535.0 * half(a)),
        }
    }

    /// The colour's channels over 65535 (`redF()` and the others).
    pub(super) fn to_color(self) -> Color {
        Color {
            r: unit(self.r),
            g: unit(self.g),
            b: unit(self.b),
            a: unit(self.a),
        }
    }

    /// `QColor::toHsv` (`qcolor.cpp:2364-2405`).
    pub(super) fn to_hsv(self) -> Hsv16 {
        let (r, g, b) = (self.r, self.g, self.b);
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        if max == min {
            return Hsv16 {
                hue: u16::MAX,
                saturation: 0,
                value: max,
                alpha: self.a,
            };
        }
        let chroma = f32::from(max.saturating_sub(min));
        let (rf, gf, bf) = (f32::from(r), f32::from(g), f32::from(b));
        let hue = if max == r {
            let hue = (gf - bf) / chroma;
            if hue < 0.0 { hue + 6.0 } else { hue }
        } else if max == g {
            2.0 + (bf - rf) / chroma
        } else {
            4.0 + (rf - gf) / chroma
        };
        Hsv16 {
            hue: q_round(hue * 6000.0),
            saturation: q_round((chroma / f32::from(max)) * 65535.0),
            value: max,
            alpha: self.a,
        }
    }
}

impl Hsv16 {
    /// `QColor::fromHsvF(h, s, v, a)` (`qcolor.cpp:2697-2710`) for a hue in
    /// `0..=1` and the rest in `0..=1`, the ranges its callers here clamp
    /// them to: each `qRound(x * 65535)`, the hue `qRound(h * 36000)`.
    pub(super) fn from_hsv_f(hue: f32, saturation: f32, value: f32, alpha: f32) -> Hsv16 {
        let channel = |x: f32| q_round(x.clamp(0.0, 1.0) * 65535.0);
        Hsv16 {
            hue: q_round(hue.clamp(0.0, 1.0) * 36000.0),
            saturation: channel(saturation),
            value: channel(value),
            alpha: channel(alpha),
        }
    }

    /// `hueF()`: -1 where the hue is undefined (`qcolor.cpp:1962-1967`).
    pub(super) fn hue_f(self) -> f32 {
        if self.hue == u16::MAX {
            -1.0
        } else {
            f32::from(self.hue) / 36000.0
        }
    }

    /// `QColor::toRgb` from HSV (`qcolor.cpp:2216-2280`). A hue from
    /// `from_hsv_f` or `to_hsv` is at most 36000, Qt's 0, so the sector is
    /// `0..=5`.
    ///
    /// `1.0f - (s * f)` rounds the product, then the difference, as the
    /// pinned build computes it (`sources.toml`'s Arch build for x86-64, a
    /// target with no fused multiply-add); a build that fuses them gives
    /// another 16-bit channel on rare inputs, which the vector table holds
    /// (`tools/kde-vectors`).
    pub(super) fn to_rgb(self) -> Rgb16 {
        let alpha = self.alpha;
        if self.saturation == 0 || self.hue == u16::MAX {
            let v = self.value;
            return Rgb16 {
                r: v,
                g: v,
                b: v,
                a: alpha,
            };
        }
        let h = if self.hue == 36000 {
            0.0
        } else {
            f32::from(self.hue) / 6000.0
        };
        let s = unit(self.saturation);
        let v = unit(self.value);
        // `h` is below 6: the hue is below 36000 here.
        let sector = h as u8;
        let f = h - f32::from(sector);
        let p = v * (1.0 - s);
        let (r, g, b) = if sector & 1 == 1 {
            let q = v * (1.0 - s * f);
            match sector {
                1 => (q, v, p),
                3 => (p, q, v),
                _ => (v, p, q),
            }
        } else {
            let t = v * (1.0 - s * (1.0 - f));
            match sector {
                0 => (v, t, p),
                2 => (p, v, t),
                _ => (t, p, v),
            }
        };
        let channel = |x: f32| q_round(x * 65535.0);
        Rgb16 {
            r: channel(r),
            g: channel(g),
            b: channel(b),
            a: alpha,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_rounds_to_binary16() {
        // 0.4 lies between the halves 0.39990234375 and 0.400146484375.
        assert_eq!(half(0.4), 0.399_902_34);
        assert_eq!(half(1.25), 1.25);
        assert_eq!(half(-0.25), -0.25);
        // Below the smallest normal half, spacing 2^-24.
        assert_eq!(half(3.0e-8), 2.0_f32.powi(-24));
        assert_eq!(half(1.0e6), f32::INFINITY);
    }

    #[test]
    fn rgb_and_extended_rgb_storage() {
        let c = Color {
            r: 0.4,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        // qRound(0.4 * 65535) = qRound(26214.0).
        assert_eq!(
            Rgb16::from_color(c),
            Rgb16 {
                r: 26214,
                g: 0,
                b: 65535,
                a: 65535
            }
        );
        // A channel outside 0..=1: every channel a half float, then clamped:
        // qRound(65535 * 0.39990234375) = qRound(26207.6).
        let wide = Color { r: 1.25, ..c };
        assert_eq!(
            Rgb16::from_color(Color { g: 0.4, ..wide }),
            Rgb16 {
                r: 65535,
                g: 26208,
                b: 65535,
                a: 65535
            }
        );
    }

    #[test]
    fn hsv_round_trip_of_primaries() {
        let red = Rgb16 {
            r: 65535,
            g: 0,
            b: 0,
            a: 65535,
        };
        let hsv = red.to_hsv();
        assert_eq!((hsv.hue, hsv.saturation, hsv.value), (0, 65535, 65535));
        assert_eq!(hsv.to_rgb(), red);
        let grey = Rgb16 {
            r: 32896,
            g: 32896,
            b: 32896,
            a: 65535,
        };
        assert_eq!(grey.to_hsv().hue_f(), -1.0);
        assert_eq!(grey.to_hsv().to_rgb(), grey);
    }
}
