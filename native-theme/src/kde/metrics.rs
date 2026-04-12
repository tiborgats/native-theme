// KDE Breeze widget metrics from breezemetrics.h constants
// Populates per-widget sizing fields directly on ThemeMode.

/// Populate per-widget sizing fields with Breeze metrics constants.
///
/// All values are in logical pixels (integers from breezemetrics.h, cast to f32).
/// Each value is annotated with the corresponding breezemetrics.h constant name.
pub(crate) fn populate_widget_sizing(variant: &mut crate::ThemeMode) {
    // Button
    variant.button.min_width = Some(80.0); // Button_MinWidth
    variant
        .button
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(6.0); // Button_MarginWidth
    variant
        .button
        .border
        .get_or_insert_default()
        .padding_vertical = Some(5.0); // Breeze measured frame+margin
    variant.button.icon_text_gap = Some(4.0); // Button_ItemSpacing

    // Checkbox
    variant.checkbox.indicator_width = Some(20.0); // CheckBox_Size
    variant.checkbox.label_gap = Some(4.0); // CheckBox_ItemSpacing

    // Input
    variant
        .input
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(6.0); // LineEdit_FrameWidth
    variant
        .input
        .border
        .get_or_insert_default()
        .padding_vertical = Some(3.0); // Breeze measured frame

    // Scrollbar
    variant.scrollbar.groove_width = Some(21.0); // ScrollBar_Extend
    variant.scrollbar.min_thumb_length = Some(20.0); // ScrollBar_MinSliderHeight
    variant.scrollbar.thumb_width = Some(8.0); // ScrollBar_SliderWidth

    // Slider
    variant.slider.track_height = Some(6.0); // Slider_GrooveThickness
    variant.slider.thumb_diameter = Some(20.0); // Slider_ControlThickness
    variant.slider.tick_mark_length = Some(8.0); // Slider_TickLength

    // Progress bar
    variant.progress_bar.track_height = Some(6.0); // ProgressBar_Thickness
    // min_width: KDE has no native minimum (platform-facts §2.10).
    // ProgressBar_BusyIndicatorSize (14) is the busy-indicator animation
    // segment width, not a widget minimum. Preset provides the value.

    // Tab
    variant.tab.min_width = Some(80.0); // TabBar_TabMinWidth
    variant.tab.min_height = Some(30.0); // TabBar_TabMinHeight
    variant
        .tab
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(8.0); // TabBar_TabMarginWidth
    variant.tab.border.get_or_insert_default().padding_vertical = Some(4.0); // TabBar_TabMarginHeight

    // Menu
    variant
        .menu
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(4.0); // MenuItem_MarginWidth
    variant.menu.border.get_or_insert_default().padding_vertical = Some(4.0); // MenuItem_MarginHeight
    variant.menu.icon_text_gap = Some(8.0); // MenuItem_TextLeftMargin

    // Tooltip
    variant
        .tooltip
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(3.0); // ToolTip_FrameWidth
    variant
        .tooltip
        .border
        .get_or_insert_default()
        .padding_vertical = Some(3.0); // ToolTip_FrameWidth

    // List
    variant
        .list
        .border
        .get_or_insert_default()
        .padding_horizontal = Some(2.0); // ItemView_ItemMarginLeft
    variant.list.border.get_or_insert_default().padding_vertical = Some(1.0); // ItemView_ItemMarginTop

    // Toolbar
    variant.toolbar.item_gap = Some(0.0); // ToolBar_ItemSpacing

    // Splitter
    variant.splitter.divider_width = Some(1.0); // Splitter_SplitterWidth
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use crate::ThemeMode;

    #[test]
    fn breeze_metrics_populates_button_min_width() {
        let mut v = ThemeMode::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.button.min_width, Some(80.0), "Button_MinWidth");
    }

    #[test]
    fn breeze_metrics_populates_checkbox_indicator_width() {
        let mut v = ThemeMode::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.checkbox.indicator_width, Some(20.0), "CheckBox_Size");
    }

    #[test]
    fn breeze_metrics_populates_scrollbar_groove_width() {
        let mut v = ThemeMode::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(v.scrollbar.groove_width, Some(21.0), "ScrollBar_Extend");
    }

    #[test]
    fn breeze_metrics_populates_slider_thumb() {
        let mut v = ThemeMode::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(
            v.slider.thumb_diameter,
            Some(20.0),
            "Slider_ControlThickness"
        );
    }

    #[test]
    fn breeze_metrics_populates_splitter() {
        let mut v = ThemeMode::default();
        super::populate_widget_sizing(&mut v);
        assert_eq!(
            v.splitter.divider_width,
            Some(1.0),
            "Splitter_SplitterWidth"
        );
    }

    #[test]
    fn breeze_metrics_variant_not_empty() {
        let mut v = ThemeMode::default();
        super::populate_widget_sizing(&mut v);
        assert!(
            !v.is_empty(),
            "variant should not be empty after widget sizing"
        );
    }
}
