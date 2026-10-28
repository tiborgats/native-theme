# v0.7.1 — Model completeness: Specification, traceability (macOS)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-macos.md`](todo_v0.7.1_model-completeness-gap-map-macos.md) mapped to
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
| MAC-W01 | `window.title_bar.min_height @ variant @no_toolbar / @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W01 | `reader:macos:NSWindow frameRect(forContentRect:styleMask:) and contentLayoutRect` | reader |
| MAC-W02 | `window.caption_buttons.padding` | field |
| MAC-W02 | `window.caption_buttons.padding, window.values.caption_button_gap @ variant @no_toolbar / @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W02 | `window.values.caption_button_gap_px` | field |
| MAC-W02 | `window.caption_button_glyph.{fill,stroke,min_width,min_height}` | field |
| MAC-W02 | `window.caption_button_glyph.fill @ variant close / variant minimize / variant maximize` | rule |
| MAC-W02 | `window.caption_button_glyph.glyph @ variant close / variant minimize / variant maximize` | rule |
| MAC-W02 | `window.caption_button_glyph.glyph.fill @ context window.caption_buttons {hover}` | rule |
| MAC-W02 | `window.caption_button_glyph.fill, window.caption_button_glyph.glyph.fill @ {pressed}` | rule |
| MAC-W02 | `window.caption_button_glyph.fill @ cond window_inactive` | rule |
| MAC-W02 | `window.caption_button_glyph.fill @ {disabled}` | rule |
| MAC-W02 | `window.values.caption_button_capabilities (WhenMissing::Disable: the zoom button of a window that cannot zoom)` | field |
| MAC-W02 | `window.caption_button_glyph.glyph @ variant close context window.frame {@document_edited}` | rule |
| MAC-W02 | `window flags @document_edited` | flag |
| MAC-W02 | `reader:macos:standardWindowButton frames per toolbar style` | reader |
| MAC-W03 | `window.values.title_alignment @ variant @toolbar_unified / @toolbar_unified_compact` | rule |
| MAC-W03 | `window.subtitle.text` | field |
| MAC-W03 | `window.subtitle.text.fill @ cond window_inactive` | rule |
| MAC-W03 | `window.title_icon.icon` | field |
| MAC-W03 | `window.values.title_icon_gap` | field |
| MAC-W03 | `window.title.text.elision` | field |
| MAC-W04 | `window.title_bar.stroke @ variant @separator_line {scrolled}` | rule |
| MAC-W04 | `window.title_bar.shadow @ variant @separator_shadow {scrolled}` | rule |
| MAC-W05 | `window.frame.corners (Continuous)` | field |
| MAC-W05 | `window.frame.corners @ variant utility / @hud / dialog / @no_toolbar / @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W05 | `dialog.body.corners @ variant @sheet` | rule |
| MAC-W05 | `popover.body.corners (the popover window)` | field |
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
| MAC-W09 | `tab.bar.{min_height,fill,stroke} @ variant document context window.title_bar` | rule |
| MAC-W09 | `tab.tab.fill @ variant document context window.title_bar / variant document {selected} context window.title_bar` | rule |
| MAC-W09 | `tab.separator.stroke @ context window.title_bar` | rule |
| MAC-W09 | `tab.label.text @ context window.title_bar / {selected} context window.title_bar` | rule |
| MAC-W09 | `button.body, button.icon @ variant close context tab.tab` | rule |
| MAC-W09 | `tab.values.close_button_shown @ variant document context window.title_bar (OnHover)` | rule |
| MAC-W09 | `button.body, button.icon @ variant add context tab.bar` | rule |
| MAC-W09 | `tab.bar.fill, tab.tab.fill, tab.label.text.fill @ variant document context window.title_bar cond window_inactive` | rule |
| MAC-W09 | `tab.tab.{opacity,shadow} @ {dragged} context window.title_bar` | rule |
| MAC-W10 | `window.title_bar.fill (titlebar material) @ variant @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W11† | `window.title_chevron.{glyph,opacity}` | field |
| MAC-W11† | `window.title_chevron.{opacity,transitions} @ {hover}` (the `title` element's hover; the chevron is queried with it) | rule |
| MAC-W11† | `window.title_status.text @ {@document_edited}` | rule |
| MAC-W11† | `popover.body, popover.arrow @ context window.title (the rename and location popover)` | rule |
| MAC-W12† | `window.title_bar.fill (fullScreenUI material) @ cond full_screen` | rule |
| MAC-W12† | `window.values.full_screen_title_bar_reveal (PointerAtTopEdge)` | field |
| MAC-W12† | `window.values.presentations event @title_bar_reveal` | field |
| MAC-W12† | `window.frame.{corners,stroke} @ cond full_screen` | rule |
| MAC-W12† | - (the menu bar coming in with the reveal; the divider between two windows in a full-screen Split View) | out (R1.1) |
| MAC-W13† | `window.values.caption_button_menus (the zoom button's tiling menu)` | field |
| MAC-W13† | `menu.* @ context window.caption_button variant maximize` | rule |
| MAC-W13† | `window.values.caption_button_menu_placement` | placement |
| MAC-W13† | `window.frame.corners @ cond tiled:left / tiled:right / tiled:top / tiled:bottom` | rule |
| MAC-W13† | - (the drag-to-edge tiling preview; the margins between tiled windows) | out (R1.1) |
| MAC-W14† | `window.title_bar.{fill,stroke} @ variant transparent_title_bar` | rule |
| MAC-W14† | `window.title @ variant @title_hidden` (`absent = true`: the title goes, the bar and its traffic lights stay) | rule |
| MAC-W14† | `window.title_bar.{fill,stroke,min_height}, window.values.title_alignment @ variant @no_toolbar / @toolbar_unified / @toolbar_unified_compact / @toolbar_expanded / @toolbar_preference` | rule |
| MAC-W15† | `window.title.text.fill @ cond window_inactive` | rule |
| MAC-B01 | `button.body.*, button.label.* @ variant @flexible_push / @small_square / tool / @accessory_bar / @accessory_bar_action / @badge / circular / help / glass / @recessed / @round_rect / @inline / @textured_rounded / @shadowless_square / flat (borderless)` | rule |
| MAC-B01 | `expander.header, expander.arrow @ variant @push_disclosure; expander.arrow (the disclosure bezel, MAC-EX01)` | rule |
| MAC-B01 | `button.body.*, button.label.* @ {pressed} / {disabled} / {checked} per bezel variant` | rule |
| MAC-B02 | `button.body.fill, button.label.text.fill @ {default} !cond window_inactive !cond app_inactive` | rule |
| MAC-B02 | `button.body.fill, button.label.text.fill @ {default} cond window_inactive / {default} cond app_inactive (a plain button)` | rule |
| MAC-B02 | `button.body.fill, button.label.text.fill @ {default,pressed} / {default,disabled} / {default,hover}` | rule |
| MAC-B03 | `button.body.fill, button.label.text.fill @ {default,hover} / {default,pressed} / {default,disabled}` | rule |
| MAC-B04 | `button.body.shadow (layers; macos-monterey)` | field |
| MAC-B05 | `button.body.fill (Linear gradient layer)` | field |
| MAC-B05 | `button.body.shadow (inset highlight layer for the bevel)` | field |
| MAC-B06 | `button.label.text.fill @ variant destructive (and context any dialog where it differs)` | rule |
| MAC-B07 | `button.body.fill, button.label.text.fill @ variant tinted / variant primary / variants {glass,tinted}` | rule |
| MAC-B07 | `button.body.fill, button.icon.icon.fill, button.label.text.fill` as expressions on `element.color` (bezelColor, contentTintColor; the parts declare it, `data = { body = ["color"], label = ["color"], icon = ["color"] }`; R25.3, ADD-B16) | field |
| MAC-B08 | `button.body.corners @ variant pill / @rounded_rectangle / circular` | rule |
| MAC-B08 | `button.body.corners @ size small / regular / large / extra_large (automatic shape)` | rule |
| MAC-B09 | `button.icon.icon.size @ size mini / small / regular / large / extra_large` | rule |
| MAC-B09 | `button.icon.icon.fill @ {pressed} / {disabled} / {default}` | rule |
| MAC-B09 | `button.values.icon_placement` | field |
| MAC-B09 | `button.body.padding @ variant icon_only` | rule |
| MAC-B10 | `button.body.fill @ {checked,pressed} / {checked,disabled} variant toggle` | rule |
| MAC-B10 | `button.body.fill @ {checked} variant toggle cond window_inactive` | rule |
| MAC-B11† | `button.body.min_width @ size mini / small / regular / large / extra_large` | rule |
| MAC-B11† | `reader:macos:intrinsicContentSize of a push button with an empty and a one-character title, per control size -> button.body.min_width` | reader |
| MAC-I01 | `input.body @ variant @square_bezel / @rounded_bezel / line / flat` | rule |
| MAC-I01 | `input.body, input.text.text @ context list.cell {editing} (the field editor in a cell)` | rule |
| MAC-I02 | `input.{body.fill,body.stroke,text.text.fill} @ {read_only}` | rule |
| MAC-I03 | `input.values.mask_character @ variant secure` | rule |
| MAC-I03 | `input.caps_lock_indicator.{glyph,padding} @ variant secure` | rule |
| MAC-I04 | `input.body.shadow (inset edge layers; outer ring layer)` | field |
| MAC-I04 | `input.body.stroke` | field |
| MAC-I05 | `input.selection.{fill,text.fill} @ context input.body !{focused} / cond window_inactive (= the two unfocused roles)` | rule |
| MAC-I05 | `text_area.selection.{fill,text.fill} @ context text_area.body !{focused} / cond window_inactive (= the two unfocused roles)` | rule |
| MAC-I05 | `role.global.selection_text_background_unfocused` | role |
| MAC-I05 | `role.global.selection_text_foreground_unfocused` | role |
| MAC-I05 | `reader:macos:NSColor.unemphasizedSelectedTextBackgroundColor, unemphasizedSelectedTextColor -> role.global.selection_text_background_unfocused, role.global.selection_text_foreground_unfocused` | reader |
| MAC-I06 | `input.prediction.text; text_area.prediction.text` | field |
| MAC-I06 | `autocomplete.body @ context text_area (NSTextView's completions)` | rule |
| MAC-I06 | `menu.body, menu.item, menu.item_label.text @ context autocomplete (the completions list)` | rule |
| MAC-I07 | `text_area.text.text.font` | field |
| MAC-I07 | `reader:macos:NSFont.userFont(ofSize:0) -> text_area.text.text.font; NSFont.userFixedPitchFont(ofSize:0) -> token.text.roles.monospace` | reader |
| MAC-I08 | `scroll_view.frame @ variant line / @bezel / @groove / flat` | rule |
| MAC-I09† | `input.autofill_affordance.{icon,fill,corners,padding}` | field |
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
| MAC-C06 | `checkbox.mark.glyph.{source,size,stroke_width} (path and extent; measured)` | field |
| MAC-C06 | `checkbox.mark.padding` (the mark's inset inside the box; measured) | field |
| MAC-C07 | `radio.ring.fill @ {pressed}; radio.dot.fill @ {checked,disabled}` | rule |
| MAC-C07 | `radio.ring.*, radio.dot.* @ cond window_inactive` | rule |
| MAC-C07 | `radio.dot.glyph = SystemControl(AppKitRadio)` | system-control |
| MAC-MN01 | `menu.item.corners` | field |
| MAC-MN01 | `menu.body.padding (highlight inset)` | field |
| MAC-MN02 | `menu.body.padding (top, bottom)` | field |
| MAC-MN03 | `menu.separator.{stroke,padding,min_height} (the line's colour and width as its stroke; its insets as padding; the row's height as min_height)` | field |
| MAC-MN04 | `menu.accelerator.text` | field |
| MAC-MN04 | `menu.accelerator.text.fill @ {hover} / {disabled}` | rule |
| MAC-MN04 | `menu.values.{accelerator_gap_px,accelerator_alignment}` | field |
| MAC-MN05 | `menu.check_indicator.glyph @ {checked} / {indeterminate} (NamedSystemImage)` | rule |
| MAC-MN05 | `menu.check_indicator.glyph.fill @ {hover} / {disabled}` | rule |
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
| MAC-MN12 | `menu.scroll_arrow.{glyph,fill,min_height}` | field |
| MAC-MN13 | `menu.item @ variant palette` | rule |
| MAC-MN13 | `menu.item.stroke @ variant palette {checked} (selection ring)` | rule |
| MAC-MN14 | `menu.body.{stroke,shadow,fill}` | field |
| MAC-MN15 | `combo_box.values.popup_placement (AppKitMenu, over_selected_item)` | placement |
| MAC-MN15 | `menu.item_label.text @ context combo_box.body[size mini] / context combo_box.body[size small] / context combo_box.body[size large]` (the pop-up button's control size, `ContextMatch.sizes`, ADD-A84) | rule |
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
| MAC-T06† | `tooltip.values.show_delay_ms @ variant @expansion` | rule |
| MAC-T06† | `tooltip.values.placement @ variant @expansion (AppKitExpansionTip)` | placement |
| MAC-S01 | `scrollbar.thumb.{min_width,min_height} @ cond overlay_scrollbars context scrollbar {hover}` | rule |
| MAC-S01 | `scrollbar.thumb.transitions (Geometry: the knob's expansion)` | field |
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
| MAC-SL07 | `slider.thumb.{fill,transform} @ {pressed} / {dragged}` | rule |
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
| MAC-TB06† | `tab.separator.stroke @ prev tab.tab{selected} / next tab.tab{selected} (absent beside the selected tab)` | rule |
| MAC-TB07† | `tab.label.text.elision, tab.values.overflow` | field |
| MAC-SB01 | `list.row.min_height, list.leading_icon.icon.size, list.row.text @ variant sidebar cond container_size(sidebar, small) / container_size(sidebar, medium) / container_size(sidebar, large)` | rule |
| MAC-SB01 | `reader:macos:effectiveRowSizeStyle, rowHeight, NSTableViewDefaultSizeMode` | reader |
| MAC-SB01 | `cond container_size(sidebar)` | condition |
| MAC-SB02 | `list.leading_icon.icon.fill @ variant sidebar / variant sidebar {selected} / variant sidebar {selected} context list.view !{focus_within} / variant sidebar {selected} cond window_inactive` | rule |
| MAC-SB03 | `list.section_header.{text,min_height} @ variant sidebar` | rule |
| MAC-SB03 | `list.disclosure @ context list.section_header {hover} variant sidebar` | rule |
| MAC-SB04 | `list.row_background.corners, list.values.row_background_inset @ variant sidebar` | rule |
| MAC-SB04 | `list.row_background.corners @ variant sidebar {selected} prev list.row{selected} / variant sidebar {selected} next list.row{selected} (ADD-B07)` | rule |
| MAC-SB05 | `list.row_background.fill, list.row.text.fill @ variant sidebar {selected} context list.view !{focus_within}` | rule |
| MAC-SB05 | `list.row_background.fill, list.row.text.fill @ variant sidebar {selected} cond window_inactive` | rule |
| MAC-SB05 | `reader:macos:unemphasizedSelectedContentBackgroundColor` | reader |
| MAC-SB06 | `sidebar.pane.{fill,corners,shadow} @ variant floating (material fill, Concentric corners)` | rule |
| MAC-SB06 | `sidebar.values.floating_inset` | field |
| MAC-SB06 | `background_extension.extension.fill @ context sidebar.pane variant floating (the content extending under it)` | rule |
| MAC-SB06 | `reader:macos:effectiveCornerRadii` | reader |
| MAC-SB07 | `sidebar.pane.fill @ cond window_inactive` | rule |
| MAC-SB08 | `list.disclosure, list.values.indent_per_level @ variant sidebar` | rule |
| MAC-SB08 | `reader:macos:NSOutlineView.indentationPerLevel` | reader |
| MAC-SB09† | `sidebar.values.width_px; sidebar.pane.min_width, sidebar.pane.max_width` | field |
| MAC-SB09† | `sidebar.values.breakpoints (the collapse rule)` | field |
| MAC-SB09† | `sidebar.values.presentations event @collapse` | field |
| MAC-SB09† | `reader:macos:NSSplitViewItem minimumThickness, maximumThickness` | reader |
| MAC-TL01 | `toolbar.bar.min_height, toolbar.bar.padding @ variant @unified / @unified_compact / @expanded / @preference` | rule |
| MAC-TL01 | `reader:macos:contentLayoutRect per toolbar style` | reader |
| MAC-TL02 | `button.body.fill @ variant tool context toolbar.bar / {hover} variant tool context toolbar.bar / {pressed} variant tool context toolbar.bar / {disabled} variant tool context toolbar.bar` | rule |
| MAC-TL02 | `button.icon.icon.fill, button.label.text @ variant tool context toolbar.bar / {hover} variant tool context toolbar.bar / {pressed} variant tool context toolbar.bar / {disabled} variant tool context toolbar.bar` | rule |
| MAC-TL02 | `button.values.icon_text_gap, button.values.label_position (ButtonLabelPosition; the icon-and-label display mode) @ variant tool context toolbar.bar` | rule |
| MAC-TL03 | `toolbar.item_group.{fill,padding,min_height,corners} (material fill)` | field |
| MAC-TL03 | `toolbar.values.group_gap` | field |
| MAC-TL03 | `segmented_control.* @ context toolbar.bar (NSToolbarItemGroup's segmented look)` | rule |
| MAC-TL04 | `button.body.fill @ variant primary context toolbar.bar (the accent tint)` | rule |
| MAC-TL04 | `button.body.fill @ variant primary context toolbar.bar` as an expression on `element.color` (backgroundTintColor; `button.body` declares `color`; R25.3, ADD-B16) | rule |
| MAC-TL05 | `badge.* @ context toolbar.bar` | rule |
| MAC-TL06 | `button.body, button.icon @ variant overflow context toolbar.bar` | rule |
| MAC-TL06 | `menu.* @ context button variant overflow` | rule |
| MAC-TL06 | `button.values.menu_placement @ variant overflow context toolbar.bar` | placement |
| MAC-TL07 | `button.body, button.icon @ variant tool context toolbar.bar (the collapsed search item)` | rule |
| MAC-TL07 | `search_field.*, input.* @ context toolbar.bar (the expanded field)` | rule |
| MAC-TL07 | `search_field.values.presentations event @expand` (the one home of the expand motion, not the toolbar's) | field |
| MAC-TL08 | `toolbar.separator @ variant @tracking` | rule |
| MAC-TL09 | `toolbar.bar.fill` (the material on the 12–15 generation; `Absent` on `macos-tahoe`, whose bar is transparent, the scroll edge effect beneath it drawing the material once content lies under it, MAC-X05) | field |
| MAC-TL09 | `scroll_view.edge_effect.fill @ variant top {scrolled} (under the toolbar, MAC-X05)` | rule |
| MAC-TL10† | `toolbar.values.fixed_space_width, flexible_space_min_width` | field |
| MAC-TL11† | `button.body.fill, button.icon.icon.fill, button.label.text.fill @ {selected} variant tool context toolbar.bar` | rule |
| MAC-TL11† | `button.body.fill @ {selected} variant tool context toolbar.bar cond window_inactive` | rule |
| MAC-ST01 | `status_bar.bar.min_height, status_bar.bar.fill, status_bar.bar.stroke.top` | field |
| MAC-ST01 | `status_bar.bar @ cond window_inactive` | rule |
| MAC-L01 | `list.values.row_background_inset, list.row_background.corners, list.header.padding @ variant @full_width / inset / sidebar / plain` (the automatic style is the base) | rule |
| MAC-L01 | `list.values.section_inset @ variant @full_width / inset / sidebar / plain` (the automatic style is the base) | rule |
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
| MAC-L06 | `list.row_separator.stroke @ context list.view variant row_separators / context list.view variant @dashed_horizontal_grid (dash, ADD-A32; the separator is an element, the variants the list's)` | rule |
| MAC-L06 | `list.column_separator.stroke @ variant column_separators` | rule |
| MAC-L07 | `list.values.intercell_gap` | field |
| MAC-L07 | `reader:macos:NSTableView.intercellSpacing` | reader |
| MAC-L08 | `list.row.min_height @ variant @row_size_small / @row_size_medium / @row_size_large` | rule |
| MAC-L08 | `reader:macos:rowHeight per rowSizeStyle` | reader |
| MAC-L09 | `list.section_header.{text,fill,min_height} @ variant table (material fill)` | rule |
| MAC-L09 | `list.values.section_headers_sticky (floating group rows)` | field |
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
| MAC-SP01 | `splitter.divider @ variant @thin / variant wide / variants {wide,@pane_splitter}` | rule |
| MAC-SP01 | `reader:macos:NSSplitView.dividerThickness per style` | reader |
| MAC-SP02 | `splitter.grip @ variant wide / variants {wide,@pane_splitter}` (the thick divider is the typed `wide`; the pane splitter adds only its difference) | rule |
| MAC-SP03 | `splitter.divider @ {dragged}` | rule |
| MAC-SP04 | `splitter.divider @ variant sidebar (fill and stroke absent)` | rule |
| MAC-SW01 | `switch.thumb.min_width, switch.thumb.min_height, switch.thumb.corners (Full)` | field |
| MAC-SW02 | `switch.thumb.shadow (layers)` | field |
| MAC-SW02 | `switch.thumb.stroke (the rim)` | field |
| MAC-SW03 | `switch.thumb.min_width, switch.thumb.fill, switch.track.fill @ {pressed}` | rule |
| MAC-SW04 | `switch.track.fill @ {checked} cond window_inactive` | rule |
| MAC-SW05 | `switch.track.min_width, switch.track.min_height, switch.thumb.min_width, switch.thumb.min_height @ size mini / small / regular / large` | rule |
| MAC-SW05 | `declare.widget.switch.sizes (mini, small, regular, large)` | field |
| MAC-SW05 | `reader:macos:intrinsicContentSize of an NSSwitch per control size -> switch sizes` | reader |
| MAC-SW06 | `switch.on_glyph.glyph, switch.off_glyph.glyph @ cond differentiate_without_color (to verify)` | rule |
| MAC-D01 | `dialog.values.{title_alignment,message_alignment,action_layout} @ variant alert` | rule |
| MAC-D01 | `dialog.body.min_width @ variant alert (fixed width)` | rule |
| MAC-D01 | `dialog.message.text @ variant alert` | rule |
| MAC-D01 | `checkbox.* @ context dialog.body variant alert (suppression check box)` | rule |
| MAC-D01 | `button.* @ variant help context dialog.body variant alert (the help button)` | rule |
| MAC-D01 | `reader:macos:NSAlert.layout(), then the window's and subviews' frames -> dialog alert width and layout` | reader |
| MAC-D02 | `dialog.icon_badge.{icon,padding}` | field |
| MAC-D02 | `dialog.icon_badge.{icon,padding} @ variant error` | rule |
| MAC-D03 | `dialog.body.{corners,fill,shadow} @ variant @sheet` | rule |
| MAC-D03 | `dialog.scrim @ variant @sheet` | rule |
| MAC-D03 | `dialog.values.window_margin @ variant @sheet (macos-tahoe's floating inset sheet)` | rule |
| MAC-D04 | `dialog.values.presentations events Appear, Disappear` | field |
| MAC-D04 | `dialog.values.presentations events Appear, Disappear @ variant @sheet` (the sheet attaching to and detaching from its window) | rule |
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
| MAC-CB04 | `menu.body, menu.item.{min_height,fill} @ context combo_box variant editable (NSComboBox's list)` | rule |
| MAC-CB04 | `menu.item.fill @ {selected} context combo_box variant editable (its highlight)` | rule |
| MAC-CB04 | `combo_box.values.popup_placement @ variant editable (its max_visible_items: numberOfVisibleItems)` | placement |
| MAC-CB04 | `scrollbar.* @ context combo_box variant editable` | rule |
| MAC-CB05 | `combo_box.body.{fill,stroke,shadow} @ variant flat` | rule |
| MAC-CB06 | `combo_box.body.fill @ cond window_inactive` | rule |
| MAC-SG01 | `segmented_control.separator.stroke` | field |
| MAC-SG01 | `segmented_control.separator.opacity @ prev segmented_control.segment{selected} / next segmented_control.segment{selected}` | rule |
| MAC-SG02 | `segmented_control.indicator.shadow (macos-monterey)` | field |
| MAC-SG03 | `segmented_control.label.text.font.weight, segmented_control.label.text.fill @ {selected}` | rule |
| MAC-SG03 | `segmented_control.indicator.fill (macos-tahoe: the accent)` | field |
| MAC-SG04 | `segmented_control.track.*, segmented_control.segment.* @ variant @rounded / @round_rect / @textured_rounded / pill / @textured_square / @small_square / @separated / @rounded_rectangle` | rule |
| MAC-SG05 | `segmented_control.* @ variant @select_any / momentary` | rule |
| MAC-SG05 | `segmented_control.indicator.fill (selectedSegmentBezelColor default)` | field |
| MAC-SG05 | `segmented_control.indicator.fill` as an expression on `element.color` (an application's selectedSegmentBezelColor; `data = { indicator = ["color"] }`; R25.3, ADD-B16) | field |
| MAC-SG05 | `reader:macos:a new NSSegmentedControl's selectedSegmentBezelColor -> segmented_control.indicator.fill` | reader |
| MAC-SG06 | `segmented_control.segment.fill @ {pressed}; segmented_control.indicator.fill @ {selected,pressed}` | rule |
| MAC-SG06 | `segmented_control.segment.opacity, segmented_control.label.text.fill @ {disabled} (per segment)` | rule |
| MAC-SG06 | `segmented_control.indicator.fill, segmented_control.label.text.fill @ {selected} cond window_inactive` | rule |
| MAC-SG07 | `segmented_control.menu_indicator.glyph @ variant menu` | rule |
| MAC-SG08 | `segmented_control.segment.corners @ pos first / pos last / pos only (outer corners only)` | rule |
| MAC-SG08 | `segmented_control.indicator.corners (Concentric)` | field |
| MAC-SG09 | `segmented_control.values.presentations event @indicator_move (part indicator; macos-tahoe)` | field |
| MAC-SG10† | `segmented_control.icon.icon.size @ size mini / small / regular / large` | rule |
| MAC-SG10† | `segmented_control.values.image_scaling` | field |
| MAC-SG10† | `segmented_control.icon.icon.fill @ {selected} / {pressed} / {disabled}` | rule |
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
| MAC-LK01 | `link.text.text.decoration (style, paint, thickness, offset) @ variant inline` (the default link attributes, a single underline; `NSUnderlineStyle`'s dash-dot, dash-dot-dot and by-word patterns have no `DecorationStyle`, which only a reader of a non-default value would meet) | rule |
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
| MAC-DF01 | `reader:macos:NSColor.tertiaryLabelColor, quaternaryLabelColor, quinaryLabelColor` | reader |
| MAC-DF02 | `role.@system_fill` | role |
| MAC-DF02 | `role.@system_fill_secondary` | role |
| MAC-DF02 | `role.@system_fill_tertiary` | role |
| MAC-DF02 | `role.@system_fill_quaternary` | role |
| MAC-DF02 | `role.@system_fill_quinary` | role |
| MAC-DF02 | `reader:macos:NSColor.systemFillColor, secondarySystemFillColor, tertiarySystemFillColor, quaternarySystemFillColor, quinarySystemFillColor` | reader |
| MAC-DF03 | `role.@system_yellow` | role |
| MAC-DF03 | `role.@system_purple` | role |
| MAC-DF03 | `role.@system_pink` | role |
| MAC-DF03 | `role.@system_teal` | role |
| MAC-DF03 | `role.@system_indigo` | role |
| MAC-DF03 | `role.@system_cyan` | role |
| MAC-DF03 | `role.@system_mint` | role |
| MAC-DF03 | `role.@system_brown` | role |
| MAC-DF03 | `role.@system_gray` | role |
| MAC-DF03 | `reader:macos:NSColor.systemYellow, systemPurple, systemPink, systemTeal, systemIndigo, systemCyan, systemMint, systemBrown, systemGray` | reader |
| MAC-DF04 | `role.@under_page_background` | role |
| MAC-DF04 | `role.global.find_highlight` | role |
| MAC-DF04 | `role.@selected_control` | role |
| MAC-DF04 | `role.@selected_control_secondary` | role |
| MAC-DF04 | `role.view.background` | role |
| MAC-DF04 | `role.@text_background` | role |
| MAC-DF04 | `reader:macos:NSColor.underPageBackgroundColor, findHighlightColor, selectedControlColor, secondarySelectedControlColor, controlBackgroundColor` | reader |
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
| MAC-LY01 | `layout.values.label_gap, layout.values.button_gap @ size mini / small / regular / large` | rule |
| MAC-LY01 | `layout.values.window_margin (concentric with the window's corners)` | field |
| MAC-IC01 | `token.icons.sizes.keyed.button` | token |
| MAC-IC01 | `token.icons.sizes.keyed.list_row` | token |
| MAC-IC01 | `token.icons.sizes.keyed.tab` | token |
| MAC-IC01 | `token.icons.sizes.toolbar` | token |
| MAC-IC01 | `token.icons.sizes.keyed.status_item` | token |
| MAC-IC01 | `token.icons.sizes.keyed.sidebar` | token |
| MAC-IC01 | `token.icons.sizes.keyed.sidebar @ cond container_size(sidebar, small) / container_size(sidebar, medium) / container_size(sidebar, large)` | rule |
| MAC-IC01 | `button.icon.icon.size @ variant tool size small context toolbar.bar (NSToolbar sizeMode .small)` | rule |
| MAC-IC02 | `token.icons.sizes.keyed.*.weight` | token |
| MAC-IC02 | `token.icons.sizes.keyed.*.scale` | token |
| MAC-IC02 | `token.icons.sizes.keyed.*.rendering_mode` | token |
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
| MAC-LY02† | `layout.values.form_label_alignment, layout.values.form_baseline_alignment @ variant form` | rule |
| MAC-LY02† | `layout.values.label_gap @ variant form size mini / small / regular / large` | rule |
| MAC-IC04† | `token.icons.sizes.dialog` | token |
| MAC-IC04† | `reader:macos:NSAlert icon view frame after layout()` | reader |
| MAC-IC05† | `<part>.transitions (PropertyGroup::Icon, TransitionKind::Keyed: bounce, replace, appear, disappear, draw_on, draw_off; ADD-A27, R25.12)` | field |
| MAC-IC05† | `IconStyle.animation (Loop: pulse, variable color, breathe, rotate, wiggle while a state holds; R25.12)` | field |
| MAC-IC05† | `PartStyle.icon.symbol_variants @ {selected} / {checked} (the fill or outline variant a control picks per state, R25.8)` | rule |
| MAC-N01 | `stepper.increment.fill, stepper.decrement.fill @ {pressed} / {disabled}` | rule |
| MAC-N01 | `stepper.increment_glyph.glyph, stepper.decrement_glyph.glyph` | field |
| MAC-N01 | `stepper.separator.stroke` | field |
| MAC-N01 | `stepper.body.fill (macos-tahoe's translucent body)` | field |
| MAC-N01 | `stepper.body, stepper.increment, stepper.decrement @ size mini / small / regular` | rule |
| MAC-N01 | `stepper.values.field_gap, stepper.values.repeat_delay_ms, stepper.values.repeat_interval_ms` | field |
| MAC-N01 | `reader:macos:the NSStepper metrics the map marks readable` | reader |
| MAC-N02 | `input.body.{fill,stroke,corners,focus} @ context any search_field (the bezel and its focus ring)` | rule |
| MAC-N02 | `input.icon.icon @ variant leading context any search_field (the magnifier)` | rule |
| MAC-N02 | `button.* @ variant clear_field context any search_field (the cancel button)` | rule |
| MAC-N02 | `search_field.recents_indicator.glyph` | field |
| MAC-N02 | `input.placeholder.{text,transitions} @ context any search_field` | rule |
| MAC-N02 | `search_field.values.placeholder_alignment @ {focus_within}` | rule |
| MAC-N02 | `input.* @ context any search_field size mini / small / regular / large` | rule |
| MAC-N02 | `reader:macos:searchButtonBounds, cancelButtonBounds, searchTextBounds of a new NSSearchField` | reader |
| MAC-N03 | `chip.body.fill @ context any input` (the tokens: `chip` elements in a flow `layout` in the field, the structure Windows' token field has) | rule |
| MAC-N03 | `chip.body.stroke @ context any input` | rule |
| MAC-N03 | `chip.body.corners @ context any input` | rule |
| MAC-N03 | `chip.label.text @ context any input` | rule |
| MAC-N03 | `chip.body.fill @ {selected} context any input` | rule |
| MAC-N03 | `chip.body.* @ variant @rounded / @squared / @plain_squared` (the default style is the base) | rule |
| MAC-N03 | `chip.menu_arrow.glyph` | field |
| MAC-N03 | `layout.values.item_gap_px @ variant flow context any input` (the token spacing) | rule |
| MAC-N03 | `input.{body,text,caret}.*` (the token field's frame, text and caret are the `input`'s) | rule |
| MAC-N04 | `date_picker.segment.{fill,text} @ {selected} context date_picker.field variant text_field / {selected} context date_picker.field variant @text_field_and_stepper` | rule |
| MAC-N04 | `stepper.* @ context date_picker variant @text_field_and_stepper` | rule |
| MAC-N04 | `calendar.header, calendar.title.text @ context any date_picker variant inline (the month header)` | rule |
| MAC-N04 | `button.* @ variant previous / next context calendar.header` | rule |
| MAC-N04 | `calendar.weekday_label.text, calendar.day, calendar.day_label.text @ context any date_picker variant inline` | rule |
| MAC-N04 | `calendar.day.fill, calendar.day_label.text.fill @ {selected} / {today} / {other_month}` | rule |
| MAC-N04 | `calendar.today_marker.{stroke,corners}` | field |
| MAC-N04 | `calendar.clock_face.{fill,stroke}, calendar.clock_hand.stroke` | field |
| MAC-N04 | `calendar.* @ context date_picker.popup (the calendar overlay, presentsCalendarOverlay)` | rule |
| MAC-N04 | `date_picker.popup.{fill,corners,shadow,padding} @ variant text_field` | rule |
| MAC-N04 | `date_picker.values.placement (AppKitPopover, to verify; the calendar overlay)` | placement |
| MAC-N05 | `color_button.body.{fill,stroke,corners}, color_button.swatch.{corners,stroke,min_width,min_height} @ variant @minimal / @expanded` (the default style is the base) | rule |
| MAC-N05 | `color_button.swatch_ring.stroke @ {@active}` | rule |
| MAC-N05 | `color_button.chevron.glyph @ variant @minimal` | rule |
| MAC-N05 | `color_button.expand_button.{fill,glyph} @ variant @expanded` | rule |
| MAC-N05 | `color_button.{body,swatch}.* @ variant @hdr` (the HDR well, `maximumLinearExposure` above one) | rule |
| MAC-N06 | `level_indicator.fill.fill, level_indicator.block.fill @ {@warning} / {@critical}; level_indicator.block.fill @ {filled} / !{filled} / {filled,@warning} variant @tiered / {filled,@critical} variant @tiered` (the block an element, tiered blocks each in their own tier) | rule |
| MAC-N06 | `level_indicator.block, level_indicator.fill @ variant continuous / discrete / @relevancy` | rule |
| MAC-N06 | `level_indicator.tick.fill` | field |
| MAC-N06 | `level_indicator.values.thresholds` | field |
| MAC-N06 | `reader:macos:a new NSLevelIndicator's fillColor, warningFillColor, criticalFillColor` | reader |
| MAC-N06 | `rating.star.glyph @ {filled} / {placeholder} (the rating style's stars and placeholders)` | rule |
| MAC-N07 | `breadcrumb.separator.glyph` | field |
| MAC-N07 | `breadcrumb.crumb_icon.icon` | field |
| MAC-N07 | `breadcrumb.crumb_label.text.elision` | field |
| MAC-N07 | `breadcrumb.crumb.fill @ {hover}` | rule |
| MAC-N07 | `breadcrumb.placeholder.text` | field |
| MAC-N07 | `breadcrumb.* @ variant @standard / @pop_up` | rule |
| MAC-N08 | `list.disclosure.glyph (the branch arrows)` | field |
| MAC-N08 | `list.column_separator.stroke` | field |
| MAC-N08 | `list.column_resize_handle.glyph` | field |
| MAC-N08 | `list.row_background.fill @ {selected} context list.view {focus_within}` | rule |
| MAC-N08 | `list.row_background.fill @ {selected} context list.view !{focus_within}` | rule |
| MAC-N08 | `list.header (the title row)` | field |
| MAC-N09 | `component.rule_editor.row.fill @ pos parity odd` | component |
| MAC-N09 | `component.rule_editor.row.fill @ pos parity even` | component |
| MAC-N09 | `component.rule_editor.row.{padding,min_height}` | component |
| MAC-N09 | `combo_box.* @ context component.rule_editor (the criteria pop-ups)` | rule |
| MAC-N09 | `button.* @ variant circular context component.rule_editor (the round + and − buttons)` | rule |
| MAC-N09 | `component.rule_editor.values.indent_per_level` | component |
| MAC-N09 | `component.rule_editor.nested_row_area.padding` | component |
| MAC-N10 | `button.body.{fill,stroke,corners,min_width,min_height}, button.icon.glyph @ variant help` | rule |
| MAC-N10 | `button.body, button.icon.glyph @ variant help size mini / small / regular` | rule |
| MAC-N10 | `button.body.fill, button.icon.glyph.fill @ variant help {pressed}` | rule |
| MAC-N11 | `badge.body, badge.label @ variant count / circular / @inline` | rule |
| MAC-N11 | `badge.body, badge.label @ context list.row (SwiftUI's .badge())` | rule |
| MAC-N12 | `image.body.{fill,stroke,corners,shadow,padding} @ variant @photo / @gray_bezel / @groove / @button` | rule |
| MAC-N13 | `list.row_background.fill @ variant grid {selected} context list.view {focus_within}` | rule |
| MAC-N13 | `list.row_background.fill @ variant grid {selected} context list.view !{focus_within}` | rule |
| MAC-N13 | `list.row_background.corners @ variant grid` | rule |
| MAC-N13 | `list.values.row_background_inset @ variant grid` | rule |
| MAC-N14 | `window.content.fill @ variant @hud (the hudWindow material)` | rule |
| MAC-N14 | `window.title_bar.{fill,min_height}, window.title.text @ variant @hud` | rule |
| MAC-N14 | `window.caption_button_glyph @ variant close context window.frame variant @hud` | rule |
| MAC-N14 | `<widget>.* @ context any window variant @hud (the controls drawn in it)` | rule |
| MAC-N15 | `dial.groove, dial.knob_dot, dial.tick @ variant @circular_slider` | rule |
| MAC-N16 | `window.title_bar.min_height, window.caption_button_glyph.{min_width,min_height}, window.title.text @ variant utility` | rule |
| MAC-N16 | `reader:macos:a utility panel's title-bar height (frame against content rect)` | reader |
| MAC-N17 | `ruler.tick.stroke, ruler.label.text, ruler.marker.glyph` | field |
| MAC-N18 | `find_bar.bar.{fill,padding}` | field |
| MAC-N18 | `find_bar.values.bar_position` | field |
| MAC-N18 | `input.* @ context find_bar` | rule |
| MAC-N18 | `button.* @ variant previous / next context find_bar` | rule |
| MAC-N18 | `button.* @ variant done context find_bar` | rule |
| MAC-N19 | `sidebar.pane.fill @ variant end` | rule |
| MAC-N19 | `splitter.divider (the divider before the inspector)` | field |
| MAC-N19 | `sidebar.values.{width,collapsed_width} @ variant end` | rule |
| MAC-N19 | `sidebar.pane.{fill,corners,shadow} @ variants {end,floating} (macos-tahoe's floating glass inspector)` | rule |
| MAC-N19 | `splitter.divider @ variant sidebar (none beside the floating inspector)` | rule |
| MAC-N20 | `menu_bar.bar.{min_height,fill} (the height; the wallpaper-tinted translucent fill)` | field |
| MAC-N20 | `menu_bar.item_label.text` | field |
| MAC-N20 | `menu_bar.app_name.text` | field |
| MAC-N20 | `menu_bar.item.fill, menu_bar.item_label.text.fill @ {open}` | rule |
| MAC-N20 | `menu_bar.status_item.icon.size, menu_bar.values.status_item_gap` | field |
| MAC-N20 | `menu_bar.item_label.text, menu_bar.app_name.text, menu_bar.bar.min_height @ cond container_size(menu_bar, large)` | rule |
| MAC-N20 | `reader:macos:NSStatusBar.system.thickness, NSMenu.menuBarHeight, menuBarFontOfSize:` | reader |
| MAC-N21 | `form.card.{fill,corners,padding}, form.row_separator.{stroke,padding}, form.row.min_height, form.group_title.text, form.group_description.text` | field |
| MAC-N22 | `level_indicator.* @ variant circular` (the linear gauge is the base) | rule |
| MAC-N23 | `bottom_tab_bar.* @ variant @sidebar_adaptable` | rule |
| MAC-N24 | `pull_to_refresh.indicator` | field |
| MAC-N24 | `spinner.* @ context pull_to_refresh` | rule |
| MAC-N25 | - | out (R1.1) |
| MAC-N26 | - | out (R1.1) |
| MAC-N27† | - | out (R1.1) |
| MAC-N28† | `split_button.primary.{fill,stroke,corners}, split_button.secondary.{fill,stroke,corners}, split_button.divider.stroke, split_button.arrow.glyph` | field |
| MAC-N28† | `split_button.primary.fill, split_button.secondary.fill @ {pressed}` | rule |
| MAC-N28† | `split_button.secondary.fill, split_button.arrow.glyph @ {open}` | rule |
| MAC-N28† | `split_button.* @ size mini / small / regular / large` | rule |
| MAC-N29† | `button.body.*, button.menu_indicator @ variant menu {pressed} / variant menu {open}` (`NSComboButton` `.unified`) | rule |
| MAC-N29† | `combo_box.{body,arrow} @ variant pull_down {pressed} / variant pull_down {open}` (the pull-down `NSPopUpButton`, one control with MAC-CB01) | rule |
| MAC-N29† | `button.values.menu_indicator_gap` | field |
| MAC-N29† | `button.* @ variant menu context toolbar.bar (NSMenuToolbarItem)` | rule |
| MAC-N30† | `media_controls.bar.{fill,corners,shadow} @ variant @inline / @floating / @minimal` | rule |
| MAC-N30† | `button.* @ variant play / volume / full_screen / frame_step context media_controls.bar` | rule |
| MAC-N30† | `button.* @ context media_controls.bar (the sharing button)` | rule |
| MAC-N30† | `slider.* @ context media_controls (the scrubber)` | rule |
| MAC-N30† | `media_controls.time_label.text` | field |
| MAC-N30† | `media_controls.values.auto_hide_ms` | field |
| MAC-N31† | `rubber_band.body.fill, rubber_band.body.stroke, rubber_band.body.corners` | field |
| MAC-N32† | `about_dialog.icon.icon.size, about_dialog.name.text, about_dialog.version.text, about_dialog.credits.text, about_dialog.copyright.text` | field |
| MAC-N32† | `about_dialog.{icon,name,version,credits,copyright}.margin` (the gaps between the icon, name, version, credits and copyright, each part's own; read from the subview frames) | field |
| MAC-N32† | `about_dialog.values.width` | field |
| MAC-N33† | `search_field.*, input.* @ context menu.body (the Help menu's search field)` | rule |
| MAC-N33† | `menu.item, menu.item_label.text (its results, help topics among them)` | field |
| MAC-N33† | `menu.found_item_pointer.{glyph,fill,transitions}` | field |
| MAC-N34† | `menu_bar.extra_window.fill (material), menu_bar.extra_window.corners, menu_bar.extra_window.shadow, menu_bar.values.extra_window_gap` | field |
| MAC-N35† | `status_page.icon.icon, status_page.title.text, status_page.description.text` | field |
| MAC-N35† | `status_page.values.{icon_to_title,title_to_description,description_to_actions}` | field |
| MAC-N35† | `button.* @ context status_page.action_area` | rule |
| MAC-N35† | `status_page.* @ variant search` | rule |
| MAC-N36† | `teaching_tip.body.{fill,corners}, teaching_tip.image.icon, teaching_tip.title.text, teaching_tip.message.text` | field |
| MAC-N36† | `teaching_tip.* @ variant inline / popover` | rule |
| MAC-N36† | `teaching_tip.tail.{fill,stroke} @ variant popover` | rule |
| MAC-N36† | `button.* @ variant close context teaching_tip` | rule |
| MAC-N36† | `button.* @ context teaching_tip.action_area` | rule |
| MAC-N36† | `teaching_tip.values.placement (AppKitPopover)` | placement |
| MAC-N37† | `chart.grid_line_major.stroke, chart.grid_line_minor.stroke, chart.axis_tick.stroke, chart.axis_label.text` | field |
| MAC-N37† | `chart.series.fill = pick(element.item_index, <c0>, …) in Swift Charts' default order (data declared on series; R25.3)` | field |
| MAC-N37† | `chart.legend.text, chart.legend_label.text, chart.legend_swatch.fill` | field |
| MAC-N37† | `chart.selection_mark.stroke` | field |
| MAC-X01 | `window.frame.shadow @ cond window_inactive / variant utility / variant @hud` | rule |
| MAC-X01 | `dialog.body.shadow @ variant @sheet / variant alert` | rule |
| MAC-X01 | `menu.body.shadow, popover.body.shadow, tooltip.body.shadow` | field |
| MAC-X01 | `drag_image.image.shadow` | field |
| MAC-X01 | `button.body.shadow, combo_box.body.shadow, segmented_control.indicator.shadow, slider.thumb.shadow, switch.thumb.shadow (macos-monterey)` | field |
| MAC-X01 | `button.body.shadow @ variant glass; toolbar.item_group.shadow; sidebar.pane.shadow @ variant floating; glass_container.surface.shadow` (macos-tahoe's glass shadows) | rule |
| MAC-X01 | `PartStyle.shadow.layers (ShadowLayer: offset, blur, spread, colour, inset)` | field |
| MAC-X02 | `PartStyle.fill Material identities (vibrancy materials, blur, saturation, tint)` | field |
| MAC-X02 | `Material.params (the blending mode, behind or within the window; isEmphasized; ADD-A47)` | field |
| MAC-X02 | `declare.materials` | field |
| MAC-X02 | `PartStyle.fill @ cond window_inactive` | rule |
| MAC-X02 | `PartStyle.fill @ cond reduced_transparency (Material.fallback)` | rule |
| MAC-X03 | `PaintLayer.blend BlendMode::Keyed (vibrant blending)` | field |
| MAC-X03 | `role.* under the vibrant appearances (reader)` | role |
| MAC-X03 | `reader:macos:NSColor resolved under NSAppearance vibrantLight and vibrantDark` | reader |
| MAC-X04 | `PartStyle.fill Material identity glass, the regular glass (the base) and the variant clear (tint, lensing in the identity)` | field |
| MAC-X04 | `glass_container.surface.{fill,corners} @ variant clear` (the regular glass is the base) | rule |
| MAC-X04 | `PartStyle.shadow inset layer (the specular rim)` | field |
| MAC-X04 | `Material.params interactive (ADD-A47)` | field |
| MAC-X04 | `glass_container.surface.{transform,transitions} @ {pressed} (the interactive response; morphing)` | rule |
| MAC-X04 | `glass_container.values.merge_gap` | field |
| MAC-X04 | `button.body.fill @ variant glass` | rule |
| MAC-X04 | `toolbar.item_group.fill, sidebar.pane.fill, menu.body.fill, popover.body.fill, dialog.body.fill (Material identity glass on macos-tahoe)` | field |
| MAC-X04 | `PartStyle.fill @ {over_dark_content} (adapting over dark content)` | rule |
| MAC-X04 | `cond system_setting:<the Clear / Tinted setting's key, to verify> (ADD-A04)` | condition |
| MAC-X04 | `declare.widget.*.variants glass` | field |
| MAC-X05 | `scroll_view.edge_effect.fill (Material) @ variant soft_edge {scrolled} / variant hard_edge {scrolled}` | rule |
| MAC-X05 | `scroll_view.values.edge_effect_height; scroll_view.edge_effect @ variant start / variant end` (`absent = true`: the effect under the top and bottom bars only, one element per edge) | rule |
| MAC-X06 | `background_extension.extension.fill (Material identity background_extension)` | field |
| MAC-X06 | `background_extension.extension.fill @ context sidebar.pane variant floating` | rule |
| MAC-X06 | `background_extension.values.mirrors_content` | field |
| MAC-X07 | `PartStyle.corners per-corner Radius, Concentric, CornerCurve::Continuous (S2.4)` | field |
| MAC-X08 | `PartStyle.stroke per-side Edge (S2.4)` | field |
| MAC-X08 | `PartStyle.shadow.layers[inset] (rims, bevels)` | field |
| MAC-X09 | `PaintKind::Linear on PartStyle.fill (S2.3)` | field |
| MAC-X10 | `Color extended sRGB (S2.1, R5.5)` | field |
| MAC-X10 | `reader:macos:NSColor colorSpace read without clipping to sRGB` | reader |
| MAC-X11 | `token.motion.durations.<AppKit durations>` | token |
| MAC-X11 | `token.motion.timings.<curves and springs>` | token |
| MAC-X11 | `button.body.transitions (press and release; the glass bounce on macos-tahoe)` | field |
| MAC-X11 | `checkbox.mark.transitions, radio.dot.transitions (the toggle)` | field |
| MAC-X11 | `switch.values.presentations event @thumb_move (part thumb); switch.track.transitions (the cross-fade)` | field |
| MAC-X11 | `segmented_control.values.presentations event @indicator_move (part indicator)` | field |
| MAC-X11 | `slider.thumb.transitions` | field |
| MAC-X11 | `list.disclosure.transitions; list.values.presentations events @expand, @collapse, Appear, Disappear` (queried with the row: its children revealed, a row inserted and deleted) | field |
| MAC-X11 | `FocusIndicator.transition (the ring's appearance)` | field |
| MAC-X11 | `menu.values.presentations events Appear, Disappear, @submenu_expand; menu.values.activation_flash` | field |
| MAC-X11 | `popover.values.presentations events Appear, Disappear` | field |
| MAC-X11 | `dialog.values.presentations events Appear @ variant alert / Appear, Disappear @ variant @sheet` | rule |
| MAC-X11 | `window.frame.transitions (Geometry: an animated resize)` | field |
| MAC-X11 | `tab.values.presentations event @indicator_move` (the tab switch: its step on part `pane`, one event for the selection moving) | field |
| MAC-X11 | `tooltip.values.presentations events Appear, Disappear` | field |
| MAC-X11 | `scrollbar.values.presentations events Appear, Disappear, @flash; scrollbar.body.transitions, scrollbar.thumb.transitions (the expansion)` | field |
| MAC-X11 | `token.behavior.scroll.{overscroll,overscroll_effect} (elastic overscroll and its bounce-back)` | token |
| MAC-X11 | `progress_bar.values.loop; spinner.values.loop` | field |
| MAC-X11 | `token.caret.blink_style` | token |
| MAC-X11 | `search_field.values.presentations event @expand; sidebar.values.presentations event @collapse` | field |
| MAC-X11 | `text_area.values.presentations event @find_indicator` | field |
| MAC-X11 | `drag_image.values.presentations event @cancel (the failed drop sliding back)` | field |
| MAC-X11 | `button.values.presentations, list.values.presentations event @spring_load` | field |
| MAC-X11 | `glass_container.surface.transitions (glass morphing)` | field |
| MAC-X11 | `PartStyle.transitions and widget presentation transitions` | field |
| MAC-X11 | `PartStyle.transitions @ cond reduced_motion` | rule |
| MAC-X11 | `window.values.presentations events Appear, Disappear` (the base) `@ variant dialog / variant utility` (window open and close per NSWindow.AnimationBehavior, which the application chooses) | rule |
| MAC-X11 | - (the minimise effect, which the Dock draws; the full-screen enter and exit, which the window server animates) | out (R1.1) |
| MAC-X11 | `reader:macos:NSAnimationContext.duration, NSWindow.animationResizeTime, CAMediaTimingFunction control points` | reader |
| MAC-X12 | `token.caret.blink_on / blink_off` | token |
| MAC-X12 | `token.behavior.double_click_time / key_repeat_delay / key_repeat_interval / drag_threshold / spring_loading` | token |
| MAC-X12 | `tooltip.values.show_delay_ms` | field |
| MAC-X12 | `menu.values.submenu_open_delay_ms` | field |
| MAC-X12 | `menu.values.activation_flash` | field |
| MAC-X12 | `scrollbar.values.auto_hide_delay_ms` | field |
| MAC-X12 | `button.values.repeat_delay_ms / repeat_interval_ms` | field |
| MAC-X12 | `reader:macos:NSEvent.doubleClickInterval, keyRepeatDelay, keyRepeatInterval; getPeriodicDelay; NSTextInsertionPointBlinkPeriodOn/Off; NSInitialToolTipDelay` | reader |
| MAC-X13 | `PartStyle.focus FocusIndicator (shape, offset, opacity, transition) per control` | field |
| MAC-X13 | `list.row_background.focus (the ring on table rows)` | field |
| MAC-X13 | `<widget>.values.focus_visible_when (Never for buttons, checkboxes, segments, links; AnyFocus for text fields and lists; ADD-A16)` | field |
| MAC-X13 | `<widget>.values.focus_visible_when @ cond full_keyboard_access (Keyboard { … } for buttons, checkboxes, segments, links)` | rule |
| MAC-X13 | `cond full_keyboard_access (ADD-A06)` | condition |
| MAC-X13 | `reader:macos:NSApplication.isFullKeyboardAccessEnabled or AppleKeyboardUIMode` | reader |
| MAC-X14 | `token.caret.width / shape` | token |
| MAC-X14 | `token.caret.blink_style Fade (ADD-A24)` | token |
| MAC-X14 | `input.caret.min_height (the height against the line); text_area.caret` | field |
| MAC-X14 | `input.caret @ context list.cell {editing}` | rule |
| MAC-X14 | `input.caps_lock_indicator` | field |
| MAC-X14 | `input.dictation_effect` | field |
| MAC-X15 | `role.global.selection_text_background @ cond window_inactive` | rule |
| MAC-X15 | `input.selection.fill @ context input.body !{focused}` | rule |
| MAC-X15 | `list.row.text.fill @ {selected} context list.view !{focus_within} / {selected} cond window_inactive` | rule |
| MAC-X15 | `list.secondary_text.text.fill, list.leading_icon.icon.fill @ {selected} context list.view {focus_within}` | rule |
| MAC-X15 | `menu.item.fill @ {selected} context autocomplete (= role.@selected_control)` | rule |
| MAC-X15 | `role.global.find_highlight` | role |
| MAC-X15 | `text_area.find_match.{fill,corners,shadow,text} @ {selected}` | rule |
| MAC-X15 | `text_area.values.presentations event @find_indicator` | field |
| MAC-X15 | `text_area.find_dimming.fill @ {@incremental_find}` | rule |
| MAC-X15 | `list.row_background.corners @ {selected}` | rule |
| MAC-X15 | `role.@foreground_secondary_emphasized` | role |
| MAC-X16 | `button.body.fill, button.label.text.fill @ {default} cond window_inactive` | rule |
| MAC-X16 | `checkbox.box.fill @ {checked} cond window_inactive; radio.ring.fill, radio.dot.fill @ {checked} cond window_inactive` | rule |
| MAC-X16 | `switch.track.fill @ {checked} cond window_inactive` | rule |
| MAC-X16 | `slider.fill.fill @ cond window_inactive` | rule |
| MAC-X16 | `progress_bar.indicator.fill @ cond window_inactive` | rule |
| MAC-X16 | `segmented_control.indicator.fill @ {selected} cond window_inactive` | rule |
| MAC-X16 | `combo_box.arrow_area.fill @ cond window_inactive` | rule |
| MAC-X16 | `list.row_background.fill @ {selected} cond window_inactive` | rule |
| MAC-X16 | `sidebar.pane.fill @ cond window_inactive` | rule |
| MAC-X16 | `window.caption_button_glyph.fill @ cond window_inactive` | rule |
| MAC-X16 | `button.* @ context toolbar.bar cond window_inactive` | rule |
| MAC-X16 | `window.title.text.fill @ cond window_inactive` | rule |
| MAC-X16 | `PartStyle.{fill,stroke,text} @ cond not_main / cond app_inactive (the accent-coloured parts)` | rule |
| MAC-X16 | `cond window_inactive` | condition |
| MAC-X16 | `cond not_main` | condition |
| MAC-X16 | `cond app_inactive` | condition |
| MAC-X17 | `declare.widget.*.sizes (mini … extra_large)` | field |
| MAC-X17 | `declare.widget.*.densities compact` | field |
| MAC-X17 | `Selector.size` | rule |
| MAC-X17 | `PartStyle.{min_height,padding,corners,text} @ size mini / small / regular / large / extra_large (each control's height, padding, radius, indicator size, font size)` | rule |
| MAC-X17 | `scrollbar.body.{min_width,min_height}, switch.track.{min_width,min_height}, spinner.values.diameter @ size mini / small / regular / large` | rule |
| MAC-X17 | `PartStyle rules @ density compact (prefersCompactControlSizeMetrics)` | rule |
| MAC-X17 | `reader:macos:NSFont.systemFontSize(for:) per control size` | reader |
| MAC-X18 | `TextStyle.line_height / letter_spacing (S2.8)` | field |
| MAC-X18 | `Font.axes FollowsSize, Font.width, Font.features` | field |
| MAC-X18 | `Font.family (the system designs rounded, serif and monospaced, by their family names)` | field |
| MAC-X18 | `token.text.roles.*` | token |
| MAC-X18 | `token.text.rendering (font smoothing; R25.7)` | token |
| MAC-X18 | `reader:macos:NSFont and NSFontDescriptor (tracking, width, design, weight per style)` | reader |
| MAC-X19 | `PartStyle.icon.fill @ {pressed} / {disabled} / {selected} (template images and SF Symbols per state)` | rule |
| MAC-X19 | `list.leading_icon.icon.fill @ variant sidebar (the accent)` | rule |
| MAC-X19 | `list.leading_icon.icon.fill @ {selected} context list.view {focus_within} (on an emphasized selection)` | rule |
| MAC-X19 | `button.icon.icon.fill @ variant primary / {default} (on accent fills)` | rule |
| MAC-X20 | `PartStyle.cursor per part` | field |
| MAC-X20 | `input.body.cursor, text_area.body.cursor (I-beam); link.text.cursor (pointing hand)` | field |
| MAC-X20 | `splitter.divider.cursor; list.header_separator.cursor` (the column-resize cursor); `window.resize_zone.cursor` (resize) | field |
| MAC-X20 | `token.pointer.drag_cursors (copy, link, not allowed)` | token |
| MAC-X20 | `token.pointer.{size,color} (ADD-A46)` | token |
| MAC-X20 | `reader:macos:pointer accessibility user defaults (keys to verify) -> token.pointer.{size,color}; linkTextAttributes cursor` | reader |
| MAC-X21 | `drag_image.image.opacity` | field |
| MAC-X21 | `badge.body, badge.label @ context drag_image (the item count)` | rule |
| MAC-X21 | `role.global.drop_highlight` | role |
| MAC-X21 | `list.row_background.fill @ {drop_target}` | rule |
| MAC-X21 | `list.view.stroke @ {drop_target}` | rule |
| MAC-X21 | `input.drop_caret, text_area.drop_caret @ {drop_target} (the text drop caret)` | rule |
| MAC-X21 | `button.values.presentations, list.values.presentations, tab.values.presentations, segmented_control.values.presentations event @spring_load` | field |
| MAC-X22 | `input.<role> per_contrast (InputValue.per_contrast, S9)` | input |
| MAC-X22 | `role.* @ cond contrast_increased` | rule |
| MAC-X22 | `PartStyle.stroke @ cond contrast_increased (controls gain outlines, separators darken)` | rule |
| MAC-X22 | `cond contrast_increased` | condition |
| MAC-X22 | `reader:macos:NSColor resolved under accessibilityHighContrastAqua and DarkAqua` | reader |
| MAC-X23 | `PartStyle.fill @ cond reduced_transparency (opaque values)` | rule |
| MAC-X23 | `Material.fallback` | field |
| MAC-X24 | `cond differentiate_without_color` | condition |
| MAC-X24 | `reader:macos:NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` | reader |
| MAC-X24 | `PartStyle rules @ cond differentiate_without_color` | rule |
| MAC-X25 | `PartStyle.transitions and presentation transitions @ cond reduced_motion (reduced forms)` | rule |
| MAC-X25 | `cond reduced_motion` | condition |
| MAC-X26 | `cond button_shapes` | condition |
| MAC-X26 | `button.body.{fill,stroke} @ variant tool context toolbar.bar cond button_shapes` | rule |
| MAC-X26 | `reader:macos:the Text size setting (14+; no public API, to verify) -> text_scaling_factor (S9.4; the operand text_scale, ADD-A55)` | reader |
| MAC-X26 | `token.pointer.size (ADD-A46)` | token |
| MAC-X26 | - (Invert colours, which the OS applies) | out (R1.1) |
| MAC-X27 | `cond system_setting:wallpaper_tinting (ADD-A04)` | condition |
| MAC-X27 | `input.accent multicolour (no accent chosen)` | input |
| MAC-X27 | `role.* @ cond system_setting:wallpaper_tinting` | rule |
| MAC-X27 | `input.highlight` | input |
| MAC-X28 | `input.misspelling.text.decoration (Dotted / Wavy); text_area.misspelling.text.decoration` | field |
| MAC-X28 | `input.grammar_mark.text, text_area.grammar_mark.text` | field |
| MAC-X28 | `input.preedit.text, text_area.preedit.text (marked text underline)` | field |
| MAC-X28 | `input.prediction.text` | field |
| MAC-X28 | `text_area.writing_tools_glow` | field |
| MAC-X28 | `text_area.highlight.{fill,text,corners}` (macOS 15's highlight styles, keyed on `highlight` per style; to verify) | field |
| MAC-X29 | `PartStyle rules @ {checked,hover}, {checked,pressed}, {checked,disabled}, {default,pressed}, {focused,hover} per widget` | rule |
| MAC-X29 | `cond not_main` | condition |
| MAC-X29 | `declare.widget.*.exclusive` | field |
| MAC-X30† | `reader:macos:NSColor.withSystemEffect rollover, pressed, deepPressed, disabled per read colour` | reader |
| MAC-X30† | `PartStyle fill rules @ {hover} / {pressed} / {disabled}` | rule |
| MAC-X31† | - (the OS's Full Keyboard Access highlight) | out (R1.1) |
| MAC-X31† | `window.title_icon @ cond system_setting:title_icons` | rule |
| MAC-X31† | `cond system_setting:title_icons (ADD-A04)` | condition |
| MAC-X31† | `reader:macos:the "Show window title icons" user default (key to verify) -> cond system_setting:title_icons` | reader |
| MAC-X32† | `window.caption_button_glyph.transitions (PropertyGroup::Glyph) @ context window.caption_buttons {hover}` | rule |
| MAC-X32† | `window.values.presentations event @title_bar_reveal (MAC-W12†)` | field |
| MAC-X32† | `token.theme_change (the light / dark switch: a change of the Scheme condition, S4.3)` | token |
| MAC-X32† | `token.behavior.scroll.deceleration (momentum)` | token |
| MAC-X32† | `navigation_view.values.presentations events @push, @pop (SwiftUI NavigationStack)` | field |
| MAC-X32† | `tab.values.presentations events Appear, Disappear, Move @ context window.title_bar (window tabs open, close, reorder)` | rule |
| MAC-X32† | `<part>.transitions (PropertyGroup::Icon, TransitionKind::Keyed) and IconStyle.animation (MAC-IC05†)` | field |
| MAC-X32† | - (window tiling's motion, which the window server draws) | out (R1.1) |
| MAC-X33† | - | out (R1.1) |
| MAC-X34† | `PartStyle rules @ dir rtl (per part)` | rule |
| MAC-X34† | `window.values.caption_button_layout @ dir rtl (the traffic lights)` | rule |
| MAC-X34† | `list.disclosure.transform @ dir rtl` | rule |
| MAC-X34† | `token.icons.mirroring (S2.9)` | token |
| MAC-X35† | - | newest (R1.5) |
| MAC-X36† | `form.*, level_indicator.*, bottom_tab_bar.*, status_page.*, teaching_tip.*, chart.* (SwiftUI's own controls, MAC-N21–N23, MAC-N35†–N37†)` | field |
| MAC-X36† | - (Mac Catalyst's iPad and Mac idioms) | out (R1.4) |
| MAC-X37† | - | newest (R1.5) |
| MAC-X38† | `Edge.snap (ADD-A33)` | field |
| MAC-SF01† | `text_area.invisible.glyph @ variant space / tab / line_break / no_break_space (its fill the colour)` | rule |
| MAC-SF02† | `rich_text.emphasis.text, rich_text.strong.text, rich_text.strikethrough.text @ variant markdown` | rule |
| MAC-SF02† | `rich_text.heading_1 … heading_6, rich_text.code_block, rich_text.block_quote, rich_text.list_marker, rich_text.table @ variant markdown (absent: no system look)` | rule |
| MAC-SF03† | `rich_text.inline_code.text, rich_text.inline_code.fill` | field |
| MAC-SF03† | `rich_text.inline_code.text, rich_text.inline_code.fill @ cond window_inactive` | rule |
| MAC-SF04† | `link.text.text.{fill,decoration} @ variant inline context rich_text` | rule |
| MAC-SF04† | `link.text.text.{fill,decoration} @ variant inline context rich_text {pressed}` | rule |
| MAC-SF05† | `text_area.values.intent_styles` | field |
| MAC-SF06† | `rich_text.heading_1 … heading_6, rich_text.code_block, rich_text.inline_code, rich_text.preformatted, rich_text.block_quote, rich_text.list_marker, rich_text.table @ variant html` | rule |
| MAC-SF06† | `link.* @ variant inline context rich_text variant html` | rule |
| MAC-SF07† | `tab.bar.fill @ {drop_target} context window.title_bar` | rule |
| MAC-SF07† | `tab.values.presentations event Move @ context window.title_bar (the tabs parting)` | rule |
| MAC-SF08† | `dialog.values.placement @ variant @sheet (AppKitSheet)` | placement |
| MAC-SF08† | `reader:macos:NSWindowDelegate window(_:willPositionSheet:using:)'s rect -> the sheet's default attachment` | reader |
| MAC-SF09† | `dialog.values.placement @ variant separate_window (AppKitWindowCenter)` | placement |
| MAC-SF10† | `dialog.* @ variant @sheet / variant separate_window (SwiftUI's choice: a sheet or a window)` | rule |
| MAC-SF10† | `dialog.values.natural_size @ variant @sheet (presentationSizing's default)` | rule |
| MAC-SF10† | `dialog.values.placement (AppKitWindowCenter) @ variant separate_window` (the separate window centred like `center()`) | placement |
| MAC-RL01† | `layout.values.{item_gap,line_gap} @ variant flow` | rule |
| MAC-RL02† | `radio_group.values.row_gap; radio_group.values.column_gap @ variant horizontal` | rule |
| MAC-RL02† | `radio_group.header.{text,padding}, radio_group.values.header_placement` | field |
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
| MAC-P3-6† | `toolbar_editor.body` (the palette, a sheet) | field |
| MAC-P3-6† | `list @ variant grid context toolbar_editor` (the item grid) | rule |
| MAC-P3-6† | `toolbar @ context toolbar_editor` (the default set) | rule |
| MAC-P3-6† | `combo_box @ context toolbar_editor` (the display mode) | rule |
| MAC-P3-6† | `button @ variant done context toolbar_editor` | rule |
| MAC-P3-6† | `toolbar.values.presentations events @customize, @items_change` (the items parting and re-laid out in the bar while the editor runs; one home, R20) | field |
| MAC-P3-6† | `button.* @ {dragged} context toolbar.bar` | rule |
| MAC-P3-6† | `toolbar.bar @ {drop_target}` | rule |
| MAC-P3-7† | `autocomplete.body @ context input` | rule |
| MAC-P3-7† | `menu.body, menu.item, menu.item_label.text, menu.section_header.text @ context autocomplete` | rule |
| MAC-P3-7† | `menu.item.fill @ {selected} context autocomplete (the highlight)` | rule |
| MAC-P3-7† | `autocomplete.values.placement (AppKitMenu; its width an AnchorWidth)` | placement |
| MAC-P3-8† | `input.writing_tools_affordance` | field |
| MAC-P3-9† | `button.body.{fill,stroke} @ variant route` (the output-route button, `button`'s typed `route`) | rule |
| MAC-P3-9† | `button.icon.glyph.fill @ variant route` (`routePickerButtonColor(for:)` tints the glyph) | rule |
| MAC-P3-9† | `button.icon.glyph.fill @ {pressed} / {disabled} variant route` | rule |
| MAC-P3-9† | `button.icon.glyph.fill @ {route_active} variant route` | rule |
| MAC-P3-9† | `button flags route_active` | flag |
| MAC-P3-9† | `button.icon.glyph @ variant route` (AVKit's system image) | rule |
| MAC-P3-9† | `button.body.stroke @ variants {route,@bordered}` (`isRoutePickerButtonBordered`) | rule |
| MAC-P3-9† | `reader:macos:AVRoutePickerView.routePickerButtonColor(for:) per state -> button.icon.glyph.fill @ variant route` | reader |
| MAC-P3-9† | - (the device popover; the Picture in Picture window) | out (R1.1) |
| MAC-P3-10† | `navigation_view.values.presentations (PresentationKind::Keyed, ADD-A26) @ variant @stack_history / @stack_book / @horizontal_strip` | rule |
| MAC-P3-10† | `navigation_view.edge_shadow @ variant @stack_history / @stack_book (the shadow under the top page, to verify)` | rule |
| MAC-P3-10† | `navigation_view.values.presentations @ cond reduced_motion` | rule |
| MAC-P3-11† | `input.caret, text_area.caret @ cond window_inactive / !{focused}` (`absent = true`: caret visibility, ADD-A24) | rule |
| MAC-P3-11† | `input.caret (absent = true), text_area.caret (absent = true) @ {text_selected}` (the caret hidden while text is selected, ADD-A24) | rule |
| MAC-P3-11† | `input.drop_caret, text_area.drop_caret @ {drop_target} (the drop caret during a text drag)` | rule |
| MAC-P3-12† | `token.behavior.hide_pointer_while_typing` | token |
| MAC-P3-13† | - | out (R1.1) |
| MAC-P3-14† | - | out (R1.1) |
| MAC-P3-15† | - | out (R1.1) |
| MAC-P3-16† | - | out (R1.1) |
| MAC-P3-17† | `token.behavior.scroll_lines_per_step / scroll distances` | token |
| MAC-P3-17† | `reader:macos:NSScrollView line and page scroll properties` | reader |
| MAC-P4-1† | `menu.* @ context window.title (title pop-up menu)` | rule |
| MAC-P4-1† | `window.values.title_menu_placement (AppKitMenu)` | placement |
| MAC-P4-2† | - | out (R1.1) |
| MAC-P4-3† | - | out (R1.1) |
| MAC-P4-4† | `token.behavior.scroll.wheel_step_duration, token.behavior.scroll.wheel_step_timing` | token |
| MAC-P4-4† | `reader:macos:user default NSScrollAnimationEnabled` | reader |
| MAC-P4-5† | `window.values.tab_bar_overflow` | field |
| MAC-P4-5† | `tab.tab, tab.label.text.elision @ context window.title_bar (narrowed and truncated)` | rule |
| MAC-P4-6† | `token.behavior.double_click_distance` | token |
| MAC-P4-7† | `token.icons.mirroring (S2.9)` | token |
| MAC-P4-7† | `icon lookup by Element.direction and Element.language (S2.9, S8; ADD-B05)` | field |
| MAC-P4-8† | `combo_box.values.popup_placement @ variant pull_down (AppKitMenu: the offset, width and edge)` | placement |
| MAC-P4-8† | `combo_box.values.popup_placement @ variant editable (AppKitComboList: placement and width)` | placement |
| MAC-P4-9† | `token.theme_change (accent and highlight colour change)` | token |
| MAC-P4-10† | `menu.values.submenu_switch (SafeTriangle)` | field |
| MAC-P4-10† | `menu.values.submenu_close_delay_ms` | field |
| MAC-P4-11† | `context_help.body.{fill,stroke,shadow,padding}` | field |
| MAC-P4-11† | `context_help.text.text` | field |
| MAC-P4-11† | `context_help.values.placement (AppKitContextHelp)` | placement |
| MAC-P4-11† | `context_help.values.mode_cursor (CursorShape::WhatsThis, ADD-A59)` | field |
| MAC-P4-12† | `image_cropper.outline.stroke, image_cropper.mask.fill` | field |
| MAC-P4-12† | `image_cropper.thumb @ variant corner / variant edge / {hover} / {pressed}` | rule |
| MAC-P4-12† | `dialog.{body.padding,values.*} @ variant @picture_taker; image_cropper.*, slider.*, button.* @ context dialog.body[@picture_taker]` (the picture taker panel: its layout, the cropper, the zoom slider and the buttons) | rule |
| MAC-P4-13† | `redaction.mask.fill / corners` | field |
| MAC-P4-13† | `redaction flags redacted (StateFlag::Redacted, ADD-A14)` | flag |
| MAC-P4-13† | `redaction.mask.transitions` | field |
