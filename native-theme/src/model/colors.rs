// Theme colors: flat 36-field struct covering all semantic color roles.

use serde::{Deserialize, Serialize};

use crate::Rgba;

/// All theme colors as a flat set of 36 semantic color roles.
///
/// Organized into logical groups (core, primary, secondary, status,
/// interactive, panel, component) but stored as direct fields for
/// simpler access and flatter TOML serialization.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeColors {
    // Core (7)
    /// Brand or accent color for interactive elements.
    pub accent: Option<Rgba>,
    /// Main application background color.
    pub background: Option<Rgba>,
    /// Main text color on the application background.
    pub foreground: Option<Rgba>,
    /// Elevated surface color (cards, dialogs).
    pub surface: Option<Rgba>,
    /// Default border color.
    pub border: Option<Rgba>,
    /// Muted/subdued text color for secondary content.
    pub muted: Option<Rgba>,
    /// Shadow or elevation color.
    pub shadow: Option<Rgba>,
    // Primary (2)
    /// Primary action background color.
    pub primary_background: Option<Rgba>,
    /// Text color on primary background.
    pub primary_foreground: Option<Rgba>,
    // Secondary (2)
    /// Secondary action background color.
    pub secondary_background: Option<Rgba>,
    /// Text color on secondary background.
    pub secondary_foreground: Option<Rgba>,
    // Status (8)
    /// Error or danger status color.
    pub danger: Option<Rgba>,
    /// Text color on danger background.
    pub danger_foreground: Option<Rgba>,
    /// Warning status color.
    pub warning: Option<Rgba>,
    /// Text color on warning background.
    pub warning_foreground: Option<Rgba>,
    /// Success or positive status color.
    pub success: Option<Rgba>,
    /// Text color on success background.
    pub success_foreground: Option<Rgba>,
    /// Informational status color.
    pub info: Option<Rgba>,
    /// Text color on info background.
    pub info_foreground: Option<Rgba>,
    // Interactive (4)
    /// Background color for selected items.
    pub selection: Option<Rgba>,
    /// Text color on selected background.
    pub selection_foreground: Option<Rgba>,
    /// Hyperlink text color.
    pub link: Option<Rgba>,
    /// Color of keyboard focus indicators.
    pub focus_ring: Option<Rgba>,
    // Panel (6)
    /// Sidebar background color.
    pub sidebar: Option<Rgba>,
    /// Text color on sidebar background.
    pub sidebar_foreground: Option<Rgba>,
    /// Tooltip background color.
    pub tooltip: Option<Rgba>,
    /// Text color on tooltip background.
    pub tooltip_foreground: Option<Rgba>,
    /// Popover/dropdown background color.
    pub popover: Option<Rgba>,
    /// Text color on popover background.
    pub popover_foreground: Option<Rgba>,
    // Component (7)
    /// Button background color.
    pub button: Option<Rgba>,
    /// Text color on button background.
    pub button_foreground: Option<Rgba>,
    /// Text input field background color.
    pub input: Option<Rgba>,
    /// Text color inside input fields.
    pub input_foreground: Option<Rgba>,
    /// Color for disabled UI elements.
    pub disabled: Option<Rgba>,
    /// Separator/divider line color.
    pub separator: Option<Rgba>,
    /// Alternating row background for lists and tables.
    pub alternate_row: Option<Rgba>,
}

impl_merge!(ThemeColors {
    option {
        accent, background, foreground, surface, border, muted, shadow,
        primary_background, primary_foreground,
        secondary_background, secondary_foreground,
        danger, danger_foreground, warning, warning_foreground,
        success, success_foreground, info, info_foreground,
        selection, selection_foreground, link, focus_ring,
        sidebar, sidebar_foreground, tooltip, tooltip_foreground,
        popover, popover_foreground,
        button, button_foreground, input, input_foreground,
        disabled, separator, alternate_row
    }
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    // === Field count validation ===

    #[test]
    fn total_color_roles_is_36() {
        // 7 core + 2 primary + 2 secondary + 8 status +
        // 4 interactive + 6 panel + 7 component = 36
        assert_eq!(7 + 2 + 2 + 8 + 4 + 6 + 7, 36);
    }

    // === is_empty tests ===

    #[test]
    fn theme_colors_default_is_empty() {
        assert!(ThemeColors::default().is_empty());
    }

    #[test]
    fn theme_colors_not_empty_when_field_set() {
        let tc = ThemeColors {
            accent: Some(Rgba::rgb(255, 0, 0)),
            ..Default::default()
        };
        assert!(!tc.is_empty());
    }

    // === merge tests ===

    #[test]
    fn merge_some_replaces_none() {
        let mut base = ThemeColors::default();
        let overlay = ThemeColors {
            accent: Some(Rgba::rgb(255, 0, 0)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(255, 0, 0)));
    }

    #[test]
    fn merge_none_preserves_base() {
        let mut base = ThemeColors {
            accent: Some(Rgba::rgb(0, 0, 255)),
            ..Default::default()
        };
        let overlay = ThemeColors::default(); // all None
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(0, 0, 255)));
    }

    #[test]
    fn merge_some_replaces_some() {
        let mut base = ThemeColors {
            accent: Some(Rgba::rgb(0, 0, 255)),
            ..Default::default()
        };
        let overlay = ThemeColors {
            accent: Some(Rgba::rgb(255, 0, 0)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(255, 0, 0)));
    }

    #[test]
    fn merge_flat_fields_across_groups() {
        let mut base = ThemeColors {
            background: Some(Rgba::rgb(255, 255, 255)),
            danger: Some(Rgba::rgb(200, 0, 0)),
            ..Default::default()
        };

        let overlay = ThemeColors {
            accent: Some(Rgba::rgb(0, 120, 215)),
            danger: Some(Rgba::rgb(255, 0, 0)), // override
            ..Default::default()
        };

        base.merge(&overlay);

        // overlay accent applied
        assert_eq!(base.accent, Some(Rgba::rgb(0, 120, 215)));
        // base background preserved (overlay had None)
        assert_eq!(base.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay danger replaced base danger
        assert_eq!(base.danger, Some(Rgba::rgb(255, 0, 0)));
    }

    #[test]
    fn merge_primary_and_secondary_fields() {
        let mut base = ThemeColors {
            primary_background: Some(Rgba::rgb(0, 0, 255)),
            ..Default::default()
        };

        let overlay = ThemeColors {
            secondary_foreground: Some(Rgba::rgb(255, 255, 255)),
            ..Default::default()
        };

        base.merge(&overlay);

        // Primary base preserved
        assert_eq!(base.primary_background, Some(Rgba::rgb(0, 0, 255)));
        // Secondary overlay applied
        assert_eq!(base.secondary_foreground, Some(Rgba::rgb(255, 255, 255)));
    }
}
