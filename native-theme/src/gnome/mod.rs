//! GNOME portal reader: reads accent color, color scheme, contrast,
//! fonts, text scale, accessibility flags, and icon_set from the
//! XDG Desktop Portal Settings interface (via ashpd) and gsettings.
//!
//! Uses the bundled Adwaita preset as base, then overlays portal-provided
//! accent color, color scheme (light/dark), and contrast preference,
//! along with OS-readable font, accessibility, and icon settings.

use ashpd::desktop::Color;
use ashpd::desktop::settings::{ColorScheme, Contrast, ReducedMotion};

use crate::model::{DialogButtonOrder, FontSpec, TextScale, TextScaleEntry};

/// Known GNOME/Pango font weight modifiers mapped to CSS weight values.
const WEIGHT_MODIFIERS: &[(&str, u16)] = &[
    ("Ultra-Bold", 800),
    ("UltraBold", 800),
    ("Extra-Bold", 800),
    ("ExtraBold", 800),
    ("Semi-Bold", 600),
    ("SemiBold", 600),
    ("Demi-Bold", 600),
    ("DemiBold", 600),
    ("Ultra-Light", 200),
    ("UltraLight", 200),
    ("Extra-Light", 200),
    ("ExtraLight", 200),
    ("Thin", 100),
    ("Light", 300),
    ("Regular", 400),
    ("Normal", 400),
    ("Medium", 500),
    ("Bold", 700),
    ("Black", 900),
    ("Heavy", 900),
];

/// Convert an ashpd portal Color to an Rgba, returning None if any
/// component is outside the [0.0, 1.0] range (per XDG spec: out-of-range
/// means "unset").
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
fn apply_accent(variant: &mut crate::ThemeVariant, accent: &crate::Rgba) {
    variant.defaults.accent = Some(*accent);
    variant.defaults.selection = Some(*accent);
    variant.defaults.focus_ring_color = Some(*accent);
}

/// Parse a GNOME/Pango font string into a full FontSpec with weight extraction.
///
/// GNOME font strings can have weight words between the family name and size:
/// `'Cantarell Bold 11'`, `'Inter Light 10.5'`, `'Noto Sans Semi-Bold 12'`.
///
/// The parser:
/// 1. Strips quotes, parses the size number from the end
/// 2. Checks the remaining text for known weight modifiers
/// 3. Family = everything before the weight modifier (or before the size if none)
/// 4. Default weight = 400 (Regular) if no modifier found
///
/// Returns `None` if the string is empty or cannot be parsed.
pub(crate) fn parse_gnome_font_to_fontspec(s: &str) -> Option<FontSpec> {
    let trimmed = s.trim().trim_matches('\'');
    if trimmed.is_empty() {
        return None;
    }

    // Parse size from the end
    let last_space = trimmed.rfind(' ')?;
    let before_size = trimmed[..last_space].trim();
    let size_str = &trimmed[last_space + 1..];
    let size: f32 = size_str.parse().ok()?;
    if before_size.is_empty() || size <= 0.0 {
        return None;
    }

    // Check for weight modifiers at the end of the family string
    let (family, weight) = extract_weight_from_family(before_size);

    if family.is_empty() {
        return None;
    }

    Some(FontSpec {
        family: Some(family.to_string()),
        size: Some(size),
        weight: Some(weight),
        ..Default::default()
    })
}

/// Extract a weight modifier from the end of a family string.
///
/// Returns (family_without_modifier, css_weight). If no modifier is found,
/// returns the full string as family with weight 400.
fn extract_weight_from_family(s: &str) -> (&str, u16) {
    for &(modifier, weight) in WEIGHT_MODIFIERS {
        if let Some(prefix) = s.strip_suffix(modifier) {
            let family = prefix.trim_end();
            if !family.is_empty() {
                return (family, weight);
            }
        }
    }
    // No recognized modifier -- treat as Regular
    (s, 400)
}

/// Read a single gsettings value, returning None if gsettings is unavailable or fails.
fn read_gsetting(schema: &str, key: &str) -> Option<String> {
    let output = std::process::Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim().trim_matches('\'').to_string();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed)
}

/// Build a sparse ThemeVariant populated only with OS-readable fields.
///
/// This function does NOT embed any Adwaita preset data -- it only sets
/// fields that the GNOME desktop provides via gsettings and portal data.
/// The caller merges this sparse variant onto an Adwaita base.
pub(crate) fn build_gnome_variant(
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
    reduced_motion: Option<ReducedMotion>,
) -> crate::ThemeVariant {
    let mut variant = crate::ThemeVariant::default();

    // Apply accent color from portal if valid
    if let Some(color) = accent
        && let Some(rgba) = portal_color_to_rgba(&color)
    {
        apply_accent(&mut variant, &rgba);
    }

    // High contrast: portal first (GNOME-05)
    if matches!(contrast, Contrast::High) {
        variant.defaults.high_contrast = Some(true);
    }
    // gsettings fallback for high-contrast (covers GNOME < 44 without portal support)
    if variant.defaults.high_contrast.is_none()
        && let Some(hc_str) = read_gsetting("org.gnome.desktop.a11y.interface", "high-contrast")
    {
        match hc_str.as_str() {
            "true" => variant.defaults.high_contrast = Some(true),
            "false" => variant.defaults.high_contrast = Some(false),
            _ => {}
        }
    }

    // ── Fonts (GNOME-01) ────────────────────────────────────────────────
    // Primary UI font
    if let Some(font_str) = read_gsetting("org.gnome.desktop.interface", "font-name")
        && let Some(fs) = parse_gnome_font_to_fontspec(&font_str)
    {
        variant.defaults.font = fs;
    }

    // Monospace font
    if let Some(mono_str) = read_gsetting("org.gnome.desktop.interface", "monospace-font-name")
        && let Some(fs) = parse_gnome_font_to_fontspec(&mono_str)
    {
        variant.defaults.mono_font = fs;
    }

    // Titlebar font (GNOME-01 extension)
    if let Some(tb_str) = read_gsetting("org.gnome.desktop.wm.preferences", "titlebar-font")
        && let Some(fs) = parse_gnome_font_to_fontspec(&tb_str)
    {
        variant.window.title_bar_font = Some(fs);
    }

    // ── Text scale (GNOME-02) ──────────────────────────────────────────
    // Compute text scale entries from the base font size using CSS percentage multipliers
    if let Some(base_size) = variant.defaults.font.size {
        variant.text_scale = compute_text_scale(base_size);
    }

    // ── Accessibility (GNOME-03 / GNOME-05) ─────────────────────────────
    // Text scaling factor
    if let Some(factor_str) = read_gsetting("org.gnome.desktop.interface", "text-scaling-factor")
        && let Ok(factor) = factor_str.parse::<f32>()
    {
        variant.defaults.text_scaling_factor = Some(factor);
    }

    // reduce_motion: portal first, gsettings fallback (GNOME-05)
    if let Some(rm) = reduced_motion {
        match rm {
            ReducedMotion::ReducedMotion => variant.defaults.reduce_motion = Some(true),
            ReducedMotion::NoPreference => variant.defaults.reduce_motion = Some(false),
        }
    }
    // gsettings fallback: enable-animations (only if portal didn't provide a value)
    if variant.defaults.reduce_motion.is_none()
        && let Some(anim_str) = read_gsetting("org.gnome.desktop.interface", "enable-animations")
    {
        match anim_str.as_str() {
            "false" => variant.defaults.reduce_motion = Some(true),
            "true" => variant.defaults.reduce_motion = Some(false),
            _ => {}
        }
    }

    // overlay-scrolling -> scrollbar.overlay_mode (GNOME-03)
    if let Some(overlay_str) = read_gsetting("org.gnome.desktop.interface", "overlay-scrolling") {
        match overlay_str.as_str() {
            "true" => variant.scrollbar.overlay_mode = Some(true),
            "false" => variant.scrollbar.overlay_mode = Some(false),
            _ => {}
        }
    }

    // ── Icon theme (GNOME-04) ────────────────────────────────────────────
    if let Some(icon_theme) = read_gsetting("org.gnome.desktop.interface", "icon-theme") {
        variant.icon_theme = Some(icon_theme);
    }

    // ── Dialog button order (project decision) ──────────────────────────
    variant.dialog.button_order = Some(DialogButtonOrder::TrailingAffirmative);

    // Color scheme tag for the variant (not a field, but used for merge decision)
    let _ = scheme; // consumed by caller for light/dark selection

    variant
}

/// Compute text scale entries from a base font size using CSS percentage multipliers.
///
/// GNOME/Adwaita CSS type scale:
/// - caption: 82% of base
/// - dialog_title: 136% of base
/// - display: 181% of base
fn compute_text_scale(base_size: f32) -> TextScale {
    TextScale {
        caption: Some(TextScaleEntry {
            size: Some(base_size * 0.82),
            weight: None, // inherits from base preset / defaults.font.weight
            line_height: None,
        }),
        section_heading: None,
        dialog_title: Some(TextScaleEntry {
            size: Some(base_size * 1.36),
            weight: None, // comes from adwaita.toml
            line_height: None,
        }),
        display: Some(TextScaleEntry {
            size: Some(base_size * 1.81),
            weight: None, // comes from adwaita.toml
            line_height: None,
        }),
    }
}

/// Build a ThemeSpec from an Adwaita base, applying portal-provided
/// color scheme, accent color, contrast, and reduced-motion settings
/// plus OS-readable fields.
///
/// This is the testable core -- no D-Bus required.
pub(crate) fn build_theme(
    base: crate::ThemeSpec,
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
    reduced_motion: Option<ReducedMotion>,
) -> crate::Result<crate::ThemeSpec> {
    let is_dark = matches!(scheme, ColorScheme::PreferDark);

    // Pick the appropriate variant from the Adwaita base.
    // unwrap_or_default() provides an empty ThemeVariant when the base
    // preset lacks the requested variant, allowing the merge to proceed.
    let mut variant = if is_dark {
        base.dark.unwrap_or_default()
    } else {
        base.light.unwrap_or_default()
    };

    // Build sparse OS variant and merge onto Adwaita base
    let os_variant = build_gnome_variant(scheme, accent, contrast, reduced_motion);
    variant.merge(&os_variant);

    // Build ThemeSpec with only the selected variant populated
    let theme = if is_dark {
        crate::ThemeSpec {
            name: "GNOME".to_string(),
            light: None,
            dark: Some(variant),
            layout: crate::LayoutTheme::default(),
        }
    } else {
        crate::ThemeSpec {
            name: "GNOME".to_string(),
            light: Some(variant),
            dark: None,
            layout: crate::LayoutTheme::default(),
        }
    };

    Ok(theme)
}

/// Read the current GNOME theme from the XDG Desktop Portal.
///
/// Reads color scheme (light/dark), accent color, and contrast preference
/// from the `org.freedesktop.appearance` portal namespace via ashpd.
///
/// Falls back to bundled Adwaita defaults when the portal is unavailable
/// (no D-Bus session, sandboxed environment, or old portal version).
pub async fn from_gnome() -> crate::Result<crate::ThemeSpec> {
    let base = crate::ThemeSpec::preset("adwaita")?;

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

/// Read KDE theme from kdeglobals, then overlay portal accent color if available.
///
/// Reads the KDE kdeglobals file as the base theme via [`crate::kde::from_kde()`],
/// then attempts to read the accent color from the XDG Desktop Portal. If the
/// portal provides a valid accent color, it is applied to accent, selection,
/// and focus_ring_color fields via [`crate::ThemeSpec::merge`].
///
/// Falls back to the KDE-only base if the portal is unavailable or provides
/// no accent color.
///
/// Requires both `kde` and `portal` features.
#[cfg(feature = "kde")]
pub async fn from_kde_with_portal() -> crate::Result<crate::ThemeSpec> {
    let mut base = crate::kde::from_kde()?;

    // Try to get accent color from portal
    let settings = match ashpd::desktop::settings::Settings::new().await {
        Ok(s) => s,
        Err(_) => return Ok(base),
    };

    let accent = match settings.accent_color().await {
        Ok(color) => color,
        Err(_) => return Ok(base),
    };

    let rgba = match portal_color_to_rgba(&accent) {
        Some(r) => r,
        None => return Ok(base),
    };

    // Build overlay with accent applied to the same variant(s) as base
    let mut overlay = crate::ThemeSpec::new("");

    if base.light.is_some() {
        let mut variant = crate::ThemeVariant::default();
        apply_accent(&mut variant, &rgba);
        overlay.light = Some(variant);
    }
    if base.dark.is_some() {
        let mut variant = crate::ThemeVariant::default();
        apply_accent(&mut variant, &rgba);
        overlay.dark = Some(variant);
    }

    base.merge(&overlay);
    Ok(base)
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
        assert_eq!(fs.size, Some(11.0));
        assert_eq!(fs.weight, Some(700));
    }

    #[test]
    fn fontspec_light_weight() {
        let fs = parse_gnome_font_to_fontspec("Inter Light 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.size, Some(10.0));
        assert_eq!(fs.weight, Some(300));
    }

    #[test]
    fn fontspec_semi_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Semi-Bold 12").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(12.0));
        assert_eq!(fs.weight, Some(600));
    }

    #[test]
    fn fontspec_no_modifier_defaults_to_regular() {
        let fs = parse_gnome_font_to_fontspec("Cantarell 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Cantarell"));
        assert_eq!(fs.size, Some(11.0));
        assert_eq!(fs.weight, Some(400));
    }

    #[test]
    fn fontspec_medium_weight() {
        let fs = parse_gnome_font_to_fontspec("Inter Medium 10.5").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Inter"));
        assert_eq!(fs.size, Some(10.5));
        assert_eq!(fs.weight, Some(500));
    }

    #[test]
    fn fontspec_heavy_weight() {
        let fs = parse_gnome_font_to_fontspec("'Fira Sans Heavy 14'").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Fira Sans"));
        assert_eq!(fs.size, Some(14.0));
        assert_eq!(fs.weight, Some(900));
    }

    #[test]
    fn fontspec_thin_weight() {
        let fs = parse_gnome_font_to_fontspec("Roboto Thin 12").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Roboto"));
        assert_eq!(fs.size, Some(12.0));
        assert_eq!(fs.weight, Some(100));
    }

    #[test]
    fn fontspec_extra_bold_weight() {
        let fs = parse_gnome_font_to_fontspec("Source Sans Extra-Bold 11").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Source Sans"));
        assert_eq!(fs.size, Some(11.0));
        assert_eq!(fs.weight, Some(800));
    }

    #[test]
    fn fontspec_ultra_light_weight() {
        let fs = parse_gnome_font_to_fontspec("Noto Sans Ultra-Light 10").unwrap();
        assert_eq!(fs.family.as_deref(), Some("Noto Sans"));
        assert_eq!(fs.size, Some(10.0));
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
        let v = build_gnome_variant(
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        );
        assert_eq!(
            v.dialog.button_order,
            Some(DialogButtonOrder::TrailingAffirmative),
        );
    }

    #[test]
    fn build_gnome_variant_high_contrast_sets_flag() {
        let v = build_gnome_variant(ColorScheme::NoPreference, None, Contrast::High, None);
        assert_eq!(v.defaults.high_contrast, Some(true));
    }

    #[test]
    fn build_gnome_variant_normal_contrast_no_flag() {
        let v = build_gnome_variant(
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        );
        assert!(v.defaults.high_contrast.is_none());
    }

    #[test]
    fn build_gnome_variant_accent_sets_defaults() {
        let accent = Color::new(0.2, 0.4, 0.8);
        let v = build_gnome_variant(
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
            None,
        );
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);
        assert_eq!(v.defaults.accent, Some(expected));
        assert_eq!(v.defaults.selection, Some(expected));
        assert_eq!(v.defaults.focus_ring_color, Some(expected));
    }

    #[test]
    fn build_gnome_variant_invalid_accent_stays_none() {
        let accent = Color::new(1.5, 0.0, 0.0); // out of range
        let v = build_gnome_variant(
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
            None,
        );
        assert!(v.defaults.accent.is_none());
    }

    // === compute_text_scale tests ===

    #[test]
    fn text_scale_caption_from_base() {
        let ts = compute_text_scale(11.0);
        let cap = ts.caption.as_ref().unwrap();
        let expected = 11.0 * 0.82;
        assert!(
            (cap.size.unwrap() - expected).abs() < 0.01,
            "caption size: expected {expected}, got {:?}",
            cap.size
        );
        assert!(
            cap.weight.is_none(),
            "caption weight should inherit from preset"
        );
        assert!(cap.line_height.is_none());
    }

    #[test]
    fn text_scale_dialog_title_from_base() {
        let ts = compute_text_scale(11.0);
        let dt = ts.dialog_title.as_ref().unwrap();
        let expected = 11.0 * 1.36;
        assert!(
            (dt.size.unwrap() - expected).abs() < 0.01,
            "dialog_title size: expected {expected}, got {:?}",
            dt.size
        );
        assert!(dt.weight.is_none()); // comes from adwaita.toml
    }

    #[test]
    fn text_scale_display_from_base() {
        let ts = compute_text_scale(11.0);
        let d = ts.display.as_ref().unwrap();
        let expected = 11.0 * 1.81;
        assert!(
            (d.size.unwrap() - expected).abs() < 0.01,
            "display size: expected {expected}, got {:?}",
            d.size
        );
        assert!(d.weight.is_none()); // comes from adwaita.toml
    }

    #[test]
    fn text_scale_section_heading_is_none() {
        let ts = compute_text_scale(11.0);
        assert!(ts.section_heading.is_none());
    }

    // === build_theme tests ===

    fn adwaita_base() -> crate::ThemeSpec {
        crate::ThemeSpec::preset("adwaita").unwrap()
    }

    #[test]
    fn dark_scheme_produces_dark_variant_only() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::PreferDark,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.dark.is_some(), "dark variant should be Some");
        assert!(theme.light.is_none(), "light variant should be None");
    }

    #[test]
    fn light_scheme_produces_light_variant_only() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::PreferLight,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    #[test]
    fn no_preference_defaults_to_light() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();
        assert_eq!(theme.name, "GNOME");
        assert!(theme.light.is_some(), "light variant should be Some");
        assert!(theme.dark.is_none(), "dark variant should be None");
    }

    // === accent color tests ===

    #[test]
    fn valid_accent_propagates_to_three_fields() {
        let accent = Color::new(0.2, 0.4, 0.8);
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            Some(accent),
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        let expected = crate::Rgba::from_f32(0.2, 0.4, 0.8, 1.0);

        assert_eq!(variant.defaults.accent, Some(expected));
        assert_eq!(variant.defaults.selection, Some(expected));
        assert_eq!(variant.defaults.focus_ring_color, Some(expected));
    }

    // === high contrast tests ===

    #[test]
    fn high_contrast_sets_flag_on_theme_variant() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::High,
            None,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(variant.defaults.high_contrast, Some(true));
    }

    #[test]
    fn normal_contrast_preserves_adwaita_default() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        // Adwaita preset sets high_contrast = false; OS variant doesn't override it
        assert_eq!(variant.defaults.high_contrast, Some(false));
    }

    // === build_theme merge correctness ===

    #[test]
    fn build_theme_preserves_adwaita_colors_when_no_overrides() {
        let base = adwaita_base();
        let base_light = base.light.as_ref().unwrap().clone();

        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        // Adwaita colors should be preserved since OS variant doesn't set colors
        assert_eq!(variant.defaults.background, base_light.defaults.background);
        assert_eq!(variant.defaults.foreground, base_light.defaults.foreground);
    }

    #[test]
    fn build_theme_dialog_order_set() {
        let theme = build_theme(
            adwaita_base(),
            ColorScheme::NoPreference,
            None,
            Contrast::NoPreference,
            None,
        )
        .unwrap();

        let variant = theme.light.as_ref().expect("light variant");
        assert_eq!(
            variant.dialog.button_order,
            Some(DialogButtonOrder::TrailingAffirmative),
        );
    }

    // === extract_weight_from_family tests ===

    #[test]
    fn extract_weight_bold() {
        let (family, weight) = extract_weight_from_family("Cantarell Bold");
        assert_eq!(family, "Cantarell");
        assert_eq!(weight, 700);
    }

    #[test]
    fn extract_weight_no_modifier() {
        let (family, weight) = extract_weight_from_family("Cantarell");
        assert_eq!(family, "Cantarell");
        assert_eq!(weight, 400);
    }

    #[test]
    fn extract_weight_demi_bold() {
        let (family, weight) = extract_weight_from_family("Source Sans Demi-Bold");
        assert_eq!(family, "Source Sans");
        assert_eq!(weight, 600);
    }

    #[test]
    fn extract_weight_black() {
        let (family, weight) = extract_weight_from_family("Inter Black");
        assert_eq!(family, "Inter");
        assert_eq!(weight, 900);
    }
}
