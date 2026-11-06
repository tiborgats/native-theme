//! The colour GTK computes for libadwaita's standalone colours through CSS: the input
//! colour taken to OKLab, its lightness limited per scheme, and the result taken straight
//! back to sRGB, with no clamping of the colour channels.

use crate::selector::Scheme;

/// Converts one gamma-encoded sRGB channel to linear light, in double, rounded to single.
fn srgb_to_linear(v: f32) -> f32 {
    let v = f64::from(v);
    if v >= 0.04045 {
        ((v + 0.055) / (1.0 + 0.055)).powf(2.4) as f32
    } else {
        (v / 12.92) as f32
    }
}

/// Converts one linear-light channel back to gamma-encoded sRGB, in double, rounded to single.
fn linear_to_srgb(v: f32) -> f32 {
    let v = f64::from(v);
    if v > 0.0031308 {
        (1.055 * v.powf(1.0 / 2.4) - 0.055) as f32
    } else {
        (12.92 * v) as f32
    }
}

/// The product `a · b` as `(p, e)`: `p` the product rounded, `e` its
/// rounding error, so that `p + e` is `a · b` exactly (Dekker's product,
/// each half of a split holding at most 26 bits, no fused multiply-add).
fn two_product(a: f64, b: f64) -> (f64, f64) {
    fn split(x: f64) -> (f64, f64) {
        let c = 134_217_729.0 * x;
        let hi = c - (c - x);
        (hi, x - hi)
    }
    let p = a * b;
    let (ah, al) = split(a);
    let (bh, bl) = split(b);
    let e = ((ah * bh - p) + ah * bl + al * bh) + al * bl;
    (p, e)
}

/// The single next to `x` (finite) upward (`up`) or downward.
fn step(x: f32, up: bool) -> f32 {
    if x == 0.0 {
        let tiny = f32::from_bits(1);
        return if up { tiny } else { -tiny };
    }
    let bits = x.to_bits();
    f32::from_bits(if up == (x > 0.0) {
        bits.wrapping_add(1)
    } else {
        bits.wrapping_sub(1)
    })
}

/// The single nearest the exact value `p + e` (ties to even), where `p` is
/// that value rounded to a double and `e` the rest: rounding `p` alone
/// rounds right but where `p` lies on a midpoint between two singles that
/// the exact value does not.
fn round_to_single(p: f64, e: f64) -> f32 {
    let r = p as f32;
    let near = f64::from(r);
    if near == p || !r.is_finite() {
        return r;
    }
    let other = step(r, p > near);
    if !other.is_finite() || p != (near + f64::from(other)) * 0.5 || e == 0.0 {
        return r;
    }
    if (e > 0.0) == (f64::from(other) > near) {
        other
    } else {
        r
    }
}

/// `x`³ rounded once to the nearest single, the same on every platform:
/// `x²` is exact in a double and the product by `x` exact as a double's
/// sum ([`two_product`]), rounded by [`round_to_single`]. The platform's
/// `powf` is not that everywhere (glibc's `powf(x, 3.0)` is a unit in the
/// last place off on 477841 of the positive singles).
fn cube(x: f32) -> f32 {
    let x = f64::from(x);
    let (p, e) = two_product(x * x, x);
    round_to_single(p, e)
}

/// The cube root of `x` rounded once to the nearest single, the same on
/// every platform: Newton's iteration in double from a first guess off by
/// a few percent (the exponent divided by three, through the bits), then
/// the exact test of the midpoint between that single and its neighbour,
/// cubed exactly ([`two_product`]), against `x`. glibc's `cbrtf` is
/// correctly rounded on every single, so this gives its results; another
/// platform's need not.
fn cbrt(x: f32) -> f32 {
    if !x.is_finite() || x == 0.0 {
        return x;
    }
    let a = f64::from(x.abs());
    let third = a.to_bits().checked_div(3).unwrap_or(0);
    let mut y = f64::from_bits(third.wrapping_add(0x2a9f_7893_782d_a1ce));
    for _ in 0..6 {
        y = (2.0 * y + a / (y * y)) / 3.0;
    }
    let mut r = y as f32;
    for _ in 0..2 {
        let near = f64::from(r);
        let up = y > near;
        let other = step(r, up);
        let mid = (near + f64::from(other)) * 0.5;
        let (p, e) = two_product(mid * mid, mid);
        let above = (p - a) + e;
        if (up && above < 0.0) || (!up && above > 0.0) {
            r = other;
        } else {
            break;
        }
    }
    if x < 0.0 { -r } else { r }
}

/// Converts linear sRGB to OKLab `(L, a, b)`, entirely in single precision,
/// the cube roots correctly rounded ([`cbrt`]).
fn linear_srgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = (f32::from_bits(0x3ed3_0eb1) * r + f32::from_bits(0x3f09_4d17) * g)
        + f32::from_bits(0x3d52_b909) * b;
    let m = (f32::from_bits(0x3e58_fd3b) * r + f32::from_bits(0x3f2e_4253) * g)
        + f32::from_bits(0x3ddb_f2f0) * b;
    let s = (f32::from_bits(0x3db4_d7ec) * r + f32::from_bits(0x3e90_3d74) * g)
        + f32::from_bits(0x3f21_4649) * b;

    let l = cbrt(l);
    let m = cbrt(m);
    let s = cbrt(s);

    let ok_l = (f32::from_bits(0x3e57_8152) * l + f32::from_bits(0x3f4b_2a89) * m)
        - f32::from_bits(0x3b85_6ece) * s;
    let ok_a = (f32::from_bits(0x3ffd_2f0e) * l - f32::from_bits(0x401b_6e0e) * m)
        + f32::from_bits(0x3ee6_b438) * s;
    let ok_b = (f32::from_bits(0x3cd4_34b4) * l + f32::from_bits(0x3f48_63bb) * m)
        - f32::from_bits(0x3f4f_0560) * s;
    (ok_l, ok_a, ok_b)
}

/// Converts OKLab `(L, a, b)` to linear sRGB, entirely in single precision,
/// the cubes correctly rounded ([`cube`]).
fn oklab_to_linear_srgb(ok_l: f32, ok_a: f32, ok_b: f32) -> (f32, f32, f32) {
    let l = (ok_l + f32::from_bits(0x3eca_ecc9) * ok_a) + f32::from_bits(0x3e5c_fba9) * ok_b;
    let m = (ok_l - f32::from_bits(0x3dd8_308c) * ok_a) - f32::from_bits(0x3d82_c5fa) * ok_b;
    let s = (ok_l - f32::from_bits(0x3db7_437b) * ok_a) - f32::from_bits(0x3fa5_4f66) * ok_b;

    let l = cube(l);
    let m = cube(m);
    let s = cube(s);

    let r = (f32::from_bits(0x4082_74ab) * l - f32::from_bits(0x4053_b18c) * m)
        + f32::from_bits(0x3e6c_8362) * s;
    let g = (-f32::from_bits(0x3fa2_5c2d) * l + f32::from_bits(0x4027_0644) * m)
        - f32::from_bits(0x3eae_c16a) * s;
    let b = (-f32::from_bits(0x3b89_7f53) * l - f32::from_bits(0x3f34_133e) * m)
        + f32::from_bits(0x3fda_931e) * s;
    (r, g, b)
}

/// Applies the scheme's lightness limit to an OKLab lightness, evaluated in double and
/// stored back as a single.
fn limit_lightness(ok_l: f32, scheme: Scheme) -> f32 {
    let l = f64::from(ok_l);
    let limited = match scheme {
        Scheme::Light => {
            if 0.5 < l {
                0.5
            } else {
                l
            }
        }
        Scheme::Dark => {
            if 0.85 > l {
                0.85
            } else {
                l
            }
        }
    };
    limited as f32
}

/// Clamps an alpha to `0..=1`, testing the upper bound first.
#[expect(
    clippy::manual_clamp,
    reason = "the described clamp, its upper bound tested first"
)]
fn clamp_alpha(a: f32) -> f32 {
    if a > 1.0 {
        1.0
    } else if a < 0.0 {
        0.0
    } else {
        a
    }
}

/// Computes the standalone colour of `c` (straight-alpha RGBA: red, green, blue and alpha)
/// for `scheme`: the colour converted to OKLab, its lightness capped at 0.5 (light) or raised
/// to at least the single nearest 0.85 (dark), and converted back to sRGB. Returns red, green,
/// blue and alpha; the colour channels are not clamped and may lie outside `0..=1`, the alpha
/// is clamped to `0..=1`.
pub fn adw_standalone(c: [f32; 4], scheme: Scheme) -> [f32; 4] {
    let [r, g, b, a] = c;
    let (ok_l, ok_a, ok_b) =
        linear_srgb_to_oklab(srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b));
    let ok_l = limit_lightness(ok_l, scheme);
    let alpha = clamp_alpha(a);
    let (lr, lg, lb) = oklab_to_linear_srgb(ok_l, ok_a, ok_b);
    [
        linear_to_srgb(lr),
        linear_to_srgb(lg),
        linear_to_srgb(lb),
        alpha,
    ]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    const TABLE: &str = include_str!("../../tests/vectors/adw_standalone.tsv");

    fn parse_srgb(cell: &str) -> Option<[f32; 4]> {
        let inner = cell.trim().strip_prefix("srgb(")?.strip_suffix(')')?;
        let mut out = [0.0_f32; 4];
        let mut parts = inner.split(',');
        for slot in out.iter_mut() {
            *slot = parts.next()?.trim().parse().ok()?;
        }
        if parts.next().is_some() {
            return None;
        }
        Some(out)
    }

    fn parse_scheme(cell: &str) -> Option<Scheme> {
        match cell.trim() {
            "light" => Some(Scheme::Light),
            "dark" => Some(Scheme::Dark),
            _ => None,
        }
    }

    #[test]
    #[expect(
        clippy::neg_cmp_op_on_partial_ord,
        reason = "a NaN difference fails the row"
    )]
    fn vectors() {
        let mut rows = 0_usize;
        let mut max_diff = [0.0_f64; 4];
        let mut failures = Vec::new();
        for (index, line) in TABLE.lines().enumerate() {
            if line.starts_with('#') || line.trim().is_empty() || line.starts_with("c\t") {
                continue;
            }
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), 3, "line {}: {line}", index + 1);
            let c = parse_srgb(cells[0]).expect("input colour");
            let scheme = parse_scheme(cells[1]).expect("scheme");
            let expected = parse_srgb(cells[2]).expect("result colour");
            let got = adw_standalone(c, scheme);
            for k in 0..4 {
                let diff = (f64::from(got[k]) * 255.0 - f64::from(expected[k]) * 255.0).abs();
                if diff > max_diff[k] {
                    max_diff[k] = diff;
                }
                if !(diff <= 1e-3) {
                    failures.push(format!(
                        "line {}: channel {k}: got {:?} expected {:?} ({line})",
                        index + 1,
                        got,
                        expected
                    ));
                }
            }
            rows += 1;
        }
        println!("rows: {rows}, max diff (0-255) r g b a: {max_diff:?}");
        assert_eq!(rows, 628);
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    /// A finite nonzero double's `(m, e)`, its value `m · 2^e` exactly, `m`
    /// odd.
    fn parts(x: f64) -> (u128, i32) {
        let bits = x.abs().to_bits();
        let exponent = i32::try_from((bits >> 52) & 0x7ff).unwrap();
        let fraction = u128::from(bits & ((1_u64 << 52) - 1));
        let (m, e) = if exponent == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1_u128 << 52), exponent - 1075)
        };
        let zeros = i32::try_from(m.trailing_zeros()).unwrap();
        (m >> zeros, e + zeros)
    }

    fn bits_of(x: u128) -> i32 {
        128 - i32::try_from(x.leading_zeros()).unwrap()
    }

    /// `a · 2^ea` against `b · 2^eb`, both positive, exactly.
    fn compare(a: u128, ea: i32, b: u128, eb: i32) -> std::cmp::Ordering {
        let (la, lb) = (bits_of(a) + ea, bits_of(b) + eb);
        if la != lb {
            return la.cmp(&lb);
        }
        if ea >= eb {
            (a << (ea - eb)).cmp(&b)
        } else {
            a.cmp(&(b << (eb - ea)))
        }
    }

    /// The cube root of a positive finite single rounded once, by exact
    /// integer arithmetic: the single whose two midpoints, cubed exactly,
    /// bracket it (a midpoint's cube, of an odd 25-bit mantissa, is never
    /// a single).
    fn cbrt_reference(x: f32) -> f32 {
        use std::cmp::Ordering::{Greater, Less};
        let (xm, xe) = parts(f64::from(x));
        let cubed = |m: f64| {
            let (mm, me) = parts(m);
            compare(mm * mm * mm, 3 * me, xm, xe)
        };
        let mut r = f64::from(x).cbrt() as f32;
        loop {
            let near = f64::from(r);
            let above = f32::from_bits(r.to_bits() + 1);
            let below = f32::from_bits(r.to_bits() - 1);
            if cubed((near + f64::from(above)) / 2.0) == Less {
                r = above;
            } else if cubed((near + f64::from(below)) / 2.0) == Greater {
                r = below;
            } else {
                return r;
            }
        }
    }

    /// The cube of a positive finite single rounded once (ties to even),
    /// by exact integer arithmetic.
    fn cube_reference(x: f32) -> f32 {
        let (m, e) = parts(f64::from(x));
        let (c, ce) = (m * m * m, 3 * e);
        let top = bits_of(c) + ce;
        if top > 128 {
            return f32::INFINITY;
        }
        let lsb = (top - 24).max(-149);
        let shift = lsb - ce;
        let q = if shift <= 0 {
            c << -shift
        } else if shift >= 128 {
            0
        } else {
            let q = c >> shift;
            let rest = c - (q << shift);
            let half = 1_u128 << (shift - 1);
            if rest > half || (rest == half && q & 1 == 1) {
                q + 1
            } else {
                q
            }
        };
        let v = (q as f64) * 2_f64.powi(lsb);
        if v > f64::from(f32::MAX) {
            f32::INFINITY
        } else {
            v as f32
        }
    }

    /// `cube` and `cbrt` against the exact references on `x` and `-x`.
    fn rounded_once(x: f32) -> Result<(), String> {
        let (root, cubed) = (cbrt_reference(x), cube_reference(x));
        let pairs = [
            (cbrt(x), root),
            (cbrt(-x), -root),
            (cube(x), cubed),
            (cube(-x), -cubed),
        ];
        if pairs
            .iter()
            .all(|(got, want)| got.to_bits() == want.to_bits())
        {
            Ok(())
        } else {
            Err(format!("{x:e}: {pairs:?}"))
        }
    }

    /// The cube and cube root are each rounded once, the same on every
    /// platform: every single in `0.5..2`, which holds most of the OKLab
    /// round trip's operands, and every 4099th positive single besides,
    /// each also negated; zero, the infinities and NaN as they are.
    #[test]
    fn cube_and_cbrt_rounded_once() {
        let range = (0.5_f32.to_bits()..2.0_f32.to_bits()).chain((1..0x7f80_0000).step_by(4099));
        let wrong: Vec<String> = range
            .filter_map(|bits| rounded_once(f32::from_bits(bits)).err())
            .take(10)
            .collect();
        assert!(wrong.is_empty(), "{wrong:?}");
        for x in [0.0_f32, -0.0, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(cbrt(x).to_bits(), x.to_bits());
            assert_eq!(cube(x).to_bits(), x.to_bits());
        }
        assert!(cbrt(f32::NAN).is_nan() && cube(f32::NAN).is_nan());
    }

    /// Every positive finite single, as `cube_and_cbrt_rounded_once`
    /// samples them (about 7 CPU minutes, on all cores):
    /// `cargo test -p native-theme --lib cube_and_cbrt_exhaustive -- --ignored`.
    #[test]
    #[ignore = "every single: minutes; run by hand when cube or cbrt changes"]
    fn cube_and_cbrt_exhaustive() {
        let threads = std::thread::available_parallelism().map_or(1, usize::from);
        let end = 0x7f80_0000_u32;
        let chunk = end / u32::try_from(threads).unwrap() + 1;
        let wrong: Vec<String> = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..u32::try_from(threads).unwrap())
                .map(|t| {
                    scope.spawn(move || {
                        let from = (t * chunk).max(1);
                        let to = ((t + 1) * chunk).min(end);
                        (from..to)
                            .filter_map(|bits| rounded_once(f32::from_bits(bits)).err())
                            .take(10)
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            workers
                .into_iter()
                .flat_map(|w| w.join().unwrap())
                .collect()
        });
        assert!(wrong.is_empty(), "{wrong:?}");
    }

    #[test]
    fn documented_blue_accent_dark() {
        let c = [
            f32::from(0x35_u8) / 255.0,
            f32::from(0x84_u8) / 255.0,
            f32::from(0xe4_u8) / 255.0,
        ];
        let got = adw_standalone([c[0], c[1], c[2], 1.0], Scheme::Dark);
        let want = [0.504_311_3_f32, 0.817_23, 1.218_716_5];
        for k in 0..3 {
            assert!((got[k] - want[k]).abs() < 1e-6, "{got:?}");
        }
    }
}
