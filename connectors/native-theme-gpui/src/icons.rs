//! Icon conversion functions for the gpui connector.
//!
//! Provides two main functions:
//! - [`icon_name`]: Maps [`IconRole`] to gpui-component's [`IconName`] for the Lucide icon set.
//!   This is a zero-I/O shortcut since gpui-component already bundles Lucide SVGs.
//! - [`to_image_source`]: Converts [`IconData`] to a gpui [`ImageSource`] for rendering.

use gpui::{Hsla, Image, ImageFormat, ImageSource};
use gpui_component::IconName;
use native_theme::{IconData, IconRole};
use std::sync::Arc;

/// Map an [`IconRole`] to a gpui-component [`IconName`] for the Lucide icon set.
///
/// Returns `Some(IconName)` for roles that have a direct Lucide equivalent in
/// gpui-component's bundled icon set. Returns `None` for roles where
/// gpui-component doesn't ship the corresponding Lucide icon.
///
/// This is a zero-I/O operation -- no icon files are loaded. The returned
/// `IconName` can be rendered directly via gpui-component's `Icon::new()`.
///
/// # Coverage
///
/// Maps 30 of the 42 `IconRole` variants to `IconName`. The 12 unmapped roles
/// (Shield, ActionSave, ActionPaste, ActionCut, ActionEdit, ActionRefresh,
/// ActionPrint, NavHome, TrashFull, DialogQuestion, Help, Lock) have no
/// corresponding Lucide icon in gpui-component 0.5.
///
/// # Examples
///
/// ```ignore
/// use native_theme::IconRole;
/// use native_theme_gpui::icons::icon_name;
///
/// assert_eq!(icon_name(IconRole::DialogWarning), Some(IconName::TriangleAlert));
/// assert_eq!(icon_name(IconRole::Shield), None);
/// ```
pub fn icon_name(role: IconRole) -> Option<IconName> {
    Some(match role {
        // Dialog / Alert
        IconRole::DialogWarning => IconName::TriangleAlert,
        IconRole::DialogError => IconName::CircleX,
        IconRole::DialogInfo => IconName::Info,
        IconRole::DialogSuccess => IconName::CircleCheck,

        // Window Controls
        IconRole::WindowClose => IconName::WindowClose,
        IconRole::WindowMinimize => IconName::WindowMinimize,
        IconRole::WindowMaximize => IconName::WindowMaximize,
        IconRole::WindowRestore => IconName::WindowRestore,

        // Common Actions
        IconRole::ActionDelete => IconName::Delete,
        IconRole::ActionCopy => IconName::Copy,
        IconRole::ActionUndo => IconName::Undo2,
        IconRole::ActionRedo => IconName::Redo2,
        IconRole::ActionSearch => IconName::Search,
        IconRole::ActionSettings => IconName::Settings,
        IconRole::ActionAdd => IconName::Plus,
        IconRole::ActionRemove => IconName::Minus,

        // Navigation
        IconRole::NavBack => IconName::ChevronLeft,
        IconRole::NavForward => IconName::ChevronRight,
        IconRole::NavUp => IconName::ChevronUp,
        IconRole::NavDown => IconName::ChevronDown,
        IconRole::NavMenu => IconName::Menu,

        // Files / Places
        IconRole::FileGeneric => IconName::File,
        IconRole::FolderClosed => IconName::FolderClosed,
        IconRole::FolderOpen => IconName::FolderOpen,
        IconRole::TrashEmpty => IconName::Delete,

        // Status
        IconRole::StatusLoading => IconName::Loader,
        IconRole::StatusCheck => IconName::Check,
        IconRole::StatusError => IconName::CircleX,

        // System
        IconRole::UserAccount => IconName::User,
        IconRole::Notification => IconName::Bell,

        // No Lucide equivalent in gpui-component 0.5
        _ => return None,
    })
}

/// Map a gpui-component icon name string to its canonical Lucide icon name.
///
/// Returns the kebab-case Lucide name for use with
/// [`native_theme::bundled_icon_by_name`].
///
/// Covers all 86 gpui-component `IconName` variants.
pub fn lucide_name_for_gpui_icon(gpui_name: &str) -> Option<&'static str> {
    Some(match gpui_name {
        "ALargeSmall" => "a-large-small",
        "ArrowDown" => "arrow-down",
        "ArrowLeft" => "arrow-left",
        "ArrowRight" => "arrow-right",
        "ArrowUp" => "arrow-up",
        "Asterisk" => "asterisk",
        "Bell" => "bell",
        "BookOpen" => "book-open",
        "Bot" => "bot",
        "Building2" => "building-2",
        "Calendar" => "calendar",
        "CaseSensitive" => "case-sensitive",
        "ChartPie" => "chart-pie",
        "Check" => "check",
        "ChevronDown" => "chevron-down",
        "ChevronLeft" => "chevron-left",
        "ChevronRight" => "chevron-right",
        "ChevronsUpDown" => "chevrons-up-down",
        "ChevronUp" => "chevron-up",
        "CircleCheck" => "circle-check",
        "CircleUser" => "circle-user",
        "CircleX" => "circle-x",
        "Close" => "close",
        "Copy" => "copy",
        "Dash" => "dash",
        "Delete" => "delete",
        "Ellipsis" => "ellipsis",
        "EllipsisVertical" => "ellipsis-vertical",
        "ExternalLink" => "external-link",
        "Eye" => "eye",
        "EyeOff" => "eye-off",
        "File" => "file",
        "Folder" => "folder",
        "FolderClosed" => "folder-closed",
        "FolderOpen" => "folder-open",
        "Frame" => "frame",
        "GalleryVerticalEnd" => "gallery-vertical-end",
        "GitHub" => "github",
        "Globe" => "globe",
        "Heart" => "heart",
        "HeartOff" => "heart-off",
        "Inbox" => "inbox",
        "Info" => "info",
        "Inspector" => "inspect",
        "LayoutDashboard" => "layout-dashboard",
        "Loader" => "loader",
        "LoaderCircle" => "loader-circle",
        "Map" => "map",
        "Maximize" => "maximize",
        "Menu" => "menu",
        "Minimize" => "minimize",
        "Minus" => "minus",
        "Moon" => "moon",
        "Palette" => "palette",
        "PanelBottom" => "panel-bottom",
        "PanelBottomOpen" => "panel-bottom-open",
        "PanelLeft" => "panel-left",
        "PanelLeftClose" => "panel-left-close",
        "PanelLeftOpen" => "panel-left-open",
        "PanelRight" => "panel-right",
        "PanelRightClose" => "panel-right-close",
        "PanelRightOpen" => "panel-right-open",
        "Plus" => "plus",
        "Redo" => "redo",
        "Redo2" => "redo-2",
        "Replace" => "replace",
        "ResizeCorner" => "resize-corner",
        "Search" => "search",
        "Settings" => "settings",
        "Settings2" => "settings-2",
        "SortAscending" => "sort-ascending",
        "SortDescending" => "sort-descending",
        "SquareTerminal" => "square-terminal",
        "Star" => "star",
        "StarOff" => "star-off",
        "Sun" => "sun",
        "ThumbsDown" => "thumbs-down",
        "ThumbsUp" => "thumbs-up",
        "TriangleAlert" => "triangle-alert",
        "Undo" => "undo",
        "Undo2" => "undo-2",
        "User" => "user",
        "WindowClose" => "window-close",
        "WindowMaximize" => "window-maximize",
        "WindowMinimize" => "window-minimize",
        "WindowRestore" => "window-restore",
        _ => return None,
    })
}

/// Map a gpui-component icon name string to its canonical Material icon name.
///
/// Returns the snake_case Material Symbols name for use with
/// [`native_theme::bundled_icon_by_name`].
///
/// Covers all 86 gpui-component `IconName` variants.
pub fn material_name_for_gpui_icon(gpui_name: &str) -> Option<&'static str> {
    Some(match gpui_name {
        "ALargeSmall" => "font_size",
        "ArrowDown" => "arrow_downward",
        "ArrowLeft" => "arrow_back",
        "ArrowRight" => "arrow_forward",
        "ArrowUp" => "arrow_upward",
        "Asterisk" => "emergency",
        "Bell" => "notifications",
        "BookOpen" => "menu_book",
        "Bot" => "smart_toy",
        "Building2" => "apartment",
        "Calendar" => "calendar_today",
        "CaseSensitive" => "match_case",
        "ChartPie" => "pie_chart",
        "Check" => "check",
        "ChevronDown" => "expand_more",
        "ChevronLeft" => "chevron_left",
        "ChevronRight" => "chevron_right",
        "ChevronsUpDown" => "unfold_more",
        "ChevronUp" => "expand_less",
        "CircleCheck" => "check_circle",
        "CircleUser" => "account_circle",
        "CircleX" => "cancel",
        "Close" => "close",
        "Copy" => "content_copy",
        "Dash" => "remove",
        "Delete" => "delete",
        "Ellipsis" => "more_horiz",
        "EllipsisVertical" => "more_vert",
        "ExternalLink" => "open_in_new",
        "Eye" => "visibility",
        "EyeOff" => "visibility_off",
        "File" => "description",
        "Folder" => "folder",
        "FolderClosed" => "folder",
        "FolderOpen" => "folder_open",
        "Frame" => "crop_free",
        "GalleryVerticalEnd" => "view_carousel",
        "GitHub" => "code",
        "Globe" => "language",
        "Heart" => "favorite",
        "HeartOff" => "heart_broken",
        "Inbox" => "inbox",
        "Info" => "info",
        "Inspector" => "developer_mode",
        "LayoutDashboard" => "dashboard",
        "Loader" => "progress_activity",
        "LoaderCircle" => "autorenew",
        "Map" => "map",
        "Maximize" => "open_in_full",
        "Menu" => "menu",
        "Minimize" => "minimize",
        "Minus" => "remove",
        "Moon" => "dark_mode",
        "Palette" => "palette",
        "PanelBottom" => "dock_to_bottom",
        "PanelBottomOpen" => "web_asset",
        "PanelLeft" => "side_navigation",
        "PanelLeftClose" => "left_panel_close",
        "PanelLeftOpen" => "left_panel_open",
        "PanelRight" => "right_panel_close",
        "PanelRightClose" => "right_panel_close",
        "PanelRightOpen" => "right_panel_open",
        "Plus" => "add",
        "Redo" => "redo",
        "Redo2" => "redo",
        "Replace" => "find_replace",
        "ResizeCorner" => "drag_indicator",
        "Search" => "search",
        "Settings" => "settings",
        "Settings2" => "tune",
        "SortAscending" => "arrow_upward",
        "SortDescending" => "arrow_downward",
        "SquareTerminal" => "terminal",
        "Star" => "star",
        "StarOff" => "star_border",
        "Sun" => "light_mode",
        "ThumbsDown" => "thumb_down",
        "ThumbsUp" => "thumb_up",
        "TriangleAlert" => "warning",
        "Undo" => "undo",
        "Undo2" => "undo",
        "User" => "person",
        "WindowClose" => "close",
        "WindowMaximize" => "open_in_full",
        "WindowMinimize" => "minimize",
        "WindowRestore" => "close_fullscreen",
        _ => return None,
    })
}

/// Map a gpui-component icon name string to its freedesktop icon name for the
/// given desktop environment.
///
/// Returns the best freedesktop name for the detected DE's naming
/// convention. When KDE and GNOME use different names for the same
/// concept, the DE parameter selects the right one. For freedesktop
/// standard names (present in all themes), the DE is ignored.
///
/// ## Confidence levels
///
/// Each mapping is annotated with a confidence level:
/// - `exact`: the freedesktop icon is semantically identical
/// - `close`: same concept, minor visual difference
/// - `approximate`: best available match, different metaphor
///
/// Covers all 86 gpui-component `IconName` variants.
#[cfg(target_os = "linux")]
pub fn freedesktop_name_for_gpui_icon(
    gpui_name: &str,
    de: native_theme::LinuxDesktop,
) -> Option<&'static str> {
    use native_theme::LinuxDesktop;

    let is_gnome = matches!(
        de,
        LinuxDesktop::Gnome | LinuxDesktop::Budgie | LinuxDesktop::Cinnamon | LinuxDesktop::Mate
    );

    Some(match gpui_name {
        // --- Icons with freedesktop standard names (all DEs) ---
        "BookOpen"         => "help-contents",          // close
        "Bot"              => "face-smile",             // approximate
        "ChevronDown"      => "go-down",                // close: full nav arrow, not disclosure chevron
        "ChevronLeft"      => "go-previous",            // close
        "ChevronRight"     => "go-next",                // close
        "ChevronUp"        => "go-up",                  // close
        "CircleX"          => "dialog-error",           // close
        "Copy"             => "edit-copy",              // exact
        "Dash"             => "list-remove",            // exact
        "Delete"           => "edit-delete",            // exact
        "File"             => "text-x-generic",         // exact
        "Folder"           => "folder",                 // exact
        "FolderClosed"     => "folder",                 // exact
        "FolderOpen"       => "folder-open",            // exact
        "HeartOff"         => "non-starred",            // close: un-favorite semantics
        "Info"             => "dialog-information",     // exact
        "LayoutDashboard"  => "view-grid",              // close
        "Map"              => "find-location",          // close
        "Maximize"         => "view-fullscreen",        // exact
        "Menu"             => "open-menu",              // exact
        "Minimize"         => "window-minimize",        // exact
        "Minus"            => "list-remove",            // exact
        "Moon"             => "weather-clear-night",    // close: dark mode toggle
        "Plus"             => "list-add",               // exact
        "Redo"             => "edit-redo",              // exact
        "Redo2"            => "edit-redo",              // exact
        "Replace"          => "edit-find-replace",      // exact
        "Search"           => "edit-find",              // exact
        "Settings"         => "preferences-system",     // exact
        "SortAscending"    => "view-sort-ascending",    // exact
        "SortDescending"   => "view-sort-descending",   // exact
        "SquareTerminal"   => "utilities-terminal",     // close
        "Star"             => "starred",                // exact
        "StarOff"          => "non-starred",            // exact
        "Sun"              => "weather-clear",          // close: light mode toggle
        "TriangleAlert"    => "dialog-warning",         // exact
        "Undo"             => "edit-undo",              // exact
        "Undo2"            => "edit-undo",              // exact
        "User"             => "system-users",           // exact
        "WindowClose"      => "window-close",           // exact
        "WindowMaximize"   => "window-maximize",        // exact
        "WindowMinimize"   => "window-minimize",        // exact
        "WindowRestore"    => "window-restore",         // exact

        // --- Icons where KDE and GNOME diverge ---
        "Ellipsis"         => if is_gnome { "view-more-horizontal" } else { "overflow-menu" },  // exact
        "EllipsisVertical" => if is_gnome { "view-more" } else { "overflow-menu" },             // close: no vertical variant in KDE
        "Eye"              => if is_gnome { "view-reveal" } else { "view-visible" },            // exact
        "EyeOff"           => if is_gnome { "view-conceal" } else { "view-hidden" },            // exact
        "Heart"            => if is_gnome { "starred" } else { "emblem-favorite" },             // close
        "PanelLeft"        => if is_gnome { "sidebar-show" } else { "sidebar-expand-left" },    // close
        "PanelLeftClose"   => if is_gnome { "sidebar-show" } else { "view-left-close" },        // close
        "PanelLeftOpen"    => if is_gnome { "sidebar-show" } else { "view-left-new" },          // close
        "PanelRight"       => if is_gnome { "sidebar-show-right" } else { "view-right-new" },   // close
        "PanelRightClose"  => if is_gnome { "sidebar-show-right" } else { "view-right-close" }, // close
        "PanelRightOpen"   => if is_gnome { "sidebar-show-right" } else { "view-right-new" },   // close
        "ResizeCorner"     => if is_gnome { "list-drag-handle" } else { "drag-handle" },        // close

        // --- KDE-only names (no GNOME equivalent — GNOME falls back to bundled) ---
        "ALargeSmall"      => "format-font-size-more",  // close
        "ArrowDown"        => "go-down-skip",           // close: full arrow vs nav arrow
        "ArrowLeft"        => "go-previous-skip",       // close
        "ArrowRight"       => "go-next-skip",           // close
        "ArrowUp"          => "go-up-skip",             // close
        "Asterisk"         => "rating",                 // approximate
        "Bell"             => "notification-active",    // close
        "Building2"        => "applications-office",    // approximate
        "Calendar"         => "view-calendar",          // exact
        "CaseSensitive"    => "format-text-uppercase",  // close
        "ChartPie"         => "office-chart-pie",       // exact
        "Check"            => "dialog-ok",              // close: checkmark vs OK button
        "ChevronsUpDown"   => "handle-sort",            // close
        "CircleCheck"      => "emblem-ok-symbolic",     // close
        "CircleUser"       => "user-identity",          // close
        "Close"            => "tab-close",              // exact
        "ExternalLink"     => "external-link",          // exact
        "Frame"            => "select-rectangular",     // close
        "GalleryVerticalEnd" => "view-list-icons",      // approximate
        "GitHub"           => "vcs-branch",             // approximate: VCS branch as substitute
        "Globe"            => "globe",                  // exact
        "Inbox"            => "mail-folder-inbox",      // exact
        "Inspector"        => "code-context",           // close
        "Loader"           => "process-working",        // exact
        "LoaderCircle"     => "process-working",        // exact
        "Palette"          => "palette",                // exact
        "PanelBottom"      => "view-split-top-bottom",  // close
        "PanelBottomOpen"  => "view-split-top-bottom",  // close: no separate open variant
        "Settings2"        => "configure",              // exact
        "ThumbsDown"       => "rating-unrated",         // approximate
        "ThumbsUp"         => "approved",               // approximate

        _ => return None,
    })
}

/// Convert [`IconData`] to a gpui [`ImageSource`] for rendering.
///
/// - `IconData::Svg`: Wraps the SVG bytes in `Image::from_bytes(ImageFormat::Svg, ...)`.
/// - `IconData::Rgba`: Encodes as BMP with a BITMAPV4HEADER and wraps in
///   `Image::from_bytes(ImageFormat::Bmp, ...)`. This avoids needing the `png` crate.
///
/// # Examples
///
/// ```ignore
/// use native_theme::IconData;
/// use native_theme_gpui::icons::to_image_source;
///
/// let svg = IconData::Svg(b"<svg></svg>".to_vec());
/// let source = to_image_source(&svg);
/// ```
pub fn to_image_source(data: &IconData) -> ImageSource {
    match data {
        IconData::Svg(bytes) => {
            let image = Image::from_bytes(ImageFormat::Svg, bytes.clone());
            ImageSource::Image(Arc::new(image))
        }
        IconData::Rgba {
            width,
            height,
            data,
        } => {
            let bmp = encode_rgba_as_bmp(*width, *height, data);
            let image = Image::from_bytes(ImageFormat::Bmp, bmp);
            ImageSource::Image(Arc::new(image))
        }
        // Forward-compatible: treat unknown variants as empty SVG
        _ => {
            let image = Image::from_bytes(ImageFormat::Svg, Vec::new());
            ImageSource::Image(Arc::new(image))
        }
    }
}

/// Convert [`IconData`] to a gpui [`ImageSource`], colorizing SVGs with the
/// given color.
///
/// Intended for **monochrome** bundled icon sets (Material, Lucide) where
/// SVGs use `currentColor` or implicit black fill. When rendered via
/// `gpui::img()`, `currentColor` resolves to black and implicit fills stay
/// black — making icons invisible in dark themes. This function rewrites
/// SVG bytes so strokes and fills use the provided color instead.
///
/// **Do not use for system/OS icons** (freedesktop, SF Symbols, Segoe Fluent)
/// — those may be multi-colored and should be rendered with [`to_image_source`]
/// to preserve their native palette.
///
/// RGBA icons are passed through unchanged (they carry their own colors).
pub fn to_image_source_colored(data: &IconData, color: Hsla) -> ImageSource {
    match data {
        IconData::Svg(bytes) => {
            let colored = colorize_svg(bytes, color);
            let image = Image::from_bytes(ImageFormat::Svg, colored);
            ImageSource::Image(Arc::new(image))
        }
        other => to_image_source(other),
    }
}

/// Rewrite SVG bytes to use the given color for strokes and fills.
///
/// - Replaces all occurrences of `currentColor` with the hex color.
/// - If the SVG has no `fill=` attribute in its root `<svg>` tag and didn't
///   contain `currentColor`, injects `fill="<hex>"` so that paths with
///   implicit black fill use the theme color instead.
fn colorize_svg(svg_bytes: &[u8], color: Hsla) -> Vec<u8> {
    let rgba: gpui::Rgba = color.into();
    let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

    let svg_str = String::from_utf8_lossy(svg_bytes);

    // Replace currentColor (handles Lucide-style SVGs)
    if svg_str.contains("currentColor") {
        return svg_str.replace("currentColor", &hex).into_bytes();
    }

    // No currentColor found — inject fill into the root <svg> tag
    // (handles Material-style SVGs with implicit black fill)
    if let Some(pos) = svg_str.find("<svg") {
        if let Some(close) = svg_str[pos..].find('>') {
            let tag_end = pos + close;
            // Check if root <svg> already has a fill attribute
            let tag = &svg_str[pos..tag_end];
            if !tag.contains("fill=") {
                let mut result = String::with_capacity(svg_str.len() + 20);
                result.push_str(&svg_str[..tag_end]);
                result.push_str(&format!(" fill=\"{}\"", hex));
                result.push_str(&svg_str[tag_end..]);
                return result.into_bytes();
            }
        }
    }

    // SVG already has explicit fill and no currentColor — return as-is
    svg_bytes.to_vec()
}

/// Encode RGBA pixel data as a BMP with BITMAPV4HEADER.
///
/// BMP with a V4 header supports 32-bit RGBA via channel masks.
/// The pixel data is stored bottom-up (BMP convention) with no compression.
fn encode_rgba_as_bmp(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let pixel_data_size = (width * height * 4) as usize;
    let header_size: u32 = 14; // BITMAPFILEHEADER
    let dib_header_size: u32 = 108; // BITMAPV4HEADER
    let file_size = header_size + dib_header_size + pixel_data_size as u32;

    let mut buf = Vec::with_capacity(file_size as usize);

    // BITMAPFILEHEADER (14 bytes)
    buf.extend_from_slice(b"BM"); // signature
    buf.extend_from_slice(&file_size.to_le_bytes()); // file size
    buf.extend_from_slice(&0u16.to_le_bytes()); // reserved1
    buf.extend_from_slice(&0u16.to_le_bytes()); // reserved2
    buf.extend_from_slice(&(header_size + dib_header_size).to_le_bytes()); // pixel data offset

    // BITMAPV4HEADER (108 bytes)
    buf.extend_from_slice(&dib_header_size.to_le_bytes()); // header size
    buf.extend_from_slice(&(width as i32).to_le_bytes()); // width
    // Negative height = top-down (avoids flipping rows)
    buf.extend_from_slice(&(-(height as i32)).to_le_bytes()); // height (top-down)
    buf.extend_from_slice(&1u16.to_le_bytes()); // planes
    buf.extend_from_slice(&32u16.to_le_bytes()); // bits per pixel
    buf.extend_from_slice(&3u32.to_le_bytes()); // compression = BI_BITFIELDS
    buf.extend_from_slice(&(pixel_data_size as u32).to_le_bytes()); // image size
    buf.extend_from_slice(&2835u32.to_le_bytes()); // x pixels per meter (~72 DPI)
    buf.extend_from_slice(&2835u32.to_le_bytes()); // y pixels per meter
    buf.extend_from_slice(&0u32.to_le_bytes()); // colors used
    buf.extend_from_slice(&0u32.to_le_bytes()); // important colors

    // Channel masks (RGBA -> BGRA in BMP, but we use BI_BITFIELDS to specify layout)
    buf.extend_from_slice(&0x00FF0000u32.to_le_bytes()); // red mask
    buf.extend_from_slice(&0x0000FF00u32.to_le_bytes()); // green mask
    buf.extend_from_slice(&0x000000FFu32.to_le_bytes()); // blue mask
    buf.extend_from_slice(&0xFF000000u32.to_le_bytes()); // alpha mask

    // Color space type: LCS_sRGB
    buf.extend_from_slice(&0x73524742u32.to_le_bytes()); // 'sRGB'

    // CIEXYZTRIPLE endpoints (36 bytes of zeros)
    buf.extend_from_slice(&[0u8; 36]);

    // Gamma values (red, green, blue) - unused with sRGB
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());

    // Pixel data: RGBA -> BGRA conversion for BMP
    for pixel in rgba.chunks_exact(4) {
        buf.push(pixel[2]); // B
        buf.push(pixel[1]); // G
        buf.push(pixel[0]); // R
        buf.push(pixel[3]); // A
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- icon_name tests ---

    #[test]
    fn icon_name_dialog_warning_maps_to_triangle_alert() {
        assert!(matches!(icon_name(IconRole::DialogWarning), Some(IconName::TriangleAlert)));
    }

    #[test]
    fn icon_name_dialog_error_maps_to_circle_x() {
        assert!(matches!(icon_name(IconRole::DialogError), Some(IconName::CircleX)));
    }

    #[test]
    fn icon_name_dialog_info_maps_to_info() {
        assert!(matches!(icon_name(IconRole::DialogInfo), Some(IconName::Info)));
    }

    #[test]
    fn icon_name_dialog_success_maps_to_circle_check() {
        assert!(matches!(icon_name(IconRole::DialogSuccess), Some(IconName::CircleCheck)));
    }

    #[test]
    fn icon_name_window_close_maps() {
        assert!(matches!(icon_name(IconRole::WindowClose), Some(IconName::WindowClose)));
    }

    #[test]
    fn icon_name_action_copy_maps_to_copy() {
        assert!(matches!(icon_name(IconRole::ActionCopy), Some(IconName::Copy)));
    }

    #[test]
    fn icon_name_nav_back_maps_to_chevron_left() {
        assert!(matches!(icon_name(IconRole::NavBack), Some(IconName::ChevronLeft)));
    }

    #[test]
    fn icon_name_file_generic_maps_to_file() {
        assert!(matches!(icon_name(IconRole::FileGeneric), Some(IconName::File)));
    }

    #[test]
    fn icon_name_status_check_maps_to_check() {
        assert!(matches!(icon_name(IconRole::StatusCheck), Some(IconName::Check)));
    }

    #[test]
    fn icon_name_user_account_maps_to_user() {
        assert!(matches!(icon_name(IconRole::UserAccount), Some(IconName::User)));
    }

    #[test]
    fn icon_name_notification_maps_to_bell() {
        assert!(matches!(icon_name(IconRole::Notification), Some(IconName::Bell)));
    }

    // None cases
    #[test]
    fn icon_name_shield_returns_none() {
        assert!(icon_name(IconRole::Shield).is_none());
    }

    #[test]
    fn icon_name_lock_returns_none() {
        assert!(icon_name(IconRole::Lock).is_none());
    }

    #[test]
    fn icon_name_action_save_returns_none() {
        assert!(icon_name(IconRole::ActionSave).is_none());
    }

    #[test]
    fn icon_name_help_returns_none() {
        assert!(icon_name(IconRole::Help).is_none());
    }

    #[test]
    fn icon_name_dialog_question_returns_none() {
        assert!(icon_name(IconRole::DialogQuestion).is_none());
    }

    // Count test: at least 28 roles map to Some
    #[test]
    fn icon_name_maps_at_least_28_roles() {
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r).is_some())
            .count();
        assert!(
            some_count >= 28,
            "Expected at least 28 mappings, got {}",
            some_count
        );
    }

    #[test]
    fn icon_name_maps_exactly_30_roles() {
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r).is_some())
            .count();
        assert_eq!(some_count, 30, "Expected exactly 30 mappings, got {some_count}");
    }

    // --- to_image_source tests ---

    #[test]
    fn to_image_source_svg_returns_image_source() {
        let svg = IconData::Svg(b"<svg></svg>".to_vec());
        let source = to_image_source(&svg);
        // Verify it's an ImageSource::Image variant
        match source {
            ImageSource::Image(arc) => {
                assert_eq!(arc.format, ImageFormat::Svg);
                assert_eq!(arc.bytes, b"<svg></svg>");
            }
            _ => panic!("Expected ImageSource::Image for SVG data"),
        }
    }

    #[test]
    fn to_image_source_rgba_returns_bmp_image_source() {
        let rgba = IconData::Rgba {
            width: 2,
            height: 2,
            data: vec![
                255, 0, 0, 255, // red
                0, 255, 0, 255, // green
                0, 0, 255, 255, // blue
                255, 255, 0, 255, // yellow
            ],
        };
        let source = to_image_source(&rgba);
        match source {
            ImageSource::Image(arc) => {
                assert_eq!(arc.format, ImageFormat::Bmp);
                // BMP header starts with "BM"
                assert_eq!(&arc.bytes[0..2], b"BM");
            }
            _ => panic!("Expected ImageSource::Image for RGBA data"),
        }
    }

    // --- BMP encoding tests ---

    #[test]
    fn encode_rgba_as_bmp_correct_file_size() {
        let rgba = vec![0u8; 4 * 4 * 4]; // 4x4 image
        let bmp = encode_rgba_as_bmp(4, 4, &rgba);
        let expected_size = 14 + 108 + (4 * 4 * 4); // header + dib + pixels
        assert_eq!(bmp.len(), expected_size);
    }

    #[test]
    fn encode_rgba_as_bmp_starts_with_bm() {
        let rgba = vec![0u8; 4]; // 1x1 image
        let bmp = encode_rgba_as_bmp(1, 1, &rgba);
        assert_eq!(&bmp[0..2], b"BM");
    }

    #[test]
    fn encode_rgba_as_bmp_pixel_order_is_bgra() {
        // Input RGBA: R=0xAA, G=0xBB, B=0xCC, A=0xDD
        let rgba = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let bmp = encode_rgba_as_bmp(1, 1, &rgba);
        let pixel_offset = (14 + 108) as usize;
        // BMP stores as BGRA
        assert_eq!(bmp[pixel_offset], 0xCC); // B
        assert_eq!(bmp[pixel_offset + 1], 0xBB); // G
        assert_eq!(bmp[pixel_offset + 2], 0xAA); // R
        assert_eq!(bmp[pixel_offset + 3], 0xDD); // A
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod freedesktop_mapping_tests {
    use super::*;
    use native_theme::LinuxDesktop;

    #[test]
    fn all_86_gpui_icons_have_mapping_on_kde() {
        let all_names = [
            "ALargeSmall", "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp",
            "Asterisk", "Bell", "BookOpen", "Bot", "Building2",
            "Calendar", "CaseSensitive", "ChartPie", "Check", "ChevronDown",
            "ChevronLeft", "ChevronRight", "ChevronsUpDown", "ChevronUp",
            "CircleCheck", "CircleUser", "CircleX", "Close", "Copy",
            "Dash", "Delete", "Ellipsis", "EllipsisVertical", "ExternalLink",
            "Eye", "EyeOff", "File", "Folder", "FolderClosed",
            "FolderOpen", "Frame", "GalleryVerticalEnd", "GitHub", "Globe",
            "Heart", "HeartOff", "Inbox", "Info", "Inspector",
            "LayoutDashboard", "Loader", "LoaderCircle", "Map", "Maximize",
            "Menu", "Minimize", "Minus", "Moon", "Palette",
            "PanelBottom", "PanelBottomOpen", "PanelLeft", "PanelLeftClose",
            "PanelLeftOpen", "PanelRight", "PanelRightClose", "PanelRightOpen",
            "Plus", "Redo", "Redo2", "Replace", "ResizeCorner",
            "Search", "Settings", "Settings2", "SortAscending", "SortDescending",
            "SquareTerminal", "Star", "StarOff", "Sun", "ThumbsDown",
            "ThumbsUp", "TriangleAlert", "Undo", "Undo2", "User",
            "WindowClose", "WindowMaximize", "WindowMinimize", "WindowRestore",
        ];
        let mut missing = Vec::new();
        for name in &all_names {
            if freedesktop_name_for_gpui_icon(name, LinuxDesktop::Kde).is_none() {
                missing.push(*name);
            }
        }
        assert!(
            missing.is_empty(),
            "Missing KDE freedesktop mappings for: {:?}",
            missing,
        );
    }

    #[test]
    fn eye_differs_by_de() {
        assert_eq!(
            freedesktop_name_for_gpui_icon("Eye", LinuxDesktop::Kde),
            Some("view-visible"),
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon("Eye", LinuxDesktop::Gnome),
            Some("view-reveal"),
        );
    }

    #[test]
    fn freedesktop_standard_ignores_de() {
        // edit-copy is freedesktop standard — same for all DEs
        assert_eq!(
            freedesktop_name_for_gpui_icon("Copy", LinuxDesktop::Kde),
            freedesktop_name_for_gpui_icon("Copy", LinuxDesktop::Gnome),
        );
    }

    #[test]
    fn unknown_name_returns_none() {
        assert!(freedesktop_name_for_gpui_icon("NotARealIcon", LinuxDesktop::Kde).is_none());
    }

    #[test]
    fn all_kde_names_resolve_in_breeze() {
        let theme = native_theme::system_icon_theme();
        // Only meaningful on a KDE system with Breeze installed
        if !theme.to_lowercase().contains("breeze") {
            eprintln!("Skipping: system theme is '{}', not Breeze", theme);
            return;
        }

        let all_names = [
            "ALargeSmall", "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp",
            "Asterisk", "Bell", "BookOpen", "Bot", "Building2",
            "Calendar", "CaseSensitive", "ChartPie", "Check", "ChevronDown",
            "ChevronLeft", "ChevronRight", "ChevronsUpDown", "ChevronUp",
            "CircleCheck", "CircleUser", "CircleX", "Close", "Copy",
            "Dash", "Delete", "Ellipsis", "EllipsisVertical", "ExternalLink",
            "Eye", "EyeOff", "File", "Folder", "FolderClosed",
            "FolderOpen", "Frame", "GalleryVerticalEnd", "GitHub", "Globe",
            "Heart", "HeartOff", "Inbox", "Info", "Inspector",
            "LayoutDashboard", "Loader", "LoaderCircle", "Map", "Maximize",
            "Menu", "Minimize", "Minus", "Moon", "Palette",
            "PanelBottom", "PanelBottomOpen", "PanelLeft", "PanelLeftClose",
            "PanelLeftOpen", "PanelRight", "PanelRightClose", "PanelRightOpen",
            "Plus", "Redo", "Redo2", "Replace", "ResizeCorner",
            "Search", "Settings", "Settings2", "SortAscending", "SortDescending",
            "SquareTerminal", "Star", "StarOff", "Sun", "ThumbsDown",
            "ThumbsUp", "TriangleAlert", "Undo", "Undo2", "User",
            "WindowClose", "WindowMaximize", "WindowMinimize", "WindowRestore",
        ];

        let mut missing = Vec::new();
        for name in &all_names {
            let fd_name = freedesktop_name_for_gpui_icon(name, LinuxDesktop::Kde)
                .unwrap_or_else(|| panic!("{} has no KDE mapping", name));
            if native_theme::load_freedesktop_icon_by_name(fd_name, &theme).is_none() {
                missing.push(format!("{} -> {}", name, fd_name));
            }
        }
        assert!(
            missing.is_empty(),
            "These gpui icons did not resolve in Breeze:\n  {}",
            missing.join("\n  "),
        );
    }
}
