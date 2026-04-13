//! Icon loading and dispatch.

#[allow(unused_imports)]
use std::borrow::Cow;

#[allow(unused_imports)]
use crate::model::icons::{icon_name, system_icon_theme};
use crate::model::{AnimatedIcon, IconData, IconProvider, IconRole, IconSet};
#[allow(unused_imports)]
use crate::model::{bundled_icon_by_name, bundled_icon_svg};

/// Load an icon for the given role using the specified icon set.
///
/// Dispatches to the appropriate platform loader or bundled icon set
/// based on the [`IconSet`] variant:
///
/// - [`IconSet::Freedesktop`] -- freedesktop theme lookup at 24 px using the
///   system's installed icon theme (requires `system-icons` feature, Linux only)
/// - [`IconSet::SfSymbols`] -- SF Symbols lookup
///   (requires `system-icons` feature, macOS only)
/// - [`IconSet::SegoeIcons`] -- Segoe Fluent lookup
///   (requires `system-icons` feature, Windows only)
/// - [`IconSet::Material`] -- bundled Material SVG
///   (requires `material-icons` feature)
/// - [`IconSet::Lucide`] -- bundled Lucide SVG
///   (requires `lucide-icons` feature)
///
/// Returns `None` when the required feature is not enabled, the platform
/// doesn't match, or the role has no icon in the requested set.
/// There is **no cross-set fallback** -- each set is self-contained.
///
/// # Examples
///
/// ```
/// use native_theme::icons::load_icon;
/// use native_theme::theme::{IconRole, IconSet};
///
/// // With material-icons feature enabled
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_icon(IconRole::ActionCopy, IconSet::Material, None);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use]
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
pub fn load_icon(role: IconRole, set: IconSet, fg_color: Option<[u8; 3]>) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => crate::freedesktop::load_freedesktop_icon(role, 24, fg_color),

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

/// Load an icon using a specific freedesktop icon theme instead of the
/// system default.
///
/// For [`IconSet::Freedesktop`], loads from the `preferred_theme` directory
/// (e.g. `"Adwaita"`, `"breeze"`). For bundled icon sets ([`IconSet::Material`],
/// [`IconSet::Lucide`]), `preferred_theme` is ignored --- the icons are compiled
/// in and always available.
///
/// Use [`is_freedesktop_theme_available()`] first to check whether the theme
/// is installed. If the theme is not installed, freedesktop lookups will fall
/// through to `hicolor` and may return unexpected icons.
///
/// # Examples
///
/// ```
/// use native_theme::icons::load_icon_from_theme;
/// use native_theme::theme::{IconRole, IconSet};
///
/// # #[cfg(feature = "material-icons")]
/// # {
/// // Bundled sets ignore the theme parameter
/// let icon = load_icon_from_theme(IconRole::ActionCopy, IconSet::Material, "anything", None);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use]
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
pub fn load_icon_from_theme(
    role: IconRole,
    set: IconSet,
    preferred_theme: &str,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => {
            let name = icon_name(role, IconSet::Freedesktop)?;
            crate::freedesktop::load_freedesktop_icon_by_name(name, preferred_theme, 24, fg_color)
        }

        // Bundled and platform sets --- preferred_theme is irrelevant
        _ => load_icon(role, set, fg_color),
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

/// Load a system icon by its platform-specific name string.
///
/// Dispatches to the appropriate platform loader based on the icon set:
/// - [`IconSet::Freedesktop`] -- freedesktop icon theme lookup (system theme)
/// - [`IconSet::SfSymbols`] -- macOS SF Symbols
/// - [`IconSet::SegoeIcons`] -- Windows Segoe Fluent / stock icons
/// - [`IconSet::Material`] / [`IconSet::Lucide`] -- bundled SVG lookup by name
///
/// Returns `None` if the icon is not found on the current platform or
/// the icon set is not available.
///
/// # Examples
///
/// ```
/// use native_theme::icons::load_system_icon_by_name;
/// use native_theme::theme::IconSet;
///
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_system_icon_by_name("content_copy", IconSet::Material, None);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use]
#[allow(unreachable_patterns, unused_variables)]
pub fn load_system_icon_by_name(
    name: &str,
    set: IconSet,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => {
            let theme = system_icon_theme();
            crate::freedesktop::load_freedesktop_icon_by_name(name, theme, 24, fg_color)
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

/// Return the loading/spinner animation for the given icon set.
///
/// This is the animated-icon counterpart of [`load_icon()`].
///
/// # Dispatch
///
/// - [`IconSet::Material`] -- `progress_activity.svg` with continuous spin transform (1000ms)
/// - [`IconSet::Lucide`] -- `loader.svg` with continuous spin transform (1000ms)
/// - [`IconSet::Freedesktop`] -- loads `process-working` sprite sheet from active icon theme
/// - Other sets -- `None`
///
/// # Examples
///
/// ```
/// // Result depends on enabled features and platform
/// let anim = native_theme::icons::loading_indicator(native_theme::theme::IconSet::Lucide);
/// # #[cfg(feature = "lucide-icons")]
/// # assert!(anim.is_some());
/// ```
#[must_use]
pub fn loading_indicator(set: IconSet) -> Option<AnimatedIcon> {
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

/// Load an icon from any [`IconProvider`], dispatching through the standard
/// platform loading chain.
///
/// # Fallback chain
///
/// 1. Provider's [`icon_name()`](IconProvider::icon_name) -- passed to platform
///    system loader via [`load_system_icon_by_name()`]
/// 2. Provider's [`icon_svg()`](IconProvider::icon_svg) -- bundled SVG data
/// 3. `None` -- **no cross-set fallback** (mixing icon sets is forbidden)
///
/// # Examples
///
/// ```
/// use native_theme::icons::load_custom_icon;
/// use native_theme::theme::{IconRole, IconSet};
///
/// // IconRole implements IconProvider, so it works with load_custom_icon
/// # #[cfg(feature = "material-icons")]
/// # {
/// let icon = load_custom_icon(&IconRole::ActionCopy, IconSet::Material, None);
/// assert!(icon.is_some());
/// # }
/// ```
#[must_use]
pub fn load_custom_icon(
    provider: &(impl IconProvider + ?Sized),
    set: IconSet,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    // Step 1: Try system loader with provider's name mapping
    if let Some(name) = provider.icon_name(set)
        && let Some(data) = load_system_icon_by_name(name, set, fg_color)
    {
        return Some(data);
    }

    // Step 2: Try bundled SVG from provider
    if let Some(svg) = provider.icon_svg(set) {
        return Some(IconData::Svg(svg));
    }

    // No cross-set fallback -- return None
    None
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
        let result = load_icon(IconRole::ActionCopy, IconSet::Material, None);
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
        let result = load_icon(IconRole::ActionCopy, IconSet::Lucide, None);
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
        let result = load_icon(IconRole::ActionCopy, IconSet::Freedesktop, None);
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
            if load_icon(role, IconSet::Material, None).is_some() {
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
            if load_icon(role, IconSet::Lucide, None).is_some() {
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
        let _result = load_icon(IconRole::ActionCopy, IconSet::SfSymbols, None);
        // Just verifying it doesn't panic
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn bundled_icon_load_produces_cow_borrowed() {
        let result = load_icon(IconRole::ActionCopy, IconSet::Material, None);
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
        let result = load_system_icon_by_name("content_copy", IconSet::Material, None);
        assert!(
            result.is_some(),
            "content_copy should be found in Material set"
        );
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn system_icon_by_name_lucide() {
        let result = load_system_icon_by_name("copy", IconSet::Lucide, None);
        assert!(result.is_some(), "copy should be found in Lucide set");
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn system_icon_by_name_unknown_returns_none() {
        let result = load_system_icon_by_name("nonexistent_xyz", IconSet::Material, None);
        assert!(result.is_none(), "nonexistent name should return None");
    }

    #[test]
    fn system_icon_by_name_sf_on_linux_returns_none() {
        // On Linux, SfSymbols set is not available (cfg-gated to macOS)
        #[cfg(not(target_os = "macos"))]
        {
            let result = load_system_icon_by_name("doc.on.doc", IconSet::SfSymbols, None);
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
        let result = load_custom_icon(&IconRole::ActionCopy, IconSet::Material, None);
        assert!(
            result.is_some(),
            "IconRole::ActionCopy should load via material"
        );
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn custom_icon_with_icon_role_lucide() {
        let result = load_custom_icon(&IconRole::ActionCopy, IconSet::Lucide, None);
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

        let result = load_custom_icon(&NullProvider, IconSet::Material, None);
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
        let _result = load_custom_icon(&NullProvider, IconSet::Freedesktop, None);
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn custom_icon_via_dyn_dispatch() {
        let boxed: Box<dyn IconProvider> = Box::new(IconRole::ActionCopy);
        let result = load_custom_icon(&*boxed, IconSet::Material, None);
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

        let result = load_custom_icon(&SvgOnlyProvider, IconSet::Material, None);
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

    // === Dispatch tests (through loading_indicator public API) ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn loading_indicator_lucide_returns_frames() {
        let anim = loading_indicator(IconSet::Lucide);
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
        let anim = loading_indicator(IconSet::Freedesktop);
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
        let _result = loading_indicator(IconSet::Freedesktop);
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
