//! Extended palette overrides from [`native_theme::ResolvedThemeVariant`] fields.
//!
//! After iced generates an `Extended` palette from the base `Palette`,
//! this module overrides specific sub-palette entries with native-theme
//! values. All fields are guaranteed populated in ResolvedThemeVariant, so
//! overrides are always applied unconditionally.

use crate::palette::to_color;
use native_theme::Rgba;

/// Captured color values for Extended palette overrides.
///
/// Holds the 8 `Rgba` values extracted from `ResolvedThemeVariant` that
/// `to_theme()` captures into its closure. Using a struct instead of 8
/// individual parameters keeps the API clean.
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
}

/// Override auto-generated Extended palette entries with resolved theme fields.
///
/// Always applies these overrides (all fields guaranteed populated):
/// - `secondary.base.color` <- button background
/// - `secondary.base.text` <- button foreground
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
/// because the auto-generation uses `defaults.foreground` instead of the
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
    extended.success.base.text = to_color(colors.success_fg);
    extended.danger.base.text = to_color(colors.danger_fg);
    extended.warning.base.text = to_color(colors.warning_fg);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use iced_core::theme::palette::Extended;
    use native_theme::ThemeSpec;

    fn make_extended() -> Extended {
        let palette = iced_core::theme::Palette::DARK;
        Extended::generate(palette)
    }

    fn make_resolved(is_dark: bool) -> native_theme::ResolvedThemeVariant {
        ThemeSpec::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(is_dark)
            .unwrap()
            .into_resolved()
            .unwrap()
    }

    fn colors_from_resolved(r: &native_theme::ResolvedThemeVariant) -> OverrideColors {
        OverrideColors {
            btn_bg: r.button.background,
            btn_fg: r.button.foreground,
            surface: r.defaults.surface,
            foreground: r.defaults.foreground,
            accent_fg: r.defaults.accent_foreground,
            success_fg: r.defaults.success_foreground,
            danger_fg: r.defaults.danger_foreground,
            warning_fg: r.defaults.warning_foreground,
        }
    }

    fn apply_from_resolved(ext: &mut Extended, r: &native_theme::ResolvedThemeVariant) {
        apply_overrides(ext, &colors_from_resolved(r));
    }

    #[test]
    fn apply_overrides_sets_secondary_base_color() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.button.background);
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

        let expected = to_color(resolved.button.foreground);
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

        let expected = to_color(resolved.defaults.surface);
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

        let expected = to_color(resolved.defaults.foreground);
        assert_eq!(
            extended.background.weak.text, expected,
            "background.weak.text should match resolved.defaults.foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_primary_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.accent_foreground);
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

        let expected = to_color(resolved.defaults.success_foreground);
        assert_eq!(
            extended.success.base.text, expected,
            "success.base.text should match resolved.defaults.success_foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_danger_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.danger_foreground);
        assert_eq!(
            extended.danger.base.text, expected,
            "danger.base.text should match resolved.defaults.danger_foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_warning_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved(false);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.warning_foreground);
        assert_eq!(
            extended.warning.base.text, expected,
            "warning.base.text should match resolved.defaults.warning_foreground"
        );
    }

    #[test]
    fn apply_overrides_dark_variant() {
        let mut extended = make_extended();
        let resolved = make_resolved(true);

        apply_from_resolved(&mut extended, &resolved);

        let expected = to_color(resolved.button.background);
        assert_eq!(
            extended.secondary.base.color, expected,
            "dark variant: secondary.base.color should match"
        );
    }

    #[test]
    fn apply_overrides_multiple_presets() {
        for name in ["catppuccin-mocha", "dracula", "nord"] {
            let resolved = ThemeSpec::preset(name)
                .unwrap()
                .into_variant(true)
                .unwrap()
                .into_resolved()
                .unwrap();
            let mut extended = make_extended();
            apply_from_resolved(&mut extended, &resolved);

            assert_eq!(
                extended.secondary.base.color,
                to_color(resolved.button.background),
                "{name}: secondary.base.color mismatch"
            );
        }
    }
}
