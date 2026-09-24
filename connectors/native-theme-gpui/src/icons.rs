//! Icon conversion functions for the gpui connector.
//!
//! # Function Overview
//!
//! | Function | Purpose |
//! |----------|---------|
//! | [`icon_name`] | Map [`IconRole`] → [`IconName`] (Lucide, zero-I/O) |
//! | [`lucide_name_for_gpui_icon`] | Map [`IconName`] → Lucide name (`Option<&str>`) |
//! | [`material_name_for_gpui_icon`] | Map [`IconName`] → Material name (`Option<&str>`) |
//! | [`freedesktop_name_for_gpui_icon`] | Map [`IconName`] → freedesktop name (`Option<&str>`, Linux only) |
//! | [`to_image_source`] | Convert [`IconData`] → [`ImageSource`] with optional color/size |
//! | [`into_image_source`] | Consuming variant of [`to_image_source`] (avoids clone) |
//! | [`custom_icon_to_image_source`] | Load + convert via [`IconProvider`] |
//! | [`bundled_icon_to_image_source`] | Convert [`IconName`] + [`native_theme::theme::IconSet`] → [`ImageSource`] in one call |
//! | [`animated_frames_to_image_sources`] | Convert animation frames → [`AnimatedImageSources`] |
//! | [`with_spin_animation`] | Wrap an SVG element with spin animation |
//!
//! Every [`ImageSource`] this module builds is an `ImageSource::Render`: an
//! image gpui draws from as it stands, in the frame that asks for it. The
//! alternative, `ImageSource::Image`, hands gpui encoded bytes and is decoded
//! in the background, so an element holding one paints nothing the first time
//! it comes up (gpui-pre `src/elements/img.rs:534-553`) -- an animation
//! flickers its way through its first pass, one blank frame at a time. A
//! [`gpui::RenderImage`] the caller keeps holds a tile in the window's sprite
//! atlas until it is handed to `App::drop_image`; see
//! [`to_image_source`] for what that asks of an application that rebuilds its
//! icons.
//!
//! That holds with the `svg-rasterize` feature (on by default), which
//! rasterizes SVG icons here. Without it, an SVG icon becomes an
//! `ImageSource::Image` holding the (colorized) SVG bytes, which gpui decodes
//! itself: it paints nothing the first time it comes up, and gpui chooses the
//! raster size. RGBA icons are decoded images either way.

use gpui::{
    Animation, AnimationExt, Hsla, ImageSource, RenderImage, Svg, Transformation, percentage,
};
use gpui_component::IconName;
#[cfg(all(test, target_os = "linux", feature = "system-icons"))]
use native_theme::icons::FreedesktopLoader;
use native_theme::icons::load_icon;
use native_theme::theme::{AnimatedIcon, IconData, IconProvider, IconRole};
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
/// corresponding Lucide icon in gpui-component 0.6.
///
/// # Examples
///
/// ```ignore
/// use native_theme::theme::IconRole;
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
        // Issue 33: both ActionDelete and TrashEmpty map to IconName::Delete.
        // gpui-component's Delete icon is a backspace/erase glyph, not a
        // trash can. This is the closest available match for both roles.
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
        // Issue 46: StatusError maps to the same CircleX as DialogError.
        // Both roles use the same visual glyph; the semantic distinction
        // (inline status vs dialog header) is handled by size/placement.
        IconRole::StatusError => IconName::CircleX,

        // System
        IconRole::UserAccount => IconName::User,
        IconRole::Notification => IconName::Bell,

        // No Lucide equivalent in gpui-component 0.6
        _ => return None,
    })
}

/// Map a gpui-component [`IconName`] to its canonical Lucide icon name.
///
/// Returns the kebab-case Lucide name for use with
/// [`native_theme::icons::LucideLoader::new`].
///
/// Covers all 101 gpui-component 0.6.6 `IconName` variants. Returns `None`
/// where Lucide has no equivalent (today only `StarFill`, spec §10.2); every
/// `Some` is Lucide's own file name (`LucideLoader::new(name)` resolves it).
#[must_use]
pub fn lucide_name_for_gpui_icon(icon: IconName) -> Option<&'static str> {
    Some(match icon {
        IconName::ALargeSmall => "a-large-small",
        IconName::ArrowDown => "arrow-down",
        IconName::ArrowLeft => "arrow-left",
        IconName::ArrowRight => "arrow-right",
        IconName::ArrowUp => "arrow-up",
        IconName::Asterisk => "asterisk",
        IconName::Battery => "battery",
        IconName::BatteryCharging => "battery-charging",
        IconName::BatteryFull => "battery-full",
        IconName::BatteryLow => "battery-low",
        IconName::BatteryMedium => "battery-medium",
        IconName::BatteryWarning => "battery-warning",
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
        IconName::Close => "x",
        IconName::Copy => "copy",
        IconName::Cpu => "cpu",
        IconName::Dash => "minus",
        IconName::Delete => "delete",
        IconName::Ellipsis => "ellipsis",
        IconName::EllipsisVertical => "ellipsis-vertical",
        IconName::ExternalLink => "external-link",
        IconName::Eye => "eye",
        IconName::EyeOff => "eye-off",
        IconName::File => "file",
        IconName::FileText => "file-text",
        IconName::Folder => "folder",
        IconName::FolderClosed => "folder-closed",
        IconName::FolderOpen => "folder-open",
        IconName::Frame => "frame",
        IconName::GalleryVerticalEnd => "gallery-vertical-end",
        IconName::Github => "github",
        IconName::Globe => "globe",
        IconName::HardDrive => "hard-drive",
        IconName::Heart => "heart",
        IconName::HeartOff => "heart-off",
        IconName::Inbox => "inbox",
        IconName::Info => "info",
        IconName::Inspector => "scan",
        IconName::LayoutDashboard => "layout-dashboard",
        IconName::Loader => "loader",
        IconName::LoaderCircle => "loader-circle",
        IconName::Map => "map",
        IconName::Maximize => "maximize",
        IconName::MemoryStick => "memory-stick",
        IconName::Menu => "menu",
        IconName::Minimize => "minimize",
        IconName::Minus => "minus",
        IconName::Moon => "moon",
        IconName::Network => "network",
        IconName::Palette => "palette",
        IconName::PanelBottom => "panel-bottom",
        IconName::PanelBottomOpen => "panel-bottom-open",
        IconName::PanelLeft => "panel-left",
        IconName::PanelLeftClose => "panel-left-close",
        IconName::PanelLeftOpen => "panel-left-open",
        IconName::PanelRight => "panel-right",
        IconName::PanelRightClose => "panel-right-close",
        IconName::PanelRightOpen => "panel-right-open",
        IconName::Pause => "pause",
        IconName::Play => "play",
        IconName::Plus => "plus",
        IconName::Redo => "redo",
        IconName::Redo2 => "redo-2",
        IconName::Replace => "replace",
        IconName::ResizeCorner => "grip",
        IconName::RotateCw => "rotate-cw",
        IconName::Search => "search",
        IconName::Settings => "settings",
        IconName::Settings2 => "settings-2",
        IconName::SortAscending => "arrow-up-narrow-wide",
        IconName::SortDescending => "arrow-down-wide-narrow",
        IconName::SquareTerminal => "square-terminal",
        IconName::Star => "star",
        // Lucide ships no filled star (star-fill / star-filled absent at 1.41.0);
        // gpui-kit's star-fill.svg is Lucide's `star` with fill="currentColor"
        // added, which a bundled Lucide file cannot express, and the hollow
        // `star` would make Star and StarFill indistinguishable (spec §10.2).
        IconName::StarFill => return None,
        IconName::StarOff => "star-off",
        IconName::Sun => "sun",
        IconName::ThumbsDown => "thumbs-down",
        IconName::ThumbsUp => "thumbs-up",
        IconName::TriangleAlert => "triangle-alert",
        IconName::Undo => "undo",
        IconName::Undo2 => "undo-2",
        IconName::User => "user",
        IconName::WindowClose => "x",
        IconName::WindowMaximize => "maximize",
        IconName::WindowMinimize => "minus",
        IconName::WindowRestore => "minimize-2",
    })
}

/// Map a gpui-component [`IconName`] to its canonical Material icon name.
///
/// Returns the snake_case Material Symbols name for use with
/// [`native_theme::icons::MaterialLoader`].
///
/// Covers all 101 gpui-component 0.6.6 `IconName` variants. Returns `None`
/// where Material Symbols has no equivalent (today only `StarOff`); every
/// `Some` is a bundled Material Symbols Outlined 24px file.
///
/// Material icon name collisions (multiple IconName variants -> same name):
/// - ArrowUp, SortAscending -> "arrow_upward"
/// - ArrowDown, SortDescending -> "arrow_downward"
/// - Close, WindowClose -> "close"
/// - Dash, Minus -> "remove"
/// - File, FileText -> "description"
/// - Folder, FolderClosed -> "folder"
/// - Maximize, WindowMaximize -> "open_in_full"
/// - Minimize, WindowMinimize -> "minimize"
/// - PanelRight, PanelRightClose -> "right_panel_close"
/// - Redo, Redo2 -> "redo"
/// - Undo, Undo2 -> "undo"
#[must_use]
pub fn material_name_for_gpui_icon(icon: IconName) -> Option<&'static str> {
    Some(match icon {
        IconName::ALargeSmall => "format_size",
        IconName::ArrowDown => "arrow_downward",
        IconName::ArrowLeft => "arrow_back",
        IconName::ArrowRight => "arrow_forward",
        IconName::ArrowUp => "arrow_upward",
        IconName::Asterisk => "emergency",
        IconName::Battery => "battery_0_bar", // close
        IconName::BatteryCharging => "battery_charging_full", // exact
        IconName::BatteryFull => "battery_full", // exact
        IconName::BatteryLow => "battery_2_bar", // close: one of three bars <-> two of six
        IconName::BatteryMedium => "battery_4_bar", // close: two of three <-> four of six
        IconName::BatteryWarning => "battery_alert", // exact
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
        IconName::Cpu => "memory", // close
        IconName::Dash => "remove",
        IconName::Delete => "delete",
        IconName::Ellipsis => "more_horiz",
        IconName::EllipsisVertical => "more_vert",
        IconName::ExternalLink => "open_in_new",
        IconName::Eye => "visibility",
        IconName::EyeOff => "visibility_off",
        IconName::File => "description",
        IconName::FileText => "description", // exact
        IconName::Folder => "folder",
        IconName::FolderClosed => "folder",
        IconName::FolderOpen => "folder_open",
        IconName::Frame => "crop_free",
        IconName::GalleryVerticalEnd => "view_carousel",
        IconName::Github => "code",
        IconName::Globe => "language",
        IconName::HardDrive => "hard_drive", // exact
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
        IconName::MemoryStick => "memory_alt", // close: a RAM module with pins, like Lucide's
        IconName::Menu => "menu",
        IconName::Minimize => "minimize",
        IconName::Minus => "remove",
        IconName::Moon => "dark_mode",
        IconName::Network => "lan", // close
        IconName::Palette => "palette",
        IconName::PanelBottom => "dock_to_bottom",
        IconName::PanelBottomOpen => "web_asset",
        IconName::PanelLeft => "side_navigation",
        IconName::PanelLeftClose => "left_panel_close",
        IconName::PanelLeftOpen => "left_panel_open",
        IconName::PanelRight => "right_panel_close",
        IconName::PanelRightClose => "right_panel_close",
        IconName::PanelRightOpen => "right_panel_open",
        IconName::Pause => "pause",     // exact
        IconName::Play => "play_arrow", // exact
        IconName::Plus => "add",
        IconName::Redo => "redo",
        IconName::Redo2 => "redo",
        IconName::Replace => "find_replace",
        IconName::ResizeCorner => "drag_indicator",
        IconName::RotateCw => "rotate_right", // exact
        IconName::Search => "search",
        IconName::Settings => "settings",
        IconName::Settings2 => "tune",
        IconName::SortAscending => "arrow_upward",
        IconName::SortDescending => "arrow_downward",
        IconName::SquareTerminal => "terminal",
        IconName::Star => "star",
        IconName::StarFill => "star_fill1", // exact
        // Material Symbols has no star-off glyph; the duplicate hollow star that
        // used to back this variant was removed (spec §10.3). None, no substitute.
        IconName::StarOff => return None,
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
    })
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
/// Covers all 101 gpui-component 0.6.6 `IconName` variants. Returns `None`
/// where the freedesktop icon naming specification has no name for the
/// concept (today every variant has one, some only as a labelled `close` or
/// `approximate` substitute: `StarOff` and `HeartOff` both take
/// `non-starred`, the "not starred / not favourite" state).
#[cfg(target_os = "linux")]
#[must_use]
pub fn freedesktop_name_for_gpui_icon(
    icon: IconName,
    de: native_theme::detect::LinuxDesktop,
) -> Option<&'static str> {
    use native_theme::detect::LinuxDesktop;

    // GTK-based DEs follow GNOME/Adwaita naming; Qt-based follow KDE/Breeze
    let is_gtk = matches!(
        de,
        LinuxDesktop::Gnome
            | LinuxDesktop::Budgie
            | LinuxDesktop::Cinnamon
            | LinuxDesktop::Mate
            | LinuxDesktop::Xfce
    );

    Some(match icon {
        // --- Icons with freedesktop standard names (all DEs) ---
        IconName::Battery => "battery",                     // exact
        IconName::BookOpen => "help-contents",              // close
        IconName::Bot => "face-smile",                      // approximate
        IconName::ChevronDown => "go-down", // close: full nav arrow, not disclosure chevron
        IconName::ChevronLeft => "go-previous", // close
        IconName::ChevronRight => "go-next", // close
        IconName::ChevronUp => "go-up",     // close
        IconName::CircleX => "dialog-error", // close
        IconName::Copy => "edit-copy",      // exact
        IconName::Dash => "list-remove",    // exact
        IconName::Delete => "edit-delete",  // exact
        IconName::File => "text-x-generic", // exact
        IconName::FileText => "text-x-generic", // exact
        IconName::Folder => "folder",       // exact
        IconName::FolderClosed => "folder", // exact
        IconName::FolderOpen => "folder-open", // exact
        IconName::HardDrive => "drive-harddisk", // exact
        IconName::HeartOff => "non-starred", // approximate: un-favourite semantics, star metaphor
        IconName::Info => "dialog-information", // exact
        IconName::LayoutDashboard => "view-grid", // close
        IconName::Map => "find-location",   // close
        IconName::Maximize => "view-fullscreen", // exact
        IconName::Menu => "open-menu",      // exact
        IconName::Minimize => "window-minimize", // exact
        IconName::Minus => "list-remove",   // exact
        IconName::Moon => "weather-clear-night", // close: dark mode toggle
        IconName::Network => "network-workgroup", // close
        IconName::Pause => "media-playback-pause", // exact
        IconName::Play => "media-playback-start", // exact
        IconName::Plus => "list-add",       // exact
        IconName::Redo => "edit-redo",      // exact
        IconName::Redo2 => "edit-redo",     // exact
        IconName::Replace => "edit-find-replace", // exact
        IconName::RotateCw => "object-rotate-right", // exact
        IconName::Search => "edit-find",    // exact
        IconName::Settings => "preferences-system", // exact
        IconName::SortAscending => "view-sort-ascending", // exact
        IconName::SortDescending => "view-sort-descending", // exact
        IconName::SquareTerminal => "utilities-terminal", // close
        IconName::Star => "non-starred", // close: the hollow star, the "not starred" state; `starred` is StarFill's
        IconName::StarFill => "starred", // exact: the filled "starred" state
        IconName::StarOff => "non-starred", // approximate: freedesktop has no slashed star; the "not starred" state
        IconName::Sun => "weather-clear",   // close: light mode toggle
        IconName::TriangleAlert => "dialog-warning", // exact
        IconName::Undo => "edit-undo",      // exact
        IconName::Undo2 => "edit-undo",     // exact
        IconName::User => "system-users",   // exact
        IconName::WindowClose => "window-close", // exact
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
        IconName::BatteryCharging => {
            if is_gtk {
                "battery-full-charging"
            } else {
                "battery-100-charging"
            }
        } // close
        IconName::BatteryFull => {
            if is_gtk {
                "battery-full"
            } else {
                "battery-100"
            }
        } // exact
        IconName::BatteryLow => {
            if is_gtk {
                "battery-low"
            } else {
                "battery-020"
            }
        } // close
        IconName::BatteryMedium => {
            if is_gtk {
                "battery-good"
            } else {
                "battery-050"
            }
        } // close
        IconName::BatteryWarning => {
            if is_gtk {
                "battery-caution"
            } else {
                "battery-010"
            }
        } // close (GNOME) / approximate (KDE: near-empty level, no alert icon)
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
        IconName::Cpu => {
            if is_gtk {
                "computer"
            } else {
                "cpu"
            }
        } // approximate (GNOME) / exact (KDE)
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
        IconName::MemoryStick => {
            if is_gtk {
                "media-flash"
            } else {
                "memory"
            }
        } // approximate (GNOME) / exact (KDE: devices/64/memory.svg is a RAM module)
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
                "sidebar-expand-right"
            }
        } // Breeze's pair of PanelLeft's sidebar-expand-left
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
        IconName::Github => {
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
    })
}

/// Default rasterization size for SVG icons.
///
/// SVGs are rasterized at 2x the typical display size (24px) to look sharp
/// on HiDPI screens. gpui uses the same 2x scale factor internally.
const SVG_RASTERIZE_SIZE: u32 = 48;

/// Maximum allowed rasterization size for icon conversion.
///
/// Values above this are clamped to prevent excessive memory allocation
/// from a single icon render.
const MAX_ICON_SIZE: u32 = 512;

/// Convert [`IconData`] to a gpui [`ImageSource`] for rendering.
///
/// Returns `None` if the icon data cannot be converted (corrupt SVG,
/// unknown variant).
///
/// # Parameters
///
/// - `color`: If `Some`, colorizes monochrome SVGs with the given color
///   (replaces `currentColor`, explicit black fills/strokes, or injects a fill
///   attribute). Best for bundled icon sets (Material, Lucide). Pass `None`
///   for system/OS icons to preserve their native palette.
///   RGBA icons are passed through unchanged regardless of this parameter --
///   the color's alpha channel is discarded during SVG colorization because
///   SVG fill/stroke attributes only accept opaque hex (`#rrggbb`).
/// - `size`: Rasterize size in pixels for SVG icons. `None` defaults to 48px
///   (2x HiDPI at 24px logical). Clamped to 1..=512 range. Pass
///   `logical_size * scale_factor` for DPI-correct rendering. Not used
///   without `svg-rasterize` (below).
///
/// # Without `svg-rasterize`
///
/// The `svg-rasterize` feature (on by default) rasterizes SVG icons in this
/// crate. Without it an SVG icon is still converted, never `None` for want
/// of the feature: the source is an
/// `ImageSource::Image(Image::from_bytes(ImageFormat::Svg, bytes))` holding
/// the SVG bytes, colorized first when `color` is `Some`, and gpui decodes it
/// with its own resvg (gpui-pre `src/platform.rs:3034-3037`). That costs two
/// things. gpui decodes it in the background, so an element holding it paints
/// nothing the first time it comes up (gpui-pre `src/elements/img.rs:534-553`).
/// And gpui chooses the raster size, twice the SVG's own size
/// (`src/svg_renderer.rs:81, 200-206`), so `size` has no effect. The bytes are
/// not parsed here, so a corrupt SVG yields `Some` and paints nothing. RGBA
/// icons are unaffected.
///
/// # Memory
///
/// With `svg-rasterize`, and for RGBA icons always, the returned source
/// carries a decoded [`gpui::RenderImage`], which takes a tile in each
/// window's sprite atlas from the first frame that draws it and
/// keeps it until the image is handed to `App::drop_image` /
/// `Window::drop_image` (gpui-pre `src/app.rs:2841-2851`,
/// `src/window.rs:4997-5008`); nothing releases it on its own. An application
/// that rebuilds its icons -- on an icon-theme change, or a colour change that
/// re-colorizes them -- should drop each replaced source through `drop_image`
/// before it lets go of it. Without `svg-rasterize`, an SVG icon's source is
/// an `ImageSource::Image`, which gpui decodes through its own asset cache
/// (`window.use_asset`, gpui-pre `src/elements/img.rs:550`): the caller holds
/// no `RenderImage` to hand to `drop_image`.
///
/// # Examples
///
/// ```ignore
/// use native_theme::theme::IconData;
/// use native_theme_gpui::icons::to_image_source;
/// use std::borrow::Cow;
///
/// let svg = IconData::Svg(Cow::Borrowed(b"<svg></svg>"));
/// let source = to_image_source(&svg, None, None);         // uncolorized, 48px
/// let colored = to_image_source(&svg, Some(color), None);  // colorized, 48px
/// let sized = to_image_source(&svg, None, Some(96));       // uncolorized, 96px
/// ```
#[must_use]
pub fn to_image_source(
    data: &IconData,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    // Issue 28: clamp icon size to 1..=512
    let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE).clamp(1, MAX_ICON_SIZE);
    match data {
        IconData::Svg(bytes) => {
            if let Some(c) = color {
                let colored = colorize_svg(bytes, c);
                svg_to_render_source(&colored, raster_size)
            } else {
                svg_to_render_source(bytes, raster_size)
            }
        }
        IconData::Rgba {
            width,
            height,
            data,
        } => rgba_to_render_source(*width, *height, data),
        _ => None,
    }
}

/// Convert [`IconData`] to a gpui [`ImageSource`], consuming the data.
///
/// This is the consuming variant of [`to_image_source()`]. It takes ownership
/// of the `IconData` so the caller doesn't need to keep it alive. Internally
/// delegates to [`to_image_source()`] -- the `Vec<u8>` is borrowed, not moved,
/// because rasterization always produces new output buffers.
///
/// Returns `None` if the icon data cannot be converted (corrupt SVG,
/// unknown variant).
///
/// See [`to_image_source()`] for details on the `color` and `size` parameters,
/// and for what an SVG becomes without the `svg-rasterize` feature.
#[must_use]
pub fn into_image_source(
    data: IconData,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    to_image_source(&data, color, size)
}

/// Load a custom icon from an [`IconProvider`] and convert to a gpui [`ImageSource`].
///
/// Equivalent to calling [`load_icon(provider, icon_set)`](native_theme::icons::load_icon)
/// followed by [`to_image_source()`], composing the loading and conversion steps.
///
/// Returns `None` if the provider has no icon for the given set or if
/// conversion fails.
///
/// See [`to_image_source()`] for details on the `color` and `size` parameters,
/// and for what an SVG becomes without the `svg-rasterize` feature.
#[must_use]
pub fn custom_icon_to_image_source(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    let data = load_custom_via_builder(provider, icon_set)?;
    to_image_source(&data, color, size)
}

/// Internal helper: load an icon from a provider using the typed per-set loaders.
///
/// Uses the provider's `icon_name` and `icon_svg` methods directly, then
/// dispatches through [`load_icon`] for system lookups. This preserves
/// the `?Sized` bound on the public API.
fn load_custom_via_builder(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
) -> Option<IconData> {
    // Step 1: Try system loader with provider's name mapping
    if let Some(name) = provider.icon_name(icon_set)
        && let Some(data) = load_icon(name, icon_set)
    {
        return Some(data);
    }
    // Step 2: Try bundled SVG from provider
    if let Some(svg) = provider.icon_svg(icon_set) {
        return Some(IconData::Svg(svg));
    }
    None
}

/// Load a gpui-component icon from a bundled icon set and convert to an [`ImageSource`].
///
/// Combines the icon-name mapping and loading steps into a single call for
/// bundled icon sets. Supports [`native_theme::theme::IconSet::Lucide`] and [`native_theme::theme::IconSet::Material`].
/// Returns `None` for other icon sets (use [`to_image_source`] with
/// [`native_theme::icons::FreedesktopLoader`] for freedesktop system icons).
///
/// See [`to_image_source()`] for details on the `color` and `size` parameters,
/// and for what an SVG becomes without the `svg-rasterize` feature.
///
/// # Examples
///
/// ```ignore
/// use gpui_component::IconName;
/// use native_theme::theme::IconSet;
/// use native_theme_gpui::icons::bundled_icon_to_image_source;
///
/// let source = bundled_icon_to_image_source(IconName::Search, IconSet::Lucide, None, None);
/// ```
#[must_use]
pub fn bundled_icon_to_image_source(
    icon: IconName,
    icon_set: native_theme::theme::IconSet,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    let name = match icon_set {
        native_theme::theme::IconSet::Lucide => lucide_name_for_gpui_icon(icon)?,
        native_theme::theme::IconSet::Material => material_name_for_gpui_icon(icon)?,
        _ => return None,
    };
    // G3 (Phase 93-03): per-set loader replaces the demoted bundled_icon_by_name.
    // Bundled icon sets always return IconData::Svg with Cow::Borrowed to static bytes,
    // so cow.as_ref() is a zero-copy borrow back to &'static [u8].
    let data = load_icon(name, icon_set)?;
    let IconData::Svg(cow) = data else {
        return None;
    };
    // Issue 27: pass &[u8] directly without copying to IconData::Svg
    svg_bytes_to_image_source(cow.as_ref(), color, size)
}

/// Convert raw SVG bytes to an [`ImageSource`].
///
/// This is a convenience wrapper for callers that already have SVG bytes
/// (e.g. from a typed per-set loader in [`native_theme::icons`]) and want to skip
/// the `IconData` intermediate.
///
/// See [`to_image_source()`] for details on the `color` and `size` parameters,
/// and for what an SVG becomes without the `svg-rasterize` feature.
#[must_use]
pub fn bundled_svg_to_image_source(
    svg_bytes: &[u8],
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    // Issue 27: pass &[u8] directly without copying to IconData::Svg
    svg_bytes_to_image_source(svg_bytes, color, size)
}

/// Internal: rasterize SVG bytes to an [`ImageSource`] without copying to [`IconData`].
///
/// Issue 27: avoids the heap copy that wrapping in `IconData::Svg(bytes.to_vec())`
/// would incur. Uses the same colorize + rasterize logic as `to_image_source`'s
/// SVG branch.
fn svg_bytes_to_image_source(
    svg_bytes: &[u8],
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource> {
    let raster_size = size.unwrap_or(SVG_RASTERIZE_SIZE).clamp(1, MAX_ICON_SIZE);
    if let Some(c) = color {
        let colored = colorize_svg(svg_bytes, c);
        svg_to_render_source(&colored, raster_size)
    } else {
        svg_to_render_source(svg_bytes, raster_size)
    }
}

/// Convert all frames of an [`AnimatedIcon::Frames`] to gpui [`ImageSource`]s.
///
/// Returns `Some(AnimatedImageSources)` when the icon is the `Frames` variant,
/// with one `ImageSource` per frame. Returns `None` for `Transform` variants
/// or if any frame fails to convert.
///
/// **All-or-nothing semantics:** if any single frame fails to rasterize, the
/// entire animation returns `None`. This prevents timing glitches where a
/// dropped frame would cause the animation to play faster than intended.
///
/// **Call this once and cache the result.** Do not call on every frame tick --
/// SVG rasterization is expensive. Index into the cached `Vec` using a
/// timer-driven frame counter.
///
/// Without the `svg-rasterize` feature each SVG frame is an undecoded
/// `ImageSource::Image` that gpui decodes in the background (see
/// [`to_image_source()`]), so each frame is blank the first time it is shown
/// and the animation flickers through its first pass.
///
/// Callers should check [`native_theme::detect::prefers_reduced_motion()`] and fall
/// back to [`AnimatedIcon::first_frame()`] for a static display when the user
/// has requested reduced motion.
///
/// # Examples
///
/// ```no_run
/// use native_theme::icons::load_icon_indicator;
/// use native_theme::theme::IconSet;
/// use native_theme_gpui::icons::{animated_frames_to_image_sources, AnimatedImageSources};
///
/// if let Some(AnimatedImageSources { sources, frame_duration_ms }) =
///     load_icon_indicator(IconSet::Lucide)
///         .and_then(|anim| animated_frames_to_image_sources(&anim, None, None))
/// {
///     // Cache `sources`, then on each timer tick (every `frame_duration_ms` ms):
///     // frame_index = (frame_index + 1) % sources.len();
///     // gpui::img(sources[frame_index].clone())
/// }
/// ```
#[must_use]
pub fn animated_frames_to_image_sources(
    anim: &AnimatedIcon,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<AnimatedImageSources> {
    match anim {
        AnimatedIcon::Frames(data) => {
            // FrameList is guaranteed non-empty by construction, so no
            // is_empty() check needed.
            // Issue 4: use map + collect::<Option<Vec<_>>> so the whole
            // animation fails if any frame fails. This prevents timing
            // glitches from silently dropped frames.
            let sources: Option<Vec<ImageSource>> = data
                .frames()
                .iter()
                .map(|f| to_image_source(f, color, size))
                .collect();
            sources.map(|s| AnimatedImageSources {
                sources: s,
                frame_duration_ms: data.frame_duration_ms().get(),
            })
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
/// `duration_ms` comes from [`native_theme::theme::TransformAnimation::Spin`].
/// `animation_id` must be unique among sibling animated elements (accepts
/// `&'static str`, integer IDs, or any `impl Into<ElementId>`).
///
/// This is pure data construction -- no gpui render context is needed to call
/// this function. Only `paint()` on the resulting element requires a window.
///
/// Callers should check [`native_theme::detect::prefers_reduced_motion()`] and fall
/// back to a static icon when the user has requested reduced motion.
///
/// A `duration_ms` of 0 produces a zero-duration animation -- caller's
/// responsibility to pass valid durations.
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

/// Rasterize SVG bytes and return them as a decoded [`ImageSource`].
///
/// Returns `None` if rasterization fails (corrupt SVG, empty data).
#[cfg(feature = "svg-rasterize")]
fn svg_to_render_source(svg_bytes: &[u8], size: u32) -> Option<ImageSource> {
    let Ok(IconData::Rgba {
        width,
        height,
        data,
    }) = native_theme::rasterize::rasterize_svg(svg_bytes, size)
    else {
        return None;
    };
    rgba_to_render_source(width, height, &data)
}

/// Hand SVG bytes to gpui as an undecoded [`ImageSource::Image`].
///
/// Without `svg-rasterize` this crate has no rasterizer of its own; gpui
/// decodes `ImageFormat::Svg` itself (gpui-pre `src/platform.rs:3034-3037`),
/// in the background and at twice the SVG's own size
/// (`src/svg_renderer.rs:81, 200-206`), so `size` is not used. The bytes are
/// not parsed here: this returns `Some` for any input, and an SVG gpui cannot
/// parse paints nothing.
#[cfg(not(feature = "svg-rasterize"))]
fn svg_to_render_source(svg_bytes: &[u8], _size: u32) -> Option<ImageSource> {
    Some(ImageSource::Image(Arc::new(gpui::Image::from_bytes(
        gpui::ImageFormat::Svg,
        svg_bytes.to_vec(),
    ))))
}

/// Wrap RGBA pixels in a [`RenderImage`], which is what gpui draws from.
///
/// `ImageSource::Render` is answered from the value itself, in the frame that
/// asks for it; `ImageSource::Image` goes through `window.use_asset`, which
/// returns nothing until a background decode finishes, so an element holding
/// one paints nothing the first time it comes up (gpui-pre
/// `src/elements/img.rs:534-553`). An icon this connector has already
/// rasterized has no reason to be encoded only for gpui to decode it again.
///
/// The pixels are converted the way gpui's own decoder converts them: it turns
/// the decoded image into RGBA8 and then swaps each pixel's first and third
/// byte, leaving a BGRA buffer, and premultiplies nothing (gpui-pre
/// `src/platform.rs:2923-2937`, the `RenderImage` it builds at `:3042`).
///
/// Returns `None` for zero dimensions, for a buffer whose length is not
/// `width × height × 4`, and for dimensions whose product overflows.
///
/// A [`RenderImage`] the caller keeps holds a tile in each window's sprite
/// atlas from the first frame that draws it, and nothing releases that tile on
/// its own: gpui frees one only through `App::drop_image` /
/// `Window::drop_image` (gpui-pre `src/app.rs:2841-2851`,
/// `src/window.rs:4997-5008`), which its own image cache calls when it evicts
/// an entry (`src/elements/image_cache.rs:240, 269, 280`). An application that
/// rebuilds its icons -- on an icon-theme change, or a colour change that
/// re-colorizes them -- should hand each replaced source to `App::drop_image`
/// before dropping it, or the old tiles stay for the life of the window.
fn rgba_to_render_source(width: u32, height: u32, rgba: &[u8]) -> Option<ImageSource> {
    if width == 0 || height == 0 {
        return None;
    }
    let expected = (width as usize)
        .checked_mul(height as usize)?
        .checked_mul(4)?;
    if rgba.len() != expected {
        return None;
    }
    // `as_chunks_mut::<4>()` yields `[u8; 4]` pixels, so the destructuring is
    // irrefutable and needs no indexing.
    let mut pixels = rgba.to_vec();
    for [r, _g, b, _a] in pixels.as_chunks_mut::<4>().0 {
        std::mem::swap(r, b);
    }
    let buffer = image::RgbaImage::from_raw(width, height, pixels)?;
    Some(ImageSource::Render(Arc::new(RenderImage::new([
        image::Frame::new(buffer),
    ]))))
}

/// Rewrite SVG bytes to use the given color for strokes and fills.
///
/// Handles four SVG color patterns (in order):
/// 1. **`currentColor`** -- replaced with the hex color (Lucide-style SVGs).
/// 2. **Explicit black fills** -- `fill="black"`, `fill="#000000"`, `fill="#000"`
///    are replaced with the hex color (third-party SVGs with hardcoded black).
/// 3. **Explicit black strokes** -- `stroke="black"`, `stroke="#000000"`,
///    `stroke="#000"` are also replaced (Issue 10).
/// 4. **Implicit black** -- if the root `<svg>` tag has no `fill=` attribute,
///    injects `fill="<hex>"` (Material-style SVGs).
///
/// **Limitations** (Issue 34): CSS inline styles (`style="fill:black"`),
/// `fill="rgb(0,0,0)"`, and explicit black on child elements when the root
/// tag has a different fill are not handled. This function is designed for
/// monochrome icon sets; multi-color SVGs should not be colorized.
///
/// **Alpha discard** (Issue 21): the Hsla color's alpha channel is discarded.
/// SVG fill/stroke attributes only accept opaque hex (`#rrggbb`); semi-transparent
/// colors are converted to their opaque RGB equivalent.
fn colorize_svg(svg_bytes: &[u8], color: Hsla) -> Vec<u8> {
    let rgba: gpui::Rgba = color.into();
    let r = (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let hex = format!("#{r:02x}{g:02x}{b:02x}");

    let Ok(svg_str) = std::str::from_utf8(svg_bytes) else {
        // Non-UTF-8 SVGs pass through unmodified -- no corruption risk.
        // These are typically multi-color system icons that shouldn't be colorized.
        return svg_bytes.to_vec();
    };

    // 1. Replace currentColor (handles Lucide-style SVGs)
    let replaced = if svg_str.contains("currentColor") {
        svg_str.replace("currentColor", &hex)
    } else {
        svg_str.to_owned()
    };

    // 2. Replace explicit black fills (handles third-party SVGs)
    let fill_hex = format!("fill=\"{hex}\"");
    let replaced = replaced
        .replace("fill=\"black\"", &fill_hex)
        .replace("fill=\"#000000\"", &fill_hex)
        .replace("fill=\"#000\"", &fill_hex);

    // 3. Issue 10: also replace explicit black strokes
    let stroke_hex = format!("stroke=\"{hex}\"");
    let replaced = replaced
        .replace("stroke=\"black\"", &stroke_hex)
        .replace("stroke=\"#000000\"", &stroke_hex)
        .replace("stroke=\"#000\"", &stroke_hex);

    if replaced != svg_str {
        return replaced.into_bytes();
    }

    // 4. No currentColor or explicit black -- inject fill into root <svg> tag
    // (handles Material-style SVGs with implicit black fill)
    if let Some(pos) = svg_str.find("<svg")
        && let Some(tail) = svg_str.get(pos..)
        && let Some(close) = tail.find('>')
    {
        let tag_end = pos.saturating_add(close);
        if let Some(tag) = svg_str.get(pos..tag_end)
            && !tag.contains("fill=")
        {
            // Handle self-closing tags: inject before '/' in '<svg .../>'
            let is_self_closing = tag_end > 0
                && svg_str
                    .as_bytes()
                    .get(tag_end.saturating_sub(1))
                    .is_some_and(|&b| b == b'/');
            let inject_pos = if is_self_closing {
                tag_end.saturating_sub(1)
            } else {
                tag_end
            };
            if let Some(before) = svg_str.get(..inject_pos)
                && let Some(after) = svg_str.get(inject_pos..)
            {
                let mut result = String::with_capacity(svg_str.len().saturating_add(20));
                result.push_str(before);
                result.push_str(&format!(" fill=\"{hex}\""));
                result.push_str(after);
                return result.into_bytes();
            }
        }
    }

    // SVG already has non-black fill and no currentColor -- return as-is
    svg_bytes.to_vec()
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
        IconName::Battery,
        IconName::BatteryCharging,
        IconName::BatteryFull,
        IconName::BatteryLow,
        IconName::BatteryMedium,
        IconName::BatteryWarning,
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
        IconName::Cpu,
        IconName::Dash,
        IconName::Delete,
        IconName::Ellipsis,
        IconName::EllipsisVertical,
        IconName::ExternalLink,
        IconName::Eye,
        IconName::EyeOff,
        IconName::File,
        IconName::FileText,
        IconName::Folder,
        IconName::FolderClosed,
        IconName::FolderOpen,
        IconName::Frame,
        IconName::GalleryVerticalEnd,
        IconName::Github,
        IconName::Globe,
        IconName::HardDrive,
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
        IconName::MemoryStick,
        IconName::Menu,
        IconName::Minimize,
        IconName::Minus,
        IconName::Moon,
        IconName::Network,
        IconName::Palette,
        IconName::PanelBottom,
        IconName::PanelBottomOpen,
        IconName::PanelLeft,
        IconName::PanelLeftClose,
        IconName::PanelLeftOpen,
        IconName::PanelRight,
        IconName::PanelRightClose,
        IconName::PanelRightOpen,
        IconName::Pause,
        IconName::Play,
        IconName::Plus,
        IconName::Redo,
        IconName::Redo2,
        IconName::Replace,
        IconName::ResizeCorner,
        IconName::RotateCw,
        IconName::Search,
        IconName::Settings,
        IconName::Settings2,
        IconName::SortAscending,
        IconName::SortDescending,
        IconName::SquareTerminal,
        IconName::Star,
        IconName::StarFill,
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

    fn same_variant(a: &IconName, b: &IconName) -> bool {
        // IconName derives neither PartialEq nor Debug (icon_named! emits Clone only).
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    /// Variants a set legitimately lacks (spec §10.1). Every `None` a table
    /// returns must be listed here with its reason; a missing mapping cannot
    /// hide as an intentional one.
    const LUCIDE_NONE_ALLOWED: &[(IconName, &str)] = &[(
        IconName::StarFill,
        "Lucide has no filled star; gpui-kit's star-fill.svg is Lucide's star with fill added",
    )];
    const MATERIAL_NONE_ALLOWED: &[(IconName, &str)] =
        &[(IconName::StarOff, "Material Symbols has no star-off glyph")];

    #[test]
    fn every_none_is_an_allowed_gap() {
        for icon in ALL_ICON_NAMES {
            if lucide_name_for_gpui_icon(icon.clone()).is_none() {
                assert!(
                    LUCIDE_NONE_ALLOWED
                        .iter()
                        .any(|(a, _)| same_variant(a, icon)),
                    "an IconName has no Lucide mapping and is not in LUCIDE_NONE_ALLOWED"
                );
            }
            if material_name_for_gpui_icon(icon.clone()).is_none() {
                assert!(
                    MATERIAL_NONE_ALLOWED
                        .iter()
                        .any(|(a, _)| same_variant(a, icon)),
                    "an IconName has no Material mapping and is not in MATERIAL_NONE_ALLOWED"
                );
            }
        }
    }

    /// §10.1: a table may only name a file that is actually bundled.
    #[cfg(all(feature = "lucide-icons", feature = "material-icons"))]
    #[test]
    fn every_some_resolves_in_its_bundle() {
        use native_theme::theme::IconSet;
        for icon in ALL_ICON_NAMES {
            if let Some(name) = lucide_name_for_gpui_icon(icon.clone()) {
                assert!(
                    bundled_icon_to_image_source(icon.clone(), IconSet::Lucide, None, None)
                        .is_some(),
                    "Lucide name {name} is not bundled"
                );
            }
            if let Some(name) = material_name_for_gpui_icon(icon.clone()) {
                assert!(
                    bundled_icon_to_image_source(icon.clone(), IconSet::Material, None, None)
                        .is_some(),
                    "Material name {name} is not bundled"
                );
            }
        }
    }

    #[test]
    fn lucide_table_returns_lucide_names_for_the_former_gpui_names() {
        assert_eq!(lucide_name_for_gpui_icon(IconName::Close), Some("x"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::WindowClose), Some("x"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::Dash), Some("minus"));
        assert_eq!(
            lucide_name_for_gpui_icon(IconName::WindowMinimize),
            Some("minus")
        );
        assert_eq!(lucide_name_for_gpui_icon(IconName::Inspector), Some("scan"));
        assert_eq!(
            lucide_name_for_gpui_icon(IconName::ResizeCorner),
            Some("grip")
        );
        assert_eq!(
            lucide_name_for_gpui_icon(IconName::SortAscending),
            Some("arrow-up-narrow-wide")
        );
        assert_eq!(
            lucide_name_for_gpui_icon(IconName::SortDescending),
            Some("arrow-down-wide-narrow")
        );
        assert_eq!(
            lucide_name_for_gpui_icon(IconName::WindowMaximize),
            Some("maximize")
        );
        assert_eq!(
            lucide_name_for_gpui_icon(IconName::WindowRestore),
            Some("minimize-2")
        );
        assert_eq!(lucide_name_for_gpui_icon(IconName::StarFill), None);
        assert_eq!(material_name_for_gpui_icon(IconName::StarOff), None);
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

    // Issue 41: ALL_ICON_NAMES count tripwire test. `IconName` is generated by
    // `icon_named!` from gpui-kit-assets' icons directory with no `ALL` or
    // iterator, so the list is audited by hand against gpui-component 0.6.6's
    // 101 files. A *removed* variant breaks the list at compile time; an
    // *added* one is not detected here and must be caught by re-auditing on
    // every gpui-component bump (ROADMAP: an iterable `IconName::ALL`).
    #[test]
    fn all_icon_names_count_matches_gpui_component() {
        assert_eq!(
            ALL_ICON_NAMES.len(),
            101,
            "ALL_ICON_NAMES count changed (got {}) -- update the list",
            ALL_ICON_NAMES.len()
        );
    }

    // Issue 45: data-driven icon mapping regression tests covering all 30 Some() mappings.
    // Uses matches!() since IconName doesn't implement PartialEq.
    #[test]
    fn icon_name_data_driven() {
        // Dialog / Alert
        assert!(matches!(
            icon_name(IconRole::DialogWarning),
            Some(IconName::TriangleAlert)
        ));
        assert!(matches!(
            icon_name(IconRole::DialogError),
            Some(IconName::CircleX)
        ));
        assert!(matches!(
            icon_name(IconRole::DialogInfo),
            Some(IconName::Info)
        ));
        assert!(matches!(
            icon_name(IconRole::DialogSuccess),
            Some(IconName::CircleCheck)
        ));
        // Window Controls
        assert!(matches!(
            icon_name(IconRole::WindowClose),
            Some(IconName::WindowClose)
        ));
        assert!(matches!(
            icon_name(IconRole::WindowMinimize),
            Some(IconName::WindowMinimize)
        ));
        assert!(matches!(
            icon_name(IconRole::WindowMaximize),
            Some(IconName::WindowMaximize)
        ));
        assert!(matches!(
            icon_name(IconRole::WindowRestore),
            Some(IconName::WindowRestore)
        ));
        // Common Actions
        assert!(matches!(
            icon_name(IconRole::ActionDelete),
            Some(IconName::Delete)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionCopy),
            Some(IconName::Copy)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionUndo),
            Some(IconName::Undo2)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionRedo),
            Some(IconName::Redo2)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionSearch),
            Some(IconName::Search)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionSettings),
            Some(IconName::Settings)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionAdd),
            Some(IconName::Plus)
        ));
        assert!(matches!(
            icon_name(IconRole::ActionRemove),
            Some(IconName::Minus)
        ));
        // Navigation
        assert!(matches!(
            icon_name(IconRole::NavBack),
            Some(IconName::ChevronLeft)
        ));
        assert!(matches!(
            icon_name(IconRole::NavForward),
            Some(IconName::ChevronRight)
        ));
        assert!(matches!(
            icon_name(IconRole::NavUp),
            Some(IconName::ChevronUp)
        ));
        assert!(matches!(
            icon_name(IconRole::NavDown),
            Some(IconName::ChevronDown)
        ));
        assert!(matches!(icon_name(IconRole::NavMenu), Some(IconName::Menu)));
        // Files / Places
        assert!(matches!(
            icon_name(IconRole::FileGeneric),
            Some(IconName::File)
        ));
        assert!(matches!(
            icon_name(IconRole::FolderClosed),
            Some(IconName::FolderClosed)
        ));
        assert!(matches!(
            icon_name(IconRole::FolderOpen),
            Some(IconName::FolderOpen)
        ));
        assert!(matches!(
            icon_name(IconRole::TrashEmpty),
            Some(IconName::Delete)
        ));
        // Status
        assert!(matches!(
            icon_name(IconRole::StatusBusy),
            Some(IconName::Loader)
        ));
        assert!(matches!(
            icon_name(IconRole::StatusCheck),
            Some(IconName::Check)
        ));
        assert!(matches!(
            icon_name(IconRole::StatusError),
            Some(IconName::CircleX)
        ));
        // System
        assert!(matches!(
            icon_name(IconRole::UserAccount),
            Some(IconName::User)
        ));
        assert!(matches!(
            icon_name(IconRole::Notification),
            Some(IconName::Bell)
        ));
        // None cases
        assert!(icon_name(IconRole::Shield).is_none());
        assert!(icon_name(IconRole::Lock).is_none());
        assert!(icon_name(IconRole::Help).is_none());
    }

    // --- to_image_source tests ---

    /// The decoded image behind a source, or a failure naming the variant that
    /// came instead. `ImageSource::Image` is the one that blinks: gpui decodes
    /// it through `window.use_asset` and the element paints nothing until that
    /// finishes (gpui-pre `src/elements/img.rs:534-553`).
    fn rendered(source: ImageSource) -> std::sync::Arc<gpui::RenderImage> {
        match source {
            ImageSource::Render(image) => image,
            ImageSource::Image(_) => panic!("an icon was handed to gpui undecoded, as bytes"),
            _ => panic!("an icon was handed to gpui as something other than a decoded image"),
        }
    }

    #[cfg(feature = "svg-rasterize")]
    #[test]
    fn to_image_source_svg_returns_a_decoded_image() {
        // Valid SVG that resvg can parse
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>"),
        );
        let source = to_image_source(&svg, None, Some(8)).expect("valid SVG should convert");
        let image = rendered(source);
        assert_eq!(image.frame_count(), 1);
        assert_eq!(
            image.size(0),
            gpui::size(gpui::DevicePixels(8), gpui::DevicePixels(8))
        );
    }

    #[test]
    fn to_image_source_rgba_returns_a_decoded_image_in_gpuis_channel_order() {
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
        let image = rendered(source);
        assert_eq!(image.frame_count(), 1);
        // A `RenderImage` holds BGRA, which is what gpui's own decoder leaves
        // behind: it converts to RGBA8 and then swaps each pixel's first and
        // third byte (gpui-pre `src/platform.rs:2932-2935`).
        assert_eq!(
            image.as_bytes(0),
            Some(
                [
                    0, 0, 255, 255, // red
                    0, 255, 0, 255, // green
                    255, 0, 0, 255, // blue
                    0, 255, 255, 255, // yellow
                ]
                .as_slice()
            )
        );
    }

    #[test]
    fn to_image_source_with_color() {
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M0 0' stroke='currentColor'/></svg>"),
        );
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = to_image_source(&svg, Some(color), None);
        assert!(result.is_some(), "colorized SVG should convert");
    }

    #[cfg(feature = "svg-rasterize")]
    #[test]
    fn to_image_source_with_custom_size() {
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>"),
        );
        let result = to_image_source(&svg, None, Some(32));
        assert!(result.is_some(), "custom size SVG should convert");
    }

    // Issue 28: size clamping
    #[cfg(feature = "svg-rasterize")]
    #[test]
    fn to_image_source_clamps_oversized() {
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>"),
        );
        // Should not panic or OOM with a huge size -- gets clamped to 512
        let result = to_image_source(&svg, None, Some(99999));
        assert!(result.is_some(), "oversized should clamp and still convert");
    }

    #[cfg(feature = "svg-rasterize")]
    #[test]
    fn to_image_source_clamps_zero_size() {
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>"),
        );
        // Size 0 should clamp to 1
        let result = to_image_source(&svg, None, Some(0));
        assert!(result.is_some(), "zero size should clamp to 1 and convert");
    }

    /// Without `svg-rasterize` an SVG is still an icon: gpui gets the
    /// (colorized) SVG bytes and decodes them itself.
    #[cfg(not(feature = "svg-rasterize"))]
    #[test]
    fn to_image_source_svg_without_rasterize_hands_gpui_the_svg() {
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M0 0' stroke='currentColor'/></svg>"),
        );
        let Some(ImageSource::Image(image)) = to_image_source(&svg, None, None) else {
            panic!("an SVG without svg-rasterize should become an undecoded SVG image");
        };
        assert_eq!(image.format, gpui::ImageFormat::Svg);
        assert_eq!(image.bytes.as_slice(), svg_bytes(&svg));

        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let Some(ImageSource::Image(colored)) = to_image_source(&svg, Some(color), None) else {
            panic!("a colorized SVG without svg-rasterize should become an undecoded SVG image");
        };
        assert_eq!(colored.format, gpui::ImageFormat::Svg);
        assert_ne!(colored.bytes.as_slice(), svg_bytes(&svg));
        assert_eq!(colored.bytes, colorize_svg(svg_bytes(&svg), color));
    }

    #[cfg(not(feature = "svg-rasterize"))]
    fn svg_bytes(data: &IconData) -> &[u8] {
        match data {
            IconData::Svg(bytes) => bytes,
            _ => panic!("not an SVG"),
        }
    }

    // --- decoded-image tests ---

    #[test]
    fn rgba_to_render_source_keeps_the_pixels_and_the_size() {
        let rgba = vec![0u8; 4 * 4 * 4]; // 4x4 image
        let image = rendered(rgba_to_render_source(4, 4, &rgba).expect("valid input"));
        assert_eq!(
            image.size(0),
            gpui::size(gpui::DevicePixels(4), gpui::DevicePixels(4))
        );
        assert_eq!(image.as_bytes(0).map(<[u8]>::len), Some(rgba.len()));
    }

    #[test]
    fn rgba_to_render_source_pixel_order_is_bgra() {
        // Input RGBA: R=0xAA, G=0xBB, B=0xCC, A=0xDD
        let rgba = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let image = rendered(rgba_to_render_source(1, 1, &rgba).expect("valid input"));
        assert_eq!(image.as_bytes(0), Some([0xCC, 0xBB, 0xAA, 0xDD].as_slice()));
    }

    #[test]
    fn rgba_to_render_source_zero_width_returns_none() {
        let rgba = vec![0u8; 4];
        assert!(rgba_to_render_source(0, 1, &rgba).is_none());
    }

    #[test]
    fn rgba_to_render_source_zero_height_returns_none() {
        let rgba = vec![0u8; 4];
        assert!(rgba_to_render_source(1, 0, &rgba).is_none());
    }

    #[test]
    fn rgba_to_render_source_mismatched_length_returns_none() {
        // 2x2 image expects 16 bytes, provide 12
        let rgba = vec![0u8; 12];
        assert!(rgba_to_render_source(2, 2, &rgba).is_none());
    }

    #[test]
    fn rgba_to_render_source_oversized_length_returns_none() {
        // 2x2 image expects 16 bytes, provide 20
        let rgba = vec![0u8; 20];
        assert!(rgba_to_render_source(2, 2, &rgba).is_none());
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
    fn colorize_svg_non_utf8_returns_original() {
        // Non-UTF-8 bytes: valid SVG prefix followed by invalid byte sequence
        let mut svg = b"<svg><path fill=\"black\" d=\"M0 0\"/>".to_vec();
        svg.push(0xFF); // invalid UTF-8 byte
        svg.extend_from_slice(b"</svg>");
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = colorize_svg(&svg, color);
        assert_eq!(result, svg, "non-UTF-8 input should be returned unchanged");
    }

    // Issue 10: stroke="black" replacement
    #[test]
    fn colorize_svg_replaces_stroke_black() {
        let svg = b"<svg><path stroke=\"black\" d=\"M0 0h24\"/></svg>";
        let color = gpui::hsla(0.6, 0.7, 0.5, 1.0);
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("stroke=\"black\""),
            "stroke=\"black\" should be replaced, got: {}",
            result_str
        );
        assert!(
            result_str.contains("stroke=\"#"),
            "should contain hex stroke, got: {}",
            result_str
        );
    }

    #[test]
    fn colorize_svg_replaces_stroke_hex_black() {
        let svg = b"<svg><line stroke=\"#000000\" x1=\"0\" y1=\"0\" x2=\"24\" y2=\"24\"/></svg>";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            !result_str.contains("#000000"),
            "stroke=\"#000000\" should be replaced"
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

    // Issue 66: colorize_svg preserves non-black fill on root <svg>
    #[test]
    fn colorize_svg_with_fill_white_root() {
        // SVG root has fill="white" — should NOT be replaced with target color
        let svg = b"<svg fill=\"white\"><path/></svg>";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0); // red
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        assert!(
            result_str.contains("fill=\"white\""),
            "fill=\"white\" should be preserved, got: {}",
            result_str
        );
    }

    // Issue 66: colorize_svg replaces stroke="black" even when root has fill="none"
    #[test]
    fn colorize_svg_with_fill_none_root() {
        let svg = b"<svg fill=\"none\"><path stroke=\"black\"/></svg>";
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0); // red
        let result = colorize_svg(svg, color);
        let result_str = String::from_utf8(result).unwrap();
        // stroke="black" should be replaced with the target color hex
        assert!(
            !result_str.contains("stroke=\"black\""),
            "stroke=\"black\" should be replaced, got: {}",
            result_str
        );
        assert!(
            result_str.contains("stroke=\"#"),
            "should contain hex stroke, got: {}",
            result_str
        );
    }

    // --- into_image_source tests ---

    #[test]
    fn into_image_source_svg_returns_some() {
        let svg = IconData::Svg(
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>"),
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
            std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M0 0' stroke='currentColor'/></svg>"),
        );
        let color = gpui::hsla(0.0, 1.0, 0.5, 1.0);
        let result = into_image_source(svg, Some(color), None);
        assert!(result.is_some(), "colorized SVG should convert");
    }

    // --- custom_icon tests ---

    // Test helper: minimal IconProvider that returns a bundled SVG
    #[derive(Debug)]
    struct TestCustomIcon;

    impl native_theme::theme::IconProvider for TestCustomIcon {
        fn icon_name(&self, _set: native_theme::theme::IconSet) -> Option<&str> {
            None // No system name -- forces bundled SVG path
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            Some(std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10'/></svg>"))
        }
    }

    // Provider with no mappings at all
    #[derive(Debug)]
    struct EmptyProvider;

    impl native_theme::theme::IconProvider for EmptyProvider {
        fn icon_name(&self, _set: native_theme::theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            None
        }
    }

    #[test]
    fn custom_icon_to_image_source_with_svg_provider_returns_some() {
        let result = custom_icon_to_image_source(
            &TestCustomIcon,
            native_theme::theme::IconSet::Material,
            None,
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_source_with_empty_provider_returns_none() {
        let result = custom_icon_to_image_source(
            &EmptyProvider,
            native_theme::theme::IconSet::Material,
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
            native_theme::theme::IconSet::Material,
            Some(color),
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_source_accepts_dyn_provider() {
        let boxed: Box<dyn native_theme::theme::IconProvider> = Box::new(TestCustomIcon);
        let result = custom_icon_to_image_source(
            &*boxed,
            native_theme::theme::IconSet::Material,
            None,
            None,
        );
        assert!(result.is_some());
    }

    // --- bundled_icon_to_image_source tests ---

    #[cfg(feature = "lucide-icons")]
    #[test]
    fn bundled_icon_lucide_returns_some() {
        let result = bundled_icon_to_image_source(
            IconName::Search,
            native_theme::theme::IconSet::Lucide,
            None,
            None,
        );
        assert!(result.is_some(), "Lucide search icon should convert");
    }

    #[cfg(feature = "material-icons")]
    #[test]
    fn bundled_icon_material_returns_some() {
        let result = bundled_icon_to_image_source(
            IconName::Search,
            native_theme::theme::IconSet::Material,
            None,
            None,
        );
        assert!(result.is_some(), "Material search icon should convert");
    }

    #[test]
    fn bundled_icon_freedesktop_returns_none() {
        let result = bundled_icon_to_image_source(
            IconName::Search,
            native_theme::theme::IconSet::Freedesktop,
            None,
            None,
        );
        assert!(
            result.is_none(),
            "Freedesktop is not bundled -- should return None"
        );
    }

    #[cfg(feature = "lucide-icons")]
    #[test]
    fn bundled_icon_with_color() {
        let color = Hsla {
            h: 0.0,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };
        let result = bundled_icon_to_image_source(
            IconName::Check,
            native_theme::theme::IconSet::Lucide,
            Some(color),
            None,
        );
        assert!(result.is_some(), "colorized bundled icon should convert");
    }

    // --- animated icon tests ---

    /// Every frame arrives decoded. An undecoded one would be blank on the
    /// first pass through the animation, which is what made an animated icon
    /// flicker for its first few seconds.
    #[cfg(feature = "svg-rasterize")]
    #[test]
    fn animated_frames_returns_decoded_sources() {
        let anim = AnimatedIcon::frames(
            vec![
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10' fill='red'/></svg>")),
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='8' fill='blue'/></svg>")),
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='6' fill='green'/></svg>")),
            ],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_image_sources(&anim, None, None);
        let ais = result.expect("Frames variant should return Some");
        assert_eq!(ais.sources.len(), 3);
        assert_eq!(ais.frame_duration_ms, 80);
        for source in ais.sources {
            assert_eq!(rendered(source).frame_count(), 1);
        }
    }

    #[test]
    fn animated_frames_transform_returns_none() {
        let anim = AnimatedIcon::transform(
            IconData::Svg(std::borrow::Cow::Borrowed(
                b"<svg xmlns='http://www.w3.org/2000/svg'><circle cx='12' cy='12' r='10'/></svg>",
            )),
            native_theme::theme::TransformAnimation::Spin {
                duration_ms: std::num::NonZeroU32::new(1000).expect("test constant"),
            },
        );
        let result = animated_frames_to_image_sources(&anim, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn animated_frames_empty_returns_none() {
        // Empty FrameList is rejected at construction, so this test verifies that.
        let result = AnimatedIcon::frames(
            vec![],
            std::num::NonZeroU32::new(80).expect("test constant"),
        );
        assert!(result.is_err());
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
    use native_theme::detect::LinuxDesktop;

    #[test]
    fn every_gpui_icon_has_a_freedesktop_name_on_kde() {
        for name in ALL_ICON_NAMES {
            let fd_name = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Kde);
            assert!(
                fd_name.is_some_and(|n| !n.is_empty()),
                "Empty KDE freedesktop mapping for an IconName variant",
            );
        }
    }

    #[test]
    fn eye_differs_by_de() {
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Eye, LinuxDesktop::Kde),
            Some("view-visible"),
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Eye, LinuxDesktop::Gnome),
            Some("view-reveal"),
        );
    }

    /// The status bar's two panel toggles are a matching pair on KDE: Breeze
    /// ships `sidebar-expand-left` and `sidebar-expand-right` side by side
    /// (`actions/22/`). The GTK arm stays Adwaita's `sidebar-show-right`.
    #[test]
    fn panel_right_is_breezes_pair_of_panel_left() {
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::PanelLeft, LinuxDesktop::Kde),
            Some("sidebar-expand-left"),
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::PanelRight, LinuxDesktop::Kde),
            Some("sidebar-expand-right"),
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::PanelRight, LinuxDesktop::Gnome),
            Some("sidebar-show-right"),
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
    fn every_gpui_icon_has_a_freedesktop_name_on_gnome() {
        for name in ALL_ICON_NAMES {
            let fd_name = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Gnome);
            assert!(
                fd_name.is_some_and(|n| !n.is_empty()),
                "Empty GNOME freedesktop mapping for an IconName variant",
            );
        }
    }

    /// §10.4: the two star states must not share a glyph.
    #[test]
    fn star_states_have_distinct_freedesktop_names() {
        for de in [LinuxDesktop::Kde, LinuxDesktop::Gnome] {
            assert_eq!(
                freedesktop_name_for_gpui_icon(IconName::Star, de),
                Some("non-starred")
            );
            assert_eq!(
                freedesktop_name_for_gpui_icon(IconName::StarFill, de),
                Some("starred")
            );
        }
    }

    #[test]
    fn xfce_uses_gnome_names() {
        // XFCE is GTK-based and should use GNOME naming convention
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Eye, LinuxDesktop::Xfce),
            Some("view-reveal"),
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon(IconName::Bell, LinuxDesktop::Xfce),
            Some("alarm"),
        );
    }

    #[cfg(feature = "system-icons")]
    #[test]
    fn all_kde_names_resolve_in_breeze() {
        let theme = match native_theme::theme::system_icon_theme() {
            Ok(theme) => theme,
            Err(e) => {
                eprintln!("Skipping: no system icon theme detected ({e})");
                return;
            }
        };
        // Only meaningful on a KDE system with Breeze installed
        if !theme.to_lowercase().contains("breeze") {
            eprintln!("Skipping: system theme is '{}', not Breeze", theme);
            return;
        }

        let mut missing = Vec::new();
        for name in ALL_ICON_NAMES {
            let Some(fd_name) = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Kde)
            else {
                continue;
            };
            if FreedesktopLoader::new(fd_name)
                .theme(&theme)
                .size(24)
                .load()
                .is_none()
            {
                missing.push(format!("{} (not found)", fd_name));
            }
        }
        assert!(
            missing.is_empty(),
            "These gpui icons did not resolve in Breeze:\n  {}",
            missing.join("\n  "),
        );
    }

    #[cfg(feature = "system-icons")]
    #[test]
    fn gnome_names_resolve_in_adwaita() {
        // Verify GNOME mappings resolve against installed Adwaita theme.
        // Only runs when Adwaita is installed (it usually is on any Linux).
        let mut missing = Vec::new();
        for name in ALL_ICON_NAMES {
            let Some(fd_name) = freedesktop_name_for_gpui_icon(name.clone(), LinuxDesktop::Gnome)
            else {
                continue;
            };
            if FreedesktopLoader::new(fd_name)
                .theme("Adwaita")
                .size(24)
                .load()
                .is_none()
            {
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
