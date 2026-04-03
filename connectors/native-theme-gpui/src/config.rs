//! ResolvedThemeVariant -> gpui_component::theme::ThemeConfig mapping.
//!
//! Maps native-theme's resolved font and geometry settings to gpui-component's
//! `ThemeConfig`, which controls per-theme font family, font size, radius,
//! and shadow settings.

use gpui::SharedString;
use gpui_component::theme::{ThemeConfig, ThemeMode};
use native_theme::ResolvedThemeVariant;

/// Build a [`ThemeConfig`] from a [`ResolvedThemeVariant`].
///
/// Maps ResolvedThemeDefaults font/geometry fields to font_family/mono_font_family/
/// font_size/mono_font_size, radius/radius_lg/shadow. IMPORTANT: ResolvedFontSpec
/// sizes are already in logical pixels -- no pt-to-px conversion is applied.
pub fn to_theme_config(
    resolved: &ResolvedThemeVariant,
    name: &str,
    mode: ThemeMode,
) -> ThemeConfig {
    let d = &resolved.defaults;

    // Issue 14: clamp radius to non-negative before rounding
    let radius = d.radius.max(0.0).round() as usize;
    let radius_lg = d.radius_lg.max(0.0).round() as usize;

    ThemeConfig {
        name: SharedString::from(name.to_string()),
        mode,

        // Font sizes are already in logical pixels (NOT points) -- use directly
        font_family: Some(SharedString::from(d.font.family.clone())),
        font_size: Some(d.font.size),
        mono_font_family: Some(SharedString::from(d.mono_font.family.clone())),
        mono_font_size: Some(d.mono_font.size),

        radius: Some(radius),
        radius_lg: Some(radius_lg),
        shadow: Some(d.shadow_enabled),

        // Issue 5: ThemeConfigColors left at default (all None) intentionally.
        // Populating them with hex strings loses alpha channels (overlay, blended
        // colors), causing rendering artifacts if apply_config() is called at
        // runtime. The primary ThemeColor set via Theme::from(&tc) is authoritative.
        ..ThemeConfig::default()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::ThemeSpec;

    fn test_resolved() -> native_theme::ResolvedThemeVariant {
        let nt = ThemeSpec::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(true)
            .expect("preset must have dark variant");
        variant
            .into_resolved()
            .expect("resolved preset must validate")
    }

    #[test]
    fn to_theme_config_from_resolved() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Test Theme", ThemeMode::Dark);

        assert_eq!(config.name.to_string(), "Test Theme");
        assert_eq!(config.mode, ThemeMode::Dark);
        assert!(config.font_family.is_some(), "font_family should be set");
        assert!(
            config.mono_font_family.is_some(),
            "mono_font_family should be set"
        );
        assert_eq!(config.font_size, Some(resolved.defaults.font.size));
        assert_eq!(
            config.mono_font_size,
            Some(resolved.defaults.mono_font.size)
        );
        assert_eq!(
            config.radius,
            Some(resolved.defaults.radius.max(0.0).round() as usize)
        );
        assert_eq!(config.shadow, Some(resolved.defaults.shadow_enabled));
    }

    #[test]
    fn to_theme_config_dark_mode() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Dark", ThemeMode::Dark);
        assert_eq!(config.mode, ThemeMode::Dark);
    }

    #[test]
    fn font_size_is_not_converted_from_points() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "SizeCheck", ThemeMode::Dark);
        let expected = resolved.defaults.font.size;
        assert_eq!(config.font_size, Some(expected));
        let pt_converted = expected * (96.0 / 72.0);
        assert_ne!(
            config.font_size,
            Some(pt_converted),
            "font size should NOT be pt-to-px converted"
        );
    }

    #[test]
    fn negative_radius_clamped() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Clamp", ThemeMode::Dark);
        assert!(config.radius.unwrap() < 1000, "radius should be reasonable");
    }

    #[test]
    fn multi_preset_config() {
        let presets = [
            ("catppuccin-mocha", ThemeMode::Dark),
            ("catppuccin-latte", ThemeMode::Light),
            ("dracula", ThemeMode::Dark),
            ("adwaita", ThemeMode::Light),
        ];
        for (name, mode) in presets {
            let nt = ThemeSpec::preset(name).expect("preset must exist");
            let is_dark = mode.is_dark();
            let variant = nt.into_variant(is_dark).expect("variant must exist");
            let resolved = variant.into_resolved().expect("must validate");
            let config = to_theme_config(&resolved, name, mode);
            assert_eq!(config.mode, mode, "mode mismatch for {name}");
        }
    }
}
