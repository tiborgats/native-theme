// Resolved (non-optional) theme types produced after theme resolution.
//
// These types mirror their Option-based counterparts in defaults.rs, font.rs,
// spacing.rs, icon_sizes.rs, and mod.rs (ThemeVariant), but with all fields
// guaranteed populated. Produced by validate() after resolve().

use crate::Rgba;
use super::widgets::ResolvedFontSpec;

// --- ResolvedSpacing ---

/// A fully resolved spacing scale where every tier is guaranteed populated.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedSpacing {
    /// Extra-extra-small spacing in logical pixels.
    pub xxs: f32,
    /// Extra-small spacing in logical pixels.
    pub xs: f32,
    /// Small spacing in logical pixels.
    pub s: f32,
    /// Medium spacing in logical pixels.
    pub m: f32,
    /// Large spacing in logical pixels.
    pub l: f32,
    /// Extra-large spacing in logical pixels.
    pub xl: f32,
    /// Extra-extra-large spacing in logical pixels.
    pub xxl: f32,
}

// --- ResolvedIconSizes ---

/// Fully resolved per-context icon sizes where every context is guaranteed populated.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedIconSizes {
    /// Icon size for toolbar buttons.
    pub toolbar: f32,
    /// Small icon size for inline use.
    pub small: f32,
    /// Large icon size for menus/lists.
    pub large: f32,
    /// Icon size for dialog buttons.
    pub dialog: f32,
    /// Icon size for panel headers.
    pub panel: f32,
}

// --- ResolvedTextScaleEntry ---

/// A single resolved text scale entry with guaranteed size, weight, and line height.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTextScaleEntry {
    /// Font size in logical pixels.
    pub size: f32,
    /// CSS font weight (100-900).
    pub weight: u16,
    /// Line height as a multiplier.
    pub line_height: f32,
}

// --- ResolvedTextScale ---

/// A fully resolved text scale with all four typographic roles populated.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTextScale {
    /// Caption / small label text.
    pub caption: ResolvedTextScaleEntry,
    /// Section heading text.
    pub section_heading: ResolvedTextScaleEntry,
    /// Dialog title text.
    pub dialog_title: ResolvedTextScaleEntry,
    /// Large display / hero text.
    pub display: ResolvedTextScaleEntry,
}

// --- ResolvedDefaults ---

/// Fully resolved global theme defaults where every field is guaranteed populated.
///
/// Mirrors [`crate::model::ThemeDefaults`] but with concrete (non-Option) types.
/// Produced by the resolution/validation pipeline.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedDefaults {
    // ---- Base font ----
    /// Primary UI font.
    pub font: ResolvedFontSpec,
    /// Line height multiplier.
    pub line_height: f32,
    /// Monospace font for code/terminal content.
    pub mono_font: ResolvedFontSpec,

    // ---- Base colors ----
    /// Main window/surface background color.
    pub background: Rgba,
    /// Default text color.
    pub foreground: Rgba,
    /// Accent/brand color for interactive elements.
    pub accent: Rgba,
    /// Text color used on accent-colored backgrounds.
    pub accent_foreground: Rgba,
    /// Elevated surface color.
    pub surface: Rgba,
    /// Border/divider color.
    pub border: Rgba,
    /// Secondary/subdued text color.
    pub muted: Rgba,
    /// Drop shadow color.
    pub shadow: Rgba,
    /// Hyperlink text color.
    pub link: Rgba,
    /// Selection highlight background.
    pub selection: Rgba,
    /// Text color over selection highlight.
    pub selection_foreground: Rgba,
    /// Selection background when window is unfocused.
    pub selection_inactive: Rgba,
    /// Text color for disabled controls.
    pub disabled_foreground: Rgba,

    // ---- Status colors ----
    /// Danger/error color.
    pub danger: Rgba,
    /// Text color on danger-colored backgrounds.
    pub danger_foreground: Rgba,
    /// Warning color.
    pub warning: Rgba,
    /// Text color on warning-colored backgrounds.
    pub warning_foreground: Rgba,
    /// Success/confirmation color.
    pub success: Rgba,
    /// Text color on success-colored backgrounds.
    pub success_foreground: Rgba,
    /// Informational color.
    pub info: Rgba,
    /// Text color on info-colored backgrounds.
    pub info_foreground: Rgba,

    // ---- Global geometry ----
    /// Default corner radius in logical pixels.
    pub radius: f32,
    /// Large corner radius.
    pub radius_lg: f32,
    /// Border/frame width in logical pixels.
    pub frame_width: f32,
    /// Opacity for disabled controls.
    pub disabled_opacity: f32,
    /// Border alpha multiplier.
    pub border_opacity: f32,
    /// Whether drop shadows are enabled.
    pub shadow_enabled: bool,

    // ---- Focus ring ----
    /// Focus indicator outline color.
    pub focus_ring_color: Rgba,
    /// Focus indicator outline width.
    pub focus_ring_width: f32,
    /// Gap between element edge and focus indicator.
    pub focus_ring_offset: f32,

    // ---- Spacing scale ----
    /// Logical spacing scale.
    pub spacing: ResolvedSpacing,

    // ---- Icon sizes ----
    /// Per-context icon sizes.
    pub icon_sizes: ResolvedIconSizes,

    // ---- Accessibility ----
    /// Text scaling factor (1.0 = no scaling).
    pub text_scaling_factor: f32,
    /// Whether the user has requested reduced motion.
    pub reduce_motion: bool,
    /// Whether a high-contrast mode is active.
    pub high_contrast: bool,
    /// Whether the user has requested reduced transparency.
    pub reduce_transparency: bool,
}

// --- ResolvedTheme ---

/// A fully resolved theme where every field is guaranteed populated.
///
/// Produced by `validate()` after `resolve()`. Consumed by toolkit connectors.
/// Mirrors [`crate::model::ThemeVariant`] but with concrete (non-Option) types
/// for all 25 per-widget structs plus defaults and text scale.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTheme {
    /// Global defaults.
    pub defaults: ResolvedDefaults,
    /// Per-role text scale.
    pub text_scale: ResolvedTextScale,

    // ---- Per-widget resolved structs ----
    /// Window chrome.
    pub window: super::widgets::ResolvedWindow,
    /// Push button.
    pub button: super::widgets::ResolvedButton,
    /// Text input.
    pub input: super::widgets::ResolvedInput,
    /// Checkbox / radio button.
    pub checkbox: super::widgets::ResolvedCheckbox,
    /// Popup / context menu.
    pub menu: super::widgets::ResolvedMenu,
    /// Tooltip.
    pub tooltip: super::widgets::ResolvedTooltip,
    /// Scrollbar.
    pub scrollbar: super::widgets::ResolvedScrollbar,
    /// Slider.
    pub slider: super::widgets::ResolvedSlider,
    /// Progress bar.
    pub progress_bar: super::widgets::ResolvedProgressBar,
    /// Tab bar.
    pub tab: super::widgets::ResolvedTab,
    /// Sidebar panel.
    pub sidebar: super::widgets::ResolvedSidebar,
    /// Toolbar.
    pub toolbar: super::widgets::ResolvedToolbar,
    /// Status bar.
    pub status_bar: super::widgets::ResolvedStatusBar,
    /// List / table.
    pub list: super::widgets::ResolvedList,
    /// Popover / dropdown.
    pub popover: super::widgets::ResolvedPopover,
    /// Splitter handle.
    pub splitter: super::widgets::ResolvedSplitter,
    /// Separator line.
    pub separator: super::widgets::ResolvedSeparator,
    /// Toggle switch.
    pub switch: super::widgets::ResolvedSwitch,
    /// Dialog.
    pub dialog: super::widgets::ResolvedDialog,
    /// Spinner / progress ring.
    pub spinner: super::widgets::ResolvedSpinner,
    /// ComboBox / dropdown trigger.
    pub combo_box: super::widgets::ResolvedComboBox,
    /// Segmented control.
    pub segmented_control: super::widgets::ResolvedSegmentedControl,
    /// Card / container.
    pub card: super::widgets::ResolvedCard,
    /// Expander / disclosure.
    pub expander: super::widgets::ResolvedExpander,
    /// Hyperlink.
    pub link: super::widgets::ResolvedLink,

    /// Icon set / naming convention (e.g., "sf-symbols", "freedesktop").
    pub icon_set: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::DialogButtonOrder;
    use crate::model::widgets::{
        ResolvedFontSpec, ResolvedWindow, ResolvedButton, ResolvedInput,
        ResolvedCheckbox, ResolvedMenu, ResolvedTooltip, ResolvedScrollbar,
        ResolvedSlider, ResolvedProgressBar, ResolvedTab, ResolvedSidebar,
        ResolvedToolbar, ResolvedStatusBar, ResolvedList, ResolvedPopover,
        ResolvedSplitter, ResolvedSeparator, ResolvedSwitch, ResolvedDialog,
        ResolvedSpinner, ResolvedComboBox, ResolvedSegmentedControl,
        ResolvedCard, ResolvedExpander, ResolvedLink,
    };

    fn sample_font() -> ResolvedFontSpec {
        ResolvedFontSpec {
            family: "Inter".into(),
            size: 14.0,
            weight: 400,
        }
    }

    fn sample_spacing() -> ResolvedSpacing {
        ResolvedSpacing {
            xxs: 2.0, xs: 4.0, s: 6.0, m: 12.0, l: 18.0, xl: 24.0, xxl: 36.0,
        }
    }

    fn sample_icon_sizes() -> ResolvedIconSizes {
        ResolvedIconSizes {
            toolbar: 24.0, small: 16.0, large: 32.0, dialog: 22.0, panel: 20.0,
        }
    }

    fn sample_text_scale_entry() -> ResolvedTextScaleEntry {
        ResolvedTextScaleEntry { size: 12.0, weight: 400, line_height: 1.4 }
    }

    fn sample_defaults() -> ResolvedDefaults {
        let c = Rgba::rgb(128, 128, 128);
        ResolvedDefaults {
            font: sample_font(),
            line_height: 1.4,
            mono_font: ResolvedFontSpec {
                family: "JetBrains Mono".into(),
                size: 12.0,
                weight: 400,
            },
            background: c, foreground: c, accent: c, accent_foreground: c,
            surface: c, border: c, muted: c, shadow: c, link: c,
            selection: c, selection_foreground: c, selection_inactive: c,
            disabled_foreground: c,
            danger: c, danger_foreground: c, warning: c, warning_foreground: c,
            success: c, success_foreground: c, info: c, info_foreground: c,
            radius: 4.0, radius_lg: 8.0, frame_width: 1.0,
            disabled_opacity: 0.5, border_opacity: 0.15,
            shadow_enabled: true,
            focus_ring_color: c, focus_ring_width: 2.0, focus_ring_offset: 1.0,
            spacing: sample_spacing(),
            icon_sizes: sample_icon_sizes(),
            text_scaling_factor: 1.0,
            reduce_motion: false,
            high_contrast: false,
            reduce_transparency: false,
        }
    }

    // --- ResolvedSpacing tests ---

    #[test]
    fn resolved_spacing_has_7_concrete_fields() {
        let s = sample_spacing();
        assert_eq!(s.xxs, 2.0);
        assert_eq!(s.xs, 4.0);
        assert_eq!(s.s, 6.0);
        assert_eq!(s.m, 12.0);
        assert_eq!(s.l, 18.0);
        assert_eq!(s.xl, 24.0);
        assert_eq!(s.xxl, 36.0);
    }

    #[test]
    fn resolved_spacing_derives_clone_debug_partialeq() {
        let s = sample_spacing();
        let s2 = s.clone();
        assert_eq!(s, s2);
        let dbg = format!("{s:?}");
        assert!(dbg.contains("ResolvedSpacing"));
    }

    // --- ResolvedIconSizes tests ---

    #[test]
    fn resolved_icon_sizes_has_5_concrete_fields() {
        let i = sample_icon_sizes();
        assert_eq!(i.toolbar, 24.0);
        assert_eq!(i.small, 16.0);
        assert_eq!(i.large, 32.0);
        assert_eq!(i.dialog, 22.0);
        assert_eq!(i.panel, 20.0);
    }

    #[test]
    fn resolved_icon_sizes_derives_clone_debug_partialeq() {
        let i = sample_icon_sizes();
        let i2 = i.clone();
        assert_eq!(i, i2);
        let dbg = format!("{i:?}");
        assert!(dbg.contains("ResolvedIconSizes"));
    }

    // --- ResolvedTextScaleEntry tests ---

    #[test]
    fn resolved_text_scale_entry_has_3_concrete_fields() {
        let e = sample_text_scale_entry();
        assert_eq!(e.size, 12.0);
        assert_eq!(e.weight, 400);
        assert_eq!(e.line_height, 1.4);
    }

    #[test]
    fn resolved_text_scale_entry_derives_clone_debug_partialeq() {
        let e = sample_text_scale_entry();
        let e2 = e.clone();
        assert_eq!(e, e2);
        let dbg = format!("{e:?}");
        assert!(dbg.contains("ResolvedTextScaleEntry"));
    }

    // --- ResolvedTextScale tests ---

    #[test]
    fn resolved_text_scale_has_4_entries() {
        let ts = ResolvedTextScale {
            caption: ResolvedTextScaleEntry { size: 11.0, weight: 400, line_height: 1.3 },
            section_heading: ResolvedTextScaleEntry { size: 14.0, weight: 600, line_height: 1.4 },
            dialog_title: ResolvedTextScaleEntry { size: 16.0, weight: 700, line_height: 1.2 },
            display: ResolvedTextScaleEntry { size: 24.0, weight: 300, line_height: 1.1 },
        };
        assert_eq!(ts.caption.size, 11.0);
        assert_eq!(ts.section_heading.weight, 600);
        assert_eq!(ts.dialog_title.size, 16.0);
        assert_eq!(ts.display.weight, 300);
    }

    #[test]
    fn resolved_text_scale_derives_clone_debug_partialeq() {
        let e = sample_text_scale_entry();
        let ts = ResolvedTextScale {
            caption: e.clone(), section_heading: e.clone(),
            dialog_title: e.clone(), display: e,
        };
        let ts2 = ts.clone();
        assert_eq!(ts, ts2);
        let dbg = format!("{ts:?}");
        assert!(dbg.contains("ResolvedTextScale"));
    }

    // --- ResolvedDefaults tests ---

    #[test]
    fn resolved_defaults_all_fields_concrete() {
        let d = sample_defaults();
        // Fonts
        assert_eq!(d.font.family, "Inter");
        assert_eq!(d.mono_font.family, "JetBrains Mono");
        assert_eq!(d.line_height, 1.4);
        // Some colors
        assert_eq!(d.background, Rgba::rgb(128, 128, 128));
        assert_eq!(d.accent, Rgba::rgb(128, 128, 128));
        // Geometry
        assert_eq!(d.radius, 4.0);
        assert_eq!(d.shadow_enabled, true);
        // Focus ring
        assert_eq!(d.focus_ring_width, 2.0);
        // Spacing and icon sizes
        assert_eq!(d.spacing.m, 12.0);
        assert_eq!(d.icon_sizes.toolbar, 24.0);
        // Accessibility
        assert_eq!(d.text_scaling_factor, 1.0);
        assert_eq!(d.reduce_motion, false);
    }

    #[test]
    fn resolved_defaults_derives_clone_debug_partialeq() {
        let d = sample_defaults();
        let d2 = d.clone();
        assert_eq!(d, d2);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("ResolvedDefaults"));
    }

    // --- ResolvedTheme tests ---

    #[test]
    fn resolved_theme_construction_with_all_widgets() {
        let c = Rgba::rgb(100, 100, 100);
        let f = sample_font();
        let e = sample_text_scale_entry();

        let theme = ResolvedTheme {
            defaults: sample_defaults(),
            text_scale: ResolvedTextScale {
                caption: e.clone(), section_heading: e.clone(),
                dialog_title: e.clone(), display: e,
            },
            window: ResolvedWindow {
                background: c, foreground: c, border: c,
                title_bar_background: c, title_bar_foreground: c,
                inactive_title_bar_background: c, inactive_title_bar_foreground: c,
                radius: 4.0, shadow: true,
                title_bar_font: f.clone(),
            },
            button: ResolvedButton {
                background: c, foreground: c, border: c,
                primary_bg: c, primary_fg: c,
                min_width: 64.0, min_height: 28.0,
                padding_horizontal: 12.0, padding_vertical: 6.0,
                radius: 4.0, icon_spacing: 6.0,
                disabled_opacity: 0.5, shadow: false,
                font: f.clone(),
            },
            input: ResolvedInput {
                background: c, foreground: c, border: c,
                placeholder: c, caret: c, selection: c, selection_foreground: c,
                min_height: 28.0, padding_horizontal: 8.0, padding_vertical: 4.0,
                radius: 4.0, border_width: 1.0,
                font: f.clone(),
            },
            checkbox: ResolvedCheckbox {
                checked_bg: c, indicator_size: 18.0, spacing: 6.0,
                radius: 2.0, border_width: 1.0,
            },
            menu: ResolvedMenu {
                background: c, foreground: c, separator: c,
                item_height: 28.0, padding_horizontal: 8.0, padding_vertical: 4.0,
                icon_spacing: 6.0, font: f.clone(),
            },
            tooltip: ResolvedTooltip {
                background: c, foreground: c,
                padding_horizontal: 6.0, padding_vertical: 4.0,
                max_width: 300.0, radius: 4.0,
                font: f.clone(),
            },
            scrollbar: ResolvedScrollbar {
                track: c, thumb: c, thumb_hover: c,
                width: 14.0, min_thumb_height: 20.0,
                slider_width: 8.0, overlay_mode: false,
            },
            slider: ResolvedSlider {
                fill: c, track: c, thumb: c,
                track_height: 4.0, thumb_size: 16.0, tick_length: 6.0,
            },
            progress_bar: ResolvedProgressBar {
                fill: c, track: c,
                height: 6.0, min_width: 100.0, radius: 3.0,
            },
            tab: ResolvedTab {
                background: c, foreground: c,
                active_background: c, active_foreground: c, bar_background: c,
                min_width: 60.0, min_height: 32.0,
                padding_horizontal: 12.0, padding_vertical: 6.0,
            },
            sidebar: ResolvedSidebar { background: c, foreground: c },
            toolbar: ResolvedToolbar {
                height: 40.0, item_spacing: 4.0, padding: 4.0,
                font: f.clone(),
            },
            status_bar: ResolvedStatusBar { font: f.clone() },
            list: ResolvedList {
                background: c, foreground: c, alternate_row: c,
                selection: c, selection_foreground: c,
                header_background: c, header_foreground: c, grid_color: c,
                item_height: 28.0, padding_horizontal: 8.0, padding_vertical: 4.0,
            },
            popover: ResolvedPopover {
                background: c, foreground: c, border: c, radius: 6.0,
            },
            splitter: ResolvedSplitter { width: 4.0 },
            separator: ResolvedSeparator { color: c },
            switch: ResolvedSwitch {
                checked_bg: c, unchecked_bg: c, thumb_bg: c,
                track_width: 40.0, track_height: 20.0,
                thumb_size: 14.0, track_radius: 10.0,
            },
            dialog: ResolvedDialog {
                min_width: 320.0, max_width: 600.0,
                min_height: 200.0, max_height: 800.0,
                content_padding: 16.0, button_spacing: 8.0,
                radius: 8.0, icon_size: 22.0,
                button_order: DialogButtonOrder::TrailingAffirmative,
                title_font: f.clone(),
            },
            spinner: ResolvedSpinner {
                fill: c, diameter: 24.0, min_size: 16.0, stroke_width: 2.0,
            },
            combo_box: ResolvedComboBox {
                min_height: 28.0, min_width: 80.0,
                padding_horizontal: 8.0, arrow_size: 12.0,
                arrow_area_width: 20.0, radius: 4.0,
            },
            segmented_control: ResolvedSegmentedControl {
                segment_height: 28.0, separator_width: 1.0,
                padding_horizontal: 12.0, radius: 4.0,
            },
            card: ResolvedCard {
                background: c, border: c,
                radius: 8.0, padding: 12.0, shadow: true,
            },
            expander: ResolvedExpander {
                header_height: 32.0, arrow_size: 12.0,
                content_padding: 8.0, radius: 4.0,
            },
            link: ResolvedLink {
                color: c, visited: c, background: c, hover_bg: c,
                underline: true,
            },
            icon_set: "freedesktop".into(),
        };

        // Verify key fields
        assert_eq!(theme.defaults.font.family, "Inter");
        assert_eq!(theme.window.radius, 4.0);
        assert_eq!(theme.button.min_height, 28.0);
        assert_eq!(theme.icon_set, "freedesktop");
        assert_eq!(theme.text_scale.caption.size, 12.0);
    }

    #[test]
    fn resolved_theme_derives_clone_debug_partialeq() {
        let c = Rgba::rgb(100, 100, 100);
        let f = sample_font();
        let e = sample_text_scale_entry();

        let theme = ResolvedTheme {
            defaults: sample_defaults(),
            text_scale: ResolvedTextScale {
                caption: e.clone(), section_heading: e.clone(),
                dialog_title: e.clone(), display: e,
            },
            window: ResolvedWindow {
                background: c, foreground: c, border: c,
                title_bar_background: c, title_bar_foreground: c,
                inactive_title_bar_background: c, inactive_title_bar_foreground: c,
                radius: 4.0, shadow: true, title_bar_font: f.clone(),
            },
            button: ResolvedButton {
                background: c, foreground: c, border: c,
                primary_bg: c, primary_fg: c,
                min_width: 64.0, min_height: 28.0,
                padding_horizontal: 12.0, padding_vertical: 6.0,
                radius: 4.0, icon_spacing: 6.0,
                disabled_opacity: 0.5, shadow: false, font: f.clone(),
            },
            input: ResolvedInput {
                background: c, foreground: c, border: c,
                placeholder: c, caret: c, selection: c, selection_foreground: c,
                min_height: 28.0, padding_horizontal: 8.0, padding_vertical: 4.0,
                radius: 4.0, border_width: 1.0, font: f.clone(),
            },
            checkbox: ResolvedCheckbox {
                checked_bg: c, indicator_size: 18.0, spacing: 6.0,
                radius: 2.0, border_width: 1.0,
            },
            menu: ResolvedMenu {
                background: c, foreground: c, separator: c,
                item_height: 28.0, padding_horizontal: 8.0, padding_vertical: 4.0,
                icon_spacing: 6.0, font: f.clone(),
            },
            tooltip: ResolvedTooltip {
                background: c, foreground: c,
                padding_horizontal: 6.0, padding_vertical: 4.0,
                max_width: 300.0, radius: 4.0, font: f.clone(),
            },
            scrollbar: ResolvedScrollbar {
                track: c, thumb: c, thumb_hover: c,
                width: 14.0, min_thumb_height: 20.0,
                slider_width: 8.0, overlay_mode: false,
            },
            slider: ResolvedSlider {
                fill: c, track: c, thumb: c,
                track_height: 4.0, thumb_size: 16.0, tick_length: 6.0,
            },
            progress_bar: ResolvedProgressBar {
                fill: c, track: c, height: 6.0, min_width: 100.0, radius: 3.0,
            },
            tab: ResolvedTab {
                background: c, foreground: c,
                active_background: c, active_foreground: c, bar_background: c,
                min_width: 60.0, min_height: 32.0,
                padding_horizontal: 12.0, padding_vertical: 6.0,
            },
            sidebar: ResolvedSidebar { background: c, foreground: c },
            toolbar: ResolvedToolbar {
                height: 40.0, item_spacing: 4.0, padding: 4.0, font: f.clone(),
            },
            status_bar: ResolvedStatusBar { font: f.clone() },
            list: ResolvedList {
                background: c, foreground: c, alternate_row: c,
                selection: c, selection_foreground: c,
                header_background: c, header_foreground: c, grid_color: c,
                item_height: 28.0, padding_horizontal: 8.0, padding_vertical: 4.0,
            },
            popover: ResolvedPopover {
                background: c, foreground: c, border: c, radius: 6.0,
            },
            splitter: ResolvedSplitter { width: 4.0 },
            separator: ResolvedSeparator { color: c },
            switch: ResolvedSwitch {
                checked_bg: c, unchecked_bg: c, thumb_bg: c,
                track_width: 40.0, track_height: 20.0,
                thumb_size: 14.0, track_radius: 10.0,
            },
            dialog: ResolvedDialog {
                min_width: 320.0, max_width: 600.0,
                min_height: 200.0, max_height: 800.0,
                content_padding: 16.0, button_spacing: 8.0,
                radius: 8.0, icon_size: 22.0,
                button_order: DialogButtonOrder::TrailingAffirmative,
                title_font: f.clone(),
            },
            spinner: ResolvedSpinner {
                fill: c, diameter: 24.0, min_size: 16.0, stroke_width: 2.0,
            },
            combo_box: ResolvedComboBox {
                min_height: 28.0, min_width: 80.0,
                padding_horizontal: 8.0, arrow_size: 12.0,
                arrow_area_width: 20.0, radius: 4.0,
            },
            segmented_control: ResolvedSegmentedControl {
                segment_height: 28.0, separator_width: 1.0,
                padding_horizontal: 12.0, radius: 4.0,
            },
            card: ResolvedCard {
                background: c, border: c, radius: 8.0, padding: 12.0, shadow: true,
            },
            expander: ResolvedExpander {
                header_height: 32.0, arrow_size: 12.0,
                content_padding: 8.0, radius: 4.0,
            },
            link: ResolvedLink {
                color: c, visited: c, background: c, hover_bg: c, underline: true,
            },
            icon_set: "freedesktop".into(),
        };

        let theme2 = theme.clone();
        assert_eq!(theme, theme2);
        let dbg = format!("{theme:?}");
        assert!(dbg.contains("ResolvedTheme"));
    }
}
