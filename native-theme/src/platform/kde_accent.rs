//! Plasma's accent colours, as KDE applications read them: the kdeglobals colour that
//! `applyScheme()` writes from a colour scheme and an accent, read back through KColorScheme
//! (`kde_accent`), and the `[WM]` colours it writes (`kde_accent_wm`). Every step is one IEEE
//! operation, never fused, on Qt's stored colours.

use super::kde_color_common::{
    QtColor, Rgba, enter, from_hcy, from_rgbf, luma, norm, q16, read, round_to_half, to_hcy,
};

/// A colour set of a KDE colour scheme (`[Colors:Window]` … `[Colors:Header]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccentSet {
    /// `[Colors:Window]`.
    Window,
    /// `[Colors:View]`.
    View,
    /// `[Colors:Button]`.
    Button,
    /// `[Colors:Selection]`.
    Selection,
    /// `[Colors:Tooltip]`.
    Tooltip,
    /// `[Colors:Complementary]`.
    Complementary,
    /// `[Colors:Header]`.
    Header,
}

/// A colour key of a set, in the order Plasma processes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccentKey {
    /// `BackgroundNormal`.
    BackgroundNormal,
    /// `BackgroundAlternate`.
    BackgroundAlternate,
    /// `ForegroundNormal`.
    ForegroundNormal,
    /// `ForegroundInactive`.
    ForegroundInactive,
    /// `ForegroundActive`.
    ForegroundActive,
    /// `ForegroundLink`.
    ForegroundLink,
    /// `ForegroundVisited`.
    ForegroundVisited,
    /// `ForegroundNegative`.
    ForegroundNegative,
    /// `ForegroundNeutral`.
    ForegroundNeutral,
    /// `ForegroundPositive`.
    ForegroundPositive,
    /// `DecorationFocus`.
    DecorationFocus,
    /// `DecorationHover`.
    DecorationHover,
}

impl AccentKey {
    /// The twelve keys, in Plasma's order.
    pub const ALL: [AccentKey; 12] = [
        AccentKey::BackgroundNormal,
        AccentKey::BackgroundAlternate,
        AccentKey::ForegroundNormal,
        AccentKey::ForegroundInactive,
        AccentKey::ForegroundActive,
        AccentKey::ForegroundLink,
        AccentKey::ForegroundVisited,
        AccentKey::ForegroundNegative,
        AccentKey::ForegroundNeutral,
        AccentKey::ForegroundPositive,
        AccentKey::DecorationFocus,
        AccentKey::DecorationHover,
    ];

    /// The keys the accent replaces.
    fn is_accent_key(self) -> bool {
        matches!(
            self,
            AccentKey::ForegroundActive
                | AccentKey::ForegroundLink
                | AccentKey::DecorationFocus
                | AccentKey::DecorationHover
        )
    }
}

/// The state whose colours are read: the active one, or the inactive one before KColorScheme's
/// inactive effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccentGroup {
    /// The set's `[Colors:X]` group.
    Active,
    /// The set's `[Colors:X][Inactive]` group, where the written kdeglobals has one.
    Inactive,
}

/// A key of the `[WM]` group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccentWmKey {
    /// `activeBackground`.
    ActiveBackground,
    /// `activeForeground`.
    ActiveForeground,
    /// `inactiveBackground`.
    InactiveBackground,
    /// `inactiveForeground`.
    InactiveForeground,
}

/// One value per colour key of a set.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AccentKeys<T> {
    /// `BackgroundNormal`.
    pub background_normal: T,
    /// `BackgroundAlternate`.
    pub background_alternate: T,
    /// `ForegroundNormal`.
    pub foreground_normal: T,
    /// `ForegroundInactive`.
    pub foreground_inactive: T,
    /// `ForegroundActive`.
    pub foreground_active: T,
    /// `ForegroundLink`.
    pub foreground_link: T,
    /// `ForegroundVisited`.
    pub foreground_visited: T,
    /// `ForegroundNegative`.
    pub foreground_negative: T,
    /// `ForegroundNeutral`.
    pub foreground_neutral: T,
    /// `ForegroundPositive`.
    pub foreground_positive: T,
    /// `DecorationFocus`.
    pub decoration_focus: T,
    /// `DecorationHover`.
    pub decoration_hover: T,
}

impl<T: Copy> AccentKeys<T> {
    /// The value of `key`.
    pub fn get(&self, key: AccentKey) -> T {
        *self.slot(key)
    }

    fn slot(&self, key: AccentKey) -> &T {
        match key {
            AccentKey::BackgroundNormal => &self.background_normal,
            AccentKey::BackgroundAlternate => &self.background_alternate,
            AccentKey::ForegroundNormal => &self.foreground_normal,
            AccentKey::ForegroundInactive => &self.foreground_inactive,
            AccentKey::ForegroundActive => &self.foreground_active,
            AccentKey::ForegroundLink => &self.foreground_link,
            AccentKey::ForegroundVisited => &self.foreground_visited,
            AccentKey::ForegroundNegative => &self.foreground_negative,
            AccentKey::ForegroundNeutral => &self.foreground_neutral,
            AccentKey::ForegroundPositive => &self.foreground_positive,
            AccentKey::DecorationFocus => &self.decoration_focus,
            AccentKey::DecorationHover => &self.decoration_hover,
        }
    }

    /// The value of `key`, to change.
    pub fn get_mut(&mut self, key: AccentKey) -> &mut T {
        match key {
            AccentKey::BackgroundNormal => &mut self.background_normal,
            AccentKey::BackgroundAlternate => &mut self.background_alternate,
            AccentKey::ForegroundNormal => &mut self.foreground_normal,
            AccentKey::ForegroundInactive => &mut self.foreground_inactive,
            AccentKey::ForegroundActive => &mut self.foreground_active,
            AccentKey::ForegroundLink => &mut self.foreground_link,
            AccentKey::ForegroundVisited => &mut self.foreground_visited,
            AccentKey::ForegroundNegative => &mut self.foreground_negative,
            AccentKey::ForegroundNeutral => &mut self.foreground_neutral,
            AccentKey::ForegroundPositive => &mut self.foreground_positive,
            AccentKey::DecorationFocus => &mut self.decoration_focus,
            AccentKey::DecorationHover => &mut self.decoration_hover,
        }
    }
}

/// One set's scheme inputs and what the scheme file holds of them.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AccentSetInput {
    /// KColorScheme's reading of the file's `[Colors:X]` keys (its default where a key is unset).
    pub active: AccentKeys<Rgba>,
    /// KColorScheme's reading of the file's `[Colors:X][Inactive]` keys (its default where unset).
    pub inactive: AccentKeys<Rgba>,
    /// The `[Colors:X]` keys the file leaves unset.
    pub active_unset: AccentKeys<bool>,
    /// The `[Colors:X][Inactive]` keys the file leaves unset.
    pub inactive_unset: AccentKeys<bool>,
    /// Whether the file has the `[Colors:X]` group (it, or its `[Inactive]` subgroup, holds an
    /// entry).
    pub has_group: bool,
    /// Whether the file has the `[Colors:X][Inactive]` group (it holds an entry).
    pub has_inactive_group: bool,
}

/// One value per colour set.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AccentSets<T> {
    /// `[Colors:Window]`.
    pub window: T,
    /// `[Colors:View]`.
    pub view: T,
    /// `[Colors:Button]`.
    pub button: T,
    /// `[Colors:Selection]`.
    pub selection: T,
    /// `[Colors:Tooltip]`.
    pub tooltip: T,
    /// `[Colors:Complementary]`.
    pub complementary: T,
    /// `[Colors:Header]`.
    pub header: T,
}

impl<T> AccentSets<T> {
    /// The value of `set`.
    pub fn get(&self, set: AccentSet) -> &T {
        match set {
            AccentSet::Window => &self.window,
            AccentSet::View => &self.view,
            AccentSet::Button => &self.button,
            AccentSet::Selection => &self.selection,
            AccentSet::Tooltip => &self.tooltip,
            AccentSet::Complementary => &self.complementary,
            AccentSet::Header => &self.header,
        }
    }

    /// The value of `set`, to change.
    pub fn get_mut(&mut self, set: AccentSet) -> &mut T {
        match set {
            AccentSet::Window => &mut self.window,
            AccentSet::View => &mut self.view,
            AccentSet::Button => &mut self.button,
            AccentSet::Selection => &mut self.selection,
            AccentSet::Tooltip => &mut self.tooltip,
            AccentSet::Complementary => &mut self.complementary,
            AccentSet::Header => &mut self.header,
        }
    }
}

/// The `[WM]` group's start values: the file's keys, else KColorScheme's `Header` colours over
/// the file.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AccentWmColors {
    /// `activeBackground`.
    pub active_background: Rgba,
    /// `activeForeground`.
    pub active_foreground: Rgba,
    /// `inactiveBackground`.
    pub inactive_background: Rgba,
    /// `inactiveForeground`.
    pub inactive_foreground: Rgba,
}

impl AccentWmColors {
    /// The value of `key`.
    pub fn get(&self, key: AccentWmKey) -> Rgba {
        match key {
            AccentWmKey::ActiveBackground => self.active_background,
            AccentWmKey::ActiveForeground => self.active_foreground,
            AccentWmKey::InactiveBackground => self.inactive_background,
            AccentWmKey::InactiveForeground => self.inactive_foreground,
        }
    }
}

/// Everything `applyScheme()` reads: the scheme, the accent and the two options.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct KdeAccentInput {
    /// The scheme's sets.
    pub sets: AccentSets<AccentSetInput>,
    /// The `[WM]` start values.
    pub wm: AccentWmColors,
    /// The accent; one that stores as all zeros (`#00000000`) is no accent.
    pub accent: Rgba,
    /// The scheme's `TintFactor`; `None` where the file has none (no tint).
    pub tint_factor: Option<f32>,
    /// Whether the title bar takes the accent colour.
    pub titlebar_is_accent_colored: bool,
}

/// The title bar's tint factor where the scheme has no `TintFactor` (`0.15` as a single).
const DEFAULT_TINT_FACTOR: f32 = 0.15;
/// The alpha `accentBackground` gives the accent.
const ACCENT_BACKGROUND_ALPHA: f32 = 0.7;
/// The alpha of an inactive `accentForeground`.
const INACTIVE_FOREGROUND_ALPHA: f32 = 0.6;

/// KColorScheme's `Header` defaults, 8-bit opaque.
const HEADER_DEFAULTS: AccentKeys<[u8; 3]> = AccentKeys {
    background_normal: [0xde, 0xe0, 0xe2],
    background_alternate: [0xef, 0xf0, 0xf1],
    foreground_normal: [0x23, 0x26, 0x29],
    foreground_inactive: [0x70, 0x7d, 0x8a],
    foreground_active: [0x3d, 0xae, 0xe9],
    foreground_link: [0x29, 0x80, 0xb9],
    foreground_visited: [0x9b, 0x59, 0xb6],
    foreground_negative: [0xda, 0x44, 0x53],
    foreground_neutral: [0xf6, 0x74, 0x00],
    foreground_positive: [0x27, 0xae, 0x60],
    decoration_focus: [0x3d, 0xae, 0xe9],
    decoration_hover: [0x93, 0xce, 0xe9],
};

const WHITE: QtColor = QtColor::Standard([65535, 65535, 65535, 65535]);
const BLACK: QtColor = QtColor::Standard([0, 0, 0, 65535]);

/// The channels as `redF()` .. `alphaF()` read them, an invalid colour as opaque black.
fn read_opaque(c: QtColor) -> Rgba {
    match c {
        QtColor::Invalid => [0.0, 0.0, 0.0, 1.0],
        _ => read(c),
    }
}

/// `x` within `[0, 1]`; a NaN stays NaN, which `q16` stores as 0.
fn unit(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

/// A 16-bit channel to 8 bits, rounded.
fn div257(x: u16) -> u8 {
    let y = u32::from(x).saturating_add(128);
    u8::try_from(y.saturating_sub(y >> 8) >> 8).unwrap_or(u8::MAX)
}

/// The 16-bit channels of an extended colour converted to standard.
fn standard_of(h: [f32; 4]) -> [u16; 4] {
    let [r, g, b, a] = h;
    [q16(unit(r)), q16(unit(g)), q16(unit(b)), q16(a)]
}

/// `red()` .. `alpha()`: the 8-bit channels.
fn bytes(c: QtColor) -> [u8; 4] {
    match c {
        QtColor::Standard(s) => s.map(div257),
        QtColor::Extended(h) => standard_of(h).map(div257),
        QtColor::Invalid => [0, 0, 0, 255],
    }
}

/// A standard colour of 8-bit channels.
fn from_bytes(b: [u8; 4]) -> QtColor {
    QtColor::Standard(b.map(|v| u16::from(v).saturating_mul(257)))
}

/// KConfig's write and read back of a colour, in bytes.
fn written(c: QtColor) -> QtColor {
    from_bytes(bytes(c))
}

/// An extended colour converted to standard; others unchanged.
fn to_standard(c: QtColor) -> QtColor {
    match c {
        QtColor::Extended(h) => QtColor::Standard(standard_of(h)),
        other => other,
    }
}

/// `setAlphaF`.
fn with_alpha(c: QtColor, alpha: f32) -> QtColor {
    let alpha = unit(alpha);
    match c {
        QtColor::Standard([r, g, b, _]) => QtColor::Standard([r, g, b, q16(alpha)]),
        QtColor::Extended([r, g, b, _]) => QtColor::Extended([r, g, b, round_to_half(alpha)]),
        QtColor::Invalid => QtColor::Invalid,
    }
}

/// A double converted toward zero to an integer, where it is a byte.
fn trunc_byte(x: f64) -> Option<u8> {
    let t = x.trunc();
    if (0.0..=255.0).contains(&t) {
        Some(t as u8)
    } else {
        None
    }
}

/// Plasma's `alphaBlend(fg, bg)`.
fn blend(fg: QtColor, bg: QtColor) -> QtColor {
    let fa = read_opaque(fg)[3];
    if fa == 0.0 {
        return bg;
    }
    let ia = 1.0 - f64::from(fa);
    let ba = read_opaque(bg)[3];
    let (k, alpha) = if ba == 1.0 {
        (ia, Some(255))
    } else {
        let ib = f64::from(ba) * ia;
        let fin = f64::from(fa) + ib;
        (ib, trunc_byte(fin))
    };
    let [fr, fg_, fb, _] = bytes(fg);
    let [br, bg_, bb, _] = bytes(bg);
    let channel = |f: u8, b: u8| trunc_byte(f64::from(fa * f32::from(f)) + k * f64::from(b));
    match (channel(fr, br), channel(fg_, bg_), channel(fb, bb), alpha) {
        (Some(r), Some(g), Some(b), Some(a)) => from_bytes([r, g, b, a]),
        _ => QtColor::Invalid,
    }
}

/// Plasma's `accentBackground(accent, bg)`.
fn accent_background(accent: QtColor, bg: QtColor) -> QtColor {
    blend(with_alpha(accent, ACCENT_BACKGROUND_ALPHA), bg)
}

/// The contrast ratio of two lumas.
fn contrast_ratio(y1: f64, y2: f64) -> f64 {
    if y1 > y2 {
        (y1 + 0.05) / (y2 + 0.05)
    } else {
        (y2 + 0.05) / (y1 + 0.05)
    }
}

/// Plasma's `accentForeground(c, active)`.
fn accent_foreground(c: QtColor, active: bool) -> QtColor {
    let base = if contrast_ratio(luma(c), luma(WHITE)) < 4.5 {
        BLACK
    } else {
        WHITE
    };
    let alpha = if active {
        1.0
    } else {
        INACTIVE_FOREGROUND_ALPHA
    };
    blend(with_alpha(base, alpha), c)
}

/// `KColorUtils::lighten`.
fn lighten(x: QtColor, ky: f64, kc: f64) -> QtColor {
    let mut h = to_hcy(x);
    h.y = 1.0 - norm((1.0 - h.y) * (1.0 - ky));
    h.c = 1.0 - norm((1.0 - h.c) * kc);
    from_hcy(h)
}

/// `KColorUtils::darken`.
fn darken(x: QtColor, ky: f64, kc: f64) -> QtColor {
    let mut h = to_hcy(x);
    h.y = norm(h.y * (1.0 - ky));
    h.c = norm(h.c * kc);
    from_hcy(h)
}

/// The link colour Plasma writes: the accent, lightened or darkened where it reads poorly on
/// `bg`.
fn readable_link(accent: QtColor, bg: QtColor) -> QtColor {
    let (yb, ya) = (luma(bg), luma(accent));
    if contrast_ratio(yb, ya) < 5.0 {
        if ya > yb {
            lighten(accent, 0.5, 1.0)
        } else {
            darken(accent, 0.5, 1.0)
        }
    } else {
        accent
    }
}

/// A parametric transfer function, `(A, B, C, D, E, F, G)`.
struct Trc {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
}

impl Trc {
    fn eval(&self, t: f32) -> f32 {
        if t < self.d {
            self.c * t + self.f
        } else {
            let p = (self.a * t + self.b).powf(self.g);
            if p.is_finite() && p > -32768.0 && p < 32767.0 {
                p + self.e
            } else if p > 0.0 {
                1.0
            } else {
                0.0
            }
        }
    }

    /// The function extended to negative values by symmetry.
    fn apply(&self, v: f32) -> f32 {
        let r = self.eval(v.abs());
        if v.is_sign_negative() { -r } else { r }
    }
}

/// sRGB decoding.
const SRGB_DECODE: Trc = Trc {
    a: f32::from_bits(0x3f72_a76f),
    b: f32::from_bits(0x3d55_891a),
    c: f32::from_bits(0x3d9e_8391),
    d: f32::from_bits(0x3d25_aee6),
    e: 0.0,
    f: 0.0,
    g: f32::from_bits(0x4019_999a),
};

/// sRGB encoding.
const SRGB_ENCODE: Trc = Trc {
    a: f32::from_bits(0x3f91_8d1c),
    b: -0.0,
    c: f32::from_bits(0x414e_b852),
    d: f32::from_bits(0x3b4d_2e31),
    e: f32::from_bits(0xbd61_47ae),
    f: -0.0,
    g: f32::from_bits(0x3ed5_5555),
};

/// The linear transfer function.
const LINEAR: Trc = Trc {
    a: 1.0,
    b: 0.0,
    c: 1.0,
    d: 0.0,
    e: 0.0,
    f: 0.0,
    g: 1.0,
};

/// The sRGB to linear sRGB matrix (the same both ways), as Qt computes it, by rows.
const MATRIX: [[f32; 3]; 3] = [
    [
        f32::from_bits(0x3f7f_ffff),
        f32::from_bits(0xb276_387a),
        f32::from_bits(0xafec_5f80),
    ],
    [
        f32::from_bits(0x31a9_2059),
        f32::from_bits(0x3f7f_fffd),
        f32::from_bits(0x3113_9f94),
    ],
    [
        f32::from_bits(0x3195_f0e6),
        f32::from_bits(0x3299_bf1c),
        f32::from_bits(0x3f7f_ffff),
    ],
];

/// `QColorTransform::map` of a colour.
fn transform(x: QtColor, input: &Trc, output: &Trc) -> QtColor {
    let [r, g, b, alpha] = read_opaque(x);
    let (r, g, b) = (input.apply(r), input.apply(g), input.apply(b));
    let row = |[m0, m1, m2]: [f32; 3]| output.apply((r * m0 + g * m1) + b * m2);
    let [mx, my, mz] = MATRIX;
    from_rgbf(row(mx), row(my), row(mz), alpha)
}

/// A colour in OKLab.
fn oklab(c: QtColor) -> [f64; 3] {
    let [r, g, b, _] = read(c);
    let (r, g, b) = (f64::from(r), f64::from(g), f64::from(b));
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
    let (l, m, s) = (l.powf(1.0 / 3.0), m.powf(1.0 / 3.0), s.powf(1.0 / 3.0));
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}

/// An OKLab colour back to an opaque standard colour (invalid on a NaN).
fn from_oklab(lab: [f64; 3]) -> QtColor {
    let [ll, a, b] = lab;
    let l = ll + 0.3963377774 * a + 0.2158037573 * b;
    let m = ll - 0.1055613458 * a - 0.0638541728 * b;
    let s = ll - 0.0894841775 * a - 1.2914855480 * b;
    let (l, m, s) = ((l * l) * l, (m * m) * m, (s * s) * s);
    let red = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    let green = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    let blue = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;
    to_standard(from_rgbf(red as f32, green as f32, blue as f32, 1.0))
}

/// Plasma's `lerp`.
fn lerp(u: f64, v: f64, f: f32) -> f64 {
    let (u, v) = (u as f32, v as f32);
    f64::from((f64::from(u) * (1.0 - f64::from(f)) + f64::from(v * f)) as f32)
}

/// Plasma's `tintColor(base, with, factor)`.
fn tint(base: QtColor, with: QtColor, factor: f32) -> QtColor {
    let [l, a, b] = oklab(transform(base, &SRGB_DECODE, &LINEAR));
    let [_, wa, wb] = oklab(transform(with, &SRGB_DECODE, &LINEAR));
    let x = from_oklab([l, lerp(a, wa, factor), lerp(b, wb, factor)]);
    transform(x, &LINEAR, &SRGB_ENCODE)
}

/// A colour entry in the written kdeglobals.
#[derive(Clone, Copy)]
enum Entry {
    /// A computed colour, as KConfig reads it back.
    Stored(QtColor),
    /// A scheme input copied unchanged.
    Copied(Rgba),
}

impl Entry {
    fn stored(c: QtColor) -> Self {
        Entry::Stored(written(c))
    }

    fn colour(self) -> Rgba {
        match self {
            Entry::Stored(c) => read(c),
            Entry::Copied(c) => c,
        }
    }
}

/// What `applyScheme()` derives from its input.
struct Scheme<'a> {
    input: &'a KdeAccentInput,
    accent: QtColor,
    has_accent: bool,
    tinting: bool,
    factor: f32,
}

impl<'a> Scheme<'a> {
    fn new(input: &'a KdeAccentInput) -> Self {
        let accent = enter(input.accent);
        Scheme {
            input,
            accent,
            has_accent: accent != QtColor::Standard([0, 0, 0, 0]),
            tinting: input.tint_factor.is_some(),
            factor: input.tint_factor.unwrap_or(DEFAULT_TINT_FACTOR),
        }
    }

    fn set(&self, set: AccentSet) -> &AccentSetInput {
        self.input.sets.get(set)
    }

    fn is_set(&self, set: AccentSet, key: AccentKey) -> bool {
        let s = self.set(set);
        s.has_group && !s.active_unset.get(key)
    }

    fn is_set_inactive(&self, set: AccentSet, key: AccentKey) -> bool {
        let s = self.set(set);
        s.has_inactive_group && !s.inactive_unset.get(key)
    }

    /// The file's `[Colors:X]` key, invalid where unset.
    fn raw(&self, set: AccentSet, key: AccentKey) -> QtColor {
        if self.is_set(set, key) {
            enter(self.set(set).active.get(key))
        } else {
            QtColor::Invalid
        }
    }

    /// The file's `[Colors:X][Inactive]` key, invalid where unset.
    fn raw_inactive(&self, set: AccentSet, key: AccentKey) -> QtColor {
        if self.is_set_inactive(set, key) {
            enter(self.set(set).inactive.get(key))
        } else {
            QtColor::Invalid
        }
    }

    fn header_skipped(&self, set: AccentSet) -> bool {
        set == AccentSet::Header && !self.set(AccentSet::Header).has_group
    }

    fn accent_titlebar(&self, set: AccentSet, key: AccentKey) -> bool {
        set == AccentSet::Header
            && self.has_accent
            && self.input.titlebar_is_accent_colored
            && matches!(
                key,
                AccentKey::BackgroundNormal | AccentKey::ForegroundNormal
            )
    }

    /// The `[Colors:X]` entry written for `key`.
    fn out(&self, set: AccentSet, key: AccentKey) -> Option<Entry> {
        if self.header_skipped(set) {
            return None;
        }
        if self.accent_titlebar(set, key) {
            let aw = accent_background(
                self.accent,
                self.raw(AccentSet::Window, AccentKey::BackgroundNormal),
            );
            return Some(if key == AccentKey::BackgroundNormal {
                Entry::stored(aw)
            } else {
                Entry::stored(accent_foreground(aw, true))
            });
        }
        if set == AccentSet::Selection && self.has_accent {
            let ab = || {
                accent_background(
                    self.accent,
                    self.raw(AccentSet::View, AccentKey::BackgroundNormal),
                )
            };
            match key {
                AccentKey::BackgroundNormal | AccentKey::BackgroundAlternate => {
                    return Some(Entry::stored(ab()));
                }
                AccentKey::ForegroundNormal | AccentKey::ForegroundInactive => {
                    return Some(Entry::stored(accent_foreground(ab(), true)));
                }
                _ => {}
            }
        }
        if set == AccentSet::Button && self.has_accent && key == AccentKey::BackgroundAlternate {
            return Some(Entry::stored(accent_background(
                self.accent,
                self.raw(AccentSet::Button, AccentKey::BackgroundNormal),
            )));
        }
        if self.has_accent && key.is_accent_key() {
            return Some(if key == AccentKey::ForegroundLink {
                Entry::stored(readable_link(
                    self.accent,
                    self.raw(set, AccentKey::BackgroundNormal),
                ))
            } else {
                Entry::stored(self.accent)
            });
        }
        if self.has_accent && self.tinting {
            return Some(Entry::stored(tint(
                self.raw(set, key),
                self.accent,
                self.factor,
            )));
        }
        self.is_set(set, key)
            .then(|| Entry::Copied(self.set(set).active.get(key)))
    }

    /// The `[Colors:X][Inactive]` entry written for `key`.
    fn out_inactive(&self, set: AccentSet, key: AccentKey) -> Option<Entry> {
        if self.header_skipped(set) {
            return None;
        }
        if self.accent_titlebar(set, key) {
            let iw = tint(
                self.raw(AccentSet::Window, AccentKey::BackgroundNormal),
                self.accent,
                self.factor,
            );
            return Some(if key == AccentKey::BackgroundNormal {
                Entry::stored(iw)
            } else {
                Entry::stored(accent_foreground(iw, false))
            });
        }
        if !self.set(set).has_inactive_group {
            return None;
        }
        if self.tinting {
            Some(Entry::stored(tint(
                self.raw_inactive(set, key),
                self.accent,
                self.factor,
            )))
        } else if set == AccentSet::Header {
            self.is_set(AccentSet::Window, key)
                .then(|| Entry::Copied(self.set(AccentSet::Window).active.get(key)))
        } else {
            self.is_set_inactive(set, key)
                .then(|| Entry::Copied(self.set(set).inactive.get(key)))
        }
    }

    /// Whether the written kdeglobals has `[Colors:X][Inactive]`.
    fn inactive_exists(&self, set: AccentSet) -> bool {
        AccentKey::ALL
            .iter()
            .any(|key| self.out_inactive(set, *key).is_some())
    }

    /// What KColorScheme reads where the group it reads lacks `key`.
    fn default(&self, set: AccentSet, key: AccentKey, group: AccentGroup) -> Rgba {
        if set == AccentSet::Header {
            return match self.out(AccentSet::Window, key) {
                Some(entry) => entry.colour(),
                None => {
                    let [r, g, b] = HEADER_DEFAULTS.get(key);
                    read(from_bytes([r, g, b, 255]))
                }
            };
        }
        let s = self.set(set);
        match group {
            AccentGroup::Active => s.active.get(key),
            AccentGroup::Inactive => s.inactive.get(key),
        }
    }

    fn active(&self, set: AccentSet, key: AccentKey) -> Rgba {
        match self.out(set, key) {
            Some(entry) => entry.colour(),
            None => self.default(set, key, AccentGroup::Active),
        }
    }
}

/// The colour KColorScheme reads for `key` of `set` in `group` from the kdeglobals Plasma's
/// `applyScheme()` writes for `input` (before KColorScheme's inactive effects).
pub fn kde_accent(
    input: &KdeAccentInput,
    set: AccentSet,
    key: AccentKey,
    group: AccentGroup,
) -> Rgba {
    let scheme = Scheme::new(input);
    match group {
        AccentGroup::Active => scheme.active(set, key),
        AccentGroup::Inactive => {
            if scheme.inactive_exists(set) {
                match scheme.out_inactive(set, key) {
                    Some(entry) => entry.colour(),
                    None => scheme.default(set, key, AccentGroup::Inactive),
                }
            } else {
                scheme.active(set, key)
            }
        }
    }
}

/// The `[WM]` group's `key` as Plasma's `applyScheme()` writes it for `input`, read back by
/// KConfig.
pub fn kde_accent_wm(input: &KdeAccentInput, key: AccentWmKey) -> Rgba {
    let scheme = Scheme::new(input);
    let mut v = enter(input.wm.get(key));
    let titlebar = input.titlebar_is_accent_colored;
    if scheme.has_accent && (scheme.tinting || titlebar) {
        let wb = scheme.raw(AccentSet::Window, AccentKey::BackgroundNormal);
        if scheme.tinting {
            let t = tint(wb, scheme.accent, scheme.factor);
            match key {
                AccentWmKey::ActiveBackground if !titlebar => v = t,
                AccentWmKey::ActiveForeground if !titlebar => v = accent_foreground(t, true),
                AccentWmKey::InactiveBackground => v = t,
                AccentWmKey::InactiveForeground => v = accent_foreground(t, false),
                _ => {}
            }
        }
        if titlebar {
            let a = accent_background(scheme.accent, wb);
            match key {
                AccentWmKey::ActiveBackground => v = a,
                AccentWmKey::ActiveForeground => v = accent_foreground(a, true),
                _ => {}
            }
        }
    }
    read(written(v))
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::arithmetic_side_effects
    )]

    use super::*;

    fn colour(cell: &str) -> Result<Rgba, String> {
        if let Some(hex) = cell.strip_prefix('#') {
            let byte = |i: usize| -> Result<f32, String> {
                let digits = hex
                    .get(i..i + 2)
                    .ok_or_else(|| format!("short colour {cell}"))?;
                let v = u8::from_str_radix(digits, 16).map_err(|e| e.to_string())?;
                Ok((f64::from(v) / 255.0) as f32)
            };
            return match hex.len() {
                6 => Ok([byte(0)?, byte(2)?, byte(4)?, 1.0]),
                8 => Ok([byte(0)?, byte(2)?, byte(4)?, byte(6)?]),
                _ => Err(format!("bad colour {cell}")),
            };
        }
        let inner = cell
            .strip_prefix("srgb(")
            .and_then(|c| c.strip_suffix(')'))
            .ok_or_else(|| format!("bad colour {cell}"))?;
        let parts: Vec<f32> = inner
            .split(',')
            .map(|p| p.trim().parse::<f32>().map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;
        match parts.as_slice() {
            [r, g, b, a] => Ok([*r, *g, *b, *a]),
            _ => Err(format!("bad colour {cell}")),
        }
    }

    fn names(cell: &str) -> Result<Vec<String>, String> {
        let inner = cell
            .strip_prefix('"')
            .and_then(|c| c.strip_suffix('"'))
            .ok_or_else(|| format!("not a JSON string: {cell}"))?;
        if inner.contains('\\') {
            return Err(format!("escape in {cell}"));
        }
        Ok(if inner.is_empty() {
            Vec::new()
        } else {
            inner.split(',').map(str::to_owned).collect()
        })
    }

    fn set_of(name: &str) -> Result<AccentSet, String> {
        Ok(match name {
            "window" => AccentSet::Window,
            "view" => AccentSet::View,
            "button" => AccentSet::Button,
            "selection" => AccentSet::Selection,
            "tooltip" => AccentSet::Tooltip,
            "complementary" => AccentSet::Complementary,
            "header" => AccentSet::Header,
            _ => return Err(format!("bad set {name}")),
        })
    }

    fn key_of(name: &str) -> Result<AccentKey, String> {
        Ok(match name {
            "background_normal" => AccentKey::BackgroundNormal,
            "background_alternate" => AccentKey::BackgroundAlternate,
            "foreground_normal" => AccentKey::ForegroundNormal,
            "foreground_inactive" => AccentKey::ForegroundInactive,
            "foreground_active" => AccentKey::ForegroundActive,
            "foreground_link" => AccentKey::ForegroundLink,
            "foreground_visited" => AccentKey::ForegroundVisited,
            "foreground_negative" => AccentKey::ForegroundNegative,
            "foreground_neutral" => AccentKey::ForegroundNeutral,
            "foreground_positive" => AccentKey::ForegroundPositive,
            "decoration_focus" => AccentKey::DecorationFocus,
            "decoration_hover" => AccentKey::DecorationHover,
            _ => return Err(format!("bad key {name}")),
        })
    }

    fn wm_key_of(name: &str) -> Result<AccentWmKey, String> {
        Ok(match name {
            "active_background" => AccentWmKey::ActiveBackground,
            "active_foreground" => AccentWmKey::ActiveForeground,
            "inactive_background" => AccentWmKey::InactiveBackground,
            "inactive_foreground" => AccentWmKey::InactiveForeground,
            _ => return Err(format!("bad WM key {name}")),
        })
    }

    fn tint_factor(cell: &str) -> Result<Option<f32>, String> {
        if cell == "none" {
            Ok(None)
        } else {
            cell.parse::<f32>()
                .map(Some)
                .map_err(|_| format!("not a number: {cell}"))
        }
    }

    fn boolean(cell: &str) -> Result<bool, String> {
        match cell {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("not a bool: {cell}")),
        }
    }

    /// Marks the groups and the unset keys the row's lists name.
    fn lists(
        input: &mut KdeAccentInput,
        groups: &str,
        inactive_groups: &str,
        unset_keys: &str,
    ) -> Result<(), String> {
        for name in names(groups)? {
            input.sets.get_mut(set_of(&name)?).has_group = true;
        }
        for name in names(inactive_groups)? {
            input.sets.get_mut(set_of(&name)?).has_inactive_group = true;
        }
        for name in names(unset_keys)? {
            let mut parts = name.split('.');
            let set = set_of(parts.next().unwrap_or_default())?;
            let s = input.sets.get_mut(set);
            match (parts.next(), parts.next(), parts.next()) {
                (Some("inactive"), Some(key), None) => {
                    *s.inactive_unset.get_mut(key_of(key)?) = true
                }
                (Some(key), None, None) => *s.active_unset.get_mut(key_of(key)?) = true,
                _ => return Err(format!("bad unset key {name}")),
            }
        }
        Ok(())
    }

    fn close(got: Rgba, want: Rgba) -> bool {
        got.iter()
            .zip(want.iter())
            .all(|(g, w)| (f64::from(*g) * 255.0 - f64::from(*w) * 255.0).abs() <= 1e-3)
    }

    fn accent_row(cells: &[&str]) -> Result<(Rgba, Rgba), String> {
        let [
            set,
            key,
            group,
            accent,
            value,
            inactive_value,
            background,
            window_value,
            window_background,
            view_background,
            groups,
            inactive_groups,
            unset_keys,
            factor,
            titlebar,
            result,
        ] = cells
        else {
            return Err(format!("{} cells", cells.len()));
        };
        let set = set_of(set)?;
        let key = key_of(key)?;
        let group = match *group {
            "active" => AccentGroup::Active,
            "inactive" => AccentGroup::Inactive,
            other => return Err(format!("bad group {other}")),
        };
        let mut input = KdeAccentInput {
            accent: colour(accent)?,
            tint_factor: tint_factor(factor)?,
            titlebar_is_accent_colored: boolean(titlebar)?,
            ..KdeAccentInput::default()
        };
        input.sets.view.active.background_normal = colour(view_background)?;
        input.sets.window.active.background_normal = colour(window_background)?;
        *input.sets.window.active.get_mut(key) = colour(window_value)?;
        let s = input.sets.get_mut(set);
        s.active.background_normal = colour(background)?;
        *s.active.get_mut(key) = colour(value)?;
        *s.inactive.get_mut(key) = colour(inactive_value)?;
        lists(&mut input, groups, inactive_groups, unset_keys)?;
        Ok((kde_accent(&input, set, key, group), colour(result)?))
    }

    fn wm_row(cells: &[&str]) -> Result<(Rgba, Rgba), String> {
        let [
            key,
            accent,
            value,
            window_background,
            groups,
            unset_keys,
            factor,
            titlebar,
            result,
        ] = cells
        else {
            return Err(format!("{} cells", cells.len()));
        };
        let key = wm_key_of(key)?;
        let mut input = KdeAccentInput {
            accent: colour(accent)?,
            tint_factor: tint_factor(factor)?,
            titlebar_is_accent_colored: boolean(titlebar)?,
            ..KdeAccentInput::default()
        };
        input.wm = AccentWmColors {
            active_background: colour(value)?,
            active_foreground: colour(value)?,
            inactive_background: colour(value)?,
            inactive_foreground: colour(value)?,
        };
        input.sets.window.active.background_normal = colour(window_background)?;
        lists(&mut input, groups, "\"\"", unset_keys)?;
        Ok((kde_accent_wm(&input, key), colour(result)?))
    }

    fn run(
        table: &str,
        header: &str,
        row: impl Fn(&[&str]) -> Result<(Rgba, Rgba), String>,
    ) -> usize {
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(lines.next(), Some(header));
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            match row(&cells) {
                Ok((got, want)) => {
                    if !close(got, want) {
                        failures.push(format!("{line}\n  got  {got:?}\n  want {want:?}"));
                    }
                }
                Err(e) => failures.push(format!("{line}\n  unreadable row: {e}")),
            }
        }
        assert!(
            failures.is_empty(),
            "{} of {rows} rows fail:\n{}",
            failures.len(),
            failures.join("\n")
        );
        rows
    }

    #[test]
    fn vectors_kde_accent() {
        let rows = run(
            include_str!("../../tests/vectors/kde_accent.tsv"),
            "set\tkey\tgroup\taccent\tvalue\tinactive_value\tbackground\twindow_value\t\
             window_background\tview_background\tgroups\tinactive_groups\tunset_keys\t\
             tint_factor\ttitlebar_is_accent_colored\tresult",
            accent_row,
        );
        assert_eq!(rows, 3259);
        println!("kde_accent: {rows} of {rows} rows pass");
    }

    #[test]
    fn vectors_kde_accent_wm() {
        let rows = run(
            include_str!("../../tests/vectors/kde_accent_wm.tsv"),
            "key\taccent\tvalue\twindow_background\tgroups\tunset_keys\ttint_factor\t\
             titlebar_is_accent_colored\tresult",
            wm_row,
        );
        assert_eq!(rows, 211);
        println!("kde_accent_wm: {rows} of {rows} rows pass");
    }
}
