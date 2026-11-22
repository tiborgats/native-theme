//! Colours in gamma-encoded extended sRGB, their literal forms and their
//! conversions.
//!
//! Sources of the formulas:
//! - the sRGB transfer function, extended symmetrically to negative values,
//!   and the sRGB and Display P3 matrices: CSS Color Module Level 4, §19
//!   "Sample code for Color Conversions", its `conversions.js`
//!   (<https://drafts.csswg.org/css-color-4/conversions.js>, read
//!   2026-11-22, last modified 2026-11-20): `lin_sRGB`, `gam_sRGB`,
//!   `lin_P3_to_XYZ`, `XYZ_to_lin_sRGB`, `lin_sRGB_to_XYZ`, `XYZ_to_lin_P3`;
//!   Display P3 uses the sRGB transfer function (`lin_P3`, `gam_P3`);
//! - relative luminance and contrast ratio: WCAG 2.1 (W3C Recommendation,
//!   2025-05-06 edition), §6 Glossary, "relative luminance" (note 1, with the
//!   0.04045 threshold of note 2) and "contrast ratio";
//! - source-over: Porter and Duff, "Compositing Digital Images" (SIGGRAPH
//!   1984), the `over` operator, as W3C Compositing and Blending Level 1,
//!   §9.1.4 "Source Over", writes it on premultiplied values:
//!   `co = cs + cb × (1 − αs)`, `αo = αs + αb × (1 − αs)`;
//! - `color-mix(in srgb)`: CSS Color Module Level 5, §3.3 "Calculating the
//!   Result of color-mix" (progress = p2 / (p1 + p2)), interpolated as CSS
//!   Color Module Level 4, §13.5 "Interpolating with Alpha" says: premultiply,
//!   interpolate, un-premultiply, "if the interpolated alpha value is zero
//!   ..., the un-premultiplied value is the premultiplied value".

use std::fmt;
use std::str::FromStr;

use super::error::Error;

/// Gamma-encoded extended-sRGB color (matching AppKit/UIKit `extendedSRGB` and the CSS sRGB
/// transfer function extended symmetrically beyond `0.0..=1.0`).
/// Channels may lie outside `0.0..=1.0` to represent Display P3 losslessly; `a` is in `0.0..=1.0`.
///
/// Its text forms, read by [`FromStr`] and written by [`Display`](fmt::Display):
/// `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`, `srgb(r, g, b, a)` and
/// `p3(r, g, b, a)`, the last a Display P3 colour converted to extended sRGB
/// when read. `Display` writes `#rrggbb` (opaque) or `#rrggbbaa` where every
/// channel is an exact 8-bit sRGB value in `0.0..=1.0`, and the lossless
/// `srgb(r, g, b, a)` otherwise.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    /// Red, gamma-encoded.
    pub r: f32,
    /// Green, gamma-encoded.
    pub g: f32,
    /// Blue, gamma-encoded.
    pub b: f32,
    /// Alpha, straight (not premultiplied), `0.0..=1.0`.
    pub a: f32,
}

/// The scale of an 8-bit channel, 255: an 8-bit value `v` encodes `v / 255`
/// (WCAG 2.1, §6 Glossary, "relative luminance", note 1: `RsRGB = R8bit/255`).
const SRGB8_SCALE: f32 = u8::MAX as f32;

/// `lin_sRGB` (CSS Color 4, §19): at or below this encoded magnitude the
/// transfer function is linear.
const SRGB_LINEAR_THRESHOLD: f64 = 0.04045;
/// `lin_sRGB` / `gam_sRGB` (CSS Color 4, §19): the slope of the linear segment.
const SRGB_LINEAR_SLOPE: f64 = 12.92;
/// `lin_sRGB` / `gam_sRGB` (CSS Color 4, §19): the offset of the power segment.
const SRGB_POWER_OFFSET: f64 = 0.055;
/// `lin_sRGB` / `gam_sRGB` (CSS Color 4, §19): the scale of the power segment.
const SRGB_POWER_SCALE: f64 = 1.055;
/// `lin_sRGB` / `gam_sRGB` (CSS Color 4, §19): the exponent of the power segment.
const SRGB_GAMMA: f64 = 2.4;
/// `gam_sRGB` (CSS Color 4, §19): at or below this linear magnitude the
/// inverse transfer function is linear.
const SRGB_GAMMA_THRESHOLD: f64 = 0.0031308;

/// Linear-light Display P3 to CIE XYZ, D65 (`lin_P3_to_XYZ`, CSS Color 4, §19),
/// in the source's rational form.
const LIN_P3_TO_XYZ: [[f64; 3]; 3] = [
    [
        608311.0 / 1250200.0,
        189793.0 / 714400.0,
        198249.0 / 1000160.0,
    ],
    [
        35783.0 / 156275.0,
        247089.0 / 357200.0,
        198249.0 / 2500400.0,
    ],
    [0.0 / 1.0, 32229.0 / 714400.0, 5220557.0 / 5000800.0],
];
/// CIE XYZ to linear-light sRGB, D65 (`XYZ_to_lin_sRGB`, CSS Color 4, §19).
const XYZ_TO_LIN_SRGB: [[f64; 3]; 3] = [
    [12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
    [
        -851781.0 / 878810.0,
        1648619.0 / 878810.0,
        36519.0 / 878810.0,
    ],
    [705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0],
];
/// Linear-light sRGB to CIE XYZ, D65 (`lin_sRGB_to_XYZ`, CSS Color 4, §19).
const LIN_SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [506752.0 / 1228815.0, 87881.0 / 245763.0, 12673.0 / 70218.0],
    [87098.0 / 409605.0, 175762.0 / 245763.0, 12673.0 / 175545.0],
    [7918.0 / 409605.0, 87881.0 / 737289.0, 1001167.0 / 1053270.0],
];
/// CIE XYZ to linear-light Display P3, D65 (`XYZ_to_lin_P3`, CSS Color 4, §19).
const XYZ_TO_LIN_P3: [[f64; 3]; 3] = [
    [
        446124.0 / 178915.0,
        -333277.0 / 357830.0,
        -72051.0 / 178915.0,
    ],
    [-14852.0 / 17905.0, 63121.0 / 35810.0, 423.0 / 17905.0],
    [11844.0 / 330415.0, -50337.0 / 660830.0, 316169.0 / 330415.0],
];

/// The red coefficient of relative luminance (WCAG 2.1, §6 Glossary,
/// "relative luminance", note 1).
const WCAG_LUMINANCE_R: f64 = 0.2126;
/// The green coefficient of relative luminance (WCAG 2.1, as above).
const WCAG_LUMINANCE_G: f64 = 0.7152;
/// The blue coefficient of relative luminance (WCAG 2.1, as above).
const WCAG_LUMINANCE_B: f64 = 0.0722;
/// The term added to both luminances of the contrast ratio,
/// `(L1 + 0.05) / (L2 + 0.05)` (WCAG 2.1, §6 Glossary, "contrast ratio").
const WCAG_CONTRAST_OFFSET: f64 = 0.05;
/// The smallest contrast ratio, that of a colour with itself (WCAG 2.1, §6
/// Glossary, "contrast ratio", note 1: ratios range from 1 to 21).
const WCAG_CONTRAST_MIN: f32 = 1.0;

impl Color {
    /// Fully transparent black.
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    /// Opaque black.
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// Opaque white.
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// The colour of these channels, taken as they are.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// The opaque colour of these channels.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// The colour of 8-bit sRGB channels, each `v / 255`.
    #[must_use]
    pub fn from_srgb8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let unit = |v: u8| f32::from(v) / SRGB8_SCALE;
        Self {
            r: unit(r),
            g: unit(g),
            b: unit(b),
            a: unit(a),
        }
    }

    /// The colour of a hex literal: `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa`,
    /// the `#` required, digits in either case.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidColor`] for any other text.
    pub fn from_hex(hex: &str) -> Result<Self, Error> {
        parse_hex(hex).ok_or_else(|| Error::InvalidColor {
            input: hex.to_owned(),
        })
    }

    /// The colour a gamma-encoded Display P3 value states, in extended sRGB;
    /// `a` is taken as it is.
    #[must_use]
    pub fn from_display_p3(r: f32, g: f32, b: f32, a: f32) -> Self {
        let p3 = [r, g, b].map(|c| srgb_to_linear(f64::from(c)));
        let [r, g, b] = multiply(&XYZ_TO_LIN_SRGB, multiply(&LIN_P3_TO_XYZ, p3))
            .map(|c| linear_to_srgb(c) as f32);
        Self { r, g, b, a }
    }

    /// This colour with alpha `a`, clamped to `0.0..=1.0`; a non-finite `a`
    /// gives 0.0.
    #[must_use]
    pub fn with_alpha(self, a: f32) -> Self {
        Self {
            a: clamp_unit(a),
            ..self
        }
    }

    /// This colour with its alpha multiplied by `factor`, the product
    /// clamped to `0.0..=1.0`; a non-finite product gives 0.0.
    #[must_use]
    pub fn mul_alpha(self, factor: f32) -> Self {
        Self {
            a: clamp_unit(self.a * factor),
            ..self
        }
    }

    /// This colour with every channel clamped to `0.0..=1.0`; a non-finite
    /// channel gives 0.0.
    #[must_use]
    pub fn clamped(self) -> Self {
        Self {
            r: clamp_unit(self.r),
            g: clamp_unit(self.g),
            b: clamp_unit(self.b),
            a: clamp_unit(self.a),
        }
    }

    /// `[r, g, b, a]` in 8-bit sRGB: [`clamped`](Self::clamped), then each
    /// channel times 255, rounded.
    #[must_use]
    pub fn to_srgb8(self) -> [u8; 4] {
        let c = self.clamped();
        [c.r, c.g, c.b, c.a].map(to_byte)
    }

    /// `[r, g, b, a]` in 8-bit premultiplied sRGB: the alpha clamped to
    /// `0.0..=1.0`, each colour channel multiplied by it and then clamped
    /// (a non-finite value gives 0.0), each times 255, rounded.
    #[must_use]
    pub fn to_premultiplied_srgb8(self) -> [u8; 4] {
        let a = clamp_unit(self.a);
        [
            clamp_unit(self.r * a),
            clamp_unit(self.g * a),
            clamp_unit(self.b * a),
            a,
        ]
        .map(to_byte)
    }

    /// `[r, g, b, a]` in Display P3, gamma-encoded and not clamped.
    #[must_use]
    pub fn to_display_p3(self) -> [f32; 4] {
        let srgb = [self.r, self.g, self.b].map(|c| srgb_to_linear(f64::from(c)));
        let [r, g, b] = multiply(&XYZ_TO_LIN_P3, multiply(&LIN_SRGB_TO_XYZ, srgb))
            .map(|c| linear_to_srgb(c) as f32);
        [r, g, b, self.a]
    }

    /// `self` composited over `backdrop` by Porter-Duff source-over, in
    /// gamma-encoded sRGB on premultiplied values (the module's sources); a
    /// result of alpha 0 is [`Color::TRANSPARENT`]. A colour of weight zero
    /// contributes nothing, whatever its channels hold.
    #[must_use]
    pub fn composite_over(self, backdrop: Self) -> Self {
        let back = backdrop.a * (1.0 - self.a);
        let a = self.a + back;
        if a.is_nan() || a <= 0.0 {
            return Self::TRANSPARENT;
        }
        let channel = |s: f32, b: f32| (weighted(s, self.a) + weighted(b, back)) / a;
        Self {
            r: channel(self.r, backdrop.r),
            g: channel(self.g, backdrop.g),
            b: channel(self.b, backdrop.b),
            a,
        }
    }

    /// CSS `color-mix(in srgb, self (1 − t), other t)` on premultiplied
    /// values (the module's sources): `t = 0.0` gives `self`, `t = 1.0`
    /// gives `other`. `t` is clamped to `0.0..=1.0`; a non-finite `t` gives
    /// 0.0. A colour of weight zero contributes nothing, whatever its
    /// channels hold.
    #[must_use]
    pub fn mix_srgb(self, other: Self, t: f32) -> Self {
        let t = clamp_unit(t);
        let (ws, wo) = (self.a * (1.0 - t), other.a * t);
        let a = ws + wo;
        let channel = |x: f32, y: f32| {
            let premultiplied = weighted(x, ws) + weighted(y, wo);
            if a == 0.0 {
                premultiplied
            } else {
                premultiplied / a
            }
        };
        Self {
            r: channel(self.r, other.r),
            g: channel(self.g, other.g),
            b: channel(self.b, other.b),
            a,
        }
    }

    /// WCAG 2.1 relative luminance of the colour channels (alpha is not
    /// used), through the extended transfer function, clamped to at least
    /// 0.0 (a result that is not finite as an `f32` gives 0.0).
    #[must_use]
    pub fn relative_luminance(self) -> f32 {
        let l = (WCAG_LUMINANCE_R * srgb_to_linear(f64::from(self.r))
            + WCAG_LUMINANCE_G * srgb_to_linear(f64::from(self.g))
            + WCAG_LUMINANCE_B * srgb_to_linear(f64::from(self.b))) as f32;
        if l.is_finite() { l.max(0.0) } else { 0.0 }
    }

    /// WCAG 2.1 contrast ratio of the two colours' relative luminances,
    /// `(L1 + 0.05) / (L2 + 0.05)` with `L1` the lighter; at least 1.0.
    #[must_use]
    pub fn contrast_ratio(self, other: Self) -> f32 {
        let (x, y) = (
            f64::from(self.relative_luminance()),
            f64::from(other.relative_luminance()),
        );
        let ratio = (x.max(y) + WCAG_CONTRAST_OFFSET) / (x.min(y) + WCAG_CONTRAST_OFFSET);
        (ratio as f32).max(WCAG_CONTRAST_MIN)
    }

    /// Whether every channel is an exact 8-bit sRGB value in `0.0..=1.0`.
    fn is_exact_srgb8(self) -> bool {
        let [r, g, b, a] = self.to_srgb8();
        Self::from_srgb8(r, g, b, a) == self
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_exact_srgb8() {
            let [r, g, b, a] = self.to_srgb8();
            if a == u8::MAX {
                write!(f, "#{r:02x}{g:02x}{b:02x}")
            } else {
                write!(f, "#{r:02x}{g:02x}{b:02x}{a:02x}")
            }
        } else {
            write!(f, "srgb({}, {}, {}, {})", self.r, self.g, self.b, self.a)
        }
    }
}

impl FromStr for Color {
    type Err = Error;

    /// Reads `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`, `srgb(r, g, b, a)` or
    /// `p3(r, g, b, a)`, whitespace allowed around the whole, the name, the
    /// parentheses and the commas; every number finite and the alpha in
    /// `0.0..=1.0`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).ok_or_else(|| Error::InvalidColor {
            input: s.to_owned(),
        })
    }
}

/// The colour of any text form, or `None`.
fn parse(text: &str) -> Option<Color> {
    let text = text.trim();
    if text.starts_with('#') {
        return parse_hex(text);
    }
    let (name, rest) = text.split_once('(')?;
    let args = rest.trim_end().strip_suffix(')')?;
    let numbers = args
        .split(',')
        .map(|n| n.trim().parse::<f32>().ok().filter(|v| v.is_finite()))
        .collect::<Option<Vec<f32>>>()?;
    let [r, g, b, a] = numbers.as_slice() else {
        return None;
    };
    if !(0.0..=1.0).contains(a) {
        return None;
    }
    match name.trim() {
        "srgb" => Some(Color::new(*r, *g, *b, *a)),
        "p3" => Some(Color::from_display_p3(*r, *g, *b, *a))
            .filter(|c| c.r.is_finite() && c.g.is_finite() && c.b.is_finite()),
        _ => None,
    }
}

/// `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa`, or `None`.
fn parse_hex(text: &str) -> Option<Color> {
    let digits = text.strip_prefix('#')?.as_bytes();
    if !digits.iter().all(u8::is_ascii_hexdigit) {
        return None;
    }
    let bytes = match digits.len() {
        3 | 4 => digits
            .iter()
            .map(|&d| byte_of_pair([d, d]))
            .collect::<Option<Vec<u8>>>()?,
        6 | 8 => {
            let (pairs, rest) = digits.as_chunks::<2>();
            if !rest.is_empty() {
                return None;
            }
            pairs
                .iter()
                .map(|&pair| byte_of_pair(pair))
                .collect::<Option<Vec<u8>>>()?
        }
        _ => return None,
    };
    match bytes.as_slice() {
        [r, g, b] => Some(Color::from_srgb8(*r, *g, *b, u8::MAX)),
        [r, g, b, a] => Some(Color::from_srgb8(*r, *g, *b, *a)),
        _ => None,
    }
}

/// Two hex digits as a byte.
fn byte_of_pair(pair: [u8; 2]) -> Option<u8> {
    std::str::from_utf8(&pair)
        .ok()
        .and_then(|s| u8::from_str_radix(s, 16).ok())
}

/// `v` clamped to `0.0..=1.0`; a non-finite `v` gives 0.0.
fn clamp_unit(v: f32) -> f32 {
    if v.is_finite() {
        v.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// `channel × weight`, or 0.0 where the weight is zero: a term of weight zero
/// contributes nothing, so a channel that is not finite cannot make NaN.
fn weighted(channel: f32, weight: f32) -> f32 {
    if weight == 0.0 { 0.0 } else { channel * weight }
}

/// A channel in `0.0..=1.0` times 255, rounded.
fn to_byte(v: f32) -> u8 {
    (v * SRGB8_SCALE).round() as u8
}

fn multiply(matrix: &[[f64; 3]; 3], [x, y, z]: [f64; 3]) -> [f64; 3] {
    matrix.map(|[a, b, c]| a * x + b * y + c * z)
}

/// `lin_sRGB` (CSS Color 4, §19): gamma-encoded to linear light, reflected
/// for negative values.
fn srgb_to_linear(v: f64) -> f64 {
    let abs = v.abs();
    if abs <= SRGB_LINEAR_THRESHOLD {
        v / SRGB_LINEAR_SLOPE
    } else {
        ((abs + SRGB_POWER_OFFSET) / SRGB_POWER_SCALE)
            .powf(SRGB_GAMMA)
            .copysign(v)
    }
}

/// `gam_sRGB` (CSS Color 4, §19), the inverse of [`srgb_to_linear`].
fn linear_to_srgb(v: f64) -> f64 {
    let abs = v.abs();
    if abs > SRGB_GAMMA_THRESHOLD {
        (SRGB_POWER_SCALE * abs.powf(SRGB_GAMMA.recip()) - SRGB_POWER_OFFSET).copysign(v)
    } else {
        SRGB_LINEAR_SLOPE * v
    }
}

#[cfg(test)]
mod tests;
