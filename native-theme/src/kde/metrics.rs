// KDE Breeze widget metrics from breezemetrics.h constants
// Populates per-widget sizing fields directly on ThemeVariant.

/// Populate per-widget sizing fields with Breeze metrics constants.
///
/// All values are in logical pixels (integers from breezemetrics.h, cast to f32).
/// Each value is annotated with the corresponding breezemetrics.h constant name.
pub(crate) fn populate_widget_sizing(variant: &mut crate::ThemeVariant) {
    // Button
    variant.button.min_width = Some(80.0);         // Button_MinWidth
    variant.button.padding_horizontal = Some(6.0); // Button_MarginWidth
    variant.button.icon_spacing = Some(4.0);       // Button_ItemSpacing

    // Checkbox
    variant.checkbox.indicator_size = Some(20.0);   // CheckBox_Size
    variant.checkbox.spacing = Some(4.0);           // CheckBox_ItemSpacing

    // Input
    variant.input.padding_horizontal = Some(6.0);  // LineEdit_FrameWidth

    // Scrollbar
    variant.scrollbar.width = Some(21.0);           // ScrollBar_Extend
    variant.scrollbar.min_thumb_height = Some(20.0); // ScrollBar_MinSliderHeight
    variant.scrollbar.slider_width = Some(8.0);     // ScrollBar_SliderWidth

    // Slider
    variant.slider.track_height = Some(6.0);        // Slider_GrooveThickness
    variant.slider.thumb_size = Some(20.0);          // Slider_ControlThickness
    variant.slider.tick_length = Some(8.0);          // Slider_TickLength

    // Progress bar
    variant.progress_bar.height = Some(6.0);         // ProgressBar_Thickness
    variant.progress_bar.min_width = Some(14.0);     // ProgressBar_BusyIndicatorSize

    // Tab
    variant.tab.min_width = Some(80.0);              // TabBar_TabMinWidth
    variant.tab.min_height = Some(30.0);             // TabBar_TabMinHeight
    variant.tab.padding_horizontal = Some(8.0);      // TabBar_TabMarginWidth
    variant.tab.padding_vertical = Some(4.0);        // TabBar_TabMarginHeight

    // Menu
    variant.menu.padding_horizontal = Some(4.0);     // MenuItem_MarginWidth
    variant.menu.padding_vertical = Some(4.0);        // MenuItem_MarginHeight
    variant.menu.icon_spacing = Some(8.0);            // MenuItem_TextLeftMargin

    // Tooltip
    variant.tooltip.padding_horizontal = Some(3.0);   // ToolTip_FrameWidth
    variant.tooltip.padding_vertical = Some(3.0);      // ToolTip_FrameWidth

    // List
    variant.list.padding_horizontal = Some(2.0);     // ItemView_ItemMarginLeft
    variant.list.padding_vertical = Some(1.0);       // ItemView_ItemMarginTop

    // Toolbar
    variant.toolbar.item_spacing = Some(0.0);         // ToolBar_ItemSpacing
    variant.toolbar.padding = Some(6.0);              // ToolBar_ItemMargin

    // Splitter
    variant.splitter.width = Some(1.0);               // Splitter_SplitterWidth
}

#[cfg(test)]
mod tests {
    use crate::ThemeVariant;

    #[test]
    fn breeze_metrics_populates_button_min_width() {
        let mut v = ThemeVariant::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.button.min_width, Some(80.0), "Button_MinWidth");
    }

    #[test]
    fn breeze_metrics_populates_checkbox_indicator_size() {
        let mut v = ThemeVariant::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.checkbox.indicator_size, Some(20.0), "CheckBox_Size");
    }

    #[test]
    fn breeze_metrics_populates_scrollbar_width() {
        let mut v = ThemeVariant::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.scrollbar.width, Some(21.0), "ScrollBar_Extend");
    }

    #[test]
    fn breeze_metrics_populates_slider_thumb() {
        let mut v = ThemeVariant::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.slider.thumb_size, Some(20.0), "Slider_ControlThickness");
    }

    #[test]
    fn breeze_metrics_populates_splitter() {
        let mut v = ThemeVariant::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.splitter.width, Some(1.0), "Splitter_SplitterWidth");
    }

    #[test]
    fn breeze_metrics_variant_not_empty() {
        let mut v = ThemeVariant::default();
        super::populate_widget_sizing(&mut v);
        assert!(!v.is_empty(), "variant should not be empty after widget sizing");
    }
}
