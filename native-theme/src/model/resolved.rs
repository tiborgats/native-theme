// Resolved (non-optional) theme types produced after theme resolution.
//
// These types mirror their Option-based counterparts in defaults.rs, font.rs,
// spacing.rs, icon_sizes.rs, and mod.rs (ThemeVariant), but with all fields
// guaranteed populated. Produced by validate() after resolve().

use super::font::ResolvedFontSpec;
use crate::Rgba;

// --- ResolvedThemeSpacing ---

/// A fully resolved spacing scale where every tier is guaranteed populated.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResolvedThemeSpacing {
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
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResolvedTextScaleEntry {
    /// Font size in logical pixels.
    pub size: f32,
    /// CSS font weight (100-900).
    pub weight: u16,
    /// Line height in logical pixels (computed as `defaults.line_height × size`).
    pub line_height: f32,
}

// --- ResolvedTextScale ---

/// A fully resolved text scale with all four typographic roles populated.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

// --- ResolvedThemeDefaults ---

/// Fully resolved global theme defaults where every field is guaranteed populated.
///
/// Mirrors [`crate::model::ThemeDefaults`] but with concrete (non-Option) types.
/// Produced by the resolution/validation pipeline.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResolvedThemeDefaults {
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
    pub spacing: ResolvedThemeSpacing,

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

// --- ResolvedThemeVariant ---

/// A fully resolved theme where every field is guaranteed populated.
///
/// Produced by `validate()` after `resolve()`. Consumed by toolkit connectors.
/// Mirrors [`crate::model::ThemeVariant`] but with concrete (non-Option) types
/// for all 25 per-widget structs plus defaults and text scale.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResolvedThemeVariant {
    /// Global defaults.
    pub defaults: ResolvedThemeDefaults,
    /// Per-role text scale.
    pub text_scale: ResolvedTextScale,

    // ---- Per-widget resolved structs ----
    /// Window chrome.
    pub window: super::widgets::ResolvedWindowTheme,
    /// Push button.
    pub button: super::widgets::ResolvedButtonTheme,
    /// Text input.
    pub input: super::widgets::ResolvedInputTheme,
    /// Checkbox / radio button.
    pub checkbox: super::widgets::ResolvedCheckboxTheme,
    /// Popup / context menu.
    pub menu: super::widgets::ResolvedMenuTheme,
    /// Tooltip.
    pub tooltip: super::widgets::ResolvedTooltipTheme,
    /// Scrollbar.
    pub scrollbar: super::widgets::ResolvedScrollbarTheme,
    /// Slider.
    pub slider: super::widgets::ResolvedSliderTheme,
    /// Progress bar.
    pub progress_bar: super::widgets::ResolvedProgressBarTheme,
    /// Tab bar.
    pub tab: super::widgets::ResolvedTabTheme,
    /// Sidebar panel.
    pub sidebar: super::widgets::ResolvedSidebarTheme,
    /// Toolbar.
    pub toolbar: super::widgets::ResolvedToolbarTheme,
    /// Status bar.
    pub status_bar: super::widgets::ResolvedStatusBarTheme,
    /// List / table.
    pub list: super::widgets::ResolvedListTheme,
    /// Popover / dropdown.
    pub popover: super::widgets::ResolvedPopoverTheme,
    /// Splitter handle.
    pub splitter: super::widgets::ResolvedSplitterTheme,
    /// Separator line.
    pub separator: super::widgets::ResolvedSeparatorTheme,
    /// Toggle switch.
    pub switch: super::widgets::ResolvedSwitchTheme,
    /// Dialog.
    pub dialog: super::widgets::ResolvedDialogTheme,
    /// Spinner / progress ring.
    pub spinner: super::widgets::ResolvedSpinnerTheme,
    /// ComboBox / dropdown trigger.
    pub combo_box: super::widgets::ResolvedComboBoxTheme,
    /// Segmented control.
    pub segmented_control: super::widgets::ResolvedSegmentedControlTheme,
    /// Card / container.
    pub card: super::widgets::ResolvedCardTheme,
    /// Expander / disclosure.
    pub expander: super::widgets::ResolvedExpanderTheme,
    /// Hyperlink.
    pub link: super::widgets::ResolvedLinkTheme,

    /// Which icon loading mechanism to use — determines *how* icons are looked
    /// up (freedesktop theme directories, bundled SVG tables, SF Symbols, etc.).
    pub icon_set: crate::IconSet,

    /// The name of the visual icon theme that provides the actual icon files
    /// (e.g. `"breeze"`, `"Adwaita"`, `"Lucide"`).
    pub icon_theme: String,
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::bool_assert_comparison
)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::DialogButtonOrder;
    use crate::model::ResolvedFontSpec;
    use crate::model::font::FontStyle;
    use crate::model::widgets::{
        ResolvedButtonTheme, ResolvedCardTheme, ResolvedCheckboxTheme, ResolvedComboBoxTheme,
        ResolvedDialogTheme, ResolvedExpanderTheme, ResolvedInputTheme, ResolvedLinkTheme,
        ResolvedListTheme, ResolvedMenuTheme, ResolvedPopoverTheme, ResolvedProgressBarTheme,
        ResolvedScrollbarTheme, ResolvedSegmentedControlTheme, ResolvedSeparatorTheme,
        ResolvedSidebarTheme, ResolvedSliderTheme, ResolvedSpinnerTheme, ResolvedSplitterTheme,
        ResolvedStatusBarTheme, ResolvedSwitchTheme, ResolvedTabTheme, ResolvedToolbarTheme,
        ResolvedTooltipTheme, ResolvedWindowTheme,
    };

    fn sample_font() -> ResolvedFontSpec {
        ResolvedFontSpec {
            family: "Inter".into(),
            size: 14.0,
            weight: 400,
            style: FontStyle::Normal,
            color: Rgba::rgb(128, 128, 128),
        }
    }

    fn sample_spacing() -> ResolvedThemeSpacing {
        ResolvedThemeSpacing {
            xxs: 2.0,
            xs: 4.0,
            s: 6.0,
            m: 12.0,
            l: 18.0,
            xl: 24.0,
            xxl: 36.0,
        }
    }

    fn sample_icon_sizes() -> ResolvedIconSizes {
        ResolvedIconSizes {
            toolbar: 24.0,
            small: 16.0,
            large: 32.0,
            dialog: 22.0,
            panel: 20.0,
        }
    }

    fn sample_text_scale_entry() -> ResolvedTextScaleEntry {
        ResolvedTextScaleEntry {
            size: 12.0,
            weight: 400,
            line_height: 1.4,
        }
    }

    fn sample_defaults() -> ResolvedThemeDefaults {
        let c = Rgba::rgb(128, 128, 128);
        ResolvedThemeDefaults {
            font: sample_font(),
            line_height: 1.4,
            mono_font: ResolvedFontSpec {
                family: "JetBrains Mono".into(),
                size: 12.0,
                weight: 400,
                style: FontStyle::Normal,
                color: Rgba::rgb(128, 128, 128),
            },
            background: c,
            foreground: c,
            accent: c,
            accent_foreground: c,
            surface: c,
            border: c,
            muted: c,
            shadow: c,
            link: c,
            selection: c,
            selection_foreground: c,
            selection_inactive: c,
            disabled_foreground: c,
            danger: c,
            danger_foreground: c,
            warning: c,
            warning_foreground: c,
            success: c,
            success_foreground: c,
            info: c,
            info_foreground: c,
            radius: 4.0,
            radius_lg: 8.0,
            frame_width: 1.0,
            disabled_opacity: 0.5,
            border_opacity: 0.15,
            shadow_enabled: true,
            focus_ring_color: c,
            focus_ring_width: 2.0,
            focus_ring_offset: 1.0,
            spacing: sample_spacing(),
            icon_sizes: sample_icon_sizes(),
            text_scaling_factor: 1.0,
            reduce_motion: false,
            high_contrast: false,
            reduce_transparency: false,
        }
    }

    // --- ResolvedThemeSpacing tests ---

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
        assert!(dbg.contains("ResolvedThemeSpacing"));
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
            caption: ResolvedTextScaleEntry {
                size: 11.0,
                weight: 400,
                line_height: 1.3,
            },
            section_heading: ResolvedTextScaleEntry {
                size: 14.0,
                weight: 600,
                line_height: 1.4,
            },
            dialog_title: ResolvedTextScaleEntry {
                size: 16.0,
                weight: 700,
                line_height: 1.2,
            },
            display: ResolvedTextScaleEntry {
                size: 24.0,
                weight: 300,
                line_height: 1.1,
            },
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
            caption: e.clone(),
            section_heading: e.clone(),
            dialog_title: e.clone(),
            display: e,
        };
        let ts2 = ts.clone();
        assert_eq!(ts, ts2);
        let dbg = format!("{ts:?}");
        assert!(dbg.contains("ResolvedTextScale"));
    }

    // --- ResolvedThemeDefaults tests ---

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
        assert!(dbg.contains("ResolvedThemeDefaults"));
    }

    // --- ResolvedThemeVariant tests ---

    #[test]
    fn resolved_theme_construction_with_all_widgets() {
        let c = Rgba::rgb(100, 100, 100);
        let f = sample_font();
        let e = sample_text_scale_entry();

        let theme = ResolvedThemeVariant {
            defaults: sample_defaults(),
            text_scale: ResolvedTextScale {
                caption: e.clone(),
                section_heading: e.clone(),
                dialog_title: e.clone(),
                display: e,
            },
            window: ResolvedWindowTheme {
                background: c,
                foreground: c,
                border: c,
                title_bar_background: c,
                title_bar_foreground: c,
                inactive_title_bar_background: c,
                inactive_title_bar_foreground: c,
                radius: 4.0,
                shadow: true,
                title_bar_font: f.clone(),
            },
            button: ResolvedButtonTheme {
                background: c,
                foreground: c,
                border: c,
                primary_background: c,
                primary_foreground: c,
                min_width: 64.0,
                min_height: 28.0,
                padding_horizontal: 12.0,
                padding_vertical: 6.0,
                radius: 4.0,
                icon_spacing: 6.0,
                disabled_opacity: 0.5,
                shadow: false,
                font: f.clone(),
            },
            input: ResolvedInputTheme {
                background: c,
                foreground: c,
                border: c,
                placeholder: c,
                caret: c,
                selection: c,
                selection_foreground: c,
                min_height: 28.0,
                padding_horizontal: 8.0,
                padding_vertical: 4.0,
                radius: 4.0,
                border_width: 1.0,
                font: f.clone(),
            },
            checkbox: ResolvedCheckboxTheme {
                checked_background: c,
                indicator_size: 18.0,
                spacing: 6.0,
                radius: 2.0,
                border_width: 1.0,
            },
            menu: ResolvedMenuTheme {
                background: c,
                foreground: c,
                separator: c,
                item_height: 28.0,
                padding_horizontal: 8.0,
                padding_vertical: 4.0,
                icon_spacing: 6.0,
                font: f.clone(),
            },
            tooltip: ResolvedTooltipTheme {
                background: c,
                foreground: c,
                padding_horizontal: 6.0,
                padding_vertical: 4.0,
                max_width: 300.0,
                radius: 4.0,
                font: f.clone(),
            },
            scrollbar: ResolvedScrollbarTheme {
                track: c,
                thumb: c,
                thumb_hover: c,
                width: 14.0,
                min_thumb_height: 20.0,
                slider_width: 8.0,
                overlay_mode: false,
            },
            slider: ResolvedSliderTheme {
                fill: c,
                track: c,
                thumb: c,
                track_height: 4.0,
                thumb_size: 16.0,
                tick_length: 6.0,
            },
            progress_bar: ResolvedProgressBarTheme {
                fill: c,
                track: c,
                height: 6.0,
                min_width: 100.0,
                radius: 3.0,
            },
            tab: ResolvedTabTheme {
                background: c,
                foreground: c,
                active_background: c,
                active_foreground: c,
                bar_background: c,
                min_width: 60.0,
                min_height: 32.0,
                padding_horizontal: 12.0,
                padding_vertical: 6.0,
            },
            sidebar: ResolvedSidebarTheme {
                background: c,
                foreground: c,
            },
            toolbar: ResolvedToolbarTheme {
                height: 40.0,
                item_spacing: 4.0,
                padding: 4.0,
                font: f.clone(),
            },
            status_bar: ResolvedStatusBarTheme { font: f.clone() },
            list: ResolvedListTheme {
                background: c,
                foreground: c,
                alternate_row: c,
                selection: c,
                selection_foreground: c,
                header_background: c,
                header_foreground: c,
                grid_color: c,
                item_height: 28.0,
                padding_horizontal: 8.0,
                padding_vertical: 4.0,
            },
            popover: ResolvedPopoverTheme {
                background: c,
                foreground: c,
                border: c,
                radius: 6.0,
            },
            splitter: ResolvedSplitterTheme { width: 4.0 },
            separator: ResolvedSeparatorTheme { color: c },
            switch: ResolvedSwitchTheme {
                checked_background: c,
                unchecked_background: c,
                thumb_background: c,
                track_width: 40.0,
                track_height: 20.0,
                thumb_size: 14.0,
                track_radius: 10.0,
            },
            dialog: ResolvedDialogTheme {
                min_width: 320.0,
                max_width: 600.0,
                min_height: 200.0,
                max_height: 800.0,
                content_padding: 16.0,
                button_spacing: 8.0,
                radius: 8.0,
                icon_size: 22.0,
                button_order: DialogButtonOrder::TrailingAffirmative,
                title_font: f.clone(),
            },
            spinner: ResolvedSpinnerTheme {
                fill: c,
                diameter: 24.0,
                min_size: 16.0,
                stroke_width: 2.0,
            },
            combo_box: ResolvedComboBoxTheme {
                min_height: 28.0,
                min_width: 80.0,
                padding_horizontal: 8.0,
                arrow_size: 12.0,
                arrow_area_width: 20.0,
                radius: 4.0,
            },
            segmented_control: ResolvedSegmentedControlTheme {
                segment_height: 28.0,
                separator_width: 1.0,
                padding_horizontal: 12.0,
                radius: 4.0,
            },
            card: ResolvedCardTheme {
                background: c,
                border: c,
                radius: 8.0,
                padding: 12.0,
                shadow: true,
            },
            expander: ResolvedExpanderTheme {
                header_height: 32.0,
                arrow_size: 12.0,
                content_padding: 8.0,
                radius: 4.0,
            },
            link: ResolvedLinkTheme {
                color: c,
                visited: c,
                background: c,
                hover_bg: c,
                underline: true,
            },
            icon_set: crate::IconSet::Freedesktop,
            icon_theme: "breeze".into(),
        };

        // Verify key fields
        assert_eq!(theme.defaults.font.family, "Inter");
        assert_eq!(theme.window.radius, 4.0);
        assert_eq!(theme.button.min_height, 28.0);
        assert_eq!(theme.icon_set, crate::IconSet::Freedesktop);
        assert_eq!(theme.icon_theme, "breeze");
        assert_eq!(theme.text_scale.caption.size, 12.0);
    }

    #[test]
    fn resolved_theme_derives_clone_debug_partialeq() {
        let c = Rgba::rgb(100, 100, 100);
        let f = sample_font();
        let e = sample_text_scale_entry();

        let theme = ResolvedThemeVariant {
            defaults: sample_defaults(),
            text_scale: ResolvedTextScale {
                caption: e.clone(),
                section_heading: e.clone(),
                dialog_title: e.clone(),
                display: e,
            },
            window: ResolvedWindowTheme {
                background: c,
                foreground: c,
                border: c,
                title_bar_background: c,
                title_bar_foreground: c,
                inactive_title_bar_background: c,
                inactive_title_bar_foreground: c,
                radius: 4.0,
                shadow: true,
                title_bar_font: f.clone(),
            },
            button: ResolvedButtonTheme {
                background: c,
                foreground: c,
                border: c,
                primary_background: c,
                primary_foreground: c,
                min_width: 64.0,
                min_height: 28.0,
                padding_horizontal: 12.0,
                padding_vertical: 6.0,
                radius: 4.0,
                icon_spacing: 6.0,
                disabled_opacity: 0.5,
                shadow: false,
                font: f.clone(),
            },
            input: ResolvedInputTheme {
                background: c,
                foreground: c,
                border: c,
                placeholder: c,
                caret: c,
                selection: c,
                selection_foreground: c,
                min_height: 28.0,
                padding_horizontal: 8.0,
                padding_vertical: 4.0,
                radius: 4.0,
                border_width: 1.0,
                font: f.clone(),
            },
            checkbox: ResolvedCheckboxTheme {
                checked_background: c,
                indicator_size: 18.0,
                spacing: 6.0,
                radius: 2.0,
                border_width: 1.0,
            },
            menu: ResolvedMenuTheme {
                background: c,
                foreground: c,
                separator: c,
                item_height: 28.0,
                padding_horizontal: 8.0,
                padding_vertical: 4.0,
                icon_spacing: 6.0,
                font: f.clone(),
            },
            tooltip: ResolvedTooltipTheme {
                background: c,
                foreground: c,
                padding_horizontal: 6.0,
                padding_vertical: 4.0,
                max_width: 300.0,
                radius: 4.0,
                font: f.clone(),
            },
            scrollbar: ResolvedScrollbarTheme {
                track: c,
                thumb: c,
                thumb_hover: c,
                width: 14.0,
                min_thumb_height: 20.0,
                slider_width: 8.0,
                overlay_mode: false,
            },
            slider: ResolvedSliderTheme {
                fill: c,
                track: c,
                thumb: c,
                track_height: 4.0,
                thumb_size: 16.0,
                tick_length: 6.0,
            },
            progress_bar: ResolvedProgressBarTheme {
                fill: c,
                track: c,
                height: 6.0,
                min_width: 100.0,
                radius: 3.0,
            },
            tab: ResolvedTabTheme {
                background: c,
                foreground: c,
                active_background: c,
                active_foreground: c,
                bar_background: c,
                min_width: 60.0,
                min_height: 32.0,
                padding_horizontal: 12.0,
                padding_vertical: 6.0,
            },
            sidebar: ResolvedSidebarTheme {
                background: c,
                foreground: c,
            },
            toolbar: ResolvedToolbarTheme {
                height: 40.0,
                item_spacing: 4.0,
                padding: 4.0,
                font: f.clone(),
            },
            status_bar: ResolvedStatusBarTheme { font: f.clone() },
            list: ResolvedListTheme {
                background: c,
                foreground: c,
                alternate_row: c,
                selection: c,
                selection_foreground: c,
                header_background: c,
                header_foreground: c,
                grid_color: c,
                item_height: 28.0,
                padding_horizontal: 8.0,
                padding_vertical: 4.0,
            },
            popover: ResolvedPopoverTheme {
                background: c,
                foreground: c,
                border: c,
                radius: 6.0,
            },
            splitter: ResolvedSplitterTheme { width: 4.0 },
            separator: ResolvedSeparatorTheme { color: c },
            switch: ResolvedSwitchTheme {
                checked_background: c,
                unchecked_background: c,
                thumb_background: c,
                track_width: 40.0,
                track_height: 20.0,
                thumb_size: 14.0,
                track_radius: 10.0,
            },
            dialog: ResolvedDialogTheme {
                min_width: 320.0,
                max_width: 600.0,
                min_height: 200.0,
                max_height: 800.0,
                content_padding: 16.0,
                button_spacing: 8.0,
                radius: 8.0,
                icon_size: 22.0,
                button_order: DialogButtonOrder::TrailingAffirmative,
                title_font: f.clone(),
            },
            spinner: ResolvedSpinnerTheme {
                fill: c,
                diameter: 24.0,
                min_size: 16.0,
                stroke_width: 2.0,
            },
            combo_box: ResolvedComboBoxTheme {
                min_height: 28.0,
                min_width: 80.0,
                padding_horizontal: 8.0,
                arrow_size: 12.0,
                arrow_area_width: 20.0,
                radius: 4.0,
            },
            segmented_control: ResolvedSegmentedControlTheme {
                segment_height: 28.0,
                separator_width: 1.0,
                padding_horizontal: 12.0,
                radius: 4.0,
            },
            card: ResolvedCardTheme {
                background: c,
                border: c,
                radius: 8.0,
                padding: 12.0,
                shadow: true,
            },
            expander: ResolvedExpanderTheme {
                header_height: 32.0,
                arrow_size: 12.0,
                content_padding: 8.0,
                radius: 4.0,
            },
            link: ResolvedLinkTheme {
                color: c,
                visited: c,
                background: c,
                hover_bg: c,
                underline: true,
            },
            icon_set: crate::IconSet::Freedesktop,
            icon_theme: "breeze".into(),
        };

        let theme2 = theme.clone();
        assert_eq!(theme, theme2);
        let dbg = format!("{theme:?}");
        assert!(dbg.contains("ResolvedThemeVariant"));
    }

    // --- Behavioral tests (issue 2d) ---

    /// Verify that resolving a preset produces a ResolvedThemeVariant where
    /// button.background inherits from defaults.background when not overridden.
    #[test]
    fn resolve_fills_button_background_from_defaults() {
        let spec = crate::ThemeSpec::preset("catppuccin-mocha").expect("preset exists");
        let variant = spec.into_variant(true).expect("dark variant");
        let resolved = variant.into_resolved().expect("resolves");
        // Button background should be set (inherited from defaults if not explicit)
        assert_ne!(
            resolved.button.background,
            Rgba::default(),
            "button.background should be populated, not default"
        );
    }

    /// Verify that the resolved theme's status colors are distinct.
    #[test]
    fn resolved_status_colors_are_distinct() {
        let spec = crate::ThemeSpec::preset("kde-breeze").expect("preset exists");
        let variant = spec.into_variant(false).expect("light variant");
        let resolved = variant.into_resolved().expect("resolves");
        assert_ne!(resolved.defaults.danger, resolved.defaults.success);
        assert_ne!(resolved.defaults.warning, resolved.defaults.info);
        assert_ne!(resolved.defaults.danger, resolved.defaults.warning);
    }

    /// Verify that resolve fills selection_inactive from selection.
    #[test]
    fn resolved_selection_inactive_derived_from_selection() {
        let spec = crate::ThemeSpec::preset("nord").expect("preset exists");
        let variant = spec.into_variant(true).expect("dark variant");
        let resolved = variant.into_resolved().expect("resolves");
        // selection_inactive should be populated (derived from selection)
        assert_ne!(
            resolved.defaults.selection_inactive,
            Rgba::default(),
            "selection_inactive should be derived"
        );
    }

    /// Verify that all spacing fields are populated after resolution.
    #[test]
    fn resolved_spacing_all_fields_populated() {
        let spec = crate::ThemeSpec::preset("dracula").expect("preset exists");
        let variant = spec.into_variant(true).expect("dark variant");
        let resolved = variant.into_resolved().expect("resolves");
        assert!(resolved.defaults.spacing.xxs > 0.0);
        assert!(resolved.defaults.spacing.xs > 0.0);
        assert!(resolved.defaults.spacing.s > 0.0);
        assert!(resolved.defaults.spacing.m > 0.0);
        assert!(resolved.defaults.spacing.l > 0.0);
        assert!(resolved.defaults.spacing.xl > 0.0);
        assert!(resolved.defaults.spacing.xxl > 0.0);
    }

    /// Verify that text_scale entries have positive sizes after resolution.
    #[test]
    fn resolved_text_scale_has_positive_sizes() {
        let spec = crate::ThemeSpec::preset("adwaita").expect("preset exists");
        let variant = spec.into_variant(false).expect("light variant");
        let resolved = variant.into_resolved().expect("resolves");
        assert!(resolved.text_scale.caption.size > 0.0);
        assert!(resolved.text_scale.section_heading.size > 0.0);
        assert!(resolved.text_scale.dialog_title.size > 0.0);
        assert!(resolved.text_scale.display.size > 0.0);
        // caption < section_heading < dialog_title < display
        assert!(resolved.text_scale.caption.size < resolved.text_scale.display.size);
    }
}
