//! Extended palette overrides from [`native_theme::theme::ResolvedTheme`] fields.
//!
//! After iced generates an `Extended` palette from the base `Palette`,
//! this module overrides specific sub-palette entries with native-theme
//! values. All fields are guaranteed populated in ResolvedTheme, so
//! overrides are always applied unconditionally.

use crate::palette::to_color;
use iced_core::theme::palette::Pair;
use native_theme::color::Rgba;

/// Captured color values for Extended palette overrides.
///
/// Holds the `Rgba` values extracted from `ResolvedTheme` that
/// `to_theme()` captures into its closure. Using a struct instead of
/// individual parameters keeps the API clean.
#[derive(Clone, Copy)]
pub(crate) struct OverrideColors {
    pub placeholder: Rgba,
    pub surface: Rgba,
    pub foreground: Rgba,
    pub accent_fg: Rgba,
    pub success_fg: Rgba,
    pub danger_fg: Rgba,
    pub warning_fg: Rgba,
}

/// WCAG 2.1 relative luminance from an iced Color.
///
/// Uses sRGB linearization and ITU-R BT.709 coefficients, matching the
/// algorithm in the gpui connector's `derive::relative_luminance()`.
#[cfg(test)]
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
///
/// Test-only: the mapping contract's contrast report is its only reader --
/// nothing in the emitted palette is chosen by measuring contrast.
#[cfg(test)]
pub(crate) fn contrast_ratio(a: iced_core::Color, b: iced_core::Color) -> f32 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Override auto-generated Extended palette entries with resolved theme fields.
///
/// Always applies these overrides (all fields guaranteed populated):
/// - `background.base.text` <- foreground text color
/// - `secondary.base` <- placeholder color, labelled by the window's text
/// - `secondary.strong` <- a copy of `secondary.base`
/// - `background.weak.color` <- surface color
/// - `background.weak.text` <- foreground text color
/// - `primary.base.text` <- accent foreground (text on accent bg)
/// - `success.base.text` <- success foreground (text on success bg)
/// - `danger.base.text` <- danger foreground (text on danger bg)
/// - `warning.base.text` <- warning foreground (text on warning bg)
///
/// Note: `.base.color` overrides for primary/success/danger/warning are
/// redundant because `Extended::generate()` already sets them correctly
/// from the base palette. Only the `.base.text` fields need overriding
/// because the auto-generation uses `defaults.text_color` instead of the
/// per-status foreground colors. `background.base.text` is overridden for a
/// different reason: the auto-generation does start from `defaults.text_color`
/// there, but passes it through `readable()`, which swaps it for one of its own
/// whenever the platform's pair falls below iced's contrast threshold.
pub(crate) fn apply_overrides(
    extended: &mut iced_core::theme::palette::Extended,
    colors: &OverrideColors,
) {
    // What iced paints inherited text with: `Base::base` reads this slot for
    // every widget that states no color of its own, and `Extended::generate`
    // passes the platform's text color through `readable()`, which substitutes
    // another whenever the contrast falls below its own threshold.
    extended.background.base.text = to_color(colors.foreground);
    // Ordering: the label is read from `background.base.text`, which the line
    // above has just set to the platform's, so the pair is labelled natively.
    extended.secondary.base =
        Pair::new(to_color(colors.placeholder), extended.background.base.text);
    // A hovered `button::secondary` keeps the base label and swaps only the
    // fill, so the fill must be one that label was chosen for.
    extended.secondary.strong = extended.secondary.base;
    extended.background.weak.color = to_color(colors.surface);
    extended.background.weak.text = to_color(colors.foreground);
    extended.primary.base.text = to_color(colors.accent_fg);
    extended.success.base.text = to_color(colors.success_fg);
    extended.danger.base.text = to_color(colors.danger_fg);
    extended.warning.base.text = to_color(colors.warning_fg);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ColorMode;
    use iced_core::theme::palette::Extended;
    use native_theme::theme::Theme;

    fn make_extended() -> Extended {
        let palette = iced_core::theme::Palette::DARK;
        Extended::generate(palette)
    }

    fn make_resolved_preset(name: &str, is_dark: bool) -> native_theme::theme::ResolvedTheme {
        Theme::preset(name)
            .unwrap()
            .into_variant(if is_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            })
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap()
    }

    fn make_resolved(is_dark: bool) -> native_theme::theme::ResolvedTheme {
        make_resolved_preset("catppuccin-mocha", is_dark)
    }

    fn colors_from_resolved(r: &native_theme::theme::ResolvedTheme) -> OverrideColors {
        OverrideColors {
            placeholder: r.input.placeholder_color,
            surface: r.defaults.surface_color,
            foreground: r.defaults.text_color,
            accent_fg: r.defaults.accent_text_color,
            success_fg: r.defaults.success_text_color,
            danger_fg: r.defaults.danger_text_color,
            warning_fg: r.defaults.warning_text_color,
        }
    }

    fn apply_from_resolved(ext: &mut Extended, r: &native_theme::theme::ResolvedTheme) {
        apply_overrides(ext, &colors_from_resolved(r));
    }

    #[test]
    fn apply_overrides_sets_secondary_base_color() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.input.placeholder_color);
        assert_eq!(
            extended.secondary.base.color, expected,
            "secondary.base.color should match resolved.input.placeholder"
        );
    }

    #[test]
    fn apply_overrides_copies_secondary_base_into_strong() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        assert_eq!(
            extended.secondary.strong, extended.secondary.base,
            "secondary.strong should be a copy of secondary.base, label included"
        );
    }

    #[test]
    fn apply_overrides_sets_background_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.text_color);
        assert_eq!(
            extended.background.base.text, expected,
            "background.base.text should match resolved.defaults.text_color"
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

        let expected = to_color(resolved.defaults.success_text_color);
        assert_eq!(
            extended.success.base.text, expected,
            "success.base.text should match the native success foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_danger_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.danger_text_color);
        assert_eq!(
            extended.danger.base.text, expected,
            "danger.base.text should match the native danger foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_warning_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.warning_text_color);
        assert_eq!(
            extended.warning.base.text, expected,
            "warning.base.text should match the native warning foreground"
        );
    }

    #[test]
    fn apply_overrides_dark_variant() {
        let mut extended = make_extended();
        let resolved = make_resolved(true);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.input.placeholder_color);
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
                .into_variant(ColorMode::Dark)
                .unwrap()
                .into_resolved(&native_theme::ResolutionContext::for_tests())
                .unwrap();
            let mut extended = make_extended();
            apply_from_resolved(&mut extended, &resolved);

            assert_eq!(
                extended.secondary.base.color,
                to_color(resolved.input.placeholder_color),
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
            to_color(resolved.input.placeholder_color),
            "adwaita: secondary.base.color mismatch"
        );
        assert_eq!(
            extended.primary.base.text,
            to_color(resolved.defaults.accent_text_color),
            "adwaita: primary.base.text mismatch"
        );
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
