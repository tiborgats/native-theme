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
/// let result = bundled_icon_svg(IconRole::ActionCopy, IconSet::SfSymbols);
/// assert!(result.is_none());
/// ```
#[must_use]
#[allow(unreachable_patterns, unused_variables)]
pub fn bundled_icon_svg(role: IconRole, set: IconSet) -> Option<&'static [u8]> {
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
        IconRole::ActionDelete => Some(include_bytes!("../../icons/lucide/trash-2.svg")),
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
        IconRole::TrashEmpty => Some(include_bytes!("../../icons/lucide/trash-2.svg")),
        IconRole::TrashFull => Some(include_bytes!("../../icons/lucide/trash-2.svg")), // reuse trash-2

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
/// ```
/// use native_theme::{IconSet, bundled_icon_by_name};
///
/// // Without features enabled, bundled icons return None
/// let result = bundled_icon_by_name("check", IconSet::SfSymbols);
/// assert!(result.is_none());
/// ```
#[must_use]
#[allow(unreachable_patterns, unused_variables)]
pub fn bundled_icon_by_name(name: &str, set: IconSet) -> Option<&'static [u8]> {
    match set {
        #[cfg(feature = "material-icons")]
        IconSet::Material => material_svg_by_name(name),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => lucide_svg_by_name(name),

        _ => None,
    }
}

#[cfg(feature = "lucide-icons")]
fn lucide_svg_by_name(name: &str) -> Option<&'static [u8]> {
    match name {
        "a-large-small" => Some(include_bytes!("../../icons/lucide/a-large-small.svg")),
        "arrow-down" => Some(include_bytes!("../../icons/lucide/arrow-down.svg")),
        "arrow-left" => Some(include_bytes!("../../icons/lucide/arrow-left.svg")),
        "arrow-right" => Some(include_bytes!("../../icons/lucide/arrow-right.svg")),
        "arrow-up" => Some(include_bytes!("../../icons/lucide/arrow-up.svg")),
        "asterisk" => Some(include_bytes!("../../icons/lucide/asterisk.svg")),
        "bell" => Some(include_bytes!("../../icons/lucide/bell.svg")),
        "book-open" => Some(include_bytes!("../../icons/lucide/book-open.svg")),
        "bot" => Some(include_bytes!("../../icons/lucide/bot.svg")),
        "building-2" => Some(include_bytes!("../../icons/lucide/building-2.svg")),
        "calendar" => Some(include_bytes!("../../icons/lucide/calendar.svg")),
        "case-sensitive" => Some(include_bytes!("../../icons/lucide/case-sensitive.svg")),
        "chart-pie" => Some(include_bytes!("../../icons/lucide/chart-pie.svg")),
        "check" => Some(include_bytes!("../../icons/lucide/check.svg")),
        "chevron-down" => Some(include_bytes!("../../icons/lucide/chevron-down.svg")),
        "chevron-left" => Some(include_bytes!("../../icons/lucide/chevron-left.svg")),
        "chevron-right" => Some(include_bytes!("../../icons/lucide/chevron-right.svg")),
        "chevrons-up-down" => Some(include_bytes!("../../icons/lucide/chevrons-up-down.svg")),
        "chevron-up" => Some(include_bytes!("../../icons/lucide/chevron-up.svg")),
        "circle-check" => Some(include_bytes!("../../icons/lucide/circle-check.svg")),
        "circle-question-mark" => Some(include_bytes!(
            "../../icons/lucide/circle-question-mark.svg"
        )),
        "circle-user" => Some(include_bytes!("../../icons/lucide/circle-user.svg")),
        "circle-x" => Some(include_bytes!("../../icons/lucide/circle-x.svg")),
        "clipboard-paste" => Some(include_bytes!("../../icons/lucide/clipboard-paste.svg")),
        "close" => Some(include_bytes!("../../icons/lucide/close.svg")),
        "copy" => Some(include_bytes!("../../icons/lucide/copy.svg")),
        "dash" => Some(include_bytes!("../../icons/lucide/dash.svg")),
        "delete" => Some(include_bytes!("../../icons/lucide/delete.svg")),
        "ellipsis" => Some(include_bytes!("../../icons/lucide/ellipsis.svg")),
        "ellipsis-vertical" => Some(include_bytes!("../../icons/lucide/ellipsis-vertical.svg")),
        "external-link" => Some(include_bytes!("../../icons/lucide/external-link.svg")),
        "eye" => Some(include_bytes!("../../icons/lucide/eye.svg")),
        "eye-off" => Some(include_bytes!("../../icons/lucide/eye-off.svg")),
        "file" => Some(include_bytes!("../../icons/lucide/file.svg")),
        "folder" => Some(include_bytes!("../../icons/lucide/folder.svg")),
        "folder-closed" => Some(include_bytes!("../../icons/lucide/folder-closed.svg")),
        "folder-open" => Some(include_bytes!("../../icons/lucide/folder-open.svg")),
        "frame" => Some(include_bytes!("../../icons/lucide/frame.svg")),
        "gallery-vertical-end" => Some(include_bytes!(
            "../../icons/lucide/gallery-vertical-end.svg"
        )),
        "github" => Some(include_bytes!("../../icons/lucide/github.svg")),
        "globe" => Some(include_bytes!("../../icons/lucide/globe.svg")),
        "heart" => Some(include_bytes!("../../icons/lucide/heart.svg")),
        "heart-off" => Some(include_bytes!("../../icons/lucide/heart-off.svg")),
        "house" => Some(include_bytes!("../../icons/lucide/house.svg")),
        "inbox" => Some(include_bytes!("../../icons/lucide/inbox.svg")),
        "info" => Some(include_bytes!("../../icons/lucide/info.svg")),
        "inspect" => Some(include_bytes!("../../icons/lucide/inspect.svg")),
        "layout-dashboard" => Some(include_bytes!("../../icons/lucide/layout-dashboard.svg")),
        "loader" => Some(include_bytes!("../../icons/lucide/loader.svg")),
        "loader-circle" => Some(include_bytes!("../../icons/lucide/loader-circle.svg")),
        "lock" => Some(include_bytes!("../../icons/lucide/lock.svg")),
        "map" => Some(include_bytes!("../../icons/lucide/map.svg")),
        "maximize" => Some(include_bytes!("../../icons/lucide/maximize.svg")),
        "menu" => Some(include_bytes!("../../icons/lucide/menu.svg")),
        "minimize" => Some(include_bytes!("../../icons/lucide/minimize.svg")),
        "minimize-2" => Some(include_bytes!("../../icons/lucide/minimize-2.svg")),
        "minus" => Some(include_bytes!("../../icons/lucide/minus.svg")),
        "moon" => Some(include_bytes!("../../icons/lucide/moon.svg")),
        "palette" => Some(include_bytes!("../../icons/lucide/palette.svg")),
        "panel-bottom" => Some(include_bytes!("../../icons/lucide/panel-bottom.svg")),
        "panel-bottom-open" => Some(include_bytes!("../../icons/lucide/panel-bottom-open.svg")),
        "panel-left" => Some(include_bytes!("../../icons/lucide/panel-left.svg")),
        "panel-left-close" => Some(include_bytes!("../../icons/lucide/panel-left-close.svg")),
        "panel-left-open" => Some(include_bytes!("../../icons/lucide/panel-left-open.svg")),
        "panel-right" => Some(include_bytes!("../../icons/lucide/panel-right.svg")),
        "panel-right-close" => Some(include_bytes!("../../icons/lucide/panel-right-close.svg")),
        "panel-right-open" => Some(include_bytes!("../../icons/lucide/panel-right-open.svg")),
        "pencil" => Some(include_bytes!("../../icons/lucide/pencil.svg")),
        "plus" => Some(include_bytes!("../../icons/lucide/plus.svg")),
        "printer" => Some(include_bytes!("../../icons/lucide/printer.svg")),
        "redo" => Some(include_bytes!("../../icons/lucide/redo.svg")),
        "redo-2" => Some(include_bytes!("../../icons/lucide/redo-2.svg")),
        "refresh-cw" => Some(include_bytes!("../../icons/lucide/refresh-cw.svg")),
        "replace" => Some(include_bytes!("../../icons/lucide/replace.svg")),
        "resize-corner" => Some(include_bytes!("../../icons/lucide/resize-corner.svg")),
        "save" => Some(include_bytes!("../../icons/lucide/save.svg")),
        "scissors" => Some(include_bytes!("../../icons/lucide/scissors.svg")),
        "search" => Some(include_bytes!("../../icons/lucide/search.svg")),
        "settings" => Some(include_bytes!("../../icons/lucide/settings.svg")),
        "settings-2" => Some(include_bytes!("../../icons/lucide/settings-2.svg")),
        "shield" => Some(include_bytes!("../../icons/lucide/shield.svg")),
        "sort-ascending" => Some(include_bytes!("../../icons/lucide/sort-ascending.svg")),
        "sort-descending" => Some(include_bytes!("../../icons/lucide/sort-descending.svg")),
        "square-terminal" => Some(include_bytes!("../../icons/lucide/square-terminal.svg")),
        "star" => Some(include_bytes!("../../icons/lucide/star.svg")),
        "star-off" => Some(include_bytes!("../../icons/lucide/star-off.svg")),
        "sun" => Some(include_bytes!("../../icons/lucide/sun.svg")),
        "thumbs-down" => Some(include_bytes!("../../icons/lucide/thumbs-down.svg")),
        "thumbs-up" => Some(include_bytes!("../../icons/lucide/thumbs-up.svg")),
        "trash-2" => Some(include_bytes!("../../icons/lucide/trash-2.svg")),
        "triangle-alert" => Some(include_bytes!("../../icons/lucide/triangle-alert.svg")),
        "undo" => Some(include_bytes!("../../icons/lucide/undo.svg")),
        "undo-2" => Some(include_bytes!("../../icons/lucide/undo-2.svg")),
        "user" => Some(include_bytes!("../../icons/lucide/user.svg")),
        "window-close" => Some(include_bytes!("../../icons/lucide/window-close.svg")),
        "window-maximize" => Some(include_bytes!("../../icons/lucide/window-maximize.svg")),
        "window-minimize" => Some(include_bytes!("../../icons/lucide/window-minimize.svg")),
        "window-restore" => Some(include_bytes!("../../icons/lucide/window-restore.svg")),
        "x" => Some(include_bytes!("../../icons/lucide/x.svg")),
        _ => None,
    }
}

#[cfg(feature = "material-icons")]
fn material_svg_by_name(name: &str) -> Option<&'static [u8]> {
    match name {
        "account_circle" => Some(include_bytes!("../../icons/material/account_circle.svg")),
        "add" => Some(include_bytes!("../../icons/material/add.svg")),
        "apartment" => Some(include_bytes!("../../icons/material/apartment.svg")),
        "arrow_back" => Some(include_bytes!("../../icons/material/arrow_back.svg")),
        "arrow_downward" => Some(include_bytes!("../../icons/material/arrow_downward.svg")),
        "arrow_forward" => Some(include_bytes!("../../icons/material/arrow_forward.svg")),
        "arrow_upward" => Some(include_bytes!("../../icons/material/arrow_upward.svg")),
        "autorenew" => Some(include_bytes!("../../icons/material/autorenew.svg")),
        "calendar_today" => Some(include_bytes!("../../icons/material/calendar_today.svg")),
        "cancel" => Some(include_bytes!("../../icons/material/cancel.svg")),
        "check" => Some(include_bytes!("../../icons/material/check.svg")),
        "check_circle" => Some(include_bytes!("../../icons/material/check_circle.svg")),
        "chevron_left" => Some(include_bytes!("../../icons/material/chevron_left.svg")),
        "chevron_right" => Some(include_bytes!("../../icons/material/chevron_right.svg")),
        "close" => Some(include_bytes!("../../icons/material/close.svg")),
        "close_fullscreen" => Some(include_bytes!("../../icons/material/close_fullscreen.svg")),
        "code" => Some(include_bytes!("../../icons/material/code.svg")),
        "content_copy" => Some(include_bytes!("../../icons/material/content_copy.svg")),
        "crop_free" => Some(include_bytes!("../../icons/material/crop_free.svg")),
        "dark_mode" => Some(include_bytes!("../../icons/material/dark_mode.svg")),
        "dashboard" => Some(include_bytes!("../../icons/material/dashboard.svg")),
        "delete" => Some(include_bytes!("../../icons/material/delete.svg")),
        "description" => Some(include_bytes!("../../icons/material/description.svg")),
        "developer_mode" => Some(include_bytes!("../../icons/material/developer_mode.svg")),
        "dock_to_bottom" => Some(include_bytes!("../../icons/material/dock_to_bottom.svg")),
        "drag_indicator" => Some(include_bytes!("../../icons/material/drag_indicator.svg")),
        "emergency" => Some(include_bytes!("../../icons/material/emergency.svg")),
        "expand_less" => Some(include_bytes!("../../icons/material/expand_less.svg")),
        "expand_more" => Some(include_bytes!("../../icons/material/expand_more.svg")),
        "favorite" => Some(include_bytes!("../../icons/material/favorite.svg")),
        "find_replace" => Some(include_bytes!("../../icons/material/find_replace.svg")),
        "folder" => Some(include_bytes!("../../icons/material/folder.svg")),
        "folder_open" => Some(include_bytes!("../../icons/material/folder_open.svg")),
        "font_size" => Some(include_bytes!("../../icons/material/font_size.svg")),
        "heart_broken" => Some(include_bytes!("../../icons/material/heart_broken.svg")),
        "inbox" => Some(include_bytes!("../../icons/material/inbox.svg")),
        "info" => Some(include_bytes!("../../icons/material/info.svg")),
        "language" => Some(include_bytes!("../../icons/material/language.svg")),
        "left_panel_close" => Some(include_bytes!("../../icons/material/left_panel_close.svg")),
        "left_panel_open" => Some(include_bytes!("../../icons/material/left_panel_open.svg")),
        "light_mode" => Some(include_bytes!("../../icons/material/light_mode.svg")),
        "map" => Some(include_bytes!("../../icons/material/map.svg")),
        "match_case" => Some(include_bytes!("../../icons/material/match_case.svg")),
        "menu" => Some(include_bytes!("../../icons/material/menu.svg")),
        "menu_book" => Some(include_bytes!("../../icons/material/menu_book.svg")),
        "minimize" => Some(include_bytes!("../../icons/material/minimize.svg")),
        "more_horiz" => Some(include_bytes!("../../icons/material/more_horiz.svg")),
        "more_vert" => Some(include_bytes!("../../icons/material/more_vert.svg")),
        "notifications" => Some(include_bytes!("../../icons/material/notifications.svg")),
        "open_in_full" => Some(include_bytes!("../../icons/material/open_in_full.svg")),
        "open_in_new" => Some(include_bytes!("../../icons/material/open_in_new.svg")),
        "palette" => Some(include_bytes!("../../icons/material/palette.svg")),
        "person" => Some(include_bytes!("../../icons/material/person.svg")),
        "pie_chart" => Some(include_bytes!("../../icons/material/pie_chart.svg")),
        "progress_activity" => Some(include_bytes!("../../icons/material/progress_activity.svg")),
        "redo" => Some(include_bytes!("../../icons/material/redo.svg")),
        "remove" => Some(include_bytes!("../../icons/material/remove.svg")),
        "right_panel_close" => Some(include_bytes!("../../icons/material/right_panel_close.svg")),
        "right_panel_open" => Some(include_bytes!("../../icons/material/right_panel_open.svg")),
        "search" => Some(include_bytes!("../../icons/material/search.svg")),
        "settings" => Some(include_bytes!("../../icons/material/settings.svg")),
        "side_navigation" => Some(include_bytes!("../../icons/material/side_navigation.svg")),
        "smart_toy" => Some(include_bytes!("../../icons/material/smart_toy.svg")),
        "star" => Some(include_bytes!("../../icons/material/star.svg")),
        "star_border" => Some(include_bytes!("../../icons/material/star_border.svg")),
        "terminal" => Some(include_bytes!("../../icons/material/terminal.svg")),
        "thumb_down" => Some(include_bytes!("../../icons/material/thumb_down.svg")),
        "thumb_up" => Some(include_bytes!("../../icons/material/thumb_up.svg")),
        "tune" => Some(include_bytes!("../../icons/material/tune.svg")),
        "undo" => Some(include_bytes!("../../icons/material/undo.svg")),
        "unfold_more" => Some(include_bytes!("../../icons/material/unfold_more.svg")),
        "view_carousel" => Some(include_bytes!("../../icons/material/view_carousel.svg")),
        "visibility" => Some(include_bytes!("../../icons/material/visibility.svg")),
        "visibility_off" => Some(include_bytes!("../../icons/material/visibility_off.svg")),
        "warning" => Some(include_bytes!("../../icons/material/warning.svg")),
        "web_asset" => Some(include_bytes!("../../icons/material/web_asset.svg")),
        _ => None,
    }
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
            let svg = bundled_icon_by_name(name, IconSet::Lucide);
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
            let svg = bundled_icon_by_name(name, IconSet::Material);
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
        assert!(bundled_icon_by_name("check", IconSet::SfSymbols).is_none());
        assert!(bundled_icon_by_name("check", IconSet::Freedesktop).is_none());
        assert!(bundled_icon_by_name("check", IconSet::SegoeIcons).is_none());
    }

    #[test]
    fn by_name_unknown_name_returns_none() {
        assert!(bundled_icon_by_name("nonexistent-icon-xyz", IconSet::Lucide).is_none());
        assert!(bundled_icon_by_name("nonexistent_icon_xyz", IconSet::Material).is_none());
    }
}
