// KDE Breeze widget metrics from breezemetrics.h constants

use crate::model::widget_metrics::*;

/// Return widget metrics populated from KDE Breeze breezemetrics.h constants.
///
/// All values are in logical pixels (integers from breezemetrics.h, cast to f32).
/// Each value is annotated with the corresponding breezemetrics.h constant name.
pub(crate) fn breeze_widget_metrics() -> WidgetMetrics {
    WidgetMetrics {
        button: ButtonMetrics {
            min_width: Some(80.0),           // Button_MinWidth
            padding_horizontal: Some(6.0),   // Button_MarginWidth
            icon_spacing: Some(4.0),         // Button_ItemSpacing
            ..Default::default()
        },
        checkbox: CheckboxMetrics {
            indicator_size: Some(20.0), // CheckBox_Size
            spacing: Some(4.0),        // CheckBox_ItemSpacing
            ..Default::default()
        },
        input: InputMetrics {
            padding_horizontal: Some(6.0), // LineEdit_FrameWidth
            ..Default::default()
        },
        scrollbar: ScrollbarMetrics {
            width: Some(21.0),            // ScrollBar_Extend
            min_thumb_height: Some(20.0), // ScrollBar_MinSliderHeight
            slider_width: Some(8.0),      // ScrollBar_SliderWidth
        },
        slider: SliderMetrics {
            track_height: Some(6.0),  // Slider_GrooveThickness
            thumb_size: Some(20.0),   // Slider_ControlThickness
            tick_length: Some(8.0),   // Slider_TickLength
        },
        progress_bar: ProgressBarMetrics {
            height: Some(6.0),      // ProgressBar_Thickness
            min_width: Some(14.0),  // ProgressBar_BusyIndicatorSize
        },
        tab: TabMetrics {
            min_width: Some(80.0),           // TabBar_TabMinWidth
            min_height: Some(30.0),          // TabBar_TabMinHeight
            padding_horizontal: Some(8.0),   // TabBar_TabMarginWidth
            padding_vertical: Some(4.0),     // TabBar_TabMarginHeight
        },
        menu_item: MenuItemMetrics {
            padding_horizontal: Some(4.0), // MenuItem_MarginWidth
            padding_vertical: Some(4.0),   // MenuItem_MarginHeight
            icon_spacing: Some(8.0),       // MenuItem_TextLeftMargin
            ..Default::default()
        },
        tooltip: TooltipMetrics {
            padding: Some(3.0), // ToolTip_FrameWidth
            ..Default::default()
        },
        list_item: ListItemMetrics {
            padding_horizontal: Some(2.0), // ItemView_ItemMarginLeft
            padding_vertical: Some(1.0),   // ItemView_ItemMarginTop
            ..Default::default()
        },
        toolbar: ToolbarMetrics {
            item_spacing: Some(0.0), // ToolBar_ItemSpacing
            padding: Some(6.0),     // ToolBar_ItemMargin
            ..Default::default()
        },
        splitter: SplitterMetrics {
            width: Some(1.0), // Splitter_SplitterWidth
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::model::widget_metrics::*;

    #[test]
    fn breeze_metrics_not_empty() {
        let wm = super::breeze_widget_metrics();
        assert!(!wm.is_empty(), "breeze widget metrics should not be empty");
    }

    #[test]
    fn breeze_metrics_spot_check() {
        let wm = super::breeze_widget_metrics();
        assert_eq!(wm.button.min_width, Some(80.0), "Button_MinWidth");
        assert_eq!(wm.checkbox.indicator_size, Some(20.0), "CheckBox_Size");
        assert_eq!(wm.scrollbar.width, Some(21.0), "ScrollBar_Extend");
        assert_eq!(wm.slider.thumb_size, Some(20.0), "Slider_ControlThickness");
    }
}
