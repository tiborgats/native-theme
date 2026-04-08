// Resolution inheritance phases: fills None fields from defaults and widget-to-widget chains.

use crate::model::border::BorderSpec;
use crate::model::font::FontSize;
use crate::model::{DialogButtonOrder, FontSpec, TextScaleEntry, ThemeVariant};

/// Resolve a per-widget font from defaults.
/// If the widget font is None, clone defaults entirely.
/// If the widget font is Some but has None sub-fields, fill from defaults.
fn resolve_font(widget_font: &mut Option<FontSpec>, defaults_font: &FontSpec) {
    match widget_font {
        None => {
            *widget_font = Some(defaults_font.clone());
        }
        Some(font) => {
            if font.family.is_none() {
                font.family = defaults_font.family.clone();
            }
            if font.size.is_none() {
                font.size = defaults_font.size;
            }
            if font.weight.is_none() {
                font.weight = defaults_font.weight;
            }
            if font.style.is_none() {
                font.style = defaults_font.style;
            }
            if font.color.is_none() {
                font.color = defaults_font.color;
            }
        }
    }
}

/// Resolve a per-widget border from defaults.border.
/// If the widget border is None, creates it from defaults.
/// If Some, fills None sub-fields from defaults.
/// `use_lg_radius`: if true, uses corner_radius_lg instead of corner_radius.
/// padding_horizontal and padding_vertical are NOT inherited (sizing fields).
fn resolve_border(
    widget_border: &mut Option<BorderSpec>,
    defaults_border: &BorderSpec,
    use_lg_radius: bool,
) {
    let border = widget_border.get_or_insert_with(BorderSpec::default);
    if border.color.is_none() {
        border.color = defaults_border.color;
    }
    if border.corner_radius.is_none() {
        border.corner_radius = if use_lg_radius {
            defaults_border.corner_radius_lg
        } else {
            defaults_border.corner_radius
        };
    }
    if border.line_width.is_none() {
        border.line_width = defaults_border.line_width;
    }
    if border.shadow_enabled.is_none() {
        border.shadow_enabled = defaults_border.shadow_enabled;
    }
}

/// Resolve a text scale entry from defaults.
/// Creates the entry if None, fills sub-fields from defaults.font,
/// fills None sub-fields from defaults.font, then computes line_height
/// from defaults.line_height * resolved_size.
///
/// Per `inheritance-rules.toml [text_scale_inheritance]`:
/// - size inherits from defaults.font.size
/// - weight inherits from defaults.font.weight
fn resolve_text_scale_entry(
    entry: &mut Option<TextScaleEntry>,
    defaults_font: &FontSpec,
    defaults_line_height: Option<f32>,
) {
    let entry = entry.get_or_insert_with(TextScaleEntry::default);
    if entry.size.is_none() {
        entry.size = defaults_font.size;
    }
    if entry.weight.is_none() {
        entry.weight = defaults_font.weight;
    }
    if entry.line_height.is_none()
        && let (Some(lh_mult), Some(font_size)) = (defaults_line_height, entry.size)
    {
        entry.line_height = Some(match font_size {
            FontSize::Pt(_) => FontSize::Pt(lh_mult * font_size.raw()),
            FontSize::Px(_) => FontSize::Px(lh_mult * font_size.raw()),
        });
    }
}

/// Return the platform-appropriate dialog button order.
///
/// On Linux/KDE, the convention is leading-affirmative (OK on the left).
/// On Windows, GNOME, macOS, iOS the convention is trailing-affirmative.
fn platform_button_order() -> DialogButtonOrder {
    #[cfg(target_os = "linux")]
    {
        if crate::detect_linux_de(&crate::xdg_current_desktop()) == crate::LinuxDesktop::Kde {
            return DialogButtonOrder::PrimaryLeft;
        }
    }
    // Windows, GNOME, macOS, iOS all use primary-right
    DialogButtonOrder::PrimaryRight
}

impl ThemeVariant {
    // --- Phase 1: Defaults internal chains ---

    pub(crate) fn resolve_defaults_internal(&mut self) {
        let d = &mut self.defaults;

        // selection_background <- accent_color
        if d.selection_background.is_none() {
            d.selection_background = d.accent_color;
        }
        // focus_ring_color <- accent_color
        if d.focus_ring_color.is_none() {
            d.focus_ring_color = d.accent_color;
        }
        // selection_inactive_background <- selection_background (MUST run after selection_background is set)
        if d.selection_inactive_background.is_none() {
            d.selection_inactive_background = d.selection_background;
        }
        // text_selection_background <- selection_background (inline text highlight defaults to selection)
        if d.text_selection_background.is_none() {
            d.text_selection_background = d.selection_background;
        }
        // text_selection_color <- selection_text_color
        if d.text_selection_color.is_none() {
            d.text_selection_color = d.selection_text_color;
        }
        // defaults.font.color <- defaults.text_color
        if d.font.color.is_none() {
            d.font.color = d.text_color;
        }
        // defaults.mono_font.color <- defaults.font.color (MUST run AFTER font.color is set)
        if d.mono_font.color.is_none() {
            d.mono_font.color = d.font.color;
        }
        // defaults.border padding -- defaults-level border carries no meaningful
        // padding; per-widget border specs carry the actual padding values.
        // Derive from line_width presence to ensure the field is populated.
        if d.border.padding_horizontal.is_none() {
            d.border.padding_horizontal = d.border.line_width.map(|_| 0.0_f32);
            if d.border.padding_horizontal.is_none() {
                d.border.padding_horizontal = d.border.corner_radius.map(|_| 0.0_f32);
            }
        }
        if d.border.padding_vertical.is_none() {
            d.border.padding_vertical = d.border.line_width.map(|_| 0.0_f32);
            if d.border.padding_vertical.is_none() {
                d.border.padding_vertical = d.border.corner_radius.map(|_| 0.0_f32);
            }
        }
    }

    // --- Phase 2: Safety nets ---

    pub(crate) fn resolve_safety_nets(&mut self) {
        // dialog.button_order <- platform convention
        if self.dialog.button_order.is_none() {
            self.dialog.button_order = Some(platform_button_order());
        }
        // input.caret_color <- defaults.text_color
        if self.input.caret_color.is_none() {
            self.input.caret_color = self.defaults.text_color;
        }
        // scrollbar.track <- defaults.background_color
        if self.scrollbar.track_color.is_none() {
            self.scrollbar.track_color = self.defaults.background_color;
        }
        // spinner.fill <- defaults.accent_color (all platforms use accent)
        if self.spinner.fill_color.is_none() {
            self.spinner.fill_color = self.defaults.accent_color;
        }
        // popover.background <- defaults.background_color
        if self.popover.background_color.is_none() {
            self.popover.background_color = self.defaults.background_color;
        }
        // list.background <- defaults.background_color
        if self.list.background_color.is_none() {
            self.list.background_color = self.defaults.background_color;
        }
        // dialog.background <- defaults.background_color (per_platform fallback)
        if self.dialog.background_color.is_none() {
            self.dialog.background_color = self.defaults.background_color;
        }
    }

    // --- Phase 3: Widget-from-defaults ---

    pub(crate) fn resolve_widgets_from_defaults(&mut self) {
        self.resolve_color_inheritance();
        self.resolve_border_inheritance();
        self.resolve_font_inheritance();
        self.resolve_text_scale();
    }

    pub(crate) fn resolve_color_inheritance(&mut self) {
        let d = &self.defaults;

        // --- window ---
        if self.window.background_color.is_none() {
            self.window.background_color = d.background_color;
        }
        if self.window.title_bar_background.is_none() {
            self.window.title_bar_background = d.surface_color;
        }

        // --- button ---
        if self.button.background_color.is_none() {
            self.button.background_color = d.background_color;
        }
        if self.button.primary_background.is_none() {
            self.button.primary_background = d.accent_color;
        }
        if self.button.primary_text_color.is_none() {
            self.button.primary_text_color = d.accent_text_color;
        }
        if self.button.disabled_opacity.is_none() {
            self.button.disabled_opacity = d.disabled_opacity;
        }
        if self.button.hover_background.is_none() {
            self.button.hover_background = d.background_color;
        }
        if self.button.disabled_text_color.is_none() {
            self.button.disabled_text_color = d.disabled_text_color;
        }

        // --- input ---
        if self.input.background_color.is_none() {
            self.input.background_color = d.background_color;
        }
        if self.input.placeholder_color.is_none() {
            self.input.placeholder_color = d.muted_color;
        }
        if self.input.selection_background.is_none() {
            self.input.selection_background = d.text_selection_background;
        }
        if self.input.selection_text_color.is_none() {
            self.input.selection_text_color = d.text_selection_color;
        }
        if self.input.disabled_opacity.is_none() {
            self.input.disabled_opacity = d.disabled_opacity;
        }
        if self.input.disabled_text_color.is_none() {
            self.input.disabled_text_color = d.disabled_text_color;
        }

        // --- checkbox ---
        if self.checkbox.background_color.is_none() {
            self.checkbox.background_color = d.background_color;
        }
        if self.checkbox.checked_background.is_none() {
            self.checkbox.checked_background = d.accent_color;
        }
        if self.checkbox.indicator_color.is_none() {
            self.checkbox.indicator_color = d.text_color;
        }
        if self.checkbox.disabled_opacity.is_none() {
            self.checkbox.disabled_opacity = d.disabled_opacity;
        }
        if self.checkbox.disabled_text_color.is_none() {
            self.checkbox.disabled_text_color = d.disabled_text_color;
        }

        // --- menu ---
        if self.menu.background_color.is_none() {
            self.menu.background_color = d.background_color;
        }
        if self.menu.separator_color.is_none() {
            self.menu.separator_color = d.border.color;
        }
        if self.menu.icon_size.is_none() {
            self.menu.icon_size = d.icon_sizes.toolbar;
        }
        if self.menu.hover_background.is_none() {
            self.menu.hover_background = d.selection_background;
        }
        if self.menu.hover_text_color.is_none() {
            self.menu.hover_text_color = d.selection_text_color;
        }
        if self.menu.disabled_text_color.is_none() {
            self.menu.disabled_text_color = d.disabled_text_color;
        }

        // --- tooltip ---
        if self.tooltip.background_color.is_none() {
            self.tooltip.background_color = d.background_color;
        }

        // --- scrollbar ---
        if self.scrollbar.thumb_color.is_none() {
            self.scrollbar.thumb_color = d.muted_color;
        }
        // scrollbar.thumb_hover_color <- defaults.muted_color (per inheritance-rules.toml line 154)
        // Both thumb_color and thumb_hover_color inherit from defaults.muted_color independently.
        if self.scrollbar.thumb_hover_color.is_none() {
            self.scrollbar.thumb_hover_color = d.muted_color;
        }

        // --- slider ---
        if self.slider.fill_color.is_none() {
            self.slider.fill_color = d.accent_color;
        }
        if self.slider.track_color.is_none() {
            self.slider.track_color = d.muted_color;
        }
        if self.slider.thumb_color.is_none() {
            self.slider.thumb_color = d.surface_color;
        }
        if self.slider.disabled_opacity.is_none() {
            self.slider.disabled_opacity = d.disabled_opacity;
        }

        // --- progress_bar ---
        if self.progress_bar.fill_color.is_none() {
            self.progress_bar.fill_color = d.accent_color;
        }
        if self.progress_bar.track_color.is_none() {
            self.progress_bar.track_color = d.muted_color;
        }

        // --- tab ---
        if self.tab.background_color.is_none() {
            self.tab.background_color = d.background_color;
        }
        if self.tab.active_background.is_none() {
            self.tab.active_background = d.background_color;
        }
        if self.tab.active_text_color.is_none() {
            self.tab.active_text_color = d.text_color;
        }
        if self.tab.bar_background.is_none() {
            self.tab.bar_background = d.background_color;
        }

        // --- sidebar ---
        if self.sidebar.background_color.is_none() {
            self.sidebar.background_color = d.background_color;
        }
        if self.sidebar.selection_background.is_none() {
            self.sidebar.selection_background = d.selection_background;
        }
        if self.sidebar.selection_text_color.is_none() {
            self.sidebar.selection_text_color = d.selection_text_color;
        }
        if self.sidebar.hover_background.is_none() {
            self.sidebar.hover_background = d.background_color;
        }

        // --- toolbar ---
        if self.toolbar.background_color.is_none() {
            self.toolbar.background_color = d.background_color;
        }
        if self.toolbar.icon_size.is_none() {
            self.toolbar.icon_size = d.icon_sizes.toolbar;
        }

        // --- status_bar ---
        if self.status_bar.background_color.is_none() {
            self.status_bar.background_color = d.background_color;
        }

        // --- list ---
        // list.alternate_row_background: no fallback — all presets specify it
        // explicitly and validate() enforces its presence.
        if self.list.selection_background.is_none() {
            self.list.selection_background = d.selection_background;
        }
        if self.list.selection_text_color.is_none() {
            self.list.selection_text_color = d.selection_text_color;
        }
        if self.list.header_background.is_none() {
            self.list.header_background = d.surface_color;
        }
        if self.list.grid_color.is_none() {
            self.list.grid_color = d.border.color;
        }
        if self.list.hover_background.is_none() {
            self.list.hover_background = d.background_color;
        }
        if self.list.disabled_text_color.is_none() {
            self.list.disabled_text_color = d.disabled_text_color;
        }

        // --- splitter ---
        if self.splitter.divider_color.is_none() {
            self.splitter.divider_color = d.border.color;
        }

        // --- separator ---
        if self.separator.line_color.is_none() {
            self.separator.line_color = d.border.color;
        }
        if self.separator.line_width.is_none() {
            self.separator.line_width = d.border.line_width;
        }

        // --- switch ---
        if self.switch.checked_background.is_none() {
            self.switch.checked_background = d.accent_color;
        }
        // switch.unchecked_background: no inheritance -- each platform has
        // distinct off-track color (SPEC-3 fix, per no_inheritance spec)
        if self.switch.thumb_background.is_none() {
            self.switch.thumb_background = d.surface_color;
        }
        if self.switch.disabled_opacity.is_none() {
            self.switch.disabled_opacity = d.disabled_opacity;
        }

        // --- combo_box ---
        if self.combo_box.background_color.is_none() {
            self.combo_box.background_color = d.background_color;
        }
        if self.combo_box.disabled_opacity.is_none() {
            self.combo_box.disabled_opacity = d.disabled_opacity;
        }
        if self.combo_box.disabled_text_color.is_none() {
            self.combo_box.disabled_text_color = d.disabled_text_color;
        }

        // --- segmented_control ---
        if self.segmented_control.background_color.is_none() {
            self.segmented_control.background_color = d.background_color;
        }
        if self.segmented_control.active_background.is_none() {
            self.segmented_control.active_background = d.accent_color;
        }
        if self.segmented_control.active_text_color.is_none() {
            self.segmented_control.active_text_color = d.accent_text_color;
        }
        if self.segmented_control.disabled_opacity.is_none() {
            self.segmented_control.disabled_opacity = d.disabled_opacity;
        }

        // --- card ---
        if self.card.background_color.is_none() {
            self.card.background_color = d.surface_color;
        }
        // card.border: all sub-fields are platform-specific or (none) per §2.26
        // -- no inheritance from defaults.border (INH-3 fix)

        // --- link ---
        if self.link.visited_text_color.is_none() {
            self.link.visited_text_color = d.link_color;
        }
        if self.link.disabled_text_color.is_none() {
            self.link.disabled_text_color = d.disabled_text_color;
        }
    }

    pub(crate) fn resolve_border_inheritance(&mut self) {
        let defaults_border = &self.defaults.border;

        // 13 widgets with full border inheritance
        // corner_radius_lg widgets: window, popover, dialog
        resolve_border(&mut self.window.border, defaults_border, true);
        resolve_border(&mut self.button.border, defaults_border, false);
        resolve_border(&mut self.input.border, defaults_border, false);
        resolve_border(&mut self.checkbox.border, defaults_border, false);
        resolve_border(&mut self.tooltip.border, defaults_border, false);
        resolve_border(&mut self.progress_bar.border, defaults_border, false);
        resolve_border(&mut self.toolbar.border, defaults_border, false);
        resolve_border(&mut self.list.border, defaults_border, false);
        resolve_border(&mut self.popover.border, defaults_border, true);
        resolve_border(&mut self.dialog.border, defaults_border, true);
        resolve_border(&mut self.combo_box.border, defaults_border, false);
        resolve_border(&mut self.segmented_control.border, defaults_border, false);
        resolve_border(&mut self.expander.border, defaults_border, false);

        // Partial border: sidebar (color + line_width only)
        {
            let border = self.sidebar.border.get_or_insert_with(BorderSpec::default);
            if border.color.is_none() {
                border.color = defaults_border.color;
            }
            if border.line_width.is_none() {
                border.line_width = defaults_border.line_width;
            }
        }

        // Partial border: status_bar (color + line_width only)
        {
            let border = self
                .status_bar
                .border
                .get_or_insert_with(BorderSpec::default);
            if border.color.is_none() {
                border.color = defaults_border.color;
            }
            if border.line_width.is_none() {
                border.line_width = defaults_border.line_width;
            }
        }
    }

    pub(crate) fn resolve_font_inheritance(&mut self) {
        let defaults_font = &self.defaults.font;
        // 19 widget fonts
        resolve_font(&mut self.window.title_bar_font, defaults_font);
        resolve_font(&mut self.button.font, defaults_font);
        resolve_font(&mut self.input.font, defaults_font);
        resolve_font(&mut self.checkbox.font, defaults_font);
        resolve_font(&mut self.menu.font, defaults_font);
        resolve_font(&mut self.tooltip.font, defaults_font);
        resolve_font(&mut self.tab.font, defaults_font);
        resolve_font(&mut self.sidebar.font, defaults_font);
        resolve_font(&mut self.toolbar.font, defaults_font);
        resolve_font(&mut self.status_bar.font, defaults_font);
        resolve_font(&mut self.list.item_font, defaults_font);
        resolve_font(&mut self.list.header_font, defaults_font);
        resolve_font(&mut self.popover.font, defaults_font);
        resolve_font(&mut self.dialog.title_font, defaults_font);
        resolve_font(&mut self.dialog.body_font, defaults_font);
        resolve_font(&mut self.combo_box.font, defaults_font);
        resolve_font(&mut self.segmented_control.font, defaults_font);
        resolve_font(&mut self.expander.font, defaults_font);
        resolve_font(&mut self.link.font, defaults_font);

        // Exception: link.font.color inherits from defaults.link_color, NOT defaults.font.color
        // This MUST run AFTER the generic resolve_font call above
        if let Some(ref mut font) = self.link.font {
            font.color = self.defaults.link_color;
        }
    }

    pub(crate) fn resolve_text_scale(&mut self) {
        let defaults_font = &self.defaults.font;
        let defaults_lh = self.defaults.line_height;

        resolve_text_scale_entry(&mut self.text_scale.caption, defaults_font, defaults_lh);
        resolve_text_scale_entry(
            &mut self.text_scale.section_heading,
            defaults_font,
            defaults_lh,
        );
        resolve_text_scale_entry(
            &mut self.text_scale.dialog_title,
            defaults_font,
            defaults_lh,
        );
        resolve_text_scale_entry(&mut self.text_scale.display, defaults_font, defaults_lh);
    }

    // --- Phase 4: Widget-to-widget chains ---

    pub(crate) fn resolve_widget_to_widget(&mut self) {
        // inactive title bar <- active title bar
        if self.window.inactive_title_bar_background.is_none() {
            self.window.inactive_title_bar_background = self.window.title_bar_background;
        }
        if self.window.inactive_title_bar_text_color.is_none() {
            self.window.inactive_title_bar_text_color =
                self.window.title_bar_font.as_ref().and_then(|f| f.color);
        }
        // button.hover_text_color <- button.font.color (widget-to-widget)
        // Must run AFTER resolve_font_inheritance has populated button.font.color
        if self.button.hover_text_color.is_none() {
            self.button.hover_text_color = self.button.font.as_ref().and_then(|f| f.color);
        }
        // button.active_text_color <- button.font.color (widget-to-widget)
        if self.button.active_text_color.is_none() {
            self.button.active_text_color = self.button.font.as_ref().and_then(|f| f.color);
        }
        // tab.hover_text_color <- tab.font.color (widget-to-widget)
        if self.tab.hover_text_color.is_none() {
            self.tab.hover_text_color = self.tab.font.as_ref().and_then(|f| f.color);
        }
        // list.hover_text_color <- list.item_font.color (widget-to-widget)
        if self.list.hover_text_color.is_none() {
            self.list.hover_text_color = self.list.item_font.as_ref().and_then(|f| f.color);
        }
        // splitter.hover_color <- splitter.divider_color (widget-to-widget)
        if self.splitter.hover_color.is_none() {
            self.splitter.hover_color = self.splitter.divider_color;
        }
        // link.hover_text_color <- link.font.color (widget-to-widget)
        if self.link.hover_text_color.is_none() {
            self.link.hover_text_color = self.link.font.as_ref().and_then(|f| f.color);
        }
        // link.active_text_color <- link.font.color (widget-to-widget)
        if self.link.active_text_color.is_none() {
            self.link.active_text_color = self.link.font.as_ref().and_then(|f| f.color);
        }
    }
}
