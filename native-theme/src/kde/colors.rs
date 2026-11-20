// KDE color group parsing -> populate per-widget fields on ThemeMode
// Maps semantic color roles from KDE INI color groups directly to ThemeMode.

use super::kcu_tint::kcu_tint;
use super::shade::{KdeShadeRole, kde_shade};
use crate::Rgba;

/// Look up a color key from a KDE INI section and parse it as RGB.
fn get_color(ini: &configparser::ini::Ini, section: &str, key: &str) -> Option<Rgba> {
    let value = ini.get(section, key)?;
    super::parse_rgb(&value)
}

/// Breeze's `Metrics::Blend_Value` (breezemetrics.h:176 at breeze v6.7.5), the
/// alpha of the selection colour over a checked indicator's fill
/// (breezehelper.cpp:40), of the focus colour over a hovered menu item
/// (breezestyle.cpp:6021-6029 at breeze f0b1d75), of the selection colour over
/// a hovered item-view row (:4871-4875) and of the selection colour in the
/// keyboard focus band (breezestyle.cpp:6776-6779); docs/platform-facts.md §2.5,
/// §2.6, §2.15.
const BLEND_VALUE: f32 = 0.3;

/// `kdeglobals` `[KDE] frameContrast` where it is unset, KColorScheme's own
/// default (kcolorscheme.cpp:529-538 at 27066d47; docs/platform-facts.md §2.11).
const FRAME_CONTRAST_DEFAULT: f64 = 0.2;

/// `kdeglobals` `[KDE] contrast` where it is unset or not an integer,
/// KColorScheme's own default: `contrastF` is `0.1 * readEntry("contrast", 7)`
/// (kcolorscheme.cpp:519-527 at kcolorscheme v6.31.0; docs/platform-facts.md
/// §2.15).
const CONTRAST_DEFAULT: i32 = 7;

/// `color` at `alpha`, held in 8 bits as `Rgba` holds every alpha.
fn translucent(color: Rgba, alpha: f32) -> Rgba {
    let [r, g, b, _] = color.to_f32_array();
    Rgba::from_f32(r, g, b, alpha)
}

/// Breeze's keyboard focus band in `highlight`, the selection colour: that colour at
/// `Metrics::Blend_Value` (breezestyle.cpp:6776-6779, breezemetrics.h:176 at breeze
/// f0b1d75; docs/platform-facts.md §2.1.5).
pub(crate) fn focus_ring_of(highlight: Rgba) -> Rgba {
    translucent(highlight, BLEND_VALUE)
}

/// `top` painted over the opaque `bottom` (source over).
fn over(top: Rgba, bottom: Rgba) -> Rgba {
    let [tr, tg, tb, alpha] = top.to_f32_array();
    let [br, bg, bb, _] = bottom.to_f32_array();
    let channel = |t: f32, b: f32| t * alpha + b * (1.0 - alpha);
    Rgba::from_f32(channel(tr, br), channel(tg, bg), channel(tb, bb), 1.0)
}

/// `qRound(x)` for a non-negative `x` as Qt 6.11.2 computes it on x86-64,
/// `int(x + 0.5f)` (qtbase `src/corelib/global/qnumeric.h:539-544`), held
/// in the 16 bits `QColor` stores a component in. Every caller passes a
/// value in 0.0..=65535.0; the clamp keeps any other in the 16 bits.
fn q_round(x: f32) -> u16 {
    (x.clamp(0.0, 65535.0) + 0.5) as u16
}

/// An RGB colour as Qt 6.11.2's `QColor` stores one: four 16-bit channels.
/// Qt's colour arithmetic reads and rounds them, not the 8-bit values, so
/// a port that rounds to 8 bits between steps gives other colours
/// (qtbase `src/gui/painting/qcolor.cpp` at v6.11.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct QtColor {
    r: u16,
    g: u16,
    b: u16,
    a: u16,
}

/// An HSV colour as `QColor` stores one: the hue in hundredths of a degree,
/// `u16::MAX` where it is undefined (a grey), the rest in 16 bits.
#[derive(Clone, Copy, Debug)]
struct QtHsv {
    hue: u16,
    saturation: u16,
    value: u16,
    alpha: u16,
}

impl QtColor {
    /// `QColor(r, g, b, a)`: each 8-bit channel times 0x101 (`setRgb`,
    /// qcolor.cpp:1518-1531).
    fn from_rgba(color: Rgba) -> Self {
        let wide = |c: u8| u16::from_be_bytes([c, c]);
        Self {
            r: wide(color.r),
            g: wide(color.g),
            b: wide(color.b),
            a: wide(color.a),
        }
    }

    /// `QColor::fromRgbF` (qcolor.cpp:2607-2633) for components in
    /// 0.0..=1.0, each `qRound(x * 65535)`. Qt keeps a component outside that
    /// range in half floats instead; the callers here cannot make one, and
    /// the clamp keeps it so.
    fn from_rgb_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        let channel = |x: f32| q_round(x.clamp(0.0, 1.0) * 65535.0);
        Self {
            r: channel(r),
            g: channel(g),
            b: channel(b),
            a: channel(a),
        }
    }

    /// `redF()` and the others: the channel over 65535 as a float
    /// (qcolor.cpp:1665-1670, :1783-1791).
    fn f(channel: u16) -> f32 {
        f32::from(channel) / 65535.0
    }

    /// The colour at `alpha`, `QColor::setAlphaF` (qcolor.cpp:1679-1689).
    fn with_alpha_f(self, alpha: f32) -> Self {
        Self {
            a: q_round(alpha.clamp(0.0, 1.0) * 65535.0),
            ..self
        }
    }

    /// The 8-bit colour, `red()` and the others: `qt_div_257` of each
    /// channel (qcolor.cpp:1697-1702; `qdrawhelper_p.h:886-887`).
    fn to_rgba(self) -> Rgba {
        let narrow = |c: u16| u8::try_from(div_257(u64::from(c))).unwrap_or(u8::MAX);
        Rgba::new(
            narrow(self.r),
            narrow(self.g),
            narrow(self.b),
            narrow(self.a),
        )
    }

    /// `QColor::toHsv` (qcolor.cpp:2370-2415).
    fn to_hsv(self) -> QtHsv {
        let (r, g, b) = (self.r, self.g, self.b);
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        if max == min {
            return QtHsv {
                hue: u16::MAX,
                saturation: 0,
                value: max,
                alpha: self.a,
            };
        }
        let chroma = f32::from(max.saturating_sub(min));
        let (r, g, b) = (f32::from(r), f32::from(g), f32::from(b));
        let hue = if max == self.r {
            let hue = (g - b) / chroma;
            if hue < 0.0 { hue + 6.0 } else { hue }
        } else if max == self.g {
            2.0 + (b - r) / chroma
        } else {
            4.0 + (r - g) / chroma
        };
        QtHsv {
            hue: q_round(hue * 6000.0),
            saturation: q_round(chroma / f32::from(max) * 65535.0),
            value: max,
            alpha: self.a,
        }
    }
}

impl QtHsv {
    /// `QColor::fromHsvF` (qcolor.cpp:2703-2716) for components in
    /// 0.0..=1.0, the range Kirigami clamps them to.
    fn from_hsv_f(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        let channel = |x: f32| q_round(x.clamp(0.0, 1.0) * 65535.0);
        Self {
            hue: q_round(hue.clamp(0.0, 1.0) * 36000.0),
            saturation: channel(saturation),
            value: channel(value),
            alpha: channel(alpha),
        }
    }

    /// `hueF()`: -1 where the hue is undefined (qcolor.cpp:1968-1973).
    fn hue_f(self) -> f32 {
        if self.hue == u16::MAX {
            -1.0
        } else {
            f32::from(self.hue) / 36000.0
        }
    }

    /// `QColor::toRgb` from HSV (qcolor.cpp:2222-2292). `from_hsv_f` and
    /// `to_hsv` give a hue of at most 36000, Qt's 0, so the sector is 0..=5.
    ///
    /// Qt 6.11.2 as PySide6 6.11.2 ships it rounds `1.0f - (s * f)` once, a
    /// fused multiply-add. That is a property of the build, not of Qt's
    /// source: the compiler may contract the expression where the target has
    /// FMA. A build that rounds the product first differs by one 16-bit unit
    /// on rare inputs, 15 of the generator's 20 000 random interpolations
    /// and 40 of 100 000 more (seed 4243), and gave the same 8-bit colour on
    /// every one of them (scripts/generate_qt_color_vectors.py).
    fn to_rgb(self) -> QtColor {
        let alpha = self.alpha;
        if self.saturation == 0 || self.hue == u16::MAX {
            let v = self.value;
            return QtColor {
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
        let s = QtColor::f(self.saturation);
        let v = QtColor::f(self.value);
        // `h` is below 6: the hue is below 36000 here.
        let sector = h as u8;
        let f = h - f32::from(sector);
        let p = v * (1.0 - s);
        let (r, g, b) = if sector & 1 == 1 {
            let q = v * (-s).mul_add(f, 1.0);
            match sector {
                1 => (q, v, p),
                3 => (p, q, v),
                _ => (v, p, q),
            }
        } else {
            let t = v * (-s).mul_add(1.0 - f, 1.0);
            match sector {
                0 => (v, t, p),
                2 => (p, v, t),
                _ => (t, p, v),
            }
        };
        let channel = |x: f32| q_round(x * 65535.0);
        QtColor {
            r: channel(r),
            g: channel(g),
            b: channel(b),
            a: alpha,
        }
    }
}

/// `KColorUtils::mix(a, b, bias)` for opaque `a` and `b`, as kdeglobals
/// gives every colour, in 8 bits: [`mix_qt`].
fn mix(a: Rgba, b: Rgba, bias: f64) -> Rgba {
    mix_qt(a, b, bias).to_rgba()
}

/// `KColorUtils::mix(a, b, bias)` for opaque `a` and `b`: each channel moved
/// from `a` towards `b` by `bias`, in `QColor`'s channels and arithmetic
/// (kcolorutils.cpp:144-165 at kguiaddons 7c766f6f; `qreal` is f64,
/// `QColor`'s components f32).
fn mix_qt(a: Rgba, b: Rgba, bias: f64) -> QtColor {
    mix_qt_colors(QtColor::from_rgba(a), QtColor::from_rgba(b), bias)
}

/// `KColorUtils::mix(a, b, bias)` for opaque colours already in `QColor`'s
/// storage: [`mix_qt`]'s arithmetic, for a colour mixed from another mix
/// without rounding it to 8 bits, as Qt keeps it.
fn mix_qt_colors(a: QtColor, b: QtColor, bias: f64) -> QtColor {
    if bias.is_nan() || bias <= 0.0 {
        return a;
    }
    if bias >= 1.0 {
        return b;
    }
    // Both alphas are 1, so the mixed alpha is 1 and dividing by it is exact.
    let channel = |from: u16, to: u16| {
        let (from, to) = (f64::from(QtColor::f(from)), f64::from(QtColor::f(to)));
        (from + (to - from) * bias).clamp(0.0, 1.0) as f32
    };
    QtColor::from_rgb_f(channel(a.r, b.r), channel(a.g, b.g), channel(a.b, b.b), 1.0)
}

/// Kirigami's `ColorUtils::linearInterpolation(one, two, balance)` for
/// opaque colours, in 8 bits: [`linear_interpolation_qt`].
fn linear_interpolation(one: Rgba, two: Rgba, balance: f64) -> Rgba {
    linear_interpolation_qt(one, two, balance).to_rgba()
}

/// Kirigami's `ColorUtils::linearInterpolation(one, two, balance)` for
/// opaque colours: hue, saturation and value each moved from `one` towards
/// `two` by `balance`, in `QColor`'s HSV and arithmetic (kirigami
/// `src/platform/colorutils.cpp:61-80` at 8319acc7).
fn linear_interpolation_qt(one: Rgba, two: Rgba, balance: f64) -> QtColor {
    let one = QtColor::from_rgba(one).to_hsv();
    let two = QtColor::from_rgba(two).to_hsv();
    let lerp = |from: f32, to: f32| {
        let from = f64::from(from);
        from + (f64::from(to) - from) * balance
    };
    // A hue that is not above 0, a grey's undefined -1 or a red's 0, takes
    // the other colour's, and 0 where both are so.
    let hue_of = |own: QtHsv, other: QtHsv| {
        let hue = if own.hue_f() > 0.0 {
            own.hue_f()
        } else {
            other.hue_f()
        };
        hue.max(0.0)
    };
    // `std::fmod(hue, 1.0)`: the same as `rem_euclid` for the hue, which is
    // not negative.
    let hue = lerp(hue_of(one, two), hue_of(two, one)).rem_euclid(1.0);
    let saturation = lerp(QtColor::f(one.saturation), QtColor::f(two.saturation)).clamp(0.0, 1.0);
    let value = lerp(QtColor::f(one.value), QtColor::f(two.value)).clamp(0.0, 1.0);
    QtHsv::from_hsv_f(hue as f32, saturation as f32, value as f32, 1.0).to_rgb()
}

/// `Qt.tint(base, tint)`, `QQuickColorValueType::tint` (qtdeclarative
/// `src/quick/util/qquickvaluetypes.cpp:57-75` at v6.11.2, 4e3399c2): `tint`
/// laid over `base` at its own alpha, kept at `QColor`'s precision (this is
/// Qt Quick's, not `KColorUtils::tint`).
fn tint(base: QtColor, tint: QtColor) -> QtColor {
    match div_257(u64::from(tint.a)) {
        255 => return tint,
        0 => return base,
        _ => {}
    }
    let alpha = f64::from(QtColor::f(tint.a));
    let inverse = 1.0 - alpha;
    let channel = |t: u16, b: u16| {
        (f64::from(QtColor::f(t)) * alpha + f64::from(QtColor::f(b)) * inverse) as f32
    };
    QtColor::from_rgb_f(
        channel(tint.r, base.r),
        channel(tint.g, base.g),
        channel(tint.b, base.b),
        (alpha + inverse * f64::from(QtColor::f(base.a))) as f32,
    )
}

/// The alpha qqc2-desktop-style's switch tints its off track's text colour
/// by its background at, `blendBackgroundWithTextColorWithRatio(0.9)`
/// (`org.kde.desktop/private/SwitchIndicator.qml:49` at v6.31.0).
const SWITCH_OFF_TINT_ALPHA: f64 = 0.9;

/// The alpha of the highlight over the switch's on track, `Qt.alpha(
/// highlightColor, 0.5)` (SwitchIndicator.qml:62 at v6.31.0).
const SWITCH_ON_ALPHA: f64 = 0.5;

/// qqc2-desktop-style's switch off track, `Qt.tint(textColor,
/// Qt.alpha(backgroundColor, 0.9))` of the Button set
/// (SwitchIndicator.qml:21, :32-35, :49 at v6.31.0), at `QColor`'s
/// precision. `Qt.alpha` takes a double and `setAlphaF` a float.
fn switch_off_track(background: Rgba, text: Rgba) -> QtColor {
    tint(
        QtColor::from_rgba(text),
        QtColor::from_rgba(background).with_alpha_f(SWITCH_OFF_TINT_ALPHA as f32),
    )
}

/// qqc2-desktop-style's switch on track: `Qt.alpha(highlight, 0.5)` laid
/// over the off track (SwitchIndicator.qml:53-63 at v6.31.0), both at the
/// precision `QColor` stores them, in exact arithmetic, rounded once to 8
/// bits (docs/platform-facts.md §2.21).
fn switch_on_track(off: QtColor, highlight: Rgba) -> Rgba {
    let highlight = QtColor::from_rgba(highlight).with_alpha_f(SWITCH_ON_ALPHA as f32);
    let alpha = f64::from(highlight.a);
    let inverse = f64::from(u16::MAX.saturating_sub(highlight.a));
    // 255 × (h × alpha + o × inverse) is an integer below 2^41, exact in an
    // f64, so dividing by 65535² rounds only once.
    let channel = |h: u16, o: u16| {
        let numerator = (f64::from(h) * alpha + f64::from(o) * inverse) * 255.0;
        // At most 255: `h` and `o` are at most 65535 and the alphas sum to 65535.
        (numerator / (65535.0 * 65535.0)).round() as u8
    };
    Rgba::rgb(
        channel(highlight.r, off.r),
        channel(highlight.g, off.g),
        channel(highlight.b, off.b),
    )
}

/// How far a checked, non-flat Breeze button's fill moves from `Button`
/// towards `ButtonText`: `KColorUtils::mix(palette.button(),
/// palette.buttonText(), 0.125)` (breezehelper.cpp:695-699 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const CHECKED_BUTTON_MIX: f64 = 0.125;

/// The alpha of the button text a checked flat Breeze button is filled
/// with, `alphaColor(palette.buttonText().color(), 0.125)`
/// (breezehelper.cpp:681-684 at breeze f0b1d75; docs/platform-facts.md §2.13).
const FLAT_CHECKED_ALPHA: f32 = 0.125;

/// How far a pressed, enabled, non-flat Breeze button's fill moves from
/// `Button` towards `Highlight`: `KColorUtils::mix(palette.button().color(),
/// highlightColor, Metrics::Blend_Value)`, the `qreal` 0.3
/// (breezehelper.cpp:694, breezemetrics.h:176 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const PRESSED_BUTTON_MIX: f64 = 0.3;

/// How far a Qt Widgets menu's panel moves from `Window` towards `Base`:
/// `frameBackgroundColor()`, `KColorUtils::mix(Window, Base,
/// Metrics::Blend_Value)`, the `qreal` 0.3 (`drawPanelMenuPrimitive`,
/// breezestyle.cpp:4708; breezehelper.cpp:200-203; breezemetrics.h:176 at breeze
/// f0b1d75; docs/platform-facts.md §2.6).
const MENU_PANEL_MIX: f64 = 0.3;

/// How far a hovered menu item's outline moves from the focus colour towards
/// the window text: `focusOutlineColor()`, `KColorUtils::mix(focusColor,
/// WindowText, 0.15)` (breezehelper.cpp:172-175, breezestyle.cpp:6021-6049 at
/// breeze f0b1d75; docs/platform-facts.md §2.6).
const MENU_HOVER_OUTLINE_MIX: f64 = 0.15;

/// How far an active window's default button moves from `Button` towards
/// `Highlight`: `KColorUtils::mix(palette.button().color(), highlightColor,
/// 0.2)` (breezehelper.cpp:700-702 at breeze f0b1d75; docs/platform-facts.md
/// §2.3).
const DEFAULT_BUTTON_MIX: f64 = 0.2;

/// How far the default button's frame moves from `Highlight` towards the
/// plain frame, `mix(Button, ButtonText, frameContrast)`: 0.5
/// (breezehelper.cpp:700-702 at breeze f0b1d75; docs/platform-facts.md §2.3).
const DEFAULT_BUTTON_FRAME_MIX: f64 = 0.5;

/// How far a hovered or pressed header section moves from `Button` towards
/// the hover or the focus colour: `KColorUtils::mix(palette.button(),
/// hoverColor, 0.2)` and `mix(…, focusColor, 0.2)` (breezestyle.cpp:6828-6837
/// at breeze f0b1d75; docs/platform-facts.md §2.15).
const HEADER_SECTION_MIX: f64 = 0.2;

/// The alpha of `Highlight` in a hovered item-view row's 1 px outline
/// (breezestyle.cpp:4880-4884 at breeze f0b1d75; docs/platform-facts.md
/// §2.15).
const LIST_HOVER_OUTLINE_ALPHA: f32 = 0.8;

/// How far the inactive selection's background moves from the window's
/// towards the selection's: `KColorUtils::tint(Window, Selection, 0.4)`
/// (kcolorscheme.cpp:304-321 at 27066d47; docs/platform-facts.md §2.1.3).
const INACTIVE_SELECTION_TINT: f32 = 0.4;

/// The alpha of the hover decoration a hovered, unselected Breeze tab is
/// filled with, `alphaColor(hoverColor(palette), 0.2)` (breezehelper.cpp:1497
/// at breeze f0b1d75; docs/platform-facts.md §2.11).
const TAB_HOVER_ALPHA: f32 = 0.2;

/// The alpha a Breeze scroll-bar handle's colour is filled with over the
/// window, `overlayColors(bg, alphaColor(fg, 0.5))` (breezehelper.cpp:1284
/// at breeze f0b1d75; docs/platform-facts.md §2.8).
const SCROLLBAR_HANDLE_ALPHA: f32 = 0.5;

/// The alpha of the window text over the window in a Breeze scroll-bar handle at
/// rest, the pointer off the bar: the handle colour `WindowText` at 0.5
/// (breezehelper.cpp:279), times 0.7 off the bar (breezestyle.cpp:6401-6403),
/// filled at half of that over the window, `overlayColors(Window, alphaColor(fg,
/// 0.5))` (breezehelper.cpp:1284, :349-355; breezestyle.cpp:6405) at breeze
/// f0b1d75, with animations on, their default (breeze.kcfg:45-47);
/// docs/platform-facts.md §2.8.
const SCROLLBAR_IDLE_HANDLE_ALPHA: f32 = 0.175;

/// The alpha of the accent over the window in a Breeze progress bar's
/// contents, `alphaColor(fg, 0.7)` (breezehelper.cpp:1216 at breeze f0b1d75;
/// docs/platform-facts.md §2.10).
const PROGRESS_FILL_ALPHA: f32 = 0.7;

/// `(x * a + 0x7f) / 0xff` as Qt's raster engine rounds it, `BYTE_MUL`
/// (qtbase `qdrawhelper_p.h`), for 8-bit `x` and `a`.
fn byte_mul(x: u64, a: u64) -> u64 {
    let t = x.saturating_mul(a);
    t.saturating_add(t.wrapping_shr(8))
        .saturating_add(0x80)
        .wrapping_shr(8)
}

/// `x * a / 0xffff` rounded, for 16-bit `x` and `a` (qtbase `qt_div_65535`).
fn mul_65535(x: u64, a: u64) -> u64 {
    let t = x.saturating_mul(a);
    t.saturating_add(t.wrapping_shr(16))
        .saturating_add(0x8000)
        .wrapping_shr(16)
}

/// A 16-bit value to 8 bits, `x / 257` rounded to nearest.
fn div_257(x: u64) -> u64 {
    let t = x.saturating_add(0x80);
    t.saturating_sub(t.wrapping_shr(8)).wrapping_shr(8)
}

/// `KColorUtils::overlayColors(base, paint)` for an opaque `paint` colour
/// given the alpha `alpha` by `QColor::setAlphaF`: `paint` filled
/// source-over onto the opaque `base` in a 1×1 `ARGB32_Premultiplied` image
/// (kcolorutils.cpp:168-181 at kguiaddons 7c766f6f). Qt premultiplies the
/// colour in 16 bits, stores it in 8, and blends the 8-bit destination by the
/// 8-bit inverse alpha; this arithmetic reproduces Qt 6.11.2's pixel on
/// 20 000 random inputs, so the result is Breeze's to the unit.
fn overlay_colors(base: Rgba, paint: Rgba, alpha: f32) -> Rgba {
    // QColor keeps alpha in 16 bits: qRound(alpha * 65535), in f32.
    let alpha16 = (alpha.clamp(0.0, 1.0) * 65535.0 + 0.5).floor() as u64;
    let alpha8 = div_257(alpha16);
    let inverse = 255_u64.saturating_sub(alpha8);
    let channel = |p: u8, b: u8| {
        let premultiplied = div_257(mul_65535(u64::from(p).saturating_mul(257), alpha16));
        let v = premultiplied.saturating_add(byte_mul(u64::from(b), inverse));
        u8::try_from(v).unwrap_or(u8::MAX)
    };
    Rgba::rgb(
        channel(paint.r, base.r),
        channel(paint.g, base.g),
        channel(paint.b, base.b),
    )
}

/// `KColorScheme::frameContrast()`: `[KDE] frameContrast`, clamped to
/// 0.0..=1.0, or its default where unset or not a number; a `qreal`, as
/// KConfig reads it.
fn frame_contrast(ini: &configparser::ini::Ini) -> f64 {
    ini.get("KDE", "frameContrast")
        .and_then(|v| v.trim().parse::<f64>().ok())
        .filter(|v| v.is_finite())
        .map_or(FRAME_CONTRAST_DEFAULT, |v| v.clamp(0.0, 1.0))
}

/// KColorScheme's contrast step: `[KDE] contrast` as an integer, or
/// [`CONTRAST_DEFAULT`] where unset or not an integer. [`kde_shade`] takes a
/// step beyond ±10 as ±10, so it is not bounded here.
fn contrast(ini: &configparser::ini::Ini) -> i32 {
    ini.get("KDE", "contrast")
        .and_then(|v| v.trim().parse::<i32>().ok())
        .unwrap_or(CONTRAST_DEFAULT)
}

/// KColorScheme's `MidShade` of `window` at `contrast`, which `QPalette::Mid`
/// is (kcolorscheme.cpp:685 at 27066d47; docs/platform-facts.md §2.15), read
/// back to 8 bits: [`kde_shade`] returns Qt's 16-bit channels over 65535, and
/// [`Rgba::from_f32`] rounds each times 255 to the nearest, which is
/// `qt_div_257` of the 16-bit value (`div_257` here): a channel `s / 65535`
/// times 255 is `s / 257`, never halfway between two integers.
fn mid_shade(window: Rgba, contrast: i32) -> Rgba {
    let [r, g, b, a] = kde_shade(window.to_f32_array(), KdeShadeRole::Mid, contrast as f32);
    Rgba::from_f32(r, g, b, a)
}

/// A `kdeglobals` boolean: `true` or `false`, trimmed and in any ASCII case;
/// any other value, or none, is `None`. Breeze's schemes write these two
/// forms; KConfig's whole boolean syntax was not read, so no other form is
/// accepted.
fn kconfig_bool(value: Option<String>) -> Option<bool> {
    let value = value?;
    let value = value.trim();
    if value.eq_ignore_ascii_case("true") {
        Some(true)
    } else if value.eq_ignore_ascii_case("false") {
        Some(false)
    } else {
        None
    }
}

/// Whether KColorScheme builds an inactive window's selection from the Window
/// set: `[ColorEffects:Inactive] ChangeSelectionColor`, else the group's
/// `Enable`, else true (kcolorscheme.cpp:304-316 at kcolorscheme v6.31.0).
fn inactive_change_selection_color(ini: &configparser::ini::Ini) -> bool {
    kconfig_bool(ini.get("ColorEffects:Inactive", "ChangeSelectionColor"))
        .or_else(|| kconfig_bool(ini.get("ColorEffects:Inactive", "Enable")))
        .unwrap_or(true)
}

/// The inactive Selection colour of a role, as KColorScheme builds it before
/// any inactive effect: the rule of `kde_selection_inactive` on `v0.8.0-rc1`,
/// rewritten for this reader and checked against its table (`vector_tests`).
/// `None` where an input its branch reads is missing; an input the branch
/// does not read may be missing. A scheme that enables
/// `[ColorEffects:Inactive]` has its effects applied on top, which this does
/// not do (docs/platform-facts.md §2.1.3).
///
/// Where `change` is false the role's own `selection` colour; else, for the
/// `background` role, `window` tinted towards `selection_background`, and
/// for a text role `window`.
pub(super) fn inactive_selection(
    selection: Option<[f32; 4]>,
    window: Option<[f32; 4]>,
    selection_background: Option<[f32; 4]>,
    background: bool,
    change: bool,
) -> Option<[f32; 4]> {
    if !change {
        selection
    } else if background {
        window
            .zip(selection_background)
            .map(|(w, sb)| kcu_tint(w, sb, INACTIVE_SELECTION_TINT))
    } else {
        window
    }
}

/// Populate a ThemeMode with colors from KDE INI color groups.
///
/// Maps all standard KDE color groups (View, Window, Button, Selection,
/// Tooltip, Header, WM) to per-widget fields on the variant.
/// Missing INI keys result in None fields (no hardcoded fallbacks), so a
/// colour the reader cannot derive keeps the preset's value.
///
/// Some colours Plasma computes from the scheme, and the reader computes
/// them as Breeze, Kirigami and qqc2-desktop-style do, in Qt's arithmetic:
/// the checked button, the default button's fill and frame, the checkbox and
/// progress-bar blends, the switch tracks and outlines, the toolbar's fill and
/// edge, a toolbar button's pressed and checked fills, the popover's and the
/// card's edges, the menu's hover, its outline, separator and panel, the list's
/// hover and its outline, the header's hovered and pressed fills and the
/// selection of a view without the focus, the table's grid, the scroll bar's
/// resting handle and the focus ring.
/// The blends on the accent (the default button's fill and frame, the checked
/// checkbox, the progress fill, the switch's on track and its outline, the
/// list's hover and its outline, the inactive selection, and the toolbar
/// button's pressed fill) come from kdeglobals'
/// `[Colors:Selection] BackgroundNormal`, so an application's `with_overlay` accent or the
/// portal's (`gnome::apply_kde_accent`, applied by `GnomePortalKdeReader`) does
/// not move them. The menu's hover and its outline, an open menu-bar title's
/// fill and the check box's hover outline come from
/// `[Colors:View] DecorationFocus`, so the same holds for them. The combo box's
/// and the toolbar button's hover pen is `[Colors:Selection] BackgroundNormal`
/// as it is. The focus ring is the selection colour at
/// `Blend_Value`; the KDE portal's accent replaces it (`gnome::apply_kde_accent`).
/// A menu's fills and outlines are Breeze's with `MenuItemDrawStrongFocus` on,
/// its default (breeze.kcfg:130-132 at breeze f0b1d75); the reader does not read
/// breezerc.
///
/// The inactive selection follows `[ColorEffects:Inactive] ChangeSelectionColor`
/// and `Enable`; a scheme that sets `[ColorEffects:Inactive] Enable=true` gets
/// the pair before the inactive group's contrast, intensity and colour effects,
/// which recolour every colour of an inactive window, a palette the model does
/// not hold (decision 18; docs/todo.md, *Model: an inactive window's palette*).
///
/// The toolbar takes the Header set's fill as Breeze paints it where the
/// window decoration's border size is automatic, None or NoSides
/// (breezehelper.cpp:1904-1941 at breeze f0b1d75); the reader does not read
/// kwinrc and assumes Plasma's default, automatic.
pub(crate) fn populate_colors(ini: &configparser::ini::Ini, variant: &mut crate::ThemeMode) {
    let window_fg = get_color(ini, "Colors:Window", "ForegroundNormal");
    let view_focus = get_color(ini, "Colors:View", "DecorationFocus");
    let selection_bg = get_color(ini, "Colors:Selection", "BackgroundNormal");
    let selection_fg = get_color(ini, "Colors:Selection", "ForegroundNormal");

    // === defaults-level colors ===
    variant.defaults.accent_color = view_focus;
    variant.defaults.background_color = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.defaults.text_color = window_fg;
    variant.defaults.surface_color = get_color(ini, "Colors:View", "BackgroundNormal");
    // border: not set by reader -- KDE has no native border color API.
    // The preset provides the correct neutral gray (platform-facts: "(preset)").
    variant.defaults.muted_color = get_color(ini, "Colors:Window", "ForegroundInactive");
    // KDE does not expose shadow color in kdeglobals
    variant.defaults.link_color = get_color(ini, "Colors:View", "ForegroundLink");
    // Breeze's keyboard focus band: the selection colour at `Blend_Value`; a pointer
    // focus draws none (docs/platform-facts.md §2.1.5)
    variant.defaults.focus_ring_color = selection_bg.map(focus_ring_of);

    // Selection
    variant.defaults.selection_background = selection_bg;
    variant.defaults.selection_text_color = get_color(ini, "Colors:Selection", "ForegroundNormal");
    // accent_text_color: foreground on accent-colored backgrounds (platform-facts 2.1.3)
    variant.defaults.accent_text_color = get_color(ini, "Colors:Selection", "ForegroundNormal");

    // Status colors
    variant.defaults.danger_color = get_color(ini, "Colors:View", "ForegroundNegative");
    variant.defaults.danger_text_color = window_fg;
    variant.defaults.warning_color = get_color(ini, "Colors:View", "ForegroundNeutral");
    variant.defaults.warning_text_color = window_fg;
    variant.defaults.success_color = get_color(ini, "Colors:View", "ForegroundPositive");
    variant.defaults.success_text_color = window_fg;
    variant.defaults.info_color = get_color(ini, "Colors:View", "ForegroundActive");
    variant.defaults.info_text_color = window_fg;

    // Disabled
    variant.defaults.disabled_text_color = get_color(ini, "Colors:View", "ForegroundInactive");

    // === per-widget colors ===

    // Button
    variant.button.background_color = get_color(ini, "Colors:Button", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Button", "ForegroundNormal") {
        variant.button.font.get_or_insert_default().color = Some(color);
    }
    // A checked button (docs/platform-facts.md §2.3): the button colour moved
    // an eighth towards the button text, lettered in the button text.
    let button_fg = get_color(ini, "Colors:Button", "ForegroundNormal");
    variant.button.checked_background = variant
        .button
        .background_color
        .zip(button_fg)
        .map(|(button, text)| mix(button, text, CHECKED_BUTTON_MIX));
    variant.button.checked_text_color = button_fg;
    // A pressed button (docs/platform-facts.md §2.3): the button colour moved
    // `Metrics::Blend_Value` towards the highlight (breezehelper.cpp:694 at
    // breeze f0b1d75); its text stays the button text, which
    // `active_text_color` inherits from the font.
    variant.button.active_background = variant
        .button
        .background_color
        .zip(selection_bg)
        .map(|(button, highlight)| mix(button, highlight, PRESSED_BUTTON_MIX));
    // A hovered button keeps its fill and its text: Breeze's hover changes
    // only the pen (breezehelper.cpp:693-707, :710-712 at breeze f0b1d75), and
    // a non-flat button is lettered in `QPalette::ButtonText` in every state
    // (breezestyle.cpp:5350-5362; docs/platform-facts.md §2.3).
    variant.button.hover_background = variant.button.background_color;
    variant.button.hover_text_color = button_fg;
    // The pen of a non-flat button turns the active group's `Highlight` while it is
    // hovered, has the visual focus or is pressed (breezehelper.cpp:672-673, :710-712 at
    // breeze f0b1d75; docs/platform-facts.md §2.3): the hover border, which a pressed
    // button keeps, and the focus border.
    variant.button.hover_border_color = selection_bg;
    variant.button.focus_border_color = selection_bg;
    // The default button of an active window (docs/platform-facts.md §2.3): the
    // button colour moved a fifth towards the highlight, framed in the
    // highlight moved halfway towards the plain frame, `mix(Button,
    // ButtonText, frameContrast)`, kept at Qt's precision, and lettered in the
    // button text (breezehelper.cpp:700-702 and breezestyle.cpp:5350-5362 at
    // breeze f0b1d75).
    variant.button.default_button_background = variant
        .button
        .background_color
        .zip(selection_bg)
        .map(|(button, highlight)| mix(button, highlight, DEFAULT_BUTTON_MIX));
    variant.button.default_button_border_color = variant
        .button
        .background_color
        .zip(button_fg)
        .zip(selection_bg)
        .map(|((button, text), highlight)| {
            let frame = mix_qt(button, text, frame_contrast(ini));
            mix_qt_colors(
                QtColor::from_rgba(highlight),
                frame,
                DEFAULT_BUTTON_FRAME_MIX,
            )
            .to_rgba()
        });
    variant.button.default_button_text_color = button_fg;

    // Tooltip
    variant.tooltip.background_color = get_color(ini, "Colors:Tooltip", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Tooltip", "ForegroundNormal") {
        variant.tooltip.font.get_or_insert_default().color = Some(color);
    }

    // Sidebar (docs/platform-facts.md:1370, :1372, §2.12): the View set, as the KDE HIG's
    // sidebar, the non-modal GlobalDrawer, is docked in it (Kirigami GlobalDrawer.qml:337)
    // and System Settings' sidebar sets it (CategoriesPage.qml:19-20).
    variant.sidebar.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.sidebar.font.get_or_insert_default().color = Some(color);
    }

    // Input
    variant.input.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.input.font.get_or_insert_default().color = Some(color);
    }
    // KDE-02: placeholder from View/ForegroundInactive
    variant.input.placeholder_color = get_color(ini, "Colors:View", "ForegroundInactive");
    // input.caret from View/DecorationFocus (the focus decoration color)
    variant.input.caret_color = view_focus;
    // A field's frame (docs/platform-facts.md §2.4): `frameOutlineColor`
    // strokes a hovered one in `hoverColor()` and a focused one in
    // `focusColor()`, focus winning, the View set's DecorationHover and
    // DecorationFocus (breezehelper.cpp:89-90, :142-168 at breeze f0b1d75).
    let view_hover = get_color(ini, "Colors:View", "DecorationHover");
    variant.input.hover_border_color = view_hover;
    variant.input.focus_border_color = view_focus;

    // Checkbox and radio (docs/platform-facts.md §2.5): Breeze fills an
    // unchecked indicator with the button colour and outlines it in
    // `separatorColor()`; a checked one is that fill under the selection
    // colour at `Metrics::Blend_Value`, outlined in the selection colour; the
    // mark is the text colour; under the pointer the indicator's outline is
    // stroked in `focusColor()`, `[Colors:View] DecorationFocus`
    // (breezehelper.cpp:895-907 at breeze f0b1d75).
    let button_bg = get_color(ini, "Colors:Button", "BackgroundNormal");
    variant.checkbox.unchecked_background = button_bg;
    variant.checkbox.checked_background = button_bg
        .zip(selection_bg)
        .map(|(button, selection)| over(translucent(selection, BLEND_VALUE), button));
    variant.checkbox.indicator_color = get_color(ini, "Colors:View", "ForegroundNormal");
    if let Some(color) = selection_bg {
        variant.checkbox.border.get_or_insert_default().color = Some(color);
    }
    variant.checkbox.unchecked_border_color = get_color(ini, "Colors:Window", "BackgroundNormal")
        .zip(window_fg)
        .map(|(window, text)| mix(window, text, frame_contrast(ini)));
    variant.checkbox.hover_border_color = view_focus;

    // Slider (docs/platform-facts.md §2.9): Breeze fills the handle with the
    // button colour (breezestyle.cpp:8122 at breeze f0b1d75).
    variant.slider.thumb_color = button_bg;

    // Switch (docs/platform-facts.md §2.21): qqc2-desktop-style's tracks, the
    // off one the Button set's text tinted by its background, the on one the
    // highlight at half alpha over it; hover changes no fill (SwitchIndicator.qml
    // :49, :62, :77 at v6.31.0). The handle is the Button set's background
    // (:29). The off track and the handle are outlined in the Button set's
    // background moved towards its text by the frame contrast, the on track
    // in the highlight (:27, :30, :50, :78).
    let switch_off = button_bg
        .zip(button_fg)
        .map(|(background, text)| switch_off_track(background, text));
    variant.switch.unchecked_background = switch_off.map(QtColor::to_rgba);
    variant.switch.checked_background = switch_off
        .zip(selection_bg)
        .map(|(off, highlight)| switch_on_track(off, highlight));
    variant.switch.hover_unchecked_background = variant.switch.unchecked_background;
    variant.switch.hover_checked_background = variant.switch.checked_background;
    variant.switch.thumb_background = button_bg;
    let switch_edge = button_bg
        .zip(button_fg)
        .map(|(bg, fg)| linear_interpolation(bg, fg, frame_contrast(ini)));
    variant.switch.unchecked_track_border_color = switch_edge;
    variant.switch.thumb_border_color = switch_edge;
    variant.switch.checked_track_border_color = selection_bg;

    // Tab (docs/platform-facts.md §2.11): Breeze marks the selected tab with a
    // strip of `QPalette::Highlight`, the selection background.
    variant.tab.active_indicator_color = selection_bg;
    // A hovered, unselected tab is filled with the hover decoration at alpha
    // 0.2 in place of its own fill (breezehelper.cpp:1496-1503 at breeze
    // f0b1d75; docs/platform-facts.md §2.11).
    variant.tab.hover_background = view_hover.map(|hover| translucent(hover, TAB_HOVER_ALPHA));

    // Segmented control (docs/platform-facts.md §2.25): the selected segment
    // is the selection background; its text, the selection's text, reaches
    // it through `defaults.accent_text_color`.
    variant.segmented_control.active_background = selection_bg;

    // Progress bar (docs/platform-facts.md §2.10): Breeze fills the contents
    // with `QPalette::Accent`, the selection background
    // (kcolorscheme.cpp:681 at 27066d47), at alpha 0.7 over the window, and
    // strokes the groove in the window text at alpha `frameContrast`.
    let window_bg = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.progress_bar.fill_color = selection_bg
        .zip(window_bg)
        .map(|(accent, window)| overlay_colors(window, accent, PROGRESS_FILL_ALPHA));
    if let Some(text) = window_fg {
        variant.progress_bar.border.get_or_insert_default().color =
            Some(translucent(text, frame_contrast(ini) as f32));
    }

    // Scroll bar (docs/platform-facts.md §2.8): a hovered handle is the hover
    // decoration, at full alpha while the pointer is over the scroll bar,
    // filled at half alpha over the window (breezehelper.cpp:292, :1284;
    // breezestyle.cpp:6400-6405 at breeze f0b1d75); Breeze has no colour of
    // its own for a dragged handle, which keeps the hovered one.
    let handle = window_bg
        .zip(view_hover)
        .map(|(window, hover)| overlay_colors(window, hover, SCROLLBAR_HANDLE_ALPHA));
    variant.scrollbar.thumb_hover_color = handle;
    variant.scrollbar.thumb_active_color = handle;
    // A handle at rest, the pointer off the bar, is the window text at alpha
    // 0.175 over the window (breezehelper.cpp:279, :1284;
    // breezestyle.cpp:6401-6405 at breeze f0b1d75; docs/platform-facts.md
    // §2.8); the focused view's handle and the pointer-on-the-bar shade have
    // no field. Breeze paints no groove, which kde-breeze states as a
    // transparent track.
    variant.scrollbar.thumb_color = window_bg
        .zip(window_fg)
        .map(|(window, text)| overlay_colors(window, text, SCROLLBAR_IDLE_HANDLE_ALPHA));

    // Toolbar (docs/platform-facts.md §2.13): Breeze fills a main window's
    // tools area with the Header set's background and strokes its bottom edge
    // in that set's `separatorColor()` (breezestyle.cpp:1322, :1341-1350,
    // breezehelper.cpp:329-332, breezetoolsareamanager.cpp:201-215 at breeze
    // f0b1d75). KColorScheme takes each key the Header group lacks, or the
    // whole set where the scheme has no Header group, from `[Colors:Window]`
    // (kcolorscheme.cpp:330-332 at 27066d47); without the group Breeze paints
    // no fill and the toolbar shows the window, the same colour.
    let header_bg = get_color(ini, "Colors:Header", "BackgroundNormal").or(window_bg);
    let header_fg = get_color(ini, "Colors:Header", "ForegroundNormal").or(window_fg);
    variant.toolbar.background_color = header_bg;
    if let Some(edge) = header_bg
        .zip(header_fg)
        .map(|(bg, fg)| mix(bg, fg, frame_contrast(ini)))
    {
        variant.toolbar.border.get_or_insert_default().color = Some(edge);
    }
    // A flat toolbar button (docs/platform-facts.md §2.13): under the pointer,
    // focused or pressed its pen is `Highlight`; pressed it is filled with
    // `Highlight` at `Blend_Value`, checked with the button text at 0.125
    // (breezehelper.cpp:679-691, :710-712 at breeze f0b1d75). Its hover fill,
    // transparent, is the preset's alone.
    variant.toolbar.button_hover_border_color = selection_bg;
    variant.toolbar.button_active_background =
        selection_bg.map(|highlight| translucent(highlight, BLEND_VALUE));
    variant.toolbar.button_checked_background =
        button_fg.map(|text| translucent(text, FLAT_CHECKED_ALPHA));

    // Popover (docs/platform-facts.md §2.16): its fill and text inherit the
    // window's, as qqc2-desktop-style's Popup fills the Window set's; its edge
    // is the Window set's background moved towards its text by the frame
    // contrast (Popup.qml:50 at v6.31.0). A separator inherits the border
    // colour (§2.18), not set here.
    if let Some(edge) = window_bg
        .zip(window_fg)
        .map(|(bg, fg)| linear_interpolation(bg, fg, frame_contrast(ini)))
    {
        variant.popover.border.get_or_insert_default().color = Some(edge);
    }

    // Card (docs/platform-facts.md §2.26): Kirigami's Card takes the View set,
    // its edge the background moved towards the text by the frame contrast
    // (DefaultCardBackground.qml:70, templates/AbstractCard.qml:92-93 at
    // kirigami 8319acc7); its fill inherits the surface colour.
    let view_bg = get_color(ini, "Colors:View", "BackgroundNormal");
    let view_fg = get_color(ini, "Colors:View", "ForegroundNormal");
    if let Some(edge) = view_bg
        .zip(view_fg)
        .map(|(bg, fg)| linear_interpolation(bg, fg, frame_contrast(ini)))
    {
        variant.card.border.get_or_insert_default().color = Some(edge);
    }

    // Combo box (docs/platform-facts.md §2.24): the Button set, and under the
    // pointer the pen `Highlight`, `[Colors:Selection] BackgroundNormal`
    // (breezehelper.cpp:710-712 at breeze f0b1d75).
    variant.combo_box.background_color = button_bg;
    if let Some(color) = button_fg {
        variant.combo_box.font.get_or_insert_default().color = Some(color);
    }
    variant.combo_box.hover_border_color = selection_bg;

    // Menu (docs/platform-facts.md §2.6): the panel is `frameBackgroundColor()`,
    // the window moved towards the view by `Blend_Value`; the text is the
    // window text; a hovered item is the focus colour at `Blend_Value`
    // lettered in the window text; the separator is `separatorColor()`, the
    // window moved towards its text by the frame contrast (breezestyle.cpp:4708,
    // :5984, :6021-6029, :6155; breezehelper.cpp:200-203, :329-332 at breeze
    // f0b1d75).
    if let Some(color) = window_fg {
        variant.menu.font.get_or_insert_default().color = Some(color);
    }
    variant.menu.hover_background = view_focus.map(|focus| translucent(focus, BLEND_VALUE));
    variant.menu.hover_text_color = window_fg;
    variant.menu.separator_color = window_bg
        .zip(window_fg)
        .map(|(window, text)| mix(window, text, frame_contrast(ini)));
    variant.menu.background_color = window_bg
        .zip(view_bg)
        .map(|(window, base)| mix(window, base, MENU_PANEL_MIX));
    // A hovered item's highlight is outlined 1 px in `focusOutlineColor()`; a menu bar's
    // title is filled with the opaque hover colour under the pointer and the focus colour
    // while its menu is open, lettered `HighlightedText`; the bar is the toolbar's fill
    // (breezestyle.cpp:5841-5945, :6021-6049, breezehelper.cpp:172-175 at breeze f0b1d75;
    // docs/platform-facts.md §2.6).
    variant.menu.hover_border_color = view_focus
        .zip(window_fg)
        .map(|(focus, text)| mix(focus, text, MENU_HOVER_OUTLINE_MIX));
    variant.menu.bar_hover_background = view_hover;
    variant.menu.bar_open_background = view_focus;
    variant.menu.bar_hover_text_color = selection_fg;

    // KDE-02: list fields (Colors:View is the native source for list/table content areas); a
    // hovered row is the selection at `Blend_Value` (breezestyle.cpp:4871-4875), outlined
    // 1 px in it at alpha 0.8 (:4880-4884)
    variant.list.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.list.item_font.get_or_insert_default().color = Some(color);
    }
    variant.list.alternate_row_background = get_color(ini, "Colors:View", "BackgroundAlternate");
    // A table's grid is `QPalette::Mid`, the window's `MidShade` at the
    // scheme's contrast (qcommonstyle.cpp:5126-5128 at qtbase v6.11.0;
    // kcolorscheme.cpp:685 at 27066d47; docs/platform-facts.md §2.15).
    variant.list.grid_color = window_bg.map(|window| mid_shade(window, contrast(ini)));
    variant.list.hover_background =
        selection_bg.map(|selection| translucent(selection, BLEND_VALUE));
    variant.list.hover_border_color =
        selection_bg.map(|s| translucent(s, LIST_HOVER_OUTLINE_ALPHA));
    // The header is the Button set: Breeze fills a `QHeaderView` section with
    // `QPalette::Button` (breezestyle.cpp:6827, :6838-6839 at breeze f0b1d75) and Qt
    // letters it `ButtonText` (qcommonstyle.cpp:1659-1660), which Breeze does not
    // override; neither reads the Header set (docs/platform-facts.md §2.15).
    variant.list.header_background = button_bg;
    if let Some(color) = button_fg {
        variant.list.header_font.get_or_insert_default().color = Some(color);
    }
    // A header section (docs/platform-facts.md §2.15): hovered, the Button
    // set's background moved a fifth towards the hover decoration; pressed,
    // towards the focus decoration (breezestyle.cpp:6828-6837 at breeze f0b1d75).
    variant.list.header_hover_background = button_bg
        .zip(view_hover)
        .map(|(button, hover)| mix(button, hover, HEADER_SECTION_MIX));
    variant.list.header_active_background = button_bg
        .zip(view_focus)
        .map(|(button, focus)| mix(button, focus, HEADER_SECTION_MIX));

    // The selection of a view without the focus (docs/platform-facts.md
    // §2.1.3): where the scheme's `[ColorEffects:Inactive]` changes it, the
    // window tinted towards the selection under the window text; where not,
    // the focused pair, written so that kde-breeze's tint does not stand.
    let change = inactive_change_selection_color(ini);
    let inactive = |selection: Option<Rgba>, window: Option<Rgba>, background: bool| {
        inactive_selection(
            selection.map(|c| c.to_f32_array()),
            window.map(|c| c.to_f32_array()),
            selection_bg.map(|c| c.to_f32_array()),
            background,
            change,
        )
        .map(|[r, g, b, a]| Rgba::from_f32(r, g, b, a))
    };
    variant.defaults.selection_inactive_background = inactive(selection_bg, window_bg, true);
    variant.defaults.selection_inactive_text_color = inactive(selection_fg, window_fg, false);

    // KDE-02: link.visited
    variant.link.visited_text_color = get_color(ini, "Colors:View", "ForegroundVisited");
    // link.font.color: not set here -- it inherits View/ForegroundLink from
    // defaults.link_color, so an overlay's link_color recolours link text.

    // === KDE-01: Window Manager title bar colors ===
    variant.window.title_bar_background = get_color(ini, "WM", "activeBackground");
    if let Some(color) = get_color(ini, "WM", "activeForeground") {
        variant.window.title_bar_font.get_or_insert_default().color = Some(color);
    }
    variant.window.inactive_title_bar_background = get_color(ini, "WM", "inactiveBackground");
    variant.window.inactive_title_bar_text_color = get_color(ini, "WM", "inactiveForeground");
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ThemeMode;

    /// Full Breeze Dark kdeglobals fixture with all 6 color groups + WM + Header.
    const BREEZE_DARK: &str = "\
[Colors:View]
BackgroundNormal=35,38,41
BackgroundAlternate=30,33,36
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
ForegroundVisited=155,89,182
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Window]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Button]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177

[Colors:Selection]
BackgroundNormal=61,174,233
BackgroundAlternate=29,153,243
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177

[Colors:Tooltip]
BackgroundNormal=49,54,59
ForegroundNormal=252,252,252

[Colors:Complementary]
BackgroundNormal=42,46,50
ForegroundNormal=239,240,241

[Colors:Header]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252

[WM]
activeBackground=49,54,59
activeForeground=239,240,241
inactiveBackground=42,46,50
inactiveForeground=161,169,177
";

    fn populate_fixture(content: &str) -> ThemeMode {
        let mut ini = super::super::create_kde_parser();
        ini.read(content.to_string()).unwrap();
        let mut variant = ThemeMode::default();
        populate_colors(&ini, &mut variant);
        variant
    }

    // === defaults-level color mapping ===

    #[test]
    fn test_accent_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    }

    /// A scheme's accent other than Breeze's reaches the slider's value through
    /// inheritance (docs/platform-facts.md:1317, KDE ← `defaults.accent_color`),
    /// which kde-breeze does not state, and its selection colour reaches the
    /// three blends the reader writes, the checked checkbox, the on switch and
    /// the progress bar.
    #[test]
    fn a_custom_accent_reaches_the_accent_fills() {
        let breeze = populate_fixture(BREEZE_DARK);
        let custom = BREEZE_DARK
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=200,40,90");
        assert_ne!(custom, BREEZE_DARK, "the fixture's accent moved");
        let v = populate_fixture(&custom);
        assert_eq!(
            v.slider.fill_color, None,
            "the reader leaves the slider to inheritance"
        );
        let mut mode = crate::Theme::preset("kde-breeze")
            .ok()
            .and_then(|t| t.dark)
            .unwrap_or_default();
        assert_eq!(
            mode.slider.fill_color, None,
            "kde-breeze states no slider fill"
        );
        mode.merge(&v);
        let r = mode
            .into_resolved(&crate::ResolutionContext::for_tests())
            .unwrap();
        assert_eq!(r.slider.fill_color, Rgba::rgb(200, 40, 90));
        for (field, ours, breezes) in [
            (
                "checkbox",
                v.checkbox.checked_background,
                breeze.checkbox.checked_background,
            ),
            (
                "switch",
                v.switch.checked_background,
                breeze.switch.checked_background,
            ),
            (
                "progress_bar",
                v.progress_bar.fill_color,
                breeze.progress_bar.fill_color,
            ),
        ] {
            assert!(ours.is_some(), "{field}: no fill");
            assert_ne!(
                ours, breezes,
                "{field}: the custom selection colour did not reach it"
            );
        }
    }

    /// D7: the colours Breeze paints a pressed button, a hovered and a
    /// focused field's frame, a hovered tab, the selected segment and a
    /// hovered or dragged scroll-bar handle with, each from the key it reads
    /// (docs/platform-facts.md §2.3, §2.4, §2.11, §2.25, §2.8). The fixture's
    /// DecorationHover differs from its DecorationFocus, so the frames are
    /// told apart.
    #[test]
    fn the_pressed_hovered_and_focused_colours_come_from_their_keys() {
        let v = populate_fixture(BREEZE_DARK);
        let button = Rgba::rgb(49, 54, 59);
        let selection = Rgba::rgb(61, 174, 233);
        let hover = Rgba::rgb(29, 153, 243);
        let window = Rgba::rgb(49, 54, 59);
        assert_eq!(
            v.button.active_background,
            Some(mix(button, selection, PRESSED_BUTTON_MIX))
        );
        assert_eq!(v.input.hover_border_color, Some(hover));
        assert_eq!(v.input.focus_border_color, Some(selection));
        assert_eq!(
            v.tab.hover_background,
            Some(translucent(hover, TAB_HOVER_ALPHA))
        );
        assert_eq!(v.tab.hover_background.map(|c| c.a), Some(0x33));
        assert_eq!(v.segmented_control.active_background, Some(selection));
        let handle = Some(overlay_colors(window, hover, SCROLLBAR_HANDLE_ALPHA));
        assert_eq!(v.scrollbar.thumb_hover_color, handle);
        assert_eq!(v.scrollbar.thumb_active_color, handle);
    }

    /// The default button's fill, label and frame under Breeze Light and
    /// Breeze Dark, as Breeze paints them (docs/platform-facts.md §2.3).
    #[test]
    fn test_default_button_is_breezes() {
        let [light, dark] = breeze_light_and_dark();
        let cases = [
            (
                light,
                Rgba::rgb(0xd6, 0xec, 0xf8),
                Rgba::rgb(0x23, 0x26, 0x29),
                Rgba::rgb(0x87, 0xc0, 0xdd),
            ),
            (
                dark,
                Rgba::rgb(0x2d, 0x46, 0x55),
                Rgba::rgb(0xfc, 0xfc, 0xfc),
                Rgba::rgb(0x48, 0x82, 0xa1),
            ),
        ];
        for (content, fill, text, frame) in cases {
            let v = populate_fixture(&content);
            assert_eq!(v.button.default_button_background, Some(fill));
            assert_eq!(v.button.default_button_text_color, Some(text));
            assert_eq!(v.button.default_button_border_color, Some(frame));
        }
    }

    /// The default button's frame mixes the highlight with the plain frame
    /// as Qt keeps it, in 16 bits: rounded to 8 bits first, Breeze Light's
    /// frame would be one level bluer than Breeze's.
    #[test]
    fn test_default_button_frame_keeps_qts_precision() {
        let [light, _] = breeze_light_and_dark();
        let v = populate_fixture(&light);
        let (button, text) = (Rgba::rgb(252, 252, 252), Rgba::rgb(35, 38, 41));
        let highlight = Rgba::rgb(61, 174, 233);
        let rounded_early = mix(
            highlight,
            mix(button, text, FRAME_CONTRAST_DEFAULT),
            DEFAULT_BUTTON_FRAME_MIX,
        );
        assert_eq!(rounded_early.b, 0xde);
        assert_eq!(
            v.button.default_button_border_color,
            Some(Rgba::rgb(0x87, 0xc0, 0xdd))
        );
        assert_ne!(v.button.default_button_border_color, Some(rounded_early));
    }

    /// A hovered and a pressed header section under Breeze Light and Breeze
    /// Dark, the Button set's background moved a fifth towards the hover and
    /// the focus decoration (docs/platform-facts.md §2.15); Breeze's two
    /// decorations are one colour, so the two fills are too, and a scheme
    /// whose decorations differ gives two.
    #[test]
    fn test_header_sections_are_breezes() {
        let hover = "DecorationHover=61,174,233\n";
        let [light, dark] = breeze_light_and_dark();
        for (content, fill) in [
            (light.clone(), Rgba::rgb(0xd6, 0xec, 0xf8)),
            (dark, Rgba::rgb(0x2d, 0x46, 0x55)),
        ] {
            let v = populate_fixture(&format!("{content}{hover}"));
            assert_eq!(v.list.header_hover_background, Some(fill));
            assert_eq!(v.list.header_active_background, Some(fill));
        }
        let v = populate_fixture(&format!("{light}DecorationHover=240,160,20\n"));
        assert!(v.list.header_hover_background.is_some());
        assert!(v.list.header_active_background.is_some());
        assert_ne!(
            v.list.header_hover_background,
            v.list.header_active_background
        );
    }

    /// The selection of a view without the focus under Breeze Light and
    /// Breeze Dark: the window tinted towards the selection, lettered in the
    /// window text (docs/platform-facts.md §2.1.3). `ChangeSelectionColor`
    /// reads `true` and `false` trimmed and in any case; without the
    /// Selection background the fill is unwritten and the text, which reads
    /// the window alone, is still the window text.
    #[test]
    fn test_inactive_selection_is_breezes() {
        let effects = "\n[ColorEffects:Inactive]\nChangeSelectionColor=true\nEnable=false\n";
        let [light, dark] = breeze_light_and_dark();
        for (content, fill, text) in [
            (
                &light,
                Rgba::rgb(0xc2, 0xe0, 0xf5),
                Rgba::rgb(0x23, 0x26, 0x29),
            ),
            (
                &dark,
                Rgba::rgb(0x1b, 0x41, 0x55),
                Rgba::rgb(0xfc, 0xfc, 0xfc),
            ),
        ] {
            let v = populate_fixture(&format!("{content}{effects}"));
            assert_eq!(v.defaults.selection_inactive_background, Some(fill));
            assert_eq!(v.defaults.selection_inactive_text_color, Some(text));
        }

        let selection = "[Colors:Selection]\nBackgroundNormal=61,174,233\n";
        let with_text = light.replace(
            selection,
            "[Colors:Selection]\nBackgroundNormal=61,174,233\nForegroundNormal=255,255,255\n",
        );
        assert_ne!(with_text, light, "the fixture's Selection group moved");
        let spaced = populate_fixture(&format!(
            "{with_text}\n[ColorEffects:Inactive]\nChangeSelectionColor= TRUE \nEnable=false\n"
        ));
        assert_eq!(
            spaced.defaults.selection_inactive_background,
            Some(Rgba::rgb(0xc2, 0xe0, 0xf5))
        );
        assert_eq!(
            spaced.defaults.selection_inactive_text_color,
            Some(Rgba::rgb(0x23, 0x26, 0x29))
        );
        let off = populate_fixture(&format!(
            "{with_text}\n[ColorEffects:Inactive]\nChangeSelectionColor=False\n"
        ));
        assert_eq!(
            off.defaults.selection_inactive_background,
            Some(Rgba::rgb(0x3d, 0xae, 0xe9))
        );
        assert_eq!(
            off.defaults.selection_inactive_text_color,
            Some(Rgba::rgb(0xff, 0xff, 0xff))
        );

        let without_selection = light.replace(selection, "");
        assert_ne!(
            without_selection, light,
            "the fixture's Selection group moved"
        );
        let v = populate_fixture(&without_selection);
        assert_eq!(v.defaults.selection_inactive_background, None);
        assert_eq!(
            v.defaults.selection_inactive_text_color,
            Some(Rgba::rgb(0x23, 0x26, 0x29))
        );
    }

    /// D7: a scheme whose selection, focus and hover colours are not
    /// Breeze's reaches the same fields, so a live desktop does not keep
    /// kde-breeze's blue there.
    #[test]
    fn a_custom_scheme_reaches_the_pressed_hovered_and_focused_colours() {
        let breeze = populate_fixture(BREEZE_DARK);
        let custom = BREEZE_DARK
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90")
            .replace("DecorationHover=29,153,243", "DecorationHover=240,160,20");
        assert_ne!(custom, BREEZE_DARK, "the fixture's colours moved");
        let v = populate_fixture(&custom);
        assert_eq!(v.input.hover_border_color, Some(Rgba::rgb(240, 160, 20)));
        assert_eq!(v.input.focus_border_color, Some(Rgba::rgb(20, 200, 90)));
        assert_eq!(
            v.segmented_control.active_background,
            Some(Rgba::rgb(200, 40, 90))
        );
        for (field, ours, breezes) in [
            (
                "button.active_background",
                v.button.active_background,
                breeze.button.active_background,
            ),
            (
                "tab.hover_background",
                v.tab.hover_background,
                breeze.tab.hover_background,
            ),
            (
                "scrollbar.thumb_hover_color",
                v.scrollbar.thumb_hover_color,
                breeze.scrollbar.thumb_hover_color,
            ),
            (
                "scrollbar.thumb_active_color",
                v.scrollbar.thumb_active_color,
                breeze.scrollbar.thumb_active_color,
            ),
        ] {
            assert!(ours.is_some(), "{field}: not written");
            assert_ne!(ours, breezes, "{field}: the custom scheme did not reach it");
        }
    }

    #[test]
    fn test_background_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_text_color_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
    }

    #[test]
    fn test_surface_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_border_not_set_by_reader() {
        // Border is a preset value, not reader-provided (platform-facts: "(preset)")
        let v = populate_fixture(BREEZE_DARK);
        assert!(v.defaults.border.is_empty());
    }

    #[test]
    fn test_muted_from_window_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.muted_color, Some(Rgba::rgb(161, 169, 177)));
    }

    // === Status colors ===

    #[test]
    fn test_status_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.danger_color, Some(Rgba::rgb(218, 68, 83)));
        assert_eq!(v.defaults.warning_color, Some(Rgba::rgb(246, 116, 0)));
        assert_eq!(v.defaults.success_color, Some(Rgba::rgb(39, 174, 96)));
        assert_eq!(v.defaults.info_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Selection ===

    #[test]
    fn test_selection_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.defaults.selection_background,
            Some(Rgba::rgb(61, 174, 233))
        );
        assert_eq!(
            v.defaults.selection_text_color,
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Checkbox and radio ===

    /// The groups Breeze's checkbox reads, from BreezeLight.colors and
    /// BreezeDark.colors at breeze v6.7.5 (:29-39, :85-95, :113-137).
    fn breeze_checkbox_groups(
        window: &str,
        window_fg: &str,
        button: &str,
        view_fg: &str,
    ) -> String {
        format!(
            "[Colors:Window]\nBackgroundNormal={window}\nForegroundNormal={window_fg}\n\n\
             [Colors:Button]\nBackgroundNormal={button}\n\n\
             [Colors:View]\nForegroundNormal={view_fg}\n\n\
             [Colors:Selection]\nBackgroundNormal=61,174,233\n"
        )
    }

    /// The groups the menu, scroll-bar and list colours read, from BreezeLight.colors
    /// and BreezeDark.colors at breeze v6.7.5: Window :129/:130, :137/:138; Button
    /// :31/:32, :39/:40; Selection :87/:88; View DecorationFocus :116/:117.
    fn breeze_menu_list_groups(
        window: &str,
        window_fg: &str,
        button: &str,
        button_fg: &str,
    ) -> String {
        format!(
            "[Colors:Window]\nBackgroundNormal={window}\nForegroundNormal={window_fg}\n\n\
             [Colors:Button]\nBackgroundNormal={button}\nForegroundNormal={button_fg}\n\n\
             [Colors:Selection]\nBackgroundNormal=61,174,233\n\n\
             [Colors:View]\nDecorationFocus=61,174,233\n"
        )
    }

    /// docs/platform-facts.md §2.5's KDE cells, which the kde-breeze preset states.
    #[test]
    fn test_checkbox_colors_are_breezes() {
        let cases = [
            (
                breeze_checkbox_groups("239,240,241", "35,38,41", "252,252,252", "35,38,41"),
                [
                    (194, 228, 246),
                    (252, 252, 252),
                    (35, 38, 41),
                    (198, 200, 201),
                ],
            ),
            (
                breeze_checkbox_groups("32,35,38", "252,252,252", "41,44,48", "252,252,252"),
                [(47, 83, 104), (41, 44, 48), (252, 252, 252), (76, 78, 81)],
            ),
        ];
        for (content, [checked, unchecked, mark, unchecked_border]) in cases {
            let v = populate_fixture(&content);
            let rgb = |(r, g, b): (u8, u8, u8)| Some(Rgba::rgb(r, g, b));
            assert_eq!(v.checkbox.checked_background, rgb(checked));
            assert_eq!(v.checkbox.unchecked_background, rgb(unchecked));
            assert_eq!(v.checkbox.indicator_color, rgb(mark));
            assert_eq!(v.checkbox.unchecked_border_color, rgb(unchecked_border));
            assert_eq!(
                v.checkbox.border.as_ref().and_then(|b| b.color),
                rgb((61, 174, 233))
            );
        }
    }

    #[test]
    fn test_checkbox_unchecked_border_follows_frame_contrast() {
        let content = breeze_checkbox_groups("0,0,0", "200,100,0", "0,0,0", "0,0,0");
        let with = |contrast: &str| {
            populate_fixture(&format!("{content}\n[KDE]\nframeContrast={contrast}\n"))
                .checkbox
                .unchecked_border_color
        };
        assert_eq!(with("0.5"), Some(Rgba::rgb(100, 50, 0)));
        assert_eq!(with("2"), Some(Rgba::rgb(200, 100, 0)), "clamped to 1");
        assert_eq!(with("none"), Some(Rgba::rgb(40, 20, 0)), "the 0.2 default");
    }

    /// Pixels `KColorUtils::overlayColors` returned under Qt 6.11.2 and KF6
    /// KGuiAddons for a colour `setAlphaF`'d to the alpha, over an opaque base
    /// (scratchpad program `ovtest.cpp`, 20 000 random inputs, of which these
    /// are six); `overlay_colors` matched all of them.
    #[test]
    fn overlay_colors_is_qts_pixel() {
        let cases = [
            ((220, 4, 101), (170, 31, 173), 0.29, (205, 12, 122)),
            ((90, 218, 229), (172, 27, 30), 0.95, (168, 37, 40)),
            ((209, 238, 57), (16, 203, 72), 0.48, (117, 221, 65)),
            ((116, 55, 154), (111, 144, 12), 0.19, (115, 72, 127)),
            ((164, 249, 31), (161, 98, 185), 0.75, (162, 135, 147)),
            ((148, 102, 144), (95, 160, 85), 0.04, (146, 104, 141)),
        ];
        for (base, paint, alpha, expected) in cases {
            let rgb = |(r, g, b): (u8, u8, u8)| Rgba::rgb(r, g, b);
            assert_eq!(
                overlay_colors(rgb(base), rgb(paint), alpha),
                rgb(expected),
                "{base:?} under {paint:?} at {alpha}"
            );
        }
    }

    /// A test vector: two colours, the factor, and the 16-bit red, green,
    /// blue and alpha `QColor` holds for the result.
    type QtVector = ([u8; 3], [u8; 3], f64, [u16; 4]);

    include!("qt_color_vectors.rs");

    fn rgb([r, g, b]: [u8; 3]) -> Rgba {
        Rgba::rgb(r, g, b)
    }

    /// `tint` gives the colour Qt 6.11.2's own `Qt.tint` stores, to the
    /// 16-bit channel (scripts/generate_qt_color_vectors.py).
    #[test]
    fn tint_is_qts() {
        for (base, over, alpha, [r, g, b, a]) in QT_TINT {
            let tinted = tint(
                QtColor::from_rgba(rgb(base)),
                QtColor::from_rgba(rgb(over)).with_alpha_f(alpha as f32),
            );
            assert_eq!(
                tinted,
                QtColor { r, g, b, a },
                "{base:?} tinted by {over:?} at {alpha}"
            );
        }
    }

    /// `mix_qt` gives the colour `KColorUtils::mix` makes over Qt 6.11.2's
    /// `QColor`, to the 16-bit channel (scripts/generate_qt_color_vectors.py).
    #[test]
    fn mix_is_kcolorutils() {
        for (one, two, bias, [r, g, b, a]) in KCOLORUTILS_MIX {
            assert_eq!(
                mix_qt(rgb(one), rgb(two), bias),
                QtColor { r, g, b, a },
                "{one:?} towards {two:?} by {bias}"
            );
        }
    }

    /// `linear_interpolation_qt` gives the colour Kirigami's
    /// `linearInterpolation` makes over Qt 6.11.2's `QColor`, to the 16-bit
    /// channel (scripts/generate_qt_color_vectors.py).
    #[test]
    fn linear_interpolation_is_kirigamis() {
        for (one, two, balance, [r, g, b, a]) in KIRIGAMI_LINEAR_INTERPOLATION {
            assert_eq!(
                linear_interpolation_qt(rgb(one), rgb(two), balance),
                QtColor { r, g, b, a },
                "{one:?} towards {two:?} by {balance}"
            );
        }
    }

    /// docs/platform-facts.md §2.10's KDE cells, which the kde-breeze preset
    /// states: the accent at 0.7 over the window, and the window text at the
    /// frame contrast for the outline.
    #[test]
    fn test_progress_bar_colors_are_breezes() {
        let cases = [
            (
                breeze_checkbox_groups("239,240,241", "35,38,41", "252,252,252", "35,38,41"),
                Rgba::rgb(0x72, 0xc2, 0xeb),
                Rgba::new(0x23, 0x26, 0x29, 0x33),
            ),
            (
                breeze_checkbox_groups("32,35,38", "252,252,252", "41,44,48", "252,252,252"),
                Rgba::rgb(0x35, 0x84, 0xae),
                Rgba::new(0xfc, 0xfc, 0xfc, 0x33),
            ),
        ];
        for (content, fill, outline) in cases {
            let v = populate_fixture(&content);
            assert_eq!(v.progress_bar.fill_color, Some(fill));
            assert_eq!(
                v.progress_bar.border.as_ref().and_then(|b| b.color),
                Some(outline)
            );
        }
    }

    /// docs/platform-facts.md §2.3's KDE cells, which the kde-breeze preset
    /// states: a checked button is `Button` mixed an eighth towards
    /// `ButtonText`, lettered in `ButtonText`.
    #[test]
    fn test_checked_button_colors_are_breezes() {
        let cases = [
            ("252,252,252", "35,38,41", Rgba::rgb(0xe1, 0xe1, 0xe2)),
            ("41,44,48", "252,252,252", Rgba::rgb(0x43, 0x46, 0x4a)),
        ];
        for (button, text, checked) in cases {
            let v = populate_fixture(&format!(
                "[Colors:Button]\nBackgroundNormal={button}\nForegroundNormal={text}\n"
            ));
            assert_eq!(v.button.checked_background, Some(checked), "{button}");
            assert_eq!(
                v.button.checked_text_color,
                v.button.font.as_ref().and_then(|f| f.color)
            );
        }
    }

    // === Per-widget: Button ===

    #[test]
    fn test_button_background_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_button_font_color_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.button.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    // === Per-widget: Tooltip ===

    #[test]
    fn test_tooltip_background_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_tooltip_font_color_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.tooltip.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Sidebar (View) ===

    #[test]
    fn test_sidebar_background_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_sidebar_font_color_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    /// The Complementary set, which no KDE sidebar uses, does not reach the
    /// sidebar (it gave Breeze Light a dark one).
    #[test]
    fn test_sidebar_ignores_complementary() {
        let content = "\
[Colors:Complementary]
BackgroundNormal=42,46,50
ForegroundNormal=239,240,241
";
        let v = populate_fixture(content);
        assert!(v.sidebar.background_color.is_none());
        assert!(v.sidebar.font.is_none());
    }

    // === KDE-01: Title bar from WM ===

    #[test]
    fn test_title_bar_background_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.window.title_bar_background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_title_bar_font_color_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.title_bar_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_inactive_title_bar_background_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_background,
            Some(Rgba::rgb(42, 46, 50))
        );
    }

    #[test]
    fn test_inactive_title_bar_text_color_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_text_color,
            Some(Rgba::rgb(161, 169, 177))
        );
    }

    // === KDE-02: Input placeholder and caret ===

    #[test]
    fn test_input_placeholder_from_view_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.placeholder_color, Some(Rgba::rgb(161, 169, 177)));
    }

    #[test]
    fn test_input_caret_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.caret_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === accent_text_color from Selection ===

    #[test]
    fn test_accent_text_color_from_selection() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent_text_color, Some(Rgba::rgb(252, 252, 252)));
    }

    // === KDE-02: List background/item_font from View ===

    #[test]
    fn test_list_background_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.background_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_item_font_color_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.item_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === KDE-02: List alternate_row ===

    #[test]
    fn test_list_alternate_row_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.alternate_row_background, Some(Rgba::rgb(30, 33, 36)));
    }

    // === List header from the Button set ===

    #[test]
    fn test_list_header_background_from_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.header_background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_list_header_font_color_from_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    // === Menu, scroll bar and list colours Breeze derives ===

    /// The Breeze Light and Breeze Dark groups of `breeze_menu_list_groups`.
    fn breeze_light_and_dark() -> [String; 2] {
        [
            breeze_menu_list_groups("239,240,241", "35,38,41", "252,252,252", "35,38,41"),
            breeze_menu_list_groups("32,35,38", "252,252,252", "41,44,48", "252,252,252"),
        ]
    }

    #[test]
    fn test_menu_hover_and_separator_are_breezes() {
        let [light, dark] = breeze_light_and_dark();
        let cases = [
            (
                light,
                Rgba::rgb(0x23, 0x26, 0x29),
                Rgba::rgb(0xc6, 0xc8, 0xc9),
            ),
            (
                dark,
                Rgba::rgb(0xfc, 0xfc, 0xfc),
                Rgba::rgb(0x4c, 0x4e, 0x51),
            ),
        ];
        for (content, text, separator) in cases {
            let v = populate_fixture(&content);
            assert_eq!(
                v.menu.hover_background,
                Some(Rgba::new(0x3d, 0xae, 0xe9, 0x4d))
            );
            assert_eq!(v.menu.hover_text_color, Some(text));
            assert_eq!(v.menu.separator_color, Some(separator));
        }
    }

    /// A hovered item's outline, `focusOutlineColor()`: the focus colour
    /// mixed 15 % towards the window text (docs/platform-facts.md §2.6).
    #[test]
    fn test_menu_hover_outline_is_breezes() {
        let [light, dark] = breeze_light_and_dark();
        let cases = [
            (light, Rgba::rgb(0x39, 0x9a, 0xcc)),
            (dark, Rgba::rgb(0x5a, 0xba, 0xec)),
        ];
        for (content, outline) in cases {
            let v = populate_fixture(&content);
            assert_eq!(v.menu.hover_border_color, Some(outline));
        }
    }

    #[test]
    fn test_scrollbar_thumb_is_breezes() {
        let [light, dark] = breeze_light_and_dark();
        let cases = [
            (light, Rgba::rgb(0xcb, 0xcd, 0xcd)),
            (dark, Rgba::rgb(0x46, 0x49, 0x4b)),
        ];
        for (content, thumb) in cases {
            let v = populate_fixture(&content);
            assert_eq!(v.scrollbar.thumb_color, Some(thumb));
            assert!(
                v.scrollbar.track_color.is_none(),
                "the reader leaves the track to the preset"
            );
        }
    }

    #[test]
    fn test_list_hover_and_header_are_breezes() {
        let [light, dark] = breeze_light_and_dark();
        let cases = [
            (
                light,
                Rgba::rgb(0xfc, 0xfc, 0xfc),
                Rgba::rgb(0x23, 0x26, 0x29),
            ),
            (
                dark,
                Rgba::rgb(0x29, 0x2c, 0x30),
                Rgba::rgb(0xfc, 0xfc, 0xfc),
            ),
        ];
        for (content, header, header_text) in cases {
            let v = populate_fixture(&content);
            assert_eq!(
                v.list.hover_background,
                Some(Rgba::new(0x3d, 0xae, 0xe9, 0x4d))
            );
            assert_eq!(v.list.header_background, Some(header));
            assert_eq!(
                v.list.header_font.as_ref().and_then(|f| f.color),
                Some(header_text)
            );
        }
    }

    /// A scheme whose window, button, selection and focus colours are not
    /// Breeze's reaches the menu's, the scroll bar's and the list's colours.
    #[test]
    fn a_custom_scheme_moves_the_menu_scrollbar_and_list_colours() {
        let [light, _] = breeze_light_and_dark();
        let breeze = populate_fixture(&light);
        let custom = breeze_menu_list_groups("230,220,200", "90,40,20", "200,180,150", "60,20,10")
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90");
        let v = populate_fixture(&custom);
        let window = Rgba::rgb(230, 220, 200);
        let text = Rgba::rgb(90, 40, 20);
        assert_eq!(v.menu.hover_background, Some(Rgba::new(20, 200, 90, 0x4d)));
        assert_eq!(v.menu.hover_text_color, Some(text));
        assert_eq!(
            v.menu.separator_color,
            Some(mix(window, text, FRAME_CONTRAST_DEFAULT))
        );
        assert_eq!(
            v.scrollbar.thumb_color,
            Some(overlay_colors(window, text, SCROLLBAR_IDLE_HANDLE_ALPHA))
        );
        assert_eq!(v.list.hover_background, Some(Rgba::new(200, 40, 90, 0x4d)));
        assert_eq!(v.list.header_background, Some(Rgba::rgb(200, 180, 150)));
        let header_text = |mode: &ThemeMode| mode.list.header_font.as_ref().and_then(|f| f.color);
        assert_eq!(header_text(&v), Some(Rgba::rgb(60, 20, 10)));
        for (field, ours, breezes) in [
            (
                "menu.hover_background",
                v.menu.hover_background,
                breeze.menu.hover_background,
            ),
            (
                "menu.hover_text_color",
                v.menu.hover_text_color,
                breeze.menu.hover_text_color,
            ),
            (
                "menu.separator_color",
                v.menu.separator_color,
                breeze.menu.separator_color,
            ),
            (
                "scrollbar.thumb_color",
                v.scrollbar.thumb_color,
                breeze.scrollbar.thumb_color,
            ),
            (
                "list.hover_background",
                v.list.hover_background,
                breeze.list.hover_background,
            ),
            (
                "list.header_background",
                v.list.header_background,
                breeze.list.header_background,
            ),
            (
                "list.header_font.color",
                header_text(&v),
                header_text(&breeze),
            ),
        ] {
            assert_ne!(ours, breezes, "{field}: the custom scheme did not reach it");
        }
    }

    /// A button's hover and focus border is the scheme's selection colour
    /// (docs/platform-facts.md §2.3), on a scheme whose selection is not Breeze's.
    #[test]
    fn a_buttons_hover_and_focus_border_is_the_selection() {
        let custom = breeze_menu_list_groups("230,220,200", "90,40,20", "200,180,150", "60,20,10")
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90");
        let v = populate_fixture(&custom);
        assert_eq!(v.button.hover_border_color, Some(Rgba::rgb(200, 40, 90)));
        assert_eq!(v.button.focus_border_color, Some(Rgba::rgb(200, 40, 90)));
    }

    /// The check box's hover outline is the scheme's `DecorationFocus`, the
    /// combo box's and a toolbar button's hover pen its selection colour, a
    /// toolbar button's pressed fill that colour at `Blend_Value` and its
    /// checked fill the button text at 0.125 (docs/platform-facts.md §2.5,
    /// §2.13, §2.24), on a scheme whose colours are not Breeze's: both Breeze
    /// schemes give `DecorationFocus` and the selection the same #3daee9.
    #[test]
    fn a_custom_scheme_moves_the_hover_borders() {
        let [light, _] = breeze_light_and_dark();
        let breeze = populate_fixture(&light);
        let custom = breeze_menu_list_groups("230,220,200", "90,40,20", "200,180,150", "60,20,10")
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90");
        let v = populate_fixture(&custom);
        assert_eq!(v.checkbox.hover_border_color, Some(Rgba::rgb(20, 200, 90)));
        assert_eq!(v.combo_box.hover_border_color, Some(Rgba::rgb(200, 40, 90)));
        assert_eq!(
            v.toolbar.button_hover_border_color,
            Some(Rgba::rgb(200, 40, 90))
        );
        assert_eq!(
            v.toolbar.button_active_background,
            Some(Rgba::new(200, 40, 90, 0x4d))
        );
        assert_eq!(
            v.toolbar.button_checked_background,
            Some(Rgba::new(60, 20, 10, 0x20))
        );
        for (field, ours, breezes) in [
            (
                "checkbox.hover_border_color",
                v.checkbox.hover_border_color,
                breeze.checkbox.hover_border_color,
            ),
            (
                "combo_box.hover_border_color",
                v.combo_box.hover_border_color,
                breeze.combo_box.hover_border_color,
            ),
            (
                "toolbar.button_hover_border_color",
                v.toolbar.button_hover_border_color,
                breeze.toolbar.button_hover_border_color,
            ),
            (
                "toolbar.button_active_background",
                v.toolbar.button_active_background,
                breeze.toolbar.button_active_background,
            ),
            (
                "toolbar.button_checked_background",
                v.toolbar.button_checked_background,
                breeze.toolbar.button_checked_background,
            ),
        ] {
            assert_ne!(ours, breezes, "{field}: the custom scheme did not reach it");
        }
    }

    /// A flat Breeze toolbar button's states (docs/platform-facts.md §2.13):
    /// the pen `Highlight` under the pointer, `Highlight` at `Blend_Value`
    /// pressed, the button text at 0.125 checked; the transparent hover fill
    /// is the preset's alone.
    #[test]
    fn test_toolbar_button_states_are_breezes() {
        let [light, dark] = breeze_light_and_dark();
        for (content, checked) in [
            (light, Rgba::new(0x23, 0x26, 0x29, 0x20)),
            (dark, Rgba::new(0xfc, 0xfc, 0xfc, 0x20)),
        ] {
            let v = populate_fixture(&content);
            assert_eq!(
                v.toolbar.button_hover_border_color,
                Some(Rgba::rgb(0x3d, 0xae, 0xe9))
            );
            assert_eq!(
                v.toolbar.button_active_background,
                Some(Rgba::new(0x3d, 0xae, 0xe9, 0x4d))
            );
            assert_eq!(v.toolbar.button_checked_background, Some(checked));
            assert_eq!(v.toolbar.button_hover_background, None);
        }
    }

    /// Breeze's keyboard focus band is the selection colour at `Blend_Value`
    /// (docs/platform-facts.md §2.1.5), not `DecorationFocus`, which stays the
    /// caret's, opaque: under Breeze Light and Dark, and under a scheme whose two
    /// keys differ.
    #[test]
    fn the_focus_ring_is_the_selection_at_blend_value() {
        let [light, dark] = breeze_light_and_dark();
        let custom = light
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90");
        let cases = [
            (
                light,
                Rgba::new(61, 174, 233, 0x4d),
                Rgba::rgb(61, 174, 233),
            ),
            (dark, Rgba::new(61, 174, 233, 0x4d), Rgba::rgb(61, 174, 233)),
            (custom, Rgba::new(200, 40, 90, 0x4d), Rgba::rgb(20, 200, 90)),
        ];
        for (content, ring, caret) in cases {
            let v = populate_fixture(&content);
            assert_eq!(v.defaults.focus_ring_color, Some(ring));
            assert_eq!(v.input.caret_color, Some(caret));
        }
    }

    #[test]
    fn test_menu_separator_follows_frame_contrast() {
        let content = breeze_menu_list_groups("0,0,0", "200,100,0", "252,252,252", "35,38,41");
        let with = |contrast: &str| {
            populate_fixture(&format!("{content}\n[KDE]\nframeContrast={contrast}\n"))
                .menu
                .separator_color
        };
        assert_eq!(with("0.5"), Some(Rgba::rgb(100, 50, 0)));
        assert_eq!(with("2"), Some(Rgba::rgb(200, 100, 0)), "clamped to 1");
        assert_eq!(with("none"), Some(Rgba::rgb(40, 20, 0)), "the 0.2 default");
    }

    /// The column header is the Button set's; the Header set reaches the
    /// toolbar only.
    #[test]
    fn test_the_header_ignores_the_header_set() {
        let v = populate_fixture(
            "[Colors:Header]\nBackgroundNormal=1,2,3\nForegroundNormal=4,5,6\n\n\
             [Colors:Button]\nBackgroundNormal=7,8,9\nForegroundNormal=10,11,12\n",
        );
        assert_eq!(v.list.header_background, Some(Rgba::rgb(7, 8, 9)));
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(10, 11, 12))
        );
        assert_eq!(v.toolbar.background_color, Some(Rgba::rgb(1, 2, 3)));
    }

    #[test]
    fn test_missing_groups_leave_the_new_fields_to_the_preset() {
        let v = populate_fixture("[Colors:Selection]\nBackgroundNormal=61,174,233\n");
        assert!(v.list.hover_background.is_some());
        assert!(v.menu.hover_background.is_none());
        assert!(v.menu.hover_text_color.is_none());
        assert!(v.menu.separator_color.is_none());
        assert!(v.scrollbar.thumb_color.is_none());
        assert!(v.list.header_background.is_none());
        assert!(v.list.header_font.is_none());
        assert!(v.checkbox.hover_border_color.is_none());
        assert!(v.combo_box.hover_border_color.is_some());
        assert!(v.toolbar.button_hover_border_color.is_some());
        assert!(v.toolbar.button_active_background.is_some());
        assert!(v.toolbar.button_checked_background.is_none());

        let v = populate_fixture("[Colors:Window]\nBackgroundNormal=239,240,241\n");
        assert!(v.menu.separator_color.is_none());
        assert!(v.scrollbar.thumb_color.is_none());
        assert!(v.list.hover_background.is_none());
        assert!(v.menu.hover_background.is_none());
        assert!(v.menu.hover_text_color.is_none());
        assert!(v.combo_box.hover_border_color.is_none());
        assert!(v.toolbar.button_hover_border_color.is_none());
        assert!(v.toolbar.button_active_background.is_none());
        assert!(v.toolbar.button_checked_background.is_none());
    }

    /// A table's grid is `QPalette::Mid`, the window's `MidShade` at the
    /// scheme's contrast: Breeze's measured #c4c8cc / #1c1f21
    /// (docs/platform-facts.md §2.15).
    #[test]
    fn test_table_grid_is_qts_mid_shade() {
        for (window, expected) in [
            ("239,240,241", Rgba::rgb(0xc4, 0xc8, 0xcc)),
            ("32,35,38", Rgba::rgb(0x1c, 0x1f, 0x21)),
        ] {
            let v = populate_fixture(&format!(
                "[Colors:Window]\nBackgroundNormal={window}\n\n[KDE]\ncontrast=4\n"
            ));
            assert_eq!(v.list.grid_color, Some(expected), "{window}");
        }
    }

    /// The grid follows `[KDE] contrast`; a step beyond 10 acts as 10, and a
    /// missing or non-integer value is KColorScheme's default, 7.
    #[test]
    fn test_table_grid_follows_the_contrast() {
        let grid = |kde: &str| {
            populate_fixture(&format!(
                "[Colors:Window]\nBackgroundNormal=239,240,241\n\n{kde}"
            ))
            .list
            .grid_color
        };
        assert_eq!(
            grid("[KDE]\ncontrast=0\n"),
            Some(Rgba::rgb(0xce, 0xd1, 0xd4))
        );
        assert_eq!(
            grid("[KDE]\ncontrast=-3\n"),
            Some(Rgba::rgb(0xd5, 0xd7, 0xda))
        );
        assert_eq!(
            grid("[KDE]\ncontrast=10\n"),
            Some(Rgba::rgb(0xb5, 0xba, 0xbf))
        );
        assert_eq!(
            grid("[KDE]\ncontrast=12\n"),
            grid("[KDE]\ncontrast=10\n"),
            "a step beyond 10 acts as 10"
        );
        assert_eq!(
            grid(""),
            Some(Rgba::rgb(0xbd, 0xc1, 0xc6)),
            "KColorScheme's default, 7"
        );
        assert_eq!(grid("[KDE]\ncontrast=high\n"), grid(""), "not an integer");
        assert_eq!(grid("[KDE]\ncontrast=4.5\n"), grid(""), "not an integer");
    }

    /// Breeze's menu panel, `mix(Window, Base, 0.3)`: the measured #f4f5f5 /
    /// #1c1f22 (docs/platform-facts.md §2.6).
    #[test]
    fn test_menu_panel_is_breezes() {
        for (window, view, expected) in [
            ("239,240,241", "255,255,255", Rgba::rgb(0xf4, 0xf5, 0xf5)),
            ("32,35,38", "20,22,24", Rgba::rgb(0x1c, 0x1f, 0x22)),
        ] {
            let v = populate_fixture(&format!(
                "[Colors:Window]\nBackgroundNormal={window}\n\n\
                 [Colors:View]\nBackgroundNormal={view}\n"
            ));
            assert_eq!(v.menu.background_color, Some(expected), "{window}");
        }
    }

    /// Another scheme's Window and View sets give their own panel and grid,
    /// the grid at the default step where the scheme states no contrast.
    #[test]
    fn a_custom_scheme_moves_the_menu_panel_and_the_grid() {
        let window = Rgba::rgb(230, 220, 200);
        let view = Rgba::rgb(250, 240, 230);
        let v = populate_fixture(
            "[Colors:Window]\nBackgroundNormal=230,220,200\n\n\
             [Colors:View]\nBackgroundNormal=250,240,230\n",
        );
        assert_eq!(
            v.menu.background_color,
            Some(mix(window, view, MENU_PANEL_MIX))
        );
        assert_eq!(v.list.grid_color, Some(mid_shade(window, CONTRAST_DEFAULT)));
        assert_ne!(v.menu.background_color, Some(Rgba::rgb(0xf4, 0xf5, 0xf5)));
        assert_ne!(v.list.grid_color, Some(Rgba::rgb(0xc4, 0xc8, 0xcc)));
    }

    #[test]
    fn test_missing_groups_leave_the_panel_and_the_grid_to_the_preset() {
        let v = populate_fixture("[Colors:View]\nBackgroundNormal=255,255,255\n");
        assert!(v.list.grid_color.is_none());
        assert!(v.menu.background_color.is_none());

        let v = populate_fixture("[Colors:Window]\nBackgroundNormal=239,240,241\n");
        assert!(v.list.grid_color.is_some());
        assert!(v.menu.background_color.is_none());
    }

    // === KDE-02: Link visited ===

    #[test]
    fn test_link_visited_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.link.visited_text_color, Some(Rgba::rgb(155, 89, 182)));
    }

    // === Combo box from Button, menu text from Window ===

    /// docs/platform-facts.md §2.24 and §2.6's KDE cells: the combo box in
    /// the Button set, the menu text in the Window set's foreground.
    #[test]
    fn test_combo_box_from_button_and_menu_text_from_window() {
        let v = populate_fixture(
            "[Colors:Window]\nBackgroundNormal=1,1,1\nForegroundNormal=2,2,2\n\n\
             [Colors:Button]\nBackgroundNormal=3,3,3\nForegroundNormal=4,4,4\n",
        );
        assert_eq!(v.combo_box.background_color, Some(Rgba::rgb(3, 3, 3)));
        assert_eq!(
            v.combo_box.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(4, 4, 4))
        );
        assert_eq!(
            v.menu.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(2, 2, 2))
        );
    }

    /// docs/platform-facts.md §2.16 and §2.18's KDE cells: a popover takes
    /// the window's fill and text and a separator the border colour, by
    /// inheritance, so the reader states none of them; it derives only the
    /// popover's edge.
    #[test]
    fn test_popover_and_separator_are_inherited() {
        let v = populate_fixture(BREEZE_DARK);
        assert!(v.popover.background_color.is_none());
        assert!(v.popover.font.is_none());
        assert!(v.separator.line_color.is_none());
    }

    // === Sidebar None when View missing ===

    #[test]
    fn test_sidebar_none_without_view() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
ForegroundNormal=252,252,252
";
        let v = populate_fixture(content);
        assert!(v.sidebar.background_color.is_none());
        assert!(v.sidebar.font.is_none());
    }

    // === Empty INI ===

    #[test]
    fn test_empty_ini() {
        let ini = super::super::create_kde_parser();
        let mut variant = ThemeMode::default();
        populate_colors(&ini, &mut variant);
        assert!(variant.defaults.accent_color.is_none());
        assert!(variant.defaults.background_color.is_none());
        assert!(variant.defaults.text_color.is_none());
        assert!(variant.button.background_color.is_none());
    }

    // === Partial sections ===

    #[test]
    fn test_partial_sections() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
DecorationFocus=61,174,233
";
        let v = populate_fixture(content);
        assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
        assert_eq!(v.defaults.muted_color, Some(Rgba::rgb(161, 169, 177)));
        assert!(v.defaults.border.is_empty()); // border not set by reader
        assert!(v.defaults.accent_color.is_none());
        assert!(v.defaults.surface_color.is_none());
        assert!(v.button.background_color.is_none());
    }

    // === Malformed values ===

    #[test]
    fn test_malformed_values() {
        let content = "\
[Colors:View]
BackgroundNormal=abc,def,ghi
ForegroundNormal=252,252,252
DecorationFocus=61,174,233
";
        let v = populate_fixture(content);
        assert!(v.defaults.surface_color.is_none()); // BackgroundNormal was malformed
        assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Full mapping: all non-None fields populated ===

    #[test]
    fn test_full_mapping_completeness() {
        let v = populate_fixture(BREEZE_DARK);

        // defaults-level
        assert!(v.defaults.accent_color.is_some(), "accent_color missing");
        assert!(
            v.defaults.background_color.is_some(),
            "background_color missing"
        );
        assert!(v.defaults.text_color.is_some(), "text_color missing");
        assert!(v.defaults.surface_color.is_some(), "surface_color missing");
        // border is not set by reader (preset value)
        assert!(v.defaults.muted_color.is_some(), "muted_color missing");
        assert!(v.defaults.link_color.is_some(), "link_color missing");
        assert!(
            v.defaults.selection_background.is_some(),
            "selection_background missing"
        );
        assert!(
            v.defaults.selection_text_color.is_some(),
            "selection_text_color missing"
        );
        assert!(
            v.defaults.accent_text_color.is_some(),
            "accent_text_color missing"
        );
        assert!(v.defaults.danger_color.is_some(), "danger_color missing");
        assert!(v.defaults.warning_color.is_some(), "warning_color missing");
        assert!(v.defaults.success_color.is_some(), "success_color missing");
        assert!(v.defaults.info_color.is_some(), "info_color missing");

        // per-widget
        assert!(
            v.button.background_color.is_some(),
            "button.background_color missing"
        );
        assert!(
            v.button.font.as_ref().and_then(|f| f.color).is_some(),
            "button.font.color missing"
        );
        assert!(
            v.tooltip.background_color.is_some(),
            "tooltip.background_color missing"
        );
        assert!(
            v.tooltip.font.as_ref().and_then(|f| f.color).is_some(),
            "tooltip.font.color missing"
        );
        assert!(
            v.sidebar.background_color.is_some(),
            "sidebar.background_color missing"
        );
        assert!(
            v.sidebar.font.as_ref().and_then(|f| f.color).is_some(),
            "sidebar.font.color missing"
        );
        assert!(
            v.input.background_color.is_some(),
            "input.background_color missing"
        );
        assert!(
            v.input.font.as_ref().and_then(|f| f.color).is_some(),
            "input.font.color missing"
        );
        assert!(
            v.input.placeholder_color.is_some(),
            "input.placeholder_color missing"
        );
        assert!(v.input.caret_color.is_some(), "input.caret_color missing");
        assert!(
            v.combo_box.background_color.is_some(),
            "combo_box.background_color missing"
        );
        assert!(
            v.combo_box.font.as_ref().and_then(|f| f.color).is_some(),
            "combo_box.font.color missing"
        );
        assert!(
            v.checkbox.hover_border_color.is_some(),
            "checkbox.hover_border_color missing"
        );
        assert!(
            v.combo_box.hover_border_color.is_some(),
            "combo_box.hover_border_color missing"
        );
        assert!(
            v.toolbar.button_hover_border_color.is_some(),
            "toolbar.button_hover_border_color missing"
        );
        assert!(
            v.toolbar.button_active_background.is_some(),
            "toolbar.button_active_background missing"
        );
        assert!(
            v.toolbar.button_checked_background.is_some(),
            "toolbar.button_checked_background missing"
        );
        assert!(
            v.button.default_button_background.is_some(),
            "button.default_button_background missing"
        );
        assert!(
            v.button.default_button_text_color.is_some(),
            "button.default_button_text_color missing"
        );
        assert!(
            v.button.default_button_border_color.is_some(),
            "button.default_button_border_color missing"
        );
        assert!(
            v.menu.font.as_ref().and_then(|f| f.color).is_some(),
            "menu.font.color missing"
        );
        assert!(
            v.menu.hover_background.is_some(),
            "menu.hover_background missing"
        );
        assert!(
            v.menu.hover_text_color.is_some(),
            "menu.hover_text_color missing"
        );
        assert!(
            v.menu.separator_color.is_some(),
            "menu.separator_color missing"
        );
        assert!(
            v.scrollbar.thumb_color.is_some(),
            "scrollbar.thumb_color missing"
        );
        assert!(
            v.list.hover_background.is_some(),
            "list.hover_background missing"
        );
        assert!(
            v.list.background_color.is_some(),
            "list.background_color missing"
        );
        assert!(
            v.list.item_font.as_ref().and_then(|f| f.color).is_some(),
            "list.item_font.color missing"
        );
        assert!(
            v.list.alternate_row_background.is_some(),
            "list.alternate_row_background missing"
        );
        assert!(
            v.list.header_background.is_some(),
            "list.header_background missing"
        );
        assert!(
            v.list.header_font.as_ref().and_then(|f| f.color).is_some(),
            "list.header_font.color missing"
        );
        assert!(
            v.list.header_hover_background.is_some(),
            "list.header_hover_background missing"
        );
        assert!(
            v.list.header_active_background.is_some(),
            "list.header_active_background missing"
        );
        assert!(
            v.defaults.selection_inactive_background.is_some(),
            "defaults.selection_inactive_background missing"
        );
        assert!(
            v.defaults.selection_inactive_text_color.is_some(),
            "defaults.selection_inactive_text_color missing"
        );
        assert!(v.list.grid_color.is_some(), "list.grid_color missing");
        assert!(
            v.menu.background_color.is_some(),
            "menu.background_color missing"
        );
        assert!(
            v.menu.hover_border_color.is_some(),
            "menu.hover_border_color missing"
        );
        assert!(
            v.menu.bar_hover_background.is_some(),
            "menu.bar_hover_background missing"
        );
        assert!(
            v.menu.bar_open_background.is_some(),
            "menu.bar_open_background missing"
        );
        assert!(
            v.menu.bar_hover_text_color.is_some(),
            "menu.bar_hover_text_color missing"
        );
        assert!(
            v.list.hover_border_color.is_some(),
            "list.hover_border_color missing"
        );
        assert!(
            v.link.visited_text_color.is_some(),
            "link.visited_text_color missing"
        );

        // WM title bar
        assert!(
            v.window.title_bar_background.is_some(),
            "window.title_bar_background missing"
        );
        assert!(
            v.window
                .title_bar_font
                .as_ref()
                .and_then(|f| f.color)
                .is_some(),
            "window.title_bar_font.color missing"
        );
        assert!(
            v.window.inactive_title_bar_background.is_some(),
            "window.inactive_title_bar_background missing"
        );
        assert!(
            v.window.inactive_title_bar_text_color.is_some(),
            "window.inactive_title_bar_text_color missing"
        );
    }
}
