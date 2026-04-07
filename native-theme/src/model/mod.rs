// Theme model: ThemeVariant and ThemeSpec, plus sub-module re-exports

/// Animated icon types (frame sequences and transforms).
pub mod animated;
/// Border specification sub-struct for widget border properties.
pub mod border;
/// Bundled SVG icon lookup tables.
pub mod bundled;
/// Global theme defaults shared across widgets.
pub mod defaults;
/// Dialog button ordering convention.
pub mod dialog_order;
/// Per-widget font specification and text scale.
pub mod font;
/// Per-context icon sizes.
pub mod icon_sizes;
/// Icon roles, sets, and provider trait.
pub mod icons;
/// Resolved (non-optional) theme types produced after resolution.
pub mod resolved;
/// Per-widget struct pairs and macros.
pub mod widgets;

pub use animated::{AnimatedIcon, TransformAnimation};
pub use border::{BorderSpec, ResolvedBorderSpec};
pub use bundled::{bundled_icon_by_name, bundled_icon_svg};
pub use defaults::ThemeDefaults;
pub use dialog_order::DialogButtonOrder;
pub use font::{FontSpec, FontStyle, ResolvedFontSpec, TextScale, TextScaleEntry};
pub use icon_sizes::IconSizes;
pub use icons::{
    IconData, IconProvider, IconRole, IconSet, icon_name, system_icon_set, system_icon_theme,
};
pub use resolved::{
    ResolvedIconSizes, ResolvedTextScale, ResolvedTextScaleEntry, ResolvedThemeDefaults,
    ResolvedThemeVariant,
};
pub use widgets::*; // All 25 XxxTheme + ResolvedXxxTheme pairs

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
/// variant.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
/// variant.defaults.font.family = Some("Inter".into());
/// assert!(!variant.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
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

    /// Which icon loading mechanism to use (`Freedesktop`, `Material`, `Lucide`,
    /// `SfSymbols`, `SegoeIcons`).  Determines *how* icons are looked up — e.g.
    /// freedesktop theme directories vs. bundled SVG tables.
    /// When `None`, filled by [`resolve()`](ThemeVariant::resolve) from
    /// [`system_icon_set()`](crate::system_icon_set).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_set: Option<IconSet>,

    /// The name of the visual icon theme that provides the actual icon files
    /// (e.g. `"breeze"`, `"Adwaita"`, `"Lucide"`).  For `Freedesktop` this
    /// selects the theme directory; for bundled sets it is a display label.
    /// When `None`, filled by [`resolve_platform_defaults()`](ThemeVariant::resolve_platform_defaults)
    /// from [`system_icon_theme()`](crate::system_icon_theme).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_theme: Option<String>,
}

impl_merge!(ThemeVariant {
    option { icon_set, icon_theme }
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
/// use native_theme::ThemeSpec;
///
/// // Load a bundled preset
/// let theme = ThemeSpec::preset("dracula").unwrap();
/// assert_eq!(theme.name, "Dracula");
///
/// // Parse from a TOML string
/// let toml = r##"
/// name = "Custom"
/// [light.defaults]
/// accent_color = "#ff6600"
/// "##;
/// let custom = ThemeSpec::from_toml(toml).unwrap();
/// assert_eq!(custom.name, "Custom");
///
/// // Merge themes (overlay wins for populated fields)
/// let mut base = ThemeSpec::preset("catppuccin-mocha").unwrap();
/// base.merge(&custom);
/// assert_eq!(base.name, "Catppuccin Mocha"); // base name is preserved
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[must_use = "constructing a theme without using it is likely a bug"]
pub struct ThemeSpec {
    /// Theme name (e.g., "Breeze", "Adwaita", "Windows 11").
    pub name: String,

    /// Light variant of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light: Option<ThemeVariant>,

    /// Dark variant of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark: Option<ThemeVariant>,

    /// Layout spacing constants (shared between light and dark variants).
    #[serde(default, skip_serializing_if = "LayoutTheme::is_empty")]
    pub layout: LayoutTheme,
}

impl ThemeSpec {
    /// Create a new theme with the given name and no variants.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            light: None,
            dark: None,
            layout: LayoutTheme::default(),
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

        self.layout.merge(&overlay.layout);
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

    /// Extract a variant by consuming the theme, avoiding a clone.
    ///
    /// When `is_dark` is true, returns the `dark` variant (falling back to
    /// `light`). When false, returns `light` (falling back to `dark`).
    /// Returns `None` only if the theme has no variants at all.
    ///
    /// Use this when you own the `ThemeSpec` and don't need it afterward.
    /// For read-only inspection, use [`pick_variant()`](Self::pick_variant).
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = native_theme::ThemeSpec::preset("dracula").unwrap();
    /// let variant = theme.into_variant(true).unwrap();
    /// let resolved = variant.into_resolved().unwrap();
    /// ```
    #[must_use = "this returns the extracted variant; it does not apply it"]
    pub fn into_variant(self, is_dark: bool) -> Option<ThemeVariant> {
        if is_dark {
            self.dark.or(self.light)
        } else {
            self.light.or(self.dark)
        }
    }

    /// Returns true if the theme has no variants set.
    pub fn is_empty(&self) -> bool {
        self.light.is_none() && self.dark.is_none() && self.layout.is_empty()
    }

    /// Load a bundled theme preset by name.
    ///
    /// Returns the preset as a fully populated [`ThemeSpec`] with both
    /// light and dark variants.
    ///
    /// # Errors
    /// Returns [`crate::Error::Unavailable`] if the preset name is not recognized.
    ///
    /// # Examples
    /// ```
    /// let theme = native_theme::ThemeSpec::preset("catppuccin-mocha").unwrap();
    /// assert!(theme.light.is_some());
    /// ```
    #[must_use = "this returns a theme preset; it does not apply it"]
    pub fn preset(name: &str) -> crate::Result<Self> {
        crate::presets::preset(name)
    }

    /// Parse a TOML string into a [`ThemeSpec`].
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
    /// accent_color = "#4a90d9"
    /// background_color = "#fafafa"
    /// text_color = "#2e3436"
    /// surface_color = "#ffffff"
    /// muted_color = "#929292"
    /// shadow_color = "#00000018"
    /// danger_color = "#dc3545"
    /// warning_color = "#f0ad4e"
    /// success_color = "#28a745"
    /// info_color = "#4a90d9"
    /// selection_background = "#4a90d9"
    /// selection_text_color = "#ffffff"
    /// link_color = "#2a6cb6"
    /// focus_ring_color = "#4a90d9"
    /// disabled_text_color = "#c0c0c0"
    /// disabled_opacity = 0.5
    ///
    /// [light.defaults.font]
    /// family = "sans-serif"
    /// size = 10.0
    ///
    /// [light.defaults.mono_font]
    /// family = "monospace"
    /// size = 10.0
    ///
    /// [light.defaults.border]
    /// color = "#c0c0c0"
    /// corner_radius = 6.0
    /// corner_radius_lg = 12.0
    /// line_width = 1.0
    /// opacity = 0.15
    /// shadow_enabled = true
    ///
    /// [light.button]
    /// background_color = "#e8e8e8"
    /// min_height = 32.0
    ///
    /// [light.button.font]
    /// color = "#2e3436"
    ///
    /// [light.button.border]
    /// padding_horizontal = 12.0
    /// padding_vertical = 6.0
    ///
    /// [light.tooltip]
    /// background_color = "#2e3436"
    /// max_width = 300.0
    ///
    /// [light.tooltip.font]
    /// color = "#f0f0f0"
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
    /// accent_color = "#ff0000"
    /// "##;
    /// let theme = native_theme::ThemeSpec::from_toml(toml).unwrap();
    /// assert_eq!(theme.name, "My Theme");
    /// ```
    #[must_use = "this parses a TOML string into a theme; it does not apply it"]
    pub fn from_toml(toml_str: &str) -> crate::Result<Self> {
        crate::presets::from_toml(toml_str)
    }

    /// Parse custom TOML and merge onto a base preset.
    ///
    /// This is the recommended way to create custom themes. The base preset
    /// provides geometry, spacing, and widget defaults. The custom TOML
    /// overrides colors, fonts, and any other fields.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Unavailable`] if the base preset name is not
    /// recognized, or [`crate::Error::Format`] if the custom TOML is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = native_theme::ThemeSpec::from_toml_with_base(
    ///     r##"name = "My Theme"
    /// [dark.defaults]
    /// accent_color = "#ff6600"
    /// background_color = "#1e1e1e"
    /// text_color = "#e0e0e0""##,
    ///     "material",
    /// ).unwrap();
    /// assert!(theme.dark.is_some());
    /// ```
    pub fn from_toml_with_base(toml_str: &str, base: &str) -> crate::Result<Self> {
        let mut theme = Self::preset(base)?;
        let overlay = Self::from_toml(toml_str)?;
        theme.merge(&overlay);
        Ok(theme)
    }

    /// Load a [`ThemeSpec`] from a TOML file.
    ///
    /// # Errors
    /// Returns [`crate::Error::Io`] if the file cannot be read, or
    /// [`crate::Error::Format`] if the TOML content is invalid.
    ///
    /// # Examples
    /// ```no_run
    /// let theme = native_theme::ThemeSpec::from_file("my-theme.toml").unwrap();
    /// ```
    #[must_use = "this loads a theme from a file; it does not apply it"]
    pub fn from_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        crate::presets::from_file(path)
    }

    /// List all available bundled preset names.
    ///
    /// # Examples
    /// ```
    /// let names = native_theme::ThemeSpec::list_presets();
    /// assert_eq!(names.len(), 16);
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
    /// let names = native_theme::ThemeSpec::list_presets_for_platform();
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
    /// let theme = native_theme::ThemeSpec::preset("catppuccin-mocha").unwrap();
    /// let toml_str = theme.to_toml().unwrap();
    /// assert!(toml_str.contains("name = \"Catppuccin Mocha\""));
    /// ```
    #[must_use = "this serializes the theme to TOML; it does not write to a file"]
    pub fn to_toml(&self) -> crate::Result<String> {
        crate::presets::to_toml(self)
    }

    /// Check a TOML string for unrecognized field names.
    ///
    /// Parses the TOML as a generic table and walks all keys, comparing
    /// against the known fields for each section. Returns a `Vec<String>`
    /// of warnings for any keys that don't match a known field. An empty
    /// vec means all keys are recognized.
    ///
    /// This is an opt-in linting tool for theme authors. It does NOT affect
    /// `from_toml()` behavior (which silently ignores unknown fields via serde).
    ///
    /// # Errors
    ///
    /// Returns `Err` if the TOML string cannot be parsed at all.
    ///
    /// # Examples
    ///
    /// ```
    /// let warnings = native_theme::ThemeSpec::lint_toml(r##"
    /// name = "Test"
    /// [light.defaults]
    /// backround = "#ffffff"
    /// "##).unwrap();
    /// assert_eq!(warnings.len(), 1);
    /// assert!(warnings[0].contains("backround"));
    /// ```
    pub fn lint_toml(toml_str: &str) -> crate::Result<Vec<String>> {
        use crate::model::defaults::ThemeDefaults;

        let value: toml::Value = toml::from_str(toml_str)
            .map_err(|e: toml::de::Error| crate::Error::Format(e.to_string()))?;

        let mut warnings = Vec::new();

        let top_table = match &value {
            toml::Value::Table(t) => t,
            _ => return Ok(warnings),
        };

        // Known top-level keys
        const TOP_KEYS: &[&str] = &["name", "light", "dark", "layout"];

        for key in top_table.keys() {
            if !TOP_KEYS.contains(&key.as_str()) {
                warnings.push(format!("unknown field: {key}"));
            }
        }

        // Variant-level known keys: widget names + special fields
        const VARIANT_KEYS: &[&str] = &[
            "defaults",
            "text_scale",
            "window",
            "button",
            "input",
            "checkbox",
            "menu",
            "tooltip",
            "scrollbar",
            "slider",
            "progress_bar",
            "tab",
            "sidebar",
            "toolbar",
            "status_bar",
            "list",
            "popover",
            "splitter",
            "separator",
            "switch",
            "dialog",
            "spinner",
            "combo_box",
            "segmented_control",
            "card",
            "expander",
            "link",
            "icon_set",
            "icon_theme",
        ];

        // FontSpec, BorderSpec, TextScaleEntry, TextScale, and IconSizes
        // all use their own FIELD_NAMES constants (issue 3b).

        /// Look up the known field names for a given widget section key.
        fn widget_fields(section: &str) -> Option<&'static [&'static str]> {
            match section {
                "window" => Some(WindowTheme::FIELD_NAMES),
                "button" => Some(ButtonTheme::FIELD_NAMES),
                "input" => Some(InputTheme::FIELD_NAMES),
                "checkbox" => Some(CheckboxTheme::FIELD_NAMES),
                "menu" => Some(MenuTheme::FIELD_NAMES),
                "tooltip" => Some(TooltipTheme::FIELD_NAMES),
                "scrollbar" => Some(ScrollbarTheme::FIELD_NAMES),
                "slider" => Some(SliderTheme::FIELD_NAMES),
                "progress_bar" => Some(ProgressBarTheme::FIELD_NAMES),
                "tab" => Some(TabTheme::FIELD_NAMES),
                "sidebar" => Some(SidebarTheme::FIELD_NAMES),
                "toolbar" => Some(ToolbarTheme::FIELD_NAMES),
                "status_bar" => Some(StatusBarTheme::FIELD_NAMES),
                "list" => Some(ListTheme::FIELD_NAMES),
                "popover" => Some(PopoverTheme::FIELD_NAMES),
                "splitter" => Some(SplitterTheme::FIELD_NAMES),
                "separator" => Some(SeparatorTheme::FIELD_NAMES),
                "switch" => Some(SwitchTheme::FIELD_NAMES),
                "dialog" => Some(DialogTheme::FIELD_NAMES),
                "spinner" => Some(SpinnerTheme::FIELD_NAMES),
                "combo_box" => Some(ComboBoxTheme::FIELD_NAMES),
                "segmented_control" => Some(SegmentedControlTheme::FIELD_NAMES),
                "card" => Some(CardTheme::FIELD_NAMES),
                "expander" => Some(ExpanderTheme::FIELD_NAMES),
                "link" => Some(LinkTheme::FIELD_NAMES),
                _ => None,
            }
        }

        // Lint a text_scale section
        fn lint_text_scale(
            table: &toml::map::Map<String, toml::Value>,
            prefix: &str,
            warnings: &mut Vec<String>,
        ) {
            for key in table.keys() {
                if !TextScale::FIELD_NAMES.contains(&key.as_str()) {
                    warnings.push(format!("unknown field: {prefix}.{key}"));
                } else if let Some(toml::Value::Table(entry_table)) = table.get(key) {
                    for ekey in entry_table.keys() {
                        if !TextScaleEntry::FIELD_NAMES.contains(&ekey.as_str()) {
                            warnings.push(format!("unknown field: {prefix}.{key}.{ekey}"));
                        }
                    }
                }
            }
        }

        // Lint a defaults section (with nested font, mono_font, border, icon_sizes)
        fn lint_defaults(
            table: &toml::map::Map<String, toml::Value>,
            prefix: &str,
            warnings: &mut Vec<String>,
        ) {
            for key in table.keys() {
                if !ThemeDefaults::FIELD_NAMES.contains(&key.as_str()) {
                    warnings.push(format!("unknown field: {prefix}.{key}"));
                    continue;
                }
                // Check sub-tables for nested struct fields
                if let Some(toml::Value::Table(sub)) = table.get(key) {
                    let known = match key.as_str() {
                        "font" | "mono_font" => FontSpec::FIELD_NAMES,
                        "border" => BorderSpec::FIELD_NAMES,
                        "icon_sizes" => IconSizes::FIELD_NAMES,
                        _ => continue,
                    };
                    for skey in sub.keys() {
                        if !known.contains(&skey.as_str()) {
                            warnings.push(format!("unknown field: {prefix}.{key}.{skey}"));
                        }
                    }
                }
            }
        }

        // Lint a variant section (light or dark)
        fn lint_variant(
            table: &toml::map::Map<String, toml::Value>,
            prefix: &str,
            warnings: &mut Vec<String>,
        ) {
            for key in table.keys() {
                if !VARIANT_KEYS.contains(&key.as_str()) {
                    warnings.push(format!("unknown field: {prefix}.{key}"));
                    continue;
                }

                if let Some(toml::Value::Table(sub)) = table.get(key) {
                    let sub_prefix = format!("{prefix}.{key}");
                    match key.as_str() {
                        "defaults" => lint_defaults(sub, &sub_prefix, warnings),
                        "text_scale" => lint_text_scale(sub, &sub_prefix, warnings),
                        _ => {
                            if let Some(fields) = widget_fields(key) {
                                for skey in sub.keys() {
                                    if !fields.contains(&skey.as_str()) {
                                        warnings
                                            .push(format!("unknown field: {sub_prefix}.{skey}"));
                                    }
                                    // Validate sub-tables (font/border nested structs)
                                    if let Some(toml::Value::Table(nested)) = sub.get(skey) {
                                        let nested_known = match skey.as_str() {
                                            s if s == "font" || s.ends_with("_font") => {
                                                Some(FontSpec::FIELD_NAMES)
                                            }
                                            "border" => Some(BorderSpec::FIELD_NAMES),
                                            _ => None,
                                        };
                                        if let Some(known) = nested_known {
                                            for nkey in nested.keys() {
                                                if !known.contains(&nkey.as_str()) {
                                                    warnings.push(format!(
                                                        "unknown field: {sub_prefix}.{skey}.{nkey}"
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Lint light and dark variant sections
        for variant_key in &["light", "dark"] {
            if let Some(toml::Value::Table(variant_table)) = top_table.get(*variant_key) {
                lint_variant(variant_table, variant_key, &mut warnings);
            }
        }

        // Lint top-level [layout] section
        if let Some(toml::Value::Table(layout_table)) = top_table.get("layout") {
            for key in layout_table.keys() {
                if !LayoutTheme::FIELD_NAMES.contains(&key.as_str()) {
                    warnings.push(format!("unknown field: layout.{key}"));
                }
            }
        }

        Ok(warnings)
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
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
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
        base.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        base.defaults.font.family = Some("Noto Sans".into());

        let mut overlay = ThemeVariant::default();
        overlay.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        overlay.defaults.border.corner_radius = Some(4.0);

        base.merge(&overlay);

        // base background preserved
        assert_eq!(
            base.defaults.background_color,
            Some(Rgba::rgb(255, 255, 255))
        );
        // overlay accent applied
        assert_eq!(base.defaults.accent_color, Some(Rgba::rgb(0, 120, 215)));
        // base font preserved
        assert_eq!(base.defaults.font.family.as_deref(), Some("Noto Sans"));
        // overlay border applied
        assert_eq!(base.defaults.border.corner_radius, Some(4.0));
    }

    #[test]
    fn theme_variant_has_all_widgets() {
        let mut v = ThemeVariant::default();
        // Set a field on each of the 25 widgets
        v.window.background_color = Some(Rgba::rgb(255, 255, 255));
        v.button.min_height = Some(32.0);
        v.input.min_height = Some(32.0);
        v.checkbox.indicator_width = Some(18.0);
        v.menu.row_height = Some(28.0);
        v.tooltip.max_width = Some(300.0);
        v.scrollbar.groove_width = Some(14.0);
        v.slider.track_height = Some(4.0);
        v.progress_bar.track_height = Some(6.0);
        v.tab.min_height = Some(32.0);
        v.sidebar.background_color = Some(Rgba::rgb(240, 240, 240));
        v.toolbar.bar_height = Some(40.0);
        v.status_bar.background_color = Some(Rgba::rgb(240, 240, 240));
        v.list.row_height = Some(28.0);
        v.popover.background_color = Some(Rgba::rgb(255, 255, 255));
        v.splitter.divider_width = Some(4.0);
        v.separator.line_color = Some(Rgba::rgb(200, 200, 200));
        v.switch.track_width = Some(32.0);
        v.dialog.min_width = Some(320.0);
        v.spinner.diameter = Some(24.0);
        v.combo_box.min_height = Some(32.0);
        v.segmented_control.segment_height = Some(28.0);
        v.card.background_color = Some(Rgba::rgb(255, 255, 255));
        v.expander.header_height = Some(32.0);
        v.link.underline_enabled = Some(true);

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
        base.button.background_color = Some(Rgba::rgb(200, 200, 200));
        base.button.min_height = Some(28.0);
        base.tooltip.background_color = Some(Rgba::rgb(50, 50, 50));

        let mut overlay = ThemeVariant::default();
        overlay.button.background_color = Some(Rgba::rgb(255, 255, 255));
        overlay.button.min_width = Some(64.0);

        base.merge(&overlay);

        // overlay background wins
        assert_eq!(base.button.background_color, Some(Rgba::rgb(255, 255, 255)));
        // overlay min_width added
        assert_eq!(base.button.min_width, Some(64.0));
        // base min_height preserved
        assert_eq!(base.button.min_height, Some(28.0));
        // tooltip from base preserved
        assert_eq!(base.tooltip.background_color, Some(Rgba::rgb(50, 50, 50)));
    }

    // === ThemeSpec tests ===

    #[test]
    fn native_theme_new_constructor() {
        let theme = ThemeSpec::new("Breeze");
        assert_eq!(theme.name, "Breeze");
        assert!(theme.light.is_none());
        assert!(theme.dark.is_none());
    }

    #[test]
    fn native_theme_default_is_empty() {
        let theme = ThemeSpec::default();
        assert!(theme.is_empty());
        assert_eq!(theme.name, "");
    }

    #[test]
    fn native_theme_merge_keeps_base_name() {
        let mut base = ThemeSpec::new("Base Theme");
        let overlay = ThemeSpec::new("Overlay Theme");
        base.merge(&overlay);
        assert_eq!(base.name, "Base Theme");
    }

    #[test]
    fn native_theme_merge_overlay_light_into_none() {
        let mut base = ThemeSpec::new("Theme");

        let mut overlay = ThemeSpec::new("Overlay");
        let mut light = ThemeVariant::default();
        light.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        overlay.light = Some(light);

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().defaults.accent_color,
            Some(Rgba::rgb(0, 120, 215))
        );
    }

    #[test]
    fn native_theme_merge_both_light_variants() {
        let mut base = ThemeSpec::new("Theme");
        let mut base_light = ThemeVariant::default();
        base_light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        base.light = Some(base_light);

        let mut overlay = ThemeSpec::new("Overlay");
        let mut overlay_light = ThemeVariant::default();
        overlay_light.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        overlay.light = Some(overlay_light);

        base.merge(&overlay);

        let light = base.light.as_ref().unwrap();
        // base background preserved
        assert_eq!(
            light.defaults.background_color,
            Some(Rgba::rgb(255, 255, 255))
        );
        // overlay accent merged in
        assert_eq!(light.defaults.accent_color, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn native_theme_merge_base_light_only_preserved() {
        let mut base = ThemeSpec::new("Theme");
        let mut base_light = ThemeVariant::default();
        base_light.defaults.font.family = Some("Inter".into());
        base.light = Some(base_light);

        let overlay = ThemeSpec::new("Overlay"); // no light

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().defaults.font.family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn native_theme_merge_dark_variant() {
        let mut base = ThemeSpec::new("Theme");

        let mut overlay = ThemeSpec::new("Overlay");
        let mut dark = ThemeVariant::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        overlay.dark = Some(dark);

        base.merge(&overlay);

        assert!(base.dark.is_some());
        assert_eq!(
            base.dark.as_ref().unwrap().defaults.background_color,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn native_theme_not_empty_with_light() {
        let mut theme = ThemeSpec::new("Theme");
        theme.light = Some(ThemeVariant::default());
        assert!(!theme.is_empty());
    }

    // === pick_variant tests ===

    #[test]
    fn pick_variant_dark_with_both_variants_returns_dark() {
        let mut theme = ThemeSpec::new("Test");
        let mut light = ThemeVariant::default();
        light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeVariant::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(true).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn pick_variant_light_with_both_variants_returns_light() {
        let mut theme = ThemeSpec::new("Test");
        let mut light = ThemeVariant::default();
        light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeVariant::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(false).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(255, 255, 255))
        );
    }

    #[test]
    fn pick_variant_dark_with_only_light_falls_back() {
        let mut theme = ThemeSpec::new("Test");
        let mut light = ThemeVariant::default();
        light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);

        let picked = theme.pick_variant(true).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(255, 255, 255))
        );
    }

    #[test]
    fn pick_variant_light_with_only_dark_falls_back() {
        let mut theme = ThemeSpec::new("Test");
        let mut dark = ThemeVariant::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(false).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn pick_variant_with_no_variants_returns_none() {
        let theme = ThemeSpec::new("Empty");
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
            icon_set: Some(IconSet::Material),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.icon_set, Some(IconSet::Material));
    }

    #[test]
    fn icon_set_merge_none_preserves() {
        let mut base = ThemeVariant {
            icon_set: Some(IconSet::SfSymbols),
            ..Default::default()
        };
        let overlay = ThemeVariant::default();
        base.merge(&overlay);
        assert_eq!(base.icon_set, Some(IconSet::SfSymbols));
    }

    #[test]
    fn icon_set_is_empty_when_set() {
        assert!(ThemeVariant::default().is_empty());
        let v = ThemeVariant {
            icon_set: Some(IconSet::Material),
            ..Default::default()
        };
        assert!(!v.is_empty());
    }

    #[test]
    fn icon_set_toml_round_trip() {
        let variant = ThemeVariant {
            icon_set: Some(IconSet::Material),
            ..Default::default()
        };
        let toml_str = toml::to_string(&variant).unwrap();
        assert!(toml_str.contains("icon_set"));
        let deserialized: ThemeVariant = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.icon_set, Some(IconSet::Material));
    }

    #[test]
    fn icon_set_toml_absent_deserializes_to_none() {
        let toml_str = r##"
[defaults]
accent_color = "#ff0000"
"##;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert!(variant.icon_set.is_none());
    }

    // native_theme_serde_toml_round_trip: deferred until widget renames + preset updates (Plans 02-04)

    // === from_toml_with_base tests ===

    // from_toml_with_base_merges_colors_onto_preset: deferred until preset updates (Plan 03)

    #[test]
    fn from_toml_with_base_unknown_preset_returns_error() {
        let err = ThemeSpec::from_toml_with_base("name = \"X\"", "nonexistent").unwrap_err();
        match err {
            crate::Error::Unavailable(msg) => assert!(msg.contains("nonexistent")),
            other => panic!("expected Unavailable, got: {other:?}"),
        }
    }

    #[test]
    fn from_toml_with_base_invalid_toml_returns_error() {
        let err = ThemeSpec::from_toml_with_base("{{{{invalid", "material").unwrap_err();
        match err {
            crate::Error::Format(_) => {}
            other => panic!("expected Format, got: {other:?}"),
        }
    }

    // === lint_toml tests ===

    #[test]
    fn lint_toml_valid_returns_empty() {
        let toml = r##"
name = "Valid Theme"
[light.defaults]
accent_color = "#ff0000"
background_color = "#ffffff"
[light.defaults.font]
family = "Inter"
size = 14.0
[light.button]
min_height = 32.0
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert!(
            warnings.is_empty(),
            "Expected no warnings, got: {warnings:?}"
        );
    }

    #[test]
    fn lint_toml_detects_unknown_top_level() {
        let toml = r##"
name = "Test"
theme_version = 2
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("theme_version"));
    }

    #[test]
    fn lint_toml_detects_misspelled_defaults_field() {
        let toml = r##"
name = "Test"
[light.defaults]
backround = "#ffffff"
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("backround"));
        assert!(warnings[0].contains("light.defaults.backround"));
    }

    #[test]
    fn lint_toml_detects_unknown_widget_field() {
        let toml = r##"
name = "Test"
[dark.button]
primary_bg = "#0078d7"
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("primary_bg"));
    }

    #[test]
    fn lint_toml_detects_unknown_variant_section() {
        let toml = r##"
name = "Test"
[light.badges]
color = "#ff0000"
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("badges"));
    }

    #[test]
    fn lint_toml_detects_unknown_font_subfield() {
        let toml = r##"
name = "Test"
[light.defaults.font]
famly = "Inter"
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("famly"));
    }

    #[test]
    fn lint_toml_detects_unknown_border_subfield() {
        let toml = r##"
name = "Test"
[light.defaults.border]
radiusss = 4.0
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("radiusss"));
    }

    #[test]
    fn lint_toml_detects_unknown_text_scale_entry() {
        let toml = r##"
name = "Test"
[light.text_scale.headline]
size = 24.0
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("headline"));
    }

    #[test]
    fn lint_toml_detects_unknown_text_scale_entry_field() {
        let toml = r##"
name = "Test"
[light.text_scale.caption]
font_size = 12.0
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("font_size"));
    }

    #[test]
    fn lint_toml_multiple_errors() {
        let toml = r##"
name = "Test"
author = "Me"
[light.defaults]
backround = "#ffffff"
[light.button]
primay_bg = "#0078d7"
"##;
        let warnings = ThemeSpec::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 3);
    }

    #[test]
    fn lint_toml_invalid_toml_returns_error() {
        let result = ThemeSpec::lint_toml("{{{{invalid");
        assert!(result.is_err());
    }

    // lint_toml_preset_has_no_warnings: deferred until preset updates (Plan 03)
    // lint_toml_all_presets_clean: deferred until preset updates (Plan 03)

    // === ThemeSpec layout integration tests ===

    #[test]
    fn theme_spec_layout_merge() {
        let mut base = ThemeSpec::new("Base");
        base.layout.widget_gap = Some(6.0);

        let mut overlay = ThemeSpec::new("Overlay");
        overlay.layout.container_margin = Some(8.0);

        base.merge(&overlay);
        assert_eq!(base.layout.widget_gap, Some(6.0));
        assert_eq!(base.layout.container_margin, Some(8.0));
    }

    #[test]
    fn theme_spec_layout_toml_round_trip() {
        let mut theme = ThemeSpec::new("Layout Test");
        theme.layout.widget_gap = Some(8.0);
        theme.layout.container_margin = Some(12.0);
        theme.layout.window_margin = Some(16.0);
        theme.layout.section_gap = Some(24.0);

        let toml_str = theme.to_toml().unwrap();
        let theme2 = ThemeSpec::from_toml(&toml_str).unwrap();
        assert_eq!(theme.layout, theme2.layout);
    }

    #[test]
    fn theme_spec_is_empty_with_layout() {
        let mut theme = ThemeSpec::new("Layout Only");
        assert!(theme.is_empty()); // name doesn't count
        theme.layout.widget_gap = Some(8.0);
        assert!(!theme.is_empty());
    }

    #[test]
    fn theme_spec_layout_top_level_toml() {
        let mut theme = ThemeSpec::new("Top Level");
        theme.layout.widget_gap = Some(8.0);

        let toml_str = theme.to_toml().unwrap();
        // [layout] must be at top level, not under [light.layout] or [dark.layout]
        assert!(
            toml_str.contains("[layout]"),
            "TOML should have [layout] section"
        );
        assert!(!toml_str.contains("[light.layout]"));
        assert!(!toml_str.contains("[dark.layout]"));
    }
}
