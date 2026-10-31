//! Kirigami's linear colour interpolation in HSV, computed the way Qt 6's colour type stores,
//! rounds and converts its components.
//!
//! A colour enters as 16-bit RGB storage (red, green, blue, alpha, each 0 ..= 65535), is read in
//! HSV, interpolated component by component in double precision, rebuilt from real HSV
//! components and read back out as RGB.

/// The largest value of a 16-bit component.
const MAX16: u16 = 65535;

/// The stored hue that marks an achromatic colour.
const NO_HUE: u16 = 65535;

/// A colour in Qt's HSV storage: hue in hundredths of a degree (0 ..= 36000, or [`NO_HUE`]),
/// saturation, value and alpha as 16-bit components.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hsv16 {
    hue: u16,
    saturation: u16,
    value: u16,
    alpha: u16,
}

/// Qt's rounding of a single to an integer: adds a half of the argument's sign in single
/// precision, then truncates toward zero. NaN gives 0; results beyond `i32` saturate.
fn qround(x: f32) -> i32 {
    let shifted = if x >= 0.0 { x + 0.5 } else { x - 0.5 };
    shifted.trunc() as i32
}

/// [`qround`] clamped into the 16-bit range.
fn qround_u16(x: f32) -> u16 {
    let rounded = qround(x).clamp(0, i32::from(MAX16));
    u16::try_from(rounded).unwrap_or(MAX16)
}

/// The single-precision real (0 ..= 1) of a 16-bit component.
fn unit_of(component: u16) -> f32 {
    f32::from(component) / f32::from(MAX16)
}

/// Whether a single lies within [0, 1]; NaN does not.
fn in_unit(x: f32) -> bool {
    (0.0..=1.0).contains(&x)
}

/// Converts RGB storage to HSV storage as Qt does, alpha copied.
#[allow(
    clippy::arithmetic_side_effects,
    reason = "differences of 16-bit channels widened to i32, and the largest less the smallest: none can overflow"
)]
fn rgb_to_hsv(rgba: [u16; 4]) -> Hsv16 {
    let [r, g, b, alpha] = rgba;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    if max == min {
        return Hsv16 {
            hue: NO_HUE,
            saturation: 0,
            value: max,
            alpha,
        };
    }
    let chroma = f32::from(max - min);
    let saturation = qround_u16((chroma / f32::from(max)) * 65535.0);
    let (ri, gi, bi) = (i32::from(r), i32::from(g), i32::from(b));
    let sector = if r == max {
        let h = (gi - bi) as f32 / chroma;
        if h < 0.0 { h + 6.0 } else { h }
    } else if g == max {
        2.0 + (bi - ri) as f32 / chroma
    } else {
        4.0 + (ri - gi) as f32 / chroma
    };
    Hsv16 {
        hue: qround_u16(sector * 6000.0),
        saturation,
        value: max,
        alpha,
    }
}

/// Converts HSV storage to RGB storage as Qt does, alpha copied. Every product and difference
/// is rounded to single separately.
fn hsv_to_rgb(hsv: Hsv16) -> [u16; 4] {
    if hsv.saturation == 0 || hsv.hue == NO_HUE {
        return [hsv.value, hsv.value, hsv.value, hsv.alpha];
    }
    let h = if hsv.hue == 36000 {
        0.0
    } else {
        f32::from(hsv.hue) / 6000.0
    };
    let s = unit_of(hsv.saturation);
    let v = unit_of(hsv.value);
    let sector = h.trunc();
    let f = h - sector;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    let (red, green, blue) = match sector as i32 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    [
        qround_u16(red * 65535.0),
        qround_u16(green * 65535.0),
        qround_u16(blue * 65535.0),
        hsv.alpha,
    ]
}

/// Qt's colour built from real HSV components (singles); `None` where Qt yields an invalid
/// colour: a hue outside [0, 1] other than exactly −1, or a saturation, value or alpha outside
/// [0, 1], NaN included.
fn hsv_from_reals(h: f32, s: f32, v: f32, a: f32) -> Option<Hsv16> {
    let hue_ok = in_unit(h) || h == -1.0;
    if !(hue_ok && in_unit(s) && in_unit(v) && in_unit(a)) {
        return None;
    }
    let hue = if h == -1.0 {
        NO_HUE
    } else {
        qround_u16(h * 36000.0)
    };
    Some(Hsv16 {
        hue,
        saturation: qround_u16(s * 65535.0),
        value: qround_u16(v * 65535.0),
        alpha: qround_u16(a * 65535.0),
    })
}

/// The hue as Qt's real accessor reads it: −1 for an achromatic colour, else the stored hue
/// over 36000 (so a stored 36000 reads as exactly 1).
fn hue_of(hsv: Hsv16) -> f32 {
    if hsv.hue == NO_HUE {
        -1.0
    } else {
        f32::from(hsv.hue) / 36000.0
    }
}

/// `x + (y − x)·t` in double, the endpoints widened from single.
fn lerp(x: f32, y: f32, t: f64) -> f64 {
    let (x, y) = (f64::from(x), f64::from(y));
    x + (y - x) * t
}

/// Interpolates between two colours given as 16-bit RGB storage `[red, green, blue, alpha]`,
/// returning the result's 16-bit RGB storage.
///
/// `balance` 0 gives `a`'s components, 1 gives `b`'s; it is clamped to [0, 1] first. Hue,
/// saturation, value and alpha are interpolated separately and linearly (the hue never wraps
/// the short way round). A colour without a hue, or with hue exactly 0, takes the other
/// colour's hue for both ends. `None` stands for Qt's invalid colour, which only a NaN
/// balance produces.
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "the 16-bit entry the vector table tests; the model's colours enter by `kirigami_lerp`"
    )
)]
fn kirigami_lerp_rgb16(a: [u16; 4], b: [u16; 4], balance: f32) -> Option<[u16; 4]> {
    lerp_entered(Entered::from_rgb16(a), Entered::from_rgb16(b), balance)
}

/// Interpolates between two model colours `[red, green, blue, alpha]`, as
/// `kirigami_lerp_rgb16` does, returning the result's channels (16-bit storage over 65535).
///
/// Channels within [0, 1] enter Qt's storage rounded to 16 bits. A colour with red, green or
/// blue outside [0, 1] (alpha within it) is Qt's extended RGB: all four channels are rounded to
/// half precision, its hue, saturation and value are those of its red, green and blue clamped
/// to [0, 1] and stored in 16 bits, and its alpha is the half value. `None` when an input has a
/// NaN channel or an alpha outside [0, 1] (Qt's invalid colour), or for a NaN balance.
pub fn kirigami_lerp(a: [f32; 4], b: [f32; 4], balance: f32) -> Option<[f32; 4]> {
    let [r, g, bl, al] = lerp_entered(Entered::from_reals(a)?, Entered::from_reals(b)?, balance)?;
    Some([unit_of(r), unit_of(g), unit_of(bl), unit_of(al)])
}

/// What Kirigami reads of an input colour: its HSV storage (for hue, saturation and value) and
/// its real alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Entered {
    hsv: Hsv16,
    alpha: f32,
}

impl Entered {
    /// An RGB colour from its 16-bit storage.
    fn from_rgb16(rgba: [u16; 4]) -> Self {
        let hsv = rgb_to_hsv(rgba);
        Self {
            hsv,
            alpha: unit_of(hsv.alpha),
        }
    }

    /// A colour built from real channels as Qt's `fromRgbF` builds it; `None` for an invalid
    /// colour (alpha outside [0, 1] or NaN, or a NaN channel).
    fn from_reals(c: [f32; 4]) -> Option<Self> {
        let [r, g, b, a] = c;
        if !in_unit(a) {
            return None;
        }
        if in_unit(r) && in_unit(g) && in_unit(b) {
            return Some(Self::from_rgb16(c.map(|x| qround_u16(x * 65535.0))));
        }
        if r.is_nan() || g.is_nan() || b.is_nan() {
            return None;
        }
        let [hr, hg, hb, ha] = c.map(round_to_half);
        let clamped = |x: f32| qround_u16(65535.0 * x.clamp(0.0, 1.0));
        let rgb16 = [
            clamped(hr),
            clamped(hg),
            clamped(hb),
            qround_u16(65535.0 * ha),
        ];
        Some(Self {
            hsv: rgb_to_hsv(rgb16),
            alpha: ha,
        })
    }
}

/// Kirigami's interpolation of two read colours, built back through Qt's from-HSV constructor
/// and converted to 16-bit RGB storage.
#[allow(
    clippy::modulo_arithmetic,
    reason = "a float remainder, which cannot panic"
)]
fn lerp_entered(a: Entered, b: Entered, balance: f32) -> Option<[u16; 4]> {
    let t = f64::from(balance.clamp(0.0, 1.0));
    let (ha, hb) = (a.hsv, b.hsv);
    let (hue_a, hue_b) = (hue_of(ha), hue_of(hb));
    let start = if hue_a > 0.0 { hue_a } else { hue_b }.max(0.0);
    let end = if hue_b > 0.0 { hue_b } else { hue_a }.max(0.0);
    let hue = lerp(start, end, t) % 1.0;
    let sat = lerp(unit_of(ha.saturation), unit_of(hb.saturation), t).clamp(0.0, 1.0);
    let val = lerp(unit_of(ha.value), unit_of(hb.value), t).clamp(0.0, 1.0);
    let alpha = lerp(a.alpha, b.alpha, t).clamp(0.0, 1.0);
    let built = hsv_from_reals(hue as f32, sat as f32, val as f32, alpha as f32)?;
    Some(hsv_to_rgb(built))
}

/// The power of two `2^e` as a single, for `e` within the normal exponent range.
#[allow(
    clippy::arithmetic_side_effects,
    reason = "the exponent clamped to -126..=127 plus 127, within 1..=254: no overflow"
)]
fn pow2(e: i32) -> f32 {
    let biased = u32::try_from(e.clamp(-126, 127) + 127).unwrap_or(127);
    f32::from_bits(biased << 23)
}

/// A single rounded to the nearest IEEE half-precision value (ties to even), returned widened
/// back to single. Beyond half's range the result is an infinity of the same sign; NaN and
/// infinities pass through.
#[allow(
    clippy::arithmetic_side_effects,
    reason = "an 8-bit biased exponent less 137, and its negation: no overflow"
)]
fn round_to_half(x: f32) -> f32 {
    if !x.is_finite() {
        return x;
    }
    let magnitude = x.abs();
    if magnitude >= 65520.0 {
        return f32::INFINITY.copysign(x);
    }
    let quantum_exp = if magnitude < pow2(-14) {
        -24
    } else {
        let biased = i32::try_from((magnitude.to_bits() >> 23) & 0xff).unwrap_or(127);
        biased - 127 - 10
    };
    let steps = (magnitude * pow2(-quantum_exp)).round_ties_even();
    (steps * pow2(quantum_exp)).copysign(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = include_str!("../../../tests/vectors/kirigami_lerp.tsv");

    fn parse_hex(text: &str) -> Option<[f32; 4]> {
        let hex = text.strip_prefix('#')?;
        let byte = |i: usize| -> Option<f32> {
            let pair = hex.get(i..i + 2)?;
            u8::from_str_radix(pair, 16)
                .ok()
                .map(|n| f32::from(n) / 255.0)
        };
        let alpha = match hex.len() {
            6 => 1.0,
            8 => byte(6)?,
            _ => return None,
        };
        Some([byte(0)?, byte(2)?, byte(4)?, alpha])
    }

    fn parse_srgb(text: &str) -> Option<[f64; 4]> {
        let inner = text.strip_prefix("srgb(")?.strip_suffix(')')?;
        let mut out = [0.0; 4];
        let mut parts = inner.split(',');
        for slot in out.iter_mut() {
            *slot = parts.next()?.trim().parse().ok()?;
        }
        if parts.next().is_some() {
            return None;
        }
        Some(out)
    }

    #[test]
    fn vectors() {
        let mut rows = 0;
        let mut failures = Vec::new();
        let body = TABLE
            .lines()
            .filter(|line| !(line.starts_with("# ") || *line == "#" || line.trim().is_empty()))
            .skip(1);
        for line in body {
            rows += 1;
            let cols: Vec<&str> = line.split('\t').collect();
            let parsed = match cols.as_slice() {
                [a, b, t, result] => (|| {
                    Some((
                        parse_hex(a)?,
                        parse_hex(b)?,
                        t.parse::<f32>().ok()?,
                        parse_srgb(result)?,
                    ))
                })(),
                _ => None,
            };
            let Some((a, b, t, expected)) = parsed else {
                failures.push(format!("unparsable row: {line}"));
                continue;
            };
            match kirigami_lerp(a, b, t) {
                Some(got) => {
                    let close = got
                        .iter()
                        .zip(expected.iter())
                        .all(|(&g, &e)| ((f64::from(g) - e) * 255.0).abs() <= 1e-3);
                    if !close {
                        failures.push(format!("{line}: got {got:?}"));
                    }
                }
                None => failures.push(format!("{line}: got an invalid colour")),
            }
        }
        assert_eq!(rows, 48, "row count");
        assert!(
            failures.is_empty(),
            "{} of {rows} rows failed:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    #[test]
    fn nan_balance_is_invalid() {
        assert_eq!(
            kirigami_lerp_rgb16([65535, 0, 0, 65535], [0, 0, 65535, 65535], f32::NAN),
            None
        );
    }

    #[test]
    fn balance_is_clamped() {
        let a = [15677, 44718, 59881, 65535];
        let b = [8995, 9766, 10537, 65535];
        assert_eq!(
            kirigami_lerp_rgb16(a, b, -3.0),
            kirigami_lerp_rgb16(a, b, 0.0)
        );
        assert_eq!(
            kirigami_lerp_rgb16(a, b, 7.0),
            kirigami_lerp_rgb16(a, b, 1.0)
        );
    }

    #[test]
    #[allow(
        clippy::excessive_precision,
        reason = "half-precision values written exactly"
    )]
    fn half_rounding() {
        assert_eq!(round_to_half(0.3), 0.300048828125);
        assert_eq!(qround_u16(65535.0 * round_to_half(0.3)), 19664);
        assert_eq!(round_to_half(1.0), 1.0);
        assert_eq!(
            round_to_half(-0.1).to_bits(),
            (-0.0999755859375f32).to_bits()
        );
        assert_eq!(round_to_half(65504.0), 65504.0);
        assert_eq!(round_to_half(65520.0), f32::INFINITY);
        assert_eq!(round_to_half(pow2(-25)), 0.0);
        assert_eq!(round_to_half(pow2(-24)), pow2(-24));
    }

    #[test]
    #[allow(
        clippy::excessive_precision,
        reason = "half-precision values written exactly"
    )]
    fn extended_entry() {
        let Some(entered) = Entered::from_reals([1.5, 0.3, 0.3, 0.3]) else {
            panic!("an extended colour is valid");
        };
        assert_eq!(entered.alpha, 0.300048828125);
        assert_eq!(entered.hsv, rgb_to_hsv([65535, 19664, 19664, 19664]));
        assert!(Entered::from_reals([f32::INFINITY, -2.0, 0.5, 1.0]).is_some());
        assert!(Entered::from_reals([f32::NAN, 0.5, 0.5, 1.0]).is_none());
        assert!(Entered::from_reals([1.2, 0.5, 0.5, 1.5]).is_none());
        assert!(Entered::from_reals([0.5, 0.5, 0.5, f32::NAN]).is_none());
        let opaque = [0.2, 0.4, 0.6, 1.0];
        for t in [0.0, 0.3, 1.0] {
            assert!(kirigami_lerp([1.2, 0.5, 0.25, 0.3], opaque, t).is_some());
        }
    }

    /// An input or expected value the description has not stated yet is `None`.
    struct PendingRow {
        a: Option<[f32; 4]>,
        b: Option<&'static str>,
        balance: Option<f32>,
        expected: Option<[f64; 4]>,
    }

    /// The six extended-RGB rows of `qcolor_helpers.md` section 11. Their expected values come from
    /// the generator (`tools/kde-vectors`, a `fromRgbF` input form) once it computes them; the
    /// inputs the section leaves open are `None` until then.
    const EXTENDED_ROWS: [PendingRow; 6] = [
        PendingRow {
            a: Some([1.2, 0.5, 0.25, 1.0]),
            b: Some("#3daee9"),
            balance: Some(0.5),
            expected: None,
        },
        PendingRow {
            a: Some([-0.1, 0.5, 1.0, 1.0]),
            b: Some("#232629"),
            balance: Some(0.2),
            expected: None,
        },
        PendingRow {
            a: Some([1.5, 0.3, 0.3, 1.0]),
            b: None,
            balance: None,
            expected: None,
        },
        PendingRow {
            a: Some([1.2, 0.5, 0.25, 0.3]),
            b: None,
            balance: Some(0.5),
            expected: None,
        },
        PendingRow {
            a: None,
            b: Some("#3daee9"),
            balance: Some(0.2),
            expected: None,
        },
        PendingRow {
            a: None,
            b: None,
            balance: None,
            expected: None,
        },
    ];

    #[test]
    #[ignore = "expected values come from the generator"]
    fn extended_vectors() {
        let mut failures = Vec::new();
        for (i, row) in EXTENDED_ROWS.iter().enumerate() {
            let inputs = (|| Some((row.a?, parse_hex(row.b?)?, row.balance?, row.expected?)))();
            let Some((a, b, t, expected)) = inputs else {
                failures.push(format!("row {i}: not fully stated"));
                continue;
            };
            match kirigami_lerp(a, b, t) {
                Some(got)
                    if got
                        .iter()
                        .zip(expected.iter())
                        .all(|(&g, &e)| ((f64::from(g) - e) * 255.0).abs() <= 1e-3) => {}
                got => failures.push(format!("row {i}: got {got:?}")),
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
