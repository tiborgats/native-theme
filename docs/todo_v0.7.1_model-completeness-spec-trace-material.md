# v0.7.1 — Model completeness: Specification, traceability (Material)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-material.md`](todo_v0.7.1_model-completeness-gap-map-material.md) mapped to
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
| MAT-W1 | `window.values.status_bar_style (system-bar icon tone)` | field |
| MAT-W1 | `window.values.navigation_bar_contrast_scrim` | field |
| MAT-W1 | - | readers (R18) |
| MAT-W2 | `window.values.presentations event @predictive_back` | field |
| MAT-W2 | - (the activity and window transitions the system runs) | out (R1.1) |
| MAT-W4† | `window.values.caption_bar_transparent (the opt-in to a transparent caption bar, APPEARANCE_TRANSPARENT_CAPTION_BAR_BACKGROUND); window.title_bar.fill (the caption's fill taken from the app)` | field |
| MAT-W4† | `reader:android:WindowInsets.Type.captionBar(), the caption's fill taken from the app` | readers (R18) |
| MAT-W4† | - (the caption buttons, chip, outline and shadow) | out (R1.1) |
| MAT-W5† | `window.frame.corners (Radius::Concentric per corner, following the display, the root's container: RootInputs.display_corner_radius, which Android reports, else the four tokens token.params.display.corner_radius_{top_left,top_right,bottom_right,bottom_left} (per corner), else none; S4.6, S8, R25.11)` | field |
| MAT-W5† | `window.frame.corners @ cond freeform` (a freeform window's radius in desktop windowing, ADD-A83) | rule |
| MAT-W5† | - | readers (R18) |
| MAT-B1 | `button.body.fill, button.label.text.fill @ variant @elevated / variant primary / variant tinted / variant @outlined / variant flat (elevated, filled, tonal, outlined, text)` | rule |
| MAT-B1 | `button.body.stroke @ variant @outlined` | rule |
| MAT-B1 | `button.body.shadow @ variant @elevated (Shadow::Elevation)` | rule |
| MAT-B2 | `button.body.fill.append (state layer), button.label.text.fill @ {hover} variant primary / {focus_visible} variant primary / {pressed} variant primary` | rule |
| MAT-B2 | `button.body.fill, button.label.text.fill @ {disabled} variant primary` | rule |
| MAT-B3 | `button.body.stroke @ {hover} / {focus_visible} / {pressed} / {disabled}, each variant @outlined` | rule |
| MAT-B3 | `button.body.fill (Absent) @ variant @outlined` (no container; the line MAT-B1 states) | rule |
| MAT-B4 | `button.body.padding, button.body.fill @ variant flat` | rule |
| MAT-B4 | `button.label.text.fill @ {hover} / {focus_visible} / {pressed} / {disabled}, each variant flat` | rule |
| MAT-B5 | `button.body.fill.append (the focus state layer), button.label.text.fill, button.body.shadow @ {focus_visible}` | rule |
| MAT-B6 | `button.body.shadow = Shadow::Elevation(<key>) @ {hover} / {focus_visible} / {pressed} / {disabled}, each variant @elevated / variant primary / variant tinted` | rule |
| MAT-B7 | `button.icon.icon.{size,fill}` | field |
| MAT-B7 | `button.icon.icon.fill @ {hover} / {focus_visible} / {pressed} / {disabled} (the disabled opacity as the paint's alpha: the part lists no opacity)` | rule |
| MAT-B8 | `button.body.padding @ variant with_icon` | rule |
| MAT-B9 | `button.body.{min_height,padding}, button.icon.icon.size, button.values.icon_text_gap, button.label.text @ size extra_small / size small / size regular / size large / size extra_large, each cond flavor:expressive (ADD-A62)` | rule |
| MAT-B9 | `button.body.stroke @ size extra_small / size small / size regular / size large / size extra_large, each variant @outlined cond flavor:expressive` | rule |
| MAT-B10 | `button.body.corners @ variant @square cond flavor:expressive (round the base)` | rule |
| MAT-B10 | `button.body.corners @ {checked} variant toggle / {checked} variants {toggle,@square} / {pressed}, each size extra_small / size small / size regular / size large / size extra_large cond flavor:expressive` | rule |
| MAT-B11 | `button.body.transitions (Corners, Timing::Spring) @ size extra_small / size small / size regular / size large / size extra_large, each cond flavor:expressive (the spring of the pressed corners)` | rule |
| MAT-B12 | `button.body.fill, button.label.text.fill, button.icon.icon.fill @ !{checked} / {checked}, each variants {toggle,primary} / variants {toggle,tinted} / variants {toggle,@outlined} / variants {toggle,@elevated}` | rule |
| MAT-B12 | `button.body.fill.append (state layer), button.label.text.fill, button.icon.icon.fill @ {hover} / {focus_visible} / {pressed} / {checked,hover} / {checked,focus_visible} / {checked,pressed}, each variant toggle` | rule |
| MAT-B13 | `button.body.transitions (Shadow: duration and timing)` | field |
| MAT-B13 | `token.motion.durations.<key>, token.motion.timings.emphasized` | token |
| MAT-B15 | `button.label.text (= token.text.keyed.label_large)` (a block reference, S2.2: its letter spacing and line height with it) | field |
| MAT-I1 | `input.body.stroke @ {focused} / {hover} / {disabled}` | rule |
| MAT-I2 | `input.body.stroke @ {disabled}` | rule |
| MAT-I3 | `input.floating_label.{text,padding} (the label's font at rest; the space below it)` | field |
| MAT-I3 | `input.floating_label.text.fill @ {hover} / {focused} / {invalid} / {disabled}; input.floating_label.opacity @ {disabled}` | rule |
| MAT-I3 | `input.floating_label.{text,transform} @ {focused} / !{empty} (floated: smaller text with its own line height)` | rule |
| MAT-I3 | `input.values.floating_label_gap` | field |
| MAT-I3 | `input flags empty` | flag |
| MAT-I4 | `input.values.notch_padding` | field |
| MAT-I5 | `input.supporting_text.{text,padding}` | field |
| MAT-I5 | `input.supporting_text.text.fill @ {hover} / {focused} / {invalid} / {disabled}; input.supporting_text.opacity @ {disabled}` | rule |
| MAT-I5 | `input.values.supporting_text_gap` | field |
| MAT-I6 | `input.counter.text` | field |
| MAT-I6 | `input.counter.text.fill @ {hover} / {focused} / {invalid} / {disabled} (the supporting text's style)` | rule |
| MAT-I7 | `input.icon.icon.{size,fill} @ variant leading / variant trailing` | rule |
| MAT-I7 | `input.icon.icon.fill @ context input.body {hover} / context input.body {focused} / context input.body {invalid} / context input.body {disabled}; input.icon.opacity @ context input.body {disabled}` | rule |
| MAT-I7 | `input.values.icon_text_gap; input.icon.padding (the content-side spaces with an icon)` | field |
| MAT-I8 | `input.prefix.text; input.suffix.text` | field |
| MAT-I8 | `input.values.affix_gap` | field |
| MAT-I9 | `input.{body.stroke,caret.fill,text.text.fill,floating_label.text.fill,supporting_text.text.fill} @ {invalid} / {invalid,hover} / {invalid,focused}` | rule |
| MAT-I9 | `input.icon.icon.fill @ context input.body {invalid} / context input.body {invalid,hover} / context input.body {invalid,focused}` | rule |
| MAT-I10 | `input.{text.text.fill,floating_label.text.fill} @ {hover} / {focused}` | rule |
| MAT-I11 | `input.body.{fill,corners,stroke} @ variant @filled` | rule |
| MAT-I11 | `input.body.fill.append (state layer) @ {hover} variant @filled` | rule |
| MAT-I11 | `input.body.stroke (bottom edge: the active indicator) @ variant @filled; @ {hover} / {focused} / {invalid} / {disabled}, each variant @filled` | rule |
| MAT-I11 | `input.body.fill @ {disabled} variant @filled` | rule |
| MAT-I12 | `token.caret.{width,blink_on,blink_off}` | token |
| MAT-I12 | - | readers (R18) |
| MAT-I13 | `input.selection_handle.{glyph,fill,min_width,min_height}` | field |
| MAT-I14 | `input.floating_label.transitions (Transform, Text); input.placeholder.transitions (Opacity)` | field |
| MAT-I14 | `input.body.transitions (Stroke: the notch opening)` | field |
| MAT-I15 | `input.text.text.{letter_spacing,line_height}` | field |
| MAT-I16 | `text_area.values.min_rows; text_area.values.resize` | field |
| MAT-I17† | `input.magnifier.{corners,shadow,min_width,min_height}` | field |
| MAT-I17† | `input.values.magnifier_zoom; input.values.magnifier_offset` | field |
| MAT-I18† | `input.preedit.text.decoration` | field |
| MAT-I18† | `input.misspelling.text.decoration; input.correction.text.decoration` | field |
| MAT-I19† | `input.values.mask_character; input.values.mask_reveal` | field |
| MAT-I19† | - | readers (R18) |
| MAT-C1 | `radio.ring.stroke @ !{checked} / {checked}; radio.dot.fill @ {checked}` | rule |
| MAT-C1 | `radio.ring.stroke, radio.dot.fill @ {hover} / {focus_visible} / {pressed} / {disabled} / {checked,hover} / {checked,focus_visible} / {checked,pressed} / {checked,disabled} (the disabled opacity as the paint's alpha: ring and dot list no opacity)` | rule |
| MAT-C2 | `checkbox.state_layer.{min_width,min_height,corners}; radio.state_layer.{min_width,min_height,corners} (corners Radius::Full)` | field |
| MAT-C2 | `checkbox.state_layer.{fill,opacity}, radio.state_layer.{fill,opacity} @ {hover} / {focus_visible} / {pressed} / {checked,hover} / {checked,focus_visible} / {checked,pressed}` | rule |
| MAT-C3 | `checkbox.box.fill, checkbox.mark.glyph.fill, checkbox.box.stroke @ {checked,hover} / {checked,focus_visible} / {checked,pressed}` | rule |
| MAT-C4 | `checkbox.box.stroke @ {hover} / {focus_visible} / {pressed}, each !{checked,indeterminate}` | rule |
| MAT-C5 | `checkbox.box.fill, checkbox.mark.glyph.fill @ {invalid,checked} / {invalid,checked,hover} / {invalid,checked,focus_visible} / {invalid,checked,pressed}` | rule |
| MAT-C5 | `checkbox.box.stroke @ {invalid} !{checked}; checkbox.state_layer.fill @ {invalid,hover} / {invalid,focus_visible} / {invalid,pressed}` | rule |
| MAT-C6 | `checkbox.mark.glyph @ {indeterminate} (GlyphSource::Path)` | rule |
| MAT-C6 | `checkbox.box.fill, checkbox.mark.glyph.fill @ {indeterminate}` | rule |
| MAT-C7 | `checkbox.box.stroke @ {disabled} !{checked} (colour, width, opacity as the paint's alpha); checkbox.mark.glyph.fill @ {checked,disabled}; checkbox.box.fill @ {checked,disabled} (the container's disabled opacity as the paint's alpha: the box lists no opacity)` | rule |
| MAT-C8 | `checkbox.mark.{opacity,transform} @ !{checked} / {checked}; checkbox.mark.transitions (Opacity, Transform)` | rule |
| MAT-C8 | `radio.dot.transform @ !{checked} / {checked}; radio.dot.transitions (Transform, Fill); radio.ring.transitions (Stroke)` | rule |
| MAT-C8 | `checkbox.box.transitions (Fill, Stroke)` | field |
| MAT-C8 | `token.motion.timings.emphasized_decelerate, token.motion.timings.emphasized_accelerate` | token |
| MAT-C9 | `checkbox.body.touch_target; radio.body.touch_target (ADD-A72)` | field |
| MAT-M1 | `menu.{item.fill,item_label.text.fill,item_icon.icon.fill,item_trailing_icon.icon.fill} @ {selected} context combo_box / {checked}` (the chosen option of a select menu, `selected` in `context combo_box`; an item chosen in a selectable menu outside one, toggled on, `checked`: S7.2; the keyboard highlight is the focus state layer) | rule |
| MAT-M2 | `menu.item.fill.append (state layer) @ {focus_visible} / {pressed}` | rule |
| MAT-M2 | `menu.item_label.text.fill @ {focus_visible} / {pressed}` | rule |
| MAT-M3 | `menu.{item_icon,item_trailing_icon}.icon.fill @ {hover} / {focus_visible} / {pressed} / {disabled}` | rule |
| MAT-M3 | `menu.item_trailing_icon.icon.size` | field |
| MAT-M3 | `menu.item_icon.opacity @ {disabled}` | rule |
| MAT-M4 | `menu.item_subtitle.text` | field |
| MAT-M4 | `menu.accelerator.text` | field |
| MAT-M5 | `menu.submenu_arrow.glyph` | field |
| MAT-M5 | `menu.values.{submenu_open_delay,submenu_close_delay}` | field |
| MAT-M6 | `menu.separator.{min_height,padding}` | field |
| MAT-M7 | `menu.body.padding (top, bottom)` | field |
| MAT-M8 | `menu.body.{min_width,max_width}` | field |
| MAT-M9 | `menu.values.presentations (event Appear: ClipReveal on the body, Fade on item with a stagger; event Disappear: its own duration and timing; ADD-B09, ADD-B10, ADD-A25)` | field |
| MAT-M10 | `menu.body.corners @ cond flavor:expressive` | rule |
| MAT-M10 | `menu.item.corners @ cond flavor:expressive / pos first cond flavor:expressive / pos last cond flavor:expressive / {selected} context combo_box cond flavor:expressive / {checked} cond flavor:expressive` (the selected item's shape: S7.2, as MAT-M1) | rule |
| MAT-M10 | `menu.group.{fill,corners,padding}` | field |
| MAT-M10 | `menu.values.group_gap` | field |
| MAT-M10 | `menu.{body.fill,group.fill,item.fill,item_label.text.fill,item_icon.icon.fill} @ variant @vibrant cond flavor:expressive / {selected} variant @vibrant context combo_box cond flavor:expressive / {checked} variant @vibrant cond flavor:expressive` | rule |
| MAT-M10 | `menu.body.padding, menu.item.{min_width,padding} @ variant horizontal; menu.item.{min_width,padding}, menu.item_icon.icon @ variant icon_only` | rule |
| MAT-M10 | `menu.section_header.{text,padding,min_height} @ cond flavor:expressive` | rule |
| MAT-M10 | `menu.item.{min_height,padding}, menu.values.icon_text_gap @ cond flavor:expressive` | rule |
| MAT-M11 | `menu.item_label.text.{letter_spacing,line_height}` | field |
| MAT-T1 | `tooltip.label.text @ variant rich (the supporting text)` | rule |
| MAT-T1 | `tooltip.title.text` | field |
| MAT-T1 | `tooltip.body.{fill,shadow,corners,padding} @ variant rich` | rule |
| MAT-T1 | `button.label.text.fill, button.body.fill.append (state layer) @ {hover} / {focus_visible} / {pressed}, each context tooltip.body variant rich (the actions)` | rule |
| MAT-T2 | `tooltip.values.display_time` | field |
| MAT-T3 | `tooltip.values.presentations (events Appear, Disappear: steps Fade and Scale, ADD-B10)` | field |
| MAT-T4 | `tooltip.values.placement (MaterialTooltip with its params.gap)` | placement |
| MAT-T4 | `tooltip.arrow.{min_width,min_height,glyph}` | field |
| MAT-T5 | `tooltip.body.min_height` | field |
| MAT-T6 | `tooltip.values.{show_on_press,show_delay} @ cond input:touch (show_delay_ms = token.behavior.long_press)` | rule |
| MAT-T6 | - | readers (R18) |
| MAT-T7 | `tooltip.label.text.{letter_spacing,line_height}` | field |
| MAT-SC1 | `scrollbar.values.auto_hide_delay; scrollbar.values.presentations (event Disappear: Fade)` | field |
| MAT-SC1 | - | readers (R18) |
| MAT-SC2 | `token.behavior.scroll.{overscroll,overscroll_effect,overscroll_extent} (OverscrollEffect::Stretch, how far, its spring back; ADD-A29)` | token |
| MAT-SC3† | `scrollbar.thumb.{fill,min_width,min_height} (fill = alpha(role.@<role>, <fraction>)); scrollbar.track.fill; scrollbar.body.padding (the insets along and across the axis)` | field |
| MAT-SC3† | `scrollbar.values.{thumb_max_fraction,auto_hide_delay,always_visible}; scrollbar.values.presentations (events Appear: Fade while scrolling, Disappear: Fade after the delay)` | field |
| MAT-SC3† | `scrollbar.values.edge @ variant vertical / !variant vertical` (the end edge of a vertical bar, mirrored in RTL; the bottom of a horizontal one); `scrollbar.body.padding @ variant vertical / dir rtl` | rule |
| MAT-SC4† | - | newest (R1.5) |
| MAT-S1 | `slider.thumb.{min_width,min_height,corners}` | field |
| MAT-S1 | `slider.thumb.{min_width,min_height} @ {hover} / {focused} / {pressed} / {disabled}` | rule |
| MAT-S2 | `slider.values.thumb_track_gap` | field |
| MAT-S3 | `slider.track.{corners,min_height}; slider.fill.{corners,min_height} (per corner: the outer and the inner ends)` | field |
| MAT-S4 | `slider.stop_indicator.{fill,min_width,min_height,corners}` | field |
| MAT-S4 | `slider.stop_indicator.fill @ !{in_fill} / {in_fill} / {disabled} / {in_fill,disabled}` | rule |
| MAT-S4 | `slider.values.stop_indicator_gap` | field |
| MAT-S5 | `slider.tick.fill @ !{in_fill} / {in_fill} / {disabled} / {in_fill,disabled} (colour and opacity)` | rule |
| MAT-S5 | `slider.tick.corners` | field |
| MAT-S6 | `slider.value_indicator.{fill,min_height,corners,text}` | field |
| MAT-S6 | `slider.values.value_indicator_gap` | field |
| MAT-S6 | `slider.value_indicator.{transform,transitions}; slider.value_indicator (absent = true) outside the states it shows in` (its show and hide: it enters from transparent and leaves to transparent by its transitions, S2.7) | rule |
| MAT-S6 | `slider.value_indicator.transitions @ cond reduced_motion` | rule |
| MAT-S7 | `slider.halo.{fill,min_width,min_height} @ {hover} / {focused} / {pressed}` | rule |
| MAT-S8 | `slider.thumb.fill @ {focused} / {pressed}` | rule |
| MAT-S8 | `slider.{fill,track}.fill @ {focused} / {pressed}` | rule |
| MAT-S9 | `slider.thumb.shadow = Shadow::Elevation(<key>) (its layers' colour the shadow colour)` | field |
| MAT-S9 | `slider.thumb.shadow @ {disabled}` | rule |
| MAT-S9 | `slider.track.shadow` (its elevation) | field |
| MAT-S10 | `slider.thumb.stroke @ {@overlapping}` | rule |
| MAT-S10 | `slider flags @overlapping` | flag |
| MAT-S11 | `slider.{track,fill}.{min_height,corners}, slider.thumb.{min_height,corners}, slider.inset_icon.{icon,padding} @ size extra_small / size small / size regular / size large / size extra_large (ADD-A62)` | rule |
| MAT-S11 | `slider.inset_icon.{icon,padding}` | field |
| MAT-S12 | `slider.{track,fill,thumb}.* @ variant range / variant centered / variant vertical` | rule |
| MAT-P1 | `progress_bar.values.{wave_amplitude,wavelength} @ variant @wavy / {indeterminate} variant @wavy` | rule |
| MAT-P1 | `progress_bar.{indicator,body}.min_height @ variant @wavy` | rule |
| MAT-P2 | `progress_bar.values.indicator_track_gap` | field |
| MAT-P3 | `progress_bar.stop_indicator.{fill,min_width,min_height,corners}` | field |
| MAT-P3 | `progress_bar.values.stop_indicator_gap` | field |
| MAT-P4 | `progress_bar.{track,indicator,body}.min_height @ variant @thick` | rule |
| MAT-P5 | `progress_bar.indicator.corners` | field |
| MAT-P6 | `progress_bar.values.loop (cycle and keyframe curves); progress_bar.secondary_indicator.{fill,corners}` | field |
| MAT-P6 | `progress_bar.indicator.transitions (Geometry: the value change)` | field |
| MAT-P6 | `progress_bar.values.loop (colour keyframes) @ variant @four_color` | rule |
| MAT-P7 | `progress_bar.buffer.{fill,corners}; progress_bar.buffer_dots.{fill,corners,min_width,min_height}` (the dots' colour, shape and size) | field |
| MAT-TB1 | `tab.indicator.{fill,min_height}, tab.values.indicator_length @ variant @secondary` | rule |
| MAT-TB1 | `tab.label.text.fill, tab.icon.icon.fill @ {selected} / {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}, each variant @secondary` | rule |
| MAT-TB2 | `tab.indicator.corners` | field |
| MAT-TB2 | `tab.values.indicator_length` | field |
| MAT-TB3 | `tab.tab.fill.append @ {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}` | rule |
| MAT-TB3 | `tab.label.text.fill @ {focus_visible} / {pressed} / {selected,focus_visible} / {selected,pressed}` | rule |
| MAT-TB4 | `tab.icon.icon.size` | field |
| MAT-TB4 | `tab.icon.icon.fill @ {selected} / {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}` | rule |
| MAT-TB4 | `tab.tab.min_height @ variant with_icon` | rule |
| MAT-TB4 | `tab.values.icon_text_gap` | field |
| MAT-TB5 | `tab.bar.stroke (the bottom edge: colour and width)` | field |
| MAT-TB6 | `tab.values.presentations (event @indicator_move, part indicator: the slide between tabs)` | field |
| MAT-TB6 | `tab.values.presentations (event @indicator_move, part indicator) @ cond reduced_motion` | rule |
| MAT-TB7 | `tab.bar.padding (start) @ variant @scrollable` | rule |
| MAT-TB8 | `tab.label.text.{letter_spacing,line_height}` | field |
| MAT-SB1 | `list.row_background.corners @ {selected} variant sidebar (Radius::Full)` | rule |
| MAT-SB1 | `list.row.min_height, list.values.row_background_inset @ variant sidebar (the indicator's height; its width, the row less the insets)` | rule |
| MAT-SB2 | `list.row_background.fill.append @ {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}, each variant sidebar` | rule |
| MAT-SB2 | `list.leading_icon.icon.fill, list.row.text.fill @ {selected} / {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}, each variant sidebar` | rule |
| MAT-SB3 | `list.leading_icon.icon.fill @ {selected} variant sidebar / !{selected} variant sidebar` | rule |
| MAT-SB4 | `list.section_header.text @ variant sidebar` | rule |
| MAT-SB5 | `list.trailing_text.text @ variant sidebar` | rule |
| MAT-SB6 | `sidebar.pane.fill, sidebar.pane.shadow = Shadow::Elevation(<key>) @ variant overlay` | rule |
| MAT-SB6 | `sidebar.scrim.{fill,opacity}` | field |
| MAT-SB7 | `sidebar.values.width; sidebar.pane.min_height` (the container's width and height) | field |
| MAT-SB7 | `sidebar.pane.corners (per corner: the end corners rounded)` | field |
| MAT-SB7 | `sidebar.pane.corners @ dir rtl` | rule |
| MAT-SB7 | `sidebar.pane.corners @ variant bottom_sheet` | rule |
| MAT-SB8 | `list.row.text.font.weight @ {selected} variant sidebar` | rule |
| MAT-SB9 | `sidebar.values.presentations (events Appear, Disappear: steps on pane, Slide, and scrim, Fade, each with its duration, timing and delay: the scrim's fade is a step of these events; ADD-B09)` | field |
| MAT-SB9 | `sidebar.pane.transitions (Geometry: the width change)` | field |
| MAT-TA1 | `toolbar.bar.fill, toolbar.bar.shadow (elevation) @ {scrolled}` | rule |
| MAT-TA1 | `toolbar.bar.transitions (Fill, Shadow)` | field |
| MAT-TA2 | `button.icon.icon.fill @ context toolbar.bar (trailing) / variant back context toolbar.bar / variant pane_toggle context toolbar.bar (leading)` (the general rule first, so the later leading ones win, S8) | rule |
| MAT-TA3 | `toolbar.bar.min_height, toolbar.title.text @ variant @center_aligned / variant @medium / variant @large / variant @medium_flexible / variant @large_flexible` | rule |
| MAT-TA3 | `toolbar.values.title_alignment, toolbar.values.collapse_on_scroll` (the base: the small bar's) | field |
| MAT-TA3 | `toolbar.values.title_alignment @ variant @center_aligned; toolbar.values.collapse_on_scroll @ variant @medium / variant @large / variant @medium_flexible / variant @large_flexible` | rule |
| MAT-TA4 | `toolbar.subtitle.text` | field |
| MAT-TA5 | `avatar.body.{min_width,min_height,corners} @ context toolbar.bar / context toolbar.bar variant @center_aligned` | rule |
| MAT-TA6 | `search_field.bar.{fill,min_height,corners,padding} @ context toolbar.bar; input.placeholder.text.fill @ context any search_field context toolbar.bar (the label colour)` | rule |
| MAT-TA6 | `search_field.bar.fill @ context toolbar.bar {scrolled}` | rule |
| MAT-TA7 | `toolbar.title.text.{letter_spacing,line_height}` | field |
| MAT-TA8† | `button.icon.icon, button.body.{min_width,min_height} @ variant overflow context toolbar.bar / variant overflow context toolbar.bar variant vertical` | rule |
| MAT-TA8† | `menu.* @ context button variant overflow; button.values.menu_placement @ variant overflow context toolbar.bar` | rule |
| MAT-L1 | `list.row.min_height @ variant with_subtitle / variant three_line` | rule |
| MAT-L1 | `list.secondary_text.text, list.overline_text.text, list.trailing_text.text` | field |
| MAT-L2 | `list.leading_icon.{icon,corners,min_width,min_height} @ variant with_avatar / variant with_image / variant with_video` | rule |
| MAT-L2 | `avatar.body.{fill,min_width,min_height,corners}, avatar.initials.text @ context list.row (the leading avatar and its label)` | rule |
| MAT-L3 | `list.trailing_icon.icon.{size,fill}` | field |
| MAT-L3 | `list.trailing_icon.icon.fill @ !{selected} / {selected}` | rule |
| MAT-L4 | `list.row_background.fill.append @ {focus_visible} / {pressed} / {dragged}; list.row_background.fill @ {disabled}` | rule |
| MAT-L4 | `list.row_background.shadow @ {dragged}` | rule |
| MAT-L4 | `list.row.text.fill, list.leading_icon.icon.fill, list.trailing_icon.icon.fill @ {hover} / {focus_visible} / {pressed} / {dragged} / {disabled}` | rule |
| MAT-L4 | `list.leading_icon.icon.fill, list.trailing_icon.icon.fill @ {disabled} (the disabled opacities as the paint's alpha: the icon parts list no opacity); list.row.opacity @ {disabled}` | rule |
| MAT-L5 | `list.row_background.fill @ {selected}` | rule |
| MAT-L5 | `list.row_background.fill.append @ {selected,hover} / {selected,focus_visible} / {selected,pressed} / {selected,dragged} / {selected,disabled}` | rule |
| MAT-L5 | `list.row.text.fill, list.overline_text.text.fill, list.secondary_text.text.fill, list.leading_icon.icon.fill, list.trailing_icon.icon.fill @ {selected} / {selected,hover} / {selected,focus_visible} / {selected,pressed} / {selected,dragged}` | rule |
| MAT-L5 | `list.row_background.fill, list.row.opacity, list.leading_icon.icon.fill, list.trailing_icon.icon.fill @ {selected,disabled} (the selected-disabled opacities per part; the background and icons as the paint's alpha)` | rule |
| MAT-L6 | `list.row_separator.stroke, list.values.row_separator_inset, list.values.row_gap` | field |
| MAT-L7 | `list.values.leading_gap` | field |
| MAT-L8 | `list.row_background.fill, list.values.row_gap @ variant separated cond flavor:expressive` | rule |
| MAT-L8 | `list.row_background.corners @ cond flavor:expressive (the rest shape)` | rule |
| MAT-L8 | `list.row_background.corners @ {hover} / {focus_visible} / {pressed} / {dragged} / {disabled} / {selected} / {selected,hover} / {selected,focus_visible} / {selected,pressed} / {selected,dragged} / {selected,disabled}, each cond flavor:expressive` | rule |
| MAT-L8 | `list.row_background.transitions (Corners) @ cond flavor:expressive` | rule |
| MAT-L8 | `list.leading_icon.icon.size, list.trailing_icon.icon.size, list.leading_icon.corners @ cond flavor:expressive` | rule |
| MAT-L9 | `list.row_background.{fill,corners} @ {open}` (the expanded item's container colour and shape) | rule |
| MAT-L9 | `list.disclosure.{fill,corners,glyph} @ {open} / !{open}` | rule |
| MAT-L10 | `list.row_background.fill, list.row.text.fill, list.leading_icon.icon.fill, list.row_background.corners, list.row_background.shadow @ {dragged}` | rule |
| MAT-L10 | `list.drop_indicator.fill (drop zone)` | field |
| MAT-L11 | `list.swipe_action.{fill,icon,corners}` | field |
| MAT-L11 | `list.swipe_action.fill.append @ {hover} / {focus_visible} / {pressed}` | rule |
| MAT-L12 | `list.header_section.text.fill @ {hover} variant table` | rule |
| MAT-L12 | `list.sort_indicator.glyph.fill @ variant table` | rule |
| MAT-L12 | `list.footer.{fill,min_height,text}` | field |
| MAT-L13 | `list.row_background.fill @ {selected} variant table / !{selected} variant table; list.row_background.fill.append @ {hover} variant table` | rule |
| MAT-L13 | `list.view.stroke, list.view.corners @ variant table` | rule |
| MAT-L14 | `list.row.text.{letter_spacing,line_height}` | field |
| MAT-PO1 | `popover.body.shadow = Shadow::Elevation(<key>)` | field |
| MAT-PO1 | `popover.body.fill.append = alpha(role.@surface_tint, <fraction>) (the surface-tint layer)` | field |
| MAT-SP1 | `splitter.grip.{min_width,min_height,corners}` | field |
| MAT-SP1 | `splitter.grip.{min_width,min_height,corners,fill} @ {pressed}` (the handle's pressed width, height, shape and colour, `_md-comp-drag-handle.scss:23-53`) | rule |
| MAT-SP1 | `splitter.divider.min_width` | field |
| MAT-SP2 | `splitter.grip.fill.append @ {hover} / {focus_visible} / {pressed}` | rule |
| MAT-SP2 | `splitter.grip.shadow` | field |
| MAT-SP3 | `layout.values.pane_gap @ h_size_class compact / h_size_class medium / h_size_class expanded / h_size_class large / h_size_class extra_large` | rule |
| MAT-D1 | `separator.line.margin @ variant @inset / variant @inset_start / variant @inset_end` | rule |
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
| MAT-TS1 | `token.text.roles.display_large (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.display_medium (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.display_small (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.large_title (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.title1 (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.title2 (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.headline (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.subheadline (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.body (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.callout (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.caption (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.caption2 (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.label (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.title3 (= token.text.keyed.<key>)` | token |
| MAT-TS1 | `token.text.roles.footnote (= token.text.keyed.<key>)` | token |
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
| MAT-TS3 | `token.text.keyed.<key>.font.weight (each style's weight, by md.ref.typeface's weight names)` | token |
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
| MAT-TS4 | `token.text.roles.body_emphasized (= token.text.keyed.<key>)` | token |
| MAT-TS4 | `token.text.roles.caption_emphasized (= token.text.keyed.<key>)` | token |
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
| MAT-LA1 | `layout.values.window_margin, layout.values.pane_gap @ h_size_class compact / h_size_class medium / h_size_class expanded / h_size_class large / h_size_class extra_large` | rule |
| MAT-LA1 | `token.size_classes` | token |
| MAT-SW1 | `switch.thumb.min_width, switch.thumb.min_height @ {pressed}` | rule |
| MAT-SW2 | `switch.thumb.fill @ {hover} / {focus_visible} / {pressed} / {checked,hover} / {checked,focus_visible} / {checked,pressed}` | rule |
| MAT-SW3 | `switch.thumb_icon.icon.{size,fill}` | field |
| MAT-SW3 | `switch.thumb_icon.glyph @ !{checked} / {checked}` (the close and the check symbol) | rule |
| MAT-SW3 | `switch.thumb_icon.icon.fill @ {hover} / {focus_visible} / {pressed} / {disabled} / {checked,hover} / {checked,focus_visible} / {checked,pressed} / {checked,disabled}; switch.thumb_icon.opacity @ {disabled}` | rule |
| MAT-SW3 | `switch.thumb.{min_width,min_height} @ variant @icons` | rule |
| MAT-SW4 | `switch.state_layer.{min_width,min_height,corners} (corners Radius::Full)` | field |
| MAT-SW4 | `switch.state_layer.{fill,opacity} @ {hover} / {focus_visible} / {pressed} / {checked,hover} / {checked,focus_visible} / {checked,pressed}` | rule |
| MAT-SW5 | `switch.track.fill, switch.track.stroke @ {focus_visible} / {pressed} / {checked,focus_visible} / {checked,pressed}` | rule |
| MAT-SW6 | `switch.thumb.fill @ {disabled,checked} / {disabled} !{checked} (colour and opacity, the opacity as the paint's alpha: the thumb lists no opacity)` | rule |
| MAT-SW7 | `switch.thumb.shadow = Shadow::Elevation(<key>) (its layers' colour the shadow colour)` | field |
| MAT-SW8 | `switch.thumb.transitions (Geometry: the handle's travel; timing = token.motion.timings.<key>, a cubic Bézier that overshoots)` | field |
| MAT-SW8 | `switch.thumb.transitions (Geometry, Fill); switch.track.transitions (Fill, Stroke); switch.thumb_icon.transitions` | field |
| MAT-SW9 | `switch.track.focus.offset` | field |
| MAT-DG1 | `dialog.{header,message,action_area}.padding` | field |
| MAT-DG1 | `dialog.values.{title_message_gap,message_actions_gap}` | field |
| MAT-DG2 | `dialog.icon.icon.fill` | field |
| MAT-DG2 | `dialog.values.title_alignment @ variant with_icon (ADD-A61)` | rule |
| MAT-DG3 | `button.label.text @ context dialog.action_area (the font)` | rule |
| MAT-DG3 | `button.label.text.fill @ {hover} / {focus_visible} / {pressed} / {disabled}, each context dialog.action_area` | rule |
| MAT-DG3 | `button.body.fill.append (state layer) @ {hover} / {focus_visible} / {pressed}, each context dialog.action_area` | rule |
| MAT-DG4 | `dialog.header.stroke @ {scrolled}` | rule |
| MAT-DG4 | `dialog.action_area.stroke @ {overflowing} (ADD-A13)` | rule |
| MAT-DG6 | `dialog.values.presentations (events Appear, Disappear; steps on body, message, scrim; ADD-B09)` | field |
| MAT-DG7 | `dialog.subtitle.text` | field |
| MAT-DG8 | `dialog.values.window_margin` | field |
| MAT-DG9 | `dialog.{title,message}.text.{letter_spacing,line_height}` | field |
| MAT-DG10† | `dialog.values.action_layout (alignment, stack_rule WhenOverflowing, stacked_order, row_gap_px, column_gap_px)` | field |
| MAT-SN1 | `spinner.track.fill` | field |
| MAT-SN1 | `spinner.values.{stroke_width,track_stroke_width,indicator_track_gap}` | field |
| MAT-SN2 | `spinner.values.{wave_amplitude,wavelength} @ variant @wavy` | rule |
| MAT-SN2 | `spinner.body.{min_width,min_height} @ variant @wavy` | rule |
| MAT-SN3 | `spinner.{body.min_width,body.min_height,values.stroke_width,values.track_stroke_width,values.indicator_track_gap} @ variant @thick` | rule |
| MAT-SN4 | `spinner.values.cap` | field |
| MAT-SN5 | `spinner.values.loop (colour keyframes) @ variant @four_color` | rule |
| MAT-SN6 | `spinner.values.loop` | field |
| MAT-SN6 | `spinner.indicator.transitions (Geometry: the determinate value change)` | field |
| MAT-CB1 | `combo_box.floating_label.{text,padding,opacity,transform,transitions}; combo_box.supporting_text.{text,padding,opacity}; combo_box.icon.{icon,opacity}; combo_box.values.{supporting_text_gap,notch_padding,floating_label_gap}` | field |
| MAT-CB1 | `combo_box.body.stroke @ {hover} / {focused} / {invalid} / {disabled} (colour and width)` | rule |
| MAT-CB1 | `combo_box.{floating_label.text.fill,supporting_text.text.fill,icon.icon.fill} @ {hover} / {focused} / {invalid} / {disabled}; combo_box.floating_label.{text,transform} @ {empty} / {focused} / {open}` (the label resting inside an empty select, floated otherwise) | rule |
| MAT-CB1 | `combo_box.body.{fill,corners,stroke} @ variant @filled; combo_box.body.fill.append (state layer) @ {hover} variant @filled` | rule |
| MAT-CB2 | `combo_box.arrow.glyph.fill @ {hover} / {focused} / {invalid} / {disabled} (the combo box's states, which its arrow takes; the disabled opacity as the paint's alpha)` | rule |
| MAT-CB3 | `combo_box.arrow.transform @ {open}` | rule |
| MAT-CB3 | `combo_box.arrow.transitions` | field |
| MAT-CB3 | `menu.item.fill @ {selected} context combo_box; menu.item.min_height @ context combo_box` | rule |
| MAT-SG1 | `segmented_control.check.glyph (its dimensions and fill)` | field |
| MAT-SG1 | `segmented_control.check.glyph.fill @ {selected} / {selected,hover} / {selected,focus_visible} / {selected,pressed}; segmented_control.check.transitions (Geometry, Opacity: its show and width animation)` | rule |
| MAT-SG2 | `segmented_control.segment.fill.append (state layers), segmented_control.label.text.fill, segmented_control.icon.icon.fill @ {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}` | rule |
| MAT-SG3 | `segmented_control.segment.corners @ pos first / pos last / pos only` | rule |
| MAT-SG4 | `segmented_control.segment.stroke, segmented_control.icon.icon.fill, segmented_control.icon.opacity @ {disabled}` | rule |
| MAT-CA1 | `card.body.fill, card.body.stroke, card.body.shadow (elevation) @ variant @filled / variant @elevated / variant @outlined` | rule |
| MAT-CA2 | `card.body.fill.append @ {hover} / {focus_visible} / {pressed} / {dragged}` | rule |
| MAT-CA2 | `card.body.shadow @ {hover} / {focus_visible} / {pressed} / {dragged}, each variant @elevated / variant @filled` | rule |
| MAT-CA2 | `card.body.stroke @ {hover} / {focus_visible} / {pressed} / {disabled}, each variant @outlined` | rule |
| MAT-CA2 | `card.body.fill, card.body.opacity @ {disabled}` | rule |
| MAT-CA3 | `card.icon.icon.{size,fill}` | field |
| MAT-EX1 | `list.row_background.{fill,corners} @ {open}` (`md.comp.list.expand.container.shape` with the fill) | rule |
| MAT-EX1 | `list.disclosure.fill, list.disclosure.corners, list.disclosure.glyph @ {open}` | rule |
| MAT-N1 | `button.body.{fill,min_width,min_height,corners,shadow}, button.icon.icon.{size,fill} @ variant @fab size small / variant @fab size regular / variant @fab size large cond flavor:expressive / variant @fab size extra_large (shadow Shadow::Elevation; small, standard, Expressive's medium and large FAB, declared by sizes_by_variant, S10)` | rule |
| MAT-N1 | `button.body.fill.append (state layer), button.body.shadow @ {hover} / {focus_visible} / {pressed}, each variant @fab / variants {@fab,@lowered}; button.body.shadow @ variants {@fab,@lowered}` | rule |
| MAT-N1 | `button.body.fill, button.icon.icon.fill @ variants {@fab,primary} / variants {@fab,@secondary} / variants {@fab,@tertiary} / variants {@fab,@surface} / variants {@fab,@primary_container} / variants {@fab,@secondary_container} / variants {@fab,@tertiary_container} / variants {@fab,@branded}` (the colour styles, the Expressive container styles, the branded FAB) | rule |
| MAT-N1 | `button.body.fill.append = alpha(role.@surface_tint, <fraction>) @ variant @fab (the surface tint); the shadow colour as the elevation layers' colour` | rule |
| MAT-N1 | `button.body.focus @ {focus_visible} variant @fab; button.body.transitions (Shadow) @ variant @fab` | rule |
| MAT-N2 | `button.label.text, button.values.icon_text_gap, button.body.{padding,min_height,corners} @ variant @extended_fab size small / variant @extended_fab size regular / variant @extended_fab size large` | rule |
| MAT-N2 | `button.body.shadow @ variants {@extended_fab,@lowered}; button.body.fill, button.label.text.fill, button.icon.icon.fill @ variants {@extended_fab,primary} / variants {@extended_fab,@secondary} / variants {@extended_fab,@tertiary} / variants {@extended_fab,@surface}` | rule |
| MAT-N3 | `component.fab_menu.item.{fill,corners,padding,shadow,min_height}, component.fab_menu.item_label.text, component.fab_menu.item_icon.icon, component.fab_menu.values.item_gap, component.fab_menu.values.presentations (events Appear, Disappear: Morph); component.fab_menu.{item.fill,item_label.text,item_icon.icon} @ variant @primary / variant @secondary / variant @tertiary` (the colour styles) | component |
| MAT-N3 | `button.body.{min_width,min_height,corners,shadow}, button.icon.icon @ variant close context component.fab_menu (the close button)` | rule |
| MAT-N3 | `button.body.fill, button.icon.icon.fill @ variant close context component.fab_menu variant @primary / variant close context component.fab_menu variant @secondary / variant close context component.fab_menu variant @tertiary` (the close button's colours per style, the primary, secondary and tertiary `_md-comp-fab-menu-*-close-button.scss`) | rule |
| MAT-N4 | `button.body.{fill,stroke,focus}, button.icon.icon.{size,fill} @ variants {icon_only,primary} / variants {icon_only,tinted} / variants {icon_only,@outlined} / variant icon_only` | rule |
| MAT-N4 | `button.body.fill.append (state layer), button.icon.icon.fill, button.body.stroke @ {hover} / {focus_visible} / {pressed} / {disabled}, each variant icon_only (the disabled opacities as the paints' alpha)` | rule |
| MAT-N4 | `button.body.fill, button.icon.icon.fill, button.body.stroke @ !{checked} / {checked} / {checked,hover} / {checked,focus_visible} / {checked,pressed}, each variants {icon_only,toggle}` | rule |
| MAT-N5 | `button.body.{min_width,min_height,padding}, button.icon.icon.size @ size extra_small / size small / size regular / size large / size extra_large, each variant icon_only cond flavor:expressive` | rule |
| MAT-N5 | `button.body.padding @ variants {icon_only,@narrow} / variants {icon_only,@wide}, each cond flavor:expressive` | rule |
| MAT-N5 | `button.body.corners @ variants {icon_only,@square} / {checked} variant icon_only / {pressed} variant icon_only, each cond flavor:expressive; button.body.transitions (Corners, Timing::Spring) @ variant icon_only cond flavor:expressive` | rule |
| MAT-N6 | `segmented_control.segment.corners @ variant @connected / {pressed} variant @connected / {selected} variant @connected (the inner corners; the outer by pos first / pos last); segmented_control.track.corners @ variant @connected` | rule |
| MAT-N6 | `segmented_control.values.pressed_width_factor @ variant @standard_group` (the pressed width multiplier, which the connector applies at layout); `segmented_control.segment.transitions (Geometry, Timing::Spring) @ variant @standard_group` | rule |
| MAT-N6 | `segmented_control.values.item_gap, segmented_control.segment.min_height @ size extra_small / size small / size regular / size large / size extra_large, each variant @standard_group / variant @connected` | rule |
| MAT-N7 | `split_button.values.part_gap, split_button.{primary,secondary}.{padding,corners}, split_button.arrow.glyph (its size) @ size extra_small / size small / size regular / size large / size extra_large` | rule |
| MAT-N7 | `split_button.{primary,secondary}.corners @ {hover} / {pressed}, split_button.secondary.corners @ {open} (the trailing half's one state, its menu open, MAT-N7), each size extra_small / size small / size regular / size large / size extra_large (the inner and outer corners)` | rule |
| MAT-N7 | `split_button.arrow.{transform,transitions} @ {open} (the trailing button's rotation)` | rule |
| MAT-N8 | `button.body.{fill,stroke,corners}, button.label.text, button.icon.icon, button.menu_indicator.glyph @ variants {menu,primary} / variants {menu,@outlined} / variant menu` | rule |
| MAT-N8 | `button.body.fill.append (state layer), button.label.text.fill, button.icon.icon.fill, button.menu_indicator.glyph.fill @ {hover} / {focus_visible} / {pressed} / {disabled}, each variant menu` | rule |
| MAT-N9 | `chip.body.{fill,stroke,shadow,corners,min_height,padding}, chip.label.text, chip.leading_icon.icon, chip.trailing_icon.icon @ variant @assist / variant @filter / variant @input / variant @suggestion / variants {@assist,@elevated} / variants {@suggestion,@elevated} / variants {@filter,@elevated}` | rule |
| MAT-N9 | `chip.body.{fill,stroke}, chip.label.text.fill, chip.leading_icon.icon.fill, chip.trailing_icon.icon.fill @ {checked} variant @filter / {selected} variant @input` | rule |
| MAT-N9 | `chip.body.fill.append (state layer), chip.body.shadow @ {hover} / {focus_visible} / {pressed} / {dragged}; chip.body.opacity, chip.body.stroke @ {disabled}; chip.body.focus @ {focus_visible}` | rule |
| MAT-N9 | `avatar.body.{min_width,min_height,corners} @ context chip.body variant @input (the input chip's avatar)` | rule |
| MAT-N10 | `badge.body.{fill,min_width,min_height,corners} @ variant dot; badge.body.{fill,min_width,min_height,corners,padding}, badge.label.text @ variant count` | rule |
| MAT-N10 | `badge.values.{anchor_offset_x,anchor_offset_y}` | field |
| MAT-N11 | `toolbar.bar.{fill,min_height,corners,shadow} @ variant bottom (shadow Shadow::Elevation); toolbar.bar.fill.append = alpha(role.@surface_tint, <fraction>) @ variant bottom` | rule |
| MAT-N12 | `bottom_tab_bar.bar.{fill,min_height,corners,shadow}; bottom_tab_bar.indicator.{fill,min_width,min_height,corners}; bottom_tab_bar.item_label.text; bottom_tab_bar.values.item_gap` | field |
| MAT-N12 | `bottom_tab_bar.item.fill.append (state layer), bottom_tab_bar.item_icon.icon.fill, bottom_tab_bar.item_label.text.fill @ !{selected} / {selected} / {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}` | rule |
| MAT-N12 | `bottom_tab_bar.item.padding, bottom_tab_bar.indicator.{min_width,min_height} @ variant stacked / variant inline, each cond flavor:expressive` | rule |
| MAT-N12 | `badge.* @ context bottom_tab_bar.item` | rule |
| MAT-N12 | `bottom_tab_bar.values.presentations (event @indicator_move, part indicator)` | field |
| MAT-N13 | `sidebar.pane.{fill,corners,shadow,min_width} @ variant compact` | rule |
| MAT-N13 | `list.row_selection_indicator.{fill,corners,min_width,min_height} @ {selected} variant sidebar context sidebar.pane variant compact / {selected} variants {sidebar,icon_only} context sidebar.pane variant compact` (the no-label form, the content variant `icon_only` on the row) | rule |
| MAT-N13 | `list.leading_icon.icon.fill, list.row.text.fill @ {selected} / {hover} / {focus_visible} / {pressed} / {selected,hover} / {selected,focus_visible} / {selected,pressed}, each variant sidebar context sidebar.pane variant compact` | rule |
| MAT-N13 | `button.icon.icon @ variant pane_toggle context sidebar.header (the menu icon)` | rule |
| MAT-N13 | `badge.* @ context list.row context sidebar.pane variant compact` | rule |
| MAT-N13 | `sidebar.header.padding @ variant compact` | rule |
| MAT-N13 | `sidebar.pane.min_width @ variants {compact,@expanded}` | rule |
| MAT-N13 | `list.values.row_gap @ variant sidebar context sidebar.pane variant compact / variant sidebar context sidebar.pane variants {compact,@expanded}` | rule |
| MAT-N13 | `sidebar.pane.{fill,shadow} @ variants {compact,@modal}` | rule |
| MAT-N14 | `search_field.bar.{fill,min_height,corners,shadow,padding}; search_field.supporting_text.text` | field |
| MAT-N14 | `input.{text.text,placeholder.text} @ context any search_field; input.icon.icon @ variant leading context any search_field` | rule |
| MAT-N14 | `button.icon.icon @ variant icon_only context search_field (the trailing icons); avatar.body.{min_width,min_height} @ context search_field` | rule |
| MAT-N14 | `search_field.bar.{fill,corners,padding}, search_field.body.padding @ variant @contained cond flavor:expressive; search_field.values.presentations (event @view_show, Timing::Spring) @ variant @contained` | rule |
| MAT-N15 | `search_field.view.{fill,shadow,corners}, search_field.view_header.min_height, search_field.view_divider.stroke @ variant @docked / variant @full_screen` | rule |
| MAT-N15 | `input.{text.text,placeholder.text} @ context search_field.view; search_field.supporting_text.text` | rule |
| MAT-N15 | `list.row_background.corners, list.values.row_gap @ context search_field.view variant @contained (the contained bar's results)` | rule |
| MAT-N15 | `search_field.values.presentations (events @view_show, @view_hide)` | field |
| MAT-N16 | `date_picker.popup.{fill,corners,shadow,min_width,min_height}, date_picker.header.text, date_picker.headline.text, date_picker.supporting_text.text, date_picker.divider.stroke @ variant with_popup / variant @modal / variant @input` | rule |
| MAT-N16 | `calendar.weekday_label.text; calendar.day.{min_width,min_height,corners}; calendar.year.{min_width,min_height,corners,text}` | field |
| MAT-N16 | `calendar.day.fill, calendar.day_label.text.fill @ {selected} / {today}; calendar.today_marker.stroke @ {today}; calendar.day.opacity @ {other_month}` | rule |
| MAT-N16 | `calendar.day.fill.append (state layer) @ {hover} / {pressed} / {focus_visible}; calendar.year.fill @ {selected} / {hover} / {pressed} / {focus_visible}` | rule |
| MAT-N16 | `calendar.range_band.fill, calendar.day.fill @ {@range_start} / {@range_end} / {@in_range} (the band behind each day, by the day's own flags); calendar.title.text (the range picker's month headers)` | rule |
| MAT-N16 | `button @ variant menu context calendar.header (the month and year menu button)` | rule |
| MAT-N16 | `input.* @ context any date_picker variant @input` | rule |
| MAT-N17 | `time_picker.{dial,dial_selector,dial_center}.{fill,corners,min_width,min_height}, time_picker.dial_track.{fill,min_width}, time_picker.dial_label.text @ variant @dial` | rule |
| MAT-N17 | `time_picker.dial_label.text.fill @ {selected}` (the label under the selector, an element) | rule |
| MAT-N17 | `time_picker.{selector_cell,period_selector}.{fill,stroke,corners,min_width,min_height,text} @ {selected} / {hover} / {pressed} / {focus_visible} / {selected,hover} / {selected,pressed} / {selected,focus_visible}, each variant @dial` (the period selector also under `variant @input`; the input's hour and minute fields are `input` elements, below) | rule |
| MAT-N17 | `time_picker.headline.text, time_picker.time_separator.text` | field |
| MAT-N17 | `time_picker.popup.{fill,corners,shadow}` (the time picker's container: its colour, shape and elevation tokens, `_md-comp-time-picker.scss:84-89`) | field |
| MAT-N17 | `input.{body.stroke,floating_label.text,supporting_text.text} @ context time_picker.body[@input]` (the time input fields, their outline, label and supporting text: `input` elements, Nested controls) | rule |
| MAT-N18 | `carousel.item.{fill,corners,shadow,stroke}; carousel.values.item_gap` | field |
| MAT-N18 | `carousel.item.fill.append (state layer) @ {hover} / {pressed} / {focus_visible}` | rule |
| MAT-N18 | `carousel.item.{min_width,max_width,corners} = expressions over element.scroll_progress (the masked shape changing with the scroll, ADD-B13)` | field |
| MAT-N19 | `sheet.body.{fill,corners,shadow}, sheet.body.fill.append (surface tint) @ variants {bottom,@standard} / variants {bottom,@modal} / {minimized} variants {bottom,@standard}` | rule |
| MAT-N19 | `sheet.handle.{fill,min_width,min_height,opacity}; sheet.handle.focus @ {focus_visible}` | rule |
| MAT-N19 | `sheet.values.presentations (events Appear, Disappear, @settle: the drag and settle motion)` | field |
| MAT-N20 | `sheet.body.{fill,corners,shadow,max_width}, sheet.title.text, sheet.divider.stroke @ variants {@side,@standard} / variants {@side,@modal} / variants {@side,@detached}` | rule |
| MAT-N20 | `button.label.text.fill, button.body.fill.append (state layer) @ {hover} / {focus_visible} / {pressed}, each context sheet.body variant @side (the actions)` | rule |
| MAT-N20 | `button @ variant close context sheet.header (the close button)` | rule |
| MAT-N21 | `sheet.body.{fill,shadow,corners}, sheet.body.fill.append (surface tint) @ variant floating` | rule |
| MAT-N22 | `toast.body.{fill,corners,shadow,min_height}; toast.title.text` | field |
| MAT-N22 | `toast.body.min_height @ {multiline}` | rule |
| MAT-N22 | `button.label.text.fill, button.body.fill.append (state layer) @ {hover} / {focus_visible} / {pressed}, each context toast.body (the action)` | rule |
| MAT-N22 | `button.icon.icon.{size,fill} @ variant close context toast.body; button.body.fill.append (state layer) @ {hover} / {focus_visible} / {pressed}, each variant close context toast.body` | rule |
| MAT-N22 | `toast.values.durations; toast.values.presentations (events Appear, Disappear)` | field |
| MAT-N23 | `banner.body.{fill,corners,shadow}, banner.title.text, banner.message.text, banner.icon.icon, banner.image.{corners,min_width,min_height}, banner.divider.stroke @ variant @basic / variant @rich / variant @vibrant` | rule |
| MAT-N23 | `banner.body.min_height / @ {multiline} / @ h_size_class compact / @ {multiline} h_size_class compact` (the heights per line count and form factor, one home); `banner.values.action_gap` | rule |
| MAT-N23 | `button @ variant close context banner.body; button.label.text.fill @ context banner.action_area (the actions)` | rule |
| MAT-N24 | `spinner.indicator.fill, spinner.body.{min_width,min_height} @ variant @loading_indicator (the active indicator's colour and size)` | rule |
| MAT-N24 | `spinner.body.{fill,corners,padding} @ variants {@loading_indicator,@contained} (the container's colour, shape and space round the indicator)` | rule |
| MAT-N24 | `spinner.indicator.shape (a ShapeKind::Material shape, S2.4, ADD-B03, ADD-A99); spinner.values.loop @ variant @loading_indicator (keyframes setting the indicator's shape, with their timing, R25.5)` | rule |
| MAT-N25 | `toolbar.bar.{fill,min_height,corners,shadow,padding}, toolbar.values.item_gap @ variant @docked / variant floating / variants {floating,@vibrant}` | rule |
| MAT-N25 | `toolbar.bar.{min_width,min_height} @ variants {floating,vertical} (the floating toolbar's height and width per orientation, R25.14); toolbar.values.floating_inset @ variant floating` | rule |
| MAT-N25 | `button.{body.fill,icon.icon.fill,label.text.fill} @ {hover} / {focus_visible} / {pressed} / {disabled}, each context toolbar.bar variant floating / context toolbar.bar variants {floating,@vibrant} / context toolbar.bar variant @docked` | rule |
| MAT-N25 | `button.{body.min_width,body.min_height,icon.icon.size} @ variant @fab context any toolbar variant floating (the FAB in the toolbar, its sizes)` | rule |
| MAT-N25 | `toolbar.values.presentations (events Appear, Disappear)` | field |
| MAT-N26 | `dialog.body.{fill,corners,shadow}, dialog.header.{fill,padding,min_height}, dialog.title.text, dialog.action_area.{fill,padding,min_height}, dialog.action_area.stroke (the divider) @ variant full_size` | rule |
| MAT-N26 | `dialog.header.{fill,stroke,shadow} @ {scrolled} variant full_size` (the on-scroll colour and elevation) | rule |
| MAT-N26 | `button.icon.icon @ variant close context dialog.header; button.label.text, button.body.fill.append (state layer) @ {hover} / {focus_visible} / {pressed}, each context dialog.header (the header's action)` | rule |
| MAT-N27 | `menu.{body,item,item_label,item_icon}.* @ context autocomplete` | rule |
| MAT-N27 | `input.* @ context autocomplete / variant @filled context autocomplete (the field: outlined the base)` | rule |
| MAT-N27 | `autocomplete.values.placement (MaterialMenu)` | placement |
| MAT-N28 | `pull_to_refresh.indicator.{fill,corners,shadow,min_width,min_height}; pull_to_refresh.values.trigger_distance` | field |
| MAT-N28 | `spinner.* @ context pull_to_refresh (its spinner or loading shape); pull_to_refresh.values.progress_sweep (the arc following the pull, element.progress)` | rule |
| MAT-N28 | `pull_to_refresh.values.presentations (event Disappear)` | field |
| MAT-N28 | `pull_to_refresh flags pulling, refreshing` | flag |
| MAT-N30† | `edit_menu.body.{fill,corners,shadow}` | field |
| MAT-N30† | `button.label.text @ context edit_menu.body (the items' font and colours); button @ variant overflow context edit_menu.body; edit_menu.* @ variant vertical (the overflow list)` | rule |
| MAT-N30† | `edit_menu.values.placement (AndroidTextToolbar)` | placement |
| MAT-N31† | `picker_wheel.container.{fill,corners,min_height}; picker_wheel.values.visible_items` | field |
| MAT-N31† | `picker_wheel.container.fill @ {disabled}; picker_wheel.item.text.fill @ !{selected} / {selected} / {disabled}; picker_wheel.item.focus @ {focus_visible}` | rule |
| MAT-N31† | `picker_wheel.item.text.{font.size,line_height,letter_spacing}, picker_wheel.item.opacity` as expressions over `element.scroll_progress`, `element.item_index` (the style interpolated between two text roles, ADD-B13) | field |
| MAT-N31† | `picker_wheel.item.transitions (Geometry with Timing::Spring, Fill, Text: the snap to the centre)` | field |
| MAT-X1 | `role.@tertiary, role.@on_tertiary, role.@tertiary_container, role.@on_tertiary_container; role.@primary_container, role.@on_primary_container, role.@secondary_container, role.@on_secondary_container, role.@error_container, role.@on_error_container` | role |
| MAT-X1 | `role.@primary_fixed, role.@primary_fixed_dim, role.@on_primary_fixed, role.@on_primary_fixed_variant (and the same for secondary and tertiary)` | role |
| MAT-X1 | `role.@surface_dim, role.@surface_bright, role.@surface_container_lowest, role.@surface_container_low, role.@surface_container, role.@surface_container_high, role.@surface_container_highest` | role |
| MAT-X1 | `role.@inverse_surface, role.@inverse_on_surface, role.@inverse_primary, role.@outline_variant, role.@surface_variant, role.@surface_tint, role.@background, role.@on_background` | role |
| MAT-X1 | `role.global.scrim (= role.@scrim)` | role |
| MAT-X2 | `token.elevation.<level0 … level5> (the six levels)` | token |
| MAT-X2 | `PartStyle.shadow = Shadow::Elevation(<key>) per part and state (each component's level)` | field |
| MAT-X3 | `token.elevation.<key> layers (material-web's key and ambient ShadowLayers per level)` | token |
| MAT-X3 | `reader:android:android:ambientShadowAlpha, android:spotShadowAlpha, light source dimensions` | readers (R18) |
| MAT-X4 | `PartStyle.shadow = Shadow::Elevation(<key>) @ {hover} / {focus_visible} / {pressed} / {dragged} / {disabled} (buttons, FABs, cards, chips, list rows)` | rule |
| MAT-X4 | `button.body.shadow @ variants {@fab,@lowered} / variants {@extended_fab,@lowered} (the lowered elevation)` | rule |
| MAT-X5 | `token.elevation.<key> layers' ShadowLayer.color (= role.@shadow); a part whose shadow colour differs states Shadow::Layers` | field |
| MAT-X6 | `PartStyle.fill.append = alpha(role.@surface_tint, <fraction>) per elevation` | field |
| MAT-X6 | `role.@surface_tint` | role |
| MAT-X7 | `PartStyle.transitions group Shadow` | field |
| MAT-X8 | `PartStyle.fill.append = alpha(<state-layer role>, token.params.opacity.<state>) @ {hover} / {focus_visible} / {pressed} / {dragged} / {disabled} (per component its state-layer colour, over any container)` | rule |
| MAT-X8 | `token.params.opacity.<key> (the md.sys.state opacities: hover, focus, pressed, dragged, disabled; R1.6 picks the latest set, ADD-A44)` | token |
| MAT-X9 | `PartStyle.transitions (Fill, TransitionKind::Ripple with its max_radius, initial_scale, padding, soft_edge, opacity, fade, min_press, touch_delay) @ {pressed} (on the state layer, S2.7)` | rule |
| MAT-X9 | `Transition.{duration,delay,timing} of the ripple (grow time, easing)` | field |
| MAT-X9 | `PartStyle.transitions (Fill, Interpolate) @ {hover} (the hover fade)` | rule |
| MAT-X10 | `token.focus: FocusIndicator.width, FocusIndicator.offset (per side, negative for the inward offset, ADD-A18), FocusIndicator.corners (Radius::FollowPart: the ring following the part's per-corner shape, concentric)` (the system tokens: thickness, inner and outer offset) | token |
| MAT-X10 | - (material-web's growing focus ring and its reduced-motion form, `md-focus-ring`'s own component tokens, `_md-comp-focus-ring.scss:20-52`; Compose, the reference, has its own focus indication, MAT-X34†) | reference (R1.6) |
| MAT-X10 | `button.body.focus, switch.track.focus, checkbox.box.focus (per-component colour, width and offset over = token.focus)` | field |
| MAT-X11 | `token.params.shape.<scale step> (none … extra_extra_large, ADD-A44)` | token |
| MAT-X11 | `PartStyle.corners per-corner Radius (the -top, -start, -end families) and Radius::Full (half the smaller side)` | field |
| MAT-X12 | `button.body.shape @ {pressed} / {selected} / {checked} cond flavor:expressive` (a `ShapeKind::Material` shape in place of the corners' outline; buttons and icon buttons; R25.5, ADD-B03, ADD-A99) | rule |
| MAT-X12 | `button.body.transitions (Shape, Interpolate, Timing::Spring) @ {pressed} / {selected} / {checked} cond flavor:expressive` (the morph between the shapes, `native_theme::shapes::morph`, S2.4) | rule |
| MAT-X12 | `ShapeKind::Material(MaterialShape)` (Material's shapes implemented once in code to `androidx.graphics.shapes` and `MaterialShapes.kt` at a pinned commit, `MaterialShape` listing exactly that file's shapes in its order, named in UpperCamelCase, the list written by the value phase, with `native_theme::shapes::outline` and `morph`; S2.4, R25.5, ADD-A99, ADD-A101) | field |
| MAT-X12 | `spinner.values.loop @ variant @loading_indicator (keyframes setting the indicator's shape)` | rule |
| MAT-X13 | `token.motion.durations.<duration key>` | token |
| MAT-X13 | `token.motion.timings.<easing or spring key> (cubic Béziers, Timing::Spring)` | token |
| MAT-X13 | `declare.flavors baseline, expressive` | field |
| MAT-X13 | `token.motion.timings @ cond flavor:expressive (the expressive motion scheme over the standard one)` | rule |
| MAT-X14 | `navigation_view.values.presentations events @push, @pop` (kinds `PresentationKind::Keyed` container_transform / shared_axis / fade_through, `PresentationKind::Fade`, ADD-A26) | field |
| MAT-X16 | `TextStyle.{letter_spacing,line_height}, TextStyle.font.{axes,features}` | field |
| MAT-X16 | `token.text.keyed.<key>.font.family (brand or plain typeface)` | token |
| MAT-X16 | `token.text.roles.*` | token |
| MAT-X17 | `input.accent (the seed, S9.1)` | input |
| MAT-X17 | `token.material.variant (the scheme variant the user chose)` | token |
| MAT-X17 | `role.@<m3 role> = m3_role(<role>)` (it reads `input.accent`, `token.material.{variant,spec_version,contrast_level}` and the colours' scheme itself, S2.2) | role |
| MAT-X17 | `reader:android:dynamic colour scheme roles and variant` | readers (R18) |
| MAT-X18 | `cond contrast_increased (medium)` | condition |
| MAT-X18 | `cond contrast_high (high)` | condition |
| MAT-X18 | `token.material.contrast_level @ cond contrast_increased / cond contrast_high (the operand m3_role reads, ADD-A74)` | rule |
| MAT-X18 | `reader:android:UiModeManager.getContrast` | readers (R18) |
| MAT-X19 | `token.text.scaling` (`TextScaling::AndroidNonLinear`: large sizes scale less, S9.4, ADD-A88); `text_scale` (S2.2 operand: lengths that follow text scaling, ADD-A55) | token |
| MAT-X19 | `cond bold_text` | condition |
| MAT-X19 | `token.text.keyed.<key>.font.weight @ cond bold_text (the weight adjustment)` | rule |
| MAT-X19 | `reader:android:Configuration.fontScale, FontScaleConverter, fontWeightAdjustment` | readers (R18) |
| MAT-X20 | `PartStyle.transitions, <widget>.values.presentations @ cond reduced_motion` | rule |
| MAT-X20 | `token.motion.speed_factor` | token |
| MAT-X20 | `reader:android:Settings.Global.ANIMATOR_DURATION_SCALE, TRANSITION_ANIMATION_SCALE` | readers (R18) |
| MAT-X21 | - (material-web's browser forced-colours blocks: every component listed has a Compose component, Material's reference; Material's high contrast is MAT-X18's scheme) | reference (R1.6) |
| MAT-X22 | `declare.widget.<w>.densities (the density scale)` | field |
| MAT-X22 | `declare.widget.<w>.sizes = extra_small, small, regular, large, extra_large (ADD-A62)` | field |
| MAT-X22 | `PartStyle.touch_target (the minimum interactive size, ADD-A72)` | field |
| MAT-X22 | `PartStyle.{padding,min_height} @ density <d>` | rule |
| MAT-X23 | `role.global.scrim` | role |
| MAT-X23 | `dialog.scrim.fill; sheet.scrim.{fill,opacity}; sidebar.scrim.{fill,opacity}; search_field.scrim.{fill,opacity}` (the search view's) | field |
| MAT-X24 | `PartStyle.{fill,stroke,text.fill,icon.fill} = alpha(<role>, token.params.opacity.<key>) @ {disabled} per part (container, label, icon, outline); PartStyle.opacity @ {disabled} where the part lists it` | rule |
| MAT-X24 | `PartStyle.fill.append @ {disabled} (the disabled state-layer opacity)` | rule |
| MAT-X25 | `tooltip.values.display_time` | field |
| MAT-X25 | `toast.values.durations` | field |
| MAT-X25 | `menu.values.{submenu_open_delay,submenu_close_delay}` | field |
| MAT-X25 | `token.behavior.long_press (ADD-A45)` | token |
| MAT-X25 | `token.behavior.touch.double_tap_time (ADD-A45)` | token |
| MAT-X25 | `scrollbar.values.auto_hide_delay; scrollbar.values.presentations (event Disappear: the fade)` | field |
| MAT-X25 | `token.caret.{blink_on,blink_off}` | token |
| MAT-X25 | `PartStyle.transitions (TransitionKind::Ripple's touch_delay, min_press)` | field |
| MAT-X25 | `reader:android:ViewConfiguration.getLongPressTimeout, getDoubleTapTimeout, getScrollDefaultDelay, getScrollBarFadeDuration` | readers (R18) |
| MAT-X27 | `IconStyle.axes (FILL, GRAD, opsz); IconStyle.weight (wght)` | field |
| MAT-X27 | `Glyph.axes (a Material Symbols glyph's fill, grade, optical size)` | field |
| MAT-X27 | `bottom_tab_bar.item_icon.icon.axes @ {selected}; list.leading_icon.icon.axes @ {selected} variant sidebar (FILL: filled when active; nav bar, rail, drawer)` | rule |
| MAT-X27 | `IconStyle.symbol_variants @ {selected} (where the icon set has a filled variant, ADD-A37)` | rule |
| MAT-X27 | `PartStyle.icon.fill per state` | rule |
| MAT-X29 | `card flags dragged; chip flags dragged; list flags dragged; splitter flags dragged (StateFlag::Dragged, S7.2)` | flag |
| MAT-X29 | `PartStyle.fill.append @ {dragged}` | rule |
| MAT-X29 | `PartStyle.shadow = Shadow::Elevation(<key>) @ {dragged}` | rule |
| MAT-X29 | `splitter.grip.{fill.append,shadow} @ {dragged} (the drag handle)` | rule |
| MAT-X31 | `cond input:mouse` | condition |
| MAT-X31 | `cond input:touch` | condition |
| MAT-X31 | `cond input:pen` | condition |
| MAT-X31 | `PartStyle.{min_width,min_height,padding} @ cond input:mouse / cond input:touch` | rule |
| MAT-X32† | `token.behavior.touch.{slop,paging_slop,double_tap_slop} (ADD-A45)` | token |
| MAT-X32† | `token.behavior.scroll_px_per_step` (Android states pixels per wheel step, ADD-A86) | token |
| MAT-X32† | `reader:android:ViewConfiguration.getScaledTouchSlop, getScaledPagingTouchSlop, getScaledDoubleTapSlop, getScaledVerticalScrollFactor` | readers (R18) |
| MAT-X33† | `PartStyle.cursor per part and state` | field |
| MAT-X33† | `token.pointer.{size,color} (ADD-A46)` | token |
| MAT-X33† | `reader:android:the pointer appearance settings (size, colour; per API level, keys to verify)` | readers (R18) |
| MAT-X34† | `token.behavior.focus_visible_when` (Compose's rule, the preset's reference where the tokens are silent, R1.6: when Compose shows its focus indication, read from its focus handling, a `FocusInteraction` under Android's touch mode, to verify in the value phase; material-web's `:focus-visible` only a cross-check; each widget's focus_visible_when refers to it, ADD-A16) | token |
| MAT-X34† | `PartStyle.fill.append @ {focus_visible} (Compose's default focus style: a state-layer opacity)` | rule |
| MAT-X34† | `FocusIndicator.{width,paint,offset,shadow} (Compose's inset ring: the outer stroke the indicator's width and offset, the inner stroke a shadow layer, R7, ADD-A17)` | field |
| MAT-X34† | `FocusIndicator.corners` (stated per component: the shape each component supplies, as expressions over `part.corners` and the offsets, cited) | field |
| MAT-X35† | - | out (R1.1) |
| MAT-X36† | `token.text.roles.{body,monospace}.font.family` | token |
| MAT-X36† | `reader:android:Typeface.DEFAULT, Typeface.MONOSPACE` | readers (R18) |
| MAT-X37† | `split_button.{primary_label,primary_icon}.transform.translate_x = min(mul(token.params.layout.optical_centering, sub(widget.primary.corners.<outer>, widget.primary.corners.<inner>)), <max>); split_button.arrow.transform.translate_x` likewise over `widget.secondary.corners` (ADD-A56, ADD-A80; a transform, which S8's layout restriction does not cover) | field |
| MAT-X38† | `button.values.scaffold_placement @ variant @fab` (`MaterialScaffoldSlot(Fab)`: end or centre, its spacing); `toast.values.placement` (`MaterialScaffoldSlot(Snackbar)`: above the FAB and the bottom bar) | placement |
| MAT-SF01† | `dialog.values.placement (CenteredInHost)` | placement |
| MAT-SF02† | `dialog.values.breakpoints (measured Window, setting full_size; its comparison and width to verify, as the row says)` | field |
| MAT-SF02† | `dialog.* @ variant full_size` | rule |
| MAT-SF03† | `sheet.values.anchor (BottomCenter), sheet.body.max_width @ variants {bottom,@modal}` | rule |
| MAT-SF04† | `layout.values.levitated_placement @ variant pane` (`MaterialLevitatedPane`: its alignment, `side` and `align`) | placement |
| MAT-SF04† | `layout.scrim.fill (= role.global.scrim) @ variant pane` (the optional `LevitatedPaneScrim`, a part) | rule |
| MAT-SF04† | `role.global.scrim` | role |
| MAT-RL01† | `layout.values.{item_gap,line_gap} @ variant flow (chips wrapping in rows, one gap both ways)` | rule |
| MAT-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| MAT-FM01† | `reader:android:AFontMatcher_match on an emoji string` | readers (R18) |
| MAT-FM02† | `token.text.fallbacks.{families,per_language} (ADD-A41)` | token |
| MAT-FM02† | `reader:android:AFontMatcher_match per locale` | readers (R18) |
| MAT-FM03† | `RootInputs.direction, Element.direction (the layout direction, S8)` | field |
| MAT-FM03† | `PartStyle.corners @ dir rtl (material-web's logical corners start-start … end-end stated as physical corners per direction)` | rule |
| MAT-FM03† | `slider.{track,fill,thumb}.* @ dir rtl (the active track's clip, the thumb's offset, the inside and outside corners); progress_bar.{indicator,stop_indicator}.* @ dir rtl; tab.indicator.* @ dir rtl; calendar.range_band.* @ dir rtl; text_area.* @ dir rtl (the outlined field's resize handle)` | rule |
| MAT-FM03† | `sidebar.values.presentations @ dir rtl (the drawer's swipe and predictive-back origin); search_field.values.presentations @ dir rtl (its predictive-back offset); menu.values.placement (PlacementParams.align, mirrored in RTL, S13)` | rule |
| MAT-FM03† | `reader:android:Configuration.getLayoutDirection` | readers (R18) |
| MAT-FM04† | `token.icons.mirroring (IconRole, Mirroring::ExceptLanguages: Arabic and Farsi, not Hebrew; S2.9)` | token |
| MAT-FM04† | `Element.language, Element.direction (icon lookup per language and script, S2.9, S8; ADD-B05)` | field |
| MAT-P3-1† | - | out (R1.1) |
| MAT-P3-2† | - | out (R1.1) |
| MAT-P3-3† | - | out (R1.1) |
| MAT-P3-4† | - | out (R1.1) |
| MAT-P3-5† | - | out (R1.1) |
| MAT-P3-6† | `window.launch_screen.fill` | field |
| MAT-P3-6† | `window.launch_icon.{icon,fill}` | field |
| MAT-P3-6† | `window.values.presentations event @launch_exit` | field |
| MAT-P3-7† | `input.body.fill.append @ {autofilled}` | rule |
| MAT-P3-8† | `checkbox.mark.glyph (GlyphSource::Path from Compose drawCheck)` | field |
| MAT-P3-8† | `checkbox.mark.glyph @ {indeterminate} (the same path pulled to the centre line)` | rule |
| MAT-P3-9† | `combo_box.values.popup_placement (MaterialMenu: width AnchorWidth::Anchor, Compose's matchAnchorWidth; the height limited by the window space left minus MenuVerticalMargin, its window_edge_margin; ADD-A21, ADD-A22)` | placement |
| MAT-P3-9† | - (material-web's select: its at-least-the-field width, `clamp-menu-width` and `menu-align`) | reference (R1.6) |
| MAT-P3-9† | `autocomplete.values.placement (MaterialMenu: the same anchor width and height limit for the autocomplete's list; ADD-A21, ADD-A22)` | placement |
| MAT-P3-10† | `menu.values.placement (MaterialMenu: side, align mirrored in RTL, flip, window_edge_margin)` | placement |
| MAT-P3-11† | `menu.body.{fill,min_width,max_width,shadow,corners,padding}, menu.item.{min_height,padding}, menu.item_icon.icon.size, menu.separator.{stroke,padding}, menu.item_label.text @ variant @text_context` | rule |
| MAT-P3-11† | `menu.item_label.text.fill, menu.item_icon.icon.fill @ {disabled} variant @text_context (the disabled alpha)` | rule |
| MAT-P3-11† | `role.@popup_menu_background, role.@popup_menu_text` (the platform popup-menu style, `material-3`'s keyed roles) | role |
| MAT-P3-12† | `segmented_control.values.segment_width (FillEqually)` | field |
| MAT-P3-12† | `segmented_control.segment.min_width` | field |
| MAT-P3-13† | `layout.values.{max_panes_horizontal,max_panes_vertical,pane_width,pane_height,avoids_hinge} @ h_size_class compact / h_size_class medium / h_size_class expanded / h_size_class large / h_size_class extra_large / v_size_class <c> / cond posture:tabletop` (the tall single-pane window; two stacked panes in tabletop posture, ADD-A83); `layout.values.vertical_pane_gap` (the vertical spacer) | rule |
| MAT-P3-14† | `token.behavior.{key_repeat_delay,key_repeat_interval}` | token |
| MAT-P3-14† | `reader:android:ViewConfiguration.getKeyRepeatTimeout, getKeyRepeatDelay` | readers (R18) |
| MAT-P3-15† | `token.behavior.{hide_pointer_while_typing,hide_pointer_fade}` (its gradual fade-out, ADD-A86) | token |
| MAT-P3-16† | `token.behavior.scroll.{deceleration,min_fling_velocity,max_fling_velocity} (the fling's spline decay, started and capped by the velocities)` | token |
| MAT-P3-16† | `reader:android:ViewConfiguration.getScrollFriction, getScaledMinimumFlingVelocity, getScaledMaximumFlingVelocity` | readers (R18) |
| MAT-P3-17† | - | out (R1.1) |
| MAT-P3-18† | `token.theme_change` | token |
| MAT-P3-18† | `reader:android:Configuration.uiMode` | readers (R18) |
| MAT-P3-19† | `cond flavor:baseline` | condition |
| MAT-P3-19† | `cond flavor:expressive` | condition |
| MAT-P3-19† | - (the Compose Material 3 library version an app ships) | newest (R1.5) |
| MAT-P3-19† | - (material-web's look where it differs from Compose, Material's reference) | reference (R1.6) |
| MAT-P3-20† | `rich_text.heading_1 … heading_6.text, rich_text.inline_code.text (<tt>), rich_text.list_marker.glyph, rich_text.values.list_indent_per_level, rich_text.{subscript,superscript,strikethrough}.text, rich_text.block_quote.{fill,stroke} @ variant html; rich_text.underline.text @ variant html` | rule |
| MAT-P3-20† | `link.* @ variant inline context rich_text.body variant html` | rule |
| MAT-P3-21† | `token.caret.width` | token |
| MAT-P3-21† | `input.caret (absent = true), text_area.caret (absent = true) @ {text_selected}` (none while the selection is not collapsed, ADD-A24) | rule |
| MAT-P3-21† | `input.caret @ cond window_inactive / !{focused}; text_area.caret @ cond window_inactive / !{focused}` (`absent = true`: not drawn) | rule |
| MAT-P3-21† | `reader:android:ViewConfiguration.getTextCursorBlinkIntervalMillis (token.caret.{blink_on,blink_off})` | readers (R18) |
| MAT-P4-1† | - | out (R1.1) |
| MAT-P4-2† | `window.values.keyboard_avoidance (KeyboardAvoid(AndroidPan) / KeyboardAvoid(AndroidResize) / KeyboardAvoid(AndroidInsetsAnimation), S13)` | placement |
| MAT-P4-2† | `reader:android:softInputMode, WindowInsetsAnimation` | readers (R18) |
| MAT-P4-3† | `token.pointer.drag_cursors.<drop operation> (ADD-A46)` | token |
| MAT-P4-4† | - | out (R1.1) |
| MAT-P4-5† | - | out (R1.1) |
| MAT-P4-6† | `tab.values.{overflow,scroll_to_selected,scroll_margin}` | field |
| MAT-P4-6† | `tab.label.text.elision (Elision::None)` | field |
| MAT-P4-6† | `tab.values.presentations (event @scroll: the animated scroll to the selected tab)` | field |
| MAT-P4-7† | `toolbar.values.presentations (events @expand, @collapse; steps on leading_content and trailing_content: ClipReveal; ADD-B09) @ variant floating / variants {floating,vertical}` | rule |
| MAT-P4-7† | `toolbar.bar.shadow @ variant floating / variants {floating,@collapsed}` | rule |
| MAT-P4-7† | `toolbar.bar.transitions (Shadow; Geometry: the main content's balancing padding) @ variant floating` | rule |
| MAT-P4-8† | `drag_image.values.presentations (events @drop_not_consumed: the shadow flies back fading; @cancel: it shrinks and fades out)` | field |
| MAT-P4-8† | `drag_image.values.presentations @ {@source_hidden} (both outcomes only fade in place)` | rule |
| MAT-P4-9† | `tooltip.values.placement (MaterialTooltip: side, flip, clamped into the window)` | placement |
