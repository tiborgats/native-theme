# v0.7.1 — Model completeness: Specification, traceability (macOS)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-macos.md`](todo_v0.7.1_model-completeness-gap-map-macos.md) mapped to
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
`{a,b}` states that hold together, `!{a}` states absent, `variant v` (`@v` keyed, `!variant v` negated),
`cond c` (a condition, S7.1; `flavour:kirigami`, `system_setting:<key>`; `!cond c` negated), `context w.part` or
`context any w` (an ancestor), `not_context w.part`, `pos first` / `pos last` / `pos only` / `pos parity odd` and
`prev w{…}` / `next w{…}` (siblings), `size s`, `h_size_class c` / `v_size_class c`, `dir rtl`; ` / ` separates
alternatives, each a rule of its own. A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| MAC-W01 | `window.title_bar.min_height @ variant @no_toolbar / @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W01 | `reader:macos:NSWindow frameRect(forContentRect:styleMask:) and contentLayoutRect` | reader |
| MAC-W02 | `window.caption_buttons.padding` | field |
| MAC-W02 | `window.values.caption_button_gap_px` | field |
| MAC-W02 | `window.caption_button_glyph.{fill,stroke,min_width,min_height}` | field |
| MAC-W02 | `window.caption_button_glyph.glyph.fill @ context window.caption_buttons {hover}` | rule |
| MAC-W02 | `window.caption_button_glyph.fill @ {pressed}` | rule |
| MAC-W02 | `window.caption_button_glyph.fill @ cond window_inactive` | rule |
| MAC-W02 | `window.caption_button_glyph.fill @ {disabled}` | rule |
| MAC-W02 | `window.caption_button_glyph.glyph @ variant close context window {@document_edited}` | rule |
| MAC-W02 | `window flags @document_edited` | flag |
| MAC-W02 | `reader:macos:standardWindowButton frames per toolbar style` | reader |
| MAC-W03 | `window.values.title_alignment @ variant @toolbar_unified / @toolbar_unified_compact` | rule |
| MAC-W03 | `window.subtitle.text` | field |
| MAC-W03 | `window.subtitle.text.fill @ cond window_inactive` | rule |
| MAC-W03 | `window.title_icon.{icon,padding}` | field |
| MAC-W03 | `window.title.text.elision` | field |
| MAC-W04 | `window.title_bar.stroke @ variant @separator_line {scrolled}` | rule |
| MAC-W04 | `window.title_bar.shadow @ variant @separator_shadow {scrolled}` | rule |
| MAC-W05 | `window.frame.corners (Continuous)` | field |
| MAC-W05 | `window.frame.corners @ variant utility / @hud / dialog / @no_toolbar / @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W05 | `reader:macos:effectiveCornerRadii (26; to verify)` | reader |
| MAC-W06 | `window.frame.stroke (outer edge line)` | field |
| MAC-W06 | `window.frame.shadow (inset rim layer)` | field |
| MAC-W06 | `window.frame.{stroke,shadow} @ cond scheme:dark` | rule |
| MAC-W06 | `window.frame.{stroke,shadow} @ cond window_inactive` | rule |
| MAC-W07 | `window.frame.shadow` | field |
| MAC-W07 | `window.frame.shadow @ cond window_inactive` | rule |
| MAC-W07 | `window.frame.shadow @ variant utility / @hud` | rule |
| MAC-W08 | `window.content.fill (material)` | field |
| MAC-W08 | `window.content.fill @ cond reduced_transparency` | rule |
| MAC-W08 | `window.content.fill @ !cond system_setting:wallpaper_tinting` | rule |
| MAC-W09 | `window.{tab_bar,tab,tab_title,tab_close_button,new_tab_button,tab_separator}` | field |
| MAC-W09 | `window.{tab,tab_close_button} @ {selected} / {hover}` | rule |
| MAC-W09 | `window.tab_bar.fill @ cond window_inactive` | rule |
| MAC-W10 | `window.title_bar.fill (titlebar material) @ variant @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W11† | `window.title_chevron.{glyph,opacity}` | field |
| MAC-W11† | `window.title_chevron.{opacity,transitions} @ context window.title {hover}` | rule |
| MAC-W11† | `window.title_status.text @ context window {@document_edited}` | rule |
| MAC-W11† | `popover.* @ context window.title (rename popover)` | rule |
| MAC-W12† | `window.title_bar.fill (fullScreenUI material) @ cond full_screen` | rule |
| MAC-W12† | `window.values.full_screen_title_bar_reveal (PointerAtTopEdge)` | field |
| MAC-W12† | `window.values.presentations event @title_bar_reveal` | field |
| MAC-W12† | `window.frame.{corners,stroke} @ cond full_screen` | rule |
| MAC-W12† | - (the menu bar coming in with the reveal; the divider between two windows in a full-screen Split View) | out (R1.1) |
| MAC-W13† | `window.values.caption_button_menus (the zoom button's tiling menu)` | field |
| MAC-W13† | `menu.* @ context window.caption_button variant maximize` | rule |
| MAC-W13† | `window.frame.corners @ cond tiled:left / tiled:right / tiled:top / tiled:bottom` | rule |
| MAC-W13† | - (the drag-to-edge tiling preview; the margins between tiled windows) | out (R1.1) |
| MAC-W14† | `window.title_bar.{fill,stroke} @ variant transparent_title_bar` | rule |
| MAC-W14† | `window.title @ variant @title_hidden (absent)` | rule |
| MAC-W14† | `window.title_bar.min_height @ variant @no_toolbar` | rule |
| MAC-W15† | `window.title.text.fill @ cond window_inactive` | rule |
| MAC-B01 | `button.body.*, button.label.* @ variant @flexible_push / @small_square / tool / @accessory_bar / @accessory_bar_action / @badge / circular / help / glass / @recessed / @round_rect / @inline / @textured_rounded / @shadowless_square / flat (borderless)` | rule |
| MAC-B01 | `expander.header, expander.arrow @ variant @push_disclosure; expander.arrow (the disclosure bezel, MAC-EX01)` | rule |
| MAC-B01 | `button.body.*, button.label.* @ {pressed} / {disabled} / {checked} per bezel variant` | rule |
| MAC-B02 | `button.body.fill, button.label.text.fill @ {default} !cond window_inactive !cond app_inactive` | rule |
| MAC-B02 | `button.body.fill @ {default,pressed} / {default,disabled} / {default,hover}` | rule |
| MAC-B03 | `button.body.fill, button.label.text.fill @ {default,hover} / {default,pressed} / {default,disabled}` | rule |
| MAC-B04 | `button.body.shadow (layers; macos-monterey)` | field |
| MAC-B05 | `button.body.fill (Linear gradient layer)` | field |
| MAC-B05 | `button.body.shadow (inset highlight layer for the bevel)` | field |
| MAC-B06 | `button.label.text.fill @ variant destructive (and context any dialog where it differs)` | rule |
| MAC-B07 | `button.body.fill, button.label.text.fill @ variant tinted / variant primary / variants {glass,tinted}` | rule |
| MAC-B08 | `button.body.corners @ variant pill / @rounded_rectangle / circular` | rule |
| MAC-B08 | `button.body.corners @ size small / regular / large / extra_large (automatic shape)` | rule |
| MAC-B09 | `button.icon.icon.size @ size mini / small / regular / large / extra_large` | rule |
| MAC-B09 | `button.icon.icon.fill @ {pressed} / {disabled} / {default}` | rule |
| MAC-B09 | `button.values.icon_placement` | field |
| MAC-B09 | `button.body.padding @ variant icon_only` | rule |
| MAC-B10 | `button.body.fill @ {checked,pressed} / {checked,disabled} variant toggle` | rule |
| MAC-B10 | `button.body.fill @ {checked} variant toggle cond window_inactive` | rule |
| MAC-B11† | `button.body.min_width @ size mini / small / regular / large / extra_large` | rule |
| MAC-I01 | `input.body @ variant @square_bezel / @rounded_bezel / line / flat` | rule |
| MAC-I01 | `input.body @ context list.row` | rule |
| MAC-I02 | `input.{body.fill,body.stroke,text.text.fill} @ {read_only}` | rule |
| MAC-I03 | `input.values.mask_character` | field |
| MAC-I03 | `input.caps_lock_indicator` | field |
| MAC-I04 | `input.body.shadow (inset edge layers; outer ring layer)` | field |
| MAC-I04 | `input.body.stroke` | field |
| MAC-I05 | `input.selection.{fill,text.fill} @ !{focused}` | rule |
| MAC-I05 | `role.global.selection_text_background_unfocused` | role |
| MAC-I05 | `role.global.selection_text_background, role.global.selection_text_foreground @ cond window_inactive` | rule |
| MAC-I05 | `reader:macos:NSColor.unemphasizedSelectedTextBackgroundColor, unemphasizedSelectedTextColor -> role.global.selection_text_background_unfocused, role.global.selection_text_foreground @ cond window_inactive` | reader |
| MAC-I06 | `input.prediction.text; text_area.prediction.text` | field |
| MAC-I06 | `popover.body @ context text_area` | rule |
| MAC-I06 | `list.row @ context text_area` | rule |
| MAC-I07 | `text_area.text.text.font` | field |
| MAC-I07 | `reader:macos:NSFont.userFont(ofSize:0) -> text_area.text.text.font; NSFont.userFixedPitchFont(ofSize:0) -> token.text.roles.monospace` | reader |
| MAC-I08 | `scroll_view.frame @ variant line / @bezel / @groove / flat` | rule |
| MAC-I09† | `input.autofill_button` | field |
| MAC-I09† | `input.{body.fill,text.text.fill} @ {autofilled}` | rule |
| MAC-I10† | `text_area.values.document_margin; text_area.values.line_padding` | field |
| MAC-I10† | `reader:macos:NSTextView.textContainerInset, textContainer.lineFragmentPadding -> text_area.values.document_margin, line_padding` | reader |
| MAC-C01 | `checkbox.mark.glyph @ {indeterminate} = SystemControl(AppKitCheckbox)` | system-control |
| MAC-C01 | `checkbox.box.fill, checkbox.mark.glyph.size, checkbox.mark.glyph.fill @ {indeterminate}` | rule |
| MAC-C02 | `checkbox.box.fill @ {pressed} / {checked,pressed}` | rule |
| MAC-C02 | `radio.ring.fill @ {pressed} / {checked,pressed}` | rule |
| MAC-C03 | `checkbox.box.fill, checkbox.mark.glyph.fill @ {checked,disabled}` | rule |
| MAC-C04 | `checkbox.box.fill @ {checked,hover}` | rule |
| MAC-C05 | `checkbox.box.fill @ cond scheme:dark (Linear gradient)` | rule |
| MAC-C05 | `checkbox.box.stroke (gradient paint), checkbox.box.shadow (inset bevel layer)` | field |
| MAC-C06 | `checkbox.mark.glyph = SystemControl(AppKitCheckbox)` | system-control |
| MAC-C06 | `checkbox.mark.glyph.size, checkbox.mark.glyph.fill (measured)` | field |
| MAC-C07 | `radio.ring.fill @ {pressed}; radio.dot.fill @ {checked,disabled}` | rule |
| MAC-C07 | `radio.ring.*, radio.dot.* @ cond window_inactive` | rule |
| MAC-C07 | `radio.dot.glyph = SystemControl(AppKitRadio)` | system-control |
| MAC-MN01 | `menu.item.corners` | field |
| MAC-MN01 | `menu.body.padding (highlight inset)` | field |
| MAC-MN02 | `menu.body.padding (top, bottom)` | field |
| MAC-MN03 | `menu.separator.{min_height,padding,stroke}` | field |
| MAC-MN04 | `menu.accelerator.text` | field |
| MAC-MN04 | `menu.accelerator.text.fill @ {hover} / {disabled}` | rule |
| MAC-MN04 | `menu.values.{accelerator_gap_px,accelerator_alignment}` | field |
| MAC-MN05 | `menu.check_indicator.glyph @ {checked} / {indeterminate} (NamedSystemImage)` | rule |
| MAC-MN05 | `menu.values.{check_column,check_column_width_px}` | field |
| MAC-MN06 | `menu.submenu_arrow.glyph` | field |
| MAC-MN06 | `menu.submenu_arrow.glyph.fill @ {hover} / {disabled}` | rule |
| MAC-MN06 | `menu.item.fill @ {open}` | rule |
| MAC-MN07 | `menu.section_header.{text,min_height}` | field |
| MAC-MN08 | `badge.{body,label} @ context menu.item` | rule |
| MAC-MN09 | `menu.item_subtitle.text` | field |
| MAC-MN10 | `menu.item_icon.icon.fill @ {hover} / {disabled}` | rule |
| MAC-MN10 | `menu.values.show_item_icons` | field |
| MAC-MN11 | `menu.values.indent_step_px` | field |
| MAC-MN12 | `menu.scroll_arrow` | field |
| MAC-MN13 | `menu.item @ variant palette` | rule |
| MAC-MN13 | `menu.item.stroke @ variant palette {checked} (selection ring)` | rule |
| MAC-MN14 | `menu.body.{stroke,shadow,fill}` | field |
| MAC-MN15 | `menu.values.placement (AppKitMenu, over_selected_item)` | placement |
| MAC-MN15 | `menu.item_label.text @ context combo_box size mini / small / large` | rule |
| MAC-MN16 | `menu.body.min_width` | field |
| MAC-MN17† | `menu.item_label.text.fill @ variant destructive` | rule |
| MAC-T01 | `tooltip.body.shadow` | field |
| MAC-T02 | `tooltip.body.fill (material)` | field |
| MAC-T03 | `tooltip.values.placement (AppKitToolTip)` | placement |
| MAC-T04 | `tooltip.values.{reshow_delay_ms,display_time}` | field |
| MAC-T04 | `tooltip.values.presentations` | field |
| MAC-T05 | `tooltip.values.show_delay_ms` | field |
| MAC-T05 | `reader:macos:NSInitialToolTipDelay user default` | reader |
| MAC-T06† | `tooltip.* @ variant @expansion` | rule |
| MAC-T06† | `tooltip.values.placement @ variant @expansion (AppKitExpansionTip)` | placement |
| MAC-S01 | `scrollbar.thumb.{min_width,min_height} @ cond overlay_scrollbars context scrollbar {hover}` | rule |
| MAC-S02 | `scrollbar.track.{fill,stroke} @ {hover} cond overlay_scrollbars` | rule |
| MAC-S03 | `scrollbar.thumb.{stroke,fill} @ variant dark / light` | rule |
| MAC-S04 | `scrollbar.thumb.corners` | field |
| MAC-S04 | `scrollbar.track.padding` | field |
| MAC-S05 | `scrollbar.track.stroke @ !cond overlay_scrollbars` | rule |
| MAC-S06 | `scroll_view.corner.fill` | field |
| MAC-S07 | `scrollbar.body.{min_width,min_height} @ size small / size mini` | rule |
| MAC-S07 | `reader:macos:NSScroller.scrollerWidth(for:scrollerStyle:) per control size -> scrollbar.body sizes` | reader |
| MAC-S08 | `scrollbar.thumb.fill @ {pressed}` | rule |
| MAC-SL01 | `slider.thumb.{min_width,min_height,corners}` | field |
| MAC-SL01 | `reader:macos:NSSliderCell.knobRect(flipped:) -> slider.thumb size` | reader |
| MAC-SL02 | `slider.thumb.shadow (drop layers and rim layer)` | field |
| MAC-SL03 | `slider.track.corners; slider.fill.corners` | field |
| MAC-SL04 | `slider.tick.{fill,min_width,min_height,corners}` | field |
| MAC-SL04 | `slider.values.tick_gap` | field |
| MAC-SL04 | `slider.tick @ variant ticks_before / ticks_after (and with vertical)` | rule |
| MAC-SL04 | `reader:macos:NSSlider.rectOfTickMark(at:) -> slider.tick geometry, slider.values.tick_gap` | reader |
| MAC-SL05 | `slider.thumb.{corners,min_width,min_height} @ variant ticks_before / ticks_after` | rule |
| MAC-SL06 | `slider.fill.fill @ variant ticks_before / ticks_after` | rule |
| MAC-SL06 | `slider.fill @ variant centered` | rule |
| MAC-SL07 | `slider.thumb.{fill,transform} @ {pressed}` | rule |
| MAC-P01 | `progress_bar.indicator @ {indeterminate}` | rule |
| MAC-P01 | `progress_bar.values.loop` | field |
| MAC-P02 | `progress_bar.track.corners; progress_bar.indicator.corners` | field |
| MAC-P03 | `progress_bar.indicator.fill @ cond window_inactive` | rule |
| MAC-P04 | `progress_bar.indicator.transitions (PropertyGroup::Geometry)` | field |
| MAC-P05† | `progress_bar.track.stroke` | field |
| MAC-TB01 | `tab.pane (fill, stroke, corners, padding)` | field |
| MAC-TB01 | `reader:macos:NSTabView.contentRect` | reader |
| MAC-TB02 | `tab.values.bar_pane_overlap` | field |
| MAC-TB03 | `tab.bar @ variant top / bottom / left / right / @no_tabs` | rule |
| MAC-TB03 | `tab.pane @ variant @border_none / @border_line / @border_bezel` | rule |
| MAC-TB04 | `tab.tab.fill, tab.label.text.fill @ {pressed}` | rule |
| MAC-TB04 | `tab.tab.fill, tab.label.text.fill @ {selected} cond window_inactive` | rule |
| MAC-TB05 | `tab.tab, tab.label.text, tab.bar.min_height @ size mini / small / regular / large` | rule |
| MAC-TB06† | `tab.bar.stroke, tab.separator.stroke, tab.tab.fill @ {selected}, tab.label.text.fill @ {selected}` | rule |
| MAC-TB07† | `tab.label.text.elision, tab.values.overflow` | field |
| MAC-SB01 | `list.row.min_height, list.leading_icon.icon.size, list.row.text @ variant sidebar cond container_size(sidebar, small) / container_size(sidebar, medium) / container_size(sidebar, large)` | rule |
| MAC-SB01 | `reader:macos:effectiveRowSizeStyle, rowHeight, NSTableViewDefaultSizeMode` | reader |
| MAC-SB01 | `cond container_size(sidebar)` | condition |
| MAC-SB02 | `list.leading_icon.icon.fill @ variant sidebar / variant sidebar {selected} / variant sidebar {selected} context list.view !{focus_within}` | rule |
| MAC-SB03 | `list.section_header.{text,min_height} @ variant sidebar` | rule |
| MAC-SB03 | `list.disclosure @ context list.section_header {hover} variant sidebar` | rule |
| MAC-SB04 | `list.row_background.corners, list.values.row_background_inset @ variant sidebar` | rule |
| MAC-SB04 | `list.row_background.corners @ variant sidebar {selected} prev list.row{selected} / variant sidebar {selected} next list.row{selected} (ADD-B07)` | rule |
| MAC-SB05 | `list.row_background.fill, list.row.text.fill @ variant sidebar {selected} context list.view !{focus_within}` | rule |
| MAC-SB05 | `list.row_background.fill, list.row.text.fill @ variant sidebar {selected} cond window_inactive` | rule |
| MAC-SB05 | `reader:macos:unemphasizedSelectedContentBackgroundColor` | reader |
| MAC-SB06 | `sidebar.pane.{fill,corners,shadow} @ variant floating (material fill, Concentric corners)` | rule |
| MAC-SB06 | `sidebar.values.floating_inset` | field |
| MAC-SB06 | `reader:macos:effectiveCornerRadii` | reader |
| MAC-SB07 | `sidebar.pane.fill @ cond window_inactive` | rule |
| MAC-SB08 | `list.disclosure, list.values.indent_per_level @ variant sidebar` | rule |
| MAC-SB08 | `reader:macos:NSOutlineView.indentationPerLevel` | reader |
| MAC-SB09† | `sidebar.values.width, min_width, max_width` | field |
| MAC-SB09† | `sidebar.values.presentations event @collapse` | field |
| MAC-SB09† | `reader:macos:NSSplitViewItem minimumThickness, maximumThickness` | reader |
| MAC-TL01 | `toolbar.bar.min_height, toolbar.bar.padding @ variant @unified / @unified_compact / @expanded / @preference` | rule |
| MAC-TL01 | `reader:macos:contentLayoutRect per toolbar style` | reader |
| MAC-TL02 | `button.body.fill @ context toolbar.bar variant tool {hover} / {pressed} / {disabled}` | rule |
| MAC-TL02 | `button.icon.icon.fill, button.label.text @ context toolbar.bar variant tool {hover} / {pressed} / {disabled}` | rule |
| MAC-TL02 | `button.values.icon_text_gap, button.values.label_position (ButtonLabelPosition; the icon-and-label display mode) @ context toolbar.bar variant tool` | rule |
| MAC-TL03 | `toolbar.item_group.{fill,padding,min_height,corners} (material fill)` | field |
| MAC-TL03 | `toolbar.values.group_gap` | field |
| MAC-TL04 | `button.body.fill @ context toolbar.bar variant primary (the accent tint)` | rule |
| MAC-TL05 | `badge.* @ context toolbar.bar` | rule |
| MAC-TL06 | `toolbar.overflow_button` | field |
| MAC-TL06 | `menu.* @ context toolbar.overflow_button` | rule |
| MAC-TL07 | `button.* @ context toolbar.bar variant tool (the collapsed search item)` | rule |
| MAC-TL07 | `search_field.* @ context toolbar.bar` | rule |
| MAC-TL07 | `toolbar.values.presentations event @search_expand` | field |
| MAC-TL08 | `toolbar.separator @ variant @tracking` | rule |
| MAC-TL09 | `toolbar.bar.fill (material)` | field |
| MAC-TL09 | `toolbar.bar.fill @ !{scrolled} (transparent over content)` | rule |
| MAC-TL10† | `toolbar.values.fixed_space_width, flexible_space_min_width` | field |
| MAC-TL11† | `button.body.fill, button.icon.icon.fill, button.label.text.fill @ context toolbar.bar variant tool {selected}` | rule |
| MAC-TL11† | `button.body.fill @ context toolbar.bar variant tool {selected} cond window_inactive` | rule |
| MAC-ST01 | `status_bar.bar.min_height, status_bar.bar.fill, status_bar.bar.stroke.top` | field |
| MAC-ST01 | `status_bar.bar @ cond window_inactive` | rule |
| MAC-L01 | `list.values.row_background_inset, list.row_background.corners, list.header.padding @ variant @automatic / @full_width / inset / sidebar / plain` | rule |
| MAC-L01 | `reader:macos:headerRect(ofColumn:), rect(ofRow:) per style` | reader |
| MAC-L02 | `list.row_background.corners, list.values.row_background_inset @ {selected}` | rule |
| MAC-L02 | `list.row_background.corners @ {selected} prev list.row{selected} / {selected} next list.row{selected} (ADD-B07)` | rule |
| MAC-L03 | `list.row_background.fill, list.row.text.fill @ {selected} context list.view !{focus_within} / {selected} cond window_inactive` | rule |
| MAC-L03 | `reader:macos:unemphasizedSelectedContentBackgroundColor` | reader |
| MAC-L04 | `list.secondary_text.text.fill, list.leading_icon.icon.fill @ {selected} context list.view {focus_within}` | rule |
| MAC-L05 | `list.header_separator.stroke, list.header.stroke.bottom` | field |
| MAC-L05 | `list.sort_indicator.glyph (NamedSystemImage NSAscendingSortIndicator, NSDescendingSortIndicator)` | field |
| MAC-L05 | `list.header_section.fill @ {pressed}` | rule |
| MAC-L05 | `list.header_section.padding, list.values.header_text_alignment` | field |
| MAC-L06 | `list.row_separator.stroke @ variant row_separators / @dashed_horizontal_grid (dash, ADD-A32)` | rule |
| MAC-L06 | `list.column_separator.stroke @ variant column_separators` | rule |
| MAC-L07 | `list.values.intercell_gap` | field |
| MAC-L07 | `reader:macos:NSTableView.intercellSpacing` | reader |
| MAC-L08 | `list.row.min_height @ variant @row_size_small / @row_size_medium / @row_size_large` | rule |
| MAC-L08 | `reader:macos:rowHeight per rowSizeStyle` | reader |
| MAC-L09 | `list.section_header.{text,fill,min_height} @ variant table (material fill)` | rule |
| MAC-L10 | `list.values.indent_per_level` | field |
| MAC-L10 | `list.disclosure.glyph (SystemControl AppKitDisclosure)` | system-control |
| MAC-L10 | `list.disclosure.glyph.fill @ {pressed}; list.disclosure.transform @ {open}` | rule |
| MAC-L10 | `list.disclosure.transitions (transform)` | field |
| MAC-L10 | `reader:macos:NSOutlineView.indentationPerLevel` | reader |
| MAC-L11 | `list.row_background.stroke, list.row_background.corners @ {drop_target}` | rule |
| MAC-L11 | `list.drop_indicator (stroke, glyph end circle)` | field |
| MAC-L12 | `list.swipe_action.{fill,text} @ variant destructive / !variant destructive` | rule |
| MAC-L13 | `input.body.{fill,focus} @ context list.cell {editing}` | rule |
| MAC-L14 | `list.view.focus @ {focus_visible}` | rule |
| MAC-L14 | `list.values.focus_visible_when (ADD-A16)` | field |
| MAC-L15 | `list.header_section.opacity @ {dragged}` | rule |
| MAC-L16 | `list.view.stroke` | field |
| MAC-L17† | `button.*, checkbox.*, combo_box.*, progress_bar.* @ context list.row {selected} context list.view {focus_within}` | rule |
| MAC-L17† | `list.row_background.fill @ {selected} context list.view {disabled}` | rule |
| MAC-L18† | `list.header_section.fill @ {sorted} (highlighted column)` | rule |
| MAC-L18† | `list.header_corner (fill, stroke)` | field |
| MAC-L19† | `list.row_background.{stroke,corners} @ {context_menu_open}` | rule |
| MAC-L19† | `list.values.row_background_inset (the outline's inset, as the selection's; to verify)` | field |
| MAC-PO01 | `popover.arrow.{min_width,min_height,corners}` | field |
| MAC-PO01 | `popover.values.placement (AppKitPopover gap_px)` | placement |
| MAC-PO01 | `popover.values.full_size_content` | field |
| MAC-PO02 | `popover.body.fill (material)` | field |
| MAC-PO03 | `popover.body.shadow` | field |
| MAC-PO04 | `popover.* @ variant @detached` | rule |
| MAC-PO05 | `popover.values.presentations` | field |
| MAC-SP01 | `splitter.divider @ variant @thin / @thick / @pane_splitter` | rule |
| MAC-SP01 | `reader:macos:NSSplitView.dividerThickness per style` | reader |
| MAC-SP02 | `splitter.grip @ variant @thick / @pane_splitter` | rule |
| MAC-SP03 | `splitter.divider @ {dragged}` | rule |
| MAC-SP04 | `splitter.divider @ variant sidebar (fill and stroke absent)` | rule |
| MAC-SW01 | `switch.thumb.min_width, switch.thumb.min_height, switch.thumb.corners (Full)` | field |
| MAC-SW02 | `switch.thumb.shadow (layers)` | field |
| MAC-SW02 | `switch.thumb.stroke (the rim)` | field |
| MAC-SW03 | `switch.thumb.min_width, switch.thumb.fill, switch.track.fill @ {pressed}` | rule |
| MAC-SW04 | `switch.track.fill @ {checked} cond window_inactive` | rule |
| MAC-SW05 | `switch.track.min_width, switch.track.min_height, switch.thumb.min_width, switch.thumb.min_height @ size mini / small / regular / large` | rule |
| MAC-SW05 | `[declare.widget.switch] sizes for macos-*` | field |
| MAC-SW06 | `switch.on_glyph.glyph, switch.off_glyph.glyph @ cond differentiate_without_colour (to verify)` | rule |
| MAC-D01 | `dialog.values.{title_alignment,message_alignment,action_layout} @ variant alert` | rule |
| MAC-D01 | `dialog.body.min_width @ variant alert (fixed width)` | rule |
| MAC-D01 | `dialog.message.text @ variant alert` | rule |
| MAC-D01 | `checkbox.* @ context dialog.body variant alert (suppression check box)` | rule |
| MAC-D02 | `dialog.icon_badge` | field |
| MAC-D02 | `dialog.icon_badge @ variant error` | rule |
| MAC-D03 | `dialog.body.{corners,fill,shadow} @ variant @sheet` | rule |
| MAC-D03 | `dialog.scrim @ variant @sheet` | rule |
| MAC-D04 | `dialog.values.presentations event Disappear` | field |
| MAC-D04 | `dialog.values.presentations events @sheet_attach, @sheet_detach @ variant @sheet` | rule |
| MAC-SN01 | `spinner.spoke.fill` | field |
| MAC-SN01 | `spinner.values.{spoke_count,spoke_inner_radius,spoke_length,spoke_width,spoke_opacity_ramp,cap}` | field |
| MAC-SN02 | `spinner.values.loop (stepped timing)` | field |
| MAC-SN03 | `spinner.values.diameter @ size small` | rule |
| MAC-SN03 | `reader:macos:NSProgressIndicator.sizeToFit per control size -> spinner.values.diameter` | reader |
| MAC-SN04 | `spinner.{indicator,track} @ variant determinate` | rule |
| MAC-SN05† | `spinner.values.presentations` | field |
| MAC-CB01 | `combo_box.arrow.glyph (SystemControl::AppKitPopUp)` | system-control |
| MAC-CB01 | `combo_box.arrow.glyph @ variant pull_down` | rule |
| MAC-CB01 | `combo_box.arrow_area.{fill,stroke,corners} @ variant editable` | rule |
| MAC-CB01 | `input.body @ context combo_box variant editable` | rule |
| MAC-CB02 | `combo_box.arrow_area.fill` | field |
| MAC-CB02 | `combo_box.arrow_area.fill @ cond window_inactive` | rule |
| MAC-CB03 | `combo_box.arrow.glyph.fill @ {pressed} / {disabled}` | rule |
| MAC-CB04 | `combo_box.{body,arrow} @ {open}` | rule |
| MAC-CB04 | `combo_box.{popup,item} @ variant editable` | rule |
| MAC-CB04 | `combo_box.values.max_visible_items` | field |
| MAC-CB05 | `combo_box.body.{fill,stroke,shadow} @ variant flat` | rule |
| MAC-CB06 | `combo_box.body.fill @ cond window_inactive` | rule |
| MAC-SG01 | `segmented_control.separator.fill` | field |
| MAC-SG01 | `segmented_control.separator.opacity @ prev segmented_control.segment{checked} / next segmented_control.segment{checked}` | rule |
| MAC-SG02 | `segmented_control.indicator.shadow (macos-monterey)` | field |
| MAC-SG03 | `segmented_control.label.text.font.weight, segmented_control.label.text.fill @ {checked}` | rule |
| MAC-SG03 | `segmented_control.indicator.fill (macos-tahoe: the accent)` | field |
| MAC-SG04 | `segmented_control.track.*, segmented_control.segment.* @ variant @rounded / @round_rect / @textured_rounded / pill / @textured_square / @small_square / @separated / @rounded_rectangle` | rule |
| MAC-SG05 | `segmented_control.* @ variant @select_any / momentary` | rule |
| MAC-SG05 | `segmented_control.indicator.fill (selectedSegmentBezelColor default)` | field |
| MAC-SG06 | `segmented_control.segment.fill @ {pressed}; segmented_control.indicator.fill @ {checked,pressed}` | rule |
| MAC-SG06 | `segmented_control.segment.opacity, segmented_control.label.text.fill @ {disabled} (per segment)` | rule |
| MAC-SG06 | `segmented_control.indicator.fill, segmented_control.label.text.fill @ {checked} cond window_inactive` | rule |
| MAC-SG07 | `segmented_control.menu_indicator.glyph @ variant menu` | rule |
| MAC-SG08 | `segmented_control.segment.corners @ pos first / pos last / pos only (outer corners only)` | rule |
| MAC-SG08 | `segmented_control.indicator.corners (Concentric)` | field |
| MAC-SG09 | `segmented_control.values.presentations event Move (part indicator; macos-tahoe)` | field |
| MAC-SG10† | `segmented_control.icon.icon.size @ size mini / small / regular / large` | rule |
| MAC-SG10† | `segmented_control.values.image_scaling` | field |
| MAC-SG10† | `segmented_control.icon.icon.fill @ {checked} / {pressed} / {disabled}` | rule |
| MAC-BX01 | `group_box.title.text, group_box.values.title_position, group_box.values.title_margin` | field |
| MAC-BX01 | `reader:macos:NSBox titleFont and titleRect` | reader |
| MAC-BX02 | `group_box.* @ variant @primary / @custom / @separator / @transparent` | rule |
| MAC-EX01 | `expander.header, expander.arrow @ variant @push_disclosure` | rule |
| MAC-EX01 | `expander.arrow.glyph (GlyphSource SystemControl AppKitPushDisclosure, ADD-A58)` | system-control |
| MAC-EX02 | `expander.arrow.glyph (GlyphSource SystemControl AppKitDisclosure; AppKitPushDisclosure, ADD-A58)` | system-control |
| MAC-EX03 | `expander.arrow.glyph.fill @ {pressed}` | rule |
| MAC-EX04 | `expander.arrow.transitions (transform)` | field |
| MAC-EX05† | `expander.arrow.min_width, expander.arrow.min_height @ size mini / small / regular / large` | rule |
| MAC-EX05† | `expander.values.arrow_gap, expander.values.content_indent` | field |
| MAC-EX05† | `expander.header.min_height` | field |
| MAC-EX05† | `reader:macos:intrinsicContentSize of a disclosure button per control size` | reader |
| MAC-LK01 | `link.text.text.decoration (style, paint, thickness, offset) @ variant inline` | rule |
| MAC-LK01 | `reader:macos:NSTextView.linkTextAttributes (underline style, colour)` | reader |
| MAC-LK02 | `link.text.cursor` | field |
| MAC-LK02 | `reader:macos:NSTextView.linkTextAttributes[.cursor]` | reader |
| MAC-LK03 | `link.text.focus` | field |
| MAC-LK03 | `link.values.focus_visible_when (Never)` | field |
| MAC-LK03 | `link.values.focus_visible_when @ cond full_keyboard_access (Keyboard { … })` | rule |
| MAC-LK03 | `cond full_keyboard_access (ADD-A06)` | condition |
| MAC-DF01 | `role.@label_tertiary` | role |
| MAC-DF01 | `role.@label_quaternary` | role |
| MAC-DF01 | `role.@label_quinary` | role |
| MAC-DF02 | `role.@system_fill` | role |
| MAC-DF02 | `role.@system_fill_secondary` | role |
| MAC-DF02 | `role.@system_fill_tertiary` | role |
| MAC-DF02 | `role.@system_fill_quaternary` | role |
| MAC-DF02 | `role.@system_fill_quinary` | role |
| MAC-DF03 | `role.@system_yellow` | role |
| MAC-DF03 | `role.@system_purple` | role |
| MAC-DF03 | `role.@system_pink` | role |
| MAC-DF03 | `role.@system_teal` | role |
| MAC-DF03 | `role.@system_indigo` | role |
| MAC-DF03 | `role.@system_cyan` | role |
| MAC-DF03 | `role.@system_mint` | role |
| MAC-DF03 | `role.@system_brown` | role |
| MAC-DF03 | `role.@system_gray` | role |
| MAC-DF04 | `role.@under_page_background` | role |
| MAC-DF04 | `role.global.find_highlight` | role |
| MAC-DF04 | `role.@selected_control` | role |
| MAC-DF04 | `role.@selected_control_secondary` | role |
| MAC-DF04 | `role.view.background` | role |
| MAC-DF04 | `role.@text_background` | role |
| MAC-TS01 | `token.text.keyed.large_title` | token |
| MAC-TS01 | `token.text.keyed.title_1` | token |
| MAC-TS01 | `token.text.keyed.title_2` | token |
| MAC-TS01 | `token.text.keyed.title_3` | token |
| MAC-TS01 | `token.text.keyed.headline` | token |
| MAC-TS01 | `token.text.keyed.subheadline` | token |
| MAC-TS01 | `token.text.keyed.body` | token |
| MAC-TS01 | `token.text.keyed.callout` | token |
| MAC-TS01 | `token.text.keyed.footnote` | token |
| MAC-TS01 | `token.text.keyed.caption_1` | token |
| MAC-TS01 | `token.text.keyed.caption_2` | token |
| MAC-TS01 | `token.text.keyed.large_title_emphasized` | token |
| MAC-TS01 | `token.text.keyed.title_1_emphasized` | token |
| MAC-TS01 | `token.text.keyed.title_2_emphasized` | token |
| MAC-TS01 | `token.text.keyed.title_3_emphasized` | token |
| MAC-TS01 | `token.text.keyed.headline_emphasized` | token |
| MAC-TS01 | `token.text.keyed.subheadline_emphasized` | token |
| MAC-TS01 | `token.text.keyed.body_emphasized` | token |
| MAC-TS01 | `token.text.keyed.callout_emphasized` | token |
| MAC-TS01 | `token.text.keyed.footnote_emphasized` | token |
| MAC-TS01 | `token.text.keyed.caption_1_emphasized` | token |
| MAC-TS01 | `token.text.keyed.caption_2_emphasized` | token |
| MAC-TS02 | `token.text.keyed.label` | token |
| MAC-TS02 | `token.text.keyed.message` | token |
| MAC-TS02 | `token.text.keyed.palette` | token |
| MAC-TS02 | `token.text.keyed.menu_bar` | token |
| MAC-TS02 | `token.text.keyed.control_content` | token |
| MAC-TS02 | `token.text.keyed.small_system` | token |
| MAC-LY01 | `layout.values.label_gap, layout.values.button_gap @ size mini, small, regular, large` | rule |
| MAC-LY01 | `layout.values.window_margin (concentric with the window's corners)` | field |
| MAC-IC01 | `token.icon_sizes.keyed.button` | token |
| MAC-IC01 | `token.icon_sizes.keyed.list_row` | token |
| MAC-IC01 | `token.icon_sizes.keyed.tab` | token |
| MAC-IC01 | `token.icon_sizes.toolbar` | token |
| MAC-IC01 | `token.icon_sizes.keyed.status_item` | token |
| MAC-IC01 | `token.icon_sizes.keyed.sidebar` | token |
| MAC-IC01 | `token.icon_sizes.keyed.sidebar @ cond container_size(sidebar, small) / container_size(sidebar, medium) / container_size(sidebar, large)` | rule |
| MAC-IC01 | `button.icon.icon.size @ context toolbar.bar variant tool size small (NSToolbar sizeMode .small)` | rule |
| MAC-IC02 | `token.icon_sizes.keyed.*.weight` | token |
| MAC-IC02 | `token.icon_sizes.keyed.*.scale` | token |
| MAC-IC02 | `token.icon_sizes.keyed.*.rendering_mode` | token |
| MAC-IC03 | - | out (R1.1) |
| MAC-DF05† | `reader:macos:NSColor.textColor` | reader |
| MAC-DF05† | `reader:macos:NSColor.textBackgroundColor` | reader |
| MAC-DF05† | `reader:macos:NSColor.controlColor` | reader |
| MAC-DF05† | `reader:macos:NSColor.controlTextColor` | reader |
| MAC-DF05† | `reader:macos:NSColor.selectedMenuItemTextColor` | reader |
| MAC-DF05† | `reader:macos:NSColor.selectedControlTextColor` | reader |
| MAC-DF05† | `role.@selected_control_text` | role |
| MAC-DF05† | `reader:macos:NSColor.highlightColor` | reader |
| MAC-DF05† | `input.highlight` | input |
| MAC-DF05† | `role.@highlight` | role |
| MAC-DF05† | `reader:macos:NSColor.currentControlTint` | reader |
| MAC-DF05† | `input.accent` | input |
| MAC-TS03† | `token.text.keyed.large_title_tight` | token |
| MAC-TS03† | `token.text.keyed.large_title_loose` | token |
| MAC-TS03† | `token.text.keyed.title_1_tight` | token |
| MAC-TS03† | `token.text.keyed.title_1_loose` | token |
| MAC-TS03† | `token.text.keyed.title_2_tight` | token |
| MAC-TS03† | `token.text.keyed.title_2_loose` | token |
| MAC-TS03† | `token.text.keyed.title_3_tight` | token |
| MAC-TS03† | `token.text.keyed.title_3_loose` | token |
| MAC-TS03† | `token.text.keyed.headline_tight` | token |
| MAC-TS03† | `token.text.keyed.headline_loose` | token |
| MAC-TS03† | `token.text.keyed.subheadline_tight` | token |
| MAC-TS03† | `token.text.keyed.subheadline_loose` | token |
| MAC-TS03† | `token.text.keyed.body_tight` | token |
| MAC-TS03† | `token.text.keyed.body_loose` | token |
| MAC-TS03† | `token.text.keyed.callout_tight` | token |
| MAC-TS03† | `token.text.keyed.callout_loose` | token |
| MAC-TS03† | `token.text.keyed.footnote_tight` | token |
| MAC-TS03† | `token.text.keyed.footnote_loose` | token |
| MAC-TS03† | `token.text.keyed.caption_1_tight` | token |
| MAC-TS03† | `token.text.keyed.caption_1_loose` | token |
| MAC-TS03† | `token.text.keyed.caption_2_tight` | token |
| MAC-TS03† | `token.text.keyed.caption_2_loose` | token |
| MAC-LY02† | `layout.values.form_label_alignment, form_baseline_alignment, label_gap @ variant form` | rule |
| MAC-IC04† | `token.icon_sizes.dialog` | token |
| MAC-IC04† | `reader:macos:NSAlert icon view frame after layout()` | reader |
| MAC-IC05† | `<part>.transitions (PropertyGroup::Icon, TransitionKind::Keyed: bounce, replace, appear, disappear, draw_on, draw_off; ADD-A27, R25.12)` | field |
| MAC-IC05† | `IconStyle.animation (Loop: pulse, variable color, breathe, rotate, wiggle while a state holds; R25.12)` | field |
| MAC-IC05† | `PartStyle.icon.symbol_variants @ {selected} / {checked} (the fill or outline variant a control picks per state, R25.8)` | rule |
| MAC-N01 | `stepper.increment, stepper.decrement, stepper.separator @ {pressed} / {disabled} size mini / small / regular; stepper.values.field_gap, stepper.values.repeat_delay_ms, stepper.values.repeat_interval_ms` | rule |
| MAC-N01 | `reader:macos:the NSStepper metrics the map marks readable` | reader |
| MAC-N02 | `search_field.field.corners, search_field.search_icon, search_field.cancel_button, search_field.recents_indicator, search_field.field.focus, search_field.values.placeholder_alignment @ {focused}; search_field.* @ size mini / small / regular / large` | rule |
| MAC-N02 | `reader:macos:searchButtonBounds, cancelButtonBounds, searchTextBounds of a new NSSearchField` | reader |
| MAC-N03 | `chip.body.fill @ context component.token_field` | rule |
| MAC-N03 | `chip.body.stroke @ context component.token_field` | rule |
| MAC-N03 | `chip.body.corners @ context component.token_field` | rule |
| MAC-N03 | `chip.label.text @ context component.token_field` | rule |
| MAC-N03 | `chip.body.fill @ {selected} context component.token_field` | rule |
| MAC-N03 | `chip.body.* @ variant @rounded / @squared / @plain_squared` | rule |
| MAC-N03 | `chip.menu_arrow.glyph` | field |
| MAC-N03 | `component.token_field.values.chip_gap` | component |
| MAC-N03 | `component.token_field.field` | component |
| MAC-N04 | `date_picker.segment @ variant text_field {selected} / variant @text_field_and_stepper {selected}` | rule |
| MAC-N04 | `stepper.* @ context date_picker variant @text_field_and_stepper` | rule |
| MAC-N04 | `calendar.* @ context date_picker variant inline` | rule |
| MAC-N04 | `calendar.clock_face, calendar.clock_hand` | field |
| MAC-N04 | `calendar.* @ context date_picker.popup variant text_field (the calendar overlay, presentsCalendarOverlay)` | rule |
| MAC-N04 | `date_picker.values.placement (AppKitPopover, to verify; the calendar overlay)` | placement |
| MAC-N05 | `color_button.body, color_button.swatch, color_button.swatch_ring @ {@active}, color_button.chevron, color_button.expand_button @ variant @default / @minimal / @expanded` | rule |
| MAC-N06 | `level_indicator.fill.fill @ {@warning} / {@critical}; level_indicator.block, level_indicator.tick @ variant continuous / discrete / @relevancy; level_indicator.values.thresholds` | rule |
| MAC-N06 | `reader:macos:a new NSLevelIndicator's fillColor, warningFillColor, criticalFillColor` | reader |
| MAC-N06 | `rating.star @ {filled} / {placeholder} (rating style)` | rule |
| MAC-N07 | `breadcrumb.separator.glyph, breadcrumb.crumb_icon, breadcrumb.crumb_label.text.elision, breadcrumb.crumb.fill @ {hover}, breadcrumb.placeholder @ variant @standard / @pop_up` | rule |
| MAC-N08 | `list.disclosure.glyph` | field |
| MAC-N08 | `list.column_separator.stroke` | field |
| MAC-N08 | `list.column_resize_handle` | field |
| MAC-N08 | `list.row_background.fill @ {selected,focus_within}` | rule |
| MAC-N08 | `list.row_background.fill @ {selected} !{focus_within}` | rule |
| MAC-N08 | `list.header` | field |
| MAC-N09 | `component.rule_editor.row.fill @ pos parity odd` | component |
| MAC-N09 | `component.rule_editor.row.fill @ pos parity even` | component |
| MAC-N09 | `component.rule_editor.popup_button` | component |
| MAC-N09 | `component.rule_editor.add_button` | component |
| MAC-N09 | `component.rule_editor.remove_button` | component |
| MAC-N09 | `component.rule_editor.values.indent_per_level` | component |
| MAC-N09 | `component.rule_editor.nested_row_area.padding` | component |
| MAC-N10 | `button.body, button.icon.glyph @ variant help size mini / small / regular {pressed}` | rule |
| MAC-N11 | `badge.body, badge.label @ variant count / circular / @inline` | rule |
| MAC-N12 | `group_box.body @ variant @photo / @gray_bezel / @groove / @button` | rule |
| MAC-N13 | `list.row_background.fill @ {selected,focus_within} variant grid` | rule |
| MAC-N13 | `list.row_background.fill @ variant grid {selected} !{focus_within}` | rule |
| MAC-N13 | `list.row_background.corners @ variant grid` | rule |
| MAC-N13 | `list.row_background.padding @ variant grid` | rule |
| MAC-N14 | `window.* @ variant @hud (material, title bar, close button)` | rule |
| MAC-N15 | `dial.groove, dial.knob_dot, dial.tick @ variant @circular_slider` | rule |
| MAC-N16 | `window.title_bar, window.caption_button, window.title @ variant utility` | rule |
| MAC-N16 | `reader:macos:a utility panel's title-bar height (frame against content rect)` | reader |
| MAC-N17 | `ruler.tick, ruler.label.text, ruler.marker` | field |
| MAC-N18 | `find_bar.bar, find_bar.nav_button, find_bar.done_button, find_bar.values.bar_position; input.* @ context any find_bar` | rule |
| MAC-N19 | `sidebar.pane.fill, sidebar.separator.stroke @ variant end` | rule |
| MAC-N19 | `sidebar.values.{width,collapsed_width} @ variant end` | rule |
| MAC-N19 | `sidebar.pane.{fill,corners,shadow} @ variants {end,floating} (macos-tahoe's floating glass inspector)` | rule |
| MAC-N20 | `menu_bar.values.height, menu_bar.item_label.text, menu_bar.app_name.text, menu_bar.item.fill @ {open}, menu_bar.bar.fill, menu_bar.status_item.icon.size, menu_bar.values.status_item_gap` | field |
| MAC-N20 | `menu_bar.item_label.text, menu_bar.values.height @ cond container_size(menu_bar, large)` | rule |
| MAC-N20 | `reader:macos:NSStatusBar.system.thickness, NSMenu.menuBarHeight, menuBarFontOfSize:` | reader |
| MAC-N21 | `form.card.corners, form.row_separator, form.row.min_height, form.group_title.text, form.group_description.text` | field |
| MAC-N22 | `level_indicator.* @ variant linear / circular` | rule |
| MAC-N23 | `bottom_tab_bar.* @ variant @sidebar_adaptable` | rule |
| MAC-N24 | `pull_to_refresh.indicator, pull_to_refresh.spinner` | field |
| MAC-N25 | - | out (R1.1) |
| MAC-N26 | - | out (R1.1) |
| MAC-N27† | - | out (R1.1) |
| MAC-N28† | `split_button.primary, split_button.secondary, split_button.divider, split_button.arrow @ {pressed} / {open} size mini / small / regular / large` | rule |
| MAC-N29† | `button.body.*, button.menu_indicator @ variant menu {pressed} / variant menu {open}` | rule |
| MAC-N29† | `button.values.menu_indicator_gap` | field |
| MAC-N29† | `button.* @ context toolbar.bar variant menu (NSMenuToolbarItem)` | rule |
| MAC-N30† | `media_controls.* @ variant @inline / @floating / @minimal; media_controls.values.auto_hide_ms` | rule |
| MAC-N31† | `rubber_band.body.fill, rubber_band.body.stroke, rubber_band.body.corners` | field |
| MAC-N32† | `about_dialog.icon.icon.size, about_dialog.name.text, about_dialog.version.text, about_dialog.credits.text, about_dialog.copyright.text` | field |
| MAC-N32† | `about_dialog.values.gap, about_dialog.values.width` | field |
| MAC-N33† | `search_field.* @ context menu` | rule |
| MAC-N33† | `menu.found_item_pointer` | field |
| MAC-N34† | `menu_bar.extra_window.fill (material), menu_bar.extra_window.corners, menu_bar.extra_window.shadow, menu_bar.values.extra_window_gap` | field |
| MAC-N35† | `status_page.icon, status_page.title.text, status_page.description.text, status_page.action_area, status_page.values.icon_to_title @ variant search` | rule |
| MAC-N36† | `teaching_tip.body.fill, teaching_tip.body.corners, teaching_tip.image, teaching_tip.title.text, teaching_tip.message.text, teaching_tip.close_button, teaching_tip.tail @ variant inline / popover` | rule |
| MAC-N36† | `teaching_tip.values.placement (AppKitPopover)` | placement |
| MAC-N37† | `chart.grid_line_major, chart.axis_tick, chart.axis_label.text, chart.values.series_colors, chart.legend_label.text, chart.legend_swatch, chart.selection_mark` | field |
| MAC-X01 | `PartStyle.shadow.layers on window.frame, menu.body, popover.body, button.body, glass parts` | field |
| MAC-X02 | `PartStyle.fill Material identities (vibrancy materials, blur, saturation, tint)` | field |
| MAC-X02 | `declare.materials` | field |
| MAC-X02 | `PartStyle.fill @ cond window_inactive` | rule |
| MAC-X02 | `PartStyle.fill @ cond reduced_transparency (Material.fallback)` | rule |
| MAC-X03 | `PaintLayer.blend BlendMode::Keyed (vibrant blending)` | field |
| MAC-X03 | `role.* under the vibrant appearances (reader)` | role |
| MAC-X03 | `reader:macos:NSColor resolved under NSAppearance vibrantLight and vibrantDark` | reader |
| MAC-X04 | `PartStyle.fill Material identity liquid_glass regular and clear (tint, rim as inset shadow, lensing in the identity)` | field |
| MAC-X04 | `Material.params interactive (ADD-A47)` | field |
| MAC-X04 | `PartStyle.transitions (morphing, press response)` | field |
| MAC-X04 | `declare.widget.*.variants glass` | field |
| MAC-X05 | `scroll_view.edge_effect.fill (Material) @ variant soft_edge / hard_edge` | rule |
| MAC-X06 | `background_extension.extension.fill (Material identity background_extension)` | field |
| MAC-X07 | `PartStyle.corners per-corner Radius, Concentric, CornerCurve::Continuous (S2.4)` | field |
| MAC-X08 | `PartStyle.stroke per-side Edge (S2.4)` | field |
| MAC-X08 | `PartStyle.shadow.layers[inset] (rims, bevels)` | field |
| MAC-X09 | `PaintKind::Linear on PartStyle.fill (S2.3)` | field |
| MAC-X10 | `Color extended sRGB (S2.1, R5.5)` | field |
| MAC-X10 | `reader:macos:NSColor colorSpace read without clipping to sRGB` | reader |
| MAC-X11 | `token.motion.durations.<AppKit durations>` | token |
| MAC-X11 | `token.motion.timings.<curves and springs>` | token |
| MAC-X11 | `PartStyle.transitions and widget presentation transitions` | field |
| MAC-X11 | `PartStyle.transitions @ cond reduced_motion` | rule |
| MAC-X11 | `reader:macos:NSAnimationContext.duration, NSWindow.animationResizeTime, CAMediaTimingFunction control points` | reader |
| MAC-X12 | `token.caret.blink_on_ms / blink_off_ms` | token |
| MAC-X12 | `token.behaviour.double_click_ms / key_repeat_delay_ms / key_repeat_interval_ms / drag_threshold / spring_loading_ms` | token |
| MAC-X12 | `tooltip.values.show_delay_ms` | field |
| MAC-X12 | `menu.values.submenu_open_delay_ms` | field |
| MAC-X12 | `menu.values.activation_flash count` | field |
| MAC-X12 | `scrollbar.values.auto_hide_delay_ms` | field |
| MAC-X12 | `button.values.repeat_delay_ms / repeat_interval_ms` | field |
| MAC-X12 | `reader:macos:NSEvent.doubleClickInterval, keyRepeatDelay, keyRepeatInterval; getPeriodicDelay; NSTextInsertionPointBlinkPeriodOn/Off; NSInitialToolTipDelay` | reader |
| MAC-X13 | `PartStyle.focus FocusIndicator (shape, opacity, transition) per control` | field |
| MAC-X13 | `<widget>.values.focus_visible_when (Never for buttons, checkboxes, segments, links; AnyFocus for text fields and lists; ADD-A16)` | field |
| MAC-X13 | `<widget>.values.focus_visible_when @ cond full_keyboard_access (Keyboard { … } for buttons, checkboxes, segments, links)` | rule |
| MAC-X13 | `cond full_keyboard_access (ADD-A06)` | condition |
| MAC-X13 | `reader:macos:NSApplication.isFullKeyboardAccessEnabled or AppleKeyboardUIMode` | reader |
| MAC-X14 | `token.caret.width / shape` | token |
| MAC-X14 | `token.caret.blink_style Fade (ADD-A24)` | token |
| MAC-X14 | `input.caps_lock_indicator` | field |
| MAC-X14 | `input.dictation_effect` | field |
| MAC-X15 | `role.global.selection_text_background @ cond window_inactive` | rule |
| MAC-X15 | `input.selection.fill @ !{focused}` | rule |
| MAC-X15 | `role.global.find_highlight` | role |
| MAC-X15 | `text_area.find_match @ {selected}` | field |
| MAC-X15 | `role.@foreground_secondary_emphasized` | role |
| MAC-X16 | `PartStyle.{fill,stroke,text} @ cond window_inactive / cond not_main / cond app_inactive (the accent-coloured parts)` | rule |
| MAC-X16 | `cond window_inactive` | condition |
| MAC-X16 | `cond not_main` | condition |
| MAC-X16 | `cond app_inactive` | condition |
| MAC-X17 | `declare.widget.*.sizes (mini … extra_large)` | field |
| MAC-X17 | `declare.widget.*.densities compact` | field |
| MAC-X17 | `Selector.size` | rule |
| MAC-X18 | `TextStyle.line_height / letter_spacing (S2.8)` | field |
| MAC-X18 | `Font.axes FollowsSize, Font.width, Font.features` | field |
| MAC-X18 | `token.text.roles.*` | token |
| MAC-X19 | `PartStyle.icon.fill @ {selected} / context sidebar (accent fills)` | rule |
| MAC-X20 | `PartStyle.cursor per part` | field |
| MAC-X20 | `token.pointer.{size,color} (ADD-A46)` | token |
| MAC-X20 | `reader:macos:pointer accessibility user defaults (keys to verify) -> token.pointer.{size,color}; linkTextAttributes cursor` | reader |
| MAC-X21 | `drag_image.image.opacity` | field |
| MAC-X21 | `drag_image.count_badge` | field |
| MAC-X21 | `role.global.drop_highlight` | role |
| MAC-X21 | `list.row_background.fill @ {drop_target}` | rule |
| MAC-X21 | `list.view.stroke @ {drop_target}` | rule |
| MAC-X22 | `input.<role> per_contrast (InputValue.per_contrast, S9)` | input |
| MAC-X22 | `role.* @ cond contrast_increased` | rule |
| MAC-X22 | `cond contrast_increased` | condition |
| MAC-X22 | `reader:macos:NSColor resolved under accessibilityHighContrastAqua and DarkAqua` | reader |
| MAC-X23 | `PartStyle.fill @ cond reduced_transparency (opaque values)` | rule |
| MAC-X23 | `Material.fallback` | field |
| MAC-X24 | `cond differentiate_without_colour` | condition |
| MAC-X24 | `reader:macos:NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` | reader |
| MAC-X24 | `PartStyle rules @ cond differentiate_without_colour` | rule |
| MAC-X25 | `PartStyle.transitions and presentation transitions @ cond reduced_motion (reduced forms)` | rule |
| MAC-X25 | `cond reduced_motion` | condition |
| MAC-X26 | `cond button_shapes` | condition |
| MAC-X26 | `button.body.{fill,stroke} @ context toolbar.bar variant tool cond button_shapes` | rule |
| MAC-X26 | `reader:macos:the Text size setting (14+; no public API, to verify) -> text_scaling_factor (S9.4; the operand text_scale, ADD-A55)` | reader |
| MAC-X26 | `token.pointer.size (ADD-A46)` | token |
| MAC-X26 | - (Invert colours, which the OS applies) | out (R1.1) |
| MAC-X27 | `cond system_setting:wallpaper_tinting (ADD-A04)` | condition |
| MAC-X27 | `input.accent multicolour (no accent chosen)` | input |
| MAC-X27 | `role.* @ cond system_setting:wallpaper_tinting` | rule |
| MAC-X27 | `input.highlight` | input |
| MAC-X28 | `input.misspelling.text.decoration (Dotted / Wavy)` | field |
| MAC-X28 | `input.grammar_mark.text` | field |
| MAC-X28 | `input.preedit.text (marked text underline)` | field |
| MAC-X28 | `input.prediction.text` | field |
| MAC-X28 | `text_area.writing_tools_glow` | field |
| MAC-X29 | `PartStyle rules @ {checked,hover}, {checked,pressed}, {checked,disabled}, {default,pressed}, {focused,hover} per widget` | rule |
| MAC-X29 | `cond not_main` | condition |
| MAC-X29 | `declare.widget.*.exclusive` | field |
| MAC-X30† | `reader:macos:NSColor.withSystemEffect rollover, pressed, deepPressed, disabled per read colour` | reader |
| MAC-X30† | `PartStyle fill rules @ {hover} / {pressed} / {disabled}` | rule |
| MAC-X31† | - (the OS's Full Keyboard Access highlight) | out (R1.1) |
| MAC-X31† | `window.title_icon @ cond system_setting:title_icons` | rule |
| MAC-X31† | `cond system_setting:title_icons (ADD-A04)` | condition |
| MAC-X31† | `reader:macos:the "Show window title icons" user default (key to verify) -> cond system_setting:title_icons` | reader |
| MAC-X32† | `window.caption_button_glyph.{opacity,transitions} @ context window.caption_buttons {hover}` | rule |
| MAC-X32† | `window.values.presentations event @title_bar_reveal (MAC-W12†)` | field |
| MAC-X32† | `token.theme_change (the light / dark switch: a change of the Scheme condition, S4.3)` | token |
| MAC-X32† | `token.behaviour.scroll.deceleration (momentum)` | token |
| MAC-X32† | `window.values.presentations events @tab_open, @tab_close (part tab), Move (MAC-W09, MAC-SF07†)` | field |
| MAC-X32† | `<part>.transitions (PropertyGroup::Icon, TransitionKind::Keyed) and IconStyle.animation (MAC-IC05†)` | field |
| MAC-X32† | - (window tiling's motion, which the window server draws) | out (R1.1) |
| MAC-X32† | - (SwiftUI NavigationStack push and pop) | out (R1.4) |
| MAC-X33† | - | out (R1.1) |
| MAC-X34† | `PartStyle rules @ dir rtl (per part)` | rule |
| MAC-X34† | `token.icon_mirroring (S2.9)` | token |
| MAC-X35† | - | newest (R1.5) |
| MAC-X36† | - | out (R1.4) |
| MAC-X37† | - | newest (R1.5) |
| MAC-X38† | `Edge.snap (ADD-A33)` | field |
| MAC-SF01† | `text_area.invisible.{glyph,fill} @ variant space / tab / line_break / no_break_space` | rule |
| MAC-SF02† | `rich_text.* @ variant markdown (block parts absent)` | rule |
| MAC-SF03† | `rich_text.inline_code.text, rich_text.inline_code.fill @ cond window_inactive` | rule |
| MAC-SF04† | `link.text.text.{fill,decoration} @ variant inline context rich_text {pressed}` | rule |
| MAC-SF05† | `text_area.values.intent_styles` | field |
| MAC-SF06† | `rich_text.heading_1 … heading_6, rich_text.code_block, rich_text.preformatted, rich_text.block_quote, rich_text.list_marker, rich_text.table @ variant html` | rule |
| MAC-SF06† | `link.* @ variant inline context rich_text variant html` | rule |
| MAC-SF07† | `window.tab_bar.fill @ {drop_target}` | rule |
| MAC-SF07† | `window.values.presentations event Move` | field |
| MAC-SF08† | `dialog.values.placement @ variant @sheet (AppKitSheet)` | placement |
| MAC-SF09† | `dialog.values.placement @ variant separate_window (AppKitWindowCenter)` | placement |
| MAC-SF10† | `dialog.* @ variant @sheet / variant separate_window (SwiftUI's choice: a sheet or a window)` | rule |
| MAC-RL01† | `layout.values.{item_gap,line_gap} @ variant flow` | rule |
| MAC-RL02† | `radio_group.values.row_gap; radio_group.values.column_gap @ variant horizontal` | rule |
| MAC-RL02† | `radio_group.header (part), radio_group.values.header_placement` | field |
| MAC-RL02† | `radio_group.values.* @ size mini / small / regular / large` | rule |
| MAC-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| MAC-FM01† | `reader:macos:CTFontCreateForString on an emoji string` | reader |
| MAC-FM02† | `token.text.fallbacks.{per_family,per_language} (ADD-A41)` | token |
| MAC-FM02† | `reader:macos:CTFontCopyDefaultCascadeListForLanguages` | reader |
| MAC-P3-1† | `window.values.resize_band` | field |
| MAC-P3-1† | `window.resize_zone.cursor per edge variant` | field |
| MAC-P3-2† | `scrollbar.values.trough_click; scrollbar.values.repeat_delay; scrollbar.values.repeat_interval` | field |
| MAC-P3-2† | `reader:macos:the "Click in the scroll bar to" user default -> scrollbar.values.trough_click` | reader |
| MAC-P3-3† | `scrollbar.thumb.{min_width,min_height} @ cond overlay_scrollbars` | rule |
| MAC-P3-3† | `scrollbar.values.overscroll_shrink` | field |
| MAC-P3-4† | `progress_bar.label.text; progress_bar.value_label.text` | field |
| MAC-P3-4† | `progress_bar.values.label_position; progress_bar.values.label_gap` | field |
| MAC-P3-5† | `segmented_control.values.segment_width (Fill default)` | field |
| MAC-P3-5† | `segmented_control.segment.padding` | field |
| MAC-P3-5† | `segmented_control.values.compression` | field |
| MAC-P3-6† | `toolbar_editor.*` | field |
| MAC-P3-6† | `toolbar.values.presentations events @customize, @items_change` | field |
| MAC-P3-7† | `menu.{body,item} @ context input` | rule |
| MAC-P3-7† | `input.values.suggestion_placement` | placement |
| MAC-P3-8† | `input.writing_tools_affordance` | field |
| MAC-P3-9† | `route_button.body.{fill,stroke}` | field |
| MAC-P3-9† | `route_button.glyph.glyph.fill` | field |
| MAC-P3-9† | `route_button.glyph.glyph.fill @ {pressed} / {disabled}` | rule |
| MAC-P3-9† | `route_button.glyph.glyph.fill @ {route_active}` | rule |
| MAC-P3-9† | `route_button flags route_active` | flag |
| MAC-P3-9† | `route_button.glyph.glyph` | field |
| MAC-P3-9† | `route_button.values.bordered` | field |
| MAC-P3-9† | `reader:macos:AVRoutePickerView.routePickerButtonColor(for:) per state -> route_button.glyph.glyph.fill` | reader |
| MAC-P3-9† | - (the device popover; the Picture in Picture window) | out (R1.1) |
| MAC-P3-10† | `navigation_view.values.presentations (PresentationKind::Keyed, ADD-A26) @ variant @stack_history / @stack_book / @horizontal_strip` | rule |
| MAC-P3-10† | `navigation_view.edge_shadow @ variant @stack_history / @stack_book (the shadow under the top page, to verify)` | rule |
| MAC-P3-10† | `navigation_view.values.presentations @ cond reduced_motion` | rule |
| MAC-P3-11† | `input.caret.fill @ cond window_inactive / !{focused} (caret visibility, ADD-A24)` | rule |
| MAC-P3-12† | `token.behaviour.hide_pointer_while_typing` | token |
| MAC-P3-13† | - | out (R1.1) |
| MAC-P3-14† | - | out (R1.1) |
| MAC-P3-15† | - | out (R1.1) |
| MAC-P3-16† | - | out (R1.1) |
| MAC-P3-17† | `token.behaviour.scroll_lines_per_step / scroll distances` | token |
| MAC-P3-17† | `reader:macos:NSScrollView line and page scroll properties` | reader |
| MAC-P4-1† | `menu.* @ context window.title (title pop-up menu)` | rule |
| MAC-P4-1† | `window.values.title_menu_placement (AppKitMenu)` | placement |
| MAC-P4-2† | - | out (R1.1) |
| MAC-P4-3† | - | out (R1.1) |
| MAC-P4-4† | `token.behaviour.scroll smooth wheel step` | token |
| MAC-P4-4† | `reader:macos:user default NSScrollAnimationEnabled` | reader |
| MAC-P4-5† | `window.values.tab_bar_overflow` | field |
| MAC-P4-5† | `window.{tab,tab_title} (narrowed and truncated)` | field |
| MAC-P4-6† | `token.behaviour.double_click_distance` | token |
| MAC-P4-7† | `token.icon_mirroring (S2.9)` | token |
| MAC-P4-7† | `icon lookup by Element.direction and Element.language (S2.9, S8; ADD-B05)` | field |
| MAC-P4-8† | `menu.values.placement (AppKitMenu pull-down: width, edge)` | placement |
| MAC-P4-8† | `combo_box.values.popup_placement (AppKitComboList)` | placement |
| MAC-P4-9† | `token.theme_change (accent and highlight colour change)` | token |
| MAC-P4-10† | `menu.values.submenu_switch (SafeTriangle)` | field |
| MAC-P4-10† | `menu.values.submenu_close_delay_ms` | field |
| MAC-P4-11† | `context_help.body.{fill,stroke,shadow,padding}` | field |
| MAC-P4-11† | `context_help.text.text` | field |
| MAC-P4-11† | `context_help.values.placement (AppKitContextHelp)` | placement |
| MAC-P4-11† | `context_help.values.mode_cursor (CursorShape::WhatsThis, ADD-A59)` | field |
| MAC-P4-12† | `image_cropper.outline, image_cropper.thumb, image_cropper.mask @ {hover} / {pressed}` | rule |
| MAC-P4-13† | `redaction.mask.fill / corners` | field |
| MAC-P4-13† | `redaction flags redacted (StateFlag::Redacted, ADD-A14)` | flag |
| MAC-P4-13† | `redaction.mask.transitions` | field |
