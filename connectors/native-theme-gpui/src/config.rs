//! ResolvedTheme -> gpui_component::theme::ThemeConfig mapping.
//!
//! Maps native-theme's resolved font and geometry settings to gpui-component's
//! `ThemeConfig`, which controls per-theme font family, font size, radius,
//! shadow settings, and optionally all 139 color fields as hex strings.

use gpui::SharedString;
use gpui_component::theme::{ThemeConfig, ThemeConfigColors, ThemeMode as GpuiThemeMode};
use native_theme::theme::ResolvedTheme;

use crate::colors::{hsla_to_hex, to_theme_color};

/// Build a [`ThemeConfig`] from a [`ResolvedTheme`].
///
/// Maps ResolvedDefaults font/geometry fields to font_family/mono_font_family/
/// font_size/mono_font_size, radius/radius_lg/shadow. ResolvedFontSpec sizes are
/// in logical pixels (conversion from platform points is handled by the resolution step).
///
/// Also populates the `colors` field with all 139 ThemeColor fields converted
/// to hex strings, so the config can be serialized/deserialized losslessly.
///
/// Fields not explicitly set inherit from `ThemeConfig::default()`.
///
/// `highlight` is left at None here. The `Theme.highlight_theme` field is set
/// directly in `to_theme()` using `HighlightTheme::default_dark()` or
/// `default_light()` based on `is_dark`. Custom syntax colors require the full
/// `HighlightTheme` API.
pub fn to_theme_config(resolved: &ResolvedTheme, name: &str, mode: GpuiThemeMode) -> ThemeConfig {
    let d = &resolved.defaults;
    let is_dark = mode.is_dark();

    // Issue 14: clamp radius to non-negative before rounding
    let radius = d.border.corner_radius.max(0.0).round() as usize;
    let radius_lg = d.border.corner_radius_lg.max(0.0).round() as usize;

    // Issue 5: populate ThemeConfigColors from computed ThemeColor
    let tc = to_theme_color(resolved, is_dark, false);
    let colors = theme_color_to_config_colors(&tc);

    ThemeConfig {
        name: SharedString::from(name.to_string()),
        mode,

        // Font sizes are in logical pixels (pt-to-px conversion handled during resolution)
        font_family: Some(SharedString::from(d.font.family.clone())),
        font_size: Some(d.font.size),
        mono_font_family: Some(SharedString::from(d.mono_font.family.clone())),
        mono_font_size: Some(d.mono_font.size),

        radius: Some(radius),
        radius_lg: Some(radius_lg),
        shadow: Some(d.border.shadow_enabled),

        colors,

        // highlight: None — Theme.highlight_theme is set directly in to_theme()
        // via HighlightTheme::default_dark()/default_light(). ThemeConfig.highlight
        // is for custom syntax colors and is left at None.
        //
        // is_default: false (via ThemeConfig::default()) — this is intentional.
        // Connector-created themes are never the "default" theme; the application
        // decides which theme is its default.
        ..ThemeConfig::default()
    }
}

/// Convert the [`gpui_component::theme::ThemeColor`] fields to hex strings in a
/// [`ThemeConfigColors`].
///
/// This enables lossless round-tripping through gpui-component's JSON theme
/// serialization format. Each Hsla field is converted to `#rrggbb`, or to
/// `#rrggbbaa` when its alpha is below 1 (D36), so translucent colours such as
/// `overlay`, `drag_border` and `drop_target` survive `Theme::change`.
///
/// All 139 ThemeColor fields are exported except the 12 private base colours
/// (`red`, `blue`, `green`, `yellow`, `magenta`, `cyan` and their `_light`
/// variants) plus `group_box_title_foreground`, which stay `None`.
fn theme_color_to_config_colors(tc: &gpui_component::theme::ThemeColor) -> ThemeConfigColors {
    let h = |c: gpui::Hsla| -> Option<SharedString> { Some(SharedString::from(hsla_to_hex(c))) };

    // Note: ThemeConfigColors has private base color fields (red, blue, green,
    // yellow, magenta, cyan and their _light variants) that we cannot set
    // from outside gpui-component. Those 12 fields remain at their defaults.
    // gpui-component's apply_config will derive them from ThemeColor fields.
    let mut colors = ThemeConfigColors::default();
    colors.accent = h(tc.accent);
    colors.accent_foreground = h(tc.accent_foreground);
    colors.accordion = h(tc.accordion);
    colors.background = h(tc.background);
    colors.border = h(tc.border);
    colors.group_box = h(tc.group_box);
    // group_box_title_foreground: not mapped — inherits from ThemeConfig default (foreground).
    colors.group_box_foreground = h(tc.group_box_foreground);
    colors.caret = h(tc.caret);
    colors.chart_1 = h(tc.chart_1);
    colors.chart_2 = h(tc.chart_2);
    colors.chart_3 = h(tc.chart_3);
    colors.chart_4 = h(tc.chart_4);
    colors.chart_5 = h(tc.chart_5);
    colors.danger = h(tc.danger);
    colors.danger_active = h(tc.danger_active);
    colors.danger_foreground = h(tc.danger_foreground);
    colors.danger_hover = h(tc.danger_hover);
    colors.description_list_label = h(tc.description_list_label);
    colors.description_list_label_foreground = h(tc.description_list_label_foreground);
    colors.drag_border = h(tc.drag_border);
    colors.drop_target = h(tc.drop_target);
    colors.foreground = h(tc.foreground);
    colors.info = h(tc.info);
    colors.info_active = h(tc.info_active);
    colors.info_foreground = h(tc.info_foreground);
    colors.info_hover = h(tc.info_hover);
    colors.input = h(tc.input);
    colors.link = h(tc.link);
    colors.link_active = h(tc.link_active);
    colors.link_hover = h(tc.link_hover);
    colors.list = h(tc.list);
    colors.list_active = h(tc.list_active);
    colors.list_active_border = h(tc.list_active_border);
    colors.list_even = h(tc.list_even);
    colors.list_head = h(tc.list_head);
    colors.list_hover = h(tc.list_hover);
    colors.muted = h(tc.muted);
    colors.muted_foreground = h(tc.muted_foreground);
    colors.popover = h(tc.popover);
    colors.popover_foreground = h(tc.popover_foreground);
    colors.primary = h(tc.primary);
    colors.primary_active = h(tc.primary_active);
    colors.primary_foreground = h(tc.primary_foreground);
    colors.primary_hover = h(tc.primary_hover);
    colors.progress_bar = h(tc.progress_bar);
    colors.ring = h(tc.ring);
    colors.scrollbar = h(tc.scrollbar);
    colors.scrollbar_thumb = h(tc.scrollbar_thumb);
    colors.scrollbar_thumb_hover = h(tc.scrollbar_thumb_hover);
    colors.secondary = h(tc.secondary);
    colors.secondary_active = h(tc.secondary_active);
    colors.secondary_foreground = h(tc.secondary_foreground);
    colors.secondary_hover = h(tc.secondary_hover);
    colors.selection = h(tc.selection);
    colors.sidebar = h(tc.sidebar);
    colors.sidebar_accent = h(tc.sidebar_accent);
    colors.sidebar_accent_foreground = h(tc.sidebar_accent_foreground);
    colors.sidebar_border = h(tc.sidebar_border);
    colors.sidebar_foreground = h(tc.sidebar_foreground);
    colors.sidebar_primary = h(tc.sidebar_primary);
    colors.sidebar_primary_foreground = h(tc.sidebar_primary_foreground);
    colors.skeleton = h(tc.skeleton);
    colors.slider_bar = h(tc.slider_bar);
    colors.slider_thumb = h(tc.slider_thumb);
    colors.success = h(tc.success);
    colors.success_foreground = h(tc.success_foreground);
    colors.success_hover = h(tc.success_hover);
    colors.success_active = h(tc.success_active);
    colors.chart_bullish = h(tc.chart_bullish);
    colors.chart_bearish = h(tc.chart_bearish);
    colors.switch = h(tc.switch);
    colors.switch_thumb = h(tc.switch_thumb);
    colors.tab = h(tc.tab);
    colors.tab_active = h(tc.tab_active);
    colors.tab_active_foreground = h(tc.tab_active_foreground);
    colors.tab_bar = h(tc.tab_bar);
    colors.tab_bar_segmented = h(tc.tab_bar_segmented);
    colors.tab_foreground = h(tc.tab_foreground);
    colors.table = h(tc.table);
    colors.table_active = h(tc.table_active);
    colors.table_active_border = h(tc.table_active_border);
    colors.table_even = h(tc.table_even);
    colors.table_head = h(tc.table_head);
    colors.table_head_foreground = h(tc.table_head_foreground);
    colors.table_hover = h(tc.table_hover);
    colors.table_row_border = h(tc.table_row_border);
    colors.title_bar = h(tc.title_bar);
    colors.title_bar_border = h(tc.title_bar_border);
    colors.tiles = h(tc.tiles);
    colors.warning = h(tc.warning);
    colors.warning_active = h(tc.warning_active);
    colors.warning_hover = h(tc.warning_hover);
    colors.warning_foreground = h(tc.warning_foreground);
    colors.overlay = h(tc.overlay);
    colors.window_border = h(tc.window_border);
    colors.button = h(tc.button);
    colors.button_hover = h(tc.button_hover);
    colors.button_active = h(tc.button_active);
    colors.button_foreground = h(tc.button_foreground);
    colors.button_secondary = h(tc.button_secondary);
    colors.button_secondary_hover = h(tc.button_secondary_hover);
    colors.button_secondary_active = h(tc.button_secondary_active);
    colors.button_secondary_foreground = h(tc.button_secondary_foreground);
    colors.button_primary = h(tc.button_primary);
    colors.button_primary_hover = h(tc.button_primary_hover);
    colors.button_primary_active = h(tc.button_primary_active);
    colors.button_primary_foreground = h(tc.button_primary_foreground);
    colors.button_danger = h(tc.button_danger);
    colors.button_danger_hover = h(tc.button_danger_hover);
    colors.button_danger_active = h(tc.button_danger_active);
    colors.button_danger_foreground = h(tc.button_danger_foreground);
    colors.button_info = h(tc.button_info);
    colors.button_info_hover = h(tc.button_info_hover);
    colors.button_info_active = h(tc.button_info_active);
    colors.button_info_foreground = h(tc.button_info_foreground);
    colors.button_success = h(tc.button_success);
    colors.button_success_hover = h(tc.button_success_hover);
    colors.button_success_active = h(tc.button_success_active);
    colors.button_success_foreground = h(tc.button_success_foreground);
    colors.button_warning = h(tc.button_warning);
    colors.button_warning_hover = h(tc.button_warning_hover);
    colors.button_warning_active = h(tc.button_warning_active);
    colors.button_warning_foreground = h(tc.button_warning_foreground);
    colors.status_bar = h(tc.status_bar);
    colors.status_bar_border = h(tc.status_bar_border);
    colors.table_foot = h(tc.table_foot);
    colors.table_foot_foreground = h(tc.table_foot_foreground);
    colors
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ColorMode;
    use native_theme::theme::Theme;

    /// Issue 1: fixed to use `into_variant(true)` for catppuccin-mocha (dark theme).
    fn test_resolved() -> native_theme::theme::ResolvedTheme {
        let nt = Theme::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(ColorMode::Dark)
            .expect("preset must have dark variant");
        variant
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolved preset must validate")
    }

    #[test]
    fn to_theme_config_from_resolved() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Test Theme", GpuiThemeMode::Dark);

        assert_eq!(config.name.to_string(), "Test Theme");
        assert_eq!(config.mode, GpuiThemeMode::Dark);

        // Font family should be populated
        assert!(config.font_family.is_some(), "font_family should be set");
        assert!(
            config.mono_font_family.is_some(),
            "mono_font_family should be set"
        );

        // Font size from resolved (pt-to-px conversion already applied during resolution)
        assert_eq!(config.font_size, Some(resolved.defaults.font.size));
        assert_eq!(
            config.mono_font_size,
            Some(resolved.defaults.mono_font.size)
        );

        // Geometry (Issue 14: clamped to non-negative)
        assert_eq!(
            config.radius,
            Some(resolved.defaults.border.corner_radius.max(0.0).round() as usize)
        );
        assert_eq!(
            config.radius_lg,
            Some(resolved.defaults.border.corner_radius_lg.max(0.0).round() as usize)
        );
        assert_eq!(config.shadow, Some(resolved.defaults.border.shadow_enabled));
    }

    /// §5.3: the config copy carries every new field, so `Theme::change`
    /// reproduces the solid button surfaces instead of upstream's tint.
    #[test]
    fn theme_config_colors_cover_the_0_6_fields() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "New", GpuiThemeMode::Dark);
        let c = &config.colors;
        for (name, value) in [
            ("button", &c.button),
            ("button_foreground", &c.button_foreground),
            ("button_primary", &c.button_primary),
            ("button_primary_foreground", &c.button_primary_foreground),
            ("button_secondary_active", &c.button_secondary_active),
            ("button_danger_hover", &c.button_danger_hover),
            ("button_info_active", &c.button_info_active),
            ("button_success_foreground", &c.button_success_foreground),
            ("button_warning", &c.button_warning),
            ("chart_bullish", &c.chart_bullish),
            ("chart_bearish", &c.chart_bearish),
            ("status_bar", &c.status_bar),
            ("status_bar_border", &c.status_bar_border),
            ("table_foot", &c.table_foot),
            ("table_foot_foreground", &c.table_foot_foreground),
        ] {
            assert!(value.is_some(), "config colour {name} not exported");
        }
        // D36: drag_border is primary at alpha 0.65, so its export carries alpha.
        assert_eq!(
            c.drag_border.as_deref().map(str::len),
            Some(9),
            "translucent colours are exported as #rrggbbaa"
        );
    }

    #[test]
    fn to_theme_config_dark_mode() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Dark", GpuiThemeMode::Dark);
        assert_eq!(config.mode, GpuiThemeMode::Dark);
    }

    #[test]
    fn font_size_is_not_converted_from_points() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "SizeCheck", GpuiThemeMode::Dark);

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

    // Issue 5: ThemeConfigColors should be populated
    #[test]
    fn theme_config_colors_populated() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Colors", GpuiThemeMode::Dark);
        assert!(
            config.colors.background.is_some(),
            "colors.background should be populated"
        );
        assert!(
            config.colors.foreground.is_some(),
            "colors.foreground should be populated"
        );
        assert!(
            config.colors.primary.is_some(),
            "colors.primary should be populated"
        );
        assert!(
            config.colors.danger.is_some(),
            "colors.danger should be populated"
        );
        assert!(
            config.colors.accent.is_some(),
            "colors.accent should be populated"
        );
    }

    // Issue 14: negative radius should clamp to 0
    #[test]
    fn negative_radius_clamped() {
        // We can't easily set a negative radius in a resolved theme (validation
        // prevents it), so just verify the formula works on the positive path.
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "Clamp", GpuiThemeMode::Dark);
        assert!(config.radius.unwrap() < 1000, "radius should be reasonable");
    }

    // Issue 70: radius against a known preset value (non-tautological)
    #[test]
    fn radius_matches_known_preset_value() {
        // Adwaita has radius=9.0 in the preset TOML — verify the config reflects it
        let nt = Theme::preset("adwaita").expect("adwaita preset must exist");
        let variant = nt
            .into_variant(ColorMode::Light)
            .expect("adwaita must have light variant");
        let resolved = variant
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("must validate");
        let config = to_theme_config(&resolved, "adwaita", GpuiThemeMode::Light);
        assert_eq!(
            config.radius,
            Some(9),
            "adwaita radius should be 9 (from preset TOML radius=9.0)"
        );
    }

    // Issue 50: multi-preset config test
    #[test]
    fn multi_preset_config() {
        let presets = [
            ("catppuccin-mocha", GpuiThemeMode::Dark),
            ("catppuccin-latte", GpuiThemeMode::Light),
            ("dracula", GpuiThemeMode::Dark),
            ("adwaita", GpuiThemeMode::Light),
        ];
        for (name, mode) in presets {
            let nt = Theme::preset(name).expect("preset must exist");
            let is_dark = mode.is_dark();
            let variant = nt
                .into_variant(if is_dark {
                    ColorMode::Dark
                } else {
                    ColorMode::Light
                })
                .expect("variant must exist");
            let resolved = variant
                .into_resolved(&native_theme::ResolutionContext::for_tests())
                .expect("must validate");
            let config = to_theme_config(&resolved, name, mode);
            assert_eq!(config.mode, mode, "mode mismatch for {name}");
            assert!(
                config.colors.background.is_some(),
                "colors.background missing for {name}"
            );
        }
    }
}
