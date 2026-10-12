//! GNOME portal reader: reads accent color, color scheme, contrast,
//! fonts, text scale, accessibility flags, and icon_set from the
//! XDG Desktop Portal Settings interface (via ashpd) and gsettings.
//!
//! Uses the bundled Adwaita preset as base, then overlays portal-provided
//! accent color, color scheme (light/dark), and contrast preference,
//! along with OS-readable font, accessibility, and icon settings.

use ashpd::desktop::Color;
use ashpd::desktop::settings::{ColorScheme, Contrast, ReducedMotion};

use crate::model::font::{FontSize, FontStyle};
use crate::model::{DialogButtonOrder, FontSpec};

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

/// Read a single gsettings value, returning None if gsettings is unavailable or fails.
///
/// Uses [`crate::detect::gsettings_get()`] to enforce a 2-second deadline,
/// preventing indefinite blocking when D-Bus is unresponsive.  Single-quote
/// wrapping (gsettings string output format) is stripped from the result.
fn read_gsetting(schema: &str, key: &str) -> Option<String> {
    let raw = crate::detect::gsettings_get(schema, key)?;
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
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) is_dark: bool,
    /// Accent color as (R, G, B) with components in 0.0..=1.0 (XDG portal spec).
    /// None means no accent color set.
    pub(crate) accent_rgb: Option<(f64, f64, f64)>,
    /// Whether high contrast is requested (from portal Contrast::High).
    pub(crate) high_contrast: bool,
    /// Whether reduced motion is requested. None = not provided by portal.
    pub(crate) reduce_motion: Option<bool>,
    /// Primary UI font string in GNOME/Pango format (e.g. "Cantarell 11").
    pub(crate) font_name: Option<String>,
    /// Monospace font string (e.g. "Source Code Pro 10").
    pub(crate) monospace_font_name: Option<String>,
    /// Window titlebar font string.
    pub(crate) titlebar_font: Option<String>,
    /// Text scaling factor (e.g. 1.0, 1.25).
    pub(crate) text_scaling_factor: Option<f32>,
    /// Whether overlay scrollbars are enabled. None = not provided.
    pub(crate) overlay_scrolling: Option<bool>,
    /// Icon theme name (e.g. "Adwaita", "Papirus").
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) icon_theme: Option<String>,
    /// High contrast from gsettings fallback (for GNOME < 44).
    /// Only used when `high_contrast` is false (portal didn't report high contrast).
    pub(crate) gsettings_high_contrast: Option<bool>,
    /// Reduced motion from gsettings enable-animations fallback.
    /// Only used when `reduce_motion` is None (portal didn't report).
    pub(crate) gsettings_enable_animations: Option<bool>,
}

/// Extract AccessibilityPreferences from GNOME portal + gsettings data.
///
/// Merges portal and gsettings sources: portal `high_contrast` and
/// `reduce_motion` take priority, gsettings values are fallback.
/// GNOME does not expose `reduce_transparency`, so it keeps the default (false).
fn accessibility_from_gnome_data(data: &GnomePortalData) -> crate::AccessibilityPreferences {
    let mut acc = crate::AccessibilityPreferences::default();
    let effective_high_contrast = data.high_contrast || data.gsettings_high_contrast == Some(true);
    let effective_reduce_motion = data
        .reduce_motion
        .or_else(|| data.gsettings_enable_animations.map(|anim| !anim));
    if let Some(tsf) = data.text_scaling_factor {
        acc.text_scaling_factor = tsf;
    }
    if let Some(rm) = effective_reduce_motion {
        acc.reduce_motion = rm;
    }
    acc.high_contrast = effective_high_contrast;
    // reduce_transparency keeps default -- GNOME does not expose this setting
    acc
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
        apply_accent(&mut variant, &rgba);
    }

    // NOTE: high_contrast, reduce_motion and text_scaling_factor
    // are no longer on ThemeDefaults. They live on AccessibilityPreferences,
    // constructed by the pipeline from GnomePortalData fields.

    // ── Fonts (GNOME-01) ────────────────────────────────────────────────
    // Primary UI font
    if let Some(ref font_str) = data.font_name
        && let Some(fs) = parse_gnome_font_to_fontspec(font_str)
    {
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
    // icon_theme is set on variant.defaults in build_gnome_spec_pure()

    // ── Dialog button order (project decision) ──────────────────────────
    variant.dialog.button_order = Some(DialogButtonOrder::PrimaryRight);

    variant
}

/// Build a GNOME ReaderResult from pre-read portal + gsettings data.
///
/// This is the fully testable entry point: no D-Bus, no gsettings,
/// no xrdb/xrandr. Loads the Adwaita preset as base, builds a sparse
/// OS variant from the provided data, and merges it onto the base.
#[cfg(test)]
fn build_gnome_spec_pure(data: &GnomePortalData) -> crate::Result<crate::ReaderResult> {
    let base = crate::Theme::preset("adwaita")?;
    let is_dark = data.is_dark;

    let mut variant = if is_dark {
        base.dark.unwrap_or_default()
    } else {
        base.light.unwrap_or_default()
    };

    let os_variant = build_gnome_variant_pure(data);
    variant.merge(&os_variant);

    let acc = accessibility_from_gnome_data(data);

    // Icon theme on variant defaults (per-variant)
    variant.defaults.icon_theme = data.icon_theme.clone().map(std::borrow::Cow::Owned);

    let output = crate::ReaderOutput::Single {
        mode: Box::new(variant),
        is_dark,
    };

    Ok(crate::ReaderResult {
        output,
        name: "GNOME".into(),
        icon_set: None,
        layout: crate::LayoutTheme::default(),
        font_dpi: Some(crate::detect::GNOME_FONT_DPI),
        accessibility: acc,
    })
}

/// Build a sparse ThemeMode populated only with OS-readable fields.
///
/// This function does NOT embed any Adwaita preset data -- it only sets
/// fields that the GNOME desktop provides via gsettings and portal data.
/// The caller merges this sparse variant onto an Adwaita base.
///
/// Converts ashpd types to primitives and delegates to [`build_gnome_variant_pure`].
pub(crate) fn build_gnome_variant(
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
    reduced_motion: Option<ReducedMotion>,
) -> (crate::ThemeMode, crate::AccessibilityPreferences, f32) {
    // Convert ashpd types to primitives
    let accent_rgb = accent.as_ref().map(|c| {
        let (r, g, b) = (c.red(), c.green(), c.blue());
        (r, g, b)
    });

    let high_contrast = matches!(contrast, Contrast::High);

    let reduce_motion = reduced_motion.map(|rm| match rm {
        ReducedMotion::ReducedMotion => true,
        ReducedMotion::NoPreference => false,
    });

    // Read gsettings values (I/O stays here, not in pure function)
    let font_name = read_gsetting("org.gnome.desktop.interface", "font-name");
    let monospace_font_name = read_gsetting("org.gnome.desktop.interface", "monospace-font-name");
    let titlebar_font = read_gsetting("org.gnome.desktop.wm.preferences", "titlebar-font");

    let text_scaling_factor = read_gsetting("org.gnome.desktop.interface", "text-scaling-factor")
        .and_then(|s| s.parse::<f32>().ok());

    let gsettings_high_contrast =
        read_gsetting("org.gnome.desktop.a11y.interface", "high-contrast").and_then(|s| {
            match s.as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        });

    let gsettings_enable_animations =
        read_gsetting("org.gnome.desktop.interface", "enable-animations").and_then(|s| {
            match s.as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        });

    let overlay_scrolling = read_gsetting("org.gnome.desktop.interface", "overlay-scrolling")
        .and_then(|s| match s.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        });

    let icon_theme = read_gsetting("org.gnome.desktop.interface", "icon-theme");

    let data = GnomePortalData {
        is_dark: false, // not used by build_gnome_variant_pure
        accent_rgb,
        high_contrast,
        reduce_motion,
        font_name,
        monospace_font_name,
        titlebar_font,
        text_scaling_factor,
        overlay_scrolling,
        icon_theme,
        gsettings_high_contrast,
        gsettings_enable_animations,
    };

    let _ = scheme; // consumed by caller for light/dark selection

    let acc = accessibility_from_gnome_data(&data);
    (
        build_gnome_variant_pure(&data),
        acc,
        crate::detect::GNOME_FONT_DPI,
    )
}

/// Build a ReaderResult from an Adwaita base, applying portal-provided
/// color scheme, accent color, contrast, and reduced-motion settings
/// plus OS-readable fields.
///
/// This is the testable core -- no D-Bus required.
pub(crate) fn build_theme(
    base: crate::Theme,
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
    reduced_motion: Option<ReducedMotion>,
) -> crate::Result<crate::ReaderResult> {
    let is_dark = matches!(scheme, ColorScheme::PreferDark);

    // Pick the appropriate variant from the Adwaita base.
    // unwrap_or_default() provides an empty ThemeMode when the base
    // preset lacks the requested variant, allowing the merge to proceed.
    let mut variant = if is_dark {
        base.dark.unwrap_or_default()
    } else {
        base.light.unwrap_or_default()
    };

    // Build sparse OS variant and merge onto Adwaita base
    let (os_variant, acc, font_dpi) = build_gnome_variant(scheme, accent, contrast, reduced_motion);
    variant.merge(&os_variant);

    // Read icon_theme from gsettings (per-variant)
    variant.defaults.icon_theme =
        read_gsetting("org.gnome.desktop.interface", "icon-theme").map(std::borrow::Cow::Owned);

    let output = crate::ReaderOutput::Single {
        mode: Box::new(variant),
        is_dark,
    };

    Ok(crate::ReaderResult {
        output,
        name: "GNOME".into(),
        icon_set: None,
        layout: crate::LayoutTheme::default(),
        font_dpi: Some(font_dpi),
        accessibility: acc,
    })
}

/// Read the current GNOME theme from the XDG Desktop Portal.
///
/// Reads color scheme (light/dark), accent color, and contrast preference
/// from the `org.freedesktop.appearance` portal namespace via ashpd.
///
/// Falls back to bundled Adwaita defaults when the portal is unavailable
/// (no D-Bus session, sandboxed environment, or old portal version).
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`pipeline::select_reader`]; external consumers should use
/// [`SystemTheme::from_system_async()`](crate::SystemTheme::from_system_async).
pub(crate) struct GnomeReader;

#[async_trait::async_trait]
impl crate::reader::ThemeReader for GnomeReader {
    async fn read(&self) -> crate::Result<crate::ReaderResult> {
        let base = crate::Theme::preset("adwaita")?;

        // Try to connect to the portal. If unavailable, return Adwaita defaults.
        let settings = match ashpd::desktop::settings::Settings::new().await {
            Ok(s) => s,
            Err(_) => {
                return build_theme(
                    base,
                    ColorScheme::NoPreference,
                    None,
                    Contrast::NoPreference,
                    None,
                );
            }
        };

        // Read the four appearance settings. Each can fail independently.
        let scheme = settings.color_scheme().await.unwrap_or_default();
        let accent = settings.accent_color().await.ok();
        let contrast = settings.contrast().await.unwrap_or_default();
        let reduced_motion = settings.reduced_motion().await.ok();

        build_theme(base, scheme, accent, contrast, reduced_motion)
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
/// Falls back to the KDE-only base if the portal is unavailable or provides
/// no accent color.
///
/// Requires both `kde` and `portal` features.
///
/// Internal reader dispatched through the [`crate::reader::ThemeReader`] trait
/// by [`pipeline::select_reader`]; external consumers should use
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

        // Try to get accent color from portal
        let settings = match ashpd::desktop::settings::Settings::new().await {
            Ok(s) => s,
            Err(_) => return Ok(result),
        };

        let accent = match settings.accent_color().await {
            Ok(color) => color,
            Err(_) => return Ok(result),
        };

        let rgba = match portal_color_to_rgba(&accent) {
            Some(r) => r,
            None => return Ok(result),
        };

        // Apply accent to the reader's variant
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

/// Detect which desktop portal backend is running via D-Bus activatable names.
///
/// Checks the session bus for activatable service names containing
/// `portal.desktop.kde` or `portal.desktop.gnome` to infer the desktop
/// environment when `XDG_CURRENT_DESKTOP` is ambiguous or unset.
///
/// Returns `None` if D-Bus is unavailable or no recognized portal backend
/// is found.
pub(crate) async fn detect_portal_backend() -> Option<super::LinuxDesktop> {
    let connection = ashpd::zbus::Connection::session().await.ok()?;
    let proxy = ashpd::zbus::fdo::DBusProxy::new(&connection).await.ok()?;
    let names = proxy.list_activatable_names().await.ok()?;

    for name in &names {
        let name_str = name.as_str();
        if name_str.contains("portal.desktop.kde") {
            return Some(super::LinuxDesktop::Kde);
        }
        if name_str.contains("portal.desktop.gnome") {
            return Some(super::LinuxDesktop::Gnome);
        }
    }

    None
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
        let reader = build_gnome_spec_pure(&data).unwrap();
        let preset = crate::Theme::preset("adwaita").unwrap().light.unwrap();
        assert_eq!(reader_mode(&reader).defaults.font, preset.defaults.font);
        assert_eq!(
            reader_mode(&reader).defaults.mono_font,
            preset.defaults.mono_font
        );
        assert!(crate::pipeline::run_pipeline(reader, "adwaita", crate::ColorMode::Light).is_ok());
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

    // === build_gnome_variant tests ===

    #[test]
    fn build_gnome_variant_default_has_dialog_button_order() {
        let (v, _acc, _dpi) = build_gnome_variant(
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        );
        assert_eq!(v.dialog.button_order, Some(DialogButtonOrder::PrimaryRight),);
    }

    #[test]
    fn build_gnome_variant_high_contrast_sets_flag() {
        let (_v, _acc, _dpi) =
            build_gnome_variant(ColorScheme::NoPreference, None, Contrast::High, None);
        // high_contrast now on AccessibilityPreferences
    }

    #[test]
    fn build_gnome_variant_normal_contrast_no_flag() {
        let (_v, _acc, _dpi) = build_gnome_variant(
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        );
        // high_contrast now on AccessibilityPreferences
    }

    #[test]
    fn build_gnome_variant_accent_sets_defaults() {
        let accent = Color::new(0.2, 0.4, 0.8);
        let (v, _acc, _dpi) = build_gnome_variant(
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
            None,
        );
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);
        assert_eq!(v.defaults.accent_color, Some(expected));
        assert_eq!(v.defaults.selection_background, Some(expected));
        assert_eq!(v.defaults.focus_ring_color, Some(expected));
    }

    #[test]
    fn build_gnome_variant_invalid_accent_stays_none() {
        let accent = Color::new(1.5, 0.0, 0.0); // out of range
        let (v, _acc, _dpi) = build_gnome_variant(
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
            None,
        );
        assert!(v.defaults.accent_color.is_none());
    }

    // === build_theme tests ===

    fn adwaita_base() -> crate::Theme {
        crate::Theme::preset("adwaita").unwrap()
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
            adwaita_base(),
            ColorScheme::PreferDark,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();
        assert_eq!(result.name, "GNOME");
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: true, .. }
        ));
    }

    #[test]
    fn light_scheme_produces_light_variant_only() {
        let result = build_theme(
            adwaita_base(),
            ColorScheme::PreferLight,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();
        assert_eq!(result.name, "GNOME");
        assert!(matches!(
            result.output,
            crate::ReaderOutput::Single { is_dark: false, .. }
        ));
    }

    #[test]
    fn no_preference_defaults_to_light() {
        let result = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();
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
            adwaita_base(),
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = reader_mode(&result);
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);

        assert_eq!(variant.defaults.accent_color, Some(expected));
        assert_eq!(variant.defaults.selection_background, Some(expected));
        assert_eq!(variant.defaults.focus_ring_color, Some(expected));
    }

    // === high contrast tests ===

    #[test]
    fn high_contrast_sets_flag_on_theme_variant() {
        let result = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::High,
            None,
        )
        .unwrap();

        let _variant = reader_mode(&result);
        // high_contrast now on AccessibilityPreferences
    }

    #[test]
    fn normal_contrast_preserves_adwaita_default() {
        let result = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let _variant = reader_mode(&result);
        // Adwaita preset sets high_contrast = false; OS variant doesn't override it
        // high_contrast now on AccessibilityPreferences
    }

    // === build_theme merge correctness ===

    #[test]
    fn build_theme_preserves_adwaita_colors_when_no_overrides() {
        let base = adwaita_base();
        let base_light = base.light.as_ref().unwrap().clone();

        let result = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = reader_mode(&result);
        // Adwaita colors should be preserved since OS variant doesn't set colors
        assert_eq!(
            variant.defaults.background_color,
            base_light.defaults.background_color
        );
        assert_eq!(variant.defaults.text_color, base_light.defaults.text_color);
        assert_eq!(result.font_dpi, Some(96.0));
    }

    #[test]
    fn build_theme_dialog_order_set() {
        let result = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = reader_mode(&result);
        assert_eq!(
            variant.dialog.button_order,
            Some(DialogButtonOrder::PrimaryRight),
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
            overlay_scrolling: Some(true),
            icon_theme: Some("Adwaita".to_string()),
            gsettings_high_contrast: None,
            gsettings_enable_animations: None,
        }
    }

    /// gnome-settings-daemon writes `Xft.dpi` as 96 × text-scaling-factor × the
    /// X11 UI scale (gnome-settings-daemon 51.0
    /// `plugins/xsettings/gsd-xsettings-manager.c:583-595`, `:665-668`,
    /// `:797-798`), and the reader reports the factor itself, which the
    /// connectors apply: the factor reaches a drawn size once, as GTK's
    /// 96 × factor does (GTK 4.24.1 `gdk/wayland/gdksettings-wayland.c:166-168`).
    #[test]
    fn text_scale_reaches_the_font_size_once() {
        let mut data = default_gnome_data();
        data.text_scaling_factor = Some(1.5);
        let reader = build_gnome_spec_pure(&data).unwrap();
        assert_eq!(reader.font_dpi, Some(96.0));
        let sys =
            crate::pipeline::run_pipeline(reader, "adwaita", crate::ColorMode::Light).unwrap();
        // Cantarell 11 pt at 96 DPI is 14.67 px; times 1.5 it is 22 px.
        let drawn = sys.accessibility.scaled(sys.light.defaults.font.size);
        assert!((drawn - 22.0).abs() < 0.01, "drawn at {drawn} px");
    }

    #[test]
    fn pure_light_scheme_produces_light_variant() {
        let data = default_gnome_data();
        let result = build_gnome_spec_pure(&data).unwrap();
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
        let result = build_gnome_spec_pure(&data).unwrap();
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
        let result = build_gnome_spec_pure(&data).unwrap();
        let variant = reader_mode(&result);
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);
        assert_eq!(variant.defaults.accent_color, Some(expected));
        assert_eq!(variant.defaults.selection_background, Some(expected));
        assert_eq!(variant.defaults.focus_ring_color, Some(expected));
    }

    #[test]
    fn pure_high_contrast_sets_flag() {
        let mut data = default_gnome_data();
        data.high_contrast = true;
        let result = build_gnome_spec_pure(&data).unwrap();
        let _variant = reader_mode(&result);
        // high_contrast now on AccessibilityPreferences
    }

    #[test]
    fn pure_fonts_parsed_correctly() {
        let mut data = default_gnome_data();
        data.font_name = Some("Inter Bold 12".to_string());
        let result = build_gnome_spec_pure(&data).unwrap();
        let variant = reader_mode(&result);
        assert_eq!(variant.defaults.font.family.as_deref(), Some("Inter"));
        assert_eq!(variant.defaults.font.weight, Some(700));
        assert_eq!(variant.defaults.font.size, Some(FontSize::Pt(12.0)));
    }

    #[test]
    fn pure_out_of_range_accent_ignored() {
        // Build without accent to get baseline
        let baseline_data = default_gnome_data();
        let baseline_result = build_gnome_spec_pure(&baseline_data).unwrap();
        let baseline_variant = reader_mode(&baseline_result);
        let baseline_accent = baseline_variant.defaults.accent_color;

        // Build with out-of-range accent
        let mut data = default_gnome_data();
        data.accent_rgb = Some((1.5, 0.0, 0.0));
        let result = build_gnome_spec_pure(&data).unwrap();
        let variant = reader_mode(&result);
        // Out-of-range accent should NOT override -- result matches baseline
        assert_eq!(variant.defaults.accent_color, baseline_accent);
    }

    #[test]
    fn pure_gsettings_high_contrast_fallback() {
        let mut data = default_gnome_data();
        data.high_contrast = false;
        data.gsettings_high_contrast = Some(true);
        let result = build_gnome_spec_pure(&data).unwrap();
        let _variant = reader_mode(&result);
        // high_contrast now on AccessibilityPreferences
    }

    #[test]
    fn pure_reduce_motion_from_portal() {
        let mut data = default_gnome_data();
        data.reduce_motion = Some(true);
        let result = build_gnome_spec_pure(&data).unwrap();
        let _variant = reader_mode(&result);
        // reduce_motion now on AccessibilityPreferences
    }

    #[test]
    fn pure_reduce_motion_gsettings_fallback() {
        let mut data = default_gnome_data();
        data.reduce_motion = None;
        data.gsettings_enable_animations = Some(false);
        let result = build_gnome_spec_pure(&data).unwrap();
        let _variant = reader_mode(&result);
        // enable-animations=false means reduce_motion=true
        // reduce_motion now on AccessibilityPreferences
    }

    #[test]
    fn pure_overlay_scrolling() {
        let mut data = default_gnome_data();
        data.overlay_scrolling = Some(false);
        let result = build_gnome_spec_pure(&data).unwrap();
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
}
