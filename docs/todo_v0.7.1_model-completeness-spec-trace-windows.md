# v0.7.1 — Model completeness: Specification, traceability (Windows)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-windows.md`](todo_v0.7.1_model-completeness-gap-map-windows.md) mapped to
what carries it (R24), one line per (row, target), the rows in the order of the platform file. Withdrawn rows and
the pointer rows the map does not count are not listed; the cross-check and pass decision tables are decisions,
not rows.

**Targets** use the widget catalogue's names
([`todo_v0.7.1_model-completeness-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md), after its
rename table): `widget.part.field` (a `PartStyle` field, S2.11), `widget.values.name` (a widget value),
`component.<key>.<part>` (a keyed component), `role.<set>.<role>`, `role.global.<role>`, `role.@<key>` (S3),
`token.<path>` (S4), `input.<name>` with kind `input` (an S9 input, not the `input` widget), `reader:<platform>:<what
is read>` (S9.3), a placement strategy, a system control (S15), a cond or flag, or a core type
(`PartStyle.…`, `FocusIndicator.…`) where the row concerns every part. The selector of a rule follows `@`:
`{a,b}` states that hold together, `!{a}` none of these states, `variant v` or `variants {v,w}` (`@v` keyed, `!`
negated), `cond c` (a condition, S7.1: `window_inactive`, `scheme:dark`, `input:touch`, `system_setting:<key>`; `!`
negated), `context w.part [variant v] [{a}]` or `context any w` (an ancestor), `not_context w.part`, `pos first`,
`pos last`, `pos only`, `pos parity odd`, `prev w{…}`, `next w{…}` (siblings), `size s`, `density d`, `h_size_class c`,
`v_size_class c`, `dir rtl`; ` / ` separates alternatives, each a rule of its own (sharing the selector parts that
follow the last alternative). A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| W1-001 | `window.frame.stroke` | field |
| W1-001 | `window.frame.stroke @ cond window_inactive / cond scheme:dark` | rule |
| W1-001 | `window.frame.stroke @ cond system_setting:accent_on_title_bars !cond window_inactive` | rule |
| W1-001 | `reader:windows:HKCU DWM ColorPrevalence (system_setting)` | reader |
| W1-002 | `window.frame.stroke (width)` | field |
| W1-002 | `reader:windows:DWMWA_VISIBLE_FRAME_BORDER_THICKNESS` | reader |
| W1-003 | `window.frame.shadow (Elevation 128)` | field |
| W1-003 | `window.frame.shadow @ cond window_inactive` | rule |
| W1-004 | `window.frame.corners` | field |
| W1-004 | `window.frame.corners @ cond maximized / cond tiled` | rule |
| W1-004 | `window.frame.corners @ variant @round_small` | rule |
| W1-005 | `window.content.fill (mica material, fallback)` | field |
| W1-005 | `window.content.fill @ variant @mica_alt` | rule |
| W1-005 | `window.content.fill @ cond window_inactive` | rule |
| W1-005 | `window.content.transitions (material to colour crossfade)` | field |
| W1-006 | `window.title_bar.min_height` | field |
| W1-006 | `window.title_bar.min_height @ variant @tall / variant @collapsed / variant @titlebar_control_expanded` | rule |
| W1-006 | `reader:windows:AppWindowTitleBar.Height` | reader |
| W1-007 | `window.caption_button.{min_width,min_height}` | field |
| W1-007 | `window.caption_button_glyph.glyph (FontCodePoint) @ variant close / variant minimize / variant maximize / variant restore` | rule |
| W1-007 | `reader:windows:DWMWA_CAPTION_BUTTON_BOUNDS, AppWindowTitleBar.LeftInset/RightInset` | reader |
| W1-008 | `window.caption_button.fill @ {hover} / {pressed}` | rule |
| W1-008 | `window.caption_button.fill @ {hover} variant close / {pressed} variant close` | rule |
| W1-008 | `window.caption_button_glyph.glyph.fill @ {hover} / {pressed} / {hover} variant close / {pressed} variant close` | rule |
| W1-008 | `window.{caption_button.fill,caption_button_glyph.glyph.fill} @ cond window_inactive` | rule |
| W1-008 | `reader:windows:AppWindowTitleBar Button*Color getters (to verify)` | reader |
| W1-009 | `window.caption_button.transitions` | field |
| W1-010 | `window.title_icon.{icon,padding}` | field |
| W1-010 | `window.title.padding` | field |
| W1-010 | `window.values.title_alignment` | field |
| W1-011 | `window.subtitle.text` | field |
| W1-011 | `window.subtitle.text.fill @ cond window_inactive` | rule |
| W1-011 | `button.* @ variant back context window.title_bar / variant @pane_toggle context window.title_bar` | rule |
| W1-012 | `window.title_bar_content.opacity @ cond window_inactive` | rule |
| W1-013 | `window.title_bar.fill @ cond system_setting:accent_on_title_bars !cond window_inactive` | rule |
| W1-013 | `reader:windows:HKCU DWM ColorPrevalence` | reader |
| W1-014 | - | out (R1.1) |
| W1-015 | `menu.* @ context window.window_menu (the system menu's look)` | rule |
| W1-015 | `window.values.window_menu_items` | field |
| W1-016 | `window.values.resize_band (outside, invisible)` | field |
| W1-016 | `reader:windows:SM_CXFRAME, SM_CXPADDEDBORDER, DWMWA_EXTENDED_FRAME_BOUNDS` | reader |
| W1-017 | `cond animations_off (read from SPI_GETCLIENTAREAANIMATION)` | condition |
| W1-017 | - (the DWM's open, close, minimize, maximize and restore animations) | out (R1.1) |
| W1-148† | `window.values.caption_button_capabilities` | field |
| W1-148† | - (the context-help "?" caption button, WS_EX_CONTEXTHELP: Win32 only) | preset (R1.3) |
| W1-148† | `window.{title_bar.min_height,caption_button.min_width,caption_button.min_height} @ variant utility` | rule |
| W1-148† | `window.values.caption_button_layout @ dir rtl` | rule |
| W1-148† | `reader:windows:SM_CYSMCAPTION, SM_CXSMSIZE, SM_CYSMSIZE` | reader |
| W1-149† | `cond attention_flash (ADD-A03)` | condition |
| W1-149† | `window.{title_bar.fill,frame.stroke} @ cond attention_flash` | rule |
| W1-149† | `window.values.attention_flash` | field |
| W1-149† | `reader:windows:SPI_GETFOREGROUNDFLASHCOUNT, GetCaretBlinkTime` | reader |
| W1-018 | `button.body.stroke (paint: Linear gradient over an absolute extent)` | field |
| W1-018 | `button.body.stroke @ cond scheme:dark (gradient at the other edge)` | rule |
| W1-019 | `button.body.stroke @ {hover} / {pressed} / {disabled}` | rule |
| W1-020 | `button.body.fill, button.label.text.fill @ {hover} / {pressed} / {disabled} variant primary` | rule |
| W1-020 | `button.body.stroke (gradient paint) @ variant primary / {hover} variant primary / {pressed} variant primary / {disabled} variant primary` | rule |
| W1-021 | `button.body.fill, button.label.text.fill, button.body.stroke @ variant flat / {hover} variant flat / {pressed} variant flat / {disabled} variant flat` | rule |
| W1-022 | `button.body.fill, button.label.text.fill, button.body.stroke @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} variant toggle` | rule |
| W1-022 | `button.* @ {indeterminate} / {indeterminate,hover} / {indeterminate,pressed} / {indeterminate,disabled} variant toggle` | rule |
| W1-023 | `button.body.transitions (Fill)` | field |
| W1-023 | `token.motion.durations.control_faster` | token |
| W1-024 | `button.icon.transitions (PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | field |
| W1-025 | `button.body.focus.offset` | field |
| W1-026 | `input.body.stroke (gradient paint per edge)` | field |
| W1-026 | `input.body.stroke (bottom edge accent) @ {focused}` | rule |
| W1-027 | `input.body.stroke (bottom edge width) @ {focused}` | rule |
| W1-028 | `input.body.fill @ {hover} / {focused}` | rule |
| W1-028 | `input.text.text.fill @ {hover} / {focused}` | rule |
| W1-028 | `input.placeholder.text.fill @ {hover} / {focused} / {disabled}` | rule |
| W1-029 | `input.body.stroke @ {disabled}` | rule |
| W1-030 | `input.clear_button.{glyph,fill,stroke,padding}` | field |
| W1-030 | `input.clear_button.{fill,glyph.fill} @ {hover} / {pressed}` | rule |
| W1-030 | `input.values.clear_button_visibility; input.values.presentations (events Appear, Disappear, part clear_button: PresentationKind::Instant)` | field |
| W1-031 | `input.header.text; input.values.header_gap` | field |
| W1-031 | `input.header.text.fill @ {disabled}` | rule |
| W1-031 | `input.supporting_text.text` | field |
| W1-032 | `input.selection.fill @ !{focused}` | rule |
| W1-032 | `input.selection.text.fill` | field |
| W1-032 | `role.global.selection_text_background_unfocused` | role |
| W1-033 | `input.{body,text} @ {read_only} / {invalid}` | rule |
| W1-034 | `reader:windows:SPI_GETCARETWIDTH, GetCaretBlinkTime, UISettings.CaretWidth, UISettings.CaretBlinkRate -> token.caret.{width,blink_on_ms,blink_off_ms}` | reader |
| W1-034 | `input.selection_handle.{glyph,fill,min_width,min_height}` | field |
| W1-036 | `text_area.values.scrollbar_inset; text_area.body.padding` | field |
| W1-036 | `scrollbar.* @ context any text_area` | rule |
| W1-150† | `input.supporting_text.text; input.values.supporting_text_gap` | field |
| W1-150† | `combo_box.supporting_text.text` | field |
| W1-151† | `input.misspelling.text.decoration` | field |
| W1-151† | `input.preedit.{fill,text.fill,text.decoration} @ variant selected_clause (the target clause; the IME's other clause attributes, TF_DISPLAYATTRIBUTE, as keyed preedit variants)` | rule |
| W1-037 | `checkbox.box.fill, checkbox.box.stroke, checkbox.mark.glyph.fill, checkbox.label.text.fill @ {hover} / {pressed} / {disabled} / {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} / {indeterminate} / {indeterminate,hover} / {indeterminate,pressed} / {indeterminate,disabled}` | rule |
| W1-038 | `checkbox.mark.glyph @ {indeterminate} (FontCodePoint), checkbox.mark.glyph.size` | rule |
| W1-038 | `checkbox.box.fill, checkbox.box.stroke @ {indeterminate}` | rule |
| W1-039 | `checkbox.mark.glyph (FontCodePoint), checkbox.mark.glyph.size` | field |
| W1-039 | `checkbox.mark.transitions @ {checked} (PropertyGroup::Icon, TransitionKind::Keyed: the AnimatedIcon accept states; ADD-A27)` | rule |
| W1-040 | `checkbox.body.min_height, checkbox.body.min_width` | field |
| W1-041 | `radio.dot.min_width, radio.dot.min_height @ {hover} / {pressed}` | rule |
| W1-041 | `radio.dot.transitions (Geometry)` | field |
| W1-042 | `radio.ring.stroke, radio.ring.fill @ {hover} / {pressed} / {disabled} / {checked} / {checked,hover} / {checked,pressed} / {checked,disabled}` | rule |
| W1-042 | `radio.dot.fill, radio.dot.stroke @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled}` | rule |
| W1-043 | `radio.{body,ring,dot,label,state_layer} (the typed radio, stated apart from checkbox)` | field |
| W1-044 | `menu.item.fill @ {pressed}` | rule |
| W1-044 | `menu.item_label.text.fill @ {pressed}` | rule |
| W1-044 | `menu.item.fill @ {disabled}` | rule |
| W1-045 | `menu.item.corners` | field |
| W1-045 | `menu.body.padding (item margin)` | field |
| W1-046 | `menu.body.{padding,min_height,max_height,stroke}` | field |
| W1-046 | `menu.values.scrollable` | field |
| W1-047 | `menu.item.padding` | field |
| W1-047 | `menu.item.padding @ context menu.body {opened_by_touch}` | rule |
| W1-048 | `menu.accelerator.text @ {hover} / {pressed} / {disabled}` | rule |
| W1-048 | `menu.values.accelerator_alignment` | field |
| W1-049 | `menu.submenu_arrow.glyph @ {hover} / {pressed} / {open} / {disabled}` | rule |
| W1-049 | `menu.{item.fill,item_label.text.fill} @ {open}` | rule |
| W1-049 | `menu.submenu_arrow.padding (chevron margin)` | field |
| W1-050 | `menu.check_indicator.glyph @ {checked} / variant radio` | rule |
| W1-050 | `menu.values.{check_column,icon_column} (single and double placeholder)` | field |
| W1-051 | `menu.{split_button,split_divider}` | field |
| W1-051 | `menu.item.* @ variant split` | rule |
| W1-052 | `menu.separator.{min_height,padding,fill}` | field |
| W1-053 | `menu.body.fill (acrylic material)` | field |
| W1-053 | `menu.body.shadow (Elevation 32)` | field |
| W1-054 | `menu.values.presentations (slide from TowardAnchor with fade, ADD-B10, ADD-A25)` | field |
| W1-054 | `menu.values.submenu_open_delay_ms` | field |
| W1-054 | `reader:windows:SPI_GETMENUSHOWDELAY, SPI_GETMENUANIMATION` | reader |
| W1-055 | `menu.dismiss_overlay.fill` | field |
| W1-056 | `tooltip.body.fill (acrylic in-app material)` | field |
| W1-057 | `tooltip.body.shadow (Elevation 16)` | field |
| W1-058 | `tooltip.values.presentations` | field |
| W1-058 | `reader:windows:SPI_GETTOOLTIPANIMATION (system_setting tooltip_fade)` | reader |
| W1-059 | `tooltip.values.{reshow_delay_ms,keyboard_show_delay_ms,display_time}` | field |
| W1-059 | `tooltip.values.show_delay_ms @ cond input:touch` | rule |
| W1-059 | `tooltip.values.placement (WinUiToolTip)` | placement |
| W1-059 | `reader:windows:SPI_GETMOUSEHOVERTIME` | reader |
| W1-060 | `scrollbar.thumb.{fill,min_width,min_height} @ cond overlay_scrollbars !{hover} (the panning indicator; the expanded bar under {hover})` | rule |
| W1-060 | `scrollbar.body.{min_width,min_height}; scrollbar.thumb.stroke` | field |
| W1-061 | `scrollbar.arrow_button.{glyph,fill,stroke,padding}` | field |
| W1-061 | `scrollbar.arrow_button.{fill,glyph.fill} @ {hover} / {pressed} / {disabled}` | rule |
| W1-061 | `scrollbar.arrow_button.transform @ {pressed}` | rule |
| W1-062 | `scrollbar.track.{fill,stroke} (fill a material)` | field |
| W1-062 | `scrollbar.track.{fill,stroke} @ {hover} / {disabled}` | rule |
| W1-063 | `scrollbar.thumb.corners` | field |
| W1-063 | `scrollbar.thumb.fill @ {disabled}` | rule |
| W1-064 | `scrollbar.{thumb,track,body}.transitions` | field |
| W1-064 | `scrollbar.values.auto_hide_delay` | field |
| W1-065 | `scroll_view.corner.{fill,transitions}` | field |
| W1-065 | `scroll_view.values.scrollbar_margin` | field |
| W1-066 | `reader:windows:UISettings.ScrollBarSize, ScrollBarArrowSize, ScrollBarThumbBoxSize -> scrollbar.body, scrollbar.arrow_button, scrollbar.thumb sizes` | reader |
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
| W1-072 | `slider.header.text; slider.values.header_gap; slider.body.{min_height,padding}` | field |
| W1-072 | `slider.header.text.fill @ {disabled}` | rule |
| W1-073 | `slider.value_indicator` | field |
| W1-073 | `slider.values.value_indicator_placement` | placement |
| W1-074 | `slider.body.fill` | field |
| W1-074 | `slider.body.fill @ {hover}` | rule |
| W1-153† | `slider.* @ variant range` | rule |
| W1-075 | `progress_bar.values.loop` | field |
| W1-075 | `progress_bar.secondary_indicator` | field |
| W1-076 | `progress_bar.indicator.fill @ {@paused} / {@error} / {@paused,indeterminate} / {@error,indeterminate}` | rule |
| W1-076 | `progress_bar flags @paused, @error` | flag |
| W1-077 | `progress_bar.track.{min_height,corners}; progress_bar.indicator.min_height` | field |
| W1-078 | `progress_bar.indicator.transitions (Geometry: the value change)` | field |
| W1-079 | `progress_bar.track.stroke @ cond contrast_high` | rule |
| W1-080 | `tab.tab.fill, tab.label.text.fill @ {pressed} / {disabled} / {selected}` | rule |
| W1-081 | `tab.icon.{icon,padding}` | field |
| W1-081 | `tab.icon.icon.fill @ {hover} / {pressed} / {selected} / {disabled}` | rule |
| W1-082 | `tab.close_button.{min_width,min_height,max_width,max_height,glyph,padding}` | field |
| W1-082 | `tab.close_button.fill, tab.close_button.glyph.fill, tab.close_button.stroke @ {hover} / {pressed} / {disabled} / context tab.tab {selected} / context tab.tab {hover} / context tab.tab {pressed}` | rule |
| W1-082 | `tab.values.close_button_shown` | field |
| W1-082 | `tab.tab.padding @ variant closable` | rule |
| W1-083 | `tab.{add_button,scroll_button}.{min_width,min_height,max_width,max_height,glyph,fill}` | field |
| W1-083 | `tab.add_button.fill, tab.scroll_button.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-084 | `tab.separator.stroke, tab.values.separator_inset` | field |
| W1-084 | `tab.separator.* @ prev tab.tab {selected} / next tab.tab {selected} (ADD-B07)` | rule |
| W1-085 | `tab.tab.max_width, tab.values.width_mode` | field |
| W1-085 | `tab.label.text.font.size` | field |
| W1-085 | `tab.bar.padding` | field |
| W1-086 | `tab.tab.stroke @ {selected} (linear gradient paint, sides and top)` | rule |
| W1-087 | `tab.tab.fill @ {dragged}` | rule |
| W1-087 | `tab.values.presentations (events Move, Appear, Disappear, @drag)` | field |
| W1-088 | `tab.bar.fill (material mica_alt)` | field |
| W1-088 | `tab.pane.fill, tab.tab.fill @ {selected}` | rule |
| W1-089 | `list.row_selection_indicator.{fill,corners,min_width,min_height,max_height,padding} @ variant sidebar (padding: its place at the row's leading edge)` | rule |
| W1-089 | `list.row_selection_indicator.fill @ {hover} / {pressed} / {disabled} variant sidebar` | rule |
| W1-090 | `list.values.presentations (event Move, part row_selection_indicator, PresentationKind::Morph) @ variant sidebar` | rule |
| W1-091 | `list.row_background.fill, list.row_background.stroke, list.row.text.fill @ {pressed} / {selected,hover} / {selected,pressed} / {disabled} / {checked} variant sidebar` | rule |
| W1-092 | `list.row.min_height, list.row_background.corners, list.values.row_background_inset, list.leading_icon.{min_width,min_height,max_width,max_height,padding}, list.values.leading_gap @ variant sidebar` | rule |
| W1-093 | `list.section_header.text @ variant sidebar` | rule |
| W1-093 | `separator.line.stroke, separator.values.inset @ context sidebar.pane` | rule |
| W1-093 | `list.disclosure.{glyph,padding} @ variant sidebar` | rule |
| W1-093 | `list.disclosure.transform @ {open} variant sidebar` | rule |
| W1-093 | `list.disclosure.transitions (Transform)` | field |
| W1-094 | `sidebar.values.width, sidebar.values.collapsed_width` | field |
| W1-094 | `sidebar.* @ variant compact / variant @left_minimal (LeftCompact, LeftMinimal; Left is the base)` | rule |
| W1-094 | `sidebar.values.display_mode_breakpoints` | field |
| W1-094 | `view_switcher.* @ variant @navigation_top (Top)` | rule |
| W1-095 | `sidebar.toggle_button.{min_width,min_height,glyph} (the pane toggle and back buttons)` | field |
| W1-095 | `sidebar.header.min_height (the AutoSuggestBox area)` | field |
| W1-095 | `list.row.* @ variant sidebar pos last (footer items, the settings item)` | rule |
| W1-096 | `sidebar.pane.fill (material acrylic), sidebar.pane.shadow (elevation), sidebar.pane.corners @ variant overlay` | rule |
| W1-097 | `sidebar.content.{fill,stroke,corners}` | field |
| W1-098 | `sidebar.values.presentations (events @open, @close)` | field |
| W1-099 | `button.body.{min_width,min_height,padding,corners} @ variant tool context toolbar.bar` | rule |
| W1-099 | `button.{body.fill,label.text.fill,icon.icon.fill} @ {hover} / {pressed} / {disabled} variant tool context toolbar.bar` | rule |
| W1-099 | `button.values.label_position @ variant tool context toolbar.bar (TextUnderIcon, TextBesideIcon, IconOnly)` | rule |
| W1-099 | `button.{label.text,label.padding,icon.padding} @ variant tool context toolbar.bar` | rule |
| W1-100 | `button.{body.fill,label.text.fill} @ {checked} / {checked,hover} / {checked,pressed} / {checked,disabled} variants {tool,toggle} context toolbar.bar` | rule |
| W1-100 | `menu.check_indicator.glyph @ context toolbar.overflow_button` | rule |
| W1-101 | `toolbar.separator.{stroke,corners,min_width,padding}` | field |
| W1-101 | `separator.values.inset @ context toolbar.bar` | rule |
| W1-102 | `toolbar.overflow_button.{glyph,fill,min_width,min_height}` | field |
| W1-102 | `menu.body.{fill,stroke,padding,min_width,max_width,max_height,shadow} @ context toolbar.overflow_button` | rule |
| W1-102 | `toolbar.values.overflow_placement (WinUiFlyout)` | placement |
| W1-102 | `menu.{accelerator.text.fill,submenu_arrow.glyph.fill} @ context toolbar.overflow_button` | rule |
| W1-103 | `toolbar.bar.{fill,stroke,corners} @ {open}` | rule |
| W1-103 | `toolbar.bar.min_height @ variant @compact / variant @minimal / variant @hidden` | rule |
| W1-104 | `toolbar.values.presentations (events @open, @close)` | field |
| W1-105 | - | preset (R1.3) |
| W1-106 | `list.row_background.fill, list.row.text.fill @ {pressed} / {selected,hover} / {selected,pressed} / {selected,disabled}` | rule |
| W1-107 | `list.row_selection_indicator.{fill,corners,min_width,min_height,max_height}` | field |
| W1-107 | `list.row_selection_indicator.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-107 | `list.row_selection_indicator.transitions` | field |
| W1-108 | `list.row_background.corners, list.values.row_background_inset` | field |
| W1-109 | `list.selection_check.{fill,stroke,corners,glyph}` | field |
| W1-109 | `list.selection_check.fill @ {selected}` | rule |
| W1-110 | `list.row.opacity, list.row.transform @ {dragged} / {@reorder_target}` | rule |
| W1-110 | `list.row_background.fill, list.row.text.fill @ {dragged}` | rule |
| W1-110 | `list.values.reorder_hint_offset` | field |
| W1-110 | `drag_image.image.opacity, drag_image.values.hotspot @ context list` | rule |
| W1-110 | `badge.* @ context list.row {dragged}` | rule |
| W1-111 | `list.row_background.focus (FocusIndicator.secondary: the two-colour ring, ADD-A17)` | field |
| W1-112 | `list.row.opacity @ {disabled}` | rule |
| W1-113 | `list.values.presentations (events Appear, Disappear, Move)` | field |
| W1-114 | `popover.body.fill (acrylic material: tint, luminosity, blur, noise)` | field |
| W1-115 | `popover.body.shadow (Elevation 32)` | field |
| W1-116 | `popover.values.presentations (slide from TowardAnchor with fade, ADD-B10, ADD-A25)` | field |
| W1-116 | `popover.values.placement (WinUiFlyout; its gap)` | placement |
| W1-116 | `popover.dismiss_overlay.fill` | field |
| W1-117 | `popover.body.{min_width,max_width,min_height,max_height}` | field |
| W1-119 | `separator.values.inset @ variant horizontal context menu / variant vertical context toolbar.bar / variant horizontal context sidebar.pane` | rule |
| W1-157† | `splitter.divider.fill @ {hover} / {pressed} / {disabled}` | rule |
| W1-157† | `splitter.grip.{fill,glyph,min_width,min_height,corners,padding}` | field |
| W1-158† | - | preset (R1.3) |
| W1-159† | - | preset (R1.3) |
| W1-120 | `switch.track.fill @ {checked,pressed} / {pressed} !{checked}` | rule |
| W1-120 | `switch.thumb.fill @ {pressed} / {checked,pressed}; switch.thumb.min_width @ {pressed}` | rule |
| W1-121 | `switch.thumb.fill @ {hover} / {checked,hover}` | rule |
| W1-122 | `switch.thumb.min_width, switch.thumb.min_height @ {hover}` | rule |
| W1-123 | `switch.thumb.transitions (Geometry)` | field |
| W1-123 | `switch.values.presentations (event Move, part thumb; measured)` | field |
| W1-123 | `token.motion.durations.control_faster, token.motion.timings.control_fast_out_slow_in` | token |
| W1-124 | `switch.state_label.text` | field |
| W1-124 | `switch.state_label.text.fill @ {disabled}` | rule |
| W1-124 | `switch.header.text` | field |
| W1-124 | `switch.body.min_width, switch.values.pre_content_margin, switch.values.post_content_margin` | field |
| W1-125 | `dialog.scrim.fill (= role.global.scrim)` | field |
| W1-126 | `dialog.body.fill (content zone)` | field |
| W1-126 | `dialog.action_area.{fill,stroke}` | field |
| W1-127 | `dialog.values.title_message_gap_px` | field |
| W1-127 | `dialog.action.{min_width,max_width,min_height}` | field |
| W1-127 | `dialog.values.action_layout.equal_widths` | field |
| W1-128 | `dialog.values.presentations (events Appear, Disappear: scale with fade, ADD-B10)` | field |
| W1-129 | `dialog.body.shadow (Elevation 128)` | field |
| W1-130 | `spinner.{indicator,track}.* @ variant determinate` | rule |
| W1-130 | `spinner.track.fill` | field |
| W1-130 | `spinner.values.loop` | field |
| W1-131 | `combo_box.body.fill @ {pressed} / {focused}` | rule |
| W1-131 | `combo_box.{label,placeholder}.text.fill @ {focused}` | rule |
| W1-131 | `combo_box.body.stroke (gradient) @ {hover} / {pressed}` | rule |
| W1-132 | `combo_box.arrow.glyph.fill @ {disabled} / {focused} / {focused,pressed}` | rule |
| W1-132 | `combo_box.arrow.transform @ {pressed}; combo_box.arrow.transitions` | rule |
| W1-133 | `combo_box.popup.{fill,stroke,padding,corners,shadow,min_width}` | field |
| W1-133 | `combo_box.values.max_visible_items` | field |
| W1-134 | `combo_box.item.{fill,text,stroke} @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {selected,disabled} / {disabled}` | rule |
| W1-134 | `combo_box.item.fill @ {selected} context combo_box.popup !{focus_within}` | rule |
| W1-134 | `combo_box.item.padding @ context combo_box.popup {opened_by_touch} / cond input:game_controller` | rule |
| W1-134 | `combo_box.item.corners` | field |
| W1-135 | `combo_box.item_pill.{fill,min_width,min_height,corners}` | field |
| W1-135 | `combo_box.item_pill.transitions (Transform: the scale)` | field |
| W1-136 | `combo_box.values.presentations (ClipReveal from the selected item)` | field |
| W1-136 | `combo_box.values.popup_placement (WinUiComboBox, ADD-A21, ADD-A22)` | placement |
| W1-137 | `combo_box.* @ variant editable` | rule |
| W1-137 | `combo_box.header.text; combo_box.values.header_gap` | field |
| W1-138 | `segmented_control.segment.fill @ {pressed} / {checked,hover}` | rule |
| W1-138 | `segmented_control.indicator.stroke, segmented_control.indicator.shadow` | field |
| W1-138 | `segmented_control.values.presentations (event Move, part indicator)` | field |
| W1-139 | `card.body.fill @ variant @secondary / variant @tertiary` | rule |
| W1-139 | `form.card.fill @ {hover} / {pressed} (the clickable SettingsCard)` | rule |
| W1-139 | `card.body.shadow (elevation entry, measured)` | field |
| W1-140 | `expander.header.text, expander.header.stroke @ {hover} / {pressed} / {disabled}` | rule |
| W1-140 | `expander.arrow.fill, expander.arrow.glyph.fill, expander.arrow.stroke @ {hover} / {pressed}` | rule |
| W1-141 | `expander.content.fill, expander.content.stroke, expander.content.corners` | field |
| W1-141 | `expander.content.* @ variant @expand_up (opens above the header)` | rule |
| W1-141 | `expander.content.stroke, expander.content.corners @ variant @expand_up` | rule |
| W1-142 | `expander.values.presentations (events @expand, @collapse)` | field |
| W1-142 | `expander.arrow.transitions (Transform)` | field |
| W1-143 | `link.text.text.decoration @ variant inline / {hover} variant inline / {pressed} variant inline` | rule |
| W1-144 | `link.body.fill @ {pressed} / {disabled}` | rule |
| W1-144 | `link.body.stroke @ {hover} / {pressed} / {disabled}` | rule |
| W1-145 | `link.text.focus @ variant inline` | rule |
| W1-146 | `layout.values.button_gap, label_gap, header_gap, section_gap, surface_text_margin` | field |
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
| W1-154† | `segmented_control.* @ variant @pivot / variant @button` | rule |
| W1-154† | `segmented_control.segment.* @ variant icon_only / variant with_icon (item layouts)` | rule |
| W1-155† | `dialog.body.{max_height,min_height} @ variant full_size` | rule |
| W1-156† | - | preset (R1.3) |
| W2-01 | `stepper.{increment,decrement}.* @ {hover} variant @inline / {pressed} variant @inline / {disabled} variant @inline / {hover} variant @compact / {pressed} variant @compact / {disabled} variant @compact` | rule |
| W2-01 | `stepper.popup.{fill,stroke,shadow}, stepper.values.{popup_offset_x,popup_offset_y,invalid_input}, stepper.header.text` | field |
| W2-02 | `calendar.{day,month,year}.{fill,stroke}, calendar.day_label.text @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {selected,disabled} / {today} / {today,selected} / {@blackout} / {other_month} / {disabled}` | rule |
| W2-02 | `calendar.{weekday_label,title,nav_button}.* @ {hover} / {pressed}` | rule |
| W2-02 | `calendar.day_label.text, calendar.day.corners, calendar.values.presentations (event @view_switch)` | field |
| W2-03 | `date_picker.glyph.fill @ {hover} / {pressed} / {disabled}; date_picker.field_label.text.fill @ !{empty}; date_picker.field.fill @ {focused}` | rule |
| W2-03 | `date_picker.header.text, date_picker.popup.shadow` | field |
| W2-04 | `time_picker.field, time_picker.field_segment, time_picker.segment_separator, time_picker.header @ {hover} / {pressed} / {focused} / {disabled}; time_picker.values.field_min_width, time_picker.values.field_max_width` | rule |
| W2-04 | `time_picker.{flyout,highlight_band,accept_button,dismiss_button}, time_picker.values.item_height, time_picker.values.presentations (event @open)` | field |
| W2-04 | `picker_wheel.* @ context any time_picker / context any date_picker` | rule |
| W2-04 | `date_picker.{field,segment,segment_separator}.* @ variant text_field` | rule |
| W2-05 | `color_picker.plane.* @ variant @box / variant @ring` | rule |
| W2-05 | `color_picker.{crosshair,preview,checkerboard}` | field |
| W2-05 | `slider.* @ variant hue context any color_picker / variant alpha context any color_picker / variant value context any color_picker` | rule |
| W2-05 | `input.* @ context any color_picker (the hex and channel inputs)` | rule |
| W2-05 | `button.* @ context any color_picker (the more and fewer button)` | rule |
| W2-06 | `rating.star.glyph, rating.values.item_gap, rating.caption.text` | field |
| W2-06 | `rating.star.glyph.fill @ {filled} / !{filled} / {placeholder} / {preview} / {disabled}` | rule |
| W2-06 | `rating.star.transform @ {preview} (the hover preview scale, to verify)` | rule |
| W2-07 | `banner.body.fill, banner.icon_backplate.fill, banner.icon.glyph @ variant info / variant success / variant warning / variant error` | rule |
| W2-07 | `banner.title.text, banner.message.text, banner.close_button, banner.body.stroke, banner.body.min_height, banner.body.padding, banner.values.actions_wrap; link.text.fill @ context any banner` | field |
| W2-08 | `badge.{body,label,icon}.* @ variant dot / variant count / variant icon` | rule |
| W2-08 | `badge.body.fill @ variant attention / variant info / variant success / variant warning / variant error` | rule |
| W2-09 | `teaching_tip.{tail,top_highlight,hero}, teaching_tip.body.{stroke,shadow}, teaching_tip.values.{min_width,max_width,tail_size,tail_margin,light_dismiss}` | field |
| W2-09 | `teaching_tip.{title,message}.text.fill (the title and the subtitle)` | field |
| W2-09 | `teaching_tip.close_button.* @ variant @alternate_close` | rule |
| W2-09 | `teaching_tip.values.presentations (events @expand, @contract)` | field |
| W2-09 | `teaching_tip.values.placement (WinUiTeachingTip)` | placement |
| W2-10 | `breadcrumb.crumb_label.text, breadcrumb.separator.glyph, breadcrumb.values.separator_padding` | field |
| W2-10 | `breadcrumb.crumb_label.text.fill @ {current} / {hover} / {pressed} / {disabled} / {focus_visible}; breadcrumb.{ellipsis,ellipsis_popup}.* @ {hover} / {pressed}` | rule |
| W2-11 | `view_switcher.{bar.fill,bar.min_height,indicator.fill,item_separator.stroke,overflow_button.min_width} @ variant @navigation_top` | rule |
| W2-11 | `view_switcher.{item.fill,item_label.text.fill} @ {hover} / {pressed} / {selected} / {disabled} variant @navigation_top` | rule |
| W2-12 | `list.row_background.fill @ {hover}` | rule |
| W2-12 | `list.row_background.fill @ {selected}` | rule |
| W2-12 | `list.row_background.fill @ {selected,hover}` | rule |
| W2-12 | `list.row_background.fill @ {selected,pressed}` | rule |
| W2-12 | `list.row.text.fill @ {disabled}` | rule |
| W2-12 | `list.values.indent_per_level` | field |
| W2-12 | `list.disclosure.glyph` | field |
| W2-12 | `list.disclosure.transform @ {open}` | rule |
| W2-12 | `list.row_selection_indicator.fill` | field |
| W2-12 | `list.selection_check` | field |
| W2-12 | `list.row.min_height @ density compact` | rule |
| W2-13 | `list.row_background.stroke @ {selected} variant grid` | rule |
| W2-13 | `list.row_background.stroke @ {selected,hover} variant grid` | rule |
| W2-13 | `list.row_inner_border.stroke @ {selected} variant grid` | rule |
| W2-13 | `list.selection_check.* @ variant grid` | rule |
| W2-13 | `list.row_background.corners @ variant grid` | rule |
| W2-13 | `list.row_background.opacity @ {dragged} variant grid` | rule |
| W2-14 | `list.row_selection_indicator.fill @ {hover}` | rule |
| W2-14 | `list.row_selection_indicator.fill @ {pressed}` | rule |
| W2-14 | `list.row_inner_border.stroke @ {selected}` | rule |
| W2-14 | `list.values.check_box_placement` | field |
| W2-14 | `list.row.opacity @ {disabled}` | rule |
| W2-15 | `search_field.query_button.{padding,glyph}; search_field.clear_button.* @ {hover} / {pressed}` | rule |
| W2-15 | `autocomplete.popup.{fill,stroke,shadow,max_height}; autocomplete.item.* @ {hover} / {pressed} / {selected}` | rule |
| W2-15 | `autocomplete.popup.corners @ {placed_above} context any search_field` | rule |
| W2-15 | `autocomplete.values.placement (WinUiAutoSuggest)` | placement |
| W2-16 | `input.reveal_button.* @ variant secure / {hover} variant secure / {pressed} variant secure` | rule |
| W2-16 | `input.values.{mask_character,peek_while_pressed}` | field |
| W2-17 | `split_button.{primary,secondary,divider}.* @ {hover} / {pressed} / {disabled} / {checked} variant toggle` | rule |
| W2-18 | `button.menu_indicator.glyph, button.values.menu_indicator_gap @ variant menu; button.menu_indicator.glyph.fill @ {hover} variant menu / {pressed} variant menu; button.menu_indicator.transitions @ {pressed}` | rule |
| W2-19 | `button.body.fill @ variant @repeat` | rule |
| W2-19 | `button.values.repeat_delay_ms` | field |
| W2-19 | `button.values.repeat_interval_ms` | field |
| W2-19 | `reader:windows:SPI_GETKEYBOARDDELAY/SPI_GETKEYBOARDSPEED relation to RepeatButton.Delay/Interval` | reader |
| W2-20 | `page_indicator.dot.* @ {selected} / {hover} / {pressed} / {disabled}; page_indicator.nav_button.* @ {hover} / {pressed}; page_indicator.* @ variant vertical` | rule |
| W2-20 | `page_indicator.values.{nav_buttons_visibility,max_visible}` | field |
| W2-21 | `page_indicator.{page_button,selection_indicator,nav_button}.* @ {hover} / {pressed} variant @button_panel / variant @combo_box / variant @number_box` | rule |
| W2-22 | `menu_bar.bar.fill, menu_bar.values.height, menu_bar.item.padding; menu_bar.item.{fill,stroke} @ {hover} / {pressed} / {open}` | rule |
| W2-23 | `edit_menu.primary_commands.fill` | field |
| W2-23 | `edit_menu.primary_commands.corners` | field |
| W2-23 | `edit_menu.primary_commands.stroke` | field |
| W2-23 | `edit_menu.primary_commands.fill @ {checked}` | rule |
| W2-23 | `edit_menu.secondary_menu` | field |
| W2-23 | `edit_menu.expand_button` | field |
| W2-23 | `edit_menu.primary_commands.shadow` | field |
| W2-23 | `edit_menu.values.presentations (event Appear)` | field |
| W2-23 | `edit_menu.values.presentations (event @expand)` | field |
| W2-24 | `avatar.body.fill, avatar.body.stroke, avatar.initials.text, avatar.fallback_icon.glyph, avatar.values.default_size` | field |
| W2-24 | `badge.{body,label}.*, badge.values.{anchor_offset_x,anchor_offset_y} @ context any avatar` | rule |
| W2-25 | `radio_group.header.text.fill` | field |
| W2-25 | `radio_group.header.text.fill @ {disabled}` | rule |
| W2-25 | `radio_group.header.padding` | field |
| W2-25 | `radio_group.values.row_gap` | field |
| W2-25 | `radio_group.values.column_gap` | field |
| W2-26 | `pull_to_refresh.{indicator,arrow}, pull_to_refresh.values.{trigger_distance,loop}` | field |
| W2-27 | `view_switcher.{item,item_icon,item_label,indicator}.* @ {hover} / {pressed} / {selected} / {disabled} variant @selector_bar` | rule |
| W2-27 | `view_switcher.values.presentations (event Move, part indicator), view_switcher.values.item_gap` | field |
| W2-28 | `scrollbar.annotation_labels.text` | field |
| W2-28 | `scrollbar.thumb.fill @ variant @annotated` | rule |
| W2-28 | `scrollbar.thumb.corners @ variant @annotated` | rule |
| W2-28 | `scrollbar.thumb.min_height @ variant @annotated` | rule |
| W2-28 | `scrollbar.annotation_tooltip.min_height` | field |
| W2-28 | `scrollbar.annotation_tooltip.max_width` | field |
| W2-28 | `scrollbar.arrow_button.fill @ variant @annotated` | rule |
| W2-29 | `list.swipe_action.fill` | field |
| W2-29 | `list.swipe_action.fill @ {pressed}` | rule |
| W2-29 | `list.swipe_action.fill @ {@pre_threshold}` | rule |
| W2-29 | `list.swipe_action.fill @ {@post_threshold}` | rule |
| W2-29 | `list.swipe_action.text.fill` | field |
| W2-29 | `list.values.presentations (event @reveal)` | field |
| W2-30 | `sidebar.values.{width,collapsed_width}, sidebar.pane.{stroke,corners}` | field |
| W2-30 | `sidebar.scrim.fill @ variant overlay` | rule |
| W2-30 | `sidebar.values.presentations (events @open, @close)` | field |
| W2-31 | `carousel.{nav_button,nav_glyph}.* @ {hover} / {pressed}; carousel.nav_glyph.transform @ {pressed}` | rule |
| W2-31 | `carousel.values.nav_buttons_visibility, carousel.item.fill` | field |
| W2-32 | `view_switcher.item_label.text @ {selected} / !{selected} / {hover} / {pressed} / {disabled} variant @pivot` | rule |
| W2-32 | `view_switcher.nav_button.*, view_switcher.values.presentations (event Move, part indicator) @ variant @pivot` | rule |
| W2-33 | `list.row.padding @ variant @listbox` | rule |
| W2-33 | `list.row_background.fill @ {selected,hover} variant @listbox` | rule |
| W2-33 | `list.row_background.fill @ {selected,pressed} variant @listbox` | rule |
| W2-33 | `list.row.text.fill @ {disabled} variant @listbox` | rule |
| W2-33 | `list.view.stroke @ variant @listbox` | rule |
| W2-34 | `component.semantic_zoom.zoom_out_button.fill` | component |
| W2-34 | `component.semantic_zoom.zoom_out_button.fill @ {hover}` | component |
| W2-34 | `component.semantic_zoom.zoom_out_button.fill @ {pressed}` | component |
| W2-34 | `component.semantic_zoom.zoom_out_button.stroke` | component |
| W2-34 | `component.semantic_zoom.zoom_out_glyph.glyph` | component |
| W2-34 | `component.semantic_zoom.zoom_out_glyph.text` | component |
| W2-34 | `component.semantic_zoom.values.presentations (event @zoom)` | component |
| W2-35 | `media_controls.{bar,button,time_label,volume_popup}, media_controls.values.auto_hide_ms, media_controls.values.presentations (event @hide)` | field |
| W2-35 | `slider.* @ context any media_controls` | rule |
| W2-36 | `ink_toolbar.tool_button, ink_toolbar.palette, ink_toolbar.palette_swatch, ink_toolbar.stroke_width_selector, ink_toolbar.ruler` | field |
| W2-37 | `component.key_tip.badge.fill` | component |
| W2-37 | `component.key_tip.badge.stroke` | component |
| W2-37 | `component.key_tip.badge.padding` | component |
| W2-37 | `component.key_tip.text.text` | component |
| W2-37 | `component.key_tip.values.placement (WinUiKeyTip)` | placement |
| W2-40† | `form.card.{fill,stroke}, form.row_title.text.fill @ {hover} / {pressed} / {disabled}` | rule |
| W2-40† | `form.card.{padding,min_width,min_height}, form.row_subtitle.text, form.row_icon, form.row_arrow` | field |
| W2-40† | `form.* @ variant @right / variant @right_wrapped / variant @right_wrapped_no_icon / variant @left / variant vertical` | rule |
| W2-40† | `form.values.collapse` | field |
| W2-40† | `form.row_icon.opacity @ {disabled}; form.row.* @ variant expander` | rule |
| W2-41† | `level_indicator.{scale_arc,trail_arc,tick,scale_tick,needle,value_label,unit_label}.* @ variant circular / {disabled} variant circular` | rule |
| W2-41† | `level_indicator.values.{start_angle_deg,end_angle_deg,thickness}` | field |
| W2-42† | `color_button.chevron.glyph.fill @ {hover} / {pressed}` | rule |
| W2-42† | `color_button.swatch.corners, color_button.values.{swatch_min_width,swatch_min_height}, color_button.body.padding, color_button.checkerboard (Pattern paint, ADD-B01)` | field |
| W2-43† | `chip.body.{fill,stroke}, chip.label.text.fill @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed}; chip.remove_button.* @ {selected}` | rule |
| W2-43† | `chip.body.{padding,corners}, chip.remove_button.{glyph,corners}` | field |
| W2-43† | `layout.values.{item_gap,line_gap} @ variant flow context any input (the token spacing)` | rule |
| W2-43† | `input.body.padding (the box's padding)` | field |
| W3-01 | `token.elevation.<WinUI elevation steps> (ShadowLayer ambient layer)` | token |
| W3-01 | `token.elevation @ cond scheme:dark (intensity)` | rule |
| W3-02 | `PartStyle.shadow Elevation(key) @ {pressed} per control` | rule |
| W3-03 | `PartStyle.shadow Elevation(key) base per part` | field |
| W3-04 | `PartStyle.stroke.{top,bottom} paint Linear (the elevation stroke gradient)` | field |
| W3-05 | `PartStyle.shadow @ cond contrast_high (none)` | rule |
| W3-05 | `PartStyle.fill Material @ cond contrast_high / cond reduced_transparency (opaque)` | rule |
| W3-06 | `PartStyle.fill Material identities acrylic_background, acrylic_in_app (tint, luminosity, blur, noise, exclusion params ADD-A47) per scheme` | field |
| W3-06 | `declare.materials (acrylic_background, acrylic_in_app)` | field |
| W3-07 | `PartStyle.fill @ cond reduced_transparency / cond battery_saver / cond contrast_high / cond system_setting:material_policy / cond window_inactive (Material.fallback; window_inactive for background acrylic)` | rule |
| W3-07 | `cond system_setting:material_policy (the material policy AcrylicBrush follows, low-end hardware among its causes; ADD-A04)` | condition |
| W3-07 | `reader:windows:the material policy state (AcrylicBrush's OnMaterialPolicyStatusChanged; its inputs to verify)` | reader |
| W3-07 | `PartStyle.transitions (crossfade, tint transition)` | field |
| W3-07 | `cond battery_saver` | condition |
| W3-08 | `Material.tint = role.global.accent derivation (accent-tinted variants)` | field |
| W3-09 | `PartStyle.fill.layers over Material (two-layer system)` | field |
| W3-10 | `PartStyle.fill Material mica @ cond window_inactive to Material.fallback` | rule |
| W3-10 | `PartStyle.transitions (material to colour crossfade)` | field |
| W3-11 | `role.global.scrim` | role |
| W3-11 | `dialog.scrim.fill` | field |
| W3-12 | `role.global.accent_ramp (Light1-3, Dark1-3)` | role |
| W3-12 | `input.accent_ramp` | input |
| W3-12 | `reader:windows:UISettings.GetColorValue AccentDark1-3, AccentLight1-3` | reader |
| W3-13 | `role.@text_tertiary` | role |
| W3-14 | `role.@text_on_accent_secondary, role.@text_on_accent_disabled, role.@text_on_accent_selected` | role |
| W3-15 | `role.@stroke_control, role.@stroke_card, role.@stroke_surface, role.@stroke_divider` | role |
| W3-16 | `role.@fill_* tiers` | role |
| W3-16 | `role.@fill_subtle` | role |
| W3-16 | `role.@background_solid_*` | role |
| W3-17 | `role.global.status.{info,success,warning,error}.fill (attention, success, caution, critical)` | role |
| W3-18 | `role.@status_neutral_fill` | role |
| W3-45† | `Color.a on fills (S2.1, R5.3)` | field |
| W3-45† | `PartStyle.fill alpha rules for rest, hover, pressed` | field |
| W3-46† | `role.@text_inverse (status icon backplates)` | role |
| W3-19 | `FocusIndicator.secondary (inner ring colour and width, ADD-A17)` | field |
| W3-20 | `FocusIndicator.offset and corners per part; FocusIndicator.kind per control` | field |
| W3-21 | `token.behaviour.focus_visible_when (Keyboard { … }; ADD-A16)` | token |
| W3-21 | `reader:windows:SPI_GETKEYBOARDCUES` | reader |
| W3-22 | `token.text.rendering (ADD-B04)` | token |
| W3-22 | `reader:windows:SPI_GETCLEARTYPE, SPI_GETFONTSMOOTHING, SPI_GETFONTSMOOTHINGTYPE, SPI_GETFONTSMOOTHINGCONTRAST` | reader |
| W3-23 | `Font.axes (opsz: AxisValue::FollowsSize)` | field |
| W3-23 | `token.text.roles.*.{line_height,letter_spacing}` | token |
| W3-24 | `token.caret.width` | token |
| W3-24 | `token.caret.{blink_on_ms,blink_off_ms,blink_timeout_ms}` | token |
| W3-24 | `reader:windows:SPI_GETCARETWIDTH, GetCaretBlinkTime, UISettings.CaretWidth, UISettings.CaretBlinkRate, SPI_GETCARETTIMEOUT` | reader |
| W3-24 | - (the text cursor indicator: drawn by the system) | out (R1.1) |
| W3-25 | `role.global.selection_text_background_unfocused; role.global.selection_text_foreground (a reference to the text's own colour: WinUI keeps it)` | role |
| W3-25 | `input.selection.fill @ !{focused}` | rule |
| W3-25 | `input.selection_handle (grippers)` | field |
| W3-26 | `menu.accelerator.text` | field |
| W3-26 | `tooltip.accelerator.text` | field |
| W3-27 | `PartStyle.glyph.size per part` | field |
| W3-28 | `combo_box.arrow.transitions (PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | field |
| W3-28 | `expander.arrow.transitions (PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | field |
| W3-28 | `button.menu_indicator.transitions @ variant menu (PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | rule |
| W3-28 | `button.icon.transitions @ variant back (PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | rule |
| W3-28 | `checkbox.mark.transitions @ {checked} (PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | rule |
| W3-28 | `list.leading_icon.transitions @ variant sidebar (the settings gear; PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | rule |
| W3-28 | `sidebar.toggle_button.transitions (the hamburger; PropertyGroup::Icon, TransitionKind::Keyed per AnimatedIcon state; ADD-A27)` | field |
| W3-29 | `PartStyle.cursor per part` | field |
| W3-29 | `token.pointer.size (ADD-A46)` | token |
| W3-29 | `reader:windows:UISettings.CursorSize, SM_CXCURSOR` | reader |
| W3-47† | `token.text.roles.monospace` | token |
| W3-47† | `reader:windows:HKCU\Console FaceName, FontSize, FontWeight` | reader |
| W3-48† | `token.behaviour.mnemonics (ADD-A12)` | token |
| W3-48† | `reader:windows:SPI_GETKEYBOARDCUES` | reader |
| W3-49† | `drag_image.image.{fill,opacity}` | field |
| W3-49† | `drag_image.caption.text` | field |
| W3-49† | `drag_image.glyph` | field |
| W3-49† | `drag_image.drop_description.text` | field |
| W3-49† | `role.global.drop_highlight` | role |
| W3-49† | `list.row_background.fill @ {drop_target}` | rule |
| W3-50† | `token.pointer.{color,theme,size} (ADD-A46)` | token |
| W3-50† | `reader:windows:HKCU\Software\Microsoft\Accessibility (the pointer's colour and style; value names to verify)` | reader |
| W3-50† | - (the pointer's shadow, the Ctrl locator, pointer trails, the touch and pen contact and gesture visualisation) | out (R1.1) |
| W3-51† | `PartStyle.opacity of icon parts @ {disabled}` | rule |
| W3-51† | `PartStyle.glyph.fill @ {disabled}` | rule |
| W3-30 | `token.motion.durations.{normal,fast,faster}` | token |
| W3-31 | `token.motion.timings.{entering,exiting,control_key_spline}` | token |
| W3-32 | `popover.values.presentations, dialog.values.presentations, list.values.presentations, tab.values.presentations (one entry per WinUI theme transition)` | field |
| W3-32 | `PresentationKind::Keyed (ADD-A26)` | field |
| W3-33 | `PresentationKind::Keyed { key: connected_animation } with its parallax parameters (ADD-A26)` | field |
| W3-33 | `token.motion.durations.connected, token.motion.timings.connected` | token |
| W3-34 | `PartStyle.transitions and widget presentation transitions per control` | field |
| W3-34 | `token.motion.durations.*` | token |
| W3-52† | `token.behaviour.scroll (wheel animation, inertia, overpan, snap settle)` | token |
| W3-52† | `reader:windows:SPI_GETWHEELSCROLLLINES, SPI_GETWHEELSCROLLCHARS` | reader |
| W3-52† | `token.motion.speed_factor @ cond animations_off` | rule |
| W3-53† | `cond system_setting:menu_fade / cond system_setting:tooltip_fade / cond system_setting:selection_fade / cond system_setting:combo_animation / cond system_setting:listbox_smooth_scroll / cond system_setting:hot_tracking / cond system_setting:drop_shadow / cond system_setting:ui_effects (ADD-A04)` | condition |
| W3-53† | `menu.values.presentations @ !cond system_setting:menu_fade` | rule |
| W3-53† | `tooltip.values.presentations @ !cond system_setting:tooltip_fade` | rule |
| W3-53† | `combo_box.values.presentations @ !cond system_setting:combo_animation` | rule |
| W3-53† | `list.row_background.fill @ !cond system_setting:hot_tracking` | rule |
| W3-53† | `window.frame.shadow, menu.body.shadow @ !cond system_setting:drop_shadow` | rule |
| W3-53† | `menu.item.transitions @ !cond system_setting:selection_fade (the chosen item's fade-out)` | rule |
| W3-53† | `token.behaviour.scroll @ !cond system_setting:listbox_smooth_scroll (list boxes' animated scrolling)` | rule |
| W3-53† | `PartStyle.transitions, <widget>.values.presentations @ !cond system_setting:ui_effects (the master switch)` | rule |
| W3-53† | `reader:windows:SPI_GETMENUFADE, SPI_GETTOOLTIPFADE, SPI_GETSELECTIONFADE, SPI_GETCOMBOBOXANIMATION, SPI_GETLISTBOXSMOOTHSCROLLING, SPI_GETHOTTRACKING, SPI_GETDROPSHADOW, SPI_GETUIEFFECTS` | reader |
| W3-35 | `role.* @ cond contrast_high (eight system colours)` | rule |
| W3-35 | `PartStyle stroke and fill rules @ cond contrast_high (borders that appear only there)` | rule |
| W3-35 | `cond contrast_high` | condition |
| W3-35 | `reader:windows:GetSysColor under high contrast, AccessibilitySettings.HighContrastScheme, UISettings.UIElementColor` | reader |
| W3-36 | `PartStyle.{min_height,padding} = an expression on text_scale (controls that grow with the text; ADD-A55)` | field |
| W3-36 | `PartStyle.min_height = an expression on font_size` | field |
| W3-37 | `PartStyle.transitions @ cond animations_off per animation` | rule |
| W3-37 | `token.motion.speed_factor @ cond animations_off` | rule |
| W3-37 | `reader:windows:UISettings.AnimationsEnabled, SPI_GETCLIENTAREAANIMATION` | reader |
| W3-38 | `Material.fallback @ cond reduced_transparency per material` | rule |
| W3-38 | `reader:windows:UISettings.AdvancedEffectsEnabled, registry EnableTransparency` | reader |
| W3-39 | `declare.widget.*.densities (compact)` | field |
| W3-39 | `PartStyle.{padding,min_height} @ density compact` | rule |
| W3-40 | `menu.item.padding @ context menu.body {opened_by_touch}` | rule |
| W3-40 | `combo_box.item.padding @ context combo_box.popup {opened_by_touch}` | rule |
| W3-40 | `combo_box.popup.min_width @ {opened_by_touch}` | rule |
| W3-40 | `cond input:touch` | condition |
| W3-41 | `token.behaviour.{double_click_ms,hover_open_delay_ms,drag_threshold,scroll_lines_per_step,key_repeat_delay_ms}` | token |
| W3-41 | `menu.values.submenu_open_delay_ms` | field |
| W3-41 | - (`UISettings.MessageDuration`: how long the system's notifications stay, which the shell draws; WinUI has no timed message control) | out (R1.1) |
| W3-41 | `reader:windows:GetDoubleClickTime, UISettings.DoubleClickTime, SPI_GETMOUSEHOVERTIME, SPI_GETMENUSHOWDELAY, SM_CXDRAG, SM_CYDRAG, SPI_GETWHEELSCROLLLINES, UISettings.MessageDuration, SPI_GETKEYBOARDDELAY` | reader |
| W3-43 | - | preset (R1.3) |
| W3-44 | `window.title_bar.fill @ cond scheme:dark` | rule |
| W3-44 | `window.frame.stroke @ cond scheme:dark` | rule |
| W3-54† | `PartStyle rules @ dir rtl (the mirrored templates)` | rule |
| W3-54† | `token.icon_mirroring (S2.9)` | token |
| W3-54† | `Glyph.mirrors_in_rtl (AnimatedIcon's MirroredWhenRightToLeft; R25.8)` | field |
| W3-54† | `window.values.caption_button_layout` | field |
| W3-55† | `PlacementParams.handedness for WinUiFlyout (ADD-A22)` | placement |
| W3-55† | - (Win32 menus' drop alignment) | preset (R1.3) |
| W3-55† | `reader:windows:SPI_GETMENUDROPALIGNMENT` | reader |
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
| W-SF09† | `tab.values.presentations (event @tear_out)` | field |
| W-SF10† | `dialog.values.placement (CenteredInHost)` | placement |
| W-SF11† | `dialog.values.placement (CenteredInHost, host ClientArea or Window)` | placement |
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
| W-P3-1† | `window.frame.corners @ cond tiled` | rule |
| W-P3-1† | - (the snap overlay, the Snap bar and the shared edge of snapped windows: drawn by the shell) | out (R1.1) |
| W-P3-2† | `window.title_bar.* @ cond full_screen (Absent)` | rule |
| W-P3-2† | `window.values.full_screen_title_bar_reveal (SwipeFromTop)` | field |
| W-P3-2† | `window.values.presentations (event @title_bar_reveal)` | field |
| W-P3-2† | `window.* @ variant @compact_overlay` | rule |
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
| W-P3-9† | `token.caret.hides_while_selecting` | token |
| W-P3-9† | `input.caret.fill @ {drop_target}` | rule |
| W-P3-10† | `token.behaviour.hide_pointer_while_typing` | token |
| W-P3-10† | `reader:windows:SPI_GETMOUSEVANISH` | reader |
| W-P3-10† | `PartStyle.cursor Progress (Working in Background)` | field |
| W-P3-11† | `cond system_setting:show_sounds (ADD-A04)` | condition |
| W-P3-11† | `reader:windows:SM_SHOWSOUNDS` | reader |
| W-P3-11† | - (the SoundSentry flash) | out (R1.1) |
| W-P3-12† | `token.behaviour.double_click_distance (width and height, ADD-A45)` | token |
| W-P3-12† | `reader:windows:SM_CXDOUBLECLK, SM_CYDOUBLECLK` | reader |
| W-P3-13† | - | out (R1.1) |
| W-P3-14† | - | newest (R1.5) |
| W-P3-15† | - | preset (R1.3) |
| W-P3-16† | `token.theme_change` | token |
| W-P3-17† | `autocomplete.token.text.fill, autocomplete.token.fill, autocomplete.popup.corners, autocomplete.values.trigger_prefix` | field |
| W-P3-17† | `autocomplete.values.placement (WinUiAutoSuggest, relative to the caret: to verify)` | placement |
| W-RV-1† | `image_cropper.frame, image_cropper.mask.fill, image_cropper.overlay, image_cropper.values.padding` | field |
| W-RV-1† | `image_cropper.thumb.* @ variant corner / variant edge / {hover} / {pressed} / {disabled} / {focus_visible}` | rule |
| W-RV-2† | `list.placeholder.{fill,min_width,min_height} @ {placeholder}` | rule |
| W-RV-2† | `list.placeholder.fill @ cond contrast_high` | rule |
| W-RV-2† | `list.values.presentations (event @placeholder_fade)` | field |
| W-RV-3† | `component.tabbed_command_bar.content_border.fill` | component |
| W-RV-3† | `component.tabbed_command_bar.content_border.stroke` | component |
| W-RV-3† | `component.tabbed_command_bar.content_border.corners` | component |
| W-RV-3† | `component.tabbed_command_bar.content_border.{fill,stroke} @ cond contrast_high (the window colour, a transparent stroke)` | component |
| W-RV-3† | `component.tabbed_command_bar.command_bar` | component |
| W-RV-3† | `component.tabbed_command_bar.element_container.padding` | component |
| W-RV-3† | `component.tabbed_command_bar.contextual_tab.text` | component |
| W-RV-3† | `component.tabbed_command_bar.contextual_tab.text @ {hover} / {pressed} / {disabled}` | component |
| W-RV-3† | `component.tabbed_command_bar.contextual_tab.text @ {selected} / {selected,hover} / {selected,pressed}` | component |
| W-RV-3† | `component.tabbed_command_bar.values.presentations (event @tab_change: PresentationKind::Keyed { key: pop_in })` | component |
| W-RV-3† | `button.values.label_position @ context component.tabbed_command_bar.command_bar (TextBesideIcon)` | rule |
| W-P4-1† | `window.launch_screen.fill` | field |
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
| W-P4-12† | `token.icon_mirroring (S2.9; the "Mirrored" code points as the right-to-left forms)` | token |
| W-P4-12† | `GlyphSource per direction (ADD-B05, R25.8)` | field |
| W-P4-13† | - | preset (R1.3) |
| W-P4-14† | - | out (R1.1) |
| W-P4-15† | - | out (R1.1) |
| W-P4-16† | `scrollbar.values.repeat_delay; scrollbar.values.repeat_interval` | field |
| W-P4-16† | `scrollbar.values.trough_click` | field |
| W-P4-17† | `menu.values.submenu_close_delay_ms (= the read MenuShowDelay)` | field |
| W-P4-17† | `menu.values.submenu_switch (CloseTimer, mouse only)` | field |
| W-P4-17† | `reader:windows:HKCU Control Panel Desktop MenuShowDelay (also the close delay)` | reader |
| W-P4-18† | `component.casting_picker.picker.fill` | component |
| W-P4-18† | `component.casting_picker.title.text` | component |
| W-P4-18† | `component.casting_picker.item.fill @ {selected}` | component |
| W-P4-18† | `component.casting_picker.item.text.fill @ {selected}` | component |
| W-P4-18† | `component.casting_picker.values.placement (WinUiFlyout)` | placement |
| W-P4-18† | `route_button.{body,glyph} @ context media_controls (the CastButton)` | rule |
| W-P4-18† | `reader:windows:DevicePickerAppearance getters` | reader |
| W-P4-19† | `sidebar.values.display_mode_breakpoints` | field |
| W-P4-19† | `layout.values.{max_panes_horizontal,max_panes_vertical,pane_width} @ variant pane` | rule |
