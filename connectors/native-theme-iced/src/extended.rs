//! Extended palette overrides from [`native_theme::ThemeVariant`] colors.
//!
//! After iced generates an `Extended` palette from the base `Palette`,
//! this module overrides specific sub-palette entries with native-theme
//! values where available.

use crate::palette::to_color;

/// Override auto-generated Extended palette entries with native-theme colors.
///
/// Applies these overrides when the corresponding native-theme field is `Some`:
/// - `secondary.base.color` <- `colors.secondary_background`
/// - `secondary.base.text` <- `colors.secondary_foreground`
/// - `background.weak.color` <- `colors.surface`
/// - `background.weak.text` <- `colors.foreground`
pub fn apply_overrides(
    extended: &mut iced_core::theme::palette::Extended,
    variant: &native_theme::ThemeVariant,
) {
    let c = &variant.colors;

    if let Some(secondary_bg) = c.secondary_background {
        extended.secondary.base.color =
            to_color(Some(secondary_bg), extended.secondary.base.color);
    }
    if let Some(secondary_fg) = c.secondary_foreground {
        extended.secondary.base.text =
            to_color(Some(secondary_fg), extended.secondary.base.text);
    }
    if let Some(surface) = c.surface {
        extended.background.weak.color =
            to_color(Some(surface), extended.background.weak.color);
    }
    if let Some(fg) = c.foreground {
        extended.background.weak.text =
            to_color(Some(fg), extended.background.weak.text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::theme::palette::Extended;
    use native_theme::{Rgba, ThemeVariant};

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

    #[test]
    fn apply_overrides_sets_secondary_base_color() {
        let mut extended = make_extended();
        let mut variant = ThemeVariant::default();
        variant.colors.secondary_background = Some(Rgba::rgb(100, 150, 200));

        apply_overrides(&mut extended, &variant);

        assert!(color_approx_eq(
            extended.secondary.base.color,
            iced_core::Color::from_rgba(100.0 / 255.0, 150.0 / 255.0, 200.0 / 255.0, 1.0)
        ));
    }

    #[test]
    fn apply_overrides_sets_secondary_base_text() {
        let mut extended = make_extended();
        let mut variant = ThemeVariant::default();
        variant.colors.secondary_foreground = Some(Rgba::rgb(240, 240, 240));

        apply_overrides(&mut extended, &variant);

        assert!(color_approx_eq(
            extended.secondary.base.text,
            iced_core::Color::from_rgba(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0, 1.0)
        ));
    }

    #[test]
    fn apply_overrides_sets_background_weak_color() {
        let mut extended = make_extended();
        let mut variant = ThemeVariant::default();
        variant.colors.surface = Some(Rgba::rgb(50, 50, 50));

        apply_overrides(&mut extended, &variant);

        assert!(color_approx_eq(
            extended.background.weak.color,
            iced_core::Color::from_rgba(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 1.0)
        ));
    }

    #[test]
    fn apply_overrides_sets_background_weak_text() {
        let mut extended = make_extended();
        let mut variant = ThemeVariant::default();
        variant.colors.foreground = Some(Rgba::rgb(200, 200, 200));

        apply_overrides(&mut extended, &variant);

        assert!(color_approx_eq(
            extended.background.weak.text,
            iced_core::Color::from_rgba(200.0 / 255.0, 200.0 / 255.0, 200.0 / 255.0, 1.0)
        ));
    }

    #[test]
    fn apply_overrides_leaves_unchanged_when_all_none() {
        let mut extended = make_extended();
        let original_secondary_color = extended.secondary.base.color;
        let original_secondary_text = extended.secondary.base.text;
        let original_bg_weak_color = extended.background.weak.color;
        let original_bg_weak_text = extended.background.weak.text;

        let variant = ThemeVariant::default(); // all None

        apply_overrides(&mut extended, &variant);

        assert_eq!(extended.secondary.base.color, original_secondary_color);
        assert_eq!(extended.secondary.base.text, original_secondary_text);
        assert_eq!(extended.background.weak.color, original_bg_weak_color);
        assert_eq!(extended.background.weak.text, original_bg_weak_text);
    }
}
