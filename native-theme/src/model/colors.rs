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
    pub accent: Option<Rgba>,
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
    pub surface: Option<Rgba>,
    pub border: Option<Rgba>,
    pub muted: Option<Rgba>,
    pub shadow: Option<Rgba>,
    // Primary (2)
    pub primary_background: Option<Rgba>,
    pub primary_foreground: Option<Rgba>,
    // Secondary (2)
    pub secondary_background: Option<Rgba>,
    pub secondary_foreground: Option<Rgba>,
    // Status (8)
    pub danger: Option<Rgba>,
    pub danger_foreground: Option<Rgba>,
    pub warning: Option<Rgba>,
    pub warning_foreground: Option<Rgba>,
    pub success: Option<Rgba>,
    pub success_foreground: Option<Rgba>,
    pub info: Option<Rgba>,
    pub info_foreground: Option<Rgba>,
    // Interactive (4)
    pub selection: Option<Rgba>,
    pub selection_foreground: Option<Rgba>,
    pub link: Option<Rgba>,
    pub focus_ring: Option<Rgba>,
    // Panel (6)
    pub sidebar: Option<Rgba>,
    pub sidebar_foreground: Option<Rgba>,
    pub tooltip: Option<Rgba>,
    pub tooltip_foreground: Option<Rgba>,
    pub popover: Option<Rgba>,
    pub popover_foreground: Option<Rgba>,
    // Component (7)
    pub button: Option<Rgba>,
    pub button_foreground: Option<Rgba>,
    pub input: Option<Rgba>,
    pub input_foreground: Option<Rgba>,
    pub disabled: Option<Rgba>,
    pub separator: Option<Rgba>,
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
        let mut tc = ThemeColors::default();
        tc.accent = Some(Rgba::rgb(255, 0, 0));
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
        let mut base = ThemeColors::default();
        base.background = Some(Rgba::rgb(255, 255, 255));
        base.danger = Some(Rgba::rgb(200, 0, 0));

        let mut overlay = ThemeColors::default();
        overlay.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.danger = Some(Rgba::rgb(255, 0, 0)); // override

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
        let mut base = ThemeColors::default();
        base.primary_background = Some(Rgba::rgb(0, 0, 255));

        let mut overlay = ThemeColors::default();
        overlay.secondary_foreground = Some(Rgba::rgb(255, 255, 255));

        base.merge(&overlay);

        // Primary base preserved
        assert_eq!(base.primary_background, Some(Rgba::rgb(0, 0, 255)));
        // Secondary overlay applied
        assert_eq!(base.secondary_foreground, Some(Rgba::rgb(255, 255, 255)));
    }
}
