// Theme model: ThemeVariant and NativeTheme, plus sub-module re-exports

pub mod colors;
pub mod fonts;
pub mod geometry;
pub mod icons;
pub mod spacing;
pub mod widget_metrics;

pub use colors::ThemeColors;
pub use fonts::ThemeFonts;
pub use geometry::ThemeGeometry;
pub use spacing::ThemeSpacing;
pub use icons::{IconData, IconRole, IconSet, icon_name, system_icon_set};
pub use widget_metrics::{
    ButtonMetrics, CheckboxMetrics, InputMetrics, ListItemMetrics, MenuItemMetrics,
    ProgressBarMetrics, ScrollbarMetrics, SliderMetrics, SplitterMetrics, TabMetrics,
    ToolbarMetrics, TooltipMetrics, WidgetMetrics,
};

use serde::{Deserialize, Serialize};

/// A single light or dark theme variant containing all visual properties.
///
/// Composes colors, fonts, geometry, and spacing into one coherent set.
/// Empty sub-structs are omitted from serialization to keep TOML files clean.
///
/// # Examples
///
/// ```
/// use native_theme::{ThemeVariant, Rgba};
///
/// let mut variant = ThemeVariant::default();
/// variant.colors.accent = Some(Rgba::rgb(0, 120, 215));
/// variant.fonts.family = Some("Inter".into());
/// assert!(!variant.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeVariant {
    #[serde(default, skip_serializing_if = "ThemeColors::is_empty")]
    pub colors: ThemeColors,

    #[serde(default, skip_serializing_if = "ThemeFonts::is_empty")]
    pub fonts: ThemeFonts,

    #[serde(default, skip_serializing_if = "ThemeGeometry::is_empty")]
    pub geometry: ThemeGeometry,

    #[serde(default, skip_serializing_if = "ThemeSpacing::is_empty")]
    pub spacing: ThemeSpacing,

    /// Per-widget sizing and spacing metrics.
    ///
    /// Optional because not all themes or presets provide widget metrics.
    /// When merging, if both base and overlay have widget metrics they are
    /// merged recursively; if only the overlay has them they are cloned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_metrics: Option<WidgetMetrics>,
}

impl ThemeVariant {
    /// Merge an overlay into this value. `Some` fields in the overlay
    /// replace the corresponding fields in self; `None` fields are
    /// left unchanged. Nested structs are merged recursively.
    pub fn merge(&mut self, overlay: &Self) {
        self.colors.merge(&overlay.colors);
        self.fonts.merge(&overlay.fonts);
        self.geometry.merge(&overlay.geometry);
        self.spacing.merge(&overlay.spacing);

        match (&mut self.widget_metrics, &overlay.widget_metrics) {
            (Some(base), Some(over)) => base.merge(over),
            (None, Some(over)) => self.widget_metrics = Some(over.clone()),
            _ => {}
        }
    }

    /// Returns true if all fields are at their default (None/empty) state.
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
            && self.fonts.is_empty()
            && self.geometry.is_empty()
            && self.spacing.is_empty()
            && self.widget_metrics.as_ref().is_none_or(|wm| wm.is_empty())
    }
}

/// A complete native theme with a name and optional light/dark variants.
///
/// This is the top-level type that theme files deserialize into and that
/// platform readers produce.
///
/// # Examples
///
/// ```
/// use native_theme::NativeTheme;
///
/// // Load a bundled preset
/// let theme = NativeTheme::preset("dracula").unwrap();
/// assert_eq!(theme.name, "Dracula");
///
/// // Parse from a TOML string
/// let toml = r##"
/// name = "Custom"
/// [light.colors]
/// accent = "#ff6600"
/// "##;
/// let custom = NativeTheme::from_toml(toml).unwrap();
/// assert_eq!(custom.name, "Custom");
///
/// // Merge themes (overlay wins for populated fields)
/// let mut base = NativeTheme::preset("default").unwrap();
/// base.merge(&custom);
/// assert_eq!(base.name, "Default"); // base name is preserved
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NativeTheme {
    /// Theme name (e.g., "Breeze", "Adwaita", "Windows 11").
    pub name: String,

    /// Light variant of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light: Option<ThemeVariant>,

    /// Dark variant of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark: Option<ThemeVariant>,
}

impl NativeTheme {
    /// Create a new theme with the given name and no variants.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            light: None,
            dark: None,
        }
    }

    /// Merge an overlay theme into this theme.
    ///
    /// The base name is kept. For each variant (light/dark):
    /// - If both base and overlay have a variant, they are merged recursively.
    /// - If only the overlay has a variant, it is cloned into the base.
    /// - If only the base has a variant (or neither), no change.
    pub fn merge(&mut self, overlay: &Self) {
        // Keep base name (do not overwrite)

        match (&mut self.light, &overlay.light) {
            (Some(base), Some(over)) => base.merge(over),
            (None, Some(over)) => self.light = Some(over.clone()),
            _ => {}
        }

        match (&mut self.dark, &overlay.dark) {
            (Some(base), Some(over)) => base.merge(over),
            (None, Some(over)) => self.dark = Some(over.clone()),
            _ => {}
        }
    }

    /// Returns true if the theme has no variants set.
    pub fn is_empty(&self) -> bool {
        self.light.is_none() && self.dark.is_none()
    }

    /// Load a bundled theme preset by name.
    ///
    /// Returns the preset as a fully populated [`NativeTheme`] with both
    /// light and dark variants.
    ///
    /// # Errors
    /// Returns [`crate::Error::Unavailable`] if the preset name is not recognized.
    ///
    /// # Examples
    /// ```
    /// let theme = native_theme::NativeTheme::preset("default").unwrap();
    /// assert!(theme.light.is_some());
    /// ```
    pub fn preset(name: &str) -> crate::Result<Self> {
        crate::presets::preset(name)
    }

    /// Parse a TOML string into a [`NativeTheme`].
    ///
    /// # Errors
    /// Returns [`crate::Error::Format`] if the TOML is invalid.
    ///
    /// # Examples
    /// ```
    /// let toml = r##"
    /// name = "My Theme"
    /// [light.colors]
    /// accent = "#ff0000"
    /// "##;
    /// let theme = native_theme::NativeTheme::from_toml(toml).unwrap();
    /// assert_eq!(theme.name, "My Theme");
    /// ```
    pub fn from_toml(toml_str: &str) -> crate::Result<Self> {
        crate::presets::from_toml(toml_str)
    }

    /// Load a [`NativeTheme`] from a TOML file.
    ///
    /// # Errors
    /// Returns [`crate::Error::Unavailable`] if the file cannot be read.
    ///
    /// # Examples
    /// ```no_run
    /// let theme = native_theme::NativeTheme::from_file("my-theme.toml").unwrap();
    /// ```
    pub fn from_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        crate::presets::from_file(path)
    }

    /// List all available bundled preset names.
    ///
    /// # Examples
    /// ```
    /// let names = native_theme::NativeTheme::list_presets();
    /// assert_eq!(names.len(), 17);
    /// ```
    pub fn list_presets() -> &'static [&'static str] {
        crate::presets::list_presets()
    }

    /// Serialize this theme to a TOML string.
    ///
    /// # Errors
    /// Returns [`crate::Error::Format`] if serialization fails.
    ///
    /// # Examples
    /// ```
    /// let theme = native_theme::NativeTheme::preset("default").unwrap();
    /// let toml_str = theme.to_toml().unwrap();
    /// assert!(toml_str.contains("name = \"Default\""));
    /// ```
    pub fn to_toml(&self) -> crate::Result<String> {
        crate::presets::to_toml(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    // === ThemeVariant tests ===

    #[test]
    fn theme_variant_default_is_empty() {
        assert!(ThemeVariant::default().is_empty());
    }

    #[test]
    fn theme_variant_not_empty_when_color_set() {
        let mut v = ThemeVariant::default();
        v.colors.accent = Some(Rgba::rgb(0, 120, 215));
        assert!(!v.is_empty());
    }

    #[test]
    fn theme_variant_not_empty_when_font_set() {
        let mut v = ThemeVariant::default();
        v.fonts.family = Some("Inter".into());
        assert!(!v.is_empty());
    }

    #[test]
    fn theme_variant_merge_recursively() {
        let mut base = ThemeVariant::default();
        base.colors.background = Some(Rgba::rgb(255, 255, 255));
        base.fonts.family = Some("Noto Sans".into());

        let mut overlay = ThemeVariant::default();
        overlay.colors.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.spacing.m = Some(12.0);

        base.merge(&overlay);

        // base background preserved
        assert_eq!(base.colors.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay accent applied
        assert_eq!(base.colors.accent, Some(Rgba::rgb(0, 120, 215)));
        // base font preserved
        assert_eq!(base.fonts.family.as_deref(), Some("Noto Sans"));
        // overlay spacing applied
        assert_eq!(base.spacing.m, Some(12.0));
    }

    // === NativeTheme tests ===

    #[test]
    fn native_theme_new_constructor() {
        let theme = NativeTheme::new("Breeze");
        assert_eq!(theme.name, "Breeze");
        assert!(theme.light.is_none());
        assert!(theme.dark.is_none());
    }

    #[test]
    fn native_theme_default_is_empty() {
        let theme = NativeTheme::default();
        assert!(theme.is_empty());
        assert_eq!(theme.name, "");
    }

    #[test]
    fn native_theme_merge_keeps_base_name() {
        let mut base = NativeTheme::new("Base Theme");
        let overlay = NativeTheme::new("Overlay Theme");
        base.merge(&overlay);
        assert_eq!(base.name, "Base Theme");
    }

    #[test]
    fn native_theme_merge_overlay_light_into_none() {
        let mut base = NativeTheme::new("Theme");

        let mut overlay = NativeTheme::new("Overlay");
        let mut light = ThemeVariant::default();
        light.colors.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.light = Some(light);

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().colors.accent,
            Some(Rgba::rgb(0, 120, 215))
        );
    }

    #[test]
    fn native_theme_merge_both_light_variants() {
        let mut base = NativeTheme::new("Theme");
        let mut base_light = ThemeVariant::default();
        base_light.colors.background = Some(Rgba::rgb(255, 255, 255));
        base.light = Some(base_light);

        let mut overlay = NativeTheme::new("Overlay");
        let mut overlay_light = ThemeVariant::default();
        overlay_light.colors.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.light = Some(overlay_light);

        base.merge(&overlay);

        let light = base.light.as_ref().unwrap();
        // base background preserved
        assert_eq!(light.colors.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay accent merged in
        assert_eq!(light.colors.accent, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn native_theme_merge_base_light_only_preserved() {
        let mut base = NativeTheme::new("Theme");
        let mut base_light = ThemeVariant::default();
        base_light.fonts.family = Some("Inter".into());
        base.light = Some(base_light);

        let overlay = NativeTheme::new("Overlay"); // no light

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().fonts.family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn native_theme_merge_dark_variant() {
        let mut base = NativeTheme::new("Theme");

        let mut overlay = NativeTheme::new("Overlay");
        let mut dark = ThemeVariant::default();
        dark.colors.background = Some(Rgba::rgb(30, 30, 30));
        overlay.dark = Some(dark);

        base.merge(&overlay);

        assert!(base.dark.is_some());
        assert_eq!(
            base.dark.as_ref().unwrap().colors.background,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn native_theme_not_empty_with_light() {
        let mut theme = NativeTheme::new("Theme");
        theme.light = Some(ThemeVariant::default());
        assert!(!theme.is_empty());
    }

    // === icon_theme tests ===

    #[test]
    fn icon_theme_default_is_none() {
        assert!(ThemeVariant::default().icon_theme.is_none());
    }

    #[test]
    fn icon_theme_merge_overlay() {
        let mut base = ThemeVariant::default();
        let mut overlay = ThemeVariant::default();
        overlay.icon_theme = Some("material".into());
        base.merge(&overlay);
        assert_eq!(base.icon_theme.as_deref(), Some("material"));
    }

    #[test]
    fn icon_theme_merge_none_preserves() {
        let mut base = ThemeVariant::default();
        base.icon_theme = Some("sf-symbols".into());
        let overlay = ThemeVariant::default(); // icon_theme is None
        base.merge(&overlay);
        assert_eq!(base.icon_theme.as_deref(), Some("sf-symbols"));
    }

    #[test]
    fn icon_theme_is_empty_when_set() {
        let mut v = ThemeVariant::default();
        assert!(v.is_empty());
        v.icon_theme = Some("material".into());
        assert!(!v.is_empty());
    }

    #[test]
    fn icon_theme_toml_round_trip() {
        let mut variant = ThemeVariant::default();
        variant.icon_theme = Some("material".into());
        let toml_str = toml::to_string(&variant).unwrap();
        assert!(toml_str.contains("icon_theme"));
        let deserialized: ThemeVariant = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.icon_theme.as_deref(), Some("material"));
    }

    #[test]
    fn icon_theme_toml_absent_deserializes_to_none() {
        let toml_str = r#"
[colors]
accent = "#ff0000"
"#;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert!(variant.icon_theme.is_none());
    }

    #[test]
    fn native_theme_serde_toml_round_trip() {
        let mut theme = NativeTheme::new("Test Theme");
        let mut light = ThemeVariant::default();
        light.colors.accent = Some(Rgba::rgb(0, 120, 215));
        light.fonts.family = Some("Segoe UI".into());
        light.geometry.radius = Some(4.0);
        light.spacing.m = Some(12.0);
        theme.light = Some(light);

        let toml_str = toml::to_string(&theme).unwrap();
        let deserialized: NativeTheme = toml::from_str(&toml_str).unwrap();

        assert_eq!(deserialized.name, "Test Theme");
        let l = deserialized.light.unwrap();
        assert_eq!(l.colors.accent, Some(Rgba::rgb(0, 120, 215)));
        assert_eq!(l.fonts.family.as_deref(), Some("Segoe UI"));
        assert_eq!(l.geometry.radius, Some(4.0));
        assert_eq!(l.spacing.m, Some(12.0));
    }
}
