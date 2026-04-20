// Per-widget struct pairs: all 25 per-variant widgets + LayoutTheme use #[derive(ThemeWidget)].

use crate::Rgba;
use crate::model::border::{ResolvedBorderSpec, WidgetBorderSpec};
use crate::model::{DialogButtonOrder, FontSpec, ResolvedFontSpec};
use native_theme_derive::{ThemeFields, ThemeWidget};

// ── 2.2 Window / Application Chrome ────────────────────────────────────────

/// Window chrome: background, title bar colors, inactive states, geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full_lg", font = "title_bar_font")]
pub struct WindowTheme {
    /// Main window background fill.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Active title bar background fill.
    #[theme(inherit_from = "defaults.surface_color")]
    pub title_bar_background: Option<Rgba>,
    /// Title bar background when the window is unfocused.
    pub inactive_title_bar_background: Option<Rgba>,
    /// Title bar text color when the window is unfocused.
    pub inactive_title_bar_text_color: Option<Rgba>,
    /// Title bar font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub title_bar_font: Option<FontSpec>,
    /// Window border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.3 Button ──────────────────────────────────────────────────────────────

/// Push button: colors, sizing, spacing, geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct ButtonTheme {
    /// Default button background fill.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Primary / accent button background fill.
    #[theme(inherit_from = "defaults.accent_color")]
    pub primary_background: Option<Rgba>,
    /// Primary / accent button text/icon color.
    #[theme(inherit_from = "defaults.accent_text_color")]
    pub primary_text_color: Option<Rgba>,
    /// Minimum button width in logical pixels.
    #[serde(rename = "min_width_px")]
    #[theme(check = "non_negative")]
    pub min_width: Option<f32>,
    /// Minimum button height in logical pixels.
    #[serde(rename = "min_height_px")]
    #[theme(check = "non_negative")]
    pub min_height: Option<f32>,
    /// Space between icon and label.
    #[serde(rename = "icon_text_gap_px")]
    #[theme(check = "non_negative")]
    pub icon_text_gap: Option<f32>,
    /// Opacity multiplier when the button is disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// Button background on hover.
    #[theme(inherit_from = "defaults.background_color")]
    pub hover_background: Option<Rgba>,
    /// Button text color on hover.
    pub hover_text_color: Option<Rgba>,
    /// Button text color when pressed/active.
    pub active_text_color: Option<Rgba>,
    /// Button text color when disabled.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// Button background when pressed/active.
    #[theme(category = "soft_option")]
    pub active_background: Option<Rgba>,
    /// Button background when disabled.
    #[theme(category = "soft_option")]
    pub disabled_background: Option<Rgba>,
    /// Button label font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Button border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.4 Text Input ──────────────────────────────────────────────────────────

/// Single-line and multi-line text input fields.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct InputTheme {
    /// Input field background fill.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Placeholder text color.
    #[theme(inherit_from = "defaults.muted_color")]
    pub placeholder_color: Option<Rgba>,
    /// Text cursor (caret) color.
    pub caret_color: Option<Rgba>,
    /// Text selection highlight color.
    #[theme(inherit_from = "defaults.text_selection_background")]
    pub selection_background: Option<Rgba>,
    /// Text color inside the selection highlight.
    #[theme(inherit_from = "defaults.text_selection_color")]
    pub selection_text_color: Option<Rgba>,
    /// Minimum field height in logical pixels.
    #[serde(rename = "min_height_px")]
    #[theme(check = "non_negative")]
    pub min_height: Option<f32>,
    /// Opacity multiplier when disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// Input text color when disabled.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// Border color when the input is hovered.
    #[theme(category = "soft_option")]
    pub hover_border_color: Option<Rgba>,
    /// Border color when the input has focus.
    #[theme(category = "soft_option")]
    pub focus_border_color: Option<Rgba>,
    /// Input background when disabled.
    #[theme(category = "soft_option")]
    pub disabled_background: Option<Rgba>,
    /// Input text font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Input border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.5 Checkbox / Radio Button ────────────────────────────────────────────

/// Checkbox and radio button theme: colors, indicator, label font, border, and interactive states.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct CheckboxTheme {
    /// Checkbox background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Indicator background when checked.
    #[theme(inherit_from = "defaults.accent_color")]
    pub checked_background: Option<Rgba>,
    /// Indicator (check mark / radio dot) color.
    #[theme(inherit_from = "defaults.text_color")]
    pub indicator_color: Option<Rgba>,
    /// Indicator (check mark / radio dot) width in logical pixels.
    #[serde(rename = "indicator_width_px")]
    #[theme(check = "non_negative")]
    pub indicator_width: Option<f32>,
    /// Space between indicator and label.
    #[serde(rename = "label_gap_px")]
    #[theme(check = "non_negative")]
    pub label_gap: Option<f32>,
    /// Opacity multiplier when disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// Checkbox label text color when disabled.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// Checkbox background on hover.
    #[theme(category = "soft_option")]
    pub hover_background: Option<Rgba>,
    /// Checkbox background when disabled.
    #[theme(category = "soft_option")]
    pub disabled_background: Option<Rgba>,
    /// Indicator background when unchecked.
    #[theme(category = "soft_option")]
    pub unchecked_background: Option<Rgba>,
    /// Border color when unchecked.
    #[theme(category = "soft_option")]
    pub unchecked_border_color: Option<Rgba>,
    /// Checkbox label font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Checkbox border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.6 Menu ────────────────────────────────────────────────────────────────

/// Popup and context menu appearance.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_layer(border_kind = "none")]
#[theme_inherit(font = "font")]
pub struct MenuTheme {
    /// Menu panel background fill.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Separator line color between menu items.
    #[theme(inherit_from = "defaults.border.color")]
    pub separator_color: Option<Rgba>,
    /// Height of a single menu item row.
    #[serde(rename = "row_height_px")]
    #[theme(check = "non_negative")]
    pub row_height: Option<f32>,
    /// Space between a menu item's icon and its label.
    #[serde(rename = "icon_text_gap_px")]
    #[theme(check = "non_negative")]
    pub icon_text_gap: Option<f32>,
    /// Menu item icon size in logical pixels.
    #[serde(rename = "icon_size_px")]
    #[theme(check = "non_negative", inherit_from = "defaults.icon_sizes.toolbar")]
    pub icon_size: Option<f32>,
    /// Menu item background on hover.
    #[theme(inherit_from = "defaults.selection_background")]
    pub hover_background: Option<Rgba>,
    /// Menu item text color on hover.
    #[theme(inherit_from = "defaults.selection_text_color")]
    pub hover_text_color: Option<Rgba>,
    /// Disabled menu item text color.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// Menu item font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Menu border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.7 Tooltip ─────────────────────────────────────────────────────────────

/// Tooltip popup appearance.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct TooltipTheme {
    /// Tooltip background fill.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Maximum tooltip width before wrapping.
    #[serde(rename = "max_width_px")]
    #[theme(check = "non_negative")]
    pub max_width: Option<f32>,
    /// Tooltip font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Tooltip border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.8 Scrollbar ───────────────────────────────────────────────────────────

/// Scrollbar colors and geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
pub struct ScrollbarTheme {
    /// Scrollbar track (gutter) color.
    pub track_color: Option<Rgba>,
    /// Scrollbar thumb color.
    #[theme(inherit_from = "defaults.muted_color")]
    pub thumb_color: Option<Rgba>,
    /// Thumb color on hover.
    #[theme(inherit_from = "defaults.muted_color")]
    pub thumb_hover_color: Option<Rgba>,
    /// Scrollbar groove width in logical pixels.
    #[serde(rename = "groove_width_px")]
    #[theme(check = "non_negative")]
    pub groove_width: Option<f32>,
    /// Minimum thumb length in logical pixels.
    #[serde(rename = "min_thumb_length_px")]
    #[theme(check = "non_negative")]
    pub min_thumb_length: Option<f32>,
    /// Width of the thumb rail within the scrollbar.
    #[serde(rename = "thumb_width_px")]
    #[theme(check = "non_negative")]
    pub thumb_width: Option<f32>,
    /// Whether the scrollbar overlays content instead of taking layout space.
    pub overlay_mode: Option<bool>,
    /// Thumb color when pressed/dragging.
    #[theme(category = "soft_option")]
    pub thumb_active_color: Option<Rgba>,
}

// ── 2.9 Slider ──────────────────────────────────────────────────────────────

/// Slider control colors and geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
pub struct SliderTheme {
    /// Filled portion of the slider track.
    #[theme(inherit_from = "defaults.accent_color")]
    pub fill_color: Option<Rgba>,
    /// Unfilled track color.
    #[theme(inherit_from = "defaults.muted_color")]
    pub track_color: Option<Rgba>,
    /// Thumb (handle) color.
    #[theme(inherit_from = "defaults.surface_color")]
    pub thumb_color: Option<Rgba>,
    /// Track height in logical pixels.
    #[serde(rename = "track_height_px")]
    #[theme(check = "non_negative")]
    pub track_height: Option<f32>,
    /// Thumb diameter in logical pixels.
    #[serde(rename = "thumb_diameter_px")]
    #[theme(check = "non_negative")]
    pub thumb_diameter: Option<f32>,
    /// Tick mark length in logical pixels.
    #[serde(rename = "tick_mark_length_px")]
    #[theme(check = "non_negative")]
    pub tick_mark_length: Option<f32>,
    /// Opacity multiplier when disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// Thumb color on hover.
    #[theme(category = "soft_option")]
    pub thumb_hover_color: Option<Rgba>,
    /// Filled track color when disabled.
    #[theme(category = "soft_option")]
    pub disabled_fill_color: Option<Rgba>,
    /// Unfilled track color when disabled.
    #[theme(category = "soft_option")]
    pub disabled_track_color: Option<Rgba>,
    /// Thumb color when disabled.
    #[theme(category = "soft_option")]
    pub disabled_thumb_color: Option<Rgba>,
}

// ── 2.10 Progress Bar ───────────────────────────────────────────────────────

/// Progress bar colors and geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full")]
pub struct ProgressBarTheme {
    /// Filled progress bar color.
    #[theme(inherit_from = "defaults.accent_color")]
    pub fill_color: Option<Rgba>,
    /// Background track color.
    #[theme(inherit_from = "defaults.muted_color")]
    pub track_color: Option<Rgba>,
    /// Bar height in logical pixels.
    #[serde(rename = "track_height_px")]
    #[theme(check = "non_negative")]
    pub track_height: Option<f32>,
    /// Minimum bar width in logical pixels.
    #[serde(rename = "min_width_px")]
    #[theme(check = "non_negative")]
    pub min_width: Option<f32>,
    /// Progress bar border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.11 Tab Bar ─────────────────────────────────────────────────────────────

/// Tab bar colors and sizing.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_layer(border_kind = "none")]
#[theme_inherit(font = "font")]
pub struct TabTheme {
    /// Inactive tab background.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Active (selected) tab background.
    #[theme(inherit_from = "defaults.background_color")]
    pub active_background: Option<Rgba>,
    /// Active (selected) tab text color.
    #[theme(inherit_from = "defaults.text_color")]
    pub active_text_color: Option<Rgba>,
    /// Tab bar strip background.
    #[theme(inherit_from = "defaults.background_color")]
    pub bar_background: Option<Rgba>,
    /// Minimum tab width in logical pixels.
    #[serde(rename = "min_width_px")]
    #[theme(check = "non_negative")]
    pub min_width: Option<f32>,
    /// Minimum tab height in logical pixels.
    #[serde(rename = "min_height_px")]
    #[theme(check = "non_negative")]
    pub min_height: Option<f32>,
    /// Tab text color on hover.
    pub hover_text_color: Option<Rgba>,
    /// Tab background on hover.
    #[theme(category = "soft_option")]
    pub hover_background: Option<Rgba>,
    /// Tab font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Tab border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.12 Sidebar ─────────────────────────────────────────────────────────────

/// Sidebar panel background, selection, and hover colors.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_layer(border_kind = "partial")]
#[theme_inherit(border_kind = "partial", font = "font")]
pub struct SidebarTheme {
    /// Sidebar panel background fill.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Selected item background color.
    #[theme(inherit_from = "defaults.selection_background")]
    pub selection_background: Option<Rgba>,
    /// Selected item text color.
    #[theme(inherit_from = "defaults.selection_text_color")]
    pub selection_text_color: Option<Rgba>,
    /// Hovered item background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub hover_background: Option<Rgba>,
    /// Sidebar font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Sidebar border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.13 Toolbar ─────────────────────────────────────────────────────────────

/// Toolbar sizing, spacing, and font.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct ToolbarTheme {
    /// Toolbar background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Toolbar height in logical pixels.
    #[serde(rename = "bar_height_px")]
    #[theme(check = "non_negative")]
    pub bar_height: Option<f32>,
    /// Horizontal space between toolbar items.
    #[serde(rename = "item_gap_px")]
    #[theme(check = "non_negative")]
    pub item_gap: Option<f32>,
    /// Toolbar icon size in logical pixels.
    #[serde(rename = "icon_size_px")]
    #[theme(check = "non_negative", inherit_from = "defaults.icon_sizes.toolbar")]
    pub icon_size: Option<f32>,
    /// Toolbar label font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Toolbar border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.14 Status Bar ──────────────────────────────────────────────────────────

/// Status bar font and background.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_layer(border_kind = "partial")]
#[theme_inherit(border_kind = "partial", font = "font")]
pub struct StatusBarTheme {
    /// Status bar background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Status bar font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Status bar border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.15 List / Table ────────────────────────────────────────────────────────

/// List and table colors and row geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "item_font")]
#[theme_inherit(font = "header_font")]
pub struct ListTheme {
    /// List background fill.
    pub background_color: Option<Rgba>,
    /// Alternate row background for striped lists.
    pub alternate_row_background: Option<Rgba>,
    /// Selected row highlight color.
    #[theme(inherit_from = "defaults.selection_background")]
    pub selection_background: Option<Rgba>,
    /// Text color inside a selected row.
    #[theme(inherit_from = "defaults.selection_text_color")]
    pub selection_text_color: Option<Rgba>,
    /// Column header background fill.
    #[theme(inherit_from = "defaults.surface_color")]
    pub header_background: Option<Rgba>,
    /// Grid line color between rows/columns.
    #[theme(inherit_from = "defaults.border.color")]
    pub grid_color: Option<Rgba>,
    /// Row height in logical pixels.
    #[serde(rename = "row_height_px")]
    #[theme(check = "non_negative")]
    pub row_height: Option<f32>,
    /// Hovered row background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub hover_background: Option<Rgba>,
    /// Hovered row text color.
    pub hover_text_color: Option<Rgba>,
    /// Disabled row text color.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// List item font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub item_font: Option<FontSpec>,
    /// Column header font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub header_font: Option<FontSpec>,
    /// List border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.16 Popover / Dropdown ──────────────────────────────────────────────────

/// Popover / dropdown panel appearance.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full_lg", font = "font")]
pub struct PopoverTheme {
    /// Panel background fill.
    pub background_color: Option<Rgba>,
    /// Popover font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Popover border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.17 Splitter ────────────────────────────────────────────────────────────

/// Splitter handle width and color.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
pub struct SplitterTheme {
    /// Handle width in logical pixels.
    #[serde(rename = "divider_width_px")]
    #[theme(check = "non_negative")]
    pub divider_width: Option<f32>,
    /// Divider color.
    #[theme(inherit_from = "defaults.border.color")]
    pub divider_color: Option<Rgba>,
    /// Divider color on hover.
    pub hover_color: Option<Rgba>,
}

// ── 2.18 Separator ───────────────────────────────────────────────────────────

/// Separator line color and width.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
pub struct SeparatorTheme {
    /// Separator line color.
    #[theme(inherit_from = "defaults.border.color")]
    pub line_color: Option<Rgba>,
    /// Separator line width in logical pixels.
    #[serde(rename = "line_width_px")]
    #[theme(check = "non_negative", inherit_from = "defaults.border.line_width")]
    pub line_width: Option<f32>,
}

// ── 2.21 Switch / Toggle ─────────────────────────────────────────────────────

/// Toggle switch track, thumb, geometry, and interactive states.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
pub struct SwitchTheme {
    /// Track background when the switch is on.
    #[theme(inherit_from = "defaults.accent_color")]
    pub checked_background: Option<Rgba>,
    /// Track background when the switch is off.
    pub unchecked_background: Option<Rgba>,
    /// Thumb (knob) color.
    #[theme(inherit_from = "defaults.surface_color")]
    pub thumb_background: Option<Rgba>,
    /// Track width in logical pixels.
    #[serde(rename = "track_width_px")]
    #[theme(check = "non_negative")]
    pub track_width: Option<f32>,
    /// Track height in logical pixels.
    #[serde(rename = "track_height_px")]
    #[theme(check = "non_negative")]
    pub track_height: Option<f32>,
    /// Thumb diameter in logical pixels.
    #[serde(rename = "thumb_diameter_px")]
    #[theme(check = "non_negative")]
    pub thumb_diameter: Option<f32>,
    /// Track corner radius in logical pixels.
    #[serde(rename = "track_radius_px")]
    #[theme(check = "non_negative")]
    pub track_radius: Option<f32>,
    /// Opacity multiplier when disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// Track hover color when checked (on).
    #[theme(category = "soft_option")]
    pub hover_checked_background: Option<Rgba>,
    /// Track hover color when unchecked (off).
    #[theme(category = "soft_option")]
    pub hover_unchecked_background: Option<Rgba>,
    /// Track color when disabled and checked.
    #[theme(category = "soft_option")]
    pub disabled_checked_background: Option<Rgba>,
    /// Track color when disabled and unchecked.
    #[theme(category = "soft_option")]
    pub disabled_unchecked_background: Option<Rgba>,
    /// Thumb color when disabled.
    #[theme(category = "soft_option")]
    pub disabled_thumb_color: Option<Rgba>,
}

// ── 2.22 Dialog ──────────────────────────────────────────────────────────────

/// Dialog sizing, spacing, button order, fonts, border, and background.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full_lg", font = "title_font")]
#[theme_inherit(font = "body_font")]
pub struct DialogTheme {
    /// Dialog background color.
    pub background_color: Option<Rgba>,
    /// Minimum dialog width in logical pixels.
    #[serde(rename = "min_width_px")]
    #[theme(check = "non_negative", min_max_pair = "max_width")]
    pub min_width: Option<f32>,
    /// Maximum dialog width in logical pixels.
    #[serde(rename = "max_width_px")]
    #[theme(check = "non_negative")]
    pub max_width: Option<f32>,
    /// Minimum dialog height in logical pixels.
    #[serde(rename = "min_height_px")]
    #[theme(check = "non_negative", min_max_pair = "max_height")]
    pub min_height: Option<f32>,
    /// Maximum dialog height in logical pixels.
    #[serde(rename = "max_height_px")]
    #[theme(check = "non_negative")]
    pub max_height: Option<f32>,
    /// Horizontal space between dialog buttons.
    #[serde(rename = "button_gap_px")]
    #[theme(check = "non_negative")]
    pub button_gap: Option<f32>,
    /// Icon size for dialog type icons (warning, error, etc.).
    #[serde(rename = "icon_size_px")]
    #[theme(check = "non_negative")]
    pub icon_size: Option<f32>,
    /// Platform button order convention (e.g., OK/Cancel vs Cancel/OK).
    pub button_order: Option<DialogButtonOrder>,
    /// Dialog title font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub title_font: Option<FontSpec>,
    /// Dialog body font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub body_font: Option<FontSpec>,
    /// Dialog border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.23 Spinner / Progress Ring ─────────────────────────────────────────────

/// Spinner / indeterminate progress indicator.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
pub struct SpinnerTheme {
    /// Spinner arc fill color.
    pub fill_color: Option<Rgba>,
    /// Spinner outer diameter in logical pixels.
    #[serde(rename = "diameter_px")]
    #[theme(check = "non_negative")]
    pub diameter: Option<f32>,
    /// Minimum rendered size in logical pixels.
    #[serde(rename = "min_diameter_px")]
    #[theme(check = "non_negative")]
    pub min_diameter: Option<f32>,
    /// Arc stroke width in logical pixels.
    #[serde(rename = "stroke_width_px")]
    #[theme(check = "non_negative")]
    pub stroke_width: Option<f32>,
}

// ── 2.24 ComboBox / Dropdown Trigger ─────────────────────────────────────────

/// ComboBox / dropdown trigger sizing.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct ComboBoxTheme {
    /// ComboBox background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Minimum trigger height in logical pixels.
    #[serde(rename = "min_height_px")]
    #[theme(check = "non_negative")]
    pub min_height: Option<f32>,
    /// Minimum trigger width in logical pixels.
    #[serde(rename = "min_width_px")]
    #[theme(check = "non_negative")]
    pub min_width: Option<f32>,
    /// Dropdown arrow size in logical pixels.
    #[serde(rename = "arrow_icon_size_px")]
    #[theme(check = "non_negative")]
    pub arrow_icon_size: Option<f32>,
    /// Width of the arrow clickable area.
    #[serde(rename = "arrow_area_width_px")]
    #[theme(check = "non_negative")]
    pub arrow_area_width: Option<f32>,
    /// Opacity multiplier when disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// ComboBox text color when disabled.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// ComboBox background on hover.
    #[theme(category = "soft_option")]
    pub hover_background: Option<Rgba>,
    /// ComboBox background when disabled.
    #[theme(category = "soft_option")]
    pub disabled_background: Option<Rgba>,
    /// ComboBox font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// ComboBox border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.25 Segmented Control ───────────────────────────────────────────────────

/// Segmented control sizing (macOS-primary; KDE uses tab bar metrics as proxy).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct SegmentedControlTheme {
    /// Segmented control background color.
    #[theme(inherit_from = "defaults.background_color")]
    pub background_color: Option<Rgba>,
    /// Active segment background.
    #[theme(inherit_from = "defaults.accent_color")]
    pub active_background: Option<Rgba>,
    /// Active segment text color.
    #[theme(inherit_from = "defaults.accent_text_color")]
    pub active_text_color: Option<Rgba>,
    /// Segment height in logical pixels.
    #[serde(rename = "segment_height_px")]
    #[theme(check = "non_negative")]
    pub segment_height: Option<f32>,
    /// Width of the separator between segments.
    #[serde(rename = "separator_width_px")]
    #[theme(check = "non_negative")]
    pub separator_width: Option<f32>,
    /// Opacity multiplier when disabled (0.0-1.0).
    #[theme(range = "0.0..=1.0", inherit_from = "defaults.disabled_opacity")]
    pub disabled_opacity: Option<f32>,
    /// Segment background on hover.
    #[theme(category = "soft_option")]
    pub hover_background: Option<Rgba>,
    /// Segmented control font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Segmented control border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.26 Card / Container ────────────────────────────────────────────────────

/// Card / container colors and geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_layer(border_kind = "none")]
pub struct CardTheme {
    /// Card background fill.
    #[theme(inherit_from = "defaults.surface_color")]
    pub background_color: Option<Rgba>,
    /// Card border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.27 Expander / Disclosure ───────────────────────────────────────────────

/// Expander / disclosure row geometry.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(border_kind = "full", font = "font")]
pub struct ExpanderTheme {
    /// Collapsed header row height in logical pixels.
    #[serde(rename = "header_height_px")]
    #[theme(check = "non_negative")]
    pub header_height: Option<f32>,
    /// Disclosure arrow size in logical pixels.
    #[serde(rename = "arrow_icon_size_px")]
    #[theme(check = "non_negative")]
    pub arrow_icon_size: Option<f32>,
    /// Expander header background on hover.
    #[theme(category = "soft_option")]
    pub hover_background: Option<Rgba>,
    /// Disclosure arrow/chevron color.
    #[theme(category = "soft_option")]
    pub arrow_color: Option<Rgba>,
    /// Expander font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
    /// Expander border specification.
    #[theme(nested, resolved_type = "ResolvedBorderSpec")]
    pub border: Option<WidgetBorderSpec>,
}

// ── 2.28 Link ────────────────────────────────────────────────────────────────

/// Hyperlink colors and underline setting.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_inherit(font = "font")]
pub struct LinkTheme {
    /// Visited link text color.
    #[theme(inherit_from = "defaults.link_color")]
    pub visited_text_color: Option<Rgba>,
    /// Whether links are underlined.
    pub underline_enabled: Option<bool>,
    /// Link background fill (typically transparent).
    pub background_color: Option<Rgba>,
    /// Link background on hover.
    pub hover_background: Option<Rgba>,
    /// Link text color on hover.
    pub hover_text_color: Option<Rgba>,
    /// Link text color when pressed/active.
    pub active_text_color: Option<Rgba>,
    /// Link text color when disabled.
    #[theme(inherit_from = "defaults.disabled_text_color")]
    pub disabled_text_color: Option<Rgba>,
    /// Link font specification.
    #[theme(nested, resolved_type = "ResolvedFontSpec")]
    pub font: Option<FontSpec>,
}

// -- Layout (top-level, not per-variant) ------------------------------------------

/// Layout spacing constants shared between light and dark variants.
///
/// Unlike other widget themes, LayoutTheme lives on [`crate::Theme`] (top-level)
/// rather than [`crate::ThemeMode`] because spacing is variant-independent.
// Phase 93-05 G5: LayoutTheme keeps `skip_inventory` so it does NOT register
// in the per-variant WidgetFieldInfo registry (it is top-level, not per-variant),
// but also derives ThemeFields so lint_toml can look up its fields in the
// non-widget FieldInfo registry under the key "LayoutTheme".
#[derive(
    Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget, ThemeFields,
)]
#[serde_with::skip_serializing_none]
#[serde(default)]
#[theme_layer(skip_inventory)]
pub struct LayoutTheme {
    /// Space between adjacent widgets in logical pixels.
    #[serde(rename = "widget_gap_px")]
    #[theme(check = "non_negative")]
    pub widget_gap: Option<f32>,
    /// Padding inside containers in logical pixels.
    #[serde(rename = "container_margin_px")]
    #[theme(check = "non_negative")]
    pub container_margin: Option<f32>,
    /// Padding inside the main window in logical pixels.
    #[serde(rename = "window_margin_px")]
    #[theme(check = "non_negative")]
    pub window_margin: Option<f32>,
    /// Space between major content sections in logical pixels.
    #[serde(rename = "section_gap_px")]
    #[theme(check = "non_negative")]
    pub section_gap: Option<f32>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, dead_code)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::border::{ResolvedBorderSpec, WidgetBorderSpec};
    use crate::model::font::FontSize;
    use crate::model::{DialogButtonOrder, FontSpec};

    // Test widget using derive (validates derive macro works in test context)
    /// Test widget for macro verification.
    #[derive(
        Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget,
    )]
    #[serde_with::skip_serializing_none]
    #[serde(default)]
    #[theme_layer(skip_inventory)]
    pub struct TestWidget {
        pub size: Option<f32>,
        pub label: Option<String>,
        #[theme(nested, resolved_type = "ResolvedFontSpec")]
        pub font: Option<FontSpec>,
    }

    // === ResolvedFontSpec tests ===

    #[test]
    fn resolved_font_spec_fields_are_concrete() {
        let rfs = ResolvedFontSpec {
            family: "Inter".into(),
            size: 14.0,
            weight: 400,
            style: crate::model::font::FontStyle::Normal,
            color: crate::Rgba::rgb(0, 0, 0),
        };
        assert_eq!(rfs.family.as_ref(), "Inter");
        assert_eq!(rfs.size, 14.0);
        assert_eq!(rfs.weight, 400);
    }

    // === derive(ThemeWidget) generated struct tests ===

    #[test]
    fn generated_option_struct_has_option_fields() {
        let w = TestWidget::default();
        assert!(w.size.is_none());
        assert!(w.label.is_none());
        assert!(w.font.is_none());
    }

    #[test]
    fn generated_option_struct_is_empty_by_default() {
        assert!(TestWidget::default().is_empty());
    }

    #[test]
    fn generated_option_struct_not_empty_when_size_set() {
        let w = TestWidget {
            size: Some(24.0),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    #[test]
    fn generated_option_struct_not_empty_when_font_set() {
        let w = TestWidget {
            font: Some(FontSpec {
                size: Some(FontSize::Px(14.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    #[test]
    fn generated_resolved_struct_has_concrete_fields() {
        let resolved = ResolvedTestWidget {
            size: 24.0,
            label: "Click me".into(),
            font: ResolvedFontSpec {
                family: "Inter".into(),
                size: 14.0,
                weight: 400,
                style: crate::model::font::FontStyle::Normal,
                color: crate::Rgba::rgb(0, 0, 0),
            },
        };
        assert_eq!(resolved.size, 24.0);
        assert_eq!(resolved.label, "Click me");
        assert_eq!(resolved.font.family.as_ref(), "Inter");
    }

    // === merge tests for generated structs ===

    #[test]
    fn generated_merge_option_field_overlay_wins() {
        let mut base = TestWidget {
            size: Some(20.0),
            ..Default::default()
        };
        let overlay = TestWidget {
            size: Some(24.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.size, Some(24.0));
    }

    #[test]
    fn generated_merge_option_field_none_preserves_base() {
        let mut base = TestWidget {
            size: Some(20.0),
            ..Default::default()
        };
        let overlay = TestWidget::default();
        base.merge(&overlay);
        assert_eq!(base.size, Some(20.0));
    }

    #[test]
    fn generated_merge_optional_nested_both_some_merges_inner() {
        let mut base = TestWidget {
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                size: Some(FontSize::Px(12.0)),
                weight: None,
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = TestWidget {
            font: Some(FontSpec {
                family: None,
                size: None,
                weight: Some(700),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(font.size, Some(FontSize::Px(12.0))); // preserved
        assert_eq!(font.weight, Some(700)); // overlay sets
    }

    #[test]
    fn generated_merge_optional_nested_none_plus_some_clones() {
        let mut base = TestWidget::default();
        let overlay = TestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(FontSize::Px(14.0)),
                weight: Some(400),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Inter"));
        assert_eq!(font.size, Some(FontSize::Px(14.0)));
        assert_eq!(font.weight, Some(400));
    }

    #[test]
    fn generated_merge_optional_nested_some_plus_none_preserves_base() {
        let mut base = TestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(FontSize::Px(14.0)),
                weight: Some(400),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = TestWidget::default();
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Inter"));
    }

    #[test]
    fn generated_merge_optional_nested_none_plus_none_stays_none() {
        let mut base = TestWidget::default();
        let overlay = TestWidget::default();
        base.merge(&overlay);
        assert!(base.font.is_none());
    }

    // === impl_merge! optional_nested clause direct tests ===

    // Verify the optional_nested clause directly on a FontSpec-containing struct
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct WithFont {
        name: Option<String>,
        font: Option<FontSpec>,
    }

    impl_merge!(WithFont {
        option { name }
        optional_nested { font }
    });

    #[test]
    fn impl_merge_optional_nested_none_none_stays_none() {
        let mut base = WithFont::default();
        let overlay = WithFont::default();
        base.merge(&overlay);
        assert!(base.font.is_none());
    }

    #[test]
    fn impl_merge_optional_nested_some_none_preserves_base() {
        let mut base = WithFont {
            font: Some(FontSpec {
                size: Some(FontSize::Px(12.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = WithFont::default();
        base.merge(&overlay);
        assert_eq!(base.font.as_ref().unwrap().size, Some(FontSize::Px(12.0)));
    }

    #[test]
    fn impl_merge_optional_nested_none_some_clones_overlay() {
        let mut base = WithFont::default();
        let overlay = WithFont {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.font.as_ref().unwrap().family.as_deref(), Some("Inter"));
    }

    #[test]
    fn impl_merge_optional_nested_some_some_merges_inner() {
        let mut base = WithFont {
            font: Some(FontSpec {
                family: Some("Noto".into()),
                size: Some(FontSize::Px(11.0)),
                weight: None,
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = WithFont {
            font: Some(FontSpec {
                family: None,
                size: Some(FontSize::Px(14.0)),
                weight: Some(400),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let f = base.font.as_ref().unwrap();
        assert_eq!(f.family.as_deref(), Some("Noto")); // preserved
        assert_eq!(f.size, Some(FontSize::Px(14.0))); // overlay wins
        assert_eq!(f.weight, Some(400)); // overlay sets
    }

    #[test]
    fn impl_merge_optional_nested_is_empty_none() {
        let w = WithFont::default();
        assert!(w.is_empty());
    }

    #[test]
    fn impl_merge_optional_nested_is_empty_some_default() {
        // Some(FontSpec::default()) with all-None sub-fields counts as empty (D-2 fix).
        let w = WithFont {
            font: Some(FontSpec::default()),
            ..Default::default()
        };
        assert!(w.is_empty());
    }

    #[test]
    fn impl_merge_optional_nested_is_not_empty_when_populated() {
        let w = WithFont {
            font: Some(FontSpec {
                size: Some(FontSize::Px(14.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    // === ButtonTheme tests ===

    #[test]
    fn button_theme_default_is_empty() {
        assert!(ButtonTheme::default().is_empty());
    }

    #[test]
    fn button_theme_not_empty_when_set() {
        let b = ButtonTheme {
            background_color: Some(Rgba::rgb(200, 200, 200)),
            min_width: Some(64.0),
            ..Default::default()
        };
        assert!(!b.is_empty());
    }

    #[test]
    fn button_theme_merge_font_optional_nested() {
        let mut base = ButtonTheme {
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                size: Some(FontSize::Px(11.0)),
                weight: None,
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = ButtonTheme {
            font: Some(FontSpec {
                family: None,
                weight: Some(700),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let f = base.font.as_ref().unwrap();
        assert_eq!(f.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(f.weight, Some(700)); // overlay
    }

    #[test]
    fn button_theme_toml_round_trip_with_font_and_border() {
        let b = ButtonTheme {
            background_color: Some(Rgba::rgb(200, 200, 200)),
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(FontSize::Px(14.0)),
                weight: Some(400),
                ..Default::default()
            }),
            border: Some(WidgetBorderSpec {
                corner_radius: Some(4.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        let toml_str = toml::to_string(&b).unwrap();
        let b2: ButtonTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(b, b2);
    }

    // === WindowTheme tests ===

    #[test]
    fn window_theme_has_new_fields() {
        let w = WindowTheme {
            inactive_title_bar_background: Some(Rgba::rgb(180, 180, 180)),
            inactive_title_bar_text_color: Some(Rgba::rgb(120, 120, 120)),
            title_bar_font: Some(FontSpec {
                weight: Some(700),
                ..Default::default()
            }),
            border: Some(WidgetBorderSpec {
                corner_radius: Some(4.0),
                shadow_enabled: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
        assert!(w.inactive_title_bar_background.is_some());
        assert!(w.inactive_title_bar_text_color.is_some());
        assert!(w.title_bar_font.is_some());
        assert!(w.border.is_some());
    }

    #[test]
    fn window_theme_default_is_empty() {
        assert!(WindowTheme::default().is_empty());
    }

    // === DialogTheme tests ===

    #[test]
    fn dialog_theme_button_order_works() {
        let d = DialogTheme {
            button_order: Some(DialogButtonOrder::PrimaryRight),
            min_width: Some(300.0),
            ..Default::default()
        };
        assert_eq!(d.button_order, Some(DialogButtonOrder::PrimaryRight));
        assert_eq!(d.min_width, Some(300.0));
        assert!(!d.is_empty());
    }

    #[test]
    fn dialog_theme_button_order_toml_round_trip() {
        let d = DialogTheme {
            button_order: Some(DialogButtonOrder::PrimaryLeft),
            ..Default::default()
        };
        let toml_str = toml::to_string(&d).unwrap();
        let d2: DialogTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(d, d2);
    }

    #[test]
    fn dialog_theme_default_is_empty() {
        assert!(DialogTheme::default().is_empty());
    }

    // === SplitterTheme tests ===

    #[test]
    fn splitter_theme_single_field_merge() {
        let mut base = SplitterTheme {
            divider_width: Some(4.0),
            ..Default::default()
        };
        let overlay = SplitterTheme {
            divider_width: Some(6.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.divider_width, Some(6.0));
    }

    #[test]
    fn splitter_theme_merge_none_preserves_base() {
        let mut base = SplitterTheme {
            divider_width: Some(4.0),
            ..Default::default()
        };
        let overlay = SplitterTheme::default();
        base.merge(&overlay);
        assert_eq!(base.divider_width, Some(4.0));
    }

    #[test]
    fn splitter_theme_default_is_empty() {
        assert!(SplitterTheme::default().is_empty());
    }

    #[test]
    fn splitter_theme_not_empty_when_set() {
        assert!(
            !SplitterTheme {
                divider_width: Some(4.0),
                ..Default::default()
            }
            .is_empty()
        );
    }

    // === SeparatorTheme tests ===

    #[test]
    fn separator_theme_single_field() {
        let s = SeparatorTheme {
            line_color: Some(Rgba::rgb(200, 200, 200)),
            ..Default::default()
        };
        assert!(!s.is_empty());
    }

    // === All 25 widget theme defaults are empty ===

    #[test]
    fn all_widget_theme_defaults_are_empty() {
        assert!(WindowTheme::default().is_empty());
        assert!(ButtonTheme::default().is_empty());
        assert!(InputTheme::default().is_empty());
        assert!(CheckboxTheme::default().is_empty());
        assert!(MenuTheme::default().is_empty());
        assert!(TooltipTheme::default().is_empty());
        assert!(ScrollbarTheme::default().is_empty());
        assert!(SliderTheme::default().is_empty());
        assert!(ProgressBarTheme::default().is_empty());
        assert!(TabTheme::default().is_empty());
        assert!(SidebarTheme::default().is_empty());
        assert!(ToolbarTheme::default().is_empty());
        assert!(StatusBarTheme::default().is_empty());
        assert!(ListTheme::default().is_empty());
        assert!(PopoverTheme::default().is_empty());
        assert!(SplitterTheme::default().is_empty());
        assert!(SeparatorTheme::default().is_empty());
        assert!(SwitchTheme::default().is_empty());
        assert!(DialogTheme::default().is_empty());
        assert!(SpinnerTheme::default().is_empty());
        assert!(ComboBoxTheme::default().is_empty());
        assert!(SegmentedControlTheme::default().is_empty());
        assert!(CardTheme::default().is_empty());
        assert!(ExpanderTheme::default().is_empty());
        assert!(LinkTheme::default().is_empty());
    }

    // === Representative TOML round-trips ===

    #[test]
    fn input_theme_toml_round_trip() {
        let t = InputTheme {
            background_color: Some(Rgba::rgb(255, 255, 255)),
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                ..Default::default()
            }),
            border: Some(WidgetBorderSpec {
                color: Some(Rgba::rgb(180, 180, 180)),
                corner_radius: Some(4.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        let toml_str = toml::to_string(&t).unwrap();
        let t2: InputTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn switch_theme_toml_round_trip() {
        let s = SwitchTheme {
            checked_background: Some(Rgba::rgb(0, 120, 215)),
            track_width: Some(40.0),
            track_height: Some(20.0),
            thumb_diameter: Some(14.0),
            track_radius: Some(10.0),
            ..Default::default()
        };
        let toml_str = toml::to_string(&s).unwrap();
        let s2: SwitchTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(s, s2);
    }

    #[test]
    fn card_theme_with_border() {
        let c = CardTheme {
            background_color: Some(Rgba::rgb(255, 255, 255)),
            border: Some(WidgetBorderSpec {
                corner_radius: Some(8.0),
                shadow_enabled: Some(true),
                ..Default::default()
            }),
        };
        assert!(!c.is_empty());
    }

    #[test]
    fn link_theme_has_underline_enabled_bool_field() {
        let l = LinkTheme {
            visited_text_color: Some(Rgba::rgb(100, 0, 200)),
            underline_enabled: Some(true),
            ..Default::default()
        };
        assert!(!l.is_empty());
        assert_eq!(l.underline_enabled, Some(true));
    }

    #[test]
    fn status_bar_theme_has_font_and_background() {
        let s = StatusBarTheme {
            background_color: Some(Rgba::rgb(240, 240, 240)),
            font: Some(FontSpec {
                size: Some(FontSize::Px(11.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!s.is_empty());
    }

    // === SC4: Dual optional_nested (font + border) test widget ===

    // SC4: Verify derive handles dual optional_nested (font + border)
    /// Test widget with both font and border nested sub-structs.
    #[derive(
        Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ThemeWidget,
    )]
    #[serde_with::skip_serializing_none]
    #[serde(default)]
    #[theme_layer(skip_inventory)]
    pub struct DualNestedTestWidget {
        pub background: Option<Rgba>,
        pub min_height: Option<f32>,
        #[theme(nested, resolved_type = "ResolvedFontSpec")]
        pub font: Option<FontSpec>,
        #[theme(nested, resolved_type = "ResolvedBorderSpec")]
        pub border: Option<WidgetBorderSpec>,
    }

    #[test]
    fn dual_nested_default_is_empty() {
        assert!(DualNestedTestWidget::default().is_empty());
    }

    #[test]
    fn dual_nested_field_names() {
        assert_eq!(DualNestedTestWidget::FIELD_NAMES.len(), 4);
        assert!(DualNestedTestWidget::FIELD_NAMES.contains(&"background"));
        assert!(DualNestedTestWidget::FIELD_NAMES.contains(&"min_height"));
        assert!(DualNestedTestWidget::FIELD_NAMES.contains(&"font"));
        assert!(DualNestedTestWidget::FIELD_NAMES.contains(&"border"));
    }

    #[test]
    fn dual_nested_not_empty_when_font_set() {
        let w = DualNestedTestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    #[test]
    fn dual_nested_not_empty_when_border_set() {
        let w = DualNestedTestWidget {
            border: Some(WidgetBorderSpec {
                color: Some(Rgba::rgb(100, 100, 100)),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    #[test]
    fn dual_nested_merge_both_nested() {
        let mut base = DualNestedTestWidget {
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = DualNestedTestWidget {
            border: Some(WidgetBorderSpec {
                corner_radius: Some(4.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        assert!(base.font.is_some());
        assert!(base.border.is_some());
        assert_eq!(
            base.font.as_ref().and_then(|f| f.family.as_deref()),
            Some("Noto Sans")
        );
        assert_eq!(
            base.border.as_ref().and_then(|b| b.corner_radius),
            Some(4.0)
        );
    }

    #[test]
    fn dual_nested_merge_inner_font_fields() {
        let mut base = DualNestedTestWidget {
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = DualNestedTestWidget {
            font: Some(FontSpec {
                size: Some(FontSize::Px(14.0)),
                ..Default::default()
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(font.size, Some(FontSize::Px(14.0))); // overlay sets
    }

    #[test]
    fn dual_nested_toml_round_trip() {
        let w = DualNestedTestWidget {
            background: Some(Rgba::rgb(240, 240, 240)),
            min_height: Some(32.0),
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(FontSize::Px(14.0)),
                weight: Some(400),
                ..Default::default()
            }),
            border: Some(WidgetBorderSpec {
                color: Some(Rgba::rgb(180, 180, 180)),
                corner_radius: Some(4.0),
                line_width: Some(1.0),
                ..Default::default()
            }),
        };
        let toml_str = toml::to_string(&w).unwrap();
        let w2: DualNestedTestWidget = toml::from_str(&toml_str).unwrap();
        assert_eq!(w, w2);
    }

    // === LayoutTheme tests ===

    // === validate_widget() generation tests ===

    #[test]
    fn button_validate_widget_extracts_all_fields() {
        let button = ButtonTheme {
            background_color: Some(Rgba::rgb(200, 200, 200)),
            primary_background: Some(Rgba::rgb(0, 120, 215)),
            primary_text_color: Some(Rgba::rgb(255, 255, 255)),
            min_width: Some(80.0),
            min_height: Some(32.0),
            icon_text_gap: Some(8.0),
            disabled_opacity: Some(0.4),
            hover_background: Some(Rgba::rgb(210, 210, 210)),
            hover_text_color: Some(Rgba::rgb(0, 0, 0)),
            active_text_color: Some(Rgba::rgb(0, 0, 0)),
            disabled_text_color: Some(Rgba::rgb(128, 128, 128)),
            active_background: Some(Rgba::rgb(180, 180, 180)),
            disabled_background: Some(Rgba::rgb(220, 220, 220)),
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(FontSize::Px(14.0)),
                weight: Some(400),
                style: Some(crate::model::font::FontStyle::Normal),
                color: Some(Rgba::rgb(0, 0, 0)),
            }),
            border: Some(WidgetBorderSpec {
                color: Some(Rgba::rgb(100, 100, 100)),
                corner_radius: Some(4.0),
                line_width: Some(1.0),
                shadow_enabled: Some(false),
                padding_horizontal: Some(12.0),
                padding_vertical: Some(6.0),
            }),
        };
        let mut missing = Vec::new();
        let resolved = ResolvedButtonTheme::validate_widget(&button, "button", 96.0, &mut missing);
        assert!(missing.is_empty(), "unexpected missing: {missing:?}");
        assert_eq!(resolved.background_color, Rgba::rgb(200, 200, 200));
        assert_eq!(resolved.min_width, 80.0);
        assert_eq!(resolved.font.family.as_ref(), "Inter");
        assert_eq!(resolved.font.size, 14.0);
        assert_eq!(resolved.border.corner_radius, 4.0);
        // soft_option fields pass through as Option
        assert_eq!(resolved.active_background, Some(Rgba::rgb(180, 180, 180)));
        assert_eq!(resolved.disabled_background, Some(Rgba::rgb(220, 220, 220)));
    }

    #[test]
    fn button_validate_widget_records_missing_fields() {
        let button = ButtonTheme::default(); // all None
        let mut missing = Vec::new();
        let _ = ResolvedButtonTheme::validate_widget(&button, "button", 96.0, &mut missing);
        // option fields should be recorded as missing
        assert!(missing.contains(&"button.background_color".to_string()));
        assert!(missing.contains(&"button.min_width".to_string()));
        // font (optional_nested) should be recorded
        assert!(missing.contains(&"button.font".to_string()));
        // border (optional_nested) should be recorded
        assert!(missing.contains(&"button.border".to_string()));
        // soft_option fields should NOT be recorded as missing
        assert!(!missing.iter().any(|m| m.contains("active_background")));
        assert!(!missing.iter().any(|m| m.contains("disabled_background")));
    }

    // === LayoutTheme tests ===

    #[test]
    fn layout_theme_default_is_empty() {
        assert!(LayoutTheme::default().is_empty());
    }

    #[test]
    fn layout_theme_not_empty_when_widget_gap_set() {
        let l = LayoutTheme {
            widget_gap: Some(8.0),
            ..Default::default()
        };
        assert!(!l.is_empty());
    }

    #[test]
    fn layout_theme_field_names() {
        assert_eq!(LayoutTheme::FIELD_NAMES.len(), 4);
        assert!(LayoutTheme::FIELD_NAMES.contains(&"widget_gap_px"));
        assert!(LayoutTheme::FIELD_NAMES.contains(&"container_margin_px"));
        assert!(LayoutTheme::FIELD_NAMES.contains(&"window_margin_px"));
        assert!(LayoutTheme::FIELD_NAMES.contains(&"section_gap_px"));
    }

    #[test]
    fn layout_theme_toml_round_trip() {
        let l = LayoutTheme {
            widget_gap: Some(8.0),
            container_margin: Some(12.0),
            window_margin: Some(16.0),
            section_gap: Some(24.0),
        };
        let toml_str = toml::to_string(&l).unwrap();
        let l2: LayoutTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(l, l2);
    }

    #[test]
    fn layout_theme_merge() {
        let mut base = LayoutTheme {
            widget_gap: Some(6.0),
            container_margin: Some(10.0),
            ..Default::default()
        };
        let overlay = LayoutTheme {
            widget_gap: Some(8.0),
            section_gap: Some(24.0),
            ..Default::default()
        };
        base.merge(&overlay);
        // overlay widget_gap replaces base
        assert_eq!(base.widget_gap, Some(8.0));
        // base container_margin preserved
        assert_eq!(base.container_margin, Some(10.0));
        // overlay section_gap added
        assert_eq!(base.section_gap, Some(24.0));
        // window_margin stays None
        assert!(base.window_margin.is_none());
    }
}
