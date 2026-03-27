// Resolution engine: resolve() fills inheritance rules, validate() produces ResolvedTheme.

use crate::model::{FontSpec, TextScaleEntry, ThemeVariant};

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
        }
    }
}

/// Resolve a text scale entry from defaults.
/// Creates the entry if None, fills sub-fields from defaults.font,
/// computes line_height from defaults.line_height * resolved_size.
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
    if entry.line_height.is_none() {
        if let (Some(lh_mult), Some(size)) = (defaults_line_height, entry.size) {
            entry.line_height = Some(lh_mult * size);
        }
    }
}

impl ThemeVariant {
    /// Apply all ~90 inheritance rules in 4-phase order.
    ///
    /// After calling resolve(), most Option fields that were None will be filled
    /// from defaults or related widget fields. Calling resolve() twice produces
    /// the same result (idempotent).
    ///
    /// # Phases
    ///
    /// 1. **Defaults internal chains** -- accent derives selection, focus_ring_color;
    ///    selection derives selection_inactive.
    /// 2. **Safety nets** -- platform-divergent fields get a reasonable fallback.
    /// 3. **Widget-from-defaults** -- colors, geometry, fonts, text scale entries
    ///    all inherit from defaults.
    /// 4. **Widget-to-widget** -- inactive title bar fields fall back to active.
    pub fn resolve(&mut self) {
        self.resolve_defaults_internal();
        self.resolve_safety_nets();
        self.resolve_widgets_from_defaults();
        self.resolve_widget_to_widget();
    }

    // --- Phase 1: Defaults internal chains ---

    fn resolve_defaults_internal(&mut self) {
        let d = &mut self.defaults;

        // selection <- accent
        if d.selection.is_none() {
            d.selection = d.accent;
        }
        // focus_ring_color <- accent
        if d.focus_ring_color.is_none() {
            d.focus_ring_color = d.accent;
        }
        // selection_inactive <- selection (MUST run after selection is set)
        if d.selection_inactive.is_none() {
            d.selection_inactive = d.selection;
        }
    }

    // --- Phase 2: Safety nets ---

    fn resolve_safety_nets(&mut self) {
        // input.caret <- defaults.foreground
        if self.input.caret.is_none() {
            self.input.caret = self.defaults.foreground;
        }
        // scrollbar.track <- defaults.background
        if self.scrollbar.track.is_none() {
            self.scrollbar.track = self.defaults.background;
        }
        // spinner.fill <- defaults.foreground
        if self.spinner.fill.is_none() {
            self.spinner.fill = self.defaults.foreground;
        }
        // popover.background <- defaults.background
        if self.popover.background.is_none() {
            self.popover.background = self.defaults.background;
        }
        // list.background <- defaults.background
        if self.list.background.is_none() {
            self.list.background = self.defaults.background;
        }
    }

    // --- Phase 3: Widget-from-defaults ---

    fn resolve_widgets_from_defaults(&mut self) {
        self.resolve_color_inheritance();
        self.resolve_font_inheritance();
        self.resolve_text_scale();
    }

    fn resolve_color_inheritance(&mut self) {
        let d = &self.defaults;

        // --- window ---
        if self.window.background.is_none() { self.window.background = d.background; }
        if self.window.foreground.is_none() { self.window.foreground = d.foreground; }
        if self.window.border.is_none() { self.window.border = d.border; }
        if self.window.title_bar_background.is_none() { self.window.title_bar_background = d.surface; }
        if self.window.title_bar_foreground.is_none() { self.window.title_bar_foreground = d.foreground; }
        if self.window.radius.is_none() { self.window.radius = d.radius_lg; }
        if self.window.shadow.is_none() { self.window.shadow = d.shadow_enabled; }

        // --- button ---
        if self.button.background.is_none() { self.button.background = d.background; }
        if self.button.foreground.is_none() { self.button.foreground = d.foreground; }
        if self.button.border.is_none() { self.button.border = d.border; }
        if self.button.primary_bg.is_none() { self.button.primary_bg = d.accent; }
        if self.button.primary_fg.is_none() { self.button.primary_fg = d.accent_foreground; }
        if self.button.radius.is_none() { self.button.radius = d.radius; }
        if self.button.disabled_opacity.is_none() { self.button.disabled_opacity = d.disabled_opacity; }
        if self.button.shadow.is_none() { self.button.shadow = d.shadow_enabled; }

        // --- input ---
        if self.input.background.is_none() { self.input.background = d.background; }
        if self.input.foreground.is_none() { self.input.foreground = d.foreground; }
        if self.input.border.is_none() { self.input.border = d.border; }
        if self.input.placeholder.is_none() { self.input.placeholder = d.muted; }
        if self.input.selection.is_none() { self.input.selection = d.selection; }
        if self.input.selection_foreground.is_none() { self.input.selection_foreground = d.selection_foreground; }
        if self.input.radius.is_none() { self.input.radius = d.radius; }
        if self.input.border_width.is_none() { self.input.border_width = d.frame_width; }

        // --- checkbox ---
        if self.checkbox.checked_bg.is_none() { self.checkbox.checked_bg = d.accent; }
        if self.checkbox.radius.is_none() { self.checkbox.radius = d.radius; }
        if self.checkbox.border_width.is_none() { self.checkbox.border_width = d.frame_width; }

        // --- menu ---
        if self.menu.background.is_none() { self.menu.background = d.background; }
        if self.menu.foreground.is_none() { self.menu.foreground = d.foreground; }
        if self.menu.separator.is_none() { self.menu.separator = d.border; }

        // --- tooltip ---
        if self.tooltip.background.is_none() { self.tooltip.background = d.background; }
        if self.tooltip.foreground.is_none() { self.tooltip.foreground = d.foreground; }
        if self.tooltip.radius.is_none() { self.tooltip.radius = d.radius; }

        // --- scrollbar ---
        if self.scrollbar.thumb.is_none() { self.scrollbar.thumb = d.muted; }
        if self.scrollbar.thumb_hover.is_none() { self.scrollbar.thumb_hover = d.muted; }

        // --- slider ---
        if self.slider.fill.is_none() { self.slider.fill = d.accent; }
        if self.slider.track.is_none() { self.slider.track = d.muted; }
        if self.slider.thumb.is_none() { self.slider.thumb = d.surface; }

        // --- progress_bar ---
        if self.progress_bar.fill.is_none() { self.progress_bar.fill = d.accent; }
        if self.progress_bar.track.is_none() { self.progress_bar.track = d.muted; }
        if self.progress_bar.radius.is_none() { self.progress_bar.radius = d.radius; }

        // --- tab ---
        if self.tab.background.is_none() { self.tab.background = d.background; }
        if self.tab.foreground.is_none() { self.tab.foreground = d.foreground; }
        if self.tab.active_background.is_none() { self.tab.active_background = d.background; }
        if self.tab.active_foreground.is_none() { self.tab.active_foreground = d.foreground; }
        if self.tab.bar_background.is_none() { self.tab.bar_background = d.background; }

        // --- sidebar ---
        if self.sidebar.background.is_none() { self.sidebar.background = d.background; }
        if self.sidebar.foreground.is_none() { self.sidebar.foreground = d.foreground; }

        // --- list ---
        if self.list.foreground.is_none() { self.list.foreground = d.foreground; }
        if self.list.alternate_row.is_none() { self.list.alternate_row = d.background; }
        if self.list.selection.is_none() { self.list.selection = d.selection; }
        if self.list.selection_foreground.is_none() { self.list.selection_foreground = d.selection_foreground; }
        if self.list.header_background.is_none() { self.list.header_background = d.surface; }
        if self.list.header_foreground.is_none() { self.list.header_foreground = d.foreground; }
        if self.list.grid_color.is_none() { self.list.grid_color = d.border; }

        // --- popover ---
        if self.popover.foreground.is_none() { self.popover.foreground = d.foreground; }
        if self.popover.border.is_none() { self.popover.border = d.border; }
        if self.popover.radius.is_none() { self.popover.radius = d.radius_lg; }

        // --- separator ---
        if self.separator.color.is_none() { self.separator.color = d.border; }

        // --- switch ---
        if self.switch.checked_bg.is_none() { self.switch.checked_bg = d.accent; }
        if self.switch.thumb_bg.is_none() { self.switch.thumb_bg = d.surface; }

        // --- dialog ---
        if self.dialog.radius.is_none() { self.dialog.radius = d.radius_lg; }

        // --- combo_box ---
        if self.combo_box.radius.is_none() { self.combo_box.radius = d.radius; }

        // --- segmented_control ---
        if self.segmented_control.radius.is_none() { self.segmented_control.radius = d.radius; }

        // --- card ---
        if self.card.background.is_none() { self.card.background = d.surface; }
        if self.card.border.is_none() { self.card.border = d.border; }
        if self.card.radius.is_none() { self.card.radius = d.radius_lg; }
        if self.card.shadow.is_none() { self.card.shadow = d.shadow_enabled; }

        // --- expander ---
        if self.expander.radius.is_none() { self.expander.radius = d.radius; }

        // --- link ---
        if self.link.color.is_none() { self.link.color = d.link; }
        if self.link.visited.is_none() { self.link.visited = d.link; }
    }

    fn resolve_font_inheritance(&mut self) {
        let defaults_font = &self.defaults.font.clone();
        resolve_font(&mut self.window.title_bar_font, defaults_font);
        resolve_font(&mut self.button.font, defaults_font);
        resolve_font(&mut self.input.font, defaults_font);
        resolve_font(&mut self.menu.font, defaults_font);
        resolve_font(&mut self.tooltip.font, defaults_font);
        resolve_font(&mut self.toolbar.font, defaults_font);
        resolve_font(&mut self.status_bar.font, defaults_font);
        resolve_font(&mut self.dialog.title_font, defaults_font);
    }

    fn resolve_text_scale(&mut self) {
        let defaults_font = &self.defaults.font.clone();
        let defaults_lh = self.defaults.line_height;
        resolve_text_scale_entry(&mut self.text_scale.caption, defaults_font, defaults_lh);
        resolve_text_scale_entry(&mut self.text_scale.section_heading, defaults_font, defaults_lh);
        resolve_text_scale_entry(&mut self.text_scale.dialog_title, defaults_font, defaults_lh);
        resolve_text_scale_entry(&mut self.text_scale.display, defaults_font, defaults_lh);
    }

    // --- Phase 4: Widget-to-widget chains ---

    fn resolve_widget_to_widget(&mut self) {
        // inactive title bar <- active title bar
        if self.window.inactive_title_bar_background.is_none() {
            self.window.inactive_title_bar_background = self.window.title_bar_background;
        }
        if self.window.inactive_title_bar_foreground.is_none() {
            self.window.inactive_title_bar_foreground = self.window.title_bar_foreground;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;
    use crate::model::FontSpec;

    /// Helper: build a ThemeVariant with all defaults.* fields populated.
    fn variant_with_defaults() -> ThemeVariant {
        let c1 = Rgba::rgb(0, 120, 215);   // accent
        let c2 = Rgba::rgb(255, 255, 255);  // background
        let c3 = Rgba::rgb(30, 30, 30);     // foreground
        let c4 = Rgba::rgb(240, 240, 240);  // surface
        let c5 = Rgba::rgb(200, 200, 200);  // border
        let c6 = Rgba::rgb(128, 128, 128);  // muted
        let c7 = Rgba::rgb(0, 0, 0);        // shadow
        let c8 = Rgba::rgb(0, 100, 200);    // link
        let c9 = Rgba::rgb(255, 255, 255);  // accent_foreground
        let c10 = Rgba::rgb(220, 53, 69);   // danger
        let c11 = Rgba::rgb(255, 255, 255); // danger_foreground
        let c12 = Rgba::rgb(240, 173, 78);  // warning
        let c13 = Rgba::rgb(30, 30, 30);    // warning_foreground
        let c14 = Rgba::rgb(40, 167, 69);   // success
        let c15 = Rgba::rgb(255, 255, 255); // success_foreground
        let c16 = Rgba::rgb(0, 120, 215);   // info
        let c17 = Rgba::rgb(255, 255, 255); // info_foreground

        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(c1);
        v.defaults.background = Some(c2);
        v.defaults.foreground = Some(c3);
        v.defaults.surface = Some(c4);
        v.defaults.border = Some(c5);
        v.defaults.muted = Some(c6);
        v.defaults.shadow = Some(c7);
        v.defaults.link = Some(c8);
        v.defaults.accent_foreground = Some(c9);
        v.defaults.disabled_foreground = Some(Rgba::rgb(160, 160, 160));
        v.defaults.danger = Some(c10);
        v.defaults.danger_foreground = Some(c11);
        v.defaults.warning = Some(c12);
        v.defaults.warning_foreground = Some(c13);
        v.defaults.success = Some(c14);
        v.defaults.success_foreground = Some(c15);
        v.defaults.info = Some(c16);
        v.defaults.info_foreground = Some(c17);

        v.defaults.radius = Some(4.0);
        v.defaults.radius_lg = Some(8.0);
        v.defaults.frame_width = Some(1.0);
        v.defaults.disabled_opacity = Some(0.5);
        v.defaults.border_opacity = Some(0.15);
        v.defaults.shadow_enabled = Some(true);

        v.defaults.focus_ring_width = Some(2.0);
        v.defaults.focus_ring_offset = Some(1.0);

        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        v.defaults.line_height = Some(1.4);
        v.defaults.mono_font = FontSpec {
            family: Some("JetBrains Mono".into()),
            size: Some(13.0),
            weight: Some(400),
        };

        v.defaults.spacing.xxs = Some(2.0);
        v.defaults.spacing.xs = Some(4.0);
        v.defaults.spacing.s = Some(6.0);
        v.defaults.spacing.m = Some(12.0);
        v.defaults.spacing.l = Some(18.0);
        v.defaults.spacing.xl = Some(24.0);
        v.defaults.spacing.xxl = Some(36.0);

        v.defaults.icon_sizes.toolbar = Some(24.0);
        v.defaults.icon_sizes.small = Some(16.0);
        v.defaults.icon_sizes.large = Some(32.0);
        v.defaults.icon_sizes.dialog = Some(22.0);
        v.defaults.icon_sizes.panel = Some(20.0);

        v.defaults.text_scaling_factor = Some(1.0);
        v.defaults.reduce_motion = Some(false);
        v.defaults.high_contrast = Some(false);
        v.defaults.reduce_transparency = Some(false);

        v
    }

    // ===== Phase 1: Defaults internal chains =====

    #[test]
    fn resolve_phase1_accent_fills_selection_and_focus_ring() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.resolve();
        assert_eq!(v.defaults.selection, Some(Rgba::rgb(0, 120, 215)));
        assert_eq!(v.defaults.focus_ring_color, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn resolve_phase1_selection_fills_selection_inactive() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.resolve();
        // selection_inactive should be set from selection (which was set from accent)
        assert_eq!(v.defaults.selection_inactive, Some(Rgba::rgb(0, 120, 215)));
    }

    #[test]
    fn resolve_phase1_explicit_selection_preserved() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.defaults.selection = Some(Rgba::rgb(100, 100, 100));
        v.resolve();
        // Explicit selection preserved
        assert_eq!(v.defaults.selection, Some(Rgba::rgb(100, 100, 100)));
        // selection_inactive inherits from the explicit selection
        assert_eq!(v.defaults.selection_inactive, Some(Rgba::rgb(100, 100, 100)));
    }

    // ===== Phase 2: Safety nets =====

    #[test]
    fn resolve_phase2_safety_nets() {
        let mut v = ThemeVariant::default();
        v.defaults.foreground = Some(Rgba::rgb(30, 30, 30));
        v.defaults.background = Some(Rgba::rgb(255, 255, 255));
        v.resolve();

        assert_eq!(v.input.caret, Some(Rgba::rgb(30, 30, 30)), "input.caret <- foreground");
        assert_eq!(v.scrollbar.track, Some(Rgba::rgb(255, 255, 255)), "scrollbar.track <- background");
        assert_eq!(v.spinner.fill, Some(Rgba::rgb(30, 30, 30)), "spinner.fill <- foreground");
        assert_eq!(v.popover.background, Some(Rgba::rgb(255, 255, 255)), "popover.background <- background");
        assert_eq!(v.list.background, Some(Rgba::rgb(255, 255, 255)), "list.background <- background");
    }

    // ===== Phase 3: Accent propagation (RESOLVE-06) =====

    #[test]
    fn resolve_phase3_accent_propagation() {
        let mut v = ThemeVariant::default();
        v.defaults.accent = Some(Rgba::rgb(0, 120, 215));
        v.resolve();

        assert_eq!(v.button.primary_bg, Some(Rgba::rgb(0, 120, 215)), "button.primary_bg <- accent");
        assert_eq!(v.checkbox.checked_bg, Some(Rgba::rgb(0, 120, 215)), "checkbox.checked_bg <- accent");
        assert_eq!(v.slider.fill, Some(Rgba::rgb(0, 120, 215)), "slider.fill <- accent");
        assert_eq!(v.progress_bar.fill, Some(Rgba::rgb(0, 120, 215)), "progress_bar.fill <- accent");
        assert_eq!(v.switch.checked_bg, Some(Rgba::rgb(0, 120, 215)), "switch.checked_bg <- accent");
    }

    // ===== Phase 3: Font sub-field inheritance (RESOLVE-04) =====

    #[test]
    fn resolve_phase3_font_subfield_inheritance() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        // Menu has a font with only size set
        v.menu.font = Some(FontSpec {
            family: None,
            size: Some(12.0),
            weight: None,
        });
        v.resolve();

        let menu_font = v.menu.font.as_ref().unwrap();
        assert_eq!(menu_font.family.as_deref(), Some("Inter"), "family from defaults");
        assert_eq!(menu_font.size, Some(12.0), "explicit size preserved");
        assert_eq!(menu_font.weight, Some(400), "weight from defaults");
    }

    #[test]
    fn resolve_phase3_font_entire_inheritance() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        // button.font is None, should inherit entire defaults.font
        assert!(v.button.font.is_none());
        v.resolve();

        let button_font = v.button.font.as_ref().unwrap();
        assert_eq!(button_font.family.as_deref(), Some("Inter"));
        assert_eq!(button_font.size, Some(14.0));
        assert_eq!(button_font.weight, Some(400));
    }

    // ===== Phase 3: Text scale inheritance (RESOLVE-05) =====

    #[test]
    fn resolve_phase3_text_scale_inheritance() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        v.defaults.line_height = Some(1.4);
        // Leave text_scale.caption as None
        v.resolve();

        let caption = v.text_scale.caption.as_ref().unwrap();
        assert_eq!(caption.size, Some(14.0), "size from defaults.font.size");
        assert_eq!(caption.weight, Some(400), "weight from defaults.font.weight");
        // line_height = defaults.line_height * resolved_size = 1.4 * 14.0 = 19.6
        assert!((caption.line_height.unwrap() - 19.6).abs() < 0.001, "line_height computed");
    }

    // ===== Phase 3: Color inheritance =====

    #[test]
    fn resolve_phase3_color_inheritance() {
        let mut v = variant_with_defaults();
        v.resolve();

        // window
        assert_eq!(v.window.background, Some(Rgba::rgb(255, 255, 255)));
        assert_eq!(v.window.border, v.defaults.border);
        // button
        assert_eq!(v.button.border, v.defaults.border);
        // tooltip
        assert_eq!(v.tooltip.radius, v.defaults.radius);
    }

    // ===== Phase 4: Widget-to-widget =====

    #[test]
    fn resolve_phase4_inactive_title_bar_from_active() {
        let mut v = ThemeVariant::default();
        v.defaults.surface = Some(Rgba::rgb(240, 240, 240));
        v.defaults.foreground = Some(Rgba::rgb(30, 30, 30));
        v.resolve();

        // title_bar_background was set to surface in Phase 3
        // inactive should inherit from active
        assert_eq!(v.window.inactive_title_bar_background, v.window.title_bar_background);
        assert_eq!(v.window.inactive_title_bar_foreground, v.window.title_bar_foreground);
    }

    // ===== Preserve explicit values =====

    #[test]
    fn resolve_does_not_overwrite_existing_some_values() {
        let mut v = variant_with_defaults();
        let explicit = Rgba::rgb(255, 0, 0);
        v.window.background = Some(explicit);
        v.button.primary_bg = Some(explicit);
        v.resolve();

        assert_eq!(v.window.background, Some(explicit), "window.background preserved");
        assert_eq!(v.button.primary_bg, Some(explicit), "button.primary_bg preserved");
    }

    // ===== Idempotent =====

    #[test]
    fn resolve_is_idempotent() {
        let mut v = variant_with_defaults();
        v.resolve();
        let after_first = v.clone();
        v.resolve();
        assert_eq!(v, after_first, "second resolve() produces same result");
    }

    // ===== All 8 font-carrying widgets get resolved fonts =====

    #[test]
    fn resolve_all_font_carrying_widgets_get_resolved_fonts() {
        let mut v = ThemeVariant::default();
        v.defaults.font = FontSpec {
            family: Some("Inter".into()),
            size: Some(14.0),
            weight: Some(400),
        };
        v.resolve();

        // All 8 should now have Some(FontSpec)
        assert!(v.window.title_bar_font.is_some(), "window.title_bar_font");
        assert!(v.button.font.is_some(), "button.font");
        assert!(v.input.font.is_some(), "input.font");
        assert!(v.menu.font.is_some(), "menu.font");
        assert!(v.tooltip.font.is_some(), "tooltip.font");
        assert!(v.toolbar.font.is_some(), "toolbar.font");
        assert!(v.status_bar.font.is_some(), "status_bar.font");
        assert!(v.dialog.title_font.is_some(), "dialog.title_font");

        // Each should have the defaults values
        for (name, font) in [
            ("window.title_bar_font", &v.window.title_bar_font),
            ("button.font", &v.button.font),
            ("input.font", &v.input.font),
            ("menu.font", &v.menu.font),
            ("tooltip.font", &v.tooltip.font),
            ("toolbar.font", &v.toolbar.font),
            ("status_bar.font", &v.status_bar.font),
            ("dialog.title_font", &v.dialog.title_font),
        ] {
            let f = font.as_ref().unwrap();
            assert_eq!(f.family.as_deref(), Some("Inter"), "{name} family");
            assert_eq!(f.size, Some(14.0), "{name} size");
            assert_eq!(f.weight, Some(400), "{name} weight");
        }
    }
}
