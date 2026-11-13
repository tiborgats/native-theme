//! Shared arithmetic of the KDE colour functions: Qt's colour storage (`QColor`'s 16-bit and
//! half-precision channels), KDE's HCY colour space and the `KColorUtils` mix, tint and shade on
//! stored colours. Every step is one IEEE operation, never fused.

/// A model colour: straight-alpha `[r, g, b, a]`, nominally within `[0, 1]`.
pub(super) type Rgba = [f32; 4];

/// A colour as Qt stores it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum QtColor {
    /// Four 16-bit channels `R, G, B, A`.
    Standard([u16; 4]),
    /// Four half-precision values, held as the `f32`s they equal exactly.
    Extended([f32; 4]),
    /// No colour.
    Invalid,
}

/// Rec. 709 red weight.
pub(super) const K_R: f64 = 0.2126;
/// Rec. 709 green weight.
pub(super) const K_G: f64 = 0.7152;
/// Rec. 709 blue weight.
pub(super) const K_B: f64 = 0.0722;

/// `qRound(x * 65535)` in single precision: one multiplication, one addition, truncation.
pub(super) fn q16(x: f32) -> u16 {
    let p = x * 65535.0_f32;
    let s = p + 0.5_f32;
    s as u16
}

/// `x` rounded to the nearest half-precision value (ties to even), as an `f32`; a magnitude of
/// at least 65520 becomes an infinity.
pub(super) fn round_to_half(x: f32) -> f32 {
    if !x.is_finite() {
        return x;
    }
    let ax = x.abs();
    if ax >= 65520.0 {
        return if x < 0.0 {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };
    }
    let min_normal = f32::from_bits(113 << 23);
    let quantum = if ax < min_normal {
        f32::from_bits(103 << 23)
    } else {
        match (ax.to_bits() & 0x7f80_0000).checked_sub(10 << 23) {
            Some(bits) => f32::from_bits(bits),
            None => return x,
        }
    };
    (x / quantum).round_ties_even() * quantum
}

/// `QColor::fromRgbF` with single arguments.
pub(super) fn from_rgbf(r: f32, g: f32, b: f32, a: f32) -> QtColor {
    let unit = |x: f32| (0.0..=1.0).contains(&x);
    if !unit(a) {
        QtColor::Invalid
    } else if unit(r) && unit(g) && unit(b) {
        QtColor::Standard([q16(r), q16(g), q16(b), q16(a)])
    } else if r.is_nan() || g.is_nan() || b.is_nan() {
        QtColor::Invalid
    } else {
        QtColor::Extended([r, g, b, a].map(round_to_half))
    }
}

/// `QColor::fromRgbF` with double arguments, each first rounded to a single.
pub(super) fn from_rgbd(r: f64, g: f64, b: f64, a: f64) -> QtColor {
    from_rgbf(r as f32, g as f32, b as f32, a as f32)
}

/// A model colour entering Qt.
pub(super) fn enter(c: Rgba) -> QtColor {
    let [r, g, b, a] = c;
    from_rgbf(r, g, b, a)
}

/// The four channels as `redF()` .. `alphaF()` read them; an invalid colour reads as zeros.
pub(super) fn read(c: QtColor) -> Rgba {
    match c {
        QtColor::Standard(s) => s.map(|v| f32::from(v) / 65535.0_f32),
        QtColor::Extended(h) => h,
        QtColor::Invalid => [0.0; 4],
    }
}

/// `Qt::transparent`.
pub(super) const TRANSPARENT: QtColor = QtColor::Standard([0, 0, 0, 0]);

/// KDE's normalize: `x` inside `(0, 1)`, 1 at or above 1, else 0 (NaN too).
pub(super) fn norm(x: f64) -> f64 {
    if x > 0.0 && x < 1.0 {
        x
    } else if x >= 1.0 {
        1.0
    } else {
        0.0
    }
}

/// `a < b ? b : a`.
pub(super) fn maxd(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `a < b ? a : b`.
pub(super) fn mind(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// A colour in KDE's HCY space: hue, chroma, luma, alpha.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Hcy {
    /// Hue, unwrapped.
    pub(super) h: f64,
    /// Chroma.
    pub(super) c: f64,
    /// Luma.
    pub(super) y: f64,
    /// Alpha.
    pub(super) a: f64,
}

fn linear(c: QtColor) -> (f64, f64, f64, f64) {
    let [r0, g0, b0, a] = read(c);
    (
        norm(f64::from(r0)).powf(2.2),
        norm(f64::from(g0)).powf(2.2),
        norm(f64::from(b0)).powf(2.2),
        f64::from(a),
    )
}

fn luma_of(r: f64, g: f64, b: f64) -> f64 {
    (r * K_R + g * K_G) + b * K_B
}

/// The luma of a stored colour.
pub(super) fn luma(c: QtColor) -> f64 {
    let (r, g, b, _) = linear(c);
    luma_of(r, g, b)
}

/// A stored colour in HCY.
pub(super) fn to_hcy(c: QtColor) -> Hcy {
    let (r, g, b, a) = linear(c);
    let y = luma_of(r, g, b);
    let p = maxd(maxd(r, g), b);
    let n = mind(mind(r, g), b);
    let d = 6.0 * (p - n);
    let h = if n == p {
        0.0
    } else if r == p {
        (g - b) / d
    } else if g == p {
        (b - r) / d + 1.0 / 3.0
    } else {
        (r - g) / d + 2.0 / 3.0
    };
    let c = if r == g && g == b {
        0.0
    } else {
        maxd((y - n) / y, (p - y) / (1.0 - y))
    };
    Hcy { h, c, y, a }
}

#[allow(
    clippy::modulo_arithmetic,
    reason = "a float remainder, which cannot panic"
)]
fn wrap(h: f64) -> f64 {
    let q = h % 1.0;
    if q < 0.0 {
        1.0 + q
    } else if q > 0.0 {
        q
    } else {
        0.0
    }
}

/// An HCY colour back to a stored colour.
pub(super) fn from_hcy(x: Hcy) -> QtColor {
    let hh = wrap(x.h);
    let cc = norm(x.c);
    let yy = norm(x.y);
    let hs = hh * 6.0;
    let (sextant, th, tm) = if hs < 1.0 {
        (0, hs, K_R + K_G * hs)
    } else if hs < 2.0 {
        let th = 2.0 - hs;
        (1, th, K_G + K_R * th)
    } else if hs < 3.0 {
        let th = hs - 2.0;
        (2, th, K_G + K_B * th)
    } else if hs < 4.0 {
        let th = 4.0 - hs;
        (3, th, K_B + K_G * th)
    } else if hs < 5.0 {
        let th = hs - 4.0;
        (4, th, K_B + K_R * th)
    } else {
        let th = 6.0 - hs;
        (5, th, K_R + K_B * th)
    };
    let (tp, to, tn) = if tm >= yy {
        (
            yy + ((yy * cc) * (1.0 - tm)) / tm,
            yy + ((yy * cc) * (th - tm)) / tm,
            yy - (yy * cc),
        )
    } else {
        let u = (1.0 - yy) * cc;
        (
            yy + u,
            yy + (u * (th - tm)) / (1.0 - tm),
            yy - (u * tm) / (1.0 - tm),
        )
    };
    let (r, g, b) = match sextant {
        0 => (tp, to, tn),
        1 => (to, tp, tn),
        2 => (tn, tp, to),
        3 => (tn, to, tp),
        4 => (to, tn, tp),
        _ => (tp, tn, to),
    };
    let ig = |t: f64| norm(t).powf(1.0 / 2.2);
    from_rgbd(ig(r), ig(g), ig(b), x.a)
}

/// `KColorUtils::mix` on stored colours with a double bias.
pub(super) fn mix_q(a: QtColor, b: QtColor, t: f64) -> QtColor {
    if t.is_nan() || t <= 0.0 {
        return a;
    }
    if t >= 1.0 {
        return b;
    }
    let [r1, g1, b1, a1] = read(a);
    let [r2, g2, b2, a2] = read(b);
    let alpha = f64::from(a1) + (f64::from(a2) - f64::from(a1)) * t;
    if alpha <= 0.0 {
        return TRANSPARENT;
    }
    let channel = |x1: f32, x2: f32| {
        let p1 = f64::from(x1 * a1);
        let p2 = f64::from(x2 * a2);
        let m = p1 + (p2 - p1) * t;
        maxd(0.0, mind(m, 1.0)) / alpha
    };
    from_rgbd(channel(r1, r2), channel(g1, g2), channel(b1, b2), alpha)
}

fn contrast_ratio(y1: f64, y2: f64) -> f64 {
    if y1 > y2 {
        (y1 + 0.05) / (y2 + 0.05)
    } else {
        (y2 + 0.05) / (y1 + 0.05)
    }
}

/// `KColorUtils::tint` on stored colours with a double amount.
pub(super) fn tint_q(a: QtColor, b: QtColor, k: f64) -> QtColor {
    if k.is_nan() || k <= 0.0 {
        return a;
    }
    if k >= 1.0 {
        return b;
    }
    let yb = luma(a);
    let ri = contrast_ratio(yb, luma(b));
    let rg = 1.0 + (((ri + 1.0) * k) * k) * k;
    let mut u = 1.0_f64;
    let mut l = 0.0_f64;
    let mut s = 0.5_f64;
    for _ in 0..12 {
        s = 0.5 * (l + u);
        let m = mix_q(a, b, s.powf(0.3));
        let ym = yb + (luma(m) - yb) * s;
        if contrast_ratio(yb, ym) > rg {
            u = s;
        } else {
            l = s;
        }
    }
    let m = mix_q(a, b, s.powf(0.3));
    let mut x = to_hcy(m);
    x.y = yb + (x.y - yb) * s;
    from_hcy(x)
}

/// `KColorUtils::shade` on a stored colour with double offsets.
pub(super) fn shade_q(c: QtColor, ky: f64, kc: f64) -> QtColor {
    let mut x = to_hcy(c);
    x.y = norm(x.y + ky);
    x.c = norm(x.c + kc);
    from_hcy(x)
}
