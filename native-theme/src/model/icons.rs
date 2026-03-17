// Icon type definitions: IconRole, IconData, IconSet
//
// These are the core icon types for the native-theme icon system.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// Semantic icon roles for cross-platform icon resolution.
///
/// Each variant represents a conceptual icon role (not a specific icon image).
/// Platform-specific icon identifiers are resolved via
/// [`icon_name()`](crate::icon_name) using an [`IconSet`].
///
/// # Categories
///
/// Variants are grouped by prefix into 7 categories:
/// - **Dialog** (6): Alerts and dialog indicators
/// - **Window** (4): Window control buttons
/// - **Action** (14): Common user actions
/// - **Navigation** (6): Directional and structural navigation
/// - **Files** (5): File and folder representations
/// - **Status** (3): State indicators
/// - **System** (4): System-level UI elements
///
/// # Examples
///
/// ```
/// use native_theme::IconRole;
///
/// let role = IconRole::ActionSave;
/// match role {
///     IconRole::ActionSave => println!("save icon"),
///     _ => println!("other icon"),
/// }
///
/// // Iterate all roles
/// assert_eq!(IconRole::ALL.len(), 42);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconRole {
    // Dialog / Alert (6)
    /// Warning indicator for dialogs
    DialogWarning,
    /// Error indicator for dialogs
    DialogError,
    /// Informational indicator for dialogs
    DialogInfo,
    /// Question indicator for dialogs
    DialogQuestion,
    /// Success/confirmation indicator for dialogs
    DialogSuccess,
    /// Security/shield indicator
    Shield,

    // Window Controls (4)
    /// Close window button
    WindowClose,
    /// Minimize window button
    WindowMinimize,
    /// Maximize window button
    WindowMaximize,
    /// Restore window button (from maximized state)
    WindowRestore,

    // Common Actions (14)
    /// Save action
    ActionSave,
    /// Delete action
    ActionDelete,
    /// Copy to clipboard
    ActionCopy,
    /// Paste from clipboard
    ActionPaste,
    /// Cut to clipboard
    ActionCut,
    /// Undo last action
    ActionUndo,
    /// Redo last undone action
    ActionRedo,
    /// Search / find
    ActionSearch,
    /// Settings / preferences
    ActionSettings,
    /// Edit / modify
    ActionEdit,
    /// Add / create new item
    ActionAdd,
    /// Remove item
    ActionRemove,
    /// Refresh / reload
    ActionRefresh,
    /// Print
    ActionPrint,

    // Navigation (6)
    /// Navigate backward
    NavBack,
    /// Navigate forward
    NavForward,
    /// Navigate up in hierarchy
    NavUp,
    /// Navigate down in hierarchy
    NavDown,
    /// Navigate to home / root
    NavHome,
    /// Open menu / hamburger
    NavMenu,

    // Files / Places (5)
    /// Generic file icon
    FileGeneric,
    /// Closed folder
    FolderClosed,
    /// Open folder
    FolderOpen,
    /// Empty trash / recycle bin
    TrashEmpty,
    /// Full trash / recycle bin
    TrashFull,

    // Status (3)
    /// Loading / in-progress indicator
    StatusLoading,
    /// Check / success indicator
    StatusCheck,
    /// Error state indicator
    StatusError,

    // System (4)
    /// User account / profile
    UserAccount,
    /// Notification / bell
    Notification,
    /// Help / question mark
    Help,
    /// Lock / security
    Lock,
}

impl IconRole {
    /// All icon role variants, useful for iteration and exhaustive testing.
    ///
    /// Contains exactly 42 variants, one for each role, in declaration order.
    pub const ALL: [IconRole; 42] = [
        // Dialog (6)
        Self::DialogWarning,
        Self::DialogError,
        Self::DialogInfo,
        Self::DialogQuestion,
        Self::DialogSuccess,
        Self::Shield,
        // Window (4)
        Self::WindowClose,
        Self::WindowMinimize,
        Self::WindowMaximize,
        Self::WindowRestore,
        // Action (14)
        Self::ActionSave,
        Self::ActionDelete,
        Self::ActionCopy,
        Self::ActionPaste,
        Self::ActionCut,
        Self::ActionUndo,
        Self::ActionRedo,
        Self::ActionSearch,
        Self::ActionSettings,
        Self::ActionEdit,
        Self::ActionAdd,
        Self::ActionRemove,
        Self::ActionRefresh,
        Self::ActionPrint,
        // Navigation (6)
        Self::NavBack,
        Self::NavForward,
        Self::NavUp,
        Self::NavDown,
        Self::NavHome,
        Self::NavMenu,
        // Files (5)
        Self::FileGeneric,
        Self::FolderClosed,
        Self::FolderOpen,
        Self::TrashEmpty,
        Self::TrashFull,
        // Status (3)
        Self::StatusLoading,
        Self::StatusCheck,
        Self::StatusError,
        // System (4)
        Self::UserAccount,
        Self::Notification,
        Self::Help,
        Self::Lock,
    ];
}

/// Icon data returned by loading functions.
///
/// Represents the actual pixel or vector data for an icon. This type is
/// produced by platform icon loaders and bundled icon accessors.
///
/// # Examples
///
/// ```
/// use native_theme::IconData;
///
/// let svg = IconData::Svg(b"<svg></svg>".to_vec());
/// match svg {
///     IconData::Svg(bytes) => assert!(!bytes.is_empty()),
///     _ => unreachable!(),
/// }
///
/// let rgba = IconData::Rgba { width: 16, height: 16, data: vec![0; 16*16*4] };
/// match rgba {
///     IconData::Rgba { width, height, .. } => {
///         assert_eq!(width, 16);
///         assert_eq!(height, 16);
///     }
///     _ => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[must_use = "loading icon data without using it is likely a bug"]
pub enum IconData {
    /// SVG content as raw bytes (from freedesktop themes, bundled icon sets).
    Svg(Vec<u8>),

    /// Rasterized RGBA pixels (from macOS/Windows system APIs).
    Rgba {
        /// Image width in pixels.
        width: u32,
        /// Image height in pixels.
        height: u32,
        /// Raw RGBA pixel data (4 bytes per pixel, row-major).
        data: Vec<u8>,
    },
}

/// Known icon sets that provide platform-specific icon identifiers.
///
/// Each variant corresponds to a well-known icon naming system.
/// Use [`from_name`](IconSet::from_name) to parse from TOML strings
/// and [`name`](IconSet::name) to serialize back to kebab-case.
///
/// # Examples
///
/// ```
/// use native_theme::IconSet;
///
/// let set = IconSet::from_name("sf-symbols").unwrap();
/// assert_eq!(set, IconSet::SfSymbols);
/// assert_eq!(set.name(), "sf-symbols");
///
/// // Round-trip
/// let name = IconSet::Material.name();
/// assert_eq!(IconSet::from_name(name), Some(IconSet::Material));
///
/// // Unknown names return None
/// assert_eq!(IconSet::from_name("unknown"), None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IconSet {
    /// Apple SF Symbols (macOS, iOS).
    SfSymbols,
    /// Microsoft Segoe Fluent Icons (Windows).
    SegoeIcons,
    /// freedesktop Icon Naming Specification (Linux).
    Freedesktop,
    /// Google Material Symbols.
    Material,
    /// Lucide Icons (fork of Feather).
    Lucide,
}

impl IconSet {
    /// Parse an icon set from its kebab-case string identifier.
    ///
    /// Accepts the names used in TOML configuration:
    /// `"sf-symbols"`, `"segoe-fluent"`, `"freedesktop"`, `"material"`, `"lucide"`.
    ///
    /// Returns `None` for unrecognized names.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "sf-symbols" => Some(Self::SfSymbols),
            "segoe-fluent" => Some(Self::SegoeIcons),
            "freedesktop" => Some(Self::Freedesktop),
            "material" => Some(Self::Material),
            "lucide" => Some(Self::Lucide),
            _ => None,
        }
    }

    /// The kebab-case string identifier for this icon set, as used in TOML.
    pub fn name(&self) -> &'static str {
        match self {
            Self::SfSymbols => "sf-symbols",
            Self::SegoeIcons => "segoe-fluent",
            Self::Freedesktop => "freedesktop",
            Self::Material => "material",
            Self::Lucide => "lucide",
        }
    }
}

/// Trait for types that map icon identifiers to platform-specific names and SVG data.
///
/// Implement this trait on an enum to make its variants loadable via
/// [`load_custom_icon()`](crate::load_custom_icon). The typical pattern is
/// for each enum variant to represent an icon role, with `icon_name()` returning
/// the platform-specific identifier and `icon_svg()` returning embedded SVG bytes.
///
/// The `native-theme-build` crate can auto-generate implementations from TOML
/// definitions at build time, so manual implementation is only needed for
/// special cases.
///
/// [`IconRole`] implements this trait, delegating to the built-in icon mappings.
///
/// # Object Safety
///
/// This trait is object-safe (only requires [`Debug`] as a supertrait).
/// `Box<dyn IconProvider>` works for dynamic dispatch.
///
/// # Examples
///
/// ```
/// use native_theme::{IconProvider, IconSet};
///
/// #[derive(Debug)]
/// enum MyIcon { Play, Pause }
///
/// impl IconProvider for MyIcon {
///     fn icon_name(&self, set: IconSet) -> Option<&str> {
///         match (self, set) {
///             (MyIcon::Play, IconSet::SfSymbols) => Some("play.fill"),
///             (MyIcon::Play, IconSet::Material) => Some("play_arrow"),
///             (MyIcon::Pause, IconSet::SfSymbols) => Some("pause.fill"),
///             (MyIcon::Pause, IconSet::Material) => Some("pause"),
///             _ => None,
///         }
///     }
///     fn icon_svg(&self, _set: IconSet) -> Option<&'static [u8]> {
///         None // No bundled SVGs in this example
///     }
/// }
/// ```
pub trait IconProvider: std::fmt::Debug {
    /// Return the platform/theme-specific icon name for this icon in the given set.
    fn icon_name(&self, set: IconSet) -> Option<&str>;

    /// Return bundled SVG bytes for this icon in the given set.
    fn icon_svg(&self, set: IconSet) -> Option<&'static [u8]>;
}

impl IconProvider for IconRole {
    fn icon_name(&self, set: IconSet) -> Option<&str> {
        icon_name(set, *self)
    }

    fn icon_svg(&self, set: IconSet) -> Option<&'static [u8]> {
        crate::model::bundled::bundled_icon_svg(set, *self)
    }
}

/// Look up the platform-specific icon identifier for a given icon set and role.
///
/// Returns `Some(name)` if the icon set has a standard icon for the role,
/// or `None` if no standard icon exists (e.g., SF Symbols has no open-folder
/// variant).
///
/// # Examples
///
/// ```
/// use native_theme::{IconSet, IconRole, icon_name};
///
/// assert_eq!(icon_name(IconSet::SfSymbols, IconRole::ActionCopy), Some("doc.on.doc"));
/// assert_eq!(icon_name(IconSet::Freedesktop, IconRole::ActionCopy), Some("edit-copy"));
/// assert_eq!(icon_name(IconSet::SfSymbols, IconRole::FolderOpen), None);
/// ```
#[allow(unreachable_patterns)] // wildcard arm kept for #[non_exhaustive] forward compat
pub fn icon_name(set: IconSet, role: IconRole) -> Option<&'static str> {
    match set {
        IconSet::SfSymbols => sf_symbols_name(role),
        IconSet::SegoeIcons => segoe_name(role),
        IconSet::Freedesktop => freedesktop_name(role),
        IconSet::Material => material_name(role),
        IconSet::Lucide => lucide_name(role),
        _ => None,
    }
}

/// Detect the native icon set for the current operating system.
///
/// Returns the platform-appropriate icon set at runtime using `cfg!()` macros:
/// - macOS / iOS: [`IconSet::SfSymbols`]
/// - Windows: [`IconSet::SegoeIcons`]
/// - Linux: [`IconSet::Freedesktop`]
/// - Other: [`IconSet::Material`] (safe cross-platform fallback)
///
/// # Examples
///
/// ```
/// use native_theme::{IconSet, system_icon_set};
///
/// let set = system_icon_set();
/// // On Linux, this returns Freedesktop
/// ```
#[must_use = "this returns the current icon set for the platform"]
pub fn system_icon_set() -> IconSet {
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        IconSet::SfSymbols
    } else if cfg!(target_os = "windows") {
        IconSet::SegoeIcons
    } else if cfg!(target_os = "linux") {
        IconSet::Freedesktop
    } else {
        IconSet::Material
    }
}

/// Detect the icon theme name for the current platform.
///
/// Returns the name of the icon theme that provides the actual icon files:
/// - **macOS / iOS:** `"sf-symbols"` (no user-configurable icon theme)
/// - **Windows:** `"segoe-fluent"` (no user-configurable icon theme)
/// - **Linux:** DE-specific detection (e.g., `"breeze-dark"`, `"Adwaita"`)
/// - **Other:** `"material"` (bundled fallback)
///
/// On Linux, the detection method depends on the desktop environment:
/// - KDE: reads `[Icons] Theme` from `kdeglobals`
/// - GNOME/Budgie: `gsettings get org.gnome.desktop.interface icon-theme`
/// - Cinnamon: `gsettings get org.cinnamon.desktop.interface icon-theme`
/// - XFCE: `xfconf-query -c xsettings -p /Net/IconThemeName`
/// - MATE: `gsettings get org.mate.interface icon-theme`
/// - LXQt: reads `icon_theme` from `~/.config/lxqt/lxqt.conf`
/// - Unknown: tries KDE, then GNOME gsettings, then `"hicolor"`
///
/// # Examples
///
/// ```
/// use native_theme::system_icon_theme;
///
/// let theme = system_icon_theme();
/// // On a KDE system with Breeze Dark: "breeze-dark"
/// // On macOS: "sf-symbols"
/// ```
#[must_use = "this returns the current icon theme name"]
pub fn system_icon_theme() -> String {
    #[cfg(target_os = "linux")]
    static CACHED_ICON_THEME: OnceLock<String> = OnceLock::new();

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        return "sf-symbols".to_string();
    }

    #[cfg(target_os = "windows")]
    {
        return "segoe-fluent".to_string();
    }

    #[cfg(target_os = "linux")]
    {
        CACHED_ICON_THEME
            .get_or_init(detect_linux_icon_theme)
            .clone()
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "ios"
    )))]
    {
        "material".to_string()
    }
}

/// Linux icon theme detection, dispatched by desktop environment.
#[cfg(target_os = "linux")]
fn detect_linux_icon_theme() -> String {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let de = crate::detect_linux_de(&desktop);

    match de {
        crate::LinuxDesktop::Kde => detect_kde_icon_theme(),
        crate::LinuxDesktop::Gnome | crate::LinuxDesktop::Budgie => {
            gsettings_icon_theme("org.gnome.desktop.interface")
        }
        crate::LinuxDesktop::Cinnamon => gsettings_icon_theme("org.cinnamon.desktop.interface"),
        crate::LinuxDesktop::Xfce => detect_xfce_icon_theme(),
        crate::LinuxDesktop::Mate => gsettings_icon_theme("org.mate.interface"),
        crate::LinuxDesktop::LxQt => detect_lxqt_icon_theme(),
        crate::LinuxDesktop::Unknown => {
            let kde = detect_kde_icon_theme();
            if kde != "hicolor" {
                return kde;
            }
            let gnome = gsettings_icon_theme("org.gnome.desktop.interface");
            if gnome != "hicolor" {
                return gnome;
            }
            "hicolor".to_string()
        }
    }
}

/// Read icon theme from KDE's kdeglobals INI file.
///
/// Checks `~/.config/kdeglobals` first, then `~/.config/kdedefaults/kdeglobals`
/// (Plasma 6 stores distro defaults there, including the icon theme).
///
/// Uses simple line parsing — no `configparser` dependency required — so this
/// works without the `kde` feature enabled.
#[cfg(target_os = "linux")]
fn detect_kde_icon_theme() -> String {
    let config_dir = xdg_config_dir();
    let paths = [
        config_dir.join("kdeglobals"),
        config_dir.join("kdedefaults").join("kdeglobals"),
    ];

    for path in &paths {
        if let Some(theme) = read_ini_value(path, "Icons", "Theme") {
            return theme;
        }
    }
    "hicolor".to_string()
}

/// Query gsettings for icon-theme with the given schema.
#[cfg(target_os = "linux")]
fn gsettings_icon_theme(schema: &str) -> String {
    std::process::Command::new("gsettings")
        .args(["get", schema, "icon-theme"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "hicolor".to_string())
}

/// Read icon theme from XFCE's xfconf-query.
#[cfg(target_os = "linux")]
fn detect_xfce_icon_theme() -> String {
    std::process::Command::new("xfconf-query")
        .args(["-c", "xsettings", "-p", "/Net/IconThemeName"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "hicolor".to_string())
}

/// Read icon theme from LXQt's config file.
///
/// LXQt uses a flat `key=value` format (no section headers for the icon_theme
/// key), so we scan for the bare `icon_theme=` prefix.
#[cfg(target_os = "linux")]
fn detect_lxqt_icon_theme() -> String {
    let path = xdg_config_dir().join("lxqt").join("lxqt.conf");

    if let Ok(content) = std::fs::read_to_string(&path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(value) = trimmed.strip_prefix("icon_theme=") {
                let value = value.trim();
                if !value.is_empty() {
                    return value.to_string();
                }
            }
        }
    }
    "hicolor".to_string()
}

/// Resolve `$XDG_CONFIG_HOME`, falling back to `$HOME/.config`.
#[cfg(target_os = "linux")]
fn xdg_config_dir() -> std::path::PathBuf {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME")
        && !config_home.is_empty()
    {
        return std::path::PathBuf::from(config_home);
    }
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
        .join(".config")
}

/// Read a value from an INI file by section and key.
///
/// Simple line-based parser — no external crate needed. Handles `[Section]`
/// headers and `Key=Value` lines. Returns `None` if the file doesn't exist,
/// the section/key is missing, or the value is empty.
#[cfg(target_os = "linux")]
fn read_ini_value(path: &std::path::Path, section: &str, key: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let target_section = format!("[{}]", section);
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_section = trimmed == target_section;
            continue;
        }
        if in_section && let Some(value) = trimmed.strip_prefix(key) {
            let value = value.trim_start();
            if let Some(value) = value.strip_prefix('=') {
                let value = value.trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

// --- Private mapping functions ---

#[allow(unreachable_patterns)]
fn sf_symbols_name(role: IconRole) -> Option<&'static str> {
    Some(match role {
        // Dialog / Alert
        IconRole::DialogWarning => "exclamationmark.triangle.fill",
        IconRole::DialogError => "xmark.circle.fill",
        IconRole::DialogInfo => "info.circle.fill",
        IconRole::DialogQuestion => "questionmark.circle.fill",
        IconRole::DialogSuccess => "checkmark.circle.fill",
        IconRole::Shield => "shield.fill",

        // Window Controls
        IconRole::WindowClose => "xmark",
        IconRole::WindowMinimize => "minus",
        IconRole::WindowMaximize => "arrow.up.left.and.arrow.down.right",
        IconRole::WindowRestore => "arrow.down.right.and.arrow.up.left",

        // Common Actions
        IconRole::ActionSave => "square.and.arrow.down",
        IconRole::ActionDelete => "trash",
        IconRole::ActionCopy => "doc.on.doc",
        IconRole::ActionPaste => "doc.on.clipboard",
        IconRole::ActionCut => "scissors",
        IconRole::ActionUndo => "arrow.uturn.backward",
        IconRole::ActionRedo => "arrow.uturn.forward",
        IconRole::ActionSearch => "magnifyingglass",
        IconRole::ActionSettings => "gearshape",
        IconRole::ActionEdit => "pencil",
        IconRole::ActionAdd => "plus",
        IconRole::ActionRemove => "minus",
        IconRole::ActionRefresh => "arrow.clockwise",
        IconRole::ActionPrint => "printer",

        // Navigation
        IconRole::NavBack => "chevron.backward",
        IconRole::NavForward => "chevron.forward",
        IconRole::NavUp => "chevron.up",
        IconRole::NavDown => "chevron.down",
        IconRole::NavHome => "house",
        IconRole::NavMenu => "line.horizontal.3",

        // Files / Places
        IconRole::FileGeneric => "doc",
        IconRole::FolderClosed => "folder",
        // FolderOpen: no SF Symbol equivalent
        IconRole::FolderOpen => return None,
        IconRole::TrashEmpty => "trash",
        IconRole::TrashFull => "trash.fill",

        // Status
        // StatusLoading: no static SF Symbol (loading is animated)
        IconRole::StatusLoading => return None,
        IconRole::StatusCheck => "checkmark",
        IconRole::StatusError => "xmark.circle.fill",

        // System
        IconRole::UserAccount => "person.fill",
        IconRole::Notification => "bell.fill",
        IconRole::Help => "questionmark.circle",
        IconRole::Lock => "lock.fill",

        _ => return None,
    })
}

#[allow(unreachable_patterns)]
fn segoe_name(role: IconRole) -> Option<&'static str> {
    Some(match role {
        // Dialog / Alert (SHSTOCKICONID constants)
        IconRole::DialogWarning => "SIID_WARNING",
        IconRole::DialogError => "SIID_ERROR",
        IconRole::DialogInfo => "SIID_INFO",
        IconRole::DialogQuestion => "IDI_QUESTION",
        IconRole::DialogSuccess => "CheckMark",
        IconRole::Shield => "SIID_SHIELD",

        // Window Controls (Segoe Fluent Icons glyphs)
        IconRole::WindowClose => "ChromeClose",
        IconRole::WindowMinimize => "ChromeMinimize",
        IconRole::WindowMaximize => "ChromeMaximize",
        IconRole::WindowRestore => "ChromeRestore",

        // Common Actions (mix of SHSTOCKICONID and Segoe Fluent)
        IconRole::ActionSave => "Save",
        IconRole::ActionDelete => "SIID_DELETE",
        IconRole::ActionCopy => "Copy",
        IconRole::ActionPaste => "Paste",
        IconRole::ActionCut => "Cut",
        IconRole::ActionUndo => "Undo",
        IconRole::ActionRedo => "Redo",
        IconRole::ActionSearch => "SIID_FIND",
        IconRole::ActionSettings => "SIID_SETTINGS",
        IconRole::ActionEdit => "Edit",
        IconRole::ActionAdd => "Add",
        IconRole::ActionRemove => "Remove",
        IconRole::ActionRefresh => "Refresh",
        IconRole::ActionPrint => "SIID_PRINTER",

        // Navigation (Segoe Fluent Icons)
        IconRole::NavBack => "Back",
        IconRole::NavForward => "Forward",
        IconRole::NavUp => "Up",
        IconRole::NavDown => "Down",
        IconRole::NavHome => "Home",
        IconRole::NavMenu => "GlobalNavigationButton",

        // Files / Places (SHSTOCKICONID)
        IconRole::FileGeneric => "SIID_DOCNOASSOC",
        IconRole::FolderClosed => "SIID_FOLDER",
        IconRole::FolderOpen => "SIID_FOLDEROPEN",
        IconRole::TrashEmpty => "SIID_RECYCLER",
        IconRole::TrashFull => "SIID_RECYCLERFULL",

        // Status
        // StatusLoading: no static Windows icon (progress ring is animated)
        IconRole::StatusLoading => return None,
        IconRole::StatusCheck => "CheckMark",
        IconRole::StatusError => "SIID_ERROR",

        // System
        IconRole::UserAccount => "SIID_USERS",
        IconRole::Notification => "Ringer",
        IconRole::Help => "SIID_HELP",
        IconRole::Lock => "SIID_LOCK",

        _ => return None,
    })
}

#[allow(unreachable_patterns)]
fn freedesktop_name(role: IconRole) -> Option<&'static str> {
    Some(match role {
        // Dialog / Alert
        IconRole::DialogWarning => "dialog-warning",
        IconRole::DialogError => "dialog-error",
        IconRole::DialogInfo => "dialog-information",
        IconRole::DialogQuestion => "dialog-question",
        IconRole::DialogSuccess => "emblem-ok-symbolic",
        IconRole::Shield => "security-high",

        // Window Controls
        IconRole::WindowClose => "window-close",
        IconRole::WindowMinimize => "window-minimize",
        IconRole::WindowMaximize => "window-maximize",
        IconRole::WindowRestore => "window-restore",

        // Common Actions
        IconRole::ActionSave => "document-save",
        IconRole::ActionDelete => "edit-delete",
        IconRole::ActionCopy => "edit-copy",
        IconRole::ActionPaste => "edit-paste",
        IconRole::ActionCut => "edit-cut",
        IconRole::ActionUndo => "edit-undo",
        IconRole::ActionRedo => "edit-redo",
        IconRole::ActionSearch => "edit-find",
        IconRole::ActionSettings => "preferences-system",
        IconRole::ActionEdit => "document-edit",
        IconRole::ActionAdd => "list-add",
        IconRole::ActionRemove => "list-remove",
        IconRole::ActionRefresh => "view-refresh",
        IconRole::ActionPrint => "document-print",

        // Navigation
        IconRole::NavBack => "go-previous",
        IconRole::NavForward => "go-next",
        IconRole::NavUp => "go-up",
        IconRole::NavDown => "go-down",
        IconRole::NavHome => "go-home",
        IconRole::NavMenu => "open-menu",

        // Files / Places
        IconRole::FileGeneric => "text-x-generic",
        IconRole::FolderClosed => "folder",
        IconRole::FolderOpen => "folder-open",
        IconRole::TrashEmpty => "user-trash",
        IconRole::TrashFull => "user-trash-full",

        // Status
        IconRole::StatusLoading => "process-working",
        IconRole::StatusCheck => "emblem-default",
        IconRole::StatusError => "dialog-error",

        // System
        IconRole::UserAccount => "system-users",
        // KDE convention (Breeze, Oxygen); GNOME themes return None from lookup
        IconRole::Notification => "notification-active",
        IconRole::Help => "help-browser",
        IconRole::Lock => "system-lock-screen",

        _ => return None,
    })
}

#[allow(unreachable_patterns)]
fn material_name(role: IconRole) -> Option<&'static str> {
    Some(match role {
        // Dialog / Alert
        IconRole::DialogWarning => "warning",
        IconRole::DialogError => "error",
        IconRole::DialogInfo => "info",
        IconRole::DialogQuestion => "help",
        IconRole::DialogSuccess => "check_circle",
        IconRole::Shield => "shield",

        // Window Controls
        IconRole::WindowClose => "close",
        IconRole::WindowMinimize => "minimize",
        IconRole::WindowMaximize => "open_in_full",
        IconRole::WindowRestore => "close_fullscreen",

        // Common Actions
        IconRole::ActionSave => "save",
        IconRole::ActionDelete => "delete",
        IconRole::ActionCopy => "content_copy",
        IconRole::ActionPaste => "content_paste",
        IconRole::ActionCut => "content_cut",
        IconRole::ActionUndo => "undo",
        IconRole::ActionRedo => "redo",
        IconRole::ActionSearch => "search",
        IconRole::ActionSettings => "settings",
        IconRole::ActionEdit => "edit",
        IconRole::ActionAdd => "add",
        IconRole::ActionRemove => "remove",
        IconRole::ActionRefresh => "refresh",
        IconRole::ActionPrint => "print",

        // Navigation
        IconRole::NavBack => "arrow_back",
        IconRole::NavForward => "arrow_forward",
        IconRole::NavUp => "arrow_upward",
        IconRole::NavDown => "arrow_downward",
        IconRole::NavHome => "home",
        IconRole::NavMenu => "menu",

        // Files / Places
        IconRole::FileGeneric => "description",
        IconRole::FolderClosed => "folder",
        IconRole::FolderOpen => "folder_open",
        IconRole::TrashEmpty => "delete",
        // same as TrashEmpty -- Material has no full-trash variant
        IconRole::TrashFull => "delete",

        // Status
        IconRole::StatusLoading => "progress_activity",
        IconRole::StatusCheck => "check",
        IconRole::StatusError => "error",

        // System
        IconRole::UserAccount => "person",
        IconRole::Notification => "notifications",
        IconRole::Help => "help",
        IconRole::Lock => "lock",

        _ => return None,
    })
}

#[allow(unreachable_patterns)]
fn lucide_name(role: IconRole) -> Option<&'static str> {
    Some(match role {
        // Dialog / Alert
        IconRole::DialogWarning => "triangle-alert",
        IconRole::DialogError => "circle-x",
        IconRole::DialogInfo => "info",
        IconRole::DialogQuestion => "circle-question-mark",
        IconRole::DialogSuccess => "circle-check",
        IconRole::Shield => "shield",

        // Window Controls
        IconRole::WindowClose => "x",
        IconRole::WindowMinimize => "minimize",
        IconRole::WindowMaximize => "maximize",
        IconRole::WindowRestore => "minimize-2",

        // Common Actions
        IconRole::ActionSave => "save",
        IconRole::ActionDelete => "trash-2",
        IconRole::ActionCopy => "copy",
        IconRole::ActionPaste => "clipboard-paste",
        IconRole::ActionCut => "scissors",
        IconRole::ActionUndo => "undo-2",
        IconRole::ActionRedo => "redo-2",
        IconRole::ActionSearch => "search",
        IconRole::ActionSettings => "settings",
        IconRole::ActionEdit => "pencil",
        IconRole::ActionAdd => "plus",
        IconRole::ActionRemove => "minus",
        IconRole::ActionRefresh => "refresh-cw",
        IconRole::ActionPrint => "printer",

        // Navigation
        IconRole::NavBack => "chevron-left",
        IconRole::NavForward => "chevron-right",
        IconRole::NavUp => "chevron-up",
        IconRole::NavDown => "chevron-down",
        IconRole::NavHome => "house",
        IconRole::NavMenu => "menu",

        // Files / Places
        IconRole::FileGeneric => "file",
        IconRole::FolderClosed => "folder-closed",
        IconRole::FolderOpen => "folder-open",
        IconRole::TrashEmpty => "trash-2",
        // same as TrashEmpty -- Lucide has no full-trash variant
        IconRole::TrashFull => "trash-2",

        // Status
        IconRole::StatusLoading => "loader",
        IconRole::StatusCheck => "check",
        IconRole::StatusError => "circle-x",

        // System
        IconRole::UserAccount => "user",
        IconRole::Notification => "bell",
        IconRole::Help => "circle-question-mark",
        IconRole::Lock => "lock",

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // === IconRole tests ===

    #[test]
    fn icon_role_all_has_42_variants() {
        assert_eq!(IconRole::ALL.len(), 42);
    }

    #[test]
    fn icon_role_all_contains_every_variant() {
        // Verify specific variants from each category are present
        let all = &IconRole::ALL;

        // Dialog (6)
        assert!(all.contains(&IconRole::DialogWarning));
        assert!(all.contains(&IconRole::DialogError));
        assert!(all.contains(&IconRole::DialogInfo));
        assert!(all.contains(&IconRole::DialogQuestion));
        assert!(all.contains(&IconRole::DialogSuccess));
        assert!(all.contains(&IconRole::Shield));

        // Window (4)
        assert!(all.contains(&IconRole::WindowClose));
        assert!(all.contains(&IconRole::WindowMinimize));
        assert!(all.contains(&IconRole::WindowMaximize));
        assert!(all.contains(&IconRole::WindowRestore));

        // Action (14)
        assert!(all.contains(&IconRole::ActionSave));
        assert!(all.contains(&IconRole::ActionDelete));
        assert!(all.contains(&IconRole::ActionCopy));
        assert!(all.contains(&IconRole::ActionPaste));
        assert!(all.contains(&IconRole::ActionCut));
        assert!(all.contains(&IconRole::ActionUndo));
        assert!(all.contains(&IconRole::ActionRedo));
        assert!(all.contains(&IconRole::ActionSearch));
        assert!(all.contains(&IconRole::ActionSettings));
        assert!(all.contains(&IconRole::ActionEdit));
        assert!(all.contains(&IconRole::ActionAdd));
        assert!(all.contains(&IconRole::ActionRemove));
        assert!(all.contains(&IconRole::ActionRefresh));
        assert!(all.contains(&IconRole::ActionPrint));

        // Navigation (6)
        assert!(all.contains(&IconRole::NavBack));
        assert!(all.contains(&IconRole::NavForward));
        assert!(all.contains(&IconRole::NavUp));
        assert!(all.contains(&IconRole::NavDown));
        assert!(all.contains(&IconRole::NavHome));
        assert!(all.contains(&IconRole::NavMenu));

        // Files (5)
        assert!(all.contains(&IconRole::FileGeneric));
        assert!(all.contains(&IconRole::FolderClosed));
        assert!(all.contains(&IconRole::FolderOpen));
        assert!(all.contains(&IconRole::TrashEmpty));
        assert!(all.contains(&IconRole::TrashFull));

        // Status (3)
        assert!(all.contains(&IconRole::StatusLoading));
        assert!(all.contains(&IconRole::StatusCheck));
        assert!(all.contains(&IconRole::StatusError));

        // System (4)
        assert!(all.contains(&IconRole::UserAccount));
        assert!(all.contains(&IconRole::Notification));
        assert!(all.contains(&IconRole::Help));
        assert!(all.contains(&IconRole::Lock));
    }

    #[test]
    fn icon_role_all_no_duplicates() {
        let all = &IconRole::ALL;
        for (i, role) in all.iter().enumerate() {
            for (j, other) in all.iter().enumerate() {
                if i != j {
                    assert_ne!(role, other, "Duplicate at index {i} and {j}");
                }
            }
        }
    }

    #[test]
    fn icon_role_derives_copy_clone() {
        let role = IconRole::ActionCopy;
        let copied1 = role;
        let copied2 = role;
        assert_eq!(role, copied1);
        assert_eq!(role, copied2);
    }

    #[test]
    fn icon_role_derives_debug() {
        let s = format!("{:?}", IconRole::DialogWarning);
        assert!(s.contains("DialogWarning"));
    }

    #[test]
    fn icon_role_derives_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(IconRole::ActionSave);
        set.insert(IconRole::ActionDelete);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&IconRole::ActionSave));
    }

    // === IconData tests ===

    #[test]
    fn icon_data_svg_construct_and_match() {
        let svg_bytes = b"<svg></svg>".to_vec();
        let data = IconData::Svg(svg_bytes.clone());
        match data {
            IconData::Svg(bytes) => assert_eq!(bytes, svg_bytes),
            _ => panic!("Expected Svg variant"),
        }
    }

    #[test]
    fn icon_data_rgba_construct_and_match() {
        let pixels = vec![255, 0, 0, 255]; // 1 red pixel
        let data = IconData::Rgba {
            width: 1,
            height: 1,
            data: pixels.clone(),
        };
        match data {
            IconData::Rgba {
                width,
                height,
                data,
            } => {
                assert_eq!(width, 1);
                assert_eq!(height, 1);
                assert_eq!(data, pixels);
            }
            _ => panic!("Expected Rgba variant"),
        }
    }

    #[test]
    fn icon_data_derives_debug() {
        let data = IconData::Svg(vec![]);
        let s = format!("{:?}", data);
        assert!(s.contains("Svg"));
    }

    #[test]
    fn icon_data_derives_clone() {
        let data = IconData::Rgba {
            width: 16,
            height: 16,
            data: vec![0; 16 * 16 * 4],
        };
        let cloned = data.clone();
        assert_eq!(data, cloned);
    }

    #[test]
    fn icon_data_derives_eq() {
        let a = IconData::Svg(b"<svg/>".to_vec());
        let b = IconData::Svg(b"<svg/>".to_vec());
        assert_eq!(a, b);

        let c = IconData::Svg(b"<other/>".to_vec());
        assert_ne!(a, c);
    }

    // === IconSet tests ===

    #[test]
    fn icon_set_from_name_sf_symbols() {
        assert_eq!(IconSet::from_name("sf-symbols"), Some(IconSet::SfSymbols));
    }

    #[test]
    fn icon_set_from_name_segoe_fluent() {
        assert_eq!(
            IconSet::from_name("segoe-fluent"),
            Some(IconSet::SegoeIcons)
        );
    }

    #[test]
    fn icon_set_from_name_freedesktop() {
        assert_eq!(
            IconSet::from_name("freedesktop"),
            Some(IconSet::Freedesktop)
        );
    }

    #[test]
    fn icon_set_from_name_material() {
        assert_eq!(IconSet::from_name("material"), Some(IconSet::Material));
    }

    #[test]
    fn icon_set_from_name_lucide() {
        assert_eq!(IconSet::from_name("lucide"), Some(IconSet::Lucide));
    }

    #[test]
    fn icon_set_from_name_unknown() {
        assert_eq!(IconSet::from_name("unknown"), None);
    }

    #[test]
    fn icon_set_name_sf_symbols() {
        assert_eq!(IconSet::SfSymbols.name(), "sf-symbols");
    }

    #[test]
    fn icon_set_name_segoe_fluent() {
        assert_eq!(IconSet::SegoeIcons.name(), "segoe-fluent");
    }

    #[test]
    fn icon_set_name_freedesktop() {
        assert_eq!(IconSet::Freedesktop.name(), "freedesktop");
    }

    #[test]
    fn icon_set_name_material() {
        assert_eq!(IconSet::Material.name(), "material");
    }

    #[test]
    fn icon_set_name_lucide() {
        assert_eq!(IconSet::Lucide.name(), "lucide");
    }

    #[test]
    fn icon_set_from_name_name_round_trip() {
        let sets = [
            IconSet::SfSymbols,
            IconSet::SegoeIcons,
            IconSet::Freedesktop,
            IconSet::Material,
            IconSet::Lucide,
        ];
        for set in &sets {
            let name = set.name();
            let parsed = IconSet::from_name(name);
            assert_eq!(parsed, Some(*set), "Round-trip failed for {:?}", set);
        }
    }

    #[test]
    fn icon_set_derives_copy_clone() {
        let set = IconSet::Material;
        let copied1 = set;
        let copied2 = set;
        assert_eq!(set, copied1);
        assert_eq!(set, copied2);
    }

    #[test]
    fn icon_set_derives_hash() {
        use std::collections::HashSet;
        let mut map = HashSet::new();
        map.insert(IconSet::SfSymbols);
        map.insert(IconSet::Lucide);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn icon_set_derives_debug() {
        let s = format!("{:?}", IconSet::Freedesktop);
        assert!(s.contains("Freedesktop"));
    }

    #[test]
    fn icon_set_serde_round_trip() {
        let set = IconSet::SfSymbols;
        let json = serde_json::to_string(&set).unwrap();
        let deserialized: IconSet = serde_json::from_str(&json).unwrap();
        assert_eq!(set, deserialized);
    }

    // === icon_name() tests ===

    #[test]
    fn icon_name_sf_symbols_action_copy() {
        assert_eq!(
            icon_name(IconSet::SfSymbols, IconRole::ActionCopy),
            Some("doc.on.doc")
        );
    }

    #[test]
    fn icon_name_segoe_action_copy() {
        assert_eq!(
            icon_name(IconSet::SegoeIcons, IconRole::ActionCopy),
            Some("Copy")
        );
    }

    #[test]
    fn icon_name_freedesktop_action_copy() {
        assert_eq!(
            icon_name(IconSet::Freedesktop, IconRole::ActionCopy),
            Some("edit-copy")
        );
    }

    #[test]
    fn icon_name_material_action_copy() {
        assert_eq!(
            icon_name(IconSet::Material, IconRole::ActionCopy),
            Some("content_copy")
        );
    }

    #[test]
    fn icon_name_lucide_action_copy() {
        assert_eq!(
            icon_name(IconSet::Lucide, IconRole::ActionCopy),
            Some("copy")
        );
    }

    #[test]
    fn icon_name_sf_symbols_dialog_warning() {
        assert_eq!(
            icon_name(IconSet::SfSymbols, IconRole::DialogWarning),
            Some("exclamationmark.triangle.fill")
        );
    }

    // None cases for known gaps
    #[test]
    fn icon_name_sf_symbols_folder_open_is_none() {
        assert_eq!(icon_name(IconSet::SfSymbols, IconRole::FolderOpen), None);
    }

    #[test]
    fn icon_name_sf_symbols_trash_full() {
        assert_eq!(
            icon_name(IconSet::SfSymbols, IconRole::TrashFull),
            Some("trash.fill")
        );
    }

    #[test]
    fn icon_name_sf_symbols_status_loading_is_none() {
        assert_eq!(icon_name(IconSet::SfSymbols, IconRole::StatusLoading), None);
    }

    #[test]
    fn icon_name_sf_symbols_window_restore() {
        assert_eq!(
            icon_name(IconSet::SfSymbols, IconRole::WindowRestore),
            Some("arrow.down.right.and.arrow.up.left")
        );
    }

    #[test]
    fn icon_name_segoe_dialog_success() {
        assert_eq!(
            icon_name(IconSet::SegoeIcons, IconRole::DialogSuccess),
            Some("CheckMark")
        );
    }

    #[test]
    fn icon_name_segoe_status_loading_is_none() {
        assert_eq!(
            icon_name(IconSet::SegoeIcons, IconRole::StatusLoading),
            None
        );
    }

    #[test]
    fn icon_name_freedesktop_notification() {
        assert_eq!(
            icon_name(IconSet::Freedesktop, IconRole::Notification),
            Some("notification-active")
        );
    }

    #[test]
    fn icon_name_material_trash_full() {
        assert_eq!(
            icon_name(IconSet::Material, IconRole::TrashFull),
            Some("delete")
        );
    }

    #[test]
    fn icon_name_lucide_trash_full() {
        assert_eq!(
            icon_name(IconSet::Lucide, IconRole::TrashFull),
            Some("trash-2")
        );
    }

    // Spot-check across all 5 icon sets for multiple roles
    #[test]
    fn icon_name_spot_check_dialog_error() {
        assert_eq!(
            icon_name(IconSet::SfSymbols, IconRole::DialogError),
            Some("xmark.circle.fill")
        );
        assert_eq!(
            icon_name(IconSet::SegoeIcons, IconRole::DialogError),
            Some("SIID_ERROR")
        );
        assert_eq!(
            icon_name(IconSet::Freedesktop, IconRole::DialogError),
            Some("dialog-error")
        );
        assert_eq!(
            icon_name(IconSet::Material, IconRole::DialogError),
            Some("error")
        );
        assert_eq!(
            icon_name(IconSet::Lucide, IconRole::DialogError),
            Some("circle-x")
        );
    }

    #[test]
    fn icon_name_spot_check_nav_home() {
        assert_eq!(
            icon_name(IconSet::SfSymbols, IconRole::NavHome),
            Some("house")
        );
        assert_eq!(
            icon_name(IconSet::SegoeIcons, IconRole::NavHome),
            Some("Home")
        );
        assert_eq!(
            icon_name(IconSet::Freedesktop, IconRole::NavHome),
            Some("go-home")
        );
        assert_eq!(
            icon_name(IconSet::Material, IconRole::NavHome),
            Some("home")
        );
        assert_eq!(icon_name(IconSet::Lucide, IconRole::NavHome), Some("house"));
    }

    // Count test: verify expected Some/None count for each icon set
    #[test]
    fn icon_name_sf_symbols_expected_count() {
        // SF Symbols: 42 - 2 None (FolderOpen, StatusLoading) = 40 Some
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(IconSet::SfSymbols, **r).is_some())
            .count();
        assert_eq!(some_count, 40, "SF Symbols should have 40 mappings");
    }

    #[test]
    fn icon_name_segoe_expected_count() {
        // Segoe: 42 - 1 None (StatusLoading) = 41 Some
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(IconSet::SegoeIcons, **r).is_some())
            .count();
        assert_eq!(some_count, 41, "Segoe Icons should have 41 mappings");
    }

    #[test]
    fn icon_name_freedesktop_expected_count() {
        // Freedesktop: all 42 roles mapped
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(IconSet::Freedesktop, **r).is_some())
            .count();
        assert_eq!(some_count, 42, "Freedesktop should have 42 mappings");
    }

    #[test]
    fn icon_name_material_expected_count() {
        // Material: all 42 roles mapped
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(IconSet::Material, **r).is_some())
            .count();
        assert_eq!(some_count, 42, "Material should have 42 mappings");
    }

    #[test]
    fn icon_name_lucide_expected_count() {
        // Lucide: all 42 roles mapped
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(IconSet::Lucide, **r).is_some())
            .count();
        assert_eq!(some_count, 42, "Lucide should have 42 mappings");
    }

    // === system_icon_set() tests ===

    #[test]
    #[cfg(target_os = "linux")]
    fn system_icon_set_returns_freedesktop_on_linux() {
        assert_eq!(system_icon_set(), IconSet::Freedesktop);
    }

    #[test]
    fn system_icon_theme_returns_non_empty() {
        let theme = system_icon_theme();
        assert!(
            !theme.is_empty(),
            "system_icon_theme() should return a non-empty string"
        );
    }

    // === IconProvider trait tests ===

    #[test]
    fn icon_provider_is_object_safe() {
        // Box<dyn IconProvider> must compile and be usable
        let provider: Box<dyn IconProvider> = Box::new(IconRole::ActionCopy);
        let debug_str = format!("{:?}", provider);
        assert!(
            debug_str.contains("ActionCopy"),
            "Debug should print variant name"
        );
    }

    #[test]
    fn icon_role_provider_icon_name() {
        // IconRole::ActionCopy should return "content_copy" for Material via IconProvider
        let role = IconRole::ActionCopy;
        let name = IconProvider::icon_name(&role, IconSet::Material);
        assert_eq!(name, Some("content_copy"));
    }

    #[test]
    fn icon_role_provider_icon_name_sf_symbols() {
        let role = IconRole::ActionCopy;
        let name = IconProvider::icon_name(&role, IconSet::SfSymbols);
        assert_eq!(name, Some("doc.on.doc"));
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn icon_role_provider_icon_svg_material() {
        let role = IconRole::ActionCopy;
        let svg = IconProvider::icon_svg(&role, IconSet::Material);
        assert!(svg.is_some(), "Material SVG should be Some");
        let content = std::str::from_utf8(svg.unwrap()).expect("valid UTF-8");
        assert!(content.contains("<svg"), "should contain <svg tag");
    }

    #[test]
    fn icon_role_provider_icon_svg_non_bundled() {
        // SfSymbols is not a bundled set, so icon_svg should return None
        let role = IconRole::ActionCopy;
        let svg = IconProvider::icon_svg(&role, IconSet::SfSymbols);
        assert!(svg.is_none(), "SfSymbols should not have bundled SVGs");
    }

    #[test]
    fn icon_role_provider_all_roles() {
        // All 42 IconRole variants implement IconProvider -- iterate and call icon_name
        for role in IconRole::ALL {
            // All 42 roles are mapped for Material
            let _name = IconProvider::icon_name(&role, IconSet::Material);
            // Just verifying it doesn't panic
        }
    }

    #[test]
    fn icon_provider_dyn_dispatch() {
        // Call icon_name and icon_svg through &dyn IconProvider
        let role = IconRole::ActionCopy;
        let provider: &dyn IconProvider = &role;
        let name = provider.icon_name(IconSet::Material);
        assert_eq!(name, Some("content_copy"));
        let svg = provider.icon_svg(IconSet::SfSymbols);
        assert!(svg.is_none(), "SfSymbols should not have bundled SVGs");
    }

    // === Coverage tests ===

    fn known_gaps() -> &'static [(IconSet, IconRole)] {
        &[
            (IconSet::SfSymbols, IconRole::FolderOpen),
            (IconSet::SfSymbols, IconRole::StatusLoading),
            (IconSet::SegoeIcons, IconRole::StatusLoading),
        ]
    }

    #[test]
    fn no_unexpected_icon_gaps() {
        let gaps = known_gaps();
        let system_sets = [IconSet::SfSymbols, IconSet::SegoeIcons, IconSet::Freedesktop];
        for &set in &system_sets {
            for role in IconRole::ALL {
                let is_known_gap = gaps.contains(&(set, role));
                let is_mapped = icon_name(set, role).is_some();
                if !is_known_gap {
                    assert!(
                        is_mapped,
                        "{role:?} has no mapping for {set:?} and is not in known_gaps()"
                    );
                }
            }
        }
    }

    #[test]
    #[cfg(all(feature = "material-icons", feature = "lucide-icons"))]
    fn all_roles_have_bundled_svg() {
        use crate::bundled_icon_svg;
        for set in [IconSet::Material, IconSet::Lucide] {
            for role in IconRole::ALL {
                assert!(
                    bundled_icon_svg(set, role).is_some(),
                    "{role:?} has no bundled SVG for {set:?}"
                );
            }
        }
    }
}
