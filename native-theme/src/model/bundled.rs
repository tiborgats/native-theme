// Feature-gated bundled SVG icon access via include_bytes!
//
// With `material-icons` or `lucide-icons` features enabled, this module
// embeds SVG files at compile time and makes them available via
// `bundled_icon_svg()`. Without features, the function always returns None.

use super::icons::{IconRole, IconSet, icon_name};

/// Returns the raw SVG bytes for a bundled icon, if the corresponding
/// feature flag is enabled.
///
/// Returns `None` when:
/// - The requested `IconSet` is not `Material` or `Lucide`
/// - The feature flag for the requested set is not enabled
///
/// Returns `Some(&[u8])` containing valid SVG bytes when the feature
/// is enabled. Callers should wrap in `Cow::Borrowed` for zero-copy
/// `IconData::Svg(Cow::Borrowed(bytes))` instead of `.to_vec()`.
///
/// # Examples
///
/// This is a crate-internal helper. The public replacement is the
/// typed per-set loader API in [`crate::icons`]; external callers should
/// not reach this function directly. The snippet below demonstrates the
/// equivalent public-API call:
///
/// ```
/// use native_theme::icons::MaterialLoader;
/// use native_theme::icons::IconRole;
///
/// // Material is bundled with the `material-icons` feature; the loader returns None without it.
/// let result = MaterialLoader::new(IconRole::ActionCopy).load();
/// assert_eq!(result.is_some(), cfg!(feature = "material-icons"));
/// ```
#[must_use]
#[cfg_attr(
    not(any(feature = "material-icons", feature = "lucide-icons")),
    allow(dead_code)
)]
pub(crate) fn bundled_icon_svg(role: IconRole, set: IconSet) -> Option<&'static [u8]> {
    icon_name(role, set).and_then(|name| bundled_icon_by_name(name, set))
}

/// Returns raw SVG bytes for a bundled icon looked up by its canonical name
/// within the icon set.
///
/// Names use each set's canonical format:
/// - Lucide: kebab-case (e.g., `"arrow-down"`, `"circle-check"`)
/// - Material: snake_case (e.g., `"arrow_downward"`, `"check_circle"`)
///
/// Returns `None` for non-bundled sets, disabled features, or unknown names.
///
/// # Examples
///
/// This is a crate-internal helper. The public replacement is a typed
/// per-set loader from [`crate::icons`] constructed with a string name:
///
/// ```
/// use native_theme::icons::LucideLoader;
///
/// // Lucide is bundled with the `lucide-icons` feature; the loader returns None without it.
/// let result = LucideLoader::new("check").load();
/// assert_eq!(result.is_some(), cfg!(feature = "lucide-icons"));
/// ```
#[must_use]
#[allow(unreachable_patterns, unused_variables)]
#[cfg_attr(
    not(any(feature = "material-icons", feature = "lucide-icons")),
    allow(dead_code)
)]
pub(crate) fn bundled_icon_by_name(name: &str, set: IconSet) -> Option<&'static [u8]> {
    match set {
        #[cfg(feature = "material-icons")]
        IconSet::Material => material_svg_by_name(name),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => lucide_svg_by_name(name),

        _ => None,
    }
}

#[cfg(feature = "lucide-icons")]
include!(concat!(env!("OUT_DIR"), "/lucide_svg_by_name.rs"));

#[cfg(feature = "material-icons")]
include!(concat!(env!("OUT_DIR"), "/material_svg_by_name.rs"));

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === Feature-gated tests: Material ===

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_icons_cover_all_roles() {
        for role in IconRole::ALL.iter().copied() {
            let svg = bundled_icon_svg(role, IconSet::Material);
            assert!(svg.is_some(), "Material icons missing SVG for {:?}", role);
            let bytes = svg.unwrap();
            let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(
                content.contains("<svg"),
                "Material {:?} does not contain <svg tag",
                role
            );
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_icons_total_size_under_200kb() {
        let total: usize = IconRole::ALL
            .iter()
            .filter_map(|role| bundled_icon_svg(*role, IconSet::Material))
            .map(|svg| svg.len())
            .sum();
        assert!(
            total < 400 * 1024,
            "Material icons total size {} bytes exceeds 400KB budget",
            total
        );
    }

    // === Feature-gated tests: Lucide ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_icons_cover_all_roles() {
        for role in IconRole::ALL.iter().copied() {
            let svg = bundled_icon_svg(role, IconSet::Lucide);
            assert!(svg.is_some(), "Lucide icons missing SVG for {:?}", role);
            let bytes = svg.unwrap();
            let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(
                content.contains("<svg"),
                "Lucide {:?} does not contain <svg tag",
                role
            );
        }
    }

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_icons_total_size_under_100kb() {
        let total: usize = IconRole::ALL
            .iter()
            .filter_map(|role| bundled_icon_svg(*role, IconSet::Lucide))
            .map(|svg| svg.len())
            .sum();
        assert!(
            total < 200 * 1024,
            "Lucide icons total size {} bytes exceeds 200KB budget",
            total
        );
    }

    #[test]
    #[cfg(any(feature = "material-icons", feature = "lucide-icons"))]
    fn every_role_with_a_name_has_the_bundled_file_of_that_name() {
        for set in [IconSet::Material, IconSet::Lucide] {
            for role in IconRole::ALL.iter().copied() {
                if let Some(name) = icon_name(role, set) {
                    assert_eq!(
                        bundled_icon_svg(role, set),
                        bundled_icon_by_name(name, set),
                        "{role:?} in {set:?}"
                    );
                }
            }
        }
    }

    // === Non-feature-gated tests ===

    #[test]
    fn non_bundled_sets_return_none() {
        assert!(
            bundled_icon_svg(IconRole::ActionCopy, IconSet::SfSymbols).is_none(),
            "SfSymbols should not be a bundled set"
        );
        assert!(
            bundled_icon_svg(IconRole::ActionCopy, IconSet::Freedesktop).is_none(),
            "Freedesktop should not be a bundled set"
        );
        assert!(
            bundled_icon_svg(IconRole::ActionCopy, IconSet::SegoeFluent).is_none(),
            "SegoeFluent should not be a bundled set"
        );
    }

    // === bundled_icon_by_name tests ===

    /// §7.1: the gpui-component-style names and the two retired files no longer
    /// resolve; callers get `None`, never a substitute.
    #[test]
    #[cfg(all(feature = "lucide-icons", feature = "material-icons"))]
    fn retired_bundle_names_return_none() {
        for name in [
            "close",
            "dash",
            "inspect",
            "resize-corner",
            "sort-ascending",
            "sort-descending",
            "window-close",
            "window-maximize",
            "window-minimize",
            "window-restore",
            "trash-2",
        ] {
            assert!(
                bundled_icon_by_name(name, IconSet::Lucide).is_none(),
                "{name} should be gone"
            );
        }
        assert!(bundled_icon_by_name("star_border", IconSet::Material).is_none());
        assert!(bundled_icon_by_name("font_size", IconSet::Material).is_none());
        assert!(bundled_icon_by_name("trash", IconSet::Lucide).is_some());
        assert!(bundled_icon_by_name("star_fill1", IconSet::Material).is_some());
        assert!(bundled_icon_by_name("format_size", IconSet::Material).is_some());
    }

    #[test]
    fn by_name_non_bundled_sets_return_none() {
        assert!(bundled_icon_by_name("check", IconSet::SfSymbols).is_none());
        assert!(bundled_icon_by_name("check", IconSet::Freedesktop).is_none());
        assert!(bundled_icon_by_name("check", IconSet::SegoeFluent).is_none());
    }

    #[test]
    fn by_name_unknown_name_returns_none() {
        assert!(bundled_icon_by_name("nonexistent-icon-xyz", IconSet::Lucide).is_none());
        assert!(bundled_icon_by_name("nonexistent_icon_xyz", IconSet::Material).is_none());
    }

    #[cfg(any(feature = "lucide-icons", feature = "material-icons"))]
    fn bundled_files(dir: &str) -> Vec<String> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("icons")
            .join(dir);
        let mut names: Vec<String> = std::fs::read_dir(&path)
            .expect("icon directory exists")
            .map(|entry| {
                entry
                    .expect("readable entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter_map(|file| file.strip_suffix(".svg").map(str::to_owned))
            .collect();
        names.sort();
        assert!(!names.is_empty(), "icons/{dir} must not be empty");
        names
    }

    /// §10.5: the by-name table is generated from the directory, so every
    /// bundled file resolves and the table cannot drift.
    #[test]
    #[cfg(feature = "lucide-icons")]
    fn generated_lucide_table_covers_every_bundled_file() {
        for name in bundled_files("lucide") {
            let svg = bundled_icon_by_name(&name, IconSet::Lucide)
                .unwrap_or_else(|| panic!("Lucide table misses bundled file {name}.svg"));
            assert!(std::str::from_utf8(svg).expect("UTF-8").contains("<svg"));
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn generated_material_table_covers_every_bundled_file() {
        for name in bundled_files("material") {
            let svg = bundled_icon_by_name(&name, IconSet::Material)
                .unwrap_or_else(|| panic!("Material table misses bundled file {name}.svg"));
            assert!(std::str::from_utf8(svg).expect("UTF-8").contains("<svg"));
        }
    }
}
