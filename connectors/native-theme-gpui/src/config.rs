//! ResolvedTheme -> gpui_component::theme::ThemeConfig mapping.
//!
//! Maps native-theme's resolved font and geometry settings to gpui-component's
//! `ThemeConfig`, which controls per-theme font family, font size, radius,
//! and shadow settings.

use gpui::SharedString;
use gpui_component::theme::{ThemeConfig, ThemeMode};
use native_theme::ResolvedTheme;

/// Build a [`ThemeConfig`] from a [`ResolvedTheme`].
///
/// Maps ResolvedDefaults font/geometry fields to font_family/mono_font_family/
/// font_size/mono_font_size, radius/radius_lg/shadow. IMPORTANT: ResolvedFontSpec
/// sizes are already in logical pixels -- no pt-to-px conversion is applied.
pub fn to_theme_config(resolved: &ResolvedTheme, name: &str, mode: ThemeMode) -> ThemeConfig {
    let d = &resolved.defaults;

    ThemeConfig {
        name: SharedString::from(name.to_string()),
        mode,

        // Font sizes are already in logical pixels (NOT points) -- use directly
        font_family: Some(SharedString::from(d.font.family.clone())),
        font_size: Some(d.font.size),
        mono_font_family: Some(SharedString::from(d.mono_font.family.clone())),
        mono_font_size: Some(d.mono_font.size),

        radius: Some(d.radius as usize),
        radius_lg: Some(d.radius_lg as usize),
        shadow: Some(d.shadow_enabled),

        ..ThemeConfig::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use native_theme::NativeTheme;

    /// Create a ResolvedTheme via the preset resolve+validate pipeline.
    fn test_resolved() -> native_theme::ResolvedTheme {
        let nt = NativeTheme::preset("catppuccin-mocha").expect("preset must exist");
        let mut v = nt
            .pick_variant(false)
            .expect("preset must have light variant")
            .clone();
        v.resolve();
        v.validate().expect("resolved preset must validate")
    }

    #[test]
    fn to_theme_config_from_resolved() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Test Theme", ThemeMode::Light);

        assert_eq!(config.name.to_string(), "Test Theme");
        assert_eq!(config.mode, ThemeMode::Light);

        // Font family should be populated
        assert!(config.font_family.is_some(), "font_family should be set");
        assert!(config.mono_font_family.is_some(), "mono_font_family should be set");

        // Font size should be directly from resolved (no pt-to-px conversion)
        assert_eq!(config.font_size, Some(resolved.defaults.font.size));
        assert_eq!(config.mono_font_size, Some(resolved.defaults.mono_font.size));

        // Geometry
        assert_eq!(config.radius, Some(resolved.defaults.radius as usize));
        assert_eq!(config.radius_lg, Some(resolved.defaults.radius_lg as usize));
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
        let config = to_theme_config(&resolved, "SizeCheck", ThemeMode::Light);

        // The old code applied pt * (96.0/72.0) conversion. ResolvedFontSpec sizes
        // are already logical pixels, so font_size should equal the resolved value directly.
        let expected = resolved.defaults.font.size;
        assert_eq!(config.font_size, Some(expected));
        // Verify it is NOT the pt-converted value
        let pt_converted = expected * (96.0 / 72.0);
        assert_ne!(
            config.font_size,
            Some(pt_converted),
            "font size should NOT be pt-to-px converted"
        );
    }
}
