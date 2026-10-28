# v0.7.1 — Model completeness: Specification, traceability (Material)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-material.md`](todo_v0.7.1_model-completeness-gap-map-material.md) mapped to
what carries it (R24), one line per (row, target), the rows in the order of the platform file. Withdrawn rows and
the pointer rows the map does not count are not listed; the cross-check and pass decision tables are decisions,
not rows.

**Targets** use the widget catalogue's names
([`todo_v0.7.1_model-completeness-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md), after its
rename table): `widget.part.field` (a `PartStyle` field, S2.11), `widget.values.name` (a widget value),
`component.<key>.<part>` (a keyed component), `role.<set>.<role>`, `role.global.<role>`, `role.@<key>` (S3),
`token.<path>` (S4), `input.<name>` with kind `input` (an S9 input, not the `input` widget), `reader:<platform>:<what
is read>` (S9.3), a placement strategy, a system control (S15), a condition or flag, or a core type
(`PartStyle.…`, `FocusIndicator.…`) where the row concerns every part. The selector of a rule follows `@`:
`{a,b}` states that hold together, `!{a}` or `not {a}` states absent, `variant v` (`@v` keyed, `!` negated),
`cond c` (a condition, S7.1; `flavour:kirigami`), `context w.part` or `ctx any w` (an ancestor), `pos first|last|only`
and `prev`/`next` (siblings), `size s`, `size_class`, `h_size_class`/`v_size_class`, `dir rtl`; `/` separates
alternatives, each a rule of its own. A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| MAT-W1 | `window.values.status_bar_style (system-bar icon tone)` | field |
| MAT-W1 | `window.values.navigation_bar_contrast_scrim` | field |
| MAT-W1 | - | readers (R18) |
| MAT-W2 | `window.values.presentations event @predictive_back` | field |
| MAT-W2 | - | out (R1.1) |
| MAT-W4† | - | out (R1.1) |
| MAT-W5† | `window.frame.corners (per corner)` | field |
| MAT-W5† | - | readers (R18) |
| MAT-B1 | `button.body.fill, button.label.text.fill @ variant @elevated / primary (filled) / tinted (tonal) / @outlined / flat (text)` | rule |
| MAT-B1 | `button.body.stroke @ variant @outlined` | rule |
| MAT-B1 | `button.body.shadow @ variant @elevated (Shadow::Elevation)` | rule |
| MAT-B2 | `button.body.fill, button.label.text.fill @ {hover} / {focus_visible} / {pressed} variant primary (state layers appended)` | rule |
| MAT-B2 | `button.body.fill, button.label.text.fill @ {disabled} variant primary` | rule |
| MAT-B3 | `button.body.stroke @ {hover} / {focus_visible} / {pressed} / {disabled} variant @outlined` | rule |
| MAT-B4 | `button.body.padding, button.body.fill @ variant flat` | rule |
| MAT-B4 | `button.label.text.fill @ {hover} / {focus_visible} / {pressed} / {disabled} variant flat` | rule |
| MAT-B5 | `button.body.fill, button.label.text.fill, button.body.shadow @ {focus_visible}` | rule |
| MAT-B6 | `button.body.shadow @ {hover} / {focus_visible} / {pressed} / {disabled} per variant (Shadow::Elevation)` | rule |
| MAT-B7 | `button.icon.icon.size` | field |
| MAT-B7 | `button.icon.icon.fill @ {hover} / {focus_visible} / {pressed} / {disabled}; button.icon.opacity @ {disabled}` | rule |
| MAT-B8 | `button.body.padding @ variant with_icon` | rule |
| MAT-B9 | `button.body.min_height, button.icon.icon.size, button.values.icon_text_gap, button.body.padding, button.label.text @ size mini / small / regular / large / extra_large cond flavour:expressive` | rule |
| MAT-B9 | `button.body.stroke @ size … variant @outlined cond flavour:expressive` | rule |
| MAT-B10 | `button.body.corners @ variant @square cond flavour:expressive` | rule |
| MAT-B10 | `button.body.corners @ {checked} variant toggle; @ {pressed} (per size) cond flavour:expressive` | rule |
| MAT-B11 | `button.body.transitions (Corners, Timing::Spring) cond flavour:expressive (base and @ {pressed})` | rule |
| MAT-B12 | `button.body.fill @ !{checked} variant toggle; @ {checked} variant toggle` | rule |
| MAT-B12 | `button.body.fill, button.label.text.fill, button.icon.icon.fill @ {hover\|focus_visible\|pressed} × {checked or !checked} variant toggle` | rule |
| MAT-B13 | `button.body.transitions (Shadow)` | field |
| MAT-B13 | `token.motion.durations (elevation), token.motion.timings.emphasized` | token |
| MAT-B15 | `button.label.text.letter_spacing, button.label.text.line_height (= token.text label_large)` | field |
| MAT-I1 | `input.body.stroke @ {focused} / {hover} / {disabled}` | rule |
| MAT-I2 | `input.body.stroke @ {disabled}` | rule |
| MAT-I3 | `input.floating_label.text` | field |
| MAT-I3 | `input.floating_label.text.fill @ {hover} / {focused} / {invalid} / {disabled}` | rule |
| MAT-I3 | `input.floating_label.{text,transform} @ {focused}; @ not {empty}` | rule |
| MAT-I3 | `input.values.floating_label_gap` | field |
| MAT-I3 | `flag empty (input)` | flag |
| MAT-I4 | `input.values.notch_padding` | field |
| MAT-I5 | `input.supporting_text.{text,padding}` | field |
| MAT-I5 | `input.supporting_text.text.fill @ {invalid} / {disabled}` | rule |
| MAT-I5 | `input.values.supporting_text_gap` | field |
| MAT-I6 | `input.counter.text` | field |
| MAT-I7 | `input.icon.icon.{size,fill}` | field |
| MAT-I7 | `input.icon.icon.fill @ {hover} / {focused} / {invalid} / {disabled}` | rule |
| MAT-I7 | `input.values.icon_text_gap` | field |
| MAT-I8 | `input.prefix.text; input.suffix.text` | field |
| MAT-I8 | `input.values.affix_gap` | field |
| MAT-I9 | `input.{body.stroke,caret.fill,text.text.fill,floating_label.text.fill,supporting_text.text.fill,icon.icon.fill} @ {invalid}; @ {invalid,hover}; @ {invalid,focused}` | rule |
| MAT-I10 | `input.{text.text.fill,floating_label.text.fill} @ {hover} / {focused}` | rule |
| MAT-I11 | `input.body.{fill,corners,stroke} @ variant @filled` | rule |
| MAT-I11 | `input.body.fill.append (state layer) @ {hover} variant @filled` | rule |
| MAT-I11 | `input.body.stroke (bottom edge: active indicator) @ variant @filled with {hover} / {focused} / {invalid} / {disabled}` | rule |
| MAT-I11 | `input.body.fill @ {disabled} variant @filled` | rule |
| MAT-I12 | `token.caret.{width,blink_on_ms,blink_off_ms}` | token |
| MAT-I12 | - | readers (R18) |
| MAT-I13 | `input.selection_handle.{glyph,fill}` | field |
| MAT-I14 | `input.floating_label.transitions; input.placeholder.transitions` | field |
| MAT-I14 | `input.body.transitions (stroke: the notch opening)` | field |
| MAT-I15 | `input.text.text.{letter_spacing,line_height}` | field |
| MAT-I16 | `text_area.values.min_rows; text_area.values.resize` | field |
| MAT-I17† | `input.magnifier.{corners,shadow,min_width,min_height}` | field |
| MAT-I17† | `input.values.magnifier_zoom; input.values.magnifier_offset` | field |
| MAT-I18† | `input.preedit.text.decoration` | field |
| MAT-I18† | `input.misspelling.text.decoration; input.correction.text.decoration` | field |
| MAT-I19† | `input.values.mask_character; input.values.mask_reveal_ms` | field |
| MAT-I19† | - | readers (R18) |
| MAT-C1 | `radio.ring.stroke, radio.dot.fill @ {checked}; @ {hover} / {focus_visible} / {pressed} / {disabled} × {checked or !checked}` | rule |
| MAT-C1 | `radio.ring.opacity, radio.dot.opacity @ {disabled}` | rule |
| MAT-C2 | `checkbox.state_layer (part: min_width, min_height, corners Full); radio.state_layer` | field |
| MAT-C2 | `checkbox.state_layer.fill, radio.state_layer.fill @ {hover\|focus_visible\|pressed} × {checked or !checked}` | rule |
| MAT-C3 | `checkbox.box.fill, checkbox.mark.glyph.fill, checkbox.box.stroke.width @ {checked,hover} / {checked,focus_visible} / {checked,pressed}` | rule |
| MAT-C4 | `checkbox.box.stroke @ {hover} / {focus_visible} / {pressed} !{checked,indeterminate}` | rule |
| MAT-C5 | `checkbox.box.fill, checkbox.mark.glyph.fill @ {invalid,checked} and × {hover\|focus_visible\|pressed}` | rule |
| MAT-C5 | `checkbox.box.stroke @ {invalid} !{checked}; checkbox.state_layer.fill @ {invalid,hover\|focus_visible\|pressed}` | rule |
| MAT-C6 | `checkbox.mark.glyph @ {indeterminate} (Path)` | rule |
| MAT-C6 | `checkbox.box.fill, checkbox.mark.glyph.fill @ {indeterminate}` | rule |
| MAT-C7 | `checkbox.box.stroke @ {disabled} !{checked}; checkbox.mark.glyph.fill @ {checked,disabled}; checkbox.box.opacity @ {checked,disabled}` | rule |
| MAT-C8 | `checkbox.mark.opacity, checkbox.mark.transform @ {checked}; checkbox.mark.transitions (base and @ {checked})` | rule |
| MAT-C8 | `radio.dot.transform @ {checked}; radio.dot.transitions` | rule |
| MAT-C8 | `checkbox.box.transitions (Fill, Stroke)` | field |
| MAT-C8 | `token.motion.timings.emphasized_decelerate, token.motion.timings.emphasized_accelerate` | token |
| MAT-C9 | `checkbox.values.touch_target; radio.values.touch_target` | field |
| MAT-M1 | `menu.{item.fill,item_label.text.fill,item_icon.icon.fill,item_trailing_icon.icon.fill} @ {selected}` | rule |
| MAT-M2 | `menu.item.fill.append (state layer) @ {focus_visible} / {pressed}` | rule |
| MAT-M2 | `menu.item_label.text.fill @ {focus_visible} / {pressed}` | rule |
| MAT-M3 | `menu.{item_icon,item_trailing_icon}.icon.fill @ {hover} / {focus_visible} / {pressed} / {disabled}` | rule |
| MAT-M3 | `menu.item_trailing_icon.icon.size` | field |
| MAT-M3 | `menu.item_icon.opacity @ {disabled}` | rule |
| MAT-M4 | `menu.item_subtitle.text` | field |
| MAT-M4 | `menu.accelerator.text` | field |
| MAT-M5 | `menu.submenu_arrow.glyph` | field |
| MAT-M5 | `menu.values.{submenu_open_delay_ms,submenu_close_delay_ms}` | field |
| MAT-M6 | `menu.separator.{min_height,padding}` | field |
| MAT-M7 | `menu.body.padding (top, bottom)` | field |
| MAT-M8 | `menu.body.{min_width,max_width}` | field |
| MAT-M9 | `menu.values.presentations (ClipReveal; item fade with stagger; per part, ADD-B09, ADD-B10, ADD-A25)` | field |
| MAT-M10 | `menu.body.corners @ cond flavour:expressive` | rule |
| MAT-M10 | `menu.item.corners @ cond flavour:expressive pos first / last / {selected}` | rule |
| MAT-M10 | `menu.group` | field |
| MAT-M10 | `menu.values.group_gap_px` | field |
| MAT-M10 | `menu.* @ variant @vibrant cond flavour:expressive` | rule |
| MAT-M10 | `menu.* @ variant @horizontal / icon_only` | rule |
| MAT-M10 | `menu.section_header @ cond flavour:expressive` | rule |
| MAT-M10 | `menu.item.{min_height,padding} @ cond flavour:expressive` | rule |
| MAT-M11 | `menu.item_label.text.{letter_spacing,line_height}` | field |
| MAT-T1 | `tooltip.* @ variant rich` | rule |
| MAT-T1 | `tooltip.title.text` | field |
| MAT-T1 | `tooltip.body.{fill,shadow,corners} @ variant rich` | rule |
| MAT-T1 | `button.* @ context tooltip.body variant rich (actions)` | rule |
| MAT-T2 | `tooltip.values.display_time` | field |
| MAT-T3 | `tooltip.values.presentations (fade with scale, ADD-B10)` | field |
| MAT-T4 | `tooltip.values.placement (MaterialTooltip gap_px)` | placement |
| MAT-T4 | `tooltip.arrow.{min_width,min_height,glyph}` | field |
| MAT-T5 | `tooltip.body.min_height` | field |
| MAT-T6 | `tooltip.values.{show_on_press,show_delay_ms} @ cond input:touch (= token.behaviour.long_press_ms)` | rule |
| MAT-T6 | - | readers (R18) |
| MAT-T7 | `tooltip.label.text.{letter_spacing,line_height}` | field |
| MAT-SC1 | `scrollbar.values.auto_hide_delay; scrollbar.values.presentations` | field |
| MAT-SC1 | - | readers (R18) |
| MAT-SC2 | `token.behaviour.scroll.overscroll (with the overscroll effect, ADD-A29)` | token |
| MAT-SC3† | `scrollbar.thumb.{fill,min_width,min_height}; scrollbar.track.fill; scrollbar.body.padding` | field |
| MAT-SC3† | `scrollbar.values.{thumb_max_fraction,auto_hide_delay,presentations,always_visible}` | field |
| MAT-SC4† | - | newest (R1.5) |
| MAT-S1 | `slider.thumb.{min_width,min_height,corners}` | field |
| MAT-S1 | `slider.thumb.{min_width,min_height} @ {hover} / {focused} / {pressed} / {disabled}` | rule |
| MAT-S2 | `slider.values.thumb_track_gap` | field |
| MAT-S3 | `slider.track.{corners,min_height}; slider.fill.{corners,min_height}` | field |
| MAT-S4 | `slider.stop_indicator.{fill,min_width,min_height,corners}` | field |
| MAT-S4 | `slider.stop_indicator.fill @ {in_fill}; @ {disabled}` | rule |
| MAT-S4 | `slider.values.stop_indicator_gap` | field |
| MAT-S5 | `slider.tick.fill @ {in_fill}; @ {disabled}` | rule |
| MAT-S5 | `slider.tick.corners` | field |
| MAT-S6 | `slider.value_indicator.{fill,min_height,corners,text}` | field |
| MAT-S6 | `slider.values.value_indicator_gap` | field |
| MAT-S6 | `slider.values.presentations (value_indicator: Appear, Disappear)` | field |
| MAT-S6 | `slider.values.presentations @ cond reduced_motion` | rule |
| MAT-S7 | `slider.halo.{fill,min_width,min_height} @ {hover} / {focused} / {pressed}` | rule |
| MAT-S8 | `slider.thumb.fill @ {focused} / {pressed}` | rule |
| MAT-S8 | `slider.{fill,track}.fill @ {focused} / {pressed}` | rule |
| MAT-S9 | `slider.thumb.shadow (elevation entry); slider.track.shadow` | field |
| MAT-S9 | `slider.thumb.shadow @ {disabled}` | rule |
| MAT-S10 | `slider.thumb.stroke @ {overlapping}` | rule |
| MAT-S10 | `flag overlapping (slider thumb)` | flag |
| MAT-S11 | `slider.{track,fill,thumb,inset_icon} @ size (Material's five Expressive sizes; finding ADD-A62)` | rule |
| MAT-S11 | `slider.inset_icon.{icon,padding}` | field |
| MAT-S12 | `slider @ variant range / centered / vertical` | rule |
| MAT-P1 | `progress_bar.values.{wave_amplitude,wavelength} @ variant @wavy` | rule |
| MAT-P1 | `progress_bar.indicator.min_height @ variant @wavy` | rule |
| MAT-P2 | `progress_bar.values.indicator_track_gap` | field |
| MAT-P3 | `progress_bar.stop_indicator.{fill,min_width,min_height,corners}` | field |
| MAT-P3 | `progress_bar.values.stop_indicator_gap` | field |
| MAT-P4 | `progress_bar.{track,indicator,body}.min_height @ variant @thick` | rule |
| MAT-P5 | `progress_bar.indicator.corners` | field |
| MAT-P6 | `progress_bar.values.indeterminate_loop; progress_bar.secondary_indicator` | field |
| MAT-P6 | `progress_bar.values.value_transition` | field |
| MAT-P6 | `progress_bar.values.indeterminate_loop @ variant @four_colour` | rule |
| MAT-P7 | `progress_bar.buffer.fill; progress_bar.buffer_dots.{fill,min_width,corners}` | field |
| MAT-TB1 | `tab.indicator, tab.label.text.fill, tab.icon.icon.fill @ variant @secondary` | rule |
| MAT-TB2 | `tab.indicator.corners` | field |
| MAT-TB2 | `tab.values.indicator_length` | field |
| MAT-TB3 | `tab.tab.fill.append @ {hover}, {focus_visible}, {pressed}, {selected,hover}, {selected,focus_visible}, {selected,pressed}` | rule |
| MAT-TB3 | `tab.label.text.fill @ {focus_visible}, {pressed}` | rule |
| MAT-TB4 | `tab.icon.icon.size, tab.icon.icon.fill @ {selected}, {hover}, {focus_visible}, {pressed}` | rule |
| MAT-TB4 | `tab.tab.min_height @ variant @icon_and_label` | rule |
| MAT-TB4 | `tab.values.icon_text_gap` | field |
| MAT-TB5 | `tab.bar.stroke.bottom` | field |
| MAT-TB6 | `tab.values.presentations indicator_move` | field |
| MAT-TB6 | `tab.values.presentations indicator_move @ cond reduced_motion` | rule |
| MAT-TB7 | `tab.bar.padding (start) @ variant @scrollable` | rule |
| MAT-TB8 | `tab.label.text (letter_spacing, line_height)` | field |
| MAT-SB1 | `list.row_background (corners full, min_height, max_width) @ {selected} variant sidebar` | rule |
| MAT-SB2 | `list.row_background.fill.append @ {hover}, {focus_visible}, {pressed}, {selected,hover}, {selected,focus_visible}, {selected,pressed} variant sidebar` | rule |
| MAT-SB2 | `list.leading_icon.icon.fill, list.row.text.fill @ {selected}, {hover}, {focus_visible}, {pressed} variant sidebar` | rule |
| MAT-SB3 | `list.leading_icon.icon.fill @ {selected} / not {selected} variant sidebar` | rule |
| MAT-SB4 | `list.section_header.text @ variant sidebar` | rule |
| MAT-SB5 | `list.trailing_text.text @ variant sidebar` | rule |
| MAT-SB6 | `sidebar.pane.fill, sidebar.pane.shadow (elevation) @ variant overlay` | rule |
| MAT-SB6 | `sidebar.scrim.fill` | field |
| MAT-SB7 | `sidebar.values.width; sidebar.pane.min_height` | field |
| MAT-SB7 | `sidebar.pane.corners (end corners) @ direction ltr, rtl` | rule |
| MAT-SB7 | `sidebar.pane.corners @ variant @bottom_sheet` | rule |
| MAT-SB8 | `list.row.text.font.weight @ {selected} variant sidebar` | rule |
| MAT-SB9 | `sidebar.values.presentations open, close, scrim_fade` | field |
| MAT-TA1 | `toolbar.bar.fill, toolbar.bar.shadow (elevation) @ {scrolled}` | rule |
| MAT-TA1 | `toolbar.bar.transitions (fill, shadow)` | field |
| MAT-TA2 | `toolbar.item_icon.icon.fill (leading, trailing by position first)` | rule |
| MAT-TA3 | `toolbar.bar.min_height, toolbar.title.text @ variant @center_aligned, @medium, @large, @medium_flexible, @large_flexible` | rule |
| MAT-TA3 | `toolbar.values.title_alignment, toolbar.values.collapse_on_scroll` | field |
| MAT-TA4 | `toolbar.subtitle.text` | field |
| MAT-TA5 | `avatar.* @ context toolbar.bar` | rule |
| MAT-TA6 | `search_field.* @ context toolbar.bar` | rule |
| MAT-TA6 | `search_field.* @ context toolbar.bar {scrolled}` | rule |
| MAT-TA7 | `toolbar.title.text (letter_spacing, line_height)` | field |
| MAT-TA8† | `toolbar.overflow_button (glyph, size limits)` | field |
| MAT-TA8† | `menu.* @ context toolbar.overflow_button` | rule |
| MAT-L1 | `list.row.min_height @ variant @two_line, @three_line` | rule |
| MAT-L1 | `list.secondary_text.text, list.overline_text.text, list.trailing_text.text` | field |
| MAT-L2 | `list.leading_icon (icon size, fill, corners, size limits) @ variant @avatar, @image, @video` | rule |
| MAT-L3 | `list.trailing_icon.icon (size, fill)` | field |
| MAT-L3 | `list.trailing_icon.icon.fill @ {selected}` | rule |
| MAT-L4 | `list.row_background.fill.append @ {focus_visible}, {pressed}, {dragged}` | rule |
| MAT-L4 | `list.row_background.shadow @ {dragged}` | rule |
| MAT-L4 | `list.row.text.fill, list.leading_icon.icon.fill, list.trailing_icon.icon.fill @ {hover}, {focus_visible}, {pressed}, {dragged}, {disabled}` | rule |
| MAT-L4 | `list.leading_icon.opacity, list.trailing_icon.opacity @ {disabled}` | rule |
| MAT-L5 | `list.row_background.fill @ {selected}` | rule |
| MAT-L5 | `list.row_background.fill.append @ {selected,hover}, {selected,focus_visible}, {selected,pressed}, {selected,dragged}` | rule |
| MAT-L5 | `list.row.text.fill, list.overline_text.text.fill, list.secondary_text.text.fill, list.leading_icon.icon.fill, list.trailing_icon.icon.fill @ {selected} and {selected,…}` | rule |
| MAT-L5 | `list.row_background.opacity, list.row.opacity @ {selected,disabled}` | rule |
| MAT-L6 | `list.row_separator.stroke, list.values.row_separator_inset, list.values.row_gap` | field |
| MAT-L7 | `list.values.leading_gap` | field |
| MAT-L8 | `list.row_background.fill, list.values.row_gap @ variant separated flavour expressive` | rule |
| MAT-L8 | `list.row_background.corners @ {hover}, {focus_visible}, {pressed}, {dragged}, {disabled}, {selected}, {selected,…} flavour expressive` | rule |
| MAT-L8 | `list.row_background.transitions (corners) flavour expressive` | rule |
| MAT-L8 | `list.leading_icon.icon.size, list.trailing_icon.icon.size, list.leading_icon.corners flavour expressive` | rule |
| MAT-L9 | `list.row_background.fill @ {open}` | rule |
| MAT-L9 | `list.disclosure (fill, corners, glyph fill) @ {open} / not {open}` | rule |
| MAT-L10 | `list.row_background.fill, list.row.text.fill, list.leading_icon.icon.fill, list.row_background.corners, list.row_background.shadow @ {dragged}` | rule |
| MAT-L10 | `list.drop_indicator.fill (drop zone)` | field |
| MAT-L11 | `list.swipe_action (fill, icon, corners)` | field |
| MAT-L11 | `list.swipe_action.fill.append @ {hover}, {focus_visible}, {pressed}` | rule |
| MAT-L12 | `list.header_section.text.fill @ {hover} variant table` | rule |
| MAT-L12 | `list.sort_indicator.glyph.fill variant table` | rule |
| MAT-L12 | `list.footer (fill, min_height, text)` | field |
| MAT-L13 | `list.row_background.fill @ {selected}, not {selected}; fill.append @ {hover} variant table` | rule |
| MAT-L13 | `list.view.stroke, list.view.corners @ variant table` | rule |
| MAT-L14 | `list.row.text (letter_spacing, line_height)` | field |
| MAT-PO1 | `popover.body.shadow (Elevation level 2)` | field |
| MAT-PO1 | `popover.body.fill.append (surface tint layer)` | field |
| MAT-SP1 | `splitter.grip (size limits, corners)` | field |
| MAT-SP1 | `splitter.divider.min_width` | field |
| MAT-SP2 | `splitter.grip.fill.append @ {hover}, {focus_visible}, {pressed}` | rule |
| MAT-SP2 | `splitter.grip.shadow` | field |
| MAT-SP3 | `layout.values.pane_gap @ size_class` | rule |
| MAT-D1 | `separator.values.inset @ variant @inset, @inset_start, @inset_end` | rule |
| MAT-TS1 | `token.text.keyed.display_large` | token |
| MAT-TS1 | `token.text.keyed.display_medium` | token |
| MAT-TS1 | `token.text.keyed.display_small` | token |
| MAT-TS1 | `token.text.keyed.headline_large` | token |
| MAT-TS1 | `token.text.keyed.headline_medium` | token |
| MAT-TS1 | `token.text.keyed.headline_small` | token |
| MAT-TS1 | `token.text.keyed.title_large` | token |
| MAT-TS1 | `token.text.keyed.title_medium` | token |
| MAT-TS1 | `token.text.keyed.title_small` | token |
| MAT-TS1 | `token.text.keyed.body_large` | token |
| MAT-TS1 | `token.text.keyed.body_medium` | token |
| MAT-TS1 | `token.text.keyed.body_small` | token |
| MAT-TS1 | `token.text.keyed.label_large` | token |
| MAT-TS1 | `token.text.keyed.label_medium` | token |
| MAT-TS1 | `token.text.keyed.label_small` | token |
| MAT-TS1 | `token.text.roles.DisplayLarge` | token |
| MAT-TS1 | `token.text.roles.DisplayMedium` | token |
| MAT-TS1 | `token.text.roles.DisplaySmall` | token |
| MAT-TS1 | `token.text.roles.LargeTitle` | token |
| MAT-TS1 | `token.text.roles.Title1` | token |
| MAT-TS1 | `token.text.roles.Title2` | token |
| MAT-TS1 | `token.text.roles.Headline` | token |
| MAT-TS1 | `token.text.roles.Subheadline` | token |
| MAT-TS1 | `token.text.roles.Body` | token |
| MAT-TS1 | `token.text.roles.Callout` | token |
| MAT-TS1 | `token.text.roles.Caption` | token |
| MAT-TS1 | `token.text.roles.Caption2` | token |
| MAT-TS1 | `token.text.roles.Label` | token |
| MAT-TS2 | `token.text.keyed.display_large.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.display_medium.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.display_small.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.headline_large.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.headline_medium.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.headline_small.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.title_large.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.title_medium.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.title_small.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.body_large.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.body_medium.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.body_small.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.label_large.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.label_medium.letter_spacing` | token |
| MAT-TS2 | `token.text.keyed.label_small.letter_spacing` | token |
| MAT-TS3 | `token.text.keyed.display_large.font.family` | token |
| MAT-TS3 | `token.text.keyed.display_medium.font.family` | token |
| MAT-TS3 | `token.text.keyed.display_small.font.family` | token |
| MAT-TS3 | `token.text.keyed.headline_large.font.family` | token |
| MAT-TS3 | `token.text.keyed.headline_medium.font.family` | token |
| MAT-TS3 | `token.text.keyed.headline_small.font.family` | token |
| MAT-TS3 | `token.text.keyed.title_large.font.family` | token |
| MAT-TS3 | `token.text.keyed.title_medium.font.family` | token |
| MAT-TS3 | `token.text.keyed.title_small.font.family` | token |
| MAT-TS3 | `token.text.keyed.body_large.font.family` | token |
| MAT-TS3 | `token.text.keyed.body_medium.font.family` | token |
| MAT-TS3 | `token.text.keyed.body_small.font.family` | token |
| MAT-TS3 | `token.text.keyed.label_large.font.family` | token |
| MAT-TS3 | `token.text.keyed.label_medium.font.family` | token |
| MAT-TS3 | `token.text.keyed.label_small.font.family` | token |
| MAT-TS4 | `token.text.keyed.display_large_emphasized` | token |
| MAT-TS4 | `token.text.keyed.display_medium_emphasized` | token |
| MAT-TS4 | `token.text.keyed.display_small_emphasized` | token |
| MAT-TS4 | `token.text.keyed.headline_large_emphasized` | token |
| MAT-TS4 | `token.text.keyed.headline_medium_emphasized` | token |
| MAT-TS4 | `token.text.keyed.headline_small_emphasized` | token |
| MAT-TS4 | `token.text.keyed.title_large_emphasized` | token |
| MAT-TS4 | `token.text.keyed.title_medium_emphasized` | token |
| MAT-TS4 | `token.text.keyed.title_small_emphasized` | token |
| MAT-TS4 | `token.text.keyed.body_large_emphasized` | token |
| MAT-TS4 | `token.text.keyed.body_medium_emphasized` | token |
| MAT-TS4 | `token.text.keyed.body_small_emphasized` | token |
| MAT-TS4 | `token.text.keyed.label_large_emphasized` | token |
| MAT-TS4 | `token.text.keyed.label_medium_emphasized` | token |
| MAT-TS4 | `token.text.keyed.label_small_emphasized` | token |
| MAT-TS4 | `token.text.roles.BodyEmphasized` | token |
| MAT-TS4 | `token.text.roles.CaptionEmphasized` | token |
| MAT-TS5 | `token.text.keyed.display_large.font.axes` | token |
| MAT-TS5 | `token.text.keyed.display_medium.font.axes` | token |
| MAT-TS5 | `token.text.keyed.display_small.font.axes` | token |
| MAT-TS5 | `token.text.keyed.headline_large.font.axes` | token |
| MAT-TS5 | `token.text.keyed.headline_medium.font.axes` | token |
| MAT-TS5 | `token.text.keyed.headline_small.font.axes` | token |
| MAT-TS5 | `token.text.keyed.title_large.font.axes` | token |
| MAT-TS5 | `token.text.keyed.title_medium.font.axes` | token |
| MAT-TS5 | `token.text.keyed.title_small.font.axes` | token |
| MAT-TS5 | `token.text.keyed.body_large.font.axes` | token |
| MAT-TS5 | `token.text.keyed.body_medium.font.axes` | token |
| MAT-TS5 | `token.text.keyed.body_small.font.axes` | token |
| MAT-TS5 | `token.text.keyed.label_large.font.axes` | token |
| MAT-TS5 | `token.text.keyed.label_medium.font.axes` | token |
| MAT-TS5 | `token.text.keyed.label_small.font.axes` | token |
| MAT-LA1 | `layout.values.window_margin, layout.values.pane_gap @ size_class compact, medium, expanded, large, extra_large` | rule |
| MAT-LA1 | `token.size_classes` | token |
| MAT-SW1 | `switch.thumb.min_width, switch.thumb.min_height @ {pressed}` | rule |
| MAT-SW2 | `switch.thumb.fill @ {hover} / {focus_visible} / {pressed} × {checked or !checked}` | rule |
| MAT-SW3 | `switch.thumb_icon (part: glyph, size, fill)` | field |
| MAT-SW3 | `switch.thumb_icon.glyph @ {checked}; switch.thumb_icon.glyph.fill @ {hover\|focus_visible\|pressed\|disabled} × {checked or !checked}` | rule |
| MAT-SW3 | `switch.thumb.min_width, switch.thumb.min_height @ variant @icons` | rule |
| MAT-SW4 | `switch.state_layer (part: size, corners Full)` | field |
| MAT-SW4 | `switch.state_layer.fill @ {hover} / {focus_visible} / {pressed} × {checked or !checked}` | rule |
| MAT-SW5 | `switch.track.fill, switch.track.stroke @ {focus_visible} / {pressed} × {checked or !checked}` | rule |
| MAT-SW6 | `switch.thumb.fill, switch.thumb.opacity @ {disabled,checked} / {disabled} !{checked}` | rule |
| MAT-SW7 | `switch.thumb.shadow (Shadow::Elevation)` | field |
| MAT-SW8 | `switch.values.thumb_move (CubicBezier overshoot)` | field |
| MAT-SW8 | `switch.thumb.transitions (Geometry, Fill); switch.thumb_icon.transitions` | field |
| MAT-SW9 | `switch.track.focus.offset` | field |
| MAT-DG1 | `dialog.{header,message,action_area}.padding` | field |
| MAT-DG1 | `dialog.values.{title_message_gap_px,message_actions_gap_px}` | field |
| MAT-DG2 | `dialog.icon.icon.fill` | field |
| MAT-DG2 | `dialog.values.title_alignment @ variant @with_icon` | rule |
| MAT-DG3 | `dialog.action.text @ {hover} / {focus_visible} / {pressed} / {disabled}` | rule |
| MAT-DG3 | `dialog.action.fill.append (state layers) @ {hover} / {focus_visible} / {pressed}` | rule |
| MAT-DG4 | `dialog.header.stroke @ {scrolled}` | rule |
| MAT-DG4 | `dialog.action_area.stroke @ {overflowing} (ADD-A13)` | rule |
| MAT-DG6 | `dialog.values.presentations events Appear, Disappear (parts body, message, scrim; ADD-B09)` | field |
| MAT-DG7 | `dialog.subtitle.text` | field |
| MAT-DG8 | `dialog.values.window_margin` | field |
| MAT-DG9 | `dialog.{title,message}.text.{letter_spacing,line_height}` | field |
| MAT-DG10† | `dialog.values.action_layout (end-aligned, wrap to a column, AcceptFirst, row and column gaps)` | field |
| MAT-SN1 | `spinner.track.fill` | field |
| MAT-SN1 | `spinner.values.stroke_width; spinner.values.indicator_track_gap` | field |
| MAT-SN2 | `spinner.values.{wave_amplitude,wavelength} @ variant @wavy` | rule |
| MAT-SN2 | `spinner.body.{min_width,min_height} @ variant @wavy` | rule |
| MAT-SN3 | `spinner.{body.min_width,body.min_height,values.stroke_width,values.track_stroke_width,values.indicator_track_gap} @ variant @thick` | rule |
| MAT-SN4 | `spinner.values.cap` | field |
| MAT-SN5 | `spinner.values.loop (colour keyframes) @ variant @four_colour` | rule |
| MAT-SN6 | `spinner.values.loop` | field |
| MAT-SN6 | `spinner.values.value_transition` | field |
| MAT-CB1 | `combo_box.{floating_label,supporting_text,icon}` | field |
| MAT-CB1 | `combo_box.body @ variant @filled; combo_box.body.stroke @ {focused} / {disabled} / {invalid}` | rule |
| MAT-CB2 | `combo_box.arrow.glyph.fill @ {hover} / {focused} / {invalid} / {disabled}` | rule |
| MAT-CB3 | `combo_box.arrow.transform @ {open}` | rule |
| MAT-CB3 | `combo_box.arrow.transitions` | field |
| MAT-CB3 | `combo_box.item.fill @ {selected}; combo_box.item.min_height` | rule |
| MAT-SG1 | `segmented_control.check (part: glyph, size, fill)` | field |
| MAT-SG1 | `segmented_control.check.glyph.fill @ {checked,hover\|focus_visible\|pressed}; segmented_control.check.transitions (Geometry, Opacity)` | rule |
| MAT-SG2 | `segmented_control.segment.fill, segmented_control.label.text.fill, segmented_control.icon.icon.fill @ {hover\|focus_visible\|pressed} × {checked or !checked}` | rule |
| MAT-SG3 | `segmented_control.segment.corners @ pos first / pos last / pos only` | rule |
| MAT-SG4 | `segmented_control.segment.stroke, segmented_control.icon.icon.fill, segmented_control.icon.opacity @ {disabled}` | rule |
| MAT-CA1 | `card.body.fill, card.body.stroke, card.body.shadow (elevation) @ variant filled, elevated, outlined` | rule |
| MAT-CA2 | `card.body.fill.append @ {hover}, {focus_visible}, {pressed}, {dragged}` | rule |
| MAT-CA2 | `card.body.shadow @ {hover}, {focus_visible}, {pressed}, {dragged} variant elevated, filled` | rule |
| MAT-CA2 | `card.body.stroke @ {hover}, {focus_visible}, {pressed}, {disabled} variant outlined` | rule |
| MAT-CA2 | `card.body.fill, card.body.opacity @ {disabled}` | rule |
| MAT-CA3 | `card.icon` | field |
| MAT-EX1 | `list.row_background.fill @ {open}` | rule |
| MAT-EX1 | `list.disclosure.fill, list.disclosure.corners, list.disclosure.glyph @ {open}` | rule |
| MAT-N1 | `button.body, button.icon, button.body.shadow (elevation entries) @ variant @fab size small / regular / large and {hover} / {focus_visible} / {pressed} / @lowered; button.body.fill @ variant @fab,@primary / @secondary / @tertiary / @surface` | rule |
| MAT-N1 | `button.body.focus @ variant @fab {focus_visible}; button.transitions @ variant @fab` | rule |
| MAT-N2 | `button.label.text, button.values.icon_text_gap, button.body.padding @ variant @extended_fab size small / regular / large` | rule |
| MAT-N3 | `component.fab_menu.close_button, component.fab_menu.item, component.fab_menu.item_label, component.fab_menu.item_icon, component.fab_menu.values.item_gap, presentations @open / @close (morph)` | component |
| MAT-N4 | `button.body, button.icon, button.body.stroke, button.body.focus @ variant icon_only,primary / icon_only,tinted / icon_only,@outlined / icon_only {checked} / {hover} / {pressed} / {disabled}` | rule |
| MAT-N5 | `button.body.min_height, button.icon.icon.size, button.body.padding, button.body.corners, button.body.transitions @ variant icon_only size mini / small / regular / large / extra_large {pressed} / {checked}` | rule |
| MAT-N6 | `segmented_control.* @ variant @standard_group / @connected: inner corners @ {pressed} / {selected}, width @ {pressed} with spring transition` | rule |
| MAT-N7 | `split_button.values.part_gap, split_button.primary.padding, split_button.secondary.padding, split_button.primary.corners, split_button.secondary.corners @ {hover} / {pressed} / {open} size mini / small / regular / large / extra_large; split_button.arrow.transform, split_button.arrow.transitions @ {open}` | rule |
| MAT-N8 | `button.* @ variant menu,primary / menu,@outlined / menu; button.menu_indicator.icon @ {hover} / {pressed} / {focus_visible} / {disabled}` | rule |
| MAT-N9 | `chip.body, chip.label, chip.leading_icon, chip.trailing_icon, chip.avatar @ variant @assist / @filter / @input / @suggestion and elevated` | rule |
| MAT-N9 | `chip.body @ {selected} / {hover} / {pressed} / {focus_visible} / {dragged} / {disabled}` | rule |
| MAT-N10 | `badge.body @ variant dot; badge.body, badge.label.text @ variant count; badge.values.anchor_offset_x, badge.values.anchor_offset_y` | rule |
| MAT-N11 | `toolbar.* @ variant bottom: fill, min_height, corners, shadow (elevation)` | rule |
| MAT-N12 | `bottom_tab_bar.bar, bottom_tab_bar.indicator, bottom_tab_bar.item_icon, bottom_tab_bar.item_label @ {selected} / {hover} / {pressed} / {focus_visible}; badge @ context any bottom_tab_bar` | rule |
| MAT-N13 | `sidebar.pane.fill @ variant compact` | rule |
| MAT-N13 | `sidebar.pane.corners @ variant compact` | rule |
| MAT-N13 | `sidebar.pane.shadow @ variant compact` | rule |
| MAT-N13 | `sidebar.pane.min_width @ variant compact` | rule |
| MAT-N13 | `list.row_selection_indicator.fill @ variant sidebar context sidebar.pane variant compact` | rule |
| MAT-N13 | `list.row_selection_indicator.corners @ variant sidebar context sidebar.pane variant compact` | rule |
| MAT-N13 | `list.leading_icon.icon @ variant sidebar context sidebar.pane variant compact` | rule |
| MAT-N13 | `list.row.text.fill @ variant sidebar context sidebar.pane variant compact` | rule |
| MAT-N13 | `sidebar.toggle_button.glyph @ variant compact` | rule |
| MAT-N13 | `badge.body.fill @ context sidebar.pane variant compact` | rule |
| MAT-N13 | `sidebar.header.padding @ variant compact` | rule |
| MAT-N13 | `sidebar.pane.min_width @ variant compact,@expanded` | rule |
| MAT-N13 | `list.values.row_gap @ context sidebar.pane variant compact,@expanded` | rule |
| MAT-N13 | `sidebar.pane.fill @ variant compact,@modal` | rule |
| MAT-N14 | `search_field.bar.fill, search_field.bar.min_height, search_field.bar.corners, search_field.bar.shadow, search_field.leading_icon, search_field.trailing_icon, search_field.avatar, search_field.supporting_text @ variant @contained` | rule |
| MAT-N15 | `search_field.view, search_field.view_header, search_field.view_divider @ variant @docked / @full_screen; search_field.values.presentations @view_open` | rule |
| MAT-N16 | `date_picker.popup, date_picker.header, date_picker.headline, date_picker.supporting_text, date_picker.divider @ variant with_popup / @modal / @input` | rule |
| MAT-N16 | `calendar.day, calendar.today_marker, calendar.day_label.opacity @ {@other_month}, calendar.range_band, calendar.year @ {selected} / {@today} / {hover} / {pressed} / {focus_visible}` | rule |
| MAT-N17 | `time_picker.dial, time_picker.dial_selector, time_picker.dial_track, time_picker.dial_centre, time_picker.dial_label, time_picker.selector_cell, time_picker.period_selector, time_picker.headline, time_picker.time_separator @ {selected} / {hover} / {pressed} / {focus_visible} variant @dial / @input` | rule |
| MAT-N17 | `input.* @ context any time_picker variant @input` | rule |
| MAT-N18 | `carousel.item.fill, carousel.item.corners, carousel.item.shadow, carousel.item.stroke, carousel.item @ {hover} / {pressed} / {focus_visible}; carousel.values.item_gap` | rule |
| MAT-N18 | `carousel.item.corners by scroll position (ADD-B13)` | field |
| MAT-N19 | `sheet.body.fill, sheet.body.corners, sheet.body.shadow, sheet.handle, sheet.handle.focus @ variant bottom,@standard / bottom,@modal {@minimised}; sheet.values.presentations @settle` | rule |
| MAT-N20 | `sheet.body, sheet.title.text, sheet.divider, sheet.close_button, sheet.body.max_width @ variant @side,@standard / @side,@modal / @side,@detached` | rule |
| MAT-N21 | `sheet.body.fill, sheet.body.shadow, sheet.body.corners @ variant floating` | rule |
| MAT-N22 | `toast.body, toast.title.text, toast.action_button @ {hover} / {pressed} / {focus_visible}, toast.close_button, toast.body.min_height @ {@multiline}, toast.values.durations, toast.values.presentations @appear / @disappear` | rule |
| MAT-N23 | `banner.body, banner.title, banner.message, banner.icon, banner.image, banner.close_button, banner.divider, banner.values.heights, banner.values.action_gap @ variant @basic / @rich / @vibrant` | rule |
| MAT-N24 | `spinner.indicator.fill @ variant @loading_indicator` | rule |
| MAT-N24 | `spinner.indicator.min_width @ variant @loading_indicator` | rule |
| MAT-N24 | `spinner.body.fill @ variant @contained` | rule |
| MAT-N24 | `spinner.body.corners @ variant @contained` | rule |
| MAT-N24 | `spinner.body.min_width @ variant @contained` | rule |
| MAT-N24 | `spinner.values.loop @ variant @loading_indicator` | rule |
| MAT-N25 | `toolbar.* @ variant @docked / @floating / @vibrant; button.* @ context any toolbar {hover} / {pressed}; toolbar.values.presentations @show / @hide` | rule |
| MAT-N26 | `dialog.body, dialog.header, dialog.title, dialog.close_button, dialog.action_area, dialog.header.stroke @ variant full_size {@scrolled}` | rule |
| MAT-N27 | `autocomplete.popup, autocomplete.item, autocomplete.item_label, autocomplete.item_icon @ variant @filled / @outlined; input.* @ context any autocomplete` | rule |
| MAT-N27 | `placement MaterialMenu (autocomplete.values.placement)` | placement |
| MAT-N28 | `pull_to_refresh.indicator.fill, pull_to_refresh.indicator.min_width, pull_to_refresh.indicator.shadow, pull_to_refresh.spinner, pull_to_refresh.values.trigger_distance, pull_to_refresh.values.presentations` | field |
| MAT-N30† | `edit_menu.body.fill, edit_menu.body.corners, edit_menu.body.shadow, edit_menu.item_label.text` | field |
| MAT-N30† | `placement AndroidTextToolbar (edit_menu.values.placement)` | placement |
| MAT-N31† | `picker_wheel.container @ {disabled}, picker_wheel.item.text.fill @ {selected} / {disabled}, picker_wheel.values.snap, picker_wheel.item.focus` | rule |
| MAT-N31† | `picker_wheel.item.text interpolated by scroll position (ADD-B13)` | field |
| MAT-X1 | `role.@surface_container_* / tertiary_* / fixed_* / inverse_* / outline_variant / scrim` | role |
| MAT-X1 | `role.global.scrim` | role |
| MAT-X2 | `token.elevation.level0 ... level5` | token |
| MAT-X2 | `PartStyle.shadow Elevation(key) per component` | field |
| MAT-X3 | `token.elevation.<levels> layers (key and ambient)` | token |
| MAT-X3 | `reader:android:android:ambientShadowAlpha, android:spotShadowAlpha, light source dimensions` | readers (R18) |
| MAT-X4 | `PartStyle.shadow Elevation(key) @ {hover} / {focused} / {pressed} / {dragged}` | rule |
| MAT-X5 | `ShadowLayer.color per elevation entry and per part` | field |
| MAT-X6 | `PartStyle.fill.append = alpha(surface tint role, elevation derived)` | field |
| MAT-X6 | `role.@surface_tint` | role |
| MAT-X7 | `PartStyle.transitions group Shadow` | field |
| MAT-X8 | `PartStyle.fill.append state layers = alpha(foreground role, opacity) @ {hover} / {focused} / {pressed} / {dragged}` | rule |
| MAT-X9 | `TransitionKind::Ripple on the pressed state layer (S2.7)` | field |
| MAT-X10 | `PartStyle.focus FocusIndicator offset, corners, transition` | field |
| MAT-X10 | `FocusIndicator.secondary (ADD-A17)` | field |
| MAT-X10 | `FocusIndicator.transition @ cond reduced_motion` | rule |
| MAT-X11 | `token.params.shape.<scale steps> (ADD-A44)` | token |
| MAT-X11 | `PartStyle.corners per-corner Radius and Radius::Full` | field |
| MAT-X12 | `PartStyle.corners @ {pressed} / {selected} with Spring transition` | rule |
| MAT-X12 | `Glyph morph Loop over Path keyframes (ADD-B03)` | field |
| MAT-X13 | `token.motion.durations.<duration tokens>` | token |
| MAT-X13 | `token.motion.timings.<easing and spring tokens>` | token |
| MAT-X13 | `declare.flavours baseline expressive (motion schemes)` | field |
| MAT-X14 | `PresentationKind Keyed container_transform / shared_axis / fade_through (ADD-A26)` | field |
| MAT-X16 | `TextStyle.letter_spacing / line_height, Font.axes, Font.features` | field |
| MAT-X16 | `token.text.roles.*` | token |
| MAT-X17 | `input.seed` | input |
| MAT-X17 | `function m3_role (S2.2)` | field |
| MAT-X17 | `reader:android:dynamic colour scheme roles and variant` | readers (R18) |
| MAT-X18 | `condition:ContrastIncreased / ContrastHigh (medium and high)` | condition |
| MAT-X18 | `token.material_scheme.contrast @ cond contrast_increased / cond contrast_high` | rule |
| MAT-X18 | `reader:android:UiModeManager.getContrast` | readers (R18) |
| MAT-X19 | `expression operand text_scale (ADD-A55)` | field |
| MAT-X19 | `condition:BoldText` | condition |
| MAT-X19 | `reader:android:Configuration.fontScale, FontScaleConverter, fontWeightAdjustment` | readers (R18) |
| MAT-X20 | `PartStyle.transitions and presentation @ cond reduced_motion` | rule |
| MAT-X20 | `token.motion.speed_factor` | token |
| MAT-X20 | `reader:android:Settings.Global.ANIMATOR_DURATION_SCALE, TRANSITION_ANIMATION_SCALE` | readers (R18) |
| MAT-X21 | `PartStyle fill, stroke rules @ cond contrast_high (forced colours, parts gain outlines)` | rule |
| MAT-X21 | `condition:ContrastHigh` | condition |
| MAT-X22 | `declare.widget.*.densities (density scale)` | field |
| MAT-X22 | `declare.widget.*.sizes (Expressive size ladder)` | field |
| MAT-X22 | `PartStyle.min_width / min_height (touch target)` | field |
| MAT-X22 | `PartStyle padding @ density` | rule |
| MAT-X23 | `role.global.scrim` | role |
| MAT-X23 | `dialog.scrim.fill` | field |
| MAT-X23 | `sheet.scrim.fill` | field |
| MAT-X24 | `PartStyle.opacity @ {disabled} per part (container, label, icon, outline)` | rule |
| MAT-X25 | `tooltip.values.hide_delay_ms` | field |
| MAT-X25 | `toast.values.durations` | field |
| MAT-X25 | `menu.values.submenu_open_delay_ms / close_delay_ms` | field |
| MAT-X25 | `token.behaviour.long_press_ms (ADD-A45)` | token |
| MAT-X25 | `token.behaviour.touch.double_tap_ms (ADD-A45)` | token |
| MAT-X25 | `scrollbar.values.fade_delay_ms / fade_duration_ms` | field |
| MAT-X25 | `token.caret.blink_on_ms` | token |
| MAT-X25 | `TransitionKind::Ripple delay and minimum press` | field |
| MAT-X25 | `reader:android:ViewConfiguration.getLongPressTimeout, getDoubleTapTimeout, getScrollDefaultDelay, getScrollBarFadeDuration` | readers (R18) |
| MAT-X27 | `IconStyle.variant filled when active (ADD-A37)` | rule |
| MAT-X27 | `Font.axes for the symbols font (FILL, wght, GRAD, opsz)` | field |
| MAT-X27 | `PartStyle.icon.fill per state` | rule |
| MAT-X29 | `StateFlag Dragged` | flag |
| MAT-X29 | `PartStyle.fill.append @ {dragged}` | rule |
| MAT-X29 | `PartStyle.shadow Elevation(key) @ {dragged}` | rule |
| MAT-X31 | `condition:Input(Mouse) / Input(Touch) / Input(Pen)` | condition |
| MAT-X31 | `PartStyle sizes @ condition input` | rule |
| MAT-X32† | `token.behaviour.touch.slop / paging_slop / double_tap_slop (ADD-A45)` | token |
| MAT-X32† | `token.behaviour.scroll_lines_per_step` | token |
| MAT-X32† | `reader:android:ViewConfiguration.getScaledTouchSlop, getScaledPagingTouchSlop, getScaledDoubleTapSlop, getScaledVerticalScrollFactor` | readers (R18) |
| MAT-X33† | `PartStyle.cursor per part and state` | field |
| MAT-X33† | `token.pointer.size / colours (ADD-A46)` | token |
| MAT-X34† | `<widget>.values.focus_visible_when AfterKeyboard (ADD-A16)` | field |
| MAT-X34† | `FocusIndicator.secondary two-stroke style (ADD-A17)` | field |
| MAT-X35† | - | out (R1.1) |
| MAT-X36† | `token.text.roles.body / monospace family` | token |
| MAT-X36† | `reader:android:Typeface.DEFAULT, Typeface.MONOSPACE` | readers (R18) |
| MAT-X37† | `PartStyle.padding / transform translate = mul(token.params.layout.optical_centering, sub(corner radii)) clamped by min (ADD-A56)` | field |
| MAT-X37† | `expression operand corners.<corner> (ADD-A56, ADD-B11)` | field |
| MAT-X38† | `placement strategy MaterialScaffoldSlot (FAB end or centre, snackbar above FAB and bar)` | placement |
| MAT-SF01† | `dialog.values.placement (CenteredInHost)` | placement |
| MAT-SF02† | `dialog.values.presentation_breakpoint` | field |
| MAT-SF02† | `dialog.* @ variant full_size` | rule |
| MAT-SF03† | `sheet.values.anchor, sheet.values.max_width @ variant bottom,@modal` | field |
| MAT-SF04† | `placement strategy MaterialLevitatedPane (alignment, scrim)` | placement |
| MAT-SF04† | `role.global.scrim` | role |
| MAT-RL01† | `wrap_box.values.child_gap, wrap_box.values.line_gap, wrap_box.values.wraps @ context chip children` | field |
| MAT-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| MAT-FM01† | `reader:android:AFontMatcher_match on an emoji string` | readers (R18) |
| MAT-FM02† | `token.text.fallbacks.fallbacks and language_fallbacks (ADD-A41)` | token |
| MAT-FM02† | `reader:android:AFontMatcher_match per locale` | readers (R18) |
| MAT-FM03† | `Selector.direction Rtl and element direction` | rule |
| MAT-FM03† | `reader:android:Configuration.getLayoutDirection` | readers (R18) |
| MAT-FM04† | `IconRole.mirrors_in_rtl (per role, script exceptions)` | field |
| MAT-FM04† | `icon variants per language and script (ADD-B05)` | field |
| MAT-P3-1† | - | out (R1.1) |
| MAT-P3-2† | - | out (R1.1) |
| MAT-P3-3† | - | out (R1.1) |
| MAT-P3-4† | - | out (R1.1) |
| MAT-P3-5† | - | out (R1.1) |
| MAT-P3-6† | `window.launch_screen.fill` | field |
| MAT-P3-6† | `window.launch_icon.{icon,fill}` | field |
| MAT-P3-6† | `window.values.presentations event @launch_exit` | field |
| MAT-P3-7† | `input.body.fill.append @ {autofilled}` | rule |
| MAT-P3-8† | `checkbox.mark.glyph (Path from Compose drawCheck)` | field |
| MAT-P3-8† | `checkbox.mark.glyph @ {indeterminate} (the same path pulled to the centre line)` | rule |
| MAT-P3-9† | `combo_box.values.popup_placement (MaterialMenu with the params of ADD-A21, ADD-A22)` | placement |
| MAT-P3-10† | `menu.values.placement (MaterialMenu)` | placement |
| MAT-P3-11† | `menu.* @ variant @text_context` | rule |
| MAT-P3-11† | `role.@popup_menu_background, role.@popup_menu_text (the platform popup-menu style)` | role |
| MAT-P3-12† | `segmented_control.values.segment_width (FillEqually)` | field |
| MAT-P3-12† | `segmented_control.segment.min_width` | field |
| MAT-P3-13† | `layout.values.max_panes_horizontal, max_panes_vertical, pane_width, pane_height, avoids_hinge @ size_class` | rule |
| MAT-P3-14† | `token.behaviour.key_repeat_delay_ms / key_repeat_interval_ms` | token |
| MAT-P3-14† | `reader:android:ViewConfiguration.getKeyRepeatTimeout, getKeyRepeatDelay` | readers (R18) |
| MAT-P3-15† | `token.behaviour.hide_pointer_while_typing` | token |
| MAT-P3-16† | `token.behaviour.scroll deceleration (fling)` | token |
| MAT-P3-16† | `reader:android:ViewConfiguration.getScrollFriction, getScaledMinimumFlingVelocity, getScaledMaximumFlingVelocity` | readers (R18) |
| MAT-P3-17† | - | out (R1.1) |
| MAT-P3-18† | `token.theme_change` | token |
| MAT-P3-18† | `reader:android:Configuration.uiMode` | readers (R18) |
| MAT-P3-19† | `condition:Flavour(baseline) / Flavour(expressive)` | condition |
| MAT-P3-19† | `token.material_scheme specification version` | token |
| MAT-P3-20† | `rich_text.heading_1 … heading_6.text, rich_text.preformatted.text, rich_text.list_marker, rich_text.values.list_indent_per_level, rich_text.link, rich_text.subscript, rich_text.superscript, rich_text.block_quote @ variant html` | rule |
| MAT-P3-21† | `token.caret.{hides_while_selecting,width}` | token |
| MAT-P3-21† | `input.caret.opacity @ cond window_inactive` | rule |
| MAT-P3-21† | - | readers (R18) |
| MAT-P4-1† | - | out (R1.1) |
| MAT-P4-2† | `window.values.keyboard_avoidance AndroidPan / AndroidResize / AndroidInsetsAnimation (section placement strategies)` | placement |
| MAT-P4-2† | `reader:android:softInputMode, WindowInsetsAnimation` | readers (R18) |
| MAT-P4-3† | `token.pointer.drag_cursors.<drop operation> (ADD-A46)` | token |
| MAT-P4-4† | - | out (R1.1) |
| MAT-P4-5† | - | out (R1.1) |
| MAT-P4-6† | `tab.values.overflow, tab.values.scroll_to_selected, tab.values.scroll_margin` | field |
| MAT-P4-6† | `tab.label.text.elision (None)` | field |
| MAT-P4-6† | `tab.values.presentations scroll` | field |
| MAT-P4-7† | `toolbar.values.presentations @expand / @collapse (parts leading_content, trailing_content; ADD-B09) @ variant @floating` | rule |
| MAT-P4-7† | `toolbar.bar.shadow @ variant @floating,@collapsed` | rule |
| MAT-P4-7† | `toolbar.bar.transitions @ variant @floating` | rule |
| MAT-P4-8† | `drag_image.values.presentations Move (drop outcome on the drag shadow)` | field |
| MAT-P4-9† | `tooltip.values.placement (MaterialTooltip)` | placement |
