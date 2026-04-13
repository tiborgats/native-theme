// Theme validation: orchestrate defaults extraction, per-widget dispatch, range checks,
// and ResolvedTheme construction.
// Helper functions, range-check utilities, and ValidateNested trait live in validate_helpers.rs.

use super::validate_helpers::{
    self, DEFAULT_FONT_DPI, require, require_font, require_text_scale_entry,
};
use crate::model::ThemeMode;
use crate::model::resolved::{
    ResolvedDefaults, ResolvedIconSizes, ResolvedTextScale, ResolvedTheme,
};

impl ThemeMode {
    /// Convert this ThemeMode into a [`ResolvedTheme`] using the default DPI (96.0).
    ///
    /// Convenience wrapper around [`validate_with_dpi(DEFAULT_FONT_DPI)`](Self::validate_with_dpi).
    pub fn validate(&self) -> crate::Result<ResolvedTheme> {
        self.validate_with_dpi(DEFAULT_FONT_DPI)
    }

    /// Convert this ThemeMode into a [`ResolvedTheme`] with all fields guaranteed.
    ///
    /// Should be called after [`resolve()`](ThemeMode::resolve). Walks every field
    /// and collects missing (None) field paths, then validates that numeric values
    /// are within legal ranges (e.g., spacing >= 0, opacity 0..=1, font weight
    /// 100..=900). Returns `Ok(ResolvedTheme)` if all fields are populated
    /// and in range.
    ///
    /// # Arguments
    ///
    /// * `font_dpi` -- Font DPI for pt-to-px conversion (e.g. 96.0 on
    ///   Linux/Windows, 72.0 on macOS).
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::ResolutionIncomplete`] if any required fields
    /// are still `None` after resolution. Returns
    /// [`crate::Error::ResolutionInvalid`] if all fields are present but
    /// some numeric values fall outside allowed ranges.
    pub fn validate_with_dpi(&self, font_dpi: f32) -> crate::Result<ResolvedTheme> {
        let mut missing = Vec::new();
        let dpi = font_dpi;

        // --- defaults extraction ---
        let defaults_font = require_font(&self.defaults.font, "defaults.font", dpi, &mut missing);
        let defaults_line_height = require(
            &self.defaults.line_height,
            "defaults.line_height",
            &mut missing,
        );
        let defaults_mono_font = require_font(
            &self.defaults.mono_font,
            "defaults.mono_font",
            dpi,
            &mut missing,
        );

        let defaults_background = require(
            &self.defaults.background_color,
            "defaults.background_color",
            &mut missing,
        );
        let defaults_foreground = require(
            &self.defaults.text_color,
            "defaults.text_color",
            &mut missing,
        );
        let defaults_accent = require(
            &self.defaults.accent_color,
            "defaults.accent_color",
            &mut missing,
        );
        let defaults_accent_foreground = require(
            &self.defaults.accent_text_color,
            "defaults.accent_text_color",
            &mut missing,
        );
        let defaults_surface = require(
            &self.defaults.surface_color,
            "defaults.surface_color",
            &mut missing,
        );
        let defaults_border = require(
            &self.defaults.border.color,
            "defaults.border.color",
            &mut missing,
        );
        let defaults_muted = require(
            &self.defaults.muted_color,
            "defaults.muted_color",
            &mut missing,
        );
        let defaults_shadow = require(
            &self.defaults.shadow_color,
            "defaults.shadow_color",
            &mut missing,
        );
        let defaults_link = require(
            &self.defaults.link_color,
            "defaults.link_color",
            &mut missing,
        );
        let defaults_selection = require(
            &self.defaults.selection_background,
            "defaults.selection_background",
            &mut missing,
        );
        let defaults_selection_foreground = require(
            &self.defaults.selection_text_color,
            "defaults.selection_text_color",
            &mut missing,
        );
        let defaults_selection_inactive = require(
            &self.defaults.selection_inactive_background,
            "defaults.selection_inactive_background",
            &mut missing,
        );
        let defaults_disabled_foreground = require(
            &self.defaults.disabled_text_color,
            "defaults.disabled_text_color",
            &mut missing,
        );

        let defaults_danger = require(
            &self.defaults.danger_color,
            "defaults.danger_color",
            &mut missing,
        );
        let defaults_danger_foreground = require(
            &self.defaults.danger_text_color,
            "defaults.danger_text_color",
            &mut missing,
        );
        let defaults_warning = require(
            &self.defaults.warning_color,
            "defaults.warning_color",
            &mut missing,
        );
        let defaults_warning_foreground = require(
            &self.defaults.warning_text_color,
            "defaults.warning_text_color",
            &mut missing,
        );
        let defaults_success = require(
            &self.defaults.success_color,
            "defaults.success_color",
            &mut missing,
        );
        let defaults_success_foreground = require(
            &self.defaults.success_text_color,
            "defaults.success_text_color",
            &mut missing,
        );
        let defaults_info = require(
            &self.defaults.info_color,
            "defaults.info_color",
            &mut missing,
        );
        let defaults_info_foreground = require(
            &self.defaults.info_text_color,
            "defaults.info_text_color",
            &mut missing,
        );

        let defaults_radius = require(
            &self.defaults.border.corner_radius,
            "defaults.border.corner_radius",
            &mut missing,
        );
        let defaults_radius_lg = require(
            &self.defaults.border.corner_radius_lg,
            "defaults.border.corner_radius_lg",
            &mut missing,
        );
        let defaults_frame_width = require(
            &self.defaults.border.line_width,
            "defaults.border.line_width",
            &mut missing,
        );
        let defaults_disabled_opacity = require(
            &self.defaults.disabled_opacity,
            "defaults.disabled_opacity",
            &mut missing,
        );
        let defaults_border_opacity = require(
            &self.defaults.border.opacity,
            "defaults.border.opacity",
            &mut missing,
        );
        let defaults_shadow_enabled = require(
            &self.defaults.border.shadow_enabled,
            "defaults.border.shadow_enabled",
            &mut missing,
        );

        let defaults_focus_ring_color = require(
            &self.defaults.focus_ring_color,
            "defaults.focus_ring_color",
            &mut missing,
        );
        let defaults_focus_ring_width = require(
            &self.defaults.focus_ring_width,
            "defaults.focus_ring_width",
            &mut missing,
        );
        let defaults_focus_ring_offset = require(
            &self.defaults.focus_ring_offset,
            "defaults.focus_ring_offset",
            &mut missing,
        );

        let defaults_border_padding_h = require(
            &self.defaults.border.padding_horizontal,
            "defaults.border.padding_horizontal",
            &mut missing,
        );
        let defaults_border_padding_v = require(
            &self.defaults.border.padding_vertical,
            "defaults.border.padding_vertical",
            &mut missing,
        );
        let defaults_text_selection_background = require(
            &self.defaults.text_selection_background,
            "defaults.text_selection_background",
            &mut missing,
        );
        let defaults_text_selection_color = require(
            &self.defaults.text_selection_color,
            "defaults.text_selection_color",
            &mut missing,
        );

        let defaults_icon_sizes_toolbar = require(
            &self.defaults.icon_sizes.toolbar,
            "defaults.icon_sizes.toolbar",
            &mut missing,
        );
        let defaults_icon_sizes_small = require(
            &self.defaults.icon_sizes.small,
            "defaults.icon_sizes.small",
            &mut missing,
        );
        let defaults_icon_sizes_large = require(
            &self.defaults.icon_sizes.large,
            "defaults.icon_sizes.large",
            &mut missing,
        );
        let defaults_icon_sizes_dialog = require(
            &self.defaults.icon_sizes.dialog,
            "defaults.icon_sizes.dialog",
            &mut missing,
        );
        let defaults_icon_sizes_panel = require(
            &self.defaults.icon_sizes.panel,
            "defaults.icon_sizes.panel",
            &mut missing,
        );

        let ts_caption = require_text_scale_entry(
            &self.text_scale.caption,
            "text_scale.caption",
            dpi,
            &mut missing,
        );
        let ts_section_heading = require_text_scale_entry(
            &self.text_scale.section_heading,
            "text_scale.section_heading",
            dpi,
            &mut missing,
        );
        let ts_dialog_title = require_text_scale_entry(
            &self.text_scale.dialog_title,
            "text_scale.dialog_title",
            dpi,
            &mut missing,
        );
        let ts_display = require_text_scale_entry(
            &self.text_scale.display,
            "text_scale.display",
            dpi,
            &mut missing,
        );

        // --- construct defaults and text_scale structs (before range checks) ---
        use crate::model::border::ResolvedBorderSpec;
        let defaults = ResolvedDefaults {
            font: defaults_font,
            line_height: defaults_line_height,
            mono_font: defaults_mono_font,
            background_color: defaults_background,
            text_color: defaults_foreground,
            accent_color: defaults_accent,
            accent_text_color: defaults_accent_foreground,
            surface_color: defaults_surface,
            border: ResolvedBorderSpec {
                color: defaults_border,
                corner_radius: defaults_radius,
                corner_radius_lg: defaults_radius_lg,
                line_width: defaults_frame_width,
                opacity: defaults_border_opacity,
                shadow_enabled: defaults_shadow_enabled,
                padding_horizontal: defaults_border_padding_h,
                padding_vertical: defaults_border_padding_v,
            },
            muted_color: defaults_muted,
            shadow_color: defaults_shadow,
            link_color: defaults_link,
            selection_background: defaults_selection,
            selection_text_color: defaults_selection_foreground,
            selection_inactive_background: defaults_selection_inactive,
            text_selection_background: defaults_text_selection_background,
            text_selection_color: defaults_text_selection_color,
            disabled_text_color: defaults_disabled_foreground,
            danger_color: defaults_danger,
            danger_text_color: defaults_danger_foreground,
            warning_color: defaults_warning,
            warning_text_color: defaults_warning_foreground,
            success_color: defaults_success,
            success_text_color: defaults_success_foreground,
            info_color: defaults_info,
            info_text_color: defaults_info_foreground,
            disabled_opacity: defaults_disabled_opacity,
            focus_ring_color: defaults_focus_ring_color,
            focus_ring_width: defaults_focus_ring_width,
            focus_ring_offset: defaults_focus_ring_offset,
            icon_sizes: ResolvedIconSizes {
                toolbar: defaults_icon_sizes_toolbar,
                small: defaults_icon_sizes_small,
                large: defaults_icon_sizes_large,
                dialog: defaults_icon_sizes_dialog,
                panel: defaults_icon_sizes_panel,
            },
        };
        let text_scale = ResolvedTextScale {
            caption: ts_caption,
            section_heading: ts_section_heading,
            dialog_title: ts_dialog_title,
            display: ts_display,
        };

        // --- per-widget extraction (generated by define_widget_pair!) ---
        use crate::model::widgets::*;
        let window =
            ResolvedWindowTheme::validate_widget(&self.window, "window", dpi, &mut missing);
        let button =
            ResolvedButtonTheme::validate_widget(&self.button, "button", dpi, &mut missing);
        let input = ResolvedInputTheme::validate_widget(&self.input, "input", dpi, &mut missing);
        let checkbox =
            ResolvedCheckboxTheme::validate_widget(&self.checkbox, "checkbox", dpi, &mut missing);
        let menu = ResolvedMenuTheme::validate_widget(&self.menu, "menu", dpi, &mut missing);
        let tooltip =
            ResolvedTooltipTheme::validate_widget(&self.tooltip, "tooltip", dpi, &mut missing);
        let scrollbar = ResolvedScrollbarTheme::validate_widget(
            &self.scrollbar,
            "scrollbar",
            dpi,
            &mut missing,
        );
        let slider =
            ResolvedSliderTheme::validate_widget(&self.slider, "slider", dpi, &mut missing);
        let progress_bar = ResolvedProgressBarTheme::validate_widget(
            &self.progress_bar,
            "progress_bar",
            dpi,
            &mut missing,
        );
        let tab = ResolvedTabTheme::validate_widget(&self.tab, "tab", dpi, &mut missing);
        let sidebar =
            ResolvedSidebarTheme::validate_widget(&self.sidebar, "sidebar", dpi, &mut missing);
        let toolbar =
            ResolvedToolbarTheme::validate_widget(&self.toolbar, "toolbar", dpi, &mut missing);
        let status_bar = ResolvedStatusBarTheme::validate_widget(
            &self.status_bar,
            "status_bar",
            dpi,
            &mut missing,
        );
        let list = ResolvedListTheme::validate_widget(&self.list, "list", dpi, &mut missing);
        let popover =
            ResolvedPopoverTheme::validate_widget(&self.popover, "popover", dpi, &mut missing);
        let splitter =
            ResolvedSplitterTheme::validate_widget(&self.splitter, "splitter", dpi, &mut missing);
        let separator = ResolvedSeparatorTheme::validate_widget(
            &self.separator,
            "separator",
            dpi,
            &mut missing,
        );
        let switch =
            ResolvedSwitchTheme::validate_widget(&self.switch, "switch", dpi, &mut missing);
        let dialog =
            ResolvedDialogTheme::validate_widget(&self.dialog, "dialog", dpi, &mut missing);
        let spinner =
            ResolvedSpinnerTheme::validate_widget(&self.spinner, "spinner", dpi, &mut missing);
        let combo_box =
            ResolvedComboBoxTheme::validate_widget(&self.combo_box, "combo_box", dpi, &mut missing);
        let segmented_control = ResolvedSegmentedControlTheme::validate_widget(
            &self.segmented_control,
            "segmented_control",
            dpi,
            &mut missing,
        );
        let card = ResolvedCardTheme::validate_widget(&self.card, "card", dpi, &mut missing);
        let expander =
            ResolvedExpanderTheme::validate_widget(&self.expander, "expander", dpi, &mut missing);
        let link = ResolvedLinkTheme::validate_widget(&self.link, "link", dpi, &mut missing);

        // --- Phase 1 short-circuit: if any fields are missing, return immediately ---
        // This is the BUG-01 fix: check_ranges never runs on placeholder data.
        if !missing.is_empty() {
            return Err(crate::Error::ResolutionIncomplete { missing });
        }

        // --- Phase 2: range checks on fully-validated data (separate Vec) ---
        let mut range_errors: Vec<crate::error::RangeViolation> = Vec::new();
        validate_helpers::check_defaults_ranges(&defaults, &text_scale, &mut range_errors);
        window.check_ranges("window", &mut range_errors);
        button.check_ranges("button", &mut range_errors);
        input.check_ranges("input", &mut range_errors);
        checkbox.check_ranges("checkbox", &mut range_errors);
        menu.check_ranges("menu", &mut range_errors);
        tooltip.check_ranges("tooltip", &mut range_errors);
        scrollbar.check_ranges("scrollbar", &mut range_errors);
        slider.check_ranges("slider", &mut range_errors);
        progress_bar.check_ranges("progress_bar", &mut range_errors);
        tab.check_ranges("tab", &mut range_errors);
        sidebar.check_ranges("sidebar", &mut range_errors);
        toolbar.check_ranges("toolbar", &mut range_errors);
        status_bar.check_ranges("status_bar", &mut range_errors);
        list.check_ranges("list", &mut range_errors);
        popover.check_ranges("popover", &mut range_errors);
        splitter.check_ranges("splitter", &mut range_errors);
        separator.check_ranges("separator", &mut range_errors);
        switch.check_ranges("switch", &mut range_errors);
        dialog.check_ranges("dialog", &mut range_errors);
        spinner.check_ranges("spinner", &mut range_errors);
        combo_box.check_ranges("combo_box", &mut range_errors);
        segmented_control.check_ranges("segmented_control", &mut range_errors);
        expander.check_ranges("expander", &mut range_errors);
        link.check_ranges("link", &mut range_errors);

        if !range_errors.is_empty() {
            return Err(crate::Error::ResolutionInvalid {
                errors: range_errors,
            });
        }

        Ok(ResolvedTheme {
            defaults,
            text_scale,
            window,
            button,
            input,
            checkbox,
            menu,
            tooltip,
            scrollbar,
            slider,
            progress_bar,
            tab,
            sidebar,
            toolbar,
            status_bar,
            list,
            popover,
            splitter,
            separator,
            switch,
            dialog,
            spinner,
            combo_box,
            segmented_control,
            card,
            expander,
            link,
        })
    }
}
