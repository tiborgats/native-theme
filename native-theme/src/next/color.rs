//! Colours in extended sRGB, their literal forms and conversions (S2.1).

/// Extended sRGB, straight (not premultiplied) alpha. Channels may lie outside
/// `0..=1` (Display P3 colours expressed in extended sRGB); alpha is `0..=1`.
///
/// Its literal forms, in TOML and in expressions: `#rrggbb`, `#rrggbbaa`,
/// `srgb(r, g, b[, a])` and `display_p3(r, g, b[, a])`, the last a colour as
/// it is published in Display P3, converted to extended sRGB when read.
///
/// ```
/// use native_theme::next::Color;
///
/// let accent = Color::parse("#3daee9").ok_or("not a colour")?;
/// assert_eq!(accent.to_srgb8_clamped(), [0x3d, 0xae, 0xe9, 0xff]);
/// let p3_red = Color::parse("display_p3(1, 0, 0)").ok_or("not a colour")?;
/// assert!(p3_red.r > 1.0 && p3_red.g < 0.0);
/// # Ok::<(), &str>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Red, gamma-encoded.
    pub r: f32,
    /// Green, gamma-encoded.
    pub g: f32,
    /// Blue, gamma-encoded.
    pub b: f32,
    /// Alpha, `0..=1`.
    pub a: f32,
}

// The Display P3 and sRGB conversions of CSS Color Module Level 4, section 19,
// "Sample code for Color Conversions" (https://drafts.csswg.org/css-color-4/#color-conversion-code,
// its conversions.js, Editor's Draft of 29 October 2026): `lin_P3_to_XYZ`,
// `XYZ_to_lin_sRGB`, `lin_sRGB_to_XYZ` and `XYZ_to_lin_P3`, D65 throughout, no
// chromatic adaptation; the matrices as the source writes them, in rational form.

/// Linear-light Display P3 to CIE XYZ (`lin_P3_to_XYZ`).
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

/// CIE XYZ to linear-light sRGB (`XYZ_to_lin_sRGB`).
const XYZ_TO_LIN_SRGB: [[f64; 3]; 3] = [
    [12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
    [
        -851781.0 / 878810.0,
        1648619.0 / 878810.0,
        36519.0 / 878810.0,
    ],
    [705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0],
];

/// Linear-light sRGB to CIE XYZ (`lin_sRGB_to_XYZ`).
const LIN_SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [506752.0 / 1228815.0, 87881.0 / 245763.0, 12673.0 / 70218.0],
    [87098.0 / 409605.0, 175762.0 / 245763.0, 12673.0 / 175545.0],
    [7918.0 / 409605.0, 87881.0 / 737289.0, 1001167.0 / 1053270.0],
];

/// CIE XYZ to linear-light Display P3 (`XYZ_to_lin_P3`).
const XYZ_TO_LIN_P3: [[f64; 3]; 3] = [
    [
        446124.0 / 178915.0,
        -333277.0 / 357830.0,
        -72051.0 / 178915.0,
    ],
    [-14852.0 / 17905.0, 63121.0 / 35810.0, 423.0 / 17905.0],
    [11844.0 / 330415.0, -50337.0 / 660830.0, 316169.0 / 330415.0],
];

impl Color {
    /// The colour a literal states: `#rrggbb`, `#rrggbbaa`, `srgb(r, g, b[, a])`
    /// or `display_p3(r, g, b[, a])`, each number `["-"] digits ["." digits]`;
    /// `None` for any other text, a non-finite channel or an alpha outside
    /// `0..=1`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Color> {
        if let Some(hex) = text.strip_prefix('#') {
            return parse_hex(hex);
        }
        let (name, rest) = text.split_once('(')?;
        let mut numbers = rest.strip_suffix(')')?.split(',').map(|n| number(n.trim()));
        let [r, g, b] = [numbers.next()??, numbers.next()??, numbers.next()??];
        let a = numbers.next().unwrap_or(Some(1.0))?;
        if numbers.next().is_some() {
            return None;
        }
        let [r, g, b] = match name {
            "srgb" => [r, g, b],
            "display_p3" => display_p3_to_srgb([r, g, b]),
            _ => return None,
        };
        let color = Color {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        };
        let finite = [color.r, color.g, color.b].iter().all(|c| c.is_finite());
        (finite && (0.0..=1.0).contains(&color.a)).then_some(color)
    }

    /// The colour a Display P3 value states (gamma-encoded channels), in
    /// extended sRGB.
    #[must_use]
    pub fn from_display_p3(r: f32, g: f32, b: f32, a: f32) -> Color {
        let [r, g, b] = display_p3_to_srgb([r, g, b].map(f64::from));
        Color {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a,
        }
    }

    /// `[r, g, b, a]` in 8-bit sRGB, each channel clamped to `0..=1` first:
    /// for a connector that renders sRGB.
    #[must_use]
    pub fn to_srgb8_clamped(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a].map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8)
    }

    /// `[r, g, b, a]` in Display P3, gamma-encoded and not clamped.
    #[must_use]
    pub fn to_display_p3(self) -> [f32; 4] {
        let xyz = multiply(
            &LIN_SRGB_TO_XYZ,
            [self.r, self.g, self.b].map(|c| linear(f64::from(c))),
        );
        let [r, g, b] = multiply(&XYZ_TO_LIN_P3, xyz).map(|c| gamma(c) as f32);
        [r, g, b, self.a]
    }
}

/// `layer` composited over `base`: source-over with straight (not
/// premultiplied) alpha in sRGB.
///
/// The result covers `a = layer.a + base.a × (1 − layer.a)`, and each channel
/// is the two channels weighted by what each contributes, divided by `a`.
/// Where one input decides the result, that input is returned exactly: an
/// opaque layer (`layer.a >= 1`) is the layer; a transparent layer
/// (`layer.a <= 0`) is `base`; a layer over a transparent base
/// (`base.a <= 0`) is the layer.
///
/// This is how a platform lays a translucent colour over what lies beneath,
/// for a toolkit that draws the composite as one colour. The blend is in sRGB
/// rather than linear light, as platform compositors blend.
///
/// ```
/// use native_theme::next::Color;
/// use native_theme::next::color::composite_over;
///
/// let half_black = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 };
/// let white = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
/// assert_eq!(composite_over(half_black, white), Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 });
/// assert_eq!(composite_over(half_black, half_black).a, 0.75);
/// ```
#[must_use]
pub fn composite_over(layer: Color, base: Color) -> Color {
    if layer.a >= 1.0 {
        return layer;
    }
    if layer.a <= 0.0 {
        return base;
    }
    if base.a <= 0.0 {
        return layer;
    }
    // How much of `base` reaches the result, in the composite's own alpha.
    let under = base.a * (1.0 - layer.a);
    let a = layer.a + under;
    let channel = |l: f32, b: f32| (l * layer.a + b * under) / a;
    Color {
        r: channel(layer.r, base.r),
        g: channel(layer.g, base.g),
        b: channel(layer.b, base.b),
        a,
    }
}

/// `#rrggbb` or `#rrggbbaa` without its `#`, hex digits in either case.
fn parse_hex(hex: &str) -> Option<Color> {
    if !matches!(hex.len(), 6 | 8) || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let mut channels = hex
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            std::str::from_utf8(pair)
                .ok()
                .and_then(|p| u8::from_str_radix(p, 16).ok())
        })
        .map(|byte| byte.map(|byte| f32::from(byte) / 255.0));
    let [r, g, b] = [channels.next()??, channels.next()??, channels.next()??];
    let a = channels.next().unwrap_or(Some(1.0))?;
    Some(Color { r, g, b, a })
}

/// A number literal, `["-"] digits ["." digits]`, if finite.
fn number(text: &str) -> Option<f64> {
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let (whole, fraction) = match unsigned.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (unsigned, None),
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !digits(whole) || !fraction.is_none_or(digits) {
        return None;
    }
    text.parse::<f64>().ok().filter(|n| n.is_finite())
}

/// Gamma-encoded Display P3 to gamma-encoded extended sRGB.
fn display_p3_to_srgb(p3: [f64; 3]) -> [f64; 3] {
    let xyz = multiply(&LIN_P3_TO_XYZ, p3.map(linear));
    multiply(&XYZ_TO_LIN_SRGB, xyz).map(gamma)
}

fn multiply(matrix: &[[f64; 3]; 3], [x, y, z]: [f64; 3]) -> [f64; 3] {
    matrix.map(|[a, b, c]| a * x + b * y + c * z)
}

/// CSS Color 4's `lin_sRGB` (Display P3's `lin_P3` too): the extended sRGB
/// transfer function, reflected for negative values.
fn linear(v: f64) -> f64 {
    let abs = v.abs();
    if abs <= 0.04045 {
        v / 12.92
    } else {
        ((abs + 0.055) / 1.055).powf(2.4).copysign(v)
    }
}

/// CSS Color 4's `gam_sRGB` (Display P3's `gam_P3` too), the inverse of
/// [`linear`].
fn gamma(v: f64) -> f64 {
    let abs = v.abs();
    if abs > 0.0031308 {
        (1.055 * abs.powf(1.0 / 2.4) - 0.055).copysign(v)
    } else {
        12.92 * v
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    fn assert_close(actual: [f32; 4], expected: [f64; 4]) {
        let close = actual
            .iter()
            .zip(expected)
            .all(|(a, e)| (f64::from(*a) - e).abs() < 1e-5);
        assert!(close, "{actual:?} != {expected:?}");
    }

    fn channels(c: Color) -> [f32; 4] {
        [c.r, c.g, c.b, c.a]
    }

    #[test]
    fn color_parses_s2_1_forms() {
        assert_eq!(
            Color::parse("#ff8000"),
            Some(rgba(1.0, 128.0 / 255.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::parse("#FF800080"),
            Some(rgba(1.0, 128.0 / 255.0, 0.0, 128.0 / 255.0))
        );
        assert_eq!(
            Color::parse("srgb(0.25, 0.5, 1)"),
            Some(rgba(0.25, 0.5, 1.0, 1.0))
        );
        assert_eq!(
            Color::parse("srgb(1.5,-0.25,0,0.5)"),
            Some(rgba(1.5, -0.25, 0.0, 0.5))
        );
        assert_eq!(
            Color::parse("srgb( 0 , 0 , 0 , 0 )"),
            Some(rgba(0.0, 0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::parse("display_p3(0.5, 0.25, 0.75)"),
            Some(Color::from_display_p3(0.5, 0.25, 0.75, 1.0))
        );
        assert_eq!(
            Color::parse("display_p3(1, 0, 0, 0.5)"),
            Some(Color::from_display_p3(1.0, 0.0, 0.0, 0.5))
        );
        for refused in [
            "",
            "#",
            "#fff",
            "#ffff",
            "#fffffff",
            "#fffffffff",
            "#gggggg",
            "#+fffff",
            "ff8000",
            "srgb(0.5, 0.5)",
            "srgb(0.5, 0.5, 0.5, 0.5, 0.5)",
            "srgb(0.5, 0.5, 0.5, 1.5)",
            "srgb(0.5, 0.5, 0.5, -0.5)",
            "srgb(.5, 0.5, 0.5)",
            "srgb(0.5., 0.5, 0.5)",
            "srgb(+0.5, 0.5, 0.5)",
            "srgb(1e0, 0.5, 0.5)",
            "srgb(0.5, 0.5, 0.5",
            "srgb 0.5, 0.5, 0.5)",
            "rgb(0.5, 0.5, 0.5)",
            "display_p3(0.5, 0.5)",
            "display-p3(0.5, 0.5, 0.5)",
        ] {
            assert_eq!(Color::parse(refused), None, "{refused}");
        }
        let huge = format!("srgb({}, 0, 0)", "9".repeat(60));
        assert_eq!(Color::parse(&huge), None);
    }

    #[test]
    fn display_p3_converts_by_css_color_4() {
        // Expected values: CSS Color 4's conversions.js chain, gam_sRGB(XYZ_to_lin_sRGB(
        // lin_P3_to_XYZ(lin_P3(c)))) and its inverse gam_P3(XYZ_to_lin_P3(lin_sRGB_to_XYZ(
        // lin_sRGB(c)))), evaluated in Python with the matrices as exact fractions (each entry
        // the published ratio) and the transfer functions in double precision, printed to seven
        // decimals.
        let p3_to_srgb = [
            ([1.0, 0.0, 0.0], [1.0930664, -0.2267420, -0.1501346]),
            ([0.0, 1.0, 0.0], [-0.5116050, 1.0182657, -0.3106746]),
            ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0420216]),
            ([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
            ([0.5, 0.25, 0.75], [0.5378318, 0.2321319, 0.7770569]),
        ];
        for ([r, g, b], [er, eg, eb]) in p3_to_srgb {
            assert_close(
                channels(Color::from_display_p3(r, g, b, 0.5)),
                [er, eg, eb, 0.5],
            );
            assert_close(
                Color::from_display_p3(r, g, b, 1.0).to_display_p3(),
                [r, g, b, 1.0].map(f64::from),
            );
        }
        let srgb_to_p3 = [
            ([1.0, 0.0, 0.0], [0.9174876, 0.2002868, 0.1385606]),
            ([0.0, 1.0, 0.0], [0.4584016, 0.9852646, 0.2982947]),
            ([0.0, 0.0, 1.0], [0.0, 0.0, 0.9595880]),
        ];
        for ([r, g, b], [er, eg, eb]) in srgb_to_p3 {
            assert_close(rgba(r, g, b, 1.0).to_display_p3(), [er, eg, eb, 1.0]);
        }
    }

    #[test]
    fn srgb8_clamps_then_rounds() {
        assert_eq!(
            rgba(1.093, -0.2, 0.5, 1.0).to_srgb8_clamped(),
            [255, 0, 128, 255]
        );
        assert_eq!(
            rgba(0.2, 0.4, 0.6, 0.0).to_srgb8_clamped(),
            [51, 102, 153, 0]
        );
    }

    #[test]
    fn composite_over_is_source_over_with_straight_alpha() {
        let half_red = rgba(1.0, 0.0, 0.0, 0.5);
        let half_blue = rgba(0.0, 0.0, 1.0, 0.5);
        let over = composite_over(half_red, half_blue);
        // a = 0.5 + 0.5 × 0.5; red = 1 × 0.5 / 0.75, blue = 1 × 0.25 / 0.75.
        assert_close(channels(over), [2.0 / 3.0, 0.0, 1.0 / 3.0, 0.75]);
        let opaque = rgba(0.1, 0.2, 0.3, 1.0);
        assert_eq!(composite_over(opaque, half_blue), opaque);
        assert_eq!(
            composite_over(rgba(0.1, 0.2, 0.3, 0.0), half_blue),
            half_blue
        );
        let clear = rgba(0.0, 0.0, 0.0, 0.0);
        assert_eq!(composite_over(half_red, clear), half_red);
    }
}
