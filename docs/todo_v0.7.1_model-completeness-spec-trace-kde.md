# v0.7.1 — Model completeness: Specification, traceability (KDE)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-kde.md`](todo_v0.7.1_model-completeness-gap-map-kde.md) mapped to
what carries it (R24), one line per (row, target), the rows in the order of the platform file. Withdrawn rows and
the pointer rows the map does not count are not listed; the cross-check and pass decision tables are decisions,
not rows.

**Targets** use the widget catalogue's names
([`todo_v0.7.1_model-completeness-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md), after its
rename table): `widget.part.field` (a `PartStyle` field, S2.11), `widget.values.name` (a widget value),
`component.<key>.<part>` (a keyed component), `role.<set>.<role>`, `role.global.<role>`, `role.@<key>` (S3),
`token.<path>` (S4), `input.<name>` with kind `input` (an S9 input, not the `input` widget), `reader:<platform>:<what
is read>` (S9.3), a placement strategy, a system control (S15), a cond or flag, or a core type
(`PartStyle.…`, `FocusIndicator.…`) where the row concerns every part. The selector of a rule follows `@`, in
the catalogue's selector notation: `{a,b}` states that hold together, `!{a}` none of these, `variant v`,
`variants {v,w}`, `!variant v` (`@v` keyed), `cond c`, `!cond c` (a condition, S7.1: `flavor:kirigami`,
`input:touch`, `system_setting:<key>`), `context w.part [variant v] [{a}]`, `w.part[v]` or `context any w` (an
ancestor), `not_context w.part`, `pos first`, `pos last`, `pos parity odd`, `prev w {…}`, `next w {…}` (siblings),
`size s`, `h_size_class c`, `v_size_class c`, `dir rtl`; ` / ` separates alternatives, each a rule of its own.
Presentation events are written in parentheses after `values.presentations`: typed `Appear`, `Disappear`, `Move`,
keyed `@event`. A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| K1.1 | `window.title_bar.min_height (Auto: the larger of title and buttons)` | field |
| K1.1 | `window.title_bar_content.padding (top, bottom)` | field |
| K1.2 | `window.title_bar_content.padding (sides)` | field |
| K1.2 | `window.title.padding` | field |
| K1.3 | `window.values.title_alignment (CenterFullWidth)` | field |
| K1.3 | `window.title.text.elision (Middle)` | field |
| K1.3 | `reader:kde:breezerc [Windeco] TitleAlignment` | reader |
| K1.4 | `window.caption_button.{min_width,min_height} (= mul(token.params.layout.grid_unit, <factor>), the factor by the ButtonSize the reader reads)` | field |
| K1.4 | `window.values.caption_button_gap_px` | field |
| K1.4 | `window.values.caption_spacer_width_px` | field |
| K1.4 | `reader:kde:breezerc [Windeco] ButtonSize` | reader |
| K1.5 | `window.values.caption_button_layout` | field |
| K1.5 | `reader:kde:kwinrc [org.kde.kdecoration2] ButtonsOnLeft, ButtonsOnRight` | reader |
| K1.6 | `window.caption_button_glyph.glyph (Path, per identity) @ variant close / variant maximize / variant restore / variant minimize / variant @help / variant @keep_above / variant @keep_below / variant @shade / variant @on_all_desktops / variant @application_menu / variant @exclude_from_capture` | rule |
| K1.6 | `window.caption_button_glyph.icon @ variant window_menu (window icon, recoloured)` | rule |
| K1.7 | `window.caption_button_glyph.{fill,glyph.fill} @ {hover}` | rule |
| K1.7 | `window.caption_button_glyph.fill @ variant close {hover} (qt_lighter when !window_inactive; ADD-A50)` | rule |
| K1.7 | `window.caption_button_glyph.fill @ {pressed} (kcu_mix)` | rule |
| K1.7 | `window.caption_button_glyph.fill @ variant close {pressed} (qt_darker)` | rule |
| K1.7 | `window.caption_button_glyph.fill @ {checked}` | rule |
| K1.7 | `window.caption_button_glyph.fill @ variant close cond system_setting:outline_close_button` | rule |
| K1.7 | `input.inactive.header.foreground_negative (the Inactive group's stated value, R2.6)` | input |
| K1.8 | `window.caption_button_glyph.transitions (Fill; InOutQuad; duration = the decoration's own animation duration, value phase)` | field |
| K1.9 | `window.frame.stroke (outside; kcu_mix with token.params.color.frame_contrast)` | field |
| K1.9 | `window.frame.stroke @ cond window_inactive` | rule |
| K1.9 | `window.frame.stroke @ cond maximized (absent)` | rule |
| K1.9 | `window.frame.stroke @ cond !system_setting:outline_enabled (absent)` | rule |
| K1.10 | `window.frame.corners (per corner)` | field |
| K1.10 | `window.frame.corners @ cond tiled / maximized` | rule |
| K1.10 | `reader:kde:breezerc [Common] RoundedCorners` | reader |
| K1.11 | `window.frame.padding (border band per side)` | field |
| K1.11 | `reader:kde:kwinrc BorderSize, BorderSizeAuto; breezerc [Windeco] BorderSize and exceptions` | reader |
| K1.12 | `window.frame.fill` | field |
| K1.12 | `window.frame.fill @ cond window_inactive` | rule |
| K1.13 | `reader:kde:kdeglobals [Colors:Header] and [Colors:Header][Inactive] (not [WM])` | reader |
| K1.13 | `role.header.*` | role |
| K1.13 | `input.inactive.header.* (KDE's stated Inactive values)` | input |
| K1.14 | `window.title_bar.fill (Linear, qt_lighter) @ cond system_setting:title_bar_gradient !window_inactive` | rule |
| K1.14 | `reader:kde:breezerc [Windeco] DrawBackgroundGradient` | reader |
| K1.15 | `window.title_bar @ cond system_setting:hide_title_bar` | rule |
| K1.15 | `window.frame.padding @ cond maximized system_setting:draw_border_on_maximized` | rule |
| K1.15 | `reader:kde:breezerc [Windeco] HideTitleBar, DrawBorderOnMaximizedWindows` | reader |
| K1.16 | `window.frame.{corners,padding,stroke,shadow} @ cond maximized / tiled` | rule |
| K1.16 | `cond shaded (ADD-A02)` | condition |
| K1.16 | `window.frame.{corners,padding} @ cond shaded` | rule |
| K1.17 | `window.frame.transitions (OutCubic on the active base)` | field |
| K1.17 | `window.frame.transitions @ cond window_inactive (InCubic)` | rule |
| K1.17 | `token.motion.durations.breeze_state (factor-scaled)` | token |
| K1.17 | `window.title.transitions (Text cross-fade; duration as the decoration states it, value phase)` | field |
| K1.18 | `window.frame.shadow @ cond window_inactive (layers at half strength)` | rule |
| K1.19† | - | out (R1.1) |
| K1.20 | `button.body.stroke @ {hover} / {focus_visible} / {pressed}` | rule |
| K1.20 | `button.body.stroke @ {checked}` | rule |
| K1.20 | `button.body.stroke @ {default}` | rule |
| K1.21 | `button flags focus_visible` | flag |
| K1.21 | `button.values.focus_visible_when (FocusVisibleWhen::Keyboard { … }: after a keyboard focus change; no timeout)` | field |
| K1.22 | `button.body.fill, button.body.stroke @ {default} !cond window_inactive` | rule |
| K1.23 | `button.body.shadow @ !{pressed,checked,disabled} !variant flat !cond window_inactive` | rule |
| K1.24 | `button.body.fill @ variant flat; @ {pressed} / {checked} / {default} variant flat` | rule |
| K1.24 | `button.body.stroke @ {checked} / {hover} variant flat` | rule |
| K1.24 | `button.label.text.fill @ variant flat (= role.window.foreground)` | rule |
| K1.25 | `button.body.fill @ {checked} variant warning` | rule |
| K1.25 | `button.body.stroke @ variant warning` | rule |
| K1.26 | `button.body.corners @ variant circular (Full)` | rule |
| K1.27 | `button.menu_indicator (part: min_width, glyph Path, fill)` | field |
| K1.27 | `button.values.menu_indicator_gap` | field |
| K1.28 | `button.icon.icon.size (= token.icon_sizes.small)` | field |
| K1.28 | `button.values.shows_icon` | field |
| K1.28 | `reader:kde:kdeglobals [KDE] ShowIconsOnPushButtons` | reader |
| K1.29 | `button.body.fill, button.body.stroke @ {checked,hover}` | rule |
| K1.29 | `button.body.fill @ {checked,pressed}` | rule |
| K1.29 | `button.body.fill, button.body.stroke @ {default,hover}` | rule |
| K1.30 | `button.body.transitions (Fill, Stroke)` | field |
| K1.30 | `token.motion.durations.breeze, token.motion.speed_factor` | token |
| K1.40 | `input.body.stroke @ variant warning` | rule |
| K1.40 | `input.body.stroke @ variant warning {hover}; @ variant warning {focused}` | rule |
| K1.41 | `input.body.{stroke,shadow} @ variant flat` | rule |
| K1.42 | `input.body.stroke (position inside) @ variant @border_top / variant @border_right / variant @border_bottom / variant @border_left` | rule |
| K1.44 | `input.body.transitions (stroke)` | field |
| K1.45 | `role.selection.background; role.selection.foreground @ cond window_inactive` | role |
| K1.45 | `input.selection.{fill,text.fill}` | field |
| K1.46† | `input.clear_button.{icon,padding,min_width,min_height}` | field |
| K1.46† | `input.clear_button.icon.state_effect @ {disabled} / {pressed}` | rule |
| K1.46† | `input.values.presentations (clear_button: Appear, Disappear fade)` | field |
| K1.46† | `input.values.clear_button_visibility` | field |
| K1.46† | `input flags empty` | flag |
| K1.47† | `input.block_caret.{fill,text.fill}; text_area.block_caret.{fill,text.fill}` | field |
| K1.47† | `token.caret.shape` | token |
| K1.48† | `input.values.mask_character; input.values.mask_reveal_ms` | field |
| K1.48† | `input.reveal_button.glyph; input.values.reveal_button_visibility` | field |
| K1.49† | `text_area.misspelling.text.decoration; input.misspelling.text.decoration` | field |
| K1.50 | `checkbox.mark.glyph @ {indeterminate} (three dots, Path)` | rule |
| K1.51 | `checkbox.box.fill @ {pressed}; radio.ring.fill @ {pressed} (= qt_darker(..))` | rule |
| K1.52 | `checkbox.box.shadow @ {hover} (inset ring layer); radio.ring.shadow @ {hover}` | rule |
| K1.52 | `checkbox.box.shadow @ {hover} variant warning; radio.ring.shadow @ {hover} variant warning` | rule |
| K1.52 | `checkbox.box.transitions (Shadow); radio.ring.transitions` | field |
| K1.53 | `checkbox.values.indicator_cell` | field |
| K1.53 | `checkbox.box.min_width, checkbox.box.min_height, checkbox.box.corners` | field |
| K1.54 | `checkbox.box.fill, checkbox.box.stroke @ {checked,hover} / {checked,pressed}` | rule |
| K1.54 | `checkbox.mark.* @ {indeterminate,hover}` | rule |
| K1.54 | `checkbox.box.stroke @ {checked,disabled}` | rule |
| K1.55 | `checkbox.mark.glyph.draw_extent @ {checked} / {indeterminate}; checkbox.mark.transitions (Glyph)` | rule |
| K1.55 | `radio.dot.transform @ {checked}; radio.dot.transitions (Transform)` | rule |
| K1.55 | `checkbox.box.transitions (Fill); radio.ring.transitions` | field |
| K1.55 | `token.motion.durations.breeze, token.motion.speed_factor` | token |
| K1.56 | `checkbox.box.stroke, checkbox.box.fill @ {checked} variant warning; radio.ring.{stroke,fill} @ {checked} variant warning` | rule |
| K1.57 | `checkbox.box.focus; radio.ring.focus` | field |
| K1.58† | `checkbox.mark.glyph (Path, stroke_width, line join)` | field |
| K1.58† | `checkbox.mark.glyph @ {indeterminate} (three dots, spacing)` | rule |
| K1.60 | `menu.item.{fill,stroke,corners} @ {hover}` | rule |
| K1.61 | `menu.item.fill @ {pressed}` | rule |
| K1.61 | `menu.item_label.text.fill @ {pressed}` | rule |
| K1.61 | `menu.item_icon.icon.state_effect @ {pressed} (Selected mode)` | rule |
| K1.61 | `menu.submenu_arrow.glyph.fill @ {pressed}` | rule |
| K1.62 | `menu.item.stroke (bottom focus line) @ {hover} / {pressed} cond !system_setting:menu_item_strong_focus` | rule |
| K1.62 | `reader:kde:breezerc [Style] MenuItemDrawStrongFocus` | reader |
| K1.63 | `menu.body.padding (highlight inset)` | field |
| K1.64 | `menu.body.corners` | field |
| K1.64 | `menu.body.corners @ cond not_composited (ADD-A01)` | rule |
| K1.64 | `menu.body.corners @ {placed_below} context any menu_bar / {placed_above} context any menu_bar / {placed_below} context any combo_box / {placed_above} context any combo_box (the seamless edge)` | rule |
| K1.64 | `menu flags placed_below, placed_above (shared, set by the connector from the placement result)` | flag |
| K1.65 | `menu.body.fill (alpha by MenuOpacity; kwin_blur material layer)` | field |
| K1.65 | `menu.body.stroke` | field |
| K1.65 | `menu.separator.fill` | field |
| K1.65 | `reader:kde:breezerc [Style] MenuOpacity` | reader |
| K1.66 | `menu.section_header.{text,stroke}` | field |
| K1.67 | `menu.accelerator.{text,opacity}` | field |
| K1.67 | `menu.values.{accelerator_gap_px,accelerator_alignment}` | field |
| K1.68 | `menu.submenu_arrow.{glyph,min_width}` | field |
| K1.69 | `checkbox.{box,mark} @ context menu.item` | rule |
| K1.69 | `menu.values.check_column` | field |
| K1.70 | `menu.values.{icon_column,icon_column_width_px,icon_text_gap_px,show_item_icons}` | field |
| K1.70 | `reader:kde:kdeglobals [KDE] ShowIconsInMenuItems` | reader |
| K1.71 | `menu.separator.padding` | field |
| K1.71 | `menu.item.min_height` | field |
| K1.72 | `menu.scroll_arrow` | field |
| K1.72 | `menu.values.scrollable` | field |
| K1.73 | `menu.values.submenu_open_delay_ms` | field |
| K1.81 | `tooltip.body.shadow` | field |
| K1.81 | - (the fade KWin draws) | out (R1.1) |
| K1.82 | `tooltip.body.corners @ cond not_composited (ADD-A01)` | rule |
| K1.83† | `tooltip.values.{show_delay_ms,browse_timeout_ms,display_time,hide_delay_ms}` | field |
| K1.83† | `tooltip.label.text.wrap` | field |
| K1.84† | `tooltip.* @ variant rich` | rule |
| K1.84† | `tooltip.values.placement @ variant rich (KToolTipWidget)` | placement |
| K1.84† | `tooltip.values.hide_delay_ms @ variant rich` | rule |
| K1.90 | `scrollbar.thumb.{stroke,fill,corners}` | field |
| K1.91 | `scrollbar.thumb.fill @ context scrollbar.body {hover}` | rule |
| K1.92 | `scrollbar.thumb.fill @ context any scroll_view {focus_within}` | rule |
| K1.93 | `scrollbar.track.stroke (content-side edge)` | field |
| K1.93 | `scrollbar.track.stroke @ variant vertical; @ dir rtl` | rule |
| K1.94 | `scrollbar.arrow_button.{glyph,fill}` | field |
| K1.94 | `scrollbar.arrow_button.glyph.fill @ {hover} / {pressed} / {disabled}` | rule |
| K1.94 | `scrollbar.values.arrow_buttons_start; scrollbar.values.arrow_buttons_end` | field |
| K1.94 | `reader:kde:breezerc [Style] ScrollBarAddLineButtons, ScrollBarSubLineButtons -> scrollbar.values.arrow_buttons_*` | reader |
| K1.95 | `scrollbar.thumb.transitions; scrollbar.track.transitions` | field |
| K1.96 | `scroll_view.corner.fill` | field |
| K1.100 | `slider.thumb.stroke` | field |
| K1.100 | `slider.thumb.stroke @ {hover}; @ {focus_visible}; @ variant warning` | rule |
| K1.101 | `slider.thumb.shadow` | field |
| K1.101 | `slider.thumb.shadow @ {pressed}` | rule |
| K1.102 | `slider.thumb.{fill,shadow} @ {pressed}` | rule |
| K1.103 | `slider.tick.fill` | field |
| K1.103 | `slider.tick.fill @ {in_fill}` | rule |
| K1.103 | `flag in_fill (slider tick, stop_indicator)` | flag |
| K1.103 | `slider.values.tick_gap` | field |
| K1.103 | `reader:kde:breezerc [Style] SliderDrawTickMarks -> slider.values.draw_ticks` | reader |
| K1.104 | `slider.track.corners` | field |
| K1.105 | `slider.{fill.fill,tick.fill} @ variant warning` | rule |
| K1.106 | `slider.thumb.focus` | field |
| K1.106 | `slider.thumb.transitions` | field |
| K1.110 | `progress_bar.indicator.fill (repeating linear gradient) @ {indeterminate}` | rule |
| K1.110 | `progress_bar.values.loop` | field |
| K1.110 | `reader:kde:breezerc [Style] ProgressBarAnimated, ProgressBarBusyStepDuration -> progress_bar.values.loop` | reader |
| K1.111 | `progress_bar.indicator.{stroke,fill}` | field |
| K1.112 | `progress_bar.indicator.fill @ context list.row {selected}` | rule |
| K1.113 | `progress_bar.value_label.text` | field |
| K1.113 | `progress_bar.value_label.text.fill @ context list.row {selected}` | rule |
| K1.113 | `progress_bar.values.label_gap; progress_bar.values.label_alignment; progress_bar.values.label_position` | field |
| K1.114 | `progress_bar.values.min_indicator_length` | field |
| K1.115† | `progress_bar.track.min_height @ variant @capacity` | rule |
| K1.115† | `progress_bar.values.label_position @ variant @capacity / variant @capacity_inline` | rule |
| K1.120 | `tab.values.indicator_edge, tab.indicator @ variant left / variant right` | rule |
| K1.121 | `tab.tab.fill, tab.tab.corners (bar side) @ !{selected}` | rule |
| K1.122 | `tab.tab.fill @ {selected} / {selected} variant document` | rule |
| K1.123 | `tab.tab.min_height, tab.tab.fill @ {hover} / {selected} variant @static (= qt_darker, ADD-A50, ADD-A52)` | rule |
| K1.123 | `tab.separator.stroke @ variant @static prev tab.tab !{selected} (ADD-B07)` | rule |
| K1.123 | `tab.values.tab_gap, tab.values.expanding @ variant @static` | rule |
| K1.124 | `tab.close_button.glyph.fill, tab.close_button.fill @ {hover} / {pressed} / {disabled}` | rule |
| K1.124 | `tab.close_button.glyph.size (= token.icon_sizes small)` | field |
| K1.125 | `tab.scroll_button (fill: the filled panel)` | field |
| K1.126 | `tab.overflow_edge.stroke` | field |
| K1.127 | `tab.bar.stroke (base line)` | field |
| K1.128 | `tab.pane (fill, stroke, corners, padding)` | field |
| K1.128 | `tab.pane.corners @ variant top / variant bottom / variant left / variant right` | rule |
| K1.129 | `tab.label.focus (Underline) @ {selected,focus_visible}` | rule |
| K1.129 | `tab.label.focus.transition` | field |
| K1.130 | `tab.icon.icon.size` | field |
| K1.130 | `tab.values.icon_text_gap` | field |
| K1.131 | `tab.values.alignment` | field |
| K1.131 | `reader:kde:breezerc [Style] TabBarDrawCenteredTabs` | reader |
| K1.132 | `tab.label.text.fill @ {disabled}` | rule |
| K1.132 | `tab.tab.transitions (fill, hover fade)` | field |
| K1.133† | `tab.values.presentations (Move: the tabs sliding while one is dragged)` | field |
| K1.133† | `tab.values.drag_hover_delay_ms` | field |
| K1.140 | `sidebar.pane.stroke (inner edge) @ variant @side_panel` | rule |
| K1.140 | `sidebar.pane.stroke @ {focus_within} variant @side_panel` | rule |
| K1.140 | `reader:kde:breezerc [Style] SidePanelDrawFrame` | reader |
| K1.141 | `list.values.row_background_inset @ context sidebar.pane[@side_panel]` | rule |
| K1.142† | `list.section_header.text @ context sidebar.pane[@places]` | rule |
| K1.142† | `list.leading_icon.icon.size @ context sidebar.pane[@places]` | rule |
| K1.142† | `sidebar.capacity_bar (fill, corners full, min_height)` | field |
| K1.142† | `sidebar.capacity_bar.fill @ context list.row {selected}` | rule |
| K1.142† | `list.trailing_icon @ context sidebar.pane[@places] (eject action)` | rule |
| K1.142† | `list.row.opacity @ {@hidden_place} context sidebar.pane[@places]` | rule |
| K1.142† | `list.values.presentations (@insert, @delete: the sections' opacity) @ context sidebar.pane[@places]` | rule |
| K1.150 | `input.inactive.header (the stated [Colors:Header][Inactive] roles)` | input |
| K1.150 | `role.header.background @ cond window_inactive` | role |
| K1.150 | `toolbar.bar.bindings (surface Header) variant top` | rule |
| K1.150 | `reader:kde:kdeglobals [Colors:Header][Inactive]` | reader |
| K1.151 | `toolbar.bar.stroke.bottom` | field |
| K1.151 | `toolbar.bar.stroke.top @ context dialog` | rule |
| K1.152 | `button.body.fill, button.body.stroke @ variant tool context toolbar.bar (none at rest: auto-raise) / {hover} variant tool context toolbar.bar / {pressed} variant tool context toolbar.bar / {checked} variant tool context toolbar.bar / {focus_visible} variant tool context toolbar.bar` | rule |
| K1.153 | `button.values.label_position @ variant tool context toolbar.bar variant @main / variant tool context toolbar.bar !variant @main` | rule |
| K1.153 | `reader:kde:kdeglobals [Toolbar style] ToolButtonStyle, ToolButtonStyleOtherToolbars` | reader |
| K1.154 | `button.body.padding @ variant tool context toolbar.bar` | rule |
| K1.154 | `button.values.icon_text_gap @ variant tool context toolbar.bar (beside or under)` | rule |
| K1.155 | `button.menu_indicator.{glyph,stroke,min_width} @ variants {tool,split} (the split sub-control and its separator line) / variants {tool,@inline_small} / variants {tool,@inline_large}` | rule |
| K1.155 | `button.menu_indicator.glyph.fill @ {hover} variant tool / {pressed} variant tool / {disabled} variant tool` | rule |
| K1.156 | `toolbar.separator.stroke, toolbar.separator.min_width` | field |
| K1.156 | `toolbar.handle.{stroke,min_width,min_height}` | field |
| K1.156 | `reader:kde:breezerc [Style] ToolBarDrawItemSeparator` | reader |
| K1.157 | `toolbar.overflow_button.glyph` | field |
| K1.158 | `button.icon.icon.size @ variant tool context toolbar.bar variant @main / variant tool context toolbar.bar !variant @main` | rule |
| K1.158 | `token.icon_sizes.toolbar, token.icon_sizes.keyed.main_toolbar` | token |
| K1.158 | `reader:kde:icon theme index.theme MainToolbarDefault, ToolbarDefault` | reader |
| K1.160 | `status_bar.bar.stroke.top @ variant @main_window` | rule |
| K1.170 | `list.row_background.corners, list.values.row_background_inset` | field |
| K1.170 | `list.values.row_background_inset @ pos first` | rule |
| K1.170 | `list.cell.corners @ pos first / pos last (among cells, ADD-B07)` | rule |
| K1.170 | `list.cell.corners @ variant table` | rule |
| K1.171 | `list.row_background.stroke @ {selected} / {hover}` | rule |
| K1.172 | `list.row_background.fill @ {selected,hover} (= qt_lighter, ADD-A50, ADD-A52)` | rule |
| K1.173 | `list.row_background.fill, list.row.text.fill @ {selected} cond window_inactive` | rule |
| K1.174 | `list.row_background.fill, list.row.text.fill @ {selected,disabled}` | rule |
| K1.175 | `list.row_background.focus @ {focused} !{selected}` | rule |
| K1.175 | `list.values.focus_visible_when (ADD-A16)` | field |
| K1.175 | `reader:kde:breezerc [Style] ViewDrawFocusIndicator` | reader |
| K1.176 | `list.disclosure.glyph, list.disclosure.glyph.fill @ {hover}` | rule |
| K1.176 | `list.branch_line.stroke` | field |
| K1.176 | `list.values.indent_per_level` | field |
| K1.176 | `reader:kde:breezerc [Style] ViewDrawTreeBranchLines` | reader |
| K1.177 | `list.header_section.fill @ {hover} / {pressed} / {sorted}` | rule |
| K1.177 | `list.header_section.transitions (fill)` | field |
| K1.178 | `list.header_separator.stroke, list.header.stroke.bottom` | field |
| K1.178 | `list.header_corner` | field |
| K1.179 | `list.sort_indicator.glyph` | field |
| K1.179 | `list.values.sort_indicator_inverted` | field |
| K1.179 | `list.header_section.padding, list.values.header_item_gap` | field |
| K1.179 | `reader:kde:breezerc [Style] ViewInvertSortIndicator` | reader |
| K1.180 | `list.row, list.leading_icon.icon.size, list.values.secondary_text_gap @ variant grid` | rule |
| K1.181 | `list.leading_icon.icon.size (= token.icon_sizes small)` | field |
| K1.181 | `reader:kde:icon theme SmallDefault (KStyle PM_ListViewIconSize)` | reader |
| K1.182 | `list.drop_indicator (stroke, glyph)` | field |
| K1.182 | `list.row_background.stroke @ {drop_target}` | rule |
| K1.183 | `list.cell.fill @ {@sort_column}` | rule |
| K1.183 | `reader:kde:kdeglobals [General] shadeSortColumn` | reader |
| K1.184† | `list.disclosure.glyph, list.disclosure.glyph.fill @ {selected} variant @column_view` | rule |
| K1.184† | `role.window.shade (Mid, Dark shade roles)` | role |
| K1.185† | `list.values.presentations (@expand, @collapse; = mul(token.motion.durations.<key>, token.motion.speed_factor))` | field |
| K1.190 | `popover.body.shadow (Breeze popup elevation entry)` | field |
| K1.191 | `popover.body.corners @ cond not_composited (ADD-A01)` | rule |
| K1.200 | `splitter.divider.fill @ variant @dock` | rule |
| K1.201 | `separator.line` | field |
| K1.201 | `component.multi_tab_bar.separator` | component |
| K1.210 | `layout.values.form_label_alignment, form_field_growth, form_wraps @ variant form` | rule |
| K1.211 | `group_box.title (margin, title space)` | field |
| K1.220 | `dialog.values.action_layout.row_order (KdeLayout roles)` | field |
| K1.221 | `dialog.values.button_icons` | field |
| K1.221 | `reader:kde:kdeglobals [KDE] ShowIconsOnPushButtons` | reader |
| K1.222 | `dialog.header.{fill,bindings} @ {@has_toolbar}` | rule |
| K1.222 | `dialog.header.stroke @ !{@has_toolbar} (top separator)` | rule |
| K1.223 | - | out (R1.1) |
| K1.224† | `dialog.icon.{glyph,icon.size} @ variants {alert,warning} / variants {alert,info} / variants {alert,error} / variants {alert,@question}` | rule |
| K1.230 | `spinner.icon.glyph` | field |
| K1.230 | `spinner.values.loop` | field |
| K1.240 | `combo_box.body.{fill,stroke} @ variant editable / variant editable {hover} / variant editable {focused}` | rule |
| K1.240 | `combo_box.arrow.glyph.fill @ variant editable {hover}` | rule |
| K1.240 | `combo_box.arrow.transitions` | field |
| K1.241 | `combo_box.body.{fill,stroke,shadow} @ variant flat` | rule |
| K1.242 | `combo_box.body.fill @ {open} / {pressed}` | rule |
| K1.243 | `combo_box.body.stroke @ {hover} / {focused} / {pressed}` | rule |
| K1.244 | `combo_box.body.shadow @ !variant flat !variant editable` | rule |
| K1.245 | `combo_box.popup.{fill,stroke,corners,shadow}` | field |
| K1.245 | `combo_box.item; combo_box.item.fill @ {selected}` | field |
| K1.245 | `combo_box.values.popup_placement (QtComboBox)` | placement |
| K1.246 | `combo_box.arrow.glyph.fill @ {disabled} / {hover} / {focused}` | rule |
| K1.247 | `combo_box.{body.stroke,arrow.glyph.fill} @ variant warning` | rule |
| K1.250 | `expander.arrow.glyph.fill @ {hover}` | rule |
| K1.251 | `expander.values.presentations (@expand, @collapse)` | field |
| K1.260† | `link.text.text.fill @ {hover} variant @url_label` | rule |
| K1.260† | `link.text.text.decoration @ variants {@url_label,@float} / {hover} variants {@url_label,@float}` | rule |
| K1.260† | `link.text.cursor @ variant @url_label` | rule |
| K1.270 | `token.text.keyed.heading_1` | token |
| K1.270 | `token.text.keyed.heading_2` | token |
| K1.270 | `token.text.keyed.heading_3` | token |
| K1.270 | `token.text.keyed.heading_4` | token |
| K1.271 | `token.text.keyed.smallest_readable` | token |
| K1.271 | `reader:kde:kdeglobals [General] smallestReadableFont (family, style, size)` | reader |
| K1.272 | `token.text.keyed.flat_group_title` | token |
| K1.272 | `token.text.keyed.command_link_title` | token |
| K1.272 | `token.text.keyed.menu_section` | token |
| Q1.1 | `token.icon_sizes.keyed.small` | token |
| Q1.1 | `token.icon_sizes.keyed.small_medium` | token |
| Q1.1 | `token.icon_sizes.keyed.medium` | token |
| Q1.1 | `token.icon_sizes.keyed.large` | token |
| Q1.1 | `token.icon_sizes.keyed.huge` | token |
| Q1.1 | `token.icon_sizes.keyed.enormous` | token |
| Q1.1 | `token.icon_sizes.keyed.for_labels` | token |
| Q1.2 | `token.text.keyed.smallest_readable` | token |
| Q1.2 | `token.text.roles.caption (= token.text.keyed.<key>)` | token |
| Q1.3 | `token.text.keyed.heading_1` | token |
| Q1.3 | `token.text.keyed.heading_2` | token |
| Q1.3 | `token.text.keyed.heading_3` | token |
| Q1.3 | `token.text.keyed.heading_4` | token |
| Q1.3 | `token.text.keyed.heading_5` | token |
| Q1.4 | `token.text.keyed.heading_1_primary` | token |
| Q1.4 | `token.text.keyed.heading_1_secondary` | token |
| Q1.4 | `token.text.keyed.heading_2_primary` | token |
| Q1.4 | `token.text.keyed.heading_2_secondary` | token |
| Q1.4 | `token.text.keyed.heading_3_primary` | token |
| Q1.4 | `token.text.keyed.heading_3_secondary` | token |
| Q1.4 | `token.text.keyed.heading_4_primary` | token |
| Q1.4 | `token.text.keyed.heading_4_secondary` | token |
| Q1.4 | `token.text.keyed.heading_5_primary` | token |
| Q1.4 | `token.text.keyed.heading_5_secondary` | token |
| Q1.5 | `token.text.roles.title1 (= token.text.keyed.<key>)` | token |
| Q1.5 | `token.text.roles.title2 (= token.text.keyed.<key>)` | token |
| Q1.5 | `token.text.roles.title3 (= token.text.keyed.<key>)` | token |
| Q1.5 | `token.text.roles.headline (= token.text.keyed.<key>)` | token |
| Q1.5 | `token.text.roles.subheadline (= token.text.keyed.<key>)` | token |
| Q1.5 | `token.text.roles.body_emphasized (= token.text.keyed.<key>)` | token |
| Q1.6 | `window.values.default_size (grid units)` | field |
| Q1.6 | `window.values.default_size @ cond mobile_form_factor` | rule |
| Q1.7 | `token.size_classes (Kirigami's wideScreen width)` | token |
| Q1.8 | `button.body.stroke @ {focused} cond flavor:kirigami (activeFocus shows the highlighted pen)` | rule |
| Q1.8 | `button.body.* @ variant @highlighted cond flavor:kirigami` | rule |
| Q1.8 | `button.icon.icon.fill (= foreground)` | field |
| Q1.8 | `button.* @ variant flat / variant menu / {checked} / {default} cond flavor:kirigami` | rule |
| Q1.9 | `button.body.bindings (surface Button) @ variant tool cond flavor:kirigami / (surface Window) @ variants {tool,flat} cond flavor:kirigami` | rule |
| Q1.9 | `button.body.stroke (the highlighted pen) @ {focus_visible} variant tool cond flavor:kirigami / {pressed} variant tool !variant flat cond flavor:kirigami / variants {tool,@highlighted} cond flavor:kirigami` | rule |
| Q1.9 | `button.values.label_position @ variant tool cond flavor:kirigami (the tool button style: IconOnly, TextBesideIcon, TextUnderIcon)` | rule |
| Q1.10 | `button.label.text.mnemonic_decoration @ cond mnemonics_visible` | rule |
| Q1.10 | `cond mnemonics_visible (ADD-A12)` | condition |
| Q1.11 | `input.body, text_area.body @ cond flavor:kirigami (the same Breeze frame)` | rule |
| Q1.12 | `input.values.default_width` | field |
| Q1.12 | `text_area.values.default_width; text_area.values.default_height` | field |
| Q1.13 | `text_area.misspelling.text.decoration.paint @ cond flavor:kirigami (= role.view.negative)` | rule |
| Q1.14 | `input.caret.min_width @ cond tablet_mode` | rule |
| Q1.14 | `input.caret.fill @ cond tablet_mode` | rule |
| Q1.14 | `input.selection_handle.{glyph,min_width,min_height,fill}` | field |
| Q1.15 | `input.icon.{icon,padding} @ context any search_field` | rule |
| Q1.15 | `input.body.padding @ context any search_field` | rule |
| Q1.15 | `input.clear_button.icon @ context any search_field` | rule |
| Q1.16 | `popover.body.{fill,stroke,corners,shadow,padding,bindings} @ context any search_field` | rule |
| Q1.16 | `popover.values.placement (gap) @ context any search_field` | rule |
| Q1.16 | `list.row.padding @ context any search_field` | rule |
| Q1.17 | `input.icon.icon.{size,fill} @ context any search_field cond flavor:kirigami` | rule |
| Q1.17 | `input.values.search_delay @ context any search_field` | rule |
| Q1.18 | `input.icon.{icon.size,padding} @ cond flavor:kirigami` | rule |
| Q1.18 | `input.values.icon_text_gap @ cond flavor:kirigami` | rule |
| Q1.18 | `input.body.transitions (geometry: padding)` | field |
| Q1.18 | `tooltip.body @ context any input` | rule |
| Q1.19 | `input.reveal_button.glyph; input.reveal_button.glyph @ {checked}` | rule |
| Q1.20 | `checkbox.values.label_gap, radio.values.label_gap @ cond flavor:kirigami` | rule |
| Q1.20 | `checkbox.mark.glyph @ {indeterminate} (as K1.50)` | rule |
| Q1.21 | `checkbox.label.focus, radio.label.focus @ cond flavor:kirigami (offset per side)` | rule |
| Q1.21 | `checkbox.box.focus, radio.ring.focus @ cond flavor:kirigami (Absent)` | rule |
| Q1.22 | `checkbox.label.opacity, radio.label.opacity @ {disabled} cond flavor:kirigami` | rule |
| Q1.23 | `menu.body.shadow @ cond flavor:kirigami` | rule |
| Q1.24 | `menu.values.presentations @ cond flavor:kirigami` | rule |
| Q1.25 | `menu.body.{padding,min_width} @ cond flavor:kirigami` | rule |
| Q1.26 | `menu.item.{fill,stroke,corners} @ {hover} cond flavor:kirigami` | rule |
| Q1.27 | `menu.item.fill @ {pressed} cond flavor:kirigami` | rule |
| Q1.27 | `menu.{item_label.text.fill,item_icon.icon.state_effect} @ {pressed} cond flavor:kirigami` | rule |
| Q1.28 | `menu.item.padding @ cond flavor:kirigami` | rule |
| Q1.28 | `menu.item.padding @ cond flavor:kirigami cond input:touch` | rule |
| Q1.29 | `menu.item_icon.icon.size @ cond flavor:kirigami cond input:touch` | rule |
| Q1.29 | `menu.values.{check_column,icon_column} (WhenAnyItemNeeds)` | field |
| Q1.30 | `menu.accelerator.text @ cond flavor:kirigami` | rule |
| Q1.30 | `menu.submenu_arrow.glyph @ cond flavor:kirigami` | rule |
| Q1.31 | `checkbox.{box,mark} @ context menu.item cond flavor:kirigami` | rule |
| Q1.32 | `menu.separator.{padding,min_width} @ cond flavor:kirigami` | rule |
| Q1.33 | `tooltip.body.shadow @ cond flavor:kirigami` | rule |
| Q1.34 | `tooltip.values.presentations @ cond flavor:kirigami` | rule |
| Q1.35 | `tooltip.values.placement @ cond flavor:kirigami (QtQuickTooltip)` | placement |
| Q1.35 | `tooltip.body.padding @ cond flavor:kirigami` | rule |
| Q1.36 | `tooltip.values.display_time @ cond flavor:kirigami (absent while hovered)` | rule |
| Q1.36 | `tooltip.values.{show_on_press,show_delay_ms} @ cond tablet_mode` | rule |
| Q1.37 | `tooltip.body.bindings (Tooltip)` | field |
| Q1.37 | `tooltip.body.bindings (Complementary) @ context any toast (the Complementary-bound widget the catalogue has; a parent bound to Complementary in general has no selector: core change needed)` | rule |
| Q1.38 | `tooltip.body.stroke (mix of background and text by frame contrast)` | field |
| Q1.39 | `scrollbar.track.padding` | field |
| Q1.39 | `scrollbar.values.trough_click` | field |
| Q1.40 | `scrollbar.{thumb,track}.transitions @ cond flavor:kirigami` | rule |
| Q1.41 | `scrollbar.thumb.{fill,min_width,min_height,corners} @ cond input:touch` | rule |
| Q1.41 | `scrollbar.values.auto_hide_delay @ cond input:touch` | rule |
| Q1.41 | `scrollbar.values.presentations (Appear: slide in; Disappear: fade out) @ cond input:touch` | rule |
| Q1.42 | `scrollbar.values.repeat_interval` | field |
| Q1.43 | `scroll_view.frame @ variant @framed` | rule |
| Q1.44 | `slider.thumb.{min_width,min_height}; slider.tick` | field |
| Q1.45 | `slider.{body,thumb,track,fill} @ variant range cond flavor:kirigami` | rule |
| Q1.46 | `slider.thumb.stroke @ variant range cond flavor:kirigami` | rule |
| Q1.46 | `slider.thumb.stroke @ {focused} variant range cond flavor:kirigami` | rule |
| Q1.47 | `slider.thumb.shadow @ variant range cond flavor:kirigami` | rule |
| Q1.48 | `progress_bar.track; progress_bar.indicator @ {indeterminate}` | field |
| Q1.49 | `progress_bar.body.padding @ cond flavor:kirigami` | rule |
| Q1.50 | `tab.tab, tab.bar @ cond flavor:kirigami (Breeze's tab element)` | rule |
| Q1.50 | `tab.values.tab_gap (negative: tabOverlap)` | field |
| Q1.51 | `tab.values.focus_visible_when (ADD-A16)` | field |
| Q1.52 | `tab.bar.bindings (surface Window) @ cond flavor:kirigami` | rule |
| Q1.52 | `tab.bar.min_width, tab.tab.min_width, tab.tab.min_height (= token.params.layout.grid_unit, ADD-A44) @ cond flavor:kirigami` | rule |
| Q1.53 | `sidebar.pane.bindings (surface View) @ variant overlay / (surface Window) @ !variant overlay / (surface Window) @ variants {overlay,@global} / (surface View) @ variant @global !variant overlay` | rule |
| Q1.54 | `sidebar.pane.stroke (inner edge, Header surface) @ cond flavor:kirigami` | rule |
| Q1.54 | `sidebar.pane.stroke @ variant overlay (absent)` | rule |
| Q1.55 | `sidebar.separator (stroke)` | field |
| Q1.55 | `sidebar.values.separator_inset` | field |
| Q1.55 | `sidebar.separator.transitions (geometry)` | field |
| Q1.56 | `sidebar.edge_shadow.fill (linear gradient), sidebar.edge_shadow.min_width` | field |
| Q1.56 | `sidebar.edge_shadow.transitions (opacity)` | field |
| Q1.57 | `sidebar.scrim.fill @ variant overlay` | rule |
| Q1.58 | `sidebar.values.presentations (@open, @close, @snap_back)` | field |
| Q1.59 | `sidebar.values.collapsed_width @ variant compact` | rule |
| Q1.59 | `sidebar.values.presentations (@collapse, @expand, @labels_fade)` | field |
| Q1.60 | `sidebar.values.{min_width,max_width} (side drawers) / @ variant @edge_top / variant @edge_bottom` | rule |
| Q1.60 | `sidebar.pane.padding` | field |
| Q1.60 | `sidebar.values.resize_grip_width` | field |
| Q1.61 | `sidebar.handle.{fill,corners,shadow,icon,min_width,min_height,max_width,max_height,padding}` | field |
| Q1.61 | `sidebar.values.presentations (@handle_slide)` | field |
| Q1.62 | `list.leading_icon.icon.size, list.values.leading_gap @ variant sidebar context sidebar.pane[@global]` | rule |
| Q1.62 | `list.row.text.font.size (= mul(font_size, <factor>)) @ {@expandable} context sidebar.pane[@global] (an item with children)` | rule |
| Q1.62 | `list.row.opacity, list.disclosure @ {disabled} context sidebar.pane[@global]` | rule |
| Q1.62 | `list.row.text.fill @ {selected} / {checked} context sidebar.pane[@global]` | rule |
| Q1.63 | `sidebar.header.text (text ramp heading level 1), sidebar.header.icon` | field |
| Q1.63 | `list.section_header.text, list.section_header.icon @ context sidebar.pane[@global]` | rule |
| Q1.64 | `sidebar.values.presentations (@submenu_push, @submenu_pop)` | field |
| Q1.65 | `tooltip.* @ context sidebar.pane[compact]` | rule |
| Q1.65 | `tooltip.values.show_delay_ms @ cond flavor:kirigami (toolTipDelay)` | rule |
| Q1.66 | `sidebar.values.width @ variant @context` | rule |
| Q1.66 | `sidebar.header (min_height, padding, text) @ variant @context` | rule |
| Q1.66 | `list.row.padding @ context sidebar.pane[@context]` | rule |
| Q1.66 | `list.row.text.font.size (= mul(font_size, <factor>)) @ {@expandable} context sidebar.pane[@context]` | rule |
| Q1.66 | `list.row.opacity @ {disabled} context sidebar.pane[@context]` | rule |
| Q1.67 | `sidebar.pane.{stroke,padding} (inner edge) @ cond flavor:kirigami` | rule |
| Q1.67 | `sidebar.values.presentations (@open, @close; Timing::Keyed velocity, CONTRA-2)` | field |
| Q1.68 | `toolbar.bar.bindings (surface Header) @ variant top cond flavor:kirigami / (surface Window) @ variant bottom cond flavor:kirigami` | rule |
| Q1.68 | `toolbar.bar.stroke.bottom @ variant top cond flavor:kirigami; toolbar.bar.stroke.top @ variant bottom cond flavor:kirigami` | rule |
| Q1.69 | `toolbar.bar.{padding,min_height} @ cond flavor:kirigami` | rule |
| Q1.70 | `toolbar.separator.{stroke,min_height} @ cond flavor:kirigami` | rule |
| Q1.71 | `toolbar.bar.bindings (surface Header) @ variant @page_header` | rule |
| Q1.71 | `toolbar.bar.stroke.bottom @ variant @page_header` | rule |
| Q1.72 | `toolbar.title.text (text ramp heading level 1, elision end), toolbar.title.padding @ variant @global` | rule |
| Q1.72 | `toolbar.bar.padding, toolbar.bar.min_height, toolbar.values.item_gap @ variant @global` | rule |
| Q1.73 | `toolbar.separator.{stroke,bindings} (surface Header) @ variant @global pos first (while the columns scroll)` | rule |
| Q1.73 | `button.* @ variants {tool,icon_only} context toolbar.bar variant @global (back and forward: button.icon.glyph ThemeIcon)` | rule |
| Q1.74 | `toolbar.values.presentations (@height_change, @separator_fade: = token.motion.durations.long, InOutQuad)` | field |
| Q1.74 | `toolbar.bar.max_height @ variant @global` | rule |
| Q1.75 | `toolbar.overflow_button.glyph @ cond flavor:kirigami (icon only, no menu indicator)` | rule |
| Q1.75 | `toolbar.values.overflow_placement (QtQuickMenu, below the button, ADD-A21) @ cond flavor:kirigami` | placement |
| Q1.75 | `toolbar.values.overflow_placement (above the button) @ variant bottom cond flavor:kirigami` | rule |
| Q1.76 | `toolbar.values.items_flat_by_default @ cond flavor:kirigami (its tool buttons are flat unless the action says otherwise)` | rule |
| Q1.76 | `button.values.label_position @ variant tool context toolbar.bar cond flavor:kirigami (TextBesideIcon)` | rule |
| Q1.76 | `toolbar.separator @ cond flavor:kirigami (ToolSeparator, Q1.70)` | rule |
| Q1.76 | `toolbar.values.tooltip_shows_shortcut @ cond flavor:kirigami` | rule |
| Q1.77 | `list.row_background.fill @ {hover} cond flavor:kirigami / {selected} cond flavor:kirigami` | rule |
| Q1.77 | `list.row_background.fill @ pos parity odd variant alternating_rows cond flavor:kirigami (ADD-B06)` | rule |
| Q1.78 | `list.row_background.fill @ {focus_visible} cond flavor:kirigami (the hover look) / {pressed} cond flavor:kirigami (the selected look)` | rule |
| Q1.78 | `list.values.focus_visible_when (ADD-A16)` | field |
| Q1.79 | `list.row.padding, list.values.row_background_inset, list.values.row_gap @ cond flavor:kirigami` | rule |
| Q1.79 | `list.row.padding @ pos last cond flavor:kirigami` | rule |
| Q1.79 | `list.values.row_background_inset @ variant table cond flavor:kirigami` | rule |
| Q1.80 | `list.leading_icon.icon.size @ cond flavor:kirigami / variant grid cond flavor:kirigami` | rule |
| Q1.81 | `list.row.text.fill @ {selected} cond flavor:kirigami / {pressed} cond flavor:kirigami / {disabled} cond flavor:kirigami` | rule |
| Q1.82 | `list.values.elided_tooltip` | field |
| Q1.82 | `tooltip.values.show_delay_ms @ cond flavor:kirigami; list.values.elided_tooltip @ cond tablet_mode (press and hold)` | rule |
| Q1.83 | `checkbox.*, radio.*, switch.* @ context list.row` | rule |
| Q1.83 | `list.row.text.fill @ {pressed} !{checked} / {selected}` | rule |
| Q1.84 | `list.values.presentations (@swipe; Timing::Keyed velocity, CONTRA-2)` | field |
| Q1.84 | `list.row_background @ {swiped} (Absent)` | rule |
| Q1.85 | `list.disclosure, list.branch_line @ cond flavor:kirigami` | rule |
| Q1.85 | `list.values.{indent_per_level,disclosure_gap} @ cond flavor:kirigami` | rule |
| Q1.86 | `list.header_section, list.sort_indicator @ cond flavor:kirigami` | rule |
| Q1.86 | `list.values.header_fills_trailing` | field |
| Q1.87 | `list.secondary_text.text (fill = mix, font), list.secondary_text.text.fill @ {selected}` | rule |
| Q1.87 | `list.leading_icon.icon.size @ variant @with_subtitle` | rule |
| Q1.87 | `list.values.leading_gap` | field |
| Q1.88 | `list.swipe_action.opacity @ context list.row {hover}` | rule |
| Q1.88 | `list.swipe_action.transitions (opacity)` | field |
| Q1.88 | `list.swipe_handle.{glyph,min_width,min_height,max_width,max_height}` | field |
| Q1.89 | `list.swipe_area.fill, list.swipe_area.fill @ {pressed} (= qt_darker, ADD-A50, ADD-A52)` | rule |
| Q1.89 | `list.swipe_area.shadow (top and leading edges)` | field |
| Q1.89 | `list.values.presentations (@swipe: the snap)` | field |
| Q1.90 | `popover.body.{shadow,padding} @ cond flavor:kirigami` | rule |
| Q1.91 | `popover.values.presentations @ cond flavor:kirigami` | rule |
| Q1.92 | `splitter.divider @ cond flavor:kirigami` | rule |
| Q1.92 | `splitter.values.hit_extension @ {hover}` | rule |
| Q1.93 | `splitter.divider.bindings (surface Header) @ variant @column_view` | rule |
| Q1.93 | `splitter.values.hit_extension @ variant @column_view` | rule |
| Q1.93 | `layout.values.column_min_width @ variant @column_view` | rule |
| Q1.94 | `layout.values.column_width @ variant @column_view` | rule |
| Q1.94 | `layout.values.presentations (@column_scroll) @ variant @column_view` | rule |
| Q1.95 | `separator.line.stroke @ variant @light cond flavor:kirigami` | rule |
| Q1.96 | `separator.line.stroke (= the bound surface's roles; bindings by container)` | field |
| Q1.97 | `token.params.layout.small_spacing, medium_spacing, large_spacing, grid_unit (ADD-A44)` | token |
| Q1.98 | `layout.values.container_margin @ variant pane cond flavor:kirigami / variant page cond flavor:kirigami` | rule |
| Q1.99 | `switch.values.track_inset (= floor(mul(token.params.layout.grid_unit, <fraction>)))` | field |
| Q1.99 | `switch.track.min_width, switch.track.min_height, switch.thumb.min_height` | field |
| Q1.100 | `switch.track_fill (part: fill)` | field |
| Q1.101 | `switch.thumb.stroke (= mix(.., .., token.params.color.frame_contrast)) and @ {hover} / {focus_visible}` | rule |
| Q1.101 | `switch.track.stroke @ {checked}; @ {hover} / {focus_visible} !{checked}` | rule |
| Q1.102 | `switch.values.presentations (Move, part thumb; Timing::Keyed smoothed, = token.motion.durations.short)` | field |
| Q1.102 | `switch.values.presentations @ {pressed} (Move: Instant while pressed)` | rule |
| Q1.102 | `switch.thumb.transitions (Fill)` | field |
| Q1.102 | `token.motion.durations.short (Kirigami shortDuration)` | token |
| Q1.103 | `switch.values.label_gap; switch.body.padding` | field |
| Q1.103 | `switch.label.opacity @ {disabled}` | rule |
| Q1.104 | `dialog.scrim.fill @ cond flavor:kirigami` | rule |
| Q1.104 | `dialog.values.presentations (Appear, Disappear: part scrim, Fade; ADD-B09)` | field |
| Q1.105 | `dialog.values.presentations (Appear, Disappear) @ cond flavor:kirigami` | rule |
| Q1.106 | `dialog.body.shadow @ cond flavor:kirigami` | rule |
| Q1.107 | `dialog.{body.corners,body.stroke,header} @ variant separate_window cond flavor:kirigami` | rule |
| Q1.108 | `dialog.body.padding @ cond flavor:kirigami` | rule |
| Q1.108 | `dialog.body.padding @ {@scroll_view_content}` | rule |
| Q1.108 | `dialog.values.window_margin @ cond flavor:kirigami` | rule |
| Q1.109 | `dialog.action_area.{padding,min_height} @ cond flavor:kirigami` | rule |
| Q1.109 | `dialog.action_area.stroke @ {overflowing} (ADD-A13)` | rule |
| Q1.109 | `dialog.values.action_layout.alignment` | field |
| Q1.110 | `dialog.values.standard_button_icons` | field |
| Q1.110 | `dialog.values.button_icons @ cond flavor:kirigami` | rule |
| Q1.111 | `dialog.values.presentations (Appear: Fade with a Translate rise, ADD-B10, ADD-A25) @ cond flavor:kirigami` | rule |
| Q1.112 | `dialog.{scrim.fill,body.shadow,body.stroke,body.bindings} @ cond flavor:kirigami` | rule |
| Q1.113 | `dialog.values.window_margin @ cond flavor:kirigami` | rule |
| Q1.114 | `dialog.header.padding` | field |
| Q1.114 | `dialog.header.stroke @ {overflowing}` | rule |
| Q1.114 | `dialog.title.text (Heading 1, elision End)` | field |
| Q1.114 | `tooltip @ context dialog.title (the elided title's full text)` | rule |
| Q1.114 | `dialog.close_button.glyph @ {hover} (icon swap)` | rule |
| Q1.115 | `dialog.action_area.{padding,min_height} @ cond flavor:kirigami` | rule |
| Q1.115 | `dialog.action_area.stroke @ {overflowing}` | rule |
| Q1.115 | `dialog.action.* @ variant @flat_footer` | rule |
| Q1.116 | `dialog.icon.{glyph,icon.size} @ variants {alert,warning} / variants {alert,info} / variants {alert,error} / variants {alert,success}` | rule |
| Q1.116 | `dialog.values.type_icons` | field |
| Q1.116 | `dialog.subtitle.text` | field |
| Q1.116 | `dialog.body.padding @ variant alert cond flavor:kirigami` | rule |
| Q1.117 | `dialog.* @ variant @menu_dialog (natural width, rows, padding)` | rule |
| Q1.117 | `list.row.min_height @ context dialog.body variant @menu_dialog` | rule |
| Q1.118 | `dialog.* @ variant @search_dialog (natural size)` | rule |
| Q1.118 | `input.* @ context dialog.body variant @search_dialog (frameless)` | rule |
| Q1.119 | `spinner.icon.glyph @ cond flavor:kirigami` | rule |
| Q1.119 | `spinner.values.loop (unscaled duration) @ cond flavor:kirigami` | rule |
| Q1.120 | `spinner.values.diameter @ cond flavor:kirigami` | rule |
| Q1.120 | `spinner.values.presentations @ cond flavor:kirigami` | rule |
| Q1.120 | `spinner.values.loop @ cond animations_off` | rule |
| Q1.121 | `spinner.{indicator.fill,values.stroke_width} @ context any pull_to_refresh` | rule |
| Q1.122 | `combo_box.body, combo_box.label @ cond flavor:kirigami (the same Breeze element)` | rule |
| Q1.123 | `combo_box.body.bindings (surface Button)` | field |
| Q1.123 | `combo_box.body.bindings (surface View) @ variant editable` | rule |
| Q1.124 | `combo_box.body.padding @ cond flavor:kirigami; @ variant editable cond flavor:kirigami` | rule |
| Q1.124 | `combo_box.label.min_width @ cond flavor:kirigami` | rule |
| Q1.125 | `combo_box.{popup,item} @ cond flavor:kirigami` | rule |
| Q1.125 | `combo_box.values.presentations @ cond flavor:kirigami` | rule |
| Q1.126 | `card.body.fill (= kirigami_tint, ADD-A50, ADD-A52) @ {hover} cond flavor:kirigami / {pressed} cond flavor:kirigami / {checked} cond flavor:kirigami` | rule |
| Q1.127 | `card.body.shadow @ cond flavor:kirigami` | rule |
| Q1.128 | `card.body.shadow (zero-blur lift layer, = qt_darker, ADD-A50, ADD-A52) @ cond flavor:kirigami` | rule |
| Q1.128 | `card.body.shadow @ {pressed} cond flavor:kirigami` | rule |
| Q1.129 | `card.body.stroke, card.body.corners @ cond flavor:kirigami` | rule |
| Q1.129 | `card.body.bindings (surface View) @ cond flavor:kirigami` | rule |
| Q1.130 | `card.body.padding @ cond flavor:kirigami` | rule |
| Q1.130 | `card.values.section_gap` | field |
| Q1.131 | `card.banner, card.banner_band, card.banner_title @ variant @banner` | field |
| Q1.131 | `card.values.banner_band_height_factor` | field |
| Q1.132 | `toolbar.bar @ variant bottom context card.body cond flavor:kirigami` | rule |
| Q1.133 | `layout.values.max_columns, column_min_width, column_max_width, widget_gap @ variant @cards` | rule |
| Q1.133 | `layout.values.widget_gap, container_margin @ variant @cards_list` | rule |
| Q1.134 | `link.text.text.fill @ {pressed} (= qt_darker(surface.link, ..))` | rule |
| Q1.134 | `link.text.text.fill @ {disabled} (= surface.foreground)` | rule |
| Q1.135 | `link.text.text.font.weight @ {focus_visible}` | rule |
| Q1.135 | `link.text.focus (Absent)` | field |
| Q1.136 | `link.icon (part: glyph ThemeIcon open-link-symbolic, size, fill = surface.link) @ variant @url` | rule |
| Q1.136 | `link.values.icon_text_gap` | field |
| Q1.136 | `link.values.shows_url_tooltip @ variant @url` | rule |
| Q1.136 | `tooltip.values.show_delay_ms @ cond flavor:kirigami (g1's tooltip)` | rule |
| K2.1 | `menu_bar.item.padding, menu_bar.item.fill @ {hover} / {pressed}, menu_bar.item_label.text.fill @ {hover} / {pressed}, menu_bar.focus_line @ {@weak_focus}, menu_bar.bar.bindings (Header set); menu_bar.bar.fill @ cond window_inactive` | rule |
| K2.3 | `stepper.increment_glyph.glyph.fill @ {hover} / {disabled}; stepper.increment_glyph.transitions; stepper.values.button_layout; stepper.* @ variant flat` | rule |
| K2.4 | `calendar.header, calendar.nav_button, calendar.title, calendar.weekday_label, calendar.week_number` | field |
| K2.4 | `calendar.day @ {today} / {selected} / {@non_working_day} / {other_month}` | rule |
| K2.5 | `dial.groove.stroke, dial.value_arc.stroke, dial.handle.shadow, dial.handle @ {hover}, dial.values.thickness` | rule |
| K2.6 | `component.lcd_number.frame, component.lcd_number.segment @ variant @outline / variant @filled / variant @flat` | component |
| K2.7 | `button.body.fill @ variant @command_link` | rule |
| K2.7 | `button.body.stroke @ variant @command_link` | rule |
| K2.7 | `button.icon.icon @ variant @command_link` | rule |
| K2.7 | `button.label.text @ variant @command_link` | rule |
| K2.7 | `button.subtitle.text @ variant @command_link` | rule |
| K2.7 | `button.values.icon_text_gap @ variant @command_link` | rule |
| K2.8 | `group_box.body.fill, group_box.body.stroke, group_box.title; group_box.title.text @ variant flat; checkbox.* @ context any group_box; group_box.focus_line @ {focus_visible}` | rule |
| K2.9 | `component.tool_box.page_button.fill` | component |
| K2.9 | `component.tool_box.page_button.fill @ {hover}` | component |
| K2.9 | `component.tool_box.page_button.stroke @ {checked}` | component |
| K2.9 | `component.tool_box.page_button.stroke @ {hover}` | component |
| K2.9 | `component.tool_box.page_button.transitions` | component |
| K2.9 | `component.tool_box.page_icon.icon` | component |
| K2.9 | `component.tool_box.values.item_gap` | component |
| K2.9 | `component.tool_box.values.tab_margin_width` | component |
| K2.9 | `component.tool_box.page_button.min_width` | component |
| K2.10 | `component.dock_widget.title_bar.fill` | component |
| K2.10 | `component.dock_widget.title.text` | component |
| K2.10 | `component.dock_widget.frame.stroke` | component |
| K2.10 | `component.dock_widget.float_button.glyph` | component |
| K2.10 | `component.dock_widget.close_button.glyph` | component |
| K2.10 | `component.dock_widget.separator.stroke` | component |
| K2.10 | `component.dock_widget.values.draws_frame` | component |
| K2.10 | `reader:kde:breezerc.Style.DockWidgetDrawFrame` | reader |
| K2.11 | `component.mdi_sub_window.title_bar.fill` | component |
| K2.11 | `component.mdi_sub_window.title_bar.corners` | component |
| K2.11 | `component.mdi_sub_window.frame.stroke @ !{@active}` | component |
| K2.11 | `component.mdi_sub_window.active_separator.stroke @ {@active}` | component |
| K2.11 | `component.mdi_sub_window.caption_button.glyph` | component |
| K2.11 | `component.mdi_sub_window.shadow` | component |
| K2.11 | `component.mdi_sub_window.values.title_margin_width` | component |
| K2.12 | `rubber_band.body.stroke, rubber_band.body.corners; rubber_band.body.fill @ context any list / context any window / variant @filled (set by the connector where the parent is a QGraphicsView, which the model has no widget for)` | rule |
| K2.13 | `banner.body.fill @ variant info / variant success / variant warning / variant error (literal colours of the default scheme); banner.body.stroke, banner.body.corners @ variant @inline; banner.edge_line @ variant @header / variant @footer` | rule |
| K2.13 | `banner.icon.icon.size, banner.close_button; banner.values.presentations (@reveal, @conceal)` | field |
| K2.14 | `component.page_view.icon_list, component.page_view.title_header, component.page_view.header_separator, component.page_view.search_container, component.page_view.values.margins` | component |
| K2.15 | `component.title_widget.title.text` | component |
| K2.15 | `component.title_widget.comment.text.fill @ variant @info` | component |
| K2.15 | `component.title_widget.comment.text.fill @ variant @positive` | component |
| K2.15 | `component.title_widget.comment.text.fill @ variant @warning` | component |
| K2.15 | `component.title_widget.comment.text.fill @ variant @error` | component |
| K2.15 | `component.title_widget.icon.icon` | component |
| K2.16 | `component.multi_tab_bar.tab_button.fill` | component |
| K2.16 | `component.multi_tab_bar.tab_button.fill @ {checked}` | component |
| K2.16 | `component.multi_tab_bar.tab_button.fill @ {hover}` | component |
| K2.16 | `component.multi_tab_bar.separator.stroke` | component |
| K2.16 | `component.multi_tab_bar.bar.fill` | component |
| K2.16 | `component.multi_tab_bar.tab_label.text` | component |
| K2.17 | `component.splitter_collapser.button.fill` | component |
| K2.17 | `component.splitter_collapser.arrow.glyph` | component |
| K2.17 | `component.splitter_collapser.values.presentations (Appear: Fade)` | component |
| K2.17 | `component.splitter_collapser.values.presentations (Disappear: Fade)` | component |
| K2.18 | `component.led.body @ variant @round / variant @rectangular / variant @raised / variant @sunken / variant @flat {@on}` | component |
| K2.19 | `rating.star.glyph, rating.star @ {preview} / {half} / {filled}; rating.values.half_steps` | rule |
| K2.20 | `color_button.body, color_button.swatch, color_button.values.swatch_inset` | field |
| K2.21 | `group_box.body.fill, group_box.body.stroke @ {hover} / {focused} variant @styled_panel; group_box.body @ variant @box / variant @panel / variant @win_panel (shade roles)` | rule |
| K2.22 | `wizard.header, wizard.title, wizard.subtitle, wizard.banner, wizard.action_area @ variant @classic` | rule |
| K2.23† | `date_picker.field @ variant with_popup (combo); date_picker.quick_item; menu.* @ context any date_picker` | rule |
| K2.23† | `calendar.weekday_label, calendar.day_label.text @ {@non_working_day} / {@observance_day} / {today} / {selected} / {other_month} / {@out_of_range}; the darkRed rules @ !cond contrast_high (CONTRA-1)` | rule |
| K2.24† | `time_picker.field @ variant @combo / variant @spin_box; combo_box.*, stepper.* @ context any time_picker; time_picker.values.invalid_entry` | rule |
| K2.25† | `color_picker.plane, color_picker.crosshair, color_picker.value_strip.stroke, color_picker.swatch.stroke, color_picker.swatch.focus @ variant @qt_dialog` | rule |
| K2.26† | `file_chooser.preview_pane, file_chooser.values.gap, file_chooser.values.icon_size_slider; slider.*, combo_box.*, list.* @ context any file_chooser` | rule |
| K2.27† | `breadcrumb.crumb_label.text.fill @ {@navigator_active} / !{@navigator_active} / !{hover}; breadcrumb.crumb_label.text @ {current}` | rule |
| K2.27† | `breadcrumb.crumb.fill @ {hover} / {drop_target} / {open}; breadcrumb.separator.glyph @ {hover}; breadcrumb.crumb_label.text.elision (Fade, ADD-A38); breadcrumb.places_selector; breadcrumb.values.editable` | rule |
| K2.28† | `font_chooser.requester @ variant @sunken_label; font_chooser.preview, font_chooser.values.preview_min_lines; list.* @ context any font_chooser` | rule |
| K2.29† | `autocomplete.popup.stroke, autocomplete.values.width, autocomplete.values.max_visible_items, autocomplete.values.mode` | field |
| K2.29† | `placement QtCompletionBox (autocomplete.values.placement)` | placement |
| K2.30† | `input.body.fill @ context find_bar {@match} / {@no_match} / {@neutral}; find_bar.status_label.text.fill @ {@no_match}` | rule |
| K2.30† | `code_editor.search_match.fill, code_editor.replace_match.fill` | field |
| K2.31† | `context_help.body.{fill,stroke,corners,padding} (fill ToolTipBase, frame ToolTipText; the rich text's inset as padding)` | field |
| K2.31† | `context_help.inner_frame.stroke (= role.@shade_dark, K3.53)` | field |
| K2.31† | `context_help.text.text` | field |
| K2.31† | `context_help.body.shadow (Absent: Qt's hatched shadow needs the DropShadow hint the KDE platform theme leaves unset; Breeze's popup shadow, K3.2, to verify)` | field |
| K2.31† | `context_help.values.mode_cursor (WhatsThis, ADD-A59)` | field |
| K2.31† | `context_help.values.placement (QtWhatsThis: by the pointer)` | placement |
| K2.32† | `ruler.values.thickness, ruler.values.tick_lengths, ruler.label.text, ruler.marker, ruler.tick.stroke` | field |
| Q2.1 | `stepper.body.bindings (View set), stepper.body.padding, stepper.increment @ {hover} cond flavor:kirigami` | rule |
| Q2.2 | `dial.values.default_size @ cond flavor:kirigami` | rule |
| Q2.3 | `component.delay_button.body, component.delay_button.progress_overlay` | component |
| Q2.3 | `component.delay_button.progress_overlay.transitions (Geometry; a duration depending on element.progress, ADD-B12)` | component |
| Q2.4 | `button.body.padding, button.icon.icon.size, button.body.corners, button.values.icon_text_gap @ variant circular cond flavor:kirigami` | rule |
| Q2.5 | `button.body.fill, button.body.stroke @ variant circular {checked} / variant circular {pressed} / variants {circular,flat}; button.body.stroke (its width) @ variants {circular,flat} !{hover}` | rule |
| Q2.6 | `button.body.fill (appended linear layer) @ variant circular !variant flat` | rule |
| Q2.7 | `button.body.shadow @ variant circular / variant circular {pressed} / variant circular {checked} / variants {circular,flat}; button.label.transform, button.icon.transform @ variant circular {pressed} / variant circular {checked}` | rule |
| Q2.8 | `button.body.bindings (Selection set) @ variant circular !variant flat {focused}` | rule |
| Q2.9 | `menu_bar.bar.bindings (Header set), menu_bar.item.padding, menu_bar.item.min_height, menu_bar.item.min_width @ cond flavor:kirigami` | rule |
| Q2.10 | `menu_bar.item.fill @ {pressed} cond flavor:kirigami / {hover} cond flavor:kirigami; menu_bar.item.transitions; menu_bar.item_label.text.fill @ {hover} !{pressed}` | rule |
| Q2.11 | `page_indicator.dot.min_width, page_indicator.dot.opacity @ {selected} / {pressed}, page_indicator.values.dot_gap, page_indicator.dot.transitions` | rule |
| Q2.12 | `group_box.body.stroke, group_box.body.corners, group_box.body.padding @ variant @quick_frame` | rule |
| Q2.13 | `group_box.body.stroke, group_box.title.text, group_box.body.padding @ variant @quick_group_box` | rule |
| Q2.14 | `navigation_view.values.presentations (@push, @pop, @replace: slide and fade together: ADD-B10, ADD-A25, ADD-A26)` | field |
| Q2.15 | `navigation_view.values.presentations (@layer_push, @layer_pop; ADD-B10, ADD-A25, ADD-A26) @ variant @layer` | field |
| Q2.16 | `banner.body.stroke @ variant info / variant success / variant warning / variant error` | rule |
| Q2.17 | `banner.body.fill (appended status layer) @ variant info / variant success / variant warning / variant error; banner.values.inner_corner_factor` | rule |
| Q2.18 | `banner.body.stroke, banner.body.corners @ variant @inline / variant @header / variant @footer` | rule |
| Q2.19 | `banner.body.padding, banner.icon.icon.size, banner.icon.glyph @ variant info / variant success / variant warning / variant error; banner.message.padding; banner.icon @ {multiline}` | rule |
| Q2.20 | `banner.action_area, banner.close_button.glyph, banner.values.actions_wrap` | field |
| Q2.21 | `banner.values.presentations (@reveal, @conceal: height, opacity, content opacity)` | field |
| Q2.22 | `chip.body.fill @ {checked} / {pressed}` | rule |
| Q2.23 | `chip.body.stroke @ {hover} / {checked} / {pressed}; chip.body.corners` | rule |
| Q2.24 | `chip.leading_icon.icon.size, chip.body.padding, chip.values.padding_without_icon, chip.remove_button @ variant @interactive / variant @closable` | rule |
| Q2.25 | `badge.body.stroke, badge.body.fill @ variant info / variant success / variant warning / variant error` | rule |
| Q2.26 | `badge.label.text.fill = function kirigami_badge_text (ADD-A53, ADD-A57)` | field |
| Q2.26 | `badge.body.stroke from an application colour (ADD-B16)` | field |
| Q2.27 | `badge.label.text, badge.body.corners (Full while one line), badge.body.padding, badge.icon.icon.size; badge.body.corners @ {multiline}; badge.body.padding (the pill's extra horizontal padding) @ !{multiline}` | rule |
| Q2.28 | `avatar.values.default_size, avatar.values.palette (ADD-B14)` | field |
| Q2.29 | `avatar.body.fill, avatar.body.stroke, avatar.initials.text, avatar.fallback_icon, avatar.values.fallback_icon_inset, avatar.image.corners, avatar.body.padding` | field |
| Q2.30 | `status_page.icon.icon.size, status_page.title.text, status_page.description, status_page.values.description_to_actions, status_page.values.icon_to_title` | field |
| Q2.31 | `status_page.icon.opacity, status_page.title.opacity, status_page.description.opacity @ variant @informational` | rule |
| Q2.32 | `status_page.* @ variant loading; status_page.values.max_width; progress_bar.* @ context any status_page` | rule |
| Q2.33 | `bottom_tab_bar.bar.fill; bottom_tab_bar.values.max_content_width, bottom_tab_bar.values.equal_width_items` | rule |
| Q2.34 | `bottom_tab_bar.item, bottom_tab_bar.item_icon.icon.size, bottom_tab_bar.item_label.text, bottom_tab_bar.item.padding` | rule |
| Q2.35 | `bottom_tab_bar.item.fill, bottom_tab_bar.item.stroke, bottom_tab_bar.item.corners, bottom_tab_bar.item.transitions @ {pressed} / {hover} / {checked}` | rule |
| Q2.36 | `bottom_tab_bar.item.focus @ {focus_visible}` | rule |
| Q2.37 | `sheet.body.fill (card background reference) @ variant @overlay` | rule |
| Q2.38 | `sheet.scrim, sheet.title.text, sheet.header.padding, sheet.header_separator, sheet.footer_separator, sheet.footer.padding; sheet.scrim.transitions` | field |
| Q2.39 | `sheet.handle @ cond mobile_form_factor` | rule |
| Q2.40 | `sheet.values.presentations (Appear, Disappear: fade and slide; ADD-B10, ADD-A25, ADD-A26), sheet.values.min_top_inset, sheet.values.swipe_to_close` | field |
| Q2.41 | `list.section_header.text, list.section_header.opacity, list.section_header.icon, list.section_header_line` | field |
| Q2.42 | `list.inline_header.fill, list.inline_header.padding, list.inline_header.text, list.inline_header.stroke` | field |
| Q2.43 | `component.list_item_drag_handle.handle.icon` | component |
| Q2.43 | `component.list_item_drag_handle.handle.opacity` | component |
| Q2.43 | `component.list_item_drag_handle.handle.opacity @ {hover}` | component |
| Q2.43 | `component.list_item_drag_handle.handle.opacity @ {pressed}` | component |
| Q2.43 | `component.list_item_drag_handle.values.presentations (@drop)` | component |
| Q2.43 | `component.list_item_drag_handle.values.autoscroll_step` | component |
| Q2.43 | `component.list_item_drag_handle.values.autoscroll_interval_ms` | component |
| Q2.44 | `breadcrumb.bottom_separator, breadcrumb.separator.glyph @ cond flavor:kirigami; breadcrumb.crumb.fill @ {hover} cond flavor:kirigami; breadcrumb.crumb_label.opacity @ {current} cond flavor:kirigami` | rule |
| Q2.44 | `breadcrumb.values.presentations (@scroll)` | field |
| Q2.45 | `form.values.row_gap, form.values.column_gap, form.values.narrow_max_width, form.values.collapse` | field |
| Q2.46 | `form.values.label_alignment` | field |
| Q2.47 | `form.section_title.text, form.field_label.text, form.values.group_gap` | field |
| Q2.48 | `button.icon.glyph, button.values.help_tooltip_on_click, button.body.cursor @ variant help` | rule |
| Q2.49 | `toast.body.bindings (Complementary set), toast.body.opacity, toast.body.corners, toast.body.shadow` | field |
| Q2.50 | `toast.body.padding, toast.values.content_gap, toast.values.stack_gap, toast.values.margins, toast.values.stack_max_height` | field |
| Q2.50 | `toast.values.max_width (= min(mul(token.params.layout.grid_unit, <n>), a fraction of the window's width); the window's width has no operand: core change needed)` | field |
| Q2.51 | `toast.values.durations (short, default, long)` | field |
| Q2.52 | `toast.values.presentations (Appear, Disappear, @displace; ADD-B10, ADD-A25, ADD-A26)` | field |
| Q2.53† | `edit_menu.item_icon.glyph, edit_menu.body, edit_menu.values.gap_to_selection` | field |
| Q2.53† | `placement KirigamiTextActions (edit_menu.values.placement)` | placement |
| Q2.54† | `pull_to_refresh.strip, pull_to_refresh.spinner @ {pulling}, pull_to_refresh.values.refresh_delay_ms, pull_to_refresh.values.content_shift, pull_to_refresh.values.presentations (@content_shift); spinner.* @ context any pull_to_refresh {refreshing}` | rule |
| Q2.55† | `form.group_title, form.card (card in context), form.row.padding, form.row_subtitle.text.fill, form.row.min_width @ variant @cards` | rule |
| Q2.55† | `form.row.fill @ {@clickable,hover} variant @cards / {@clickable,pressed} variant @cards; form.row.corners @ pos first / pos last; form.row.transitions` | rule |
| Q2.56† | `form.row_separator.opacity, form.row_separator.padding @ variant @cards; form.row_separator.opacity @ prev form.row {hover} / next form.row {hover} (with transitions); form.row_icon, form.row_arrow @ variant action` | rule |
| Q2.56† | `form.* @ variant flat (separators, group title, spacing, actions as push buttons); form.values.collapse` | rule |
| Q2.57† | `about_dialog.icon.icon.size, about_dialog.values.icon_max_width_fraction, about_dialog.name.text, about_dialog.description.text @ cond flavor:kirigami` | rule |
| Q2.57† | `about_dialog.section_title.text; avatar.body.min_width @ context any about_dialog` | rule |
| Q2.58† | `dialog.* @ cond mobile_form_factor (the inset in-window dialog) / variant full_size cond mobile_form_factor (the full-window layer of a narrow window) / variant separate_window (the desktop's window); the connector picks it by width, dialog.values.presentation_breakpoint (R16)` | rule |
| K3.1 | `window.frame.shadow` | field |
| K3.1 | `token.elevation.<key> (Breeze's window shadow sizes)` | token |
| K3.1 | `token.params.shadow.strength (ADD-A44)` | token |
| K3.1 | `reader:kde:breezerc [Common] ShadowSize, ShadowStrength, ShadowColor` | reader |
| K3.2 | `menu.body.shadow` | field |
| K3.2 | `tooltip.body.shadow` | field |
| K3.2 | `popover.body.shadow` | field |
| K3.2 | `token.elevation.<key> (Breeze's popup shadow sizes)` | token |
| K3.2 | `reader:kde:breezerc [Common] ShadowSize, ShadowStrength, ShadowColor` | reader |
| K3.3 | `component.mdi_sub_window.shadow (the K3.2 table round sub-windows)` | component |
| K3.4 | `button.body.shadow` | field |
| K3.4 | `combo_box.body.shadow` | field |
| K3.4 | `slider.thumb.shadow` | field |
| K3.4 | `dial.handle.shadow` | field |
| K3.11 | `window.frame.fill Material identity kwin_blur (blur_radius, saturation, noise_opacity)` | field |
| K3.11 | `declare.materials kwin_blur` | field |
| K3.11 | `reader:kde:kwinrc [Effect-blur] BlurStrength, NoiseStrength, Saturation` | reader |
| K3.12 | - | out (R1.1) |
| K3.13 | - | out (R1.1) |
| K3.14 | - | out (R1.1) |
| K3.15† | - | out (R1.1) |
| K3.16† | - | out (R1.1) |
| K3.30 | `token.motion.speed_factor` | token |
| K3.30 | `reader:kde:kdeglobals [KDE] AnimationDurationFactor` | reader |
| K3.30 | `cond animations_off (factor 0)` | condition |
| K3.31 | `token.motion.durations.breeze_state (factor times a base while the factor is set, breezerc AnimationsDuration otherwise)` | token |
| K3.31 | `token.params.motion.animation_steps (ADD-A44)` | token |
| K3.31 | `cond animations_off` | condition |
| K3.31 | `PartStyle.transitions per part` | field |
| K3.31 | `reader:kde:breezerc [Style] AnimationsEnabled, AnimationSteps, AnimationsDuration, StackedWidgetTransitionsEnabled` | reader |
| K3.33 | - | out (R1.1) |
| K3.34 | - | out (R1.1) |
| K3.35 | - | out (R1.1) |
| K3.37 | `banner.values.presentations (@reveal, @conceal: the message widget)` | field |
| K3.37 | `expander.values.presentations (@expand, @collapse: the collapsible group)` | field |
| K3.37 | `token.motion.durations.kf_timeline` | token |
| K3.37 | `token.motion.timings.in_out_sine` | token |
| K3.38† | `component.dock_widget.values.presentations (Move: the dock widgets' and toolbars' re-layout slide, InOutQuad, = token.motion.durations.breeze)` | component |
| K3.39† | - | out (R1.1) |
| K3.20 | `token.focus` | token |
| K3.20 | `PartStyle.focus on button, input, checkbox, slider, combo_box (ring)` | field |
| K3.20 | `<widget>.values.focus_visible_when (FocusVisibleWhen::Keyboard { … }, ADD-A16)` | field |
| K3.21 | `tab.label.focus kind Underline` | field |
| K3.21 | `PartStyle.focus on group title and weak-focus menu parts` | field |
| K3.22 | `FocusIndicator.kind Underline and the item-view rect (list.row_background.focus)` | field |
| K3.23 | `PartStyle.stroke @ {focus_visible} (frames)` | rule |
| K3.23 | `PartStyle.glyph.fill @ {focus_visible} (arrows, thumbs)` | rule |
| K3.23 | `role.window.decoration_focus` | role |
| K3.24 | `FocusIndicator.transition` | field |
| K3.24 | `PartStyle.transitions group Focus` | field |
| K3.40 | `role.selection.* @ cond window_inactive (kde_color, kde_intensity over the Window set)` | rule |
| K3.40 | `token.kde.inactive, token.kde.inactive_change_selection_color, token.kde.inactive_enabled` | token |
| K3.40 | `reader:kde:kdeglobals [ColorEffects:Inactive] ChangeSelectionColor, Enable` | reader |
| K3.41 | `token.caret.blink_on_ms / blink_off_ms (Absent = no blink)` | token |
| K3.41 | `reader:kde:kdeglobals [KDE] CursorBlinkRate` | reader |
| K3.42 | `token.caret.width` | token |
| K3.43 | `token.caret.hides_while_selecting (false: blinks with a selection)` | token |
| K3.44 | `token.behavior.mnemonics (ADD-A12)` | token |
| K3.44 | `reader:kde:breezerc [Style] MnemonicsMode` | reader |
| K3.45 | `token.text.rendering (ADD-B04)` | token |
| K3.45 | `reader:kde:kdeglobals [General] XftAntialias, XftHintStyle, XftSubPixel` | reader |
| K3.46 | `Font.width (stretch), Font.axes, Font.features, TextStyle.letter_spacing, TextStyle.word_spacing (S2.8)` | field |
| K3.46 | `Font.weight, Font.width, Font.style (the style name mapped to them)` | field |
| K3.46 | `TextStyle.case (ADD-A39)` | field |
| K3.46 | `reader:kde:kdeglobals font strings (capitalization, letter and word spacing, stretch, style name, features, axes)` | reader |
| K3.47 | `token.text.keyed.taskbar` | token |
| K3.47 | `reader:kde:kdeglobals [General] taskbarFont` | reader |
| K3.48† | `input.preedit.text (TextStyle.decoration)` | field |
| K3.48† | `input.caret @ {composing}` | rule |
| K3.50 | `token.kde.disabled` | token |
| K3.50 | `role.<set>.disabled.* derived by kde_intensity, kde_color, kde_contrast` | role |
| K3.50 | `reader:kde:kdeglobals [ColorEffects:Disabled] IntensityEffect, IntensityAmount, ColorEffect, ColorAmount, ContrastEffect, ContrastAmount` | reader |
| K3.51 | `role.* @ cond window_inactive (kde_* over the inactive input roles)` | rule |
| K3.51 | `input.inactive (KDE stated Inactive values)` | input |
| K3.51 | `token.kde.inactive, token.kde.inactive_enabled` | token |
| K3.51 | `reader:kde:kdeglobals [ColorEffects:Inactive], [Colors:*][Inactive]` | reader |
| K3.52 | `role.<set>.background_active / background_link / background_visited / background_negative / background_neutral / background_positive (ADD-A48)` | role |
| K3.53 | `role.@shade_light / midlight / mid / dark / shadow (ADD-A48)` | role |
| K3.53 | `token.params.color.contrast (ADD-A44)` | token |
| K3.53 | `reader:kde:kdeglobals [KDE] contrast` | reader |
| K3.54 | `role.complementary.*` | role |
| K3.54 | `role.<set>.foreground_inactive / foreground_active / link / link_visited / negative / neutral / positive` | role |
| K3.54 | `reader:kde:kdeglobals [Colors:*] all seven sets` | reader |
| K3.60 | `IconStyle.state_effect @ {disabled} / {hover} (keys declared in icon_state_effects, each implemented to KIconEffect's source; S2.9)` | field |
| K3.60 | `PartStyle.icon.state_effect, PartStyle.opacity @ {disabled} (to gray and semi-transparent)` | rule |
| K3.60 | `token.params.icon_effect.<key> (the effects' amounts: the gamma of the active effect, the gray and transparency of the disabled one)` | token |
| K3.61 | `PartStyle.icon.fill and palette from roles per icon mode (ADD-A37)` | field |
| K3.62 | `PartStyle.icon.fill @ {selected} / {pressed} (selection foreground)` | rule |
| K3.63 | `token.icon_sizes.<toolbar, list, tab, button groups>` | token |
| K3.63 | `reader:kde:icon theme index.theme *Default sizes` | reader |
| K3.64 | `token.pointer.theme / size (ADD-A46)` | token |
| K3.64 | `reader:kde:kcminputrc [Mouse] cursorTheme, cursorSize` | reader |
| K3.65† | `PartStyle.cursor per part` | field |
| K3.65† | `token.behavior.hide_pointer_while_typing` | token |
| K3.66† | - | out (R1.1) |
| K3.70 | `token.behavior.double_click_ms` | token |
| K3.70 | `reader:kde:kdeglobals [KDE] DoubleClickInterval` | reader |
| K3.71 | `token.behavior.drag_threshold` | token |
| K3.71 | `token.behavior.drag_time_ms (ADD-A45)` | token |
| K3.71 | `reader:kde:kdeglobals [KDE] StartDragDist, StartDragTime` | reader |
| K3.72 | `token.behavior.scroll_lines_per_step` | token |
| K3.72 | `reader:kde:kdeglobals [KDE] WheelScrollLines` | reader |
| K3.73 | `token.behavior.single_click_activates (ADD-A45)` | token |
| K3.73 | `reader:kde:kdeglobals [KDE] SingleClick` | reader |
| K3.76† | `stepper.values.repeat_delay_ms / repeat_interval_ms (the mouse's interval)` | field |
| K3.76† | `stepper.values.key_repeat_interval_ms (the interval while an arrow key is held)` | field |
| K3.76† | `token.behavior.long_press_ms` | token |
| K3.83 | `cond tablet_mode` | condition |
| K3.83 | `reader:kde:org.kde.KWin.TabletModeManager D-Bus property` | reader |
| K3.83 | `PartStyle sizes @ cond tablet_mode per widget` | rule |
| K3.84† | - | out (R1.1) |
| K3.90 | `token.motion.speed_factor` | token |
| K3.90 | `reader:kde:kdeglobals [KDE] AnimationDurationFactor` | reader |
| K3.90 | `cond reduced_motion` | condition |
| K3.91 | `token.params.color.contrast (ADD-A44)` | token |
| K3.91 | `token.params.color.frame_contrast` | token |
| K3.91 | `reader:kde:kdeglobals [KDE] contrast, frameContrast` | reader |
| K3.92 | `menu.body.fill (translucent, Color.a)` | field |
| K3.92 | `Material identity kwin_blur on menu.body.fill` | field |
| K3.92 | `reader:kde:breezerc and kwinrc menu opacity and blur (K3.10, K3.11)` | reader |
| K3.93 | - | out (R1.1) |
| K3.95† | - | out (R1.1) |
| K3.96† | - | out (R1.1) |
| Q3.1 | `role.window.* role.view.* role.button.* role.selection.* role.tooltip.* role.complementary.* role.header.*` | role |
| Q3.1 | `reader:kde:kdeglobals [Colors:*]` | reader |
| Q3.2 | `PartStyle.bindings Surface(set) per widget (S2.11)` | field |
| Q3.3 | `role.* @ cond window_inactive` | rule |
| Q3.3 | `cond window_inactive` | condition |
| Q3.4 | `role.<set>.disabled.*` | role |
| Q3.4 | `PartStyle fill, text, icon @ {disabled} (refer to disabled roles)` | rule |
| Q3.5 | `role.global.status.info = role.<set>.foreground_active` | role |
| Q3.10 | `role.<set>.negative / neutral / positive foregrounds` | role |
| Q3.11 | `role.<set>.decoration_hover` | role |
| Q3.11 | `PartStyle.stroke @ {hover}` | rule |
| Q3.12 | `role.<set>.decoration_focus` | role |
| Q3.12 | `PartStyle.fill @ {focus_visible}` | rule |
| Q3.13 | `role.<set>.background_alternate` | role |
| Q3.13 | `list.row_background.fill @ pos parity odd variant alternating_rows (ADD-B06)` | rule |
| Q3.14 | `token.params.color.frame_contrast (ADD-A44)` | token |
| Q3.14 | `reader:kde:kdeglobals [KDE] frameContrast (store the factor, not only finished colours)` | reader |
| Q3.15 | `role.<set>.border = mix(background, foreground, token.params.color.frame_contrast)` | role |
| Q3.16 | `PartStyle.fill.append = alpha(...) @ {hover} / {pressed} / {checked}` | rule |
| Q3.17 | `PartStyle.icon.fill (text or highlighted-text role)` | field |
| Q3.17 | `PartStyle.icon.fill @ {selected}` | rule |
| Q3.18 | `IconStyle.state_effect @ {hover} cond flavor:kirigami / {disabled} cond flavor:kirigami (Kirigami's icon shader: highlight amount, desaturation; keys declared in icon_state_effects; S2.9)` | field |
| Q3.18 | `PartStyle.opacity @ {disabled} cond flavor:kirigami (the disabled icon)` | rule |
| Q3.18 | `token.params.icon_effect.<key> (the shader's highlight amount and desaturation)` | token |
| Q3.19 | `PartStyle.transitions group Icon (cross-fade)` | field |
| Q3.20 | `ShadowLayer x, y, blur, color, spread absent (S2.5)` | field |
| Q3.21 | `token.elevation.<Kirigami and qqc2 surfaces>` | token |
| Q3.21 | `PartStyle.shadow Elevation(key) per surface part` | field |
| Q3.22 | `role.global.scrim` | role |
| Q3.22 | `dialog.scrim.fill` | field |
| Q3.22 | `dialog.values.presentations (Appear, Disappear: part scrim, Fade)` | field |
| Q3.23 | `token.motion.durations.very_short / short / long / very_long` | token |
| Q3.24 | `token.motion.durations.long = plasmarc longDuration times speed factor (with a floor by max)` | token |
| Q3.24 | `function max (ADD-A56)` | field |
| Q3.24 | `reader:kde:plasmarc [Units] longDuration` | reader |
| Q3.24 | `reader:kde:kdeglobals [KDE] AnimationDurationFactor` | reader |
| Q3.25 | `token.motion.speed_factor @ cond animations_off` | rule |
| Q3.25 | `token.motion.durations.floor (a floor via max, ADD-A56)` | token |
| Q3.26 | `token.motion.timings.<easing curves in use>` | token |
| Q3.26 | `Timing::Keyed(velocity based)` | token |
| Q3.27 | `token.motion.durations.human_moment (unscaled)` | token |
| Q3.28 | `tooltip.values.show_delay_ms` | field |
| Q3.28 | `tooltip.values.show_delay_ms @ cond tablet_mode = token.behavior.long_press_ms` | rule |
| Q3.29 | `token.params.layout.grid_unit (ADD-A44)` | token |
| Q3.29 | `Length expressions mul(token.params.layout.grid_unit, <n>)` | field |
| Q3.30 | `PartStyle.focus per Breeze part (no single indicator)` | field |
| Q3.31 | `cond tablet_mode` | condition |
| Q3.31 | `cond input:touch` | condition |
| Q3.31 | `menu.item.min_height @ cond input:touch` | rule |
| Q3.31 | `scrollbar.thumb.*, scrollbar.values.{auto_hide_delay,presentations} @ cond tablet_mode cond flavor:kirigami (the overlay scroll bar of Q1.41) / cond input:touch cond flavor:kirigami` | rule |
| Q3.31 | `tooltip.values.show_on_press @ cond tablet_mode` | rule |
| Q3.31 | `input.body.fill @ {hover} cond tablet_mode (as at rest: no hover look)` | rule |
| Q3.31 | `input.selection_handle` | field |
| Q3.31 | `sidebar.handle` | field |
| Q3.31 | `sheet.handle` | field |
| Q3.32 | `token.text.rendering.engine (ADD-B04)` | token |
| Q3.33 | `PartStyle.opacity @ {disabled} per widget part` | rule |
| K-SF01† | `reader:kde:the KSyntaxHighlighting theme chosen by matching [Colors:View] BackgroundNormal and [Colors:Selection] BackgroundNormal` | reader |
| K-SF01† | `code_editor.* bases from the Breeze Light and Breeze Dark editor themes` | field |
| K-SF02† | `code_editor.text.fill, code_editor.selection.fill` | field |
| K-SF03† | `code_editor.current_line.fill; code_editor.current_line.fill (= mix(base, the mark's colour, <fraction>)) @ {@line_has_mark_bookmark} / {@line_has_mark_breakpoint_active} / {@line_has_mark_breakpoint_reached} / {@line_has_mark_breakpoint_disabled} / {@line_has_mark_execution} / {@line_has_mark_warning} / {@line_has_mark_error}` | rule |
| K-SF04† | `code_editor.gutter.fill, code_editor.line_number, code_editor.current_line_number, code_editor.gutter_separator, code_editor.fold_marker, code_editor.mark @ variant @bookmark / variant @breakpoint_active / variant @breakpoint_reached / variant @breakpoint_disabled / variant @execution / variant warning / variant error` | rule |
| K-SF05† | `code_editor.modified_line_marker.fill, code_editor.saved_line_marker.fill; minimap strip = function kte_minimap_mark (ADD-A53, ADD-A57)` | field |
| K-SF06† | `code_editor.whitespace_mark.glyph @ variant @tab / variant @space / variant @nbsp, code_editor.values.whitespace_mark_size, code_editor.indentation_guide.stroke` | rule |
| K-SF07† | `code_editor.bracket_match.fill, code_editor.bracket_match.transitions (the optional animation)` | field |
| K-SF08† | `code_editor.search_match.fill, code_editor.replace_match.fill` | field |
| K-SF09† | `code_editor.right_margin.stroke, code_editor.dynamic_wrap_indicator.fill (pattern: ADD-B01)` | field |
| K-SF10† | `code_editor.syntax_span.text, code_editor.syntax_span.fill @ variant (typed and keyed syntax roles); code_editor.syntax_span.text.fill @ {selected}` | rule |
| K-SF11† | `code_editor.misspelled_word.text.decoration, code_editor.template_field.fill, code_editor.template_placeholder @ {focused} / {read_only}` | rule |
| K-SF12† | `rich_text.heading_1 … heading_6.text @ variant markdown` | rule |
| K-SF13† | `rich_text.inline_code.text, rich_text.code_block.text, rich_text.values.block_margins @ variant markdown` | rule |
| K-SF14† | `rich_text.block_quote.padding, rich_text.values.quote_indent_per_level` | field |
| K-SF15† | `rich_text.list_marker.glyph, rich_text.values.list_marker_kinds, rich_text.values.list_marker_gap, rich_text.task_box.glyph @ {checked}` | rule |
| K-SF16† | `rich_text.table.stroke, rich_text.table_cell.padding, rich_text.table_header_cell.text` | field |
| K-SF17† | `rich_text.rule.stroke, rich_text.values.block_margins (paragraph, an expression on font_size)` | field |
| K-SF18† | `rubber_band.body.fill @ context window (filled)` | rule |
| K-SF18† | `rubber_band.body.stroke (outline otherwise)` | field |
| K-SF18† | `rubber_band.body.corners` | field |
| K-SF18† | `component.dock_widget.values.presentations (Move: the dock gap sliding open, AnimatedDocks; as K3.38†)` | component |
| K-SF19† | `dialog.values.placement @ variant separate_window (QtDialog)` | placement |
| K-SF20† | - | out (R1.1) |
| K-SF21† | `dialog.values.placement @ cond flavor:kirigami (CenteredInHost)` | placement |
| K-SF22† | `dialog.values.placement (CenteredInHost, host Overlay)` | placement |
| K-SF23† | `sheet.values.anchor, sheet.values.min_top_inset @ variant @overlay` | field |
| K-RL01† | `component.command_bar.frame.stroke` | component |
| K-RL01† | `component.command_bar.frame.shadow` | component |
| K-RL01† | `component.command_bar.search_field` | component |
| K-RL01† | `component.command_bar.results` | component |
| K-RL01† | `keycap.key.fill` | field |
| K-RL01† | `keycap.key.shadow` | field |
| K-RL01† | `keycap.key.corners` | field |
| K-RL01† | `keycap.key.text.fill` | field |
| K-RL01† | `keycap.separator.text.fill` | field |
| K-RL01† | `keycap.separator.text.fill @ {selected}` | rule |
| K-RL01† | `keycap.values.key_horizontal_margin` | field |
| K-RL01† | `component.command_bar.frame.min_width` | component |
| K-RL01† | `component.command_bar.frame.min_height` | component |
| K-RL01† | `component.command_bar.values.preferred_width_divisor` | component |
| K-RL01† | `component.command_bar.values.preferred_height_divisor` | component |
| K-RL01† | `component.command_bar.values.top_offset` | component |
| K-RL01† | `component.command_bar.values.placement (KCommandBar)` | placement |
| K-RL01† | `role.@shade_dark` | role |
| K-RL01† | `role.@shade_shadow` | role |
| K-RL02† | `shortcuts_dialog.extender_indicator.glyph @ {open}; shortcuts_dialog.key_sequence_button @ {@recording}; shortcuts_dialog.clear_button.icon` | rule |
| K-RL03† | - | out (R1.1) |
| K-RL04† | `component.char_select.table.fill` | component |
| K-RL04† | `component.char_select.cell.fill` | component |
| K-RL04† | `component.char_select.cell.fill @ {@glyph_missing}` | component |
| K-RL04† | `component.char_select.cell.text.font` | component |
| K-RL04† | `component.char_select.cell.min_width (an expression over element.char_width; the x-height and the font height have no operand: core change needed)` | component |
| K-RL04† | `component.char_select.cell.min_height (the same form)` | component |
| K-RL05† | `input.* @ context list.cell {editing} (frameless)` | rule |
| K-RL05† | `input.body.min_width (Auto: widens with its content) @ context list.cell {editing}` | rule |
| K-RL06† | `drag_image.image.opacity, drag_image.values.hotspot @ context list` | rule |
| K-RL06† | `list.row_background @ {dragged}` | rule |
| K-RL07† | `button.values.repeat_delay, button.values.repeat_interval` | field |
| K-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| K-FM01† | `reader:kde:fontconfig emoji family (fc-match emoji)` | reader |
| K-FM02† | `token.text.fallbacks.families, token.text.fallbacks.per_language (ADD-A41)` | token |
| K-FM02† | `reader:kde:fontconfig per language (fc-match -s family:lang=tag)` | reader |
| K-FM03† | `Selector.direction Rtl (S7)` | rule |
| K-FM03† | `IconRole.mirrors_in_rtl (S2.9)` | field |
| K-FM03† | `reader:kde:QGuiApplication layoutDirection` | reader |
| K-P3-1† | `window.values.resize_band (outside, largeSpacing)` | field |
| K-P3-1† | `window.values.resize_band @ cond tiled (per edge)` | rule |
| K-P3-2† | `window.values.caption_button_capabilities` | field |
| K-P3-3† | - | out (R1.1) |
| K-P3-4† | `tab.label.text.elision (None)` | field |
| K-P3-4† | `tab.values.overflow, tab.values.scroll_to_selected, tab.values.scroll_margin` | field |
| K-P3-4† | `tab.values.tab_gap (negative: overlap) @ cond flavor:kirigami` | rule |
| K-P3-5† | `menu.values.placement (QtMenu; use_full_screen_height)` | placement |
| K-P3-5† | `combo_box.values.popup_placement (QtComboPopup; max_visible_items)` | placement |
| K-P3-6† | `tooltip.values.placement (QtTooltip)` | placement |
| K-P3-7† | `dialog.values.message_wrap.{wrap_above_px,min_width_px,elide_above_px,scroll_above_px} @ variant alert (expressions on element.screen_width, element.screen_height and element.char_width, ADD-B17)` | field |
| K-P3-7† | `dialog.values.message_wrap.item_list_max_width_px (= mul(element.screen_width, <fraction>)), dialog.values.message_wrap.details_min_lines @ variant alert` | field |
| K-P3-8† | `about_dialog.icon.icon.size, about_dialog.name.text, about_dialog.version.text @ cond flavor:widgets` | rule |
| K-P3-8† | `about_dialog.person_name.text, about_dialog.person_task.text, about_dialog.link_button.icon @ cond flavor:widgets` | rule |
| K-P3-9† | - | out (R1.1) |
| K-P3-10† | `chart.plot_area.fill, chart.grid_line_major, chart.grid_line_minor, chart.axis_label, chart.legend, chart.legend_swatch, chart.values.labels_per_axis, chart.values.legend_gap, chart.values.default_size @ variant @line / variant @pie` | rule |
| K-P3-10† | `chart.values.series_colors = function kqc_series_color per series (ADD-A53, ADD-A57, ADD-B15)` | field |
| K-P3-11† | `code_editor.caret.min_width, code_editor.caret.fill (= foreground of the span under it: syntax_span binding), code_editor.extra_caret` | field |
| K-P3-11† | `code_editor.caret @ variant @vi_block / variant @vi_underline / variant @vi_half_block` | rule |
| K-P3-12† | `code_editor.minimap, code_editor.minimap_slider.fill, code_editor.minimap_slider.stroke, code_editor.minimap_fade, code_editor.scroll_mark (HSL underlay: ADD-A53, ADD-A57), code_editor.values.minimap_width` | field |
| K-P3-13† | `code_editor.completion_popup.stroke, code_editor.completion_row, code_editor.values.completion_offset_y` | field |
| K-P3-13† | `placement KTextEditorCompletion (code_editor.values.completion_placement)` | placement |
| K-P3-14† | `code_editor.annotation_border.stroke @ {hover}; code_editor.annotation_border.padding` | rule |
| K-P3-15† | `token.theme_change` | token |
| K-P3-16† | `token.behavior.key_repeat_delay_ms / key_repeat_interval_ms` | token |
| K-P3-16† | `token.behavior.double_click_distance` | token |
| K-P3-16† | `reader:kde:kcminputrc key repeat` | reader |
| K-P3-17† | `scrollbar.values.repeat_delay, scrollbar.values.repeat_interval (the arrows and the trough)` | field |
| K-P3-17† | `slider.values.repeat_delay, slider.values.repeat_interval (the page steps)` | field |
| K-P3-18† | `token.behavior.scroll (smooth wheel step, OutCubic, duration bounds)` | token |
| K-P3-18† | `reader:kde:kdeglobals [KDE] SmoothScroll` | reader |
| K-P3-19† | - | out (R1.1) |
| K-P3-20† | `Edge.snap (ADD-A33) on PartStyle.stroke` | field |
| K-P3-20† | `separator.line.stroke.snap` | field |
| K-P3-21† | - | out (R1.1) |
| K-P3-22† | - | out (R1.1) |
| K-P3-23† | - | out (R1.1) |
| K-P3-24† | `cond system_setting:ui_effects (ADD-A04)` | condition |
| K-P3-24† | `PartStyle.transitions @ !cond system_setting:ui_effects for menu, combo_box, tooltip` | rule |
| K-P3-24† | `reader:kde:kdeglobals [KDE] GraphicEffectsLevel` | reader |
| K-RV1† | `scrollbar.values.trough_click` | field |
| K-RV1† | `reader:kde:kdeglobals [KDE] ScrollbarLeftClickNavigatesByPage -> scrollbar.values.trough_click` | reader |
| K-RV2† | `menu.values.submenu_close_delay_ms` | field |
| K-RV2† | `menu.values.submenu_switch (CloseTimer)` | field |
| K-P4-1† | `window.values.window_menu_items` | field |
| K-P4-1† | `window.values.window_menu_placement (QtMenu: at the pointer, below the button)` | placement |
| K-P4-1† | `window.window_menu.bindings (the decoration palette)` | field |
| K-P4-1† | `window.values.title_bar_click_actions` | field |
| K-P4-1† | `reader:kde:kwinrc [MouseBindings] CommandActiveTitlebar3, CommandInactiveTitlebar3` | reader |
| K-P4-2† | - | out (R1.1) |
| K-P4-3† | - | out (R1.1) |
| K-P4-4† | - | out (R1.1) |
| K-P4-5† | - | out (R1.1) |
| K-P4-6† | - | out (R1.1) |
| K-P4-7† | `token.pointer.drag_cursors.<drop operation> (ADD-A46)` | token |
| K-P4-8† | - | preset (R1.3) |
| K-P4-9† | `text_area.body.padding` | field |
| K-P4-9† | `text_area.values.document_margin` | field |
| K-P4-10† | `toolbar_editor.available_list, toolbar_editor.current_list, toolbar_editor.move_button; menu.* @ context any toolbar` | rule |
| K-P4-11† | `token.behavior.spring_loading_ms (ADD-A45)` | token |
| K-P4-11† | `breadcrumb.values.drag_hover_delay_ms (the crumb's arrow opens its sub-folder menu)` | field |
| K-P4-11† | `list.values.drag_hover_delay_ms (QTreeView's autoExpandDelay; the Places panel's activation delay @ context sidebar.pane[@places]; both off by default)` | field |
| K-P4-12† | `IconRole.mirrors_in_rtl (S2.9)` | field |
| K-P4-12† | `icon lookup by direction (ADD-B05)` | field |
| K-P4-13† | `scrollbar.values.overscroll_shrink @ cond flavor:kirigami` | rule |
| K-P4-13† | `scrollbar.thumb.{min_width,min_height}` | field |
| K-P4-14† | `rich_text.heading_1 … heading_5, rich_text.paragraph, rich_text.preformatted, rich_text.inline_code, rich_text.block_quote, rich_text.definition_term, rich_text.definition_description, rich_text.values.block_margins @ variant html` | rule |
| K-KA-1† | `form.card.bindings (surface View) @ variant @ka_form_card cond flavor:kirigami` | rule |
| K-KA-1† | `form.card.stroke (= ka_card_border(background, text), ADD-A66) @ variant @ka_form_card` | rule |
| K-KA-1† | `form.card.shadow (one layer, blur by scheme) @ variant @ka_form_card; @ cond scheme:dark` | rule |
| K-KA-1† | `form.values.card_max_width (the widget's own breakpoint, R16)` | field |
| K-KA-1† | `form.card.corners, form.card.padding @ variants {@ka_form_card,@width_limited} (set by the connector from form.values.card_max_width, R16); square and edge to edge otherwise` | rule |
| K-KA-2† | `form.row.fill.append (= alpha(foreground, …)) @ {hover} !cond tablet_mode / {focus_visible} / {pressed} / {disabled} variant @ka_form_card` | rule |
| K-KA-2† | `form.row.transitions (Fill, Linear, = token.motion.durations.short) @ variant @ka_form_card` | rule |
| K-KA-2† | `form.row.corners @ pos first variants {@ka_form_card,@width_limited} / pos last variants {@ka_form_card,@width_limited} (concentric with form.card)` | rule |
| K-KA-2† | `form.row.focus (Absent) @ variant @ka_form_card` | rule |
| K-KA-3† | `form.row.padding, form.row_icon.icon.size, form.values.icon_text_gap @ variant @ka_form_card` | rule |
| K-KA-3† | `form.row_title.text (wrap, max_lines, elision End; ADD-A67) @ variant @ka_form_card` | rule |
| K-KA-3† | `form.row_title.text (max_lines one) @ variant @ka_form_card !variant button !variant switch !variant @check !variant @radio (a plain text row)` | rule |
| K-KA-3† | `form.row_subtitle.text.fill (= role.window.disabled.foreground), form.row_subtitle.padding @ variant @ka_form_card` | rule |
| K-KA-3† | `form.values.leading_gap, form.values.trailing_gap @ variants {button,@ka_form_card} / variants {switch,@ka_form_card} / variants {@check,@ka_form_card} / variants {@radio,@ka_form_card}` | rule |
| K-KA-4† | `form.row_separator.opacity, form.values.row_separator_inset @ variant @ka_form_card` | rule |
| K-KA-4† | `form.row_separator.opacity, form.values.row_separator_inset @ prev form.row {hover} / prev form.row {focus_visible} / prev form.row {pressed} / next form.row {hover} / next form.row {focus_visible} / next form.row {pressed}, each !cond tablet_mode (ADD-B07)` | rule |
| K-KA-4† | `form.row_separator.transitions (Opacity, Geometry; = token.motion.durations.long, InOutQuad)` | field |
| K-KA-5† | `form.group_title.text (Body font, weight DemiBold, wrap), form.group_title.bindings (surface View), form.group_title.padding @ variant @ka_form_card / variants {@ka_form_card,@width_limited}` | rule |
| K-KA-5† | `form.group_description.text.fill, form.group_description.max_width, form.group_description.padding @ variant @ka_form_card` | rule |
| K-KA-6† | `form.page.fill (= mix(role.window.background, role.view.background, …)) @ variant @ka_form_card` | rule |
| K-KA-6† | `form.values.page_margins (zero: the cards reach the page's edges) @ variant @ka_form_card !variant @width_limited (below the card breakpoint, R16)` | rule |
| K-KA-7† | `form.row_arrow.glyph (ThemeIcon), form.row_arrow.min_width, form.row_arrow.min_height, form.values.arrow_gap @ variant @ka_form_card; @ dir rtl` | rule |
| K-KA-7† | `form.row_arrow.opacity @ !{hover} !{focus_visible} !{pressed} variant @ka_form_card` | rule |
| K-KA-7† | `form.row_arrow.glyph @ variant @link (open-link icon)` | rule |
| K-KA-7† | `form.row_arrow.transform @ {open} variant expander; form.values.presentations (@expand, @collapse: height and opacity; = token.motion.durations.long, InOutQuad); form.row_separator @ {open} variant expander` | rule |
| K-KA-8† | `checkbox.*, radio.*, switch.* @ context form.row variant @ka_form_card (body.padding zero, label Absent)` | rule |
| K-KA-8† | `form.values.leading_gap @ variant @check / variant @radio; form.values.trailing_gap @ variant switch` | rule |
| K-KA-8† | `form.row_title.text.fill @ {disabled} variant @ka_form_card (a colour, not the qqc2 opacity)` | rule |
| K-KA-9† | `form.field_label.text (above the field; wrap, max_lines ADD-A67), form.field_label.text.fill @ {disabled}, form.values.field_label_gap @ variant @ka_form_card` | rule |
| K-KA-9† | `form.group_description.text.fill under the field @ variant @ka_form_card` | rule |
| K-KA-9† | `input.counter.text (smallest_readable), input.counter.text.fill @ {@at_limit} context form.row variant @ka_form_card` | rule |
| K-KA-9† | `form.row_suffix.text @ variant @ka_form_card context slider (the value beside the label)` | rule |
| K-KA-9† | `form.row_edit_icon.icon, form.row_subtitle.text.fill @ variants {entry,@ka_form_card} cond mobile_form_factor; button.* @ context form.row (the Done button)` | rule |
| K-KA-10† | `banner.body.fill (= mix(background, type colour, …)), banner.body.corners, banner.body.padding @ variants {@ka_inline_message,info} / variants {@ka_inline_message,success} / variants {@ka_inline_message,warning} / variants {@ka_inline_message,error}` | rule |
| K-KA-10† | `banner.body.stroke Absent, banner.icon.icon (size, fill = the type colour), banner.values.content_gap @ variant @ka_inline_message` | rule |
| K-KA-11† | `input.strength_meter.fill (= role.<set>.background_negative / _neutral / _positive), input.strength_meter_fill.fill (the matching text role), input.strength_meter.corners, input.strength_meter.min_height, input.values.strength_meter_gap @ variant secure {@quality_bad} / {@quality_poor} / {@quality_weak} / {@quality_good} / {@quality_excellent}` | rule |
| K-KA-12† | `combo_box.* @ context form.row variant @ka_form_card !cond mobile_form_factor` | rule |
| K-KA-12† | `form.row (clickable, K-KA-2† fills), form.row_subtitle.text.fill, form.row_arrow @ variants {combo,@ka_form_card} cond mobile_form_factor; its popup a @ka_list_dialog dialog or a full_size dialog (the next line)` | rule |
| K-KA-12† | `dialog.values.natural_size, dialog.header.stroke @ {scrolled}, dialog.header.padding @ variant @ka_list_dialog; list.* @ variant @ka_rounded context dialog variant @ka_list_dialog; dialog.* @ variant full_size (page mode)` | rule |
| K-KA-13† | `input, button (icon only) @ context form.row variant @ka_form_card; autocomplete.popup (fill = the K-KA-18† surface), autocomplete.values.width (as the field), autocomplete.values.max_visible_items; list.* @ variant @ka_rounded context autocomplete` | rule |
| K-KA-13† | `autocomplete.values.placement (below the field)` | placement |
| K-KA-13† | `banner @ variants {@ka_inline_message,error} context form.row; form.row_subtitle.text.fill, form.row_arrow @ cond mobile_form_factor` | rule |
| K-KA-14† | `color_button.swatch (corners Full, min_width, min_height, stroke Absent), color_button.values.swatch_gap @ context form.row variant @ka_form_card` | rule |
| K-KA-14† | `form.row_icon.icon.size @ variants {@icon_row,@ka_form_card}` | rule |
| K-KA-15† | `form.values.grid_columns, form.values.grid_min_cell_width, form.values.grid_gap; form.card @ variant @ka_info_card (padding, corners @ variant @width_limited)` | rule |
| K-KA-15† | `form.row_icon, form.row_title.text (= token.text.keyed.heading_4_primary), form.row_subtitle.opacity @ variant @ka_info_card; form.row.fill.append @ {hover} / {focus_visible} / {pressed} variant @ka_info_card; form.row.transitions` | rule |
| K-KA-15† | `tooltip.values.show_delay_ms @ cond flavor:kirigami` | rule |
| K-KA-16† | `status_page.icon.icon.size, status_page.icon.opacity, status_page.title.text (= token.text.keyed.heading_3_primary), status_page.title.opacity, status_page.description.opacity, status_page.values.icon_to_title, status_page.values.title_to_description, status_page.values.margins @ variant @ka_card_placeholder` | rule |
| K-KA-17† | `dialog.* @ variant @ka_form_card_dialog (body: K-KA-18† surface); dialog.values.natural_size, dialog.values.presentation_breakpoint (width), dialog.values.window_margin` | rule |
| K-KA-17† | `dialog.header.padding @ variant @ka_form_card_dialog; dialog.close_button (Absent) @ !{@has_cancel} variant @ka_form_card_dialog; dialog.close_button.glyph @ {hover} variant @ka_form_card_dialog; dialog.action_area.padding, dialog.values.action_layout.row_gap @ variant @ka_form_card_dialog` | rule |
| K-KA-18† | `dialog.body.corners, dialog.body.bindings (surface View), dialog.body.stroke, dialog.body.shadow @ variant @ka_dialog cond flavor:kirigami` | rule |
| K-KA-18† | `popover.body.corners, popover.body.bindings, popover.body.stroke, popover.body.shadow @ variant @ka_dialog cond flavor:kirigami` | rule |
| K-KA-19† | `dialog.* @ variants {alert,@ka_message_dialog}: dialog.icon.icon.size, dialog.values.type_icons, dialog.body.padding, dialog.values.natural_size, dialog.values.window_margin` | rule |
| K-KA-19† | `dialog.icon.padding, dialog.values.title_alignment @ variants {@ka_message_dialog,narrow} (set by the connector from dialog.values.presentation_breakpoint, R16); dialog.values.action_layout.stack_rule` | rule |
| K-KA-19† | `checkbox @ context dialog.action_area; dialog.values.action_layout.row_gap, dialog.action_area.padding, dialog.values.standard_button_icons @ variant @ka_message_dialog` | rule |
| K-KA-19† | `dialog.values.presentations (Appear, Disappear: Fade; = token.motion.durations.long, InOutQuad), dialog.scrim.fill @ variant @ka_message_dialog` | rule |
| K-KA-20† | `date_picker.popup (the K-KA-18† surface), date_picker.popup.min_width, date_picker.popup.padding @ variant @ka_date_popup; dialog.scrim.fill, dialog.scrim.transitions @ context date_picker` | rule |
| K-KA-20† | `segmented_control @ variant @ka_segmented_button context date_picker.header; button (icon only) @ context date_picker.header; glyph @ dir rtl` | rule |
| K-KA-20† | `calendar.values.presentations (@view_switch); calendar.values.scroll_physics (deceleration, snap; ScrollPhysics)` | field |
| K-KA-20† | `dialog.values.standard_button_icons, dialog.action_area.padding @ context date_picker variant @ka_date_popup` | rule |
| K-KA-21† | `calendar.weekday_label.opacity, calendar.day.corners, calendar.day.text @ variant @ka_date_popup` | rule |
| K-KA-21† | `calendar.day.fill (= kirigami_tint, ADD-A52) @ {today} / {selected} / {hover}; calendar.day.stroke @ {focus_visible} variant @ka_date_popup` | rule |
| K-KA-21† | `calendar.day.fill Absent, calendar.day.opacity @ {other_month}; calendar.day @ {disabled} ({@out_of_range}); calendar.month.fill @ {today}; calendar.year.opacity @ {@other_decade} variant @ka_date_popup` | rule |
| K-KA-22† | `date_picker.field, time_picker.field @ context form.row variant @ka_form_card (each a flat button: icon, text, fill.append @ {hover} / {focus_visible} / {pressed})` | rule |
| K-KA-22† | `date_picker.segment_separator.opacity, date_picker.segment_separator.padding @ context form.row; @ prev / next {hover} (ADD-B07)` | rule |
| K-KA-22† | `form.field_label.padding, form.group_description.padding @ variants {@ka_form_card,@date_time}; date_picker.field.fill @ {read_only}` | rule |
| K-KA-23† | `time_picker.flyout (the K-KA-18† surface) @ variant @ka_time_popup; dialog.scrim @ context time_picker` | rule |
| K-KA-23† | `picker_wheel.values.item_height, picker_wheel.values.visible_items, picker_wheel.item.text.font.size (= mul(font_size, …), ADD-A56) @ context time_picker variant @ka_time_popup; picker_wheel.item.opacity by distance (ADD-B13)` | rule |
| K-KA-23† | `time_picker.time_separator.text.font.size (= mul(font_size, …)) @ variant @ka_time_popup` | rule |
| K-KA-24† | `segmented_control.segment.padding, segmented_control.segment.min_width (= height, ADD-B11), segmented_control.icon.icon.size, segmented_control.values.item_gap (negative) @ variant @ka_segmented_button` | rule |
| K-KA-24† | `segmented_control.segment.corners @ pos first / pos last variant @ka_segmented_button; @ dir rtl` | rule |
| K-KA-24† | `segmented_control.segment.fill @ {pressed} / {checked}; segmented_control.segment.stroke @ {hover} / {focus_visible} / {pressed} / {checked} variant @ka_segmented_button (the highlighted segment drawn above its neighbours: the connector's paint order)` | rule |
| K-KA-24† | `segmented_control.segment.transitions (Fill, Stroke; = token.motion.durations.short, OutCubic) @ {hover} / {focus_visible} / {pressed} / {checked} variant @ka_segmented_button (the Behaviors run only while the segment is highlighted)` | rule |
| K-KA-24† | `segmented_control.segment.shadow (a hard layer, one pixel lower) @ !{pressed} variant @ka_segmented_button` | rule |
| K-KA-25† | `segmented_control.segment.fill.append @ {hover}, segmented_control.segment.corners, segmented_control.segment.min_height, segmented_control.icon.icon.size @ variant @ka_radio_selector; segmented_control.segment.transitions @ !cond input:touch` | rule |
| K-KA-25† | `segmented_control.indicator.fill, segmented_control.indicator.stroke, segmented_control.indicator.corners, segmented_control.indicator.shadow @ variant @ka_radio_selector; segmented_control.values.presentations (Move, part indicator; = token.motion.durations.long, InOutCubic)` | rule |
| K-KA-25† | `segmented_control.label.text.font.weight, segmented_control.icon.icon.fill @ {checked} variant @ka_radio_selector (transitions); form.row.padding @ context segmented_control variant @ka_radio_selector` | rule |
| K-KA-26† | `button.* @ variant @ka_floating: body.bindings (surface Button), icon.icon.size, body.corners, body.padding; body.padding @ cond input:touch` | rule |
| K-KA-26† | `button.body.fill (= kirigami_tint, ADD-A52) @ {hover} / {pressed} / {focused}, button.body.stroke, button.body.shadow, button.body.transitions @ variant @ka_floating; button.values.hit_extension` | rule |
| K-KA-27† | `split_button.* @ variant @ka_double_floating: primary.corners, secondary.corners (outer only; dir rtl), body shadow shared, fills as K-KA-26†, transitions (= token.motion.durations.long, OutCubic); tooltip.values.show_delay_ms` | rule |
| K-KA-28† | `toolbar.bar.bindings (surface Window), toolbar.bar.corners, toolbar.bar.shadow, toolbar.bar.stroke @ variant floating cond flavor:kirigami` | rule |
| K-KA-29† | `banner.body.fill (= role.<set>.background_negative / _neutral / _positive / _active), banner.body.padding, banner.values.content_gap @ variant @ka_banner / variants {@ka_banner,@header} / variants {@ka_banner,@footer}` | rule |
| K-KA-29† | `banner.edge_line.stroke (= ka_banner_line(type text, frame colour), ADD-A66) @ variants {@ka_banner,@header} (bottom) / variants {@ka_banner,@footer} (top)` | rule |
| K-KA-29† | `banner.icon (size, top-aligned), banner.title.text (= token.text.keyed.heading_2_primary), banner.close_button, banner.action_area @ variant @ka_banner; banner.values.presentations (Instant); tooltip.values.show_on_press @ cond tablet_mode` | rule |
| K-KA-30† | `sheet.* @ variants {bottom,@ka_bottom_drawer}: sheet.values.max_width, sheet.values.detents (height limit), sheet.body.corners, sheet.header.corners, sheet.body.shadow` | rule |
| K-KA-30† | `sheet.handle.min_width, sheet.handle.min_height, sheet.handle.fill, sheet.handle.corners, sheet.handle.padding; sheet.header.bindings (surface Window), sheet.header_separator, sheet.body.bindings (surface View) @ variant @ka_bottom_drawer` | rule |
| K-KA-31† | `menu.* (the qqc2 Menu, Q1.23–Q1.32) / @ variant @ka_bottom_drawer (the default under cond mobile_form_factor) / variant @ka_dialog (the connector sets it from the application's display mode)` | rule |
| K-KA-31† | `sheet.*, form.row.* @ variant @ka_form_card context menu variant @ka_bottom_drawer; checkbox.*, radio.* @ context form.row; form.row_arrow (submenu); button.* @ variant back context sheet.header` | rule |
| K-KA-31† | `dialog.* @ context menu variant @ka_dialog: dialog.values.natural_size, dialog.header.padding, dialog.close_button, dialog.header.stroke` | rule |
| K-KA-32† | `search_field.field.opacity @ {focused}, search_field.view (the K-KA-18† surface), search_field.view_divider, search_field.values.presentations (@view_open: Fade with Scale and Geometry together, ADD-B10) @ variant @ka_search_popup` | rule |
| K-KA-32† | `search_field.values.view_placement (KirigamiAddonsSearchPopup, ADD-A21)` | placement |
| K-KA-32† | `search_field.values.auto_search_min_chars; list.* @ context search_field.view (surface View)` | rule |
| K-KA-33† | `avatar.values.palette (ADD-B14), avatar.body.stroke (ring width), avatar.initials.text.font.size (= own size, ADD-B11), avatar.initials.text.fill (= ka_contrast_text, ADD-A66), avatar.fallback_icon @ variant @ka_avatar` | rule |
| K-KA-33† | `avatar.body.padding, avatar.body.focus (FilledRing) @ {focus_visible} / {pressed}, avatar.body.cursor, avatar.body.bindings (surface Button) @ variants {@ka_avatar,@button}; tooltip.values.show_delay_ms` | rule |
| K-KA-34† | `component.album_viewer.scrim, component.album_viewer.header, component.album_viewer.header_separator, component.album_viewer.title, component.album_viewer.subtitle, component.album_viewer.close_button, component.album_viewer.footer, component.album_viewer.footer_separator, component.album_viewer.image` | component |
| K-KA-34† | `component.album_viewer.values.image_inset, component.album_viewer.image.transitions (the zoom and the rotation: Transform, = token.motion.durations.long, InOutCubic); button.* @ variant @ka_floating context component.album_viewer; spinner.* @ context component.album_viewer` | component |
| K-KA-35† | `media_controls.bar (opacity, corners, stroke, shadow), media_controls.button, media_controls.time_label, media_controls.volume_popup @ variant @ka_album; slider.* @ context media_controls; slider.* @ variant vertical context media_controls.volume_popup` | rule |
| K-KA-35† | `media_controls.values.auto_hide_ms, media_controls.values.volume_popup_hide_delay_ms; media_controls.play_overlay (download button, icon size) @ variant @ka_album` | rule |
| K-KA-36† | `list.header.min_height (= list.row.min_height per density), list.header_section.padding, list.header_section.fill @ {hover}, list.header_separator.stroke (right and bottom) @ variants {table,@ka_table}` | rule |
| K-KA-36† | `list.header_section.text.elision, list.values.elided_tooltip, list.sort_indicator.glyph (ThemeIcon), list.sort_indicator.transform @ {descending}, list.sort_indicator.transitions @ variant @ka_table` | rule |
| K-KA-36† | `list.header_section.cursor @ {dragged}, list.header_section.stroke @ {drop_target}, list.values.column_min_width @ variant @ka_table` | rule |
| K-KA-37† | `list.row_background.fill @ {selected} / {pressed} / {hover} variant @ka_table` | rule |
| K-KA-37† | `list.row_background.fill @ pos parity odd variants {alternating_rows,@ka_table} (ADD-B06)` | rule |
| K-KA-37† | `list.cell.stroke @ {current} variant @ka_table; menu.* @ context list variant @ka_table` | rule |
| K-KA-38† | `list.values.row_background_inset, list.row_background.corners @ variant @ka_rounded; list.values.row_background_inset @ pos first / pos last variant @ka_rounded` | rule |
| K-KA-38† | `list.row_background.fill (= kirigami_tint, ADD-A52) @ {selected} / {current} / {pressed} / {focus_visible} / {hover} / {selected,hover}; list.row_background.stroke @ {focused} variant @ka_rounded` | rule |
| K-KA-38† | `list.row.text.fill @ {selected} (unchanged), list.row.padding, list.values.leading_gap, list.row.min_height, list.leading_icon.icon.size @ variant @ka_rounded / variant @ka_rounded cond tablet_mode / variants {@ka_rounded,@with_subtitle}` | rule |
| K-KA-38† | `checkbox, radio @ context list.row[@ka_rounded,@check] / context list.row[@ka_rounded,@radio]; list.row_background.fill @ {checked} (no fill)` | rule |
| K-KA-39† | `list.secondary_text.text (= token.text.keyed.smallest_readable), list.secondary_text.opacity @ variant @ka_rounded / variants {@ka_rounded,@bold}; list.leading_icon.padding (the icon's margins) @ variants {@ka_rounded,@with_subtitle}` | rule |
| K-KA-39† | `list.values.row_background_inset, list.row_background.corners, list.row.padding, list.leading_icon.icon.size @ variant @ka_indicator; list.leading_icon.icon.size @ variants {@ka_indicator,@with_subtitle}` | rule |
| K-KA-39† | `list.unread_indicator.fill, list.unread_indicator.min_width, list.unread_indicator.padding @ {@unread} variant @ka_indicator` | rule |
| K-KA-39† | `list.row_separator.opacity, list.values.row_separator_inset @ variant @ka_indicator; list.row_separator.opacity @ prev list.row {hover} / next list.row {hover} (ADD-B07)` | rule |
| K-KA-40† | `list.branch_line.stroke, list.disclosure.glyph (ThemeIcon; dir rtl), list.disclosure.glyph.size @ variant @ka_tree !cond system_setting:qqc2_desktop_style (a Quick Controls style other than org.kde.desktop, as on Plasma Mobile; Breeze's otherwise, Q1.85)` | rule |
| K-KA-40† | `list.disclosure.glyph.fill @ {hover}, list.disclosure.transitions; list.values.indent_per_level, list.values.indent_includes_disclosure @ variant @ka_tree` | rule |
| K-KA-41† | `window.values.default_size, window.content.min_width, window.content.min_height @ variant @ka_settings` | rule |
| K-KA-41† | `sidebar.values.width, sidebar.pane.bindings (surface View), sidebar.header.min_height (as toolbar.bar.min_height of the global toolbar), search_field @ context sidebar.header, list.* @ variants {sidebar,@ka_rounded} ({checked} drawn as {selected}), button.* @ variant back context sidebar.header; each @ context window variant @ka_settings` | rule |
| K-KA-41† | `layout.values.column_width @ variant @column_view context window variant @ka_settings` | rule |
| K-KA-41† | `form @ variant @ka_form_card cond mobile_form_factor: form.row_icon.icon.size, form.values.icon_text_gap, form.row_arrow.glyph.size (= mul(token.icon_sizes.small, …)), form.row_separator` | rule |
| K-KA-42† | `dialog @ variants {@search_dialog,@ka_dialog}; list.* @ variant @ka_rounded context dialog variant @search_dialog; list.trailing_text.text.fill (= role.window.disabled.foreground); status_page.* @ context dialog variant @search_dialog` | rule |
| K-KA-43† | `shortcuts_dialog @ variant @ka_quick: shortcuts_dialog.values.natural_size, search_field @ context shortcuts_dialog, shortcuts_dialog.row (list rows @ka_rounded), shortcuts_dialog.footer.padding` | rule |
| K-KA-43† | `shortcuts_dialog.key_sequence_button (icon, text) @ {checked} / {@recording}; shortcuts_dialog.clear_button (square; glyph @ dir rtl) / @ {@recording}; dialog.* @ variants {@ka_message_dialog,warning} context shortcuts_dialog` | rule |
| K-KA-44† | `about_dialog @ variant @ka_about cond flavor:kirigami: about_dialog.values.margins, about_dialog.icon.icon.size, about_dialog.values.icon_max_width_fraction, about_dialog.name.text, about_dialog.version.text, about_dialog.description.text (= token.text.keyed.heading_3_secondary)` | rule |
| K-KA-44† | `badge @ context about_dialog; dialog.* @ variant @ka_dialog context about_dialog (release history: natural_size, close button, separator, about_dialog.values.release_gap)` | rule |
| K-KA-44† | `dialog.values.natural_size @ variant @ka_dialog context about_dialog (the page opened as a square dialog layer, a grid-unit multiple)` | rule |
| K-KA-44† | `form @ variant @ka_info_card context about_dialog (link rows); about_dialog.person_name.text, about_dialog.person_task.text (= token.text.keyed.smallest_readable, disabled colour); avatar.body.min_width @ context about_dialog; about_dialog.link_button; sheet.* @ context about_dialog (licence text margin)` | rule |
| K-KA-45† | `teaching_tip.target_frame.stroke, teaching_tip.target_frame.transitions (Geometry; = token.motion.durations.long, InOutQuad) @ variant @ka_onboarding` | rule |
| K-KA-45† | `teaching_tip.backdrop.fill (Material blur over everything but the target) @ variant @ka_onboarding` | rule |
| K-KA-45† | `teaching_tip.body.bindings (surface Tooltip), teaching_tip.body.corners, teaching_tip.body.stroke, teaching_tip.body.padding, teaching_tip.body.min_height, teaching_tip.message.max_width, teaching_tip.values.light_dismiss (never times out) @ variant @ka_onboarding` | rule |
| K-KA-45† | `teaching_tip.values.placement (KirigamiAddonsOnboardingTip: above the control, below without room; ADD-A21)` | placement |
| K-KA-46† | `list.* @ variant @ka_rounded (the sound list); list.selection_check.glyph (ThemeIcon object-select-symbolic) @ {checked} variant @ka_rounded` | rule |
| K-KA-47† | `icon_theme_fallback (the preset's icon fields, S4.5: Breeze for an application started through KirigamiApp; ADD-A65)` | field |
| K-KA-47† | - (the org.kde.breeze Quick style KirigamiApp chooses on Android and iOS: there Kirigami is another toolkit on that platform, a candidate preset of its own) | preset (R1.3) |
