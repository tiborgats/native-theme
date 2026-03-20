// Theme model: ThemeVariant and NativeTheme, plus sub-module re-exports

/// Animated icon types (frame sequences and transforms).
pub mod animated;
/// Bundled SVG icon lookup tables.
pub mod bundled;
/// Semantic theme color roles.
pub mod colors;
/// Font family and size configuration.
pub mod fonts;
/// Corner radius, border, and scroll geometry.
pub mod geometry;
/// Icon roles, sets, and provider trait.
pub mod icons;
/// Logical spacing scale (xxs through xxl).
pub mod spacing;
/// Per-widget sizing and spacing metrics.
pub mod widget_metrics;

pub use animated::{AnimatedIcon, Repeat, TransformAnimation};
pub use bundled::{bundled_icon_by_name, bundled_icon_svg};
pub use colors::ThemeColors;
pub use fonts::ThemeFonts;
pub use geometry::ThemeGeometry;
pub use icons::{
    IconData, IconProvider, IconRole, IconSet, icon_name, system_icon_set, system_icon_theme,
};
pub use spacing::ThemeSpacing;
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
    /// Semantic color roles (accent, background, status, etc.).
    #[serde(default, skip_serializing_if = "ThemeColors::is_empty")]
    pub colors: ThemeColors,

    /// Font family and size settings.
    #[serde(default, skip_serializing_if = "ThemeFonts::is_empty")]
    pub fonts: ThemeFonts,

    /// Corner radius, border width, and scroll geometry.
    #[serde(default, skip_serializing_if = "ThemeGeometry::is_empty")]
    pub geometry: ThemeGeometry,

    /// Logical spacing scale.
    #[serde(default, skip_serializing_if = "ThemeSpacing::is_empty")]
    pub spacing: ThemeSpacing,

    /// Per-widget sizing and spacing metrics.
    ///
    /// Optional because not all themes or presets provide widget metrics.
    /// When merging, if both base and overlay have widget metrics they are
    /// merged recursively; if only the overlay has them they are cloned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_metrics: Option<WidgetMetrics>,

    /// Icon set / naming convention for this variant (e.g., "sf-symbols", "freedesktop").
    /// When None, resolved at runtime via system_icon_set().
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "icon_theme")]
    pub icon_set: Option<String>,
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

        if overlay.icon_set.is_some() {
            self.icon_set.clone_from(&overlay.icon_set);
        }
    }

    /// Returns true if all fields are at their default (None/empty) state.
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
            && self.fonts.is_empty()
            && self.geometry.is_empty()
            && self.spacing.is_empty()
            && self.widget_metrics.as_ref().is_none_or(|wm| wm.is_empty())
            && self.icon_set.is_none()
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
#[must_use = "constructing a theme without using it is likely a bug"]
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

    /// Pick the appropriate variant for the given mode, with cross-fallback.
    ///
    /// When `is_dark` is true, prefers `dark` and falls back to `light`.
    /// When `is_dark` is false, prefers `light` and falls back to `dark`.
    /// Returns `None` only if the theme has no variants at all.
    #[must_use = "this returns the selected variant; it does not apply it"]
    pub fn pick_variant(&self, is_dark: bool) -> Option<&ThemeVariant> {
        if is_dark {
            self.dark.as_ref().or(self.light.as_ref())
        } else {
            self.light.as_ref().or(self.dark.as_ref())
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
    #[must_use = "this returns a theme preset; it does not apply it"]
    pub fn preset(name: &str) -> crate::Result<Self> {
        crate::presets::preset(name)
    }

    /// Parse a TOML string into a [`NativeTheme`].
    ///
    /// # TOML Format
    ///
    /// Theme files use the following structure. All fields are `Option<T>` --
    /// omit any field you don't need. Unknown fields are silently ignored.
    /// Hex colors accept `#RRGGBB` or `#RRGGBBAA` format.
    ///
    /// ```toml
    /// name = "My Theme"
    ///
    /// [light.colors]
    /// # Core (7)
    /// accent = "#4a90d9"
    /// background = "#fafafa"
    /// foreground = "#2e3436"
    /// surface = "#ffffff"
    /// border = "#c0c0c0"
    /// muted = "#929292"
    /// shadow = "#00000018"
    /// # Primary (2)
    /// primary_background = "#4a90d9"
    /// primary_foreground = "#ffffff"
    /// # Secondary (2)
    /// secondary_background = "#6c757d"
    /// secondary_foreground = "#ffffff"
    /// # Status (8) -- each has an optional _foreground variant
    /// danger = "#dc3545"
    /// danger_foreground = "#ffffff"
    /// warning = "#f0ad4e"
    /// warning_foreground = "#ffffff"
    /// success = "#28a745"
    /// success_foreground = "#ffffff"
    /// info = "#4a90d9"
    /// info_foreground = "#ffffff"
    /// # Interactive (4)
    /// selection = "#4a90d9"
    /// selection_foreground = "#ffffff"
    /// link = "#2a6cb6"
    /// focus_ring = "#4a90d9"
    /// # Panel (6) -- each has an optional _foreground variant
    /// sidebar = "#f0f0f0"
    /// sidebar_foreground = "#2e3436"
    /// tooltip = "#2e3436"
    /// tooltip_foreground = "#ffffff"
    /// popover = "#ffffff"
    /// popover_foreground = "#2e3436"
    /// # Component (7) -- button and input have _foreground variants
    /// button = "#e8e8e8"
    /// button_foreground = "#2e3436"
    /// input = "#ffffff"
    /// input_foreground = "#2e3436"
    /// disabled = "#c0c0c0"
    /// separator = "#d0d0d0"
    /// alternate_row = "#f5f5f5"
    ///
    /// [light.fonts]
    /// family = "sans-serif"
    /// size = 10.0
    /// mono_family = "monospace"
    /// mono_size = 10.0
    ///
    /// [light.geometry]
    /// radius = 6.0
    /// radius_lg = 12.0
    /// frame_width = 1.0
    /// disabled_opacity = 0.5
    /// border_opacity = 0.15
    /// scroll_width = 8.0
    ///
    /// [light.spacing]
    /// xxs = 2.0
    /// xs = 4.0
    /// s = 6.0
    /// m = 12.0
    /// l = 18.0
    /// xl = 24.0
    /// xxl = 36.0
    ///
    /// # [dark.*] mirrors the same structure as [light.*]
    /// ```
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
    #[must_use = "this parses a TOML string into a theme; it does not apply it"]
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
    #[must_use = "this loads a theme from a file; it does not apply it"]
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
    #[must_use = "this returns the list of preset names"]
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
    #[must_use = "this serializes the theme to TOML; it does not write to a file"]
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

    // === pick_variant tests ===

    #[test]
    fn pick_variant_dark_with_both_variants_returns_dark() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.colors.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeVariant::default();
        dark.colors.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(true).unwrap();
        assert_eq!(picked.colors.background, Some(Rgba::rgb(30, 30, 30)));
    }

    #[test]
    fn pick_variant_light_with_both_variants_returns_light() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.colors.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeVariant::default();
        dark.colors.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(false).unwrap();
        assert_eq!(picked.colors.background, Some(Rgba::rgb(255, 255, 255)));
    }

    #[test]
    fn pick_variant_dark_with_only_light_falls_back() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.colors.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);

        let picked = theme.pick_variant(true).unwrap();
        assert_eq!(picked.colors.background, Some(Rgba::rgb(255, 255, 255)));
    }

    #[test]
    fn pick_variant_light_with_only_dark_falls_back() {
        let mut theme = NativeTheme::new("Test");
        let mut dark = ThemeVariant::default();
        dark.colors.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(false).unwrap();
        assert_eq!(picked.colors.background, Some(Rgba::rgb(30, 30, 30)));
    }

    #[test]
    fn pick_variant_with_no_variants_returns_none() {
        let theme = NativeTheme::new("Empty");
        assert!(theme.pick_variant(true).is_none());
        assert!(theme.pick_variant(false).is_none());
    }

    // === icon_set tests ===

    #[test]
    fn icon_set_default_is_none() {
        assert!(ThemeVariant::default().icon_set.is_none());
    }

    #[test]
    fn icon_set_merge_overlay() {
        let mut base = ThemeVariant::default();
        let overlay = ThemeVariant {
            icon_set: Some("material".into()),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.icon_set.as_deref(), Some("material"));
    }

    #[test]
    fn icon_set_merge_none_preserves() {
        let mut base = ThemeVariant {
            icon_set: Some("sf-symbols".into()),
            ..Default::default()
        };
        let overlay = ThemeVariant::default();
        base.merge(&overlay);
        assert_eq!(base.icon_set.as_deref(), Some("sf-symbols"));
    }

    #[test]
    fn icon_set_is_empty_when_set() {
        assert!(ThemeVariant::default().is_empty());
        let v = ThemeVariant {
            icon_set: Some("material".into()),
            ..Default::default()
        };
        assert!(!v.is_empty());
    }

    #[test]
    fn icon_set_toml_round_trip() {
        let variant = ThemeVariant {
            icon_set: Some("material".into()),
            ..Default::default()
        };
        let toml_str = toml::to_string(&variant).unwrap();
        assert!(toml_str.contains("icon_set"));
        let deserialized: ThemeVariant = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.icon_set.as_deref(), Some("material"));
    }

    #[test]
    fn icon_set_toml_alias_backward_compat() {
        // Old TOML files use "icon_theme" — verify the serde alias works
        let toml_str = r#"icon_theme = "freedesktop""#;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert_eq!(variant.icon_set.as_deref(), Some("freedesktop"));
    }

    #[test]
    fn icon_set_toml_absent_deserializes_to_none() {
        let toml_str = r##"
[colors]
accent = "#ff0000"
"##;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert!(variant.icon_set.is_none());
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
