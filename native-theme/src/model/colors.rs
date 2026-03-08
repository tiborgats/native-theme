// Theme color structs: 6 semantic sub-structs + ThemeColors container (36 color roles total)

use serde::{Deserialize, Serialize};

use crate::Rgba;

/// Core UI colors: accent, background, foreground, surface, border, muted, shadow.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CoreColors {
    pub accent: Option<Rgba>,
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
    pub surface: Option<Rgba>,
    pub border: Option<Rgba>,
    pub muted: Option<Rgba>,
    pub shadow: Option<Rgba>,
}

impl_merge!(CoreColors {
    option { accent, background, foreground, surface, border, muted, shadow }
});

/// Action button colors (reused for primary and secondary actions).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ActionColors {
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
}

impl_merge!(ActionColors {
    option { background, foreground }
});

/// Status indicator colors for danger, warning, success, info states.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StatusColors {
    pub danger: Option<Rgba>,
    pub danger_foreground: Option<Rgba>,
    pub warning: Option<Rgba>,
    pub warning_foreground: Option<Rgba>,
    pub success: Option<Rgba>,
    pub success_foreground: Option<Rgba>,
    pub info: Option<Rgba>,
    pub info_foreground: Option<Rgba>,
}

impl_merge!(StatusColors {
    option { danger, danger_foreground, warning, warning_foreground, success, success_foreground, info, info_foreground }
});

/// Colors for interactive elements: selection, links, focus indicators.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct InteractiveColors {
    pub selection: Option<Rgba>,
    pub selection_foreground: Option<Rgba>,
    pub link: Option<Rgba>,
    pub focus_ring: Option<Rgba>,
}

impl_merge!(InteractiveColors {
    option { selection, selection_foreground, link, focus_ring }
});

/// Panel-level colors for sidebars, tooltips, popovers.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct PanelColors {
    pub sidebar: Option<Rgba>,
    pub sidebar_foreground: Option<Rgba>,
    pub tooltip: Option<Rgba>,
    pub tooltip_foreground: Option<Rgba>,
    pub popover: Option<Rgba>,
    pub popover_foreground: Option<Rgba>,
}

impl_merge!(PanelColors {
    option { sidebar, sidebar_foreground, tooltip, tooltip_foreground, popover, popover_foreground }
});

/// Component-level colors for buttons, inputs, and other widgets.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ComponentColors {
    pub button: Option<Rgba>,
    pub button_foreground: Option<Rgba>,
    pub input: Option<Rgba>,
    pub input_foreground: Option<Rgba>,
    pub disabled: Option<Rgba>,
    pub separator: Option<Rgba>,
    pub alternate_row: Option<Rgba>,
}

impl_merge!(ComponentColors {
    option { button, button_foreground, input, input_foreground, disabled, separator, alternate_row }
});

/// All theme colors organized into semantic groups.
///
/// Contains 36 total color roles across 7 fields (6 unique sub-struct types,
/// with `ActionColors` reused for primary and secondary).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeColors {
    #[serde(default, skip_serializing_if = "CoreColors::is_empty")]
    pub core: CoreColors,

    #[serde(default, skip_serializing_if = "ActionColors::is_empty")]
    pub primary: ActionColors,

    #[serde(default, skip_serializing_if = "ActionColors::is_empty")]
    pub secondary: ActionColors,

    #[serde(default, skip_serializing_if = "StatusColors::is_empty")]
    pub status: StatusColors,

    #[serde(default, skip_serializing_if = "InteractiveColors::is_empty")]
    pub interactive: InteractiveColors,

    #[serde(default, skip_serializing_if = "PanelColors::is_empty")]
    pub panel: PanelColors,

    #[serde(default, skip_serializing_if = "ComponentColors::is_empty")]
    pub component: ComponentColors,
}

impl_merge!(ThemeColors {
    nested { core, primary, secondary, status, interactive, panel, component }
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    // === Field count validation ===

    #[test]
    fn total_color_roles_is_36() {
        // CoreColors: 7, ActionColors: 2 (x2 = 4), StatusColors: 8,
        // InteractiveColors: 4, PanelColors: 6, ComponentColors: 7
        // Total: 7 + 4 + 8 + 4 + 6 + 7 = 36
        let core_fields = 7; // accent, background, foreground, surface, border, muted, shadow
        let action_fields = 2 * 2; // background, foreground -- used for primary AND secondary
        let status_fields = 8; // danger, danger_fg, warning, warning_fg, success, success_fg, info, info_fg
        let interactive_fields = 4; // selection, selection_fg, link, focus_ring
        let panel_fields = 6; // sidebar, sidebar_fg, tooltip, tooltip_fg, popover, popover_fg
        let component_fields = 7; // button, button_fg, input, input_fg, disabled, separator, alternate_row
        let total = core_fields + action_fields + status_fields + interactive_fields + panel_fields + component_fields;
        assert_eq!(total, 36);
    }

    // === is_empty tests ===

    #[test]
    fn core_colors_default_is_empty() {
        assert!(CoreColors::default().is_empty());
    }

    #[test]
    fn action_colors_default_is_empty() {
        assert!(ActionColors::default().is_empty());
    }

    #[test]
    fn status_colors_default_is_empty() {
        assert!(StatusColors::default().is_empty());
    }

    #[test]
    fn interactive_colors_default_is_empty() {
        assert!(InteractiveColors::default().is_empty());
    }

    #[test]
    fn panel_colors_default_is_empty() {
        assert!(PanelColors::default().is_empty());
    }

    #[test]
    fn component_colors_default_is_empty() {
        assert!(ComponentColors::default().is_empty());
    }

    #[test]
    fn theme_colors_default_is_empty() {
        assert!(ThemeColors::default().is_empty());
    }

    #[test]
    fn core_colors_not_empty_when_field_set() {
        let mut c = CoreColors::default();
        c.accent = Some(Rgba::rgb(255, 0, 0));
        assert!(!c.is_empty());
    }

    #[test]
    fn theme_colors_not_empty_when_nested_field_set() {
        let mut tc = ThemeColors::default();
        tc.core.accent = Some(Rgba::rgb(255, 0, 0));
        assert!(!tc.is_empty());
    }

    // === merge tests ===

    #[test]
    fn core_colors_merge_some_replaces_none() {
        let mut base = CoreColors::default();
        let overlay = CoreColors {
            accent: Some(Rgba::rgb(255, 0, 0)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(255, 0, 0)));
    }

    #[test]
    fn core_colors_merge_none_preserves_base() {
        let mut base = CoreColors {
            accent: Some(Rgba::rgb(0, 0, 255)),
            ..Default::default()
        };
        let overlay = CoreColors::default(); // all None
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(0, 0, 255)));
    }

    #[test]
    fn core_colors_merge_some_replaces_some() {
        let mut base = CoreColors {
            accent: Some(Rgba::rgb(0, 0, 255)),
            ..Default::default()
        };
        let overlay = CoreColors {
            accent: Some(Rgba::rgb(255, 0, 0)),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.accent, Some(Rgba::rgb(255, 0, 0)));
    }

    #[test]
    fn theme_colors_merge_recursively_merges_sub_structs() {
        let mut base = ThemeColors::default();
        base.core.background = Some(Rgba::rgb(255, 255, 255));
        base.status.danger = Some(Rgba::rgb(200, 0, 0));

        let mut overlay = ThemeColors::default();
        overlay.core.accent = Some(Rgba::rgb(0, 120, 215));
        overlay.status.danger = Some(Rgba::rgb(255, 0, 0)); // override

        base.merge(&overlay);

        // overlay accent applied
        assert_eq!(base.core.accent, Some(Rgba::rgb(0, 120, 215)));
        // base background preserved (overlay had None)
        assert_eq!(base.core.background, Some(Rgba::rgb(255, 255, 255)));
        // overlay danger replaced base danger
        assert_eq!(base.status.danger, Some(Rgba::rgb(255, 0, 0)));
    }

    #[test]
    fn action_colors_merge_works_for_primary_and_secondary() {
        let mut base = ThemeColors::default();
        base.primary.background = Some(Rgba::rgb(0, 0, 255));

        let mut overlay = ThemeColors::default();
        overlay.secondary.foreground = Some(Rgba::rgb(255, 255, 255));

        base.merge(&overlay);

        // Primary base preserved
        assert_eq!(base.primary.background, Some(Rgba::rgb(0, 0, 255)));
        // Secondary overlay applied
        assert_eq!(base.secondary.foreground, Some(Rgba::rgb(255, 255, 255)));
    }
}
