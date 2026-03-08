// Per-widget sizing and spacing metrics

use serde::{Deserialize, Serialize};

/// Button sizing and spacing metrics.
///
/// Defines minimum dimensions, padding, and icon spacing for push buttons.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ButtonMetrics {
    /// Minimum button width in logical pixels.
    pub min_width: Option<f32>,
    /// Minimum button height in logical pixels.
    pub min_height: Option<f32>,
    /// Horizontal padding inside the button.
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the button.
    pub padding_vertical: Option<f32>,
    /// Spacing between icon and label within the button.
    pub icon_spacing: Option<f32>,
}

impl_merge!(ButtonMetrics {
    option { min_width, min_height, padding_horizontal, padding_vertical, icon_spacing }
});

/// Checkbox and radio button metrics.
///
/// Defines the indicator (check mark area) size and spacing to its label.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CheckboxMetrics {
    /// Size of the checkbox/radio indicator in logical pixels.
    pub indicator_size: Option<f32>,
    /// Gap between indicator and label in logical pixels.
    pub spacing: Option<f32>,
}

impl_merge!(CheckboxMetrics {
    option { indicator_size, spacing }
});

/// Text input field metrics.
///
/// Defines minimum height and padding for single-line text inputs.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct InputMetrics {
    /// Minimum input field height in logical pixels.
    pub min_height: Option<f32>,
    /// Horizontal padding inside the input field.
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the input field.
    pub padding_vertical: Option<f32>,
}

impl_merge!(InputMetrics {
    option { min_height, padding_horizontal, padding_vertical }
});

/// Scrollbar metrics.
///
/// Defines track width, thumb dimensions, and slider width for scrollbars.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ScrollbarMetrics {
    /// Scrollbar track width in logical pixels.
    pub width: Option<f32>,
    /// Minimum thumb/slider height in logical pixels.
    pub min_thumb_height: Option<f32>,
    /// Thumb/slider width (may differ from track width) in logical pixels.
    pub slider_width: Option<f32>,
}

impl_merge!(ScrollbarMetrics {
    option { width, min_thumb_height, slider_width }
});

/// Slider/range control metrics.
///
/// Defines track groove thickness, thumb size, and tick mark length.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct SliderMetrics {
    /// Track groove thickness in logical pixels.
    pub track_height: Option<f32>,
    /// Control thumb diameter/width in logical pixels.
    pub thumb_size: Option<f32>,
    /// Tick mark length in logical pixels.
    pub tick_length: Option<f32>,
}

impl_merge!(SliderMetrics {
    option { track_height, thumb_size, tick_length }
});

/// Progress bar metrics.
///
/// Defines bar height and minimum width for determinate/indeterminate bars.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ProgressBarMetrics {
    /// Bar thickness in logical pixels.
    pub height: Option<f32>,
    /// Minimum bar width in logical pixels.
    pub min_width: Option<f32>,
}

impl_merge!(ProgressBarMetrics {
    option { height, min_width }
});

/// Tab bar metrics.
///
/// Defines minimum tab dimensions and padding for tabbed interfaces.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TabMetrics {
    /// Minimum tab width in logical pixels.
    pub min_width: Option<f32>,
    /// Minimum tab height in logical pixels.
    pub min_height: Option<f32>,
    /// Horizontal padding inside the tab.
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the tab.
    pub padding_vertical: Option<f32>,
}

impl_merge!(TabMetrics {
    option { min_width, min_height, padding_horizontal, padding_vertical }
});

/// Menu item metrics.
///
/// Defines height, padding, and icon spacing for menu items.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct MenuItemMetrics {
    /// Single menu item height in logical pixels.
    pub height: Option<f32>,
    /// Horizontal padding inside the menu item.
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the menu item.
    pub padding_vertical: Option<f32>,
    /// Gap between icon and label in logical pixels.
    pub icon_spacing: Option<f32>,
}

impl_merge!(MenuItemMetrics {
    option { height, padding_horizontal, padding_vertical, icon_spacing }
});

/// Tooltip metrics.
///
/// Defines inner padding and maximum width for tooltips.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct TooltipMetrics {
    /// Inner padding in logical pixels.
    pub padding: Option<f32>,
    /// Maximum tooltip width in logical pixels.
    pub max_width: Option<f32>,
}

impl_merge!(TooltipMetrics {
    option { padding, max_width }
});

/// List item / row metrics.
///
/// Defines row height and padding for list views and tables.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ListItemMetrics {
    /// Row height in logical pixels.
    pub height: Option<f32>,
    /// Horizontal padding inside the list item.
    pub padding_horizontal: Option<f32>,
    /// Vertical padding inside the list item.
    pub padding_vertical: Option<f32>,
}

impl_merge!(ListItemMetrics {
    option { height, padding_horizontal, padding_vertical }
});

/// Toolbar metrics.
///
/// Defines toolbar height, item spacing, and inner padding.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ToolbarMetrics {
    /// Toolbar height in logical pixels.
    pub height: Option<f32>,
    /// Gap between toolbar items in logical pixels.
    pub item_spacing: Option<f32>,
    /// Inner padding in logical pixels.
    pub padding: Option<f32>,
}

impl_merge!(ToolbarMetrics {
    option { height, item_spacing, padding }
});

/// Splitter/divider metrics.
///
/// Defines the handle width/thickness for split pane dividers.
/// All values are in logical pixels.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct SplitterMetrics {
    /// Splitter handle width/thickness in logical pixels.
    pub width: Option<f32>,
}

impl_merge!(SplitterMetrics {
    option { width }
});

/// Per-widget sizing and spacing metrics.
///
/// Contains sub-structs for each widget type with platform-specific
/// dimensions. All sub-structs are nested (not Option) because they
/// default to empty (all fields None). Empty sub-structs are omitted
/// from serialized output.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WidgetMetrics {
    /// Button sizing and spacing.
    #[serde(default, skip_serializing_if = "ButtonMetrics::is_empty")]
    pub button: ButtonMetrics,

    /// Checkbox and radio button sizing.
    #[serde(default, skip_serializing_if = "CheckboxMetrics::is_empty")]
    pub checkbox: CheckboxMetrics,

    /// Text input field sizing.
    #[serde(default, skip_serializing_if = "InputMetrics::is_empty")]
    pub input: InputMetrics,

    /// Scrollbar sizing.
    #[serde(default, skip_serializing_if = "ScrollbarMetrics::is_empty")]
    pub scrollbar: ScrollbarMetrics,

    /// Slider/range control sizing.
    #[serde(default, skip_serializing_if = "SliderMetrics::is_empty")]
    pub slider: SliderMetrics,

    /// Progress bar sizing.
    #[serde(default, skip_serializing_if = "ProgressBarMetrics::is_empty")]
    pub progress_bar: ProgressBarMetrics,

    /// Tab bar sizing.
    #[serde(default, skip_serializing_if = "TabMetrics::is_empty")]
    pub tab: TabMetrics,

    /// Menu item sizing.
    #[serde(default, skip_serializing_if = "MenuItemMetrics::is_empty")]
    pub menu_item: MenuItemMetrics,

    /// Tooltip sizing.
    #[serde(default, skip_serializing_if = "TooltipMetrics::is_empty")]
    pub tooltip: TooltipMetrics,

    /// List item / row sizing.
    #[serde(default, skip_serializing_if = "ListItemMetrics::is_empty")]
    pub list_item: ListItemMetrics,

    /// Toolbar sizing.
    #[serde(default, skip_serializing_if = "ToolbarMetrics::is_empty")]
    pub toolbar: ToolbarMetrics,

    /// Splitter/divider sizing.
    #[serde(default, skip_serializing_if = "SplitterMetrics::is_empty")]
    pub splitter: SplitterMetrics,
}

impl_merge!(WidgetMetrics {
    nested { button, checkbox, input, scrollbar, slider, progress_bar, tab, menu_item, tooltip, list_item, toolbar, splitter }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        assert!(WidgetMetrics::default().is_empty());
    }

    #[test]
    fn not_empty_when_button_field_set() {
        let mut wm = WidgetMetrics::default();
        wm.button.min_width = Some(80.0);
        assert!(!wm.is_empty());
    }

    #[test]
    fn merge_overlays_some_fields_preserves_none() {
        let mut base = WidgetMetrics::default();
        base.button.min_width = Some(80.0);
        base.button.min_height = Some(30.0);

        let mut overlay = WidgetMetrics::default();
        overlay.button.min_width = Some(100.0); // override
        overlay.button.padding_horizontal = Some(12.0); // new field

        base.merge(&overlay);

        // overlay value replaces base
        assert_eq!(base.button.min_width, Some(100.0));
        // base value preserved when overlay is None
        assert_eq!(base.button.min_height, Some(30.0));
        // new overlay field applied
        assert_eq!(base.button.padding_horizontal, Some(12.0));
    }

    #[test]
    fn merge_nested_sub_struct_recursion() {
        let mut base = WidgetMetrics::default();
        base.button.min_width = Some(80.0);
        base.scrollbar.width = Some(21.0);

        let mut overlay = WidgetMetrics::default();
        overlay.button.padding_horizontal = Some(6.0);
        overlay.scrollbar.slider_width = Some(8.0);

        base.merge(&overlay);

        // button: base preserved, overlay applied
        assert_eq!(base.button.min_width, Some(80.0));
        assert_eq!(base.button.padding_horizontal, Some(6.0));
        // scrollbar: base preserved, overlay applied
        assert_eq!(base.scrollbar.width, Some(21.0));
        assert_eq!(base.scrollbar.slider_width, Some(8.0));
    }

    #[test]
    fn serde_toml_round_trip_all_populated() {
        let wm = WidgetMetrics {
            button: ButtonMetrics {
                min_width: Some(80.0),
                min_height: Some(30.0),
                padding_horizontal: Some(6.0),
                padding_vertical: Some(4.0),
                icon_spacing: Some(4.0),
            },
            checkbox: CheckboxMetrics {
                indicator_size: Some(20.0),
                spacing: Some(4.0),
            },
            input: InputMetrics {
                min_height: Some(30.0),
                padding_horizontal: Some(6.0),
                padding_vertical: Some(4.0),
            },
            scrollbar: ScrollbarMetrics {
                width: Some(21.0),
                min_thumb_height: Some(20.0),
                slider_width: Some(8.0),
            },
            slider: SliderMetrics {
                track_height: Some(6.0),
                thumb_size: Some(20.0),
                tick_length: Some(8.0),
            },
            progress_bar: ProgressBarMetrics {
                height: Some(6.0),
                min_width: Some(14.0),
            },
            tab: TabMetrics {
                min_width: Some(80.0),
                min_height: Some(30.0),
                padding_horizontal: Some(8.0),
                padding_vertical: Some(4.0),
            },
            menu_item: MenuItemMetrics {
                height: Some(22.0),
                padding_horizontal: Some(4.0),
                padding_vertical: Some(4.0),
                icon_spacing: Some(8.0),
            },
            tooltip: TooltipMetrics {
                padding: Some(3.0),
                max_width: Some(300.0),
            },
            list_item: ListItemMetrics {
                height: Some(24.0),
                padding_horizontal: Some(2.0),
                padding_vertical: Some(1.0),
            },
            toolbar: ToolbarMetrics {
                height: Some(38.0),
                item_spacing: Some(0.0),
                padding: Some(6.0),
            },
            splitter: SplitterMetrics {
                width: Some(1.0),
            },
        };

        let toml_str = toml::to_string(&wm).unwrap();
        let deserialized: WidgetMetrics = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, wm);
    }

    #[test]
    fn serde_toml_round_trip_sparse() {
        let mut wm = WidgetMetrics::default();
        wm.button.min_width = Some(80.0);
        wm.button.min_height = Some(30.0);

        let toml_str = toml::to_string(&wm).unwrap();
        let deserialized: WidgetMetrics = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, wm);
        assert_eq!(deserialized.button.min_width, Some(80.0));
        // Other sub-structs remain empty
        assert!(deserialized.checkbox.is_empty());
        assert!(deserialized.scrollbar.is_empty());
    }

    #[test]
    fn empty_sub_structs_omitted_from_serialized_output() {
        let mut wm = WidgetMetrics::default();
        wm.button.min_width = Some(80.0);

        let toml_str = toml::to_string(&wm).unwrap();

        // Only button should appear in output
        assert!(toml_str.contains("button"));
        // Empty sub-structs should not appear
        assert!(!toml_str.contains("checkbox"));
        assert!(!toml_str.contains("scrollbar"));
        assert!(!toml_str.contains("slider"));
        assert!(!toml_str.contains("progress_bar"));
        assert!(!toml_str.contains("tab"));
        assert!(!toml_str.contains("menu_item"));
        assert!(!toml_str.contains("tooltip"));
        assert!(!toml_str.contains("list_item"));
        assert!(!toml_str.contains("toolbar"));
        assert!(!toml_str.contains("splitter"));
    }

    #[test]
    fn deserialize_missing_widget_metrics_produces_default() {
        // Simulate a TOML string that has no widget_metrics at all
        let toml_str = "";
        let deserialized: WidgetMetrics = toml::from_str(toml_str).unwrap();
        assert!(deserialized.is_empty());
        assert_eq!(deserialized, WidgetMetrics::default());
    }

    // === Individual sub-struct tests ===

    #[test]
    fn button_metrics_is_empty_and_merge() {
        assert!(ButtonMetrics::default().is_empty());

        let mut base = ButtonMetrics {
            min_width: Some(80.0),
            ..Default::default()
        };
        let overlay = ButtonMetrics {
            min_height: Some(30.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.min_width, Some(80.0));
        assert_eq!(base.min_height, Some(30.0));
        assert!(!base.is_empty());
    }

    #[test]
    fn checkbox_metrics_is_empty_and_merge() {
        assert!(CheckboxMetrics::default().is_empty());

        let mut base = CheckboxMetrics::default();
        let overlay = CheckboxMetrics {
            indicator_size: Some(20.0),
            spacing: Some(4.0),
        };
        base.merge(&overlay);
        assert_eq!(base.indicator_size, Some(20.0));
        assert_eq!(base.spacing, Some(4.0));
    }

    #[test]
    fn scrollbar_metrics_is_empty_and_merge() {
        assert!(ScrollbarMetrics::default().is_empty());

        let mut base = ScrollbarMetrics {
            width: Some(21.0),
            ..Default::default()
        };
        let overlay = ScrollbarMetrics {
            slider_width: Some(8.0),
            ..Default::default()
        };
        base.merge(&overlay);
        assert_eq!(base.width, Some(21.0));
        assert_eq!(base.slider_width, Some(8.0));
    }

    #[test]
    fn splitter_metrics_is_empty_and_merge() {
        assert!(SplitterMetrics::default().is_empty());

        let mut base = SplitterMetrics::default();
        let overlay = SplitterMetrics {
            width: Some(1.0),
        };
        base.merge(&overlay);
        assert_eq!(base.width, Some(1.0));
    }
}
