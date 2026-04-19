//! Icon loading and dispatch.

use std::fmt;

#[allow(unused_imports)]
use std::borrow::Cow;

#[allow(unused_imports)]
use crate::model::icons::{icon_name, system_icon_set, system_icon_theme};
use crate::model::{AnimatedIcon, IconData, IconProvider, IconRole, IconSet};
#[allow(unused_imports)]
use crate::model::{bundled_icon_by_name, bundled_icon_svg};

/// Identifies what icon to load: a semantic role, a platform-specific name,
/// or a custom provider.
///
/// Constructed via [`From`] impls -- users rarely reference this type directly.
/// Pass an [`IconRole`], `&str`, or `&dyn IconProvider` to
/// [`IconLoader::new()`].
///
/// # Examples
///
/// ```
/// use native_theme::icons::IconId;
/// use native_theme::theme::IconRole;
///
/// // All of these work via Into<IconId>:
/// let _: IconId = IconRole::ActionSave.into();
/// let _: IconId = "edit-copy".into();
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum IconId<'a> {
    /// Load by semantic role (most common).
    Role(IconRole),
    /// Load by platform-specific name string.
    Name(&'a str),
    /// Load via a custom [`IconProvider`] implementation.
    Custom(&'a dyn IconProvider),
}

impl From<IconRole> for IconId<'_> {
    fn from(role: IconRole) -> Self {
        IconId::Role(role)
    }
}

impl<'a> From<&'a str> for IconId<'a> {
    fn from(name: &'a str) -> Self {
        IconId::Name(name)
    }
}

impl<'a> From<&'a dyn IconProvider> for IconId<'a> {
    fn from(provider: &'a dyn IconProvider) -> Self {
        IconId::Custom(provider)
    }
}

/// Builder for loading icons with optional configuration.
///
/// Replaces the 13 standalone icon-loading functions with a single
/// fluent API. The common case stays short:
///
/// ```
/// use native_theme::icons::IconLoader;
/// use native_theme::theme::{IconRole, IconSet};
///
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = IconLoader::new(IconRole::ActionCopy)
///     .set(IconSet::Material)
///     .load();
/// assert!(icon.is_some());
/// # }
/// ```
///
/// Advanced usage with all options:
///
/// ```ignore
/// let icon = IconLoader::new(IconRole::ActionSave)
///     .set(IconSet::Freedesktop)
///     .size(48)
///     .color([0, 0, 0])
///     .theme("Adwaita")
///     .load();
/// ```
#[derive(Debug)]
#[must_use]
pub struct IconLoader<'a> {
    id: IconId<'a>,
    set: IconSet,
    /// Requested icon size in pixels. Only affects freedesktop lookups.
    pub(crate) size: u16,
    fg_color: Option<[u8; 3]>,
    freedesktop_theme: Option<&'a str>,
}

impl<'a> IconLoader<'a> {
    /// Create a new icon loader for the given icon identifier.
    ///
    /// Accepts [`IconRole`], `&str` (platform-specific name), or
    /// `&dyn IconProvider` via [`Into<IconId>`].
    ///
    /// Defaults:
    /// - `set`: [`system_icon_set()`] (platform-appropriate)
    /// - `size`: 24
    /// - `fg_color`: `None`
    /// - `freedesktop_theme`: `None` (uses system theme)
    pub fn new(id: impl Into<IconId<'a>>) -> Self {
        Self {
            id: id.into(),
            set: system_icon_set(),
            size: 24,
            fg_color: None,
            freedesktop_theme: None,
        }
    }

    /// Set the icon set to load from.
    ///
    /// Overrides the default platform-detected icon set.
    pub fn set(mut self, set: IconSet) -> Self {
        self.set = set;
        self
    }

    /// Set the requested icon size in pixels.
    ///
    /// Used by freedesktop theme lookups. Bundled SVG sets (Material, Lucide)
    /// and platform APIs (SF Symbols, Segoe Fluent) ignore this value
    /// since they provide scalable or render-time-sized icons.
    ///
    /// Default: 24.
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Set the foreground color for GTK symbolic icon colorization.
    ///
    /// When loading freedesktop icons with `-symbolic` suffix, the SVG's
    /// hardcoded foreground placeholders are replaced with this color.
    /// Pass `None` (the default) to fall back to `currentColor`.
    ///
    /// Ignored for non-freedesktop icon sets.
    pub fn color(mut self, rgb: [u8; 3]) -> Self {
        self.fg_color = Some(rgb);
        self
    }

    /// Set the foreground color if `Some`, or leave as default (`None`).
    ///
    /// Convenience method for call sites that have an `Option<[u8; 3]>`.
    pub fn color_opt(mut self, rgb: Option<[u8; 3]>) -> Self {
        self.fg_color = rgb;
        self
    }

    /// Set the freedesktop icon theme to load from.
    ///
    /// Overrides the system-detected icon theme. Only affects
    /// [`IconSet::Freedesktop`] lookups. Ignored for bundled and
    /// platform icon sets.
    ///
    /// Use [`is_freedesktop_theme_available()`] to check whether a theme
    /// is installed before specifying it.
    pub fn theme(mut self, theme: &'a str) -> Self {
        self.freedesktop_theme = Some(theme);
        self
    }

    /// Load the icon, returning its data.
    ///
    /// Dispatches to the appropriate platform loader or bundled icon set.
    /// Returns `None` when the icon is not found, the required feature is
    /// not enabled, or the platform doesn't match the requested set.
    ///
    /// **No cross-set fallback** -- each set is self-contained.
    #[must_use]
    pub fn load(self) -> Option<IconData> {
        match self.id {
            IconId::Role(role) => self.load_role(role),
            IconId::Name(name) => self.load_by_name(name),
            IconId::Custom(provider) => self.load_custom(provider),
        }
    }

    /// Load the animated loading/spinner indicator for the current icon set.
    ///
    /// Ignores the `id` field -- the indicator is per-set, not per-role.
    ///
    /// # Dispatch
    ///
    /// - [`IconSet::Material`] -- bundled spinner with continuous spin
    /// - [`IconSet::Lucide`] -- bundled spinner with continuous spin
    /// - [`IconSet::Freedesktop`] -- `process-working` sprite sheet from active theme
    /// - Other sets -- `None`
    #[must_use]
    pub fn load_indicator(self) -> Option<AnimatedIcon> {
        loading_indicator_inner(self.set)
    }

    /// Internal: load by role, dispatching to platform or bundled loaders.
    fn load_role(self, role: IconRole) -> Option<IconData> {
        // If a freedesktop theme override is specified, use it
        if let Some(theme) = self.freedesktop_theme {
            return load_icon_from_theme_inner(role, self.set, theme, self.size, self.fg_color);
        }
        load_icon_inner(role, self.set, self.size, self.fg_color)
    }

    /// Internal: load by platform-specific name string.
    fn load_by_name(self, name: &str) -> Option<IconData> {
        load_system_icon_by_name_inner(name, self.set, self.size, self.fg_color)
    }

    /// Internal: load via custom IconProvider.
    fn load_custom(self, provider: &dyn IconProvider) -> Option<IconData> {
        // Step 1: Try system loader with provider's name mapping
        if let Some(name) = provider.icon_name(self.set)
            && let Some(data) =
                load_system_icon_by_name_inner(name, self.set, self.size, self.fg_color)
        {
            return Some(data);
        }

        // Step 2: Try bundled SVG from provider
        if let Some(svg) = provider.icon_svg(self.set) {
            return Some(IconData::Svg(svg));
        }

        // No cross-set fallback
        None
    }
}

/// Check whether a freedesktop icon theme is installed on this system.
///
/// Looks for the theme's `index.theme` file in the standard XDG icon
/// directories (`$XDG_DATA_DIRS/icons/<theme>/` and
/// `$XDG_DATA_HOME/icons/<theme>/`).
///
/// Always returns `false` on non-Linux platforms.
#[must_use]
pub fn is_freedesktop_theme_available(theme: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
        let data_dirs = std::env::var("XDG_DATA_DIRS")
            .unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());
        for dir in data_dirs.split(':') {
            if std::path::Path::new(dir)
                .join("icons")
                .join(theme)
                .join("index.theme")
                .exists()
            {
                return true;
            }
        }
        let data_home = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| format!("{h}/.local/share"))
                .unwrap_or_default()
        });
        if !data_home.is_empty() {
            return std::path::Path::new(&data_home)
                .join("icons")
                .join(theme)
                .join("index.theme")
                .exists();
        }
        false
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

// =============================================================================
// IconSetChoice: user's icon set selection intent
// =============================================================================

/// The user's icon set selection mode.
///
/// Represents the user's intent for which icons to display.  The key
/// invariant: only [`Default`](Self::Default) is re-derived on theme changes.
/// All other variants represent an explicit user choice that is preserved
/// across theme re-applications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconSetChoice {
    /// Follow the theme preset's recommendation.
    ///
    /// The `String` is the preset's `icon_theme` name (e.g. "Adwaita"),
    /// used for display ("default (Adwaita)") and for loading when the
    /// icon set is `Freedesktop`.
    ///
    /// This is the ONLY variant that gets overwritten on theme change.
    /// It is only constructed via [`default_icon_choice()`], which
    /// guarantees the theme is available (bundled sets are always
    /// available; freedesktop themes are checked via
    /// [`is_freedesktop_theme_available`] before returning this variant).
    Default(String),

    /// Use the OS-configured icon theme.
    ///
    /// Resolved at load time via [`system_icon_set()`](crate::model::icons::system_icon_set).
    /// The display label ("system (breeze-dark)") is computed dynamically
    /// from [`system_icon_theme()`](crate::model::icons::system_icon_theme),
    /// so it tracks runtime OS theme changes.
    System,

    /// User explicitly picked a specific installed freedesktop icon theme.
    ///
    /// The `String` is the theme directory name (e.g. "char-white",
    /// "breeze", "Papirus").  Loaded via `IconSet::Freedesktop` with
    /// `.theme(name)`.
    Freedesktop(String),

    /// Google Material Symbols (bundled).
    Material,

    /// Lucide Icons (bundled).
    Lucide,
}

impl fmt::Display for IconSetChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default(name) => write!(f, "default ({name})"),
            Self::System => {
                let name = system_icon_theme();
                write!(f, "system ({name})")
            }
            Self::Freedesktop(name) => write!(f, "{name}"),
            Self::Material => write!(f, "Material (bundled)"),
            Self::Lucide => write!(f, "Lucide (bundled)"),
        }
    }
}

impl IconSetChoice {
    /// The effective [`IconSet`] loading mechanism for this choice.
    ///
    /// For [`Default`](Self::Default), returns the theme's icon set
    /// (caller passes it in).
    /// For [`Freedesktop`](Self::Freedesktop), always returns
    /// [`IconSet::Freedesktop`].
    /// For others, returns the corresponding bundled or system set.
    #[must_use]
    pub fn effective_icon_set(&self, theme_icon_set: IconSet) -> IconSet {
        match self {
            Self::Default(_) => theme_icon_set,
            Self::System => system_icon_set(),
            Self::Freedesktop(_) => IconSet::Freedesktop,
            Self::Material => IconSet::Material,
            Self::Lucide => IconSet::Lucide,
        }
    }

    /// The freedesktop theme name to pass to [`IconLoader::theme()`], if any.
    ///
    /// Returns `Some(name)` for [`Default`](Self::Default) and
    /// [`Freedesktop`](Self::Freedesktop) variants.
    /// Returns `None` for [`System`](Self::System), [`Material`](Self::Material),
    /// [`Lucide`](Self::Lucide).
    ///
    /// The caller is responsible for only passing the result to
    /// [`IconLoader::theme()`] when the effective icon set is
    /// [`IconSet::Freedesktop`].  For bundled sets (Material, Lucide),
    /// the theme name is not used by the loader.
    #[must_use]
    pub fn freedesktop_theme(&self) -> Option<&str> {
        match self {
            Self::Default(name) | Self::Freedesktop(name) => Some(name),
            Self::System | Self::Material | Self::Lucide => None,
        }
    }

    /// Whether this choice should be re-derived when the theme changes.
    ///
    /// Only [`Default`](Self::Default) follows the preset.  All others
    /// are explicit user choices that must be preserved.
    #[must_use]
    pub fn follows_preset(&self) -> bool {
        matches!(self, Self::Default(_))
    }
}

/// Determine the default icon set choice for a theme.
///
/// When the TOML specifies `icon_theme` (`Some`) and the theme is
/// available (bundled sets are always available; freedesktop themes are
/// checked via [`is_freedesktop_theme_available`]), returns
/// [`IconSetChoice::Default(icon_theme)`](IconSetChoice::Default).
///
/// When the TOML does not specify `icon_theme` (`None`), or the
/// specified freedesktop theme is not installed, returns
/// [`IconSetChoice::System`].
#[must_use]
pub fn default_icon_choice(icon_set: IconSet, icon_theme: Option<&str>) -> IconSetChoice {
    let Some(theme) = icon_theme else {
        return IconSetChoice::System;
    };
    // All five IconSet variants listed explicitly -- no wildcard.
    // Adding a new variant produces a compiler error, forcing the author
    // to decide whether the new set's theme is "always available" or
    // needs a runtime check.
    let available = match icon_set {
        IconSet::Material | IconSet::Lucide => true,
        IconSet::Freedesktop => is_freedesktop_theme_available(theme),
        IconSet::SfSymbols | IconSet::SegoeIcons => true,
    };
    if available {
        IconSetChoice::Default(theme.to_string())
    } else {
        IconSetChoice::System
    }
}

/// List installed freedesktop icon themes.
///
/// Scans `$XDG_DATA_DIRS/icons/` and `$XDG_DATA_HOME/icons/` for
/// subdirectories containing an `index.theme` file with a `Directories=`
/// line (per the freedesktop Icon Theme Specification).  This filters
/// out cursor-only themes that lack application icons.
///
/// Excludes `hicolor` (mandatory fallback) and `default` (typically a
/// symlink).  Returns a sorted, deduplicated list of theme directory
/// names.
///
/// Silently skips entries on IO errors (e.g. permission denied).
/// Returns an empty `Vec` on non-Linux platforms.
///
/// Note: themes installed via Flatpak or Snap may reside outside
/// standard XDG paths and will not be discovered.
#[must_use]
pub fn list_freedesktop_themes() -> Vec<String> {
    #[cfg(target_os = "linux")]
    {
        use std::collections::BTreeSet;
        use std::io::BufRead;

        let mut themes = BTreeSet::new();

        // Collect icon base directories from XDG paths.
        let mut icon_dirs = Vec::new();

        let data_dirs = std::env::var("XDG_DATA_DIRS")
            .unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());
        for dir in data_dirs.split(':') {
            if !dir.is_empty() {
                icon_dirs.push(std::path::PathBuf::from(dir).join("icons"));
            }
        }

        let data_home = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| format!("{h}/.local/share"))
                .unwrap_or_default()
        });
        if !data_home.is_empty() {
            icon_dirs.push(std::path::PathBuf::from(&data_home).join("icons"));
        }

        for icon_dir in &icon_dirs {
            let entries = match std::fs::read_dir(icon_dir) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = match entry.file_name().into_string() {
                    Ok(n) => n,
                    Err(_) => continue,
                };

                // Exclude hicolor (mandatory fallback) and default (symlink).
                if name == "hicolor" || name == "default" {
                    continue;
                }

                let index_path = path.join("index.theme");
                let file = match std::fs::File::open(&index_path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };

                // Check if index.theme contains a Directories= line.
                // Cursor-only themes omit this line.
                let reader = std::io::BufReader::new(file);
                let has_directories = reader
                    .lines()
                    .map_while(Result::ok)
                    .any(|line| line.starts_with("Directories="));

                if has_directories {
                    themes.insert(name);
                }
            }
        }

        themes.into_iter().collect()
    }
    #[cfg(not(target_os = "linux"))]
    {
        Vec::new()
    }
}

// =============================================================================
// Internal dispatch functions (previously public, now private)
// =============================================================================

/// Internal: load by role, dispatching to platform or bundled loaders.
/// The `size` parameter flows through to freedesktop (ICON-02: no hardcoded 24).
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
fn load_icon_inner(
    role: IconRole,
    set: IconSet,
    size: u16,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => crate::freedesktop::load_freedesktop_icon(role, size, fg_color),

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => crate::sficons::load_sf_icon(role),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => crate::winicons::load_windows_icon(role),

        #[cfg(feature = "material-icons")]
        IconSet::Material => {
            bundled_icon_svg(role, IconSet::Material).map(|b| IconData::Svg(Cow::Borrowed(b)))
        }

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => {
            bundled_icon_svg(role, IconSet::Lucide).map(|b| IconData::Svg(Cow::Borrowed(b)))
        }

        // Non-matching platform or unknown set: no cross-set fallback
        _ => None,
    }
}

/// Internal: load from a specific freedesktop theme, falling back to
/// `load_icon_inner` for non-freedesktop sets.
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
fn load_icon_from_theme_inner(
    role: IconRole,
    set: IconSet,
    preferred_theme: &str,
    size: u16,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => {
            let name = icon_name(role, IconSet::Freedesktop)?;
            crate::freedesktop::load_freedesktop_icon_by_name(name, preferred_theme, size, fg_color)
        }

        // Bundled and platform sets --- preferred_theme is irrelevant
        _ => load_icon_inner(role, set, size, fg_color),
    }
}

/// Internal: load by platform-specific name string.
#[allow(unreachable_patterns, unused_variables)]
fn load_system_icon_by_name_inner(
    name: &str,
    set: IconSet,
    size: u16,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => {
            let theme = system_icon_theme();
            crate::freedesktop::load_freedesktop_icon_by_name(name, &theme, size, fg_color)
        }

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => crate::sficons::load_sf_icon_by_name(name),

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => crate::winicons::load_windows_icon_by_name(name),

        #[cfg(feature = "material-icons")]
        IconSet::Material => {
            bundled_icon_by_name(name, IconSet::Material).map(|b| IconData::Svg(Cow::Borrowed(b)))
        }

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => {
            bundled_icon_by_name(name, IconSet::Lucide).map(|b| IconData::Svg(Cow::Borrowed(b)))
        }

        _ => None,
    }
}

/// Internal: loading/spinner animation for the given icon set.
#[allow(unreachable_patterns)]
fn loading_indicator_inner(set: IconSet) -> Option<AnimatedIcon> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => crate::freedesktop::load_freedesktop_spinner(),

        #[cfg(feature = "material-icons")]
        IconSet::Material => Some(crate::spinners::material_spinner()),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => Some(crate::spinners::lucide_spinner()),

        _ => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_icon_tests {
    use super::*;

    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn icon_loader_theme_override_honored_for_name_lookup() {
        // REGRESSION (Phase 93-03): IconLoader::new(&str).set(Freedesktop).theme("Adwaita").load()
        // silently ignores .theme() because the dispatch for IconId::Name does not read
        // self.freedesktop_theme. On KDE Plasma the system theme is Breeze, so lookups
        // that should target Adwaita fall through to Breeze and 12 icons that exist in
        // Adwaita (but not in Breeze) return None.
        //
        // Expected to FAIL at this commit. Plan 93-09 eliminates the silent-ignore design
        // by splitting IconLoader into typed-per-set loaders; this test migrates to
        // `FreedesktopLoader::new("format-text-rich").theme("Adwaita").load()` at the
        // refactor commit and passes.
        //
        // Requires the Adwaita icon theme installed on the test host (standard on Linux).
        let result = IconLoader::new("format-text-rich")
            .set(IconSet::Freedesktop)
            .theme("Adwaita")
            .size(24)
            .load();
        assert!(
            result.is_some(),
            "theme('Adwaita') must resolve 'format-text-rich' in Adwaita, regardless of system theme"
        );
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_material_returns_svg() {
        let result = IconLoader::new(IconRole::ActionCopy)
            .set(IconSet::Material)
            .load();
        assert!(result.is_some(), "material ActionCopy should return Some");
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                let s = String::from_utf8_lossy(cow);
                assert!(s.contains("<svg"), "should contain SVG data");
            }
            _ => panic!("expected IconData::Svg for bundled material icon"),
        }
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn load_icon_lucide_returns_svg() {
        let result = IconLoader::new(IconRole::ActionCopy)
            .set(IconSet::Lucide)
            .load();
        assert!(result.is_some(), "lucide ActionCopy should return Some");
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                let s = String::from_utf8_lossy(cow);
                assert!(s.contains("<svg"), "should contain SVG data");
            }
            _ => panic!("expected IconData::Svg for bundled lucide icon"),
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_unknown_theme_no_cross_set_fallback() {
        // On Linux (test platform), unknown theme resolves to system_icon_set() = Freedesktop.
        // Without system-icons feature, Freedesktop falls through to wildcard -> None.
        // No cross-set Material fallback.
        let result = IconLoader::new(IconRole::ActionCopy)
            .set(IconSet::Freedesktop)
            .load();
        // Without system-icons, this falls to wildcard which returns None
        // With system-icons, this dispatches to load_freedesktop_icon which may return Some
        // Either way, no panic
        let _ = result;
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn load_icon_all_roles_material() {
        // Material has 42 of 42 roles mapped, all return Some
        let mut some_count = 0;
        for role in IconRole::ALL {
            if IconLoader::new(role)
                .set(IconSet::Material)
                .load()
                .is_some()
            {
                some_count += 1;
            }
        }
        // bundled_icon_svg covers all 42 roles for Material
        assert_eq!(
            some_count, 42,
            "Material should cover all 42 roles via bundled SVGs"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn load_icon_all_roles_lucide() {
        let mut some_count = 0;
        for role in IconRole::ALL {
            if IconLoader::new(role).set(IconSet::Lucide).load().is_some() {
                some_count += 1;
            }
        }
        // bundled_icon_svg covers all 42 roles for Lucide
        assert_eq!(
            some_count, 42,
            "Lucide should cover all 42 roles via bundled SVGs"
        );
    }

    #[test]
    fn load_icon_unrecognized_set_no_features() {
        // SfSymbols on Linux without system-icons: falls through to wildcard -> None
        let _result = IconLoader::new(IconRole::ActionCopy)
            .set(IconSet::SfSymbols)
            .load();
        // Just verifying it doesn't panic
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn bundled_icon_load_produces_cow_borrowed() {
        let result = IconLoader::new(IconRole::ActionCopy)
            .set(IconSet::Material)
            .load();
        assert!(
            matches!(result, Some(IconData::Svg(Cow::Borrowed(_)))),
            "bundled icon should produce Some(IconData::Svg(Cow::Borrowed(_)))"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_system_icon_by_name_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_material() {
        let result = IconLoader::new("content_copy")
            .set(IconSet::Material)
            .load();
        assert!(
            result.is_some(),
            "content_copy should be found in Material set"
        );
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn system_icon_by_name_lucide() {
        let result = IconLoader::new("copy").set(IconSet::Lucide).load();
        assert!(result.is_some(), "copy should be found in Lucide set");
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_unknown_returns_none() {
        let result = IconLoader::new("nonexistent_xyz")
            .set(IconSet::Material)
            .load();
        assert!(result.is_none(), "nonexistent name should return None");
    }

    #[test]
    fn system_icon_by_name_sf_on_linux_returns_none() {
        // On Linux, SfSymbols set is not available (cfg-gated to macOS)
        #[cfg(not(target_os = "macos"))]
        {
            let result = IconLoader::new("doc.on.doc").set(IconSet::SfSymbols).load();
            assert!(
                result.is_none(),
                "SF Symbols should return None on non-macOS"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod load_custom_icon_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_with_icon_role_material() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let result = IconLoader::new(provider).set(IconSet::Material).load();
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via material"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn custom_icon_with_icon_role_lucide() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let result = IconLoader::new(provider).set(IconSet::Lucide).load();
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via lucide"
        );
    }

    #[test]
    fn custom_icon_no_cross_set_fallback() {
        // Provider that returns None for all sets -- should NOT fall back
        #[derive(Debug)]
        struct NullProvider;
        impl IconProvider for NullProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
                None
            }
        }

        let provider: &dyn IconProvider = &NullProvider;
        let result = IconLoader::new(provider).set(IconSet::Material).load();
        assert!(
            result.is_none(),
            "NullProvider should return None (no cross-set fallback)"
        );
    }

    #[test]
    fn custom_icon_unknown_set_uses_system() {
        // "unknown-set" is not a known IconSet name, falls through to system_icon_set()
        #[derive(Debug)]
        struct NullProvider;
        impl IconProvider for NullProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
                None
            }
        }

        // Just verify it doesn't panic -- the actual set chosen depends on platform
        let provider: &dyn IconProvider = &NullProvider;
        let _result = IconLoader::new(provider).set(IconSet::Freedesktop).load();
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_via_dyn_dispatch() {
        let boxed: Box<dyn IconProvider> = Box::new(IconRole::ActionCopy);
        let provider: &dyn IconProvider = &*boxed;
        let result = IconLoader::new(provider).set(IconSet::Material).load();
        assert!(
            result.is_some(),
            "dyn dispatch through Box<dyn IconProvider> should work"
        );
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_bundled_svg_fallback() {
        // Provider that returns None from icon_name but Some from icon_svg
        #[derive(Debug)]
        struct SvgOnlyProvider;
        impl IconProvider for SvgOnlyProvider {
            fn icon_name(&self, _set: IconSet) -> Option<&str> {
                None
            }
            fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
                Some(Cow::Borrowed(b"<svg>test</svg>"))
            }
        }

        let provider: &dyn IconProvider = &SvgOnlyProvider;
        let result = IconLoader::new(provider).set(IconSet::Material).load();
        assert!(
            result.is_some(),
            "provider with icon_svg should return Some"
        );
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                assert_eq!(cow.as_ref(), b"<svg>test</svg>");
            }
            _ => panic!("expected IconData::Svg"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod loading_indicator_tests {
    use super::*;

    // === Dispatch tests (through IconLoader API) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn loading_indicator_lucide_returns_frames() {
        let anim = IconLoader::new(IconRole::StatusBusy)
            .set(IconSet::Lucide)
            .load_indicator();
        assert!(anim.is_some(), "lucide should return Some");
        let anim = anim.unwrap();
        assert!(
            matches!(anim, AnimatedIcon::Frames(_)),
            "lucide should be pre-rotated Frames"
        );
        if let AnimatedIcon::Frames(data) = &anim {
            assert_eq!(data.frames().len(), 24);
            assert_eq!(data.frame_duration_ms().get(), 42);
        }
    }

    /// Freedesktop loading_indicator returns Some if the active icon theme
    /// has a `process-working` sprite sheet (e.g. Breeze), None otherwise.
    #[test]
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    fn loading_indicator_freedesktop_depends_on_theme() {
        let anim = IconLoader::new(IconRole::StatusBusy)
            .set(IconSet::Freedesktop)
            .load_indicator();
        // Result depends on installed icon theme -- Some if process-working exists
        if let Some(anim) = anim {
            match anim {
                AnimatedIcon::Frames(data) => {
                    assert!(
                        !data.frames().is_empty(),
                        "Frames variant should have at least one frame"
                    );
                }
                AnimatedIcon::Transform(_) => {
                    // Single-frame theme icon with Spin -- valid result
                }
                _ => {}
            }
        }
    }

    /// Freedesktop spinner depends on platform and icon theme.
    #[test]
    fn loading_indicator_freedesktop_does_not_panic() {
        let _result = IconLoader::new(IconRole::StatusBusy)
            .set(IconSet::Freedesktop)
            .load_indicator();
    }

    // === Direct spinner construction tests (any platform) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_spinner_is_frames() {
        let anim = crate::spinners::lucide_spinner();
        assert!(
            matches!(anim, AnimatedIcon::Frames(_)),
            "lucide should be pre-rotated Frames"
        );
    }
}

// === New builder API tests ===

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod icon_loader_tests {
    use super::*;

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_basic_role() {
        let icon = IconLoader::new(IconRole::ActionCopy)
            .set(IconSet::Material)
            .load();
        assert!(icon.is_some());
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_by_name() {
        let icon = IconLoader::new("content_copy")
            .set(IconSet::Material)
            .load();
        assert!(icon.is_some());
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_custom_provider() {
        let provider: &dyn IconProvider = &IconRole::ActionCopy;
        let icon = IconLoader::new(provider).set(IconSet::Material).load();
        assert!(icon.is_some());
    }

    #[test]
    fn icon_loader_size_is_not_hardcoded() {
        // Verify the size field is used (not hardcoded 24)
        let loader = IconLoader::new(IconRole::ActionSave)
            .set(IconSet::Freedesktop)
            .size(48);
        assert_eq!(loader.size, 48);
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_loader_load_indicator() {
        let anim = IconLoader::new(IconRole::StatusBusy)
            .set(IconSet::Material)
            .load_indicator();
        assert!(anim.is_some());
    }
}

#[cfg(all(test, feature = "svg-rasterize"))]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod spinner_rasterize_tests {
    use super::*;

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_spinner_icon_rasterizes() {
        let anim = crate::spinners::lucide_spinner();
        if let AnimatedIcon::Frames(data) = &anim {
            let first = data.frames().first();
            if let IconData::Svg(cow) = first {
                let result = crate::rasterize::rasterize_svg(cow, 24);
                assert!(result.is_ok(), "lucide loader should rasterize");
                if let Ok(IconData::Rgba { data, .. }) = &result {
                    assert!(
                        data.iter().any(|&b| b != 0),
                        "lucide loader rasterized to empty image"
                    );
                }
            } else {
                panic!("lucide spinner frame should be Svg");
            }
        } else {
            panic!("lucide spinner should be Frames");
        }
    }
}

// =============================================================================
// IconSetChoice tests
// =============================================================================

#[cfg(test)]
mod icon_set_choice_tests {
    use super::*;

    #[test]
    fn test_icon_set_choice_display_default() {
        let choice = IconSetChoice::Default("Adwaita".to_string());
        assert_eq!(choice.to_string(), "default (Adwaita)");
    }

    #[test]
    fn test_icon_set_choice_display_material() {
        assert_eq!(IconSetChoice::Material.to_string(), "Material (bundled)");
    }

    #[test]
    fn test_icon_set_choice_display_lucide() {
        assert_eq!(IconSetChoice::Lucide.to_string(), "Lucide (bundled)");
    }

    #[test]
    fn test_icon_set_choice_display_freedesktop() {
        let choice = IconSetChoice::Freedesktop("breeze".to_string());
        assert_eq!(choice.to_string(), "breeze");
    }

    #[test]
    fn test_icon_set_choice_follows_preset() {
        assert!(IconSetChoice::Default("Adwaita".to_string()).follows_preset());
        assert!(!IconSetChoice::System.follows_preset());
        assert!(!IconSetChoice::Freedesktop("breeze".to_string()).follows_preset());
        assert!(!IconSetChoice::Material.follows_preset());
        assert!(!IconSetChoice::Lucide.follows_preset());
    }

    #[test]
    fn test_icon_set_choice_effective_icon_set() {
        // Default returns whatever the theme specifies
        let choice = IconSetChoice::Default("Adwaita".to_string());
        assert_eq!(
            choice.effective_icon_set(IconSet::Freedesktop),
            IconSet::Freedesktop
        );
        assert_eq!(
            choice.effective_icon_set(IconSet::Material),
            IconSet::Material
        );

        // System returns the platform's icon set
        let sys = IconSetChoice::System.effective_icon_set(IconSet::Material);
        assert_eq!(sys, system_icon_set());

        // Freedesktop always returns Freedesktop
        let choice = IconSetChoice::Freedesktop("breeze".to_string());
        assert_eq!(
            choice.effective_icon_set(IconSet::Material),
            IconSet::Freedesktop
        );

        // Bundled sets return themselves
        assert_eq!(
            IconSetChoice::Material.effective_icon_set(IconSet::Freedesktop),
            IconSet::Material
        );
        assert_eq!(
            IconSetChoice::Lucide.effective_icon_set(IconSet::Freedesktop),
            IconSet::Lucide
        );
    }

    #[test]
    fn test_icon_set_choice_freedesktop_theme() {
        assert_eq!(
            IconSetChoice::Default("Adwaita".to_string()).freedesktop_theme(),
            Some("Adwaita")
        );
        assert_eq!(
            IconSetChoice::Freedesktop("breeze".to_string()).freedesktop_theme(),
            Some("breeze")
        );
        assert_eq!(IconSetChoice::System.freedesktop_theme(), None);
        assert_eq!(IconSetChoice::Material.freedesktop_theme(), None);
        assert_eq!(IconSetChoice::Lucide.freedesktop_theme(), None);
    }

    #[test]
    fn test_default_icon_choice_none() {
        // When icon_theme is None, always returns System regardless of icon_set
        assert_eq!(
            default_icon_choice(IconSet::Freedesktop, None),
            IconSetChoice::System
        );
        assert_eq!(
            default_icon_choice(IconSet::Material, None),
            IconSetChoice::System
        );
    }

    #[test]
    fn test_default_icon_choice_bundled() {
        // Material and Lucide are always available
        assert_eq!(
            default_icon_choice(IconSet::Material, Some("any-theme")),
            IconSetChoice::Default("any-theme".to_string())
        );
        assert_eq!(
            default_icon_choice(IconSet::Lucide, Some("any-theme")),
            IconSetChoice::Default("any-theme".to_string())
        );
    }

    #[test]
    fn test_list_freedesktop_themes_no_panic() {
        // Just verify it doesn't panic -- result varies by platform
        let themes = list_freedesktop_themes();
        // On Linux, should return a non-empty list on most desktop systems.
        // On other platforms, returns empty vec.
        #[cfg(not(target_os = "linux"))]
        assert!(themes.is_empty());
        // Suppress unused variable warning
        let _ = themes;
    }
}
