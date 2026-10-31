//! KDE colour-scheme shading: the light, midlight, mid, dark and shadow shades
//! a KDE colour scheme derives from one colour, computed in KDE's HCY colour
//! space on top of Qt's 16-bit colour storage.

/// The five shades a KDE colour scheme derives from a colour, from the
/// brightest variant to the shadow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KdeShadeRole {
    /// The brightest shade.
    Light,
    /// Between the colour and the light shade.
    Midlight,
    /// A shade slightly darker than the colour.
    Mid,
    /// A clearly darker shade.
    Dark,
    /// The darkest shade, used for shadows.
    Shadow,
}

const K_R: f64 = 0.2126;
const K_G: f64 = 0.7152;
const K_B: f64 = 0.0722;

/// A colour as Qt stores it: 16-bit RGB, half-precision extended RGB, or the
/// invalid (default-constructed) colour.
#[derive(Clone, Copy, Debug)]
enum Stored {
    Rgb { r: u16, g: u16, b: u16, a: u16 },
    Extended { r: f32, g: f32, b: f32, a: f32 },
    Invalid,
}

fn qround(x: f32) -> i32 {
    let half = if x >= 0.0 { 0.5f32 } else { -0.5f32 };
    (x + half).trunc() as i32
}

fn to_u16(x: i32) -> u16 {
    u16::try_from(x.clamp(0, i32::from(u16::MAX))).unwrap_or(u16::MAX)
}

fn store(x: f32) -> u16 {
    to_u16(qround(x * 65535.0f32))
}

fn round_to_half(x: f32) -> f32 {
    if !x.is_finite() {
        return x;
    }
    let ax = x.abs();
    if ax >= 65520.0 {
        return f32::INFINITY.copysign(x);
    }
    let quantum = if ax < f32::from_bits(113 << 23) {
        f32::from_bits(103 << 23)
    } else {
        let biased = (ax.to_bits() >> 23) & 0xff;
        f32::from_bits(biased.saturating_sub(10) << 23)
    };
    (x / quantum).round_ties_even() * quantum
}

fn in_unit(x: f32) -> bool {
    (0.0..=1.0).contains(&x)
}

impl Stored {
    fn from_rgb_f(r: f32, g: f32, b: f32, a: f32) -> Stored {
        if !in_unit(a) {
            Stored::Invalid
        } else if in_unit(r) && in_unit(g) && in_unit(b) {
            Stored::Rgb {
                r: store(r),
                g: store(g),
                b: store(b),
                a: store(a),
            }
        } else if r.is_nan() || g.is_nan() || b.is_nan() {
            Stored::Invalid
        } else {
            Stored::Extended {
                r: round_to_half(r),
                g: round_to_half(g),
                b: round_to_half(b),
                a: round_to_half(a),
            }
        }
    }

    fn real(self) -> [f32; 4] {
        match self {
            Stored::Rgb { r, g, b, a } => [
                f32::from(r) / 65535.0f32,
                f32::from(g) / 65535.0f32,
                f32::from(b) / 65535.0f32,
                f32::from(a) / 65535.0f32,
            ],
            Stored::Extended { r, g, b, a } => [r, g, b, a],
            Stored::Invalid => [0.0, 0.0, 0.0, 1.0],
        }
    }

    fn rgb_storage(self) -> [u16; 4] {
        match self {
            Stored::Rgb { r, g, b, a } => [r, g, b, a],
            Stored::Extended { r, g, b, a } => [
                store(r.clamp(0.0, 1.0)),
                store(g.clamp(0.0, 1.0)),
                store(b.clamp(0.0, 1.0)),
                store(a),
            ],
            Stored::Invalid => [0, 0, 0, u16::MAX],
        }
    }
}

#[expect(
    clippy::neg_cmp_op_on_partial_ord,
    reason = "a NaN takes the negated comparison's branch, as the described arithmetic requires"
)]
fn norm(a: f64) -> f64 {
    if !(a < 1.0) {
        1.0
    } else if a > 0.0 {
        a
    } else {
        0.0
    }
}

#[allow(
    clippy::modulo_arithmetic,
    reason = "a float remainder, which cannot panic"
)]
fn wrap(a: f64) -> f64 {
    let r = a % 1.0;
    if r < 0.0 {
        r + 1.0
    } else if r > 0.0 {
        r
    } else {
        0.0
    }
}

fn gamma(n: f64) -> f64 {
    norm(n).powf(2.2)
}

fn igamma(n: f64) -> f64 {
    norm(n).powf(1.0 / 2.2)
}

fn luma_lin(r: f64, g: f64, b: f64) -> f64 {
    (K_R * r + K_G * g) + K_B * b
}

#[derive(Clone, Copy, Debug)]
struct Hcy {
    h: f64,
    c: f64,
    y: f64,
    a: f64,
}

fn luma(colour: Stored) -> f64 {
    let [r, g, b, _] = colour.real();
    luma_lin(
        gamma(f64::from(r)),
        gamma(f64::from(g)),
        gamma(f64::from(b)),
    )
}

fn to_hcy(colour: Stored) -> Hcy {
    let [r, g, b, a] = colour.real();
    let rl = gamma(f64::from(r));
    let gl = gamma(f64::from(g));
    let bl = gamma(f64::from(b));
    let y = luma_lin(rl, gl, bl);
    let p = rl.max(gl).max(bl);
    let m = rl.min(gl).min(bl);
    let d = 6.0 * (p - m);
    let h = if m == p {
        0.0
    } else if rl == p {
        (gl - bl) / d
    } else if gl == p {
        (bl - rl) / d + 1.0 / 3.0
    } else {
        (rl - gl) / d + 2.0 / 3.0
    };
    let c = if rl == gl && gl == bl {
        0.0
    } else {
        ((y - m) / y).max((p - y) / (1.0 - y))
    };
    Hcy {
        h,
        c,
        y,
        a: f64::from(a),
    }
}

fn from_hcy(hcy: Hcy) -> Stored {
    let h = wrap(hcy.h);
    let c = norm(hcy.c);
    let y = norm(hcy.y);
    let s = 6.0 * h;
    let (sextant, t, m) = if s < 1.0 {
        (0u8, s, K_R + K_G * s)
    } else if s < 2.0 {
        let t = 2.0 - s;
        (1, t, K_G + K_R * t)
    } else if s < 3.0 {
        let t = s - 2.0;
        (2, t, K_G + K_B * t)
    } else if s < 4.0 {
        let t = 4.0 - s;
        (3, t, K_B + K_G * t)
    } else if s < 5.0 {
        let t = s - 4.0;
        (4, t, K_B + K_R * t)
    } else {
        let t = 6.0 - s;
        (5, t, K_R + K_B * t)
    };
    let (pp, oo, qq) = if m >= y {
        (
            y + y * c * (1.0 - m) / m,
            y + y * c * (t - m) / m,
            y - y * c,
        )
    } else {
        (
            y + (1.0 - y) * c,
            y + (1.0 - y) * c * (t - m) / (1.0 - m),
            y - (1.0 - y) * c * m / (1.0 - m),
        )
    };
    let (r, g, b) = match sextant {
        0 => (pp, oo, qq),
        1 => (oo, pp, qq),
        2 => (qq, pp, oo),
        3 => (qq, oo, pp),
        4 => (oo, qq, pp),
        _ => (pp, qq, oo),
    };
    Stored::from_rgb_f(
        igamma(r) as f32,
        igamma(g) as f32,
        igamma(b) as f32,
        hcy.a as f32,
    )
}

fn hcy_shade(colour: Stored, k_y: f64) -> Stored {
    let mut hcy = to_hcy(colour);
    hcy.y = norm(hcy.y + k_y);
    hcy.c = norm(hcy.c + 0.0);
    from_hcy(hcy)
}

fn hcy_darken(colour: Stored, k_y: f64, k_c: f64) -> Stored {
    let mut hcy = to_hcy(colour);
    hcy.y = norm(hcy.y * (1.0 - k_y));
    hcy.c = norm(hcy.c * k_c);
    from_hcy(hcy)
}

/// Computes the shade of colour `c` (straight-alpha RGBA, each channel in
/// 0 ..= 1) that a KDE colour scheme uses for `role`, at the kdeglobals
/// `[KDE] contrast` step `contrast` (an integer step, 7 when unset; values
/// beyond ±10 act as ±10).
///
/// Very dark and very light colours get a fixed luma offset; other colours get
/// a luma offset scaled by their own luma, the shadow shade being darkened once
/// more. The colour passes through Qt's 16-bit storage on the way in and after
/// every step, and the result is that storage read back as RGBA in 0 ..= 1,
/// with the input's alpha kept.
#[expect(
    clippy::neg_cmp_op_on_partial_ord,
    reason = "a NaN takes the negated comparison's branch, as the described arithmetic requires"
)]
pub fn kde_shade(c: [f32; 4], role: KdeShadeRole, contrast: f32) -> [f32; 4] {
    let [r, g, b, a] = c;
    let colour = Stored::from_rgb_f(r, g, b, a);
    let kappa = 0.1 * f64::from(contrast);
    let kappa = if !(kappa < 1.0) {
        1.0
    } else if kappa < -1.0 {
        -1.0
    } else {
        kappa
    };
    let y = luma(colour);
    let result = if y < 0.006 {
        let k = match role {
            KdeShadeRole::Light => 0.05 + 0.95 * kappa,
            KdeShadeRole::Mid => 0.01 + 0.20 * kappa,
            KdeShadeRole::Dark => 0.02 + 0.40 * kappa,
            KdeShadeRole::Midlight | KdeShadeRole::Shadow => 0.03 + 0.60 * kappa,
        };
        hcy_shade(colour, k)
    } else if y > 0.93 {
        let k = match role {
            KdeShadeRole::Midlight => -0.02 - 0.20 * kappa,
            KdeShadeRole::Dark => -0.06 - 0.60 * kappa,
            KdeShadeRole::Shadow => -0.10 - 0.90 * kappa,
            KdeShadeRole::Light | KdeShadeRole::Mid => -0.04 - 0.40 * kappa,
        };
        hcy_shade(colour, k)
    } else {
        let light_amount = (0.05 + 0.55 * y) * (0.25 + 0.75 * kappa);
        let dark_amount = (-y) * (0.55 + 0.35 * kappa);
        match role {
            KdeShadeRole::Light => hcy_shade(colour, light_amount),
            KdeShadeRole::Midlight => hcy_shade(colour, (0.15 + 0.35 * (1.0 - y)) * light_amount),
            KdeShadeRole::Mid => hcy_shade(colour, (0.35 + 0.15 * y) * dark_amount),
            KdeShadeRole::Dark => hcy_shade(colour, dark_amount),
            KdeShadeRole::Shadow => hcy_darken(hcy_shade(colour, dark_amount), 0.5 + 0.3 * y, 1.0),
        }
    };
    let [sr, sg, sb, sa] = result.rgb_storage();
    [
        f32::from(sr) / 65535.0f32,
        f32::from(sg) / 65535.0f32,
        f32::from(sb) / 65535.0f32,
        f32::from(sa) / 65535.0f32,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = include_str!("../../../tests/vectors/kde_shade.tsv");

    fn parse_hex(s: &str) -> Option<[f32; 4]> {
        let hex = s.strip_prefix('#')?;
        if hex.len() != 6 {
            return None;
        }
        let mut out = [0.0f32, 0.0, 0.0, 1.0];
        for (i, slot) in out.iter_mut().take(3).enumerate() {
            let part = hex.get(2 * i..2 * i + 2)?;
            let n = u8::from_str_radix(part, 16).ok()?;
            *slot = f32::from(n) / 255.0f32;
        }
        Some(out)
    }

    fn parse_role(s: &str) -> Option<KdeShadeRole> {
        match s {
            "light" => Some(KdeShadeRole::Light),
            "midlight" => Some(KdeShadeRole::Midlight),
            "mid" => Some(KdeShadeRole::Mid),
            "dark" => Some(KdeShadeRole::Dark),
            "shadow" => Some(KdeShadeRole::Shadow),
            _ => None,
        }
    }

    fn parse_srgb(s: &str) -> Option<[f32; 4]> {
        let inner = s.strip_prefix("srgb(")?.strip_suffix(')')?;
        let parts: Vec<f32> = inner
            .split(',')
            .map(|p| p.trim().parse::<f32>().ok())
            .collect::<Option<Vec<f32>>>()?;
        match parts.as_slice() {
            [r, g, b, a] => Some([*r, *g, *b, *a]),
            _ => None,
        }
    }

    #[test]
    fn rounding_to_half() {
        assert_eq!(round_to_half(1.0), 1.0);
        assert_eq!(round_to_half(65504.0), 65504.0);
        assert_eq!(round_to_half(65520.0), f32::INFINITY);
        assert_eq!(round_to_half(1.0 + 1.0 / 2048.0), 1.0);
        assert_eq!(round_to_half(1.0 + 3.0 / 4096.0), 1.0 + 1.0 / 1024.0);
        assert_eq!(round_to_half(1.0 + 3.0 / 2048.0), 1.0 + 2.0 / 1024.0);
        assert_eq!(round_to_half(-2.0), -2.0);
    }

    #[test]
    fn vectors() {
        let mut header_seen = false;
        let mut rows = 0usize;
        let mut failures: Vec<String> = Vec::new();
        for line in TABLE.lines() {
            if line.starts_with('#') && !line.contains('\t') || line.trim().is_empty() {
                continue;
            }
            if !header_seen {
                header_seen = true;
                continue;
            }
            rows += 1;
            let cols: Vec<&str> = line.split('\t').collect();
            let parsed = match cols.as_slice() {
                [c, role, contrast, result] => parse_hex(c).and_then(|c| {
                    let role = parse_role(role)?;
                    let contrast = contrast.parse::<f32>().ok()?;
                    let expected = parse_srgb(result)?;
                    Some((c, role, contrast, expected))
                }),
                _ => None,
            };
            let Some((c, role, contrast, expected)) = parsed else {
                failures.push(format!("unparsable row: {line}"));
                continue;
            };
            let got = kde_shade(c, role, contrast);
            let ok = got
                .iter()
                .zip(expected.iter())
                .all(|(g, e)| (f64::from(*g) * 255.0 - f64::from(*e) * 255.0).abs() <= 1e-3);
            if !ok {
                failures.push(format!("{line}\n  got {got:?}"));
            }
        }
        assert_eq!(rows, 260, "row count");
        assert!(
            failures.is_empty(),
            "{} of {rows} rows failed:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
