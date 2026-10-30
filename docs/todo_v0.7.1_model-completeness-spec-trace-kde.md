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
(`PartStyle.…`, `FocusIndicator.…`) where the row concerns every part. The selector of a rule follows `@`, in the catalogue's selector notation (its *Selector
notation* table): the element's own variants and states come before the first `context`, a context's
`variant`/`{…}` belong to that ancestor, and ` / ` separates alternatives.
Presentation events are written in parentheses after `values.presentations`: typed `Appear`, `Disappear`, `Move`,
keyed `@event`. A name with no unit suffix and the same name with `_px` or `_ms` are one field
(S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and the contradictions with the
rationale, each decided in the core's additions register (S22).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`, `reference (R1.6)`,
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
| K1.4 | `window.values.caption_button_gap` | field |
| K1.4 | `window.values.caption_spacer_width` | field |
| K1.4 | `reader:kde:breezerc [Windeco] ButtonSize` | reader |
| K1.5 | `window.values.caption_button_layout` | field |
| K1.5 | `reader:kde:kwinrc [org.kde.kdecoration2] ButtonsOnLeft, ButtonsOnRight` | reader |
| K1.6 | `window.caption_button_glyph.glyph (Path per identity: size, stroke: GlyphStroke width and cap) @ variant close / variant maximize / variant restore / variant minimize / variant @help / variant @keep_above / variant @keep_below / variant @shade / variant @on_all_desktops / variant @application_menu / variant @exclude_from_capture` | rule |
| K1.6 | `window.caption_button_glyph.icon (the window icon; fill = the title colour) @ variant window_menu` | rule |
| K1.7 | `window.caption_button_glyph.glyph.fill (= the title text colour), window.caption_button_glyph.fill (Absent at rest)` | field |
| K1.7 | `window.caption_button_glyph.fill, window.caption_button_glyph.glyph.fill @ {hover}` | rule |
| K1.7 | `window.caption_button_glyph.fill @ variant close {hover} (= the Header set's negative foreground of its Inactive group) / variant close {hover} !cond window_inactive (= qt_lighter of it, ADD-A50)` | rule |
| K1.7 | `window.caption_button_glyph.fill @ {pressed} (kcu_mix)` | rule |
| K1.7 | `window.caption_button_glyph.fill @ variant close {pressed} (qt_darker, ADD-A50)` | rule |
| K1.7 | `window.caption_button_glyph.fill @ {checked}` | rule |
| K1.7 | `window.caption_button_glyph.fill @ variant close cond system_setting:outline_close_button` | rule |
| K1.7 | `input.inactive.header.foreground_negative (the Inactive group's stated value, R2.6)` | input |
| K1.7 | `reader:kde:breezerc [Common] OutlineCloseButton -> cond system_setting:outline_close_button` | reader |
| K1.8 | `window.caption_button_glyph.transitions (Fill; duration and timing the decoration's own animation states, value phase)` | field |
| K1.9 | `window.frame.stroke (position Outside; edge snap to the device pixel; paint kcu_mix with token.params.color.frame_contrast)` | field |
| K1.9 | `window.frame.stroke @ cond window_inactive` | rule |
| K1.9 | `window.frame.stroke (Absent) @ cond maximized` | rule |
| K1.9 | `window.frame.stroke (Absent) @ !cond system_setting:outline_enabled` | rule |
| K1.9 | `reader:kde:breezerc [Common] OutlineEnabled -> cond system_setting:outline_enabled` | reader |
| K1.10 | `window.frame.corners (per corner)` | field |
| K1.10 | `window.frame.corners (top corners square) @ cond tiled:top / cond maximized` | rule |
| K1.10 | `window.frame.corners (bottom corners rounded) @ cond system_setting:rounded_corners / !cond system_setting:no_borders` | rule |
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
| K1.15 | `window.title_bar (absent = true) @ cond system_setting:hide_title_bar` | rule |
| K1.15 | `window.{title_bar_content,title,subtitle,title_icon,caption_buttons,caption_button,caption_button_glyph,window_menu} (absent = true) @ cond system_setting:hide_title_bar` (the parts nested in the title bar, each by its own rule: Nesting) | rule |
| K1.15 | `window.frame.padding @ cond maximized cond system_setting:draw_border_on_maximized` | rule |
| K1.15 | `reader:kde:breezerc [Windeco] HideTitleBar, DrawBorderOnMaximizedWindows` | reader |
| K1.16 | `window.frame.corners, window.frame.padding, window.frame.stroke, window.frame.shadow @ cond maximized / cond tiled:top / cond tiled:right / cond tiled:bottom / cond tiled:left` | rule |
| K1.16 | `cond shaded (ADD-A02)` | condition |
| K1.16 | `window.frame.corners, window.frame.padding, window.frame.stroke, window.frame.shadow @ cond shaded` | rule |
| K1.17 | `window.frame.transitions (Shadow; = token.motion.durations.decoration_shadow; = token.motion.timings.out_cubic both ways: the activation runs that curve forward, `breeze@6.8.2 kdecoration/breezedecoration.cpp:197`, `:279`, and the deactivation runs the in-cubic curve backward, `:278-279`, Qt applying the curve to currentTime over duration, `qtbase@6.12.0 src/corelib/animation/qvariantanimation.cpp:200-202`, which in S2.7's forward reading is the out-cubic curve)` | field |
| K1.17 | `token.motion.durations.decoration_shadow (= floor(mul(<the decoration's base>, token.params.motion.kwin_speed_factor)), the factor the decoration reads: `[KDE] AnimationDurationFactor` from KWin's main configuration, kwinrc, over kdeglobals, `KSharedConfig::openConfig()` inside KWin, which opens `<application name>rc` with kdeglobals included (`FullConfig`), KWin naming itself `kwin`, `breeze@6.8.2 kdecoration/breezedecoration.cpp:379-380`, `kconfig@6.31.0 src/core/ksharedconfig.h:84`, `src/core/kconfig.h:94-100`, `src/core/kconfig.cpp:643-649`, `kwin@6.8.2 src/main.cpp:139`, `:155` (ADD-A123); S4.1's rule naming it after Breeze's Decoration class: the product, a float, truncated to whole ms by `QVariantAnimation::setDuration(int)`, `breeze@6.8.2 kdecoration/breezedecoration.cpp:389`, `qtbase@6.12.0 src/corelib/animation/qvariantanimation.h:51`; ADD-A122)` | token |
| K1.17 | `token.params.motion.kwin_speed_factor (the decoration's and KWin's factor: kwinrc's `[KDE] AnimationDurationFactor` where it states one, else kdeglobals', as KConfig cascades KWin's main configuration over kdeglobals; ADD-A123)` | token |
| K1.17 | `reader:kde:kwinrc [KDE] AnimationDurationFactor` | reader |
| K1.17 | `window.title.transitions (Text cross-fade; duration as the decoration states it, value phase)` | field |
| K1.18 | `window.frame.shadow @ cond window_inactive (layers at half strength)` | rule |
| K1.19† | - | out (R1.1) |
| K1.20 | `button.body.stroke @ {hover} / {focus_visible} / {pressed}` | rule |
| K1.20 | `button.body.stroke @ {checked}` | rule |
| K1.20 | `button.body.stroke @ {default}` | rule |
| K1.21 | `button flags focus_visible` | flag |
| K1.21 | `button.values.focus_visible_when (FocusVisibleWhen::Keyboard { timeout: None }: after a keyboard focus change; no timeout)` | field |
| K1.22 | `button.body.fill, button.body.stroke @ {default} !cond window_inactive` | rule |
| K1.23 | `button.body.shadow @ !{pressed,checked,disabled} !variant flat !cond window_inactive` | rule |
| K1.24 | `button.body.fill (none at rest) @ variant flat / {pressed} variant flat / {checked} variant flat / {default} variant flat` | rule |
| K1.24 | `button.body.stroke @ {checked} variant flat / {hover} variant flat` | rule |
| K1.24 | `button.label.text.fill @ variant flat (= role.window.foreground)` | rule |
| K1.25 | `button.body.fill @ {checked} variant warning` | rule |
| K1.25 | `button.body.stroke @ variant warning` | rule |
| K1.26 | `button.body.corners @ variant circular (Full)` (exact for a square box; Breeze draws a non-square round button as an ellipse, a radius of half the larger side that Qt clamps per axis, `breezehelper.cpp:727`, `qpainterpath.cpp:3457-3470`, which circular corners cannot state: the ledger's queue #48) | rule |
| K1.27 | `button.menu_indicator (part: min_width, glyph Path, fill; margin, `Inline`'s margin in the row, S2.11's: the arrow's box at the right of the label's contents rect, which Breeze sizes with the arrow inside its margins, `breeze@6.8.2 kstyle/breezestyle.cpp:3546-3556`, `:5365-5371`, the value phase stating the start and end margins that move it there; ADD-A123)` | field |
| K1.27 | `button.values.menu_indicator_gap` | field |
| K1.28 | `button.icon.icon.size (= token.icons.sizes.small.size)` | field |
| K1.28 | `button.values.shows_icon` | field |
| K1.28 | `reader:kde:kdeglobals [KDE] ShowIconsOnPushButtons` | reader |
| K1.29 | `button.body.fill, button.body.stroke @ {checked,hover}` | rule |
| K1.29 | `button.body.fill @ {checked,pressed}` | rule |
| K1.29 | `button.body.fill, button.body.stroke @ {default,hover}` | rule |
| K1.30 | `button.body.transitions (Fill, Stroke)` | field |
| K1.30 | `token.motion.durations.animations, token.motion.speed_factor` | token |
| K1.40 | `input.body.stroke @ variant warning` | rule |
| K1.40 | `input.body.stroke (= qt_lighter of it, ADD-A50) @ variant warning {hover} / variant warning {focused}` | rule |
| K1.41 | `input.body.stroke, input.body.shadow (Absent; only the fill stays) @ variant flat (the connector also sets it where the field is too short for frame and font)` | rule |
| K1.42 | `input.body.stroke (only the chosen edges) @ variant @border_top / variant @border_right / variant @border_bottom / variant @border_left` | rule |
| K1.42 | `input.body.stroke (position Inside) @ variant @border_top {hover} / variant @border_top {focused} / variant @border_right {hover} / variant @border_right {focused} / variant @border_bottom {hover} / variant @border_bottom {focused} / variant @border_left {hover} / variant @border_left {focused}` | rule |
| K1.44 | `input.body.transitions (Stroke: the hover-to-focus cross-fade)` | field |
| K1.45 | `role.selection.background; role.selection.foreground @ cond window_inactive` | role |
| K1.45 | `input.selection.{fill,text.fill}` | field |
| K1.46† | `button.icon.icon.size (= token.icons.sizes.small.size), button.icon.glyph (ThemeIcon) @ variant clear_field context input` | rule |
| K1.46† | `button.body.min_width, button.body.min_height, button.body.padding @ variant clear_field context input` | rule |
| K1.46† | `button.icon.icon.state_effect @ variant clear_field context input {disabled} / variant clear_field {pressed} context input` (the field disabled; the button pressed) | rule |
| K1.46† | `button.values.presentations (Appear, Disappear: Fade; = token.motion.durations.line_edit_icon_button) @ variant clear_field context input` | rule |
| K1.46† | `input.values.clear_button_visibility` | field |
| K1.46† | `input flags empty` | flag |
| K1.47† | `input.block_caret.fill, input.block_caret.text (the character under the caret) @ {overwrite} (an input mask's caret; the connector sets overwrite on a masked field without selection)` | rule |
| K1.47† | `text_area.block_caret.fill @ {overwrite} (one character's advance wide)` | rule |
| K1.47† | `input.caret, text_area.caret (absent = true) @ {overwrite}` (the block caret replaces the line caret, ADD-A23) | rule |
| K1.48† | `input.values.mask_character, input.values.mask_reveal` | field |
| K1.48† | `button.icon.glyph (ThemeIcon with its fallback) @ variant reveal context input` | rule |
| K1.48† | `input.values.reveal_button_visibility (while the field has text or shows the password)` | field |
| K1.49† | `text_area.misspelling.text.decoration (line, style, paint: a stated colour, not a role; thickness and offset = widget.text.font_underline_thickness, widget.text.font_underline_offset: Qt draws the line from the font's metrics, S2.8); input.misspelling.text.decoration` | field |
| K1.50 | `checkbox.mark.glyph @ {indeterminate} (three dots, Path)` | rule |
| K1.51 | `checkbox.box.fill @ {pressed}; radio.ring.fill @ {pressed} (= qt_darker(..))` | rule |
| K1.52 | `checkbox.box.shadow @ {hover} (inset ring layer); radio.ring.shadow @ {hover}` | rule |
| K1.52 | `checkbox.box.shadow @ {hover} variant warning; radio.ring.shadow @ {hover} variant warning` | rule |
| K1.52 | `checkbox.box.transitions (Shadow); radio.ring.transitions` | field |
| K1.53 | `checkbox.values.indicator_cell` | field |
| K1.53 | `checkbox.box.min_width, checkbox.box.min_height, checkbox.box.corners` | field |
| K1.54 | `checkbox.box.fill, checkbox.box.stroke @ {checked,hover} / {checked,pressed}` | rule |
| K1.54 | `checkbox.box.stroke, checkbox.mark.glyph @ {indeterminate,hover}` | rule |
| K1.54 | `checkbox.box.stroke @ {checked,disabled} (the Disabled set's highlight)` | rule |
| K1.55 | `checkbox.mark.glyph.draw_extent @ {checked} / {indeterminate}; checkbox.mark.transitions (Glyph)` | rule |
| K1.55 | `radio.dot.transform @ {checked}; radio.dot.transitions (Transform)` | rule |
| K1.55 | `checkbox.box.transitions (Fill); radio.ring.transitions` | field |
| K1.55 | `token.motion.durations.animations, token.motion.speed_factor` | token |
| K1.56 | `checkbox.box.stroke, checkbox.box.fill @ {checked} variant warning / radio.ring.stroke, radio.ring.fill @ {checked} variant warning` | rule |
| K1.57 | `checkbox.box.focus; radio.ring.focus` | field |
| K1.58† | `checkbox.mark.glyph (Path, stroke: GlyphStroke width and join)` | field |
| K1.58† | `checkbox.mark.glyph @ {indeterminate} (three dots and their spacing, Path)` | rule |
| K1.60 | `menu.item.{fill,stroke,corners} @ {hover} cond system_setting:menu_item_strong_focus / {focused} cond system_setting:menu_item_strong_focus` (Breeze fills the active item only under strong focus, `breezestyle.cpp:6016`, and draws the weak-focus line otherwise, K1.62; Breeze's `State_Selected` on QMenu's active action, which the pointer and the keys set: `hover` or `focused`, S7.2) | rule |
| K1.61 | `menu.item.fill @ {pressed} cond system_setting:menu_item_strong_focus` | rule |
| K1.61 | `menu.item_label.text.fill @ {pressed} cond system_setting:menu_item_strong_focus` | rule |
| K1.61 | `menu.item_icon.icon.state_effect @ {pressed} cond system_setting:menu_item_strong_focus (Selected mode)` | rule |
| K1.61 | `menu.submenu_arrow.glyph.fill @ {pressed} cond system_setting:menu_item_strong_focus` | rule |
| K1.62 | `menu.item_label.stroke (the focus line under the text's bounding box: bottom edge, Outside, its outset the gap, breezehelper.cpp:406-421) @ {hover} / {pressed} !cond system_setting:menu_item_strong_focus` | rule |
| K1.62 | `reader:kde:breezerc [Style] MenuItemDrawStrongFocus` | reader |
| K1.63 | `menu.body.padding (highlight inset)` | field |
| K1.64 | `menu.body.corners` | field |
| K1.64 | `menu.body.corners @ cond not_composited (ADD-A01)` | rule |
| K1.64 | `menu.body.corners @ {placed_below} context any menu_bar / {placed_above} context any menu_bar / {placed_below} context any combo_box / {placed_above} context any combo_box (the seamless edge)` | rule |
| K1.64 | `menu flags placed_below, placed_above (shared, set by the connector from the placement result)` | flag |
| K1.65 | `menu.body.fill (alpha by MenuOpacity; kwin_blur material layer)` | field |
| K1.65 | `menu.body.stroke` | field |
| K1.65 | `menu.separator.stroke` (the line's paint) | field |
| K1.65 | `reader:kde:breezerc [Style] MenuOpacity` | reader |
| K1.66 | `menu.section_header.{text,stroke}` | field |
| K1.67 | `menu.accelerator.{text,opacity}` | field |
| K1.67 | `menu.values.{accelerator_gap,accelerator_alignment}` | field |
| K1.68 | `menu.submenu_arrow.{glyph,min_width}` | field |
| K1.69 | `menu.check_indicator.{fill,stroke,corners,shadow,glyph} @ variant check / variant radio` (Breeze's check box and radio look on the item's mark, which takes the item's states; Nested controls) | rule |
| K1.69 | `menu.values.check_column, menu.values.check_column_width` | field |
| K1.69 | `menu.item.padding (the text's leading margin)` | field |
| K1.70 | `menu.values.icon_column, menu.values.icon_column_width, menu.values.icon_text_gap, menu.values.show_item_icons` | field |
| K1.70 | `menu.item.padding @ !cond system_setting:show_icons_in_menus (the extra leading margin)` | rule |
| K1.70 | `reader:kde:kdeglobals [KDE] ShowIconsInMenuItems` | reader |
| K1.71 | `menu.separator.margin` | field |
| K1.71 | `menu.separator.min_height` (the separator item's height; the line is `menu.separator.stroke`) | field |
| K1.72 | `menu.scroll_arrow.glyph, menu.scroll_arrow.fill, menu.scroll_arrow.min_height` | field |
| K1.72 | `menu.values.scrollable` | field |
| K1.73 | `menu.values.submenu_open_delay` | field |
| K1.81 | `tooltip.body.shadow` | field |
| K1.81 | - (the fade KWin draws) | out (R1.1) |
| K1.82 | `tooltip.body.corners @ cond not_composited (ADD-A01)` | rule |
| K1.83† | `tooltip.values.{show_delay,browse_timeout,display_time,hide_delay}` | field |
| K1.83† | `tooltip.label.text.wrap` (Word: plain text wraps only where it is wider than the screen, the space it has; rich text word-wraps earlier at QLabel's own width heuristic, the toolkit's layout, R23, not a theme value) | field |
| K1.83† | `tooltip.body.max_width @ cond flavor:widgets` (`auto`: no other maximum width) | rule |
| K1.84† | `tooltip.body.fill, tooltip.body.stroke, tooltip.body.corners, tooltip.body.shadow, tooltip.body.padding @ variant rich` | rule |
| K1.84† | `tooltip.values.placement (KToolTipWidget: centred below, else above, else beside; its gap) @ variant rich` | placement |
| K1.84† | `tooltip.values.hide_delay @ variant rich` | rule |
| K1.90 | `scrollbar.thumb.{stroke,fill,corners}` | field |
| K1.91 | `scrollbar.thumb.opacity; scrollbar.thumb.opacity @ context scrollbar.body {hover}` (the thumb's alpha at rest, full while the pointer is over the bar) | rule |
| K1.92 | `scrollbar.thumb.fill @ context any scroll_view {focus_within}` | rule |
| K1.93 | `scrollbar.track.stroke (the content-side edge)` | field |
| K1.93 | `scrollbar.track.stroke @ variant vertical / variant vertical dir rtl` | rule |
| K1.94 | `scrollbar.arrow_button.{glyph,fill,min_width,min_height}` (the button's extent, `ScrollBar_SingleButtonHeight`) | field |
| K1.94 | `scrollbar.arrow_button.glyph.fill @ {hover} / {pressed} / {disabled}` | rule |
| K1.94 | `scrollbar.values.arrow_buttons_start; scrollbar.values.arrow_buttons_end` | field |
| K1.94 | `reader:kde:breezerc [Style] ScrollBarAddLineButtons, ScrollBarSubLineButtons -> scrollbar.values.arrow_buttons_*` | reader |
| K1.95 | `scrollbar.thumb.transitions; scrollbar.track.transitions` | field |
| K1.96 | `scroll_view.corner.fill` | field |
| K1.100 | `slider.thumb.stroke` | field |
| K1.100 | `slider.thumb.stroke @ {hover} / {focused} / variant warning` | rule |
| K1.101 | `slider.thumb.shadow` | field |
| K1.101 | `slider.thumb.shadow @ {pressed}` | rule |
| K1.102 | `slider.thumb.{fill,shadow} @ {pressed}` | rule |
| K1.103 | `slider.tick.fill` | field |
| K1.103 | `slider.tick.fill @ {in_fill}` | rule |
| K1.103 | `slider flags in_fill` | flag |
| K1.103 | `slider.values.tick_gap` | field |
| K1.103 | `reader:kde:breezerc [Style] SliderDrawTickMarks -> cond system_setting:slider_tick_marks` | reader |
| K1.103 | `slider.tick @ !cond system_setting:slider_tick_marks` (`absent = true`: ticks off, ADD-A04) | rule |
| K1.104 | `slider.track.corners` | field |
| K1.105 | `slider.value_fill.fill, slider.tick.fill @ variant warning` | rule |
| K1.106 | `slider.thumb.focus` | field |
| K1.106 | `slider.thumb.transitions` | field |
| K1.110 | `progress_bar.indicator.fill (a repeating linear gradient: Breeze's two-stripe pattern tiled over the rounded fill, breezehelper.cpp:1222-1270) @ {indeterminate}` | rule |
| K1.110 | `progress_bar.values.loop (keyframes on part indicator, each setting its fill: the gradient shifted along its axis by one pattern period over the loop, animations/breezebusyindicatorengine.cpp:94-98: the gradient's offset, S2.3; reversed in right-to-left by a dir rtl rule, breezehelper.cpp:1245-1247)` | field |
| K1.110 | `reader:kde:breezerc [Style] ProgressBarAnimated, ProgressBarBusyStepDuration -> progress_bar.values.loop` | reader |
| K1.111 | `progress_bar.indicator.{stroke,fill}` | field |
| K1.112 | `progress_bar.indicator.fill @ context list.row {selected}` | rule |
| K1.113 | `progress_bar.value_label.text` | field |
| K1.113 | `progress_bar.value_label.text.fill @ context list.row {selected}` | rule |
| K1.113 | `progress_bar.values.label_gap, progress_bar.values.label_alignment, progress_bar.values.label_position` | field |
| K1.114 | `progress_bar.values.min_indicator_length` | field |
| K1.115† | `progress_bar.track.min_height, progress_bar.track.max_height @ variant @capacity / variant @capacity_inline` | rule |
| K1.115† | `progress_bar.track.corners, progress_bar.indicator.corners @ variant @capacity_inline (its round margin)` | rule |
| K1.115† | `progress_bar.values.label_position (below) @ variant @capacity / (inline) @ variant @capacity_inline` | rule |
| K1.120 | `tab.values.indicator_edge, tab.indicator.min_width, tab.indicator.min_height @ variant left / variant right` | rule |
| K1.121 | `tab.tab.fill, tab.tab.corners (bar side) @ !{selected}` | rule |
| K1.122 | `tab.tab.fill @ {selected} / {selected} variant document` | rule |
| K1.123 | `tab.tab.min_height, tab.tab.fill (= qt_darker, ADD-A50) @ variant @static` | rule |
| K1.123 | `tab.tab.fill @ {hover} variant @static / {selected} variant @static` | rule |
| K1.123 | `tab.indicator.fill @ variant @static (the selected tab's strip)` | rule |
| K1.123 | `tab.separator.stroke, tab.separator.margin @ variant @static prev tab.tab !{selected} next tab.tab !{selected} (ADD-B07)` | rule |
| K1.123 | `tab.values.tab_gap, tab.values.bar_pane_overlap, tab.values.expanding @ variant @static` | rule |
| K1.124 | `button.icon.glyph.fill @ variant close context tab.tab` | rule |
| K1.124 | `button.body.fill (corners Full), button.icon.glyph.fill @ variant close {hover} context tab.tab / variant close {pressed} context tab.tab / variant close {disabled} context tab.tab` (the close button's own states) | rule |
| K1.124 | `button.icon.icon.size (= token.icons.sizes.small.size) @ variant close context tab.tab` | rule |
| K1.125 | `tab.scroll_arrow.fill (the filled panel), tab.scroll_arrow.glyph` | field |
| K1.126 | `tab.overflow_edge.stroke` | field |
| K1.127 | `tab.bar.stroke (base line)` | field |
| K1.128 | `tab.pane.fill, tab.pane.stroke, tab.pane.corners, tab.pane.padding` | field |
| K1.128 | `tab.pane.corners @ variant top / variant bottom / variant left / variant right` | rule |
| K1.129 | `tab.label.focus (Underline) @ {selected,focus_visible}` | rule |
| K1.129 | `tab.label.focus.transition` | field |
| K1.130 | `tab.icon.icon.size` | field |
| K1.130 | `tab.values.icon_text_gap` | field |
| K1.131 | `tab.values.alignment` | field |
| K1.131 | `reader:kde:breezerc [Style] TabBarDrawCenteredTabs` | reader |
| K1.132 | `tab.label.text.fill @ {disabled}` | rule |
| K1.132 | `tab.tab.transitions (Fill: the hover fade)` | field |
| K1.133† | `tab.values.presentations` (Move: the tabs sliding to their new place; a tab moved aside takes `= token.motion.durations.tab_bar`, a plain 250, `qtabbar.cpp:2152`, and the dropped tab `= min(token.motion.durations.tab_bar, floor(div(mul(element.move_distance, token.motion.durations.tab_bar), element.width)))`, `element.height` on vertical tabs, `:2374-2379`, over `element.move_distance` and its own size, which the values declare, `data = { values = ["move_distance", "width", "height"] }`; its timing `= token.motion.timings.in_out_quad`, `qtabbar_p.h:144`; ADD-A87) | field |
| K1.133† | `tab.values.drag_hover_delay` | field |
| K1.140 | `sidebar.pane.stroke (the inner edge only; the Inactive group's highlight) @ variant @side_panel` | rule |
| K1.140 | `sidebar.pane.stroke (the Active group's highlight) @ {focus_within} variant @side_panel` | rule |
| K1.140 | `sidebar.pane.stroke (every edge) @ variant @side_panel cond system_setting:side_panel_draw_frame` | rule |
| K1.140 | `reader:kde:breezerc [Style] SidePanelDrawFrame` | reader |
| K1.141 | `list.values.row_background_inset @ context sidebar.pane[@side_panel]` | rule |
| K1.142† | `list.section_header.text @ context sidebar.pane[@places] / context sidebar.pane[@places] cond window_inactive` | rule |
| K1.142† | `list.leading_icon.icon.size @ context sidebar.pane[@places]` | rule |
| K1.142† | `progress_bar.track.fill, progress_bar.track.corners (Full), progress_bar.track.min_height (= an expression over the row icon's own icon token, `token.icons.sizes.keyed.<key>.size`: one token, no other element's field), progress_bar.indicator.fill @ variant @capacity context sidebar.pane` | rule |
| K1.142† | `progress_bar.indicator.fill @ variant @capacity context list.row {selected} !cond window_inactive` | rule |
| K1.142† | `progress_bar.indicator.fill (= pick(floor(min(add(element.progress, sub(1, values.warning_threshold)), 1)), <normal>, <warning>)), progress_bar.values.warning_threshold @ variant @capacity context sidebar.pane` (the recolouring above the threshold an expression over the progress, the indicator's data; at the threshold itself the expression picks the warning colour, so the value phase checks the source's comparison and states the threshold to match) | rule |
| K1.142† | `list.trailing_icon.icon @ context sidebar.pane[@places] (the eject action)` | rule |
| K1.142† | `list.row.opacity @ {@hidden_place} context sidebar.pane[@places]` | rule |
| K1.142† | `list.values.presentations (Appear, Disappear, queried with each row: Fade) @ {@hidden_place} context sidebar.pane[@places]` (the hidden places' rows shown and hidden) | rule |
| K1.150 | `input.inactive.header (the stated [Colors:Header][Inactive] roles)` | input |
| K1.150 | `role.header.background @ cond window_inactive` | role |
| K1.150 | `toolbar.bar.bindings (Surface(Header)) @ variant top; menu_bar.bar.bindings (Surface(Header))` | rule |
| K1.150 | `reader:kde:kdeglobals [Colors:Header][Inactive]` | reader |
| K1.151 | `toolbar.bar.stroke (bottom edge only)` | field |
| K1.151 | `toolbar.bar.stroke (top edge) @ context dialog !{@has_toolbar}` | rule |
| K1.152 | `button.body.fill, button.body.stroke @ variant tool context toolbar.bar (none at rest: auto-raise) / {hover} variant tool context toolbar.bar / {pressed} variant tool context toolbar.bar / {checked} variant tool context toolbar.bar / {focus_visible} variant tool context toolbar.bar` | rule |
| K1.153 | `button.values.label_position @ variant tool context toolbar.bar variant @main / variant tool context toolbar.bar !variant @main` | rule |
| K1.153 | `reader:kde:kdeglobals [Toolbar style] ToolButtonStyle, ToolButtonStyleOtherToolbars` | reader |
| K1.154 | `button.body.padding @ variant tool context toolbar.bar` | rule |
| K1.154 | `button.values.icon_text_gap @ variant tool context toolbar.bar (beside or under)` | rule |
| K1.155 | `split_button.secondary.fill, split_button.secondary.stroke, split_button.divider.stroke, split_button.arrow.glyph @ context toolbar.bar (the split sub-control and its separator line)` | rule |
| K1.155 | `button.menu_indicator.glyph, button.menu_indicator.min_width @ variants {tool,@inline_small} / variants {tool,@inline_large}` | rule |
| K1.155 | `button.values.menu_indicator_anchor (BottomEnd), button.menu_indicator.{min_height,margin} @ variants {tool,@inline_small}` (the small arrow's box at the button's bottom-right corner, inset from it, `breezestyle.cpp:2940-2944`, mirrored, `:2950`; ADD-A122) | rule |
| K1.155 | `button.menu_indicator.glyph.fill @ {hover} variant tool / {pressed} variant tool / {disabled} variant tool; split_button.arrow.glyph @ {hover} context toolbar.bar / {pressed} context toolbar.bar / {disabled} context toolbar.bar` (the split button's own states) | rule |
| K1.156 | `toolbar.separator.stroke, toolbar.separator.min_height` | field |
| K1.156 | `toolbar.separator (absent = true) @ !cond system_setting:toolbar_draw_item_separator` | rule |
| K1.156 | `toolbar.grip.glyph (two lines), toolbar.grip.min_width, toolbar.grip.min_height` | field |
| K1.156 | `reader:kde:breezerc [Style] ToolBarDrawItemSeparator` | reader |
| K1.157 | `button.icon.glyph @ variant overflow context toolbar.bar` | rule |
| K1.158 | `button.icon.icon.size @ variant tool context toolbar.bar variant @main / variant tool context toolbar.bar !variant @main` | rule |
| K1.158 | `token.icons.sizes.toolbar, token.icons.sizes.keyed.main_toolbar` | token |
| K1.158 | `reader:kde:icon theme index.theme MainToolbarDefault, ToolbarDefault` | reader |
| K1.160 | `status_bar.bar.stroke.top @ variant @main_window` | rule |
| K1.170 | `list.row_background.corners, list.values.row_background_inset` | field |
| K1.170 | `list.values.row_background_inset @ pos first` | rule |
| K1.170 | `list.cell.corners @ pos first / pos last (among cells, ADD-B07)` | rule |
| K1.170 | `list.cell.corners @ variant table` | rule |
| K1.171 | `list.row_background.stroke @ {selected} / {hover}` | rule |
| K1.172 | `list.row_background.fill @ {selected,hover} (= qt_lighter, ADD-A50, ADD-A52)` | rule |
| K1.173 | `list.row_background.fill, list.row.text.fill @ {selected} cond window_inactive` (the rows take `role.selection.*`, whose inactive values are K3.40's `kde_effects(kde_selection_inactive(…), …)` derivation) | rule |
| K1.174 | `list.row_background.fill, list.row.text.fill @ {selected,disabled}` (the rows take `role.selection.disabled.*`, which start from Window's active values, K3.50) | rule |
| K1.175 | `list.row_background.focus (= qt_lighter of the highlight, ADD-A50) @ {focused} !{selected}` | rule |
| K1.175 | `list.row_background.focus (Absent) @ !cond system_setting:view_draw_focus_indicator` | rule |
| K1.175 | `reader:kde:breezerc [Style] ViewDrawFocusIndicator` | reader |
| K1.176 | `list.disclosure.glyph, list.disclosure.glyph.fill @ {hover}` | rule |
| K1.176 | `list.branch_line.stroke` | field |
| K1.176 | `list.branch_line (absent = true) @ !cond system_setting:view_draw_tree_branch_lines` | rule |
| K1.176 | `list.values.indent_per_level` | field |
| K1.176 | `reader:kde:breezerc [Style] ViewDrawTreeBranchLines` | reader |
| K1.177 | `list.header_section.fill @ {hover} / {pressed} / {sorted}` | rule |
| K1.177 | `list.header_section.transitions (Fill)` | field |
| K1.178 | `list.header_separator.stroke, list.header.stroke (bottom edge)` | field |
| K1.178 | `list.header_corner.fill, list.header_corner.stroke` | field |
| K1.179 | `list.sort_indicator.glyph` | field |
| K1.179 | `list.values.sort_indicator_inverted` | field |
| K1.179 | `list.header_section.padding, list.values.header_item_gap` | field |
| K1.179 | `reader:kde:breezerc [Style] ViewInvertSortIndicator` | reader |
| K1.180 | `list.row.padding, list.leading_icon.icon.size, list.values.secondary_text_gap @ variant grid (icon above text; the row_background as in list mode)` | rule |
| K1.181 | `list.leading_icon.icon.size (= token.icons.sizes.small.size)` | field |
| K1.181 | `reader:kde:icon theme SmallDefault (KStyle PM_ListViewIconSize)` | reader |
| K1.182 | `list.drop_indicator.stroke, list.drop_indicator.glyph` | field |
| K1.182 | `list.row_background.stroke @ {drop_target}` | rule |
| K1.183 | `list.cell.fill @ {@sort_column}` | rule |
| K1.183 | `reader:kde:kdeglobals [General] shadeSortColumn` | reader |
| K1.184† | `list.disclosure.glyph (the column view's triangle: fill = role.@shade_mid, stroke = role.@shade_dark) @ variant @column_view` | rule |
| K1.184† | `list.disclosure.glyph.fill @ {selected} variant @column_view / {selected} variant @column_view cond window_inactive` | rule |
| K1.184† | `role.@shade_mid, role.@shade_dark (KDE's 3-D shades, K3.53)` | role |
| K1.185† | `list.values.presentations (@expand, @collapse: Slide; = token.motion.durations.variant, QVariantAnimation's own default, 250, unscaled, S4.1's rule, `qtbase@6.12.0 src/corelib/animation/qvariantanimation.cpp:153`; = token.motion.timings.in_out_quad, `src/widgets/itemviews/qtreeview_p.h:89`; drawn only while the style hint, `token.motion.durations.animations`, is above 0 when the view is made, `qtreeview.cpp:3148`)` | field |
| K1.190 | `popover.body.shadow (Shadow::Elevation of Breeze's popup entry, K3.2)` | field |
| K1.191 | `popover.body.corners @ cond not_composited (ADD-A01)` | rule |
| K1.200 | `splitter.divider.fill @ variant @dock` | rule |
| K1.201 | `separator.line.stroke (centred on the half pixel: the edge's snap)` | field |
| K1.201 | `component.multi_tab_bar.separator.stroke` | component |
| K1.210 | `form.values.label_alignment (its horizontal member, `SH_FormLayoutLabelAlignment`), form.values.field_growth (`SH_FormLayoutFieldGrowthPolicy`), form.values.contents_alignment (the form's own left and top alignment, `SH_FormLayoutFormAlignment`) @ cond flavor:widgets` (Qt's `QFormLayout` as Breeze styles it, `breezestyle.cpp:948-953`) | rule |
| K1.210 | `form.values.collapse (absent: the rows never wrap, `SH_FormLayoutWrapPolicy` `DontWrapRows`, `breezestyle.cpp:954-955`); form.{group_title,group_description,section_title,row_edit_icon} (absent = true) @ cond flavor:widgets` (Qt's form layout draws labels, `form.field_label`, beside fields, the nested widgets in `context form.row`) | rule |
| K1.211 | `group_box.values.title_margin, group_box.title.text, group_box.body.padding (the space the title takes)` | field |
| K1.220 | `dialog.values.action_layout (row_order: KdeLayout's roles, Help and Reset leading, the stretch, then the rest)` | field |
| K1.221 | `dialog.values.button_icons` | field |
| K1.221 | `reader:kde:kdeglobals [KDE] ShowIconsOnPushButtons` | reader |
| K1.222 | `dialog.header.{fill,bindings} @ {@has_toolbar}` | rule |
| K1.222 | `dialog.header.stroke @ !{@has_toolbar} (top separator)` | rule |
| K1.223 | - | out (R1.1) |
| K1.224† | `dialog.icon.glyph (ThemeIcon), dialog.icon.icon.size (= token.icons.sizes.keyed.huge.size) @ variants {alert,warning} / variants {alert,info} / variants {alert,error} / variants {alert,@question}` | rule |
| K1.224† | `dialog.message.text; dialog.values.details_min_lines` | field |
| K1.224† | `checkbox @ context dialog (the "don't ask again" check box)` | rule |
| K1.230 | `spinner.icon.glyph (ThemeIcon process-working)` | field |
| K1.230 | `spinner.values.loop (rotation and its period)` | field |
| K1.240 | `combo_box.body.fill, combo_box.body.stroke @ variant editable / variant editable {hover} / variant editable {focused}` | rule |
| K1.240 | `combo_box.arrow.glyph.fill @ variant editable / variant editable context combo_box.arrow_area {hover}` (the hover colour only while the pointer is over the arrow sub-control, `SC_ComboBoxArrow`) | rule |
| K1.240 | `combo_box.arrow_area.min_width @ variant editable; combo_box.arrow_area (absent = true) @ !variant editable` (the arrow sub-control as an element, its drawing fields absent) | rule |
| K1.240 | `combo_box.arrow.transitions (Glyph: the hover fade)` | field |
| K1.241 | `combo_box.body.fill, combo_box.body.stroke, combo_box.body.shadow @ variant flat` | rule |
| K1.242 | `combo_box.body.fill @ {open} / {pressed}` | rule |
| K1.243 | `combo_box.body.stroke @ {hover} / {focused} / {pressed}` | rule |
| K1.244 | `combo_box.body.shadow @ !variant flat !variant editable` | rule |
| K1.245 | `menu.body.fill, menu.body.stroke, menu.body.corners, menu.body.shadow @ context combo_box` | rule |
| K1.245 | `menu.body.corners @ {placed_below} context combo_box / {placed_above} context combo_box (the menu's own flags: the seamless edge, K1.64)` | rule |
| K1.245 | `menu.item.fill @ {hover} context combo_box / {focused} context combo_box` (Breeze's `State_Selected` on the popup's current item, which follows the pointer and the keys: `hover` or `focused`, S7.2; the chosen value's mark is `menu.check_indicator`) | rule |
| K1.245 | `combo_box.values.popup_placement (QtComboBox)` | placement |
| K1.246 | `combo_box.arrow.glyph.fill @ {disabled} / {hover} / {focused}` | rule |
| K1.247 | `combo_box.body.stroke, combo_box.arrow.glyph.fill @ variant warning` | rule |
| K1.250 | `expander.arrow.glyph.fill @ context expander.header {hover}` (the title's hover) | rule |
| K1.251 | `expander.values.presentations (@expand, @collapse: the content's height; duration and timing the source's)` | field |
| K1.260† | `link.text.text.fill (= the link role), link.text.text.decoration @ variant @url_label` | rule |
| K1.260† | `link.text.text.fill (= the Bright text colour) @ {hover} variant @url_label` | rule |
| K1.260† | `link.text.text.decoration (Absent) @ variants {@url_label,@float}; link.text.text.decoration @ {hover} variants {@url_label,@float}` | rule |
| K1.260† | `link.icon.icon @ {hover} variant @url_label (the alternate pixmap)` | rule |
| K1.260† | `link.body.cursor @ variant @url_label` | rule |
| K1.270 | `token.text.keyed.heading_1` | token |
| K1.270 | `token.text.keyed.heading_2` | token |
| K1.270 | `token.text.keyed.heading_3` | token |
| K1.270 | `token.text.keyed.heading_4` | token |
| K1.271 | `token.text.keyed.smallest_readable` | token |
| K1.271 | `reader:kde:kdeglobals [General] smallestReadableFont (family, style, size)` | reader |
| K1.272 | `token.text.keyed.flat_group_title` | token |
| K1.272 | `token.text.keyed.command_link_title` | token |
| K1.272 | `token.text.keyed.menu_section` | token |
| Q1.1 | `token.icons.sizes.keyed.small` | token |
| Q1.1 | `token.icons.sizes.keyed.small_medium` | token |
| Q1.1 | `token.icons.sizes.keyed.medium` | token |
| Q1.1 | `token.icons.sizes.keyed.large` | token |
| Q1.1 | `token.icons.sizes.keyed.huge` | token |
| Q1.1 | `token.icons.sizes.keyed.enormous` | token |
| Q1.1 | `token.icons.sizes.keyed.for_labels` (Kirigami's value at the preset's default font, stated from the facts, for what no part carries: a token has no font metrics, S12) | token |
| Q1.1 | `input.icon.icon.size, button.icon.icon.size, link.icon.icon.size, badge.icon.icon.size, segmented_control.icon.icon.size` (each `= kirigami_label_icon_size(body_font_height)` in the Kirigami rules of the parts that use the label size, the button's only as the clear, inline-action and chip-remove buttons: Q1.15, Q1.17, Q1.18, Q1.136, Q2.24, Q2.27, K-KA-25†; each such part declares `body_font_height` in its data, so under a user's font it follows the font as Kirigami's `sizeForLabels` does, ADD-A100) | field |
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
| Q1.5 | `dialog.title.text (= token.text.keyed.heading_1) @ cond flavor:kirigami` | rule |
| Q1.5 | `toolbar.title.text (= token.text.keyed.heading_1) @ cond flavor:kirigami (the page title)` | rule |
| Q1.5 | `card.banner_title.text (= token.text.keyed.heading_1) @ cond flavor:kirigami` | rule |
| Q1.5 | `sheet.title.text, list.inline_header.text (= token.text.keyed.heading_2) @ cond flavor:kirigami` | rule |
| Q1.5 | `form.section_title.text (= token.text.keyed.heading_3), form.field_label.text (= token.text.keyed.heading_5) @ cond flavor:kirigami` | rule |
| Q1.5 | `list.section_header.text (= token.text.keyed.heading_4) @ context sidebar.pane cond flavor:kirigami (the global drawer's sections)` | rule |
| Q1.5 | `list.section_header.text (heading_5 with its weight restated) @ cond flavor:kirigami` | rule |
| Q1.5 | `token.text.roles.title1, token.text.roles.title2, token.text.roles.title3, token.text.roles.subheadline (= token.text.keyed.heading_<n>); token.text.roles.headline (= token.text.roles.title3, the slot owning heading_3, S4.4); token.text.roles.display_large, token.text.roles.display_medium, token.text.roles.display_small, token.text.roles.large_title (= token.text.roles.title1, the slot owning heading_1, S4.4)` | token |
| Q1.6 | `window.values.default_size (grid units)` | field |
| Q1.6 | `window.values.default_size @ cond mobile_form_factor` | rule |
| Q1.6 | `window.values.default_size ("absent") @ cond flavor:widgets` (a Qt Widgets window states no default size; Kirigami's window does) | rule |
| Q1.7 | `token.size_classes (Kirigami's wideScreen width)` | token |
| Q1.8 | `button.body.stroke (the highlighted pen) @ {focus_visible} cond flavor:kirigami` (QML passes `hasFocus` as active focus or `highlighted`, but Breeze draws the pen only with `State_KeyboardFocusChange`, `breezestyle.cpp:4518`, which the style item sets only after a keyboard focus reason, `kquickstyleitem.cpp:884-890`) | rule |
| Q1.8 | `button.body.stroke (the highlighted pen) @ {@keyboard_focus_change} variant @highlighted cond flavor:kirigami` (a `@highlighted` button whose last focus change, in or out, was by keyboard keeps the pen unfocused: the keyed flag, ADD-A122) | rule |
| Q1.8 | `button.icon.icon.fill (= the label's text colour) @ cond flavor:kirigami` | rule |
| Q1.8 | `button.body.fill, button.body.stroke @ variant flat cond flavor:kirigami / {checked} cond flavor:kirigami / {default} cond flavor:kirigami` | rule |
| Q1.8 | `button.menu_indicator.glyph @ variant menu cond flavor:kirigami` | rule |
| Q1.9 | `button.body.bindings (surface Button) @ variant tool !variant flat cond flavor:kirigami` (a raised ToolButton binds the Button set; a flat one binds nothing and inherits its parent's theme, `ToolButton.qml:18-19`, `platformtheme.cpp:1060-1090`; disabled, Kirigami stops inheriting but keeps the set it had, `kirigami@6.31.0 src/platform/platformtheme.cpp:1063-1065`, `:1105-1110`, so a flat one's disabled colours are `surface.disabled.*`, a Header-set toolbar's in a toolbar, never `role.window.disabled.*`: ADD-A122) | rule |
| Q1.9 | `button.body.stroke (the highlighted pen) @ {focus_visible} variant tool cond flavor:kirigami / {pressed} variant tool !variant flat cond flavor:kirigami` (`breezestyle.cpp:4577`) | rule |
| Q1.9 | `button.body.stroke (the highlighted pen) @ {@keyboard_focus_change} variants {tool,@highlighted} cond flavor:kirigami` (`ToolButton.qml:60` passes `highlighted` as focus, and Breeze's pen needs `State_KeyboardFocusChange`, as Q1.8's: ADD-A122) | rule |
| Q1.9 | `button.values.label_position @ variant tool cond flavor:kirigami (the tool button style: IconOnly, TextBesideIcon, TextUnderIcon)` | rule |
| Q1.10 | `button.label.text.mnemonic_decoration @ cond mnemonics_visible` | rule |
| Q1.10 | `cond mnemonics_visible (ADD-A12)` | condition |
| Q1.11 | `input.body.fill, input.body.stroke, text_area.body.fill, text_area.body.stroke @ cond flavor:kirigami (the same Breeze frame)` | rule |
| Q1.12 | `input.values.default_width` | field |
| Q1.12 | `text_area.values.default_width, text_area.values.default_height` | field |
| Q1.13 | `text_area.misspelling.text.decoration.paint @ cond flavor:kirigami (= role.view.negative)` | rule |
| Q1.14 | `input.caret.min_width @ cond tablet_mode` | rule |
| Q1.14 | `input.caret.fill @ cond tablet_mode` | rule |
| Q1.14 | `input.selection_handle.glyph, input.selection_handle.min_width, input.selection_handle.min_height, input.selection_handle.corners (Full), input.selection_handle.fill (= kirigami_tint, ADD-A52) @ cond tablet_mode` | rule |
| Q1.15 | `input.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100), input.icon.icon.fill (= surface.disabled.foreground), input.icon.padding @ variant leading context any search_field cond flavor:kirigami` | rule |
| Q1.15 | `input.body.padding (the text inset) @ context any search_field cond flavor:kirigami` | rule |
| Q1.15 | `button.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100) @ variant clear_field context any search_field cond flavor:kirigami` | rule |
| Q1.16 | `menu.body.fill, menu.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast): qqc2's own QML, SearchField.qml:183; the field's own frame is Breeze's style item, :117), menu.body.corners, menu.body.shadow, menu.body.padding, menu.body.bindings (Surface(View)) @ context autocomplete context any search_field cond flavor:kirigami` | rule |
| Q1.16 | `autocomplete.values.placement (its gap below the field) @ context any search_field cond flavor:kirigami` | rule |
| Q1.16 | `menu.item.padding @ context autocomplete context any search_field cond flavor:kirigami` | rule |
| Q1.17 | `input.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100), input.icon.icon.fill (= the placeholder's colour) @ variant leading context any search_field cond flavor:kirigami` | rule |
| Q1.17 | `input.values.search_delay (= token.motion.durations.short) @ context any search_field cond flavor:kirigami` | rule |
| Q1.17 | `input.values.search_delay (= token.motion.durations.human_moment) @ context search_field.body[@delay_search] cond flavor:kirigami` (the field's `delaySearch`) | rule |
| Q1.18 | `button.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100), button.body.padding @ variant tool context input cond flavor:kirigami (the inline actions)` | rule |
| Q1.18 | `input.values.icon_text_gap @ cond flavor:kirigami (the actions' spacing)` | rule |
| Q1.18 | `input.body.transitions (Geometry: the padding, as actions appear; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q1.18 | `tooltip @ context any input cond flavor:kirigami (the focus shortcut's tooltip)` | rule |
| Q1.19 | `button.icon.glyph (ThemeIcon) @ variant reveal context input cond flavor:kirigami / variant reveal {checked} context input cond flavor:kirigami` (the toggle's own state) | rule |
| Q1.20 | `checkbox.values.label_gap, radio.values.label_gap @ cond flavor:kirigami` | rule |
| Q1.20 | `checkbox.mark.glyph @ {indeterminate} cond flavor:kirigami (as K1.50)` | rule |
| Q1.21 | `checkbox.label.focus, radio.label.focus @ cond flavor:kirigami (offset per side)` | rule |
| Q1.21 | `checkbox.box.focus, radio.ring.focus @ cond flavor:kirigami (Absent)` | rule |
| Q1.22 | `checkbox.label.opacity, radio.label.opacity @ {disabled} cond flavor:kirigami` | rule |
| Q1.23 | `menu.body.shadow @ cond flavor:kirigami` | rule |
| Q1.24 | `menu.values.presentations (Appear, Disappear: Fade; = token.motion.durations.short, its timing) @ cond flavor:kirigami` | rule |
| Q1.25 | `menu.body.padding, menu.body.min_width (= mul(token.params.layout.grid_unit, <n>)) @ cond flavor:kirigami` | rule |
| Q1.26 | `menu.item.{fill,stroke,corners} @ {hover} cond flavor:kirigami / {focused} cond flavor:kirigami` (qqc2's `highlighted`, set by the menu's current item from the pointer and the keys, `MenuItem.qml:122`: `hover` or `focused`, S7.2) | rule |
| Q1.27 | `menu.item.fill @ {pressed} cond flavor:kirigami` | rule |
| Q1.27 | `menu.item_label.text.fill, menu.item_icon.icon.state_effect @ {pressed} cond flavor:kirigami` | rule |
| Q1.28 | `menu.item.padding @ cond flavor:kirigami` | rule |
| Q1.28 | `menu.item.padding @ cond flavor:kirigami cond input:touch` | rule |
| Q1.29 | `menu.item_icon.icon.size (= token.icons.sizes.keyed.small.size) @ cond flavor:kirigami / (= token.icons.sizes.keyed.small_medium.size) @ cond flavor:kirigami cond input:touch` | rule |
| Q1.29 | `menu.values.check_column, menu.values.icon_column (reserved when any item needs it)` | field |
| Q1.30 | `menu.accelerator.text (the label's fill), menu.values.accelerator_alignment @ cond flavor:kirigami` | rule |
| Q1.30 | `menu.submenu_arrow.glyph (ThemeIcon, size = token.icons.sizes.keyed.small.size) @ cond flavor:kirigami` | rule |
| Q1.31 | `menu.check_indicator.{fill,stroke,corners,shadow,glyph} @ variant check cond flavor:kirigami / variant radio cond flavor:kirigami` (qqc2's check and radio indicators on the item's mark; an exclusive group's items `radio`) | rule |
| Q1.32 | `menu.separator.margin, menu.separator.stroke, menu.separator.min_height @ cond flavor:kirigami (its implicit width is the menu's min_width, Q1.25)` | rule |
| Q1.33 | `tooltip.body.shadow @ cond flavor:kirigami` | rule |
| Q1.34 | `tooltip.values.presentations (Appear, Disappear: Fade; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q1.35 | `tooltip.values.placement (QtQuickTooltip: centred above the parent, its gap and edge margins) @ cond flavor:kirigami` | placement |
| Q1.35 | `tooltip.body.padding @ cond flavor:kirigami` | rule |
| Q1.36 | `tooltip.values.display_time ("none": no timeout while hovered) @ cond flavor:kirigami` | rule |
| Q1.36 | `tooltip.values.show_on_press, tooltip.values.show_delay (= token.behavior.long_press) @ cond tablet_mode cond flavor:kirigami` | rule |
| Q1.37 | `tooltip.body.bindings (Surface(Tooltip))` | field |
| Q1.37 | `tooltip.body.bindings (Surface(Complementary)) @ context any (surface Complementary) (ContextMatch.surface, ADD-A68)` | rule |
| Q1.38 | `tooltip.body.stroke (Breeze's frame colour role.tooltip.border, Q3.15, breeze@6.8.2 kstyle/breezestyle.cpp:4738; under cond flavor:kirigami = kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), qqc2's ToolTip.qml:115, Q3.15)` | field |
| Q1.39 | `scrollbar.track.padding` | field |
| Q1.39 | `scrollbar.values.trough_click` | field |
| Q1.40 | `scrollbar.thumb.transitions, scrollbar.track.transitions (Fill, Opacity: the hover cross-fade; = token.motion.durations.long) @ cond flavor:kirigami` | rule |
| Q1.41 | `scrollbar.thumb.fill (= alpha(surface.foreground, <fraction>)), scrollbar.thumb.min_width, scrollbar.thumb.min_height, scrollbar.thumb.corners (Full) @ cond input:touch cond flavor:kirigami` | rule |
| Q1.41 | `scrollbar.values.auto_hide_delay @ cond input:touch cond flavor:kirigami` | rule |
| Q1.41 | `scrollbar.values.presentations (Appear: Slide from the edge; Disappear: Fade, = token.motion.durations.long) @ cond input:touch cond flavor:kirigami` | rule |
| Q1.42 | `scrollbar.values.repeat_interval` | field |
| Q1.43 | `scroll_view.frame (absent = true) @ !variant @framed cond flavor:kirigami` (the framed background only under the style hint, `ScrollView.qml:35-45`; where drawn, the styled panel of K2.21) | rule |
| Q1.44 | `slider.thumb.min_width, slider.thumb.min_height @ cond flavor:kirigami` | rule |
| Q1.44 | `slider.tick @ variant ticks_before / variant ticks_after (only where the tick step is set) cond flavor:kirigami` | rule |
| Q1.45 | `slider.body.padding, slider.thumb.min_width, slider.thumb.min_height, slider.thumb.corners, slider.thumb.fill, slider.track.min_height, slider.track.fill, slider.value_fill.fill @ variant range cond flavor:kirigami` | rule |
| Q1.46 | `slider.thumb.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), RangeSlider.qml:32, :52) @ variant range cond flavor:kirigami` | rule |
| Q1.46 | `slider.thumb.stroke @ {focused} variant range cond flavor:kirigami` | rule |
| Q1.47 | `slider.thumb.shadow @ variant range cond flavor:kirigami` | rule |
| Q1.48 | `progress_bar.track @ cond flavor:kirigami; progress_bar.indicator.fill @ {indeterminate} cond flavor:kirigami (Breeze's busy stripes, K1.110)` | rule |
| Q1.49 | `progress_bar.body.padding @ cond flavor:kirigami` | rule |
| Q1.50 | `tab.tab.fill, tab.tab.stroke, tab.tab.corners, tab.bar.stroke @ cond flavor:kirigami (Breeze's tab element)` | rule |
| Q1.50 | `tab.values.tab_gap (negative: the tab overlap)` | field |
| Q1.51 | `tab.values.focus_visible_when (ADD-A16)` | field |
| Q1.52 | `tab.bar.bindings (surface Window) @ cond flavor:kirigami` | rule |
| Q1.52 | `tab.bar.min_width (= mul(token.params.layout.grid_unit, <n>), ADD-A44) @ cond flavor:kirigami` (the bar's minimum); `tab.tab.padding` start and end `= max(<Breeze margin>, token.params.layout.grid_unit)` `@ cond flavor:kirigami` (`TabButton.qml:19`: the tab's width is the larger of the style's and its text plus a grid unit on each side, for a text-only tab; only the height, `:20`, is a fallback for styles that report none) | rule |
| Q1.53 | `sidebar.pane.bindings (surface View) @ variant overlay cond flavor:kirigami / (surface Window) @ !variant overlay cond flavor:kirigami / (surface Window) @ variants {overlay,@global} / (surface View) @ variant @global !variant overlay` (Kirigami's drawers only: Qt Widgets' side panes keep theirs) | rule |
| Q1.54 | `sidebar.pane.stroke (the inner edge, paint from the Header set) @ cond flavor:kirigami !variant overlay` | rule |
| Q1.54 | `sidebar.pane.stroke (Absent) @ variant overlay cond flavor:kirigami` | rule |
| Q1.55 | `sidebar.separator.stroke` | field |
| Q1.55 | `sidebar.separator.margin` (its insets from its ends) | field |
| Q1.55 | `sidebar.separator.transitions (Geometry; = token.motion.durations.long, its timing)` | field |
| Q1.56 | `sidebar.edge_shadow.fill (Linear, its stops), sidebar.edge_shadow.min_width @ variant overlay` | rule |
| Q1.56 | `sidebar.edge_shadow.transitions (Opacity; = token.motion.durations.long, its timing)` | field |
| Q1.57 | `sidebar.scrim.fill @ variant overlay` | rule |
| Q1.58 | `sidebar.values.presentations (Appear: Slide, its timing; Disappear: Slide, its timing; @settle: = mul(token.motion.durations.long, element.progress)) @ cond flavor:kirigami` | rule |
| Q1.59 | `sidebar.values.collapsed_width (= add(token.icons.sizes.keyed.medium.size, <padding>)) @ variant compact` | rule |
| Q1.59 | `sidebar.pane.transitions (Geometry: the width; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q1.59 | `list.row.text.fill (= alpha(base, 0)) @ context sidebar.pane[@global] context sidebar.pane[compact] cond flavor:kirigami` (the collapsed drawer fades the label alone, its icon staying: `GlobalDrawerActionItem.qml:74-93`) | rule |
| Q1.59 | `list.row.transitions (Text: = mul(token.motion.durations.long, <fraction>), its timing) @ context sidebar.pane[@global] cond flavor:kirigami` (the label's fade both ways) | rule |
| Q1.59 | `list.row.opacity (fully transparent), list.row.transitions (Opacity) @ !variant with_icon context sidebar.pane[@global] context sidebar.pane[compact] cond flavor:kirigami` (an item without an icon fades whole, `:135-142`) | rule |
| Q1.60 | `sidebar.values.width_fraction (the cap's share), sidebar.pane.min_width, sidebar.pane.max_width (= min(mul(token.params.layout.grid_unit, <n>), mul(element.window_width, values.width_fraction))) @ cond flavor:kirigami` | rule |
| Q1.60 | `sidebar.pane.min_height, sidebar.pane.max_height (= min(mul(token.params.layout.grid_unit, <n>), mul(element.window_height, values.width_fraction))) @ variant @edge_top cond flavor:kirigami / variant @edge_bottom cond flavor:kirigami` (ADD-A78; the drawer's height content-sized between them, `sidebar.values.width` `auto`, `OverlayDrawer.qml:460-475`) | rule |
| Q1.60 | `sidebar.pane.padding` | field |
| Q1.60 | `sidebar.values.resize_grip_width` | field |
| Q1.61 | `sidebar.handle.fill, sidebar.handle.corners, sidebar.handle.shadow, sidebar.handle.icon.size (= token.icons.sizes.keyed.small_medium.size), sidebar.handle.min_width, sidebar.handle.min_height, sidebar.handle.padding` | field |
| Q1.61 | `sidebar.values.presentations (Appear, Disappear: Slide, step part handle, each its timing; = token.motion.durations.long)` | field |
| Q1.61 | `sidebar.handle.transitions (Geometry: the bottom margin; = token.motion.durations.short)` | field |
| Q1.62 | `list.leading_icon.icon.size (= token.icons.sizes.keyed.small_medium.size), list.values.leading_gap @ context sidebar.pane[@global]` | rule |
| Q1.62 | `list.row.text.font.size (= mul(base, <factor>)) @ {@expandable} context sidebar.pane[@global] (an item with children; base: what the earlier rules gave, S2.2)` | rule |
| Q1.62 | `list.row.text.fill (= alpha(base, <fraction>): the label only, `GlobalDrawerActionItem.qml:74-82`) @ {disabled} context sidebar.pane[@global]` | rule |
| Q1.62 | `list.disclosure.glyph (size = token.icons.sizes.keyed.small.size; fill at reduced alpha) @ context sidebar.pane[@global]` | rule |
| Q1.62 | `list.row.text.fill @ {selected} context sidebar.pane[@global] / {checked} context sidebar.pane[@global]` | rule |
| Q1.63 | `sidebar.header.text (= token.text.keyed.heading_1), sidebar.header.icon, sidebar.header.padding, sidebar.values.header_icon_gap @ variant @global` (the title icon to the title: largeSpacing) | rule |
| Q1.63 | `list.section_header.text (= token.text.keyed.heading_4), list.section_header.icon.size (= token.icons.sizes.keyed.small_medium.size) @ context sidebar.pane[@global]` | rule |
| Q1.64 | `sidebar.values.presentations (@submenu_push, @submenu_pop: Slide and Fade together, = token.motion.durations.very_long; the content's Fade, = token.motion.durations.long; each its timing) @ variant @global` | rule |
| Q1.65 | `tooltip @ context sidebar.pane[compact] cond flavor:kirigami (the "Open Sidebar" tooltip on the collapsed rail)` | rule |
| Q1.65 | `tooltip.values.show_delay @ context sidebar.pane[compact] cond flavor:kirigami (toolTipDelay)` | rule |
| Q1.66 | `sidebar.values.width @ variant @context` | rule |
| Q1.66 | `sidebar.header.min_height (= the global toolbar's height), sidebar.header.padding, sidebar.header.text (= token.text.keyed.heading_1) @ variant @context` | rule |
| Q1.66 | `list.row.padding @ context sidebar.pane[@context]` | rule |
| Q1.66 | `list.row.text.font.size (= mul(base, <factor>)) @ {@expandable} context sidebar.pane[@context]` | rule |
| Q1.66 | `list.row.opacity @ {disabled} context sidebar.pane[@context]` | rule |
| Q1.67 | `sidebar.pane.stroke, sidebar.pane.padding (the inner edge; paint from the Header set) @ cond flavor:kirigami` | rule |
| Q1.67 | `sidebar.values.presentations (Appear, Disappear: Slide; Timing::Platform(KirigamiVelocity), Qt Quick's SmoothedAnimation, CONTRA-2) @ cond flavor:kirigami` | rule |
| Q1.68 | `toolbar.bar.bindings (Surface(Header)) @ variant top cond flavor:kirigami / (Surface(Window)) @ variant bottom cond flavor:kirigami` | rule |
| Q1.68 | `toolbar.bar.stroke (bottom edge) @ variant top cond flavor:kirigami` | rule |
| Q1.68 | `toolbar.bar.stroke (top edge) @ variant bottom cond flavor:kirigami` | rule |
| Q1.69 | `toolbar.bar.padding, toolbar.bar.min_height @ cond flavor:kirigami` | rule |
| Q1.70 | `toolbar.separator.stroke, toolbar.separator.min_height @ cond flavor:kirigami` | rule |
| Q1.71 | `toolbar.bar.bindings (Surface(Header)) @ variant @page_header` | rule |
| Q1.71 | `toolbar.bar.stroke (bottom edge) @ variant @page_header` | rule |
| Q1.72 | `toolbar.title.text (= token.text.keyed.heading_1, elision End), toolbar.title.padding @ variant @global` | rule |
| Q1.72 | `toolbar.bar.padding, toolbar.bar.min_height, toolbar.values.item_gap @ variant @global` | rule |
| Q1.73 | `toolbar.separator.stroke, toolbar.separator.bindings (Surface(Header)) @ variant @global {scrolled} (before the title while the columns scroll)` | rule |
| Q1.73 | `button.icon.glyph (ThemeIcon) @ variants {tool,icon_only,previous} context toolbar.bar[@global] / variants {tool,icon_only,next} context toolbar.bar[@global]` | rule |
| Q1.74 | `toolbar.bar.transitions (Geometry: the header's height; = token.motion.durations.long, its timing) @ variant @global` | rule |
| Q1.74 | `toolbar.bar.transitions (Stroke: the separator's fade as the controls hide; = token.motion.durations.long, its timing) @ variant @global` | rule |
| Q1.74 | `toolbar.bar.max_height (= add(mul(token.params.layout.grid_unit, <n>), <paddings>)) @ variant @global` | rule |
| Q1.75 | `button.icon.glyph (ThemeIcon overflow-menu) @ variant overflow context toolbar.bar cond flavor:kirigami` | rule |
| Q1.75 | `button.menu_indicator (absent = true) @ variant overflow context toolbar.bar cond flavor:kirigami` | rule |
| Q1.75 | `button.values.menu_placement (QtQuickMenu, below the button, ADD-A21) @ variant overflow context toolbar.bar cond flavor:kirigami` | placement |
| Q1.75 | `button.values.menu_placement (above the button) @ variant overflow context toolbar.bar variant bottom cond flavor:kirigami` | rule |
| Q1.76 | `button.body.{fill,stroke,shadow} @ variant tool context toolbar.bar cond flavor:kirigami` (ActionToolBar's default `flat: true`: its tool buttons drawn flat at rest) | rule |
| Q1.76 | `button.values.label_position (TextBesideIcon) @ variant tool context toolbar.bar cond flavor:kirigami` | rule |
| Q1.76 | `toolbar.separator.stroke, toolbar.separator.min_height @ cond flavor:kirigami (ToolSeparator, Q1.70)` | rule |
| Q1.76 | `toolbar.values.tooltip_shows_shortcut @ cond flavor:kirigami` | rule |
| Q1.77 | `list.row_background.fill @ {hover} cond flavor:kirigami / {selected} cond flavor:kirigami` | rule |
| Q1.77 | `list.row_background.fill @ pos parity even variant alternating_rows cond flavor:kirigami (ADD-B06; index % 2 of a zero-based index tints the second, fourth … row, DefaultListItemBackground.qml:21-28)` | rule |
| Q1.78 | `list.row_background.fill @ {focus_visible} cond flavor:kirigami (the hover look) / {pressed} cond flavor:kirigami (the selected look)` | rule |
| Q1.78 | `list.values.focus_visible_when (ADD-A16)` | field |
| Q1.79 | `list.row.padding, list.values.row_background_inset, list.values.leading_gap @ cond flavor:kirigami` (ItemDelegate's `spacing`, the icon-to-label gap, `ItemDelegate.qml:28`, `:52-53`; rows are spaced by their insets, not a gap) | rule |
| Q1.79 | `list.row.padding (its bottom equal to its top: the last row keeps the full padding, the others the small spacing, `ItemDelegate.qml:35`) @ pos last cond flavor:kirigami` | rule |
| Q1.79 | `list.values.row_background_inset @ variant table cond flavor:kirigami` | rule |
| Q1.80 | `list.leading_icon.icon.size (= token.icons.sizes.keyed.small_medium.size) @ cond flavor:kirigami / (= token.icons.sizes.keyed.medium.size) @ variant grid cond flavor:kirigami (text under the icon)` | rule |
| Q1.81 | `list.row.text.fill @ {selected} cond flavor:kirigami / {pressed} cond flavor:kirigami / {disabled} cond flavor:kirigami` | rule |
| Q1.82 | `list.values.elided_tooltip` | field |
| Q1.82 | `tooltip.values.show_delay @ context list.row cond flavor:kirigami` | rule |
| Q1.82 | `tooltip.values.show_on_press, tooltip.values.show_delay (= token.behavior.long_press) @ context list.row cond tablet_mode cond flavor:kirigami` | rule |
| Q1.83 | `checkbox @ context list.row cond flavor:kirigami / radio @ context list.row cond flavor:kirigami / switch @ context list.row cond flavor:kirigami` | rule |
| Q1.83 | `list.row.text.fill @ {pressed} !{checked} cond flavor:kirigami / {selected} variant check cond flavor:kirigami` | rule |
| Q1.84 | `list.values.presentations (@swipe: the row's slide; Timing::Platform(KirigamiVelocity), Qt Quick's SmoothedAnimation, CONTRA-2) @ cond flavor:kirigami` | rule |
| Q1.84 | `list.row_background (absent = true) @ {swiped} cond flavor:kirigami` | rule |
| Q1.85 | `list.disclosure.glyph, list.branch_line.stroke @ cond flavor:kirigami` | rule |
| Q1.85 | `list.values.indent_per_level, list.values.disclosure_gap, list.row.padding @ cond flavor:kirigami` | rule |
| Q1.86 | `list.header_section.fill, list.header_section.text, list.sort_indicator.glyph @ cond flavor:kirigami` | rule |
| Q1.86 | `list.values.header_fills_trailing` | field |
| Q1.87 | `list.secondary_text.text (font = token.text.keyed.smallest_readable; fill = kirigami_lerp(widget.row.text.fill, surface.background, 0.25), TitleSubtitle.qml:69-71) @ cond flavor:kirigami` | rule |
| Q1.87 | `list.secondary_text.text.fill @ {selected} cond flavor:kirigami` | rule |
| Q1.87 | `list.leading_icon.icon.size (= token.icons.sizes.keyed.medium.size) @ variant with_subtitle cond flavor:kirigami / (= token.icons.sizes.keyed.small_medium.size) @ !variant with_subtitle cond flavor:kirigami` | rule |
| Q1.87 | `list.values.leading_gap @ cond flavor:kirigami` (mediumSpacing) | rule |
| Q1.88 | `list.swipe_action.opacity` (the actions hidden at rest) | field |
| Q1.88 | `list.swipe_action.opacity @ context list.row {hover}` | rule |
| Q1.88 | `list.swipe_action.transitions (Opacity; = token.motion.durations.very_short, its timing)` | field |
| Q1.88 | `list.swipe_handle.icon.size (= token.icons.sizes.keyed.small_medium.size; the overflow-menu icon by side)` | field |
| Q1.89 | `list.swipe_area.fill (= qt_darker, ADD-A50)` | field |
| Q1.89 | `list.swipe_area.fill @ {pressed}` | rule |
| Q1.89 | `list.swipe_area.shadow (the edge shadow on the top and leading edges)` | field |
| Q1.89 | `list.values.presentations (@swipe: the snap; = token.motion.durations.long, its timing)` | field |
| Q1.90 | `popover.body.shadow, popover.body.padding @ cond flavor:kirigami` | rule |
| Q1.91 | `popover.values.presentations (Appear, Disappear: Fade; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q1.92 | `splitter.divider.fill, splitter.divider.stroke @ cond flavor:kirigami (Breeze's splitter element)` | rule |
| Q1.92 | `splitter.values.hit_extension @ cond flavor:kirigami / {hover} cond flavor:kirigami` | rule |
| Q1.93 | `splitter.divider.bindings (Surface(Header)) @ variant @column_view` | rule |
| Q1.93 | `splitter.values.hit_extension @ variant @column_view` | rule |
| Q1.93 | `layout.values.column_min_width @ variant @column_view` | rule |
| Q1.94 | `layout.values.column_width @ variant @column_view` | rule |
| Q1.94 | `layout.values.presentations (@column_scroll: = token.motion.durations.very_long, its timing) @ variant @column_view` | rule |
| Q1.95 | `separator.line.stroke (Normal: = kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast)) @ cond flavor:kirigami` (`Separator.qml:65-69`) | rule |
| Q1.95 | `separator.line.stroke (Light: = kirigami_lerp(surface.background, surface.foreground, div(token.params.color.frame_contrast, 2)), Kirigami's lightFrameContrast, platformtheme.cpp:762-767) @ variant @light cond flavor:kirigami` | rule |
| Q1.96 | `separator.line.stroke (= the bound surface's roles)` | field |
| Q1.96 | `separator.line.bindings (Surface(Header)) @ context any toolbar / context any sidebar / context splitter.divider[@column_view]` | rule |
| Q1.97 | `token.params.layout.small_spacing, token.params.layout.medium_spacing, token.params.layout.large_spacing, token.params.layout.grid_unit (ADD-A44)` | token |
| Q1.98 | `layout.values.container_margin @ variant @qqc2_pane cond flavor:kirigami` (Pane's padding) | rule |
| Q1.98 | `layout.values.container_margin @ variant @page cond flavor:kirigami` (qqc2's Page: no padding) | rule |
| Q1.98 | `layout.values.container_margin @ variants {@page,@kirigami_page} cond flavor:kirigami` (Kirigami's Page: a grid unit, `= token.params.layout.grid_unit`) | rule |
| Q1.98 | `layout.values.container_margin @ cond flavor:kirigami` (a plain qqc2 Control's padding, from the style's `layout*margin` pixel metric) | rule |
| Q1.99 | `switch.values.track_inset (= floor(mul(token.params.layout.grid_unit, <fraction>)))` | field |
| Q1.99 | `switch.track.min_width, switch.track.min_height, switch.thumb.min_height` | field |
| Q1.100 | `switch.track_fill.fill (from the track's start to the thumb's trailing edge, under the thumb when unchecked)` | field |
| Q1.101 | `switch.thumb.stroke (= kirigami_lerp(role.button.background, role.button.foreground, token.params.color.frame_contrast): the indicator's own Button set)` (`private/SwitchIndicator.qml:21-22`, `:30`) | field |
| Q1.101 | `switch.thumb.stroke @ {hover} / {focus_visible}` | rule |
| Q1.101 | `switch.track.stroke @ {checked}` | rule |
| Q1.101 | `switch.track.stroke (the thumb's stroke) @ !{checked}` | rule |
| Q1.102 | `switch.thumb.transitions (Geometry: the handle's slide; Timing::Platform(KirigamiVelocity), the SmoothedAnimation, over = token.motion.durations.short)` | field |
| Q1.102 | `switch.thumb.transitions @ {pressed}` (the `Geometry` transition with a zero duration: the SmoothedAnimation is disabled while pressed, the handle following the pointer) | rule |
| Q1.102 | `switch.thumb.transitions (Fill)` | field |
| Q1.102 | `token.motion.durations.short (Kirigami shortDuration)` | token |
| Q1.103 | `switch.values.label_gap; switch.body.padding` | field |
| Q1.103 | `switch.label.opacity @ {disabled}` | rule |
| Q1.104 | `dialog.scrim.fill @ cond flavor:kirigami` | rule |
| Q1.104 | `dialog.values.presentations (Appear, Disappear: part scrim, Fade; ADD-B09)` | field |
| Q1.105 | `dialog.values.presentations (Appear, Disappear: Fade; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q1.106 | `dialog.body.shadow @ cond flavor:kirigami` | rule |
| Q1.107 | `dialog.body.corners, dialog.body.stroke (Absent), dialog.{header,title,subtitle} (absent = true) @ variant separate_window cond flavor:kirigami` (the title nested in the header has its own rule: Nesting) | rule |
| Q1.108 | `dialog.body.padding @ cond flavor:kirigami` | rule |
| Q1.108 | `dialog.body.padding @ {@scroll_view_content} cond flavor:kirigami` | rule |
| Q1.108 | `dialog.values.window_margin @ cond flavor:kirigami` | rule |
| Q1.109 | `dialog.action_area.padding, dialog.action_area.min_height @ cond flavor:kirigami` | rule |
| Q1.109 | `dialog.action_area.stroke (above the buttons) @ {overflowing} cond flavor:kirigami (ADD-A13)` | rule |
| Q1.109 | `dialog.values.action_layout (its alignment) @ cond flavor:kirigami` | rule |
| Q1.110 | `dialog.values.standard_button_icons` | field |
| Q1.110 | `dialog.values.button_icons @ cond flavor:kirigami` | rule |
| Q1.111 | `dialog.values.presentations (Appear: Fade with a Translate rise, ADD-B10, ADD-A25) @ cond flavor:kirigami` | rule |
| Q1.112 | `dialog.scrim.fill, dialog.body.shadow, dialog.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast)), dialog.body.bindings (Surface(View)) @ cond flavor:kirigami` (the stroke: qqc2's `Dialog.qml:85`, Kirigami's `Dialog.qml:341`) | rule |
| Q1.112 | `dialog.scrim.transitions (Opacity; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q1.113 | `dialog.body.max_width, dialog.body.max_height (= sub(element.window_width, <margins>), sub(element.window_height, <margins>), ADD-A78) @ cond flavor:kirigami` | rule |
| Q1.113 | `dialog.values.window_margin @ cond flavor:kirigami` | rule |
| Q1.114 | `dialog.header.padding @ cond flavor:kirigami` | rule |
| Q1.114 | `dialog.header.stroke @ {overflowing} cond flavor:kirigami` | rule |
| Q1.114 | `dialog.title.text (= token.text.keyed.heading_1, elision End) @ cond flavor:kirigami` | rule |
| Q1.114 | `tooltip @ context dialog.title cond flavor:kirigami (the elided title's full text)` | rule |
| Q1.114 | `button.icon.glyph (ThemeIcon) @ variant close context dialog.header cond flavor:kirigami / variant close {hover} context dialog.header cond flavor:kirigami (the other icon, while the button is hovered)` | rule |
| Q1.115 | `dialog.action_area.padding, dialog.action_area.min_height (the empty footer's buffer) @ cond flavor:kirigami` | rule |
| Q1.115 | `dialog.action_area.stroke @ {overflowing} cond flavor:kirigami` | rule |
| Q1.115 | `button.body.fill, button.body.stroke @ context dialog.action_area variant @flat_footer (the buttons drawn flat)` | rule |
| Q1.116 | `dialog.icon.glyph (ThemeIcon), dialog.icon.icon.size (= token.icons.sizes.keyed.huge.size) @ variants {alert,warning} / variants {alert,info} / variants {alert,error} / variants {alert,success}` | rule |
| Q1.116 | `dialog.title.text, dialog.subtitle.text @ variant alert cond flavor:kirigami` | rule |
| Q1.116 | `dialog.body.padding @ variant alert cond flavor:kirigami` | rule |
| Q1.117 | `dialog.values.natural_size, dialog.body.padding @ variant @menu_dialog` | rule |
| Q1.117 | `list.row.min_height, list.row.padding, list.leading_icon.icon.size @ context dialog.body variant @menu_dialog` | rule |
| Q1.118 | `dialog.values.natural_size (= min(mul(token.params.layout.grid_unit, <n>), …)) @ variant @search_dialog` | rule |
| Q1.118 | `input.body.stroke (Absent), input.body.margin (the field's smallSpacing margins) @ variant flat context dialog.body variant @search_dialog` | rule |
| Q1.118 | `separator.line.stroke @ context dialog.body variant @search_dialog (the line under the field)` | rule |
| Q1.119 | `spinner.icon.glyph (ThemeIcon process-working-symbolic) @ cond flavor:kirigami` | rule |
| Q1.119 | `spinner.values.loop (rotation; its period not scaled by token.motion.speed_factor) @ cond flavor:kirigami` | rule |
| Q1.120 | `spinner.values.diameter (= mul(token.params.layout.grid_unit, <n>)) @ cond flavor:kirigami` | rule |
| Q1.120 | `spinner.values.presentations (Appear, Disappear: Fade; = token.motion.durations.short, its timing) @ cond flavor:kirigami` | rule |
| Q1.120 | `spinner.values.loop (Absent: stopped) @ cond animations_off cond flavor:kirigami` | rule |
| Q1.121 | `spinner.indicator.glyph (the arc: its stroke's paint and width) @ context any pull_to_refresh cond flavor:kirigami` | rule |
| Q1.122 | `combo_box.body.fill, combo_box.body.stroke, combo_box.arrow.glyph, combo_box.label.text @ cond flavor:kirigami (the same Breeze element)` | rule |
| Q1.123 | `combo_box.body.bindings (Surface(Button))` | field |
| Q1.123 | `combo_box.body.bindings (Surface(View)) @ variant editable` | rule |
| Q1.124 | `combo_box.body.padding @ cond flavor:kirigami` | rule |
| Q1.124 | `combo_box.body.padding (its trailing padding) @ variant editable cond flavor:kirigami` | rule |
| Q1.124 | `combo_box.body.min_width (the content's minimum) @ cond flavor:kirigami` | rule |
| Q1.125 | `menu.body, menu.item @ context combo_box cond flavor:kirigami (the qqc2 Menu: Q1.23-Q1.32 apply)` | rule |
| Q1.125 | `menu.values.presentations @ context combo_box cond flavor:kirigami` | rule |
| Q1.126 | `card.body.fill (= kirigami_tint, ADD-A52) @ {hover} cond flavor:kirigami / {pressed} cond flavor:kirigami / {checked} cond flavor:kirigami` | rule |
| Q1.127 | `card.body.shadow @ cond flavor:kirigami` | rule |
| Q1.128 | `card.body.shadow (a zero-blur lift layer, = qt_darker, ADD-A50) @ cond flavor:kirigami` | rule |
| Q1.128 | `card.body.shadow (without the lift layer) @ {pressed} cond flavor:kirigami` | rule |
| Q1.129 | `card.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), DefaultCardBackground.qml:70), card.body.corners @ cond flavor:kirigami` | rule |
| Q1.129 | `card.body.bindings (Surface(View)) @ cond flavor:kirigami` | rule |
| Q1.130 | `card.body.padding @ cond flavor:kirigami` | rule |
| Q1.130 | `card.values.section_gap` | field |
| Q1.131 | `card.banner_title.text (= token.text.keyed.heading_1; its fill over images), card.banner_title.icon.size (= token.icons.sizes.keyed.large.size) @ variant @banner` | rule |
| Q1.131 | `card.banner_band.fill, card.banner.padding @ variant @banner / card.banner.padding (with the icon) @ variant @banner` | rule |
| Q1.131 | `card.values.banner_band_height_factor` | field |
| Q1.131 | `checkbox @ context card.banner (the optional check box)` | rule |
| Q1.132 | `toolbar.bar @ variant bottom context card.body cond flavor:kirigami` | rule |
| Q1.133 | `layout.values.max_columns, layout.values.column_min_width, layout.values.column_max_width, layout.values.widget_gap @ variant @cards` | rule |
| Q1.133 | `layout.values.widget_gap, layout.values.container_margin @ variant @cards_list` | rule |
| Q1.134 | `link.text.text.fill @ {pressed} (= qt_darker(surface.link, ..))` | rule |
| Q1.134 | `link.text.text.fill @ {disabled} (= surface.foreground)` | rule |
| Q1.135 | `link.text.text.font.weight @ {focus_visible}` | rule |
| Q1.135 | `link.text.focus (Absent)` | field |
| Q1.136 | `link.icon.icon (size = kirigami_label_icon_size(body_font_height), Q1.1; fill = surface.link; the open-link theme icon with its fallback) @ variant @url` | rule |
| Q1.136 | `link.values.icon_text_gap` | field |
| Q1.136 | `link.values.shows_url_tooltip @ variant @url` | rule |
| Q1.136 | `tooltip.values.show_delay @ context link cond flavor:kirigami` | rule |
| K2.1 | `menu_bar.item.padding` | field |
| K2.1 | `menu_bar.item.fill, menu_bar.item.corners @ {hover} cond system_setting:menu_item_strong_focus / {pressed} cond system_setting:menu_item_strong_focus` (the fill only under strong focus, `breezestyle.cpp:5859`; the weak-focus line otherwise, below) | rule |
| K2.1 | `menu_bar.item_label.text.fill @ {pressed} cond system_setting:menu_item_strong_focus` | rule |
| K2.1 | `menu_bar.item_label.stroke @ {hover} / {pressed} !cond system_setting:menu_item_strong_focus` (the weak-focus line under the item's text, breezestyle.cpp:5921-5936: the same setting as the menu's, K1.62) | rule |
| K2.1 | `menu_bar.bar.bindings (Surface(Header))` | field |
| K2.1 | `role.header.background @ cond window_inactive (the inactive Header group, K1.150)` | role |
| K2.3 | `stepper.increment.min_width, stepper.decrement.min_width (the arrow column)` | field |
| K2.3 | `stepper.increment_glyph.glyph, stepper.decrement_glyph.glyph (fill = arrowColor)` | field |
| K2.3 | `stepper.increment_glyph.glyph.fill @ context stepper.increment {hover} / context stepper.increment {disabled}; stepper.decrement_glyph.glyph.fill @ context stepper.decrement {hover} / context stepper.decrement {disabled}` | rule |
| K2.3 | `stepper.increment_glyph.transitions, stepper.decrement_glyph.transitions (Glyph: the hover fade)` | field |
| K2.3 | `stepper.values.button_layout` | field |
| K2.3 | `stepper.body.stroke (Absent) @ variant flat; input @ context stepper (the line-edit frame)` | rule |
| K2.4 | `button @ variant previous context calendar.header / variant next context calendar.header; button @ context calendar.header (the month and year buttons)` | rule |
| K2.4 | `calendar.weekday_label.text, calendar.week_number.text` | field |
| K2.4 | `calendar.day.fill @ {today} / {selected}; calendar.day_label.text.fill @ {@non_working_day} / {other_month}` | rule |
| K2.5 | `dial.groove.fill, dial.value_arc.fill, dial.values.stroke_width (the arcs' width, Slider_ControlThickness), dial.values.cap` | field |
| K2.5 | `dial.handle.fill, dial.handle.stroke, dial.handle.shadow (the slider's handle)` | field |
| K2.5 | `dial.handle.stroke @ {hover}` | rule |
| K2.6 | `component.lcd_number.frame.stroke, component.lcd_number.frame.bindings (the palette colours its digits take)` | component |
| K2.6 | `component.lcd_number.segment.fill, component.lcd_number.segment.stroke @ variant @outline / variant @filled / variant @flat` | component |
| K2.7 | `button.body.fill, button.body.stroke @ variant @command_link` | rule |
| K2.7 | `button.icon.icon @ variant @command_link` | rule |
| K2.7 | `button.label.text (= token.text.keyed.command_link_title) @ variant @command_link` | rule |
| K2.7 | `button.subtitle.text (wrap Word) @ variant @command_link` | rule |
| K2.7 | `button.values.icon_text_gap @ variant @command_link` | rule |
| K2.8 | `group_box.body.fill, group_box.body.stroke, group_box.body.corners, group_box.title.text` | field |
| K2.8 | `group_box.body.fill, group_box.body.stroke (Absent), group_box.title.text (= token.text.keyed.flat_group_title) @ variant flat` | rule |
| K2.8 | `checkbox @ context any group_box (the checkable title)` | rule |
| K2.8 | `group_box.focus_line.{stroke,transitions} @ {focus_visible}` (the line and its fade) | rule |
| K2.9 | `component.tool_box.page_button.min_width` (`ToolBox_TabMinWidth`; one home) | component |
| K2.9 | `component.tool_box.outline.stroke, component.tool_box.outline.corners` (the tab-shaped outline round every tab, in the frame outline colour at rest) | component |
| K2.9 | `component.tool_box.outline.stroke @ {hover} / {selected}` (the hover colour; the open page's focus colour, a later rule: Breeze takes no hover on the open page) | component |
| K2.9 | `component.tool_box.outline.transitions (Stroke: the hover fade)` | component |
| K2.9 | `component.tool_box.page_icon.icon, component.tool_box.page_label.text` | component |
| K2.9 | `component.tool_box.values.item_gap, component.tool_box.values.tab_margin_width` | component |
| K2.10 | `component.dock_widget.title_bar.padding` | component |
| K2.10 | `component.dock_widget.title.text (elision End)` | component |
| K2.10 | `component.dock_widget.frame.stroke` | component |
| K2.10 | `button.icon.glyph (ThemeIcon or the standard pixmap) @ variant @float context component.dock_widget / variant close context component.dock_widget` | rule |
| K2.10 | `button.body.fill (Absent: frameless), button.icon.glyph.fill @ variant @float {hover} context component.dock_widget / variant @float {pressed} context component.dock_widget / variant @float {disabled} context component.dock_widget / variant close {hover} context component.dock_widget / variant close {pressed} context component.dock_widget / variant close {disabled} context component.dock_widget` (the buttons' own states) | rule |
| K2.10 | `component.dock_widget.separator.stroke; component.dock_widget.separator (absent = true) @ variant @vertical_title_bar` (Breeze draws the line under a horizontal title bar only, `st:7545-7552`) | component |
| K2.10 | `component.dock_widget.frame.stroke (Absent) @ !cond system_setting:dock_widget_draw_frame` (breezerc `DockWidgetDrawFrame`, a read switch) | component |
| K2.10 | `reader:kde:breezerc [Style] DockWidgetDrawFrame` | reader |
| K2.11 | `component.mdi_sub_window.title_bar.fill (= role.@wm_active_background), component.mdi_sub_window.title_bar.corners (top rounded), component.mdi_sub_window.title.text (fill = role.@wm_active_foreground)` | component |
| K2.11 | `component.mdi_sub_window.title_bar.fill (= role.@wm_inactive_background), component.mdi_sub_window.title.text.fill (= role.@wm_inactive_foreground) @ !{@active}` | rule |
| K2.11 | `role.@wm_active_background, role.@wm_active_foreground, role.@wm_inactive_background, role.@wm_inactive_foreground` (KDE's `[WM]` colours, keyed roles: the preset states Breeze's scheme files' `[WM]` values, `BreezeLight.colors:194-200`; the reader kdeglobals' `[WM]`, which `applyScheme()` always writes, the scheme's `[WM]` or its `Header` set's colours, `colorsapplicator.cpp:287-309`, with the accent's override, `:318-338`; where kdeglobals has none, Breeze's fallback, references to the `Selection` set's active and disabled background and foreground, `breezehelper.cpp:98-102`, in the reader's `for_preset.kde-breeze` part; the accent layer states them as `kde_accent_wm(<key>)`, Plasma's derivation, over `input.kde_scheme.wm.*`; a mapping palette states them and a KDE port converts `[WM]`, S2.2, S9.1, S12, S14) | role |
| K2.11 | `reader:kde:kdeglobals [WM] activeBackground, activeForeground, inactiveBackground, inactiveForeground` (where kdeglobals has no `[WM]`, the `Selection` references Breeze falls back to, `breezehelper.cpp:98-102`) | reader |
| K2.11 | `component.mdi_sub_window.frame.stroke @ !{@active}` | rule |
| K2.11 | `component.mdi_sub_window.active_separator.stroke @ {@active}; component.mdi_sub_window.active_separator (absent = true) @ !{@active}` | rule |
| K2.11 | `button.icon.glyph @ variant minimize context component.mdi_sub_window / variant maximize context component.mdi_sub_window / variant restore context component.mdi_sub_window / variant close context component.mdi_sub_window` (the sub-window's caption buttons, Nested controls) | rule |
| K2.11 | `component.mdi_sub_window.frame.shadow` | component |
| K2.11 | `component.mdi_sub_window.values.title_margin_width` | component |
| K2.12 | `rubber_band.body.stroke, rubber_band.body.corners` | field |
| K2.12 | `rubber_band.body.fill @ context any list / variant in_canvas / variant dock_target` (the content variants the connector sets where the band's host is a QGraphicsView or a QMainWindow's dock drop zone, hosts the model has no widget for, ADD-A61) | rule |
| K2.13 | `banner.body.fill (the default scheme's colours stated, over the parent window) @ variant info / variant success / variant warning / variant error cond flavor:widgets` | rule |
| K2.13 | `banner.body.stroke, banner.body.corners (= mul(<radius>, <factor>)) @ variant @inline cond flavor:widgets; banner.inner (absent = true) @ cond flavor:widgets` (KMessageWidget draws one rounded rectangle with its pen, `kmessagewidget.cpp:364-377`) | rule |
| K2.13 | `banner.edge_line.stroke, banner.body.corners (none) @ variant @header cond flavor:widgets / variant @footer cond flavor:widgets` | rule |
| K2.13 | `banner.icon.icon.size (= token.icons.sizes.toolbar.size)` | field |
| K2.13 | `button @ variant close context banner` | rule |
| K2.13 | `banner.values.presentations (Appear, Disappear: the height and Fade, its timing quadratic in the timeline's progress; = token.motion.durations.message_widget)` | field |
| K2.14 | `list.leading_icon.icon.size (= the icon view's size) @ context component.page_view` | rule |
| K2.14 | `component.title_widget.{container.padding,title.text} @ context component.page_view` (the page title header is KTitleWidget, one home) | rule |
| K2.14 | `component.page_view.header_separator.stroke` | component |
| K2.14 | `input @ context component.page_view; component.page_view.search_container.padding (its container's margins, per side)` (the search line edit) | component |
| K2.15 | `component.title_widget.title.text (= token.text.keyed.heading_<n>) @ variant @level_1 … (one per declared level of K1.270)` | component |
| K2.15 | `component.title_widget.comment.text.fill, component.title_widget.comment.fill @ variant @info / variant @warning / variant @error` (one look for the three, from the palette's highlighted text and highlight as Qt applies the source's style sheet, value phase; `MessageType` has no positive type, `ktitlewidget.h:115-120`) | component |
| K2.15 | `component.title_widget.icon.icon` | component |
| K2.16 | `component.multi_tab_bar.tab_button.fill, component.multi_tab_bar.tab_button.padding` | component |
| K2.16 | `component.multi_tab_bar.tab_button.fill @ {checked} / {hover} / {pressed}` | component |
| K2.16 | `component.multi_tab_bar.separator.stroke; component.multi_tab_bar.separator (absent = true) @ variant @top / variant @bottom` (Breeze draws the line beside a vertical bar only, `st:1407-1409`) | component |
| K2.16 | `component.multi_tab_bar.tab_button.padding @ variant @left / variant @right (the vertical tabs)` | component |
| K2.16 | `component.multi_tab_bar.tab_icon.icon, component.multi_tab_bar.tab_label.text` | component |
| K2.17 | `splitter.collapser.{fill,opacity} / @ {hover} / @ {pressed}` (KSplitterCollapserButton, a part of the splitter it sits on) | rule |
| K2.17 | `splitter.collapser_arrow.glyph @ variant @left / variant @right / variant @top / variant @bottom` | rule |
| K2.17 | `splitter.values.presentations (Appear, Disappear, queried with the collapser: Fade; its duration the source's)` | field |
| K2.18 | `component.led.body.fill, component.led.body.stroke, component.led.body.corners @ variant @round / variant @rectangular / variant @raised / variant @sunken / variant @flat` | component |
| K2.18 | `component.led.body.fill (= element.color) @ {@on} / (= qt_darker(element.color, values.dark_factor)) @ !{@on}; component.led.values.{default_color,dark_factor}` (the application's colour, KLed's default where it sets none) | component |
| K2.19 | `rating.star.glyph` | field |
| K2.19 | `rating.star.glyph @ {filled} / {half} / {preview}` | rule |
| K2.19 | `rating.values.half_steps` | field |
| K2.20 | `color_button.body.fill, color_button.body.stroke (the push button's frame)` | field |
| K2.20 | `color_button.swatch.corners, color_button.swatch.stroke, color_button.body.padding` (the swatch at the push button's contents rectangle less half `PM_ButtonMargin`, `kcolorbutton.cpp:159-161`) | field |
| K2.21 | `group_box.body.fill, group_box.body.stroke, group_box.body.corners @ variant @styled_panel` | rule |
| K2.21 | `group_box.body.stroke @ {hover} variant @styled_panel / {focused} variant @styled_panel` | rule |
| K2.21 | `scroll_view.frame (drawn_as = { widget = "group_box", variants = ["@styled_panel"] }: Breeze's frame of a scroll area, the substitute carrying the view's hover and focus, breezestyle.cpp:3919-3975; qqc2's framed ScrollView the same styled panel, kquickstyleitem.cpp:496-503)` | field |
| K2.21 | `group_box.body.stroke (= role.@shade_light, role.@shade_dark …, K3.53) @ variant @box / variant @panel / variant @win_panel` | rule |
| K2.22 | `wizard.header.padding, wizard.title.text, wizard.subtitle.text, wizard.banner.fill, wizard.action_area.padding` (the classic style, `kde-breeze`'s base) | field |
| K2.23† | `date_picker.field @ variant with_popup cond flavor:widgets` (`drawn_as = { widget = "combo_box", variants = ["editable"] }`: KDateComboBox paints the editable `CC_ComboBox`, its constructor calling `setEditable(true)` whatever its options, kdatecombobox.cpp:214, as KTimeComboBox, K2.24†) `/ variant text_field cond flavor:widgets` (`drawn_as = { widget = "stepper", variants = [] }`: QDateEdit without the popup, K2.3) | rule |
| K2.23† | `combo_box @ variant editable context date_picker` (the editable combo box's own look, K1.240, in context) `/ stepper @ context date_picker` | rule |
| K2.23† | `menu @ context date_picker variant with_popup` (KDateComboBox's popup is a QMenu: the `menu` element `popup` is drawn as draws the box); `calendar @ context menu context date_picker; menu.item @ context date_picker (the word items), menu.separator @ context date_picker` | rule |
| K2.23† | `date_picker.popup @ variant with_popup cond flavor:widgets` (`drawn_as = { widget = "menu", variants = [] }`: the `menu` draws the one box) | rule |
| K2.23† | `calendar.weekday_label.text (its weight restated), calendar.weekday_label.fill (= role.window.background), calendar.weekday_label.stroke (the line under it)` | field |
| K2.23† | `calendar.weekday_label.text.fill (a stated colour, not a role) @ {@non_working_day}; calendar.day_label.text.fill (a stated colour) @ {@observance_day}` | rule |
| K2.23† | `calendar.day_label.text (its weight restated; fill = the visited-link role) @ {today}` | rule |
| K2.23† | `calendar.day.fill, calendar.day_label.text.fill @ {selected} / {hover} (the item-view selection, K1.170)` | rule |
| K2.23† | `calendar.day_label.text.fill @ {other_month}; calendar.day.fill @ {disabled}` (a day outside the allowed range) | rule |
| K2.23† | `calendar.weekday_label.text.fill, calendar.day_label.text.fill (the stated colours dropped) @ cond contrast_high (R25.2, CONTRA-1; which the application sets by `Theme::set_contrast` where the KDE reader reports none, S16)` | rule |
| K2.24† | `time_picker.field @ variant with_popup cond flavor:widgets` (`drawn_as = { widget = "combo_box", variants = ["editable"] }`: KTimeComboBox's editable combo, K1.240) `/ variant text_field cond flavor:widgets` (`drawn_as = { widget = "stepper", variants = [] }`: QTimeEdit, K2.3) | rule |
| K2.24† | `combo_box @ variant editable context time_picker; stepper @ context time_picker` (the look of the widget each trigger is drawn as, in context) | rule |
| K2.24† | `time_picker.values.invalid_entry (the error message box, K1.224†)` | field |
| K2.24† | `layout.values.widget_gap (the date and time fields paired, KDateTimeEdit)` | field |
| K2.25† | `color_picker.plane.stroke, color_picker.plane.cursor @ variant @qt_dialog` | rule |
| K2.25† | `color_picker.{crosshair_line_horizontal,crosshair_line_vertical}.{fill,min_width,min_height}, color_picker.values.crosshair_spans_plane (false) @ variant @qt_dialog` (QColorDialog's cross of two short bars centred on the point, K2.25†'s row) | rule |
| K2.25† | `color_picker.value_strip.stroke (the shade panel: role.@shade_light, role.@shade_dark) @ variant @qt_dialog` | rule |
| K2.25† | `color_picker.swatch.stroke, color_picker.swatch.focus @ variant @qt_dialog` | rule |
| K2.25† | `stepper @ context color_picker; input @ context color_picker; button @ context color_picker (the screen picker)` | rule |
| K2.25† | `combo_box @ context any color_button (KColorCombo's colour list)` | rule |
| K2.26† | `file_chooser.preview_pane.fill, file_chooser.values.icon_size_slider` | field |
| K2.26† | `sidebar @ context any file_chooser; breadcrumb @ context any file_chooser; list @ context any file_chooser; slider @ context any file_chooser; combo_box @ context any file_chooser; button @ context any file_chooser` | rule |
| K2.27† | `breadcrumb.crumb.padding` | field |
| K2.27† | `breadcrumb.crumb_label.text.fill @ {@navigator_active} / !{@navigator_active} / !{@navigator_active} !{hover}` | rule |
| K2.27† | `breadcrumb.crumb_label.text (its weight restated) @ {current}` | rule |
| K2.27† | `breadcrumb.crumb.fill, breadcrumb.crumb.stroke @ {hover} / {drop_target} / {open} (the flat button panel)` | rule |
| K2.27† | `breadcrumb.separator.glyph @ {hover}; menu @ context breadcrumb (the sub-folder menu)` | rule |
| K2.27† | `breadcrumb.crumb_label.text.elision (Fade, ADD-A38)` | field |
| K2.27† | `button @ variant menu context breadcrumb (the places selector); input @ context breadcrumb; breadcrumb.values.editable` | rule |
| K2.28† | `font_chooser.font_label.{fill,stroke,corners,padding}` (KFontRequester's sunken styled-panel label, its frame forced on Breeze, `_breeze_force_frame`) | field |
| K2.28† | `font_chooser.font_label.text (in the chosen font); button @ context font_chooser (the document-edit button)` | rule |
| K2.28† | `font_chooser.preview.text, font_chooser.values.preview_min_lines` | field |
| K2.28† | `list @ context any font_chooser; stepper @ context any font_chooser; checkbox @ context any font_chooser` | rule |
| K2.29† | `menu.body.stroke @ context autocomplete; autocomplete.values.placement (QtCompletionBox: width = the field's, AnchorWidth; max_visible_items)` | placement |
| K2.29† | `scrollbar @ context autocomplete` | rule |
| K2.29† | `autocomplete.values.mode (Popup, Inline)` | field |
| K2.30† | `input.body.fill @ context find_bar {@match} / context find_bar {invalid} / context find_bar {@neutral}` | rule |
| K2.30† | `find_bar.status_label.text.fill @ {invalid}` | rule |
| K2.30† | `code_editor.find_match.fill, code_editor.replace_match.fill` | field |
| K2.31† | `context_help.body.fill, context_help.body.stroke, context_help.body.corners, context_help.body.padding` | field |
| K2.31† | `context_help.inner_frame.stroke (= role.@shade_dark, K3.53)` | field |
| K2.31† | `context_help.text.text` | field |
| K2.31† | `context_help.body.shadow (Absent: Qt's hatched shadow needs the DropShadow hint the KDE platform theme leaves unset; Breeze's popup shadow, K3.2, to verify)` | field |
| K2.31† | `context_help.values.mode_cursor (WhatsThis, ADD-A59)` | field |
| K2.31† | `context_help.values.placement (QtWhatsThis: by the pointer)` | placement |
| K2.32† | `ruler.values.thickness, ruler.values.tick_lengths` | field |
| K2.32† | `ruler.tick.stroke` | field |
| K2.32† | `ruler.label.text` | field |
| K2.32† | `ruler.marker.glyph (fill = the background colour)` | field |
| Q2.1 | `stepper.body.bindings (Surface(View)), stepper.body.padding @ cond flavor:kirigami` | rule |
| Q2.1 | `stepper.increment.fill @ {hover} cond flavor:kirigami / stepper.decrement.fill @ {hover} cond flavor:kirigami` | rule |
| Q2.2 | `dial.values.default_size @ cond flavor:kirigami` | rule |
| Q2.3 | `button.body.{fill,stroke,corners} @ variant tool context component.delay_button` (the button is Breeze's tool-button panel, `PE_PanelButtonTool`: one home) | rule |
| Q2.3 | `component.delay_button.progress_overlay.fill, component.delay_button.progress_overlay.opacity` | component |
| Q2.3 | `component.delay_button.progress_overlay.transitions (Geometry: = mul(values.delay, sub(1, element.progress)) @ {pressed}; = mul(values.delay, mul(<fraction>, element.progress)) @ !{pressed}; ADD-B12)` (the overlay laid out at the extent the progress heads for, the whole button while pressed and none while released; `element.progress` the progress shown when the pressed state changed; `DelayButton.qml:48-52`) | component |
| Q2.4 | `button.body.min_width, button.body.min_height, button.body.padding, button.icon.icon.size, button.body.corners, button.values.icon_text_gap @ variant circular cond flavor:kirigami` | rule |
| Q2.5 | `button.body.fill @ variant circular cond flavor:kirigami / variant circular {checked} cond flavor:kirigami / variant circular {pressed} cond flavor:kirigami / variants {circular,flat} cond flavor:kirigami` | rule |
| Q2.5 | `button.body.stroke (= kirigami_lerp(role.button.background, role.button.foreground, token.params.color.frame_contrast): the control's palette.button and palette.buttonText, RoundButton.qml:77) @ variant circular !variant flat cond flavor:kirigami / button.body.stroke (= role.selection.background: a flat RoundButton strokes with Kirigami.Theme.highlightColor, the Selection set's background, RoundButton.qml:94) @ variants {circular,flat} cond flavor:kirigami / button.body.stroke (no width, RoundButton.qml:95) @ variants {circular,flat} !{hover} !{focused} !variant @highlighted cond flavor:kirigami / button.body.fill (= kirigami_tint(surface.foreground, with_alpha(kirigami_lerp(role.button.background, role.button.foreground, token.params.color.frame_contrast), 0.8), 1): Qt.tint of the text colour with the border colour at alpha 0.8, RoundButton.qml:86, `Qt.tint(base, tint)` being `kirigami_tint(base, tint, 1)` and `Qt.alpha` `with_alpha`, S2.2, so it holds for a translucent border colour too; ADD-A123) @ {checked} variant circular cond flavor:kirigami / {pressed} variant circular cond flavor:kirigami` | rule |
| Q2.6 | `button.body.fill.append (a linear gloss layer) @ variant circular !variant flat cond flavor:kirigami` | rule |
| Q2.7 | `button.body.shadow @ variant circular cond flavor:kirigami` | rule |
| Q2.7 | `button.body.shadow (Absent) @ variant circular {pressed} / variant circular {checked} / variants {circular,flat}` | rule |
| Q2.7 | `button.body.transform (a translation right and down) @ variant circular {pressed} cond flavor:kirigami / variant circular {checked} cond flavor:kirigami` (a `Translate` on the whole control, `RoundButton.qml:27-30`, not its content) | rule |
| Q2.8 | `button.body.bindings (Surface(Selection)) @ variant circular !variant flat {focused} / variants {circular,@highlighted} !variant flat` | rule |
| Q2.9 | `menu_bar.bar.bindings (Surface(Header)), menu_bar.item.padding, menu_bar.item.min_height, menu_bar.item.min_width @ cond flavor:kirigami` | rule |
| Q2.10 | `menu_bar.item.fill @ {pressed} cond flavor:kirigami / {hover} cond flavor:kirigami` | rule |
| Q2.10 | `menu_bar.item.transitions (Fill; = token.motion.durations.short) @ cond flavor:kirigami` | rule |
| Q2.10 | `menu_bar.item_label.text.fill @ {hover} !{pressed} cond flavor:kirigami` | rule |
| Q2.11 | `page_indicator.dot.fill, page_indicator.dot.min_width, page_indicator.dot.min_height, page_indicator.dot.corners (Full: round dots) @ cond flavor:kirigami` | rule |
| Q2.11 | `page_indicator.dot.opacity @ {selected} / {pressed} !{selected} / !{selected,pressed}` (current, pressed, other: disjoint, so the order cannot override, S8) | rule |
| Q2.11 | `page_indicator.values.dot_gap, page_indicator.body.padding` | field |
| Q2.11 | `page_indicator.dot.transitions (Opacity; = token.motion.durations.short)` | field |
| Q2.12 | `group_box.body.stroke, group_box.body.corners, group_box.body.padding @ variant @quick_frame; group_box.body.fill (Absent: the transparent box) @ variant @quick_frame` | rule |
| Q2.13 | `group_box.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), GroupBox.qml:39), group_box.body.corners (square: no radius), group_box.title.text, group_box.body.padding, group_box.values.title_position (AboveTop) @ variant @quick_group_box` | rule |
| Q2.14 | `navigation_view.values.presentations (@push, @pop, @replace: Slide and Fade together, = token.motion.durations.long, its timing; ADD-B10, ADD-A25, ADD-A26)` | field |
| Q2.15 | `navigation_view.values.presentations (@push, @pop, @replace: Fade and Slide together, each its timing; ADD-B10, ADD-A25) @ variant @layer` | rule |
| Q2.16 | `banner.body.fill (the type colour: InlineMessage's outer rectangle, which shows round `inner` as the border, `InlineMessage.qml:16-33`); banner.body.stroke (Absent) @ variant info / variant success / variant warning / variant error cond flavor:kirigami` | rule |
| Q2.17 | `banner.inner.fill (the window background with the type colour's layer over it, `InlineMessage.qml:35-58`) @ variant info / variant success / variant warning / variant error cond flavor:kirigami` | rule |
| Q2.17 | `banner.inner.corners (each corner = mul(widget.body.corners.<corner>, values.inner_corner_factor)), banner.values.inner_corner_factor @ cond flavor:kirigami` | rule |
| Q2.18 | `banner.body.corners, banner.inner.margin (its inset from the body on the sides the position leaves open, `InlineMessage.qml:37-45`) @ variant @inline cond flavor:kirigami / variant @header cond flavor:kirigami / variant @footer cond flavor:kirigami` | rule |
| Q2.19 | `banner.body.padding` | field |
| Q2.19 | `banner.icon.icon.size (= token.icons.sizes.keyed.small_medium.size), banner.icon.glyph (ThemeIcon) @ variant info / variant success / variant warning / variant error` | rule |
| Q2.19 | `banner.values.content_gap (the text's leading margin)` | field |
| Q2.19 | `banner.values.icon_alignment (Start: the icon top-aligned) @ {multiline}` | rule |
| Q2.20 | `banner.action_area.opacity; toolbar @ context banner (the non-flat action toolbar)` | rule |
| Q2.20 | `banner.values.actions_wrap` | field |
| Q2.20 | `button.icon.glyph (ThemeIcon dialog-close) @ variant close context banner` | rule |
| Q2.21 | `banner.values.presentations (Appear, Disappear: the height, = token.motion.durations.long; Fade, = token.motion.durations.short; the content's Fade, step part, twice that)` | field |
| Q2.22 | `chip.body.fill` | field |
| Q2.22 | `chip.body.fill @ {checked} / {pressed}` | rule |
| Q2.23 | `chip.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), DefaultChipBackground.qml:30), chip.body.corners` | field |
| Q2.23 | `chip.body.stroke @ {hover} / {checked} / {pressed}` | rule |
| Q2.24 | `chip.leading_icon.icon.size (= token.icons.sizes.keyed.small.size), chip.body.padding, chip.values.icon_text_gap; chip.body.padding @ !variant with_icon` (the extra padding without an icon) | rule |
| Q2.24 | `button.icon.glyph (ThemeIcon), button.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100) @ variant remove context chip` | rule |
| Q2.24 | `chip @ variant @interactive / variant closable` | rule |
| Q2.25 | `badge.body.stroke, badge.body.fill @ variant info / variant success / variant warning / variant error` | rule |
| Q2.26 | `badge.label.text.fill (= kirigami_badge_text(surface.foreground, widget.body.fill), ADD-A53: its last step kirigami_lerp(<the text colour>, <the background>, 0.17), Badge.qml:80)` | field |
| Q2.26 | `badge.body.stroke (= kirigami_lerp(element.color, surface.foreground, 0.5)) @ variant custom_color` (`Badge.qml:49-51`) (where the application gives a colour; `badge.body` declares `color`, ADD-B16) | rule |
| Q2.27 | `badge.label.text (= token.text.keyed.smallest_readable, its weight restated), badge.body.padding, badge.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100)` | field |
| Q2.27 | `badge.body.corners (Full) @ !{multiline} / (the corner radius) @ {multiline}` | rule |
| Q2.27 | `badge.body.padding (the pill's extra horizontal padding) @ !{multiline}` | rule |
| Q2.28 | `avatar.values.default_size (= token.icons.sizes.keyed.large.size)` | field |
| Q2.28 | `avatar.body.fill (= pick(name_index(element.text, <count>, kirigami), …), ADD-B14)` | field |
| Q2.29 | `avatar.body.fill (= kirigami_tint of the avatar colour, ADD-A52), avatar.body.stroke (the avatar colour), avatar.initials.text (fill the avatar colour; text.font.size = an expression on element.width, ADD-B11)` | field |
| Q2.29 | `avatar.fallback_icon.icon, avatar.values.fallback_icon_inset, avatar.image.corners (Full), avatar.body.padding` | field |
| Q2.30 | `status_page.title.text (= token.text.keyed.heading_1_primary); status_page.description.text` | field |
| Q2.30 | `status_page.values.icon_to_title, status_page.values.title_to_description, status_page.values.description_to_actions, status_page.action_area.padding` | field |
| Q2.30 | `status_page.icon.icon (size = floor(add(mul(token.icons.sizes.keyed.huge.size, <factor>), 0.5)), the source's round(); fill = surface.foreground)` | field |
| Q2.31 | `status_page.icon.opacity, status_page.title.opacity, status_page.description.opacity @ variant @informational` | rule |
| Q2.32 | `status_page.body.max_width @ variant loading` | rule |
| Q2.32 | `progress_bar @ context any status_page (indeterminate)` | rule |
| Q2.33 | `bottom_tab_bar.bar.fill (the toolbar's background)` | field |
| Q2.33 | `bottom_tab_bar.values.max_content_width (= mul(mul(token.params.layout.grid_unit, <n>), max(element.item_count, <n>)), centred; `item_count` declared under `values`, the values queried with an `item` element, whose `item_count` is the bar's), bottom_tab_bar.values.equal_width_items` | field |
| Q2.34 | `bottom_tab_bar.item.bindings (Surface(Window)), bottom_tab_bar.item.padding @ cond flavor:kirigami` | rule |
| Q2.34 | `bottom_tab_bar.item.min_height @ variant stacked cond flavor:kirigami` (the background's height) | rule |
| Q2.34 | `bottom_tab_bar.item.min_height (zero: the background takes no height) @ variant inline cond flavor:kirigami` | rule |
| Q2.34 | `bottom_tab_bar.values.icon_text_gap (= token.params.layout.small_spacing) @ cond flavor:kirigami` (the icon to label spacing) | field |
| Q2.34 | `bottom_tab_bar.item_icon.icon.size (= token.icons.sizes.keyed.small_medium.size) @ variant stacked / (= token.icons.sizes.keyed.small.size) @ variant inline` | rule |
| Q2.34 | `bottom_tab_bar.item_label.text.font.size (= mul(base, <factor>)) @ !variant with_icon !variant icon_only cond flavor:kirigami` (a text-only item: the content variants on `item`, ADD-A61) | rule |
| Q2.35 | `bottom_tab_bar.item.fill @ {pressed} / {hover} / {selected}` (Kirigami's checked tab button is `selected`, S7.2) | rule |
| Q2.35 | `bottom_tab_bar.item.stroke @ {selected} / {pressed}` | rule |
| Q2.35 | `bottom_tab_bar.item.corners` | field |
| Q2.35 | `bottom_tab_bar.item.transitions (Fill, Stroke; = token.motion.durations.short)` | field |
| Q2.36 | `bottom_tab_bar.item.focus (Ring: its width, offset outside, paint, corners) @ {focus_visible}` | rule |
| Q2.37 | `sheet.body.fill (= the card's background, Q1.126) @ variant @overlay` | rule |
| Q2.38 | `sheet.scrim.fill, sheet.scrim.transitions (Opacity; = token.motion.durations.long, its timing) @ variant @overlay` | rule |
| Q2.38 | `sheet.title.text (= token.text.keyed.heading_2), sheet.header.padding @ variant @overlay` | rule |
| Q2.38 | `sheet.header_separator.stroke, sheet.footer_separator.stroke, sheet.footer.padding @ variant @overlay` | rule |
| Q2.39 | `sheet.handle.fill, sheet.handle.opacity, sheet.handle.corners (Full), sheet.handle.min_width, sheet.handle.min_height @ cond mobile_form_factor` | rule |
| Q2.40 | `sheet.values.presentations (Appear, Disappear: Fade and Slide together, its distance = mul(token.params.layout.grid_unit, <n>); = token.motion.durations.long, its timing; ADD-B10, ADD-A25)` | field |
| Q2.40 | `sheet.values.min_top_inset, sheet.values.swipe_to_close` | field |
| Q2.41 | `list.section_header.text (= token.text.keyed.heading_5_primary, its weight restated; fill at reduced alpha), list.section_header.icon.size (= token.icons.sizes.keyed.small_medium.size), list.section_header.padding @ cond flavor:kirigami` | rule |
| Q2.41 | `list.section_header_line.stroke (filling the rest of the row)` | field |
| Q2.41 | `list.section_header.fill (Absent: no hover or background) @ cond flavor:kirigami` | rule |
| Q2.42 | `list.inline_header.fill (= kirigami_tint, ADD-A52), list.inline_header.padding, list.inline_header.text (= token.text.keyed.heading_2)` | field |
| Q2.42 | `list.inline_header.stroke` (its bottom edge) `/ list.inline_header.stroke @ variant @inline_footer` (the top edge, placed as the view's footer) | rule |
| Q2.42 | `button @ context list.inline_header (the trailing actions)` | rule |
| Q2.43 | `list.accessory.icon.size (= token.icons.sizes.keyed.small_medium.size) @ variant reorder cond flavor:kirigami` (Kirigami's ListItemDragHandle: the reorder accessory, one structure with iOS's) | rule |
| Q2.43 | `list.accessory.opacity @ variant reorder / {hover} variant reorder / {pressed} variant reorder` | rule |
| Q2.43 | `list.values.presentations (Move, queried with the dropped row: Translate; = token.motion.durations.long, its timing) @ cond flavor:kirigami` | rule |
| Q2.43 | `list.values.autoscroll_step, list.values.autoscroll_interval` | field |
| Q2.44 | `breadcrumb.bottom_separator.stroke, breadcrumb.bottom_separator.bindings (Surface(Header)), breadcrumb.bottom_separator.margin @ cond flavor:kirigami` | rule |
| Q2.44 | `breadcrumb.crumb.fill, breadcrumb.crumb.corners @ {hover} cond flavor:kirigami` | rule |
| Q2.44 | `breadcrumb.separator.glyph (ThemeIcon go-next-symbolic, size = token.icons.sizes.keyed.small.size) @ cond flavor:kirigami` | rule |
| Q2.44 | `breadcrumb.crumb_label.text (a heading) @ cond flavor:kirigami; breadcrumb.crumb_label.opacity @ {current} / !{current}` | rule |
| Q2.44 | `breadcrumb.values.presentations (@scroll: = token.motion.durations.long, its timing)` | field |
| Q2.45 | `form.values.row_gap, form.values.column_gap, form.values.narrow_max_width, form.values.collapse @ cond flavor:kirigami` | rule |
| Q2.46 | `form.values.label_alignment (both members: the label's horizontal alignment and its vertical alignment against the field, its text vertically centred) @ cond flavor:kirigami` | rule |
| Q2.46 | `form.values.label_alignment (both members restated: the label above the field) @ variant collapsed cond flavor:kirigami` | rule |
| Q2.47 | `form.section_title.text (= token.text.keyed.heading_3_primary, centred), form.field_label.text (= token.text.keyed.heading_5) @ cond flavor:kirigami` | rule |
| Q2.47 | `form.values.group_gap, form.values.field_label_gap @ cond flavor:kirigami / variant collapsed cond flavor:kirigami` | rule |
| Q2.48 | `button.icon.glyph (ThemeIcon help-contextual-symbolic), button.values.help_tooltip_on_click, button.body.cursor (WhatsThis, ADD-A59) @ variants {help,icon_only} cond flavor:kirigami` | rule |
| Q2.48 | `tooltip.values.display_time ("none": no timeout), tooltip.values.show_delay @ context button[help] cond flavor:kirigami` | rule |
| Q2.49 | `toast.body.bindings (Surface(Complementary)), toast.body.fill (at reduced alpha), toast.body.corners, toast.body.shadow @ cond flavor:kirigami` | rule |
| Q2.50 | `toast.body.padding, toast.values.content_gap, toast.values.stack_gap, toast.values.placement (its edge_margin: the bottom margin), toast.values.stack_max_height @ cond flavor:kirigami` | rule |
| Q2.50 | `toast.body.max_width (= min(mul(token.params.layout.grid_unit, <n>), mul(element.window_width, <fraction>)), ADD-A78) @ cond flavor:kirigami` | rule |
| Q2.51 | `toast.values.durations (short, default, long)` | field |
| Q2.52 | `toast.values.presentations (Appear: Fade and Translate rise; Disappear: Fade, Translate drop and Scale; @displace: the displaced toasts; each = token.motion.durations.long with its timing; ADD-B10, ADD-A25, ADD-A26)` | field |
| Q2.53† | `edit_menu.body.fill, edit_menu.body.corners, edit_menu.body.shadow, edit_menu.body.padding @ cond input:touch cond flavor:kirigami` | rule |
| Q2.53† | `button.icon.glyph (ThemeIcon), button.body.fill @ variants {tool,icon_only} context edit_menu (Cut, Copy, Paste)` (in a password field Cut and Copy are left out: which commands the field offers is the content the connector supplies, so the menu holds no button for them, S6.3; no absence rule) | rule |
| Q2.53† | `edit_menu.values.placement (KirigamiTextActions: centred on the selection's start, above, else below its end), edit_menu.values.gap_to_selection` | placement |
| Q2.53† | `menu @ context any input / menu @ context any text_area (the desktop context menu)` | rule |
| Q2.54† | `pull_to_refresh.strip.min_height, pull_to_refresh.indicator.padding` | field |
| Q2.54† | `spinner.indicator.glyph (the arc; its draw_extent = element.progress, the pull progress, ADD-B12), spinner.icon (absent = true: no busy icon while pulling) @ !{indeterminate} context pull_to_refresh {pulling}; pull_to_refresh.values.progress_sweep (Arc)` | rule |
| Q2.54† | `pull_to_refresh.values.refresh_delay, pull_to_refresh.values.content_shift` | field |
| Q2.54† | `spinner @ context pull_to_refresh {refreshing} (the BusyIndicator, Q1.119); spinner.indicator @ context pull_to_refresh {refreshing}` (`absent = true`: no arc while refreshing) | rule |
| Q2.54† | `pull_to_refresh.values.presentations (@content_shift: = token.motion.durations.long, its timing)` | field |
| Q2.55† | `form.group_title.text (= token.text.keyed.heading_5, its weight restated), form.group_title.padding @ variant @cards` | rule |
| Q2.55† | `form.card.fill, form.card.stroke, form.card.corners, form.card.shadow, form.card.min_width @ variant @cards (the card, Q1.126-Q1.130) / form.card.padding (past both sides) @ variants {@cards,collapsed}` | rule |
| Q2.55† | `form.row.padding, form.row.min_width, form.row_subtitle.text.fill (= surface.disabled.foreground) @ variant @cards` | rule |
| Q2.55† | `form.field_label.text @ variant @cards / variants {@cards,collapsed} (the inline label above)` | rule |
| Q2.55† | `form.row.fill @ {hover} variants {activatable,@cards} / {pressed} variants {activatable,@cards}` | rule |
| Q2.55† | `form.row.corners @ pos first variant @cards / pos last variant @cards (the outer corners only)` | rule |
| Q2.55† | `form.row.transitions (Fill; = token.motion.durations.long, its timing)` | field |
| Q2.56† | `form.row_separator.opacity, form.row_separator.margin @ variant @cards` | rule |
| Q2.56† | `form.row_separator.opacity @ prev form.row[activatable] {hover} / next form.row[activatable] {hover}; form.row_separator.transitions (Opacity; = token.motion.durations.long)` | rule |
| Q2.56† | `form.row_icon.icon.size (= token.icons.sizes.keyed.small_medium.size), form.row_arrow.icon.size (= token.icons.sizes.keyed.small.size) @ variant action` | rule |
| Q2.56† | `form.page.padding, form.section_title.text (= token.text.keyed.heading_3_primary), form.values.group_gap, form.values.row_gap @ variant flat` | rule |
| Q2.56† | `separator @ context form variant flat (the groups' separator, its margins)` | rule |
| Q2.56† | `button @ context form.row variant action variant flat (actions as push buttons)` | rule |
| Q2.56† | `form.values.collapse, form.values.narrow_max_width` | field |
| Q2.57† | `about_dialog.icon.icon.size (= token.icons.sizes.keyed.huge.size), about_dialog.values.icon_max_width_fraction, about_dialog.values.gap @ cond flavor:kirigami` | rule |
| Q2.57† | `about_dialog.name.text, about_dialog.version.text (= token.text.keyed.heading_1: name and version one Heading at its default level, Q1.5), about_dialog.description.text (= token.text.keyed.heading_3_secondary) @ cond flavor:kirigami` | rule |
| Q2.57† | `form @ variant @cards context any about_dialog (the sections as groups split by separators)` | rule |
| Q2.57† | `avatar.body.min_width, avatar.body.min_height (= token.icons.sizes.keyed.medium.size) @ context any about_dialog` | rule |
| Q2.58† | `dialog.values.window_margin, dialog.header.padding, dialog.title.text (a heading, elision End) @ cond mobile_form_factor` | rule |
| Q2.58† | `dialog.header.min_height @ cond mobile_form_factor` | rule |
| Q2.58† | `button.icon.icon.size (= token.icons.sizes.keyed.small_medium.size), button.icon.glyph (ThemeIcon) @ variant close context dialog.header cond mobile_form_factor / variant close {hover} context dialog.header cond mobile_form_factor` | rule |
| Q2.58† | `dialog @ variant full_size cond mobile_form_factor (a narrower window: the full-window layer, Q2.15) / variant separate_window (the desktop); dialog.values.natural_size @ variant separate_window` | rule |
| Q2.58† | `dialog.values.breakpoints @ cond mobile_form_factor` (measured Window, width AtMost the row's grid-unit width, setting `full_size`: the complement of `PageRow`'s "wider than" for the in-window dialog; R16) | rule |
| K3.1 | `window.frame.shadow (= Shadow::Elevation of the ShadowSize step the reader reads; its box radius follows window.frame.corners)` | field |
| K3.1 | `token.elevation.<key> (Breeze's window shadow sizes: two layers and a composite offset each)` | token |
| K3.1 | `token.params.shadow.strength (ADD-A44)` | token |
| K3.1 | `reader:kde:breezerc [Common] ShadowSize, ShadowStrength, ShadowColor` | reader |
| K3.2 | `menu.body.shadow, tooltip.body.shadow, popover.body.shadow, toolbar.bar.shadow @ variant floating, menu.body.shadow @ context combo_box` | field |
| K3.2 | `token.elevation.<key> (Breeze's popup shadow sizes)` | token |
| K3.2 | `reader:kde:breezerc [Common] ShadowSize, ShadowStrength, ShadowColor` | reader |
| K3.3 | `component.mdi_sub_window.frame.shadow (the K3.2 table round sub-windows)` | component |
| K3.4 | `button.body.shadow` | field |
| K3.4 | `combo_box.body.shadow` | field |
| K3.4 | `slider.thumb.shadow` | field |
| K3.4 | `dial.handle.shadow` | field |
| K3.11 | `menu.body.fill (Material kwin_blur: blur_radius, saturation, noise_opacity; under the translucent parts, K1.65)` | field |
| K3.11 | `[declare] materials kwin_blur (S10)` | field |
| K3.11 | `reader:kde:kwinrc [Effect-blur] BlurStrength, NoiseStrength, Saturation` | reader |
| K3.12 | - | out (R1.1) |
| K3.13 | - | out (R1.1) |
| K3.14 | - | out (R1.1) |
| K3.15† | - | out (R1.1) |
| K3.16† | - | out (R1.1) |
| K3.30 | `token.motion.speed_factor` | token |
| K3.30 | `reader:kde:kdeglobals [KDE] AnimationDurationFactor` | reader |
| K3.30 | `cond animations_off (factor 0)` | condition |
| K3.31 | `token.motion.durations.animations (Breeze's Animations class, S4.1: while the factor is set, = floor(mul(<base>, token.motion.speed_factor)), truncated to whole ms, `breeze@6.8.2 kstyle/breezestyle.cpp:2080`, zero, which a factor below 0.01 gives, turning Breeze's animations off, `:2085`, a positive result turning them on, `:2081-2083`; breezerc AnimationsDuration otherwise)` | token |
| K3.31 | `token.motion.timings.animations (Steps: its count read from AnimationSteps)` | token |
| K3.31 | `token.motion.durations.animations` (the reader states it zero while kdeglobals sets no `AnimationDurationFactor` and breezerc [Style] AnimationsEnabled is off; a factor set overrides that switch, its product with 100 truncated to whole ms turning Breeze's animations on where positive and off where zero, as a factor below 0.01 gives, `breezestyle.cpp:2076-2085`: Breeze's own switch stops only the Breeze engines the widgets flavour's parts use, not the Kirigami animations that `cond animations_off`, the global factor's condition, K3.30, stops) | token |
| K3.31 | `PartStyle.transitions per part (each Breeze engine's parts in the rows above; the enable/disable cross-fade a transition on {disabled})` | field |
| K3.31 | `navigation_view.values.presentations (Fade) @ cond system_setting:stacked_widget_transitions (the stacked-widget fade, off by default)` | rule |
| K3.31 | `reader:kde:breezerc [Style] AnimationsEnabled, AnimationSteps, AnimationsDuration, StackedWidgetTransitionsEnabled` | reader |
| K3.33 | - | out (R1.1) |
| K3.34 | - | out (R1.1) |
| K3.35 | - (KWin's squash and maximize effects, which KWin runs on every window; its sliding popups, which only a window that asks for them through `KWindowEffects::slideWindow` gets: Plasma's own popups ask, `libplasma src/plasmaquick/popupplasmawindow.cpp:115`, `:132`, `src/plasmaquick/dialog.cpp:318`, and no Breeze, Kirigami, qqc2-desktop-style or plasma-integration source does, by search) | out (R1.1) |
| K3.37 | `banner.values.presentations (Appear, Disappear: the message widget's height and opacity; = token.motion.durations.message_widget, = token.motion.timings.in_out_sine)` | field |
| K3.37 | `expander.values.presentations (@expand, @collapse: the collapsible group; = max(<the source's floor>, token.motion.durations.animations), the style hint's duration, kcollapsiblegroupbox.cpp:114-116; = token.motion.timings.in_out_sine)` | field |
| K3.37 | `token.motion.durations.message_widget` | token |
| K3.37 | `token.motion.timings.in_out_sine` | token |
| K3.38† | `component.dock_widget.values.presentations (Move: the dock widgets' re-layout slide; = token.motion.durations.animations, = token.motion.timings.in_out_quad, `qtbase@6.12.0 src/widgets/widgets/qwidgetanimator.cpp:62`)` | component |
| K3.38† | `toolbar.values.presentations (Move: the toolbars' re-layout slide; = token.motion.durations.animations, = token.motion.timings.in_out_quad, `qtbase@6.12.0 src/widgets/widgets/qwidgetanimator.cpp:62`)` | field |
| K3.39† | - | out (R1.1) |
| K3.20 | `token.focus (FilledRing: width, paint, offset; offset 0, the ring's inner edge on the widget's box in Breeze's default branch, `breeze@6.8.2 kstyle/breezestyle.cpp:6626`, `:6628`, `:6771-6773`, which rings none of Breeze's own focus targets, each having a branch of its own, `:1658-1663`, so every part below states its offset; corners Radius::Px at Frame_FrameRadius, the ring's inner corner whatever the part's radius, its outer corner that plus the margin, `breeze@6.8.2 kstyle/breezestyle.cpp:6631-6632`, `kstyle/breezemetrics.h:42`, S2.10)` | token |
| K3.20 | `button.body.focus, combo_box.body.focus @ !variant editable (= token.focus, its offset from the part's box: Breeze draws these frames with `renderButtonFrame`, its box the widget rect less the shadow margin, `PenWidth::Shadow`, `breeze@6.8.2 kstyle/breezehelper.cpp:670`, `kstyle/breezehelper.h:360-363`, `kstyle/breezemetrics.h:28`, `kstyle/breezestyle.cpp:4566`, `:7865`, and the ring's inner edge is the widget rect less Breeze's inset, `innerRect.adjust(1, 1, -1, -1)`, `:6648`, so the offset is the shadow margin less that inset, positive: the ring's inner edge just outside the body's box; ADD-A123)` | field |
| K3.20 | `input.body.focus, text_area.body.focus, stepper.body.focus, combo_box.body.focus @ variant editable (= token.focus, its offset from the part's box negative by Breeze's inset, `innerRect.adjust(1, 1, -1, -1)`, `breeze@6.8.2 kstyle/breezestyle.cpp:6648`: these frames are `renderFrame` on the whole widget rect, the line edit's, the text area's (a `QFrame`), the spin box's and the editable combo box's, `:4080`, `:3970`, `:7956`, `:7845`, `kstyle/breezehelper.cpp:479`, so the ring's inner edge lies inside the body's box by the inset; ADD-A123)` | field |
| K3.20 | `checkbox.box.focus, radio.ring.focus (= token.focus, the ring flush with the drawn indicator: offset 0 to the indicator rect inset by 2, where Breeze draws the box and the circle, `breezestyle.cpp:6669`, `:6689`, `kstyle/breezehelper.cpp:843-844`, `:895`, `:995-996`; the radio's corners `Radius::Full { maximum: Absent }`, `breezestyle.cpp:6691`; a checkable group box's title checkbox is a `checkbox` and is ringed so, `:6764-6770`)` | field |
| K3.20 | `slider.thumb.focus (= token.focus, flush with the drawn handle, the path renderSliderHandle draws, its inner edge inside the handle rect, a `Slider_ControlThickness` square, `breezestyle.cpp:3270`, by the inset Breeze adjusts it by plus half the frame pen, `strokedRect`, `kstyle/breezehelper.cpp:1160-1161`, `:1747-1757`, an ellipse: corners `Radius::Full { maximum: Absent }`, `breezestyle.cpp:6700`, `kstyle/breezehelper.cpp:1155-1167`), dial.handle.focus (= token.focus, its inner edge inside the handle rect, a `Slider_ControlThickness` square, `:3241`, by the inset Breeze adjusts it by, corners `Radius::Full { maximum: Absent }`, `breezestyle.cpp:6729`, `:6731`)` | field |
| K3.20 | `<widget>.values.focus_visible_when (FocusVisibleWhen::Keyboard { … }, ADD-A16)` | field |
| K3.21 | `tab.label.focus (Underline), group_box.focus_line.stroke, menu.item_label.stroke (the weak-focus line under the text, K1.62), menu_bar.item_label.stroke (K2.1)` | field |
| K3.22 | `FocusIndicator.kind Underline (= qt_lighter of the highlight): the focus of parts with no frame; list.row_background.focus (the item views' rounded rect)` | field |
| K3.23 | `input.body.stroke @ {focused}, combo_box.body.stroke @ {focused}` (the frames' focus colour, as K1.40, K1.243 state it; buttons by `{focus_visible}`, K1.20) | rule |
| K3.23 | `slider.thumb.stroke @ {focused}` (K1.100); `scrollbar.thumb.fill @ context any scroll_view {focus_within}` (K1.92): the thumbs' focus colour, each on its own part | rule |
| K3.23 | `role.window.decoration_focus` | role |
| K3.24 | `FocusIndicator.transition` | field |
| K3.24 | `PartStyle.transitions group Focus` | field |
| K3.40 | `role.selection.<role> @ cond window_inactive (= kde_effects(kde_selection_inactive(<Selection's inactive input of that role>, <Window's inactive input of that role>, active(role.selection.background), <role kind>), kde_selection_inactive(<Selection's inactive background input>, <Window's inactive background input>, active(role.selection.background), background), inactive, <role kind>): KColorScheme's start for the Selection set's Inactive group, Window's colours (the background tinted toward Selection's) while token.kde.inactive_change_selection_color holds, else Selection's own (kcolorscheme.cpp:304-316, :374-378), with the inactive effects applied over it (:380-391; S2.2, R5.1))` | rule |
| K3.40 | `token.kde.inactive, token.kde.inactive_change_selection_color, token.kde.inactive_enabled` | token |
| K3.40 | `reader:kde:kdeglobals [ColorEffects:Inactive] ChangeSelectionColor, Enable` | reader |
| K3.41 | `token.caret.blink_on, token.caret.blink_off (no blink: blink_off zero, the caret never off, as X-CA-2; the reader clamps the period)` | token |
| K3.41 | `reader:kde:kdeglobals [KDE] CursorBlinkRate` | reader |
| K3.42 | `token.caret.width` | token |
| K3.43 | `input.caret.fill, text_area.caret.fill @ {text_selected}` (no absence rule: the caret stays drawn and blinking while text is selected) | rule |
| K3.44 | `token.behavior.mnemonics (ADD-A12)` | token |
| K3.44 | `reader:kde:breezerc [Style] MnemonicsMode` | reader |
| K3.45 | `token.text.rendering (ADD-B04)` | token |
| K3.45 | `reader:kde:kdeglobals [General] XftAntialias, XftHintStyle, XftSubPixel` | reader |
| K3.46 | `Font.width (stretch), Font.axes, Font.features, TextStyle.letter_spacing, TextStyle.word_spacing (S2.8)` | field |
| K3.46 | `Font.weight, Font.width, Font.style (the style name mapped to them)` | field |
| K3.46 | `TextStyle.case (ADD-A39)` | field |
| K3.46 | `reader:kde:kdeglobals font strings (capitalization, letter and word spacing, stretch, style name, features, axes)` | reader |
| K3.46 | `reader:kde:kcmfonts [General] forceFontDPI, Xft.dpi -> ReaderState.font_dpi` (the DPI the font strings' point sizes convert by, S9.4: platform-facts' *Font DPI override*, Plasma 6.7 on X11; none on Wayland or from Plasma 6.8, where the preset's `token.text.font_dpi` applies) | reader |
| K3.46 | `token.text.font_dpi` (Plasma's DPI where the reader reports none, from the facts, S9.4, ADD-A103) | token |
| K3.47 | - (`taskbarFont`, the Plasma panel's font: the shell draws the panel, no application does) | out (R1.1) |
| K3.48† | `input.preedit.text, text_area.preedit.text (TextStyle.decoration and fill from the input method's attributes)` | field |
| K3.48† | `input.caret @ {composing}; text_area.caret @ {composing}` | rule |
| K3.50 | `token.kde.disabled` | token |
| K3.50 | `role.<set>.disabled.* (= kde_effects(<the active role>, active(role.<set>.background), disabled, <role kind>): `foreground` for the text roles, `decoration` for the decoration roles, `background` for the backgrounds), except the Selection set, whose disabled group starts from Window's active values (kcolorscheme.cpp:317-319): role.selection.disabled.* = kde_effects(active(role.window.<role>), active(role.window.background), disabled, <role kind>) (S2.2, R5.1)` | role |
| K3.50 | `reader:kde:kdeglobals [ColorEffects:Disabled] IntensityEffect, IntensityAmount, ColorEffect, ColorAmount, ContrastEffect, ContrastAmount` | reader |
| K3.51 | `role.* @ cond window_inactive (kde_* over the inactive input roles)` | rule |
| K3.51 | `input.inactive (KDE stated Inactive values)` | input |
| K3.51 | `input.inactive.<set>.<key> in the preset's bases (Breeze's values as Plasma's applyScheme() writes them on its default path, "Accent color from color scheme", with no accent derivation; the reader states kdeglobals' [Colors:*][Inactive] results, S9.1, R25.1, ADD-A109)` | input |
| K3.51 | `input.inactive.<set>.<key> in the accent layer (= kde_accent(<set>, <key>, inactive) over the scheme inputs in force, while no mapping palette applies, S2.2, S9.2)` | input |
| K3.51 | `token.kde.inactive, token.kde.inactive_enabled` | token |
| K3.51 | `reader:kde:kdeglobals [ColorEffects:Inactive], [Colors:*][Inactive]` | reader |
| K3.52 | `role.<set>.background_active / background_link / background_visited / background_negative / background_neutral / background_positive (ADD-A48)` | role |
| K3.53 | `role.@shade_light, role.@shade_midlight, role.@shade_mid, role.@shade_dark, role.@shade_shadow (ADD-A48; each = kde_shade(role.window.background, <role>, token.params.color.contrast), ADD-A121)` | role |
| K3.53 | `token.params.color.contrast (ADD-A44; kdeglobals' [KDE] contrast as stored, kde_shade scaling it as KColorScheme's contrastF does)` | token |
| K3.53 | `reader:kde:kdeglobals [KDE] contrast` | reader |
| K3.54 | `role.complementary.*` | role |
| K3.54 | `role.<set>.foreground_inactive / foreground_active / link / link_visited / negative / neutral / positive / background_alternate / decoration_focus / decoration_hover (tooltip, header, button, selection)` | role |
| K3.54 | `reader:kde:kdeglobals [Colors:*] all seven sets` | reader |
| K3.54 | `role.<set>.<role> for each accent-dependent role, in the preset's bases (Breeze's scheme values as Plasma's default path, "Accent color from color scheme", writes them, with no accent derivation; the reader overrides them with kdeglobals' results, S9.1, R25.1, ADD-A109)` | role |
| K3.54 | `role.<set>.<role> for each accent-dependent role, in the accent layer (= kde_accent(<set>, <key>, active) over the scheme inputs in force, while no mapping palette applies: the roles plasma-workspace's applyScheme() derives from the accent, S9.1's list from colorsapplicator.cpp, S2.2, S9.2)` | role |
| K3.54 | `input.kde_scheme.<set>.<key>` (the scheme file's values before Plasma applies the accent: Breeze's in the preset, the file `[General] ColorScheme` names in the reader, S9.1) | input |
| K3.54 | `input.kde_scheme.<set>.inactive.<key>` (the scheme file's `[Colors:<Set>][Inactive]` values, for every set: a set without an `[Inactive]` group states its active values, as KColorScheme reads it; the accent layer's `kde_accent(<set>, <key>, inactive)` reads them, S2.2, S9.1, S9.2) | input |
| K3.54 | `input.kde_scheme.inactive_groups` (per scheme, the sets whose scheme file has an `[Inactive]` group: Breeze's `Header` in the preset, K3.55; the user's file in the reader; a port's own; `kde_accent` branches on it, S2.2, S9.1) | input |
| K3.60 | `IconStyle.state_effect @ {disabled} / {hover} (keys declared in icon_state_effects, each implemented to KIconEffect's source; S2.9)` | field |
| K3.60 | `PartStyle.icon.state_effect, PartStyle.opacity @ {disabled} (to gray and semi-transparent)` | rule |
| K3.60 | `token.params.icon_effect.<key> (the effects' amounts: the gamma of the active effect, the gray and transparency of the disabled one)` | token |
| K3.61 | `IconStyle.fill (= surface.foreground), IconStyle.status_colors (= the status roles) per icon mode (ADD-A37)` | field |
| K3.62 | `PartStyle.icon.fill @ {selected} / {pressed} (selection foreground)` | rule |
| K3.63 | `token.icons.sizes.toolbar, token.icons.sizes.keyed.main_toolbar, token.icons.sizes.small, token.icons.sizes.large, token.icons.sizes.dialog, token.icons.sizes.panel` | token |
| K3.63 | `reader:kde:icon theme index.theme *Default sizes` | reader |
| K3.64 | `token.pointer.theme, token.pointer.size (ADD-A46)` | token |
| K3.64 | `reader:kde:kcminputrc [Mouse] cursorTheme, cursorSize` | reader |
| K3.65† | `PartStyle.cursor per part: link.body.cursor, splitter.divider.cursor @ variant @column_view, list.accessory.cursor @ variant reorder, button.body.cursor @ variant help, sidebar.handle (the drawer edges)` | field |
| K3.65† | `selectable_label.body.cursor` (Text: Kirigami's `SelectableLabel`, `kirigami@eecf183a src/controls/SelectableLabel.qml:266`; qqc2-desktop-style's `Label`, `org.kde.desktop/Label.qml:23`) | field |
| K3.65† | `token.behavior.{hide_pointer_while_typing,hide_pointer_delay}` (KLineEdit hides the pointer after its delay, ADD-A86) | token |
| K3.66† | - | out (R1.1) |
| K3.70 | `token.behavior.double_click_time` | token |
| K3.70 | `reader:kde:kdeglobals [KDE] DoubleClickInterval` | reader |
| K3.71 | `token.behavior.drag_threshold` | token |
| K3.71 | `token.behavior.drag_time (ADD-A45)` | token |
| K3.71 | `reader:kde:kdeglobals [KDE] StartDragDist, StartDragTime` | reader |
| K3.72 | `token.behavior.scroll_lines_per_step` | token |
| K3.72 | `reader:kde:kdeglobals [KDE] WheelScrollLines` | reader |
| K3.73 | `token.behavior.single_click_activates @ cond system_setting:single_click (a `Tokens` rule; one route, S4.2; ADD-A45)` | token |
| K3.73 | `reader:kde:kdeglobals [KDE] SingleClick -> cond system_setting:single_click` | reader |
| K3.73 | `cond system_setting:single_click (ADD-A04)` | condition |
| K3.73 | `list.row @ {hover} cond system_setting:single_click context any file_chooser` (the hover feedback of KIO's file views in single-click mode; the fields it sets are read from KIO and Dolphin in the value phase) | rule |
| K3.76† | `stepper.values.repeat_delay, stepper.values.repeat_interval (the mouse's interval)` | field |
| K3.76† | `stepper.values.key_repeat_interval (the interval while an arrow key is held)` | field |
| K3.76† | `token.behavior.long_press` | token |
| K3.76† | `tab.values.drag_hover_delay (K1.133†)` | field |
| K3.83 | `cond tablet_mode` | condition |
| K3.83 | `reader:kde:org.kde.KWin.TabletModeManager D-Bus property` | reader |
| K3.83 | `PartStyle sizes @ cond tablet_mode per widget` | rule |
| K3.84† | - | out (R1.1) |
| K3.90 | `token.motion.speed_factor` | token |
| K3.90 | `reader:kde:kdeglobals [KDE] AnimationDurationFactor` | reader |
| K3.90 | `cond reduced_motion` | condition |
| K3.91 | `token.params.color.contrast (ADD-A44; kdeglobals' [KDE] contrast as stored, no reader conversion: kde_shade scales it by 0.1 as KColorScheme::contrastF does, kcolorscheme.cpp:519-527, ADD-A121)` | token |
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
| Q3.1 | `PartStyle.bindings` (a widget's colours come from the set of the region it sits in: `Binding::Surface` per region, S3.3's nearest binding in scope; `colorSet` + `inherit`) | field |
| Q3.2 | `PartStyle.bindings Surface(set) per widget (S2.11): Button for button, checkbox, switch, slider; View for input, text_area, search_field, stepper, the editable combo box, scroll_view, card, dialog, list; Header for the top and page-header toolbars, menu_bar and the separators of toolbars, drawers and columns; Window for tab, the bottom toolbar and the flat tool buttons; Tooltip or Complementary for tooltip (Q1.37); Complementary for toast; Selection for the focused round button (the rules of Q1.8, Q1.9, Q1.53, Q1.68, Q1.123, Q2.8, Q2.49)` | field |
| Q3.3 | `role.* @ cond window_inactive` | rule |
| Q3.3 | `cond window_inactive` | condition |
| Q3.4 | `role.<set>.disabled.*` | role |
| Q3.4 | `PartStyle fill, text, icon @ {disabled} (refer to disabled roles)` | rule |
| Q3.5 | `role.global.status.info = role.<set>.foreground_active` | role |
| Q3.5 | `role.global.accent_standalone = role.window.foreground_active` (the plain accent `applyScheme()` writes into ForegroundActive while the Selection background takes `accentBackground`, `colorsapplicator.cpp:178-181`, `:197-217`, `:229-236`; ADD-A122) | role |
| Q3.10 | `role.<set>.negative / neutral / positive foregrounds` | role |
| Q3.10 | `role.global.status.{success,warning,error}.fill, .standalone (= role.window.positive / neutral / negative: Kirigami's InlineMessage and Badge draw the status in Theme.*TextColor, `kirigami@6.31.0 src/controls/InlineMessage.qml:26-31`, `src/controls/Badge.qml:53-62`), .foreground (= role.window.foreground: KMessageWidget keeps the parent's WindowText, `kwidgetsaddons@6.31.0 src/kmessagewidget.cpp:185-186`), .background (= role.window.background_positive / neutral / negative, K3.52), .background_solid (absent)` (ADD-A122) | role |
| Q3.11 | `role.<set>.decoration_hover` | role |
| Q3.11 | `chip.body.stroke @ {hover}` (the chip's hover border, = role.<set>.decoration_hover); `switch.thumb.stroke @ {hover} / {focus_visible}` (= role.button.decoration_hover: the handle's border hovered or with visual focus, its own Button set; at rest it is Q1.101's `kirigami_lerp`; `qqc2-desktop-style@6.31.0 org.kde.desktop/private/SwitchIndicator.qml:21-22`, `:30`; Kirigami's `hoverColor`, `kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:311`; ADD-A122) | rule |
| Q3.12 | `role.<set>.decoration_focus` | role |
| Q3.12 | `PartStyle.fill @ {focus_visible}` | rule |
| Q3.13 | `role.<set>.background_alternate` | role |
| Q3.13 | `list.row_background.fill @ pos parity even variant alternating_rows (ADD-B06; Qt's views alternate on current & 1, zero-based)` | rule |
| Q3.14 | `token.params.color.frame_contrast (ADD-A44)` | token |
| Q3.14 | `reader:kde:kdeglobals [KDE] frameContrast (store the factor, not only finished colours)` | reader |
| Q3.15 | `role.<set>.border = kcu_mix(background, foreground, token.params.color.frame_contrast)` (Breeze's frame colour, `breezehelper.cpp:144`; Breeze draws Kirigami's buttons and fields with it too, `:250`, `:684-706`) | role |
| Q3.15 | `group_box.body.stroke @ variant @quick_frame cond flavor:kirigami / variant @quick_group_box cond flavor:kirigami; menu.body.stroke, popover.body.stroke, tooltip.body.stroke @ cond flavor:kirigami = kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast)` (the frames qqc2-desktop-style's own QML draws, `Frame.qml:25`, `GroupBox.qml:39`, `Menu.qml:133`, `Popup.qml:50`, `ToolTip.qml:115`; the same expression on its own rows: qqc2's `Dialog.qml:85` and Kirigami's `Dialog.qml:341` (Q1.112), the search field's popup `SearchField.qml:183` (Q1.16), the range slider's handles `RangeSlider.qml:32`, `:52` (Q1.46), Kirigami's card `DefaultCardBackground.qml:70` (Q1.129), chip `DefaultChipBackground.qml:30` (Q2.23) and separator `Separator.qml:65-69` (Q1.95), kirigami-addons' dialog surface `DialogRoundedBackground.qml:25` (K-KA-18†), segmented button `SegmentedButton.qml:112` (K-KA-24†), floating buttons `FloatingButton.qml:137` (K-KA-26†), banner line `Banner.qml:239` (K-KA-29†), video controls `VideoMaximizeDelegate.qml:349`, `:365` (K-KA-35†) and onboarding tip `OnboardToolTip.qml:52`, `OnboardingOverlay.qml:164`, `:187` (K-KA-45†); on the Button set, not the surface in scope: the switch's thumb `private/SwitchIndicator.qml:30` (Q1.101), RoundButton `RoundButton.qml:77` (Q2.5) and the double floating button's halves `DoubleFloatingButton.qml:159`, `:256` (K-KA-27†); ADD-A121) | rule |
| Q3.16 | `PartStyle.fill.append = alpha(...) @ {hover} / {pressed} / {checked}` | rule |
| Q3.17 | `PartStyle.icon.fill (text or highlighted-text role)` | field |
| Q3.17 | `PartStyle.icon.fill @ {selected}` | rule |
| Q3.18 | `IconStyle.state_effect @ {hover} cond flavor:kirigami / {disabled} cond flavor:kirigami (Kirigami's icon shader: highlight amount, desaturation; keys declared in icon_state_effects; S2.9)` | field |
| Q3.18 | `PartStyle.opacity @ {disabled} cond flavor:kirigami (the disabled icon)` | rule |
| Q3.18 | `token.params.icon_effect.<key> (the shader's highlight amount and desaturation)` | token |
| Q3.19 | `PartStyle.transitions group Icon (cross-fade)` | field |
| Q3.20 | `ShadowLayer x, y, blur, color (spread zero; one layer per surface; S2.5), with the part's corners per corner and its stroke` | field |
| Q3.21 | `token.elevation.<key> (Kirigami's and qqc2's surfaces)` | token |
| Q3.21 | `PartStyle.shadow Elevation(key) per surface part: menu.body.shadow, popover.body.shadow, tooltip.body.shadow, dialog.body.shadow, menu.body.shadow @ context autocomplete, card.body.shadow (with its lift layer, Q1.128), toast.body.shadow, button.body.shadow @ variant circular, slider.thumb.shadow @ variant range, sidebar.edge_shadow.fill, list.swipe_area.shadow` | field |
| Q3.22 | `role.global.scrim` | role |
| Q3.22 | `dialog.scrim.fill @ cond flavor:kirigami; sidebar.scrim.fill @ variant overlay; sheet.scrim.fill @ variant @overlay` | rule |
| Q3.22 | `dialog.scrim.transitions, sidebar.scrim.transitions, sheet.scrim.transitions (Opacity; = token.motion.durations.long, its timing)` | field |
| Q3.23 | `token.motion.durations.very_short, token.motion.durations.short, token.motion.durations.long, token.motion.durations.very_long` | token |
| Q3.24 | `token.motion.durations.long (= max(round(mul(<the read longDuration>, token.motion.speed_factor)), <floor>): `qRound`, then `qMax`, `qqc2-desktop-style@6.31.0 kirigami-plasmadesktop-integration/plasmadesktopunits.cpp:45`, `:49`; ADD-A56, ADD-A122)` | token |
| Q3.24 | `token.motion.durations.very_short, token.motion.durations.short (= floor(div(token.motion.durations.long, <divisor>)): C++'s integer division, `plasmadesktopunits.cpp:51-52`; ADD-A122), token.motion.durations.very_long (= mul(token.motion.durations.long, <factor>), `:54`)` | token |
| Q3.24 | `reader:kde:plasmarc [Units] longDuration` | reader |
| Q3.24 | `reader:kde:kdeglobals [KDE] AnimationDurationFactor` | reader |
| Q3.25 | `token.motion.speed_factor @ cond animations_off` | rule |
| Q3.25 | `token.motion.durations.* (the floor through max, ADD-A56) @ cond animations_off; the transitions whose duration resolves to zero run instantly; spinner.values.loop @ cond animations_off (Q1.120)` | token |
| Q3.26 | `token.motion.timings.<key> (one declared timing per easing curve in use)` | token |
| Q3.26 | `Timing::Platform(PlatformCurve::KirigamiVelocity)` (SmoothedAnimation by velocity, CONTRA-2) | token |
| Q3.27 | `token.motion.durations.human_moment (unscaled)` | token |
| Q3.28 | `tooltip.values.show_delay` | field |
| Q3.28 | `tooltip.values.show_delay (= token.behavior.long_press) @ cond tablet_mode` | rule |
| Q3.29 | `token.params.layout.grid_unit (ADD-A44)` | token |
| Q3.29 | `Length expressions mul(token.params.layout.grid_unit, <n>)` | field |
| Q3.30 | `checkbox.label.focus, radio.label.focus (Q1.21); button.body.focus, input.body.focus, tab.label.focus (keyboard only: focus_visible_when); bottom_tab_bar.item.focus (Q2.36)` | field |
| Q3.30 | `link.text.text.font.weight @ {focus_visible} (Q1.135); list.row_background.fill @ {focus_visible} (Q1.78); switch.thumb.stroke @ {focus_visible} (Q1.101)` | rule |
| Q3.31 | `cond tablet_mode` | condition |
| Q3.31 | `cond input:touch` | condition |
| Q3.31 | `menu.item.min_height, menu.item_icon.icon.size @ cond input:touch` | rule |
| Q3.31 | `scrollbar.thumb.fill, scrollbar.thumb.min_width, scrollbar.thumb.corners, scrollbar.values.auto_hide_delay, scrollbar.values.presentations @ cond tablet_mode cond flavor:kirigami / cond input:touch cond flavor:kirigami (the overlay scroll bar, Q1.41)` | rule |
| Q3.31 | `tooltip.values.show_on_press @ cond tablet_mode` | rule |
| Q3.31 | `input.body.fill, input.body.stroke @ {hover} cond tablet_mode (as at rest: no hover look)` | rule |
| Q3.31 | `input.selection_handle.fill, input.selection_handle.min_width, input.selection_handle.min_height @ cond tablet_mode` | rule |
| Q3.31 | `sidebar.handle @ cond tablet_mode cond flavor:kirigami` | rule |
| Q3.31 | `sheet.handle @ cond mobile_form_factor` | rule |
| Q3.31 | `cond mobile_form_factor` | condition |
| Q3.31 | `reader:kde:Kirigami Settings.isMobile (what it reads, to verify) -> cond mobile_form_factor` | reader |
| Q3.32 | `token.text.rendering.engine @ cond unit_scale (native rendering, ADD-A76) / !cond unit_scale (Qt's rendering) (ADD-B04)` | rule |
| Q3.33 | `checkbox.label.opacity, radio.label.opacity, switch.label.opacity @ {disabled} cond flavor:kirigami` | rule |
| Q3.33 | `combo_box.label.text.fill (at a reduced alpha) @ {disabled} variant editable cond flavor:kirigami / input.text.text.fill (at a reduced alpha) @ {disabled} context any search_field cond flavor:kirigami` | rule |
| Q3.33 | `list.row.opacity @ {disabled} context sidebar.pane[@context]` | rule |
| K-SF01† | `reader:kde:the KSyntaxHighlighting theme chosen by matching [Colors:View] BackgroundNormal and [Colors:Selection] BackgroundNormal` | reader |
| K-SF01† | `code_editor parts' bases from the Breeze Light editor theme, and rules @ cond scheme:dark from Breeze Dark` | field |
| K-SF02† | `code_editor.body.fill, code_editor.selection.fill, code_editor.selection.text` | field |
| K-SF03† | `code_editor.current_line.fill; code_editor.current_line.fill (= mix(base, the mark's colour, <fraction>)) @ {@line_has_mark_bookmark} / {@line_has_mark_breakpoint_active} / {@line_has_mark_breakpoint_reached} / {@line_has_mark_breakpoint_disabled} / {@line_has_mark_execution} / {@line_has_mark_warning} / {@line_has_mark_error}` | rule |
| K-SF04† | `code_editor.gutter.fill, code_editor.line_number.text, code_editor.current_line_number.text, code_editor.gutter_separator.stroke, code_editor.fold_marker.glyph, code_editor.fold_marker.fill` | field |
| K-SF04† | `code_editor.mark.fill @ variant @bookmark / variant @breakpoint_active / variant @breakpoint_reached / variant @breakpoint_disabled / variant @execution / variant warning / variant error` | rule |
| K-SF05† | `code_editor.modified_line_marker.fill, code_editor.saved_line_marker.fill` | field |
| K-SF05† | `code_editor.scroll_mark.fill (the minimap strip's markers: = kte_minimap_mark(the marker's colour, the background), ADD-A57)` | field |
| K-SF06† | `code_editor.invisible.glyph (stroke.width = max of a floor and the space's width by a factor) @ variant tab / variant space / variant no_break_space` | rule |
| K-SF06† | `code_editor.invisible.glyph.size` (the marks' size) | field |
| K-SF06† | `code_editor.indentation_guide.stroke` | field |
| K-SF07† | `code_editor.bracket_match.fill, code_editor.bracket_match.text` | field |
| K-SF07† | `code_editor.bracket_match.transitions (the optional animation, off by default)` | field |
| K-SF08† | `code_editor.find_match.fill, code_editor.replace_match.fill` | field |
| K-SF09† | `code_editor.right_margin.stroke` | field |
| K-SF09† | `code_editor.dynamic_wrap_indicator.fill (Pattern QtBrush Dense4 in the marker's colour, ADD-B01)` | field |
| K-SF10† | `code_editor.syntax_span.text (fill, weight, style, decoration), code_editor.syntax_span.fill @ variant comment / variant string / variant function / variant constant / variant preprocessor / variant special_char / variant error / variant warning / variant type / variant decimal / variant base_n / variant float / variant @<role> (the keyed roles and per-language overrides)` | rule |
| K-SF10† | `code_editor.syntax_span.text.fill @ {text_selected}` (a span whose text lies in the selection: the style's selected-text colour) | rule |
| K-SF11† | `code_editor.misspelling.text (its decoration's paint; its `thickness` and `offset` = `element.font_underline_thickness`, `element.font_underline_offset`, S2.8)` | field |
| K-SF11† | `code_editor.template_field.fill` | field |
| K-SF11† | `code_editor.template_placeholder.fill / @ {focused} / @ {read_only}` | rule |
| K-SF12† | `rich_text.heading_1.text … rich_text.heading_6.text (the weight restated; text.font.size = mul(base, <factor>) each, base the document's font) @ variant markdown` | rule |
| K-SF13† | `rich_text.inline_code.text (= token.text.roles.monospace, as the catalogue states it), rich_text.inline_code.fill (Absent) @ variant markdown` | rule |
| K-SF13† | `rich_text.code_block.text (= token.text.roles.monospace), rich_text.code_block.fill (Absent), rich_text.code_block.margin (none) @ variant markdown` | rule |
| K-SF13† | `rich_text.code_block.text.wrap @ variants {markdown,fenced}` (`fenced` the `code_block` element's content variant, `markdown` its widget's, which the element's query sees: a fenced block's lines do not wrap) | rule |
| K-SF14† | `rich_text.block_quote.fill (Absent), rich_text.block_quote.stroke (Absent)` | field |
| K-SF14† | `rich_text.block_quote.margin (the right margin), rich_text.values.quote_indent_per_level` | field |
| K-SF15† | `rich_text.list_marker.glyph (disc, circle, square; size = an expression on the paragraph's line height, `widget.paragraph.line_height`, the marker having no font of its own, S2.2; fill the run's or the text role), rich_text.values.list_marker_kinds, rich_text.values.list_marker_gap` | field |
| K-SF15† | `rich_text.task_box.stroke; rich_text.task_box.glyph @ {checked}` | rule |
| K-SF16† | `rich_text.table.stroke, rich_text.table_cell.stroke (collapsed; per edge, style Outset: the top and left edges one shade of the stated border colour, Qt::darkGray, not a role, and the bottom and right the other, as qDrawEdge's Outset computes them with qt_lighter and qt_darker, ADD-A50; the exact shades from qcssutil.cpp in the value phase), rich_text.table_cell.padding` | field |
| K-SF16† | `rich_text.table_header_cell.text (its weight restated)` | field |
| K-SF17† | `rich_text.rule.stroke (= the Inactive group's window text), rich_text.paragraph.margin (an expression on font_size)` | field |
| K-SF18† | `rubber_band.body.fill @ variant dock_target` (the content variant the connector sets where the band marks the main window's dock drop zone; over a floating dock group the outline alone) | rule |
| K-SF18† | `rubber_band.body.stroke (outline only elsewhere), rubber_band.body.corners` | field |
| K-SF18† | `component.dock_widget.values.presentations (Move: a dock widget moved aside as the gap slides open, AnimatedDocks; as K3.38†)` | component |
| K-SF19† | `dialog.values.placement @ variant separate_window (QtDialog)` | placement |
| K-SF20† | - | out (R1.1) |
| K-SF21† | `dialog.values.placement @ cond flavor:kirigami (CenteredInHost)` | placement |
| K-SF22† | `dialog.values.placement (CenteredInHost, host Overlay) @ cond flavor:kirigami` | placement |
| K-SF23† | `sheet.values.anchor, sheet.values.min_top_inset @ variant @overlay` | field |
| K-RL01† | `component.command_bar.frame.fill, component.command_bar.frame.stroke, component.command_bar.frame.padding` | component |
| K-RL01† | `component.command_bar.frame.shadow (a ShadowLayer coloured role.@shade_dark)` | component |
| K-RL01† | `input @ variant flat context component.command_bar` | rule |
| K-RL01† | `input.icon @ variant leading context component.command_bar` | rule |
| K-RL01† | `button @ variant clear_field context component.command_bar` | rule |
| K-RL01† | `list @ context component.command_bar; list.cell @ context component.command_bar (the shortcut column)` | rule |
| K-RL01† | `keycap.key.fill (= role.window.background), keycap.key.shadow (a zero-blur layer coloured role.@shade_shadow), keycap.key.corners, keycap.key.padding (= the focus frame margin plus component.command_bar.values.keycap_hmargin_extra), keycap.key.min_height (= the line spacing) @ context component.command_bar` | rule |
| K-RL01† | `keycap.key.text (fill = the button foreground) @ context component.command_bar` | rule |
| K-RL01† | `keycap.separator.text (fill = the row's text) @ context component.command_bar / context component.command_bar context list.row {selected}` | rule |
| K-RL01† | `component.command_bar.frame.{min_width,max_width} (= max(div(element.window_width, <divisor>), <minimum width>)), component.command_bar.frame.{min_height,max_height} (= max(div(element.window_height, <divisor>), <minimum height>))` (its width and height follow the space it opens in, with a floor: one home in the frame's fields) | component |
| K-RL01† | `component.command_bar.values.placement (KCommandBar; the top offset its edge_margin.top)` | placement |
| K-RL01† | `role.@shade_dark, role.@shade_shadow` | role |
| K-RL02† | `list.disclosure.glyph @ context shortcuts_dialog / context shortcuts_dialog dir rtl / {open} context shortcuts_dialog (the in-row extender's arrow on the item panel, open: its own state)` | rule |
| K-RL02† | `list.row_background @ context shortcuts_dialog (the item panel)` | rule |
| K-RL02† | `button.icon.glyph (ThemeIcon configure), button.label.text (the native shortcut text) @ variant @key_sequence context shortcuts_dialog` | rule |
| K-RL02† | `button.body.fill, button.body.stroke (held pressed), button.label.text @ variant @key_sequence {@recording} context shortcuts_dialog` | rule |
| K-RL02† | `button.icon.glyph (ThemeIcon by direction) @ variants {tool,clear_field} context shortcuts_dialog / variants {tool,clear_field} context shortcuts_dialog dir rtl` | rule |
| K-RL03† | - (plasma-emojier, Plasma's emoji picker: a separate Plasma program, `emojier/app/org.kde.plasma.emojier.desktop`, which no application draws) | out (R1.1) |
| K-RL04† | `component.char_select.table.fill (Surface View background), component.char_select.table.bindings` | component |
| K-RL04† | `component.char_select.cell.fill (= the View surface's background)` | component |
| K-RL04† | `component.char_select.cell.fill (= the Button surface's background) @ {@glyph_missing}` | component |
| K-RL04† | `component.char_select.cell.text (the chosen font, centred)` | component |
| K-RL04† | `component.char_select.cell.min_width (= add(max(max(min(element.max_char_width, mul(element.font_x_height, <max factor>)), mul(element.font_x_height, <min factor>)), element.font_height), mul(2, <text margin>)); data max_char_width, font_x_height, font_height)` (the widest glyph, `QFontMetrics::maxWidth`, ADD-A98) | component |
| K-RL04† | `component.char_select.cell.min_height (= max(mul(element.font_x_height, <factor>), max(<minimum>, add(element.font_height, <constant>))))` | component |
| K-RL05† | `input.body.stroke (Absent: frameless, the fill only, K1.41) @ context list.cell {editing}` | rule |
| K-RL05† | `input.body.min_width (Auto: widens with its content) @ context list.cell {editing}` | rule |
| K-RL06† | `drag_image.image.opacity, drag_image.values.hotspot (the press point) @ context list` | rule |
| K-RL06† | `list.row_background.fill, list.row.text.fill (drawn as under {selected}) @ {dragged}` | rule |
| K-RL07† | `button.values.repeat_delay, button.values.repeat_interval` | field |
| K-RL07† | `tab.values.repeat_delay, tab.values.repeat_interval` (the scroll arrows' auto-repeat) | field |
| K-FM01† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| K-FM01† | `reader:kde:fontconfig emoji family (fc-match emoji)` | reader |
| K-FM02† | `token.text.fallbacks.families, token.text.fallbacks.per_language (ADD-A41)` | token |
| K-FM02† | `reader:kde:fontconfig per language (fc-match -s family:lang=tag)` | reader |
| K-FM03† | `Selector.direction Rtl (S7): the dir rtl rules per part` | rule |
| K-FM03† | `Glyph.mirroring, token.icons.mirroring (S2.9)` | field |
| K-FM03† | `window.values.caption_button_layout @ dir rtl` | rule |
| K-FM03† | `reader:kde:QGuiApplication layoutDirection` | reader |
| K-P3-1† | `window.values.resize_band (outside per side, = token.params.layout.large_spacing; the top band above the title bar)` | field |
| K-P3-1† | `window.values.resize_band (the sides and the bottom) @ cond system_setting:no_borders / (the sides only) @ cond system_setting:no_side_borders` | rule |
| K-P3-1† | `window.values.resize_band (Absent on a maximised axis) @ cond maximized / cond tiled:top / cond tiled:bottom / cond tiled:left / cond tiled:right` | rule |
| K-P3-1† | `window.resize_zone.cursor @ variant top / variant right / variant bottom / variant left / variant top_left / variant top_right / variant bottom_right / variant bottom_left (the theme's resize shapes, K3.64)` | rule |
| K-P3-2† | `window.values.caption_button_capabilities (Closable, Maximizable, Minimizable, Shadeable, ProvidesHelp; WhenMissing Hide)` | field |
| K-P3-3† | - | out (R1.1) |
| K-P3-4† | `tab.label.text.elision (None)` | field |
| K-P3-4† | `tab.values.overflow, tab.values.scroll_to_selected, tab.values.scroll_margin` | field |
| K-P3-4† | `tab.values.overflow (flick, snapping to tabs), tab.values.scroll_margin @ cond flavor:kirigami; tab.scroll_arrow (absent = true) @ cond flavor:kirigami` | rule |
| K-P3-4† | `tab.values.tab_gap (negative: the overlap) @ cond flavor:kirigami` | rule |
| K-P3-5† | `button.values.menu_placement (QtMenu: below the button, else above; aligned to the button's end under dir rtl; beside it in a vertical toolbar; use_full_screen_height) @ variant menu / variant tool` | placement |
| K-P3-5† | `combo_box.values.popup_placement (QtComboBox: below the field, not over the current item, else above, else the larger side; max_visible_items)` | placement |
| K-P3-5† | `combo_box.values.popup_placement (below the control) @ cond flavor:kirigami` | rule |
| K-P3-5† | `menu.body.corners @ {placed_below} / {placed_above} (the joined edge, K1.64)` | rule |
| K-P3-6† | `tooltip.values.placement (QtTooltip)` | placement |
| K-P3-7† | `dialog.values.message_wrap.{wrap_above_px,min_width_px,elide_above_px,scroll_above_px} @ variant alert (expressions on element.screen_width, element.screen_height and widget.message.char_width, the message part's font: a widget value has none, S2.2; ADD-B17)` | field |
| K-P3-7† | `dialog.values.message_wrap.item_list_max_width_px (= mul(element.screen_width, <fraction>)), dialog.values.message_wrap.details_min_lines @ variant alert` | field |
| K-P3-8† | `about_dialog.icon.icon.size, about_dialog.name.text, about_dialog.version.text @ cond flavor:widgets` | rule |
| K-P3-8† | `tab @ context about_dialog` | rule |
| K-P3-8† | `tab.scroll_arrow (absent = true) @ context about_dialog` | rule |
| K-P3-8† | `about_dialog.credits.text (word-wrapped, with links); about_dialog.section_title.text (the components' names, their weight restated) @ cond flavor:widgets` | rule |
| K-P3-8† | `about_dialog.person_name.text (its weight restated), about_dialog.person_task.text, about_dialog.person_task.opacity @ cond flavor:widgets` | rule |
| K-P3-8† | `button.icon.glyph (ThemeIcon) @ variant icon_only context about_dialog (internet-services, mail-send)` | rule |
| K-P3-8† | `dialog.action_area @ context about_dialog` | rule |
| K-P3-8† | `button @ variant close context dialog.action_area context about_dialog` | rule |
| K-P3-9† | `dialog; input @ context dialog; list @ context dialog; expander @ context dialog; checkbox @ context dialog` (QPrintDialog, QPageSetupDialog and KIO's KOpenWithDialog, widget dialogs drawn in the application's process, in by R1.1: KOpenWithDialog's URL requester an `input`, its application tree a `list` with its header hidden, so no header is queried, its "Terminal options" group the KCollapsibleGroupBox `expander` of K1.250–K1.251, its check boxes `checkbox`) | rule |
| K-P3-9† | `page_preview.{body.padding,body.min_width,body.min_height,page_shadow.stroke,page_shadow.transform,page.fill,margins.stroke,margins.padding,filler_text.text}, page_preview.values.{filler_gap_fraction,shadow_lines}` (QPageSetupDialog's page preview, `qpagesetupdialog_unix.cpp:105-142`: the page `= role.@shade_light`; its shadow lines, one `page_shadow` element each, the bottom and right edges of the page's box moved out by the line's step, in `role.@shade_mid` at an opacity falling with `element.item_index`; the dotted margin rectangle and the filler text in `role.@shade_dark`, the filler text's font scaled from the body font by the source's factor) | field |
| K-P3-9† | - (the print and Open With dialogs of a request that goes through the portal, which xdg-desktop-portal-kde draws in its own process: `xdg-desktop-portal-kde src/print.cpp:520`, `src/AppChooserDialog.qml`) | out (R1.1) |
| K-P3-10† | `chart.body.fill, chart.body.bindings (Surface(View)) @ variant @line / variant @pie` | rule |
| K-P3-10† | `chart.series.fill, chart.legend_swatch.fill (= kqc_series_color(<seed> = role.global.accent, element.item_count, element.item_index), ADD-A57, ADD-B15)` | field |
| K-P3-10† | `chart.series.stroke @ variant @line` (the line chart's stroked series, its width and paint by the same expression) | rule |
| K-P3-10† | `chart.grid_line_major.stroke, chart.grid_line_minor.stroke (a stated colour, not a role), chart.values.grid_lines` | field |
| K-P3-10† | `chart.axis_label.text, chart.values.labels_per_axis` | field |
| K-P3-10† | `chart.legend.padding, chart.legend_entry.padding (each entry's padding, `LegendDelegate.qml`), chart.legend_swatch.min_width, chart.legend_label.text, chart.values.legend_gap` | field |
| K-P3-10† | `chart.values.default_size @ variant @pie` | rule |
| K-P3-11† | `code_editor.caret.min_width, code_editor.caret.fill (= element.color: the colour of the text under the caret, which the connector supplies as the part's data)` | field |
| K-P3-11† | `code_editor.extra_caret.fill (= element.color: the colour of the text under each secondary caret, its own element's data), code_editor.extra_caret.min_width, code_editor.extra_caret.text` (`extra_caret` an element per secondary caret; its `text` the editor's font, which it measures, as an element part reads none of its host's parts, S2.2; `data = { extra_caret = ["color", "char_width"] }`) | field |
| K-P3-11† | `code_editor.caret.fill, code_editor.caret.min_width, code_editor.caret.min_height, code_editor.extra_caret.fill, code_editor.extra_caret.min_width, code_editor.extra_caret.min_height @ variant @vi_block (the caret: widget.text.char_width, having no font of its own; the extra caret: element.char_width of its own text, S2.2; alpha) / variant @vi_underline / variant @vi_half_block` (secondary carets drawn the same way) | rule |
| K-P3-11† | `token.caret.blink_on, token.caret.blink_off (K3.41)` | token |
| K-P3-12† | `code_editor.minimap.fill, code_editor.minimap.stroke (the line along its edge, the document's end line), code_editor.values.minimap_width` | field |
| K-P3-12† | `code_editor.minimap_slider.fill, code_editor.minimap_slider.stroke` | field |
| K-P3-12† | `code_editor.minimap_fade.fill` | field |
| K-P3-12† | `code_editor.scroll_mark.fill, code_editor.scroll_mark.min_height (Absent by default: marks off)` | field |
| K-P3-12† | `code_editor.scroll_mark.fill` underlay layer `= kte_mark_underlay(role.@shade_mid, surface.background, surface.foreground)` (ADD-A57) | field |
| K-P3-13† | `menu.body.stroke (a raised box frame, K2.21) @ context autocomplete context code_editor` | rule |
| K-P3-13† | `menu.item, menu.item_label.text (the syntax highlighting of the completed text) @ context autocomplete context code_editor` | rule |
| K-P3-13† | `autocomplete.values.placement (KTextEditorCompletion: under the completion's start, its gap = an expression on the font's height; kept inside the window) @ context code_editor` | placement |
| K-P3-13† | `autocomplete.details_pane @ context code_editor (the argument hints and the documentation tip)` | rule |
| K-P3-14† | `code_editor.annotation_border.stroke @ {hover}; code_editor.annotation_border.padding` | rule |
| K-P3-15† | `token.theme_change (the toolkit's own: KWin fades the windows)` | token |
| K-P3-15† | - (KWin's Blend Changes cross-fade of every window) | out (R1.1) |
| K-P3-16† | `token.behavior.key_repeat.delay, token.behavior.key_repeat.interval` | token |
| K-P3-16† | `token.behavior.key_repeat` (kcminputrc's `KeyRepeat` off, keys never repeating: `none`, `Behavior.key_repeat`'s `None`; ADD-A104) | token |
| K-P3-16† | `token.behavior.double_click_distance, token.behavior.touch.double_tap_slop, token.behavior.touch.slop (the flick start)` | token |
| K-P3-16† | `reader:kde:kcminputrc [Keyboard] RepeatDelay, RepeatRate, KeyRepeat` | reader |
| K-P3-17† | `scrollbar.values.repeat_delay, scrollbar.values.repeat_interval (the arrows and the trough)` | field |
| K-P3-17† | `slider.values.repeat_delay, slider.values.repeat_interval (the page steps)` | field |
| K-P3-18† | `token.behavior.scroll.{wheel_step_duration,wheel_step_duration_max,wheel_step_timing}` (the duration growing with the distance between the two bounds, ADD-A86) | token |
| K-P3-18† | `token.behavior.scroll.deceleration, token.behavior.scroll.max_fling_velocity @ cond flavor:kirigami` (ScrollablePage's touch flick) | rule |
| K-P3-18† | `token.behavior.scroll.touchpad_deceleration, token.behavior.scroll.touchpad_timing @ cond flavor:kirigami` (the touchpad inertia after a gesture: its deceleration and easing curve, stopped at the content's ends; its starting velocity, estimated from the last deltas, `touchpad_velocity_samples`) | rule |
| K-P3-18† | `token.behavior.scroll.touchpad_velocity_samples @ cond flavor:kirigami` (the deltas the inertia's starting velocity is estimated from) | rule |
| K-P3-18† | `token.behavior.scroll.overscroll_effect (to verify)` | token |
| K-P3-18† | `reader:kde:kdeglobals [KDE] SmoothScroll` | reader |
| K-P3-19† | - (KWin's pointer effects, which KWin draws over every window; its Hide Cursor effect hides the pointer itself on a key press, with no toolkit asking, `hidecursor/hidecursor.cpp:91-97`, apart from KLineEdit's own hiding, K3.65†) | out (R1.1) |
| K-P3-20† | `Edge.snap (ADD-A33) on PartStyle.stroke: None for Breeze's frames and symbols` | field |
| K-P3-20† | `window.frame.stroke (snap AtLeastOneDevicePixel)` | field |
| K-P3-20† | `separator.line.stroke (snap ToDevicePixel) @ cond flavor:kirigami` | rule |
| K-P3-21† | - | out (R1.1) |
| K-P3-22† | - | out (R1.1) |
| K-P3-23† | - | out (R1.1) |
| K-P3-24† | `cond system_setting:ui_effects (ADD-A04)` | condition |
| K-P3-24† | `menu.values.presentations, tooltip.values.presentations @ !cond system_setting:ui_effects` (the empty list: the menu and the tooltip appear without animation; `UI_AnimateMenu`, `UI_FadeMenu`, `UI_AnimateTooltip`, `UI_FadeTooltip`) | rule |
| K-P3-24† | `menu.values.presentations @ !cond system_setting:ui_effects context combo_box` (the combo box's popup, `UI_AnimateCombo`; the tool box's page animation, `UI_AnimateToolBox`, on `component.tool_box` where the value phase confirms it) | rule |
| K-P3-24† | `reader:kde:kdeglobals [KDE] GraphicEffectsLevel` | reader |
| K-RV1† | `scrollbar.values.trough_click` | field |
| K-RV1† | `reader:kde:kdeglobals [KDE] ScrollbarLeftClickNavigatesByPage -> scrollbar.values.trough_click` | reader |
| K-RV2† | `menu.values.submenu_close_delay` | field |
| K-RV2† | `menu.values.submenu_switch (CloseTimer)` | field |
| K-P4-1† | `menu @ context window.window_menu (the [menu] rows, K1.60-K1.73)` | rule |
| K-P4-1† | `window.window_menu.bindings (the decoration palette, K1.13)` | field |
| K-P4-1† | `window.values.window_menu_items (MenuEntry: the actions, the More Actions submenu and the separators; checkable items; disabled per capability)` | field |
| K-P4-1† | `window.values.window_menu_icons (each action's ThemeIcon)` | field |
| K-P4-1† | `window.values.window_menu_placement (QtMenu: at the pointer; below the button's rect)` | placement |
| K-P4-1† | `window.values.title_bar_click_actions (right_click)` | field |
| K-P4-1† | `reader:kde:kwinrc [MouseBindings] CommandActiveTitlebar3, CommandInactiveTitlebar3` | reader |
| K-P4-2† | - (the picture-in-picture window's layer, size bound, home corner and margin, which KWin applies; its content and controls are the application's own drawing, no model value) | out (R1.1) |
| K-P4-3† | - | out (R1.1) |
| K-P4-4† | - (the " (Not Responding)" suffix KWin adds to the caption while the application does not answer, and the kill prompt, which KWin's separate helper program draws, `kwin src/helpers/killer/CMakeLists.txt:7`) | out (R1.1) |
| K-P4-5† | - | out (R1.1) |
| K-P4-6† | - | out (R1.1) |
| K-P4-7† | `token.pointer.drag_cursors (copy, move, the grabbing hand for none and ask, not-allowed; each name with its fallbacks in the theme; ADD-A46)` | token |
| K-P4-8† | - | preset (R1.3) |
| K-P4-9† | `text_area.body.padding` | field |
| K-P4-9† | `text_area.values.document_margin` | field |
| K-P4-10† | `menu @ context any toolbar (the toolbar's context menu: its sections, the Text Position, Icon Size and Orientation submenus, checkable entries, icons; as a "Toolbar Settings" submenu in a split button's menu)` | rule |
| K-P4-10† | `combo_box @ context toolbar_editor` | rule |
| K-P4-10† | `separator @ context toolbar_editor` | rule |
| K-P4-10† | `list @ context toolbar_editor (the available and current actions: their rows' icons' size = token.icons.sizes.small.size)` | rule |
| K-P4-10† | `list.drop_indicator (absent = true) @ pos first context toolbar_editor` (no drop indicator on Available actions, the first of the editor's two lists) | rule |
| K-P4-10† | `list.view.{min_width,min_height} @ pos first context toolbar_editor / pos last context toolbar_editor` (Available actions' minimum, then Current actions': the lists told apart by their order, `PositionMatch` among the `list` siblings) | rule |
| K-P4-10† | `input @ context toolbar_editor (the filter lines)` | rule |
| K-P4-10† | `button.icon.glyph (ThemeIcon; previous and next swapped under dir rtl) @ variants {tool,icon_only} context toolbar_editor (the four move buttons in a cross)` | rule |
| K-P4-10† | `button.values.repeat_delay, button.values.repeat_interval @ context toolbar_editor (up and down)` | rule |
| K-P4-10† | `button @ context toolbar_editor (Change Icon, Change Text)` | rule |
| K-P4-10† | `toolbar_editor @ context dialog.body; button @ context dialog.action_area` (KEditToolBar is a dialog: the editor in its body, the button box, Defaults, OK, Apply and Cancel, its own action area) | rule |
| K-P4-11† | `token.behavior.spring_loading (ADD-A45)` | token |
| K-P4-11† | `breadcrumb.values.drag_hover_delay (the crumb's arrow opens its sub-folder menu)` | field |
| K-P4-11† | `list.values.drag_hover_delay (QTreeView's autoExpandDelay, off by default); list.values.drag_hover_delay @ context sidebar.pane[@places] (the place's activation, off by default)` | rule |
| K-P4-12† | `button.icon.glyph (ThemeIcon: the swapped previous and next names) @ variant previous dir rtl / variant next dir rtl` (KStandardGuiItem's back and forward with `UseRTL`; KAssistantDialog's buttons, `button @ context wizard.action_area`) | rule |
| K-P4-12† | `button.icon.glyph (ThemeIcon: the swapped arrow names) @ variant previous context calendar.header dir rtl / variant next context calendar.header dir rtl` (KDatePicker's arrows) | rule |
| K-P4-12† | `button.icon.glyph (ThemeIcon: the swapped arrow names) @ variant icon_only context toolbar_editor dir rtl` (KEditToolBar's move buttons) | rule |
| K-P4-12† | `button.icon.glyph (ThemeIcon: the swapped arrow names) @ variant previous context component.char_select dir rtl / variant next context component.char_select dir rtl` (KCharSelect's history arrows) | rule |
| K-P4-12† | `button.icon.glyph (ThemeIcon: the -rtl name in LTR, the -ltr name in RTL) @ variant clear_field context any shortcuts_dialog dir ltr / variant clear_field context any shortcuts_dialog dir rtl` (KKeySequenceWidget's clear button) | rule |
| K-P4-12† | `token.icons.mirroring (Mirroring::SwapWith: Back with Forward, Prior with Next; ADD-A93)` (KStandardAction's Back / Forward and Prior / Next: under right-to-left the role takes the other role's icon) | token |
| K-P4-13† | `scrollbar.values.overscroll_shrink @ cond flavor:kirigami` | rule |
| K-P4-13† | `scrollbar.thumb.min_height, scrollbar.thumb.min_width (whether it bounds the shortened thumb: to verify)` | field |
| K-P4-14† | `rich_text.heading_1.text … rich_text.heading_6.text (heading_1 to heading_5 with the weight restated and text.font.size = mul(base, <factor>); heading_6 without either) @ variant html` | rule |
| K-P4-14† | `rich_text.heading_1.margin … rich_text.heading_6.margin, rich_text.{paragraph,preformatted,block_quote,definition_term,definition_description}.margin, rich_text.values.list_indent_per_level (the list's) @ variant html` | rule |
| K-P4-14† | `rich_text.inline_code.text, rich_text.preformatted.text (font = token.text.keyed.fixed; preserved white space) @ variant html` | rule |
| K-P4-14† | `rich_text.list_marker @ variant html (nested lists indent, rich_text.values.list_indent_per_level)` | rule |
| K-P4-14† | `link.text.text (fill = the link role, underlined) @ variant inline context rich_text` | rule |
| K-P4-14† | `rich_text.table_header_cell.text (its weight restated, centred), rich_text.table_cell.text @ variant html` | rule |
| K-KA-1† | `form.card.bindings (Surface(View)) @ variant @ka_form_card cond flavor:kirigami` | rule |
| K-KA-1† | `form.card.stroke (= ka_card_border(surface.background, surface.foreground), ADD-A66: kirigami_lerp(surface.background, surface.foreground, 0.15) on a light background, FormCardBackground.qml:18-23) @ variant @ka_form_card` | rule |
| K-KA-1† | `form.card.shadow (one layer) @ variant @ka_form_card / variant @ka_form_card cond scheme:dark (its blur)` | rule |
| K-KA-1† | `form.values.breakpoints (the card width limit: measured Allocation, the card's width, width Above it, `FormCard.qml:97`'s "wider than", setting `@width_limited`; R16)` | field |
| K-KA-1† | `form.card.corners (rounded), form.card.padding @ variants {@ka_form_card,@width_limited} (set by form.values.breakpoints, R16); form.card.corners (square), form.card.padding (past both sides) @ variant @ka_form_card !variant @width_limited` | rule |
| K-KA-2† | `form.row.fill.append (= alpha(surface.foreground, <fraction>)) @ {hover} variant @ka_form_card !cond tablet_mode / {focus_visible} variant @ka_form_card / {pressed} variant @ka_form_card / {disabled} variant @ka_form_card` | rule |
| K-KA-2† | `form.row.transitions (Fill; = token.motion.durations.short, Timing::Linear: no easing set) @ variant @ka_form_card` | rule |
| K-KA-2† | `form.row.corners @ pos first variants {@ka_form_card,@width_limited} / pos last variants {@ka_form_card,@width_limited} (the card's radius)` | rule |
| K-KA-2† | `form.row.focus (Absent) @ variant @ka_form_card` | rule |
| K-KA-3† | `form.row.padding, form.row_icon.icon.size (= token.icons.sizes.keyed.small_medium.size), form.values.icon_text_gap @ variant @ka_form_card` | rule |
| K-KA-3† | `form.row_title.text (wrap Word, max_lines, elision End; ADD-A67) @ variant @ka_form_card` | rule |
| K-KA-3† | `form.row_title.text (max_lines one) @ variant @ka_form_card !variant button !variant switch !variant check !variant radio (a plain text row)` | rule |
| K-KA-3† | `form.row_subtitle.text.fill (= surface.disabled.foreground: the card's View set), form.row_subtitle.padding @ variant @ka_form_card / (under the whole row) @ variants {check,@ka_form_card} / variants {radio,@ka_form_card}` | rule |
| K-KA-3† | `form.values.leading_gap, form.values.trailing_gap @ variants {button,@ka_form_card} / variants {switch,@ka_form_card} / variants {check,@ka_form_card} / variants {radio,@ka_form_card}` | rule |
| K-KA-4† | `form.row_separator.opacity, form.row_separator.margin (its insets) @ variant @ka_form_card` | rule |
| K-KA-4† | `form.row_separator.opacity, form.row_separator.margin (none) @ prev form.row {hover} !cond tablet_mode / prev form.row {focus_visible} / prev form.row {pressed} / next form.row {hover} !cond tablet_mode / next form.row {focus_visible} / next form.row {pressed}` (the tablet-mode exclusion on hover only, as the row states; ADD-B07) | rule |
| K-KA-4† | `form.row_separator.transitions (Opacity, Geometry; = token.motion.durations.long, its timing)` | field |
| K-KA-4† | `form.values.row_gap (autoSeparators: one centred in each gap)` | field |
| K-KA-5† | `form.group_title.text (the default font, its weight restated, wrap Word), form.group_title.bindings (Surface(View)), form.group_title.padding @ variant @ka_form_card / variants {@ka_form_card,@width_limited}` | rule |
| K-KA-5† | `button @ variant tool context form.group_title (the trailing actions)` | rule |
| K-KA-5† | `form.group_description.text (fill = surface.disabled.foreground; centred) @ variant @ka_form_card` | rule |
| K-KA-5† | `form.group_description.{max_width,margin} @ variant @ka_form_card` | rule |
| K-KA-6† | `form.page.fill (= kirigami_lerp(role.window.background, role.view.background, 0.5), FormCardPage.qml:45-56) @ variant @ka_form_card` | rule |
| K-KA-6† | `form.page.padding (none: the cards reach the page's edges) @ variant @ka_form_card !variant @width_limited (below the card breakpoint, R16)` | rule |
| K-KA-7† | `form.row_arrow.icon (the arrow theme icon and its size), form.values.arrow_gap @ variant @ka_form_card / variant @ka_form_card dir rtl` | rule |
| K-KA-7† | `form.row_arrow @ !{hover} !{focus_visible} !{pressed} variant @ka_form_card` (`absent = true`: shown only while the row has its background) | rule |
| K-KA-7† | `form.row_arrow.icon (the open-link icon; its rtl form under dir rtl) @ variant @link` | rule |
| K-KA-7† | `menu @ context form.row variant @link (Open Link, Copy Link, K-KA-31†)` | rule |
| K-KA-7† | `form.row_arrow.icon (the up arrow) @ {open} variant expander` | rule |
| K-KA-7† | `form.values.presentations (@expand, @collapse: the rows' height and opacity; = token.motion.durations.long, its timing) @ variant @ka_form_card` | rule |
| K-KA-7† | `form.row_separator @ prev form.row variant expander {open} (under the open header row)` | rule |
| K-KA-8† | `checkbox.body.padding (none) @ context form.row variant @ka_form_card / radio.body.padding @ context form.row variant @ka_form_card / switch.body.padding @ context form.row variant @ka_form_card` (the label is the row's title, so the control shows none and its `label` is not queried, S6.3) | rule |
| K-KA-8† | `form.values.leading_gap @ variants {check,@ka_form_card} / variants {radio,@ka_form_card} (to the icon or text); form.values.icon_text_gap @ variants {radio,@ka_form_card}; form.values.trailing_gap @ variants {switch,@ka_form_card}` | rule |
| K-KA-8† | `form.row_title.text.fill (= surface.disabled.foreground) @ {disabled} variant @ka_form_card (a colour, not the qqc2 opacity)` | rule |
| K-KA-9† | `form.field_label.text (above the field; wrap Word, max_lines, ADD-A67), form.values.field_label_gap @ variant @ka_form_card` | rule |
| K-KA-9† | `form.field_label.text.fill (= surface.disabled.foreground) @ {disabled} variant @ka_form_card` | rule |
| K-KA-9† | `form.row_subtitle.text.fill (= surface.disabled.foreground) @ variants {entry,@ka_form_card} (the description under the field)` | rule |
| K-KA-9† | `input.counter.text (= token.text.keyed.smallest_readable; aligned to the end), input.counter.text.fill @ {@at_limit} context form.row variant @ka_form_card` | rule |
| K-KA-9† | `slider.value_label.text, slider.values.value_label_position @ context form.row variant @ka_form_card (the value beside the label)` | rule |
| K-KA-9† | `form.row_edit_icon.icon.size (= token.icons.sizes.keyed.small_medium.size), form.row_subtitle.text.fill (the value) @ variants {entry,@ka_form_card} cond mobile_form_factor` | rule |
| K-KA-9† | `input @ context form.row cond mobile_form_factor {editing}; button.icon.glyph (ThemeIcon dialog-ok) @ variant done context form.row` | rule |
| K-KA-10† | `banner.body.fill (= kirigami_lerp(surface.background, surface.foreground_active / surface.positive / surface.neutral / surface.negative, 0.2), the type's colour in the variants' order, FormInlineMessage.qml:20-31, :37), banner.body.corners, banner.body.padding @ variants {@ka_inline_message,info} / variants {@ka_inline_message,success} / variants {@ka_inline_message,warning} / variants {@ka_inline_message,error}` | rule |
| K-KA-10† | `banner.body.stroke (Absent), banner.inner (absent = true: the body's one fill), banner.icon.icon.size (= token.icons.sizes.keyed.small_medium.size), banner.icon.icon.fill (the type's colour; top-aligned), banner.icon.glyph (ThemeIcon), banner.values.content_gap @ variant @ka_inline_message` | rule |
| K-KA-10† | `banner @ variant @ka_inline_message context form.row (its gap above the field)` | rule |
| K-KA-11† | `input.values.strength_meter_segments, input.values.strength_meter_gap, input.strength_meter.corners, input.strength_meter.min_height @ variant secure` | rule |
| K-KA-11† | `input.strength_meter.fill (= role.<set>.background_negative / background_neutral / background_positive), input.strength_meter_fill.fill (the matching text role; its length by element.progress, the entropy) @ variant secure {@quality_bad} / variant secure {@quality_poor} / variant secure {@quality_weak} / variant secure {@quality_good} / variant secure {@quality_excellent}` | rule |
| K-KA-12† | `combo_box @ context form.row variant @ka_form_card !cond mobile_form_factor` | rule |
| K-KA-12† | `form.row (K-KA-2† fills), form.row_subtitle.text.fill (the value, = surface.disabled.foreground), form.row_arrow.icon (the down arrow) @ variants {combo,@ka_form_card} cond mobile_form_factor` | rule |
| K-KA-12† | `dialog.values.natural_size, dialog.body.max_height, dialog.title.text, dialog.header.padding @ variant @ka_list_dialog` | rule |
| K-KA-12† | `dialog.header.stroke @ variant @ka_list_dialog {scrolled}` | rule |
| K-KA-12† | `list @ variant @ka_rounded context dialog variant @ka_list_dialog (its margins)` | rule |
| K-KA-12† | `input @ context dialog variant @ka_list_dialog (the editable combo's field)` | rule |
| K-KA-12† | `separator @ context dialog variant @ka_list_dialog` | rule |
| K-KA-12† | `dialog @ variant full_size context form.row (page mode: the dialog layer, Q2.58†)` | rule |
| K-KA-13† | `input @ context form.row variant @ka_form_card; button.icon.glyph (ThemeIcon document-open-folder-symbolic) @ variant icon_only context form.row` | rule |
| K-KA-13† | `menu.body.fill, menu.body.corners, menu.body.stroke, menu.body.shadow (the K-KA-18† surface) @ context autocomplete context form.row; menu.item @ context autocomplete context form.row (the K-KA-38† rounded rows)` | rule |
| K-KA-13† | `autocomplete.values.placement (QtCompletionBox: right under the field, width = the field's) @ context form.row` | placement |
| K-KA-13† | `menu.body.max_height (= an expression over token.params.layout: grid units and small spacings) @ context autocomplete context form.row` (the popup's height limit, a length) | rule |
| K-KA-13† | `banner @ variants {@ka_inline_message,error} context form.row` | rule |
| K-KA-13† | `form.row_subtitle.text.fill (the path or "No file selected"), form.row_arrow.icon @ variant @ka_form_card cond mobile_form_factor; file_chooser (K2.26†)` | rule |
| K-KA-14† | `color_button.swatch.corners (Full), color_button.swatch.min_width, color_button.swatch.min_height (= token.icons.sizes.keyed.small_medium.size), color_button.swatch.stroke (Absent), color_button.values.swatch_gap @ context form.row variant @ka_form_card` | rule |
| K-KA-14† | `form.row_icon.icon (the color-picker icon) @ variant @ka_form_card context color_button` | rule |
| K-KA-14† | `form.row_icon.icon.size (= token.icons.sizes.keyed.small.size) @ variants {@icon_row,@ka_form_card}` | rule |
| K-KA-14† | `form.row_suffix.icon.size (= token.icons.sizes.keyed.small.size) @ variants {@icon_row,@ka_form_card}` (the chosen icon) | rule |
| K-KA-15† | `form.values.grid_columns, form.values.grid_min_cell_width, form.values.grid_gap @ variant @ka_info_card` | rule |
| K-KA-15† | `form.card.padding, form.card.corners @ variant @ka_info_card / variants {@ka_info_card,@width_limited}` | rule |
| K-KA-15† | `form.row_icon.icon (top-aligned), form.row_title.text (= token.text.keyed.heading_4_primary; centred without an icon), form.row_subtitle.opacity @ variant @ka_info_card` | rule |
| K-KA-15† | `form.row.fill.append (= alpha(surface.foreground, <fraction>)) @ {hover} variant @ka_info_card / {focus_visible} variant @ka_info_card / {pressed} variant @ka_info_card; form.row.transitions (Fill; = token.motion.durations.short)` | rule |
| K-KA-15† | `tooltip.values.show_delay @ context form.card variant @ka_info_card` | rule |
| K-KA-16† | `status_page.icon.icon.size (= token.icons.sizes.keyed.medium.size), status_page.icon.opacity, status_page.title.text (= token.text.keyed.heading_3_primary), status_page.title.opacity, status_page.description.opacity @ variant @ka_card_placeholder` | rule |
| K-KA-16† | `status_page.values.icon_to_title, status_page.values.title_to_description, status_page.body.padding, status_page.body.fill (Absent) @ variant @ka_card_placeholder` | rule |
| K-KA-17† | `dialog.body.fill, dialog.body.corners, dialog.body.stroke, dialog.body.shadow (the K-KA-18† surface) @ variant @ka_form_card_dialog` | rule |
| K-KA-17† | `dialog.values.natural_size (= sub(element.window_width, <margins>)) @ variant @ka_form_card_dialog / (= min(sub(element.window_width, <margins>), max(<n> grid units, the header's width))) @ variants {@ka_form_card_dialog,@wide}; dialog.values.breakpoints (measured Window, width Above the row's threshold, setting @wide), dialog.values.window_margin @ variant @ka_form_card_dialog` | rule |
| K-KA-17† | `dialog.title.text (a heading), dialog.header.padding (none below) @ variant @ka_form_card_dialog` | rule |
| K-KA-17† | `button.icon.glyph (ThemeIcon) @ variant close context dialog.header variant @ka_form_card_dialog / {hover} variant close context dialog.header variant @ka_form_card_dialog (the other icon)` (the connector draws the close button only while the dialog has a Cancel button, the dialog's `{@has_cancel}`: content, no absence rule) | rule |
| K-KA-17† | `dialog.action_area.padding, dialog.values.action_layout (row_gap) @ variant @ka_form_card_dialog` | rule |
| K-KA-18† | `dialog.body.corners, dialog.body.bindings (surface View), dialog.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), DialogRoundedBackground.qml:25), dialog.body.shadow @ variant @ka_dialog cond flavor:kirigami` | rule |
| K-KA-18† | `popover.body.corners, popover.body.bindings, popover.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), DialogRoundedBackground.qml:25), popover.body.shadow @ variant @ka_dialog cond flavor:kirigami` | rule |
| K-KA-19† | `dialog.icon.icon.size (= token.icons.sizes.keyed.huge.size), dialog.icon.glyph (ThemeIcon: the default type's here, each other type's by the same rule selecting that type too: info, warning, error, success), dialog.body.padding, dialog.values.natural_size, dialog.body.max_height, dialog.values.window_margin @ variants {alert,@ka_message_dialog}` (`natural_size`'s two width formulas by `@wide`, as K-KA-17†'s) | rule |
| K-KA-19† | `dialog.icon.padding, dialog.values.title_alignment (centred above), dialog.values.action_layout (stacked, full width, its side margins) @ variants {@ka_message_dialog,narrow}; dialog.values.breakpoints (measured Window, width Below the narrow width plus the dialog's side margins, setting narrow: MessageDialog.qml's own-width test restated exactly, R16), dialog.values.narrow_when_actions_overflow (true: also narrow while the buttons do not fit) @ variant @ka_message_dialog` | rule |
| K-KA-19† | `checkbox @ context dialog.action_area variant @ka_message_dialog; dialog.values.action_layout (row_gap), dialog.action_area.padding, dialog.values.standard_button_icons @ variant @ka_message_dialog` | rule |
| K-KA-19† | `dialog.values.presentations (Appear, Disappear: Fade; = token.motion.durations.long, its timing), dialog.scrim.fill @ variant @ka_message_dialog` | rule |
| K-KA-20† | `date_picker.popup @ variant with_popup cond flavor:kirigami` (`drawn_as = { widget = "dialog", variants = ["@ka_dialog"] }`: the dialog draws the one box; under `kirigami` every `with_popup` picker is kirigami-addons') | rule |
| K-KA-20† | `dialog.body.{fill,corners,shadow,min_width,max_width,min_height,padding} @ variant @ka_dialog context date_picker.body[with_popup] cond flavor:kirigami` (the popup is a `dialog` element, the K-KA-18† surface; narrower on small windows: `max_width` an expression over `element.window_width`) | rule |
| K-KA-20† | `dialog.scrim.fill, dialog.scrim.transitions (Opacity; = token.motion.durations.long) @ variant @ka_dialog context date_picker` | rule |
| K-KA-20† | `segmented_control @ variant @ka_segmented_button context dialog.body context date_picker (day, month, year: the view switch)` | rule |
| K-KA-20† | `segmented_control @ variant @ka_segmented_button context dialog.body context date_picker (previous, today, next: icon only; the glyph's rtl form @ dir rtl)` | rule |
| K-KA-20† | `calendar.values.presentations (@view_switch); calendar.values.scroll_physics (deceleration, snap) @ variant @ka_date_popup` | rule |
| K-KA-20† | `button.icon.glyph @ context dialog.action_area context date_picker (Cancel, Select, Reset); dialog.action_area.padding @ context date_picker` | rule |
| K-KA-21† | `calendar.weekday_label.opacity, calendar.day.corners, calendar.day_label.text (centred) @ variant @ka_date_popup` | rule |
| K-KA-21† | `calendar.day.fill (= kirigami_tint, ADD-A52) @ {today} variant @ka_date_popup / {selected} variant @ka_date_popup / {hover} variant @ka_date_popup; calendar.day.focus (a stroke) @ {focus_visible} variant @ka_date_popup` | rule |
| K-KA-21† | `calendar.day.fill (Absent), calendar.day.opacity @ {other_month} variant @ka_date_popup; calendar.day.opacity @ {disabled} variant @ka_date_popup` (a day outside the allowed range) | rule |
| K-KA-21† | `calendar.month.fill @ {today} variant @ka_date_popup; calendar.year.opacity @ {@other_decade} variant @ka_date_popup` | rule |
| K-KA-22† | `date_picker.field.fill (flat), date_picker.glyph.glyph (ThemeIcon view-calendar-symbolic), date_picker.field_label.text @ context form.row variant @date_time; time_picker.field.fill (flat), time_picker.glyph.glyph (ThemeIcon clock-symbolic), time_picker.field_label.text @ context form.row variant @date_time` (the time half, kirigami-addons' `with_popup` time picker, whose click opens the K-KA-23† popup) | rule |
| K-KA-22† | `date_picker.field.fill.append @ {hover} / {focus_visible} / {pressed} context form.row; time_picker.field.fill @ {hover} / {focus_visible} / {pressed} context form.row` | rule |
| K-KA-22† | `date_picker.segment_separator.opacity, date_picker.segment_separator.margin @ context form.row variant @date_time` | rule |
| K-KA-22† | `date_picker.segment_separator.opacity (none) @ {hover} context form.row / next time_picker {hover} context form.row (either half hovered; position among Kinds, ADD-B07)` | rule |
| K-KA-22† | `form.field_label.padding, form.row_subtitle (status and description inset) @ variants {@ka_form_card,@date_time}; date_picker.field.fill (Absent) @ {read_only} context form.row; time_picker.field.fill (Absent) @ {read_only} context form.row` | rule |
| K-KA-23† | `dialog.{body.fill,body.corners,body.stroke,body.shadow,scrim} @ variant @ka_dialog context time_picker.body[with_popup] cond flavor:kirigami` (the popup is a `dialog` element, the K-KA-18† surface); `time_picker.popup @ variant with_popup cond flavor:kirigami` (`drawn_as = { widget = "dialog", variants = ["@ka_dialog"] }`: the dialog draws the one box; under `kirigami` every `with_popup` picker is kirigami-addons') | rule |
| K-KA-23† | `picker_wheel.item.min_height, picker_wheel.values.visible_items, picker_wheel.item.text.font.size (= mul(base, <factor>), ADD-A56: a factor on the font's own pixel size) @ context time_picker.body[with_popup] cond flavor:kirigami` (the tumbler from Qt's fallback style: which, an open item of the value phase) | rule |
| K-KA-23† | `picker_wheel.item.opacity (= an expression over element.scroll_progress, element.item_index and element.item_count, ADD-B13) @ context time_picker` | rule |
| K-KA-23† | `picker_wheel.selection_band.stroke, picker_wheel.selection_band.corners @ context time_picker / {focus_visible} context time_picker (a thicker stroke)` | rule |
| K-KA-23† | `time_picker.time_separator.text.font.size (= mul(token.text.roles.body.font.size, <factor>): a factor on the default point size) @ variant with_popup cond flavor:kirigami` | rule |
| K-KA-23† | `button.icon.glyph (ThemeIcon) @ variant icon_only context time_picker (reset to now)` | rule |
| K-KA-24† | `segmented_control.segment.padding, segmented_control.segment.min_width (= part.min_height: a square segment), segmented_control.icon.icon.size (= token.icons.sizes.keyed.small_medium.size), segmented_control.values.item_gap (negative: the overlap) @ variant @ka_segmented_button` | rule |
| K-KA-24† | `segmented_control.segment.corners (the outer only) @ pos first variant @ka_segmented_button / pos last variant @ka_segmented_button / pos first variant @ka_segmented_button dir rtl / pos last variant @ka_segmented_button dir rtl` | rule |
| K-KA-24† | `segmented_control.segment.fill (= role.<set>.background_alternate) @ {pressed} variant @ka_segmented_button / {selected} variant @ka_segmented_button` | rule |
| K-KA-24† | `segmented_control.segment.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), SegmentedButton.qml:112) @ variant @ka_segmented_button; segmented_control.segment.stroke (= surface.decoration_focus) @ {hover} / {focus_visible} / {pressed} / {selected} variant @ka_segmented_button (the highlighted segment drawn above its neighbours: the connector's paint order)` | rule |
| K-KA-24† | `segmented_control.segment.transitions (Fill, Stroke; = token.motion.durations.short, its timing) @ {hover} / {focus_visible} / {pressed} / {selected} variant @ka_segmented_button (the Behaviors run only while the segment is highlighted)` | rule |
| K-KA-24† | `segmented_control.segment.shadow (a hard layer, one pixel lower) @ !{pressed} !{disabled} variant @ka_segmented_button` | rule |
| K-KA-25† | `segmented_control.segment.fill.append @ {hover} variant @ka_radio_selector; segmented_control.segment.corners, segmented_control.segment.min_height, segmented_control.icon.icon.size (= kirigami_label_icon_size(body_font_height): Kirigami's label-icon size, computed from the body font's height (ascent plus descent), Q1.1, ADD-A100) @ variant @ka_radio_selector` | rule |
| K-KA-25† | `segmented_control.segment.transitions (= token.motion.durations.short, its timing) @ variant @ka_radio_selector !cond input:touch` | rule |
| K-KA-25† | `segmented_control.indicator.fill (= kirigami_lerp(surface.decoration_hover, surface.background, 0.8), RadioSelector.qml:193), segmented_control.indicator.stroke (= kirigami_lerp(surface.decoration_hover, surface.background, 0.5), RadioSelector.qml:196), segmented_control.indicator.corners, segmented_control.indicator.shadow @ variant @ka_radio_selector` | rule |
| K-KA-25† | `segmented_control.values.presentations (@indicator_move, step part indicator; = token.motion.durations.long, its timing) @ variant @ka_radio_selector` | rule |
| K-KA-25† | `segmented_control.label.text (its weight restated), segmented_control.icon.icon.fill (= surface.decoration_hover) @ {selected} variant @ka_radio_selector; segmented_control.label.transitions, segmented_control.icon.transitions (= token.motion.durations.long)` | rule |
| K-KA-25† | `form.row.padding, form.row.fill (Absent) @ variant @ka_form_card (the radio selector's row)` | rule |
| K-KA-26† | `button.body.bindings (Surface(Button)), button.icon.icon.size (= token.icons.sizes.keyed.medium.size), button.body.corners, button.body.padding @ variant @ka_floating / button.body.padding @ variant @ka_floating cond input:touch` | rule |
| K-KA-26† | `button.body.fill (= surface.background), button.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), FloatingButton.qml:137) @ variant @ka_floating; button.body.fill, button.body.stroke (= kirigami_tint, ADD-A52) @ {hover} variant @ka_floating / {pressed} variant @ka_floating / {focused} variant @ka_floating` | rule |
| K-KA-26† | `button.body.shadow, button.body.transitions (Fill, Stroke; = token.motion.durations.very_short) @ variant @ka_floating` | rule |
| K-KA-26† | `button.body.touch_target @ variant @ka_floating (the margins enlarge the hit area)` | rule |
| K-KA-27† | `split_button.primary.corners, split_button.secondary.corners (the outer only) @ variant @ka_double_floating / variant @ka_double_floating dir rtl` | rule |
| K-KA-27† | `split_button.body.shadow (one shared) @ variant @ka_double_floating` | rule |
| K-KA-27† | `split_button.primary.fill, split_button.secondary.fill (= role.button.background), split_button.primary.stroke, split_button.secondary.stroke (= kirigami_lerp(role.button.background, role.button.foreground, token.params.color.frame_contrast): each half's own Button set, DoubleFloatingButton.qml:142-143, :159, :239-240, :256) @ variant @ka_double_floating; split_button.primary.fill, split_button.secondary.fill, split_button.primary.stroke, split_button.secondary.stroke (as K-KA-26†) @ {hover} / {pressed} / {focused} variant @ka_double_floating` | rule |
| K-KA-27† | `split_button.primary.transitions, split_button.secondary.transitions (= token.motion.durations.long, its timing) @ variant @ka_double_floating (the hovered or pressed half drawn above: the connector's paint order)` | rule |
| K-KA-27† | `tooltip.values.show_delay @ context split_button` | rule |
| K-KA-28† | `toolbar.bar.bindings (Surface(Window)), toolbar.bar.corners, toolbar.bar.shadow, toolbar.bar.stroke (= kirigami_tint by token.params.color.frame_contrast) @ variant floating cond flavor:kirigami` | rule |
| K-KA-29† | `banner.body.fill (= role.<set>.background_active / background_positive / background_neutral / background_negative) @ variants {@ka_banner,info} / variants {@ka_banner,success} / variants {@ka_banner,warning} / variants {@ka_banner,error}; banner.inner (absent = true: the body's one fill) @ variant @ka_banner` | rule |
| K-KA-29† | `banner.body.padding, banner.values.content_gap @ variant @ka_banner` | rule |
| K-KA-29† | `banner.edge_line.stroke (= ka_banner_line(<the type's text: surface.foreground_active, surface.positive, surface.neutral or surface.negative>, kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast)), ADD-A66, Banner.qml:239-255) @ variants {@ka_banner,@header} (bottom edge) / variants {@ka_banner,@footer} (top edge)` | rule |
| K-KA-29† | `banner.icon.icon.size (= token.icons.sizes.keyed.medium.size; top-aligned), banner.icon.glyph (ThemeIcon), banner.title.text (= token.text.keyed.heading_2) @ variant @ka_banner` | rule |
| K-KA-29† | `button.icon.glyph (ThemeIcon dialog-close) @ variant close context banner variant @ka_banner; tooltip.values.show_on_press @ context banner cond tablet_mode` | rule |
| K-KA-29† | `toolbar @ context banner variant @ka_banner (the non-flat action toolbar, aligned to the end)` | rule |
| K-KA-29† | `banner.values.presentations (Appear, Disappear: Instant) @ variant @ka_banner` | rule |
| K-KA-30† | `sheet.body.max_height (its height limit, a fraction of element.window_height), sheet.body.corners, sheet.header.corners, sheet.body.shadow @ variants {bottom,@ka_bottom_drawer}` | rule |
| K-KA-30† | `sheet.handle.min_width, sheet.handle.min_height, sheet.handle.fill, sheet.handle.opacity, sheet.handle.corners (Full), sheet.handle.padding @ variant @ka_bottom_drawer` | rule |
| K-KA-30† | `sheet.header.bindings (Surface(Window)), sheet.header_separator.stroke, sheet.body.bindings (Surface(View)), sheet.body.padding (the bottom safe area) @ variant @ka_bottom_drawer` | rule |
| K-KA-31† | `menu.body, menu.item (the qqc2 Menu, Q1.23-Q1.32) @ cond flavor:kirigami` | rule |
| K-KA-31† | `menu.values.convergent_variant` (`"none"`; `ka_bottom_drawer` `@ cond mobile_form_factor cond flavor:kirigami`: the variant the connector sets where the application states no display mode, `displayMode`'s default; else the connector sets `@ka_bottom_drawer` or `@ka_dialog` from the application's mode; both declared under `[declare.flavor.kirigami.widget.menu]` only) | rule |
| K-KA-31† | `sheet @ variant @ka_bottom_drawer context menu (K-KA-30†)` | rule |
| K-KA-31† | `menu.body @ variant @ka_bottom_drawer` (`drawn_as = { widget = "sheet", variants = ["bottom", "@ka_bottom_drawer"] }`) `/ variant @ka_dialog` (`drawn_as = { widget = "dialog", variants = ["@ka_dialog"] }`): the sheet or dialog draws the one box | rule |
| K-KA-31† | `menu.item @ variant @ka_bottom_drawer / variant @ka_dialog` (`drawn_as = { widget = "form", part = "row" }`), `menu.separator` (the same, `drawn_as = { widget = "form", part = "row_separator" }`), `menu.check_indicator @ variants {check,@ka_bottom_drawer} / variants {check,@ka_dialog}` (`drawn_as = { widget = "checkbox" }`) `/ variants {radio,@ka_bottom_drawer} / variants {radio,@ka_dialog}` (`drawn_as = { widget = "radio" }`): queried with the sheet or dialog as their host (Nesting) | rule |
| K-KA-31† | `menu.{item_label,item_subtitle,item_icon,item_trailing_icon,accelerator,submenu_arrow,scroll_arrow,section_header} (absent = true) @ variant @ka_bottom_drawer / variant @ka_dialog` (the form row's own parts take their place) | rule |
| K-KA-31† | `form.row @ context menu variant @ka_bottom_drawer / context menu variant @ka_dialog (K-KA-2†, K-KA-3†); form.row_arrow.icon (a submenu); form.row_separator @ context menu variant @ka_bottom_drawer / context menu variant @ka_dialog` | rule |
| K-KA-31† | `checkbox @ context form.row context menu / radio @ context form.row context menu (checkable actions)` | rule |
| K-KA-31† | `button.icon.glyph (ThemeIcon draw-arrow-back-symbolic) @ variant back context sheet.header` | rule |
| K-KA-31† | `sheet.title.text (= token.text.keyed.heading_2) @ variant @ka_bottom_drawer (a submenu's page)` | rule |
| K-KA-31† | `dialog.body (the K-KA-18† surface), dialog.values.natural_size, dialog.header.padding, dialog.header.stroke @ context menu variant @ka_dialog` | rule |
| K-KA-31† | `button @ variant close context dialog.header context menu` | rule |
| K-KA-32† | `input.body.opacity @ {focused} context search_field variant @ka_search_popup (the field's frame fading out)` | rule |
| K-KA-32† | `search_field.view.fill, search_field.view.corners, search_field.view.stroke, search_field.view.shadow (the K-KA-18† surface), search_field.view.bindings; search_field.view_divider.stroke @ variant @ka_search_popup` | rule |
| K-KA-32† | `search_field.values.presentations (@view_show: Fade, = token.motion.durations.short; then Scale and the height, = token.motion.durations.long; each its timing; closing reverses it, ADD-B10) @ variant @ka_search_popup` | rule |
| K-KA-32† | `search_field.values.view_placement (KirigamiAddonsSearchPopup: from the field, kept inside the window, ADD-A21)` | placement |
| K-KA-32† | `search_field.values.auto_search_min_chars; scroll_view @ context search_field.view (Surface(View))` | rule |
| K-KA-33† | `avatar.body.fill (= pick(name_index(element.text, <count>, kirigami), …), ADD-B14) @ variant @ka_avatar` (kirigami-addons' own `nameutils.cpp:82-130`, the row's "same palette"; whether its hash is Kirigami's is the value phase's check, and a difference would add a member to the core's `NameIndexMethod`) | rule |
| K-KA-33† | `avatar.body.stroke (the ring's width), avatar.initials.text (text.font.size = an expression on element.width, the avatar square, ADD-B11) @ variant @ka_avatar` | rule |
| K-KA-33† | `avatar.fallback_icon.icon (fill = ka_contrast_text(the avatar colour), ADD-A66) @ variant @ka_avatar` | rule |
| K-KA-33† | `avatar.body.padding, avatar.body.cursor, avatar.body.bindings (Surface(Button)) @ variants {@ka_avatar,@button}; avatar.body.focus (FilledRing, = surface.decoration_focus) @ {focus_visible} variants {@ka_avatar,@button}; avatar.body.shadow (one layer, no blur, no offset, its spread the padding, = surface.decoration_focus) @ {pressed} variants {@ka_avatar,@button}` (the circle shown on `visualFocus` or `down`, `AvatarButton.qml:105-109`; a focus indicator is drawn only under `FocusVisible`, S2.10) | rule |
| K-KA-33† | `tooltip.values.show_delay @ context avatar` | rule |
| K-KA-34† | `component.album_viewer.scrim.fill` | component |
| K-KA-34† | `component.album_viewer.header.fill (= surface.background_alternate, the colour set in scope), component.album_viewer.header.padding, component.album_viewer.header_separator.stroke` | component |
| K-KA-34† | `component.album_viewer.title.text (its weight restated), component.album_viewer.subtitle.text (fill = surface.disabled.foreground)` | component |
| K-KA-34† | `button.icon.glyph (ThemeIcon) @ variant icon_only context component.album_viewer (zoom, rotate, caption, save); button @ variant close context component.album_viewer (its square size)` | rule |
| K-KA-34† | `button @ variants {previous,@ka_floating} context component.album_viewer / variants {next,@ka_floating} context component.album_viewer (their margins from the edges)` | rule |
| K-KA-34† | `component.album_viewer.footer.fill, component.album_viewer.footer.padding, component.album_viewer.footer.max_height, component.album_viewer.footer_separator.stroke` | component |
| K-KA-34† | `component.album_viewer.values.image_inset, component.album_viewer.image.transitions (Transform: the zoom and the rotation; = token.motion.durations.long, its timing)` | component |
| K-KA-34† | `spinner @ context component.album_viewer` | rule |
| K-KA-35† | `media_controls.bar.fill, media_controls.bar.opacity, media_controls.bar.corners, media_controls.bar.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), VideoMaximizeDelegate.qml:365), media_controls.bar.shadow @ variant @ka_album` | rule |
| K-KA-35† | `button.icon.glyph (ThemeIcon) @ variant play context media_controls.bar / variant play {checked} context media_controls.bar / variant volume context media_controls.bar` | rule |
| K-KA-35† | `button.icon.glyph (ThemeIcon) @ variant volume {checked} context media_controls.bar` (the muted volume icon, `player-volume-muted`) | rule |
| K-KA-35† | `slider @ context media_controls (the seek bar)` | rule |
| K-KA-35† | `media_controls.time_label.text` | field |
| K-KA-35† | `popover.body.opacity, popover.body.corners, popover.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), VideoMaximizeDelegate.qml:349), popover.body.shadow @ context media_controls` | rule |
| K-KA-35† | `slider @ variant vertical context popover context media_controls (its height)` | rule |
| K-KA-35† | `media_controls.values.auto_hide, media_controls.values.volume_popup_hide_delay` | field |
| K-KA-35† | `button.body.fill, button.icon.icon.size (= token.icons.sizes.large.size) @ variant @ka_download context media_controls variant @ka_album` (the download Button in the middle before download: a `button` element with states of its own, Nested controls; a keyed `kde-breeze` variant of `button`; the play overlay part is absent on `kde-breeze`) | rule |
| K-KA-36† | `list.header.min_height (= list.row.min_height's per density), list.header_section.padding, list.header_section.fill / @ {hover} variant @ka_table, list.header_separator.stroke (right and bottom) @ variants {table,@ka_table}` | rule |
| K-KA-36† | `list.header_section.text (elision End), list.values.elided_tooltip @ variant @ka_table` | rule |
| K-KA-36† | `list.sort_indicator.glyph (ThemeIcon, size small), list.sort_indicator.transform @ {descending}, list.sort_indicator.transitions (Transform; = token.motion.durations.long) @ variant @ka_table` | rule |
| K-KA-36† | `list.header_section.cursor @ {dragged}, list.header_section.stroke @ {drop_target}, list.values.column_min_width @ variant @ka_table` | rule |
| K-KA-37† | `list.row_background.fill (= surface.decoration_hover at reduced alpha) @ {hover} variant @ka_table; list.row_background.fill (the highlight, solid) @ {selected} variant @ka_table / {pressed} variant @ka_table` | rule |
| K-KA-37† | `list.row_background.fill @ pos parity even variants {alternating_rows,@ka_table} (on by default; the qqc2 delegate's row % 2, zero-based; ADD-B06)` | rule |
| K-KA-37† | `list.cell.stroke @ {current} variant @ka_table` | rule |
| K-KA-37† | `menu @ context list variant @ka_table (right click)` | rule |
| K-KA-38† | `list.values.row_background_inset, list.row_background.corners @ variant @ka_rounded; list.values.row_background_inset @ pos first variant @ka_rounded / pos last variant @ka_rounded` | rule |
| K-KA-38† | `list.row_background.fill (= kirigami_tint, ADD-A52) @ {selected} variant @ka_rounded / {current} variant @ka_rounded / {pressed} variant @ka_rounded / {focus_visible} variant @ka_rounded / {hover} variant @ka_rounded / {selected,hover} variant @ka_rounded; list.row_background.stroke @ {focused} variant @ka_rounded` | rule |
| K-KA-38† | `list.row.text.fill @ {selected} variant @ka_rounded (unchanged)` | rule |
| K-KA-38† | `list.row.padding, list.values.leading_gap, list.row.min_height, list.leading_icon.icon.size @ variant @ka_rounded / variant @ka_rounded cond tablet_mode / variants {@ka_rounded,with_subtitle}` | rule |
| K-KA-38† | `checkbox @ context list.row[@ka_rounded,check] / radio @ context list.row[@ka_rounded,radio]; list.row_background.fill (Absent) @ {checked} variants {@ka_rounded,check} / {checked} variants {@ka_rounded,radio}` | rule |
| K-KA-39† | `list.secondary_text.text (= token.text.keyed.smallest_readable; fill at reduced alpha) @ variant @ka_rounded / variants {@ka_rounded,@emphasized}` | rule |
| K-KA-39† | `list.leading_icon.min_width, list.leading_icon.min_height (the icon's margins) @ variants {@ka_rounded,with_subtitle}` | rule |
| K-KA-39† | `list.values.row_background_inset (none), list.row_background.corners (none), list.row.padding, list.leading_icon.icon.size @ variant @ka_indicator / list.leading_icon.icon.size @ variants {@ka_indicator,with_subtitle}` | rule |
| K-KA-39† | `list.unread_indicator.fill, list.unread_indicator.min_width, list.unread_indicator.padding @ {@unread} variant @ka_indicator` | rule |
| K-KA-39† | `list.row_separator.stroke (at reduced alpha), list.row_separator.margin @ variant @ka_indicator; list.row_separator (absent = true) @ prev list.row {hover} variant @ka_indicator / next list.row {hover} variant @ka_indicator (ADD-B07)` | rule |
| K-KA-40† | `list.branch_line.stroke (at reduced alpha), list.disclosure.glyph (ThemeIcon by direction; size), list.disclosure.glyph.fill / @ {hover} @ variant @ka_tree !cond system_setting:qqc2_desktop_style (a Quick Controls style other than org.kde.desktop, as on Plasma Mobile; Breeze's otherwise, Q1.85)` | rule |
| K-KA-40† | `list.disclosure.transitions (Glyph; = token.motion.durations.short, its timing); list.values.indent_per_level, list.values.indent_includes_disclosure @ variant @ka_tree` | rule |
| K-KA-41† | `window.values.default_size, window.content.min_width, window.content.min_height @ variant @ka_settings` | rule |
| K-KA-41† | `sidebar.values.width, sidebar.pane.bindings (Surface(View)), sidebar.header.min_height (the global toolbar's height) @ context window variant @ka_settings` | rule |
| K-KA-41† | `search_field @ context sidebar.header context window variant @ka_settings` | rule |
| K-KA-41† | `list @ variants {sidebar,@ka_rounded} context window variant @ka_settings ({checked} drawn as {selected})` | rule |
| K-KA-41† | `button @ variant back context sidebar.header context window variant @ka_settings` | rule |
| K-KA-41† | `layout.values.column_width @ variant @column_view context window variant @ka_settings` | rule |
| K-KA-41† | `search_field @ context form variant @ka_form_card cond mobile_form_factor` (the mobile layer's search field row) | rule |
| K-KA-41† | `form @ variant @ka_form_card cond mobile_form_factor: form.row_icon.icon.size (= token.icons.sizes.keyed.medium.size), form.values.icon_text_gap, form.row_arrow.icon.size (= floor(add(mul(token.icons.sizes.small.size, <factor>), <half a pixel>)): the rounded product), form.row_separator` | rule |
| K-KA-42† | `dialog @ variants {@search_dialog,@ka_dialog} (the K-KA-18† surface)` | rule |
| K-KA-42† | `list @ variant @ka_rounded context dialog variant @search_dialog; list.trailing_text.text.fill (= role.window.disabled.foreground) @ context dialog variant @search_dialog` | rule |
| K-KA-42† | `status_page @ context dialog variant @search_dialog ("No results found")` | rule |
| K-KA-43† | `shortcuts_dialog.values.natural_size @ variant @ka_quick` | rule |
| K-KA-43† | `search_field @ context shortcuts_dialog` | rule |
| K-KA-43† | `list @ variant @ka_rounded context shortcuts_dialog` | rule |
| K-KA-43† | `toolbar @ variant bottom context shortcuts_dialog (Save and Reset); toolbar.bar.padding @ variant bottom context shortcuts_dialog` | rule |
| K-KA-43† | `button.icon.glyph (ThemeIcon configure), button.label.text @ variant @key_sequence context shortcuts_dialog / variant @key_sequence {checked} context shortcuts_dialog / variant @key_sequence {@recording} context shortcuts_dialog` | rule |
| K-KA-43† | `button.body.min_width, button.body.min_height (square), button.icon.glyph (ThemeIcon by direction) @ variant clear_field context shortcuts_dialog / variant clear_field context shortcuts_dialog dir rtl` | rule |
| K-KA-43† | `button.icon.glyph (ThemeIcon dialog-cancel) @ variant clear_field context shortcuts_dialog prev button {@recording} (ADD-B07)` | rule |
| K-KA-43† | `dialog @ variant @ka_form_card_dialog context shortcuts_dialog (K-KA-17†)` | rule |
| K-KA-43† | `dialog @ variants {@ka_message_dialog,warning} context shortcuts_dialog (the conflict)` | rule |
| K-KA-44† | `about_dialog.body.padding (its margins), about_dialog.icon.icon.size (= token.icons.sizes.keyed.huge.size), about_dialog.values.icon_max_width_fraction, about_dialog.name.text, about_dialog.version.text, about_dialog.description.text (= token.text.keyed.heading_3_secondary) @ variant @ka_about` | rule |
| K-KA-44† | `badge @ variant success context about_dialog ("What's new")` | rule |
| K-KA-44† | `dialog @ variant @ka_dialog context about_dialog (the release history: the K-KA-18† surface, its natural_size, max height, heading, close button, separator); about_dialog.values.release_gap` | rule |
| K-KA-44† | `dialog.values.natural_size @ variants {@ka_dialog,about}` (the dialog layer the About page itself opens in, a grid-unit square: the about page's ancestor, so selected by the content variant `about` the connector sets on a dialog hosting an `about_dialog`, not by `context about_dialog`) | rule |
| K-KA-44† | `form @ variant @ka_info_card context about_dialog (the link rows, K-KA-15†, their theme icons)` | rule |
| K-KA-44† | `about_dialog.person_name.text, about_dialog.person_task.text (= token.text.keyed.smallest_readable; fill = surface.disabled.foreground)` | field |
| K-KA-44† | `avatar.body.min_width @ context about_dialog` | rule |
| K-KA-44† | `button @ variant icon_only context about_dialog` | rule |
| K-KA-44† | `sheet @ context about_dialog (the licence text and its margin)` | rule |
| K-KA-45† | `teaching_tip.target_frame.stroke, teaching_tip.target_frame.transitions (Geometry; = token.motion.durations.long, its timing) @ variant @ka_onboarding` | rule |
| K-KA-45† | `teaching_tip.backdrop.fill (Material blur over everything but the target: its blur radius) @ variant @ka_onboarding` | rule |
| K-KA-45† | `teaching_tip.body.bindings (Surface(Tooltip)), teaching_tip.body.corners, teaching_tip.body.stroke (= kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast), OnboardToolTip.qml:52), teaching_tip.body.padding, teaching_tip.body.min_height, teaching_tip.body.max_width, teaching_tip.body.fill (= surface.background) @ variant @ka_onboarding` (it never times out: `teaching_tip` has no timeout on any platform, so nothing to state) | rule |
| K-KA-45† | `teaching_tip.values.placement (KirigamiAddonsOnboardingTip: above the control, below without room; its gap; ADD-A21)` | placement |
| K-KA-45† | `separator @ context teaching_tip` (its line Q1.95's Normal stroke in the tip's Tooltip set, `OnboardingOverlay.qml:164`, `:187`) | rule |
| K-KA-45† | `button @ variants {tool,icon_only,previous} context teaching_tip / variants {tool,icon_only,next} context teaching_tip / variants {tool,icon_only,close} context teaching_tip` | rule |
| K-KA-46† | `list @ variant @ka_rounded (the sound list); checkbox.mark.glyph (ThemeIcon object-select-symbolic) @ variant selection {checked} context list.row[@ka_rounded] (the current sound's check)` | rule |
| K-KA-47† | `icon_theme_fallback (the preset's icon fields, S4.5: Breeze for an application started through KirigamiApp; ADD-A65)` | field |
| K-KA-47† | - (the org.kde.breeze Quick style KirigamiApp chooses on Android and iOS: there Kirigami is another toolkit on that platform, a candidate preset of its own) | preset (R1.3) |
