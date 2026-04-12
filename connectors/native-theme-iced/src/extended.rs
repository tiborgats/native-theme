//! Extended palette overrides from [`native_theme::ResolvedTheme`] fields.
//!
//! After iced generates an `Extended` palette from the base `Palette`,
//! this module overrides specific sub-palette entries with native-theme
//! values. All fields are guaranteed populated in ResolvedTheme, so
//! overrides are always applied unconditionally.

use crate::palette::to_color;
use native_theme::Rgba;

/// Captured color values for Extended palette overrides.
///
/// Holds the `Rgba` values extracted from `ResolvedTheme` that
/// `to_theme()` captures into its closure. Using a struct instead of
/// individual parameters keeps the API clean.
///
/// The `success_bg`, `danger_bg`, and `warning_bg` fields are needed for
/// WCAG contrast enforcement: we check each status foreground against its
/// corresponding background to ensure 4.5:1 contrast.
#[derive(Clone, Copy)]
pub(crate) struct OverrideColors {
    pub btn_bg: Rgba,
    pub btn_fg: Rgba,
    pub surface: Rgba,
    pub foreground: Rgba,
    pub accent_fg: Rgba,
    pub success_fg: Rgba,
    pub danger_fg: Rgba,
    pub warning_fg: Rgba,
    pub success_bg: Rgba,
    pub danger_bg: Rgba,
    pub warning_bg: Rgba,
}

/// Minimum WCAG contrast ratio for status foreground against its background.
/// 4.5:1 is AA for normal text.
const MIN_STATUS_CONTRAST: f32 = 4.5;

/// WCAG 2.1 relative luminance from an iced Color.
///
/// Uses sRGB linearization and ITU-R BT.709 coefficients, matching the
/// algorithm in the gpui connector's `derive::relative_luminance()`.
fn relative_luminance(c: iced_core::Color) -> f32 {
    let linearize = |v: f32| -> f32 {
        let v = v.clamp(0.0, 1.0);
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linearize(c.r) + 0.7152 * linearize(c.g) + 0.0722 * linearize(c.b)
}

/// Compute the WCAG 2.1 contrast ratio between two colors.
///
/// Returns a value in [1.0, 21.0]. Ratios below 4.5 indicate insufficient
/// contrast for normal text (AA), below 3.0 for large text.
fn contrast_ratio(a: iced_core::Color, b: iced_core::Color) -> f32 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Ensure a status foreground color has sufficient contrast against its background.
///
/// If the foreground has less than 4.5:1 contrast against the background,
/// falls back to white (for dark backgrounds) or black (for light backgrounds).
///
/// Uses `relative_luminance(bg) < 0.5` instead of HSL lightness because iced
/// `Color` has no `.l` field, and luminance is more perceptually accurate for
/// determining whether a background is "dark" or "light".
fn ensure_status_contrast(fg: iced_core::Color, bg: iced_core::Color) -> iced_core::Color {
    if contrast_ratio(fg, bg) >= MIN_STATUS_CONTRAST {
        fg
    } else if relative_luminance(bg) < 0.5 {
        iced_core::Color::WHITE
    } else {
        iced_core::Color::BLACK
    }
}

/// Override auto-generated Extended palette entries with resolved theme fields.
///
/// Always applies these overrides (all fields guaranteed populated):
/// - `secondary.base.color` <- button background
/// - `secondary.base.text` <- button foreground
/// - `background.weak.color` <- surface color
/// - `background.weak.text` <- foreground text color
/// - `primary.base.text` <- accent foreground (text on accent bg)
/// - `success.base.text` <- success foreground (text on success bg, contrast-enforced)
/// - `danger.base.text` <- danger foreground (text on danger bg, contrast-enforced)
/// - `warning.base.text` <- warning foreground (text on warning bg, contrast-enforced)
///
/// Status foreground colors (success, danger, warning) are passed through
/// WCAG AA contrast enforcement: if the foreground has less than 4.5:1 contrast
/// against its status background, it falls back to white or black.
///
/// Note: `.base.color` overrides for primary/success/danger/warning are
/// redundant because `Extended::generate()` already sets them correctly
/// from the base palette. Only the `.base.text` fields need overriding
/// because the auto-generation uses `defaults.text_color` instead of the
/// per-status foreground colors.
pub(crate) fn apply_overrides(
    extended: &mut iced_core::theme::palette::Extended,
    colors: &OverrideColors,
) {
    extended.secondary.base.color = to_color(colors.btn_bg);
    extended.secondary.base.text = to_color(colors.btn_fg);
    extended.background.weak.color = to_color(colors.surface);
    extended.background.weak.text = to_color(colors.foreground);
    extended.primary.base.text = to_color(colors.accent_fg);
    extended.success.base.text =
        ensure_status_contrast(to_color(colors.success_fg), to_color(colors.success_bg));
    extended.danger.base.text =
        ensure_status_contrast(to_color(colors.danger_fg), to_color(colors.danger_bg));
    extended.warning.base.text =
        ensure_status_contrast(to_color(colors.warning_fg), to_color(colors.warning_bg));
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use iced_core::theme::palette::Extended;
    use native_theme::Theme;

    fn make_extended() -> Extended {
        let palette = iced_core::theme::Palette::DARK;
        Extended::generate(palette)
    }

    fn make_resolved_preset(name: &str, is_dark: bool) -> native_theme::ResolvedTheme {
        Theme::preset(name)
            .unwrap()
            .into_variant(is_dark)
            .unwrap()
            .into_resolved()
            .unwrap()
    }

    fn make_resolved(is_dark: bool) -> native_theme::ResolvedTheme {
        make_resolved_preset("catppuccin-mocha", is_dark)
    }

    fn colors_from_resolved(r: &native_theme::ResolvedTheme) -> OverrideColors {
        OverrideColors {
            btn_bg: r.button.background_color,
            btn_fg: r.button.font.color,
            surface: r.defaults.surface_color,
            foreground: r.defaults.text_color,
            accent_fg: r.defaults.accent_text_color,
            success_fg: r.defaults.success_text_color,
            danger_fg: r.defaults.danger_text_color,
            warning_fg: r.defaults.warning_text_color,
            success_bg: r.defaults.success_color,
            danger_bg: r.defaults.danger_color,
            warning_bg: r.defaults.warning_color,
        }
    }

    fn apply_from_resolved(ext: &mut Extended, r: &native_theme::ResolvedTheme) {
        apply_overrides(ext, &colors_from_resolved(r));
    }

    #[test]
    fn apply_overrides_sets_secondary_base_color() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.button.background_color);
        assert_eq!(
            extended.secondary.base.color, expected,
            "secondary.base.color should match resolved.button.background"
        );
    }

    #[test]
    fn apply_overrides_sets_secondary_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.button.font.color);
        assert_eq!(
            extended.secondary.base.text, expected,
            "secondary.base.text should match resolved.button.foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_background_weak_color() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.surface_color);
        assert_eq!(
            extended.background.weak.color, expected,
            "background.weak.color should match resolved.defaults.surface"
        );
    }

    #[test]
    fn apply_overrides_sets_background_weak_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.text_color);
        assert_eq!(
            extended.background.weak.text, expected,
            "background.weak.text should match resolved.defaults.text_color"
        );
    }

    #[test]
    fn apply_overrides_sets_primary_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.accent_text_color);
        assert_eq!(
            extended.primary.base.text, expected,
            "primary.base.text should match resolved.defaults.accent_foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_success_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = super::ensure_status_contrast(
            to_color(resolved.defaults.success_text_color),
            to_color(resolved.defaults.success_color),
        );
        assert_eq!(
            extended.success.base.text, expected,
            "success.base.text should match contrast-enforced success foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_danger_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        // The raw danger foreground may be contrast-corrected if it has
        // insufficient contrast against the danger background. Compute
        // the expected value through the same enforcement path.
        let expected = super::ensure_status_contrast(
            to_color(resolved.defaults.danger_text_color),
            to_color(resolved.defaults.danger_color),
        );
        assert_eq!(
            extended.danger.base.text, expected,
            "danger.base.text should match contrast-enforced danger foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_warning_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = super::ensure_status_contrast(
            to_color(resolved.defaults.warning_text_color),
            to_color(resolved.defaults.warning_color),
        );
        assert_eq!(
            extended.warning.base.text, expected,
            "warning.base.text should match contrast-enforced warning foreground"
        );
    }

    #[test]
    fn apply_overrides_dark_variant() {
        let mut extended = make_extended();
        let resolved = make_resolved(true);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.button.background_color);
        assert_eq!(
            extended.secondary.base.color, expected,
            "dark variant: secondary.base.color should match"
        );
    }

    #[test]
    fn apply_overrides_multiple_presets() {
        for name in ["catppuccin-mocha", "dracula", "nord"] {
            let resolved = Theme::preset(name)
                .unwrap()
                .into_variant(true)
                .unwrap()
                .into_resolved()
                .unwrap();
            let mut extended = make_extended();
            apply_from_resolved(&mut extended, &resolved);

            assert_eq!(
                extended.secondary.base.color,
                to_color(resolved.button.background_color),
                "{name}: secondary.base.color mismatch"
            );
        }
    }

    #[test]
    fn apply_overrides_with_adwaita() {
        let resolved = make_resolved_preset("adwaita", false);
        let mut extended = make_extended();
        apply_from_resolved(&mut extended, &resolved);

        assert_eq!(
            extended.secondary.base.color,
            to_color(resolved.button.background_color),
            "adwaita: secondary.base.color mismatch"
        );
        assert_eq!(
            extended.primary.base.text,
            to_color(resolved.defaults.accent_text_color),
            "adwaita: primary.base.text mismatch"
        );
    }

    #[test]
    fn ensure_status_contrast_corrects_low_contrast() {
        // Dark background with dark foreground = low contrast
        let dark_bg = iced_core::Color::from_rgb(0.1, 0.1, 0.1);
        let dark_fg = iced_core::Color::from_rgb(0.15, 0.15, 0.15);
        let result = super::ensure_status_contrast(dark_fg, dark_bg);
        // Should fall back to white since bg is dark
        assert_eq!(result, iced_core::Color::WHITE);

        // Light background with light foreground = low contrast
        let light_bg = iced_core::Color::from_rgb(0.9, 0.9, 0.9);
        let light_fg = iced_core::Color::from_rgb(0.85, 0.85, 0.85);
        let result = super::ensure_status_contrast(light_fg, light_bg);
        // Should fall back to black since bg is light
        assert_eq!(result, iced_core::Color::BLACK);
    }

    #[test]
    fn ensure_status_contrast_preserves_sufficient() {
        let bg = iced_core::Color::from_rgb(0.1, 0.1, 0.1);
        let fg = iced_core::Color::WHITE;
        let result = super::ensure_status_contrast(fg, bg);
        assert_eq!(result, fg, "sufficient contrast should preserve original");
    }

    #[test]
    fn contrast_ratio_black_white() {
        let ratio = super::contrast_ratio(iced_core::Color::BLACK, iced_core::Color::WHITE);
        assert!(
            ratio > 20.0,
            "black/white contrast should be ~21, got {ratio}"
        );
    }
}
