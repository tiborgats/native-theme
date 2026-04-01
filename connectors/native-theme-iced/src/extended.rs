//! Extended palette overrides from [`native_theme::ResolvedThemeVariant`] fields.
//!
//! After iced generates an `Extended` palette from the base `Palette`,
//! this module overrides specific sub-palette entries with native-theme
//! values. All fields are guaranteed populated in ResolvedThemeVariant, so
//! overrides are always applied unconditionally.

use crate::palette::to_color;

/// Override auto-generated Extended palette entries with resolved theme fields.
///
/// Always applies these overrides (all fields guaranteed populated):
/// - `secondary.base.color` <- `resolved.button.background`
/// - `secondary.base.text` <- `resolved.button.foreground`
/// - `background.weak.color` <- `resolved.defaults.surface`
/// - `background.weak.text` <- `resolved.defaults.foreground`
pub fn apply_overrides(
    extended: &mut iced_core::theme::palette::Extended,
    resolved: &native_theme::ResolvedThemeVariant,
) {
    extended.secondary.base.color = to_color(resolved.button.background);
    extended.secondary.base.text = to_color(resolved.button.foreground);
    extended.background.weak.color = to_color(resolved.defaults.surface);
    extended.background.weak.text = to_color(resolved.defaults.foreground);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use iced_core::theme::palette::Extended;
    use native_theme::ThemeSpec;

    fn color_approx_eq(a: iced_core::Color, b: iced_core::Color) -> bool {
        (a.r - b.r).abs() < 0.01
            && (a.g - b.g).abs() < 0.01
            && (a.b - b.b).abs() < 0.01
            && (a.a - b.a).abs() < 0.01
    }

    fn make_extended() -> Extended {
        let palette = iced_core::theme::Palette::DARK;
        Extended::generate(palette)
    }

    fn make_resolved() -> native_theme::ResolvedThemeVariant {
        ThemeSpec::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(false)
            .unwrap()
            .into_resolved()
            .unwrap()
    }

    #[test]
    fn apply_overrides_sets_secondary_base_color() {
        let mut extended = make_extended();
        let resolved = make_resolved();
        let original = extended.secondary.base.color;

        apply_overrides(&mut extended, &resolved);

        // Should have been overridden (button.background from resolved)
        let expected = to_color(resolved.button.background);
        assert!(
            color_approx_eq(extended.secondary.base.color, expected),
            "secondary.base.color should match resolved.button.background"
        );
        // Should differ from original (DARK palette)
        assert_ne!(
            extended.secondary.base.color, original,
            "should have changed from original"
        );
    }

    #[test]
    fn apply_overrides_sets_secondary_base_text() {
        let mut extended = make_extended();
        let resolved = make_resolved();

        apply_overrides(&mut extended, &resolved);

        let expected = to_color(resolved.button.foreground);
        assert!(
            color_approx_eq(extended.secondary.base.text, expected),
            "secondary.base.text should match resolved.button.foreground"
        );
    }

    #[test]
    fn apply_overrides_sets_background_weak_color() {
        let mut extended = make_extended();
        let resolved = make_resolved();

        apply_overrides(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.surface);
        assert!(
            color_approx_eq(extended.background.weak.color, expected),
            "background.weak.color should match resolved.defaults.surface"
        );
    }

    #[test]
    fn apply_overrides_sets_background_weak_text() {
        let mut extended = make_extended();
        let resolved = make_resolved();

        apply_overrides(&mut extended, &resolved);

        let expected = to_color(resolved.defaults.foreground);
        assert!(
            color_approx_eq(extended.background.weak.text, expected),
            "background.weak.text should match resolved.defaults.foreground"
        );
    }
}
