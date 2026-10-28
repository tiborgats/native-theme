# v0.7.1 — Model completeness: Specification, traceability (GNOME)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-gnome.md`](todo_v0.7.1_model-completeness-gap-map-gnome.md) mapped to
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
`{a,b}` states that hold together, `!{a}` states absent, `variant v` or `variants {v,w}` (`@v` keyed, `!variant v`
negated), `cond c` (a condition, S7.1; `flavour:kirigami`), `context w.part`, `context any w` or `context nearest {A, B}
= A` (an ancestor), `not_context w.part`, `pos first`, `pos last`, `pos only`, `pos parity odd`, `prev w{…}` and
`next w{…}` (siblings), `size s`, `density d`, `h_size_class c`, `v_size_class c`, `dir rtl`; ` / ` separates
alternatives, each a rule of its own. A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| GN-1.01 | `window.frame.shadow (three layers)` | field |
| GN-1.02 | `window.frame.shadow @ cond window_inactive` | rule |
| GN-1.02 | `window.frame.transitions (shadow group)` | field |
| GN-1.03 | `window.frame.stroke (inside, inset outline)` | field |
| GN-1.03 | `window.frame.stroke @ cond contrast_high` | rule |
| GN-1.04 | `window.frame.shadow @ cond contrast_high (outer ring layer)` | rule |
| GN-1.05 | `window.frame.{corners,stroke,shadow} @ cond tiled` | rule |
| GN-1.05 | `window.frame.{corners,shadow,transitions} @ cond maximized` | rule |
| GN-1.05 | `window.frame.{corners,shadow,transitions} @ cond full_screen` | rule |
| GN-1.06 | `window.frame.corners (per corner) @ variant @ssd_frame` | rule |
| GN-1.07 | `cond not_composited (ADD-A01)` | condition |
| GN-1.07 | `window.frame.{shadow,padding,corners} @ cond not_composited` | rule |
| GN-1.07 | `window.frame.shadow @ cond not_composited cond window_inactive` | rule |
| GN-1.08 | `window.frame.shadow @ variant @ssd` | rule |
| GN-1.08 | `window.frame.shadow @ variant @ssd cond contrast_high` | rule |
| GN-1.09 | `window.frame.shadow @ variant dialog` | rule |
| GN-1.10 | `window.title_bar.fill.append @ variant @devel (stripes: pattern paint, ADD-B01)` | rule |
| GN-1.11 | `window.title_bar.min_height` | field |
| GN-1.11 | `window.title_bar.min_height @ variant @standalone / variant @default_decoration / variant @collapse_spacing` | rule |
| GN-1.12 | `window.title_bar_content.padding` | field |
| GN-1.12 | `window.title_bar_content.padding @ variant @default_decoration` | rule |
| GN-1.12 | `window.values.title_bar_box_gap_px` | field |
| GN-1.12 | `window.values.title_margin_px` | field |
| GN-1.13 | `window.title_bar.shadow (inset layers: bottom shade line, side borders)` | field |
| GN-1.13 | `window.title_bar.shadow @ cond contrast_high` | rule |
| GN-1.13 | `role.header.border` | role |
| GN-1.14 | `window.title_bar.shadow @ !variant transparent_title_bar (drop layers)` | rule |
| GN-1.14 | `window.title_bar.shadow @ cond contrast_high` | rule |
| GN-1.15 | `role.header.{foreground,shade,border}` | role |
| GN-1.15 | `role.header.shade_darker (ADD-A48)` | role |
| GN-1.16 | `window.title_bar_content.opacity @ cond window_inactive` | rule |
| GN-1.16 | `window.title_bar.transitions (fill) and window.title_bar_content.transitions (opacity)` | field |
| GN-1.17 | `window.title_bar.{fill,shadow} @ variant transparent_title_bar` | rule |
| GN-1.17 | `window.title_bar.{fill,shadow} @ variant @inline cond window_inactive (no backdrop change)` | rule |
| GN-1.18 | `window.title.{text,padding}` | field |
| GN-1.18 | `window.subtitle.{text,padding,opacity}` | field |
| GN-1.19 | `window.title.text.font (= the UI font, bold)` | field |
| GN-1.19 | `reader:gnome:titlebar-uses-system-font (titlebar-font only when false)` | reader |
| GN-1.20 | `window.values.caption_button_layout` | field |
| GN-1.20 | `reader:gnome:org.gnome.desktop.wm.preferences button-layout` | reader |
| GN-1.21 | `window.caption_button.{min_width,padding}` | field |
| GN-1.21 | `window.values.caption_button_gap_px` | field |
| GN-1.21 | `window.caption_button.{min_width,min_height,padding} @ variant @default_decoration` | rule |
| GN-1.21 | `window.caption_button_glyph.padding @ variant @default_decoration` | rule |
| GN-1.21 | `window.title_icon.padding` | field |
| GN-1.22 | `window.caption_button_glyph.{fill,corners,padding}` | field |
| GN-1.22 | `window.caption_button_glyph.fill @ {hover}` | rule |
| GN-1.22 | `window.caption_button_glyph.fill @ {pressed}` | rule |
| GN-1.22 | `window.caption_button_glyph.transitions` | field |
| GN-1.22 | `window.caption_button_glyph.shadow @ cond contrast_high (inset ring)` | rule |
| GN-1.23 | `window.{frame,title_bar,caption_button} @ variants {@ssd_frame,@default_decoration} (mutter-x11-frames draws libadwaita's rules)` | rule |
| GN-1.278† | `button.{body,icon} @ variant back context window.title_bar` | rule |
| GN-1.278† | `menu.values.placement @ context button variant back (navigation menu, no arrow)` | placement |
| GN-1.282† | `window.values.resize_band (outside incl. shadow, corner)` | field |
| GN-1.282† | `window.values.resize_band @ cond maximized / cond full_screen / cond tiled (Absent)` | rule |
| GN-1.282† | `window.resize_zone.cursor per edge variant` | field |
| GN-1.24 | `button.body.fill @ {checked,hover}` | rule |
| GN-1.24 | `button.body.fill @ {checked,pressed}` | rule |
| GN-1.25 | `button.label.text.fill @ {drop_target}` | rule |
| GN-1.25 | `button.icon.icon.fill @ {drop_target}` | rule |
| GN-1.25 | `button.body.shadow @ {drop_target} (inset ring layer)` | rule |
| GN-1.26 | `button.body.fill @ {open} variant menu (and per variant: @ {open} variants {menu,flat})` | rule |
| GN-1.26 | `button flags open (S7.2)` | flag |
| GN-1.27 | `button.body.padding @ variant icon_only` | rule |
| GN-1.27 | `button.body.min_width @ variant icon_only` | rule |
| GN-1.27 | `button.body.padding @ variant with_icon` | rule |
| GN-1.27 | `button.values.icon_text_gap @ variant with_icon` | rule |
| GN-1.27 | `button.label.padding @ variant with_icon` | rule |
| GN-1.27 | `button.body.padding @ variants {menu,@arrow_only}` | rule |
| GN-1.28 | `button.body.corners @ variant circular` | rule |
| GN-1.28 | `button.body.min_width, button.body.min_height, button.body.padding @ variant circular` | rule |
| GN-1.28 | `button.body.corners, button.body.padding @ variant pill` | rule |
| GN-1.28 | `button.body.* @ variants {menu,circular} / variants {menu,pill} (menubutton)` | rule |
| GN-1.29 | `button.body.fill @ variant flat` | rule |
| GN-1.29 | `button.body.fill @ {hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant flat` | rule |
| GN-1.29 | `button.body.opacity @ {disabled} !{checked} variant flat` | rule |
| GN-1.29 | `button.body.shadow @ {hover} / {pressed} / {checked} variant flat cond contrast_high (inset ring layer)` | rule |
| GN-1.29 | `button.body.* flat look @ !variant @raised !variant primary !variant destructive !variant @opaque context nearest {toolbar, window, popover} = toolbar.bar / !variant @raised !variant primary !variant destructive !variant @opaque context nearest {toolbar, window, popover} = window.title_bar` | rule |
| GN-1.30 | `button.body.fill, button.label.text.fill @ variant primary` | rule |
| GN-1.30 | `button.body.fill @ {hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant primary (appended layers)` | rule |
| GN-1.30 | `button.body.focus @ variant primary (outer, offset)` | rule |
| GN-1.31 | `button.label.text.fill @ variant destructive (= role.global.status.destructive.standalone)` | rule |
| GN-1.31 | `button.body.fill @ variant destructive and @ {hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant destructive` | rule |
| GN-1.32 | `button.body.fill, button.label.text.fill @ variant @opaque` | rule |
| GN-1.32 | `button.body.fill @ {hover} / {pressed} / {checked} variant @opaque (appended layers)` | rule |
| GN-1.33 | `button.body.* the flat-in-container rule's !variant @raised (context nearest {toolbar, window, popover} = toolbar.bar / context nearest {toolbar, window, popover} = window.title_bar)` | rule |
| GN-1.33 | `button variant @raised (keyed, adwaita)` | field |
| GN-1.34 | `button.body.min_width, button.body.min_height, button.body.fill, button.label.text.fill @ variant @osd` | rule |
| GN-1.34 | `button.body.fill @ {hover} / {pressed} / {checked} variant @osd` | rule |
| GN-1.34 | `button.body.focus @ variant @osd (outer, offset)` | rule |
| GN-1.34 | `button.body.shadow @ variant @osd cond contrast_high (ring layer)` | rule |
| GN-1.35 | `button.body.fill @ variant @card and @ {hover} / {pressed} variant @card` | rule |
| GN-1.35 | `button.body.fill @ {checked} / {checked,hover} / {checked,pressed} variant @card` | rule |
| GN-1.35 | `button.body.focus.offset @ variant @card` | rule |
| GN-1.36 | `button.body.transitions (Fill, Shadow)` | field |
| GN-1.36 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.37 | `button.body.focus (kind, offset; base inset)` | field |
| GN-1.37 | `button.body.focus.offset @ variant @osd / variant primary / variant @opaque / variant @card` | rule |
| GN-1.38 | `button.body.shadow @ cond contrast_high (inset ring layer at rest)` | rule |
| GN-1.39 | `button.values.icon_text_gap` | field |
| GN-1.39 | `button.label.padding; button.label.text.font.weight` | field |
| GN-1.39 | `button.label.padding @ variant menu` | rule |
| GN-1.40 | `button.menu_indicator.glyph (ThemeIcon), button.menu_indicator.glyph.size` | field |
| GN-1.40 | `button.menu_indicator.glyph @ variant @arrow_up / variant @arrow_start / variant @arrow_end / variant @arrow_none` | rule |
| GN-1.41 | `button.attention_dot (part: fill radial gradient, min_width, min_height)` | field |
| GN-1.41 | `button flags @needs_attention (adwaita)` | flag |
| GN-1.41 | `button.attention_dot.opacity, button.attention_dot.transform, button.attention_dot.transitions @ {@needs_attention}` | rule |
| GN-1.41 | `button.values.attention_dot_offset` | field |
| GN-1.42 | `button.body.corners @ pos !first context any layout variant @linked / pos !last context any layout variant @linked (and under dir rtl mirrored, and under context any layout variants {@linked,vertical}); pos among siblings of any kind (CONTRA-3)` | rule |
| GN-1.42 | `layout.values.item_gap @ variant @linked (negative overlap; horizontal and vertical)` | rule |
| GN-1.43 | `input.values.icon_text_gap` | field |
| GN-1.44 | `input.icon.opacity (base) and @ {hover} / {pressed}` | rule |
| GN-1.44 | `input.icon.opacity @ cond contrast_high (rest, {hover}, {pressed})` | rule |
| GN-1.45 | `input.body.bindings (role accent bound to the error role) @ variant error (GNOME's .error)` | rule |
| GN-1.45 | `input.body.bindings (role accent bound to the warning role) @ variant warning` | rule |
| GN-1.45 | `input.body.bindings (role accent bound to the success role) @ variant success` | rule |
| GN-1.46 | `input.body.shadow (inset ring layer) @ {drop_target}` | rule |
| GN-1.47 | `input.body.{fill,corners,padding,min_height} @ variant flat` | rule |
| GN-1.48 | `input.progress.{fill,min_height,padding}` | field |
| GN-1.49 | `input.block_caret.{fill,text.fill}` | field |
| GN-1.49 | `token.caret.shape` | token |
| GN-1.50 | `input.values.focus_visible_when` | field |
| GN-1.50 | `input.body.focus (incl. transition)` | field |
| GN-1.51 | `input.body.shadow (inset ring layer) @ cond contrast_high` | rule |
| GN-1.52 | `text_area.gutter.fill` | field |
| GN-1.53 | `text_area.caret.fill @ {drop_target}` | rule |
| GN-1.54 | `text_area.{body.fill,text.text.fill} @ variant @inline` | rule |
| GN-1.281† | `input.preedit.text.decoration; text_area.preedit.text.decoration` | field |
| GN-1.281† | `input.preedit @ variant @whole / variant @selection / variant @prediction / variant @prefix / variant @suffix / variant @spelling_error / variant @compose_error (and on text_area)` | rule |
| GN-1.55 | `checkbox.box.fill @ {pressed} !{checked,indeterminate}` | rule |
| GN-1.55 | `checkbox.box.stroke @ {pressed} !{checked,indeterminate} (no ring)` | rule |
| GN-1.55 | `radio.ring.fill, radio.ring.stroke @ {pressed} !{checked}` | rule |
| GN-1.56 | `checkbox.box.stroke @ {hover} !{checked,indeterminate}` | rule |
| GN-1.56 | `radio.ring.stroke @ {hover} !{checked}` | rule |
| GN-1.57 | `checkbox.box.fill @ {checked,hover} / {checked,pressed} (appended layers; also {indeterminate,hover} / {indeterminate,pressed})` | rule |
| GN-1.57 | `radio.ring.fill @ {checked,hover} / {checked,pressed}` | rule |
| GN-1.58 | `checkbox.box.fill @ {indeterminate}` | rule |
| GN-1.58 | `checkbox.mark.glyph @ {indeterminate} (GlyphSource::Path from dash.svg)` | rule |
| GN-1.60 | `checkbox.box.corners` | field |
| GN-1.60 | `radio.ring.corners (Full)` | field |
| GN-1.61 | `checkbox.body.padding, checkbox.body.corners, checkbox.body.focus` | field |
| GN-1.61 | `checkbox.body.padding @ variant @text_button` | rule |
| GN-1.61 | `checkbox.values.label_gap` | field |
| GN-1.61 | `radio.body.padding, radio.body.corners, radio.body.focus, radio.values.label_gap` | field |
| GN-1.62 | `checkbox.box.corners, checkbox.box.padding @ variant selection` | rule |
| GN-1.62 | `checkbox.values.label_gap @ variant selection` | rule |
| GN-1.63 | `radio.body.corners (GTK's grouped checkbutton is the radio)` | field |
| GN-1.64 | `checkbox.box.transitions (Fill, Stroke); radio.ring.transitions` | field |
| GN-1.64 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.65 | `checkbox.box.stroke @ cond contrast_high; @ {hover} !{checked,indeterminate} cond contrast_high` | rule |
| GN-1.65 | `checkbox.box.fill @ {pressed} !{checked,indeterminate} cond contrast_high` | rule |
| GN-1.65 | `radio.ring.stroke, radio.ring.fill @ the same selectors` | rule |
| GN-1.66 | `menu.item.fill @ {pressed}` | rule |
| GN-1.67 | `menu.item.fill @ {selected}` | rule |
| GN-1.68 | `menu.item.corners` | field |
| GN-1.69 | `menu.body.padding` | field |
| GN-1.70 | `menu.body.min_width` | field |
| GN-1.70 | `menu.item.min_width` | field |
| GN-1.71 | `menu.separator.padding` | field |
| GN-1.72 | `menu.accelerator.{opacity,padding}` | field |
| GN-1.72 | `menu.values.accelerator_gap_px` | field |
| GN-1.73 | `menu.check_indicator.{min_width,min_height,padding,glyph}` | field |
| GN-1.73 | `menu.check_indicator.stroke @ variant radio` | rule |
| GN-1.73 | `menu.check_indicator.stroke @ variant radio cond contrast_high` | rule |
| GN-1.74 | `menu.submenu_arrow.{glyph,opacity}` | field |
| GN-1.74 | `menu.submenu_arrow.opacity @ cond contrast_high` | rule |
| GN-1.75 | `menu.submenu_header.{glyph,text,padding}` | field |
| GN-1.76 | `menu.values.submenu_presentation (Slide)` | field |
| GN-1.76 | `menu.values.presentations event @submenu_push (GtkStack's slide)` | field |
| GN-1.77 | `menu.values.submenu_open_delay_ms` | field |
| GN-1.77 | `menu.values.submenu_switch (TimerRestartedOnMotion)` | field |
| GN-1.78 | `menu.item.{min_width,min_height,fill} @ variant icon_only` | rule |
| GN-1.78 | `menu.item.{padding,stroke,fill} @ variants {icon_only,circular} / variants {icon_only,circular} {selected} / variants {icon_only,circular} {pressed}` | rule |
| GN-1.78 | `menu.values.icon_row_gap_px` | field |
| GN-1.79 | `list.row.{padding,corners,fill} @ context menu.body (hover, selected, pressed)` | rule |
| GN-1.80 | `menu.item.shadow @ {hover} / {selected} / {pressed} cond contrast_high (inset ring)` | rule |
| GN-1.285† | `menu.values.show_item_icons (OnlyWithoutLabel)` | field |
| GN-1.81 | `tooltip.values.icon_text_gap_px` | field |
| GN-1.82 | `tooltip.values.reshow_delay_ms` | field |
| GN-1.82 | `tooltip.values.browse_timeout_ms` | field |
| GN-1.83 | `tooltip.values.placement (GtkTooltip; anchor_grow)` | placement |
| GN-1.283† | `tooltip.body.max_width (= mul(element.char_width, <n>), the data declared on body; ADD-B17)` | field |
| GN-1.84 | `scrollbar.thumb.corners; scrollbar.track.corners` | field |
| GN-1.85 | `scrollbar.thumb.{min_width,min_height} @ cond overlay_scrollbars context any scrollbar !{hover}` | rule |
| GN-1.85 | `scrollbar.body.padding @ cond overlay_scrollbars !{hover}` | rule |
| GN-1.85 | `cond overlay_scrollbars` | condition |
| GN-1.86 | `scrollbar.thumb.stroke (outside) @ cond overlay_scrollbars` | rule |
| GN-1.86 | `scrollbar.thumb.stroke @ cond overlay_scrollbars context any scrollbar !{hover}` | rule |
| GN-1.86 | `scrollbar.thumb.stroke @ cond overlay_scrollbars cond contrast_high / cond overlay_scrollbars cond contrast_high context any scrollbar !{hover}` | rule |
| GN-1.87 | `scrollbar.track.fill @ {hover} cond overlay_scrollbars` | rule |
| GN-1.87 | `scrollbar.track.fill @ {hover} cond overlay_scrollbars cond contrast_high` | rule |
| GN-1.88 | `scrollbar.thumb.fill @ context any scrollbar {@fine_tune}` | rule |
| GN-1.88 | `scrollbar.thumb.fill @ cond contrast_high context any scrollbar {@fine_tune}` | rule |
| GN-1.88 | `scrollbar flags @fine_tune; slider flags @fine_tune (adwaita)` | flag |
| GN-1.89 | `scrollbar.thumb.opacity @ {disabled}` | rule |
| GN-1.90 | `scrollbar.track.transitions; scrollbar.thumb.transitions` | field |
| GN-1.91 | `scrollbar.values.auto_hide_delay` | field |
| GN-1.91 | `scrollbar.values.presentations (Disappear: fade)` | field |
| GN-1.91 | `scrollbar.values.hide_after_leave` | field |
| GN-1.92 | `scrollbar.thumb.{min_width,min_height} @ context any menu` | rule |
| GN-1.93 | `scrollbar.thumb.fill @ cond contrast_high (rest, {hover}, {pressed})` | rule |
| GN-1.94 | `slider.track.corners; slider.fill.corners` | field |
| GN-1.95 | `slider.body.padding` | field |
| GN-1.96 | `slider.thumb.shadow (spread ring layer)` | field |
| GN-1.96 | `slider.thumb.shadow @ cond contrast_high` | rule |
| GN-1.97 | `slider.thumb.shadow` | field |
| GN-1.97 | `slider.thumb.shadow @ {disabled}` | rule |
| GN-1.97 | `slider.thumb.shadow @ variant ticks_before !variant ticks_after / variant ticks_after !variant ticks_before` | rule |
| GN-1.98 | `slider.track.fill @ {hover} / {pressed}` | rule |
| GN-1.98 | `slider.fill.fill.append @ {hover} / {pressed}` | rule |
| GN-1.98 | `slider.thumb.fill @ context any slider {hover}` | rule |
| GN-1.99 | `slider.thumb.fill @ context any slider {pressed}` | rule |
| GN-1.100 | `slider.tick.{min_width,min_height,fill}` | field |
| GN-1.100 | `slider.values.tick_gap` | field |
| GN-1.100 | `slider.tick.min_height @ {@fine_tune}` | rule |
| GN-1.101 | `slider.thumb.{corners,transform} @ variant ticks_before !variant ticks_after / variant ticks_after !variant ticks_before` | rule |
| GN-1.102 | `slider.value_label.{text,opacity}` | field |
| GN-1.102 | `slider.values.value_label_gap` | field |
| GN-1.102 | `slider.values.value_label_gap @ variant ticks_before / variant ticks_after` | rule |
| GN-1.103 | `slider.fill_level.{fill,corners}` | field |
| GN-1.104 | `slider.thumb.focus` | field |
| GN-1.105 | `slider.thumb.focus.paint @ {disabled}` | rule |
| GN-1.105 | `slider.thumb.focus.paint @ {disabled} cond contrast_high` | rule |
| GN-1.106 | `slider.track.transitions; slider.fill.transitions` | field |
| GN-1.107 | `slider.{tick,track,fill} geometry @ variant vertical` | rule |
| GN-1.107 | `slider.values.value_label_position @ variant vertical` | rule |
| GN-1.108 | `slider.track.{fill,shadow} @ cond contrast_high` | rule |
| GN-1.108 | `slider.track.fill @ {hover} cond contrast_high` | rule |
| GN-1.109 | `progress_bar.track.corners; progress_bar.indicator.corners` | field |
| GN-1.110 | `progress_bar.value_label.{text,opacity}` | field |
| GN-1.111 | `progress_bar.track.{min_width,min_height} @ variant vertical` | rule |
| GN-1.112 | `progress_bar.track.{fill,min_height} @ variant @osd` | rule |
| GN-1.112 | `progress_bar.indicator.{corners,min_height} @ variant @osd` | rule |
| GN-1.113 | `progress_bar.values.pulse_block_fraction; progress_bar.values.pulse_step` | field |
| GN-1.113 | `progress_bar.values.loop` | field |
| GN-1.114 | `progress_bar.indicator.{opacity,min_width} @ {empty}` | rule |
| GN-1.114 | `progress_bar flags empty` | flag |
| GN-1.115 | `progress_bar.track.{fill,shadow} @ cond contrast_high` | rule |
| GN-1.116 | `tab.bar.stroke (the page-side edge) @ variant top / variant bottom / variant left / variant right` | rule |
| GN-1.117 | `tab.indicator.fill @ {hover} (context tab.tab)` | rule |
| GN-1.118 | `tab.values.indicator_edge @ variant left / variant right` | rule |
| GN-1.119 | `tab.scroll_button.{fill,glyph,padding,min_width,min_height}` | field |
| GN-1.119 | `tab.values.overflow` | field |
| GN-1.120 | `tab.close_button.{min_width,min_height,glyph,padding}` | field |
| GN-1.120 | `tab.close_button.opacity @ {hover}` | rule |
| GN-1.121 | `tab.bar.padding, tab.tab.padding` | field |
| GN-1.121 | `tab.values.tab_gap` | field |
| GN-1.122 | `tab.pane.fill (= the View surface)` | field |
| GN-1.123 | `tab.tab.opacity @ {disabled} (label and close button restated)` | rule |
| GN-1.124 | `tab.tab.fill.append @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} variant document` | rule |
| GN-1.124 | `tab.tab.min_height, tab.tab.padding, tab.tab.corners @ variant document` | rule |
| GN-1.124 | `tab.tab.fill @ pos only variant document` | rule |
| GN-1.125 | `tab.bar.fill, tab.bar.stroke.bottom, tab.bar.padding, tab.bar.min_height, tab.bar.bindings (surface Header) @ variant document` | rule |
| GN-1.125 | `tab.bar.fill, tab.tab.opacity @ cond window_inactive variant document` | rule |
| GN-1.125 | `tab.bar.* @ variant @inline / variant @shrunk` | rule |
| GN-1.126 | `tab.separator.stroke @ variant document` | rule |
| GN-1.126 | `tab.separator.opacity @ prev tab.tab{selected} / next tab.tab{selected} (ADD-B07)` | rule |
| GN-1.126 | `tab.separator.transitions (opacity)` | field |
| GN-1.127 | `tab.drop_indicator.{stroke,min_width,min_height,padding}` | field |
| GN-1.127 | `tab.drop_indicator.stroke @ cond contrast_high` | rule |
| GN-1.128 | `tab.attention_indicator.{fill,transform,min_width,max_width,min_height} @ {@needs_attention}` | rule |
| GN-1.128 | `tab.values.attention_width_fraction` | field |
| GN-1.128 | `tab.values.presentations event @attention` | field |
| GN-1.128 | `tab.attention_indicator.min_width @ variant @pinned` | rule |
| GN-1.129 | `tab.values.width_mode (base width), tab.tab.max_width` | field |
| GN-1.129 | `tab.tab.min_width @ variant @pinned` | rule |
| GN-1.129 | `tab.label.text.elision (Fade under close_button)` | field |
| GN-1.130 | `tab.close_button.{min_width,min_height,corners} @ variant document` | rule |
| GN-1.130 | `tab.close_button.opacity @ context tab.tab {hover} / {selected}` | rule |
| GN-1.130 | `tab.close_button.transitions (opacity)` | field |
| GN-1.131 | `tab.values.presentations events Appear (open), Disappear (close), Move (reorder), @scroll, @resize` | field |
| GN-1.131 | `tab.tab.transitions (fill)` | field |
| GN-1.132 | `tab.values.drag_hover_delay_ms` | field |
| GN-1.133 | `tab.tab.fill, tab.tab.shadow, tab.tab.padding @ {dragged}` | rule |
| GN-1.133 | `tab.tab.stroke @ {dragged} cond contrast_high` | rule |
| GN-1.134 | `tab.tab.focus @ {focus_within, focus_visible} variant document` | rule |
| GN-1.134 | `tab.tab.focus (offset) @ {selected, focus_visible}` | rule |
| GN-1.135 | `tab.tab.stroke @ {hover} / {pressed} / {selected} cond contrast_high variant document` | rule |
| GN-1.136 | `role.sidebar.background @ cond window_inactive` | role |
| GN-1.136 | `sidebar.pane.transitions (fill)` | field |
| GN-1.137 | `role.secondary_sidebar.background, foreground, border, shade (ADD-A48)` | role |
| GN-1.137 | `role.secondary_sidebar.background @ cond window_inactive` | role |
| GN-1.137 | `sidebar.pane.bindings (surface SecondarySidebar) @ variant @secondary` | rule |
| GN-1.138 | `role.sidebar.shade` | role |
| GN-1.139 | `sidebar.pane.stroke (the content-side edge) @ variant end / dir rtl` | rule |
| GN-1.140 | `list.row_background.corners, list.row.min_height, list.row.padding @ variant sidebar` | rule |
| GN-1.140 | `list.values.row_background_inset, list.values.leading_gap @ variant sidebar` | rule |
| GN-1.141 | `list.row_background.fill.append @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {context_menu_open} variant sidebar` | rule |
| GN-1.142 | `list.section_header.padding @ variant sidebar / variant sidebar pos first` | rule |
| GN-1.142 | `separator.values.inset @ context sidebar.pane` | rule |
| GN-1.143 | `list.row_background.corners, list.row.padding, list.values.row_background_inset @ variants {sidebar,grid}` | rule |
| GN-1.144 | `list.leading_icon.padding, list.trailing_icon.padding @ variant sidebar` | rule |
| GN-1.145 | `sidebar.values.drag_hover_delay_ms` | field |
| GN-1.146 | `banner.* @ context sidebar.pane` | rule |
| GN-1.147 | `sidebar.values.width, min_width, max_width, width_fraction (the lengths in libadwaita's sp: each = mul(<length>, text_scale), ADD-A55)` | field |
| GN-1.148 | `list.row_background.stroke @ {selected} / {hover} / {pressed} cond contrast_high variant sidebar` | rule |
| GN-1.149 | `toolbar.bar.padding, toolbar.bar.corners, toolbar.bar.bindings (surface Osd) @ variant @osd` | rule |
| GN-1.150 | `toolbar.bar.padding @ variant @collapse_spacing` | rule |
| GN-1.151 | `button.*, split_button.*, combo_box.* @ !variant @raised !variant primary !variant destructive context nearest {toolbar, popover} = toolbar.bar` | rule |
| GN-1.152 | `switch.* (padding) @ context toolbar.bar` | rule |
| GN-1.153 | `toolbar.bar.fill, toolbar.bar.shadow @ variant @raised` | rule |
| GN-1.153 | `toolbar.bar.shadow, toolbar.bar.stroke @ variant @raised cond contrast_high` | rule |
| GN-1.154 | `toolbar.bar.stroke (one edge), toolbar.bar.shadow (Absent) @ variants {@border,top} / variants {@border,bottom}` | rule |
| GN-1.155 | `toolbar.bar.fill @ cond window_inactive variant @raised` | rule |
| GN-1.155 | `button.body.opacity @ cond window_inactive context toolbar.bar variant @raised (the bar's content)` | rule |
| GN-1.155 | `toolbar.bar.transitions` | field |
| GN-1.155 | `button.body.transitions @ context toolbar.bar variant @raised (opacity)` | rule |
| GN-1.156 | `scroll_view.edge_effect.{fill,stroke} @ {scrolled} (the content under an AdwToolbarView bar: the undershoot)` | rule |
| GN-1.157 | `toolbar.bar.fill, toolbar.bar.stroke.top, toolbar.bar.padding @ variant bottom (GtkActionBar, AdwToolbarView's bottom bar)` | rule |
| GN-1.157 | `toolbar.values.item_gap @ variant bottom` | rule |
| GN-1.157 | `toolbar.bar.fill @ cond window_inactive variant bottom` | rule |
| GN-1.158 | `list.row_background.fill.append @ {pressed}` | rule |
| GN-1.159 | `list.row_background.fill @ {selected,hover}, {selected,pressed}` | rule |
| GN-1.160 | `list.row_background.fill @ {context_menu_open} / {selected,context_menu_open}` | rule |
| GN-1.161 | `list.row_background.focus (offset inset)` | field |
| GN-1.162 | `list.row_separator.stroke @ variant row_separators` | rule |
| GN-1.162 | `list.column_separator.stroke @ variants {row_separators,grid} (horizontal lists)` | rule |
| GN-1.163 | `list.section_header.text, list.section_header.padding` | field |
| GN-1.163 | `list.section_header.padding @ variant @rich_list` | rule |
| GN-1.164 | `list.row.padding, list.row.min_height, list.values.leading_gap @ variant @rich_list` | rule |
| GN-1.165 | `list.view.{fill,corners,shadow,stroke} @ variant inset` | rule |
| GN-1.165 | `list.row_separator.stroke @ variant inset` | rule |
| GN-1.165 | `list.row_background.fill.append @ {hover} / {pressed} variant inset` | rule |
| GN-1.165 | `list.row_background.corners @ pos first variant inset / pos last variant inset / pos only variant inset` | rule |
| GN-1.165 | `list.row_background.focus @ variant inset` | rule |
| GN-1.165 | `list.row_background.transitions @ variant inset` | rule |
| GN-1.166 | `list.row_background.{fill,corners,shadow} @ variant separated` | rule |
| GN-1.166 | `list.row_background.fill.append @ {hover} / {pressed} / {selected} variant separated` | rule |
| GN-1.166 | `list.row_background.shadow @ {drop_target} variant separated` | rule |
| GN-1.166 | `list.values.row_gap @ variant separated` | rule |
| GN-1.167 | `list.header_section.text.fill @ {hover} / {pressed} / cond contrast_high` | rule |
| GN-1.167 | `list.header_section.text, list.header_section.padding` | field |
| GN-1.168 | `list.sort_indicator.glyph @ {sorted} / {sorted,descending}` | rule |
| GN-1.169 | `list.column_separator.stroke, list.header_separator.stroke @ variant column_separators` | rule |
| GN-1.170 | `list.cell.padding` | field |
| GN-1.170 | `list.cell.padding @ variant @data_table` | rule |
| GN-1.171 | `list.header_section.fill, list.header_section.text.fill @ {dragged}` | rule |
| GN-1.172 | `list.cell.stroke @ {focus_visible} / {editing}` | rule |
| GN-1.173 | `list.disclosure.glyph @ {open} / !{open}` | rule |
| GN-1.173 | `list.values.disclosure_gap, list.values.indent_per_level` | field |
| GN-1.174 | `list.row.padding, list.row_background.corners @ variant grid` | rule |
| GN-1.174 | `list.row_background.fill.append @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} variant grid` | rule |
| GN-1.174 | `list.row_background.focus @ variant grid` | rule |
| GN-1.175 | `rubber_band.body.{fill,stroke} @ context list` | rule |
| GN-1.176 | `list.view.fill, list.row.text.fill @ {disabled}` | rule |
| GN-1.177 | `list.row_background.corners @ {selected} variant grid` | rule |
| GN-1.178 | `scroll_view.edge_effect.{fill,stroke} @ context list variant table (under the column view's header)` | rule |
| GN-1.279† | `list.branch_line.stroke (dash, ADD-A32), list.row_separator.stroke, list.column_separator.stroke (dash, ADD-A32) @ variant @tree_view` | rule |
| GN-1.279† | `list.values.indent_per_level, list.values.indent_includes_disclosure @ variant @tree_view` | rule |
| GN-1.279† | `list.disclosure.glyph, list.disclosure.opacity @ {hover} / {pressed} variant @tree_view` | rule |
| GN-1.279† | `list.row_background.corners @ {selected} variant @tree_view` | rule |
| GN-1.279† | `list.drop_indicator.stroke; list.row_background.stroke @ {drop_target} variant @tree_view` | rule |
| GN-1.279† | `list.header_section.padding, list.header.stroke.bottom @ variant @tree_view` | rule |
| GN-1.279† | `progress_bar.*, input.* @ context list.cell variant @tree_view` | rule |
| GN-1.279† | `list.values.typeahead_hide_ms, list.values.drag_hover_delay_ms` | field |
| GN-1.280† | `list.view.fill, list.row_background.focus @ variant @icon_view` | rule |
| GN-1.280† | `list.row_background.stroke @ {selected} cond contrast_high variant @icon_view` | rule |
| GN-1.280† | `list.row_background.stroke @ {drop_target} variant @icon_view` | rule |
| GN-1.280† | `list.row.padding, list.values.row_gap, list.values.secondary_text_gap @ variant @icon_view` | rule |
| GN-1.280† | `rubber_band.body @ context list variant @icon_view` | rule |
| GN-1.179 | `popover.body.shadow (three layers)` | field |
| GN-1.180 | `popover.arrow.{min_width,min_height,fill,stroke}` | field |
| GN-1.180 | `popover.arrow.stroke @ cond contrast_high` | rule |
| GN-1.180 | `popover.values.has_arrow` | field |
| GN-1.181 | `popover.values.placement (GtkPopover gap_px)` | placement |
| GN-1.182 | `role.popover.shade` | role |
| GN-1.183 | `cond not_composited (ADD-A01)` | condition |
| GN-1.183 | `popover.body.{corners,stroke,shadow} @ cond not_composited` | rule |
| GN-1.184 | `popover.body.{fill,shadow,bindings} @ variant @osd / variant @magnifier` | rule |
| GN-1.184 | `edit_menu.body (adwaita's touch-selection popover)` | field |
| GN-1.185 | `popover.body.{min_width,max_width} (Auto)` | field |
| GN-1.286† | `edit_menu.{body,item} (adwaita's touch-selection bubble)` | field |
| GN-1.286† | `edit_menu.values.placement (GtkPopover, anchor_grow)` | placement |
| GN-1.186 | `splitter.divider.stroke, splitter.divider.min_width @ variant wide` | rule |
| GN-1.187 | `separator.values.inset @ context menu / context sidebar.pane` | rule |
| GN-1.188 | `separator.line.{stroke,min_width,min_height} @ variant @spacer (stroke Absent)` | rule |
| GN-1.189 | `token.text.keyed.title_3` | token |
| GN-1.189 | `token.text.keyed.title_4` | token |
| GN-1.189 | `token.text.keyed.caption_heading` | token |
| GN-1.189 | `token.text.roles.title2` | token |
| GN-1.189 | `token.text.roles.title3` | token |
| GN-1.189 | `token.text.roles.caption_emphasized` | token |
| GN-1.190 | `token.text.keyed.body.line_height` | token |
| GN-1.190 | `token.text.keyed.caption.line_height` | token |
| GN-1.190 | `token.text.keyed.document.line_height` | token |
| GN-1.191 | `token.text.keyed.document` | token |
| GN-1.191 | `reader:gnome:document-font-name` | reader |
| GN-1.192 | `token.text.keyed.numeric.font.features` | token |
| GN-1.192 | `token.text.roles.numeric (= token.text.keyed.numeric)` | token |
| GN-1.193 | `reader:gnome:font-name Pango @axis= variations into Font.axes and #feature= into Font.features` | reader |
| GN-1.193 | `Font.axes (S2.8)` | field |
| GN-1.193 | `Font.features (S2.8)` | field |
| GN-1.194 | `token.text.keyed.smaller` | token |
| GN-1.194 | `token.text.keyed.narrow_switcher_label` | token |
| GN-1.194 | `token.text.keyed.badge_label` | token |
| GN-1.194 | `token.text.keyed.tab_counter` | token |
| GN-1.195 | `layout.values.max_content_width, layout.values.tightening_threshold @ variant clamp` | rule |
| GN-1.196 | `layout.values.container_margin, layout.values.section_gap @ variant @preferences_page` | rule |
| GN-1.197 | `window.values.mobile_height_breakpoint` | field |
| GN-1.197 | `view_switcher.values.breakpoints` | field |
| GN-1.198 | `switch.track.fill @ {pressed} !{checked}` | rule |
| GN-1.199 | `switch.track.fill @ {checked,pressed} (appended layer)` | rule |
| GN-1.200 | `switch.thumb.fill @ {hover} / {pressed}` | rule |
| GN-1.201 | `switch.thumb.shadow` | field |
| GN-1.201 | `switch.thumb.shadow @ {disabled}` | rule |
| GN-1.202 | `switch.on_glyph, switch.off_glyph (parts; glyph ThemeIcon switch-on-symbolic / switch-off-symbolic)` | field |
| GN-1.202 | `switch.on_glyph.glyph, switch.off_glyph.glyph @ cond differentiate_without_colour (base Absent)` | rule |
| GN-1.202 | `switch.on_glyph.glyph.fill, switch.off_glyph.glyph.fill @ cond differentiate_without_colour cond contrast_high` | rule |
| GN-1.202 | `reader:gnome:org.gnome.desktop.a11y.interface show-status-shapes` | reader |
| GN-1.202 | `cond differentiate_without_colour (GNOME: from show-status-shapes)` | condition |
| GN-1.203 | `switch.values.presentations (event Move, part thumb)` | field |
| GN-1.204 | `switch.track.transitions (Fill)` | field |
| GN-1.204 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.205 | `switch.track.fill @ {checked} context any variant @osd (any widget, R25.9)` | rule |
| GN-1.206 | `switch.track.focus (outer ring, offset)` | field |
| GN-1.207 | `switch.track.fill @ cond contrast_high; @ {hover} cond contrast_high; @ {pressed} cond contrast_high` | rule |
| GN-1.208 | `dialog.body.shadow` | field |
| GN-1.208 | `dialog.body.shadow @ cond contrast_high` | rule |
| GN-1.209 | `dialog.body.stroke (inner outline)` | field |
| GN-1.209 | `dialog.body.stroke @ cond contrast_high` | rule |
| GN-1.209 | `dialog.body.stroke @ variant alert cond contrast_high (dropped)` | rule |
| GN-1.210 | `dialog.scrim.fill (= the shade colour with its alpha scaled by a factor: alpha(role.window.shade, <fraction>))` | field |
| GN-1.211 | `dialog.body.corners @ variant alert` | rule |
| GN-1.212 | `dialog.action.{min_height,min_width,padding,corners} @ variant alert` | rule |
| GN-1.212 | `dialog.action.{min_width,padding} @ variants {alert,@short}` | rule |
| GN-1.213 | `dialog.action.{fill,text} @ variant primary / variant destructive (on action)` | rule |
| GN-1.214 | `dialog.values.title_message_gap_px` | field |
| GN-1.214 | `dialog.message.padding` | field |
| GN-1.214 | `dialog.body.padding @ variant alert` | rule |
| GN-1.215 | `dialog.body.padding @ variants {alert,@short}` | rule |
| GN-1.215 | `dialog.values.{title_message_gap_px,message_actions_gap_px} @ variant @short` | rule |
| GN-1.215 | `dialog.action_area.padding @ variant @short` | rule |
| GN-1.216 | `dialog.values.action_layout.stack_rule (WhenOverflowing)` | field |
| GN-1.216 | `dialog.values.action_layout.stack_rule @ variant @short` | rule |
| GN-1.217 | `dialog.body.max_width @ variants {alert,@wide}` | rule |
| GN-1.218 | `dialog.values.window_margin (growing with the window)` | field |
| GN-1.219 | `dialog.values.natural_size` | field |
| GN-1.220 | `dialog.values.presentations event Appear (spring; fade with scale, ADD-B10)` | field |
| GN-1.220 | `dialog.values.presentations @ cond reduced_motion (fade only)` | rule |
| GN-1.221 | `dialog.values.presentations event Disappear (clamped spring)` | field |
| GN-1.221 | `dialog.values.presentations @ cond reduced_motion` | rule |
| GN-1.222 | `dialog.body.{padding,stroke,corners} @ variant @bottom_sheet` | rule |
| GN-1.223 | `dialog.message.{corners,padding} @ variant alert {overflowing} (ADD-A13)` | rule |
| GN-1.223 | `scrollbar.* @ context dialog.message variant alert (top padding)` | rule |
| GN-1.224 | `dialog.{title,body,action_area,action} @ variant @message_dialog` | rule |
| GN-1.284† | `dialog.values.action_layout.{row_order,stacked_order,equal_widths,row_gap_px,column_gap_px}` | field |
| GN-1.284† | `dialog.values.action_layout @ dir rtl` | rule |
| GN-1.226 | `spinner.track.fill` | field |
| GN-1.227 | `spinner.values.stroke_width = mul(element.width, <fraction>) (data declared on indicator: data = { indicator = ["width", "height"] }; R25.3)` | field |
| GN-1.227 | `spinner.values.stroke_weight (the icon weight the stroke is scaled by, within the source's bounds)` | field |
| GN-1.227 | `spinner.indicator.glyph.line_cap (Glyph.line_cap, ADD-A34)` | field |
| GN-1.228 | `spinner.body.{min_width,min_height,max_width,max_height}` | field |
| GN-1.229 | `spinner.values.loop` | field |
| GN-1.230 | `spinner.icon.glyph @ variant @gtk_spinner` | rule |
| GN-1.230 | `spinner.icon.glyph @ variant @gtk_spinner cond reduced_motion` | rule |
| GN-1.231 | `spinner.body.opacity @ {disabled}` | rule |
| GN-1.232 | `spinner.{body.min_width,body.min_height,values.stroke_weight} @ context any status_page` | rule |
| GN-1.233 | `combo_box.values.label_arrow_gap` | field |
| GN-1.234 | `combo_box.popup.padding; combo_box.item.min_width` | field |
| GN-1.234 | `combo_box.item_mark.glyph; combo_box.values.item_mark_position` | field |
| GN-1.235 | `input.body.padding @ context combo_box.popup` | rule |
| GN-1.235 | `scroll_view.edge_effect @ context combo_box.popup` | rule |
| GN-1.236 | `combo_box.body.{fill,stroke,shadow} @ variant flat` | rule |
| GN-1.237 | `combo_box.body.fill @ {open} / {open,hover}` | rule |
| GN-1.238 | `combo_box.body.{padding,min_height} @ variant @gtk_combo_box` | rule |
| GN-1.239 | `segmented_control.track.padding, segmented_control.track.corners` | field |
| GN-1.239 | `segmented_control.segment.corners (Concentric)` | field |
| GN-1.240 | `segmented_control.segment.shadow @ {checked}` | rule |
| GN-1.241 | `segmented_control.segment.fill @ {pressed}` | rule |
| GN-1.242 | `segmented_control.track.fill @ {disabled}` | rule |
| GN-1.242 | `segmented_control.segment.fill @ {checked,disabled}` | rule |
| GN-1.242 | `segmented_control.label.opacity, segmented_control.icon.opacity @ {disabled}` | rule |
| GN-1.243 | `segmented_control.separator.fill, segmented_control.separator.padding` | field |
| GN-1.243 | `segmented_control.separator.opacity @ prev segmented_control.segment{checked} / next segmented_control.segment{checked}` | rule |
| GN-1.243 | `segmented_control.separator.transitions (Opacity)` | field |
| GN-1.244 | `segmented_control.segment.min_width (text) and @ variant icon_only / variant with_icon` | rule |
| GN-1.245 | `segmented_control.track.corners, segmented_control.segment.padding, segmented_control.separator.padding @ variant pill` | rule |
| GN-1.246 | `segmented_control.track.fill @ variant flat` | rule |
| GN-1.246 | `segmented_control.segment.fill @ {checked} / {checked,hover} / {checked,pressed} variant flat` | rule |
| GN-1.247 | `segmented_control.segment.fill, segmented_control.label.text.fill @ {checked} variant @osd` | rule |
| GN-1.247 | `segmented_control.segment.* @ {checked} context any variant @osd (any widget, R25.9)` | rule |
| GN-1.247 | `segmented_control.segment.bindings (accent roles), segmented_control.segment.shadow, segmented_control.segment.focus.paint @ {checked} variant @osd !variant flat` | rule |
| GN-1.248 | `segmented_control.segment.min_width @ variant vertical` | rule |
| GN-1.249 | `segmented_control.segment.transitions (Text, Fill, Shadow)` | field |
| GN-1.249 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.250 | `segmented_control.segment.focus (outer, offset)` | field |
| GN-1.250 | `segmented_control.segment.focus.offset @ variant @osd` | rule |
| GN-1.251 | `segmented_control.track.shadow @ cond contrast_high (inset ring)` | rule |
| GN-1.251 | `segmented_control.segment.shadow @ {hover} / {pressed} / {checked} cond contrast_high` | rule |
| GN-1.252 | `card.body.shadow` | field |
| GN-1.252 | `card.body.stroke @ cond contrast_high` | rule |
| GN-1.253 | `card.body.fill.append @ {hover} / {pressed} variant activatable` | rule |
| GN-1.253 | `card.body.transitions (focus) @ variant activatable` | rule |
| GN-1.254 | `card.body.focus.offset` | field |
| GN-1.255 | `role.card.foreground` | role |
| GN-1.255 | `role.card.shade` | role |
| GN-1.256 | `card.body.fill, card.body.shadow, card.body.bindings (Osd) @ variant @osd` | rule |
| GN-1.257 | `expander.arrow.glyph.fill, expander.arrow.opacity @ {open} variant @row` | rule |
| GN-1.258 | `expander.arrow.transform @ {open}` | rule |
| GN-1.258 | `expander.arrow.transitions (transform)` | field |
| GN-1.259 | `expander.values.presentations events @expand, @collapse` | field |
| GN-1.260 | `expander.content.fill @ variant @row` | rule |
| GN-1.261 | `expander.header.corners, expander.header.focus` | field |
| GN-1.261 | `expander.arrow.glyph` | field |
| GN-1.261 | `expander.arrow.opacity @ {hover} / {pressed}` | rule |
| GN-1.262 | `expander.values.drag_hover_delay_ms` | field |
| GN-1.263 | `expander.arrow.opacity @ {disabled}` | rule |
| GN-1.264 | `link.text.focus (ring, inset; = token.focus)` | field |
| GN-1.265 | `role.osd.link` | role |
| GN-1.265 | `role.osd.link_visited` | role |
| GN-1.265 | `link.text.text.fill (= surface.link) and @ {visited} (= surface.link_visited)` | rule |
| GN-1.266 | `link.body.fill (transparent in every state of the standalone link)` | field |
| GN-1.266 | `link.text.text.decoration (underline)` | field |
| GN-1.267 | `link.text.text.decoration.thickness, link.text.text.decoration.offset` | field |
| GN-1.268 | `role.global.accent_standalone` | role |
| GN-1.269 | `role.global.status.success.fill` | role |
| GN-1.269 | `role.global.status.success.foreground` | role |
| GN-1.269 | `role.global.status.success.standalone` | role |
| GN-1.269 | `role.global.status.warning.fill` | role |
| GN-1.269 | `role.global.status.warning.foreground` | role |
| GN-1.269 | `role.global.status.warning.standalone` | role |
| GN-1.269 | `role.global.status.error.fill` | role |
| GN-1.269 | `role.global.status.error.foreground` | role |
| GN-1.269 | `role.global.status.error.standalone` | role |
| GN-1.269 | `role.global.status.destructive.fill` | role |
| GN-1.269 | `role.global.status.destructive.foreground` | role |
| GN-1.269 | `role.global.status.destructive.standalone` | role |
| GN-1.270 | `role.window.shade` | role |
| GN-1.270 | `role.card.shade` | role |
| GN-1.270 | `role.popover.shade` | role |
| GN-1.270 | `role.header.shade` | role |
| GN-1.270 | `role.sidebar.shade` | role |
| GN-1.270 | `role.secondary_sidebar.shade` | role |
| GN-1.271 | `token.params.opacity.dim` | token |
| GN-1.271 | `token.params.opacity.dim @ cond contrast_high` | rule |
| GN-1.272 | `token.params.opacity.strong_disabled` | token |
| GN-1.273 | `token.params.opacity.dimmer` | token |
| GN-1.273 | `token.params.opacity.dimmer @ cond contrast_high` | rule |
| GN-1.274 | `role.view.background` | role |
| GN-1.274 | `role.view.foreground` | role |
| GN-1.276 | `role.@thumbnail_background` | role |
| GN-1.276 | `role.@thumbnail_foreground` | role |
| GN-1.276 | `role.@overview_background` | role |
| GN-1.276 | `role.@overview_foreground` | role |
| GN-2.01 | `form.row.min_height, form.row.padding, form.row_title, form.row_subtitle.text, form.row_subtitle.opacity, form.row_prefix, form.row_suffix, form.values.row_gap, form.row_icon @ {disabled} variant action` | rule |
| GN-2.02 | `form.row_title.text, form.row_title.opacity, form.row_subtitle.text @ variant property; form.row_subtitle.text @ variants {property,@monospace}` | rule |
| GN-2.03 | `form.row_edit_icon, form.row_indicator.opacity @ variant entry; form.row_indicator @ cond contrast_high; form.row.focus @ variant entry {focus_within}; form.row.* @ variants {entry,error} / variants {entry,warning} / variants {entry,success}; form.row.* @ variants {entry,@monospace}` | rule |
| GN-2.04 | `form.row_title.text @ variant entry !{empty} / variant entry {focus_within}; form.row_title.transitions (Geometry, Text: the title shrinking)` | rule |
| GN-2.05 | `stepper.* @ variant @row; stepper.increment.corners, stepper.decrement.corners, stepper.increment.min_width @ variant @row` | rule |
| GN-2.06 | `form.row_arrow @ variant combo; popover.body.min_width @ context any form; search_field.* @ context any form` | rule |
| GN-2.07 | `form.row @ variant switch; switch.* @ context any form` | rule |
| GN-2.08 | `form.row.min_height, form.row.padding, form.row_title.text @ variant button; form.row.fill, form.row.focus @ variants {button,primary} / variants {button,destructive}` | rule |
| GN-2.09 | `form.row @ variant expander; expander.* @ context any form` | rule |
| GN-2.10 | `form.values.group_gap, form.group_title.padding, form.group_title.min_height, form.values.page_margins, form.values.page_gap, form.values.collapse` | field |
| GN-2.11 | `banner.body.fill, banner.body.padding, banner.values.content_gap, banner.values.action_gap` | field |
| GN-2.11 | `button.body.min_width @ context any banner` | rule |
| GN-2.11 | `banner.body.fill, banner.title.opacity @ cond window_inactive; banner.body @ variant @sidebar` | rule |
| GN-2.11 | `banner.values.presentations @reveal / @conceal` | field |
| GN-2.12 | `toast.body.fill (Osd binding), toast.body.shadow, toast.body.corners, toast.values.margins, toast.values.stack_gap, toast.body.padding, toast.values.natural_width, toast.values.durations` | field |
| GN-2.12 | `toast.body.stroke @ cond contrast_high` | rule |
| GN-2.12 | `toast.values.presentations events Appear and Disappear (scale and fade), @replace` | field |
| GN-2.13 | `status_page.values.margins, status_page.values.icon_to_title, status_page.icon.icon.size, status_page.icon.opacity, status_page.title.text @ variant @compact / variant loading` | rule |
| GN-2.14 | `avatar.body.fill (values.palette entries, gradient layers), avatar.initials.text` | field |
| GN-2.14 | `avatar.initials.text.fill @ variant @contrasted` | rule |
| GN-2.14 | `avatar.values.palette chosen per element; avatar.values.initials_size (ADD-B14, ADD-B11)` | field |
| GN-2.15 | `view_switcher.bar.padding, view_switcher.item.min_height, view_switcher.item.padding, view_switcher.item_label.text, view_switcher.values.item_gap, view_switcher.values.item_min_width @ variant narrow / variant @wide / variant @title / variant @bar / variant @sidebar; badge.* @ context any view_switcher` | rule |
| GN-2.15 | `view_switcher.values.drag_hover_delay_ms, view_switcher.values.presentations @bar_reveal` | field |
| GN-2.16 | `view_switcher.* @ variant @inline: badge offsets, item_label.padding, values.item_gap, values.drag_hover_delay_ms; segmented_control.* @ context any view_switcher` | rule |
| GN-2.17 | `sheet.scrim, sheet.body.shadow, sheet.handle, sheet.body.corners, sheet.body.shadow (inset outline), sheet.bottom_bar.shadow @ variant bottom / variant @flush` | rule |
| GN-2.17 | `sheet.body.transitions, sheet.values.presentations events Appear (spring) and @bottom_bar_reveal; sheet.values.top_padding_range` | field |
| GN-2.18 | `navigation_view.values.presentations @push / @pop (spring), navigation_view.edge_shadow, navigation_view.scrim, navigation_view.values.swipe_back, navigation_view.values.swipe_duration_min_ms, navigation_view.values.swipe_duration_max_ms` | field |
| GN-2.18 | `navigation_view.values.presentations @ cond reduced_motion` | rule |
| GN-2.19 | `sidebar.* @ variant overlay: sidebar.scrim, sidebar.edge_shadow, sidebar.pane.bindings; sidebar.values.presentations events Appear, Disappear (spring)` | rule |
| GN-2.20 | `toolbar.* @ variant bottom (GtkActionBar, AdwToolbarView's bottom bar): bar.fill, bar.stroke.top (the shade line), bar.padding, values.item_gap; toolbar.bar.fill @ cond window_inactive variant bottom; button.body.opacity @ cond window_inactive context toolbar.bar variant bottom; toolbar.* @ variants {bottom,@inline} / variants {bottom,@shrunk}` | rule |
| GN-2.21 | `search_field.bar.fill, search_field.bar.stroke, search_field.bar.padding, search_field.bar_close_button @ variant @inline / variant flat / variant @shrunk; search_field.bar @ cond window_inactive` | rule |
| GN-2.22 | `search_field.search_icon, search_field.clear_button, search_field.search_icon.opacity @ {hover}; search_field.field.padding` | rule |
| GN-2.23 | `input.reveal_button, input.caps_lock_indicator.opacity @ variant secure; input.caps_lock_indicator.opacity @ cond contrast_high; input.values.mask_reveal_ms` | rule |
| GN-2.24 | `stepper.body, stepper.text.min_width, stepper.text.text (tabular figures), stepper.separator, stepper.increment, stepper.decrement @ variant vertical; stepper.separator @ cond contrast_high` | rule |
| GN-2.25 | `calendar.frame.stroke, calendar.header, calendar.nav_button, calendar.values.grid_padding, calendar.day_label.padding, calendar.day_label.text (tabular figures)` | field |
| GN-2.25 | `calendar.today_marker @ {today}; calendar.day.fill, calendar.day.corners @ {selected}; calendar.day.fill @ {@marked}; calendar.day_label.opacity @ {other_month}; calendar.day.focus @ {focus_visible}` | rule |
| GN-2.26 | `level_indicator.track, level_indicator.fill, level_indicator.block, level_indicator.values.block_gap, level_indicator.values.block_min_length, level_indicator.values.thresholds @ variant continuous / variant discrete / variant vertical` | rule |
| GN-2.26 | `level_indicator.fill.fill @ {@low} / {@high} / {@full}; level_indicator.track @ cond contrast_high; level_indicator.* @ {disabled}` | rule |
| GN-2.27 | `color_button.body.padding, color_button.swatch.corners, color_button.swatch_ring @ {@light_color}` | rule |
| GN-2.27 | `color_picker.swatch.corners @ pos first / pos last; color_picker.swatch.focus @ {focus_visible}; color_picker.swatch_ring @ {drop_target} / cond contrast_high; color_picker.preview` | rule |
| GN-2.28 | `font_chooser.requester, font_chooser.requester_separator, font_chooser.requester_label.text, font_chooser.values.grid_gap` | field |
| GN-2.29 | `split_button.primary.corners, split_button.secondary.corners, split_button.divider, split_button.values.part_gap, split_button.secondary.padding @ variant primary / variant destructive / variant flat / variant @opaque; split_button.divider.opacity @ variant flat {hover} / variant flat {focus_visible}; split_button.* @ {disabled}; split_button.divider @ cond contrast_high` | rule |
| GN-2.30 | `menu_bar.bar.stroke, menu_bar.bar.padding, menu_bar.item.padding, menu_bar.item.min_height, menu_bar.item.corners, menu_bar.values.submenu_corners` | field |
| GN-2.30 | `menu_bar.item.fill @ {open} / {hover}; menu_bar.item.stroke @ cond contrast_high` | rule |
| GN-2.31 | `banner.body.fill @ variant info / variant @question / variant warning / variant error; banner.edge_line; banner.body.padding` | rule |
| GN-2.31 | `button.body.fill @ {hover} / {pressed} context any banner; banner.close_button @ {hover} / {pressed}` | rule |
| GN-2.32 | `group_box.body.stroke, group_box.body.corners, group_box.values.title_margin` | field |
| GN-2.33 | `view_switcher.* @ variant @stack: button.* @ context any layout variant @linked context any view_switcher; view_switcher.item.min_width; badge.* @ variant dot context any view_switcher` | rule |
| GN-2.34 | `list.row.padding @ variant @stack_sidebar; badge.* @ variant dot context any sidebar` | rule |
| GN-2.35 | `component.tab_overview.grid, component.tab_overview.thumbnail, component.tab_overview.thumbnail_close_button, component.tab_overview.attention_dot, component.tab_overview.new_tab_button, component.tab_overview.drop_target, its values and presentations` | component |
| GN-2.36 | `badge.body @ variant dot; badge.body.fill @ variant dot {@needs_attention}` | rule |
| GN-2.36 | `badge.body, badge.label.text @ variant count; badge.values.cutout` | field |
| GN-2.37 | `badge.label.text, badge.values.anchor_offset_x, badge.values.anchor_offset_y @ context any tab; badge.label.text @ size small` | rule |
| GN-2.38 | `carousel.values.presentations (event Move, part item: the page spring), carousel.values.scroll_timeout_ms` | field |
| GN-2.38 | `page_indicator.dot.transitions (linear)` | field |
| GN-2.39 | `about_dialog.icon.icon.size` | field |
| GN-2.39 | `about_dialog.icon.shadow` | field |
| GN-2.39 | `about_dialog.version.text, about_dialog.version.padding, about_dialog.version.corners` | field |
| GN-2.39 | `about_dialog.values.margins, about_dialog.values.gap, about_dialog.values.subpage_margins` | field |
| GN-2.40 | `keycap.key.fill` | field |
| GN-2.40 | `keycap.key.shadow` | field |
| GN-2.40 | `keycap.key.stroke @ cond contrast_high` | rule |
| GN-2.40 | `keycap.key.corners` | field |
| GN-2.40 | `keycap.key.padding` | field |
| GN-2.40 | `keycap.key.padding @ variant @legacy` | rule |
| GN-2.40 | `keycap.key.min_width` | field |
| GN-2.40 | `keycap.key.text.font` | field |
| GN-2.40 | `keycap.values.key_gap` | field |
| GN-2.40 | `keycap.label.focus` | field |
| GN-2.40 | `keycap.label.focus @ variant @legacy` | rule |
| GN-2.41 | `shortcuts_dialog.values.page_gap, shortcuts_dialog.nav_pill.corners, shortcuts_dialog.nav_pill.padding, shortcuts_dialog.row.padding, shortcuts_dialog.values.row_min_height, shortcuts_dialog.row_subtitle.text, shortcuts_dialog.row_subtitle.opacity` | field |
| GN-2.42 | `input.selection_handle.fill` | field |
| GN-2.42 | `input.selection_handle.corners` | field |
| GN-2.42 | `input.selection_handle.padding` | field |
| GN-2.42 | `input.selection_handle.transform @ variant insertion` | rule |
| GN-2.42 | `input.magnifier.fill` | field |
| GN-2.43 | `component.emoji_chooser.section_button, component.emoji_chooser.cell, component.emoji_chooser.completion_row` | component |
| GN-2.44 | `media_controls.play_overlay.glyph` | field |
| GN-2.44 | `media_controls.play_overlay.corners` | field |
| GN-2.44 | `media_controls.play_overlay.min_width` | field |
| GN-2.45 | `input.* @ variant editable_label {focused}` | rule |
| GN-2.46 | `layout.values.{item_gap,line_gap} @ variant flow (AdwWrapBox, AdwWrapLayout)` | rule |
| GN-2.48 | `file_chooser.*; sidebar.* @ context any file_chooser; breadcrumb.* @ context any file_chooser; list.* @ context any file_chooser` | rule |
| GN-2.49 | `drag_image.image.icon.fill` | field |
| GN-2.49 | `drag_image.image.text.fill` | field |
| GN-2.50 | `toast.body, toast.body.corners, toast.body.fill (gradient layer), toast.body.padding @ variant @app_notification` | rule |
| GN-2.51 | `wizard.step_list (bindings), wizard.step.padding, wizard.step.corners, wizard.step.fill @ {current}` | rule |
| GN-2.52† | `breadcrumb.crumb, breadcrumb.crumb_icon, breadcrumb.crumb_label.padding; button.* @ context any layout variant @linked context any breadcrumb; breadcrumb.crumb @ {checked}` | rule |
| GN-2.52† | `button.body.padding @ !variant icon_only !variant with_icon context any breadcrumb (the text buttons' side padding)` | rule |
| GN-2.52† | `button.values.icon_text_gap @ variant with_icon context any breadcrumb` | rule |
| GN-2.53† | `color_picker.plane, color_picker.crosshair @ {focused} / !{focused}; color_picker.plane.focus, color_picker.plane.cursor, color_picker.preview.corners, color_picker.values.grid_gap` | rule |
| GN-2.53† | `slider.* @ variant hue context any color_picker / variant alpha context any color_picker; color_picker.checkerboard (pattern: ADD-B01); popover.body.corners @ context any color_picker` | rule |
| GN-2.54† | `media_controls.bar (Osd binding), media_controls.button @ variant @play, media_controls.time_label.text, media_controls.volume_popup, media_controls.values.auto_hide_ms, media_controls.values.presentations events Appear and Disappear; media_controls.values.gap; slider.* @ context any media_controls` | rule |
| GN-2.55† | `page_indicator.dot @ {selected}, page_indicator.line @ variant @lines, page_indicator.values.dot_gap, page_indicator.values.follows_scroll (ADD-B13)` | rule |
| GN-2.56† | `autocomplete.popup, autocomplete.item @ {hover}` | rule |
| GN-2.56† | `autocomplete.values.width, autocomplete.values.max_visible_items, autocomplete.values.filter_delay_ms, autocomplete.values.hover_selects` | field |
| GN-2.56† | `autocomplete.values.placement (GtkPopover)` | placement |
| GN-2.57† | - (GTK's print, page setup and app chooser dialogs) | out (R1.1) |
| GN-2.57† | `dialog.values.action_layout.row_gap, dialog.action_area.padding (GtkDialog's action area)` | field |
| GN-2.58† | `input.values.mask_character @ variant secure (a preference list: the first the font covers)` | field |
| GN-3.01 | `PartStyle.shadow.layers (S2.5, per part)` | field |
| GN-3.02 | `PartStyle.stroke.position Outside (S2.4)` | field |
| GN-3.02 | `PartStyle.shadow (ShadowLayer.spread, S2.5)` | field |
| GN-3.03 | `PartStyle.shadow (ShadowLayer.inset, offset to one side; S2.5, R7 second outline)` | field |
| GN-3.04 | `role.global.scrim` | role |
| GN-3.04 | `dialog.scrim.fill` | field |
| GN-3.04 | `navigation_view.scrim.fill` | field |
| GN-3.05 | `navigation_view.edge_shadow.shadow` | field |
| GN-3.05 | `navigation_view.values.presentations event @push (slide with edge shadow)` | field |
| GN-3.06 | `scroll_view.edge_effect.fill (overshoot and undershoot gradient)` | field |
| GN-3.06 | `token.behaviour.scroll.overscroll` | token |
| GN-3.07 | `button.icon.shadow (PartStyle.shadow of an icon part)` | field |
| GN-3.07 | `PartStyle.shadow.layers on every icon part (S2.5)` | field |
| GN-3.08 | `Color.a translucent roles and fills (S2.1, R5.3)` | field |
| GN-3.08 | `PartStyle.fill.append over base (S2.3)` | field |
| GN-3.09 | `role.osd.* (SurfaceSet::Osd)` | role |
| GN-3.09 | `toast.body.bindings Surface(Osd)` | field |
| GN-3.09 | `toast.body.fill` | field |
| GN-3.11 | `declare.widget.*.variants (S10, stacked style classes)` | field |
| GN-3.11 | `Selector.variants SetMatch (S7)` | rule |
| GN-3.12 | `role.global.status.* (fill, foreground, standalone)` | role |
| GN-3.12 | `PartStyle.bindings Role accent to status (S2.11)` | field |
| GN-3.13 | `role.global.accent_standalone` | role |
| GN-3.14 | `role.global.accent` | role |
| GN-3.14 | `role.global.accent_foreground` | role |
| GN-3.14 | `role.global.accent_standalone` | role |
| GN-3.15 | `reader:gnome:portal accent RGB snapped to the nearest of nine accents` | reader |
| GN-3.15 | `input.accent` | input |
| GN-3.16 | `reader:gnome:org.gnome.desktop.interface accent-color enum as fallback` | reader |
| GN-3.16 | `input.accent` | input |
| GN-3.17 | `PartStyle.bindings Role accent override on toast.body and osd surfaces` | field |
| GN-3.17 | `role.global.accent_standalone = adw_standalone(...)` | role |
| GN-3.18 | `token.motion.durations.<transition tokens>` | token |
| GN-3.18 | `token.motion.timings.<transition tokens>` | token |
| GN-3.19 | `token.motion.timings.<easing curves, default timed-animation easing>` | token |
| GN-3.20 | `token.motion.timings.<spring keys> Timing::Spring` | token |
| GN-3.21 | `token.motion.durations.<timed animations defined in code>` | token |
| GN-3.22 | `token.motion.durations.revealer` | token |
| GN-3.22 | `token.motion.durations.stack` | token |
| GN-3.22 | `navigation_view.values.presentations` | field |
| GN-3.23 | `token.theme_change Suspend (CSS transitions off for the stated duration)` | token |
| GN-3.24 | `token.behaviour.scroll.deceleration (kinetic friction)` | token |
| GN-3.24 | `token.behaviour.scroll.overscroll` | token |
| GN-3.24 | `token.behaviour.scroll.overscroll_extent (the maximum overshoot, ADD-A29)` | token |
| GN-3.24 | `token.motion.durations.scroll_animation` | token |
| GN-3.25 | `token.motion.speed_factor @ cond animations_off` | rule |
| GN-3.25 | `dialog.values.presentations @ cond reduced_motion (bottom sheet slide to crossfade)` | rule |
| GN-3.25 | `navigation_view.values.presentations @ cond reduced_motion` | rule |
| GN-3.25 | `component.tab_overview.values.presentations @ cond reduced_motion` | rule |
| GN-3.25 | `spinner.values.loop @ cond reduced_motion` | rule |
| GN-3.25 | `cond reduced_motion` | condition |
| GN-3.25 | `cond animations_off` | condition |
| GN-3.26 | `reader:gnome:org.gnome.desktop.a11y.interface reduced-motion` | reader |
| GN-3.26 | `cond reduced_motion` | condition |
| GN-3.71† | - | out (R1.1) |
| GN-3.27 | `token.focus (FocusIndicator width, offset)` | token |
| GN-3.27 | `PartStyle.focus per part (S2.10)` | field |
| GN-3.28 | `FocusIndicator.transition (grow-in)` | field |
| GN-3.29 | `FocusIndicator.corners and offset per part (target node differs from the widget box)` | field |
| GN-3.30 | `token.behaviour.focus_visible_when (FocusVisibleWhen::Keyboard with its timeout_ms)` | token |
| GN-3.30 | `reader:gnome:org.gnome.desktop.a11y.interface keyboard-focus-visible-timeout` | reader |
| GN-3.31 | `token.focus @ cond contrast_high (paint)` | rule |
| GN-3.31 | `cond contrast_high` | condition |
| GN-3.32 | `token.caret.blink_on_ms / blink_off_ms (Absent = no blink)` | token |
| GN-3.32 | `reader:gnome:org.gnome.desktop.interface caret blink setting` | reader |
| GN-3.33 | `token.caret.blink_on_ms / blink_off_ms` | token |
| GN-3.33 | `reader:gnome:org.gnome.desktop.interface caret blink period` | reader |
| GN-3.34 | `token.caret.blink_timeout_ms` | token |
| GN-3.34 | `reader:gnome:org.gnome.desktop.interface caret blink timeout` | reader |
| GN-3.35 | `token.caret.blink_style Fade (ADD-A24)` | token |
| GN-3.36 | `token.caret.width as an expression of the aspect ratio and the font, with token.params.caret (ADD-A44)` | token |
| GN-3.37 | `token.caret.secondary (the secondary caret's colour; the base of every caret part's secondary caret, ADD-A24)` | token |
| GN-3.38 | `token.caret.shown_in_read_only (ADD-A24)` | token |
| GN-3.38 | `reader:gnome:org.gnome.desktop.a11y always-show-text-caret` | reader |
| GN-3.39 | `input.caret.fill per widget (caret part, ADD-A24)` | field |
| GN-3.39 | `input.caret.fill @ {drop_target}` | rule |
| GN-3.39 | `text_area.caret.fill` | field |
| GN-3.73† | `token.caret.hides_while_selecting` | token |
| GN-3.73† | `input.caret.fill @ {drop_target} drop-position caret` | rule |
| GN-3.40 | `input.selection.fill @ !{focus_within}` | rule |
| GN-3.40 | `role.global.selection_text_background_unfocused (ADD-A49)` | role |
| GN-3.41 | `role.global.selection_text_foreground (Absent: the selected text keeps its own colour)` | role |
| GN-3.41 | `input.selection.text` | field |
| GN-3.42 | `list.row_background.fill @ {selected,hover}` | rule |
| GN-3.42 | `list.row_background.fill @ {selected,pressed}` | rule |
| GN-3.43 | `list.row_background.fill @ {selected} context sidebar (neutral foreground at an alpha)` | rule |
| GN-3.43 | `menu.item.fill @ {selected}` | rule |
| GN-3.43 | `tab.tab.fill @ {selected}` | rule |
| GN-3.66† | `role.global.drop_highlight` | role |
| GN-3.66† | `list.row_background.fill @ {drop_target}` | rule |
| GN-3.66† | `list.view.stroke @ {drop_target}` | rule |
| GN-3.66† | `PartStyle rules @ {drop_target} per widget` | rule |
| GN-3.44 | `window.title_bar, toolbar.bar, search_field.bar, tab.bar, banner.body, sidebar.pane, window.frame fill, opacity and shadow rules @ cond window_inactive` | rule |
| GN-3.44 | `PartStyle.transitions (backdrop fade)` | field |
| GN-3.44 | `cond window_inactive` | condition |
| GN-3.46 | `token.pointer.theme / token.pointer.size (ADD-A46)` | token |
| GN-3.46 | `reader:gnome:org.gnome.desktop.interface cursor theme and size` | reader |
| GN-3.67† | `PartStyle.cursor per part (S2.11)` | field |
| GN-3.67† | `token.behaviour.hide_pointer_while_typing` | token |
| GN-3.72† | - | out (R1.1) |
| GN-3.47 | `token.behaviour.double_click_ms / double_click_distance` | token |
| GN-3.47 | `reader:gnome:org.gnome.desktop.peripherals double-click` | reader |
| GN-3.48 | `token.behaviour.drag_threshold` | token |
| GN-3.48 | `reader:gnome:org.gnome.desktop.peripherals drag threshold` | reader |
| GN-3.49 | `token.behaviour.long_press_ms (ADD-A45)` | token |
| GN-3.50 | `token.behaviour.mnemonics OnAlt delay (ADD-A12)` | token |
| GN-3.51 | `token.behaviour.spring_loading_ms (ADD-A45)` | token |
| GN-3.68† | `token.behaviour.key_repeat_delay_ms / key_repeat_interval_ms` | token |
| GN-3.68† | `reader:gnome:org.gnome.desktop.peripherals key repeat` | reader |
| GN-3.69† | `stepper.values.repeat_delay_ms / repeat_interval_ms` | field |
| GN-3.69† | `scrollbar.values.repeat_delay / repeat_interval (trough paging)` | field |
| GN-3.69† | `slider.values.repeat_delay / repeat_interval (trough paging)` | field |
| GN-3.56 | `token.text.rendering.{antialiasing,hinting,subpixel_order} (ADD-B04)` | token |
| GN-3.56 | `token.text.rendering.engine (font-rendering: automatic or manual, keyed; ADD-B04)` | token |
| GN-3.56 | `reader:gnome:org.gnome.desktop.interface font-antialiasing, font-hinting, font-rgba-order, font-rendering` | reader |
| GN-3.57 | `PartStyle.icon.weight per part (S2.9)` | field |
| GN-3.58 | `PartStyle.icon.status_colors (the symbolic palette's success, warning, error; ADD-A37)` | field |
| GN-3.58 | `IconStyle.fill` | field |
| GN-3.58 | `role.global.status.*` | role |
| GN-3.59 | `token.icon_sizes.<contexts>` | token |
| GN-3.60 | `PartStyle.opacity of icon parts @ widget states` | rule |
| GN-3.61 | `PartStyle fill, stroke rules @ cond contrast_high per widget` | rule |
| GN-3.61 | `roles @ cond contrast_high` | rule |
| GN-3.61 | `cond contrast_high` | condition |
| GN-3.63 | `text_scale (the S2.2 operand, ADD-A55)` | field |
| GN-3.63 | `length bases in sp: = mul(<length>, text_scale)` | field |
| GN-3.70† | - | out (R1.1) |
| GN-SF01† | `code_editor.* bases: the Adwaita and Adwaita-dark schemes the adwaita preset carries (ADD-A64)` | field |
| GN-SF01† | `code_editor.* @ variant @classic (GtkSourceView's default scheme when the application sets none; ADD-A64, R25.11)` | rule |
| GN-SF02† | `code_editor.text.fill, code_editor.text.text.fill; code_editor.selection @ !{focused}` | rule |
| GN-SF03† | `code_editor.caret.fill; code_editor.caret.fill @ variant @bidi_secondary (its base = token.caret.secondary, GN-3.37; the scheme restates it where it states one)` | field |
| GN-SF04† | `code_editor.current_line.fill` | field |
| GN-SF05† | `code_editor.gutter.fill, code_editor.line_number.text, code_editor.current_line_number, code_editor.gutter_separator.stroke, code_editor.values.gutter_padding` | field |
| GN-SF06† | `code_editor.whitespace_mark.glyph @ variant @space / variant @tab / variant @newline / variant @nbsp` | rule |
| GN-SF07† | `code_editor.bracket_match.text, code_editor.bracket_mismatch` | field |
| GN-SF08† | `code_editor.search_match.fill, code_editor.search_match.text.fill` | field |
| GN-SF09† | `code_editor.right_margin.stroke, code_editor.right_margin_area.fill, code_editor.values.right_margin_column` | field |
| GN-SF10† | `code_editor.syntax_span.text, code_editor.syntax_span.fill @ variant (typed and keyed syntax roles, per-language keyed)` | rule |
| GN-SF11† | `code_editor.background_pattern.fill (pattern: ADD-B01), code_editor.minimap_slider.fill` | field |
| GN-SF12† | `code_editor.inline_annotation.fill @ variant warning / variant error / variant @accent` | rule |
| GN-SF13† | `code_editor.completion_popup, code_editor.completion_row, code_editor.completion_icon, code_editor.completion_detail.opacity, code_editor.completion_details_pane, code_editor.informative_popup @ variant warning / variant error` | rule |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.fill` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.fill @ {drop_target}` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.stroke @ {drop_target}` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.corners` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.transitions` | component |
| GN-SF14† | `component.dock_frame_drop.frame.shadow @ {drop_target}` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.fill` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.corners` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.glyph` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.shadow` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.fill @ {drop_target}` | component |
| GN-SF15† | `component.dock_frame_drop.values.button_gap` | component |
| GN-SF16† | `component.dock_frame_drop.switcher_indicator.fill` | component |
| GN-SF17† | `dialog.values.presentation_breakpoint` | field |
| GN-SF17† | `dialog.* @ variant @bottom_sheet / variant narrow / variant @short` | rule |
| GN-SF18† | `dialog.values.placement (CenteredInHost)` | placement |
| GN-SF19† | `dialog.values.placement @ variant @bottom_sheet (BottomSheet)` | placement |
| GN-SF19† | `dialog.body.min_width @ variant @bottom_sheet` | rule |
| GN-SF20† | `dialog.* @ variant separate_window (AdwDialog's window fallback)` | rule |
| GN-SF20† | `dialog.* @ variant separate_window` | rule |
| GN-SF21† | - | out (R1.1) |
| GN-RL01† | `wizard.action_area, wizard.values.button_order, wizard.values.button_gap, wizard.values.buttons_equal_width, wizard.values.button_placement @ variant @header_bar; button.* @ variant primary context any wizard` | rule |
| GN-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| GN-FM01† | `reader:gnome:fontconfig emoji family (fc-match emoji)` | reader |
| GN-FM02† | `token.text.fallbacks.families (the font-name family list), token.text.fallbacks.per_language (Pango's per-language order; ADD-A41)` | token |
| GN-FM02† | `reader:gnome:org.gnome.desktop.interface font-name family list (list no longer dropped)` | reader |
| GN-FM03† | `Selector.direction Rtl (S7)` | rule |
| GN-FM03† | `token.icon_mirroring (S2.9)` | token |
| GN-FM03† | `icon lookup by direction, -rtl names (ADD-B05)` | field |
| GN-FM03† | `reader:gnome:gtk_get_locale_direction` | reader |
| GN-P3-1† | - | out (R1.1) |
| GN-P3-2† | - | out (R1.1) |
| GN-P3-3† | `window.values.window_menu_items (GTK's fallback)` | field |
| GN-P3-3† | `window.values.window_menu_placement (GtkPopover at the pointer)` | placement |
| GN-P3-3† | `menu.* @ context window.window_menu` | rule |
| GN-P3-3† | `window.values.title_bar_click_actions` | field |
| GN-P3-3† | `reader:gnome:org.gnome.desktop.wm.preferences action-right-click-titlebar` | reader |
| GN-P3-3† | - (the window menu GNOME Shell draws) | out (R1.1) |
| GN-P3-4† | `window.values.caption_button_capabilities` | field |
| GN-P3-4† | `window.caption_button_glyph.glyph @ variant restore` | rule |
| GN-P3-5† | `window.title_bar @ cond full_screen (absent)` | rule |
| GN-P3-5† | `toolbar.values.presentations event @reveal` | field |
| GN-P3-5† | - (GNOME Shell's size-change animation on entering and leaving full screen) | out (R1.1) |
| GN-P3-6† | - | out (R1.1) |
| GN-P3-7† | - | out (R1.1) |
| GN-P3-8† | - | out (R1.1) |
| GN-P3-9† | - | out (R1.1) |
| GN-P3-10† | - | out (R1.1) |
| GN-P3-11† | - | out (R1.1) |
| GN-P3-12† | `token.pointer.drag_cursors.<drop operation> (ADD-A46)` | token |
| GN-P3-13† | - | out (R1.1) |
| GN-P3-14† | - | out (R1.1) |
| GN-P3-15† | `token.behaviour.scroll_lines_per_step` | token |
| GN-P3-16† | `Edge.snap (ADD-A33: libadwaita's one-pixel lines at a fractional scale)` | field |
| GN-P3-16† | - (the display scale itself) | out (R1.1) |
| GN-P3-17† | - | preset (R1.3) |
| GN-P3-18† | - | newest (R1.5) |
| GN-P3-19† | `button.icon.transitions (PropertyGroup::Icon, TransitionKind::Keyed per GtkSvg state transition: sliders.gpa on the font dialog's toggle; ADD-A27, R25.12)` | field |
| GN-P3-19† | `spinner.icon.glyph, spinner.values.loop (the looping process-working.gpa: IconStyle.animation's Loop, R25.12)` | field |
| GN-P3-20† | `font_chooser.filter_title.text, font_chooser.family_row_label, font_chooser.preview, font_chooser.values.row_margins, font_chooser.values.grid_gap; status_page.* @ context any font_chooser` | rule |
| GN-P3-21† | - | out (R1.1) |
| GN-P3-22† | `list.row.padding, list.leading_icon.opacity, list.values.leading_gap @ context any file_chooser; button.* (eject) @ variants {flat,circular} {hover} context any file_chooser / variants {flat,circular} {pressed} context any file_chooser; list.row @ variant @new_bookmark / variant @drop_placeholder; list.row @ {drop_target} / {selected}` | rule |
| GN-P3-23† | `file_chooser.frame.shadow; button.* @ variant flat context any file_chooser; list.row.padding @ context any file_chooser; list.row.corners, list.row.padding @ variant grid context any file_chooser; button.icon.transform, button.icon.transitions @ {open} context any file_chooser` | rule |
| GN-P3-24† | `layout.values.item_gap, layout.values.line_gap, layout.values.flow_max_per_line @ variant flow` | rule |
| GN-P3-25† | `segmented_control.values.segment_width (Fit)` | field |
| GN-P3-26† | `input.magnifier.{fill,corners,shadow,min_width,min_height}; text_area.magnifier` | field |
| GN-P3-26† | `input.values.magnifier_zoom; input.values.magnifier_offset (and on text_area)` | field |
| GN-P3-27† | `code_editor.misspelled_word.text.decoration` | field |
| GN-P4-1† | - | out (R1.1) |
| GN-P4-2† | `popover.values.placement (GtkPopover)` | placement |
| GN-P4-2† | `menu.values.placement (GtkPopover)` | placement |
| GN-P4-2† | `combo_box.values.popup_placement (GtkPopover, width AtLeastAnchor)` | placement |
| GN-P4-3† | - | out (R1.1) |
| GN-P4-4† | `token.caret.overwrite_shape, token.caret.block_text (ADD-A23)` | token |
| GN-P4-4† | `text_area flags overwrite; code_editor flags overwrite (ADD-A23)` | flag |
| GN-P4-4† | `text_area.caret @ context text_area {overwrite} (ADD-A23, ADD-A24)` | rule |
| GN-P4-5† | `scrollbar.values.trough_click` | field |
| GN-P4-5† | `slider.values.trough_click` | field |
| GN-P4-6† | `menu.values.submenu_switch` | field |
| GN-P4-6† | `menu.values.submenu_placement (GtkPopover nested: end side, top-aligned)` | placement |
| GN-P4-7† | `checkbox.mark.glyph (GlyphSource::Path from check.svg)` | field |
| GN-P4-7† | `radio.dot.glyph (GlyphSource::Path from bullet.svg)` | field |
| GN-P4-8† | `menu.values.placement @ context input / context text_area (GtkPopover, point anchor)` | placement |
| GN-P4-8† | `input.values.context_menu_items, text_area.values.context_menu_items` | field |
| GN-P4-9† | - | out (R1.1) |
