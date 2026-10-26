// KDE color group parsing -> populate per-widget fields on ThemeMode
// Maps semantic color roles from KDE INI color groups directly to ThemeMode.

use crate::Rgba;

/// Look up a color key from a KDE INI section and parse it as RGB.
fn get_color(ini: &configparser::ini::Ini, section: &str, key: &str) -> Option<Rgba> {
    let value = ini.get(section, key)?;
    super::parse_rgb(&value)
}

/// Breeze's `Metrics::Blend_Value`, the alpha of the selection colour over a
/// checked indicator's fill (breezemetrics.h:176 and breezehelper.cpp:40 at
/// breeze v6.7.5; docs/platform-facts.md §2.5).
const BLEND_VALUE: f32 = 0.3;

/// `kdeglobals` `[KDE] frameContrast` where it is unset, KColorScheme's own
/// default (kcolorscheme.cpp:529-538 at 27066d47; docs/platform-facts.md §2.11).
const FRAME_CONTRAST_DEFAULT: f64 = 0.2;

/// `color` at `alpha`, held in 8 bits as `Rgba` holds every alpha.
fn translucent(color: Rgba, alpha: f32) -> Rgba {
    let [r, g, b, _] = color.to_f32_array();
    Rgba::from_f32(r, g, b, alpha)
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
    let (a, b) = (QtColor::from_rgba(a), QtColor::from_rgba(b));
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

/// Populate a ThemeMode with colors from KDE INI color groups.
///
/// Maps all standard KDE color groups (View, Window, Button, Selection,
/// Tooltip, Complementary, Header, WM) to per-widget fields on the variant.
/// Missing INI keys result in None fields (no hardcoded fallbacks), so a
/// colour the reader cannot derive keeps the preset's value.
///
/// Some colours Plasma computes from the scheme, and the reader computes
/// them as Breeze, Kirigami and qqc2-desktop-style do, in Qt's arithmetic:
/// the checked button, the checkbox and progress-bar blends, the switch
/// tracks and outlines, the toolbar's fill and edge, and the popover's and the
/// card's edges. The blends on the accent (the checked checkbox, the progress
/// fill, the switch's on track and its outline) come from kdeglobals'
/// `[Colors:Selection] BackgroundNormal`, so an application's `with_overlay` accent or the
/// portal's (`gnome::apply_accent`, applied by `GnomePortalKdeReader`) does
/// not move them.
///
/// The toolbar takes the Header set's fill as Breeze paints it where the
/// window decoration's border size is automatic, None or NoSides
/// (breezehelper.cpp:1904-1941 at breeze f0b1d75); the reader does not read
/// kwinrc and assumes Plasma's default, automatic.
pub(crate) fn populate_colors(ini: &configparser::ini::Ini, variant: &mut crate::ThemeMode) {
    let window_fg = get_color(ini, "Colors:Window", "ForegroundNormal");

    // === defaults-level colors ===
    variant.defaults.accent_color = get_color(ini, "Colors:View", "DecorationFocus");
    variant.defaults.background_color = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.defaults.text_color = window_fg;
    variant.defaults.surface_color = get_color(ini, "Colors:View", "BackgroundNormal");
    // border: not set by reader -- KDE has no native border color API.
    // The preset provides the correct neutral gray (platform-facts: "(preset)").
    variant.defaults.muted_color = get_color(ini, "Colors:Window", "ForegroundInactive");
    // KDE does not expose shadow color in kdeglobals
    variant.defaults.link_color = get_color(ini, "Colors:View", "ForegroundLink");
    variant.defaults.focus_ring_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Selection
    variant.defaults.selection_background = get_color(ini, "Colors:Selection", "BackgroundNormal");
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
    // A hovered button keeps its fill and its text: Breeze's hover changes
    // only the pen (breezehelper.cpp:693-707, :710-712 at breeze f0b1d75), and
    // a non-flat button is lettered in `QPalette::ButtonText` in every state
    // (breezestyle.cpp:5350-5362; docs/platform-facts.md §2.3).
    variant.button.hover_background = variant.button.background_color;
    variant.button.hover_text_color = button_fg;

    // Tooltip
    variant.tooltip.background_color = get_color(ini, "Colors:Tooltip", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Tooltip", "ForegroundNormal") {
        variant.tooltip.font.get_or_insert_default().color = Some(color);
    }

    // Sidebar (from Complementary group)
    variant.sidebar.background_color = get_color(ini, "Colors:Complementary", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Complementary", "ForegroundNormal") {
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
    variant.input.caret_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Checkbox and radio (docs/platform-facts.md §2.5): Breeze fills an
    // unchecked indicator with the button colour and outlines it in
    // `separatorColor()`; a checked one is that fill under the selection
    // colour at `Metrics::Blend_Value`, outlined in the selection colour; the
    // mark is the text colour.
    let button_bg = get_color(ini, "Colors:Button", "BackgroundNormal");
    let selection_bg = get_color(ini, "Colors:Selection", "BackgroundNormal");
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

    // Combo box (docs/platform-facts.md §2.24): the Button set.
    variant.combo_box.background_color = button_bg;
    if let Some(color) = button_fg {
        variant.combo_box.font.get_or_insert_default().color = Some(color);
    }

    // Menu text (docs/platform-facts.md §2.6): the window text.
    if let Some(color) = window_fg {
        variant.menu.font.get_or_insert_default().color = Some(color);
    }

    // KDE-02: list fields (Colors:View is the native source for list/table content areas)
    variant.list.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.list.item_font.get_or_insert_default().color = Some(color);
    }
    variant.list.alternate_row_background = get_color(ini, "Colors:View", "BackgroundAlternate");
    variant.list.header_background = get_color(ini, "Colors:Header", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Header", "ForegroundNormal") {
        variant.list.header_font.get_or_insert_default().color = Some(color);
    }

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

    // === Per-widget: Sidebar (Complementary) ===

    #[test]
    fn test_sidebar_background_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));
    }

    #[test]
    fn test_sidebar_font_color_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
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

    // === KDE-02: List header from Header group ===

    #[test]
    fn test_list_header_background_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_header_font_color_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
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

    // === Sidebar None when Complementary missing ===

    #[test]
    fn test_sidebar_none_without_complementary() {
        let content = "\
[Colors:View]
BackgroundNormal=35,38,41
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
            v.menu.font.as_ref().and_then(|f| f.color).is_some(),
            "menu.font.color missing"
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
