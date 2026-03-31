//! Icon conversion functions for the gpui connector.
//!
//! # Function Overview
//!
//! | Function | Purpose |
//! |----------|---------|
//! | [`icon_name`] | Map [`IconRole`] → [`IconName`] (Lucide, zero-I/O) |
//! | [`lucide_name_for_gpui_icon`] | Map [`IconName`] → Lucide name (`&str`) |
//! | [`material_name_for_gpui_icon`] | Map [`IconName`] → Material name (`&str`) |
//! | [`freedesktop_name_for_gpui_icon`] | Map [`IconName`] → freedesktop name (Linux only) |
//! | [`to_image_source`] | Convert [`IconData`] → [`ImageSource`] with optional color/size |
//! | [`into_image_source`] | Consuming variant of [`to_image_source`] (avoids clone) |
//! | [`custom_icon_to_image_source`] | Load + convert via [`IconProvider`] |
//! | [`animated_frames_to_image_sources`] | Convert animation frames → [`AnimatedImageSources`] |
//! | [`with_spin_animation`] | Wrap an SVG element with spin animation |

use gpui::{
    Animation, AnimationExt, Hsla, Image, ImageFormat, ImageSource, Svg, Transformation, percentage,
};
use gpui_component::IconName;
use native_theme::{AnimatedIcon, IconData, IconProvider, IconRole, load_custom_icon};
use std::sync::Arc;
use std::time::Duration;

/// Converted animation frames with timing metadata.
///
/// Returned by [`animated_frames_to_image_sources`]. Contains the
/// rasterized frames and the per-frame duration needed to drive playback.
#[derive(Clone)]
pub struct AnimatedImageSources {
    /// Rasterized frames ready for gpui rendering.
    pub sources: Vec<ImageSource>,
    /// Duration of each frame in milliseconds.
    pub frame_duration_ms: u32,
}

impl std::fmt::Debug for AnimatedImageSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimatedImageSources")
            .field("frame_count", &self.sources.len())
            .field("frame_duration_ms", &self.frame_duration_ms)
            .finish()
    }
}

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
#[must_use]
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
        IconRole::StatusBusy => IconName::Loader,
        IconRole::StatusCheck => IconName::Check,
        IconRole::StatusError => IconName::CircleX,

        // System
        IconRole::UserAccount => IconName::User,
        IconRole::Notification => IconName::Bell,

        // No Lucide equivalent in gpui-component 0.5
        _ => return None,
    })
}

/// Map a gpui-component [`IconName`] to its canonical Lucide icon name.
///
/// Returns the kebab-case Lucide name for use with
/// [`native_theme::bundled_icon_by_name`].
///
/// Covers all 86 gpui-component `IconName` variants.
#[must_use]
pub fn lucide_name_for_gpui_icon(icon: IconName) -> &'static str {
    match icon {
        IconName::ALargeSmall => "a-large-small",
        IconName::ArrowDown => "arrow-down",
        IconName::ArrowLeft => "arrow-left",
        IconName::ArrowRight => "arrow-right",
        IconName::ArrowUp => "arrow-up",
        IconName::Asterisk => "asterisk",
        IconName::Bell => "bell",
        IconName::BookOpen => "book-open",
        IconName::Bot => "bot",
        IconName::Building2 => "building-2",
        IconName::Calendar => "calendar",
        IconName::CaseSensitive => "case-sensitive",
        IconName::ChartPie => "chart-pie",
        IconName::Check => "check",
        IconName::ChevronDown => "chevron-down",
        IconName::ChevronLeft => "chevron-left",
        IconName::ChevronRight => "chevron-right",
        IconName::ChevronsUpDown => "chevrons-up-down",
        IconName::ChevronUp => "chevron-up",
        IconName::CircleCheck => "circle-check",
        IconName::CircleUser => "circle-user",
        IconName::CircleX => "circle-x",
        IconName::Close => "close",
        IconName::Copy => "copy",
        IconName::Dash => "dash",
        IconName::Delete => "delete",
        IconName::Ellipsis => "ellipsis",
        IconName::EllipsisVertical => "ellipsis-vertical",
        IconName::ExternalLink => "external-link",
        IconName::Eye => "eye",
        IconName::EyeOff => "eye-off",
        IconName::File => "file",
        IconName::Folder => "folder",
        IconName::FolderClosed => "folder-closed",
        IconName::FolderOpen => "folder-open",
        IconName::Frame => "frame",
        IconName::GalleryVerticalEnd => "gallery-vertical-end",
        IconName::GitHub => "github",
        IconName::Globe => "globe",
        IconName::Heart => "heart",
        IconName::HeartOff => "heart-off",
        IconName::Inbox => "inbox",
        IconName::Info => "info",
        IconName::Inspector => "inspect",
        IconName::LayoutDashboard => "layout-dashboard",
        IconName::Loader => "loader",
        IconName::LoaderCircle => "loader-circle",
        IconName::Map => "map",
        IconName::Maximize => "maximize",
        IconName::Menu => "menu",
        IconName::Minimize => "minimize",
        IconName::Minus => "minus",
        IconName::Moon => "moon",
        IconName::Palette => "palette",
        IconName::PanelBottom => "panel-bottom",
        IconName::PanelBottomOpen => "panel-bottom-open",
        IconName::PanelLeft => "panel-left",
        IconName::PanelLeftClose => "panel-left-close",
        IconName::PanelLeftOpen => "panel-left-open",
        IconName::PanelRight => "panel-right",
        IconName::PanelRightClose => "panel-right-close",
        IconName::PanelRightOpen => "panel-right-open",
        IconName::Plus => "plus",
        IconName::Redo => "redo",
        IconName::Redo2 => "redo-2",
        IconName::Replace => "replace",
        IconName::ResizeCorner => "resize-corner",
        IconName::Search => "search",
        IconName::Settings => "settings",
        IconName::Settings2 => "settings-2",
        IconName::SortAscending => "sort-ascending",
        IconName::SortDescending => "sort-descending",
        IconName::SquareTerminal => "square-terminal",
        IconName::Star => "star",
        IconName::StarOff => "star-off",
        IconName::Sun => "sun",
        IconName::ThumbsDown => "thumbs-down",
        IconName::ThumbsUp => "thumbs-up",
        IconName::TriangleAlert => "triangle-alert",
        IconName::Undo => "undo",
        IconName::Undo2 => "undo-2",
        IconName::User => "user",
        IconName::WindowClose => "window-close",
        IconName::WindowMaximize => "window-maximize",
        IconName::WindowMinimize => "window-minimize",
        IconName::WindowRestore => "window-restore",
    }
}

/// Map a gpui-component [`IconName`] to its canonical Material icon name.
///
/// Returns the snake_case Material Symbols name for use with
/// [`native_theme::bundled_icon_by_name`].
///
/// Covers all 86 gpui-component `IconName` variants.
#[must_use]
pub fn material_name_for_gpui_icon(icon: IconName) -> &'static str {
    match icon {
        IconName::ALargeSmall => "font_size",
        IconName::ArrowDown => "arrow_downward",
        IconName::ArrowLeft => "arrow_back",
        IconName::ArrowRight => "arrow_forward",
        IconName::ArrowUp => "arrow_upward",
        IconName::Asterisk => "emergency",
        IconName::Bell => "notifications",
        IconName::BookOpen => "menu_book",
        IconName::Bot => "smart_toy",
        IconName::Building2 => "apartment",
        IconName::Calendar => "calendar_today",
        IconName::CaseSensitive => "match_case",
        IconName::ChartPie => "pie_chart",
        IconName::Check => "check",
        IconName::ChevronDown => "expand_more",
        IconName::ChevronLeft => "chevron_left",
        IconName::ChevronRight => "chevron_right",
        IconName::ChevronsUpDown => "unfold_more",
        IconName::ChevronUp => "expand_less",
        IconName::CircleCheck => "check_circle",
        IconName::CircleUser => "account_circle",
        IconName::CircleX => "cancel",
        IconName::Close => "close",
        IconName::Copy => "content_copy",
        IconName::Dash => "remove",
        IconName::Delete => "delete",
        IconName::Ellipsis => "more_horiz",
        IconName::EllipsisVertical => "more_vert",
        IconName::ExternalLink => "open_in_new",
        IconName::Eye => "visibility",
        IconName::EyeOff => "visibility_off",
        IconName::File => "description",
        IconName::Folder => "folder",
        IconName::FolderClosed => "folder",
        IconName::FolderOpen => "folder_open",
        IconName::Frame => "crop_free",
        IconName::GalleryVerticalEnd => "view_carousel",
        IconName::GitHub => "code",
        IconName::Globe => "language",
        IconName::Heart => "favorite",
        IconName::HeartOff => "heart_broken",
        IconName::Inbox => "inbox",
        IconName::Info => "info",
        IconName::Inspector => "developer_mode",
        IconName::LayoutDashboard => "dashboard",
        IconName::Loader => "progress_activity",
        IconName::LoaderCircle => "autorenew",
        IconName::Map => "map",
        IconName::Maximize => "open_in_full",
        IconName::Menu => "menu",
        IconName::Minimize => "minimize",
        IconName::Minus => "remove",
        IconName::Moon => "dark_mode",
        IconName::Palette => "palette",
        IconName::PanelBottom => "dock_to_bottom",
        IconName::PanelBottomOpen => "web_asset",
        IconName::PanelLeft => "side_navigation",
        IconName::PanelLeftClose => "left_panel_close",
        IconName::PanelLeftOpen => "left_panel_open",
        IconName::PanelRight => "right_panel_close",
        IconName::PanelRightClose => "right_panel_close",
        IconName::PanelRightOpen => "right_panel_open",
        IconName::Plus => "add",
        IconName::Redo => "redo",
        IconName::Redo2 => "redo",
        IconName::Replace => "find_replace",
        IconName::ResizeCorner => "drag_indicator",
        IconName::Search => "search",
        IconName::Settings => "settings",
        IconName::Settings2 => "tune",
        IconName::SortAscending => "arrow_upward",
        IconName::SortDescending => "arrow_downward",
        IconName::SquareTerminal => "terminal",
        IconName::Star => "star",
        IconName::StarOff => "star_border",
        IconName::Sun => "light_mode",
        IconName::ThumbsDown => "thumb_down",
        IconName::ThumbsUp => "thumb_up",
        IconName::TriangleAlert => "warning",
        IconName::Undo => "undo",
        IconName::Undo2 => "undo",
        IconName::User => "person",
        IconName::WindowClose => "close",
        IconName::WindowMaximize => "open_in_full",
        IconName::WindowMinimize => "minimize",
        IconName::WindowRestore => "close_fullscreen",
    }
}

/// Map a gpui-component [`IconName`] to its freedesktop icon name for the
/// given desktop environment.
///
/// Returns the best freedesktop name for the detected DE's naming
/// convention. When KDE and GNOME use different names for the same
/// concept, the DE parameter selects the right one. For freedesktop
/// standard names (present in all themes), the DE is ignored.
///
/// GTK-based DEs (GNOME, Budgie, Cinnamon, MATE, XFCE) share the
/// Adwaita/GNOME naming convention. Qt-based DEs (KDE, LxQt) and
/// Unknown share the Breeze/KDE convention.
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
#[must_use]
pub fn freedesktop_name_for_gpui_icon(
    icon: IconName,
    de: native_theme::LinuxDesktop,
) -> &'static str {
    use native_theme::LinuxDesktop;

    // GTK-based DEs follow GNOME/Adwaita naming; Qt-based follow KDE/Breeze
    let is_gtk = matches!(
        de,
        LinuxDesktop::Gnome
            | LinuxDesktop::Budgie
            | LinuxDesktop::Cinnamon
            | LinuxDesktop::Mate
            | LinuxDesktop::Xfce
    );

    match icon {
        // --- Icons with freedesktop standard names (all DEs) ---
        IconName::BookOpen => "help-contents",      // close
        IconName::Bot => "face-smile",              // approximate
        IconName::ChevronDown => "go-down",         // close: full nav arrow, not disclosure chevron
        IconName::ChevronLeft => "go-previous",     // close
        IconName::ChevronRight => "go-next",        // close
        IconName::ChevronUp => "go-up",             // close
        IconName::CircleX => "dialog-error",        // close
        IconName::Copy => "edit-copy",              // exact
        IconName::Dash => "list-remove",            // exact
        IconName::Delete => "edit-delete",          // exact
        IconName::File => "text-x-generic",         // exact
        IconName::Folder => "folder",               // exact
        IconName::FolderClosed => "folder",         // exact
        IconName::FolderOpen => "folder-open",      // exact
        IconName::HeartOff => "non-starred",        // close: un-favorite semantics
        IconName::Info => "dialog-information",     // exact
        IconName::LayoutDashboard => "view-grid",   // close
        IconName::Map => "find-location",           // close
        IconName::Maximize => "view-fullscreen",    // exact
        IconName::Menu => "open-menu",              // exact
        IconName::Minimize => "window-minimize",    // exact
        IconName::Minus => "list-remove",           // exact
        IconName::Moon => "weather-clear-night",    // close: dark mode toggle
        IconName::Plus => "list-add",               // exact
        IconName::Redo => "edit-redo",              // exact
        IconName::Redo2 => "edit-redo",             // exact
        IconName::Replace => "edit-find-replace",   // exact
        IconName::Search => "edit-find",            // exact
        IconName::Settings => "preferences-system", // exact
        IconName::SortAscending => "view-sort-ascending", // exact
        IconName::SortDescending => "view-sort-descending", // exact
        IconName::SquareTerminal => "utilities-terminal", // close
        IconName::Star => "starred",                // exact
        IconName::StarOff => "non-starred",         // exact
        IconName::Sun => "weather-clear",           // close: light mode toggle
        IconName::TriangleAlert => "dialog-warning", // exact
        IconName::Undo => "edit-undo",              // exact
        IconName::Undo2 => "edit-undo",             // exact
        IconName::User => "system-users",           // exact
        IconName::WindowClose => "window-close",    // exact
        IconName::WindowMaximize => "window-maximize", // exact
        IconName::WindowMinimize => "window-minimize", // exact
        IconName::WindowRestore => "window-restore", // exact

        // --- Icons where KDE and GNOME both have names but they differ ---
        IconName::ArrowDown => {
            if is_gtk {
                "go-bottom"
            } else {
                "go-down-skip"
            }
        } // close
        IconName::ArrowLeft => {
            if is_gtk {
                "go-first"
            } else {
                "go-previous-skip"
            }
        } // close
        IconName::ArrowRight => {
            if is_gtk {
                "go-last"
            } else {
                "go-next-skip"
            }
        } // close
        IconName::ArrowUp => {
            if is_gtk {
                "go-top"
            } else {
                "go-up-skip"
            }
        } // close
        IconName::Calendar => {
            if is_gtk {
                "x-office-calendar"
            } else {
                "view-calendar"
            }
        } // exact
        IconName::Check => {
            if is_gtk {
                "object-select"
            } else {
                "dialog-ok"
            }
        } // close
        IconName::CircleCheck => {
            if is_gtk {
                "object-select"
            } else {
                "emblem-ok-symbolic"
            }
        } // close
        IconName::CircleUser => {
            if is_gtk {
                "avatar-default"
            } else {
                "user-identity"
            }
        } // close
        IconName::Close => {
            if is_gtk {
                "window-close"
            } else {
                "tab-close"
            }
        } // close
        IconName::Ellipsis => {
            if is_gtk {
                "view-more-horizontal"
            } else {
                "overflow-menu"
            }
        } // exact
        IconName::EllipsisVertical => {
            if is_gtk {
                "view-more"
            } else {
                "overflow-menu"
            }
        } // close: no vertical variant in KDE
        IconName::Eye => {
            if is_gtk {
                "view-reveal"
            } else {
                "view-visible"
            }
        } // exact
        IconName::EyeOff => {
            if is_gtk {
                "view-conceal"
            } else {
                "view-hidden"
            }
        } // exact
        IconName::Frame => {
            if is_gtk {
                "selection-mode"
            } else {
                "select-rectangular"
            }
        } // close
        IconName::Heart => {
            if is_gtk {
                "starred"
            } else {
                "emblem-favorite"
            }
        } // close
        IconName::Loader => {
            if is_gtk {
                "content-loading"
            } else {
                "process-working"
            }
        } // exact
        IconName::LoaderCircle => {
            if is_gtk {
                "content-loading"
            } else {
                "process-working"
            }
        } // exact
        IconName::Palette => {
            if is_gtk {
                "color-select"
            } else {
                "palette"
            }
        } // close
        IconName::PanelLeft => {
            if is_gtk {
                "sidebar-show"
            } else {
                "sidebar-expand-left"
            }
        } // close
        IconName::PanelLeftClose => {
            if is_gtk {
                "sidebar-show"
            } else {
                "view-left-close"
            }
        } // close
        IconName::PanelLeftOpen => {
            if is_gtk {
                "sidebar-show"
            } else {
                "view-left-new"
            }
        } // close
        IconName::PanelRight => {
            if is_gtk {
                "sidebar-show-right"
            } else {
                "view-right-new"
            }
        } // close
        IconName::PanelRightClose => {
            if is_gtk {
                "sidebar-show-right"
            } else {
                "view-right-close"
            }
        } // close
        IconName::PanelRightOpen => {
            if is_gtk {
                "sidebar-show-right"
            } else {
                "view-right-new"
            }
        } // close
        IconName::ResizeCorner => {
            if is_gtk {
                "list-drag-handle"
            } else {
                "drag-handle"
            }
        } // close
        IconName::Settings2 => {
            if is_gtk {
                "preferences-other"
            } else {
                "configure"
            }
        } // close

        // --- Icons where GNOME uses a different (approximate) alternative ---
        IconName::ALargeSmall => {
            if is_gtk {
                "zoom-in"
            } else {
                "format-font-size-more"
            }
        } // approximate
        IconName::Asterisk => {
            if is_gtk {
                "starred"
            } else {
                "rating"
            }
        } // approximate
        IconName::Bell => {
            if is_gtk {
                "alarm"
            } else {
                "notification-active"
            }
        } // close
        IconName::Building2 => {
            if is_gtk {
                "network-workgroup"
            } else {
                "applications-office"
            }
        } // approximate
        IconName::CaseSensitive => {
            if is_gtk {
                "format-text-rich"
            } else {
                "format-text-uppercase"
            }
        } // approximate
        IconName::ChartPie => {
            if is_gtk {
                "x-office-spreadsheet"
            } else {
                "office-chart-pie"
            }
        } // approximate
        IconName::ChevronsUpDown => {
            if is_gtk {
                "list-drag-handle"
            } else {
                "handle-sort"
            }
        } // close
        IconName::ExternalLink => {
            if is_gtk {
                "insert-link"
            } else {
                "external-link"
            }
        } // close
        IconName::GalleryVerticalEnd => {
            if is_gtk {
                "view-paged"
            } else {
                "view-list-icons"
            }
        } // approximate
        IconName::GitHub => {
            if is_gtk {
                "applications-engineering"
            } else {
                "vcs-branch"
            }
        } // approximate
        IconName::Globe => {
            if is_gtk {
                "web-browser"
            } else {
                "globe"
            }
        } // close
        IconName::Inbox => {
            if is_gtk {
                "mail-send-receive"
            } else {
                "mail-folder-inbox"
            }
        } // close
        IconName::Inspector => {
            if is_gtk {
                "preferences-system-details"
            } else {
                "code-context"
            }
        } // approximate
        IconName::PanelBottom => {
            if is_gtk {
                "view-dual"
            } else {
                "view-split-top-bottom"
            }
        } // close
        IconName::PanelBottomOpen => {
            if is_gtk {
                "view-dual"
            } else {
                "view-split-top-bottom"
            }
        } // close
        IconName::ThumbsDown => {
            if is_gtk {
                "process-stop"
            } else {
                "rating-unrated"
            }
        } // approximate
        IconName::ThumbsUp => {
            if is_gtk {
                "checkbox-checked"
            } else {
                "approved"
            }
        } // approximate
    }
}

/// Default rasterization size for SVG icons.
///
/// SVGs are rasterized at 2x the typical display size (24px) to look sharp
/// on HiDPI screens. gpui uses the same 2x scale factor internally.
const SVG_RASTERIZE_SIZE: u32 = 48;

/// Convert [`IconData`] to a gpui [`ImageSource`] for rendering.
///
/// Returns `None` if the icon data cannot be converted (corrupt SVG,
/// unknown variant).
///
/// # Parameters
///
/// - `color`: If `Some`, colorizes monochrome SVGs with the given color
///   (replaces `currentColor`, explicit black fills, or injects a fill
///   attribute). Best for bundled icon sets (Material, Lucide). Pass `None`
///   for system/OS icons to preserve their native palette.
///   RGBA icons are passed through unchanged regardless of this parameter.
/// - `size`: Rasterize size in pixels for SVG icons. `None` defaults to 48px
///   (2x HiDPI at 24px logical). Pass `logical_size * scale_factor` for
///   DPI-correct rendering.
///
/// # Examples
///
/// ```ignore
/// use native_theme::IconData;
/// use native_theme_gpui::icons::to_image_source;
///
/// let svg = IconData::Svg(b"<svg></svg>".to_vec());
/// let source = to_image_source(&svg, None, None);        // uncolorized, 48px
/// let colored = to_image_source(&svg, Some(color), None); // colorized, 48px
/// let sized = to_image_source(&svg, None, Some(96));      // uncolorized, 96px
/// ```
#[must_use]
pub fn to_image_source(
    data: &IconData,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE);
    match data {
        IconData::Svg(bytes) => {
            if let Some(c) = color {
                let colored = colorize_svg(bytes, c);
                svg_to_bmp_source(&colored, raster_size)
            } else {
                svg_to_bmp_source(bytes, raster_size)
            }
        }
        IconData::Rgba {
            width,
            height,
            data,
        } => {
            let bmp = encode_rgba_as_bmp(*width, *height, data);
            let image = Image::from_bytes(ImageFormat::Bmp, bmp);
            Some(ImageSource::Image(Arc::new(image)))
        }
        _ => None,
    }
}

/// Convert [`IconData`] to a gpui [`ImageSource`], consuming the data.
///
/// This is the consuming variant of [`to_image_source()`]. It takes ownership
/// of the `IconData` to avoid cloning the underlying `Vec<u8>`. Prefer this
/// when you already own the data and won't use it again.
///
/// Returns `None` if the icon data cannot be converted (corrupt SVG,
/// unknown variant).
///
/// See [`to_image_source()`] for details on the `color` and `size` parameters.
#[must_use]
pub fn into_image_source(
    data: IconData,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE);
    match data {
        IconData::Svg(bytes) => {
            if let Some(c) = color {
                let colored = colorize_svg(&bytes, c);
                svg_to_bmp_source(&colored, raster_size)
            } else {
                svg_to_bmp_source(&bytes, raster_size)
            }
        }
        IconData::Rgba {
            width,
            height,
            data,
        } => {
            let bmp = encode_rgba_as_bmp(width, height, &data);
            let image = Image::from_bytes(ImageFormat::Bmp, bmp);
            Some(ImageSource::Image(Arc::new(image)))
        }
        _ => None,
    }
}

/// Load a custom icon from an [`IconProvider`] and convert to a gpui [`ImageSource`].
///
/// Equivalent to calling [`load_custom_icon()`](native_theme::load_custom_icon)
/// followed by [`to_image_source()`], composing the loading and conversion steps.
///
/// Returns `None` if the provider has no icon for the given set or if
/// conversion fails.
///
/// See [`to_image_source()`] for details on the `color` and `size` parameters.
#[must_use]
pub fn custom_icon_to_image_source(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::IconSet,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    let data = load_custom_icon(provider, icon_set)?;
    to_image_source(&data, color, size)
}

/// Convert all frames of an [`AnimatedIcon::Frames`] to gpui [`ImageSource`]s.
///
/// Returns `Some(Vec<ImageSource>)` when the icon is the `Frames` variant,
/// with one `ImageSource` per frame. Returns `None` for `Transform` variants.
///
/// **Call this once and cache the result.** Do not call on every frame tick --
/// SVG rasterization is expensive. Index into the cached `Vec` using a
/// timer-driven frame counter.
///
/// Callers should check [`native_theme::prefers_reduced_motion()`] and fall
/// back to [`AnimatedIcon::first_frame()`] for a static display when the user
/// has requested reduced motion.
///
/// # Examples
///
/// ```ignore
/// use native_theme_gpui::icons::{animated_frames_to_image_sources, AnimatedImageSources};
///
/// let anim = native_theme::loading_indicator();
/// if let Some(AnimatedImageSources { sources, frame_duration_ms }) =
///     animated_frames_to_image_sources(&anim)
/// {
///     // Cache `sources`, then on each timer tick (every `frame_duration_ms` ms):
///     // frame_index = (frame_index + 1) % sources.len();
///     // gpui::img(sources[frame_index].clone())
/// }
/// ```
#[must_use]
pub fn animated_frames_to_image_sources(anim: &AnimatedIcon) -> Option<AnimatedImageSources> {
    match anim {
        AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
        } => {
            let sources: Vec<ImageSource> = frames
                .iter()
                .filter_map(|f| to_image_source(f, None, None))
                .collect();
            if sources.is_empty() {
                None
            } else {
                Some(AnimatedImageSources {
                    sources,
                    frame_duration_ms: *frame_duration_ms,
                })
            }
        }
        _ => None,
    }
}

/// Wrap a gpui [`Svg`] element with continuous rotation animation.
///
/// Returns an animated element that spins 360 degrees over `duration_ms`
/// milliseconds, repeating infinitely. Uses linear easing for constant-speed
/// rotation suitable for loading spinners.
///
/// `duration_ms` comes from [`native_theme::TransformAnimation::Spin`].
/// `animation_id` must be unique among sibling animated elements (accepts
/// `&'static str`, integer IDs, or any `impl Into<ElementId>`).
///
/// This is pure data construction -- no gpui render context is needed to call
/// this function. Only `paint()` on the resulting element requires a window.
///
/// Callers should check [`native_theme::prefers_reduced_motion()`] and fall
/// back to a static icon when the user has requested reduced motion.
///
/// # Examples
///
/// ```ignore
/// use native_theme_gpui::icons::with_spin_animation;
///
/// let spinner = gpui::svg().path("spinner.svg").size_6();
/// let animated = with_spin_animation(spinner, "my-spinner", 1000);
/// // Use `animated` as a child element in your gpui view
/// ```
#[must_use]
pub fn with_spin_animation(
    element: Svg,
    animation_id: impl Into<gpui::ElementId>,
    duration_ms: u32,
) -> impl gpui::IntoElement {
    element.with_animation(
        animation_id,
        Animation::new(Duration::from_millis(duration_ms as u64)).repeat(),
        |el, delta| el.with_transformation(Transformation::rotate(percentage(delta))),
    )
}

/// Rasterize SVG bytes and return as a BMP-backed [`ImageSource`].
///
/// Returns `None` if rasterization fails (corrupt SVG, empty data).
///
/// Works around a gpui bug where `ImageFormat::Svg` in `Image::to_image_data`
/// skips the RGBA→BGRA pixel conversion that all other formats perform,
/// causing red and blue channels to be swapped.
fn svg_to_bmp_source(svg_bytes: &[u8], size: u32) -> Option<ImageSource> {
    let Ok(IconData::Rgba {
        width,
        height,
        data,
    }) = native_theme::rasterize::rasterize_svg(svg_bytes, size)
    else {
        return None;
    };
    let bmp = encode_rgba_as_bmp(width, height, &data);
    let image = Image::from_bytes(ImageFormat::Bmp, bmp);
    Some(ImageSource::Image(Arc::new(image)))
}

/// Rewrite SVG bytes to use the given color for strokes and fills.
///
/// Handles three SVG color patterns (in order):
/// 1. **`currentColor`** — replaced with the hex color (Lucide-style SVGs).
/// 2. **Explicit black fills** — `fill="black"`, `fill="#000000"`, `fill="#000"`
///    are replaced with the hex color (third-party SVGs with hardcoded black).
/// 3. **Implicit black** — if the root `<svg>` tag has no `fill=` attribute,
///    injects `fill="<hex>"` (Material-style SVGs).
///
/// Not handled: `stroke="black"`, CSS `style="fill:black"`, `fill="rgb(0,0,0)"`,
/// or explicit black on child elements when the root tag has a different fill.
/// This function is designed for monochrome icon sets; multi-color SVGs should
/// not be colorized.
fn colorize_svg(svg_bytes: &[u8], color: Hsla) -> Vec<u8> {
    let rgba: gpui::Rgba = color.into();
    let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

    let svg_str = String::from_utf8_lossy(svg_bytes);

    // 1. Replace currentColor (handles Lucide-style SVGs)
    if svg_str.contains("currentColor") {
        return svg_str.replace("currentColor", &hex).into_bytes();
    }

    // 2. Replace explicit black fills (handles third-party SVGs)
    let fill_hex = format!("fill=\"{}\"", hex);
    let replaced = svg_str
        .replace("fill=\"black\"", &fill_hex)
        .replace("fill=\"#000000\"", &fill_hex)
        .replace("fill=\"#000\"", &fill_hex);
    if replaced != svg_str {
        return replaced.into_bytes();
    }

    // 3. No currentColor or explicit black — inject fill into root <svg> tag
    // (handles Material-style SVGs with implicit black fill)
    if let Some(pos) = svg_str.find("<svg")
        && let Some(close) = svg_str[pos..].find('>')
    {
        let tag_end = pos + close;
        let tag = &svg_str[pos..tag_end];
        if !tag.contains("fill=") {
            // Handle self-closing tags: inject before '/' in '<svg .../>'
            let inject_pos = if tag_end > 0 && svg_str.as_bytes()[tag_end - 1] == b'/' {
                tag_end - 1
            } else {
                tag_end
            };
            let mut result = String::with_capacity(svg_str.len() + 20);
            result.push_str(&svg_str[..inject_pos]);
            result.push_str(&format!(" fill=\"{}\"", hex));
            result.push_str(&svg_str[inject_pos..]);
            return result.into_bytes();
        }
    }

    // SVG already has non-black fill and no currentColor — return as-is
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
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    pub(super) const ALL_ICON_NAMES: &[IconName] = &[
        IconName::ALargeSmall,
        IconName::ArrowDown,
        IconName::ArrowLeft,
        IconName::ArrowRight,
        IconName::ArrowUp,
        IconName::Asterisk,
        IconName::Bell,
        IconName::BookOpen,
        IconName::Bot,
        IconName::Building2,
        IconName::Calendar,
        IconName::CaseSensitive,
        IconName::ChartPie,
        IconName::Check,
        IconName::ChevronDown,
        IconName::ChevronLeft,
        IconName::ChevronRight,
        IconName::ChevronsUpDown,
        IconName::ChevronUp,
        IconName::CircleCheck,
        IconName::CircleUser,
        IconName::CircleX,
        IconName::Close,
        IconName::Copy,
        IconName::Dash,
        IconName::Delete,
        IconName::Ellipsis,
        IconName::EllipsisVertical,
        IconName::ExternalLink,
        IconName::Eye,
        IconName::EyeOff,
        IconName::File,
        IconName::Folder,
        IconName::FolderClosed,
        IconName::FolderOpen,
        IconName::Frame,
        IconName::GalleryVerticalEnd,
        IconName::GitHub,
        IconName::Globe,
        IconName::Heart,
        IconName::HeartOff,
        IconName::Inbox,
        IconName::Info,
        IconName::Inspector,
        IconName::LayoutDashboard,
        IconName::Loader,
        IconName::LoaderCircle,
        IconName::Map,
        IconName::Maximize,
        IconName::Menu,
        IconName::Minimize,
        IconName::Minus,
        IconName::Moon,
        IconName::Palette,
        IconName::PanelBottom,
        IconName::PanelBottomOpen,
        IconName::PanelLeft,
        IconName::PanelLeftClose,
        IconName::PanelLeftOpen,
        IconName::PanelRight,
        IconName::PanelRightClose,
        IconName::PanelRightOpen,
        IconName::Plus,
        IconName::Redo,
        IconName::Redo2,
        IconName::Replace,
        IconName::ResizeCorner,
        IconName::Search,
        IconName::Settings,
        IconName::Settings2,
        IconName::SortAscending,
        IconName::SortDescending,
        IconName::SquareTerminal,
        IconName::Star,
        IconName::StarOff,
        IconName::Sun,
        IconName::ThumbsDown,
        IconName::ThumbsUp,
        IconName::TriangleAlert,
        IconName::Undo,
        IconName::Undo2,
        IconName::User,
        IconName::WindowClose,
        IconName::WindowMaximize,
        IconName::WindowMinimize,
        IconName::WindowRestore,
    ];

    #[test]
    fn all_icons_have_lucide_mapping() {
        for icon in ALL_ICON_NAMES {
            let name = lucide_name_for_gpui_icon(icon.clone());
            assert!(
                !name.is_empty(),
                "Empty Lucide mapping for an IconName variant",
            );
        }
    }

    #[test]
    fn all_icons_have_material_mapping() {
        for icon in ALL_ICON_NAMES {
            let name = material_name_for_gpui_icon(icon.clone());
            assert!(
                !name.is_empty(),
                "Empty Material mapping for an IconName variant",
            );
        }
    }

    // --- icon_name tests ---

    #[test]
    fn icon_name_dialog_warning_maps_to_triangle_alert() {
        assert!(matches!(
            icon_name(IconRole::DialogWarning),
            Some(IconName::TriangleAlert)
        ));
    }

    #[test]
    fn icon_name_dialog_error_maps_to_circle_x() {
        assert!(matches!(
            icon_name(IconRole::DialogError),
            Some(IconName::CircleX)
        ));
    }

    #[test]
    fn icon_name_dialog_info_maps_to_info() {
        assert!(matches!(
            icon_name(IconRole::DialogInfo),
            Some(IconName::Info)
        ));
    }

    #[test]
    fn icon_name_dialog_success_maps_to_circle_check() {
        assert!(matches!(
            icon_name(IconRole::DialogSuccess),
            Some(IconName::CircleCheck)
        ));
    }

    #[test]
    fn icon_name_window_close_maps() {
        assert!(matches!(
            icon_name(IconRole::WindowClose),
            Some(IconName::WindowClose)
        ));
    }

    #[test]
    fn icon_name_action_copy_maps_to_copy() {
        assert!(matches!(
            icon_name(IconRole::ActionCopy),
            Some(IconName::Copy)
        ));
    }

    #[test]
    fn icon_name_nav_back_maps_to_chevron_left() {
        assert!(matches!(
            icon_name(IconRole::NavBack),
            Some(IconName::ChevronLeft)
        ));
    }

    #[test]
    fn icon_name_file_generic_maps_to_file() {
        assert!(matches!(
            icon_name(IconRole::FileGeneric),
            Some(IconName::File)
        ));
    }

    #[test]
    fn icon_name_status_check_maps_to_check() {
        assert!(matches!(
            icon_name(IconRole::StatusCheck),
            Some(IconName::Check)
        ));
    }

    #[test]
    fn icon_name_user_account_maps_to_user() {
        assert!(matches!(
            icon_name(IconRole::UserAccount),
            Some(IconName::User)
        ));
    }

    #[test]
    fn icon_name_notification_maps_to_bell() {
        assert!(matches!(
            icon_name(IconRole::Notification),
            Some(IconName::Bell)
        ));
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
        assert_eq!(
            some_count, 30,
            "Expected exactly 30 mappings, got {some_count}"
        );
    }

    // --- to_image_source tests ---

    #[test]
    fn to_image_source_svg_returns_bmp_rasterized() {
        // Valid SVG that resvg can parse
        let svg = IconData::Svg(
            b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>".to_vec(),
        );
        let source = to_image_source(&svg, None, None).expect("valid SVG should convert");
        // SVGs are rasterized to BMP to work around gpui's RGBA/BGRA bug
        match source {
            ImageSource::Image(arc) => {
                assert_eq!(arc.format, ImageFormat::Bmp);
                assert!(arc.bytes.starts_with(b"BM"), "BMP should start with 'BM'");
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
        let source = to_image_source(&rgba, None, None).expect("RGBA should convert");
        match source {
            ImageSource::Image(arc) => {
                assert_eq!(arc.format, ImageFormat::Bmp);
                // BMP header starts with "BM"
                assert_eq!(&arc.bytes[0..2], b"BM");
            }
            _ => panic!("Expected ImageSource::Image for RGBA data"),
        }
    }

    #[test]
    fn to_image_source_with_color() {
        let svg = IconData::Svg(
            b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M0 0' stroke='currentColor'/></svg>".to_vec(),
        );
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = to_image_source(&svg, Some(color), None);
        assert!(result.is_some(), "colorized SVG should convert");
    }

    #[test]
    fn to_image_source_with_custom_size() {
        let svg = IconData::Svg(
            b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>".to_vec(),
        );
        let result = to_image_source(&svg, None, Some(32));
        assert!(result.is_some(), "custom size SVG should convert");
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
    // --- colorize_svg tests ---

    #[test]
    fn colorize_svg_replaces_fill_black() {
        let svg = b"<svg><path fill=\"black\" d=\"M0 0h24v24H0z\"/></svg>";
        let color = gpui::hsla(0.6, 0.7, 0.5, 1.0); // a blue-ish color
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("fill=\"black\""),
            "fill=\"black\" should be replaced, got: {}",
            result_str
        );
        assert!(
            result_str.contains("fill=\"#"),
            "should contain hex fill, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_svg_replaces_fill_hex_black() {
        let svg = b"<svg><rect fill=\"#000000\" width=\"24\" height=\"24\"/></svg>";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0); // red
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("#000000"),
            "fill=\"#000000\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_svg_replaces_fill_short_hex_black() {
        let svg = b"<svg><rect fill=\"#000\" width=\"24\" height=\"24\"/></svg>";
        let color = gpui::hsla(0.3, 0.8, 0.4, 1.0); // green
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("fill=\"#000\""),
            "fill=\"#000\" should be replaced, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_svg_current_color_still_works() {
        let svg = b"<svg><path stroke=\"currentColor\" d=\"M0 0\"/></svg>";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("currentColor"),
            "currentColor should be replaced"
        );
        assert!(result_str.contains('#'), "should contain hex color");
    }

    #[test]
    fn colorize_svg_implicit_black_still_works() {
        // SVG with no fill attribute at all (Material-style)
        let svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("fill=\"#"),
            "should inject fill into root svg tag, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_self_closing_svg_produces_valid_xml() {
        // Self-closing <svg .../> tag — fill must be injected before '/'
        let svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\" />";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("fill=\"#"),
            "should inject fill, got: {}",
            result_str
        );
        // Must NOT produce '/ fill=' (broken XML)
        assert!(
            !result_str.contains("/ fill="),
            "fill must be before '/', got: {}",
            result_str
        );
        // Must end with '/>' (valid self-closing)
        assert!(
            result_str.trim().ends_with("/>"),
            "should remain self-closing, got: {}",
            result_str
        );
    }

    // --- into_image_source tests ---

    #[test]
    fn into_image_source_svg_returns_some() {
        let svg = IconData::Svg(
            b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>".to_vec(),
        );
        let result = into_image_source(svg, None, None);
        assert!(result.is_some(), "valid SVG should convert");
    }

    #[test]
    fn into_image_source_rgba_returns_some() {
        let rgba = IconData::Rgba {
            width: 2,
            height: 2,
            data: vec![
                255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
            ],
        };
        let result = into_image_source(rgba, None, None);
        assert!(result.is_some(), "RGBA should convert");
    }

    #[test]
    fn into_image_source_with_color() {
        let svg = IconData::Svg(
            b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M0 0' stroke='currentColor'/></svg>".to_vec(),
        );
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = into_image_source(svg, Some(color), None);
        assert!(result.is_some(), "colorized SVG should convert");
    }

    // --- custom_icon tests ---

    // Test helper: minimal IconProvider that returns a bundled SVG
    #[derive(Debug)]
    struct TestCustomIcon;

    impl native_theme::IconProvider for TestCustomIcon {
        fn icon_name(&self, _set: native_theme::IconSet) -> Option<&str> {
            None // No system name -- forces bundled SVG path
        }
        fn icon_svg(&self, _set: native_theme::IconSet) -> Option<&'static [u8]> {
            Some(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10'/></svg>")
        }
    }

    // Provider with no mappings at all
    #[derive(Debug)]
    struct EmptyProvider;

    impl native_theme::IconProvider for EmptyProvider {
        fn icon_name(&self, _set: native_theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(&self, _set: native_theme::IconSet) -> Option<&'static [u8]> {
            None
        }
    }

    #[test]
    fn custom_icon_to_image_source_with_svg_provider_returns_some() {
        let result = custom_icon_to_image_source(
            &TestCustomIcon,
            native_theme::IconSet::Material,
            None,
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_source_with_empty_provider_returns_none() {
        let result = custom_icon_to_image_source(
            &EmptyProvider,
            native_theme::IconSet::Material,
            None,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_image_source_with_color() {
        let color = Hsla {
            h: 0.0,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };
        let result = custom_icon_to_image_source(
            &TestCustomIcon,
            native_theme::IconSet::Material,
            Some(color),
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_source_accepts_dyn_provider() {
        let boxed: Box<dyn native_theme::IconProvider> = Box::new(TestCustomIcon);
        let result =
            custom_icon_to_image_source(&*boxed, native_theme::IconSet::Material, None, None);
        assert!(result.is_some());
    }

    // --- animated icon tests ---

    #[test]
    fn animated_frames_returns_sources() {
        let anim = AnimatedIcon::Frames {
            frames: vec![
                IconData::Svg(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>".to_vec()),
                IconData::Svg(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='8' fill='blue'/></svg>".to_vec()),
                IconData::Svg(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='6' fill='green'/></svg>".to_vec()),
            ],
            frame_duration_ms: 80,
        };
        let result = animated_frames_to_image_sources(&anim);
        let ais = result.expect("Frames variant should return Some");
        assert_eq!(ais.sources.len(), 3);
        assert_eq!(ais.frame_duration_ms, 80);
    }

    #[test]
    fn animated_frames_transform_returns_none() {
        let anim = AnimatedIcon::Transform {
            icon: IconData::Svg(
                b"<svg xmlns='http://www.w3.org/2000/svg'><circle cx='12' cy='12' r='10'/></svg>"
                    .to_vec(),
            ),
            animation: native_theme::TransformAnimation::Spin { duration_ms: 1000 },
        };
        let result = animated_frames_to_image_sources(&anim);
        assert!(result.is_none());
    }

    #[test]
    fn animated_frames_empty_returns_none() {
        let anim = AnimatedIcon::Frames {
            frames: vec![],
            frame_duration_ms: 80,
        };
        let result = animated_frames_to_image_sources(&anim);
        assert!(result.is_none());
    }

    #[test]
    fn spin_animation_constructs_without_context() {
        let svg_element = gpui::svg();
        // with_spin_animation wraps an Svg element with continuous rotation.
        // This is pure construction -- no gpui render context needed.
        let _animated = with_spin_animation(svg_element, "test-spin", 1000);
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod freedesktop_mapping_tests {
    use super::tests::ALL_ICON_NAMES;
    use super::*;
    use native_theme::LinuxDesktop;

    #[test]
    fn all_86_gpui_icons_have_mapping_on_kde() {
        for name in ALL_ICON_NAMES {
            let fd_name = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Kde);
            assert!(
                !fd_name.is_empty(),
                "Empty KDE freedesktop mapping for an IconName variant",
            );
        }
    }

    #[test]
    fn eye_differs_by_de() {
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Eye, LinuxDesktop::Kde),
            "view-visible",
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Eye, LinuxDesktop::Gnome),
            "view-reveal",
        );
    }

    #[test]
    fn freedesktop_standard_ignores_de() {
        // edit-copy is freedesktop standard — same for all DEs
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Copy, LinuxDesktop::Kde),
            freedesktop_name_for_gpui_icon(IconName::Copy, LinuxDesktop::Gnome),
        );
    }

    #[test]
    fn all_86_gpui_icons_have_mapping_on_gnome() {
        for name in ALL_ICON_NAMES {
            let fd_name = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Gnome);
            assert!(
                !fd_name.is_empty(),
                "Empty GNOME freedesktop mapping for an IconName variant",
            );
        }
    }

    #[test]
    fn xfce_uses_gnome_names() {
        // XFCE is GTK-based and should use GNOME naming convention
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Eye, LinuxDesktop::Xfce),
            "view-reveal",
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Bell, LinuxDesktop::Xfce),
            "alarm",
        );
    }

    #[test]
    fn all_kde_names_resolve_in_breeze() {
        let theme = native_theme::system_icon_theme();
        // Only meaningful on a KDE system with Breeze installed
        if !theme.to_lowercase().contains("breeze") {
            eprintln!("Skipping: system theme is '{}', not Breeze", theme);
            return;
        }

        let mut missing = Vec::new();
        for name in ALL_ICON_NAMES {
            let fd_name = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Kde);
            if native_theme::load_freedesktop_icon_by_name(fd_name, theme, 24).is_none() {
                missing.push(format!("{} (not found)", fd_name));
            }
        }
        assert!(
            missing.is_empty(),
            "These gpui icons did not resolve in Breeze:\n  {}",
            missing.join("\n  "),
        );
    }

    #[test]
    fn gnome_names_resolve_in_adwaita() {
        // Verify GNOME mappings resolve against installed Adwaita theme.
        // Only runs when Adwaita is installed (it usually is on any Linux).
        let mut missing = Vec::new();
        for name in ALL_ICON_NAMES {
            let fd_name = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Gnome);
            if native_theme::load_freedesktop_icon_by_name(fd_name, "Adwaita", 24).is_none() {
                missing.push(format!("{} (not found)", fd_name));
            }
        }
        assert!(
            missing.is_empty(),
            "These GNOME mappings did not resolve in Adwaita:\n  {}",
            missing.join("\n  "),
        );
    }
}
