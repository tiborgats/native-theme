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
(`PartStyle.…`, `FocusIndicator.…`) where the row concerns every part. The selector of a rule follows `@`, in the catalogue's selector notation (its *Selector
notation* table): the element's own variants and states come before the first `context`, a context's
`variant`/`{…}` belong to that ancestor, and ` / ` separates alternatives. A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`, `reference (R1.6)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| GN-1.01 | `window.frame.shadow (three layers)` | field |
| GN-1.02 | `window.frame.shadow @ cond window_inactive` | rule |
| GN-1.02 | `window.frame.transitions (shadow group)` | field |
| GN-1.03 | `window.frame.stroke (inside, inset outline)` | field |
| GN-1.03 | `window.frame.stroke @ cond contrast_high` | rule |
| GN-1.04 | `window.frame.shadow @ cond contrast_high (outer ring layer)` | rule |
| GN-1.05 | `window.frame.{corners,stroke,shadow} @ cond tiled:top / cond tiled:right / cond tiled:bottom / cond tiled:left` | rule |
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
| GN-1.11 | `window.title_bar.min_height @ variant @standalone / variant @default_decoration / variants {@default_decoration,@standalone} / variant @collapse_spacing / variants {@collapse_spacing,@default_decoration}` | rule |
| GN-1.12 | `window.title_bar_content.padding` | field |
| GN-1.12 | `window.title_bar_content.padding @ variant @standalone / variant @default_decoration / variants {@default_decoration,@standalone}` | rule |
| GN-1.12 | `window.values.title_bar_box_gap_px` | field |
| GN-1.12 | `window.values.title_margin_px` | field |
| GN-1.13 | `window.title_bar.shadow (inset layers: bottom shade line, side borders)` | field |
| GN-1.13 | `window.title_bar.shadow @ cond contrast_high` | rule |
| GN-1.13 | `role.header.border` | role |
| GN-1.14 | `window.title_bar.shadow @ !variant transparent_title_bar (drop layers)` | rule |
| GN-1.14 | `window.title_bar.shadow @ !variant transparent_title_bar cond contrast_high` | rule |
| GN-1.15 | `role.header.{foreground,shade,border}` | role |
| GN-1.15 | `role.header.shade_darker (ADD-A48)` | role |
| GN-1.16 | `window.title_bar_content.opacity @ cond window_inactive` | rule |
| GN-1.16 | `window.title_bar.fill @ cond window_inactive` | rule |
| GN-1.16 | `window.title_bar.transitions (fill)` | field |
| GN-1.16 | `window.title_bar_content.transitions (opacity)` | field |
| GN-1.17 | `window.title_bar.{fill,shadow} @ variant transparent_title_bar` | rule |
| GN-1.17 | `window.title_bar.{fill,shadow,transitions} @ variant @inline` | rule |
| GN-1.17 | `window.title_bar_content.{opacity,transitions} @ variant @inline cond window_inactive (no backdrop change)` | rule |
| GN-1.18 | `window.title.{text,padding}` | field |
| GN-1.18 | `window.subtitle.{text,padding,opacity}` | field |
| GN-1.18 | `window.title.margin, window.subtitle.margin` (the title box's negative margins, ADD-A79) | field |
| GN-1.19 | `window.title.text.font (the UI font's family and size, its own weight)` | field |
| GN-1.19 | `reader:gnome:titlebar-uses-system-font (titlebar-font only when false)` | reader |
| GN-1.20 | `window.values.caption_button_layout` | field |
| GN-1.20 | `reader:gnome:org.gnome.desktop.wm.preferences button-layout` | reader |
| GN-1.21 | `window.caption_button.{min_width,padding}` | field |
| GN-1.21 | `window.values.caption_button_gap_px` | field |
| GN-1.21 | `window.caption_button.{min_width,min_height,padding} @ variant @default_decoration` | rule |
| GN-1.21 | `window.caption_button_glyph.padding @ variant @default_decoration` | rule |
| GN-1.21 | `window.title_icon.padding` | field |
| GN-1.21 | `window.title_icon.padding @ variant @default_decoration` | rule |
| GN-1.22 | `window.caption_button_glyph.{fill,corners,padding}` | field |
| GN-1.22 | `window.caption_button_glyph.fill @ {hover}` | rule |
| GN-1.22 | `window.caption_button_glyph.fill @ {pressed}` | rule |
| GN-1.22 | `window.caption_button_glyph.transitions` | field |
| GN-1.22 | `window.caption_button_glyph.shadow @ cond contrast_high (inset ring)` | rule |
| GN-1.23 | `window.{frame,title_bar,caption_button} @ variants {@ssd_frame,@default_decoration} (mutter-x11-frames draws libadwaita's rules)` | rule |
| GN-1.23 | `- (mutter's frames drawn with the GTK 4 theme when libadwaita is not loaded, GN-P3-17†)` | preset (R1.3) |
| GN-1.278† | `button.{body,icon} @ variant back context window.title_bar` | rule |
| GN-1.278† | `button.values.menu_trigger @ variant back context window.title_bar` (`LongPressOrSecondaryClick`) | rule |
| GN-1.278† | `menu.values.{placement,has_arrow} @ context button variant back` (the navigation menu, start-aligned, `has_arrow` false) | placement |
| GN-1.282† | `window.values.resize_band` (outside: the CSS border plus the shadow's extent, at least a minimum, over `widget.frame.padding` and `shadow_extent(widget.frame.shadow, <side>)` (ADD-A90); corner_px `= max(<minimum>, widget.frame.corners.<corner>)`; includes_shadow; ADD-A80) | field |
| GN-1.282† | `window.values.resize_band @ cond maximized / cond full_screen / cond tiled:top / cond tiled:right / cond tiled:bottom / cond tiled:left (Absent)` | rule |
| GN-1.282† | `window.resize_zone.cursor @ variant top / variant right / variant bottom / variant left / variant top_left / variant top_right / variant bottom_right / variant bottom_left` | rule |
| GN-1.24 | `button.body.fill @ {checked,hover}` | rule |
| GN-1.24 | `button.body.fill @ {checked,pressed}` | rule |
| GN-1.25 | `button.label.text.fill @ {drop_target}` | rule |
| GN-1.25 | `button.icon.icon.fill @ {drop_target}` | rule |
| GN-1.25 | `button.body.shadow @ {drop_target} (inset ring layer)` | rule |
| GN-1.26 | `button.body.fill @ {open} variant menu / {open} variants {menu,flat} / {open,checked} variant menu` | rule |
| GN-1.26 | `button flags open (S7.2)` | flag |
| GN-1.27 | `button.body.padding @ variant icon_only` | rule |
| GN-1.27 | `button.body.min_width @ variant icon_only` | rule |
| GN-1.27 | `button.body.padding @ variant with_icon` | rule |
| GN-1.27 | `button.values.icon_text_gap @ variant with_icon` | rule |
| GN-1.27 | `button.label.padding @ variant with_icon` | rule |
| GN-1.27 | `button.body.padding @ variant @arrow_only` | rule |
| GN-1.27 | `button.values.menu_indicator_gap_px @ variant @arrow_only` | rule |
| GN-1.28 | `button.body.corners @ variant circular` | rule |
| GN-1.28 | `button.body.min_width, button.body.min_height, button.body.padding @ variant circular` | rule |
| GN-1.28 | `button.body.corners, button.body.padding @ variant pill` | rule |
| GN-1.28 | `button.body.{corners,min_width,min_height,padding} @ variants {menu,circular} / variants {menu,pill} (menubutton)` | rule |
| GN-1.29 | `button.body.fill @ variant flat` | rule |
| GN-1.29 | `button.body.fill @ {hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant flat` | rule |
| GN-1.29 | `button.body.opacity @ {disabled} !{checked} variant flat` | rule |
| GN-1.29 | `button.body.shadow @ {hover} / {pressed} / {checked} variant flat cond contrast_high (inset ring layer)` | rule |
| GN-1.29 | `button.body.{fill,shadow} @ variant icon_only !variant @raised !variant primary !variant destructive !variant @opaque not_context any layout variant @linked context nearest {toolbar, window, popover} = toolbar.bar / variant with_icon !variant @raised !variant primary !variant destructive !variant @opaque not_context any layout variant @linked context nearest {toolbar, window, popover} = toolbar.bar / variant @arrow_only !variant @raised !variant primary !variant destructive !variant @opaque not_context any layout variant @linked context nearest {toolbar, window, popover} = toolbar.bar (the flat look; libadwaita flattens image, image-text and arrow buttons only)` | rule |
| GN-1.29 | `button.body.{fill,shadow} @ variant icon_only !variant @raised !variant primary !variant destructive !variant @opaque not_context any layout variant @linked context nearest {toolbar, window, popover} = window.title_bar / variant with_icon !variant @raised !variant primary !variant destructive !variant @opaque not_context any layout variant @linked context nearest {toolbar, window, popover} = window.title_bar / variant @arrow_only !variant @raised !variant primary !variant destructive !variant @opaque not_context any layout variant @linked context nearest {toolbar, window, popover} = window.title_bar` | rule |
| GN-1.30 | `button.body.fill, button.label.text.fill @ variant primary` | rule |
| GN-1.30 | `button.body.fill @ {hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant primary (appended layers)` | rule |
| GN-1.30 | `button.body.focus @ variant primary (outer, offset)` | rule |
| GN-1.31 | `button.label.text.fill @ variant destructive (= role.global.status.destructive.standalone)` | rule |
| GN-1.31 | `button.body.fill @ variant destructive / {hover} variant destructive / {pressed} variant destructive / {checked} variant destructive / {checked,hover} variant destructive / {checked,pressed} variant destructive` | rule |
| GN-1.32 | `button.body.fill, button.label.text.fill @ variant @opaque` | rule |
| GN-1.32 | `button.body.fill @ {hover} / {pressed} / {checked} variant @opaque (appended layers)` | rule |
| GN-1.33 | `button.body.{fill,shadow} @ variant @raised context nearest {toolbar, window, popover} = toolbar.bar / variant @raised context nearest {toolbar, window, popover} = window.title_bar (the raised base look: the flat-in-container rules exclude @raised)` | rule |
| GN-1.33 | `button.body.{fill,shadow} @ context any layout variant @linked context nearest {toolbar, window, popover} = toolbar.bar / context any layout variant @linked context nearest {toolbar, window, popover} = window.title_bar (linked buttons stay raised)` | rule |
| GN-1.34 | `button.body.min_width, button.body.min_height, button.body.fill, button.label.text.fill @ variant @osd` | rule |
| GN-1.34 | `button.body.fill @ {hover} / {pressed} / {checked} variant @osd` | rule |
| GN-1.34 | `button.body.focus @ variant @osd (outer, offset)` | rule |
| GN-1.34 | `button.body.shadow @ variant @osd cond contrast_high (ring layer)` | rule |
| GN-1.35 | `button.body.fill @ variant @card / {hover} variant @card / {pressed} variant @card` | rule |
| GN-1.35 | `button.body.fill @ {checked} / {checked,hover} / {checked,pressed} variant @card` | rule |
| GN-1.35 | `button.body.focus.offset @ variant @card` | rule |
| GN-1.36 | `button.body.transitions (Fill, Shadow)` | field |
| GN-1.36 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.37 | `button.body.focus (kind, offset; base inset)` | field |
| GN-1.37 | `button.body.focus.offset @ variant @osd / variant primary / variant @opaque / variant @card` | rule |
| GN-1.38 | `button.body.shadow @ cond contrast_high (inset ring layer at rest)` | rule |
| GN-1.39 | `button.values.icon_text_gap @ variant with_icon (AdwButtonContent's box)` | rule |
| GN-1.39 | `button.label.{padding,text} @ variant with_icon / variant with_icon dir rtl (the label's heavier weight, its end padding)` | rule |
| GN-1.39 | `button.label.padding @ variants {with_icon,@arrow_only} / variants {with_icon,@arrow_only} dir rtl (no end padding next to the arrow)` | rule |
| GN-1.40 | `button.menu_indicator.glyph (ThemeIcon, size) @ variant menu` | rule |
| GN-1.40 | `button.menu_indicator.glyph @ variant @arrow_up / variant @arrow_start / variant @arrow_end / variant @arrow_none` | rule |
| GN-1.41 | `badge.body.{fill,min_width,min_height,corners} @ variants {dot,attention} context view_switcher.item` (the accent radial dot on a stack or view switcher's button, which is a `view_switcher.item` element; Nested controls) | rule |
| GN-1.41 | `badge.values.{anchor_offset_x_px,anchor_offset_y_px} @ variants {dot,attention} context view_switcher.item` (its place at the button's trailing top) | rule |
| GN-1.41 | `badge.values.presentations (Appear) @ variants {dot,attention} context view_switcher.item` (the dot's appearance when the page comes to need attention; the connector draws it while it does) | rule |
| GN-1.42 | `button.body.corners @ context any layout variant @linked / context any layout variant @linked pos first among any / context any layout variant @linked pos last among any / context any layout variant @linked pos only among any` (position among siblings of any kind, `SiblingScope::Any`, CONTRA-3) | rule |
| GN-1.42 | `button.body.corners @ context any layout variant @linked pos first among any dir rtl / context any layout variant @linked pos last among any dir rtl` | rule |
| GN-1.42 | `button.body.corners @ context any layout variants {@linked,vertical} pos first among any / context any layout variants {@linked,vertical} pos last among any` | rule |
| GN-1.42 | `layout.values.item_gap @ variant @linked / variants {@linked,vertical} (negative overlap)` | rule |
| GN-1.43 | `input.values.icon_text_gap` | field |
| GN-1.43 | `input.icon.padding @ variant leading / variant trailing (the icon's margin toward the text)` | rule |
| GN-1.44 | `input.icon.opacity` | field |
| GN-1.44 | `input.icon.opacity @ {hover} / {pressed}` | rule |
| GN-1.44 | `input.icon.opacity @ cond contrast_high / {pressed} cond contrast_high` | rule |
| GN-1.45 | `input.body.bindings (role accent bound to the error role) @ variant error (GNOME's .error)` | rule |
| GN-1.45 | `input.body.bindings (role accent bound to the warning role) @ variant warning` | rule |
| GN-1.45 | `input.body.bindings (role accent bound to the success role) @ variant success` | rule |
| GN-1.45 | `input.text.text.fill @ variant error / variant warning / variant success (= role.global.accent_standalone, rebound)` | rule |
| GN-1.46 | `input.body.shadow (inset ring layer) @ {drop_target}` | rule |
| GN-1.47 | `input.body.{fill,corners,padding,min_height} @ variant flat` | rule |
| GN-1.48 | `input.progress.{fill,min_height,margin}` (its bottom margin, ADD-A79) | field |
| GN-1.49 | `input.block_caret.{fill,text} @ {overwrite}` | rule |
| GN-1.49 | `input.caret (absent = true) @ {overwrite}` (the block caret replaces the line caret, ADD-A23) | rule |
| GN-1.50 | `input.values.focus_visible_when` | field |
| GN-1.50 | `input.body.focus (incl. transition)` | field |
| GN-1.51 | `input.body.shadow (inset ring layer) @ cond contrast_high` | rule |
| GN-1.52 | `text_area.gutter.fill` | field |
| GN-1.53 | `text_area.drop_caret.fill @ {drop_target}` (the drop-position caret in the accent) | rule |
| GN-1.54 | `text_area.{body.fill,text.text.fill} @ variant @inline` | rule |
| GN-1.281† | `input.preedit.text.decoration` | field |
| GN-1.281† | `text_area.preedit.text.decoration` | field |
| GN-1.281† | `input @ {composing}; text_area @ {composing}` | flag |
| GN-1.281† | `input.preedit.{text,fill} @ variant @whole / variant @selection / variant @prediction / variant @prefix / variant @suffix / variant @spelling_error / variant @compose_error` | rule |
| GN-1.281† | `text_area.preedit.{text,fill} @ variant @whole / variant @selection / variant @prediction / variant @prefix / variant @suffix / variant @spelling_error / variant @compose_error` (declared on `text_area` as on `input`) | rule |
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
| GN-1.61 | `checkbox.body.padding @ variant with_label` | rule |
| GN-1.61 | `checkbox.values.label_gap` | field |
| GN-1.61 | `radio.body.padding, radio.body.corners, radio.body.focus, radio.values.label_gap` | field |
| GN-1.61 | `radio.body.padding @ variant with_label` (the `.text-button` padding) | rule |
| GN-1.62 | `checkbox.box.{corners,min_width,min_height} @ variant selection (the check's padding counted into its box)` | rule |
| GN-1.62 | `checkbox.body.corners @ variant selection` | rule |
| GN-1.62 | `checkbox.values.label_gap @ variant selection` | rule |
| GN-1.63 | `radio.body.corners (GTK's grouped checkbutton is the radio)` | field |
| GN-1.64 | `checkbox.box.transitions (Fill, Stroke); radio.ring.transitions` | field |
| GN-1.64 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.65 | `checkbox.box.stroke @ cond contrast_high / {hover} !{checked,indeterminate} cond contrast_high` | rule |
| GN-1.65 | `checkbox.box.fill @ {pressed} !{checked,indeterminate} cond contrast_high` | rule |
| GN-1.65 | `radio.ring.stroke @ cond contrast_high / {hover} !{checked} cond contrast_high` | rule |
| GN-1.65 | `radio.ring.fill @ {pressed} !{checked} cond contrast_high` | rule |
| GN-1.66 | `menu.item.fill @ {pressed}` | rule |
| GN-1.67 | `menu.item.fill @ {focused} / {hover}` (GTK's `:selected` model button, the keyboard- or pointer-highlighted item, drawn as hover: `focused` or `hover`, never `selected`, S7.2) | rule |
| GN-1.68 | `menu.item.corners` | field |
| GN-1.69 | `menu.body.padding` | field |
| GN-1.70 | `menu.body.min_width` | field |
| GN-1.70 | `menu.item.min_width` | field |
| GN-1.71 | `menu.separator.padding` | field |
| GN-1.72 | `menu.accelerator.{opacity,padding}` | field |
| GN-1.72 | `menu.values.accelerator_gap_px` | field |
| GN-1.73 | `menu.check_indicator.{min_width,min_height,padding,glyph}` | field |
| GN-1.73 | `menu.check_indicator.margin / menu.check_indicator.margin @ dir rtl` (its margins, the outer one negative, ADD-A79) | rule |
| GN-1.73 | `menu.check_indicator.stroke @ variant radio` | rule |
| GN-1.73 | `menu.check_indicator.stroke @ variant radio cond contrast_high` | rule |
| GN-1.74 | `menu.submenu_arrow.{glyph,opacity,min_width}` | field |
| GN-1.74 | `menu.submenu_arrow.opacity @ cond contrast_high` | rule |
| GN-1.75 | `menu.submenu_header.{glyph,text,padding}` | field |
| GN-1.76 | `menu.values.submenu_presentation (Slide)` | field |
| GN-1.76 | `menu.values.presentations events @submenu_push, @submenu_pop (GtkStack's slide left and right, its default duration)` | field |
| GN-1.77 | `menu.values.submenu_open_delay_ms @ context any menu_bar (nested popover menus only; the base opens at once)` | rule |
| GN-1.77 | `menu.values.submenu_switch (TimerRestartedOnMotion)` | field |
| GN-1.78 | `menu.item.{min_width,min_height,padding,fill} @ variant icon_only / {focused} variant icon_only / {hover} variant icon_only / {focused,pressed} variant icon_only / {hover,pressed} variant icon_only` (GTK's `:selected`: the highlighted item, S7.2) | rule |
| GN-1.78 | `menu.item.{padding,stroke,fill} @ variants {icon_only,circular} / {focused} variants {icon_only,circular} / {hover} variants {icon_only,circular} / {focused,pressed} variants {icon_only,circular} / {hover,pressed} variants {icon_only,circular}` | rule |
| GN-1.78 | `menu.values.icon_row_gap_px` | field |
| GN-1.79 | `list.row.{padding,min_height}, list.row_background.corners @ context nearest {menu, layout} = menu` (a standalone list, the popover's own content) `/ context nearest {menu, layout} = layout context menu.body` (a list inside a menu section) | rule |
| GN-1.79 | `list.row_background.fill @ {hover} context menu.body / {selected,hover} context menu.body / {pressed} context menu.body` | rule |
| GN-1.80 | `menu.item.shadow @ {hover} / {focused} / {pressed} cond contrast_high (inset ring; GTK's `:selected` is `focused`, S7.2)` | rule |
| GN-1.285† | `menu.values.show_item_icons (OnlyWithoutLabel)` | field |
| GN-1.81 | `tooltip.values.icon_text_gap_px` | field |
| GN-1.82 | `tooltip.values.reshow_delay_ms` | field |
| GN-1.82 | `tooltip.values.browse_timeout_ms` | field |
| GN-1.83 | `tooltip.values.placement (GtkTooltip; anchor_grow)` | placement |
| GN-1.283† | `tooltip.body.max_width (= mul(element.char_width, <n>), the data declared on body; ADD-B17)` | field |
| GN-1.84 | `scrollbar.thumb.corners; scrollbar.track.corners` | field |
| GN-1.85 | `scrollbar.thumb.{min_width,min_height} @ cond overlay_scrollbars not_context scrollbar.body {hover}` | rule |
| GN-1.85 | `scrollbar.body.padding @ cond overlay_scrollbars !{hover}` | rule |
| GN-1.85 | `cond overlay_scrollbars` | condition |
| GN-1.86 | `scrollbar.thumb.stroke (outside) @ cond overlay_scrollbars` | rule |
| GN-1.86 | `scrollbar.thumb.stroke @ cond overlay_scrollbars not_context scrollbar.body {hover}` | rule |
| GN-1.86 | `scrollbar.thumb.stroke @ cond overlay_scrollbars cond contrast_high / cond overlay_scrollbars cond contrast_high not_context scrollbar.body {hover}` | rule |
| GN-1.87 | `scrollbar.track.fill @ {hover} cond overlay_scrollbars` | rule |
| GN-1.87 | `scrollbar.track.fill @ {hover} cond overlay_scrollbars cond contrast_high` | rule |
| GN-1.88 | `scrollbar.thumb.fill @ context scrollbar.body {@fine_tune}` | rule |
| GN-1.88 | `scrollbar.thumb.fill @ cond contrast_high context scrollbar.body {@fine_tune}` | rule |
| GN-1.88 | `scrollbar flags @fine_tune; slider flags @fine_tune (adwaita)` | flag |
| GN-1.89 | `scrollbar.thumb.opacity @ {disabled}` | rule |
| GN-1.90 | `scrollbar.track.transitions; scrollbar.thumb.transitions; scrollbar.body.transitions (the trough margins)` | field |
| GN-1.91 | `scrollbar.values.auto_hide_delay` | field |
| GN-1.91 | `scrollbar.values.presentations (Disappear: fade)` | field |
| GN-1.91 | `scrollbar.values.hide_after_leave` | field |
| GN-1.92 | `scrollbar.thumb.{min_width,min_height} @ context any menu` | rule |
| GN-1.93 | `scrollbar.thumb.fill @ cond contrast_high / {hover} cond contrast_high / {pressed} cond contrast_high` | rule |
| GN-1.94 | `slider.track.corners; slider.fill.corners` | field |
| GN-1.95 | `slider.body.padding` | field |
| GN-1.96 | `slider.thumb.stroke (outside ring)` | field |
| GN-1.96 | `slider.thumb.stroke @ cond contrast_high` | rule |
| GN-1.97 | `slider.thumb.shadow` | field |
| GN-1.97 | `slider.thumb.shadow @ {disabled}` | rule |
| GN-1.97 | `slider.thumb.shadow @ variant ticks_before !variant ticks_after / variant ticks_after !variant ticks_before` | rule |
| GN-1.98 | `slider.track.fill @ {hover} / {pressed}` | rule |
| GN-1.98 | `slider.fill.fill.append @ {hover} / {pressed}` | rule |
| GN-1.98 | `slider.thumb.fill @ context any slider {hover}` | rule |
| GN-1.99 | `slider.thumb.fill @ context any slider {pressed}` | rule |
| GN-1.100 | `slider.tick.{min_width,min_height,fill}` | field |
| GN-1.100 | `slider.values.tick_gap` | field |
| GN-1.100 | `slider.tick.min_height @ context slider.body {@fine_tune} / slider.tick.min_width @ context slider.body variant vertical {@fine_tune}` | rule |
| GN-1.101 | `slider.thumb.{corners,transform} @ variant ticks_before !variant ticks_after / variant ticks_after !variant ticks_before / variants {ticks_before,vertical} !variant ticks_after / variants {ticks_after,vertical} !variant ticks_before` | rule |
| GN-1.102 | `slider.value_label.{text,opacity}` | field |
| GN-1.102 | `slider.values.value_label_gap` | field |
| GN-1.102 | `slider.values.value_label_gap @ variant ticks_before / variant ticks_after` | rule |
| GN-1.103 | `slider.fill_level.{fill,corners}` | field |
| GN-1.104 | `slider.thumb.focus` | field |
| GN-1.105 | `slider.thumb.focus.paint @ {disabled}` | rule |
| GN-1.105 | `slider.thumb.focus.paint @ {disabled} cond contrast_high` | rule |
| GN-1.106 | `slider.track.transitions; slider.fill.transitions` | field |
| GN-1.107 | `slider.tick.{min_width,min_height} @ variant vertical (the mark across the vertical track)` | rule |
| GN-1.107 | `slider.values.{tick_gap,value_label_gap} @ variant vertical` | rule |
| GN-1.107 | `slider.values.value_label_position @ variant vertical` | rule |
| GN-1.108 | `slider.track.{fill,stroke} @ cond contrast_high` | rule |
| GN-1.108 | `slider.track.fill @ {hover} cond contrast_high` | rule |
| GN-1.109 | `progress_bar.track.corners; progress_bar.indicator.corners` | field |
| GN-1.110 | `progress_bar.value_label.{text,opacity}` | field |
| GN-1.111 | `progress_bar.track.{min_width,min_height} @ variant vertical` | rule |
| GN-1.111 | `progress_bar.indicator.min_width @ variant vertical` | rule |
| GN-1.112 | `progress_bar.track.{fill,corners,min_height} @ variant @osd / progress_bar.track.min_width @ variants {@osd,vertical}` | rule |
| GN-1.112 | `progress_bar.indicator.{corners,min_height} @ variant @osd / progress_bar.indicator.min_width @ variants {@osd,vertical}` | rule |
| GN-1.112 | `progress_bar.body.{min_width,min_height} @ variant @osd` | rule |
| GN-1.113 | `progress_bar.values.pulse_block_fraction; progress_bar.values.pulse_step` | field |
| GN-1.113 | `progress_bar.values.loop` | field |
| GN-1.114 | `progress_bar.indicator.{min_width,min_height} @ {empty} (the fill node's sizes unset at value zero)` | rule |
| GN-1.114 | `progress_bar flags empty` | flag |
| GN-1.115 | `progress_bar.track.{fill,stroke} @ cond contrast_high` | rule |
| GN-1.116 | `tab.bar.stroke (the page-side edge) @ variant top / variant bottom / variant left / variant right` | rule |
| GN-1.117 | `tab.indicator.fill @ {hover}` (the indicator is a part of the `tab` element) | rule |
| GN-1.118 | `tab.values.indicator_edge @ variant left / variant right` | rule |
| GN-1.119 | `tab.scroll_arrow.{fill,glyph,padding,margin,min_width}` (`margin` negative, ADD-A79) | field |
| GN-1.119 | `tab.scroll_arrow.glyph @ variant left / variant right (the pan glyphs per bar position)` | rule |
| GN-1.119 | `tab.values.overflow` | field |
| GN-1.120 | `button.body.{fill,min_width,min_height,padding,margin} @ variant close context tab.tab` (flat; its margins, one negative, ADD-A79) | rule |
| GN-1.120 | `button.icon.icon.fill @ variant close context tab.tab / {hover} variant close context tab.tab / {pressed} variant close context tab.tab` | rule |
| GN-1.121 | `tab.bar.padding, tab.tab.padding` | field |
| GN-1.121 | `tab.values.tab_gap` | field |
| GN-1.122 | `tab.pane.fill (= the View surface)` | field |
| GN-1.123 | `tab.tab.fill, tab.indicator.fill, tab.icon.icon.fill @ {disabled} (the tab dimmed by the disabled opacity except its label and close button)` | rule |
| GN-1.124 | `tab.tab.fill.append @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} variant document` | rule |
| GN-1.124 | `tab.tab.min_height, tab.tab.padding, tab.tab.corners @ variant document` | rule |
| GN-1.124 | `tab.tab.fill @ pos only variant document` | rule |
| GN-1.125 | `tab.bar.fill, tab.bar.stroke.bottom, tab.bar.padding, tab.bar.min_height, tab.bar.bindings (surface Header) @ variant document` | rule |
| GN-1.125 | `tab.tab_box.padding, tab.start_actions.padding, tab.end_actions.padding @ variant document` | rule |
| GN-1.125 | `tab.bar.fill, tab.tab.opacity, tab.separator.opacity @ cond window_inactive variant document; tab.bar.transitions` | rule |
| GN-1.125 | `tab.bar.{fill,stroke,padding} @ variant @inline; tab.bar.fill, tab.tab.opacity @ variant @inline cond window_inactive (no backdrop change)` | rule |
| GN-1.125 | `tab.bar.{padding,min_height} @ variant @shrunk` | rule |
| GN-1.126 | `tab.separator.{stroke,padding} @ variant document` | rule |
| GN-1.126 | `tab.separator.opacity @ prev tab.tab{selected} / next tab.tab{selected} (ADD-B07)` | rule |
| GN-1.126 | `tab.separator.transitions (opacity)` | field |
| GN-1.127 | `tab.drop_indicator.{fill,min_width,padding}` | field |
| GN-1.127 | `tab.drop_indicator.fill @ cond contrast_high` | rule |
| GN-1.128 | `tab.attention_indicator.{fill,transform,min_width,max_width,min_height} @ {@needs_attention}` | rule |
| GN-1.128 | `tab.values.attention_width_fraction` | field |
| GN-1.128 | `tab.values.presentations event @attention` | field |
| GN-1.128 | `tab.attention_indicator.min_width @ variant @pinned` | rule |
| GN-1.129 | `tab.values.base_width_px; tab.tab.max_width` | field |
| GN-1.129 | `tab.values.pinned_width_px @ variant @pinned` | rule |
| GN-1.129 | `tab.label.text.elision` (Fade, at both edges; `under: FadeUnder::Element { widget: button, variant: close }`, the close button, ADD-A89; the row gives two fade lengths, and `Elision::Fade` holds one with no edge: a needed core change) | field |
| GN-1.130 | `button.body.{fill,min_width,min_height,corners,padding} @ variant close context tab.tab variant document` | rule |
| GN-1.130 | `tab.values.close_button_shown @ variant document` | rule |
| GN-1.130 | `button.values.presentations @ variant close context tab.tab variant document (Appear, Disappear: fade)` | rule |
| GN-1.131 | `tab.values.presentations events Appear (open), Disappear (close), Move (a reordered tab), @scroll, @resize` | field |
| GN-1.131 | `tab.tab.transitions (fill)` | field |
| GN-1.132 | `tab.values.drag_hover_delay_ms` | field |
| GN-1.133 | `tab.tab.{fill,shadow} @ {dragged} (its drag surface's margin only leaves room for the shadow)` | rule |
| GN-1.133 | `tab.label.text.fill @ {dragged}` | rule |
| GN-1.133 | `tab.tab.stroke @ {dragged} cond contrast_high` | rule |
| GN-1.134 | `tab.tab.focus @ {focus_within,focus_visible} variant document` | rule |
| GN-1.134 | `tab.tab.focus (offset) @ {selected} !variant document context tab.bar {focused,focus_visible}` | rule |
| GN-1.135 | `tab.tab.stroke @ {hover} / {pressed} / {selected} cond contrast_high variant document` | rule |
| GN-1.136 | `role.sidebar.background @ cond window_inactive` | role |
| GN-1.136 | `sidebar.pane.transitions (fill)` | field |
| GN-1.137 | `role.secondary_sidebar.background, foreground, border, shade (ADD-A48)` | role |
| GN-1.137 | `role.secondary_sidebar.background @ cond window_inactive` | role |
| GN-1.137 | `sidebar.pane.bindings (surface SecondarySidebar) @ variant @secondary` | rule |
| GN-1.138 | `role.sidebar.shade` | role |
| GN-1.139 | `sidebar.pane.stroke (the content-side edge) @ variant end / dir rtl / variant end dir rtl` | rule |
| GN-1.140 | `list.row_background.corners, list.row.min_height, list.row.padding @ variant sidebar` | rule |
| GN-1.140 | `list.values.row_background_inset, list.values.leading_gap @ variant sidebar; list.view.padding @ variant sidebar` (the row's own padding is `list.row.padding`, AdwSidebar's box padding the view's: two parts, one variant) | rule |
| GN-1.141 | `list.row_background.fill.append @ {hover} / {pressed} / {selected} / {selected,hover} / {selected,pressed} / {context_menu_open} variant sidebar` | rule |
| GN-1.142 | `list.section_header.padding @ variant sidebar / variant sidebar pos first` | rule |
| GN-1.142 | `separator.line.margin @ context sidebar.pane` | rule |
| GN-1.143 | `list.row_background.corners, list.row.padding, list.values.row_background_inset @ variants {sidebar,grid}` | rule |
| GN-1.144 | `list.values.leading_gap_px, list.values.trailing_gap_px @ variant sidebar (the prefix and suffix areas' margins)` | rule |
| GN-1.145 | `sidebar.values.drag_hover_delay_ms` | field |
| GN-1.146 | `banner.body.{fill,stroke} @ context sidebar.pane / context sidebar.pane variant end / dir rtl context sidebar.pane / dir rtl context sidebar.pane variant end` | rule |
| GN-1.146 | `banner.body.fill @ cond window_inactive context sidebar.pane; banner.{title,message}.text.fill @ context sidebar.pane` | rule |
| GN-1.147 | `sidebar.values.width_px, sidebar.pane.min_width, sidebar.pane.max_width, sidebar.values.width_fraction (the lengths in libadwaita's sp: each = mul(<length>, text_scale), ADD-A55)` | field |
| GN-1.148 | `list.row_background.stroke @ {selected} / {hover} / {pressed} cond contrast_high variant sidebar` | rule |
| GN-1.149 | `toolbar.bar.padding, toolbar.bar.corners, toolbar.bar.bindings (surface Osd) @ variant @osd` | rule |
| GN-1.150 | `toolbar.bar.padding @ variant @collapse_spacing` | rule |
| GN-1.151 | `button.body.{fill,shadow} @ variant icon_only !variant @raised !variant primary !variant destructive !variant @opaque context nearest {toolbar, popover} = toolbar.bar / variant with_icon !variant @raised !variant primary !variant destructive !variant @opaque context nearest {toolbar, popover} = toolbar.bar / variant @arrow_only !variant @raised !variant primary !variant destructive !variant @opaque context nearest {toolbar, popover} = toolbar.bar (a popover between them restores the raised look)` | rule |
| GN-1.151 | `split_button.{primary,secondary}.fill, split_button.divider.opacity @ !variant primary !variant destructive !variant @opaque !variant @raised context nearest {toolbar, popover} = toolbar.bar` (the flat split button; `.raised` is `@raised`) | rule |
| GN-1.151 | `combo_box.body.{fill,shadow} @ !variant @raised context nearest {toolbar, popover} = toolbar.bar` (the flat drop-down; `.raised` is `@raised`) | rule |
| GN-1.152 | `switch.body.padding @ context toolbar.bar` | rule |
| GN-1.153 | `toolbar.bar.{fill,shadow,bindings} @ variants {@raised,top} / variants {@raised,bottom}` | rule |
| GN-1.153 | `toolbar.bar.shadow @ variants {@raised,top} cond contrast_high / variants {@raised,bottom} cond contrast_high` | rule |
| GN-1.154 | `toolbar.bar.stroke (one edge), toolbar.bar.shadow (Absent) @ variants {@raised,@border,top} / variants {@raised,@border,bottom}` | rule |
| GN-1.155 | `toolbar.bar.fill @ cond window_inactive variant @raised` | rule |
| GN-1.155 | `toolbar.content.{opacity,transitions} @ cond window_inactive variant @raised` (the bar's content dimmed as one group) | rule |
| GN-1.155 | `toolbar.bar.transitions` | field |
| GN-1.156 | `scroll_view.edge_effect.{fill,stroke} @ variant top {scrolled} / variant bottom {scrolled} (the content under an AdwToolbarView's top or bottom bar: the undershoot at that edge, its own element)` | rule |
| GN-1.157 | `toolbar.bar.fill, toolbar.bar.stroke.top, toolbar.bar.padding @ variant bottom (GtkActionBar, AdwToolbarView's bottom bar)` | rule |
| GN-1.157 | `toolbar.values.item_gap @ variant bottom` | rule |
| GN-1.157 | `toolbar.bar.fill @ cond window_inactive variant bottom` | rule |
| GN-1.157 | `toolbar.content.opacity @ cond window_inactive variant bottom` (the bar's content, as one group) | rule |
| GN-1.158 | `list.row_background.fill.append @ {pressed}` | rule |
| GN-1.159 | `list.row_background.fill @ {selected,hover} / {selected,pressed}` | rule |
| GN-1.160 | `list.row_background.fill @ {context_menu_open} / {selected,context_menu_open}` | rule |
| GN-1.161 | `list.row_background.focus (offset inset)` | field |
| GN-1.162 | `list.row_separator.stroke @ variant row_separators` | rule |
| GN-1.162 | `list.row_separator.stroke @ variants {row_separators,horizontal}` (on the left edge in a horizontal list) | rule |
| GN-1.163 | `list.section_header.text, list.section_header.padding` | field |
| GN-1.163 | `list.section_header.padding @ variant @rich_list` | rule |
| GN-1.164 | `list.row.padding, list.row.min_height, list.values.leading_gap @ variant @rich_list` | rule |
| GN-1.165 | `list.view.{fill,corners,shadow,stroke} @ variant inset` | rule |
| GN-1.165 | `list.row_separator.stroke @ variant inset` | rule |
| GN-1.165 | `list.row_background.fill.append @ {hover} / {pressed} variant inset` | rule |
| GN-1.165 | `list.row_background.corners @ pos first variant inset / pos last variant inset / pos only variant inset` | rule |
| GN-1.165 | `list.row_background.focus @ variant inset` | rule |
| GN-1.165 | `list.row_background.transitions @ variant inset` | rule |
| GN-1.165 | `form.card.{fill,corners,shadow,stroke}; form.row_separator.stroke; form.row.fill.append @ {hover} / {pressed}; form.row.corners @ pos first / pos last / pos only; form.row.{focus,transitions}` (an AdwPreferencesGroup's boxed list: the same source on the form's own parts, R20) | rule |
| GN-1.166 | `list.row_background.{fill,corners,shadow} @ variant separated` | rule |
| GN-1.166 | `list.row_background.fill.append @ {hover} / {pressed} / {selected} variant separated` | rule |
| GN-1.166 | `list.row_background.shadow @ {drop_target} variant separated` | rule |
| GN-1.166 | `list.values.row_gap @ variant separated` | rule |
| GN-1.167 | `list.header_section.text.fill @ {hover} / {pressed} / cond contrast_high / {hover} cond contrast_high` | rule |
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
| GN-1.178 | `scroll_view.edge_effect.{fill,stroke} @ variant top {scrolled} context list variant table (under the column view's header)` | rule |
| GN-1.279† | `list.branch_line.stroke (dash, ADD-A32), list.row_separator.stroke, list.column_separator.stroke (dash, ADD-A32) @ variant @tree_view / variant @tree_view cond contrast_high` | rule |
| GN-1.279† | `list.values.indent_per_level, list.values.indent_includes_disclosure @ variant @tree_view` | rule |
| GN-1.279† | `list.disclosure.glyph @ variant @tree_view / {hover} variant @tree_view / {pressed} variant @tree_view (the glyph's fill at a reduced alpha at rest)` | rule |
| GN-1.279† | `list.row_background.corners @ {selected} variant @tree_view` | rule |
| GN-1.279† | `list.drop_indicator.stroke; list.row_background.stroke @ {drop_target} variant @tree_view` | rule |
| GN-1.279† | `list.header_section.padding, list.header.stroke.bottom @ variant @tree_view` | rule |
| GN-1.279† | `progress_bar.{track,indicator}.fill @ context list.cell context any list variant @tree_view` | rule |
| GN-1.279† | `input.body.{corners,fill,stroke} @ context list.cell context any list variant @tree_view / {focused} context list.cell context any list variant @tree_view` | rule |
| GN-1.279† | `stepper.body.{corners,fill,stroke} @ context list.cell context any list variant @tree_view / {focus_within} context list.cell context any list variant @tree_view` | rule |
| GN-1.279† | `list.values.typeahead_hide_ms, list.values.drag_hover_delay_ms` | field |
| GN-1.280† | `list.view.fill, list.row_background.focus @ variant @icon_view` | rule |
| GN-1.280† | `list.row_background.stroke @ {selected} cond contrast_high variant @icon_view` | rule |
| GN-1.280† | `list.row_background.stroke @ {drop_target} variant @icon_view` | rule |
| GN-1.280† | `list.row.padding, list.view.padding (the view's margin), list.values.intercell_gap (row and column spacing), list.values.leading_gap_px (the icon-to-text spacing) @ variant @icon_view` | rule |
| GN-1.280† | `rubber_band.body.{fill,stroke} @ context any list variant @icon_view` | rule |
| GN-1.179 | `popover.body.shadow (three layers)` | field |
| GN-1.180 | `popover.arrow.{min_width,min_height,fill,stroke}` | field |
| GN-1.180 | `popover.arrow.stroke @ cond contrast_high` | rule |
| GN-1.180 | `popover.values.has_arrow` | field |
| GN-1.181 | `popover.values.placement (GtkPopover gap_px)` | placement |
| GN-1.182 | `role.popover.shade` | role |
| GN-1.183 | `cond not_composited (ADD-A01)` | condition |
| GN-1.183 | `popover.body.{corners,stroke,shadow} @ cond not_composited` | rule |
| GN-1.184 | `popover.body.{fill,shadow,bindings} @ variant @osd` | rule |
| GN-1.184 | `input.magnifier.{fill,shadow}; text_area.magnifier.{fill,shadow}` | field |
| GN-1.184 | `edit_menu.body.{fill,shadow} (adwaita's touch-selection popover, the OSD look)` | field |
| GN-1.185 | `popover.body.{min_width,max_width} (Auto)` | field |
| GN-1.286† | `edit_menu.body.{fill,shadow,padding}; edit_menu.arrow.{fill,min_width,min_height}` | field |
| GN-1.286† | `button.{body,icon} @ variant icon_only context edit_menu.body context any layout variant @linked (the commands: a linked row of image buttons with the touch icons)` | rule |
| GN-1.286† | `layout.values.item_gap_px @ context edit_menu.body (the content box's spacing)` | rule |
| GN-1.286† | `edit_menu.values.placement (GtkPopover, anchor_grow)` | placement |
| GN-1.186 | `splitter.divider.stroke, splitter.divider.min_width @ variant wide` | rule |
| GN-1.187 | `menu.separator.padding` (the menu's separator item, its margins) | field |
| GN-1.187 | `separator.line.margin @ context sidebar.pane` (the sidebar's separator) | rule |
| GN-1.188 | `separator.line.{stroke,min_width,min_height} @ variant @spacer (stroke Absent)` | rule |
| GN-1.189 | `token.text.keyed.title_3` | token |
| GN-1.189 | `token.text.keyed.title_4` | token |
| GN-1.189 | `token.text.keyed.caption_heading` | token |
| GN-1.189 | `token.text.roles.title2` | token |
| GN-1.189 | `token.text.roles.title3` | token |
| GN-1.189 | `token.text.roles.caption_emphasized` | token |
| GN-1.189 | `token.text.keyed.heading; token.text.keyed.body (.heading against .body); token.text.roles.headline` | token |
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
| GN-1.197 | `window.values.breakpoints` (the header bar's mobile threshold: measured Window, height Below it, setting `@mobile`; libadwaita 1.10 defines `MOBILE_WINDOW_HEIGHT` without reading it, so the value phase confirms the use first, else the list is empty) | field |
| GN-1.197 | `view_switcher.values.breakpoints` (the preferences dialog's thresholds: measured Allocation, width AtMost, setting `@bar_layout`) | field |
| GN-1.198 | `switch.track.fill @ {pressed} !{checked}` | rule |
| GN-1.199 | `switch.track.fill @ {checked,pressed} (appended layer)` | rule |
| GN-1.200 | `switch.thumb.fill @ {hover} / {pressed}` | rule |
| GN-1.201 | `switch.thumb.shadow` | field |
| GN-1.201 | `switch.thumb.shadow @ {disabled}` | rule |
| GN-1.202 | `switch.on_glyph, switch.off_glyph (parts; glyph ThemeIcon switch-on-symbolic / switch-off-symbolic)` | field |
| GN-1.202 | `switch.on_glyph.glyph, switch.off_glyph.glyph; switch.on_glyph, switch.off_glyph (absent = true) @ !cond differentiate_without_color` (stated in the base, shown under the read `show-status-shapes` setting) | rule |
| GN-1.202 | `switch.on_glyph.glyph.fill, switch.off_glyph.glyph.fill @ cond differentiate_without_color cond contrast_high` | rule |
| GN-1.202 | `reader:gnome:org.gnome.desktop.a11y.interface show-status-shapes` | reader |
| GN-1.202 | `cond differentiate_without_color (GNOME: from show-status-shapes)` | condition |
| GN-1.203 | `switch.thumb.transitions (Geometry: the thumb's slide)` | field |
| GN-1.204 | `switch.track.transitions (Fill)` | field |
| GN-1.204 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.205 | `switch.track.fill @ {checked} context any variant @osd (any widget, R25.9)` | rule |
| GN-1.206 | `switch.track.focus (outer ring, offset)` | field |
| GN-1.207 | `switch.track.fill @ cond contrast_high / {hover} cond contrast_high / {pressed} cond contrast_high` | rule |
| GN-1.208 | `dialog.body.shadow` | field |
| GN-1.208 | `dialog.body.shadow @ cond contrast_high` | rule |
| GN-1.209 | `dialog.body.stroke (inner outline)` | field |
| GN-1.209 | `dialog.body.stroke @ cond contrast_high` | rule |
| GN-1.209 | `dialog.body.stroke @ variant alert cond contrast_high (dropped)` | rule |
| GN-1.210 | `dialog.scrim.fill (= the shade colour with its alpha scaled by a factor: alpha(surface.shade, <factor>))` | field |
| GN-1.211 | `dialog.body.corners @ variant alert` | rule |
| GN-1.212 | `button.body.{min_height,min_width,padding,corners} @ context dialog.action_area variant alert` | rule |
| GN-1.212 | `button.body.{min_width,padding} @ context dialog.action_area variants {alert,@short}` | rule |
| GN-1.213 | `button.{body,label} @ variant primary context dialog.action_area / variant destructive context dialog.action_area` | rule |
| GN-1.214 | `dialog.values.title_message_gap_px @ variant alert` | rule |
| GN-1.214 | `dialog.message.padding @ variant alert` | rule |
| GN-1.214 | `dialog.body.padding @ variant alert` | rule |
| GN-1.215 | `dialog.body.padding @ variants {alert,@short}` | rule |
| GN-1.215 | `dialog.values.{title_message_gap_px,message_actions_gap_px} @ variants {alert,@short}` | rule |
| GN-1.215 | `dialog.action_area.padding @ variants {alert,@short}` | rule |
| GN-1.215 | `dialog.message.padding @ variants {alert,@short}` (the side paddings of the heading, body and child under `.short`, `_message-dialog.scss:127-150`) | rule |
| GN-1.216 | `dialog.values.action_layout.stack_rule (WhenOverflowing)` | field |
| GN-1.216 | `dialog.values.action_layout.stack_rule @ variant @short` | rule |
| GN-1.217 | `dialog.body.max_width @ variants {alert,@wide} (in sp: = mul(<length>, text_scale), ADD-A55)` | rule |
| GN-1.218 | `dialog.values.window_margin (growing with the window)` | field |
| GN-1.219 | `dialog.values.natural_size` | field |
| GN-1.220 | `dialog.values.presentations event Appear (spring; fade with scale, ADD-B10)` | field |
| GN-1.220 | `dialog.values.presentations @ cond reduced_motion (fade only)` | rule |
| GN-1.221 | `dialog.values.presentations event Disappear (clamped spring)` | field |
| GN-1.221 | `dialog.values.presentations @ cond reduced_motion` | rule |
| GN-1.222 | `dialog.body.{padding,stroke,corners} @ variant bottom_sheet` | rule |
| GN-1.222 | `dialog.values.window_margin @ variant bottom_sheet (the side margins)` | rule |
| GN-1.223 | `dialog.message.{corners,padding} @ variant alert {overflowing} (ADD-A13)` | rule |
| GN-1.223 | `scrollbar.body.padding @ context dialog.message variant alert` | rule |
| GN-1.224 | `window.title_bar.{min_height,fill,shadow} @ variant dialog (GtkMessageDialog's window)` | rule |
| GN-1.224 | `dialog.action_area.padding, dialog.values.action_layout.column_gap_px @ variant @message_dialog` | rule |
| GN-1.224 | `button.body.{min_height,padding,corners} @ context dialog.action_area variant @message_dialog` | rule |
| GN-1.284† | `dialog.values.action_layout.{row_order,stacked_order,equal_widths,row_gap_px,column_gap_px}` (`row_order` empty: the responses keep the application's order) | field |
| GN-1.284† | `dialog.values.action_layout @ dir rtl` | rule |
| GN-1.226 | `spinner.track.fill` | field |
| GN-1.227 | `spinner.values.stroke_width = mul(element.width, <fraction>), scaled by values.stroke_weight within the source's bounds (data declared on indicator: data = { indicator = ["width", "height"] }; R25.3)` | field |
| GN-1.227 | `spinner.values.stroke_weight (the icon weight the stroke is scaled by, within the source's bounds)` | field |
| GN-1.227 | `spinner.values.cap (ADD-A34)` | field |
| GN-1.228 | `spinner.body.{min_width,min_height,max_width,max_height}` | field |
| GN-1.229 | `spinner.values.loop` | field |
| GN-1.230 | `spinner.icon.glyph @ variant @gtk_spinner` | rule |
| GN-1.230 | `spinner.icon.glyph @ variant @gtk_spinner cond reduced_motion` | rule |
| GN-1.230 | `spinner.values.loop @ variant @gtk_spinner (the animated SVG's own animation, GN-P3-19†)` | rule |
| GN-1.231 | `spinner.body.opacity @ {disabled}` | rule |
| GN-1.232 | `spinner.{body.min_width,body.min_height,values.stroke_weight} @ context any status_page` | rule |
| GN-1.233 | `combo_box.values.label_arrow_gap` | field |
| GN-1.234 | `menu.body.padding @ context combo_box; menu.item.min_width @ context combo_box` | rule |
| GN-1.234 | `menu.check_indicator.glyph @ context combo_box; menu.values.check_indicator_side @ context combo_box` | rule |
| GN-1.235 | `search_field.body.padding @ context menu.body context any combo_box` | rule |
| GN-1.235 | `scroll_view.edge_effect.{fill,stroke} @ variant top {scrolled} context menu.body context any combo_box (the undershoot under the search entry)` | rule |
| GN-1.236 | `combo_box.body.{fill,stroke,shadow} @ variant flat` | rule |
| GN-1.237 | `combo_box.body.fill @ {open} / {open,hover}` | rule |
| GN-1.238 | `combo_box.body.{padding,min_height} @ variant @gtk_combo_box` | rule |
| GN-1.239 | `segmented_control.track.padding, segmented_control.track.corners` | field |
| GN-1.239 | `segmented_control.segment.corners (Concentric)` | field |
| GN-1.240 | `segmented_control.segment.shadow @ {selected}` | rule |
| GN-1.241 | `segmented_control.segment.fill @ {pressed}` | rule |
| GN-1.242 | `segmented_control.track.fill @ {disabled}` | rule |
| GN-1.242 | `segmented_control.segment.fill @ {selected,disabled}` | rule |
| GN-1.242 | `segmented_control.label.opacity, segmented_control.icon.opacity @ {disabled}` | rule |
| GN-1.243 | `segmented_control.separator.stroke, segmented_control.separator.padding` | field |
| GN-1.243 | `segmented_control.separator.opacity @ prev segmented_control.segment{selected} / next segmented_control.segment{selected}` | rule |
| GN-1.243 | `segmented_control.separator.transitions (Opacity)` | field |
| GN-1.244 | `segmented_control.segment.min_width` | field |
| GN-1.244 | `segmented_control.segment.min_width @ variant icon_only / variant with_icon` | rule |
| GN-1.245 | `segmented_control.track.corners, segmented_control.segment.padding, segmented_control.separator.padding @ variant pill` | rule |
| GN-1.246 | `segmented_control.track.fill @ variant flat` | rule |
| GN-1.246 | `segmented_control.segment.fill @ {selected} / {selected,hover} / {selected,pressed} variant flat` | rule |
| GN-1.247 | `segmented_control.segment.fill, segmented_control.label.text.fill, segmented_control.icon.icon.fill @ {selected} variant @osd / {selected} context any variant @osd (any widget, R25.9)` | rule |
| GN-1.247 | `segmented_control.segment.{shadow,focus} @ {selected} variant @osd !variant flat / {selected} !variant flat context any variant @osd (outer ring, its paint overridden)` | rule |
| GN-1.247 | `segmented_control.segment.{shadow,bindings} @ {selected} variant @osd !variant flat / {selected,disabled} variant @osd !variant flat` (the selected toggle's accent and status roles rebound) | rule |
| GN-1.248 | `segmented_control.segment.min_width @ variant vertical` | rule |
| GN-1.249 | `segmented_control.segment.transitions (Text, Fill, Shadow)` | field |
| GN-1.249 | `token.motion.durations.button, token.motion.timings.ease_out_quad` | token |
| GN-1.250 | `segmented_control.segment.focus (outer, offset)` | field |
| GN-1.250 | `segmented_control.segment.focus.offset @ variant @osd` | rule |
| GN-1.251 | `segmented_control.track.shadow @ cond contrast_high (inset ring)` | rule |
| GN-1.251 | `segmented_control.segment.shadow @ {hover} / {pressed} / {selected} cond contrast_high` | rule |
| GN-1.252 | `card.body.shadow` | field |
| GN-1.252 | `card.body.stroke @ cond contrast_high` | rule |
| GN-1.253 | `card.body.fill.append @ {hover} / {pressed} variant activatable` | rule |
| GN-1.253 | `card.body.focus @ variant activatable` (its `transition`: the focus ring's own motion, S2.10); `card.body.transitions (Fill, Shadow) @ variant activatable` (`$button_transition`, `_misc.scss:229`) | rule |
| GN-1.254 | `card.body.focus.offset` | field |
| GN-1.255 | `role.card.foreground` | role |
| GN-1.255 | `role.card.shade` | role |
| GN-1.256 | `card.body.fill, card.body.shadow, card.body.bindings (Osd) @ variant @osd` | rule |
| GN-1.257 | `form.row_arrow.{glyph,opacity} @ {open} variant expander` (an AdwExpanderRow in a preferences group); `list.disclosure.{glyph,opacity} @ context list.row {open} variant inset` (one in a plain boxed list: the same arrow, the same field kind) | rule |
| GN-1.258 | `form.row_arrow.transform @ {open} variant expander; list.disclosure.transform @ context list.row {open} variant inset` | rule |
| GN-1.258 | `form.row_arrow.transitions (Transform) @ variant expander; list.disclosure.transitions (Transform) @ variant inset` | rule |
| GN-1.259 | `form.values.presentations events @expand, @collapse (queried with the row) @ variant expander; list.values.presentations events @expand, @collapse (queried with the row) @ variant inset` | rule |
| GN-1.260 | `form.row_nested.fill @ variant expander; list.nested.fill @ variant inset` | rule |
| GN-1.261 | `expander.header.corners, expander.header.focus` | field |
| GN-1.261 | `expander.arrow.{glyph,opacity}` | field |
| GN-1.261 | `expander.arrow.glyph @ {open}` | rule |
| GN-1.261 | `expander.arrow.opacity @ {hover} / {pressed}` | rule |
| GN-1.262 | `expander.values.drag_hover_delay_ms` | field |
| GN-1.263 | `expander.arrow.opacity @ {disabled}` | rule |
| GN-1.264 | `link.text.focus (ring, inset; = token.focus)` | field |
| GN-1.265 | `role.osd.link` | role |
| GN-1.265 | `role.osd.link_visited` | role |
| GN-1.265 | `link.text.text.fill (= surface.link)` | field |
| GN-1.265 | `link.text.text.fill @ {visited} (= surface.link_visited)` | rule |
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
| GN-2.01 | `form.row.{min_height,padding} @ variant action (the header's height and margins)` | rule |
| GN-2.01 | `form.values.column_gap_px, form.values.row_gap_px @ variant action (the header's spacing; the title-subtitle spacing)` | rule |
| GN-2.01 | `form.row_prefix.padding, form.row_suffix.padding @ variant action / variant action dir rtl (the title box's margins, the prefixes' and suffixes' spacing)` | rule |
| GN-2.01 | `form.row_subtitle.{text,opacity} @ variant action` | rule |
| GN-2.01 | `form.row_icon.padding @ variant action / variant action dir rtl; form.row_icon.opacity @ {disabled} variant action` | rule |
| GN-2.02 | `form.row_title.{text,opacity} @ variant property` | rule |
| GN-2.02 | `form.row_subtitle.text @ variant property / variants {property,@monospace}` | rule |
| GN-2.03 | `form.row.padding @ variant entry (the header's margins); input.body.padding @ context form.row variant entry (the editable area)` | rule |
| GN-2.03 | `form.row_edit_icon.{min_width,min_height,padding}, form.row_indicator.{min_width,min_height,padding,opacity} @ variant entry; form.row_edit_icon.icon @ {disabled} variant entry` | rule |
| GN-2.03 | `form.row_indicator.opacity @ variant entry cond contrast_high` | rule |
| GN-2.03 | `input.text.text.fill @ {disabled} context form.row variant entry (the text at the disabled opacity)` | rule |
| GN-2.03 | `form.row.{focus,transitions} @ {focus_within} variant entry / {focus_within} variant spin (offset inset, with the row transition)` | rule |
| GN-2.03 | `form.row.bindings, form.row_title.text.fill, form.row_subtitle.opacity @ variants {entry,error} / variants {entry,warning} / variants {entry,success}` (the accent rebound to the status colour) | rule |
| GN-2.03 | `input.text.text @ context form.row variants {entry,@monospace}` | rule |
| GN-2.04 | `form.row_title.text @ variant entry !{empty} / variant entry {focus_within}` | rule |
| GN-2.04 | `form.row_title.transitions (Geometry, Text: the title shrinking)` | field |
| GN-2.05 | `stepper.body.{fill,stroke} @ variant @row (no entry frame)` | rule |
| GN-2.05 | `stepper.increment.{corners,min_width,min_height,padding}, stepper.decrement.{corners,min_width,min_height,padding} @ variant @row (circular, their margins)` | rule |
| GN-2.06 | `form.row_arrow.{icon,padding} @ variant combo` | rule |
| GN-2.06 | `list.view.fill @ context form.row variant combo (the inline list: no fill)` | rule |
| GN-2.06 | `menu.body.min_width @ context form.row variant combo` | rule |
| GN-2.06 | `search_field.body.padding @ context menu.body context form.row variant combo; scroll_view.edge_effect.{fill,stroke} @ variant top {scrolled} context menu.body context form.row variant combo` | rule |
| GN-2.07 | `form.row.{min_height,padding} @ variant switch (the action row's look)` | rule |
| GN-2.07 | `switch.{track,thumb} @ context form.row variant switch (the switch's own rules, unchanged)` | rule |
| GN-2.08 | `form.row.{min_height,padding}, form.values.column_gap_px, form.row_title.text @ variant button` | rule |
| GN-2.08 | `form.row.{fill,focus}, form.row_title.text.fill @ variants {button,primary} / variants {button,destructive}` | rule |
| GN-2.08 | `form.row.fill @ {hover} variants {button,primary} / {pressed} variants {button,primary} / {hover} variants {button,destructive} / {pressed} variants {button,destructive} (appended overlays)` | rule |
| GN-2.09 | `form.row.{min_height,padding} @ variant expander` | rule |
| GN-2.09 | `list.row.{padding,min_height}, list.row_background.corners @ variant inset` (an AdwExpanderRow in a plain boxed list: a `list` row with a `disclosure`, as Material's and iOS's expandable rows; in a preferences group a `form.row` of kind `expander`; its arrow and nested box: GN-1.257–GN-1.260) | rule |
| GN-2.10 | `form.values.group_gap_px, form.group_title.padding, form.group_title.min_height, form.page.padding, form.values.page_gap_px` | field |
| GN-2.10 | `view_switcher.values.breakpoints (the preferences dialog's two thresholds: measured Allocation, width AtMost each, setting @bar_layout)` | field |
| GN-2.11 | `banner.body.{fill,padding}, banner.values.content_gap_px, banner.values.action_gap_px, banner.values.actions_wrap (BelowWhenNotFitting)` | field |
| GN-2.11 | `button.body.min_width @ context banner.action_area / context banner.action_area[vertical]; banner.values.{content_gap_px,action_gap_px} @ variant vertical` (one width and spacing side by side, others stacked; the connector sets `vertical`) | rule |
| GN-2.11 | `banner.body.fill, banner.title.opacity, banner.message.opacity, banner.action_area.opacity @ cond window_inactive` | rule |
| GN-2.11 | `banner.body.{fill,stroke} @ context sidebar.pane` (AdwBanner in a sidebar: a context, as GN-1.146) | rule |
| GN-2.11 | `banner.values.presentations events Appear, Disappear` | field |
| GN-2.11 | `banner.values.{title_alignment,centered_title_gap_px}` (CenterWhenFits); `banner.values.title_alignment @ variant vertical` (Center) (the title centred in the whole banner while it fits with that gap beside the button on both sides, else at the start edge, `adw-banner.c:16`, `:342-347`) | rule |
| GN-2.12 | `toast.body.{fill,shadow,corners,padding,bindings} (the Osd set; accent_standalone rebound), toast.values.content_gap_px, toast.values.natural_width_px, toast.values.durations; toast.values.placement (its edge_margin: the margins)` | field |
| GN-2.12 | `toast.body.stroke @ cond contrast_high` | rule |
| GN-2.12 | `toast.values.presentations events Appear, Disappear (scale and fade), @replace` | field |
| GN-2.13 | `status_page.body.padding, status_page.values.icon_to_title_px, status_page.values.title_to_description_px, status_page.icon.{icon,opacity}, status_page.title.text` | field |
| GN-2.13 | `status_page.body.padding, status_page.values.{icon_to_title_px,title_to_description_px}, status_page.icon.icon, status_page.title.text @ variant @compact` | rule |
| GN-2.13 | `spinner.{body.min_width,body.min_height,values.stroke_weight} @ context any status_page variant loading` | rule |
| GN-2.14 | `avatar.body.{fill,corners} (gradient stops each = pick(name_index(element.text, <count>, adwaita), <c0>, …), ADD-B14; data = { body = ["text", "width"] }), avatar.initials.text (its fill picked the same way)` | field |
| GN-2.14 | `avatar.initials.text.fill @ variant @contrasted` | rule |
| GN-2.14 | `avatar.initials.text.font.size (= an expression on element.width, ADD-B11)` | field |
| GN-2.14 | `avatar.image.{corners,opacity}` | field |
| GN-2.15 | `view_switcher.values.item_gap_px, view_switcher.item.{min_height,min_width}` (the minimum natural widths, by rules on the variants) | field |
| GN-2.15 | `view_switcher.item.padding, view_switcher.item_label.text @ variant narrow` | rule |
| GN-2.15 | `view_switcher.item.padding, view_switcher.values.item_gap_px @ variant @wide` | rule |
| GN-2.15 | `view_switcher.bar.padding @ variant @title` | rule |
| GN-2.15 | `bottom_tab_bar.bar.padding; bottom_tab_bar.item.{padding,min_height}, bottom_tab_bar.item_label.text` (AdwViewSwitcherBar, the bottom bar of a narrow window: its padding and narrow buttons) | field |
| GN-2.15 | `badge.body.{min_width,min_height,corners,padding} @ variant count context any view_switcher variant @sidebar / variant dot context any view_switcher variant @sidebar` | rule |
| GN-2.15 | `view_switcher.values.drag_hover_delay_ms; bottom_tab_bar.values.presentations (Appear, Disappear)` (AdwViewSwitcherBar's reveal) | field |
| GN-2.16 | `view_switcher.{bar,item} @ variant @inline (the toggle group's look restated, GN-1.239–GN-1.251)` | rule |
| GN-2.16 | `view_switcher.item_label.padding, view_switcher.values.icon_text_gap_px @ variant @inline` (the label padding, the image-text spacing) | rule |
| GN-2.16 | `badge.values.{anchor_offset_x_px,anchor_offset_y_px} @ context any view_switcher variant @inline; view_switcher.values.drag_hover_delay_ms @ variant @inline` | rule |
| GN-2.17 | `sheet.scrim.fill (= alpha(surface.shade, <factor>)) @ variant bottom` | rule |
| GN-2.17 | `sheet.body.{shadow,corners,stroke} @ variant bottom / variants {bottom,@flush}` | rule |
| GN-2.17 | `sheet.handle.{fill,opacity,min_width,min_height,padding,corners} @ variant bottom` | rule |
| GN-2.17 | `sheet.bottom_bar.{fill,shadow}` | field |
| GN-2.17 | `sheet.body.transitions (Corners, Shadow)` | field |
| GN-2.17 | `sheet.values.presentations events Appear (spring), Disappear, @bottom_bar_show, @bottom_bar_hide; sheet.values.top_padding_range` | field |
| GN-2.18 | `navigation_view.values.presentations events @push, @pop (spring), navigation_view.page.{transform,opacity}, navigation_view.edge_shadow.{fill,min_width,opacity}, navigation_view.scrim.{fill,opacity}, navigation_view.values.swipe_back, navigation_view.values.swipe_duration_min_ms, navigation_view.values.swipe_duration_max_ms` | field |
| GN-2.18 | `navigation_view.values.presentations @ cond reduced_motion` | rule |
| GN-2.19 | `sidebar.scrim.{fill,opacity}, sidebar.edge_shadow.fill, sidebar.pane.bindings @ variant overlay` | rule |
| GN-2.19 | `sidebar.values.presentations events Appear, Disappear (spring)` | field |
| GN-2.19 | `sidebar.pane.bindings (Surface(Sidebar); the middle pane Surface(SecondarySidebar), GN-1.137)` | field |
| GN-2.20 | `toolbar.bar.{fill,stroke,padding,bindings} @ variant bottom (GtkActionBar, AdwToolbarView's bottom bar: the header-bar fill and fg, the top shade line)` | rule |
| GN-2.20 | `toolbar.bar.stroke @ variant bottom cond contrast_high` | rule |
| GN-2.20 | `toolbar.values.item_gap_px @ variant bottom` | rule |
| GN-2.20 | `toolbar.bar.fill, toolbar.content.opacity @ cond window_inactive variant bottom` (the content, as one group) | rule |
| GN-2.20 | `toolbar.bar.{fill,stroke,padding} @ variants {bottom,@inline} / variants {bottom,@shrunk}; toolbar.bar.fill @ variants {bottom,@inline} cond window_inactive (no backdrop change)` | rule |
| GN-2.21 | `search_field.bar.{fill,stroke,padding}` | field |
| GN-2.21 | `button.body.{fill,min_width,min_height,padding,corners,margin} @ variant close context search_field.bar` (round, flat; its margins, ADD-A79) | rule |
| GN-2.21 | `search_field.bar.fill @ cond window_inactive; button.body.opacity @ cond window_inactive context search_field.bar` | rule |
| GN-2.21 | `search_field.bar.{fill,stroke,padding} @ variant @inline / variant flat / variant @shrunk` | rule |
| GN-2.22 | `input.icon.{icon,opacity} @ variant leading context any search_field / {hover} variant leading context any search_field / {pressed} variant leading context any search_field (the entry icon rules, GN-1.44)` | rule |
| GN-2.22 | `button.icon.icon @ variant clear_field context input context any search_field; button.body.opacity @ variant clear_field context input / {hover} variant clear_field context input / {pressed} variant clear_field context input / variant clear_field context input cond contrast_high / {pressed} variant clear_field context input cond contrast_high` (the entry icon's opacity rules, GN-1.44, on the clear button) | rule |
| GN-2.23 | `button.icon @ variant reveal context input variant secure (the peek icon)` | rule |
| GN-2.23 | `input.caps_lock_indicator.opacity @ variant secure / variant secure cond contrast_high` | rule |
| GN-2.23 | `input.values.mask_reveal_ms` | field |
| GN-2.24 | `stepper.body.{fill,stroke,corners} (the entry frame); input.body.{min_width,padding}, input.text.text @ context stepper (tabular figures)` | rule |
| GN-2.24 | `stepper.increment.{fill,min_width,min_height,corners}, stepper.decrement.{fill,min_width,min_height,corners} (flat, the end radii); stepper.values.button_layout` | field |
| GN-2.24 | `stepper.separator.stroke / stepper.separator.stroke @ cond contrast_high` | rule |
| GN-2.24 | `stepper.values.button_layout, stepper.increment.{min_width,min_height,corners}, stepper.decrement.{min_width,min_height,corners} @ variant vertical; input.body.{min_width,min_height} @ context stepper variant vertical` | rule |
| GN-2.25 | `calendar.frame.stroke, calendar.header.{stroke,padding}, calendar.values.grid_padding, calendar.day.padding, calendar.day_label.text (tabular figures)` | field |
| GN-2.25 | `button.body.{fill,corners} @ variant previous context calendar.header / variant next context calendar.header / context calendar.header (flat square buttons: navigation and title)` | rule |
| GN-2.25 | `calendar.today_marker.stroke @ {today}` | rule |
| GN-2.25 | `calendar.day.{fill,corners}, calendar.day_label.text.fill @ {selected}` | rule |
| GN-2.25 | `calendar.day.fill @ {@marked}` | rule |
| GN-2.25 | `calendar.day.opacity @ {other_month}` | rule |
| GN-2.25 | `calendar.day.focus @ {focus_visible}` | rule |
| GN-2.26 | `level_indicator.track.{fill,corners}, level_indicator.values.thickness_px, level_indicator.values.thresholds` | field |
| GN-2.26 | `level_indicator.fill.{fill,corners} @ variant continuous; level_indicator.block.fill @ {filled} variant discrete / !{filled} variant discrete; level_indicator.block.corners @ pos first variant discrete / pos last variant discrete; level_indicator.values.{block_gap_px,block_min_length_px} @ variant discrete` (filled and empty blocks, the end radii) | rule |
| GN-2.26 | `level_indicator.values.thickness_px @ variant vertical` | rule |
| GN-2.26 | `level_indicator.fill.fill, level_indicator.block.fill @ {@low} / {@high} / {@full}` | rule |
| GN-2.26 | `level_indicator.track.{fill,stroke} @ cond contrast_high` | rule |
| GN-2.26 | `level_indicator.body.opacity @ {disabled}` | rule |
| GN-2.27 | `color_button.body.padding, color_button.swatch.corners, color_button.swatch_ring.corners` | field |
| GN-2.27 | `color_button.swatch_ring.stroke (paint = gtk_contrast_pick(element.color, alpha(role.view.foreground, <fraction>), alpha(role.view.foreground, 0)): the light-swatch ring, GTK's own lightness test, ADD-A94; data = { swatch_ring = ["color"] })` | field |
| GN-2.27 | `color_picker.swatch.corners @ pos first / pos last (the per-edge radii)` | rule |
| GN-2.27 | `color_picker.swatch.focus @ {focus_visible}` | rule |
| GN-2.27 | `color_picker.swatch_ring.stroke @ context color_picker.swatch {drop_target} / cond contrast_high` | rule |
| GN-2.27 | `color_picker.preview.corners (the editor's sample)` | field |
| GN-2.28 | `font_chooser.requester.{fill,stroke,corners,padding,focus}, font_chooser.requester_separator.stroke (hidden), font_chooser.requester_label.text, font_chooser.values.requester_gap_px` | field |
| GN-2.29 | `split_button.primary.{corners,padding}, split_button.secondary.{corners,padding}, split_button.values.part_gap_px (negative overlap), split_button.divider.{stroke,padding}, split_button.values.divider_margin_px` | field |
| GN-2.29 | `split_button.divider.stroke @ variant flat / variant flat cond contrast_high; split_button.divider.opacity @ {hover} variant flat / {focus_visible} variant flat` | rule |
| GN-2.29 | `split_button.primary.fill, split_button.secondary.fill, split_button.divider.stroke @ variant primary / variant destructive / variant @opaque (the inset separator)` | rule |
| GN-2.29 | `split_button.primary.fill, split_button.secondary.fill @ {disabled}` | rule |
| GN-2.30 | `menu_bar.bar.{stroke,padding}, menu_bar.item.{padding,min_height,corners}` | field |
| GN-2.30 | `menu.body.{padding,corners} @ context any menu_bar (the submenu popovers)` | rule |
| GN-2.30 | `menu_bar.item.fill @ {open} / {hover}; menu_bar.item.stroke @ cond contrast_high` | rule |
| GN-2.31 | `banner.body.fill @ variant info / variant @question / variant warning / variant error` | rule |
| GN-2.31 | `banner.edge_line.stroke, banner.body.padding, banner.values.content_gap_px` | field |
| GN-2.31 | `button.body.fill @ {hover} context banner.action_area / {pressed} context banner.action_area` | rule |
| GN-2.31 | `button.body.{fill,min_width,min_height,corners} @ variant close context banner (round, flat)` | rule |
| GN-2.32 | `group_box.body.stroke, group_box.body.corners, group_box.values.title_margin_px` | field |
| GN-2.32 | `scroll_view.frame.stroke @ variant @frame` (the `.frame` style class: the same border on a scrolled window, without the radius or label, `_misc.scss:4-7`; `scroll_view.frame` absent outside it) | rule |
| GN-2.33 | `view_switcher.item.{fill,stroke,padding} @ variant @stack; view_switcher.item.corners @ variant @stack pos first / variant @stack pos last / variant @stack pos only` (linked toggle buttons, the inner corners joined) | rule |
| GN-2.33 | `view_switcher.item.min_width @ variant @stack (text buttons)` | rule |
| GN-2.33 | `badge.body.fill @ variants {dot,attention} context view_switcher.item context any view_switcher variant @stack` (the needs-attention dot on a stack switcher's button) | rule |
| GN-2.34 | `list.row.padding @ variant @stack_sidebar (the row's and its label's padding)` | rule |
| GN-2.34 | `badge.body.{fill,min_width,min_height,corners} @ variants {dot,attention} context list.row context any list variant @stack_sidebar` (the needs-attention dot; the connector draws it while the page needs attention) | rule |
| GN-2.35 | `component.tab_overview.grid.{fill,padding}` | component |
| GN-2.35 | `component.tab_overview.thumbnail.{fill,corners,shadow,stroke,margin,transitions}; component.tab_overview.thumbnail.{fill,stroke} @ variant @pinned` (the pinned thumbnail's colours) | component |
| GN-2.35 | `badge.body.{fill,corners,min_width,min_height} @ variants {dot,attention} context component.tab_overview.thumbnail` (a thumbnail's needs-attention dot) | rule |
| GN-2.35 | `button.body.{fill,min_width,min_height,corners} @ variant close context component.tab_overview / {hover} variant close context component.tab_overview / {pressed} variant close context component.tab_overview / variant @unpin context component.tab_overview / {hover} variant @unpin context component.tab_overview / {pressed} variant @unpin context component.tab_overview` (the close and unpin buttons) | rule |
| GN-2.35 | `button.body.padding @ variant add context component.tab_overview (the new-tab button's margin)` | rule |
| GN-2.35 | `component.tab_overview.drop_target.{fill,stroke,corners}` | component |
| GN-2.35 | `component.tab_overview.values.{grid_gap_px,columns_min,columns_max,thumbnail_width_min_px,thumbnail_width_max_px}` | component |
| GN-2.35 | `component.tab_overview.values.presentations events Appear, Disappear, Move, @thumbnail_fade / @ cond reduced_motion (crossfade)` | component |
| GN-2.36 | `badge.body.{fill,min_width,min_height,corners,padding} @ variant dot / variants {dot,attention}` | rule |
| GN-2.36 | `badge.body.{fill,min_height,padding,corners}, badge.label.text @ variant count; badge.values.cutout_px` | rule |
| GN-2.37 | `badge.label.text @ context button.body[@tab_button] / size small context button.body[@tab_button]` (AdwTabButton's counter) | rule |
| GN-2.37 | `badge.values.{anchor_offset_x_px,anchor_offset_y_px} @ context button.body[@tab_button]` | rule |
| GN-2.38 | `carousel.item.transitions (Geometry, spring: the page settle), carousel.values.scroll_timeout_ms` | field |
| GN-2.38 | `page_indicator.dot.transitions, page_indicator.line.transitions` | field |
| GN-2.39 | `about_dialog.icon.icon.size` | field |
| GN-2.39 | `about_dialog.icon.shadow` | field |
| GN-2.39 | `about_dialog.version.text, about_dialog.version.padding, about_dialog.version.corners` | field |
| GN-2.39 | `about_dialog.body.padding (its margins), about_dialog.values.gap, about_dialog.values.subpage_margins` | field |
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
| GN-2.41 | `shortcuts_dialog.values.page_gap_px` | field |
| GN-2.41 | `button.body.{corners,padding} @ variant pill context shortcuts_dialog.body (the navigation pills)` | rule |
| GN-2.41 | `list.row.{padding,min_height}, list.secondary_text.text @ context shortcuts_dialog.body (dimmed caption subtitles)` | rule |
| GN-2.42 | `input.selection_handle.{fill,min_width,min_height}; text_area.selection_handle.{fill,min_width,min_height}` | field |
| GN-2.42 | `input.selection_handle.corners @ variant start / variant end / variant insertion` | rule |
| GN-2.42 | `text_area.selection_handle.corners @ variant start / variant end / variant insertion` | rule |
| GN-2.42 | `input.selection_handle.padding; text_area.selection_handle.padding` | field |
| GN-2.42 | `input.selection_handle.transform @ variant insertion; text_area.selection_handle.transform @ variant insertion` | rule |
| GN-2.42 | `input.magnifier.fill; text_area.magnifier.fill` | field |
| GN-2.43 | `component.emoji_chooser.body.{fill,padding,bindings}, component.emoji_chooser.cell.{fill,corners,text}, component.emoji_chooser.completion_row.{fill,text,padding}` | component |
| GN-2.43 | `button @ context component.emoji_chooser (the section buttons)` | rule |
| GN-2.44 | `media_controls.play_overlay.{icon,fill,min_height}` | field |
| GN-2.44 | `media_controls.play_overlay.corners` | field |
| GN-2.44 | `media_controls.play_overlay.min_width` | field |
| GN-2.45 | `input.body.{fill,bindings}, input.text.text.fill @ variant editable_label {focused} (view colours)` | rule |
| GN-2.46 | `layout.values.{item_gap,line_gap} @ variant flow (AdwWrapBox, AdwWrapLayout)` | rule |
| GN-2.48 | `file_chooser.body.{padding,bindings}, file_chooser.frame.stroke` | field |
| GN-2.48 | `list @ context any file_chooser (the places rows and the file list and grid, GN-P3-22†, GN-P3-23†); breadcrumb @ context any file_chooser (the path bar, GN-2.52†)` | rule |
| GN-2.48 | `- (the portal's chooser, drawn by Files, GN-P3-21†)` | out (R1.1) |
| GN-2.49 | `drag_image.image.icon.fill` | field |
| GN-2.49 | `drag_image.image.text.fill` | field |
| GN-2.50 | `toast.body.{fill,corners,padding,bindings} @ variant @app_notification (Osd; the fill with a top gradient layer)` | rule |
| GN-2.50 | `toast.values.content_gap_px @ variant @app_notification` | rule |
| GN-2.51 | `wizard.step_list.{fill,bindings}, wizard.step.{padding,corners}` | field |
| GN-2.51 | `wizard.step.fill @ {current}` | rule |
| GN-2.52† | `breadcrumb.crumb.{fill,stroke,corners,padding} (the raised button look), breadcrumb.crumb_icon.icon, breadcrumb.crumb_label.padding` | field |
| GN-2.52† | `breadcrumb.crumb.corners @ pos first / pos last / pos only (linked: the inner corners squared)` | rule |
| GN-2.52† | `breadcrumb.crumb.fill @ {hover} / {pressed} / {current} / {current,hover}` (the current folder's button, GTK's checked toggle) | rule |
| GN-2.52† | `breadcrumb.crumb.padding @ !variant with_icon / variant with_icon; breadcrumb.values.icon_text_gap_px` (the text crumbs' side padding, the icon crumbs', the icon–label spacing) | rule |
| GN-2.52† | `breadcrumb.values.presentations event @scroll (the horizontally scrolling row)` | field |
| GN-2.53† | `color_picker.values.grid_gap_px; color_picker.plane.{min_width,min_height,focus,cursor}; color_picker.preview.corners` | field |
| GN-2.53† | `color_picker.crosshair.stroke @ {focused} / !{focused}` | rule |
| GN-2.53† | `button.body.corners @ variant circular context color_picker.body (the eyedropper); input @ context color_picker.body (the hex entry)` | rule |
| GN-2.53† | `slider @ variant hue context any color_picker / variant alpha context any color_picker (the slider rules, GN-1.94–GN-1.108)` | rule |
| GN-2.53† | `slider.track.fill @ variant hue context any color_picker (the hue gradient) / variant alpha context any color_picker (a Pattern::Checkerboard layer under a linear gradient, ADD-B01)` | rule |
| GN-2.53† | `color_picker.checkerboard.fill (Pattern::Checkerboard, ADD-B01)` | field |
| GN-2.53† | `popover.body.{corners,bindings} @ context any color_picker (the OSD popups)` | rule |
| GN-2.54† | `media_controls.bar.{fill,bindings} (Osd), media_controls.values.gap_px, media_controls.values.auto_hide_ms, media_controls.values.presentations events Appear, Disappear (crossfade)` | field |
| GN-2.54† | `button.body.fill @ variant play context media_controls.bar / variant volume context media_controls.bar (flat); media_controls.time_label.text (tabular figures)` | rule |
| GN-2.54† | `slider @ context media_controls.bar (the seek scale)` | rule |
| GN-2.54† | `popover.body @ context button variant volume (.scale-popup); button.body.fill @ context popover.body context button variant volume (the flat plus and minus buttons); slider.body.min_height @ variant vertical context button variant volume` | rule |
| GN-2.55† | `page_indicator.dot.{fill,corners,min_width,min_height,opacity} (expressions over element.scroll_progress, element.item_index, element.item_count, which dot declares, ADD-B13), page_indicator.values.dot_gap_px, page_indicator.body.padding` | field |
| GN-2.55† | `page_indicator.line.{fill,min_width,min_height,opacity}, page_indicator.values.dot_gap_px, page_indicator.body.padding @ variant @lines (interpolated likewise)` | rule |
| GN-2.56† | `menu.body @ context autocomplete (a plain popover holding a header-less tree view: libadwaita states no rule)` | rule |
| GN-2.56† | `menu.item.fill @ {hover} context autocomplete (hover selection)` | rule |
| GN-2.56† | `autocomplete.values.filter_delay_ms, autocomplete.values.hover_selects` | field |
| GN-2.56† | `autocomplete.values.placement (GtkPopover; width AnchorWidth, max_visible_items, no arrow)` | placement |
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
| GN-3.05 | `navigation_view.edge_shadow.{fill,min_width} (two linear gradients of the shade colour)` | field |
| GN-3.05 | `navigation_view.values.presentations event @push (slide with edge shadow)` | field |
| GN-3.06 | `scroll_view.edge_effect.{fill,stroke} @ variant top {scrolled} / variant bottom {scrolled} / variant start {scrolled} / variant end {scrolled} (the undershoot at each edge independently, each edge its own element with its own scrolled state: shade line and gradient; its border under cond contrast_high)` | rule |
| GN-3.06 | `scroll_view.overshoot.{fill,transitions} @ {@overshoot} (two radial gradients; each edge its own element, drawn only on the edge scrolled past)` | rule |
| GN-3.06 | `token.behavior.scroll.overscroll` | token |
| GN-3.07 | `about_dialog.icon.shadow (.icon-dropshadow)` | field |
| GN-3.07 | `image.picture.shadow @ variant @icon_dropshadow / variant @lowres_icon` (a standalone image with either class) | rule |
| GN-3.08 | `Color.a translucent roles and fills (S2.1, R5.3)` | field |
| GN-3.08 | `PartStyle.fill.append over base (S2.3)` | field |
| GN-3.09 | `role.osd.* (SurfaceSet::Osd)` | role |
| GN-3.09 | `toast.body.bindings Surface(Osd)` | field |
| GN-3.09 | `toast.body.fill` | field |
| GN-3.11 | `declare.widget.*.variants (S10, stacked style classes)` | field |
| GN-3.11 | `Selector.variants SetMatch (S7)` | rule |
| GN-3.12 | `role.global.status.* (fill, foreground, standalone)` | role |
| GN-3.12 | `input.body.bindings @ variant error / variant warning / variant success; form.row.bindings @ variant error / variant warning / variant success` (`Binding::Role` entries rebinding `accent`, `accent_foreground` and `accent_standalone` to that status's `role.global.status.<s>` roles for the subtree, `_labels.scss:21-29`, `_lists.scss:284-292`; standalone labels and icons take the status roles directly, the line above) | rule |
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
| GN-3.24 | `token.behavior.scroll.deceleration (kinetic friction)` | token |
| GN-3.24 | `token.behavior.scroll.overscroll` | token |
| GN-3.24 | `token.behavior.scroll.overscroll_extent (the maximum overshoot, ADD-A29)` | token |
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
| GN-3.30 | `token.behavior.focus_visible_when (FocusVisibleWhen::Keyboard with its timeout_ms)` | token |
| GN-3.30 | `reader:gnome:org.gnome.desktop.a11y.interface keyboard-focus-visible-timeout` | reader |
| GN-3.31 | `token.focus @ cond contrast_high (paint)` | rule |
| GN-3.31 | `cond contrast_high` | condition |
| GN-3.32 | `token.caret.blink_on, token.caret.blink_off (zero while the blink setting is off: never off)` | token |
| GN-3.32 | `reader:gnome:org.gnome.desktop.interface caret blink setting` | reader |
| GN-3.33 | `token.caret.blink_on, token.caret.blink_off` | token |
| GN-3.33 | `reader:gnome:org.gnome.desktop.interface caret blink period` | reader |
| GN-3.34 | `token.caret.blink_timeout_ms` | token |
| GN-3.34 | `reader:gnome:org.gnome.desktop.interface caret blink timeout` | reader |
| GN-3.35 | `token.caret.blink_style Fade (ADD-A24)` | token |
| GN-3.36 | `input.caret.min_width, text_area.caret.min_width (= add(mul(line_height, token.params.caret.<aspect_ratio>), <length>), ADD-A44)` | field |
| GN-3.37 | `token.caret.secondary (the secondary caret's colour; the base of every caret part's secondary caret, ADD-A24)` | token |
| GN-3.37 | `input.secondary_caret.fill, text_area.secondary_caret.fill (= token.caret.secondary), input.secondary_caret.{min_width,min_height}, text_area.secondary_caret.{min_width,min_height}` (GTK's split cursor) | field |
| GN-3.38 | `token.caret.shown_in_read_only (ADD-A70)` | token |
| GN-3.38 | `reader:gnome:org.gnome.desktop.a11y always-show-text-caret` | reader |
| GN-3.39 | `input.caret.fill per widget (caret part, ADD-A24)` | field |
| GN-3.39 | `text_area.drop_caret.fill @ {drop_target}` (the accent on a drop target, `_views.scss:42-44`: the text view's drop-position caret) | rule |
| GN-3.39 | `text_area.caret.fill` | field |
| GN-3.73† | `input.caret, text_area.caret (absent = true) @ {text_selected}` (no caret while text is selected) | rule |
| GN-3.73† | `input.drop_caret.{fill,min_width} @ {drop_target}` (the drop-position caret) | rule |
| GN-3.73† | `input.caret, text_area.caret @ !{focused}` (`absent = true`: no caret outside the focused widget) | rule |
| GN-3.40 | `input.selection.fill @ context input.body !{focus_within}; text_area.selection.fill @ context text_area.body !{focus_within}` (the field's focus, not the selection element's own) | rule |
| GN-3.40 | `role.global.selection_text_background_unfocused (ADD-A49)` | role |
| GN-3.41 | `role.global.selection_text_foreground (Absent: the selected text keeps its own colour)` | role |
| GN-3.41 | `input.selection.text` | field |
| GN-3.42 | `list.row_background.fill @ {selected,hover}` | rule |
| GN-3.42 | `list.row_background.fill @ {selected,pressed}` | rule |
| GN-3.43 | `list.row_background.fill @ {selected} variant sidebar (neutral foreground at an alpha)` | rule |
| GN-3.43 | `menu.item.fill @ {focused} / {hover}` (the neutral selection colour on the highlighted menu item, GTK's `:selected`, S7.2) | rule |
| GN-3.43 | `tab.tab.fill @ {selected}` | rule |
| GN-3.43 | `segmented_control.segment.fill @ {selected} variant flat` | rule |
| GN-3.66† | `role.global.drop_highlight` | role |
| GN-3.66† | `list.row_background.fill @ {drop_target}` | rule |
| GN-3.66† | `list.view.stroke @ {drop_target}` | rule |
| GN-3.66† | `<widget>.<root part>.{stroke,shadow} @ {drop_target} (the catch-all ring on every non-window widget), the widgets named above stating their own (GN-1.25, GN-1.46, GN-1.53, GN-1.279†, GN-1.280†, GN-2.27, GN-2.35)` | rule |
| GN-3.66† | `list.drop_indicator.fill @ context any file_chooser (the places-sidebar insertion row, GN-P3-22†)` | rule |
| GN-3.44 | `window.title_bar.fill, toolbar.bar.fill, search_field.bar.fill, tab.bar.fill, banner.body.fill, sidebar.pane.fill, window.frame.shadow, window.title_bar_content.opacity @ cond window_inactive` | rule |
| GN-3.44 | `PartStyle.transitions (backdrop fade)` | field |
| GN-3.44 | `cond window_inactive` | condition |
| GN-3.46 | `token.pointer.theme, token.pointer.size (ADD-A46)` | token |
| GN-3.46 | `reader:gnome:org.gnome.desktop.interface cursor theme and size` | reader |
| GN-3.67† | `PartStyle.cursor per part (S2.11)` | field |
| GN-3.67† | `input.body.cursor, text_area.body.cursor, link.body.cursor, color_picker.plane.cursor, window.resize_zone.cursor` | field |
| GN-3.67† | `splitter.divider.cursor @ !variant vertical / variant vertical` (column or row resize by orientation) | rule |
| GN-3.67† | `token.behavior.hide_pointer_while_typing` | token |
| GN-3.72† | - | out (R1.1) |
| GN-3.47 | `token.behavior.double_click_time, token.behavior.double_click_distance` | token |
| GN-3.47 | `reader:gnome:org.gnome.desktop.peripherals double-click` | reader |
| GN-3.48 | `token.behavior.drag_threshold` | token |
| GN-3.48 | `reader:gnome:org.gnome.desktop.peripherals drag threshold` | reader |
| GN-3.49 | `token.behavior.long_press (ADD-A45)` | token |
| GN-3.50 | `token.behavior.mnemonics OnAlt delay (ADD-A12)` | token |
| GN-3.51 | `token.behavior.spring_loading (ADD-A45)` | token |
| GN-3.51 | `expander.values.drag_hover_delay_ms, view_switcher.values.drag_hover_delay_ms, sidebar.values.drag_hover_delay_ms, tab.values.drag_hover_delay_ms, component.tab_overview.values.drag_hover_delay_ms` (each `= token.behavior.spring_loading`; the last the tab grid's) | field |
| GN-3.68† | `token.behavior.key_repeat_delay, token.behavior.key_repeat_interval` | token |
| GN-3.68† | `token.behavior.{key_repeat_delay,key_repeat_interval}` (both `Absent` by the reader while the read `repeat` setting is off: keys do not repeat) | token |
| GN-3.68† | `reader:gnome:org.gnome.desktop.peripherals key repeat` | reader |
| GN-3.69† | `stepper.values.repeat_delay_ms, stepper.values.repeat_interval_ms, stepper.values.climb_after_repeats` (the step growing by the climb rate after that many repeats) | field |
| GN-3.69† | `scrollbar.values.repeat_delay_ms, scrollbar.values.repeat_interval_ms (trough paging)` | field |
| GN-3.69† | `slider.values.repeat_delay_ms, slider.values.repeat_interval_ms (trough paging)` | field |
| GN-3.56 | `token.text.rendering.{antialiasing,hinting,subpixel_order} (ADD-B04)` | token |
| GN-3.56 | `token.text.rendering.engine (font-rendering: automatic or manual, keyed; ADD-B04)` | token |
| GN-3.56 | `reader:gnome:org.gnome.desktop.interface font-antialiasing, font-hinting, font-rgba-order, font-rendering` | reader |
| GN-3.57 | `PartStyle.icon.weight per part (S2.9)` | field |
| GN-3.58 | `PartStyle.icon.status_colors (the symbolic palette's success, warning, error; ADD-A37)` | field |
| GN-3.58 | `IconStyle.fill` | field |
| GN-3.58 | `role.global.status.*` | role |
| GN-3.59 | `token.icons.sizes.<contexts>` | token |
| GN-3.60 | `PartStyle.opacity of icon parts @ widget states` | rule |
| GN-3.61 | `PartStyle fill, stroke rules @ cond contrast_high per widget` | rule |
| GN-3.61 | `roles @ cond contrast_high` | rule |
| GN-3.61 | `token.params.opacity.<key> @ cond contrast_high` (Tokens rules restating the border, dim and disabled opacities, S4.6) | rule |
| GN-3.61 | `cond contrast_high` | condition |
| GN-3.63 | `text_scale (the S2.2 operand, ADD-A55)` | field |
| GN-3.63 | `length bases in sp: = mul(<length>, text_scale)` | field |
| GN-3.70† | - | out (R1.1) |
| GN-SF01† | `code_editor.* bases: the Adwaita and Adwaita-dark schemes the adwaita preset carries (ADD-A64)` | field |
| GN-SF01† | `code_editor.* @ variant @classic (GtkSourceView's default scheme when the application sets none; ADD-A64, R25.11)` | rule |
| GN-SF02† | `code_editor.body.fill, code_editor.text.text.fill` | field |
| GN-SF02† | `code_editor.selection.{fill,text} / code_editor.selection.{fill,text} @ context code_editor.body !{focused}` (where a scheme states them; else the text-selection roles, as `text_area`'s base) | rule |
| GN-SF03† | `code_editor.caret.fill` | field |
| GN-SF03† | `code_editor.secondary_caret.fill (its base = token.caret.secondary, GN-3.37; the scheme restates it where it states one)` | field |
| GN-SF04† | `code_editor.current_line.fill` | field |
| GN-SF05† | `code_editor.gutter.fill, code_editor.line_number.text, code_editor.current_line_number.{text,fill}, code_editor.gutter_separator.stroke, code_editor.values.gutter_padding` | field |
| GN-SF06† | `code_editor.invisible.glyph @ variant space / variant tab / variant line_break / variant no_break_space` | rule |
| GN-SF07† | `code_editor.bracket_match.{fill,text}, code_editor.bracket_mismatch.{fill,text} (absent where the scheme states no mismatch style)` | field |
| GN-SF08† | `code_editor.find_match.fill, code_editor.find_match.text.fill` | field |
| GN-SF09† | `code_editor.right_margin.stroke, code_editor.right_margin_area.fill, code_editor.values.right_margin_column` | field |
| GN-SF10† | `code_editor.syntax_span.{text,fill} @ variant comment / variant string / variant function / variant constant / variant preprocessor / variant special_char / variant error / variant warning / variant type / variant decimal / variant base_n / variant float / variant @<role> (the other def: styles and the Markdown roles, keyed) / variant @<language override> (keyed)` | rule |
| GN-SF11† | `code_editor.background_pattern.fill (Pattern::Grid, ADD-B01), code_editor.minimap_slider.fill` | field |
| GN-SF12† | `code_editor.inline_annotation.text @ variant warning / variant error / variant @accent` | rule |
| GN-SF13† | `autocomplete @ context code_editor; menu.body.{corners,shadow,padding} @ context autocomplete context code_editor` | rule |
| GN-SF13† | `menu.item.min_height, menu.item_icon.icon, menu.values.icon_column_width_px, menu.item_subtitle.text (the "before" column at a reduced alpha) @ context autocomplete context code_editor` | rule |
| GN-SF13† | `autocomplete.details_pane.{fill,stroke,corners,padding,text} @ context code_editor` | rule |
| GN-SF13† | `popover.body.{fill,corners,shadow} @ context code_editor; popover.body.fill @ variant warning context code_editor / variant error context code_editor` (the informative popup and its warning and error fills) | rule |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.fill` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.fill @ {drop_target}` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.stroke @ {drop_target}` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.corners` | component |
| GN-SF14† | `component.dock_frame_drop.drop_overlay.transitions` | component |
| GN-SF14† | `component.dock_frame_drop.frame.shadow @ {drop_target}` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.fill` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.{corners,padding} / component.dock_frame_drop.drop_position_button.corners @ variant @center` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.glyph @ variant @top / variant @right / variant @bottom / variant @left / variant @center; component.dock_frame_drop.drop_position_button.glyph @ {drop_target}` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.shadow` | component |
| GN-SF15† | `component.dock_frame_drop.drop_position_button.fill @ {drop_target}` | component |
| GN-SF15† | `component.dock_frame_drop.values.button_gap_px` | component |
| GN-SF16† | `component.dock_frame_drop.switcher_indicator.fill` | component |
| GN-SF17† | `dialog.values.breakpoints` (measured Window: `bottom_sheet` with width AtMost and height AtMost, combine Any; `narrow` width AtMost; `@short` height AtMost; `adw-dialog.c:24-25`, `:2131-2133`) | field |
| GN-SF17† | `dialog.body.{padding,stroke,corners} @ variant bottom_sheet` | rule |
| GN-SF17† | `dialog.body.padding @ variants {alert,narrow} / variants {alert,@short} (GN-1.215)` | rule |
| GN-SF18† | `dialog.values.placement (CenteredInHost)` | placement |
| GN-SF19† | `dialog.values.placement @ variant bottom_sheet (BottomSheet)` | placement |
| GN-SF19† | `dialog.body.min_width @ variant bottom_sheet; dialog.values.window_margin @ variant bottom_sheet (the height capped by the top padding)` | rule |
| GN-SF20† | `dialog.body.{corners,shadow,stroke} @ variant separate_window (AdwDialog's window fallback, .dialog-window); window @ variant dialog (its non-resizable window)` | rule |
| GN-SF20† | `- (the window's placement, the compositor's: GN-SF21†)` | out (R1.1) |
| GN-SF21† | - | out (R1.1) |
| GN-RL01† | `wizard.action_area.padding, wizard.values.button_order, wizard.values.button_gap_px, wizard.values.buttons_equal_width` | field |
| GN-RL01† | `wizard.action_area @ variant @header_bar` (`absent = true`: the buttons packed in the header bar, `button @ context window.title_bar`) | rule |
| GN-RL01† | `button @ context wizard.action_area (text buttons); button.body.fill @ variant primary context any wizard (the default button)` | rule |
| GN-RL01† | `window.caption_buttons @ variant @no_title_buttons (absent; the connector sets it for a wizard in its header-bar mode)` | rule |
| GN-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| GN-FM01† | `reader:gnome:fontconfig emoji family (fc-match emoji)` | reader |
| GN-FM02† | `token.text.fallbacks.families (the font-name family list), token.text.fallbacks.per_language (Pango's per-language order; ADD-A41)` | token |
| GN-FM02† | `reader:gnome:org.gnome.desktop.interface font-name family list (list no longer dropped)` | reader |
| GN-FM03† | `Selector.direction Rtl (S7)` | rule |
| GN-FM03† | `token.icons.mirroring (S2.9)` | token |
| GN-FM03† | `icon lookup by direction, -rtl names (ADD-B05)` | field |
| GN-FM03† | `reader:gnome:gtk_get_locale_direction` | reader |
| GN-P3-1† | - | out (R1.1) |
| GN-P3-2† | - | out (R1.1) |
| GN-P3-3† | `window.values.window_menu_items (GTK's fallback)` | field |
| GN-P3-3† | `window.values.window_menu_placement (GtkPopover at the pointer)` | placement |
| GN-P3-3† | `menu.{body,item}, menu.values.has_arrow @ context window.window_menu` (GTK's arrowless popover menu) | rule |
| GN-P3-3† | `window.values.title_bar_click_actions` | field |
| GN-P3-3† | `reader:gnome:org.gnome.desktop.wm.preferences action-right-click-titlebar` | reader |
| GN-P3-3† | - (the window menu GNOME Shell draws) | out (R1.1) |
| GN-P3-4† | `window.values.caption_button_capabilities` | field |
| GN-P3-4† | `window.caption_button_glyph.glyph @ variant restore` | rule |
| GN-P3-5† | `window.title_bar @ cond full_screen (absent)` | rule |
| GN-P3-5† | `toolbar.values.presentations events Appear, Disappear` (AdwToolbarView's revealed bars) | field |
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
| GN-P3-15† | `scroll_view.values.wheel_step (= gtk_wheel_step(element.height), gtk_wheel_step(element.width): the view's page sizes, ADD-A90)` (a slider's step is its adjustment's page increment, the application's) | field |
| GN-P3-15† | `token.behavior.scroll.touchpad_factor` (ADD-A91) | token |
| GN-P3-16† | `Edge.snap (ADD-A33: libadwaita's one-pixel lines at a fractional scale)` | field |
| GN-P3-16† | - (the display scale itself) | out (R1.1) |
| GN-P3-17† | - | preset (R1.3) |
| GN-P3-18† | - | newest (R1.5) |
| GN-P3-19† | `button.icon.transitions` (PropertyGroup::Icon, TransitionKind::Keyed per GtkSvg state transition: sliders.gpa on the font dialog's toggle; ADD-A27, R25.12) | field |
| GN-P3-19† | `spinner.icon.glyph, spinner.values.loop (the looping process-working.gpa, R25.12)` | field |
| GN-P3-20† | `font_chooser.filter_title.text, font_chooser.family_row_label.text, font_chooser.preview.text, font_chooser.values.row_margins, font_chooser.values.grid_gap_px` | field |
| GN-P3-20† | `input @ context any layout variant @linked context font_chooser.body (the search entry)` | rule |
| GN-P3-20† | `button @ variant menu context font_chooser.body (Filters)` | rule |
| GN-P3-20† | `popover @ context font_chooser.body; checkbox @ context any popover context font_chooser.body` | rule |
| GN-P3-20† | `list @ context font_chooser.body (the framed family and language lists); slider @ context font_chooser.body, stepper @ context font_chooser.body (the Size row); status_page @ context font_chooser.body (No Fonts Found)` | rule |
| GN-P3-21† | - | out (R1.1) |
| GN-P3-22† | `list.row.padding, list.leading_icon.icon (at a reduced alpha), list.values.leading_gap_px @ context any file_chooser / dir rtl context any file_chooser` | rule |
| GN-P3-22† | `button.body.{fill,min_width,min_height,corners} @ variants {flat,circular} context any file_chooser; button.icon.icon @ variants {flat,circular} context any file_chooser / {hover} variants {flat,circular} context any file_chooser / {pressed} variants {flat,circular} context any file_chooser; button.body.margin @ variants {flat,circular} context any file_chooser` (the eject button and its margins, ADD-A79) | rule |
| GN-P3-22† | `spinner @ context list.row context any file_chooser (the busy spinner); separator @ context any file_chooser (between sections)` | rule |
| GN-P3-22† | `list.row.text.fill @ variant @new_bookmark; list.row.min_height, list.row_background.fill @ variant @drop_placeholder` | rule |
| GN-P3-22† | `list.row_background.{fill,stroke} @ {drop_target} context any file_chooser; list.row_background.shadow @ {selected,pressed} context any file_chooser (none)` | rule |
| GN-P3-23† | `file_chooser.frame.stroke (the line below the chooser, outside its box)` | field |
| GN-P3-23† | `button.body.{fill,shadow} @ variant flat context any file_chooser (the view-toggle and new-folder buttons beside the path bar)` | rule |
| GN-P3-23† | `list.cell.padding @ context any file_chooser` | rule |
| GN-P3-23† | `list.view.padding, list.row_background.corners, list.values.row_background_inset, list.row.padding @ variant grid context any file_chooser; scroll_view.edge_effect.{fill,stroke} @ {scrolled} context list variant grid` | rule |
| GN-P3-23† | `button.menu_indicator.{glyph,transform,transitions} @ {open} context any file_chooser` (the server-list arrow's half turn) | rule |
| GN-P3-24† | `layout.values.item_gap, layout.values.line_gap, layout.values.flow_max_per_line @ variant flow` | rule |
| GN-P3-25† | `segmented_control.values.segment_width (Fit)` | field |
| GN-P3-26† | `input.magnifier.{fill,corners,shadow,min_width,min_height}; text_area.magnifier.{fill,corners,shadow,min_width,min_height}` | field |
| GN-P3-26† | `input.values.magnifier_zoom, input.values.magnifier_offset_px, text_area.values.magnifier_zoom, text_area.values.magnifier_offset_px` | field |
| GN-P3-27† | `code_editor.misspelling.text.decoration` | field |
| GN-P4-1† | - | out (R1.1) |
| GN-P4-2† | `popover.values.placement (GtkPopover)` | placement |
| GN-P4-2† | `menu.values.placement (GtkPopover)` | placement |
| GN-P4-2† | `combo_box.values.popup_placement; menu.values.has_arrow @ context combo_box` (GtkPopover; width AtLeastAnchor, start-aligned; `has_arrow` false); `menu.arrow` where a popover menu draws its arrow | placement |
| GN-P4-2† | `button.values.menu_placement` (GtkPopover; side Below, centred, flip Y, slide X: the menu button's default direction, down) | placement |
| GN-P4-2† | `button.values.menu_placement @ variant @arrow_up / variant @arrow_start / variant @arrow_end (the menu button's direction)` | placement |
| GN-P4-3† | - | out (R1.1) |
| GN-P4-4† | `text_area.caret, code_editor.caret (absent = true) @ {overwrite}` (the block caret replaces the line caret, ADD-A23; `input.caret`'s rule is GN-1.49's) | rule |
| GN-P4-4† | `text_area flags overwrite; code_editor flags overwrite (ADD-A23)` | flag |
| GN-P4-4† | `text_area.block_caret.{fill,text} @ {overwrite}, code_editor.block_caret.{fill,text} @ {overwrite}` (the covered character redrawn in the background; ADD-A23, ADD-A24) | rule |
| GN-P4-4† | `text_area.block_caret (absent = true) @ !{overwrite} / !{focused} / {text_selected}, code_editor.block_caret (absent = true) @ !{overwrite} / !{focused} / {text_selected}` (drawn only in overwrite mode while the view has focus and no text is selected) | rule |
| GN-P4-5† | `scrollbar.values.trough_click` | field |
| GN-P4-5† | `slider.values.trough_click` | field |
| GN-P4-6† | `menu.values.submenu_switch` | field |
| GN-P4-6† | `menu.values.submenu_placement (GtkPopover nested: end side, top-aligned)` | placement |
| GN-P4-7† | `checkbox.mark.glyph (GlyphSource::Path from check.svg)` | field |
| GN-P4-7† | `radio.dot.glyph (GlyphSource::Path from bullet.svg)` | field |
| GN-P4-8† | `menu.values.{placement,has_arrow} @ context input / context text_area` (GtkPopover, point anchor; `has_arrow` false) | placement |
| GN-P4-8† | `input.values.context_menu_items, text_area.values.context_menu_items` | field |
| GN-P4-9† | - | out (R1.1) |
