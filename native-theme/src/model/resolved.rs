// Resolved (non-optional) theme types produced after theme resolution.
//
// These types mirror their Option-based counterparts in defaults.rs, font.rs,
// icon_sizes.rs, and mod.rs (ThemeVariant), but with all fields
// guaranteed populated. Produced by validate() after resolve().

use super::border::ResolvedBorderSpec;
use super::font::ResolvedFontSpec;
use crate::Rgba;

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
    /// Line height in logical pixels (computed as `defaults.line_height * size`).
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
    pub background_color: Rgba,
    /// Default text color.
    pub text_color: Rgba,
    /// Accent/brand color for interactive elements.
    pub accent_color: Rgba,
    /// Text color used on accent-colored backgrounds.
    pub accent_text_color: Rgba,
    /// Elevated surface color.
    pub surface_color: Rgba,
    /// Secondary/subdued text color.
    pub muted_color: Rgba,
    /// Drop shadow color.
    pub shadow_color: Rgba,
    /// Hyperlink text color.
    pub link_color: Rgba,
    /// Selection highlight background.
    pub selection_background: Rgba,
    /// Text color over selection highlight.
    pub selection_text_color: Rgba,
    /// Selection background when window is unfocused.
    pub selection_inactive_background: Rgba,
    /// Text selection background (inline text highlight).
    pub text_selection_background: Rgba,
    /// Text selection color (inline text highlight).
    pub text_selection_color: Rgba,
    /// Text color for disabled controls.
    pub disabled_text_color: Rgba,

    // ---- Status colors ----
    /// Danger/error color.
    pub danger_color: Rgba,
    /// Text color on danger-colored backgrounds.
    pub danger_text_color: Rgba,
    /// Warning color.
    pub warning_color: Rgba,
    /// Text color on warning-colored backgrounds.
    pub warning_text_color: Rgba,
    /// Success/confirmation color.
    pub success_color: Rgba,
    /// Text color on success-colored backgrounds.
    pub success_text_color: Rgba,
    /// Informational color.
    pub info_color: Rgba,
    /// Text color on info-colored backgrounds.
    pub info_text_color: Rgba,

    // ---- Global geometry ----
    /// Border sub-struct (color, corner_radius, line_width, etc.).
    pub border: ResolvedBorderSpec,
    /// Opacity for disabled controls.
    pub disabled_opacity: f32,

    // ---- Focus ring ----
    /// Focus indicator outline color.
    pub focus_ring_color: Rgba,
    /// Focus indicator outline width.
    pub focus_ring_width: f32,
    /// Gap between element edge and focus indicator.
    pub focus_ring_offset: f32,

    // ---- Icon sizes ----
    /// Per-context icon sizes.
    pub icon_sizes: ResolvedIconSizes,

    // ---- Font DPI ----
    /// Font DPI used for pt-to-px conversion during resolution.
    /// Defaults to 96.0 when not set on the unresolved variant.
    pub font_dpi: f32,

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

    /// Which icon loading mechanism to use -- determines *how* icons are looked
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
    use crate::model::ResolvedFontSpec;
    use crate::model::border::ResolvedBorderSpec;
    use crate::model::font::FontStyle;

    fn sample_font() -> ResolvedFontSpec {
        ResolvedFontSpec {
            family: "Inter".into(),
            size: 14.0,
            weight: 400,
            style: FontStyle::Normal,
            color: Rgba::rgb(128, 128, 128),
        }
    }

    fn sample_border() -> ResolvedBorderSpec {
        ResolvedBorderSpec {
            color: Rgba::rgb(200, 200, 200),
            corner_radius: 4.0,
            corner_radius_lg: 8.0,
            line_width: 1.0,
            opacity: 0.15,
            shadow_enabled: true,
            padding_horizontal: 0.0,
            padding_vertical: 0.0,
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
            background_color: c,
            text_color: c,
            accent_color: c,
            accent_text_color: c,
            surface_color: c,
            muted_color: c,
            shadow_color: c,
            link_color: c,
            selection_background: c,
            selection_text_color: c,
            selection_inactive_background: c,
            text_selection_background: c,
            text_selection_color: c,
            disabled_text_color: c,
            danger_color: c,
            danger_text_color: c,
            warning_color: c,
            warning_text_color: c,
            success_color: c,
            success_text_color: c,
            info_color: c,
            info_text_color: c,
            border: sample_border(),
            disabled_opacity: 0.5,
            focus_ring_color: c,
            focus_ring_width: 2.0,
            focus_ring_offset: 1.0,
            icon_sizes: sample_icon_sizes(),
            font_dpi: 96.0,
            text_scaling_factor: 1.0,
            reduce_motion: false,
            high_contrast: false,
            reduce_transparency: false,
        }
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
        assert_eq!(d.background_color, Rgba::rgb(128, 128, 128));
        assert_eq!(d.accent_color, Rgba::rgb(128, 128, 128));
        // Geometry (border sub-struct)
        assert_eq!(d.border.corner_radius, 4.0);
        assert_eq!(d.border.shadow_enabled, true);
        // Focus ring
        assert_eq!(d.focus_ring_width, 2.0);
        // Icon sizes
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
    // NOTE: These tests construct ResolvedThemeVariant with all 25 widget structs.
    // The widget Resolved* types will have new field names after Task 2,
    // but for now they reference the old names -- Plan 02 (resolve.rs) will
    // update all consumers. These tests are intentionally commented out until
    // the full atomic commit is assembled.
    //
    // The structural tests for ResolvedThemeDefaults above verify the defaults
    // rename is correct.

    // --- Behavioral tests (issue 2d) ---
    // These tests call into resolve() and presets, which will break until
    // Plans 02-04 update all consumers. They are kept for reference but
    // will not compile until the atomic commit is complete.
}
