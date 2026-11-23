//! Layout: the global gaps and default geometry, and the layout conventions
//! of a platform.

use std::sync::Arc;

/// The global layout metrics of a resolved theme.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LayoutMetrics {
    // The four gaps are 0.7.2's `layout.*`: each `None` where neither the platform nor the preset states it
    // (`platform-facts.md` §2.20; `macos-tahoe` states two of the four).
    /// The gap between adjacent widgets (0.7.2's `layout.widget_gap`);
    /// `None` where neither the platform nor the preset states it
    /// (`platform-facts.md` §2.20), as for the other three gaps.
    pub widget_gap: Option<f32>,
    /// The margin inside a container (0.7.2's `layout.container_margin`).
    pub container_margin: Option<f32>,
    /// The margin inside a window (0.7.2's `layout.window_margin`).
    pub window_margin: Option<f32>,
    /// The gap between sections (0.7.2's `layout.section_gap`).
    pub section_gap: Option<f32>,
    // Global default geometry (0.7.2's `defaults.border`, used by connectors for toolkit-wide theme structs)
    /// The default corner radius (0.7.2's `defaults.border.corner_radius`),
    /// for a connector's toolkit-wide theme structs.
    pub corner_radius: f32,
    /// The default large corner radius (0.7.2's
    /// `defaults.border.corner_radius_lg`).
    pub corner_radius_lg: f32,
    /// The default border width (0.7.2's `defaults.border.line_width`).
    pub border_width: f32,
    /// The default border opacity (0.7.2's `defaults.border.opacity`).
    pub border_opacity: f32,
    /// Whether shadows are drawn by default (0.7.2's
    /// `defaults.border.shadow_enabled`).
    pub shadow_enabled: bool,
    /// The pointer cursor's size, in logical pixels.
    pub cursor_size_px: Option<f32>,
    /// The pointer cursor theme.
    pub cursor_theme: Option<Arc<str>>,
}

/// Which end of a dialog's button row the primary button takes: a platform
/// convention, not styling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum DialogButtonOrder {
    /// Primary button at the trailing (right) end: GNOME, macOS, iOS.
    PrimaryRight,
    /// Primary button at the leading (left) end: Windows, KDE.
    PrimaryLeft,
}

/// Which end of an expander's header row its disclosure arrow sits at
/// (`platform-facts.md` §2.27, `arrow_side`). Leading and trailing are the
/// reading direction's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum ArrowSide {
    /// Before the title, at the row's start: KDE, macOS.
    Leading,
    /// After the title, at the row's end: GNOME, Windows.
    Trailing,
}

/// Which edge of the active tab its indicator line lies along, for tabs
/// above their pages (`platform-facts.md` §2.11, `active_indicator_side`).
/// Tabs below their pages mirror it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum TabIndicatorSide {
    /// Along the tab's top edge, away from its page: KDE.
    Top,
    /// Along the tab's bottom edge, next to its page: GNOME, Material.
    Bottom,
}
