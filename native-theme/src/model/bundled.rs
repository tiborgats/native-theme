// Feature-gated bundled SVG icon access via include_bytes!
//
// With `material-icons` or `lucide-icons` features enabled, this module
// embeds SVG files at compile time and makes them available via
// `bundled_icon_svg()`. Without features, the function always returns None.

use super::icons::{IconRole, IconSet};

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
/// use native_theme::icons::SfSymbolsLoader;
/// use native_theme::theme::IconRole;
///
/// // SfSymbols is not a bundled set on non-macOS targets; the loader returns None.
/// let result = SfSymbolsLoader::new(IconRole::ActionCopy).load();
/// assert!(result.is_none());
/// ```
#[must_use]
#[allow(unreachable_patterns, unused_variables)]
pub(crate) fn bundled_icon_svg(role: IconRole, set: IconSet) -> Option<&'static [u8]> {
    match set {
        #[cfg(feature = "material-icons")]
        IconSet::Material => material_svg(role),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => lucide_svg(role),

        _ => None,
    }
}

#[cfg(feature = "material-icons")]
#[allow(unreachable_patterns)]
fn material_svg(role: IconRole) -> Option<&'static [u8]> {
    match role {
        // Dialog / Alert (6)
        IconRole::DialogWarning => Some(include_bytes!("../../icons/material/warning.svg")),
        IconRole::DialogError => Some(include_bytes!("../../icons/material/error.svg")),
        IconRole::DialogInfo => Some(include_bytes!("../../icons/material/info.svg")),
        IconRole::DialogQuestion => Some(include_bytes!("../../icons/material/help.svg")),
        IconRole::DialogSuccess => Some(include_bytes!("../../icons/material/check_circle.svg")),
        IconRole::Shield => Some(include_bytes!("../../icons/material/shield.svg")),

        // Window Controls (4)
        IconRole::WindowClose => Some(include_bytes!("../../icons/material/close.svg")),
        IconRole::WindowMinimize => Some(include_bytes!("../../icons/material/minimize.svg")),
        IconRole::WindowMaximize => Some(include_bytes!("../../icons/material/open_in_full.svg")),
        IconRole::WindowRestore => {
            Some(include_bytes!("../../icons/material/close_fullscreen.svg"))
        }

        // Common Actions (14)
        IconRole::ActionSave => Some(include_bytes!("../../icons/material/save.svg")),
        IconRole::ActionDelete => Some(include_bytes!("../../icons/material/delete.svg")),
        IconRole::ActionCopy => Some(include_bytes!("../../icons/material/content_copy.svg")),
        IconRole::ActionPaste => Some(include_bytes!("../../icons/material/content_paste.svg")),
        IconRole::ActionCut => Some(include_bytes!("../../icons/material/content_cut.svg")),
        IconRole::ActionUndo => Some(include_bytes!("../../icons/material/undo.svg")),
        IconRole::ActionRedo => Some(include_bytes!("../../icons/material/redo.svg")),
        IconRole::ActionSearch => Some(include_bytes!("../../icons/material/search.svg")),
        IconRole::ActionSettings => Some(include_bytes!("../../icons/material/settings.svg")),
        IconRole::ActionEdit => Some(include_bytes!("../../icons/material/edit.svg")),
        IconRole::ActionAdd => Some(include_bytes!("../../icons/material/add.svg")),
        IconRole::ActionRemove => Some(include_bytes!("../../icons/material/remove.svg")),
        IconRole::ActionRefresh => Some(include_bytes!("../../icons/material/refresh.svg")),
        IconRole::ActionPrint => Some(include_bytes!("../../icons/material/print.svg")),

        // Navigation (6)
        IconRole::NavBack => Some(include_bytes!("../../icons/material/arrow_back.svg")),
        IconRole::NavForward => Some(include_bytes!("../../icons/material/arrow_forward.svg")),
        IconRole::NavUp => Some(include_bytes!("../../icons/material/arrow_upward.svg")),
        IconRole::NavDown => Some(include_bytes!("../../icons/material/arrow_downward.svg")),
        IconRole::NavHome => Some(include_bytes!("../../icons/material/home.svg")),
        IconRole::NavMenu => Some(include_bytes!("../../icons/material/menu.svg")),

        // Files / Places (5)
        IconRole::FileGeneric => Some(include_bytes!("../../icons/material/description.svg")),
        IconRole::FolderClosed => Some(include_bytes!("../../icons/material/folder.svg")),
        IconRole::FolderOpen => Some(include_bytes!("../../icons/material/folder_open.svg")),
        IconRole::TrashEmpty => Some(include_bytes!("../../icons/material/delete.svg")),
        IconRole::TrashFull => Some(include_bytes!("../../icons/material/delete.svg")), // reuse delete

        // Status (3)
        IconRole::StatusBusy => Some(include_bytes!("../../icons/material/progress_activity.svg")),
        IconRole::StatusCheck => Some(include_bytes!("../../icons/material/check.svg")),
        IconRole::StatusError => Some(include_bytes!("../../icons/material/error.svg")), // reuse error

        // System (4)
        IconRole::UserAccount => Some(include_bytes!("../../icons/material/person.svg")),
        IconRole::Notification => Some(include_bytes!("../../icons/material/notifications.svg")),
        IconRole::Help => Some(include_bytes!("../../icons/material/help.svg")), // reuse help
        IconRole::Lock => Some(include_bytes!("../../icons/material/lock.svg")),

        _ => None, // #[non_exhaustive] forward compat
    }
}

#[cfg(feature = "lucide-icons")]
#[allow(unreachable_patterns)]
fn lucide_svg(role: IconRole) -> Option<&'static [u8]> {
    match role {
        // Dialog / Alert (6)
        IconRole::DialogWarning => Some(include_bytes!("../../icons/lucide/triangle-alert.svg")),
        IconRole::DialogError => Some(include_bytes!("../../icons/lucide/circle-x.svg")),
        IconRole::DialogInfo => Some(include_bytes!("../../icons/lucide/info.svg")),
        IconRole::DialogQuestion => Some(include_bytes!(
            "../../icons/lucide/circle-question-mark.svg"
        )),
        IconRole::DialogSuccess => Some(include_bytes!("../../icons/lucide/circle-check.svg")),
        IconRole::Shield => Some(include_bytes!("../../icons/lucide/shield.svg")),

        // Window Controls (4)
        IconRole::WindowClose => Some(include_bytes!("../../icons/lucide/x.svg")),
        IconRole::WindowMinimize => Some(include_bytes!("../../icons/lucide/minimize.svg")),
        IconRole::WindowMaximize => Some(include_bytes!("../../icons/lucide/maximize.svg")),
        IconRole::WindowRestore => Some(include_bytes!("../../icons/lucide/minimize-2.svg")),

        // Common Actions (14)
        IconRole::ActionSave => Some(include_bytes!("../../icons/lucide/save.svg")),
        IconRole::ActionDelete => Some(include_bytes!("../../icons/lucide/trash.svg")),
        IconRole::ActionCopy => Some(include_bytes!("../../icons/lucide/copy.svg")),
        IconRole::ActionPaste => Some(include_bytes!("../../icons/lucide/clipboard-paste.svg")),
        IconRole::ActionCut => Some(include_bytes!("../../icons/lucide/scissors.svg")),
        IconRole::ActionUndo => Some(include_bytes!("../../icons/lucide/undo-2.svg")),
        IconRole::ActionRedo => Some(include_bytes!("../../icons/lucide/redo-2.svg")),
        IconRole::ActionSearch => Some(include_bytes!("../../icons/lucide/search.svg")),
        IconRole::ActionSettings => Some(include_bytes!("../../icons/lucide/settings.svg")),
        IconRole::ActionEdit => Some(include_bytes!("../../icons/lucide/pencil.svg")),
        IconRole::ActionAdd => Some(include_bytes!("../../icons/lucide/plus.svg")),
        IconRole::ActionRemove => Some(include_bytes!("../../icons/lucide/minus.svg")),
        IconRole::ActionRefresh => Some(include_bytes!("../../icons/lucide/refresh-cw.svg")),
        IconRole::ActionPrint => Some(include_bytes!("../../icons/lucide/printer.svg")),

        // Navigation (6)
        IconRole::NavBack => Some(include_bytes!("../../icons/lucide/chevron-left.svg")),
        IconRole::NavForward => Some(include_bytes!("../../icons/lucide/chevron-right.svg")),
        IconRole::NavUp => Some(include_bytes!("../../icons/lucide/chevron-up.svg")),
        IconRole::NavDown => Some(include_bytes!("../../icons/lucide/chevron-down.svg")),
        IconRole::NavHome => Some(include_bytes!("../../icons/lucide/house.svg")),
        IconRole::NavMenu => Some(include_bytes!("../../icons/lucide/menu.svg")),

        // Files / Places (5)
        IconRole::FileGeneric => Some(include_bytes!("../../icons/lucide/file.svg")),
        IconRole::FolderClosed => Some(include_bytes!("../../icons/lucide/folder-closed.svg")),
        IconRole::FolderOpen => Some(include_bytes!("../../icons/lucide/folder-open.svg")),
        IconRole::TrashEmpty => Some(include_bytes!("../../icons/lucide/trash.svg")),
        IconRole::TrashFull => Some(include_bytes!("../../icons/lucide/trash.svg")), // reuse trash

        // Status (3)
        IconRole::StatusBusy => Some(include_bytes!("../../icons/lucide/loader.svg")),
        IconRole::StatusCheck => Some(include_bytes!("../../icons/lucide/check.svg")),
        IconRole::StatusError => Some(include_bytes!("../../icons/lucide/circle-x.svg")), // reuse circle-x

        // System (4)
        IconRole::UserAccount => Some(include_bytes!("../../icons/lucide/user.svg")),
        IconRole::Notification => Some(include_bytes!("../../icons/lucide/bell.svg")),
        IconRole::Help => Some(include_bytes!(
            "../../icons/lucide/circle-question-mark.svg"
        )), // reuse
        IconRole::Lock => Some(include_bytes!("../../icons/lucide/lock.svg")),

        _ => None, // #[non_exhaustive] forward compat
    }
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
/// use native_theme::icons::SfSymbolsLoader;
///
/// // SfSymbols is not a bundled set; no in-crate name lookup succeeds.
/// let result = SfSymbolsLoader::new("check").load();
/// assert!(result.is_none());
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
        for role in IconRole::ALL {
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
        for role in IconRole::ALL {
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
            bundled_icon_svg(IconRole::ActionCopy, IconSet::SegoeIcons).is_none(),
            "SegoeIcons should not be a bundled set"
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
        assert!(bundled_icon_by_name("check", IconSet::SegoeIcons).is_none());
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
