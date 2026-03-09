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
/// is enabled. Callers typically convert to `IconData::Svg(bytes.to_vec())`.
///
/// # Examples
///
/// ```
/// use native_theme::{IconSet, IconRole, bundled_icon_svg};
///
/// // Without features enabled, bundled icons return None
/// let result = bundled_icon_svg(IconSet::SfSymbols, IconRole::ActionCopy);
/// assert!(result.is_none());
/// ```
#[allow(unreachable_patterns, unused_variables)]
pub fn bundled_icon_svg(set: IconSet, role: IconRole) -> Option<&'static [u8]> {
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
    Some(match role {
        // Dialog / Alert (6)
        IconRole::DialogWarning => include_bytes!("../../icons/material/warning.svg"),
        IconRole::DialogError => include_bytes!("../../icons/material/error.svg"),
        IconRole::DialogInfo => include_bytes!("../../icons/material/info.svg"),
        IconRole::DialogQuestion => include_bytes!("../../icons/material/help.svg"),
        IconRole::DialogSuccess => include_bytes!("../../icons/material/check_circle.svg"),
        IconRole::Shield => include_bytes!("../../icons/material/shield.svg"),

        // Window Controls (4)
        IconRole::WindowClose => include_bytes!("../../icons/material/close.svg"),
        IconRole::WindowMinimize => include_bytes!("../../icons/material/minimize.svg"),
        IconRole::WindowMaximize => include_bytes!("../../icons/material/open_in_full.svg"),
        IconRole::WindowRestore => include_bytes!("../../icons/material/close_fullscreen.svg"),

        // Common Actions (14)
        IconRole::ActionSave => include_bytes!("../../icons/material/save.svg"),
        IconRole::ActionDelete => include_bytes!("../../icons/material/delete.svg"),
        IconRole::ActionCopy => include_bytes!("../../icons/material/content_copy.svg"),
        IconRole::ActionPaste => include_bytes!("../../icons/material/content_paste.svg"),
        IconRole::ActionCut => include_bytes!("../../icons/material/content_cut.svg"),
        IconRole::ActionUndo => include_bytes!("../../icons/material/undo.svg"),
        IconRole::ActionRedo => include_bytes!("../../icons/material/redo.svg"),
        IconRole::ActionSearch => include_bytes!("../../icons/material/search.svg"),
        IconRole::ActionSettings => include_bytes!("../../icons/material/settings.svg"),
        IconRole::ActionEdit => include_bytes!("../../icons/material/edit.svg"),
        IconRole::ActionAdd => include_bytes!("../../icons/material/add.svg"),
        IconRole::ActionRemove => include_bytes!("../../icons/material/remove.svg"),
        IconRole::ActionRefresh => include_bytes!("../../icons/material/refresh.svg"),
        IconRole::ActionPrint => include_bytes!("../../icons/material/print.svg"),

        // Navigation (6)
        IconRole::NavBack => include_bytes!("../../icons/material/arrow_back.svg"),
        IconRole::NavForward => include_bytes!("../../icons/material/arrow_forward.svg"),
        IconRole::NavUp => include_bytes!("../../icons/material/arrow_upward.svg"),
        IconRole::NavDown => include_bytes!("../../icons/material/arrow_downward.svg"),
        IconRole::NavHome => include_bytes!("../../icons/material/home.svg"),
        IconRole::NavMenu => include_bytes!("../../icons/material/menu.svg"),

        // Files / Places (5)
        IconRole::FileGeneric => include_bytes!("../../icons/material/description.svg"),
        IconRole::FolderClosed => include_bytes!("../../icons/material/folder.svg"),
        IconRole::FolderOpen => include_bytes!("../../icons/material/folder_open.svg"),
        IconRole::TrashEmpty => include_bytes!("../../icons/material/delete.svg"),
        IconRole::TrashFull => include_bytes!("../../icons/material/delete.svg"), // reuse delete

        // Status (3)
        IconRole::StatusLoading => include_bytes!("../../icons/material/progress_activity.svg"),
        IconRole::StatusCheck => include_bytes!("../../icons/material/check.svg"),
        IconRole::StatusError => include_bytes!("../../icons/material/error.svg"), // reuse error

        // System (4)
        IconRole::UserAccount => include_bytes!("../../icons/material/person.svg"),
        IconRole::Notification => include_bytes!("../../icons/material/notifications.svg"),
        IconRole::Help => include_bytes!("../../icons/material/help.svg"), // reuse help
        IconRole::Lock => include_bytes!("../../icons/material/lock.svg"),

        _ => return None, // #[non_exhaustive] forward compat
    })
}

#[cfg(feature = "lucide-icons")]
#[allow(unreachable_patterns)]
fn lucide_svg(role: IconRole) -> Option<&'static [u8]> {
    Some(match role {
        // Dialog / Alert (6)
        IconRole::DialogWarning => include_bytes!("../../icons/lucide/triangle-alert.svg"),
        IconRole::DialogError => include_bytes!("../../icons/lucide/circle-x.svg"),
        IconRole::DialogInfo => include_bytes!("../../icons/lucide/info.svg"),
        IconRole::DialogQuestion => include_bytes!("../../icons/lucide/circle-question-mark.svg"),
        IconRole::DialogSuccess => include_bytes!("../../icons/lucide/circle-check.svg"),
        IconRole::Shield => include_bytes!("../../icons/lucide/shield.svg"),

        // Window Controls (4)
        IconRole::WindowClose => include_bytes!("../../icons/lucide/x.svg"),
        IconRole::WindowMinimize => include_bytes!("../../icons/lucide/minimize.svg"),
        IconRole::WindowMaximize => include_bytes!("../../icons/lucide/maximize.svg"),
        IconRole::WindowRestore => include_bytes!("../../icons/lucide/minimize-2.svg"),

        // Common Actions (14)
        IconRole::ActionSave => include_bytes!("../../icons/lucide/save.svg"),
        IconRole::ActionDelete => include_bytes!("../../icons/lucide/trash-2.svg"),
        IconRole::ActionCopy => include_bytes!("../../icons/lucide/copy.svg"),
        IconRole::ActionPaste => include_bytes!("../../icons/lucide/clipboard-paste.svg"),
        IconRole::ActionCut => include_bytes!("../../icons/lucide/scissors.svg"),
        IconRole::ActionUndo => include_bytes!("../../icons/lucide/undo-2.svg"),
        IconRole::ActionRedo => include_bytes!("../../icons/lucide/redo-2.svg"),
        IconRole::ActionSearch => include_bytes!("../../icons/lucide/search.svg"),
        IconRole::ActionSettings => include_bytes!("../../icons/lucide/settings.svg"),
        IconRole::ActionEdit => include_bytes!("../../icons/lucide/pencil.svg"),
        IconRole::ActionAdd => include_bytes!("../../icons/lucide/plus.svg"),
        IconRole::ActionRemove => include_bytes!("../../icons/lucide/minus.svg"),
        IconRole::ActionRefresh => include_bytes!("../../icons/lucide/refresh-cw.svg"),
        IconRole::ActionPrint => include_bytes!("../../icons/lucide/printer.svg"),

        // Navigation (6)
        IconRole::NavBack => include_bytes!("../../icons/lucide/chevron-left.svg"),
        IconRole::NavForward => include_bytes!("../../icons/lucide/chevron-right.svg"),
        IconRole::NavUp => include_bytes!("../../icons/lucide/chevron-up.svg"),
        IconRole::NavDown => include_bytes!("../../icons/lucide/chevron-down.svg"),
        IconRole::NavHome => include_bytes!("../../icons/lucide/house.svg"),
        IconRole::NavMenu => include_bytes!("../../icons/lucide/menu.svg"),

        // Files / Places (5)
        IconRole::FileGeneric => include_bytes!("../../icons/lucide/file.svg"),
        IconRole::FolderClosed => include_bytes!("../../icons/lucide/folder-closed.svg"),
        IconRole::FolderOpen => include_bytes!("../../icons/lucide/folder-open.svg"),
        IconRole::TrashEmpty => include_bytes!("../../icons/lucide/trash-2.svg"),
        IconRole::TrashFull => include_bytes!("../../icons/lucide/trash-2.svg"), // reuse trash-2

        // Status (3)
        IconRole::StatusLoading => include_bytes!("../../icons/lucide/loader.svg"),
        IconRole::StatusCheck => include_bytes!("../../icons/lucide/check.svg"),
        IconRole::StatusError => include_bytes!("../../icons/lucide/circle-x.svg"), // reuse circle-x

        // System (4)
        IconRole::UserAccount => include_bytes!("../../icons/lucide/user.svg"),
        IconRole::Notification => include_bytes!("../../icons/lucide/bell.svg"),
        IconRole::Help => include_bytes!("../../icons/lucide/circle-question-mark.svg"), // reuse
        IconRole::Lock => include_bytes!("../../icons/lucide/lock.svg"),

        _ => return None, // #[non_exhaustive] forward compat
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Feature-gated tests: Material ===

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_icons_cover_all_roles() {
        for role in IconRole::ALL {
            let svg = bundled_icon_svg(IconSet::Material, role);
            assert!(
                svg.is_some(),
                "Material icons missing SVG for {:?}",
                role
            );
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
            .filter_map(|role| bundled_icon_svg(IconSet::Material, *role))
            .map(|svg| svg.len())
            .sum();
        assert!(
            total < 200 * 1024,
            "Material icons total size {} bytes exceeds 200KB budget",
            total
        );
    }

    // === Feature-gated tests: Lucide ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_icons_cover_all_roles() {
        for role in IconRole::ALL {
            let svg = bundled_icon_svg(IconSet::Lucide, role);
            assert!(
                svg.is_some(),
                "Lucide icons missing SVG for {:?}",
                role
            );
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
            .filter_map(|role| bundled_icon_svg(IconSet::Lucide, *role))
            .map(|svg| svg.len())
            .sum();
        assert!(
            total < 100 * 1024,
            "Lucide icons total size {} bytes exceeds 100KB budget",
            total
        );
    }

    // === Non-feature-gated tests ===

    #[test]
    fn non_bundled_sets_return_none() {
        assert!(
            bundled_icon_svg(IconSet::SfSymbols, IconRole::ActionCopy).is_none(),
            "SfSymbols should not be a bundled set"
        );
        assert!(
            bundled_icon_svg(IconSet::Freedesktop, IconRole::ActionCopy).is_none(),
            "Freedesktop should not be a bundled set"
        );
        assert!(
            bundled_icon_svg(IconSet::SegoeIcons, IconRole::ActionCopy).is_none(),
            "SegoeIcons should not be a bundled set"
        );
    }
}
