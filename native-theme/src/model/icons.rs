// Icon type definitions: IconRole, IconData, IconSet
//
// These are the core icon types for the native-theme icon system.

use std::borrow::Cow;

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
/// use native_theme::theme::IconRole;
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
    /// Busy / working state indicator
    StatusBusy,
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

impl std::fmt::Display for IconRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl IconRole {
    /// The kebab-case string identifier for this icon role.
    ///
    /// Returns a stable string suitable for logs, serialization, and
    /// display. Category prefix and variant name are joined with hyphens
    /// in lowercase (e.g., `DialogWarning` -> `"dialog-warning"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::IconRole;
    ///
    /// assert_eq!(IconRole::ActionSave.name(), "action-save");
    /// assert_eq!(IconRole::DialogWarning.name(), "dialog-warning");
    /// ```
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            // Dialog (6)
            Self::DialogWarning => "dialog-warning",
            Self::DialogError => "dialog-error",
            Self::DialogInfo => "dialog-info",
            Self::DialogQuestion => "dialog-question",
            Self::DialogSuccess => "dialog-success",
            Self::Shield => "shield",
            // Window (4)
            Self::WindowClose => "window-close",
            Self::WindowMinimize => "window-minimize",
            Self::WindowMaximize => "window-maximize",
            Self::WindowRestore => "window-restore",
            // Action (14)
            Self::ActionSave => "action-save",
            Self::ActionDelete => "action-delete",
            Self::ActionCopy => "action-copy",
            Self::ActionPaste => "action-paste",
            Self::ActionCut => "action-cut",
            Self::ActionUndo => "action-undo",
            Self::ActionRedo => "action-redo",
            Self::ActionSearch => "action-search",
            Self::ActionSettings => "action-settings",
            Self::ActionEdit => "action-edit",
            Self::ActionAdd => "action-add",
            Self::ActionRemove => "action-remove",
            Self::ActionRefresh => "action-refresh",
            Self::ActionPrint => "action-print",
            // Navigation (6)
            Self::NavBack => "nav-back",
            Self::NavForward => "nav-forward",
            Self::NavUp => "nav-up",
            Self::NavDown => "nav-down",
            Self::NavHome => "nav-home",
            Self::NavMenu => "nav-menu",
            // Files (5)
            Self::FileGeneric => "file-generic",
            Self::FolderClosed => "folder-closed",
            Self::FolderOpen => "folder-open",
            Self::TrashEmpty => "trash-empty",
            Self::TrashFull => "trash-full",
            // Status (3)
            Self::StatusBusy => "status-busy",
            Self::StatusCheck => "status-check",
            Self::StatusError => "status-error",
            // System (4)
            Self::UserAccount => "user-account",
            Self::Notification => "notification",
            Self::Help => "help",
            Self::Lock => "lock",
        }
    }

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
        Self::StatusBusy,
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
/// use native_theme::theme::IconData;
/// use std::borrow::Cow;
///
/// let svg = IconData::Svg(Cow::Borrowed(b"<svg></svg>"));
/// assert_eq!(svg.bytes(), b"<svg></svg>");
///
/// let rgba = IconData::Rgba { width: 16, height: 16, data: vec![0; 16*16*4] };
/// assert_eq!(rgba.bytes().len(), 16 * 16 * 4);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[must_use]
pub enum IconData {
    /// SVG content as raw bytes (from freedesktop themes, bundled icon sets).
    ///
    /// Bundled icons use `Cow::Borrowed` for zero-copy access to compile-time
    /// embedded data. Runtime-loaded icons (freedesktop, custom providers)
    /// use `Cow::Owned`.
    Svg(Cow<'static, [u8]>),

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

impl IconData {
    /// Borrow the underlying bytes regardless of variant or ownership.
    ///
    /// For `Svg`, returns the SVG content bytes.
    /// For `Rgba`, returns the raw pixel data.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        match self {
            IconData::Svg(cow) => cow,
            IconData::Rgba { data, .. } => data,
        }
    }
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
/// use native_theme::theme::IconSet;
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
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum IconSet {
    /// Apple SF Symbols (macOS, iOS).
    SfSymbols,
    /// Microsoft Segoe Fluent Icons (Windows).
    #[serde(rename = "segoe-fluent")]
    SegoeIcons,
    /// freedesktop Icon Naming Specification (Linux).
    Freedesktop,
    /// Google Material Symbols.
    Material,
    /// Lucide Icons (fork of Feather).
    Lucide,
}

impl std::fmt::Display for IconSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl IconSet {
    /// Parse an icon set from its kebab-case string identifier.
    ///
    /// Accepts the names used in TOML configuration:
    /// `"sf-symbols"`, `"segoe-fluent"`, `"freedesktop"`, `"material"`, `"lucide"`.
    ///
    /// Returns `None` for unrecognized names.
    #[must_use]
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
    #[must_use]
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
/// the typed per-set loaders in [`crate::icons`]. The typical pattern is
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
/// use native_theme::theme::{IconProvider, IconSet};
/// use std::borrow::Cow;
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
///     fn icon_svg(&self, _set: IconSet) -> Option<Cow<'static, [u8]>> {
///         None // No bundled SVGs in this example
///     }
/// }
/// ```
pub trait IconProvider: std::fmt::Debug {
    /// Return the platform/theme-specific icon name for this icon in the given set.
    fn icon_name(&self, set: IconSet) -> Option<&str>;

    /// Return bundled SVG bytes for this icon in the given set.
    ///
    /// Bundled providers should return `Cow::Borrowed` for zero-copy access.
    /// Runtime providers (e.g. loading from disk) should return `Cow::Owned`.
    fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>>;
}

impl IconProvider for IconRole {
    fn icon_name(&self, set: IconSet) -> Option<&str> {
        icon_name(*self, set)
    }

    fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>> {
        crate::model::bundled::bundled_icon_svg(*self, set).map(Cow::Borrowed)
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
/// use native_theme::theme::{IconSet, IconRole, icon_name};
///
/// assert_eq!(icon_name(IconRole::ActionCopy, IconSet::SfSymbols), Some("doc.on.doc"));
/// assert_eq!(icon_name(IconRole::ActionCopy, IconSet::Freedesktop), Some("edit-copy"));
/// assert_eq!(icon_name(IconRole::FolderOpen, IconSet::SfSymbols), None);
/// ```
#[must_use]
#[allow(unreachable_patterns)] // wildcard arm kept for #[non_exhaustive] forward compat
pub fn icon_name(role: IconRole, set: IconSet) -> Option<&'static str> {
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
/// use native_theme::theme::{IconSet, system_icon_set};
///
/// let set = system_icon_set();
/// // On Linux, this returns Freedesktop
/// ```
#[must_use]
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

/// Detect the icon theme name for the current platform (cached).
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
/// Delegates to [`DetectionContext::icon_theme()`](crate::detect::DetectionContext::icon_theme)
/// which caches the result and supports per-field invalidation via
/// [`invalidate_caches()`](crate::detect::invalidate_caches).
///
/// # Examples
///
/// ```
/// use native_theme::theme::system_icon_theme;
///
/// let theme = system_icon_theme();
/// // On a KDE system with Breeze Dark: "breeze-dark"
/// // On macOS: "sf-symbols"
/// ```
#[must_use]
pub fn system_icon_theme() -> String {
    crate::detect::system().icon_theme().to_string()
}

/// Detect the icon theme name for the current platform without caching.
///
/// Unlike [`system_icon_theme()`], this function queries the OS every time it
/// is called and never caches the result. Use this when polling for icon theme
/// changes (e.g., the user switches from Breeze to Breeze Dark in system
/// settings).
///
/// See [`system_icon_theme()`] for platform behavior details.
#[must_use]
#[allow(unreachable_code)]
pub fn detect_icon_theme() -> String {
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
        detect_linux_icon_theme()
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
    let de = crate::detect::detect_linux_desktop();

    match de {
        crate::detect::LinuxDesktop::Kde => detect_kde_icon_theme(),
        crate::detect::LinuxDesktop::Gnome
        | crate::detect::LinuxDesktop::Budgie
        | crate::detect::LinuxDesktop::Hyprland
        | crate::detect::LinuxDesktop::Sway
        | crate::detect::LinuxDesktop::River
        | crate::detect::LinuxDesktop::Niri
        | crate::detect::LinuxDesktop::Wayfire
        | crate::detect::LinuxDesktop::CosmicDe => {
            gsettings_icon_theme("org.gnome.desktop.interface")
        }
        crate::detect::LinuxDesktop::Cinnamon => {
            gsettings_icon_theme("org.cinnamon.desktop.interface")
        }
        crate::detect::LinuxDesktop::Xfce => detect_xfce_icon_theme(),
        crate::detect::LinuxDesktop::Mate => gsettings_icon_theme("org.mate.interface"),
        crate::detect::LinuxDesktop::LxQt => detect_lxqt_icon_theme(),
        crate::detect::LinuxDesktop::Unknown => {
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
    let Some(config_dir) = xdg_config_dir() else {
        return "hicolor".to_string();
    };
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
    let Some(config_dir) = xdg_config_dir() else {
        return "hicolor".to_string();
    };
    let path = config_dir.join("lxqt").join("lxqt.conf");

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
///
/// Returns `None` when both `$XDG_CONFIG_HOME` and `$HOME` are unset,
/// avoiding a bogus `/tmp/.config` fallback.
#[cfg(target_os = "linux")]
fn xdg_config_dir() -> Option<std::path::PathBuf> {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME")
        && !config_home.is_empty()
    {
        return Some(std::path::PathBuf::from(config_home));
    }
    std::env::var("HOME")
        .ok()
        .filter(|h| !h.is_empty())
        .map(|h| std::path::PathBuf::from(h).join(".config"))
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
        // StatusBusy: no static SF Symbol (no static busy equivalent)
        IconRole::StatusBusy => return None,
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
        // StatusBusy: no static Windows icon (no static busy equivalent)
        IconRole::StatusBusy => return None,
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
        IconRole::StatusBusy => "process-working",
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
        IconRole::StatusBusy => "progress_activity",
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
        IconRole::StatusBusy => "loader",
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
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === IconRole tests ===

    #[test]
    fn icon_role_all_has_42_variants() {
        assert_eq!(IconRole::ALL.len(), 42);
    }

    #[test]
    fn icon_role_all_contains_every_variant() {
        // Exhaustive match — adding a new IconRole variant without adding it
        // here will cause a compile error (missing match arm). This is valid
        // within the defining crate despite #[non_exhaustive].
        use std::collections::HashSet;
        let all_set: HashSet<IconRole> = IconRole::ALL.iter().copied().collect();
        let check = |role: IconRole| {
            assert!(
                all_set.contains(&role),
                "IconRole::{role:?} missing from ALL array"
            );
        };

        // Exhaustive match forces compiler to catch new variants:
        #[deny(unreachable_patterns)]
        match IconRole::DialogWarning {
            IconRole::DialogWarning
            | IconRole::DialogError
            | IconRole::DialogInfo
            | IconRole::DialogQuestion
            | IconRole::DialogSuccess
            | IconRole::Shield
            | IconRole::WindowClose
            | IconRole::WindowMinimize
            | IconRole::WindowMaximize
            | IconRole::WindowRestore
            | IconRole::ActionSave
            | IconRole::ActionDelete
            | IconRole::ActionCopy
            | IconRole::ActionPaste
            | IconRole::ActionCut
            | IconRole::ActionUndo
            | IconRole::ActionRedo
            | IconRole::ActionSearch
            | IconRole::ActionSettings
            | IconRole::ActionEdit
            | IconRole::ActionAdd
            | IconRole::ActionRemove
            | IconRole::ActionRefresh
            | IconRole::ActionPrint
            | IconRole::NavBack
            | IconRole::NavForward
            | IconRole::NavUp
            | IconRole::NavDown
            | IconRole::NavHome
            | IconRole::NavMenu
            | IconRole::FileGeneric
            | IconRole::FolderClosed
            | IconRole::FolderOpen
            | IconRole::TrashEmpty
            | IconRole::TrashFull
            | IconRole::StatusBusy
            | IconRole::StatusCheck
            | IconRole::StatusError
            | IconRole::UserAccount
            | IconRole::Notification
            | IconRole::Help
            | IconRole::Lock => {}
        }

        // Verify each variant is in ALL:
        check(IconRole::DialogWarning);
        check(IconRole::DialogError);
        check(IconRole::DialogInfo);
        check(IconRole::DialogQuestion);
        check(IconRole::DialogSuccess);
        check(IconRole::Shield);
        check(IconRole::WindowClose);
        check(IconRole::WindowMinimize);
        check(IconRole::WindowMaximize);
        check(IconRole::WindowRestore);
        check(IconRole::ActionSave);
        check(IconRole::ActionDelete);
        check(IconRole::ActionCopy);
        check(IconRole::ActionPaste);
        check(IconRole::ActionCut);
        check(IconRole::ActionUndo);
        check(IconRole::ActionRedo);
        check(IconRole::ActionSearch);
        check(IconRole::ActionSettings);
        check(IconRole::ActionEdit);
        check(IconRole::ActionAdd);
        check(IconRole::ActionRemove);
        check(IconRole::ActionRefresh);
        check(IconRole::ActionPrint);
        check(IconRole::NavBack);
        check(IconRole::NavForward);
        check(IconRole::NavUp);
        check(IconRole::NavDown);
        check(IconRole::NavHome);
        check(IconRole::NavMenu);
        check(IconRole::FileGeneric);
        check(IconRole::FolderClosed);
        check(IconRole::FolderOpen);
        check(IconRole::TrashEmpty);
        check(IconRole::TrashFull);
        check(IconRole::StatusBusy);
        check(IconRole::StatusCheck);
        check(IconRole::StatusError);
        check(IconRole::UserAccount);
        check(IconRole::Notification);
        check(IconRole::Help);
        check(IconRole::Lock);
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
        let data = IconData::Svg(Cow::Borrowed(b"<svg></svg>"));
        assert_eq!(data.bytes(), b"<svg></svg>");
        match data {
            IconData::Svg(ref cow) => assert_eq!(cow.as_ref(), b"<svg></svg>"),
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
        let data = IconData::Svg(Cow::Borrowed(b""));
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
        let a = IconData::Svg(Cow::Borrowed(b"<svg/>"));
        let b = IconData::Svg(Cow::Owned(b"<svg/>".to_vec()));
        assert_eq!(a, b);

        let c = IconData::Svg(Cow::Borrowed(b"<other/>"));
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
            icon_name(IconRole::ActionCopy, IconSet::SfSymbols),
            Some("doc.on.doc")
        );
    }

    #[test]
    fn icon_name_segoe_action_copy() {
        assert_eq!(
            icon_name(IconRole::ActionCopy, IconSet::SegoeIcons),
            Some("Copy")
        );
    }

    #[test]
    fn icon_name_freedesktop_action_copy() {
        assert_eq!(
            icon_name(IconRole::ActionCopy, IconSet::Freedesktop),
            Some("edit-copy")
        );
    }

    #[test]
    fn icon_name_material_action_copy() {
        assert_eq!(
            icon_name(IconRole::ActionCopy, IconSet::Material),
            Some("content_copy")
        );
    }

    #[test]
    fn icon_name_lucide_action_copy() {
        assert_eq!(
            icon_name(IconRole::ActionCopy, IconSet::Lucide),
            Some("copy")
        );
    }

    #[test]
    fn icon_name_sf_symbols_dialog_warning() {
        assert_eq!(
            icon_name(IconRole::DialogWarning, IconSet::SfSymbols),
            Some("exclamationmark.triangle.fill")
        );
    }

    // None cases for known gaps
    #[test]
    fn icon_name_sf_symbols_folder_open_is_none() {
        assert_eq!(icon_name(IconRole::FolderOpen, IconSet::SfSymbols), None);
    }

    #[test]
    fn icon_name_sf_symbols_trash_full() {
        assert_eq!(
            icon_name(IconRole::TrashFull, IconSet::SfSymbols),
            Some("trash.fill")
        );
    }

    #[test]
    fn icon_name_sf_symbols_status_busy_is_none() {
        assert_eq!(icon_name(IconRole::StatusBusy, IconSet::SfSymbols), None);
    }

    #[test]
    fn icon_name_sf_symbols_window_restore() {
        assert_eq!(
            icon_name(IconRole::WindowRestore, IconSet::SfSymbols),
            Some("arrow.down.right.and.arrow.up.left")
        );
    }

    #[test]
    fn icon_name_segoe_dialog_success() {
        assert_eq!(
            icon_name(IconRole::DialogSuccess, IconSet::SegoeIcons),
            Some("CheckMark")
        );
    }

    #[test]
    fn icon_name_segoe_status_busy_is_none() {
        assert_eq!(icon_name(IconRole::StatusBusy, IconSet::SegoeIcons), None);
    }

    #[test]
    fn icon_name_freedesktop_notification() {
        assert_eq!(
            icon_name(IconRole::Notification, IconSet::Freedesktop),
            Some("notification-active")
        );
    }

    #[test]
    fn icon_name_material_trash_full() {
        assert_eq!(
            icon_name(IconRole::TrashFull, IconSet::Material),
            Some("delete")
        );
    }

    #[test]
    fn icon_name_lucide_trash_full() {
        assert_eq!(
            icon_name(IconRole::TrashFull, IconSet::Lucide),
            Some("trash-2")
        );
    }

    // Spot-check across all 5 icon sets for multiple roles
    #[test]
    fn icon_name_spot_check_dialog_error() {
        assert_eq!(
            icon_name(IconRole::DialogError, IconSet::SfSymbols),
            Some("xmark.circle.fill")
        );
        assert_eq!(
            icon_name(IconRole::DialogError, IconSet::SegoeIcons),
            Some("SIID_ERROR")
        );
        assert_eq!(
            icon_name(IconRole::DialogError, IconSet::Freedesktop),
            Some("dialog-error")
        );
        assert_eq!(
            icon_name(IconRole::DialogError, IconSet::Material),
            Some("error")
        );
        assert_eq!(
            icon_name(IconRole::DialogError, IconSet::Lucide),
            Some("circle-x")
        );
    }

    #[test]
    fn icon_name_spot_check_nav_home() {
        assert_eq!(
            icon_name(IconRole::NavHome, IconSet::SfSymbols),
            Some("house")
        );
        assert_eq!(
            icon_name(IconRole::NavHome, IconSet::SegoeIcons),
            Some("Home")
        );
        assert_eq!(
            icon_name(IconRole::NavHome, IconSet::Freedesktop),
            Some("go-home")
        );
        assert_eq!(
            icon_name(IconRole::NavHome, IconSet::Material),
            Some("home")
        );
        assert_eq!(icon_name(IconRole::NavHome, IconSet::Lucide), Some("house"));
    }

    // Count test: verify expected Some/None count for each icon set
    #[test]
    fn icon_name_sf_symbols_expected_count() {
        // SF Symbols: 42 - 2 None (FolderOpen, StatusBusy) = 40 Some
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::SfSymbols).is_some())
            .count();
        assert_eq!(some_count, 40, "SF Symbols should have 40 mappings");
    }

    #[test]
    fn icon_name_segoe_expected_count() {
        // Segoe: 42 - 1 None (StatusBusy) = 41 Some
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::SegoeIcons).is_some())
            .count();
        assert_eq!(some_count, 41, "Segoe Icons should have 41 mappings");
    }

    #[test]
    fn icon_name_freedesktop_expected_count() {
        // Freedesktop: all 42 roles mapped
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::Freedesktop).is_some())
            .count();
        assert_eq!(some_count, 42, "Freedesktop should have 42 mappings");
    }

    #[test]
    fn icon_name_material_expected_count() {
        // Material: all 42 roles mapped
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::Material).is_some())
            .count();
        assert_eq!(some_count, 42, "Material should have 42 mappings");
    }

    #[test]
    fn icon_name_lucide_expected_count() {
        // Lucide: all 42 roles mapped
        let some_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::Lucide).is_some())
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
        let cow = svg.unwrap();
        let content = std::str::from_utf8(&cow).expect("valid UTF-8");
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
            (IconSet::SfSymbols, IconRole::StatusBusy),
            (IconSet::SegoeIcons, IconRole::StatusBusy),
        ]
    }

    #[test]
    fn no_unexpected_icon_gaps() {
        let gaps = known_gaps();
        let system_sets = [
            IconSet::SfSymbols,
            IconSet::SegoeIcons,
            IconSet::Freedesktop,
        ];
        for &set in &system_sets {
            for role in IconRole::ALL {
                let is_known_gap = gaps.contains(&(set, role));
                let is_mapped = icon_name(role, set).is_some();
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
                    bundled_icon_svg(role, set).is_some(),
                    "{role:?} has no bundled SVG for {set:?}"
                );
            }
        }
    }

    /// Exhaustive icon_name() coverage: Material, Lucide, and Freedesktop map ALL 42,
    /// Segoe maps at least 41, SF Symbols maps at least 40.
    #[test]
    fn icon_name_exhaustive_all_sets() {
        // Material: all 42
        for role in IconRole::ALL {
            assert!(
                icon_name(role, IconSet::Material).is_some(),
                "Material must map {role:?} to Some"
            );
        }
        // Lucide: all 42
        for role in IconRole::ALL {
            assert!(
                icon_name(role, IconSet::Lucide).is_some(),
                "Lucide must map {role:?} to Some"
            );
        }
        // Freedesktop: all 42
        for role in IconRole::ALL {
            assert!(
                icon_name(role, IconSet::Freedesktop).is_some(),
                "Freedesktop must map {role:?} to Some"
            );
        }
        // Segoe: at least 41 (StatusBusy is None)
        let segoe_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::SegoeIcons).is_some())
            .count();
        assert!(
            segoe_count >= 41,
            "Segoe should map at least 41 roles, got {segoe_count}"
        );
        // SF Symbols: at least 40 (FolderOpen and StatusBusy are None)
        let sf_count = IconRole::ALL
            .iter()
            .filter(|r| icon_name(**r, IconSet::SfSymbols).is_some())
            .count();
        assert!(
            sf_count >= 40,
            "SfSymbols should map at least 40 roles, got {sf_count}"
        );
    }

    /// Verify icon_name returns non-empty strings for all mapped roles.
    #[test]
    fn icon_name_returns_nonempty_strings() {
        let all_sets = [
            IconSet::SfSymbols,
            IconSet::SegoeIcons,
            IconSet::Freedesktop,
            IconSet::Material,
            IconSet::Lucide,
        ];
        for set in all_sets {
            for role in IconRole::ALL {
                if let Some(name) = icon_name(role, set) {
                    assert!(
                        !name.is_empty(),
                        "icon_name({role:?}, {set:?}) returned empty string"
                    );
                }
            }
        }
    }

    // === IconSet drift-guard test (ICON-06) ===

    #[test]
    fn icon_set_name_round_trip() {
        // Drift-guard: if a new IconSet variant is added without updating
        // from_name(), this test fails.
        let all_sets = [
            IconSet::SfSymbols,
            IconSet::SegoeIcons,
            IconSet::Freedesktop,
            IconSet::Material,
            IconSet::Lucide,
        ];
        for set in all_sets {
            assert_eq!(
                IconSet::from_name(set.name()),
                Some(set),
                "IconSet::{:?}.name() = {:?} did not round-trip through from_name()",
                set,
                set.name()
            );
        }
    }

    // === IconRole::name() tests (ICON-07) ===

    #[test]
    fn icon_role_name_returns_kebab_case() {
        assert_eq!(IconRole::ActionSave.name(), "action-save");
        assert_eq!(IconRole::DialogWarning.name(), "dialog-warning");
        assert_eq!(IconRole::WindowClose.name(), "window-close");
        assert_eq!(IconRole::NavBack.name(), "nav-back");
        assert_eq!(IconRole::FileGeneric.name(), "file-generic");
        assert_eq!(IconRole::StatusBusy.name(), "status-busy");
        assert_eq!(IconRole::UserAccount.name(), "user-account");
        assert_eq!(IconRole::Shield.name(), "shield");
        assert_eq!(IconRole::Notification.name(), "notification");
        assert_eq!(IconRole::Help.name(), "help");
        assert_eq!(IconRole::Lock.name(), "lock");
    }

    #[test]
    fn icon_role_display_delegates_to_name() {
        for role in IconRole::ALL {
            assert_eq!(
                format!("{role}"),
                role.name(),
                "Display for IconRole::{:?} should delegate to name()",
                role
            );
        }
    }

    // === IconSet serde-vs-name cross-check (GAP-5) ===

    #[test]
    fn icon_set_serde_matches_name() {
        let all_sets = [
            IconSet::SfSymbols,
            IconSet::SegoeIcons,
            IconSet::Freedesktop,
            IconSet::Material,
            IconSet::Lucide,
        ];
        for set in all_sets {
            let serialized = serde_json::to_string(&set)
                .unwrap_or_else(|e| panic!("failed to serialize {set:?}: {e}"));
            let trimmed = serialized.trim_matches('"');
            assert_eq!(
                trimmed,
                set.name(),
                "serde serialization of {set:?} ({trimmed:?}) does not match name() ({:?})",
                set.name()
            );
        }
    }

    // === IconData::bytes() accessor test ===

    #[test]
    fn icon_data_bytes_accessor() {
        let svg = IconData::Svg(Cow::Borrowed(b"<svg/>"));
        assert_eq!(svg.bytes(), b"<svg/>");

        let rgba = IconData::Rgba {
            width: 2,
            height: 2,
            data: vec![1, 2, 3, 4],
        };
        assert_eq!(rgba.bytes(), &[1, 2, 3, 4]);
    }
}
