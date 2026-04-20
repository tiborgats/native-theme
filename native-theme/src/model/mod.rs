// Theme model: ThemeMode and Theme, plus sub-module re-exports

/// Light or dark color mode preference.
///
/// Used by [`SystemTheme`](crate::SystemTheme) to indicate the OS color
/// mode and by [`SystemTheme::pick()`](crate::SystemTheme::pick) to select
/// a resolved variant.
///
/// # Examples
///
/// ```
/// use native_theme::theme::ColorMode;
///
/// let mode = ColorMode::Dark;
/// assert!(mode.is_dark());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ColorMode {
    /// Light appearance.
    Light,
    /// Dark appearance.
    Dark,
}

impl ColorMode {
    /// Returns `true` if this is dark mode.
    #[must_use]
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

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

pub use animated::{
    AnimatedIcon, EmptyFrameListError, FrameList, FramesData, TransformAnimation, TransformData,
};
pub use border::{DefaultsBorderSpec, ResolvedBorderSpec, WidgetBorderSpec};
// G3 (Phase 93-03): demoted to pub(crate). Use the per-set loaders in `crate::icons` externally.
pub(crate) use bundled::{bundled_icon_by_name, bundled_icon_svg};
pub use defaults::ThemeDefaults;
pub use dialog_order::DialogButtonOrder;
pub use font::{
    FontSize, FontSpec, FontStyle, ResolvedFontSpec, TextScale, TextScaleEntry, intern_font_family,
};
pub use icon_sizes::IconSizes;
pub use icons::{
    IconData, IconProvider, IconRole, IconSet, icon_name, system_icon_set, system_icon_theme,
};
pub use resolved::{
    ResolvedDefaults, ResolvedIconSizes, ResolvedTextScale, ResolvedTextScaleEntry, ResolvedTheme,
};
pub use widgets::*; // All 25 XxxTheme + ResolvedXxxTheme pairs

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A single light or dark theme variant containing all visual properties.
///
/// Composes defaults, per-widget structs, and optional text scale into one coherent set.
/// Empty sub-structs are omitted from serialization to keep TOML files clean.
///
/// # Examples
///
/// ```
/// use native_theme::theme::ThemeMode;
/// use native_theme::color::Rgba;
///
/// let mut variant = ThemeMode::default();
/// variant.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
/// variant.defaults.font.family = Some("Inter".into());
/// assert!(!variant.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeMode {
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
}

impl_merge!(ThemeMode {
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
/// use native_theme::theme::Theme;
///
/// // Load a bundled preset
/// let theme = Theme::preset("dracula").unwrap();
/// assert_eq!(theme.name, "Dracula");
///
/// // Parse from a TOML string
/// let toml = r##"
/// name = "Custom"
/// [light.defaults]
/// accent_color = "#ff6600"
/// "##;
/// let custom = Theme::from_toml(toml).unwrap();
/// assert_eq!(custom.name, "Custom");
///
/// // Merge themes (overlay wins for populated fields)
/// let mut base = Theme::preset("catppuccin-mocha").unwrap();
/// base.merge(&custom);
/// assert_eq!(base.name, "Catppuccin Mocha"); // base name is preserved
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name (e.g., "Breeze", "Adwaita", "Windows 11").
    ///
    /// Uses `Cow<'static, str>` so bundled presets can store borrowed
    /// `&'static str` values without per-load `String` allocations.
    /// User-provided names (from TOML files or runtime detection)
    /// are `Cow::Owned`.
    pub name: Cow<'static, str>,

    /// Light variant of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light: Option<ThemeMode>,

    /// Dark variant of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark: Option<ThemeMode>,

    /// Layout spacing constants (shared between light and dark variants).
    #[serde(default, skip_serializing_if = "LayoutTheme::is_empty")]
    pub layout: LayoutTheme,

    /// Which icon loading mechanism to use (`Freedesktop`, `Material`, `Lucide`,
    /// `SfSymbols`, `SegoeIcons`). Shared across light and dark variants.
    /// When `None`, filled during resolution from
    /// [`system_icon_set()`](crate::theme::system_icon_set).
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::{Theme, IconSet};
    ///
    /// let theme = Theme::preset("material")?;
    /// assert_eq!(theme.icon_set, Some(IconSet::Material));
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_set: Option<IconSet>,

    /// Visual icon theme name (shared across light and dark variants).
    ///
    /// Acts as the default when a variant's
    /// [`ThemeDefaults::icon_theme`](crate::model::ThemeDefaults::icon_theme)
    /// is `None`. Variants that need a different icon theme per color mode
    /// (e.g. KDE Plasma: `"breeze"` light / `"breeze-dark"` dark) set the
    /// override on [`ThemeDefaults::icon_theme`].
    ///
    /// Precedence at resolve time:
    /// 1. [`ThemeMode::defaults.icon_theme`](crate::model::ThemeDefaults::icon_theme) — per-variant override (if set)
    /// 2. `Theme::icon_theme` — this field (if set)
    /// 3. [`system_icon_theme()`](crate::model::icons::system_icon_theme) — runtime fallback
    ///
    /// See doc 1 §20 and `docs/todo_v0.5.7_gaps.md` §G4 for the design rationale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_theme: Option<Cow<'static, str>>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: Cow::Borrowed(""),
            light: None,
            dark: None,
            layout: LayoutTheme::default(),
            icon_set: None,
            icon_theme: None,
        }
    }
}

impl Theme {
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

        if overlay.icon_set.is_some() {
            self.icon_set = overlay.icon_set;
        }

        if overlay.icon_theme.is_some() {
            self.icon_theme.clone_from(&overlay.icon_theme);
        }
    }

    /// Pick the appropriate variant for the given mode, with cross-fallback.
    ///
    /// When `mode` is [`ColorMode::Dark`], prefers `dark` and falls back to `light`.
    /// When `mode` is [`ColorMode::Light`], prefers `light` and falls back to `dark`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoVariant`](crate::Error::NoVariant) if the theme has
    /// no variants at all.
    pub fn pick_variant(&self, mode: ColorMode) -> crate::Result<&ThemeMode> {
        match mode {
            ColorMode::Dark => self.dark.as_ref().or(self.light.as_ref()),
            ColorMode::Light => self.light.as_ref().or(self.dark.as_ref()),
        }
        .ok_or(crate::Error::NoVariant { mode })
    }

    /// Extract a variant by consuming the theme, avoiding a clone.
    ///
    /// When `mode` is [`ColorMode::Dark`], returns the `dark` variant (falling back to
    /// `light`). When [`ColorMode::Light`], returns `light` (falling back to `dark`).
    ///
    /// Use this when you own the `Theme` and don't need it afterward.
    /// For read-only inspection, use [`pick_variant()`](Self::pick_variant).
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoVariant`](crate::Error::NoVariant) if the theme has
    /// no variants at all.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::ColorMode;
    ///
    /// let theme = native_theme::theme::Theme::preset("dracula")?;
    /// let variant = theme.into_variant(ColorMode::Dark)?;
    /// let resolved = variant.resolve_system()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_variant(self, mode: ColorMode) -> crate::Result<ThemeMode> {
        match mode {
            ColorMode::Dark => self.dark.or(self.light),
            ColorMode::Light => self.light.or(self.dark),
        }
        .ok_or(crate::Error::NoVariant { mode })
    }

    /// Returns true if the theme has no variants set.
    pub fn is_empty(&self) -> bool {
        self.light.is_none()
            && self.dark.is_none()
            && self.layout.is_empty()
            && self.icon_set.is_none()
            && self.icon_theme.is_none()
    }

    /// Load a bundled theme preset by name.
    ///
    /// Returns the preset as a fully populated [`Theme`] with both
    /// light and dark variants.
    ///
    /// # Errors
    /// Returns [`crate::Error::UnknownPreset`] if the preset name is not recognized.
    ///
    /// # Examples
    /// ```
    /// let theme = native_theme::theme::Theme::preset("catppuccin-mocha")?;
    /// assert!(theme.light.is_some());
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    ///
    /// Bundled preset names are `Cow::Borrowed` (no allocation):
    /// ```
    /// use native_theme::theme::Theme;
    ///
    /// let theme = Theme::preset("dracula")?;
    /// assert!(matches!(theme.name, std::borrow::Cow::Borrowed(_)));
    /// # Ok::<(), native_theme::error::Error>(())
    /// ```
    pub fn preset(name: &str) -> crate::Result<Self> {
        crate::presets::preset(name)
    }

    /// Parse a TOML string into a [`Theme`].
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
    /// Returns [`crate::Error::Toml`] if the TOML is invalid.
    ///
    /// # Examples
    /// ```
    /// let toml = r##"
    /// name = "My Theme"
    /// [light.defaults]
    /// accent_color = "#ff0000"
    /// "##;
    /// let theme = native_theme::theme::Theme::from_toml(toml).unwrap();
    /// assert_eq!(theme.name, "My Theme");
    /// ```
    pub fn from_toml(toml_str: &str) -> crate::Result<Self> {
        crate::presets::from_toml(toml_str)
    }

    /// Load a [`Theme`] from a TOML file.
    ///
    /// # Errors
    /// Returns [`crate::Error::Io`] if the file cannot be read, or
    /// [`crate::Error::Toml`] if the TOML content is invalid.
    ///
    /// # Examples
    /// ```no_run
    /// let theme = native_theme::theme::Theme::from_file("my-theme.toml").unwrap();
    /// ```
    pub fn from_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        crate::presets::from_file(path)
    }

    /// List all available bundled presets with structured metadata.
    ///
    /// Returns a static slice of [`PresetInfo`](crate::presets::PresetInfo) entries,
    /// one per bundled preset. Each entry carries the machine-readable key,
    /// human-readable display name, target platform tags, and a `light_only` flag.
    ///
    /// # Examples
    /// ```
    /// let presets = native_theme::theme::Theme::list_presets();
    /// assert_eq!(presets.len(), 16);
    /// assert_eq!(presets[0].key, "kde-breeze");
    /// assert_eq!(presets[0].display_name, "KDE Breeze");
    /// ```
    #[must_use]
    pub fn list_presets() -> &'static [crate::presets::PresetInfo] {
        crate::presets::list_presets()
    }

    /// List presets appropriate for the current platform, with structured metadata.
    ///
    /// Platform-specific presets (kde-breeze, adwaita, windows-11, macos-sonoma, ios)
    /// are only included on their native platform. Community themes are always included.
    ///
    /// Note: Unlike [`list_presets()`](Self::list_presets) which returns a static slice,
    /// this method returns `Vec` because it filters the preset list at runtime based
    /// on the detected platform.
    ///
    /// # Examples
    /// ```
    /// let presets = native_theme::theme::Theme::list_presets_for_platform();
    /// // On Linux KDE: includes kde-breeze, adwaita, plus all community themes
    /// // On Windows: includes windows-11 plus all community themes
    /// assert!(!presets.is_empty());
    /// ```
    #[must_use]
    pub fn list_presets_for_platform() -> Vec<crate::presets::PresetInfo> {
        crate::presets::list_presets_for_platform()
    }

    /// Serialize this theme to a TOML string.
    ///
    /// # Errors
    /// Returns [`crate::Error::ReaderFailed`] if serialization fails.
    ///
    /// # Examples
    /// ```
    /// let theme = native_theme::theme::Theme::preset("catppuccin-mocha").unwrap();
    /// let toml_str = theme.to_toml().unwrap();
    /// assert!(toml_str.contains("name = \"Catppuccin Mocha\""));
    /// ```
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
    /// let warnings = native_theme::theme::Theme::lint_toml(r##"
    /// name = "Test"
    /// [light.defaults]
    /// backround = "#ffffff"
    /// "##).unwrap();
    /// assert_eq!(warnings.len(), 1);
    /// assert!(warnings[0].contains("backround"));
    /// ```
    pub fn lint_toml(toml_str: &str) -> crate::Result<Vec<String>> {
        let value: toml::Value =
            toml::from_str(toml_str).map_err(|e: toml::de::Error| crate::Error::Toml(e))?;

        let mut warnings = Vec::new();

        let top_table = match &value {
            toml::Value::Table(t) => t,
            _ => return Ok(warnings),
        };

        // Known top-level keys
        const TOP_KEYS: &[&str] = &["name", "light", "dark", "layout", "icon_set", "icon_theme"];

        for key in top_table.keys() {
            if !TOP_KEYS.contains(&key.as_str()) {
                warnings.push(format!("unknown field: {key}"));
            }
        }

        // Structural variant-level keys that are NOT widgets
        const STRUCTURAL_KEYS: &[&str] = &["defaults", "text_scale"];

        // Phase 93-05 G5: build BOTH field registries from inventory.
        // - widget_registry: one entry per per-variant widget (ButtonTheme ->
        //   "button", etc.) populated by #[derive(ThemeWidget)].
        // - struct_registry: one entry per plain struct (FontSpec, IconSizes,
        //   ThemeDefaults, LayoutTheme, ...) populated by #[derive(ThemeFields)].
        let widget_registry: std::collections::HashMap<&str, &[&str]> =
            inventory::iter::<crate::resolve::WidgetFieldInfo>()
                .map(|info| (info.widget_name, info.field_names))
                .collect();
        let struct_registry: std::collections::HashMap<&str, &[&str]> =
            inventory::iter::<crate::resolve::FieldInfo>()
                .map(|info| (info.struct_name, info.field_names))
                .collect();

        // Helper: fetch a plain-struct field list by type name. Returns None
        // when the struct did not opt in to ThemeFields -- sub-table linting
        // is silently skipped in that case, matching the former `continue;`
        // behaviour when a sub-table's type wasn't recognised.
        let get_struct_fields =
            |name: &str| -> Option<&[&str]> { struct_registry.get(name).copied() };

        // Lint a text_scale section
        let lint_text_scale = |table: &toml::map::Map<String, toml::Value>,
                               prefix: &str,
                               warnings: &mut Vec<String>| {
            let Some(scale_fields) = get_struct_fields("TextScale") else {
                return;
            };
            let entry_fields = get_struct_fields("TextScaleEntry");
            for key in table.keys() {
                if !scale_fields.contains(&key.as_str()) {
                    warnings.push(format!("unknown field: {prefix}.{key}"));
                } else if let Some(toml::Value::Table(entry_table)) = table.get(key)
                    && let Some(entry_fields) = entry_fields
                {
                    for ekey in entry_table.keys() {
                        if !entry_fields.contains(&ekey.as_str()) {
                            warnings.push(format!("unknown field: {prefix}.{key}.{ekey}"));
                        }
                    }
                }
            }
        };

        // Lint a defaults section (with nested font, mono_font, border, icon_sizes)
        let lint_defaults = |table: &toml::map::Map<String, toml::Value>,
                             prefix: &str,
                             warnings: &mut Vec<String>| {
            let Some(defaults_fields) = get_struct_fields("ThemeDefaults") else {
                return;
            };
            for key in table.keys() {
                if !defaults_fields.contains(&key.as_str()) {
                    warnings.push(format!("unknown field: {prefix}.{key}"));
                    continue;
                }
                // Check sub-tables for nested struct fields
                if let Some(toml::Value::Table(sub)) = table.get(key) {
                    let known = match key.as_str() {
                        "font" | "mono_font" => get_struct_fields("FontSpec"),
                        "border" => get_struct_fields("DefaultsBorderSpec"),
                        "icon_sizes" => get_struct_fields("IconSizes"),
                        _ => continue,
                    };
                    let Some(known) = known else { continue };
                    for skey in sub.keys() {
                        if !known.contains(&skey.as_str()) {
                            warnings.push(format!("unknown field: {prefix}.{key}.{skey}"));
                        }
                    }
                }
            }
        };

        // Lint a variant section (light or dark).
        let lint_variant = |table: &toml::map::Map<String, toml::Value>,
                            prefix: &str,
                            warnings: &mut Vec<String>| {
            for key in table.keys() {
                let key_str = key.as_str();

                // Check structural keys first, then widget registry
                let is_structural = STRUCTURAL_KEYS.contains(&key_str);
                let widget_fields = widget_registry.get(key_str);

                if !is_structural && widget_fields.is_none() {
                    warnings.push(format!("unknown field: {prefix}.{key}"));
                    continue;
                }

                if let Some(toml::Value::Table(sub)) = table.get(key) {
                    let sub_prefix = format!("{prefix}.{key}");
                    match key_str {
                        "defaults" => lint_defaults(sub, &sub_prefix, warnings),
                        "text_scale" => lint_text_scale(sub, &sub_prefix, warnings),
                        _ => {
                            if let Some(fields) = widget_fields {
                                for skey in sub.keys() {
                                    if !fields.contains(&skey.as_str()) {
                                        warnings
                                            .push(format!("unknown field: {sub_prefix}.{skey}"));
                                    }
                                    // Validate sub-tables (font/border nested structs)
                                    if let Some(toml::Value::Table(nested)) = sub.get(skey) {
                                        let nested_known = match skey.as_str() {
                                            s if s == "font" || s.ends_with("_font") => {
                                                get_struct_fields("FontSpec")
                                            }
                                            "border" => get_struct_fields("WidgetBorderSpec"),
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
        };

        // Lint light and dark variant sections
        for variant_key in &["light", "dark"] {
            if let Some(toml::Value::Table(variant_table)) = top_table.get(*variant_key) {
                lint_variant(variant_table, variant_key, &mut warnings);
            }
        }

        // Lint top-level [layout] section. LayoutTheme is a "widget" at the
        // macro level (for its Resolved-pair codegen) but skips the widget
        // inventory -- its fields are in the struct registry instead.
        if let Some(toml::Value::Table(layout_table)) = top_table.get("layout")
            && let Some(layout_fields) = get_struct_fields("LayoutTheme")
        {
            for key in layout_table.keys() {
                if !layout_fields.contains(&key.as_str()) {
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

    // === ThemeMode tests ===

    #[test]
    fn theme_variant_default_is_empty() {
        assert!(ThemeMode::default().is_empty());
    }

    #[test]
    fn theme_variant_not_empty_when_color_set() {
        let mut v = ThemeMode::default();
        v.defaults.accent_color = Some(Rgba::rgb(0, 120, 215));
        assert!(!v.is_empty());
    }

    #[test]
    fn theme_variant_not_empty_when_font_set() {
        let mut v = ThemeMode::default();
        v.defaults.font.family = Some("Inter".into());
        assert!(!v.is_empty());
    }

    #[test]
    fn theme_variant_merge_recursively() {
        let mut base = ThemeMode::default();
        base.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        base.defaults.font.family = Some("Noto Sans".into());

        let mut overlay = ThemeMode::default();
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
        let mut v = ThemeMode::default();
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
        let mut base = ThemeMode::default();
        base.button.background_color = Some(Rgba::rgb(200, 200, 200));
        base.button.min_height = Some(28.0);
        base.tooltip.background_color = Some(Rgba::rgb(50, 50, 50));

        let mut overlay = ThemeMode::default();
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

    // === Theme tests ===

    #[test]
    fn native_theme_default_is_empty() {
        let theme = Theme::default();
        assert!(theme.is_empty());
        assert_eq!(theme.name, "");
    }

    #[test]
    fn native_theme_merge_keeps_base_name() {
        let mut base = Theme {
            name: "Base Theme".into(),
            ..Theme::default()
        };
        let overlay = Theme {
            name: "Overlay Theme".into(),
            ..Theme::default()
        };
        base.merge(&overlay);
        assert_eq!(base.name, "Base Theme");
    }

    #[test]
    fn native_theme_merge_overlay_light_into_none() {
        let mut base = Theme {
            name: "Theme".into(),
            ..Theme::default()
        };

        let mut overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        };
        let mut light = ThemeMode::default();
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
        let mut base = Theme {
            name: "Theme".into(),
            ..Theme::default()
        };
        let mut base_light = ThemeMode::default();
        base_light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        base.light = Some(base_light);

        let mut overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        };
        let mut overlay_light = ThemeMode::default();
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
        let mut base = Theme {
            name: "Theme".into(),
            ..Theme::default()
        };
        let mut base_light = ThemeMode::default();
        base_light.defaults.font.family = Some("Inter".into());
        base.light = Some(base_light);

        let overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        }; // no light

        base.merge(&overlay);

        assert!(base.light.is_some());
        assert_eq!(
            base.light.as_ref().unwrap().defaults.font.family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn native_theme_merge_dark_variant() {
        let mut base = Theme {
            name: "Theme".into(),
            ..Theme::default()
        };

        let mut overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        };
        let mut dark = ThemeMode::default();
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
        let mut theme = Theme {
            name: "Theme".into(),
            ..Theme::default()
        };
        theme.light = Some(ThemeMode::default());
        assert!(!theme.is_empty());
    }

    // === pick_variant tests ===

    #[test]
    fn pick_variant_dark_with_both_variants_returns_dark() {
        let mut theme = Theme {
            name: "Test".into(),
            ..Theme::default()
        };
        let mut light = ThemeMode::default();
        light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeMode::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(ColorMode::Dark).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn pick_variant_light_with_both_variants_returns_light() {
        let mut theme = Theme {
            name: "Test".into(),
            ..Theme::default()
        };
        let mut light = ThemeMode::default();
        light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);
        let mut dark = ThemeMode::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(ColorMode::Light).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(255, 255, 255))
        );
    }

    #[test]
    fn pick_variant_dark_with_only_light_falls_back() {
        let mut theme = Theme {
            name: "Test".into(),
            ..Theme::default()
        };
        let mut light = ThemeMode::default();
        light.defaults.background_color = Some(Rgba::rgb(255, 255, 255));
        theme.light = Some(light);

        let picked = theme.pick_variant(ColorMode::Dark).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(255, 255, 255))
        );
    }

    #[test]
    fn pick_variant_light_with_only_dark_falls_back() {
        let mut theme = Theme {
            name: "Test".into(),
            ..Theme::default()
        };
        let mut dark = ThemeMode::default();
        dark.defaults.background_color = Some(Rgba::rgb(30, 30, 30));
        theme.dark = Some(dark);

        let picked = theme.pick_variant(ColorMode::Light).unwrap();
        assert_eq!(
            picked.defaults.background_color,
            Some(Rgba::rgb(30, 30, 30))
        );
    }

    #[test]
    fn pick_variant_with_no_variants_returns_err() {
        let theme = Theme {
            name: "Empty".into(),
            ..Theme::default()
        };
        assert!(theme.pick_variant(ColorMode::Dark).is_err());
        assert!(theme.pick_variant(ColorMode::Light).is_err());
    }

    // === icon_set tests (on Theme, shared across variants) ===

    #[test]
    fn icon_set_default_is_none() {
        assert!(Theme::default().icon_set.is_none());
    }

    #[test]
    fn icon_set_merge_overlay() {
        let mut base = Theme {
            name: "Base".into(),
            ..Theme::default()
        };
        let mut overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        };
        overlay.icon_set = Some(IconSet::Material);
        base.merge(&overlay);
        assert_eq!(base.icon_set, Some(IconSet::Material));
    }

    #[test]
    fn icon_set_merge_none_preserves() {
        let mut base = Theme {
            name: "Base".into(),
            ..Theme::default()
        };
        base.icon_set = Some(IconSet::SfSymbols);
        let overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        };
        base.merge(&overlay);
        assert_eq!(base.icon_set, Some(IconSet::SfSymbols));
    }

    #[test]
    fn icon_set_is_empty_when_set() {
        assert!(Theme::default().is_empty());
        let mut t = Theme {
            name: "Test".into(),
            ..Theme::default()
        };
        t.icon_set = Some(IconSet::Material);
        assert!(!t.is_empty());
    }

    #[test]
    fn icon_set_toml_round_trip() {
        let mut theme = Theme {
            name: "Test".into(),
            ..Theme::default()
        };
        theme.icon_set = Some(IconSet::Material);
        let mut light = ThemeMode::default();
        light.defaults.icon_theme = Some("material".into());
        theme.light = Some(light);
        let toml_str = theme.to_toml().unwrap();
        assert!(toml_str.contains("icon_set"));
        let deserialized = Theme::from_toml(&toml_str).unwrap();
        assert_eq!(deserialized.icon_set, Some(IconSet::Material));
        assert_eq!(
            deserialized
                .light
                .as_ref()
                .unwrap()
                .defaults
                .icon_theme
                .as_deref(),
            Some("material")
        );
    }

    #[test]
    fn icon_set_toml_absent_deserializes_to_none() {
        let toml_str = r##"
name = "Bare"
[light.defaults]
accent_color = "#ff0000"
"##;
        let theme = Theme::from_toml(toml_str).unwrap();
        assert!(theme.icon_set.is_none());
        assert!(theme.light.as_ref().unwrap().defaults.icon_theme.is_none());
    }

    #[test]
    fn native_theme_serde_toml_round_trip() {
        // Load a preset, serialize to TOML, deserialize back, and verify equality
        let theme = Theme::preset("material").expect("material preset should load");
        let toml_str = theme.to_toml().expect("should serialize");
        let theme2 = Theme::from_toml(&toml_str).expect("should deserialize");
        assert_eq!(theme, theme2, "round-trip should preserve Theme");
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
size_px = 14.0
[light.button]
min_height_px = 32.0
"##;
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
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
        let warnings = Theme::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 3);
    }

    #[test]
    fn lint_toml_invalid_toml_returns_error() {
        let result = Theme::lint_toml("{{{{invalid");
        assert!(result.is_err());
    }

    #[test]
    fn lint_toml_preset_has_no_warnings() {
        // Spot-check one preset for lint cleanliness via round-trip
        let theme = Theme::preset("material").expect("material preset should load");
        let toml_str = theme.to_toml().expect("should serialize");
        let warnings = Theme::lint_toml(&toml_str).expect("should parse");
        assert!(
            warnings.is_empty(),
            "material preset should have no lint warnings, got: {warnings:?}"
        );
    }

    #[test]
    fn lint_toml_all_presets_clean() {
        for info in Theme::list_presets() {
            let name = info.key;
            // Load the raw TOML source for each preset via include_str
            // by loading via preset() + to_toml() round-trip
            let theme = Theme::preset(name).unwrap_or_else(|e| {
                panic!("preset {name} should load: {e}");
            });
            let toml_str = theme.to_toml().unwrap_or_else(|e| {
                panic!("preset {name} should serialize: {e}");
            });
            let warnings = Theme::lint_toml(&toml_str).unwrap_or_else(|e| {
                panic!("preset {name} should lint: {e}");
            });
            assert!(
                warnings.is_empty(),
                "preset {name} should have no lint warnings, got: {warnings:?}"
            );
        }
    }

    #[test]
    fn lint_toml_rejects_unknown_field_on_registered_widget() {
        // Verify that lint_toml discovers widget field names from inventory.
        // If a field name is not in the registered FIELD_NAMES for a widget,
        // lint_toml should report it as unknown.
        let toml = r##"
name = "Test"
[light.button]
nonexistent_field = "#ff0000"
"##;
        let warnings = Theme::lint_toml(toml).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("nonexistent_field"));
        assert!(warnings[0].contains("light.button"));
    }

    #[test]
    fn lint_toml_recognizes_all_registered_widgets() {
        // Every widget registered via inventory::submit! should be accepted
        // as a valid variant-level section key.
        for entry in inventory::iter::<crate::resolve::WidgetFieldInfo> {
            let toml_str = format!("name = \"Test\"\n[light.{}]\n", entry.widget_name,);
            let warnings = Theme::lint_toml(&toml_str).unwrap();
            assert!(
                warnings.is_empty(),
                "widget '{}' should be recognized, got: {:?}",
                entry.widget_name,
                warnings,
            );
        }
    }

    // === Theme layout integration tests ===

    #[test]
    fn theme_spec_layout_merge() {
        let mut base = Theme {
            name: "Base".into(),
            ..Theme::default()
        };
        base.layout.widget_gap = Some(6.0);

        let mut overlay = Theme {
            name: "Overlay".into(),
            ..Theme::default()
        };
        overlay.layout.container_margin = Some(8.0);

        base.merge(&overlay);
        assert_eq!(base.layout.widget_gap, Some(6.0));
        assert_eq!(base.layout.container_margin, Some(8.0));
    }

    #[test]
    fn theme_spec_layout_toml_round_trip() {
        let mut theme = Theme {
            name: "Layout Test".into(),
            ..Theme::default()
        };
        theme.layout.widget_gap = Some(8.0);
        theme.layout.container_margin = Some(12.0);
        theme.layout.window_margin = Some(16.0);
        theme.layout.section_gap = Some(24.0);

        let toml_str = theme.to_toml().unwrap();
        let theme2 = Theme::from_toml(&toml_str).unwrap();
        assert_eq!(theme.layout, theme2.layout);
    }

    #[test]
    fn theme_spec_is_empty_with_layout() {
        let mut theme = Theme {
            name: "Layout Only".into(),
            ..Theme::default()
        };
        assert!(theme.is_empty()); // name doesn't count
        theme.layout.widget_gap = Some(8.0);
        assert!(!theme.is_empty());
    }

    #[test]
    fn theme_spec_layout_top_level_toml() {
        let mut theme = Theme {
            name: "Top Level".into(),
            ..Theme::default()
        };
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

    // === Phase 93-05 G5: ThemeFields inventory baseline-equality tests ===
    //
    // Each struct that is expected to register via #[derive(ThemeFields)]
    // must produce a FieldInfo entry whose `field_names` matches the
    // pre-migration hand-authored FIELD_NAMES list bit-for-bit. Failure means
    // either the derive emission path is wrong (serde rename drift) or
    // the derive was not applied to the struct.

    fn field_info_entry(name: &'static str) -> Option<&'static [&'static str]> {
        inventory::iter::<crate::resolve::FieldInfo>()
            .find(|info| info.struct_name == name)
            .map(|info| info.field_names)
    }

    #[test]
    fn font_spec_field_info_matches_baseline() {
        let baseline: &[&str] = &["family", "size_pt", "size_px", "weight", "style", "color"];
        assert_eq!(field_info_entry("FontSpec"), Some(baseline));
    }

    #[test]
    fn text_scale_entry_field_info_matches_baseline() {
        let baseline: &[&str] = &[
            "size_pt",
            "size_px",
            "weight",
            "line_height_pt",
            "line_height_px",
        ];
        assert_eq!(field_info_entry("TextScaleEntry"), Some(baseline));
    }

    #[test]
    fn text_scale_field_info_matches_baseline() {
        let baseline: &[&str] = &["caption", "section_heading", "dialog_title", "display"];
        assert_eq!(field_info_entry("TextScale"), Some(baseline));
    }

    #[test]
    fn defaults_border_spec_field_info_matches_baseline() {
        let baseline: &[&str] = &[
            "color",
            "corner_radius_px",
            "corner_radius_lg_px",
            "line_width_px",
            "opacity",
            "shadow_enabled",
        ];
        assert_eq!(field_info_entry("DefaultsBorderSpec"), Some(baseline));
    }

    #[test]
    fn widget_border_spec_field_info_matches_baseline() {
        let baseline: &[&str] = &[
            "color",
            "corner_radius_px",
            "line_width_px",
            "shadow_enabled",
            "padding_horizontal_px",
            "padding_vertical_px",
        ];
        assert_eq!(field_info_entry("WidgetBorderSpec"), Some(baseline));
    }

    #[test]
    fn theme_defaults_field_info_matches_baseline() {
        let baseline: &[&str] = &[
            "font",
            "line_height",
            "mono_font",
            "background_color",
            "text_color",
            "accent_color",
            "accent_text_color",
            "surface_color",
            "muted_color",
            "shadow_color",
            "link_color",
            "selection_background",
            "selection_text_color",
            "selection_inactive_background",
            "text_selection_background",
            "text_selection_color",
            "disabled_text_color",
            "danger_color",
            "danger_text_color",
            "warning_color",
            "warning_text_color",
            "success_color",
            "success_text_color",
            "info_color",
            "info_text_color",
            "border",
            "disabled_opacity",
            "focus_ring_color",
            "focus_ring_width_px",
            "focus_ring_offset_px",
            "icon_sizes",
            "icon_theme",
        ];
        assert_eq!(field_info_entry("ThemeDefaults"), Some(baseline));
    }

    #[test]
    fn icon_sizes_field_info_matches_baseline() {
        let baseline: &[&str] = &[
            "toolbar_px",
            "small_px",
            "large_px",
            "dialog_px",
            "panel_px",
        ];
        assert_eq!(field_info_entry("IconSizes"), Some(baseline));
    }

    #[test]
    fn layout_theme_field_info_matches_baseline() {
        let baseline: &[&str] = &[
            "widget_gap_px",
            "container_margin_px",
            "window_margin_px",
            "section_gap_px",
        ];
        assert_eq!(field_info_entry("LayoutTheme"), Some(baseline));
    }

    #[test]
    fn widget_registry_still_has_all_widgets() {
        // Regression guard: the existing WidgetFieldInfo inventory must remain
        // populated after ThemeFields migration. LayoutTheme is NOT a widget
        // (it uses `skip_inventory`), so we expect at least the 25 per-variant
        // widgets registered through ThemeWidget.
        let widget_count = inventory::iter::<crate::resolve::WidgetFieldInfo>().count();
        assert!(
            widget_count >= 25,
            "expected >=25 widgets in WidgetFieldInfo, got {widget_count}"
        );
    }
}
