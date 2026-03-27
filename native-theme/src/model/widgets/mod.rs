// Per-widget struct pairs and macros

use crate::Rgba;
use crate::model::{DialogButtonOrder, FontSpec};

/// A resolved (non-optional) font specification produced after theme resolution.
///
/// Unlike [`crate::model::FontSpec`], all fields are required (non-optional)
/// because resolution has already filled in all defaults.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFontSpec {
    /// Font family name.
    pub family: String,
    /// Font size in logical pixels.
    pub size: f32,
    /// CSS font weight (100–900).
    pub weight: u16,
}

/// Generates a paired Option-based theme struct and a Resolved struct from a single definition.
///
/// # Usage
///
/// ```ignore
/// define_widget_pair! {
///     /// Doc comment
///     ButtonTheme / ResolvedButtonTheme {
///         option {
///             color: crate::Rgba,
///             size: f32,
///         }
///         optional_nested {
///             font: [crate::model::FontSpec, ResolvedFontSpec],
///         }
///     }
/// }
/// ```
///
/// This generates:
/// - `ButtonTheme` with all `option` fields as `Option<T>` and all `optional_nested` fields
///   as `Option<FontSpec>` (the first type in the pair). Derives: Clone, Debug, Default,
///   PartialEq, Serialize, Deserialize. Attributes: skip_serializing_none, serde(default).
/// - `ResolvedButtonTheme` with all `option` fields as plain `T` and all `optional_nested`
///   fields as `ResolvedFontSpec` (the second type in the pair). Derives: Clone, Debug, PartialEq.
/// - `impl_merge!` invocation for `ButtonTheme` using the `optional_nested` clause for font fields.
macro_rules! define_widget_pair {
    (
        $(#[$attr:meta])*
        $opt_name:ident / $resolved_name:ident {
            $(option {
                $($opt_field:ident : $opt_type:ty),* $(,)?
            })?
            $(optional_nested {
                $($on_field:ident : [$on_opt_type:ty, $on_res_type:ty]),* $(,)?
            })?
        }
    ) => {
        $(#[$attr])*
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(default)]
        pub struct $opt_name {
            $($(pub $opt_field: Option<$opt_type>,)*)?
            $($(pub $on_field: Option<$on_opt_type>,)*)?
        }

        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $resolved_name {
            $($(pub $opt_field: $opt_type,)*)?
            $($(pub $on_field: $on_res_type,)*)?
        }

        $crate::impl_merge!($opt_name {
            $(option { $($opt_field),* })?
            $(optional_nested { $($on_field),* })?
        });
    };
}

// ── §2.2 Window / Application Chrome ────────────────────────────────────────

define_widget_pair! {
    /// Window chrome: background, title bar colors, inactive states, geometry.
    WindowTheme / ResolvedWindow {
        option {
            background: Rgba,
            foreground: Rgba,
            border: Rgba,
            title_bar_background: Rgba,
            title_bar_foreground: Rgba,
            inactive_title_bar_background: Rgba,
            inactive_title_bar_foreground: Rgba,
            radius: f32,
            shadow: bool,
        }
        optional_nested {
            title_bar_font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.3 Button ──────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Push button: colors, sizing, spacing, geometry.
    ButtonTheme / ResolvedButton {
        option {
            background: Rgba,
            foreground: Rgba,
            border: Rgba,
            primary_bg: Rgba,
            primary_fg: Rgba,
            min_width: f32,
            min_height: f32,
            padding_horizontal: f32,
            padding_vertical: f32,
            radius: f32,
            icon_spacing: f32,
            disabled_opacity: f32,
            shadow: bool,
        }
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.4 Text Input ──────────────────────────────────────────────────────────

define_widget_pair! {
    /// Single-line and multi-line text input fields.
    InputTheme / ResolvedInput {
        option {
            background: Rgba,
            foreground: Rgba,
            border: Rgba,
            placeholder: Rgba,
            caret: Rgba,
            selection: Rgba,
            selection_foreground: Rgba,
            min_height: f32,
            padding_horizontal: f32,
            padding_vertical: f32,
            radius: f32,
            border_width: f32,
        }
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.5 Checkbox / Radio Button ────────────────────────────────────────────

define_widget_pair! {
    /// Checkbox and radio button indicator geometry.
    CheckboxTheme / ResolvedCheckbox {
        option {
            checked_bg: Rgba,
            indicator_size: f32,
            spacing: f32,
            radius: f32,
            border_width: f32,
        }
    }
}

// ── §2.6 Menu ────────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Popup and context menu appearance.
    MenuTheme / ResolvedMenu {
        option {
            background: Rgba,
            foreground: Rgba,
            separator: Rgba,
            item_height: f32,
            padding_horizontal: f32,
            padding_vertical: f32,
            icon_spacing: f32,
        }
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.7 Tooltip ─────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Tooltip popup appearance.
    TooltipTheme / ResolvedTooltip {
        option {
            background: Rgba,
            foreground: Rgba,
            padding_horizontal: f32,
            padding_vertical: f32,
            max_width: f32,
            radius: f32,
        }
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.8 Scrollbar ───────────────────────────────────────────────────────────

define_widget_pair! {
    /// Scrollbar colors and geometry.
    ScrollbarTheme / ResolvedScrollbar {
        option {
            track: Rgba,
            thumb: Rgba,
            thumb_hover: Rgba,
            width: f32,
            min_thumb_height: f32,
            slider_width: f32,
            overlay_mode: bool,
        }
    }
}

// ── §2.9 Slider ──────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Slider control colors and geometry.
    SliderTheme / ResolvedSlider {
        option {
            fill: Rgba,
            track: Rgba,
            thumb: Rgba,
            track_height: f32,
            thumb_size: f32,
            tick_length: f32,
        }
    }
}

// ── §2.10 Progress Bar ───────────────────────────────────────────────────────

define_widget_pair! {
    /// Progress bar colors and geometry.
    ProgressBarTheme / ResolvedProgressBar {
        option {
            fill: Rgba,
            track: Rgba,
            height: f32,
            min_width: f32,
            radius: f32,
        }
    }
}

// ── §2.11 Tab Bar ─────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Tab bar colors and sizing.
    TabTheme / ResolvedTab {
        option {
            background: Rgba,
            foreground: Rgba,
            active_background: Rgba,
            active_foreground: Rgba,
            bar_background: Rgba,
            min_width: f32,
            min_height: f32,
            padding_horizontal: f32,
            padding_vertical: f32,
        }
    }
}

// ── §2.12 Sidebar ─────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Sidebar panel background and foreground colors.
    SidebarTheme / ResolvedSidebar {
        option {
            background: Rgba,
            foreground: Rgba,
        }
    }
}

// ── §2.13 Toolbar ─────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Toolbar sizing, spacing, and font.
    ToolbarTheme / ResolvedToolbar {
        option {
            height: f32,
            item_spacing: f32,
            padding: f32,
        }
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.14 Status Bar ──────────────────────────────────────────────────────────

define_widget_pair! {
    /// Status bar font.
    StatusBarTheme / ResolvedStatusBar {
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.15 List / Table ────────────────────────────────────────────────────────

define_widget_pair! {
    /// List and table colors and row geometry.
    ListTheme / ResolvedList {
        option {
            background: Rgba,
            foreground: Rgba,
            alternate_row: Rgba,
            selection: Rgba,
            selection_foreground: Rgba,
            header_background: Rgba,
            header_foreground: Rgba,
            grid_color: Rgba,
            item_height: f32,
            padding_horizontal: f32,
            padding_vertical: f32,
        }
    }
}

// ── §2.16 Popover / Dropdown ──────────────────────────────────────────────────

define_widget_pair! {
    /// Popover / dropdown panel appearance.
    PopoverTheme / ResolvedPopover {
        option {
            background: Rgba,
            foreground: Rgba,
            border: Rgba,
            radius: f32,
        }
    }
}

// ── §2.17 Splitter ────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Splitter handle width.
    SplitterTheme / ResolvedSplitter {
        option {
            width: f32,
        }
    }
}

// ── §2.18 Separator ───────────────────────────────────────────────────────────

define_widget_pair! {
    /// Separator line color.
    SeparatorTheme / ResolvedSeparator {
        option {
            color: Rgba,
        }
    }
}

// ── §2.21 Switch / Toggle ─────────────────────────────────────────────────────

define_widget_pair! {
    /// Toggle switch track, thumb, and geometry.
    SwitchTheme / ResolvedSwitch {
        option {
            checked_bg: Rgba,
            unchecked_bg: Rgba,
            thumb_bg: Rgba,
            track_width: f32,
            track_height: f32,
            thumb_size: f32,
            track_radius: f32,
        }
    }
}

// ── §2.22 Dialog ──────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Dialog sizing, spacing, button order, and title font.
    DialogTheme / ResolvedDialog {
        option {
            min_width: f32,
            max_width: f32,
            min_height: f32,
            max_height: f32,
            content_padding: f32,
            button_spacing: f32,
            radius: f32,
            icon_size: f32,
            button_order: DialogButtonOrder,
        }
        optional_nested {
            title_font: [FontSpec, ResolvedFontSpec],
        }
    }
}

// ── §2.23 Spinner / Progress Ring ─────────────────────────────────────────────

define_widget_pair! {
    /// Spinner / indeterminate progress indicator.
    SpinnerTheme / ResolvedSpinner {
        option {
            fill: Rgba,
            diameter: f32,
            min_size: f32,
            stroke_width: f32,
        }
    }
}

// ── §2.24 ComboBox / Dropdown Trigger ─────────────────────────────────────────

define_widget_pair! {
    /// ComboBox / dropdown trigger sizing.
    ComboBoxTheme / ResolvedComboBox {
        option {
            min_height: f32,
            min_width: f32,
            padding_horizontal: f32,
            arrow_size: f32,
            arrow_area_width: f32,
            radius: f32,
        }
    }
}

// ── §2.25 Segmented Control ───────────────────────────────────────────────────

define_widget_pair! {
    /// Segmented control sizing (macOS-primary; KDE uses tab bar metrics as proxy).
    SegmentedControlTheme / ResolvedSegmentedControl {
        option {
            segment_height: f32,
            separator_width: f32,
            padding_horizontal: f32,
            radius: f32,
        }
    }
}

// ── §2.26 Card / Container ────────────────────────────────────────────────────

define_widget_pair! {
    /// Card / container colors and geometry.
    CardTheme / ResolvedCard {
        option {
            background: Rgba,
            border: Rgba,
            radius: f32,
            padding: f32,
            shadow: bool,
        }
    }
}

// ── §2.27 Expander / Disclosure ───────────────────────────────────────────────

define_widget_pair! {
    /// Expander / disclosure row geometry.
    ExpanderTheme / ResolvedExpander {
        option {
            header_height: f32,
            arrow_size: f32,
            content_padding: f32,
            radius: f32,
        }
    }
}

// ── §2.28 Link ────────────────────────────────────────────────────────────────

define_widget_pair! {
    /// Hyperlink colors and underline setting.
    LinkTheme / ResolvedLink {
        option {
            color: Rgba,
            visited: Rgba,
            background: Rgba,
            hover_bg: Rgba,
            underline: bool,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DialogButtonOrder, FontSpec};
    use crate::Rgba;

    // Define a test widget pair using the macro (validates macro itself still works)
    define_widget_pair! {
        /// Test widget for macro verification.
        TestWidget / ResolvedTestWidget {
            option {
                size: f32,
                label: String,
            }
            optional_nested {
                font: [FontSpec, ResolvedFontSpec],
            }
        }
    }

    // === ResolvedFontSpec tests ===

    #[test]
    fn resolved_font_spec_fields_are_concrete() {
        let rfs = ResolvedFontSpec {
            family: "Inter".into(),
            size: 14.0,
            weight: 400,
        };
        assert_eq!(rfs.family, "Inter");
        assert_eq!(rfs.size, 14.0);
        assert_eq!(rfs.weight, 400);
    }

    // === define_widget_pair! generated struct tests ===

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
                size: Some(14.0),
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
            },
        };
        assert_eq!(resolved.size, 24.0);
        assert_eq!(resolved.label, "Click me");
        assert_eq!(resolved.font.family, "Inter");
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
                size: Some(12.0),
                weight: None,
            }),
            ..Default::default()
        };
        let overlay = TestWidget {
            font: Some(FontSpec {
                family: None,
                size: None,
                weight: Some(700),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Noto Sans")); // preserved
        assert_eq!(font.size, Some(12.0));                     // preserved
        assert_eq!(font.weight, Some(700));                    // overlay sets
    }

    #[test]
    fn generated_merge_optional_nested_none_plus_some_clones() {
        let mut base = TestWidget::default();
        let overlay = TestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(14.0),
                weight: Some(400),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let font = base.font.as_ref().unwrap();
        assert_eq!(font.family.as_deref(), Some("Inter"));
        assert_eq!(font.size, Some(14.0));
        assert_eq!(font.weight, Some(400));
    }

    #[test]
    fn generated_merge_optional_nested_some_plus_none_preserves_base() {
        let mut base = TestWidget {
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(14.0),
                weight: Some(400),
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
                size: Some(12.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        let overlay = WithFont::default();
        base.merge(&overlay);
        assert_eq!(base.font.as_ref().unwrap().size, Some(12.0));
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
        assert_eq!(
            base.font.as_ref().unwrap().family.as_deref(),
            Some("Inter")
        );
    }

    #[test]
    fn impl_merge_optional_nested_some_some_merges_inner() {
        let mut base = WithFont {
            font: Some(FontSpec {
                family: Some("Noto".into()),
                size: Some(11.0),
                weight: None,
            }),
            ..Default::default()
        };
        let overlay = WithFont {
            font: Some(FontSpec {
                family: None,
                size: Some(14.0),
                weight: Some(400),
            }),
            ..Default::default()
        };
        base.merge(&overlay);
        let f = base.font.as_ref().unwrap();
        assert_eq!(f.family.as_deref(), Some("Noto")); // preserved
        assert_eq!(f.size, Some(14.0));                 // overlay wins
        assert_eq!(f.weight, Some(400));                // overlay sets
    }

    #[test]
    fn impl_merge_optional_nested_is_empty_none() {
        let w = WithFont::default();
        assert!(w.is_empty());
    }

    #[test]
    fn impl_merge_optional_nested_is_empty_some() {
        let w = WithFont {
            font: Some(FontSpec::default()),
            ..Default::default()
        };
        assert!(!w.is_empty());
    }

    // === ButtonTheme: 14 fields ===

    #[test]
    fn button_theme_has_all_fields_and_not_empty_when_set() {
        let b = ButtonTheme {
            background: Some(Rgba::rgb(200, 200, 200)),
            foreground: Some(Rgba::rgb(30, 30, 30)),
            border: Some(Rgba::rgb(150, 150, 150)),
            primary_bg: Some(Rgba::rgb(0, 120, 215)),
            primary_fg: Some(Rgba::rgb(255, 255, 255)),
            min_width: Some(64.0),
            min_height: Some(28.0),
            padding_horizontal: Some(12.0),
            padding_vertical: Some(6.0),
            radius: Some(4.0),
            icon_spacing: Some(6.0),
            disabled_opacity: Some(0.5),
            shadow: Some(false),
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(14.0),
                weight: Some(400),
            }),
        };
        assert!(!b.is_empty());
        assert_eq!(b.min_width, Some(64.0));
        assert_eq!(b.primary_bg, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn button_theme_default_is_empty() {
        assert!(ButtonTheme::default().is_empty());
    }

    #[test]
    fn button_theme_merge_font_optional_nested() {
        let mut base = ButtonTheme {
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
                size: Some(11.0),
                weight: None,
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
        assert_eq!(f.weight, Some(700));                    // overlay
    }

    #[test]
    fn button_theme_toml_round_trip_with_font() {
        let b = ButtonTheme {
            background: Some(Rgba::rgb(200, 200, 200)),
            radius: Some(4.0),
            font: Some(FontSpec {
                family: Some("Inter".into()),
                size: Some(14.0),
                weight: Some(400),
            }),
            ..Default::default()
        };
        let toml_str = toml::to_string(&b).unwrap();
        let b2: ButtonTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(b, b2);
    }

    // === WindowTheme: inactive title bar fields ===

    #[test]
    fn window_theme_has_inactive_title_bar_fields() {
        let w = WindowTheme {
            inactive_title_bar_background: Some(Rgba::rgb(180, 180, 180)),
            inactive_title_bar_foreground: Some(Rgba::rgb(120, 120, 120)),
            title_bar_font: Some(FontSpec {
                weight: Some(700),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!w.is_empty());
        assert!(w.inactive_title_bar_background.is_some());
        assert!(w.inactive_title_bar_foreground.is_some());
        assert!(w.title_bar_font.is_some());
    }

    #[test]
    fn window_theme_default_is_empty() {
        assert!(WindowTheme::default().is_empty());
    }

    // === DialogTheme: button_order field ===

    #[test]
    fn dialog_theme_button_order_works() {
        let d = DialogTheme {
            button_order: Some(DialogButtonOrder::TrailingAffirmative),
            min_width: Some(300.0),
            ..Default::default()
        };
        assert_eq!(d.button_order, Some(DialogButtonOrder::TrailingAffirmative));
        assert_eq!(d.min_width, Some(300.0));
        assert!(!d.is_empty());
    }

    #[test]
    fn dialog_theme_button_order_toml_round_trip() {
        let d = DialogTheme {
            button_order: Some(DialogButtonOrder::LeadingAffirmative),
            radius: Some(8.0),
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

    // === SplitterTheme: 1 field ===

    #[test]
    fn splitter_theme_single_field_merge() {
        let mut base = SplitterTheme {
            width: Some(4.0),
        };
        let overlay = SplitterTheme { width: Some(6.0) };
        base.merge(&overlay);
        assert_eq!(base.width, Some(6.0));
    }

    #[test]
    fn splitter_theme_merge_none_preserves_base() {
        let mut base = SplitterTheme { width: Some(4.0) };
        let overlay = SplitterTheme::default();
        base.merge(&overlay);
        assert_eq!(base.width, Some(4.0));
    }

    #[test]
    fn splitter_theme_default_is_empty() {
        assert!(SplitterTheme::default().is_empty());
    }

    #[test]
    fn splitter_theme_not_empty_when_set() {
        assert!(!SplitterTheme { width: Some(4.0) }.is_empty());
    }

    // === SeparatorTheme: 1 field ===

    #[test]
    fn separator_theme_single_field() {
        let s = SeparatorTheme {
            color: Some(Rgba::rgb(200, 200, 200)),
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
            background: Some(Rgba::rgb(255, 255, 255)),
            border: Some(Rgba::rgb(180, 180, 180)),
            radius: Some(4.0),
            font: Some(FontSpec {
                family: Some("Noto Sans".into()),
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
            checked_bg: Some(Rgba::rgb(0, 120, 215)),
            track_width: Some(40.0),
            track_height: Some(20.0),
            thumb_size: Some(14.0),
            track_radius: Some(10.0),
            ..Default::default()
        };
        let toml_str = toml::to_string(&s).unwrap();
        let s2: SwitchTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(s, s2);
    }

    #[test]
    fn card_theme_has_shadow_bool_field() {
        let c = CardTheme {
            shadow: Some(true),
            radius: Some(8.0),
            ..Default::default()
        };
        assert!(!c.is_empty());
        assert_eq!(c.shadow, Some(true));
    }

    #[test]
    fn link_theme_has_underline_bool_field() {
        let l = LinkTheme {
            color: Some(Rgba::rgb(0, 100, 200)),
            underline: Some(true),
            ..Default::default()
        };
        assert!(!l.is_empty());
        assert_eq!(l.underline, Some(true));
    }

    #[test]
    fn status_bar_theme_has_only_font_field() {
        // StatusBarTheme has only a font optional_nested field
        let s = StatusBarTheme {
            font: Some(FontSpec {
                size: Some(11.0),
                ..Default::default()
            }),
        };
        assert!(!s.is_empty());
    }
}
