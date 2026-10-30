//! `kde_shade(c, role, contrast)` (S2.2): KColorScheme's `shade`,
//! `kcolorscheme@6.31.0 src/kcolorscheme.cpp:566-616`, with no chroma
//! adjustment, over KColorUtils' HCY space (`kguiaddons@6.31.0
//! src/colors/kcolorspaces.cpp`, `kcolorutils.cpp:81-95`) and Qt's `QColor`.

use super::qcolor::{Rgb16, unit};
use crate::next::color::Color;

/// KColorScheme's `ShadeRole` (`kcolorscheme.h:250-254`), `kde_shade`'s
/// `role`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KdeShadeRole {
    /// `LightShade`.
    Light,
    /// `MidlightShade`.
    Midlight,
    /// `MidShade`.
    Mid,
    /// `DarkShade`.
    Dark,
    /// `ShadowShade`.
    Shadow,
}

impl KdeShadeRole {
    /// Every role, in declaration order.
    pub(crate) const ALL: [KdeShadeRole; 5] = [
        KdeShadeRole::Light,
        KdeShadeRole::Midlight,
        KdeShadeRole::Mid,
        KdeShadeRole::Dark,
        KdeShadeRole::Shadow,
    ];

    /// Every role's name, in [`KdeShadeRole::ALL`]'s order, as expressions
    /// spell it.
    pub(crate) const NAMES: [&'static str; 5] = ["light", "midlight", "mid", "dark", "shadow"];

    /// The role `name` names.
    pub(crate) fn from_name(name: &str) -> Option<KdeShadeRole> {
        KdeShadeRole::NAMES
            .iter()
            .position(|held| *held == name)
            .and_then(|index| KdeShadeRole::ALL.get(index).copied())
    }
}

/// `KColorSpaces`' luma coefficients, Rec. 709 (`kcolorspaces.cpp:25-29`).
const YC: [f64; 3] = [0.2126, 0.7152, 0.0722];

/// `normalize` (`kguiaddons_colorhelpers_p.h:15-18`): clamped to `0..=1`, a
/// NaN 0.
fn normalize(a: f64) -> f64 {
    if a < 1.0 {
        if a > 0.0 { a } else { 0.0 }
    } else {
        1.0
    }
}

/// `wrap(a)` (`kcolorspaces.cpp:16-20`): `fmod(a, 1)` taken into `0..1`, a
/// NaN 0.
fn wrap(a: f64) -> f64 {
    let r = a.rem_euclid(1.0);
    if r > 0.0 { r } else { 0.0 }
}

fn gamma(n: f64) -> f64 {
    normalize(n).powf(2.2)
}

fn igamma(n: f64) -> f64 {
    normalize(n).powf(1.0 / 2.2)
}

fn lumag(r: f64, g: f64, b: f64) -> f64 {
    r * YC[0] + g * YC[1] + b * YC[2]
}

/// A colour in `KColorSpaces::KHCY`.
#[derive(Clone, Copy, Debug)]
struct Khcy {
    h: f64,
    c: f64,
    y: f64,
    a: f64,
}

impl Khcy {
    /// `KHCY(const QColor &)` (`kcolorspaces.cpp:57-87`), over the
    /// channels' `redF()` and the others, Qt 6's floats.
    fn new(color: Rgb16) -> Khcy {
        let channel = |x: u16| f64::from(unit(x));
        let r = gamma(channel(color.r));
        let g = gamma(channel(color.g));
        let b = gamma(channel(color.b));
        let a = channel(color.a);
        let y = lumag(r, g, b);
        let p = r.max(g).max(b);
        let n = r.min(g).min(b);
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
            ((y - n) / y).max((p - y) / (1.0 - y))
        };
        Khcy { h, c, y, a }
    }

    /// `KHCY::qColor()` (`kcolorspaces.cpp:89-148`): the channels as doubles,
    /// handed to `QColor::fromRgbF`'s floats.
    fn to_rgb(self) -> Rgb16 {
        let h = wrap(self.h);
        let c = normalize(self.c);
        let y = normalize(self.y);
        let hs = h * 6.0;
        let (th, tm) = if hs < 1.0 {
            (hs, YC[0] + YC[1] * hs)
        } else if hs < 2.0 {
            let th = 2.0 - hs;
            (th, YC[1] + YC[0] * th)
        } else if hs < 3.0 {
            let th = hs - 2.0;
            (th, YC[1] + YC[2] * th)
        } else if hs < 4.0 {
            let th = 4.0 - hs;
            (th, YC[2] + YC[1] * th)
        } else if hs < 5.0 {
            let th = hs - 4.0;
            (th, YC[2] + YC[0] * th)
        } else {
            let th = 6.0 - hs;
            (th, YC[0] + YC[2] * th)
        };
        let (tn, to, tp) = if tm >= y {
            (
                y - y * c,
                y + y * c * (th - tm) / tm,
                y + y * c * (1.0 - tm) / tm,
            )
        } else {
            (
                y - (1.0 - y) * c * tm / (1.0 - tm),
                y + (1.0 - y) * c * (th - tm) / (1.0 - tm),
                y + (1.0 - y) * c,
            )
        };
        let (r, g, b) = if hs < 1.0 {
            (tp, to, tn)
        } else if hs < 2.0 {
            (to, tp, tn)
        } else if hs < 3.0 {
            (tn, tp, to)
        } else if hs < 4.0 {
            (tn, to, tp)
        } else if hs < 5.0 {
            (to, tn, tp)
        } else {
            (tp, tn, to)
        };
        let float = |x: f64| x as f32;
        Rgb16::from_color(Color {
            r: float(igamma(r)),
            g: float(igamma(g)),
            b: float(igamma(b)),
            a: float(self.a),
        })
    }
}

/// `KColorUtils::shade(color, ky, 0)` (`kcolorutils.cpp:89-95`).
fn kcu_shade(color: Rgb16, ky: f64) -> Rgb16 {
    let mut c = Khcy::new(color);
    c.y = normalize(c.y + ky);
    c.c = normalize(c.c);
    c.to_rgb()
}

/// `KColorUtils::darken(color, ky, 1)` (`kcolorutils.cpp:81-87`).
fn kcu_darken(color: Rgb16, ky: f64) -> Rgb16 {
    let mut c = Khcy::new(color);
    c.y = normalize(c.y * (1.0 - ky));
    c.c = normalize(c.c);
    c.to_rgb()
}

/// KColorScheme's `shade(color, role, contrast)` with no chroma adjustment
/// (`kcolorscheme.cpp:566-616`), `contrast` as kdeglobals' `[KDE] contrast`
/// stores it: scaled by 0.1 as `KColorScheme::contrastF` does (`:526`),
/// then clamped to `-1..=1` (a NaN 1) as `shade` does (`:569`); a
/// non-finite operand is zero first (S2.2's totality).
pub(crate) fn kde_shade(c: Color, role: KdeShadeRole, contrast: f32) -> Color {
    let contrast = if contrast.is_finite() {
        0.1 * f64::from(contrast)
    } else {
        0.0
    };
    let contrast = if 1.0 > contrast {
        if -1.0 < contrast { contrast } else { -1.0 }
    } else {
        1.0
    };
    let color = Rgb16::from_color(c);
    let channel = |x: u16| f64::from(unit(x));
    let y = lumag(
        gamma(channel(color.r)),
        gamma(channel(color.g)),
        gamma(channel(color.b)),
    );
    let yi = 1.0 - y;
    let shaded = if y < 0.006 {
        let amount = match role {
            KdeShadeRole::Light => 0.05 + 0.95 * contrast,
            KdeShadeRole::Mid => 0.01 + 0.20 * contrast,
            KdeShadeRole::Dark => 0.02 + 0.40 * contrast,
            KdeShadeRole::Midlight | KdeShadeRole::Shadow => 0.03 + 0.60 * contrast,
        };
        kcu_shade(color, amount)
    } else if y > 0.93 {
        let amount = match role {
            KdeShadeRole::Midlight => -0.02 - 0.20 * contrast,
            KdeShadeRole::Dark => -0.06 - 0.60 * contrast,
            KdeShadeRole::Shadow => -0.10 - 0.90 * contrast,
            KdeShadeRole::Light | KdeShadeRole::Mid => -0.04 - 0.40 * contrast,
        };
        kcu_shade(color, amount)
    } else {
        let light_amount = (0.05 + y * 0.55) * (0.25 + contrast * 0.75);
        let dark_amount = (-y) * (0.55 + contrast * 0.35);
        match role {
            KdeShadeRole::Light => kcu_shade(color, light_amount),
            KdeShadeRole::Midlight => kcu_shade(color, (0.15 + 0.35 * yi) * light_amount),
            KdeShadeRole::Mid => kcu_shade(color, (0.35 + 0.15 * y) * dark_amount),
            KdeShadeRole::Dark => kcu_shade(color, dark_amount),
            KdeShadeRole::Shadow => kcu_darken(kcu_shade(color, dark_amount), 0.5 + 0.3 * y),
        }
    };
    shaded.to_color()
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

    /// One row: an opaque 8-bit colour, kdeglobals' contrast step, the role,
    /// and the 16-bit colour of the result.
    type Row = ([u8; 3], f32, KdeShadeRole, [u16; 4]);

    /// Rows computed from the cited sources' formulas by a separate
    /// implementation of them in double precision with Qt 6's float
    /// channels (KColorScheme's `contrastF` and `shade`, KHCY, `shade` and
    /// `darken`, `QColor::redF` and `fromRgbF`): Breeze Light's and Dark's
    /// window backgrounds, a very dark and a very light colour (the first
    /// two branches), a saturated blue, a contrast above ten (clamped) and
    /// a negative one. Synthetic inputs.
    #[test]
    fn shade_is_kcolorschemes() {
        let rows: [Row; 45] = [
            (
                [239, 240, 241],
                4.0,
                KdeShadeRole::Light,
                [65535, 65535, 65535, 65535],
            ),
            (
                [239, 240, 241],
                4.0,
                KdeShadeRole::Midlight,
                [63312, 63448, 63585, 65535],
            ),
            (
                [239, 240, 241],
                4.0,
                KdeShadeRole::Mid,
                [50399, 51464, 52509, 65535],
            ),
            (
                [239, 240, 241],
                4.0,
                KdeShadeRole::Dark,
                [34977, 36393, 37752, 65535],
            ),
            (
                [239, 240, 241],
                4.0,
                KdeShadeRole::Shadow,
                [18207, 18944, 19651, 65535],
            ),
            (
                [32, 35, 38],
                4.0,
                KdeShadeRole::Light,
                [14585, 15952, 17319, 65535],
            ),
            (
                [32, 35, 38],
                4.0,
                KdeShadeRole::Midlight,
                [11896, 13011, 14126, 65535],
            ),
            (
                [32, 35, 38],
                4.0,
                KdeShadeRole::Mid,
                [7247, 7927, 8606, 65535],
            ),
            (
                [32, 35, 38],
                4.0,
                KdeShadeRole::Dark,
                [4829, 5282, 5735, 65535],
            ),
            (
                [32, 35, 38],
                4.0,
                KdeShadeRole::Shadow,
                [3512, 3841, 4171, 65535],
            ),
            (
                [0, 0, 0],
                4.0,
                KdeShadeRole::Light,
                [44655, 44655, 44655, 65535],
            ),
            (
                [0, 0, 0],
                4.0,
                KdeShadeRole::Midlight,
                [36141, 36141, 36141, 65535],
            ),
            (
                [0, 0, 0],
                4.0,
                KdeShadeRole::Mid,
                [21935, 21935, 21935, 65535],
            ),
            (
                [0, 0, 0],
                4.0,
                KdeShadeRole::Dark,
                [30058, 30058, 30058, 65535],
            ),
            (
                [0, 0, 0],
                4.0,
                KdeShadeRole::Shadow,
                [36141, 36141, 36141, 65535],
            ),
            (
                [1, 2, 3],
                7.0,
                KdeShadeRole::Light,
                [52353, 56591, 63528, 65535],
            ),
            (
                [1, 2, 3],
                7.0,
                KdeShadeRole::Midlight,
                [34748, 46387, 61590, 65535],
            ),
            (
                [1, 2, 3],
                7.0,
                KdeShadeRole::Mid,
                [14246, 28491, 42737, 65535],
            ),
            (
                [1, 2, 3],
                7.0,
                KdeShadeRole::Dark,
                [19521, 39041, 58562, 65535],
            ),
            (
                [1, 2, 3],
                7.0,
                KdeShadeRole::Shadow,
                [34748, 46387, 61590, 65535],
            ),
            (
                [255, 255, 255],
                4.0,
                KdeShadeRole::Light,
                [59214, 59214, 59214, 65535],
            ),
            (
                [255, 255, 255],
                4.0,
                KdeShadeRole::Midlight,
                [62470, 62470, 62470, 65535],
            ),
            (
                [255, 255, 255],
                4.0,
                KdeShadeRole::Mid,
                [59214, 59214, 59214, 65535],
            ),
            (
                [255, 255, 255],
                4.0,
                KdeShadeRole::Dark,
                [55727, 55727, 55727, 65535],
            ),
            (
                [255, 255, 255],
                4.0,
                KdeShadeRole::Shadow,
                [49526, 49526, 49526, 65535],
            ),
            (
                [250, 251, 252],
                0.0,
                KdeShadeRole::Light,
                [62759, 63320, 63879, 65535],
            ),
            (
                [250, 251, 252],
                0.0,
                KdeShadeRole::Midlight,
                [63510, 63917, 64323, 65535],
            ),
            (
                [250, 251, 252],
                0.0,
                KdeShadeRole::Mid,
                [62759, 63320, 63879, 65535],
            ),
            (
                [250, 251, 252],
                0.0,
                KdeShadeRole::Dark,
                [61997, 62717, 63430, 65535],
            ),
            (
                [250, 251, 252],
                0.0,
                KdeShadeRole::Shadow,
                [60438, 61488, 62522, 65535],
            ),
            (
                [61, 174, 233],
                7.0,
                KdeShadeRole::Light,
                [37530, 52828, 64075, 65535],
            ),
            (
                [61, 174, 233],
                7.0,
                KdeShadeRole::Midlight,
                [21175, 48262, 63634, 65535],
            ),
            (
                [61, 174, 233],
                7.0,
                KdeShadeRole::Mid,
                [13128, 37447, 50145, 65535],
            ),
            (
                [61, 174, 233],
                7.0,
                KdeShadeRole::Dark,
                [7628, 21759, 29137, 65535],
            ),
            (
                [61, 174, 233],
                7.0,
                KdeShadeRole::Shadow,
                [4954, 14132, 18924, 65535],
            ),
            (
                [61, 174, 233],
                12.0,
                KdeShadeRole::Light,
                [42710, 54775, 64277, 65535],
            ),
            (
                [61, 174, 233],
                12.0,
                KdeShadeRole::Midlight,
                [24886, 49070, 63709, 65535],
            ),
            (
                [61, 174, 233],
                12.0,
                KdeShadeRole::Mid,
                [12745, 36355, 48682, 65535],
            ),
            (
                [61, 174, 233],
                12.0,
                KdeShadeRole::Dark,
                [5504, 15701, 21025, 65535],
            ),
            (
                [61, 174, 233],
                12.0,
                KdeShadeRole::Shadow,
                [3575, 10198, 13655, 65535],
            ),
            (
                [128, 64, 32],
                -3.0,
                KdeShadeRole::Light,
                [33327, 16664, 8332, 65535],
            ),
            (
                [128, 64, 32],
                -3.0,
                KdeShadeRole::Midlight,
                [33100, 16550, 8275, 65535],
            ),
            (
                [128, 64, 32],
                -3.0,
                KdeShadeRole::Mid,
                [30370, 15185, 7592, 65535],
            ),
            (
                [128, 64, 32],
                -3.0,
                KdeShadeRole::Dark,
                [25172, 12586, 6293, 65535],
            ),
            (
                [128, 64, 32],
                -3.0,
                KdeShadeRole::Shadow,
                [17955, 8977, 4489, 65535],
            ),
        ];
        for ([r, g, b], contrast, role, expected) in rows {
            let out = kde_shade(rgb8(r, g, b), role, contrast);
            assert_eq!(sixteen(out), expected, "{r},{g},{b} {role:?} at {contrast}");
        }
    }

    #[test]
    fn roles_by_name_and_contrast_total() {
        for (role, name) in KdeShadeRole::ALL.into_iter().zip(KdeShadeRole::NAMES) {
            assert_eq!(KdeShadeRole::from_name(name), Some(role));
        }
        assert_eq!(KdeShadeRole::from_name("highlight"), None);
        let grey = rgb8(128, 128, 128);
        assert_eq!(
            kde_shade(grey, KdeShadeRole::Dark, f32::NAN),
            kde_shade(grey, KdeShadeRole::Dark, 0.0)
        );
    }
}
