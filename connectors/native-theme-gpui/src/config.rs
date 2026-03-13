//! ThemeFonts/ThemeGeometry -> gpui_component::theme::ThemeConfig mapping.
//!
//! Maps native-theme's font and geometry settings to gpui-component's
//! `ThemeConfig`, which controls per-theme font family, font size, radius,
//! and shadow settings.

use gpui::SharedString;
use gpui_component::theme::{ThemeConfig, ThemeMode};
use native_theme::ThemeVariant;

/// Build a [`ThemeConfig`] from a [`ThemeVariant`].
///
/// Maps ThemeFonts fields to font_family/mono_font_family/font_size/mono_font_size,
/// and ThemeGeometry fields to radius/radius_lg/shadow. Remaining ThemeConfig
/// fields use defaults.
pub fn to_theme_config(variant: &ThemeVariant, name: &str, mode: ThemeMode) -> ThemeConfig {
    let fonts = &variant.fonts;
    let geometry = &variant.geometry;

    ThemeConfig {
        name: SharedString::from(name.to_string()),
        mode,

        font_family: fonts.family.as_ref().map(|s| SharedString::from(s.clone())),
        font_size: fonts.size.map(|pt| pt * (96.0 / 72.0)),
        mono_font_family: fonts
            .mono_family
            .as_ref()
            .map(|s| SharedString::from(s.clone())),
        mono_font_size: fonts.mono_size.map(|pt| pt * (96.0 / 72.0)),

        radius: geometry.radius.map(|r| r as usize),
        radius_lg: geometry.radius_lg.map(|r| r as usize),
        shadow: geometry.shadow,

        ..ThemeConfig::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_theme_config_populated() {
        let mut variant = ThemeVariant::default();
        variant.fonts.family = Some("Inter".into());
        variant.fonts.size = Some(14.0);
        variant.fonts.mono_family = Some("JetBrains Mono".into());
        variant.fonts.mono_size = Some(13.0);
        variant.geometry.radius = Some(4.0);
        variant.geometry.radius_lg = Some(8.0);
        variant.geometry.shadow = Some(true);

        let config = to_theme_config(&variant, "Test Theme", ThemeMode::Light);

        assert_eq!(config.name.to_string(), "Test Theme");
        assert_eq!(
            config.font_family.as_ref().map(|s| s.to_string()),
            Some("Inter".to_string())
        );
        assert_eq!(config.font_size, Some(14.0 * (96.0 / 72.0)));
        assert_eq!(
            config.mono_font_family.as_ref().map(|s| s.to_string()),
            Some("JetBrains Mono".to_string())
        );
        assert_eq!(config.mono_font_size, Some(13.0 * (96.0 / 72.0)));
        assert_eq!(config.radius, Some(4));
        assert_eq!(config.radius_lg, Some(8));
        assert_eq!(config.shadow, Some(true));
    }

    #[test]
    fn to_theme_config_empty_uses_defaults() {
        let variant = ThemeVariant::default();
        let config = to_theme_config(&variant, "Empty", ThemeMode::Dark);

        assert_eq!(config.name.to_string(), "Empty");
        assert_eq!(config.mode, ThemeMode::Dark);
        assert!(config.font_family.is_none());
        assert!(config.font_size.is_none());
        assert!(config.mono_font_family.is_none());
        assert!(config.mono_font_size.is_none());
        assert!(config.radius.is_none());
        assert!(config.radius_lg.is_none());
        assert!(config.shadow.is_none());
    }

    #[test]
    fn to_theme_config_partial_fonts() {
        let mut variant = ThemeVariant::default();
        variant.fonts.family = Some("Segoe UI".into());
        // size, mono_family, mono_size left as None

        let config = to_theme_config(&variant, "Partial", ThemeMode::Light);

        assert_eq!(
            config.font_family.as_ref().map(|s| s.to_string()),
            Some("Segoe UI".to_string())
        );
        assert!(config.font_size.is_none());
        assert!(config.mono_font_family.is_none());
    }
}
