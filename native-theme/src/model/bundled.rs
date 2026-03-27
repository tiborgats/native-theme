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
#[must_use = "this returns SVG bytes; it does not render the icon"]
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
        IconRole::StatusBusy => include_bytes!("../../icons/material/progress_activity.svg"),
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
        IconRole::StatusBusy => include_bytes!("../../icons/lucide/loader.svg"),
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
/// ```
/// use native_theme::{IconSet, bundled_icon_by_name};
///
/// // Without features enabled, bundled icons return None
/// let result = bundled_icon_by_name(IconSet::SfSymbols, "check");
/// assert!(result.is_none());
/// ```
#[must_use = "this returns SVG bytes; it does not render the icon"]
#[allow(unreachable_patterns, unused_variables)]
pub fn bundled_icon_by_name(set: IconSet, name: &str) -> Option<&'static [u8]> {
    match set {
        #[cfg(feature = "material-icons")]
        IconSet::Material => material_svg_by_name(name),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => lucide_svg_by_name(name),

        _ => None,
    }
}

#[cfg(feature = "lucide-icons")]
#[allow(unreachable_patterns)]
fn lucide_svg_by_name(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "a-large-small" => include_bytes!("../../icons/lucide/a-large-small.svg"),
        "arrow-down" => include_bytes!("../../icons/lucide/arrow-down.svg"),
        "arrow-left" => include_bytes!("../../icons/lucide/arrow-left.svg"),
        "arrow-right" => include_bytes!("../../icons/lucide/arrow-right.svg"),
        "arrow-up" => include_bytes!("../../icons/lucide/arrow-up.svg"),
        "asterisk" => include_bytes!("../../icons/lucide/asterisk.svg"),
        "bell" => include_bytes!("../../icons/lucide/bell.svg"),
        "book-open" => include_bytes!("../../icons/lucide/book-open.svg"),
        "bot" => include_bytes!("../../icons/lucide/bot.svg"),
        "building-2" => include_bytes!("../../icons/lucide/building-2.svg"),
        "calendar" => include_bytes!("../../icons/lucide/calendar.svg"),
        "case-sensitive" => include_bytes!("../../icons/lucide/case-sensitive.svg"),
        "chart-pie" => include_bytes!("../../icons/lucide/chart-pie.svg"),
        "check" => include_bytes!("../../icons/lucide/check.svg"),
        "chevron-down" => include_bytes!("../../icons/lucide/chevron-down.svg"),
        "chevron-left" => include_bytes!("../../icons/lucide/chevron-left.svg"),
        "chevron-right" => include_bytes!("../../icons/lucide/chevron-right.svg"),
        "chevrons-up-down" => include_bytes!("../../icons/lucide/chevrons-up-down.svg"),
        "chevron-up" => include_bytes!("../../icons/lucide/chevron-up.svg"),
        "circle-check" => include_bytes!("../../icons/lucide/circle-check.svg"),
        "circle-question-mark" => include_bytes!("../../icons/lucide/circle-question-mark.svg"),
        "circle-user" => include_bytes!("../../icons/lucide/circle-user.svg"),
        "circle-x" => include_bytes!("../../icons/lucide/circle-x.svg"),
        "clipboard-paste" => include_bytes!("../../icons/lucide/clipboard-paste.svg"),
        "close" => include_bytes!("../../icons/lucide/close.svg"),
        "copy" => include_bytes!("../../icons/lucide/copy.svg"),
        "dash" => include_bytes!("../../icons/lucide/dash.svg"),
        "delete" => include_bytes!("../../icons/lucide/delete.svg"),
        "ellipsis" => include_bytes!("../../icons/lucide/ellipsis.svg"),
        "ellipsis-vertical" => include_bytes!("../../icons/lucide/ellipsis-vertical.svg"),
        "external-link" => include_bytes!("../../icons/lucide/external-link.svg"),
        "eye" => include_bytes!("../../icons/lucide/eye.svg"),
        "eye-off" => include_bytes!("../../icons/lucide/eye-off.svg"),
        "file" => include_bytes!("../../icons/lucide/file.svg"),
        "folder" => include_bytes!("../../icons/lucide/folder.svg"),
        "folder-closed" => include_bytes!("../../icons/lucide/folder-closed.svg"),
        "folder-open" => include_bytes!("../../icons/lucide/folder-open.svg"),
        "frame" => include_bytes!("../../icons/lucide/frame.svg"),
        "gallery-vertical-end" => include_bytes!("../../icons/lucide/gallery-vertical-end.svg"),
        "github" => include_bytes!("../../icons/lucide/github.svg"),
        "globe" => include_bytes!("../../icons/lucide/globe.svg"),
        "heart" => include_bytes!("../../icons/lucide/heart.svg"),
        "heart-off" => include_bytes!("../../icons/lucide/heart-off.svg"),
        "house" => include_bytes!("../../icons/lucide/house.svg"),
        "inbox" => include_bytes!("../../icons/lucide/inbox.svg"),
        "info" => include_bytes!("../../icons/lucide/info.svg"),
        "inspect" => include_bytes!("../../icons/lucide/inspect.svg"),
        "layout-dashboard" => include_bytes!("../../icons/lucide/layout-dashboard.svg"),
        "loader" => include_bytes!("../../icons/lucide/loader.svg"),
        "loader-circle" => include_bytes!("../../icons/lucide/loader-circle.svg"),
        "lock" => include_bytes!("../../icons/lucide/lock.svg"),
        "map" => include_bytes!("../../icons/lucide/map.svg"),
        "maximize" => include_bytes!("../../icons/lucide/maximize.svg"),
        "menu" => include_bytes!("../../icons/lucide/menu.svg"),
        "minimize" => include_bytes!("../../icons/lucide/minimize.svg"),
        "minimize-2" => include_bytes!("../../icons/lucide/minimize-2.svg"),
        "minus" => include_bytes!("../../icons/lucide/minus.svg"),
        "moon" => include_bytes!("../../icons/lucide/moon.svg"),
        "palette" => include_bytes!("../../icons/lucide/palette.svg"),
        "panel-bottom" => include_bytes!("../../icons/lucide/panel-bottom.svg"),
        "panel-bottom-open" => include_bytes!("../../icons/lucide/panel-bottom-open.svg"),
        "panel-left" => include_bytes!("../../icons/lucide/panel-left.svg"),
        "panel-left-close" => include_bytes!("../../icons/lucide/panel-left-close.svg"),
        "panel-left-open" => include_bytes!("../../icons/lucide/panel-left-open.svg"),
        "panel-right" => include_bytes!("../../icons/lucide/panel-right.svg"),
        "panel-right-close" => include_bytes!("../../icons/lucide/panel-right-close.svg"),
        "panel-right-open" => include_bytes!("../../icons/lucide/panel-right-open.svg"),
        "pencil" => include_bytes!("../../icons/lucide/pencil.svg"),
        "plus" => include_bytes!("../../icons/lucide/plus.svg"),
        "printer" => include_bytes!("../../icons/lucide/printer.svg"),
        "redo" => include_bytes!("../../icons/lucide/redo.svg"),
        "redo-2" => include_bytes!("../../icons/lucide/redo-2.svg"),
        "refresh-cw" => include_bytes!("../../icons/lucide/refresh-cw.svg"),
        "replace" => include_bytes!("../../icons/lucide/replace.svg"),
        "resize-corner" => include_bytes!("../../icons/lucide/resize-corner.svg"),
        "save" => include_bytes!("../../icons/lucide/save.svg"),
        "scissors" => include_bytes!("../../icons/lucide/scissors.svg"),
        "search" => include_bytes!("../../icons/lucide/search.svg"),
        "settings" => include_bytes!("../../icons/lucide/settings.svg"),
        "settings-2" => include_bytes!("../../icons/lucide/settings-2.svg"),
        "shield" => include_bytes!("../../icons/lucide/shield.svg"),
        "sort-ascending" => include_bytes!("../../icons/lucide/sort-ascending.svg"),
        "sort-descending" => include_bytes!("../../icons/lucide/sort-descending.svg"),
        "square-terminal" => include_bytes!("../../icons/lucide/square-terminal.svg"),
        "star" => include_bytes!("../../icons/lucide/star.svg"),
        "star-off" => include_bytes!("../../icons/lucide/star-off.svg"),
        "sun" => include_bytes!("../../icons/lucide/sun.svg"),
        "thumbs-down" => include_bytes!("../../icons/lucide/thumbs-down.svg"),
        "thumbs-up" => include_bytes!("../../icons/lucide/thumbs-up.svg"),
        "trash-2" => include_bytes!("../../icons/lucide/trash-2.svg"),
        "triangle-alert" => include_bytes!("../../icons/lucide/triangle-alert.svg"),
        "undo" => include_bytes!("../../icons/lucide/undo.svg"),
        "undo-2" => include_bytes!("../../icons/lucide/undo-2.svg"),
        "user" => include_bytes!("../../icons/lucide/user.svg"),
        "window-close" => include_bytes!("../../icons/lucide/window-close.svg"),
        "window-maximize" => include_bytes!("../../icons/lucide/window-maximize.svg"),
        "window-minimize" => include_bytes!("../../icons/lucide/window-minimize.svg"),
        "window-restore" => include_bytes!("../../icons/lucide/window-restore.svg"),
        "x" => include_bytes!("../../icons/lucide/x.svg"),
        _ => return None,
    })
}

#[cfg(feature = "material-icons")]
#[allow(unreachable_patterns)]
fn material_svg_by_name(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "account_circle" => include_bytes!("../../icons/material/account_circle.svg"),
        "add" => include_bytes!("../../icons/material/add.svg"),
        "apartment" => include_bytes!("../../icons/material/apartment.svg"),
        "arrow_back" => include_bytes!("../../icons/material/arrow_back.svg"),
        "arrow_downward" => include_bytes!("../../icons/material/arrow_downward.svg"),
        "arrow_forward" => include_bytes!("../../icons/material/arrow_forward.svg"),
        "arrow_upward" => include_bytes!("../../icons/material/arrow_upward.svg"),
        "autorenew" => include_bytes!("../../icons/material/autorenew.svg"),
        "calendar_today" => include_bytes!("../../icons/material/calendar_today.svg"),
        "cancel" => include_bytes!("../../icons/material/cancel.svg"),
        "check" => include_bytes!("../../icons/material/check.svg"),
        "check_circle" => include_bytes!("../../icons/material/check_circle.svg"),
        "chevron_left" => include_bytes!("../../icons/material/chevron_left.svg"),
        "chevron_right" => include_bytes!("../../icons/material/chevron_right.svg"),
        "close" => include_bytes!("../../icons/material/close.svg"),
        "close_fullscreen" => include_bytes!("../../icons/material/close_fullscreen.svg"),
        "code" => include_bytes!("../../icons/material/code.svg"),
        "content_copy" => include_bytes!("../../icons/material/content_copy.svg"),
        "crop_free" => include_bytes!("../../icons/material/crop_free.svg"),
        "dark_mode" => include_bytes!("../../icons/material/dark_mode.svg"),
        "dashboard" => include_bytes!("../../icons/material/dashboard.svg"),
        "delete" => include_bytes!("../../icons/material/delete.svg"),
        "description" => include_bytes!("../../icons/material/description.svg"),
        "developer_mode" => include_bytes!("../../icons/material/developer_mode.svg"),
        "dock_to_bottom" => include_bytes!("../../icons/material/dock_to_bottom.svg"),
        "drag_indicator" => include_bytes!("../../icons/material/drag_indicator.svg"),
        "emergency" => include_bytes!("../../icons/material/emergency.svg"),
        "expand_less" => include_bytes!("../../icons/material/expand_less.svg"),
        "expand_more" => include_bytes!("../../icons/material/expand_more.svg"),
        "favorite" => include_bytes!("../../icons/material/favorite.svg"),
        "find_replace" => include_bytes!("../../icons/material/find_replace.svg"),
        "folder" => include_bytes!("../../icons/material/folder.svg"),
        "folder_open" => include_bytes!("../../icons/material/folder_open.svg"),
        "font_size" => include_bytes!("../../icons/material/font_size.svg"),
        "heart_broken" => include_bytes!("../../icons/material/heart_broken.svg"),
        "inbox" => include_bytes!("../../icons/material/inbox.svg"),
        "info" => include_bytes!("../../icons/material/info.svg"),
        "language" => include_bytes!("../../icons/material/language.svg"),
        "left_panel_close" => include_bytes!("../../icons/material/left_panel_close.svg"),
        "left_panel_open" => include_bytes!("../../icons/material/left_panel_open.svg"),
        "light_mode" => include_bytes!("../../icons/material/light_mode.svg"),
        "map" => include_bytes!("../../icons/material/map.svg"),
        "match_case" => include_bytes!("../../icons/material/match_case.svg"),
        "menu" => include_bytes!("../../icons/material/menu.svg"),
        "menu_book" => include_bytes!("../../icons/material/menu_book.svg"),
        "minimize" => include_bytes!("../../icons/material/minimize.svg"),
        "more_horiz" => include_bytes!("../../icons/material/more_horiz.svg"),
        "more_vert" => include_bytes!("../../icons/material/more_vert.svg"),
        "notifications" => include_bytes!("../../icons/material/notifications.svg"),
        "open_in_full" => include_bytes!("../../icons/material/open_in_full.svg"),
        "open_in_new" => include_bytes!("../../icons/material/open_in_new.svg"),
        "palette" => include_bytes!("../../icons/material/palette.svg"),
        "person" => include_bytes!("../../icons/material/person.svg"),
        "pie_chart" => include_bytes!("../../icons/material/pie_chart.svg"),
        "progress_activity" => include_bytes!("../../icons/material/progress_activity.svg"),
        "redo" => include_bytes!("../../icons/material/redo.svg"),
        "remove" => include_bytes!("../../icons/material/remove.svg"),
        "right_panel_close" => include_bytes!("../../icons/material/right_panel_close.svg"),
        "right_panel_open" => include_bytes!("../../icons/material/right_panel_open.svg"),
        "search" => include_bytes!("../../icons/material/search.svg"),
        "settings" => include_bytes!("../../icons/material/settings.svg"),
        "side_navigation" => include_bytes!("../../icons/material/side_navigation.svg"),
        "smart_toy" => include_bytes!("../../icons/material/smart_toy.svg"),
        "star" => include_bytes!("../../icons/material/star.svg"),
        "star_border" => include_bytes!("../../icons/material/star_border.svg"),
        "terminal" => include_bytes!("../../icons/material/terminal.svg"),
        "thumb_down" => include_bytes!("../../icons/material/thumb_down.svg"),
        "thumb_up" => include_bytes!("../../icons/material/thumb_up.svg"),
        "tune" => include_bytes!("../../icons/material/tune.svg"),
        "undo" => include_bytes!("../../icons/material/undo.svg"),
        "unfold_more" => include_bytes!("../../icons/material/unfold_more.svg"),
        "view_carousel" => include_bytes!("../../icons/material/view_carousel.svg"),
        "visibility" => include_bytes!("../../icons/material/visibility.svg"),
        "visibility_off" => include_bytes!("../../icons/material/visibility_off.svg"),
        "warning" => include_bytes!("../../icons/material/warning.svg"),
        "web_asset" => include_bytes!("../../icons/material/web_asset.svg"),
        _ => return None,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === Feature-gated tests: Material ===

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_icons_cover_all_roles() {
        for role in IconRole::ALL {
            let svg = bundled_icon_svg(IconSet::Material, role);
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
            .filter_map(|role| bundled_icon_svg(IconSet::Material, *role))
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
            let svg = bundled_icon_svg(IconSet::Lucide, role);
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
            .filter_map(|role| bundled_icon_svg(IconSet::Lucide, *role))
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

    // === bundled_icon_by_name tests ===

    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_by_name_covers_gpui_icons() {
        let names = [
            "a-large-small",
            "arrow-down",
            "arrow-left",
            "arrow-right",
            "arrow-up",
            "asterisk",
            "bell",
            "book-open",
            "bot",
            "building-2",
            "calendar",
            "case-sensitive",
            "chart-pie",
            "check",
            "chevron-down",
            "chevron-left",
            "chevron-right",
            "chevrons-up-down",
            "chevron-up",
            "circle-check",
            "circle-user",
            "circle-x",
            "close",
            "copy",
            "dash",
            "delete",
            "ellipsis",
            "ellipsis-vertical",
            "external-link",
            "eye",
            "eye-off",
            "file",
            "folder",
            "folder-closed",
            "folder-open",
            "frame",
            "gallery-vertical-end",
            "github",
            "globe",
            "heart",
            "heart-off",
            "inbox",
            "info",
            "inspect",
            "layout-dashboard",
            "loader",
            "loader-circle",
            "map",
            "maximize",
            "menu",
            "minimize",
            "minus",
            "moon",
            "palette",
            "panel-bottom",
            "panel-bottom-open",
            "panel-left",
            "panel-left-close",
            "panel-left-open",
            "panel-right",
            "panel-right-close",
            "panel-right-open",
            "plus",
            "redo",
            "redo-2",
            "replace",
            "resize-corner",
            "search",
            "settings",
            "settings-2",
            "sort-ascending",
            "sort-descending",
            "square-terminal",
            "star",
            "star-off",
            "sun",
            "thumbs-down",
            "thumbs-up",
            "triangle-alert",
            "undo",
            "undo-2",
            "user",
            "window-close",
            "window-maximize",
            "window-minimize",
            "window-restore",
        ];
        for name in names {
            let svg = bundled_icon_by_name(IconSet::Lucide, name);
            assert!(svg.is_some(), "Lucide by-name missing: {}", name);
            let bytes = svg.unwrap();
            let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(
                content.contains("<svg"),
                "Lucide {} does not contain <svg tag",
                name
            );
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_by_name_covers_gpui_icons() {
        let names = [
            "font_size",
            "arrow_downward",
            "arrow_back",
            "arrow_forward",
            "arrow_upward",
            "emergency",
            "notifications",
            "menu_book",
            "smart_toy",
            "apartment",
            "calendar_today",
            "match_case",
            "pie_chart",
            "check",
            "expand_more",
            "chevron_left",
            "chevron_right",
            "unfold_more",
            "expand_less",
            "check_circle",
            "account_circle",
            "cancel",
            "close",
            "content_copy",
            "remove",
            "delete",
            "more_horiz",
            "more_vert",
            "open_in_new",
            "visibility",
            "visibility_off",
            "description",
            "folder",
            "folder_open",
            "crop_free",
            "view_carousel",
            "code",
            "language",
            "favorite",
            "heart_broken",
            "inbox",
            "info",
            "developer_mode",
            "dashboard",
            "progress_activity",
            "autorenew",
            "map",
            "open_in_full",
            "menu",
            "minimize",
            "dark_mode",
            "palette",
            "dock_to_bottom",
            "web_asset",
            "side_navigation",
            "left_panel_close",
            "left_panel_open",
            "right_panel_close",
            "right_panel_open",
            "add",
            "redo",
            "find_replace",
            "drag_indicator",
            "search",
            "settings",
            "tune",
            "terminal",
            "star",
            "star_border",
            "light_mode",
            "thumb_down",
            "thumb_up",
            "warning",
            "undo",
            "person",
            "close_fullscreen",
        ];
        for name in names {
            let svg = bundled_icon_by_name(IconSet::Material, name);
            assert!(svg.is_some(), "Material by-name missing: {}", name);
            let bytes = svg.unwrap();
            let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(
                content.contains("<svg"),
                "Material {} does not contain <svg tag",
                name
            );
        }
    }

    #[test]
    fn by_name_non_bundled_sets_return_none() {
        assert!(bundled_icon_by_name(IconSet::SfSymbols, "check").is_none());
        assert!(bundled_icon_by_name(IconSet::Freedesktop, "check").is_none());
        assert!(bundled_icon_by_name(IconSet::SegoeIcons, "check").is_none());
    }

    #[test]
    fn by_name_unknown_name_returns_none() {
        assert!(bundled_icon_by_name(IconSet::Lucide, "nonexistent-icon-xyz").is_none());
        assert!(bundled_icon_by_name(IconSet::Material, "nonexistent_icon_xyz").is_none());
    }
}
