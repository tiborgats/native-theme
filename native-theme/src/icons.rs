//! Icon loading and dispatch.

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
            matches!(anim, AnimatedIcon::Frames { .. }),
            "lucide should be pre-rotated Frames"
        );
        if let AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
        } = &anim
        {
            assert_eq!(frames.len(), 24);
            assert_eq!(*frame_duration_ms, 42);
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
                AnimatedIcon::Frames { frames, .. } => {
                    assert!(
                        !frames.is_empty(),
                        "Frames variant should have at least one frame"
                    );
                }
                AnimatedIcon::Transform { .. } => {
                    // Single-frame theme icon with Spin -- valid result
                }
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
            matches!(anim, AnimatedIcon::Frames { .. }),
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
        if let AnimatedIcon::Frames { frames, .. } = &anim {
            let first = frames.first().expect("should have at least one frame");
            if let IconData::Svg(ref cow) = first {
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
