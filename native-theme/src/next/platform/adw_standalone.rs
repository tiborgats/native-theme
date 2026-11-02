//! The colour GTK computes for libadwaita's standalone colours through CSS: the input
//! colour taken to OKLab, its lightness limited per scheme, and the result taken straight
//! back to sRGB, with no clamping of the colour channels.

use crate::next::selector::Scheme;

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

/// Converts linear sRGB to OKLab `(L, a, b)`, entirely in single precision.
fn linear_srgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = (f32::from_bits(0x3ed3_0eb1) * r + f32::from_bits(0x3f09_4d17) * g)
        + f32::from_bits(0x3d52_b909) * b;
    let m = (f32::from_bits(0x3e58_fd3b) * r + f32::from_bits(0x3f2e_4253) * g)
        + f32::from_bits(0x3ddb_f2f0) * b;
    let s = (f32::from_bits(0x3db4_d7ec) * r + f32::from_bits(0x3e90_3d74) * g)
        + f32::from_bits(0x3f21_4649) * b;

    let l = l.cbrt();
    let m = m.cbrt();
    let s = s.cbrt();

    let ok_l = (f32::from_bits(0x3e57_8152) * l + f32::from_bits(0x3f4b_2a89) * m)
        - f32::from_bits(0x3b85_6ece) * s;
    let ok_a = (f32::from_bits(0x3ffd_2f0e) * l - f32::from_bits(0x401b_6e0e) * m)
        + f32::from_bits(0x3ee6_b438) * s;
    let ok_b = (f32::from_bits(0x3cd4_34b4) * l + f32::from_bits(0x3f48_63bb) * m)
        - f32::from_bits(0x3f4f_0560) * s;
    (ok_l, ok_a, ok_b)
}

/// Converts OKLab `(L, a, b)` to linear sRGB, entirely in single precision.
fn oklab_to_linear_srgb(ok_l: f32, ok_a: f32, ok_b: f32) -> (f32, f32, f32) {
    let l = (ok_l + f32::from_bits(0x3eca_ecc9) * ok_a) + f32::from_bits(0x3e5c_fba9) * ok_b;
    let m = (ok_l - f32::from_bits(0x3dd8_308c) * ok_a) - f32::from_bits(0x3d82_c5fa) * ok_b;
    let s = (ok_l - f32::from_bits(0x3db7_437b) * ok_a) - f32::from_bits(0x3fa5_4f66) * ok_b;

    let l = l.powf(3.0);
    let m = m.powf(3.0);
    let s = s.powf(3.0);

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

    const TABLE: &str = include_str!("../../../tests/vectors/adw_standalone.tsv");

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
