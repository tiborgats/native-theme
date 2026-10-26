//! GNOME portal reader: reads accent color, color scheme, contrast,
//! fonts, text scale, accessibility flags, and icon_set from the
//! XDG Desktop Portal Settings interface (via ashpd) and gsettings.
//!
//! Returns only what the desktop states, a sparse variant the pipeline
//! merges over the Adwaita preset.
//!
//! # Sources
//!
//! Each a `namespace key` pair: a portal setting of `org.freedesktop.appearance`,
//! or a gsettings key, which the portal's GNOME and GTK backends forward under
//! its schema's name. The watcher observes each through the portal's
//! `SettingChanged` signal.
//!
//! - `org.freedesktop.appearance color-scheme`: light or dark.
//! - `org.freedesktop.appearance accent-color`: the accent colour.
//! - `org.freedesktop.appearance contrast`: high contrast.
//! - `org.freedesktop.appearance reduced-motion`: reduced motion.
//! - `org.gnome.desktop.interface color-scheme`: light or dark, where the
//!   portal gives none.
//! - `org.gnome.desktop.interface enable-animations`: reduced motion, where
//!   the portal gives none.
//! - `org.gnome.desktop.interface font-name`: the UI font, and from its size
//!   the type scale libadwaita sizes as a share of it (`text_scale.caption`,
//!   `.dialog_title`, `.display` and `dialog.title_font`).
//! - `org.gnome.desktop.interface monospace-font-name`: the monospace font.
//! - `org.gnome.desktop.interface text-scaling-factor`: the text scale.
//! - `org.gnome.desktop.interface overlay-scrolling`: overlay scroll bars.
//! - `org.gnome.desktop.interface icon-theme`: the icon theme.
//! - `org.gnome.desktop.wm.preferences titlebar-font`: the title bar font.
//! - `org.gnome.desktop.a11y.interface high-contrast`: high contrast, where
//!   the portal reports none.
//!
//! # Sources not watched
//!
//! - `$XDG_SESSION_TYPE`: the session type, which decides whether the
//!   text-scaling factor is reported (`GnomeReader`), fixed for a session.

mod accent;

use ashpd::desktop::Color;
use ashpd::desktop::settings::{ColorScheme, Contrast, ReducedMotion};

use crate::model::FontSpec;
use crate::model::font::{FontSize, FontStyle};

/// Pango's weight words and their `PangoWeight` (`weight_map`, pango 1.58.2
/// `pango/fonts.c:1311-1333`; the values `pango/pango-font.h:131-142`).
const PANGO_WEIGHTS: &[(&str, i32)] = &[
    ("Thin", 100),
    ("Ultra-Light", 200),
    ("Extra-Light", 200),
    ("Light", 300),
    ("Semi-Light", 350),
    ("Demi-Light", 350),
    ("Book", 380),
    ("Regular", 400),
    ("Medium", 500),
    ("Semi-Bold", 600),
    ("Demi-Bold", 600),
    ("Bold", 700),
    ("Ultra-Bold", 800),
    ("Extra-Bold", 800),
    ("Heavy", 900),
    ("Black", 900),
    ("Ultra-Heavy", 1000),
    ("Extra-Heavy", 1000),
    ("Ultra-Black", 1000),
    ("Extra-Black", 1000),
];

/// `PANGO_WEIGHT_NORMAL`, the weight of a description without a weight word
/// (`pango/pango-font.h:136`; `pfd_defaults`, `pango/fonts.c:63-82`).
const PANGO_WEIGHT_NORMAL: i32 = 400;

/// Pango's style words (`style_map`, `pango/fonts.c:1294-1299`).
const PANGO_STYLES: &[(&str, FontStyle)] = &[
    ("Roman", FontStyle::Normal),
    ("Oblique", FontStyle::Oblique),
    ("Italic", FontStyle::Italic),
];

/// Pango's stretch, variant, gravity and colour words (`width_map`,
/// `variant_map`, `gravity_map`, `color_map`, `pango/fonts.c:1301-1309`,
/// `:1335-1361`), each with the name of its `{field}=` prefix. The model's
/// font has no field for any of them, so they are read only to know where the
/// family ends, and dropped.
const PANGO_DROPPED_FIELDS: &[(&str, &[(&str, ())])] = &[
    (
        "width",
        &[
            ("Ultra-Condensed", ()),
            ("Extra-Condensed", ()),
            ("Condensed", ()),
            ("Semi-Condensed", ()),
            ("Semi-Expanded", ()),
            ("Expanded", ()),
            ("Extra-Expanded", ()),
            ("Ultra-Expanded", ()),
        ],
    ),
    (
        "variant",
        &[
            ("Small-Caps", ()),
            ("All-Small-Caps", ()),
            ("Petite-Caps", ()),
            ("All-Petite-Caps", ()),
            ("Unicase", ()),
            ("Title-Caps", ()),
        ],
    ),
    (
        "gravity",
        &[
            ("Not-Rotated", ()),
            ("South", ()),
            ("Upside-Down", ()),
            ("North", ()),
            ("Rotated-Left", ()),
            ("East", ()),
            ("Rotated-Right", ()),
            ("West", ()),
        ],
    ),
    ("color", &[("With-Color", ()), ("Without-Color", ())]),
];

/// The largest size Pango reads (`parse_size`, `pango/fonts.c:1495-1519`).
const PANGO_MAX_SIZE: f64 = 1_000_000.0;

/// Convert an ashpd portal Color to an Rgba, returning None if any
/// component is outside the [0.0, 1.0] range (per XDG spec: out-of-range
/// means "unset"). Used by the KDE-with-portal reader.
#[cfg(any(feature = "kde", test))]
pub(crate) fn portal_color_to_rgba(color: &Color) -> Option<crate::Rgba> {
    let r = color.red();
    let g = color.green();
    let b = color.blue();

    // Per XDG spec: out-of-range means "accent color not set"
    if !(0.0..=1.0).contains(&r) || !(0.0..=1.0).contains(&g) || !(0.0..=1.0).contains(&b) {
        return None;
    }

    Some(crate::Rgba::from_f32(r as f32, g as f32, b as f32, 1.0))
}

/// Apply a portal accent color across multiple semantic color roles.
fn apply_accent(variant: &mut crate::ThemeMode, accent: &crate::Rgba) {
    variant.defaults.accent_color = Some(*accent);
    variant.defaults.selection_background = Some(*accent);
    variant.defaults.focus_ring_color = Some(*accent);
    // A checked notebook tab's line is `--accent-bg-color` (docs/platform-facts.md §2.11).
    variant.tab.active_indicator_color = Some(*accent);
}

/// [`apply_accent`] as libadwaita draws the accent: its focus ring, and an
/// entry's focus edge, are `$focus_border_color`, the accent's standalone
/// colour at 50 % on a light or a dark background (`--accent-color`,
/// libadwaita 1.10.0 `doc/css-variables.md:137-140`, derived by
/// `adw_rgba_to_standalone`, `src/adw-accent-color.c:130-157`;
/// docs/platform-facts.md §2.1.5 `focus_ring_color`, §2.4
/// `focus_border_color`), not the accent background itself.
///
/// Under high contrast the ring is `$focus_border_color_hc`, the standalone
/// colour at 80 % (`_drawing.scss:24-26`; `_colors.scss:261`, `:308`).
fn apply_gnome_accent(
    variant: &mut crate::ThemeMode,
    accent: &crate::Rgba,
    dark: bool,
    high_contrast: bool,
) {
    apply_accent(variant, accent);
    // GtkInfoBar's info type is `--accent-bg-color` (libadwaita 1.10.0
    // `_deprecated.scss:174`; docs/platform-facts.md §2.1.4). Not in
    // `apply_accent`, which a KDE read shares, where the info colour is
    // `[Colors:View] ForegroundActive`.
    variant.defaults.info_color = Some(*accent);
    let ring = accent::focus_ring(*accent, dark, high_contrast);
    variant.defaults.focus_ring_color = Some(ring);
    variant.input.focus_border_color = Some(ring);
}

/// Parse a GNOME font string (gsettings `font-name`, `monospace-font-name`,
/// `titlebar-font`) as Pango's `pango_font_description_from_string` does
/// (pango 1.58.2 `pango/fonts.c:1613-1729`): `[FAMILY-LIST] [STYLE-OPTIONS]
/// [SIZE] [VARIATIONS] [FEATURES]`, such as `'Cantarell Bold Italic 11'`,
/// `'Cantarell 15px'`, `'Cantarell 11 @wght=200'`.
///
/// 1. A trailing `#features` and `@variations` word, in either order, are
///    skipped: the model's font holds neither.
/// 2. The size ([`pango_size`]): points, or pixels with a `px` suffix.
/// 3. Style-option words, read right to left until one is not Pango's
///    ([`PangoStyleOptions::apply`]): weight and style as the model holds
///    them; stretch, variant, gravity and colour dropped.
/// 4. The rest is the family list. Pango keeps the whole comma-separated list
///    as the family (`"Cantarell,Sans"`) for fontconfig to try in turn; the
///    model's family is one name, so it is the list's first non-empty one.
///
/// Weight and style default to Pango's normal ones, as Pango sets both on
/// every description. Returns `None` for a string without a family or without
/// a positive, finite size: Pango reads `'Cantarell NaN'` as the family
/// `Cantarell NaN` with no size, which names no font, so that font keeps the
/// preset's.
pub(crate) fn parse_gnome_font_to_fontspec(s: &str) -> Option<FontSpec> {
    let mut rest = s.trim().trim_matches('\'');

    // The last word runs back to whitespace, the one before it to a comma too.
    let (before, word) = pango_last_word(rest, "");
    let tail = ['#', '@'].into_iter().find(|mark| word.starts_with(*mark));
    if tail.is_some() {
        rest = before;
    }
    let (before, word) = pango_last_word(rest, ",");
    if ['#', '@']
        .into_iter()
        .any(|mark| Some(mark) != tail && word.starts_with(mark))
    {
        rest = before;
    }

    let (before, word) = pango_last_word(rest, ",");
    let size = pango_size(word)?;
    rest = before;

    let mut options = PangoStyleOptions {
        weight: PANGO_WEIGHT_NORMAL,
        style: Some(FontStyle::Normal),
    };
    loop {
        let (before, word) = pango_last_word(rest, ",");
        if word.is_empty() || !options.apply(word) {
            break;
        }
        rest = before;
    }

    let family = rest
        .split(',')
        .map(|family| family.trim_matches(|c: char| c.is_ascii_whitespace()))
        .find(|family| !family.is_empty())?;

    Some(FontSpec {
        family: Some(family.into()),
        size: Some(size),
        // PangoWeight runs to 1000; `check_font` accepts CSS 100..=900
        // (`resolve/validate_helpers.rs:492`).
        weight: u16::try_from(options.weight.clamp(100, 900)).ok(),
        style: options.style,
        ..Default::default()
    })
}

/// Pango's `getword` (`pango/fonts.c:1478-1493`): after trailing whitespace,
/// the last word of `s`, running back to whitespace or to a character of
/// `stop`. Returns the text before the word, and the word. Whitespace is
/// `g_ascii_isspace`'s (glib 2.88.3 `glib/gstrfuncs.c:262-267`), the set of
/// [`char::is_ascii_whitespace`].
fn pango_last_word<'a>(s: &'a str, stop: &str) -> (&'a str, &'a str) {
    let s = s.trim_end_matches(|c: char| c.is_ascii_whitespace());
    let word = s
        .rsplit(|c: char| c.is_ascii_whitespace() || stop.contains(c))
        .next()
        .unwrap_or_default();
    (s.strip_suffix(word).unwrap_or_default(), word)
}

/// Pango's `parse_size` (`pango/fonts.c:1495-1519`): a number of points, or
/// of pixels when `px` follows it, up to [`PANGO_MAX_SIZE`]. A size of 0, which
/// Pango reads as no size, and a NaN or infinite one, which is not a size to
/// Pango, give `None`.
fn pango_size(word: &str) -> Option<FontSize> {
    let (number, pixels) = match word.strip_suffix("px") {
        Some(number) => (number, true),
        None => (word, false),
    };
    let size = number
        .parse::<f64>()
        .ok()
        .filter(|size| *size > 0.0 && *size <= PANGO_MAX_SIZE)? as f32;
    Some(if pixels {
        FontSize::Px(size)
    } else {
        FontSize::Pt(size)
    })
}

/// A Pango style-option word's value: a word of the field's table, or the
/// integer after a `{field}=` prefix.
enum PangoField<T> {
    Word(T),
    Int(i32),
}

/// Pango's `find_field` (`pango/fonts.c:1411-1447`): `word` as one of `map`'s
/// words, or, after a `{what}=` prefix, as one of them or a non-negative
/// integer (`parse_int`, `:1391-1409`).
fn pango_find_field<T: Copy>(what: &str, map: &[(&str, T)], word: &str) -> Option<PangoField<T>> {
    let (word, had_prefix) = match word.strip_prefix(what).and_then(|w| w.strip_prefix('=')) {
        Some(value) => (value, true),
        None => (word, false),
    };
    if let Some(&(_, value)) = map.iter().find(|(name, _)| pango_word_matches(name, word)) {
        return Some(PangoField::Word(value));
    }
    if had_prefix {
        return word
            .parse::<i32>()
            .ok()
            .filter(|n| *n >= 0)
            .map(PangoField::Int);
    }
    None
}

/// Pango's `field_matches` (`pango/fonts.c:1363-1389`): `word` is `name`
/// ignoring ASCII case, any of `name`'s hyphens optional (`SemiBold` is
/// `Semi-Bold`).
fn pango_word_matches(name: &str, word: &str) -> bool {
    let mut name = name.bytes();
    for w in word.bytes() {
        loop {
            match name.next() {
                Some(n) if n.eq_ignore_ascii_case(&w) => break,
                Some(b'-') => {}
                _ => return false,
            }
        }
    }
    name.next().is_none()
}

/// What Pango's style-option words set, as the model's font holds it.
struct PangoStyleOptions {
    /// A `PangoWeight`.
    weight: i32,
    /// `None` for a `style=` integer that is no `PangoStyle`.
    style: Option<FontStyle>,
}

impl PangoStyleOptions {
    /// Read one word as Pango's `find_field_any` does (`pango/fonts.c:1449-1476`):
    /// `Normal`, then the weight, style, stretch, variant, gravity and colour
    /// fields in turn. Returns `false` when the word is none of them.
    fn apply(&mut self, word: &str) -> bool {
        if pango_word_matches("Normal", word) {
            return true;
        }
        if let Some(weight) = pango_find_field("weight", PANGO_WEIGHTS, word) {
            self.weight = match weight {
                PangoField::Word(weight) | PangoField::Int(weight) => weight,
            };
            return true;
        }
        if let Some(style) = pango_find_field("style", PANGO_STYLES, word) {
            self.style = match style {
                PangoField::Word(style) => Some(style),
                // `PangoStyle`'s values (`pango/pango-font.h:74-78`).
                PangoField::Int(0) => Some(FontStyle::Normal),
                PangoField::Int(1) => Some(FontStyle::Oblique),
                PangoField::Int(2) => Some(FontStyle::Italic),
                PangoField::Int(_) => None,
            };
            return true;
        }
        PANGO_DROPPED_FIELDS
            .iter()
            .any(|(what, map)| pango_find_field(what, map, word).is_some())
    }
}

/// One deadline for all the gsettings reads of one GNOME read together, so
/// that an unresponsive D-Bus costs it this once rather than once per key.
/// Ruled equal to the bound [`crate::detect`] gives one subprocess.
const GSETTINGS_DEADLINE: std::time::Duration = crate::detect::SUBPROCESS_TIMEOUT;

/// Read a single gsettings value within `timeout`, returning None if
/// gsettings is unavailable, fails or outlives it, or `timeout` is zero.
///
/// Single-quote wrapping (gsettings string output format) is stripped from
/// the result.
fn read_gsetting(schema: &str, key: &str, timeout: std::time::Duration) -> Option<String> {
    if timeout.is_zero() {
        return None;
    }
    let raw = crate::detect::gsettings_get(schema, key, timeout)?;
    let trimmed = raw.trim_matches('\'').to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Portal + gsettings data using primitive types only.
///
/// Constructed from ashpd portal reads + gsettings in production,
/// or directly in tests for deterministic verification.
/// No ashpd types, no D-Bus access, no gsettings subprocess calls.
pub(crate) struct GnomePortalData {
    /// Whether the user prefers a dark color scheme.
    pub(crate) is_dark: bool,
    /// Accent color as (R, G, B) with components in 0.0..=1.0 (XDG portal spec).
    /// None means no accent color set.
    pub(crate) accent_rgb: Option<(f64, f64, f64)>,
    /// Whether high contrast is requested (from portal Contrast::High).
    pub(crate) high_contrast: bool,
    /// Whether reduced motion is requested: the portal's answer, else
    /// gsettings' ([`crate::detect::gnome_reduce_motion`]). None = neither
    /// gave one.
    pub(crate) reduce_motion: Option<bool>,
    /// Primary UI font string in GNOME/Pango format (e.g. "Cantarell 11").
    pub(crate) font_name: Option<String>,
    /// Monospace font string (e.g. "Source Code Pro 10").
    pub(crate) monospace_font_name: Option<String>,
    /// Window titlebar font string.
    pub(crate) titlebar_font: Option<String>,
    /// Text scaling factor (e.g. 1.0, 1.25).
    pub(crate) text_scaling_factor: Option<f32>,
    /// The session type, `$XDG_SESSION_TYPE` (`x11`, `wayland`, …), as
    /// [`crate::detect::session_type`] reads it. None = not set.
    pub(crate) session_type: Option<String>,
    /// Whether overlay scrollbars are enabled. None = not provided.
    pub(crate) overlay_scrolling: Option<bool>,
    /// Icon theme name (e.g. "Adwaita", "Papirus").
    pub(crate) icon_theme: Option<String>,
    /// High contrast from gsettings fallback (for GNOME < 44).
    /// Only used when `high_contrast` is false (portal didn't report high contrast).
    pub(crate) gsettings_high_contrast: Option<bool>,
}

/// Whether the user asked for high contrast: the portal's `contrast`, or
/// gsettings' `high-contrast` where the portal gives none (GNOME < 44).
fn effective_high_contrast(data: &GnomePortalData) -> bool {
    data.high_contrast || data.gsettings_high_contrast == Some(true)
}

/// Extract AccessibilityPreferences from GNOME portal + gsettings data.
///
/// Merges portal and gsettings sources: portal `high_contrast` takes
/// priority, gsettings is the fallback; `reduce_motion` is already merged.
/// GNOME does not expose `reduce_transparency`, so it keeps the default (false).
fn accessibility_from_gnome_data(data: &GnomePortalData) -> crate::AccessibilityPreferences {
    let mut acc = crate::AccessibilityPreferences::default();
    let effective_high_contrast = effective_high_contrast(data);
    if let Some(tsf) = data.text_scaling_factor {
        acc.text_scaling_factor = tsf;
    }
    if let Some(rm) = data.reduce_motion {
        acc.reduce_motion = rm;
    }
    acc.high_contrast = effective_high_contrast;
    // reduce_transparency keeps default -- GNOME does not expose this setting
    acc
}

/// libadwaita's `.caption`: 82% of the base font (`docs/platform-facts.md:748`),
/// the preset's `text_scale.caption` (`:1485`).
const CAPTION_FACTOR: f32 = 0.82;

/// libadwaita's `.title-2`: 136% of the base font (`docs/platform-facts.md:754`),
/// the preset's `text_scale.dialog_title` and `dialog.title_font` (`:1487`,
/// `:1561`).
const TITLE_2_FACTOR: f32 = 1.36;

/// libadwaita's `.title-1`: 181% of the base font (`docs/platform-facts.md:755`),
/// the preset's `text_scale.display` (`:1488`).
const TITLE_1_FACTOR: f32 = 1.81;

/// `body` times `factor`, in `body`'s unit.
fn scaled_size(body: FontSize, factor: f32) -> FontSize {
    match body {
        FontSize::Pt(v) => FontSize::Pt(v * factor),
        FontSize::Px(v) => FontSize::Px(v * factor),
    }
}

/// The sizes libadwaita gives as a share of the base font, from the size of
/// the UI font `body` (`docs/platform-facts.md:746-755`): `text_scale.caption`
/// (`.caption`), `text_scale.dialog_title` and `dialog.title_font`
/// (`.title-2`), and `text_scale.display` (`.title-1`). Sizes only: the
/// weights, and `text_scale.section_heading`, whose `.heading` inherits the
/// base size, stay the preset's. The font size gsettings states carries no
/// text-scaling factor, which the reader reports apart, so the factor reaches
/// these sizes once, where the connectors apply it.
fn apply_type_scale(variant: &mut crate::ThemeMode, body: FontSize) {
    let entry = |factor| crate::TextScaleEntry {
        size: Some(scaled_size(body, factor)),
        ..Default::default()
    };
    variant.text_scale.caption = Some(entry(CAPTION_FACTOR));
    variant.text_scale.dialog_title = Some(entry(TITLE_2_FACTOR));
    variant.text_scale.display = Some(entry(TITLE_1_FACTOR));
    variant.dialog.title_font = Some(FontSpec {
        size: Some(scaled_size(body, TITLE_2_FACTOR)),
        ..Default::default()
    });
}

/// Build a sparse ThemeMode from pre-read portal + gsettings data.
/// Zero I/O -- all values are pre-read by the caller.
fn build_gnome_variant_pure(data: &GnomePortalData) -> crate::ThemeMode {
    let mut variant = crate::ThemeMode::default();

    // Apply accent color with range validation
    if let Some((r, g, b)) = data.accent_rgb
        && (0.0..=1.0).contains(&r)
        && (0.0..=1.0).contains(&g)
        && (0.0..=1.0).contains(&b)
    {
        let rgba = crate::Rgba::from_f32(r as f32, g as f32, b as f32, 1.0);
        apply_gnome_accent(
            &mut variant,
            &rgba,
            data.is_dark,
            effective_high_contrast(data),
        );
    }

    // NOTE: high_contrast, reduce_motion and text_scaling_factor
    // are no longer on ThemeDefaults. They live on AccessibilityPreferences,
    // constructed by the pipeline from GnomePortalData fields.

    // ── Fonts (GNOME-01) ────────────────────────────────────────────────
    // Primary UI font
    if let Some(ref font_str) = data.font_name
        && let Some(fs) = parse_gnome_font_to_fontspec(font_str)
    {
        if let Some(body) = fs.size {
            apply_type_scale(&mut variant, body);
        }
        variant.defaults.font = fs;
    }

    // Monospace font
    if let Some(ref mono_str) = data.monospace_font_name
        && let Some(fs) = parse_gnome_font_to_fontspec(mono_str)
    {
        variant.defaults.mono_font = fs;
    }

    // Titlebar font (GNOME-01 extension)
    if let Some(ref tb_str) = data.titlebar_font
        && let Some(fs) = parse_gnome_font_to_fontspec(tb_str)
    {
        variant.window.title_bar_font = Some(fs);
    }

    // ── Accessibility (GNOME-03 / GNOME-05) ─────────────────────────────
    // text_scaling_factor and reduce_motion are extracted from
    // GnomePortalData by the pipeline caller, not set on ThemeDefaults.

    // overlay-scrolling -> scrollbar.overlay_mode (GNOME-03)
    if let Some(overlay) = data.overlay_scrolling {
        variant.scrollbar.overlay_mode = Some(overlay);
    }

    // ── Icon theme (GNOME-04) ────────────────────────────────────────────
    variant.defaults.icon_theme = data.icon_theme.clone().map(std::borrow::Cow::Owned);

    variant
}

/// Whether on a session of `session_type` the toolkit's own scale factor
/// carries GNOME's text-scaling factor: an X11 session only.
/// gnome-settings-daemon 51.0 writes the XSETTINGS `Xft/DPI` and the
/// `Xft.dpi` resource as 96 × `text-scaling-factor` × the window scale
/// (`plugins/xsettings/gsd-xsettings-manager.c:666-668`, `:742`,
/// `:797-798`), and on X11 winit 0.30.13 takes its scale factor from that
/// DPI over 96, XSETTINGS first (`src/platform_impl/linux/x11/util/randr.rs:41-53`,
/// `:145-146`), as gpui-pre-linux 0.3.9 does from `Xft.dpi`
/// (`src/linux/x11/client.rs:2671-2678`). Each does so only while its
/// override is unset: winit's `WINIT_X11_SCALE_FACTOR` (`randr.rs:112-154`)
/// and gpui's `GPUI_X11_SCALE_FACTOR` (`client.rs:2620-2666`) set the scale
/// factor instead, and then the application owns the scale and the
/// text-scaling factor is applied nowhere. Neither crate's Wayland backend
/// reads `Xft.dpi` or XSETTINGS (only their `x11` modules do), so a native
/// Wayland client's scale factor does not carry the text-scaling factor.
fn toolkit_scale_carries_text_scale(session_type: Option<&str>) -> bool {
    session_type == Some("x11")
}

/// Build a GNOME ReaderResult from pre-read portal + gsettings data: the
/// sparse OS variant, which the pipeline merges over the Adwaita preset.
/// On an X11 session the text-scaling factor is reported as none
/// ([`toolkit_scale_carries_text_scale`]), since the toolkit applied it.
/// Zero I/O.
fn build_gnome_spec_pure(data: &GnomePortalData) -> crate::ReaderResult {
    let mut accessibility = accessibility_from_gnome_data(data);
    if toolkit_scale_carries_text_scale(data.session_type.as_deref()) {
        accessibility.text_scaling_factor =
            crate::AccessibilityPreferences::DEFAULT.text_scaling_factor;
    }
    crate::ReaderResult {
        output: crate::ReaderOutput::Single {
            mode: Box::new(build_gnome_variant_pure(data)),
            is_dark: data.is_dark,
        },
        name: "GNOME".into(),
        icon_set: None,
        layout: crate::LayoutTheme::default(),
        font_dpi: Some(crate::detect::GNOME_FONT_DPI),
        accessibility,
    }
}

/// Build a ReaderResult from what the portal answered (`None` for a setting
/// it did not give), the session type ([`crate::detect::session_type`]) and
/// the gsettings values `gsetting` reads, one read per key.
pub(crate) fn build_theme(
    scheme: Option<ColorScheme>,
    accent: Option<Color>,
    contrast: Contrast,
    reduced_motion: Option<ReducedMotion>,
    session_type: Option<String>,
    mut gsetting: impl FnMut(&str, &str) -> Option<String>,
) -> crate::ReaderResult {
    let portal_dark = scheme.map(|scheme| matches!(scheme, ColorScheme::PreferDark));
    let is_dark = crate::detect::color_scheme_is_dark(portal_dark, &mut gsetting).unwrap_or(false);
    let reduce_motion = crate::detect::gnome_reduce_motion(
        reduced_motion.map(portal_reduces_motion),
        &mut gsetting,
    );
    let flag = |value: Option<String>| value.and_then(|s| s.parse::<bool>().ok());
    let data = GnomePortalData {
        is_dark,
        accent_rgb: accent.map(|c| (c.red(), c.green(), c.blue())),
        high_contrast: matches!(contrast, Contrast::High),
        reduce_motion,
        font_name: gsetting("org.gnome.desktop.interface", "font-name"),
        monospace_font_name: gsetting("org.gnome.desktop.interface", "monospace-font-name"),
        titlebar_font: gsetting("org.gnome.desktop.wm.preferences", "titlebar-font"),
        text_scaling_factor: gsetting("org.gnome.desktop.interface", "text-scaling-factor")
            .and_then(|s| s.parse::<f32>().ok()),
        session_type,
        overlay_scrolling: flag(gsetting("org.gnome.desktop.interface", "overlay-scrolling")),
        icon_theme: gsetting("org.gnome.desktop.interface", "icon-theme"),
        gsettings_high_contrast: flag(gsetting(
            "org.gnome.desktop.a11y.interface",
            "high-contrast",
        )),
    };
    build_gnome_spec_pure(&data)
}

/// Whether the portal's `reduced-motion` setting asks for reduced motion.
fn portal_reduces_motion(reduced_motion: ReducedMotion) -> bool {
    matches!(reduced_motion, ReducedMotion::ReducedMotion)
}

/// The portal's `reduced-motion` setting, as the GNOME reader reads it, for
/// [`crate::detect::detect_reduced_motion`]; `None` when the portal does not
/// give it within [`PORTAL_DEADLINE`].
pub(crate) async fn portal_reduce_motion() -> Option<bool> {
    with_deadline(
        async {
            let settings = ashpd::desktop::settings::Settings::new().await.ok()?;
            settings.reduced_motion().await.ok()
        },
        PORTAL_DEADLINE,
    )
    .await
    .flatten()
    .map(portal_reduces_motion)
}

/// Read the current GNOME theme from the XDG Desktop Portal.
///
/// Reads color scheme (light/dark), accent color, and contrast preference
/// from the `org.freedesktop.appearance` portal namespace via ashpd.
///
/// A setting the portal does not give, because it is unavailable (no D-Bus
/// session, sandboxed environment, or old portal version), does not answer
/// within [`PORTAL_DEADLINE`] or lacks the setting, is left to gsettings
/// where gsettings has it, and otherwise to the preset. The gsettings reads
/// share one [`GSETTINGS_DEADLINE`].
///
/// The text-scaling factor is reported where the toolkit has not applied it:
/// on a Wayland session. On an X11 session (`$XDG_SESSION_TYPE`,
/// [`crate::detect::session_type`]) the reader reports 1.0: winit and gpui
/// take their X11 scale factor from the `Xft.dpi` gnome-settings-daemon
/// writes with the factor in it, so the connectors' text scaling would apply
/// it a second time ([`toolkit_scale_carries_text_scale`] cites the
/// sources). An X11 client through Xwayland on a Wayland session gets that
/// `Xft.dpi` too and draws the factor twice; the reader cannot see a
/// client's backend (docs/todo.md, *Font DPI*).
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`crate::pipeline::select_reader`]; external consumers should use
/// [`SystemTheme::from_system_async()`](crate::SystemTheme::from_system_async).
pub(crate) struct GnomeReader;

#[async_trait::async_trait]
impl crate::reader::ThemeReader for GnomeReader {
    async fn read(&self) -> crate::Result<crate::ReaderResult> {
        // Connect to the portal and read the four appearance settings, each
        // of which can fail independently, all within one deadline.
        let (scheme, accent, contrast, reduced_motion) = with_deadline(
            async {
                let settings = ashpd::desktop::settings::Settings::new().await.ok()?;
                let scheme = settings.color_scheme().await.ok();
                let accent = settings.accent_color().await.ok();
                let contrast = settings.contrast().await.unwrap_or_default();
                let reduced_motion = settings.reduced_motion().await.ok();
                Some((scheme, accent, contrast, reduced_motion))
            },
            PORTAL_DEADLINE,
        )
        .await
        .flatten()
        .unwrap_or_default();

        let start = std::time::Instant::now();
        Ok(build_theme(
            scheme,
            accent,
            contrast,
            reduced_motion,
            crate::detect::session_type(),
            |schema, key| {
                read_gsetting(
                    schema,
                    key,
                    GSETTINGS_DEADLINE.saturating_sub(start.elapsed()),
                )
            },
        ))
    }
}

/// Read KDE theme from kdeglobals, then overlay portal accent color if available.
///
/// Reads the KDE kdeglobals file as the base theme via
/// [`crate::kde::KdeReader`], then attempts to read the accent color from the
/// XDG Desktop Portal. If the portal provides a valid accent color, it is
/// applied to accent, selection, and focus_ring_color fields on the reader's
/// variant.
///
/// Falls back to the KDE-only base if the portal is unavailable, does not
/// answer within [`PORTAL_DEADLINE`], or provides no accent color.
///
/// Requires both `kde` and `portal` features.
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`crate::pipeline::select_reader`]; external consumers should use
/// [`SystemTheme::from_system_async()`](crate::SystemTheme::from_system_async).
#[cfg(feature = "kde")]
pub(crate) struct GnomePortalKdeReader;

#[cfg(feature = "kde")]
#[async_trait::async_trait]
impl crate::reader::ThemeReader for GnomePortalKdeReader {
    async fn read(&self) -> crate::Result<crate::ReaderResult> {
        // Base: invoke the KdeReader through the same trait surface so that
        // any future KDE-side migration (e.g. extra I/O, extra parsing) is
        // picked up here automatically without re-wiring the composite.
        let mut result =
            <crate::kde::KdeReader as crate::reader::ThemeReader>::read(&crate::kde::KdeReader)
                .await?;

        // Try to get accent color from portal, within one deadline
        let accent = with_deadline(
            async {
                let settings = ashpd::desktop::settings::Settings::new().await.ok()?;
                settings.accent_color().await.ok()
            },
            PORTAL_DEADLINE,
        )
        .await
        .flatten();
        let accent = match accent {
            Some(color) => color,
            None => return Ok(result),
        };

        let rgba = match portal_color_to_rgba(&accent) {
            Some(r) => r,
            None => return Ok(result),
        };

        // Apply accent to the reader's variant. The KDE portal's accent is the
        // palette's `QPalette::Highlight` (xdg-desktop-portal-kde
        // `src/settings.cpp:282`), and Breeze draws its focus ring opaque in
        // `[Colors:View] DecorationFocus` (docs/platform-facts.md §2.1.5),
        // which equals it in Breeze and under a Plasma accent: the ring takes
        // the accent as given, not libadwaita's standalone derivation.
        match &mut result.output {
            crate::ReaderOutput::Single { mode, .. } => {
                apply_accent(mode, &rgba);
            }
            crate::ReaderOutput::Dual { light, dark, .. } => {
                apply_accent(light, &rgba);
                apply_accent(dark, &rgba);
            }
        }

        Ok(result)
    }
}

/// Detect which desktop portal backend is running from the session bus names.
///
/// Looks for a bus name containing `portal.desktop.kde` or
/// `portal.desktop.gnome` to infer the desktop environment when
/// `XDG_CURRENT_DESKTOP` is ambiguous or unset. A backend that owns its name
/// (running) is preferred over one that is only activatable (installed); see
/// [`choose_portal_backend`].
///
/// Returns `None` if D-Bus is unavailable, does not answer within
/// [`PORTAL_DEADLINE`], or no single portal backend is found.
pub(crate) async fn detect_portal_backend() -> Option<super::LinuxDesktop> {
    let (running, activatable) = with_deadline(
        async {
            let connection = ashpd::zbus::Connection::session().await.ok()?;
            let proxy = ashpd::zbus::fdo::DBusProxy::new(&connection).await.ok()?;
            let running = proxy.list_names().await.ok()?;
            let activatable = proxy.list_activatable_names().await.ok()?;
            Some((running, activatable))
        },
        PORTAL_DEADLINE,
    )
    .await??;

    let running: Vec<&str> = running.iter().map(|n| n.as_str()).collect();
    let activatable: Vec<&str> = activatable.iter().map(|n| n.as_str()).collect();
    choose_portal_backend(&running, &activatable)
}

/// One deadline for all the D-Bus calls of one portal read together.
///
/// zbus sets no method timeout of its own (zbus-5.19.0
/// `src/connection/builder.rs:660`, `method_timeout: None`), and ashpd's
/// shared connection is a plain `zbus::Connection::session()` (ashpd-0.13.13
/// `src/proxy.rs:36-44`), so a hung or slow-starting portal backend would
/// block `from_system` for as long as the bus lets the call wait. Ruled equal
/// to the subprocess bound the same path already gives each gsettings read.
const PORTAL_DEADLINE: std::time::Duration = crate::detect::SUBPROCESS_TIMEOUT;

/// Await `fut`, or return `None` once `deadline` has passed.
async fn with_deadline<F: std::future::Future>(
    fut: F,
    deadline: std::time::Duration,
) -> Option<F::Output> {
    let mut fut = std::pin::pin!(fut);
    let mut timer = async_io::Timer::after(deadline);
    std::future::poll_fn(|cx| {
        if let std::task::Poll::Ready(out) = fut.as_mut().poll(cx) {
            return std::task::Poll::Ready(Some(out));
        }
        if std::pin::Pin::new(&mut timer).poll(cx).is_ready() {
            return std::task::Poll::Ready(None);
        }
        std::task::Poll::Pending
    })
    .await
}

/// Pick the desktop from the portal backends among the bus's names.
///
/// A running backend (`running`, the names with an owner) decides before an
/// activatable one (`activatable`, the installed `.service` files, in no
/// order the D-Bus spec defines). When a KDE and a GNOME backend match at the
/// same level the answer is `None`, and the caller's fallback decides.
fn choose_portal_backend(running: &[&str], activatable: &[&str]) -> Option<super::LinuxDesktop> {
    fn backend_in(names: &[&str]) -> Result<Option<super::LinuxDesktop>, ()> {
        let kde = names.iter().any(|n| n.contains("portal.desktop.kde"));
        let gnome = names.iter().any(|n| n.contains("portal.desktop.gnome"));
        match (kde, gnome) {
            (true, true) => Err(()),
            (true, false) => Ok(Some(super::LinuxDesktop::Kde)),
            (false, true) => Ok(Some(super::LinuxDesktop::Gnome)),
            (false, false) => Ok(None),
        }
    }

    match backend_in(running) {
        Ok(Some(desktop)) => Some(desktop),
        Ok(None) => backend_in(activatable).ok().flatten(),
        Err(()) => None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === parse_gnome_font_to_fontspec with weight extraction ===

    #[test]
    fn fontspec_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Cantarell Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn fontspec_light_weight() {
        let fs = parse_gnome_font_to_fontspec("Inter Light 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.size, Some(FontSize::Pt(10.0)));
        assert_eq!(fs.weight, Some(300));
    }

    #[test]
    fn fontspec_semi_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Semi-Bold 12").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(12.0)));
        assert_eq!(fs.weight, Some(600));
    }

    #[test]
    fn fontspec_no_modifier_defaults_to_regular() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(400));
    }

    #[test]
    fn fontspec_medium_weight() {
        let fs = parse_gnome_font_to_fontspec("Inter Medium 10.5").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.size, Some(FontSize::Pt(10.5)));
        assert_eq!(fs.weight, Some(500));
    }

    #[test]
    fn fontspec_heavy_weight() {
        let fs = parse_gnome_font_to_fontspec("'Fira Sans Heavy 14'").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Fira Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(14.0)));
        assert_eq!(fs.weight, Some(900));
    }

    #[test]
    fn fontspec_thin_weight() {
        let fs = parse_gnome_font_to_fontspec("Roboto Thin 12").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Roboto"));
        assert_eq!(fs.size, Some(FontSize::Pt(12.0)));
        assert_eq!(fs.weight, Some(100));
    }

    #[test]
    fn fontspec_extra_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Source Sans Extra-Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Source Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(800));
    }

    #[test]
    fn fontspec_ultra_light_weight() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Ultra-Light 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(10.0)));
        assert_eq!(fs.weight, Some(200));
    }

    #[test]
    fn fontspec_empty_returns_none() {
        assert!(parse_gnome_font_to_fontspec("").is_none());
    }

    #[test]
    fn fontspec_with_quotes() {
        let fs = parse_gnome_font_to_fontspec("'Cantarell Bold 11'").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn fontspec_non_finite_size_is_dropped() {
        assert!(parse_gnome_font_to_fontspec("Cantarell NaN").is_none());
        assert!(parse_gnome_font_to_fontspec("Cantarell inf").is_none());
        assert!(parse_gnome_font_to_fontspec("Cantarell 0").is_none());
        assert!(parse_gnome_font_to_fontspec("Cantarell -11").is_none());
    }

    #[test]
    fn fontspec_style_word() {
        let fs = parse_gnome_font_to_fontspec("Cantarell Italic 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(400));
        assert_eq!(fs.style, Some(FontStyle::Italic));
    }

    #[test]
    fn fontspec_weight_and_style_words() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Bold Italic 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        assert_eq!(fs.weight, Some(700));
        assert_eq!(fs.style, Some(FontStyle::Italic));
    }

    #[test]
    fn fontspec_pixel_size() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 15px").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Px(15.0)));
    }

    #[test]
    fn fontspec_variations_and_features_are_skipped() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 11 @wght=200").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        let fs =
            parse_gnome_font_to_fontspec("Cantarell Italic Light 15 @wght=200 #tnum=1").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(15.0)));
        assert_eq!(fs.weight, Some(300));
        assert_eq!(fs.style, Some(FontStyle::Italic));
    }

    #[test]
    fn fontspec_first_family_of_a_list() {
        let fs = parse_gnome_font_to_fontspec("Cantarell, Sans 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(FontSize::Pt(11.0)));
        let fs = parse_gnome_font_to_fontspec("Cantarell,Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.weight, Some(700));
    }

    /// Pango's words match ignoring ASCII case, and a hyphen of its table's
    /// word may be left out (`field_matches`, pango 1.58.2 `pango/fonts.c:1363-1389`).
    #[test]
    fn fontspec_words_ignore_case_and_hyphens() {
        let fs = parse_gnome_font_to_fontspec("Inter semibold OBLIQUE 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.weight, Some(600));
        assert_eq!(fs.style, Some(FontStyle::Oblique));
    }

    /// Pango's Semi-Light, Book and Ultra-Black (`pango/fonts.c:1311-1333`) are
    /// `PANGO_WEIGHT_SEMILIGHT` 350, `PANGO_WEIGHT_BOOK` 380 and
    /// `PANGO_WEIGHT_ULTRAHEAVY` 1000 (pango 1.58.2 `pango/pango-font.h:131-142`),
    /// the last clamped to the 900 that `check_font` accepts.
    #[test]
    fn fontspec_pango_only_weights() {
        let weight = |s| parse_gnome_font_to_fontspec(s).unwrap().weight;
        assert_eq!(weight("Inter Semi-Light 10"), Some(350));
        assert_eq!(weight("Inter Book 10"), Some(380));
        assert_eq!(weight("Inter Ultra-Black 10"), Some(900));
        assert_eq!(weight("Inter weight=650 10"), Some(650));
    }

    /// Stretch, variant and gravity words are Pango's, so they leave the family,
    /// and the model has no field for them.
    #[test]
    fn fontspec_stretch_variant_gravity_words_leave_the_family() {
        let fs =
            parse_gnome_font_to_fontspec("Noto Sans Condensed Small-Caps Rotated-Left 9").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(FontSize::Pt(9.0)));
        assert_eq!(fs.weight, Some(400));
        assert_eq!(fs.style, Some(FontStyle::Normal));
    }

    /// Pango reads the words right to left and stops at the first that is not
    /// one of its own: the rest is the family, and of two weights the leftmost
    /// wins.
    #[test]
    fn fontspec_family_ends_at_the_first_unknown_word() {
        let fs = parse_gnome_font_to_fontspec("DejaVu Sans Mono Book 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("DejaVu Sans Mono"));
        assert_eq!(fs.weight, Some(380));
        let fs = parse_gnome_font_to_fontspec("Noto Sans Bold Mono 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans Bold Mono"));
        assert_eq!(fs.weight, Some(400));
        let fs = parse_gnome_font_to_fontspec("Inter Bold Light 10").unwrap();
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn non_finite_font_size_keeps_the_presets_font() {
        let mut data = default_gnome_data();
        data.font_name = Some("Cantarell NaN".to_string());
        data.monospace_font_name = Some("Source Code Pro inf".to_string());
        let reader = build_gnome_spec_pure(&data);
        assert_eq!(reader_mode(&reader).defaults.font, FontSpec::default());
        assert_eq!(reader_mode(&reader).defaults.mono_font, FontSpec::default());
        assert!(crate::pipeline::run_pipeline(reader, "adwaita").is_ok());
    }

    // === portal_color_to_rgba tests ===

    #[test]
    fn portal_color_valid_converts_to_rgba() {
        let color = Color::new(0.2, 0.4, 0.6);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        let rgba = result.unwrap();
        assert_eq!(rgba, crate::Rgba::from_f32(0.2, 0.4, 0.6, 1.0));
    }

    #[test]
    fn portal_color_out_of_range_high_returns_none() {
        let color = Color::new(1.5, 0.0, 0.0);
        assert!(portal_color_to_rgba(&color).is_none());
    }

    #[test]
    fn portal_color_out_of_range_negative_returns_none() {
        let color = Color::new(-0.1, 0.5, 0.5);
        assert!(portal_color_to_rgba(&color).is_none());
    }

    #[test]
    fn portal_color_to_rgba_boundary_zero() {
        let color = Color::new(0.0, 0.0, 0.0);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), crate::Rgba::from_f32(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn portal_color_to_rgba_boundary_one() {
        let color = Color::new(1.0, 1.0, 1.0);
        let result = portal_color_to_rgba(&color);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), crate::Rgba::from_f32(1.0, 1.0, 1.0, 1.0));
    }

    // === build_theme tests ===

    fn no_gsettings(_schema: &str, _key: &str) -> Option<String> {
        None
    }

    /// Helper: extract the ThemeMode from a ReaderResult for test assertions.
    fn reader_mode(result: &crate::ReaderResult) -> &crate::ThemeMode {
        match &result.output {
            crate::ReaderOutput::Single { mode, .. } => mode,
            crate::ReaderOutput::Dual { light, .. } => light,
        }
    }

    #[test]
    fn dark_scheme_produces_dark_variant_only() {
        let result = build_theme(
            Some(ColorScheme::PreferDark),
            None,
            Contrast::NoPreference,
            None,
            None,
            no_gsettings,
        );
        assert_eq!(result.name, "GNOME");
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
    }

    #[test]
    fn light_scheme_produces_light_variant_only() {
        let result = build_theme(
            Some(ColorScheme::PreferLight),
            None,
            Contrast::NoPreference,
            None,
            None,
            no_gsettings,
        );
        assert_eq!(result.name, "GNOME");
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
    }

    #[test]
    fn no_preference_defaults_to_light() {
        let result = build_theme(
            Some(ColorScheme::NoPreference),
            None,
            Contrast::NoPreference,
            None,
            None,
            no_gsettings,
        );
        assert_eq!(result.name, "GNOME");
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
    }

    // === accent color tests ===

    #[test]
    fn valid_accent_propagates_to_three_fields() {
        let accent = Color::new(0.2, 0.4, 0.8);
        let result = build_theme(
            Some(ColorScheme::NoPreference),
            Some(accent),
            Contrast::NoPreference,
            None,
            None,
            no_gsettings,
        );

        let variant = reader_mode(&result);
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);

        assert_eq!(variant.defaults.accent_color, Some(expected));
        assert_eq!(variant.defaults.selection_background, Some(expected));
        // The ring is libadwaita's `$focus_border_color` of the accent.
        let ring = Some(super::accent::focus_ring(expected, false, false));
        assert_eq!(variant.defaults.focus_ring_color, ring);
        assert_eq!(variant.input.focus_border_color, ring);
    }

    // === high contrast tests ===

    #[test]
    fn high_contrast_sets_flag_on_theme_variant() {
        let result = build_theme(
            Some(ColorScheme::NoPreference),
            None,
            Contrast::High,
            None,
            None,
            no_gsettings,
        );

        assert!(result.accessibility.high_contrast);
    }

    #[test]
    fn normal_contrast_preserves_adwaita_default() {
        let result = build_theme(
            Some(ColorScheme::NoPreference),
            None,
            Contrast::NoPreference,
            None,
            None,
            no_gsettings,
        );

        assert!(!result.accessibility.high_contrast);
    }

    /// Without the portal's high contrast the reader reads gsettings'
    /// `org.gnome.desktop.a11y.interface high-contrast`, and only that key.
    #[test]
    fn gsettings_high_contrast_key_sets_flag() {
        let high_contrast = |schema: &str, key: &str| {
            (schema == "org.gnome.desktop.a11y.interface" && key == "high-contrast")
                .then(|| "true".to_string())
        };
        let result = build_theme(
            Some(ColorScheme::NoPreference),
            None,
            Contrast::NoPreference,
            None,
            None,
            high_contrast,
        );

        assert!(result.accessibility.high_contrast);
    }

    // === colour scheme without the portal's, one read per key (M4-27, M4-55) ===

    fn gsettings_color_scheme(value: &'static str) -> impl FnMut(&str, &str) -> Option<String> {
        move |_schema, key| (key == "color-scheme").then(|| value.to_string())
    }

    #[test]
    fn no_portal_scheme_reads_gsettings_color_scheme() {
        let result = build_theme(
            None,
            None,
            Contrast::NoPreference,
            None,
            None,
            gsettings_color_scheme("prefer-dark"),
        );
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
        let result = build_theme(
            None,
            None,
            Contrast::NoPreference,
            None,
            None,
            gsettings_color_scheme("default"),
        );
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
    }

    #[test]
    fn portal_scheme_wins_over_gsettings() {
        let result = build_theme(
            Some(ColorScheme::PreferLight),
            None,
            Contrast::NoPreference,
            None,
            None,
            gsettings_color_scheme("prefer-dark"),
        );
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
    }

    #[test]
    fn one_gsettings_read_per_key() {
        for (scheme, reads) in [(None, 9), (Some(ColorScheme::PreferDark), 8)] {
            let mut keys = Vec::new();
            let result = build_theme(
                scheme,
                None,
                Contrast::NoPreference,
                None,
                None,
                |schema, key| {
                    keys.push(format!("{schema} {key}"));
                    (key == "icon-theme").then(|| "Papirus".to_string())
                },
            );
            assert_eq!(keys.len(), reads, "{keys:?}");
            keys.sort();
            keys.dedup();
            assert_eq!(keys.len(), reads, "a key read twice");
            assert_eq!(
                reader_mode(&result).defaults.icon_theme.as_deref(),
                Some("Papirus")
            );
        }
    }

    #[test]
    fn a_spent_deadline_reads_nothing() {
        assert_eq!(
            read_gsetting(
                "org.gnome.desktop.interface",
                "font-name",
                std::time::Duration::ZERO
            ),
            None
        );
    }

    // === build_gnome_spec_pure tests (no ashpd types, no I/O) ===

    /// Default GnomePortalData for testing (light, no accent, Cantarell 11, 96 DPI).
    fn default_gnome_data() -> GnomePortalData {
        GnomePortalData {
            is_dark: false,
            accent_rgb: None,
            high_contrast: false,
            reduce_motion: None,
            font_name: Some("Cantarell 11".to_string()),
            monospace_font_name: Some("Source Code Pro 10".to_string()),
            titlebar_font: Some("Cantarell Bold 11".to_string()),
            text_scaling_factor: Some(1.0),
            session_type: None,
            overlay_scrolling: Some(true),
            icon_theme: Some("Adwaita".to_string()),
            gsettings_high_contrast: None,
        }
    }

    /// libadwaita draws its focus ring, and an entry's focus edge, in
    /// `$focus_border_color`: the portal accent's standalone colour at 50 %,
    /// lighter on a dark background (`doc/css-variables.md:137-140`;
    /// `adw_rgba_to_standalone`, `src/adw-accent-color.c:130-157`). The
    /// default blue gives #0461be80 / #81d0ff80; purple #8939a480 / #fba7ff80
    /// (the documented accent table, `doc/css-variables.md:101-230`).
    #[test]
    fn the_focus_ring_and_entry_focus_are_the_accents_standalone_colour_at_half() {
        let hex = |s: &str| s.parse::<crate::Rgba>().expect("a hex colour");
        for (accent, dark, ring) in [
            ((0x35, 0x84, 0xe4), false, "#0461be80"),
            ((0x35, 0x84, 0xe4), true, "#81d0ff80"),
            ((0x91, 0x41, 0xac), false, "#8939a480"),
            ((0x91, 0x41, 0xac), true, "#fba7ff80"),
        ] {
            let mut data = default_gnome_data();
            data.is_dark = dark;
            data.accent_rgb = Some((
                f64::from(accent.0) / 255.0,
                f64::from(accent.1) / 255.0,
                f64::from(accent.2) / 255.0,
            ));
            let result = build_gnome_spec_pure(&data);
            let variant = reader_mode(&result);
            assert_eq!(
                variant.defaults.focus_ring_color,
                Some(hex(ring)),
                "{accent:?} dark {dark}"
            );
            assert_eq!(
                variant.input.focus_border_color,
                Some(hex(ring)),
                "{accent:?} dark {dark}"
            );
        }
    }

    /// Under `prefers-contrast: more` libadwaita colours the ring
    /// `$focus_border_color_hc`, the standalone colour at 80 %
    /// (`_drawing.scss:24-26`; `_colors.scss:261`, `:308`): the high-contrast
    /// flag, from the portal or from gsettings, takes the ring to #0461becc /
    /// #81d0ffcc under the default blue.
    #[test]
    fn under_high_contrast_the_focus_ring_is_the_standalone_colour_at_80() {
        let hex = |s: &str| s.parse::<crate::Rgba>().expect("a hex colour");
        for (dark, portal, gsettings, ring) in [
            (false, true, None, "#0461becc"),
            (true, true, None, "#81d0ffcc"),
            (false, false, Some(true), "#0461becc"),
            (false, false, Some(false), "#0461be80"),
        ] {
            let mut data = default_gnome_data();
            data.is_dark = dark;
            data.high_contrast = portal;
            data.gsettings_high_contrast = gsettings;
            data.accent_rgb = Some((
                f64::from(0x35_u8) / 255.0,
                f64::from(0x84_u8) / 255.0,
                f64::from(0xe4_u8) / 255.0,
            ));
            let result = build_gnome_spec_pure(&data);
            let variant = reader_mode(&result);
            let at = format!("dark {dark}, portal {portal}, gsettings {gsettings:?}");
            assert_eq!(variant.defaults.focus_ring_color, Some(hex(ring)), "{at}");
            assert_eq!(variant.input.focus_border_color, Some(hex(ring)), "{at}");
        }
    }

    /// gnome-settings-daemon writes `Xft.dpi` as 96 × text-scaling-factor × the
    /// X11 UI scale (gnome-settings-daemon 51.0
    /// `plugins/xsettings/gsd-xsettings-manager.c:583-595`, `:665-668`,
    /// `:797-798`), and on a Wayland session the reader reports the factor
    /// itself, which the connectors apply: the factor reaches a drawn size
    /// once, as GTK's 96 × factor does (GTK 4.24.1
    /// `gdk/wayland/gdksettings-wayland.c:166-168`).
    #[test]
    fn text_scale_reaches_the_font_size_once() {
        let mut data = default_gnome_data();
        data.text_scaling_factor = Some(1.5);
        data.session_type = Some("wayland".to_string());
        let reader = build_gnome_spec_pure(&data);
        assert_eq!(reader.font_dpi, Some(96.0));
        let sys = crate::pipeline::run_pipeline(reader, "adwaita").unwrap();
        // Cantarell 11 pt at 96 DPI is 14.67 px; times 1.5 it is 22 px.
        let drawn = sys.accessibility.scaled(sys.light.defaults.font.size);
        assert!((drawn - 22.0).abs() < 0.01, "drawn at {drawn} px");
    }

    /// On an X11 session the toolkit's scale factor, `Xft.dpi` over 96,
    /// carries the text-scaling factor already, so the reader reports none.
    #[test]
    fn an_x11_session_reports_no_text_scale() {
        let mut data = default_gnome_data();
        data.text_scaling_factor = Some(1.5);
        data.session_type = Some("x11".to_string());
        let result = build_gnome_spec_pure(&data);
        assert_eq!(result.accessibility.text_scaling_factor, 1.0);
        assert_eq!(result.accessibility.text_scale_factor(), 1.0);
    }

    /// Off X11 no toolkit scale factor carries the text-scaling factor, so
    /// the reader reports it: on a Wayland session, and where the session
    /// type is not set.
    #[test]
    fn a_session_off_x11_reports_the_text_scale() {
        for session_type in [Some("wayland"), None] {
            let mut data = default_gnome_data();
            data.text_scaling_factor = Some(1.5);
            data.session_type = session_type.map(str::to_string);
            let result = build_gnome_spec_pure(&data);
            assert_eq!(
                result.accessibility.text_scaling_factor, 1.5,
                "{session_type:?}"
            );
        }
    }

    /// The reader passes the session type it is given to the X11 rule.
    #[test]
    fn build_theme_reads_the_session_type_it_is_given() {
        let scale = |_: &str, key: &str| (key == "text-scaling-factor").then(|| "1.5".to_string());
        let factor = |session_type: Option<&str>| {
            build_theme(
                None,
                None,
                Contrast::NoPreference,
                None,
                session_type.map(str::to_string),
                scale,
            )
            .accessibility
            .text_scaling_factor
        };
        assert_eq!(factor(Some("x11")), 1.0);
        assert_eq!(factor(Some("wayland")), 1.5);
    }

    #[test]
    fn pure_light_scheme_produces_light_variant() {
        let data = default_gnome_data();
        let result = build_gnome_spec_pure(&data);
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
        assert_eq!(result.name, "GNOME");
    }

    #[test]
    fn pure_dark_scheme_produces_dark_variant() {
        let mut data = default_gnome_data();
        data.is_dark = true;
        let result = build_gnome_spec_pure(&data);
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
        assert_eq!(result.name, "GNOME");
    }

    #[test]
    fn pure_accent_color_propagates_to_three_fields() {
        let mut data = default_gnome_data();
        data.accent_rgb = Some((0.2, 0.4, 0.8));
        let result = build_gnome_spec_pure(&data);
        let variant = reader_mode(&result);
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);
        assert_eq!(variant.defaults.accent_color, Some(expected));
        assert_eq!(variant.defaults.selection_background, Some(expected));
        // The ring is libadwaita's `$focus_border_color` of the accent.
        let ring = Some(super::accent::focus_ring(expected, false, false));
        assert_eq!(variant.defaults.focus_ring_color, ring);
        assert_eq!(variant.input.focus_border_color, ring);
    }

    #[test]
    fn pure_high_contrast_sets_flag() {
        let mut data = default_gnome_data();
        data.high_contrast = true;
        let result = build_gnome_spec_pure(&data);
        assert!(result.accessibility.high_contrast);
    }

    /// A portal accent other than adwaita's default reaches the four accent fills
    /// through inheritance: adwaita states none of them, as libadwaita fills a
    /// checked check and radio, an on switch, a scale's highlight and a progress
    /// bar's with `--accent-bg-color`, the portal's accent (libadwaita 1.10.0
    /// `src/stylesheet/widgets/_checks.scss:54`, `_switch.scss:43`,
    /// `_scale.scss:15`, `_progress-bar.scss:31`; `_colors.scss:166`), so they
    /// resolve to it. The info colour, which adwaita states, is the reader's
    /// (GtkInfoBar's `--accent-bg-color`, `_deprecated.scss:174`; D7).
    #[test]
    fn a_portal_accent_reaches_the_accent_fills() {
        let mut data = default_gnome_data();
        data.accent_rgb = Some((0.5, 0.25, 0.75));
        let reader = build_gnome_spec_pure(&data);
        let accent = reader_mode(&reader).defaults.accent_color;
        assert!(accent.is_some());
        let sys = crate::pipeline::run_pipeline(reader, "adwaita").unwrap();
        let r = &sys.light;
        assert_ne!(
            Some(r.defaults.accent_color),
            crate::Theme::preset("adwaita")
                .ok()
                .and_then(|t| t.light)
                .and_then(|l| l.defaults.accent_color),
            "the portal's accent is adwaita's default, so this test proves nothing"
        );
        let accent = accent.expect("the portal accent was read");
        assert_eq!(r.defaults.accent_color, accent);
        assert_eq!(r.checkbox.checked_background, accent);
        assert_eq!(r.switch.checked_background, accent);
        assert_eq!(r.slider.fill_color, accent);
        assert_eq!(r.progress_bar.fill_color, accent);
        assert_eq!(r.defaults.info_color, accent);
    }

    /// `size` in points, `None` for a pixel size or none.
    fn pt(size: Option<FontSize>) -> Option<f32> {
        match size {
            Some(FontSize::Pt(v)) => Some(v),
            _ => None,
        }
    }

    /// The text-scale entries and the dialog title a font name of `font`
    /// gives, merged over the adwaita preset: `(caption, dialog_title,
    /// display, dialog.title_font)` in points.
    fn type_scale_over_adwaita(font: &str) -> [Option<FontSize>; 4] {
        let mut data = default_gnome_data();
        data.font_name = Some(font.to_string());
        let reader = build_gnome_spec_pure(&data);
        let mut mode = crate::Theme::preset("adwaita")
            .ok()
            .and_then(|t| t.light)
            .unwrap_or_default();
        mode.merge(reader_mode(&reader));
        let size = |e: &Option<crate::TextScaleEntry>| e.as_ref().and_then(|e| e.size);
        [
            size(&mode.text_scale.caption),
            size(&mode.text_scale.dialog_title),
            size(&mode.text_scale.display),
            mode.dialog.title_font.as_ref().and_then(|f| f.size),
        ]
    }

    /// At GNOME's default 11pt the derived sizes are adwaita.toml's stated
    /// ones (caption 9, dialog title 15, display 20, the dialog's title 15),
    /// which the facts round to the whole point (≈, `docs/platform-facts.md:748-755`).
    #[test]
    fn the_default_font_gives_the_presets_type_scale() {
        let preset = crate::Theme::preset("adwaita")
            .ok()
            .and_then(|t| t.light)
            .unwrap_or_default();
        let size = |e: &Option<crate::TextScaleEntry>| e.as_ref().and_then(|e| e.size);
        let stated = [
            size(&preset.text_scale.caption),
            size(&preset.text_scale.dialog_title),
            size(&preset.text_scale.display),
            preset.dialog.title_font.as_ref().and_then(|f| f.size),
        ];
        assert_eq!(
            stated,
            [
                Some(FontSize::Pt(9.0)),
                Some(FontSize::Pt(15.0)),
                Some(FontSize::Pt(20.0)),
                Some(FontSize::Pt(15.0)),
            ]
        );
        let derived = type_scale_over_adwaita("Adwaita Sans 11");
        for (derived, stated) in derived.into_iter().zip(stated) {
            assert!(pt(derived).is_some(), "not a point size: {derived:?}");
            assert_eq!(
                pt(derived).map(f32::round),
                pt(stated),
                "{derived:?} does not round to {stated:?}"
            );
        }
    }

    /// Another font scales them by libadwaita's shares, and the preset's
    /// weights and `section_heading` stay.
    #[test]
    fn another_font_scales_the_type_scale() {
        let derived = type_scale_over_adwaita("Adwaita Sans 14");
        let expected = [14.0 * 0.82, 14.0 * 1.36, 14.0 * 1.81, 14.0 * 1.36];
        for (derived, expected) in derived.into_iter().zip(expected) {
            assert!(
                pt(derived).is_some_and(|d| (d - expected).abs() < 1e-4),
                "{derived:?} is not {expected}pt"
            );
        }
        let mut data = default_gnome_data();
        data.font_name = Some("Adwaita Sans 14px".to_string());
        let reader = build_gnome_spec_pure(&data);
        let caption = reader_mode(&reader)
            .text_scale
            .caption
            .as_ref()
            .and_then(|e| e.size);
        assert!(
            matches!(caption, Some(FontSize::Px(v)) if (v - 14.0 * 0.82).abs() < 1e-4),
            "a pixel font scales in pixels: {caption:?}"
        );
        let mut mode = crate::Theme::preset("adwaita")
            .ok()
            .and_then(|t| t.light)
            .unwrap_or_default();
        let section = mode.text_scale.section_heading.clone();
        data.font_name = Some("Adwaita Sans 14".to_string());
        mode.merge(reader_mode(&build_gnome_spec_pure(&data)));
        assert_eq!(mode.text_scale.section_heading, section);
        assert_eq!(
            mode.text_scale.dialog_title.and_then(|e| e.weight),
            Some(800)
        );
        assert_eq!(mode.text_scale.display.and_then(|e| e.weight), Some(800));
        assert_eq!(mode.dialog.title_font.and_then(|f| f.weight), Some(800));
    }

    /// The text-scaling factor is not folded into the derived sizes: it is
    /// reported apart and applied once by the connectors (M2e).
    #[test]
    fn the_text_scaling_factor_does_not_reach_the_type_scale() {
        let mut data = default_gnome_data();
        data.font_name = Some("Adwaita Sans 11".to_string());
        let plain = build_gnome_spec_pure(&data);
        data.text_scaling_factor = Some(1.5);
        data.session_type = Some("wayland".to_string());
        let scaled = build_gnome_spec_pure(&data);
        assert_eq!(
            reader_mode(&plain).text_scale,
            reader_mode(&scaled).text_scale
        );
        assert_eq!(
            reader_mode(&plain).dialog.title_font,
            reader_mode(&scaled).dialog.title_font
        );
        assert_eq!(scaled.accessibility.text_scaling_factor, 1.5);
    }

    /// Without a size in the font name the reader states no type scale, and
    /// the preset's stands.
    #[test]
    fn a_font_without_a_size_leaves_the_type_scale() {
        let mut data = default_gnome_data();
        data.font_name = Some("Adwaita Sans".to_string());
        let reader = build_gnome_spec_pure(&data);
        assert_eq!(reader_mode(&reader).text_scale, crate::TextScale::default());
        assert_eq!(reader_mode(&reader).dialog.title_font, None);
    }

    #[test]
    fn pure_fonts_parsed_correctly() {
        let mut data = default_gnome_data();
        data.font_name = Some("Inter Bold 12".to_string());
        let result = build_gnome_spec_pure(&data);
        let variant = reader_mode(&result);
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Inter"));
        assert_eq!(variant.defaults.font.weight, Some(700));
        assert_eq!(variant.defaults.font.size, Some(FontSize::Pt(12.0)));
    }

    #[test]
    fn pure_out_of_range_accent_ignored() {
        // Build without accent to get baseline
        let baseline_data = default_gnome_data();
        let baseline_result = build_gnome_spec_pure(&baseline_data);
        let baseline_variant = reader_mode(&baseline_result);
        let baseline_accent = baseline_variant.defaults.accent_color;

        // Build with out-of-range accent
        let mut data = default_gnome_data();
        data.accent_rgb = Some((1.5, 0.0, 0.0));
        let result = build_gnome_spec_pure(&data);
        let variant = reader_mode(&result);
        // Out-of-range accent should NOT override -- result matches baseline
        assert_eq!(variant.defaults.accent_color, baseline_accent);
    }

    #[test]
    fn pure_gsettings_high_contrast_fallback() {
        let mut data = default_gnome_data();
        data.high_contrast = false;
        data.gsettings_high_contrast = Some(true);
        let result = build_gnome_spec_pure(&data);
        assert!(result.accessibility.high_contrast);
    }

    #[test]
    fn pure_reduce_motion_from_portal() {
        let mut data = default_gnome_data();
        data.reduce_motion = Some(true);
        let result = build_gnome_spec_pure(&data);
        assert!(result.accessibility.reduce_motion);
    }

    /// Reduced motion is the portal's, else gsettings' `enable-animations`
    /// (`false` is reduced motion), as [`crate::detect::detect_reduced_motion`]
    /// reads it on GNOME.
    #[test]
    fn reduce_motion_is_the_portal_then_gsettings() {
        let animations_off =
            |_: &str, key: &str| (key == "enable-animations").then(|| "false".to_string());
        let read = |portal| {
            build_theme(
                None,
                None,
                Contrast::NoPreference,
                portal,
                None,
                animations_off,
            )
            .accessibility
            .reduce_motion
        };
        assert!(read(None));
        assert!(!read(Some(ReducedMotion::NoPreference)));
        assert!(read(Some(ReducedMotion::ReducedMotion)));
    }

    #[test]
    fn pure_overlay_scrolling() {
        let mut data = default_gnome_data();
        data.overlay_scrolling = Some(false);
        let result = build_gnome_spec_pure(&data);
        let variant = reader_mode(&result);
        assert_eq!(variant.scrollbar.overlay_mode, Some(false));
    }

    // === Plan 94-03 (G8): GnomeReader + GnomePortalKdeReader ThemeReader impls ===
    //
    // Locks the G8 contract on the GNOME side: two unit structs `GnomeReader`
    // and `GnomePortalKdeReader` (the latter gated on `feature = "kde"`) exist
    // at the module root and implement `crate::reader::ThemeReader`. Before
    // Task 2 lands, this fails to compile (`no struct GnomeReader` / `no struct
    // GnomePortalKdeReader` / `unresolved module crate::reader`).
    #[test]
    fn gnome_reader_exists() {
        // Compile-probe: construct trait objects for each reader available on
        // this build. Do NOT invoke .read() — portal I/O is environment-
        // dependent (D-Bus may be unavailable in the test sandbox) and the
        // structural invariant (the trait coercion) is the load-bearing check.
        let r: Box<dyn crate::reader::ThemeReader> = Box::new(super::GnomeReader);
        let _ = &r;

        #[cfg(feature = "kde")]
        {
            let rp: Box<dyn crate::reader::ThemeReader> = Box::new(super::GnomePortalKdeReader);
            let _ = &rp;
        }
    }

    // === portal deadline and backend choice (M4-26, M4-28) ===

    const KDE_BACKEND: &str = "org.freedesktop.impl.portal.desktop.kde";
    const GNOME_BACKEND: &str = "org.freedesktop.impl.portal.desktop.gnome";

    #[test]
    fn portal_backend_running_preferred_over_activatable() {
        assert_eq!(
            choose_portal_backend(&[KDE_BACKEND], &[GNOME_BACKEND, KDE_BACKEND]),
            Some(crate::LinuxDesktop::Kde)
        );
    }

    #[test]
    fn portal_backend_both_running_is_none() {
        assert_eq!(
            choose_portal_backend(&[GNOME_BACKEND, KDE_BACKEND], &[GNOME_BACKEND, KDE_BACKEND]),
            None
        );
    }

    #[test]
    fn portal_backend_only_activatable_gnome() {
        assert_eq!(
            choose_portal_backend(&["org.freedesktop.DBus"], &[GNOME_BACKEND]),
            Some(crate::LinuxDesktop::Gnome)
        );
    }

    #[test]
    fn portal_backend_both_activatable_none_running_is_none() {
        assert_eq!(
            choose_portal_backend(&[], &[KDE_BACKEND, GNOME_BACKEND]),
            None
        );
        assert_eq!(choose_portal_backend(&[], &[]), None);
    }

    #[test]
    fn deadline_ends_a_future_that_never_completes() {
        let deadline = std::time::Duration::from_millis(100);
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let out = pollster::block_on(with_deadline(std::future::pending::<()>(), deadline));
            let _ = tx.send((out, start.elapsed()));
        });
        let (out, elapsed) = rx
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("with_deadline did not return within 5 s");
        assert_eq!(out, None);
        assert!(elapsed >= deadline, "returned after {elapsed:?}");
    }

    #[test]
    fn deadline_passes_a_ready_value_through() {
        let out = pollster::block_on(with_deadline(
            std::future::ready(7),
            std::time::Duration::from_secs(5),
        ));
        assert_eq!(out, Some(7));
    }
}
