// Theme model: ThemeVariant and NativeTheme, plus sub-module re-exports

/// Global theme defaults shared across widgets.
pub mod defaults;
/// Animated icon types (frame sequences and transforms).
pub mod animated;
/// Bundled SVG icon lookup tables.
pub mod bundled;
/// Dialog button ordering convention.
pub mod dialog_order;
/// Per-widget font specification and text scale.
pub mod font;
/// Icon roles, sets, and provider trait.
pub mod icons;
/// Per-context icon sizes.
pub mod icon_sizes;
/// Logical spacing scale (xxs through xxl).
pub mod spacing;
/// Resolved (non-optional) theme types produced after resolution.
pub mod resolved;
/// Per-widget struct pairs and macros.
pub mod widgets;

pub use animated::{AnimatedIcon, Repeat, TransformAnimation};
pub use bundled::{bundled_icon_by_name, bundled_icon_svg};
pub use defaults::ThemeDefaults;
pub use dialog_order::DialogButtonOrder;
pub use font::{FontSpec, TextScale, TextScaleEntry};
pub use icon_sizes::IconSizes;
pub use icons::{
    IconData, IconProvider, IconRole, IconSet, icon_name, system_icon_set, system_icon_theme,
};
pub use spacing::ThemeSpacing;
pub use widgets::*; // All 25 XxxTheme + ResolvedXxx + ResolvedFontSpec
pub use resolved::{
    ResolvedDefaults, ResolvedIconSizes, ResolvedSpacing, ResolvedTextScale,
    ResolvedTextScaleEntry, ResolvedTheme,
};

use serde::{Deserialize, Serialize};

/// A single light or dark theme variant containing all visual properties.
///
/// Composes defaults, per-widget structs, and optional text scale into one coherent set.
/// Empty sub-structs are omitted from serialization to keep TOML files clean.
///
/// # Examples
///
/// ```
/// use native_theme::{ThemeVariant, Rgba};
///
/// let mut variant = ThemeVariant::default();
/// variant.defaults.accent = Some(Rgba::rgb(0, 120, 215));
/// variant.defaults.font.family = Some("Inter".into());
/// assert!(!variant.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeVariant {
    /// Global defaults inherited by all widgets.
    #[serde(default, skip_serializing_if = "ThemeDefaults::is_empty")]
    pub defaults: ThemeDefaults,

    /// Per-role text scale overrides.
    #[serde(default, skip_serializing_if = "TextScale::is_empty")]
    pub text_scale: TextScale,

    /// Window chrome: background, title bar, radius, shadow.
    #[serde(default, skip_serializing_if = "WindowTheme::is_empty")]
    pub window: WindowTheme,

    /// Push button: colors, sizing, spacing, geometry.
    #[serde(default, skip_serializing_if = "ButtonTheme::is_empty")]
    pub button: ButtonTheme,

    /// Single-line and multi-line text input fields.
    #[serde(default, skip_serializing_if = "InputTheme::is_empty")]
    pub input: InputTheme,

    /// Checkbox and radio button indicator geometry.
    #[serde(default, skip_serializing_if = "CheckboxTheme::is_empty")]
    pub checkbox: CheckboxTheme,

    /// Popup and context menu appearance.
    #[serde(default, skip_serializing_if = "MenuTheme::is_empty")]
    pub menu: MenuTheme,

    /// Tooltip popup appearance.
    #[serde(default, skip_serializing_if = "TooltipTheme::is_empty")]
    pub tooltip: TooltipTheme,

    /// Scrollbar colors and geometry.
    #[serde(default, skip_serializing_if = "ScrollbarTheme::is_empty")]
    pub scrollbar: ScrollbarTheme,

    /// Slider control colors and geometry.
    #[serde(default, skip_serializing_if = "SliderTheme::is_empty")]
    pub slider: SliderTheme,

    /// Progress bar colors and geometry.
    #[serde(default, skip_serializing_if = "ProgressBarTheme::is_empty")]
    pub progress_bar: ProgressBarTheme,

    /// Tab bar colors and sizing.
    #[serde(default, skip_serializing_if = "TabTheme::is_empty")]
    pub tab: TabTheme,

    /// Sidebar panel background and foreground colors.
    #[serde(default, skip_serializing_if = "SidebarTheme::is_empty")]
    pub sidebar: SidebarTheme,

    /// Toolbar sizing, spacing, and font.
    #[serde(default, skip_serializing_if = "ToolbarTheme::is_empty")]
    pub toolbar: ToolbarTheme,

    /// Status bar font.
    #[serde(default, skip_serializing_if = "StatusBarTheme::is_empty")]
    pub status_bar: StatusBarTheme,

    /// List and table colors and row geometry.
    #[serde(default, skip_serializing_if = "ListTheme::is_empty")]
    pub list: ListTheme,

    /// Popover / dropdown panel appearance.
    #[serde(default, skip_serializing_if = "PopoverTheme::is_empty")]
    pub popover: PopoverTheme,

    /// Splitter handle width.
    #[serde(default, skip_serializing_if = "SplitterTheme::is_empty")]
    pub splitter: SplitterTheme,

    /// Separator line color.
    #[serde(default, skip_serializing_if = "SeparatorTheme::is_empty")]
    pub separator: SeparatorTheme,

    /// Toggle switch track, thumb, and geometry.
    #[serde(default, skip_serializing_if = "SwitchTheme::is_empty")]
    pub switch: SwitchTheme,

    /// Dialog sizing, spacing, button order, and title font.
    #[serde(default, skip_serializing_if = "DialogTheme::is_empty")]
    pub dialog: DialogTheme,

    /// Spinner / indeterminate progress indicator.
    #[serde(default, skip_serializing_if = "SpinnerTheme::is_empty")]
    pub spinner: SpinnerTheme,

    /// ComboBox / dropdown trigger sizing.
    #[serde(default, skip_serializing_if = "ComboBoxTheme::is_empty")]
    pub combo_box: ComboBoxTheme,

    /// Segmented control sizing.
    #[serde(default, skip_serializing_if = "SegmentedControlTheme::is_empty")]
    pub segmented_control: SegmentedControlTheme,

    /// Card / container colors and geometry.
    #[serde(default, skip_serializing_if = "CardTheme::is_empty")]
    pub card: CardTheme,

    /// Expander / disclosure row geometry.
    #[serde(default, skip_serializing_if = "ExpanderTheme::is_empty")]
    pub expander: ExpanderTheme,

    /// Hyperlink colors and underline setting.
    #[serde(default, skip_serializing_if = "LinkTheme::is_empty")]
    pub link: LinkTheme,

    /// Icon set / naming convention for this variant (e.g., "sf-symbols", "freedesktop").
    /// When None, resolved at runtime via system_icon_set().
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_set: Option<String>,
}

impl_merge!(ThemeVariant {
    option { icon_set }
    nested {
        defaults, text_scale, window, button, input, checkbox, menu,
        tooltip, scrollbar, slider, progress_bar, tab, sidebar,
        toolbar, status_bar, list, popover, splitter, separator,
        switch, dialog, spinner, combo_box, segmented_control,
        card, expander, link
    }
});

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
/// [light.defaults]
/// accent = "#ff6600"
/// "##;
/// let custom = NativeTheme::from_toml(toml).unwrap();
/// assert_eq!(custom.name, "Custom");
///
/// // Merge themes (overlay wins for populated fields)
/// let mut base = NativeTheme::preset("catppuccin-mocha").unwrap();
/// base.merge(&custom);
/// assert_eq!(base.name, "Catppuccin Mocha"); // base name is preserved
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// let theme = native_theme::NativeTheme::preset("catppuccin-mocha").unwrap();
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
    /// [light.defaults]
    /// accent = "#4a90d9"
    /// background = "#fafafa"
    /// foreground = "#2e3436"
    /// surface = "#ffffff"
    /// border = "#c0c0c0"
    /// muted = "#929292"
    /// shadow = "#00000018"
    /// danger = "#dc3545"
    /// warning = "#f0ad4e"
    /// success = "#28a745"
    /// info = "#4a90d9"
    /// selection = "#4a90d9"
    /// selection_foreground = "#ffffff"
    /// link = "#2a6cb6"
    /// focus_ring_color = "#4a90d9"
    /// disabled_foreground = "#c0c0c0"
    /// radius = 6.0
    /// radius_lg = 12.0
    /// frame_width = 1.0
    /// disabled_opacity = 0.5
    /// border_opacity = 0.15
    /// shadow_enabled = true
    ///
    /// [light.defaults.font]
    /// family = "sans-serif"
    /// size = 10.0
    ///
    /// [light.defaults.mono_font]
    /// family = "monospace"
    /// size = 10.0
    ///
    /// [light.defaults.spacing]
    /// xxs = 2.0
    /// xs = 4.0
    /// s = 6.0
    /// m = 12.0
    /// l = 18.0
    /// xl = 24.0
    /// xxl = 36.0
    ///
    /// [light.button]
    /// background = "#e8e8e8"
    /// foreground = "#2e3436"
    /// min_height = 32.0
    /// padding_horizontal = 12.0
    /// padding_vertical = 6.0
    ///
    /// [light.tooltip]
    /// background = "#2e3436"
    /// foreground = "#f0f0f0"
    /// padding_horizontal = 6.0
    /// padding_vertical = 6.0
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
    /// [light.defaults]
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

    /// List preset names appropriate for the current platform.
    ///
    /// Platform-specific presets (kde-breeze, adwaita, windows-11, macos-sonoma, ios)
    /// are only included on their native platform. Community themes are always included.
    ///
    /// # Examples
    /// ```
    /// let names = native_theme::NativeTheme::list_presets_for_platform();
    /// // On Linux KDE: includes kde-breeze, adwaita, plus all community themes
    /// // On Windows: includes windows-11 plus all community themes
    /// assert!(!names.is_empty());
    /// ```
    #[must_use = "this returns the filtered list of preset names for this platform"]
    pub fn list_presets_for_platform() -> Vec<&'static str> {
        crate::presets::list_presets_for_platform()
    }

    /// Serialize this theme to a TOML string.
    ///
    /// # Errors
    /// Returns [`crate::Error::Format`] if serialization fails.
    ///
    /// # Examples
    /// ```
    /// let theme = native_theme::NativeTheme::preset("catppuccin-mocha").unwrap();
    /// let toml_str = theme.to_toml().unwrap();
    /// assert!(toml_str.contains("name = \"Catppuccin Mocha\""));
    /// ```
    #[must_use = "this serializes the theme to TOML; it does not write to a file"]
    pub fn to_toml(&self) -> crate::Result<String> {
        crate::presets::to_toml(self)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
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
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        assert!(!v.is_empty());
    }

    #[test]
    fn theme_variant_not_empty_when_font_set() {
        let mut v = ThemeVariant::default();
        v.defaults.font.family = Some("Inter".into());
        assert!(!v.is_empty());
    }

    #[test]
    fn theme_variant_merge_recursively() {
        let mut base = ThemeVariant::default();
        base.defaults.background = Some(Rgba::rgb(255, 255, 255));
        base.defaults.font.family = Some("Noto Sans".into());

        let mut overlay = ThemeVariant::default();
        overlay.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.defaults.spacing.m = Some(12.0);

        base.merge(&overlay);

        // base background preserved
        assert_eq!(base.defaults.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay accent applied
        assert_eq!(base.defaults.accent, Some(Rgba::rgb(0, 120, 215)));
        // base font preserved
        assert_eq!(base.defaults.font.family.as_deref(), Some("Noto Sans"));
        // overlay spacing applied
        assert_eq!(base.defaults.spacing.m, Some(12.0));
    }

    #[test]
    fn theme_variant_has_all_widgets() {
        let mut v = ThemeVariant::default();
        // Set a field on each of the 25 widgets
        v.window.radius = Some(4.0);
        v.button.min_height = Some(32.0);
        v.input.min_height = Some(32.0);
        v.checkbox.indicator_size = Some(18.0);
        v.menu.item_height = Some(28.0);
        v.tooltip.padding_horizontal = Some(6.0);
        v.scrollbar.width = Some(14.0);
        v.slider.track_height = Some(4.0);
        v.progress_bar.height = Some(6.0);
        v.tab.min_height = Some(32.0);
        v.sidebar.background = Some(Rgba::rgb(240, 240, 240));
        v.toolbar.height = Some(40.0);
        v.status_bar.font = Some(crate::model::FontSpec::default());
        v.list.item_height = Some(28.0);
        v.popover.radius = Some(6.0);
        v.splitter.width = Some(4.0);
        v.separator.color = Some(Rgba::rgb(200, 200, 200));
        v.switch.track_width = Some(32.0);
        v.dialog.min_width = Some(320.0);
        v.spinner.diameter = Some(24.0);
        v.combo_box.min_height = Some(32.0);
        v.segmented_control.segment_height = Some(28.0);
        v.card.radius = Some(8.0);
        v.expander.header_height = Some(32.0);
        v.link.underline = Some(true);

        assert!(!v.is_empty());
        assert!(!v.window.is_empty());
        assert!(!v.button.is_empty());
        assert!(!v.input.is_empty());
        assert!(!v.checkbox.is_empty());
        assert!(!v.menu.is_empty());
        assert!(!v.tooltip.is_empty());
        assert!(!v.scrollbar.is_empty());
        assert!(!v.slider.is_empty());
        assert!(!v.progress_bar.is_empty());
        assert!(!v.tab.is_empty());
        assert!(!v.sidebar.is_empty());
        assert!(!v.toolbar.is_empty());
        assert!(!v.status_bar.is_empty());
        assert!(!v.list.is_empty());
        assert!(!v.popover.is_empty());
        assert!(!v.splitter.is_empty());
        assert!(!v.separator.is_empty());
        assert!(!v.switch.is_empty());
        assert!(!v.dialog.is_empty());
        assert!(!v.spinner.is_empty());
        assert!(!v.combo_box.is_empty());
        assert!(!v.segmented_control.is_empty());
        assert!(!v.card.is_empty());
        assert!(!v.expander.is_empty());
        assert!(!v.link.is_empty());
    }

    #[test]
    fn theme_variant_merge_per_widget() {
        let mut base = ThemeVariant::default();
        base.button.background = Some(Rgba::rgb(200, 200, 200));
        base.button.foreground = Some(Rgba::rgb(0, 0, 0));
        base.tooltip.background = Some(Rgba::rgb(50, 50, 50));

        let mut overlay = ThemeVariant::default();
        overlay.button.background = Some(Rgba::rgb(255, 255, 255));
        overlay.button.min_height = Some(32.0);

        base.merge(&overlay);

        // overlay background wins
        assert_eq!(base.button.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay min_height added
        assert_eq!(base.button.min_height, Some(32.0));
        // base foreground preserved
        assert_eq!(base.button.foreground, Some(Rgba::rgb(0, 0, 0)));
        // tooltip from base preserved
        assert_eq!(base.tooltip.background, Some(Rgba::rgb(50, 50, 50)));
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
        light.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.light = Some(light);

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().defaults.accent,
            Some(Rgba::rgb(0, 120, 215))
        );
    }

    #[test]
    fn native_theme_merge_both_light_variants() {
        let mut base = NativeTheme::new("Theme");
        let mut base_light = ThemeVariant::default();
        base_light.defaults.background = Some(Rgba::rgb(255, 255, 255));
        base.light = Some(base_light);

        let mut overlay = NativeTheme::new("Overlay");
        let mut overlay_light = ThemeVariant::default();
        overlay_light.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.light = Some(overlay_light);

        base.merge(&overlay);

        let light = base.light.as_ref().unwrap();
        // base background preserved
        assert_eq!(light.defaults.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay accent merged in
        assert_eq!(light.defaults.accent, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn native_theme_merge_base_light_only_preserved() {
        let mut base = NativeTheme::new("Theme");
        let mut base_light = ThemeVariant::default();
        base_light.defaults.font.family = Some("Inter".into());
        base.light = Some(base_light);

        let overlay = NativeTheme::new("Overlay"); // no light

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().defaults.font.family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn native_theme_merge_dark_variant() {
        let mut base = NativeTheme::new("Theme");

        let mut overlay = NativeTheme::new("Overlay");
        let mut dark = ThemeVariant::default();
        dark.defaults.background = Some(Rgba::rgb(30, 30, 30));
        overlay.dark = Some(dark);

        base.merge(&overlay);

        assert!(base.dark.is_some());
        assert_eq!(
            base.dark.as_ref().unwrap().defaults.background,
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
        light.defaults.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeVariant::default();
        dark.defaults.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(true).unwrap();
        assert_eq!(picked.defaults.background, Some(Rgba::rgb(30, 30, 30)));
    }

    #[test]
    fn pick_variant_light_with_both_variants_returns_light() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.defaults.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeVariant::default();
        dark.defaults.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(false).unwrap();
        assert_eq!(picked.defaults.background, Some(Rgba::rgb(255, 255, 255)));
    }

    #[test]
    fn pick_variant_dark_with_only_light_falls_back() {
        let mut theme = NativeTheme::new("Test");
        let mut light = ThemeVariant::default();
        light.defaults.background = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);

        let picked = theme.pick_variant(true).unwrap();
        assert_eq!(picked.defaults.background, Some(Rgba::rgb(255, 255, 255)));
    }

    #[test]
    fn pick_variant_light_with_only_dark_falls_back() {
        let mut theme = NativeTheme::new("Test");
        let mut dark = ThemeVariant::default();
        dark.defaults.background = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(false).unwrap();
        assert_eq!(picked.defaults.background, Some(Rgba::rgb(30, 30, 30)));
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
    fn icon_set_toml_absent_deserializes_to_none() {
        let toml_str = r##"
[defaults]
accent = "#ff0000"
"##;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert!(variant.icon_set.is_none());
    }

    #[test]
    fn native_theme_serde_toml_round_trip() {
        let mut theme = NativeTheme::new("Test Theme");
        let mut light = ThemeVariant::default();
        light.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        light.defaults.font.family = Some("Segoe UI".into());
        light.defaults.radius = Some(4.0);
        light.defaults.spacing.m = Some(12.0);
        theme.light = Some(light);

        let toml_str = toml::to_string(&theme).unwrap();
        let deserialized: NativeTheme = toml::from_str(&toml_str).unwrap();

        assert_eq!(deserialized.name, "Test Theme");
        let l = deserialized.light.unwrap();
        assert_eq!(l.defaults.accent, Some(Rgba::rgb(0, 120, 215)));
        assert_eq!(l.defaults.font.family.as_deref(), Some("Segoe UI"));
        assert_eq!(l.defaults.radius, Some(4.0));
        assert_eq!(l.defaults.spacing.m, Some(12.0));
    }
}
