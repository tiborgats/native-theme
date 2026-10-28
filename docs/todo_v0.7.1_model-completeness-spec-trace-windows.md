# v0.7.1 — Model completeness: Specification, traceability (Windows)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-windows.md`](todo_v0.7.1_model-completeness-gap-map-windows.md) mapped to
what carries it (R24), one line per (row, target), the rows in the order of the platform file. Withdrawn rows and
the pointer rows the map does not count are not listed; the cross-check and pass decision tables are decisions,
not rows.

**Targets** use the widget catalogue's names
([`todo_v0.7.1_model-completeness-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md), after its rename
table): `widget.part.field` (a `PartStyle` field, S2.11), `widget.values.name` (a widget value),
`component.<key>.<part>` (a keyed component), `role.<set>.<role>`, `role.global.<role>`, `role.@<key>` (S3),
`token.<path>` (S4), `input.<name>` with kind `input` (an S9 input, not the `input` widget), `reader:<platform>:<what
is read>` (S9.3), a placement strategy, a system control (S15), a cond or flag, or a core type (`PartStyle.…`,
`FocusIndicator.…`) where the row concerns every part. The selector of a rule follows `@`, in the catalogue's selector
notation (its *Selector notation* table): the element's own variants and states come before the first `context`, a
context's `variant`/`{…}` belong to that ancestor, and ` / ` separates alternatives. A name with no unit suffix and
the same name with `_px` or `_ms` are one field (S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the
additions to the core and the contradictions with the rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`, `reference (R1.6)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| W1-001 | `window.frame.stroke` | field |
| W1-001 | `window.frame.stroke @ cond scheme:dark / cond window_inactive / cond window_inactive cond scheme:dark` | rule |
| W1-001 | `window.frame.stroke @ cond system_setting:accent_on_title_bars !cond window_inactive` | rule |
| W1-001 | `reader:windows:HKCU DWM ColorPrevalence (system_setting)` | reader |
| W1-002 | `window.frame.stroke (width)` | field |
| W1-002 | `reader:windows:DWMWA_VISIBLE_FRAME_BORDER_THICKNESS` | reader |
| W1-003 | `window.frame.shadow (Shadow::Elevation, the window's entry of the WinUI ramp)` | field |
| W1-003 | `token.elevation (that entry's ShadowLayer x, y, blur, spread, color)` | token |
| W1-003 | `window.frame.shadow @ cond window_inactive` | rule |
| W1-004 | `window.frame.corners` | field |
| W1-004 | `window.frame.corners @ cond maximized / cond tiled:left / cond tiled:right / cond tiled:top / cond tiled:bottom` | rule |
| W1-004 | `window.frame.corners @ variant @round_small` | rule |
| W1-005 | `window.content.fill (PaintKind::Material @mica: tint, luminosity, saturation, blur_radius, noise_opacity, fallback)` | field |
| W1-005 | `window.content.fill @ variant @mica_alt` | rule |
| W1-005 | `window.content.fill @ cond window_inactive / cond window_inactive variant @mica_alt (Material.fallback: the base and base-alt solid fills)` | rule |
| W1-005 | `window.content.transitions (the material to colour crossfade)` | field |
| W1-006 | `window.title_bar.min_height` | field |
| W1-006 | `window.title_bar.min_height @ variant @tall / variant @collapsed / variant @titlebar_control_expanded` | rule |
| W1-006 | `reader:windows:AppWindowTitleBar.Height` | reader |
| W1-007 | `window.caption_button.{min_width,min_height}` | field |
| W1-007 | `window.caption_button.{min_width,min_height} @ context window.frame variant @tall / context window.frame variant @collapsed / context window.frame variant @titlebar_control_expanded` | rule |
| W1-007 | `window.caption_button_glyph.glyph (FontCodePoint and size) @ variant close / variant minimize / variant maximize / variant restore` | rule |
| W1-007 | `reader:windows:DWMWA_CAPTION_BUTTON_BOUNDS, AppWindowTitleBar.LeftInset/RightInset, SM_CXSIZE/SM_CYSIZE` | reader |
| W1-008 | `window.{caption_button.fill,caption_button_glyph.glyph.fill}` | field |
| W1-008 | `window.{caption_button.fill,caption_button_glyph.glyph.fill} @ {hover} / {pressed} / {hover} variant close / {pressed} variant close` | rule |
| W1-008 | `window.{caption_button.fill,caption_button_glyph.glyph.fill} @ cond window_inactive` | rule |
| W1-008 | `reader:windows:AppWindowTitleBar Button*Color getters (to verify)` | reader |
| W1-009 | `window.caption_button.transitions (Fill: the hover fade in and out)` | field |
| W1-010 | `window.title_icon.{icon,padding}` | field |
| W1-010 | `window.values.{title_icon_gap,title_margin}` | field |
| W1-010 | `window.values.title_alignment` | field |
| W1-010 | `reader:windows:SM_CXSMICON -> window.title_icon.icon (its size)` | reader |
| W1-011 | `window.subtitle.text` | field |
| W1-011 | `window.subtitle.text.fill @ cond window_inactive` | rule |
| W1-011 | `button.{body.min_width,body.fill,icon.glyph} @ variant back context window.title_bar / variant pane_toggle context window.title_bar` | rule |
| W1-011 | `button.{body.fill,body.stroke,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant back context window.title_bar` | rule |
| W1-011 | `button.{body.fill,body.stroke,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant pane_toggle context window.title_bar` | rule |
| W1-011 | `window.title_bar_content.padding; window.values.title_bar_box_gap (the TitleBar control's header, content and footer regions)` | field |
| W1-012 | `window.title_bar_content.opacity @ cond window_inactive` | rule |
| W1-013 | `window.{title_bar.fill,frame.stroke} @ cond system_setting:accent_on_title_bars !cond window_inactive` | rule |
| W1-013 | `reader:windows:HKCU DWM ColorPrevalence` | reader |
| W1-014 | - | out (R1.1) |
| W1-015 | `menu.* @ context window.window_menu (the system menu's look)` | rule |
| W1-015 | `window.values.window_menu_items` | field |
| W1-015 | `window.values.window_menu_placement (at the title icon, or the pointer for Alt+Space)` | field |
| W1-016 | `window.values.resize_band (outside, includes_shadow: the invisible band outside the frame)` | field |
| W1-016 | `window.resize_zone.cursor @ variant top / variant right / variant bottom / variant left / variant top_left / variant top_right / variant bottom_right / variant bottom_left` | rule |
| W1-016 | `reader:windows:SM_CXFRAME, SM_CXPADDEDBORDER, DWMWA_EXTENDED_FRAME_BOUNDS` | reader |
| W1-017 | `cond animations_off (read from SPI_GETCLIENTAREAANIMATION)` | condition |
| W1-017 | - (the DWM's open, close, minimize, maximize and restore animations) | out (R1.1) |
| W1-148† | `window.values.caption_button_capabilities (the maximize button greyed on a window that cannot be resized: WhenMissing::Disable)` | field |
| W1-148† | `window.{caption_button.fill,caption_button_glyph.glyph.fill} @ {disabled}` | rule |
| W1-148† | `window.values.caption_button_layout @ variant dialog (the close-only caption)` | rule |
| W1-148† | - (the context-help "?" caption button, WS_EX_CONTEXTHELP: Win32 only) | preset (R1.3) |
| W1-148† | `window.title_bar.min_height @ variant utility` | rule |
| W1-148† | `window.caption_button.{min_width,min_height} @ context window.frame variant utility` | rule |
| W1-148† | `window.values.caption_button_layout @ dir rtl` | rule |
| W1-148† | `reader:windows:SM_CYSMCAPTION, SM_CXSMSIZE, SM_CYSMSIZE` | reader |
| W1-149† | `cond attention_flash (ADD-A03)` | condition |
| W1-149† | `window.{title_bar.fill,frame.stroke} @ cond attention_flash` | rule |
| W1-149† | `window.values.attention_flash (period_ms, count)` | field |
| W1-149† | `reader:windows:SPI_GETFOREGROUNDFLASHCOUNT, GetCaretBlinkTime` | reader |
| W1-149† | - (the taskbar button's flash: the shell's) | out (R1.1) |
| W1-018 | `button.body.stroke (paint: Linear gradient over an absolute extent)` | field |
| W1-018 | `button.body.stroke @ cond scheme:dark (gradient at the other edge)` | rule |
| W1-019 | `button.body.stroke @ {hover} / {pressed} / {disabled}` | rule |
| W1-020 | `button.{body.fill,label.text.fill} @ {hover} / {pressed} / {disabled} variant primary` | rule |
| W1-020 | `button.body.stroke (gradient paint) @ variant primary / {hover} variant primary / {pressed} variant primary / {disabled} variant primary` | rule |
| W1-020 | `button.{body.fill,label.text.fill,body.stroke} @ {default} / {default,hover} / {default,pressed} / {default,disabled} context dialog.action_area` (the dialog's default button in AccentButtonStyle, in every state) | rule |
| W1-021 | `button.body.fill, button.label.text.fill, button.body.stroke @ variant flat / {hover} variant flat / {pressed} variant flat / {disabled} variant flat` | rule |
| W1-022 | `button.{body.fill,label.text.fill,body.stroke} @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} variant toggle` | rule |
| W1-022 | `button.{body.fill,label.text.fill,body.stroke} @ {indeterminate} / {indeterminate,hover} / {indeterminate,pressed} / {indeterminate,disabled} variant toggle` | rule |
| W1-023 | `button.body.transitions (Fill)` | field |
| W1-023 | `token.motion.durations.control_faster` | token |
| W1-024 | `button.icon.transitions` (its AnimatedIcon states: a `TransitionKind::Keyed` transition of group `Icon` per state, ADD-A27) | field |
| W1-025 | `button.body.focus.offset` | field |
| W1-025 | `FocusIndicator.offset (per side, ADD-A18; stated on each widget's focus field, the per-control FocusVisualMargin)` | field |
| W1-026 | `input.body.stroke (gradient paint per edge)` | field |
| W1-026 | `input.body.stroke (bottom edge accent) @ {focused}` | rule |
| W1-027 | `input.body.stroke (bottom edge width) @ {focused}` | rule |
| W1-028 | `input.body.fill @ {hover} / {focused}` | rule |
| W1-028 | `input.text.text.fill @ {hover} / {focused}` | rule |
| W1-028 | `input.placeholder.text.fill @ {hover} / {focused} / {disabled}` | rule |
| W1-029 | `input.body.stroke @ {disabled}` | rule |
| W1-030 | `button.{icon.glyph,body.fill,body.stroke} @ variant clear_field context input.body (the glyph and its size)` | rule |
| W1-030 | `button.{body.fill,icon.glyph.fill} @ {hover} variant clear_field context input.body / {pressed} variant clear_field context input.body` | rule |
| W1-030 | `input.values.trailing_inset (the clear button's margin in the field)` | field |
| W1-030 | `input.values.clear_button_visibility (WhenFocusedAndNotEmpty)` | field |
| W1-030 | `button.values.presentations @ variant clear_field context input.body (events Appear, Disappear: PresentationKind::Instant)` | rule |
| W1-031 | `input.header.text; input.values.header_gap` | field |
| W1-031 | `input.header.text.fill @ {disabled}` | rule |
| W1-031 | `input.supporting_text.text` | field |
| W1-032 | `input.selection.fill @ context input.body !{focused}` | rule |
| W1-032 | `input.selection.text.fill` | field |
| W1-032 | `role.global.selection_text_background_unfocused` | role |
| W1-033 | `input.{body,text} @ {read_only} / {invalid}` | rule |
| W1-034 | `reader:windows:SPI_GETCARETWIDTH, GetCaretBlinkTime, UISettings.CaretWidth, UISettings.CaretBlinkRate -> token.caret.{width,blink_on,blink_off}` | reader |
| W1-034 | `input.caret.min_width (= token.caret.width)` | field |
| W1-034 | `input.selection_handle.{glyph,fill,min_width,min_height}` | field |
| W1-034 | `input.selection_handle (absent) @ !cond input:touch` | rule |
| W1-036 | `text_area.values.scrollbar_inset; text_area.body.padding` | field |
| W1-036 | `scrollbar.* @ context any text_area` | rule |
| W1-036 | `text_area.{body.fill,body.stroke,text.text.fill,placeholder.text.fill,selection.fill} @ {hover} / {focused} / {disabled} (the text box's rows W1-026 to W1-034 restated on text_area, R20)` | rule |
| W1-036 | `text_area.{header.text,supporting_text.text}; text_area.values.{header_gap_px,supporting_text_gap_px}` (W1-031's header and description, which the multi-line box also draws) | field |
| W1-150† | `input.supporting_text.text; input.values.supporting_text_gap` | field |
| W1-150† | `combo_box.supporting_text.text; combo_box.values.supporting_text_gap` | field |
| W1-150† | `input.supporting_text.text @ context stepper / context any search_field (NumberBox's and AutoSuggestBox's description; PasswordBox is input variant secure)` | rule |
| W1-151† | `input.misspelling.text.decoration` | field |
| W1-151† | `input.preedit.{fill,text.fill,text.decoration} @ variant selected_clause (the target clause; the IME's other clause attributes, TF_DISPLAYATTRIBUTE, as keyed preedit variants)` | rule |
| W1-037 | `checkbox.box.fill, checkbox.box.stroke, checkbox.mark.glyph.fill, checkbox.label.text.fill @ {hover} / {pressed} / {disabled} / {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} / {indeterminate} / {indeterminate,hover} / {indeterminate,pressed} / {indeterminate,disabled}` | rule |
| W1-038 | `checkbox.mark.glyph (FontCodePoint and size) @ {indeterminate}` | rule |
| W1-038 | `checkbox.box.{fill,stroke} @ {indeterminate}` | rule |
| W1-039 | `checkbox.mark.glyph (FontCodePoint and size)` | field |
| W1-039 | `checkbox.mark.transitions (TransitionKind::Keyed per AnimatedIcon state, the accept animation; ADD-A27)` | field |
| W1-040 | `checkbox.body.min_height, checkbox.body.min_width` | field |
| W1-041 | `radio.dot.{min_width,min_height} @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled}` | rule |
| W1-041 | `radio.dot.transitions (Geometry: the scale animation)` | field |
| W1-042 | `radio.ring.stroke, radio.ring.fill @ {hover} / {pressed} / {disabled} / {checked} / {checked,hover} / {checked,pressed} / {checked,disabled}` | rule |
| W1-042 | `radio.dot.fill, radio.dot.stroke @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled}` | rule |
| W1-043 | `radio.{body,ring,dot,label} (the typed radio, stated apart from checkbox)` | field |
| W1-044 | `menu.item.fill @ {pressed}` | rule |
| W1-044 | `menu.item_label.text.fill @ {pressed}` | rule |
| W1-044 | `menu.item.fill @ {disabled}` | rule |
| W1-045 | `menu.item.corners` | field |
| W1-045 | `menu.item.margin` (`MenuFlyoutItemMargin`: the space round each item, at the popup's edges and between items) | field |
| W1-046 | `menu.body.{padding,min_height,max_height,stroke}` | field |
| W1-046 | `menu.values.scrollable` | field |
| W1-047 | `menu.item.padding` | field |
| W1-047 | `menu.item.padding @ context menu.body {opened_by_touch}` | rule |
| W1-048 | `menu.accelerator.text` | field |
| W1-048 | `menu.accelerator.text.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-048 | `menu.values.{accelerator_alignment,accelerator_gap}` | field |
| W1-049 | `menu.submenu_arrow.glyph` | field |
| W1-049 | `menu.submenu_arrow.glyph.fill @ {hover} / {pressed} / {open} / {disabled}` | rule |
| W1-049 | `menu.{item.fill,item_label.text.fill} @ {open}` | rule |
| W1-049 | `menu.submenu_arrow.min_width (its box: the chevron margin)` | field |
| W1-050 | `menu.check_indicator.glyph (glyph and size) @ {checked} variant check / {checked} variant radio` | rule |
| W1-050 | `menu.check_indicator.glyph.fill @ {checked,hover} / {checked,pressed} / {checked,disabled}` | rule |
| W1-050 | `menu.values.{check_column,icon_column,check_column_width} (the single and double placeholder)` | field |
| W1-051 | `menu.split_chevron.{fill,glyph,min_width}; menu.split_divider.{stroke,min_height}` | field |
| W1-051 | `menu.split_chevron.fill @ {hover} / {pressed}` | rule |
| W1-051 | `menu.item.* @ variant @split` | rule |
| W1-052 | `menu.separator.{min_height,padding,stroke}` | field |
| W1-053 | `menu.body.fill (PaintKind::Material @acrylic, its fallback)` | field |
| W1-053 | `menu.body.shadow (Shadow::Elevation, the flyout entry of the ramp)` | field |
| W1-054 | `menu.values.presentations (events Appear, Disappear: Slide from TowardAnchor with Fade, ADD-B10, ADD-A25)` | field |
| W1-054 | `menu.values.presentations @ !cond system_setting:menu_animation (Instant)` | rule |
| W1-054 | `menu.values.{submenu_open_delay_ms,submenu_close_delay_ms}` | field |
| W1-054 | `reader:windows:SPI_GETMENUSHOWDELAY, SPI_GETMENUANIMATION` | reader |
| W1-055 | `menu.dismiss_overlay.fill` | field |
| W1-056 | `tooltip.body.fill (PaintKind::Material @acrylic_in_app, its fallback)` | field |
| W1-057 | `tooltip.body.shadow (Shadow::Elevation, the tooltip entry of the ramp)` | field |
| W1-058 | `tooltip.values.presentations (events Appear, Disappear: Fade)` | field |
| W1-058 | `tooltip.values.presentations @ !cond system_setting:tooltip_animation (Instant)` | rule |
| W1-058 | `reader:windows:SPI_GETTOOLTIPANIMATION (system_setting tooltip_animation)` | reader |
| W1-059 | `tooltip.values.{reshow_delay_ms,keyboard_show_delay_ms,display_time}` | field |
| W1-059 | `tooltip.values.show_delay_ms @ cond input:touch` | rule |
| W1-059 | `WinUiToolTip (tooltip.values.placement; its gap: the offset from the pointer or the target)` | placement |
| W1-059 | `reader:windows:SPI_GETMOUSEHOVERTIME -> tooltip.values.{show_delay_ms,reshow_delay_ms}` | reader |
| W1-060 | `scrollbar.{body.min_width,thumb.min_width,thumb.fill} @ cond overlay_scrollbars !{hover} (the panning indicator: a thin thumb)` | rule |
| W1-060 | `scrollbar.track.fill (Absent) @ cond overlay_scrollbars !{hover} (no track)` | rule |
| W1-060 | `scrollbar.body.{min_width,min_height}; scrollbar.thumb.{stroke,min_width} (the expanded bar)` | field |
| W1-061 | `scrollbar.arrow_button.{glyph,fill,stroke,padding}` | field |
| W1-061 | `scrollbar.arrow_button.{fill,stroke,glyph.fill} @ {hover} / {pressed} / {disabled}` | rule |
| W1-061 | `scrollbar.arrow_button.transform @ {pressed}` | rule |
| W1-061 | `scrollbar.arrow_button.transitions (Transform: the press scale's animation)` | field |
| W1-062 | `scrollbar.track.{fill,stroke} (fill a material)` | field |
| W1-062 | `scrollbar.track.{fill,stroke} @ {hover} / {disabled}` | rule |
| W1-063 | `scrollbar.thumb.corners` | field |
| W1-063 | `scrollbar.thumb.fill @ {disabled}` | rule |
| W1-064 | `scrollbar.{body,track,thumb}.transitions (Geometry, Opacity, Fill: the contract's duration and delay)` | field |
| W1-064 | `scrollbar.{body,track,thumb}.transitions @ {hover} (the expand's duration and delay)` | rule |
| W1-064 | `scrollbar.values.auto_hide_delay` | field |
| W1-065 | `scroll_view.corner.{fill,transitions}` | field |
| W1-065 | `scroll_view.values.scrollbar_margin` | field |
| W1-066 | `reader:windows:UISettings.ScrollBarSize, ScrollBarArrowSize, ScrollBarThumbBoxSize -> scrollbar.body.min_width, scrollbar.arrow_button.min_height, scrollbar.thumb.min_width` | reader |
| W1-152† | `reader:windows:UISettings.AutoHideScrollBars -> cond overlay_scrollbars` | reader |
| W1-152† | `cond overlay_scrollbars` | condition |
| W1-067 | `slider.thumb_inner.{fill,min_width,min_height}` | field |
| W1-067 | `slider.thumb_inner.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-068 | `slider.thumb_inner.transform @ {hover} / {pressed}` | rule |
| W1-068 | `slider.thumb_inner.transitions` | field |
| W1-069 | `slider.thumb.{stroke,corners}; slider.track.corners` | field |
| W1-070 | `slider.{fill,track}.fill @ {hover} / {pressed}` | rule |
| W1-071 | `slider.tick.fill` | field |
| W1-071 | `slider.tick.fill @ {disabled} / variant @ticks_inline` | rule |
| W1-071 | `slider.tick.* @ variant ticks_before / variant ticks_after / variant @ticks_inline (the placement)` | rule |
| W1-071 | `slider.values.tick_gap` | field |
| W1-072 | `slider.header.text; slider.values.header_gap; slider.body.{min_height,padding}` | field |
| W1-072 | `slider.header.text.fill @ {disabled}` | rule |
| W1-073 | `tooltip.{body,label} @ context slider.body (the thumb tooltip: the tooltip's style)` | rule |
| W1-073 | `slider.values.thumb_tooltip_placement` (the thumb tooltip's placement: the host's value, Nested controls) | placement |
| W1-074 | `slider.body.fill` | field |
| W1-074 | `slider.body.fill @ {hover}` | rule |
| W1-153† | `slider.{thumb.fill,thumb.corners,thumb.min_width,thumb.min_height,fill.fill,track.fill} @ variant range` | rule |
| W1-153† | `slider.fill.fill @ {hover} variant range / {pressed} variant range / {disabled} variant range` | rule |
| W1-153† | `slider.values.{range_min_gap_px,range_thumb_order} @ variant range` (the minimum gap and which thumb is drawn on top where they overlap) | rule |
| W1-075 | `progress_bar.values.loop (keyframes: the two indicators' widths and positions; period, timing)` | field |
| W1-075 | `progress_bar.{indicator,secondary_indicator}.{fill,corners} @ {indeterminate}` | rule |
| W1-076 | `progress_bar.{indicator,secondary_indicator}.fill @ {@paused} / {@error} / {@paused,indeterminate} / {@error,indeterminate}` | rule |
| W1-076 | `progress_bar flags @paused, @error` | flag |
| W1-077 | `progress_bar.track.{min_height,corners}; progress_bar.indicator.min_height` | field |
| W1-078 | `progress_bar.indicator.transitions (Geometry: the value change)` | field |
| W1-079 | `progress_bar.track.stroke @ cond contrast_high` | rule |
| W1-080 | `tab.tab.fill, tab.label.text.fill @ {pressed} / {disabled} / {selected}` | rule |
| W1-081 | `tab.icon.icon (size)` | field |
| W1-081 | `tab.values.icon_text_gap (the icon margin)` | field |
| W1-081 | `tab.icon.icon.fill @ {hover} / {pressed} / {selected} / {disabled}` | rule |
| W1-082 | `button.{body.min_width,body.min_height,icon.glyph} @ variant close context tab.tab (the button size, the glyph and its size)` | rule |
| W1-082 | `button.{body.fill,icon.glyph.fill,body.stroke} @ {hover} / {pressed} / {disabled} variant close context tab.tab` | rule |
| W1-082 | `button.{body.fill,icon.glyph.fill,body.stroke} @ variant close context tab.tab {selected} / variant close context tab.tab {hover} / variant close context tab.tab {pressed}` | rule |
| W1-082 | `tab.values.close_button_shown` | field |
| W1-082 | `tab.tab.padding @ variant closable` (CloseIconStates: the padding with and without the button) | rule |
| W1-082 | `button.body.margin @ variant close context tab.tab` (`TabViewItemHeaderCloseMargin`: the gap to the label) | rule |
| W1-083 | `button.{body.min_width,body.min_height,icon.glyph,body.fill} @ variant add context tab.bar` | rule |
| W1-083 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant add context tab.bar` | rule |
| W1-083 | `tab.scroll_arrow.{min_width,glyph,fill,padding}` | field |
| W1-083 | `tab.scroll_arrow.{fill,glyph.fill} @ {hover} / {pressed} / {disabled}` | rule |
| W1-084 | `tab.separator.stroke; tab.values.separator_inset` | field |
| W1-084 | `tab.separator.opacity @ prev tab.tab {selected} / next tab.tab {selected} (hidden beside the selected tab, ADD-B07)` | rule |
| W1-085 | `tab.tab.max_width` | field |
| W1-085 | `tab.values.equal_widths @ variant @width_equal` (true: every tab the same width up to `max_width`) `/ variant @width_size_to_content` (false: each tab sized by its content); `tab.label @ !{selected} variant @width_compact` (`absent = true`: Compact); TabWidthMode is the application's choice, the keyed variants `@width_equal`, `@width_size_to_content`, `@width_compact` | rule |
| W1-085 | `tab.label.text.font.size` | field |
| W1-085 | `tab.bar.padding` | field |
| W1-086 | `tab.tab.stroke @ {selected} (linear gradient paint, sides and top)` | rule |
| W1-087 | `tab.tab.fill @ {dragged}` | rule |
| W1-087 | `tab.values.presentations (events Move, Appear, Disappear, @drag)` | field |
| W1-088 | `tab.bar.fill (PaintKind::Material @mica_alt)` | field |
| W1-088 | `tab.pane.fill (a layer over Mica Alt: = role.@layer_on_mica_base_alt_*)` | field |
| W1-088 | `tab.tab.fill @ {selected} (joined to the pane's fill)` | rule |
| W1-089 | `list.row_selection_indicator.{fill,corners,min_width,min_height} @ variant sidebar` | rule |
| W1-089 | `list.row_selection_indicator.fill @ {hover} / {pressed} / {disabled} variant sidebar` | rule |
| W1-089 | `list.values.selection_indicator_edge @ variant sidebar` (the pill at the row's leading edge) | rule |
| W1-090 | `list.values.presentations (event @indicator_move, part row_selection_indicator, PresentationKind::Morph) @ variant sidebar` | rule |
| W1-091 | `list.{row_background.fill,row_background.stroke,row.text.fill} @ {pressed} / {selected} / {selected,hover} / {selected,pressed} / {disabled} / {checked} variant sidebar` | rule |
| W1-092 | `list.{row.min_height,row.padding,row_background.corners,leading_icon.min_width,leading_icon.min_height} @ variant sidebar (row height, icon box, its margin)` | rule |
| W1-092 | `list.values.{row_background_inset,leading_gap} @ variant sidebar (the item margin, the icon to text gap)` | rule |
| W1-093 | `list.section_header.text @ variant sidebar` | rule |
| W1-093 | `separator.line.{stroke,margin} @ context sidebar.pane (the colour, height and margin)` | rule |
| W1-093 | `list.disclosure.glyph @ variant sidebar` | rule |
| W1-093 | `list.values.disclosure_gap @ variant sidebar (the chevron margin)` | rule |
| W1-093 | `list.disclosure.transform @ {open} variant sidebar` | rule |
| W1-093 | `list.disclosure.transitions (Transform: the rotation)` | field |
| W1-094 | `sidebar.values.width, sidebar.values.collapsed_width` | field |
| W1-094 | `sidebar.* @ variant compact / variant @left_minimal (LeftCompact, LeftMinimal; Left is the base)` | rule |
| W1-094 | `sidebar.values.breakpoints` (the display modes by width: W-P4-19†'s) | field |
| W1-094 | `view_switcher.* @ variant @navigation_top (Top)` | rule |
| W1-095 | `button.{body.min_width,body.min_height,icon.glyph} @ variant pane_toggle context sidebar.header / variant back context sidebar.header` | rule |
| W1-095 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant pane_toggle context sidebar.header` | rule |
| W1-095 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant back context sidebar.header` | rule |
| W1-095 | `sidebar.header.min_height (the AutoSuggestBox area)` | field |
| W1-095 | `search_field.* @ context sidebar.header (the AutoSuggestBox there)` | rule |
| W1-095 | `list.row.* @ variant sidebar context sidebar.footer` (the settings item and the footer items, in the pane's footer at its bottom); `sidebar.footer.{padding,stroke}` | rule |
| W1-096 | `sidebar.pane.{fill,shadow,corners} @ variant overlay (fill PaintKind::Material @acrylic_in_app; shadow Shadow::Elevation)` | rule |
| W1-097 | `sidebar.content.{fill,stroke,corners}` | field |
| W1-098 | `sidebar.values.presentations (events Appear, Disappear)` | field |
| W1-099 | `button.body.{min_width,min_height,padding,corners} @ variant tool context toolbar.bar / variants {tool,icon_only} context toolbar.bar (labelled and compact)` | rule |
| W1-099 | `button.{body.fill,label.text.fill,icon.icon.fill} @ {hover} / {pressed} / {disabled} variant tool context toolbar.bar` | rule |
| W1-099 | `button.values.label_position @ variant tool context toolbar.bar (TextUnderIcon, TextBesideIcon, IconOnly)` | rule |
| W1-099 | `button.{label.text,values.icon_text_gap} @ variant tool context toolbar.bar (the label font, the icon and label margins)` | rule |
| W1-100 | `button.{body.fill,label.text.fill} @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} variants {tool,toggle} context toolbar.bar` | rule |
| W1-100 | `menu.check_indicator.glyph @ {checked} context button.body variant overflow` | rule |
| W1-101 | `toolbar.separator.{stroke,corners,margin} (width and colour as the stroke's, the margin on both sides of the line, the radius)` | field |
| W1-102 | `button.{icon.glyph,body.fill,body.min_width,body.min_height} @ variant overflow context toolbar.bar` | rule |
| W1-102 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} variant overflow context toolbar.bar` | rule |
| W1-102 | `menu.body.{fill,stroke,padding,min_width,max_width,max_height,shadow} @ context button.body variant overflow` | rule |
| W1-102 | `WinUiFlyout (button.values.menu_placement @ variant overflow context toolbar.bar)` | placement |
| W1-102 | `menu.{accelerator.text.fill,submenu_arrow.glyph.fill} @ context button.body variant overflow` | rule |
| W1-103 | `toolbar.bar.{fill,stroke,corners} @ {open}` | rule |
| W1-103 | `toolbar.bar.min_height @ variant @compact / variant @minimal / variant @hidden` | rule |
| W1-104 | `toolbar.values.presentations (events @expand, @collapse)` | field |
| W1-105 | - | preset (R1.3) |
| W1-106 | `list.row_background.fill, list.row.text.fill @ {pressed} / {selected,hover} / {selected,pressed} / {selected,disabled}` | rule |
| W1-107 | `list.row_selection_indicator.{fill,corners,min_width,min_height}` | field |
| W1-107 | `list.row_selection_indicator.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-107 | `list.row_selection_indicator.transitions` | field |
| W1-108 | `list.row_background.corners, list.values.row_background_inset` | field |
| W1-109 | `checkbox.{box.fill,box.stroke,box.corners,mark.glyph} @ context list.row` | rule |
| W1-109 | `checkbox.{box.fill,box.stroke,mark.glyph.fill} @ context list.row {selected} / context list.row {hover} / context list.row {pressed}` | rule |
| W1-109 | `list.values.check_box_placement (ListViewItemCheckMode: inline or overlay)` | field |
| W1-110 | `list.{row.opacity,row.transform} @ {dragged} / {@reorder_target}` | rule |
| W1-110 | `list.{row_background.fill,row.text.fill} @ {dragged}` | rule |
| W1-110 | `list.values.reorder_hint_offset` | field |
| W1-110 | `drag_image.{image.opacity,values.hotspot} @ context list` | rule |
| W1-110 | `badge.{body.fill,label.text} @ context drag_image.image (the multi-drag count)` | rule |
| W1-111 | `list.row_background.focus (FocusIndicator.paint: the outer ring; FocusIndicator.shadow: the inner ring, a shadow layer (R7, ADD-A17))` | field |
| W1-112 | `list.row.opacity @ {disabled}` | rule |
| W1-113 | `list.values.presentations (events Appear, Disappear, Move, queried with the row: added, deleted, reordered)` | field |
| W1-114 | `popover.body.fill (PaintKind::Material @acrylic_in_app: tint, luminosity, blur_radius, noise_opacity, fallback)` | field |
| W1-115 | `popover.body.shadow (Shadow::Elevation, the flyout entry of the ramp)` | field |
| W1-116 | `popover.values.presentations (events Appear, Disappear: Slide from TowardAnchor with Fade, ADD-B10, ADD-A25)` | field |
| W1-116 | `WinUiFlyout (popover.values.placement; its gap)` | placement |
| W1-116 | `popover.dismiss_overlay.fill; menu.dismiss_overlay.fill @ context combo_box` (the light-dismiss overlay, its colour per control: the combo box's own brush on its popup's) | rule |
| W1-117 | `popover.body.{min_width,max_width,min_height,max_height}` | field |
| W1-119 | `menu.separator.padding; toolbar.separator.margin (the menu's and the command bar's own separators)` | field |
| W1-119 | `separator.line.margin @ context sidebar.pane` (the navigation pane's separator insets) | rule |
| W1-157† | `splitter.divider.fill` | field |
| W1-157† | `splitter.divider.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-157† | `splitter.grip.{fill,glyph,min_width,min_height,corners,padding}` | field |
| W1-158† | - | preset (R1.3) |
| W1-159† | - | preset (R1.3) |
| W1-120 | `switch.track.fill @ {checked,pressed} / {pressed} !{checked}` | rule |
| W1-120 | `switch.thumb.{fill,min_width} @ {pressed} / {checked,pressed} (the fill and the stretch)` | rule |
| W1-121 | `switch.thumb.fill @ {hover} / {checked,hover}` | rule |
| W1-122 | `switch.thumb.{min_width,min_height} @ {hover} / {checked,hover}` | rule |
| W1-123 | `switch.thumb.transitions (Geometry)` | field |
| W1-123 | `switch.thumb.transitions (Geometry: the slide; measured)` | field |
| W1-123 | `token.motion.durations.control_faster, token.motion.timings.control_fast_out_slow_in` | token |
| W1-124 | `switch.state_label.text` | field |
| W1-124 | `switch.state_label.text.fill @ {disabled}` | rule |
| W1-124 | `switch.header.text` | field |
| W1-124 | `switch.header.text.fill @ {disabled}` | rule |
| W1-124 | `switch.body.min_width; switch.values.{pre_content_margin,post_content_margin}` | field |
| W1-124 | `switch.track.stroke @ {hover} / {disabled}` | rule |
| W1-125 | `dialog.scrim.fill (= role.global.scrim)` | field |
| W1-126 | `dialog.body.fill (content zone)` | field |
| W1-126 | `dialog.action_area.{fill,stroke}` | field |
| W1-127 | `dialog.values.title_message_gap` | field |
| W1-127 | `button.body.{min_width,max_width,min_height} @ context dialog.action_area` | rule |
| W1-127 | `dialog.values.action_layout.{equal_widths,row_gap}` | field |
| W1-128 | `dialog.values.presentations (events Appear, Disappear: Scale with Fade, their durations and timings, ADD-B10)` | field |
| W1-129 | `dialog.body.shadow (Shadow::Elevation, the dialog entry of the ramp)` | field |
| W1-130 | `spinner.{indicator,track}.* @ variant determinate` | rule |
| W1-130 | `spinner.track.fill` | field |
| W1-130 | `spinner.values.loop` | field |
| W1-131 | `combo_box.body.fill @ {pressed} / {focused}` | rule |
| W1-131 | `combo_box.{label,placeholder}.text.fill @ {pressed} / {focused}` | rule |
| W1-131 | `combo_box.body.stroke (gradient paint) @ {hover} / {pressed}` | rule |
| W1-132 | `combo_box.arrow.glyph.fill` | field |
| W1-132 | `combo_box.arrow.glyph.fill @ {disabled} / {focused} / {focused,pressed}` | rule |
| W1-132 | `combo_box.arrow.transitions (TransitionKind::Keyed per AnimatedIcon state, AnimatedChevronDownSmall; ADD-A27)` | field |
| W1-133 | `menu.body.{fill,stroke,padding,corners,shadow,min_width} @ context combo_box (fill PaintKind::Material @acrylic)` | rule |
| W1-133 | `combo_box.values.popup_placement (its max_visible_items)` | placement |
| W1-134 | `menu.{item.fill,item.stroke,item_label.text.fill} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {selected,disabled} / {disabled} context combo_box` | rule |
| W1-134 | `menu.item.fill @ {selected} !{focused} context combo_box (selected, unfocused)` | rule |
| W1-134 | `menu.item.padding @ cond input:touch context combo_box / cond input:game_controller context combo_box` | rule |
| W1-134 | `menu.item.corners @ context combo_box` | rule |
| W1-135 | `menu.item_selection_indicator.{fill,min_width,min_height,corners} @ {selected} context combo_box` | rule |
| W1-135 | `menu.item_selection_indicator.{transform,transitions} @ {selected} context combo_box (the scale it grows from, and its animation)` | rule |
| W1-136 | `menu.values.presentations @ context combo_box (events Appear, Disappear: ClipReveal from the selected item)` | rule |
| W1-136 | `WinUiComboBox (combo_box.values.popup_placement: over_selected_item; ADD-A21, ADD-A22)` | placement |
| W1-137 | `combo_box.* @ variant editable` | rule |
| W1-137 | `input.{body.padding,text.text} @ context any combo_box (the editable text box)` | rule |
| W1-137 | `combo_box.header.text; combo_box.values.header_gap` | field |
| W1-138 | `segmented_control.segment.fill @ {pressed} / {selected,hover}` | rule |
| W1-138 | `segmented_control.indicator.stroke, segmented_control.indicator.shadow` | field |
| W1-138 | `segmented_control.values.presentations (event @indicator_move, part indicator)` | field |
| W1-139 | `card.body.fill @ variant @secondary / variant @tertiary` | rule |
| W1-139 | `form.card.fill @ {hover} / {pressed} (the clickable SettingsCard)` | rule |
| W1-139 | `card.body.shadow (Shadow::Elevation, the card entry of the ramp, measured)` | field |
| W1-140 | `expander.{header.text.fill,header.stroke} @ {hover} / {pressed} / {disabled}` | rule |
| W1-140 | `expander.arrow.{fill,stroke,glyph}` (the chevron button's fill, border and glyph) | field |
| W1-140 | `expander.{arrow.fill,arrow.stroke,arrow.glyph.fill} @ {hover} / {pressed}` | rule |
| W1-141 | `expander.content.{fill,stroke,corners}` | field |
| W1-141 | `expander.content.{stroke,corners} @ variant @expand_up (the content above the header)` | rule |
| W1-142 | `expander.values.presentations (events @expand, @collapse: the content's eased slide)` | field |
| W1-142 | `expander.arrow.transitions (TransitionKind::Keyed per AnimatedIcon state, the chevron's rotation; ADD-A27)` | field |
| W1-143 | `link.text.text.decoration @ variant inline / {hover} variant inline / {pressed} variant inline` | rule |
| W1-144 | `link.body.fill @ {pressed} / {disabled}` | rule |
| W1-144 | `link.body.stroke @ {hover} / {pressed} / {disabled}` | rule |
| W1-145 | `link.text.focus @ variant inline` | rule |
| W1-146 | `layout.values.{button_gap,label_gap,header_gap,section_gap,surface_text_margin}` | field |
| W1-146 | `popover.values.placement (its gap: button to flyout)` | placement |
| W1-147 | `token.text.keyed.caption` | token |
| W1-147 | `token.text.keyed.body` | token |
| W1-147 | `token.text.keyed.body_strong` | token |
| W1-147 | `token.text.keyed.body_large` | token |
| W1-147 | `token.text.keyed.body_large_strong` | token |
| W1-147 | `token.text.keyed.subtitle` | token |
| W1-147 | `token.text.keyed.title` | token |
| W1-147 | `token.text.keyed.title_large` | token |
| W1-147 | `token.text.keyed.display` | token |
| W1-147 | `token.text.roles.body_emphasized` | token |
| W1-147 | `token.text.roles.callout` | token |
| W1-147 | `token.text.roles.large_title` | token |
| W1-147 | `token.text.roles.display_large` | token |
| W1-147 | `token.text.roles.title1` | token |
| W1-147 | `token.text.roles.title2` | token |
| W1-154† | `segmented_control.{segment.fill,segment.stroke,label.text.fill,indicator.fill,indicator.stroke} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {disabled} variant @pivot` | rule |
| W1-154† | `segmented_control.{segment.fill,segment.stroke,label.text.fill,indicator.fill,indicator.stroke} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {disabled} variant @button` | rule |
| W1-154† | `segmented_control.segment.* @ variant icon_only / variant with_icon (the item layouts: icon left, icon only; content only the base)` | rule |
| W1-155† | `dialog.body.{max_height,min_height} @ variant full_size` | rule |
| W1-156† | - | preset (R1.3) |
| W2-01 | `stepper.{increment,decrement}.{fill,stroke}; stepper.{increment_glyph,decrement_glyph}.glyph` | field |
| W2-01 | `stepper.{increment.fill,increment.stroke,decrement.fill,decrement.stroke,increment_glyph.glyph.fill,decrement_glyph.glyph.fill} @ {hover} / {pressed} / {disabled} variant @inline` | rule |
| W2-01 | `stepper.{increment.fill,increment.stroke,decrement.fill,decrement.stroke,increment_glyph.glyph.fill,decrement_glyph.glyph.fill} @ {hover} / {pressed} / {disabled} variant @compact` | rule |
| W2-01 | `stepper.popup.{fill,stroke,shadow} @ variant @compact` | rule |
| W2-01 | `stepper.values.{popup_offset_x,popup_offset_y,invalid_input}; stepper.header.text` | field |
| W2-01 | `input.* @ context stepper (the text box)` | rule |
| W2-02 | `calendar.{day.fill,day.stroke,day_label.text,month.fill,month.stroke,month.text,year.fill,year.stroke,year.text} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {selected,disabled} / {today} / {today,selected} / {@blackout} / {other_month} / {disabled}` | rule |
| W2-02 | `calendar.day_label.text.decoration @ {@blackout} (the strikethrough)` | rule |
| W2-02 | `calendar.weekday_label.text` | field |
| W2-02 | `button.{body.fill,label.text.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} context calendar.header (the previous, next and month-year header buttons)` | rule |
| W2-02 | `button.* @ variant previous context calendar.header / variant next context calendar.header` | rule |
| W2-02 | `calendar.{day_label.text,month.text,year.text} (font sizes); calendar.day.{padding,corners}; calendar.{month,year}.corners (the circular shape)` | field |
| W2-02 | `calendar.values.presentations (event @view_switch: the zoom)` | field |
| W2-03 | `date_picker.glyph.glyph` | field |
| W2-03 | `date_picker.glyph.glyph.fill @ {hover} / {pressed} / {disabled}` | rule |
| W2-03 | `date_picker.placeholder.text.fill @ {empty}` | rule |
| W2-03 | `date_picker.field_label.text.fill @ !{empty}` | rule |
| W2-03 | `date_picker.field.fill @ {focused}` | rule |
| W2-03 | `date_picker.header.text` | field |
| W2-03 | `date_picker.* @ variant with_popup` | rule |
| W2-03 | `date_picker.popup.shadow; date_picker.values.placement` | field |
| W2-03 | `calendar.* @ context date_picker.popup (the CalendarView in the flyout)` | rule |
| W2-04 | `time_picker.{field.fill,field.stroke,field.text,field_segment.text} @ {hover} / {pressed} / {focused} / {disabled}` | rule |
| W2-04 | `time_picker.{segment_separator.stroke,header.text,field.min_width,field.max_width}` | field |
| W2-04 | `time_picker.popup.{fill,stroke,corners,shadow}` (the one DateTimePickerFlyout surface, under each picker's `popup`) | field |
| W2-04 | `date_picker.popup.{fill,stroke,corners,shadow} @ variant text_field` (the same surface under the date picker) | rule |
| W2-04 | `date_picker.{field.fill,field.stroke,segment.fill,segment.text,segment_separator.stroke} @ {hover} / {pressed} / {focused} / {disabled} variant text_field` | rule |
| W2-04 | `date_picker.{header.text,field.min_width,field.max_width}` | field |
| W2-04 | `picker_wheel.{item.fill,item.text,item.padding,item.min_height,selection_band.fill,selection_band.min_height} @ context time_picker.popup / context date_picker.popup` (the looping selectors, their item height and highlight band: one home, `picker_wheel`) | rule |
| W2-04 | `picker_wheel.{item.fill,item.text,selection_band.fill} @ {hover} / {pressed} / {selected} context time_picker.popup` | rule |
| W2-04 | `picker_wheel.{item.fill,item.text} @ {hover} / {pressed} / {selected} context date_picker.popup` | rule |
| W2-04 | `button.{body.fill,icon.glyph,body.transform} @ variant previous context picker_wheel.container / variant next context picker_wheel.container (the up and down buttons)` | rule |
| W2-04 | `button.body.transform @ {pressed} variant previous context picker_wheel.container / {pressed} variant next context picker_wheel.container (their press scale)` | rule |
| W2-04 | `button.* @ variant @accept context time_picker / variant @dismiss context time_picker / variant @accept context date_picker / variant @dismiss context date_picker (the accept and dismiss row)` | rule |
| W2-04 | `time_picker.popup.transitions, date_picker.popup.transitions; time_picker.popup (absent = true) @ !{open}, date_picker.popup (absent = true) @ !{open} variant text_field` (the flyout's open animation: it enters from transparent by its transitions, S2.7) | rule |
| W2-05 | `color_picker.plane.{stroke,min_width,min_height} @ variant @box / variant @ring` | rule |
| W2-05 | `color_picker.crosshair.{stroke,min_width,min_height} (the selection ellipse)` | field |
| W2-05 | `color_picker.preview.corners (the previous and current colours)` | field |
| W2-05 | `slider.{track.fill,track.corners,thumb.fill,thumb.stroke} @ variant hue context any color_picker / variant alpha context any color_picker / variant value context any color_picker` | rule |
| W2-05 | `slider.track.fill @ variant alpha context any color_picker (a PaintKind::Pattern checkerboard layer under the gradient, ADD-B01)` | rule |
| W2-05 | `color_picker.checkerboard.fill (PaintKind::Pattern checkerboard, ADD-B01)` | field |
| W2-05 | `input.* @ context any color_picker (the hex and channel inputs)` | rule |
| W2-05 | `button.* @ context any color_picker (the more and fewer button)` | rule |
| W2-06 | `rating.star.glyph; rating.values.item_gap; rating.caption.text` | field |
| W2-06 | `rating.star.glyph.fill @ {filled} / !{filled} / {placeholder} / {preview,filled} / {preview} !{filled} / {preview,placeholder} / {disabled}` | rule |
| W2-06 | `rating.star.transform @ {preview} (the hover preview scale, to verify)` | rule |
| W2-07 | `banner.{body.fill,icon_backplate.fill,icon.glyph} @ variant info / variant success / variant warning / variant error` | rule |
| W2-07 | `banner.{title.text,message.text,body.stroke,body.min_height,body.padding}; banner.values.actions_wrap; banner.{body.padding,icon.margin,title.margin,message.margin,action_area.margin} @ variant vertical` (the horizontal and vertical layouts' padding and margins; the connector sets `vertical`) | rule |
| W2-07 | `button.{body.min_width,body.min_height,icon.glyph} @ variant close context banner.body` | rule |
| W2-07 | `button.* @ context banner.action_area (the action button)` | rule |
| W2-07 | `link.text.text.fill @ context any banner` | rule |
| W2-08 | `badge.{body,label,icon}.* @ variant dot / variant count / variant icon` | rule |
| W2-08 | `badge.{body.fill,label.text.fill,icon.icon.fill} @ variant attention / variant info / variant success / variant warning / variant error` | rule |
| W2-08 | `badge.body.{min_width,min_height,padding,corners}; badge.label.text` | field |
| W2-09 | `teaching_tip.{arrow.fill,arrow.stroke,arrow.min_width,arrow.min_height,arrow.margin,top_highlight.fill,hero.padding,body.stroke,body.shadow,body.min_width,body.max_width}; teaching_tip.values.light_dismiss` | field |
| W2-09 | `teaching_tip.hero.padding @ variant @hero_bottom` | rule |
| W2-09 | `teaching_tip.{title,subtitle}.text.fill` | field |
| W2-09 | `button.{body.min_width,body.min_height,icon.glyph} @ variant close context teaching_tip.body / variant close context teaching_tip.body variant @alternate_close` | rule |
| W2-09 | `button.* @ context teaching_tip.action_area` | rule |
| W2-09 | `teaching_tip.values.presentations (events @expand, @collapse)` | field |
| W2-09 | `WinUiTeachingTip (teaching_tip.values.placement: the tail's placement)` | placement |
| W2-10 | `breadcrumb.crumb_label.text; breadcrumb.separator.{glyph,padding}` | field |
| W2-10 | `breadcrumb.crumb_label.text.fill @ {hover} / {pressed} / {disabled} / {focus_visible} / {current} / {current,hover} / {current,pressed} / {current,disabled} / {current,focus_visible}` | rule |
| W2-10 | `breadcrumb.ellipsis.{glyph,padding}` | field |
| W2-10 | `breadcrumb.ellipsis.glyph.fill @ {hover} / {pressed}` | rule |
| W2-10 | `menu.{body.fill,item.fill} @ context breadcrumb.bar (the ellipsis flyout)` | rule |
| W2-10 | `menu.item.fill @ {hover} / {pressed} context breadcrumb.bar` | rule |
| W2-11 | `view_switcher.{bar.fill,bar.min_height,indicator.fill,indicator.min_width,indicator.min_height,item_separator.stroke} @ variant @navigation_top` | rule |
| W2-11 | `view_switcher.{item.fill,item_label.text.fill} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {disabled} variant @navigation_top` | rule |
| W2-11 | `button.{body.min_width,body.fill,icon.glyph} @ variant overflow context view_switcher.bar (the More button)` | rule |
| W2-11 | `view_switcher.item.* @ variant @navigation_top (the settings item: an item)` | rule |
| W2-12 | `list.{row_background.fill,row_background.stroke,row.text.fill} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {disabled}` | rule |
| W2-12 | `list.values.indent_per_level` | field |
| W2-12 | `list.disclosure.glyph; list.disclosure.transitions` | field |
| W2-12 | `list.disclosure.transform @ {open}` | rule |
| W2-12 | `list.row_selection_indicator.{fill,corners,min_width,min_height}` | field |
| W2-12 | `checkbox.* @ context list.row (the multi-select check box)` | rule |
| W2-12 | `list.row.min_height` | field |
| W2-12 | `list.row.min_height @ density compact` | rule |
| W2-12 | `list.{row.opacity,row_background.fill} @ {dragged} / {@reorder_target}` | rule |
| W2-13 | `list.row_background.stroke @ {selected} variant grid / {selected,hover} variant grid / {selected,pressed} variant grid / {selected,disabled} variant grid` | rule |
| W2-13 | `list.row_inner_border.stroke @ {selected} variant grid` | rule |
| W2-13 | `checkbox.{box.fill,box.stroke,box.corners,mark.glyph} @ context list.row / context list.row {selected} / context list.row {hover} / context list.row {pressed} context list.view variant grid` | rule |
| W2-13 | `list.values.check_box_placement @ variant grid (the corner)` | rule |
| W2-13 | `list.row_background.corners @ variant grid` | rule |
| W2-13 | `list.{row.opacity,row_background.fill} @ {dragged} variant grid` | rule |
| W2-14 | `list.row_background.fill @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} (ItemContainer's selection visual)` | rule |
| W2-14 | `list.row_inner_border.stroke @ {selected}` | rule |
| W2-14 | `list.values.check_box_placement` | field |
| W2-14 | `list.row.opacity @ {disabled}` | rule |
| W2-15 | `input.* @ context any search_field (the text box)` | rule |
| W2-15 | `button.{body.padding,icon.glyph} @ variant @query context search_field.body` | rule |
| W2-15 | `button.* @ variant clear_field context input.body (its clear button)` | rule |
| W2-15 | `menu.body.{fill,stroke,shadow,max_height} @ context autocomplete` | rule |
| W2-15 | `menu.item.fill @ {hover} / {pressed} / {focused} context autocomplete` (the suggestion list's selected item is the keyboard-highlighted suggestion: `focused`, S7.2) | rule |
| W2-15 | `menu.body.corners @ {placed_above} context autocomplete / {placed_below} context autocomplete (square where attached)` | rule |
| W2-15 | `WinUiAutoSuggest (autocomplete.values.placement)` | placement |
| W2-15 | `button.icon.transitions @ variant @query context search_field.body` (the row's chevron animation; which glyph animates is the gap map's to verify) | rule |
| W2-16 | `button.{icon.glyph,body.fill} @ variant reveal context input.body variant secure (the eye glyph)` | rule |
| W2-16 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {checked} variant reveal context input.body variant secure` | rule |
| W2-16 | `input.values.{mask_character,peek_while_pressed}` | field |
| W2-17 | `split_button.{primary,secondary}.{fill,stroke,min_width,min_height}; split_button.divider.stroke` | field |
| W2-17 | `split_button.{primary.fill,primary.stroke,secondary.fill,secondary.stroke,primary_label.text.fill,arrow.glyph.fill} @ {hover} / {pressed} / {disabled}` | rule |
| W2-17 | `split_button.{primary.fill,primary.stroke,secondary.fill,secondary.stroke,primary_label.text.fill,arrow.glyph.fill} @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} variant toggle` | rule |
| W2-17 | `split_button.divider.stroke @ {checked} variant toggle` | rule |
| W2-18 | `button.{menu_indicator.glyph,values.menu_indicator_gap} @ variant menu` | rule |
| W2-18 | `button.menu_indicator.glyph.fill @ {hover} / {pressed} / {disabled} variant menu` | rule |
| W2-18 | `button.menu_indicator.transitions @ variant menu (TransitionKind::Keyed per AnimatedIcon state, the press animation; ADD-A27)` | rule |
| W2-19 | `button.{body.fill,body.stroke,label.text.fill} @ variant @repeat / {hover} variant @repeat / {pressed} variant @repeat / {disabled} variant @repeat` | rule |
| W2-19 | `button.values.{repeat_delay,repeat_interval} @ variant @repeat` | rule |
| W2-19 | `reader:windows:SPI_GETKEYBOARDDELAY/SPI_GETKEYBOARDSPEED relation to RepeatButton.Delay/Interval` | reader |
| W2-20 | `page_indicator.dot.{fill,min_width,min_height}` | field |
| W2-20 | `page_indicator.dot.{fill,min_width,min_height} @ {selected} / {hover} / {pressed} / {disabled} / {selected,hover} / {selected,pressed}` | rule |
| W2-20 | `button.{body.fill,icon.glyph,icon.glyph.fill} @ variant previous context page_indicator.body / variant next context page_indicator.body` | rule |
| W2-20 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant previous context page_indicator.body` | rule |
| W2-20 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant next context page_indicator.body` | rule |
| W2-20 | `page_indicator.dot.{min_width,min_height} @ variant vertical` | rule |
| W2-20 | `page_indicator.values.{nav_buttons_visibility,max_visible}` | field |
| W2-21 | `page_indicator.selection_indicator.fill` | field |
| W2-21 | `button.{body.fill,body.min_width,label.text} @ variant @page_number context page_indicator` (PagerControl's number buttons) | rule |
| W2-21 | `button.{body.fill,label.text} @ {hover} / {pressed} / {selected} / {disabled} variant @page_number context page_indicator` | rule |
| W2-21 | `button.* @ variant previous context page_indicator.body / variant next context page_indicator.body / variant @first context page_indicator.body / variant @last context page_indicator.body` | rule |
| W2-21 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} context page_indicator.body` | rule |
| W2-21 | `page_indicator.* @ variant @button_panel / variant @combo_box / variant @number_box` | rule |
| W2-21 | `combo_box.* @ context page_indicator.body (the ComboBox display mode)` | rule |
| W2-21 | `stepper.* @ context page_indicator.body (the NumberBox display mode)` | rule |
| W2-22 | `menu_bar.bar.{fill,min_height}; menu_bar.item.{padding,fill,stroke}; menu_bar.values.item_gap (the item margin)` | field |
| W2-22 | `menu_bar.item.{fill,stroke} @ {hover} / {pressed} / {open}` | rule |
| W2-22 | `menu.* @ context menu_bar.item (the MenuFlyouts it opens)` | rule |
| W2-23 | `edit_menu.body.{fill,stroke,corners,shadow}; edit_menu.primary_commands.padding` | field |
| W2-23 | `button.* @ variant tool context edit_menu.body (the primary commands)` | rule |
| W2-23 | `button.{body.fill,label.text.fill} @ {checked} / {checked,hover} / {checked,pressed} variants {tool,toggle} context edit_menu.body` | rule |
| W2-23 | `menu.* @ context edit_menu.body (the secondary menu, its keyboard accelerator text)` | rule |
| W2-23 | `button.* @ variant overflow context edit_menu.body (the expand button)` | rule |
| W2-23 | `edit_menu.body.corners @ {@expanded} (joined to the secondary menu)` | rule |
| W2-23 | `menu.body.corners @ context edit_menu.body {@expanded}` | rule |
| W2-23 | `edit_menu.values.presentations (events Appear, Disappear, @expand, @collapse)` | field |
| W2-24 | `avatar.{body.fill,body.stroke,initials.text,fallback_icon.icon}; avatar.values.default_size` | field |
| W2-24 | `badge.{body.fill,body.stroke,label.text.fill,icon.icon} @ context avatar.body` | rule |
| W2-24 | `badge.values.{anchor_offset_x,anchor_offset_y} @ context avatar.body (the badge margin)` | rule |
| W2-24 | `badge.icon.{icon,opacity} @ context avatar.body` (the image badge and its opacity) | rule |
| W2-25 | `radio_group.header.text.fill` | field |
| W2-25 | `radio_group.header.text.fill @ {disabled}` | rule |
| W2-25 | `radio_group.header.padding` | field |
| W2-25 | `radio_group.values.row_gap` | field |
| W2-25 | `radio_group.values.column_gap` | field |
| W2-26 | `pull_to_refresh.indicator.{fill,min_width,min_height}; pull_to_refresh.arrow.glyph` | field |
| W2-26 | `pull_to_refresh.values.{trigger_distance,loop}` | field |
| W2-27 | `view_switcher.{item.padding,item_label.padding,indicator.fill,indicator.min_width,indicator.min_height}; view_switcher.values.item_gap` | field |
| W2-27 | `view_switcher.{item.fill,item.stroke,item_icon.icon,item_label.text.fill,indicator.fill} @ {hover} / {pressed} / {selected} / {disabled} variant @selector_bar` | rule |
| W2-27 | `view_switcher.values.presentations (event @indicator_move, part indicator)` | field |
| W2-28 | `scrollbar.annotation_labels.{text,min_width}` | field |
| W2-28 | `scrollbar.thumb.{fill,corners,min_height} @ variant @annotated` | rule |
| W2-28 | `tooltip.body.{min_height,max_width} @ context scrollbar.body variant @annotated (the hover tooltip)` | rule |
| W2-28 | `scrollbar.arrow_button.fill @ variant @annotated` | rule |
| W2-29 | `list.swipe_action.{fill,text,icon}` | field |
| W2-29 | `list.swipe_action.{fill,text.fill} @ {pressed} / {@pre_threshold} / {@post_threshold}` | rule |
| W2-29 | `list.values.presentations (event @swipe, queried with the row: the swipe actions' reveal)` | field |
| W2-30 | `sidebar.values.{width,collapsed_width}; sidebar.pane.{stroke,corners}` | field |
| W2-30 | `sidebar.scrim.fill @ variant overlay` (the light-dismiss overlay; `scrim` drawn on `windows-11` under `overlay`) | rule |
| W2-30 | `sidebar.values.presentations (events Appear, Disappear)` | field |
| W2-31 | `button.{body.fill,icon.glyph,icon.glyph.fill} @ variant previous context carousel.body / variant next context carousel.body (the glyph and its size)` | rule |
| W2-31 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} variant previous context carousel.body` | rule |
| W2-31 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} variant next context carousel.body` | rule |
| W2-31 | `button.body.transform @ {pressed} variant previous context carousel.body / {pressed} variant next context carousel.body` | rule |
| W2-31 | `carousel.values.nav_buttons_visibility; carousel.item.fill` | field |
| W2-32 | `view_switcher.{item.fill,item_label.text} @ variant @pivot / {selected} variant @pivot / {hover} variant @pivot / {pressed} variant @pivot / {disabled} variant @pivot` | rule |
| W2-32 | `view_switcher.item.padding @ variant @pivot (the header margins)` | rule |
| W2-32 | `button.* @ variant previous context view_switcher.bar / variant next context view_switcher.bar (the nav buttons)` | rule |
| W2-32 | `view_switcher.values.presentations @ variant @pivot (event @indicator_move, part indicator)` | rule |
| W2-32 | `view_switcher.title.{text,padding} @ variant @pivot` (the Pivot's title) | rule |
| W2-33 | `list.row.padding @ variant @listbox` | rule |
| W2-33 | `list.{row_background.fill,row.text.fill} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {disabled} variant @listbox` | rule |
| W2-33 | `list.view.stroke @ variant @listbox` | rule |
| W2-34 | `button.{body.fill,body.stroke,icon.glyph,label.text} @ variant @zoom_out context component.semantic_zoom` | rule |
| W2-34 | `button.{body.fill,body.stroke,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant @zoom_out context component.semantic_zoom` | rule |
| W2-34 | `component.semantic_zoom.values.presentations (event @zoom)` | component |
| W2-35 | `media_controls.{bar.fill,time_label.text}; media_controls.values.auto_hide_ms; media_controls.values.presentations (event Disappear)` | field |
| W2-35 | `button.* @ variant play context media_controls.bar / variant volume context media_controls.bar / variant full_screen context media_controls.bar / variant frame_step context media_controls.bar` | rule |
| W2-35 | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} context media_controls.bar` | rule |
| W2-35 | `popover.* @ context media_controls.bar (the volume flyout)` | rule |
| W2-35 | `slider.* @ context any media_controls (the seek slider and the volume slider)` | rule |
| W2-36 | `ink_toolbar.{bar.fill,palette.fill,palette.corners,palette.shadow,palette_swatch.stroke,stroke_width_selector.fill,ruler.fill,ruler.stroke}` | field |
| W2-36 | `button.{body.fill,icon.icon} @ variant tool context ink_toolbar` (the tool buttons, Nested controls) | rule |
| W2-36 | `button.body.fill @ {selected} / {hover} / {pressed} variant tool context ink_toolbar` | rule |
| W2-36 | `ink_toolbar.values.palette_placement` | field |
| W2-37 | `component.key_tip.badge.{fill,stroke,corners,padding}` | component |
| W2-37 | `component.key_tip.text.text` | component |
| W2-37 | `WinUiKeyTip (component.key_tip.values.placement: its offset the placement's gap)` | placement |
| W2-40† | `form.card.{fill,stroke} @ {hover} / {pressed} / {disabled}` (each SettingsCard its own `card` element) | rule |
| W2-40† | `form.row_title.text.fill @ context form.card {hover} / context form.card {pressed} / context form.card {disabled}` | rule |
| W2-40† | `form.card.{stroke,padding,min_width,min_height}; form.row_subtitle.text; form.row_icon.{icon,max_width,max_height,padding}; form.row_arrow.{icon,padding}` | field |
| W2-40† | `form.values.column_gap (the header to content spacing)` | field |
| W2-40† | `form.* @ variant @right / variant collapsed / variant @right_wrapped_no_icon / variant @left / variant vertical` (RightWrapped is `collapsed`) | rule |
| W2-40† | `form.values.{collapse,breakpoints} (AtBreakpoints; measured Allocation, width Below the wrap threshold setting `collapsed` and Below the no-icon threshold setting `@right_wrapped_no_icon`, under `@right`)` | field |
| W2-40† | `form.row_icon.opacity @ {disabled} variant bitmap_icon` (the bitmap header icon dimmed; a glyph icon is recoloured, W3-51†) | rule |
| W2-40† | `form.{row.padding,row.stroke,row_arrow.icon} @ variant expander / variant action (the expander's header and items, their top border, the chevron button, the clickable items)` | rule |
| W2-41† | `level_indicator.{track.fill,fill.fill,tick.fill,needle.fill,needle.stroke,needle.min_width,needle.min_height,value_label.text,unit_label.text} @ variant circular / cond contrast_high variant circular` (the scale arc is `track`, the trail arc `fill`: one structure with iOS's circular gauge); `level_indicator.tick.fill @ variant @scale` (the scale ticks) | rule |
| W2-41† | `level_indicator.body.opacity @ {disabled} variant circular` | rule |
| W2-41† | `level_indicator.values.{start_angle_deg,end_angle_deg,thickness,tick_spacing,end_cap}` (the arcs' round caps: `end_cap`) | field |
| W2-42† | `color_button.{swatch.corners,swatch.min_width,swatch.min_height,swatch.stroke,body.padding}; color_button.checkerboard.fill (PaintKind::Pattern checkerboard, ADD-B01)` | field |
| W2-42† | `color_button.chevron.glyph` | field |
| W2-42† | `color_button.chevron.glyph.fill @ {hover} / {pressed}` | rule |
| W2-42† | `color_button.{body.fill,body.stroke} @ {hover} / {pressed} / {disabled}` | rule |
| W2-42† | `color_picker.* @ context color_button.body (the Toolkit's ColorPicker in its flyout)` | rule |
| W2-43† | `chip.body.{fill,stroke,padding,corners}` | field |
| W2-43† | `chip.{body.fill,body.stroke,label.text.fill} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed}` | rule |
| W2-43† | `button.{icon.glyph,body.corners} @ variant remove context chip.body (round)` | rule |
| W2-43† | `button.{body.fill,icon.glyph.fill} @ variant remove context chip.body {selected}` | rule |
| W2-43† | `layout.values.{item_gap,line_gap} @ variant flow context any input (the token spacing)` | rule |
| W2-43† | `input.body.padding (the box's padding)` | field |
| W3-01 | `token.elevation (one entry per WinUI elevation step, keys declared: each entry's ShadowLayer x, y, blur, spread, color, inset; the ambient layer only)` | token |
| W3-01 | `token.elevation @ cond scheme:dark (the shadows' intensity per theme)` | rule |
| W3-02 | `PartStyle.shadow (Shadow::Elevation of each control's root part) @ {hover} / {pressed}` | rule |
| W3-03 | `menu.body.shadow; stepper.popup.shadow; teaching_tip.body.shadow; tab.tab.shadow (Shadow::Elevation, the step each element sits at)` | field |
| W3-03 | `menu.body.shadow @ context combo_box / context button.body variant overflow` | rule |
| W3-03 | `sidebar.pane.shadow @ variant overlay` | rule |
| W3-04 | `PartStyle.stroke (each Edge.paint a PaintKind::Linear over an absolute extent, ADD-A31: ControlElevationBorderBrush on button.body.stroke, AccentControlElevationBorderBrush under variant primary, TextControlElevationBorderBrush on input.body.stroke, CircleElevationBorderBrush on the circular controls)` | field |
| W3-05 | `PartStyle.shadow @ cond contrast_high / cond reduced_transparency (dropped or kept, measured)` | rule |
| W3-05 | `PartStyle.fill (PaintKind::Material) @ cond contrast_high / cond reduced_transparency (Material.fallback)` | rule |
| W3-06 | `PartStyle.fill (PaintKind::Material @acrylic_background, @acrylic_in_app: tint, luminosity, saturation, blur_radius, noise_opacity, fallback; the exclusion blend and the noise texture in Material.params, ADD-A47; per scheme by their colour references)` | field |
| W3-06 | `declare.materials (acrylic_background, acrylic_in_app)` | field |
| W3-07 | `PartStyle.fill @ cond reduced_transparency / cond battery_saver / cond contrast_high / cond system_setting:material_policy / cond window_inactive (Material.fallback; window_inactive for background acrylic)` | rule |
| W3-07 | `cond system_setting:material_policy (the material policy AcrylicBrush follows, low-end hardware among its causes; ADD-A04)` | condition |
| W3-07 | `reader:windows:the material policy state (AcrylicBrush's OnMaterialPolicyStatusChanged; its inputs to verify)` | reader |
| W3-07 | `PartStyle.transitions (crossfade, tint transition)` | field |
| W3-07 | `cond battery_saver` | condition |
| W3-08 | `Material.tint (an expression on role.global.accent or its ramp, role.global.accent_ramp, for the accent acrylic materials @accent_acrylic_background, @accent_acrylic_in_app)` | field |
| W3-09 | `role.@layer_fill_default, role.@layer_fill_alt, role.@layer_on_acrylic_fill_default, role.@layer_on_accent_acrylic_fill_default, role.@layer_on_mica_base_alt_*` | role |
| W3-09 | `PartStyle.fill (a Color layer over the backdrop's Material: the content and commanding layers, Paint.layers)` | field |
| W3-10 | `window.content.fill @ cond window_inactive (PaintKind::Material @mica to Material.fallback)` | rule |
| W3-10 | `window.content.transitions (the material to colour crossfade)` | field |
| W3-11 | `role.global.scrim` | role |
| W3-11 | `dialog.scrim.fill` | field |
| W3-11 | `menu.dismiss_overlay.fill; popover.dismiss_overlay.fill (the light-dismiss overlays when enabled)` | field |
| W3-12 | `role.global.accent_ramp (Light1-3, Dark1-3)` | role |
| W3-12 | `input.accent_ramp` | input |
| W3-12 | `role.@accent_* (the accent fill tiers default, secondary, tertiary, disabled and the accent text tiers primary, secondary, tertiary, one keyed role per brush)` | role |
| W3-12 | `reader:windows:UISettings.GetColorValue AccentDark1-3, AccentLight1-3` | reader |
| W3-13 | `role.@text_tertiary` | role |
| W3-14 | `role.@text_on_accent_secondary, role.@text_on_accent_disabled, role.@text_on_accent_selected` | role |
| W3-15 | `role.@stroke_control, role.@stroke_card, role.@stroke_surface, role.@stroke_divider, role.@stroke_* (the strong, on-accent, flyout, inverse and on-image tiers, one keyed role per brush)` | role |
| W3-16 | `role.@fill_* tiers` | role |
| W3-16 | `role.@fill_subtle` | role |
| W3-16 | `role.@background_solid_*` | role |
| W3-17 | `role.global.status.{info,success,warning,error,neutral}.{background,background_solid} (attention, success, caution, critical, neutral backgrounds and their solid variants)` | role |
| W3-18 | `role.global.status.neutral.fill` | role |
| W3-18 | `role.@fill_* (the solid neutral fill, a fill tier)` | role |
| W3-45† | `PartStyle.fill (an alpha Color layer appended over base, fill.append, S2.3, R6: the translucent fill drawn over what lies behind)` | field |
| W3-45† | `button.body.fill, combo_box.body.fill, switch.track.fill @ {hover} / {pressed} (appended translucent layers, not flattened colours)` | rule |
| W3-46† | `role.@text_inverse` | role |
| W3-46† | `banner.icon.glyph.fill @ variant info / variant success / variant warning / variant error (= role.@text_inverse on the backplates)` | rule |
| W3-19 | `FocusIndicator.shadow (the inner ring, a shadow layer: its colour and width, R7, ADD-A17)` | field |
| W3-20 | `FocusIndicator.offset` (per part: each control's FocusVisualMargin, ADD-A18; SelectorBar's on `view_switcher.item.focus`) | field |
| W3-20 | `FocusIndicator.corners` (stated per control as WinUI draws it: an expression over `part.corners` and the offsets, the part's radius plus FocusVisualMargin, cited) | field |
| W3-20 | `FocusIndicator.kind` (Ring, its inner ring a shadow layer: the HighVisibility default; Reveal: `FocusKind::Keyed(@reveal)`; DottedLine: `FocusIndicator.dash` `Dotted`; ADD-A82) | field |
| W3-21 | `token.behavior.focus_visible_when (Keyboard { … }; ADD-A16)` | token |
| W3-21 | `reader:windows:SPI_GETKEYBOARDCUES` | reader |
| W3-22 | `token.text.rendering (ADD-B04)` | token |
| W3-22 | `reader:windows:SPI_GETCLEARTYPE, SPI_GETFONTSMOOTHING, SPI_GETFONTSMOOTHINGTYPE, SPI_GETFONTSMOOTHINGCONTRAST` | reader |
| W3-23 | `Font.axes (opsz: AxisValue::FollowsSize)` | field |
| W3-23 | `token.text.roles.*.{line_height,letter_spacing}` | token |
| W3-24 | `token.caret.width` | token |
| W3-24 | `token.caret.{blink_on,blink_off,blink_timeout}` | token |
| W3-24 | `reader:windows:SPI_GETCARETWIDTH, GetCaretBlinkTime, UISettings.CaretWidth, UISettings.CaretBlinkRate, SPI_GETCARETTIMEOUT` | reader |
| W3-24 | - (the text cursor indicator: drawn by the system) | out (R1.1) |
| W3-25 | `role.global.selection_text_background_unfocused` | role |
| W3-25 | `role.global.selection_text_foreground (Absent: WinUI keeps the text's own colour)` | role |
| W3-25 | `input.selection.fill @ context input.body !{focused}` | rule |
| W3-25 | `input.selection_handle.{glyph,fill,min_width,min_height} (the grippers)` | field |
| W3-26 | `menu.accelerator.text` | field |
| W3-26 | `menu.accelerator.text @ context button.body variant overflow` | rule |
| W3-26 | `tooltip.accelerator.text` | field |
| W3-27 | `checkbox.mark.glyph.size; expander.arrow.glyph.size; combo_box.arrow.glyph.size; scrollbar.arrow_button.glyph.size; window.caption_button_glyph.glyph.size; banner.icon.glyph.size; menu.submenu_arrow.glyph.size` | field |
| W3-27 | `button.icon.glyph.size @ variant close context tab.tab` | rule |
| W3-28 | `combo_box.arrow.transitions (TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | field |
| W3-28 | `expander.arrow.transitions (TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | field |
| W3-28 | `button.menu_indicator.transitions @ variant menu (TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | rule |
| W3-28 | `checkbox.mark.transitions (TransitionKind::Keyed per AnimatedIcon state, the accept animation; ADD-A27)` | field |
| W3-28 | `button.icon.transitions @ variant back context window.title_bar / variant back context sidebar.header / variant pane_toggle context sidebar.header / variant pane_toggle context window.title_bar` (the back arrow and the hamburger, `TransitionKind::Keyed`, ADD-A27) | rule |
| W3-28 | `list.leading_icon.transitions @ variant sidebar` (the settings gear, `TransitionKind::Keyed`, ADD-A27) | rule |
| W3-29 | `link.body.cursor; input.body.cursor; text_area.body.cursor; splitter.divider.cursor; window.resize_zone.cursor (the hand, the I-beam, the resize cursors)` | field |
| W3-29 | `token.pointer.size (ADD-A46)` | token |
| W3-29 | `reader:windows:UISettings.CursorSize, SM_CXCURSOR` | reader |
| W3-47† | `token.text.roles.monospace` | token |
| W3-47† | `reader:windows:HKCU\Console FaceName, FontSize, FontWeight` | reader |
| W3-48† | `token.behavior.mnemonics (ADD-A12)` | token |
| W3-48† | `reader:windows:SPI_GETKEYBOARDCUES` | reader |
| W3-49† | `drag_image.image.{fill,opacity}` | field |
| W3-49† | `drag_image.caption.text` | field |
| W3-49† | `drag_image.glyph.glyph @ {drop_copy} / {drop_move} / {@drop_link}` (the glyph per drop operation) | rule |
| W3-49† | `drag_image.drop_description.text` | field |
| W3-49† | `role.global.drop_highlight` | role |
| W3-49† | `list.row_background.fill @ {drop_target}` | rule |
| W3-50† | `token.pointer.{color,theme,size} (ADD-A46)` | token |
| W3-50† | `reader:windows:HKCU\Software\Microsoft\Accessibility (the pointer's colour and style; value names to verify)` | reader |
| W3-50† | - (the pointer's shadow, the Ctrl locator, pointer trails, the touch and pen contact and gesture visualisation) | out (R1.1) |
| W3-51† | `PartStyle.opacity of icon parts @ {disabled} variant bitmap_icon` (a bitmap icon dimmed; the content variant, ADD-A61) | rule |
| W3-51† | `PartStyle.glyph.fill @ {disabled} !variant bitmap_icon` (a glyph icon recoloured) | rule |
| W3-30 | `token.motion.durations.{control_normal,control_fast,control_fast_after,control_faster}` | token |
| W3-31 | `token.motion.timings.{fast_out_slow_in,slow_out_fast_in,control_fast_out_slow_in}` | token |
| W3-32 | `popover.values.presentations, dialog.values.presentations, menu.values.presentations, tooltip.values.presentations, list.values.presentations, tab.values.presentations, sidebar.values.presentations, toolbar.values.presentations, drag_image.values.presentations (one entry per WinUI theme transition: Popup, Pane, EdgeUI, AddDelete, Reorder, Reposition, FadeIn, FadeOut, SplitOpen, SplitClose, DragItem, DragOver, DropTargetItem)` | field |
| W3-32 | `PresentationKind::Keyed (ADD-A26: one declared key per theme transition none of the typed kinds is)` | field |
| W3-32 | `button.body.transform @ {pressed} (PointerDown and PointerUp: the press tilt)` | rule |
| W3-32 | `button.body.transitions (Transform: the tilt's motion)` | field |
| W3-32 | `navigation_view.values.presentations events @push, @pop` (Frame's Entrance, Content, DrillIn, Slide and Suppress navigation transitions, keyed kinds, ADD-A26) | field |
| W3-33 | `PresentationKind::Keyed { key: connected_animation } with its parameters (ADD-A26)` | field |
| W3-33 | `token.motion.durations.connected, token.motion.timings.connected` | token |
| W3-33 | `PartStyle.transform (translate as an expression on element.scroll_offset: ParallaxView, R25.3, ADD-B13)` | field |
| W3-34 | `PartStyle.transitions and widget presentation transitions per control` | field |
| W3-34 | `token.motion.durations.*` | token |
| W3-52† | `token.behavior.scroll (wheel_step_duration, wheel_step_timing, deceleration, overscroll, overscroll_effect, overscroll_extent, snap: the wheel animation, inertia, overpan and snap settle)` | token |
| W3-52† | `token.behavior.scroll @ cond animations_off` | rule |
| W3-52† | `token.motion.speed_factor @ cond animations_off` | rule |
| W3-52† | `reader:windows:SPI_GETWHEELSCROLLLINES, SPI_GETWHEELSCROLLCHARS` | reader |
| W3-53† | `cond system_setting:menu_fade / cond system_setting:tooltip_fade / cond system_setting:selection_fade / cond system_setting:combo_animation / cond system_setting:listbox_smooth_scroll / cond system_setting:hot_tracking / cond system_setting:drop_shadow / cond system_setting:ui_effects (ADD-A04)` | condition |
| W3-53† | `menu.values.presentations @ !cond system_setting:menu_fade (slide in place of fade)` | rule |
| W3-53† | `tooltip.values.presentations @ !cond system_setting:tooltip_fade (slide in place of fade)` | rule |
| W3-53† | `menu.values.presentations @ !cond system_setting:combo_animation context combo_box` | rule |
| W3-53† | `list.row_background.fill @ {hover} !cond system_setting:hot_tracking (no hover highlight)` | rule |
| W3-53† | `window.frame.shadow, menu.body.shadow @ !cond system_setting:drop_shadow` | rule |
| W3-53† | `menu.item.transitions @ !cond system_setting:selection_fade (the chosen item's fade-out)` | rule |
| W3-53† | `token.behavior.scroll @ !cond system_setting:listbox_smooth_scroll (list boxes' animated scrolling)` | rule |
| W3-53† | `PartStyle.transitions, <widget>.values.presentations @ !cond system_setting:ui_effects (the master switch)` | rule |
| W3-53† | `reader:windows:SPI_GETMENUFADE, SPI_GETTOOLTIPFADE, SPI_GETSELECTIONFADE, SPI_GETCOMBOBOXANIMATION, SPI_GETLISTBOXSMOOTHSCROLLING, SPI_GETHOTTRACKING, SPI_GETDROPSHADOW, SPI_GETUIEFFECTS` | reader |
| W3-35 | `role.@* (the eight high-contrast system colours Window, WindowText, Hotlight, GrayText, Highlight, HighlightText, ButtonFace, ButtonText: keyed roles the reader states under cond contrast_high, per contrast theme)` | role |
| W3-35 | `PartStyle.{fill,stroke,text,focus} @ cond contrast_high (each part's mapping to those roles: focus, disabled, selected-hover among them)` | rule |
| W3-35 | `progress_bar.track.stroke @ cond contrast_high (a border drawn only there)` | rule |
| W3-35 | `switch.track.stroke @ {checked} cond contrast_high (the on track's border drawn only there)` | rule |
| W3-35 | `PartStyle.{fill,shadow} @ cond contrast_high (materials to Material.fallback, shadows removed)` | rule |
| W3-35 | `cond contrast_high` | condition |
| W3-35 | `reader:windows:GetSysColor under high contrast, AccessibilitySettings.HighContrastScheme, UISettings.UIElementColor` | reader |
| W3-36 | `PartStyle.{min_height,padding} (= an expression on text_scale or font_size where the control grows with the text; ADD-A55)` | field |
| W3-36 | `TextStyle.{wrap,max_lines} (the wrapping)` | field |
| W3-36 | `IconStyle.size (= an expression on text_scale where the icon grows; ADD-A55)` | field |
| W3-37 | `PartStyle.transitions @ cond animations_off per animation` | rule |
| W3-37 | `token.motion.speed_factor @ cond animations_off` | rule |
| W3-37 | `reader:windows:UISettings.AnimationsEnabled, SPI_GETCLIENTAREAANIMATION` | reader |
| W3-38 | `Material.fallback @ cond reduced_transparency per material` | rule |
| W3-38 | `reader:windows:UISettings.AdvancedEffectsEnabled, registry EnableTransparency` | reader |
| W3-39 | `declare.widget.{input,text_area,combo_box,list,date_picker,time_picker,picker_wheel}.densities (compact)` (each entry's *Sizes and densities*) | field |
| W3-39 | `input.body.{min_height,padding}, text_area.body.{min_height,padding}, list.row.min_height, combo_box.body.{min_height,padding}, date_picker.field.padding, time_picker.field.padding @ density compact` | rule |
| W3-39 | `list.row.min_height @ variant sidebar density compact (the nav item height)` | rule |
| W3-39 | `picker_wheel.item.padding @ density compact context date_picker.popup / density compact context time_picker.popup (the picker flyouts)` | rule |
| W3-40 | `menu.item.padding @ context menu.body {opened_by_touch}` | rule |
| W3-40 | `menu.item.padding @ cond input:touch context combo_box` | rule |
| W3-40 | `menu.body.min_width @ cond input:touch context combo_box` | rule |
| W3-40 | `menu.body.min_width @ {opened_by_touch} context button.body variant overflow` | rule |
| W3-40 | `cond input:touch` | condition |
| W3-41 | `token.behavior.{double_click_time,hover_open_delay,drag_threshold,scroll_lines_per_step,key_repeat_delay}` | token |
| W3-41 | `menu.values.submenu_open_delay_ms` | field |
| W3-41 | - (`UISettings.MessageDuration`: how long the system's notifications stay, which the shell draws; WinUI has no timed message control) | out (R1.1) |
| W3-41 | `reader:windows:GetDoubleClickTime, UISettings.DoubleClickTime, SPI_GETMOUSEHOVERTIME, SPI_GETMENUSHOWDELAY, SM_CXDRAG, SM_CYDRAG, SPI_GETWHEELSCROLLLINES, SPI_GETKEYBOARDDELAY` | reader |
| W3-43 | - | preset (R1.3) |
| W3-44 | `window.title_bar.fill @ cond scheme:dark` | rule |
| W3-44 | `window.frame.stroke @ cond scheme:dark` | rule |
| W3-54† | `PartStyle rules @ dir rtl (the mirrored templates)` | rule |
| W3-54† | `token.icons.mirroring (S2.9)` | token |
| W3-54† | `Glyph.mirroring (AnimatedIcon's MirroredWhenRightToLeft; R25.8)` | field |
| W3-54† | `window.values.caption_button_layout @ dir rtl (the caption buttons on the left)` | rule |
| W3-55† | `PlacementParams.handedness for WinUiFlyout @ cond system_setting:menu_drop_alignment` (ADD-A22, ADD-A04: the preset's rule under the read condition, one source) | placement |
| W3-55† | - (Win32 menus' drop alignment) | preset (R1.3) |
| W3-55† | `reader:windows:SPI_GETMENUDROPALIGNMENT -> cond system_setting:menu_drop_alignment` | reader |
| W3-56† | - | out (R1.1) |
| W-SF01† | - | out (R1.2) |
| W-SF02† | - | out (R1.2) |
| W-SF03† | - | out (R1.2) |
| W-SF04† | - | out (R1.2) |
| W-SF05† | - | out (R1.2) |
| W-SF06† | - | out (R1.2) |
| W-SF07† | - | out (R1.2) |
| W-SF08† | - | out (R1.2) |
| W-SF09† | `tab.bar.* @ {drop_target}` | rule |
| W-SF09† | `tab.values.presentations event Move @ {drop_target}` (the target strip's tabs parting while a torn-out window hovers it, if the measurement finds a gap; the source tab's tearing out is W1-087's `@tear_out`) | rule |
| W-SF10† | `dialog.values.placement (CenteredInHost)` | placement |
| W-SF11† | `dialog.values.placement (CenteredInHost, host ClientArea or Window)`; `dialog.scrim` covers the placement's host area (the smoke) | placement |
| W-SF12† | `dialog.values.keyboard_avoidance (KeyboardAvoid(WinUiContentDialog))` | placement |
| W-SF13† | - | preset (R1.3) |
| W-RL01† | - | preset (R1.3) |
| W-RL02† | - | preset (R1.3) |
| W-RL03† | - | preset (R1.3) |
| W-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| W-FM01† | `reader:windows:IDWriteFontFallback::MapCharacters on an emoji string` | reader |
| W-FM02† | `token.text.fallbacks.per_language (ADD-A41)` | token |
| W-FM02† | `reader:windows:IDWriteFontFallback::MapCharacters per locale, LanguageFontGroup` | reader |
| W-FM02† | - (GDI's SystemLink font-linking chains: Win32 controls) | preset (R1.3) |
| W-P3-1† | `window.frame.corners @ cond tiled:left / cond tiled:right / cond tiled:top / cond tiled:bottom` | rule |
| W-P3-1† | - (the snap overlay, the Snap bar and the shared edge of snapped windows: drawn by the shell) | out (R1.1) |
| W-P3-2† | `window.title_bar (absent) @ cond full_screen` | rule |
| W-P3-2† | `window.values.full_screen_title_bar_reveal (SwipeFromTop)` | field |
| W-P3-2† | `window.values.presentations (event @title_bar_show)` | field |
| W-P3-2† | `window.{frame.corners,title_bar.min_height,values.default_size} @ variant @compact_overlay` | rule |
| W-P3-2† | `window.caption_button.{min_width,min_height} @ context window.frame variant @compact_overlay` | rule |
| W-P3-2† | - (entering and leaving each presenter: the system's window animation) | out (R1.1) |
| W-P3-3† | - | out (R1.1) |
| W-P3-4† | - | preset (R1.3) |
| W-P3-5† | `segmented_control.values.segment_width (EqualToWidest)` | field |
| W-P3-5† | `segmented_control.values.segment_width @ variant @pivot / variant @button (Fit)` | rule |
| W-P3-5† | `segmented_control.values.item_gap` | field |
| W-P3-5† | `segmented_control.values.item_gap @ variant @button` | rule |
| W-P3-6† | - | preset (R1.3) |
| W-P3-7† | - | out (R1.1) |
| W-P3-8† | - | out (R1.2) |
| W-P3-9† | `input.caret (absent = true), text_area.caret (absent = true) @ {text_selected}` (whether TextBox and RichEditBox hide the caret while a selection exists: to verify) | rule |
| W-P3-9† | `input.drop_caret.{fill,min_width} @ {drop_target} (the drop position, to verify)` | rule |
| W-P3-9† | `text_area.drop_caret.{fill,min_width} @ {drop_target} (RichEditBox's, to verify)` | rule |
| W-P3-10† | `token.behavior.hide_pointer_while_typing` | token |
| W-P3-10† | `reader:windows:SPI_GETMOUSEVANISH` | reader |
| W-P3-10† | - (the Working in Background cursor while a process starts: the system's) | out (R1.1) |
| W-P3-11† | `cond system_setting:show_sounds (ADD-A04)` (SM_SHOWSOUNDS: what the system leaves the application to act on, R1.1; no preset rule selects on it, so the application must read it: a getter for the reader's conditions and S12's exemption for such a key are a needed core change) | condition |
| W-P3-11† | `reader:windows:SM_SHOWSOUNDS` | reader |
| W-P3-11† | - (the SoundSentry flash) | out (R1.1) |
| W-P3-12† | `token.behavior.{double_click_distance,double_click_height} (ADD-A45)` | token |
| W-P3-12† | `token.behavior.touch.double_tap_slop (the touch and pen tolerances, to verify)` | token |
| W-P3-12† | `reader:windows:SM_CXDOUBLECLK, SM_CYDOUBLECLK` | reader |
| W-P3-13† | `PartStyle.stroke (each Edge.snap a PixelSnap, ADD-A33: WinUI's layout rounding of thin lines)` | field |
| W-P3-13† | - (the display scale per monitor: the windowing system's) | out (R1.1) |
| W-P3-14† | - | newest (R1.5) |
| W-P3-15† | - | preset (R1.3) |
| W-P3-16† | `token.theme_change` | token |
| W-P3-17† | `autocomplete.token.text, autocomplete.token.fill; autocomplete.values.trigger_prefix` | field |
| W-P3-17† | `menu.{body.fill,body.stroke,body.shadow,body.corners,item.fill} @ context autocomplete (AutoSuggestBox's list, its radius the overlay radius)` | rule |
| W-P3-17† | `WinUiAutoSuggest (autocomplete.values.placement, relative to the caret: to verify)` | placement |
| W-RV-1† | `image_cropper.frame.{fill,stroke,corners,padding}; image_cropper.{mask.fill,overlay.fill}` | field |
| W-RV-1† | `image_cropper.thumb.{min_width,min_height,corners,stroke,fill} @ variant corner / variant edge` | rule |
| W-RV-1† | `image_cropper.thumb.fill @ {hover} / {pressed} / {disabled}` | rule |
| W-RV-1† | `image_cropper.thumb.focus @ {focus_visible}` | rule |
| W-RV-2† | `list.row_background.fill, list.row.min_height @ {redacted} / {redacted} variant grid` (the not-yet-loaded row's parts, styled by the state flag `redacted`, the one flag for content shown as a placeholder while it loads, ADD-A14) | rule |
| W-RV-2† | `list.row_background.fill @ {redacted} cond contrast_high` | rule |
| W-RV-2† | `list.values.presentations (event @placeholder_fade)` | field |
| W-RV-3† | `component.tabbed_command_bar.content_border.{fill,stroke,corners}` | component |
| W-RV-3† | `component.tabbed_command_bar.content_border.{fill,stroke} @ cond contrast_high (the window colour, a transparent stroke)` | rule |
| W-RV-3† | `view_switcher.* @ variant @navigation_top context component.tabbed_command_bar.body (the tab row, W2-11's look)` | rule |
| W-RV-3† | `toolbar.* @ context component.tabbed_command_bar.body (one command bar per tab)` | rule |
| W-RV-3† | `component.tabbed_command_bar.element_container.padding` | component |
| W-RV-3† | `view_switcher.item_label.text.fill @ variant @contextual context component.tabbed_command_bar.body` | rule |
| W-RV-3† | `view_switcher.item_label.text.fill @ {hover} / {pressed} / {disabled} / {selected} / {selected,hover} / {selected,pressed} variant @contextual context component.tabbed_command_bar.body` | rule |
| W-RV-3† | `component.tabbed_command_bar.values.presentations (event @indicator_move, a step on part content_border: PresentationKind::Keyed { key: pop_in })` | component |
| W-RV-3† | `button.values.label_position @ context any component.tabbed_command_bar (TextBesideIcon)` | rule |
| W-P4-1† | `window.launch_screen.fill` | field |
| W-P4-1† | `window.launch_icon.icon (the splash image's size)` | field |
| W-P4-1† | `window.{launch_screen.fill,launch_icon.icon} @ cond contrast_high` | rule |
| W-P4-1† | `window.values.presentations (event @launch_exit)` | field |
| W-P4-2† | - | out (R1.1) |
| W-P4-3† | `window.values.keyboard_avoidance (KeyboardAvoid(WinUiScrollIntoView))` | placement |
| W-P4-3† | `token.motion.durations.scroll_into_view, token.motion.timings.scroll_into_view (the scroll into view, measured)` | token |
| W-P4-4† | - | out (R1.1) |
| W-P4-5† | `token.pointer.drag_cursors.<drop operation> (ADD-A46)` | token |
| W-P4-6† | - | out (R1.1) |
| W-P4-7† | - | out (R1.1) |
| W-P4-8† | - | out (R1.1) |
| W-P4-9† | `layout.values.{item_gap,line_gap} @ variant flow` | rule |
| W-P4-10† | - | preset (R1.3) |
| W-P4-11† | `list.values.drag_hover_delay_ms` | field |
| W-P4-12† | `token.icons.mirroring (S2.9; the "Mirrored" code points as the right-to-left forms)` | token |
| W-P4-12† | `Element.direction` (icon lookup by the element's direction, S2.9, S8; ADD-B05: the "Mirrored" code point as the right-to-left form) | field |
| W-P4-13† | - | preset (R1.3) |
| W-P4-14† | - | out (R1.1) |
| W-P4-15† | - | out (R1.1) |
| W-P4-16† | `scrollbar.values.repeat_delay; scrollbar.values.repeat_interval` | field |
| W-P4-16† | `scrollbar.values.trough_click` | field |
| W-P4-17† | `menu.values.submenu_close_delay_ms (= the read MenuShowDelay)` | field |
| W-P4-17† | `menu.values.submenu_switch (CloseTimer, for top-level sub-items and the mouse only)` | field |
| W-P4-17† | `menu.values.submenu_switch @ cond input:touch (Immediate: touch skips the timers)` | rule |
| W-P4-17† | `reader:windows:HKCU Control Panel Desktop MenuShowDelay (also the close delay)` | reader |
| W-P4-18† | `component.casting_picker.picker.{fill,corners,shadow}` | component |
| W-P4-18† | `component.casting_picker.title.text` | component |
| W-P4-18† | `component.casting_picker.item.{fill,text,min_height}` | component |
| W-P4-18† | `component.casting_picker.item.{fill,text.fill} @ {hover} / {selected}` | rule |
| W-P4-18† | `WinUiFlyout (component.casting_picker.values.placement)` | placement |
| W-P4-18† | `button.{body.fill,icon.glyph} @ variant route context media_controls.bar (the CastButton)` | rule |
| W-P4-18† | `button.{body.fill,icon.glyph.fill} @ {hover} / {pressed} / {disabled} variant route context media_controls.bar` | rule |
| W-P4-18† | `reader:windows:DevicePickerAppearance getters` | reader |
| W-P4-19† | `sidebar.values.breakpoints` (NavigationView: measured Allocation, `compact` with width Below the expanded threshold, `@left_minimal` with width Below the compact threshold, its rules after `compact`'s; `NavigationView.cpp:1428-1458`) | field |
| W-P4-19† | `layout.values.{max_panes_horizontal,max_panes_vertical,pane_width,pane_height,breakpoints} @ variant pane` (TwoPaneView; `breakpoints` its wide and tall thresholds: measured Allocation, width Above setting `@two_pane_wide`, height Above setting `@two_pane_tall`, the wide rules after the tall ones, `TwoPaneView.cpp:103-150`; R16) | rule |
