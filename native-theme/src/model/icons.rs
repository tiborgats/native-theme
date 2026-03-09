// Icon type definitions: IconRole, IconData, IconSet
//
// These are the core icon types for the native-theme icon system.

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
        let cloned = role.clone();
        let copied = role;
        assert_eq!(role, cloned);
        assert_eq!(role, copied);
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
        let cloned = set.clone();
        let copied = set;
        assert_eq!(set, cloned);
        assert_eq!(set, copied);
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
}
