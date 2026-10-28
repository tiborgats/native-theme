# v0.7.1 — Model completeness: Specification, traceability (iOS)

**Status:** draft, 2026-10-28. Part of the specification
([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), S20): every gap row of
[`todo_v0.7.1_model-completeness-gap-map-ios.md`](todo_v0.7.1_model-completeness-gap-map-ios.md) mapped to
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
`variant`/`{…}` belong to that ancestor, and ` / ` separates alternatives. A name with no unit suffix and the same name with `_px` or
`_ms` are one field (S1 spells the suffix). `ADD-Ann`, `ADD-Bnn` and `CONTRA-n` cite the additions to the core and
the contradictions with the rationale, each decided in the core's additions register (S22). A presentation event
is `Appear`, `Disappear` or `Move` (typed) or `@event` (keyed, declared by the widget's entry).

**Kinds** (S20): `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`,
`system-control`, `condition`, `flag`; the verdicts `out (R1.1)` … `out (R1.4)`, `preset (R1.3)`, `newest (R1.5)`, `reference (R1.6)`,
`readers (R18)` with target `-`.

| Row | Target | Kind |
|---|---|---|
| D-1 | `token.text.keyed.large_title.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.title_1.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.title_2.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.title_3.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.headline.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.subheadline.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.body.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.callout.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.footnote.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.caption_1.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-1 | `token.text.keyed.caption_2.letter_spacing = sf_tracking(font_size, sf_pro)` (ADD-A54) | token |
| D-2 | `token.text.keyed.large_title.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.title_1.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.title_2.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.title_3.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.headline.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.subheadline.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.body.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.callout.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.footnote.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.caption_1.font.axes (opsz FollowsSize)` | token |
| D-2 | `token.text.keyed.caption_2.font.axes (opsz FollowsSize)` | token |
| D-3 | `token.text.keyed.*.font.width and font.family (system design)` | token |
| D-3 | - | readers (R18) |
| D-4 | `token.text.keyed.large_title.line_height` | token |
| D-4 | `token.text.keyed.title_1.line_height` | token |
| D-4 | `token.text.keyed.title_2.line_height` | token |
| D-4 | `token.text.keyed.title_3.line_height` | token |
| D-4 | `token.text.keyed.headline.line_height` | token |
| D-4 | `token.text.keyed.subheadline.line_height` | token |
| D-4 | `token.text.keyed.body.line_height` | token |
| D-4 | `token.text.keyed.callout.line_height` | token |
| D-4 | `token.text.keyed.footnote.line_height` | token |
| D-4 | `token.text.keyed.caption_1.line_height` | token |
| D-4 | `token.text.keyed.caption_2.line_height` | token |
| D-4 | - | readers (R18) |
| D-5 | `token.text.roles.*` (the values at the default Dynamic Type category) | token |
| D-5 | - | readers (R18) |
| D-6 | `token.text.keyed.large_title_tight` | token |
| D-6 | `token.text.keyed.large_title_loose` | token |
| D-6 | `token.text.keyed.title_1_tight` | token |
| D-6 | `token.text.keyed.title_1_loose` | token |
| D-6 | `token.text.keyed.title_2_tight` | token |
| D-6 | `token.text.keyed.title_2_loose` | token |
| D-6 | `token.text.keyed.title_3_tight` | token |
| D-6 | `token.text.keyed.title_3_loose` | token |
| D-6 | `token.text.keyed.headline_tight` | token |
| D-6 | `token.text.keyed.headline_loose` | token |
| D-6 | `token.text.keyed.subheadline_tight` | token |
| D-6 | `token.text.keyed.subheadline_loose` | token |
| D-6 | `token.text.keyed.body_tight` | token |
| D-6 | `token.text.keyed.body_loose` | token |
| D-6 | `token.text.keyed.callout_tight` | token |
| D-6 | `token.text.keyed.callout_loose` | token |
| D-6 | `token.text.keyed.footnote_tight` | token |
| D-6 | `token.text.keyed.footnote_loose` | token |
| D-6 | `token.text.keyed.caption_1_tight` | token |
| D-6 | `token.text.keyed.caption_1_loose` | token |
| D-6 | `token.text.keyed.caption_2_tight` | token |
| D-6 | `token.text.keyed.caption_2_loose` | token |
| D-6 | - | readers (R18) |
| D-7 | `token.text.keyed.large_title_emphasized` | token |
| D-7 | `token.text.keyed.title_1_emphasized` | token |
| D-7 | `token.text.keyed.title_2_emphasized` | token |
| D-7 | `token.text.keyed.title_3_emphasized` | token |
| D-7 | `token.text.keyed.headline_emphasized` | token |
| D-7 | `token.text.keyed.subheadline_emphasized` | token |
| D-7 | `token.text.keyed.body_emphasized` | token |
| D-7 | `token.text.keyed.callout_emphasized` | token |
| D-7 | `token.text.keyed.footnote_emphasized` | token |
| D-7 | `token.text.keyed.caption_1_emphasized` | token |
| D-7 | `token.text.keyed.caption_2_emphasized` | token |
| D-8 | `token.text.keyed.body.font.features (monospaced digits)` | token |
| D-8 | `token.text.roles.numeric` | token |
| D-8 | - | readers (R18) |
| D-9 | `role.@label_tertiary` | role |
| D-9 | `role.@label_quaternary` | role |
| D-9 | `role.window.disabled.foreground` | role |
| D-9 | - | readers (R18) |
| D-10 | `role.@system_fill` | role |
| D-10 | `role.@system_fill_secondary` | role |
| D-10 | `role.@system_fill_tertiary` | role |
| D-10 | `role.@system_fill_quaternary` | role |
| D-10 | - | readers (R18) |
| D-11 | `role.@system_background_tertiary` | role |
| D-11 | `role.@system_grouped_background` | role |
| D-11 | `role.@system_grouped_background_secondary` | role |
| D-11 | `role.@system_grouped_background_tertiary` | role |
| D-11 | - | readers (R18) |
| D-12 | `role.window.border` | role |
| D-12 | `role.@separator_opaque` | role |
| D-12 | - | readers (R18) |
| D-13 | `role.@system_yellow` | role |
| D-13 | `role.@system_mint` | role |
| D-13 | `role.@system_teal` | role |
| D-13 | `role.@system_cyan` | role |
| D-13 | `role.@system_indigo` | role |
| D-13 | `role.@system_purple` | role |
| D-13 | `role.@system_pink` | role |
| D-13 | `role.@system_brown` | role |
| D-13 | `role.@system_gray` | role |
| D-13 | `role.@system_gray2` | role |
| D-13 | `role.@system_gray3` | role |
| D-13 | `role.@system_gray4` | role |
| D-13 | `role.@system_gray5` | role |
| D-13 | `role.@system_gray6` | role |
| D-13 | - | readers (R18) |
| D-14 | `role.@system_background_elevated, role.@system_background_secondary_elevated, role.@system_background_tertiary_elevated, role.@system_grouped_background_elevated, role.@system_grouped_background_secondary_elevated, role.@system_grouped_background_tertiary_elevated` (the elevated level of each background) | role |
| D-14 | `dialog.body.bindings, sheet.body.bindings, popover.body.bindings` (`Binding::Surface` of the `Dialog` or `Popover` set and a `Binding::Role` from each keyed background role to its elevated twin, `role.@system_background` to `= role.@system_background_elevated` and so on: the elevated level in an alert, a sheet or a popover; R5.2) | field |
| D-14 | `role.dialog.background, role.popover.background` (the sets' backgrounds, stated as the elevated values) | role |
| D-14 | `role.<set>.background, role.@system_background, role.@system_background_secondary, role.@system_background_tertiary, role.@system_grouped_background, role.@system_grouped_background_secondary, role.@system_grouped_background_tertiary @ cond elevated` (the whole window elevated: multitasking) | rule |
| D-14 | - | readers (R18) |
| D-15 | `colors rules @ cond contrast_increased (every dynamic colour)` | rule |
| D-15 | - | readers (R18) |
| D-16 | `cond tint_dimmed` (ADD-A10) | condition |
| D-16 | `role.global.accent @ cond tint_dimmed` | rule |
| D-16 | - | readers (R18) |
| D-17 | `Color` (S2.1, extended sRGB) | field |
| D-17 | - | readers (R18) |
| D-18 | `token.focus (corners.curve, offset)` | token |
| D-18 | - | readers (R18) |
| D-19 | `token.icons.sizes.keyed.*.weight` | token |
| D-19 | `token.icons.sizes.keyed.*.scale` | token |
| D-19 | - | readers (R18) |
| D-20 | `token.params.opacity.disabled` (each widget's `{disabled}` rules carry the per-control rendering: state colours or this alpha, per control) | token |
| D-20 | `role.window.disabled.foreground` | role |
| D-20 | `role.window.disabled.background` | role |
| TS-1 | - | readers (R18) |
| TS-1 | `token.text.roles.title2` | token |
| TS-1 | `token.text.roles.title3` | token |
| TS-1 | `token.text.roles.body` | token |
| TS-1 | `token.text.roles.callout` | token |
| TS-1 | `token.text.roles.subheadline` | token |
| TS-1 | `token.text.roles.footnote` | token |
| TS-1 | `token.text.roles.caption2` | token |
| TS-2 | `token.text.roles.*` (the values at the default Dynamic Type category) | token |
| TS-2 | - | readers (R18) |
| TS-3 | `token.text.keyed.*.font.weight @ cond bold_text` | rule |
| TS-3 | - | readers (R18) |
| TS-4 | `token.text.keyed.extra_large_title` | token |
| TS-4 | `token.text.keyed.extra_large_title_2` | token |
| W-1 | - (the window controls' glyphs, sizes, colours and states, which the system draws) | out (R1.1) |
| W-1 | `window.values.windowing_control_style` (the application's `UIWindowScene.WindowingControlStyle`: automatic, minimal, unified, a value it configures for the system to draw with, R1.1) | field |
| W-2 | `window.frame.corners (the radius the content is clipped to)` | field |
| W-2 | - (the window shadow, the resize grabber and the inactive window's look, which the system draws) | out (R1.1) |
| W-3 | - | out (R1.1) |
| W-4 | `cond window_inactive (the scene's activeAppearance)` | condition |
| W-4 | `role.* @ cond window_inactive (selection, tint, bars)` | rule |
| W-4 | - | readers (R18) |
| W-5 | `window.values.status_bar_style` | field |
| W-5 | `window.values.status_bar_style @ cond scheme:dark` | rule |
| W-6 | `window.frame.corners` (`Radius::Concentric` per corner, each with its stated minimum, following the display, the root's container: `RootInputs.display_corner_radius` where the platform reports it, else `token.params.display.corner_radius`, else none; S4.6, S8, R25.11) | field |
| W-6 | `token.params.display.corner_radius @ cond device_model:<key>` (measured per device model, the map's M-r; an unlisted model's radius `Absent`, S4.6, R25.11, ADD-A83) | rule |
| W-7 | - | out (R1.1) |
| W-9 | - | out (R1.1) |
| W-10† | - | out (R1.1) |
| W-11† | `window.frame.corners @ cond full_screen / cond tiled:<edges>` (the radius the content is clipped to per state) | rule |
| W-11† | - (the shadow per state, the window controls and the open, close, minimize and arrange motion, which the system draws, as W-2) | out (R1.1) |
| B-1 | `button.body.fill, button.label.text.fill, button.icon.icon.fill @ variant flat / variant @gray / variant tinted / variant primary / variant @borderless / variant @bordered / variants {@bordered,tinted} / variants {@bordered,primary}` (plain, gray, tinted, filled, borderless, bordered, borderedTinted, borderedProminent: fill and foreground) | rule |
| B-1 | `button.body.fill, button.label.text.fill, button.icon.icon.fill @ {pressed} variant flat / {pressed} variant @gray / {pressed} variant tinted / {pressed} variant primary / {pressed} variant @borderless / {pressed} variant @bordered / {pressed} variants {@bordered,tinted} / {pressed} variants {@bordered,primary}` (each style's pressed look) | rule |
| B-2 | `button.body.fill @ variant glass / variants {glass,clear} / variants {glass,primary} / variants {glass,clear,primary}` (a `Material` layer; the prominent styles' tint is the material's `tint`) | rule |
| B-2 | `button.body.{transform,fill} @ {pressed} variant glass` (the interactive press: scale and highlight) | rule |
| B-2 | `button.body.transitions @ variant glass` (the press response's motion, groups `Transform`, `Fill`) | rule |
| B-3 | `button.label.text, button.body.padding, button.body.corners, button.body.min_height @ size mini / small / regular (UIKit medium) / large` | rule |
| B-3 | `[declare.widget.button] sizes for ios-* and its default size` | field |
| B-4 | `button.body.corners @ variant @corner_fixed / variant @corner_small / variant @corner_medium / variant @corner_large / variant pill` (capsule is `pill`) | rule |
| B-4 | `button.body.corners @ variant @corner_dynamic size mini / variant @corner_dynamic size small / variant @corner_dynamic size regular / variant @corner_dynamic size large` (each radius `= uifm_scaled(<radius>, <text role>)`, following Dynamic Type; ADD-A55) | rule |
| B-4 | `reader:ios:the Dynamic Type category the dynamic radius follows` | readers (R18) |
| B-5 | `button.body.opacity @ {pressed} variant flat` (13–18's plain button lowers its label's and image's alpha; it draws no bezel, so the group opacity is theirs) | rule |
| B-5 | `button.body.transitions @ variant flat` (the alpha's fade, group `Opacity`) | rule |
| B-6 | `button.body.fill @ {pressed} variant primary / tinted / @gray` | rule |
| B-6 | `button.body.transitions (fade-out: transition of the style entered)` | field |
| B-7 | `button.body.fill, button.label.text.fill @ {checked,pressed} variant toggle` | rule |
| B-8 | `button.subtitle.text` (font, fill) | field |
| B-8 | `button.values.subtitle_gap` | field |
| B-9 | `button.values.icon_placement` | field |
| B-9 | `button.icon.icon (size, weight, scale, rendering mode)` | field |
| B-10 | `button flag @loading` (`ios-*`) | flag |
| B-10 | `spinner.{spoke.fill,values.loop} @ context button {@loading}` (the activity indicator in the button: a `spinner` element, Nested controls; its own colour) | rule |
| B-10 | `button.icon @ {@loading}` (`absent = true`: the image swapped out for the indicator; `icon` may be absent on `ios-*`) | rule |
| B-11 | `button.label.text.fill @ variant destructive` | rule |
| B-11 | `button.label.text.font.weight @ variant @cancel context dialog.action_area` (its weight in alerts and action sheets) | rule |
| B-11 | `button.label.text @ variant @cancel` (the cancel role; iOS 26's further roles are an open item of the catalogue, not declared) | rule |
| B-12 | `button.icon.glyph, button.body.fill, button.body.min_width, button.body.min_height @ variant close / variant @info / variant @contact_add` | rule |
| B-12 | `button.body.fill, button.icon.glyph.fill @ {pressed} variant close / {pressed} variant @info / {pressed} variant @contact_add` | rule |
| B-13 | `button.label.text.font.weight @ variant primary / variant tinted / variant @gray / variant flat` | rule |
| B-13 | `button.label.text.font.weight @ variant done context toolbar.bar / variant primary context toolbar.bar` (the done and prominent bar items' weight) | rule |
| B-14 | `button.body.fill, button.label.text.fill @ {disabled} variant flat / @gray / tinted / primary` | rule |
| B-15 | `button.body.fill, button.label.text.decoration @ cond button_shapes` | rule |
| B-15 | `cond button_shapes (S7.1)` | condition |
| B-15 | `reader:ios:the Button Shapes setting` | readers (R18) |
| B-16 | `button.icon.transitions` (the symbol content transition: a `TransitionKind::Keyed` transition of group `Icon`, R25.12, ADD-A27) | field |
| B-17 | `button.body.transform, button.body.fill, button.body.shadow @ {hover}` | rule |
| B-17 | `button.body.transitions` (the lift's motion, groups `Transform`, `Shadow`, `Fill`) | field |
| B-18† | `button.menu_indicator.glyph @ variant menu` (the automatic indicator) | rule |
| B-18† | `button.menu_indicator.glyph @ variants {menu,@indicator_popup}` | rule |
| B-18† | `button.menu_indicator @ variants {menu,@indicator_none}` (`absent = true`) | rule |
| B-18† | `button.values.menu_trigger @ variants {menu,@menu_on_hold}` (the touch-and-hold variant) | rule |
| B-18† | `button.values.menu_placement @ variant menu` (`UiKitMenu`; the menu's own look and motion are the MN-* rows) | placement |
| I-1 | `input.body.{stroke,corners,fill} @ variant line / @bezel / flat` | rule |
| I-2 | `button.icon.glyph, button.icon.icon.fill, button.body.min_width, button.body.min_height @ variant clear_field context input` (the clear button's glyph, colour and size) | rule |
| I-2 | `input.values.trailing_inset` (its inset from the field's trailing edge) | field |
| I-2 | `input.values.clear_button_visibility` | field |
| I-3 | `input.icon.{icon,padding} @ variant leading / variant trailing` (the overlay views) | rule |
| I-3 | `input.values.leading_inset; input.values.trailing_inset` | field |
| I-3 | - | readers (R18) |
| I-4 | `input.preedit.{fill,text.decoration}` | field |
| I-5 | `input.misspelling.text.decoration` | field |
| I-5 | `input.correction.{fill,text,corners}` (the autocorrection mark and bubble) | field |
| I-6 | `input.prediction.text` | field |
| I-7 | `input.values.min_font_size` | field |
| I-9† | `input.values.mask_character; input.values.mask_reveal_ms` | field |
| I-9† | `input.body.fill @ {autofilled} variant secure` (the strong-password field's fill) | rule |
| I-9† | `input.autofill_affordance.{icon,fill,corners,padding,text} @ variant secure` (the suggestion and its label) | rule |
| I-10† | `input.{body.fill,text.text.fill,placeholder.text.fill} @ {disabled}` | rule |
| I-10† | `input.body.{fill,stroke,corners} @ {focused}` (whether the frame changes while editing) | rule |
| I-10† | `input.body.min_height = uifm_scaled(<px>, body)` (the height per Dynamic Type size, following `text_scale`; ADD-A55) | field |
| I-10† | - | readers (R18) |
| TA-1 | `text_area.body.{stroke,corners} @ variant @rounded_rect` | rule |
| TA-2 | `text_area.find_match.{fill,corners}` | field |
| TA-2 | `text_area.find_match.{fill,transform} @ {current}` (the current match) | rule |
| TA-2 | `text_area.values.presentations event @find_indicator` (the current match's bounce, a step on part `find_match`) | field |
| TA-2 | `role.global.find_highlight` | role |
| TA-3 | `text_area.highlight.{fill,text} @ variant @<style>` (the keyed iOS 18 highlight styles) | rule |
| TA-4 | `text_area.writing_tools_glow.{fill,shadow,transitions}` | field |
| TA-5 | `link.text.text @ variant inline context text_area` | rule |
| TA-5 | - | readers (R18) |
| CK-1 | `checkbox.box.corners (Full), checkbox.mark.glyph (NamedSystemImage) @ variant selection` | rule |
| CK-1 | `list.accessory.glyph @ variant @checkmark` (the list checkmark accessory) | rule |
| CK-2† | `checkbox.box.stroke, checkbox.box.min_width, checkbox.box.min_height @ variant selection` | rule |
| CK-2† | `checkbox.box.fill, checkbox.mark.glyph @ {checked} variant selection` | rule |
| CK-2† | `checkbox.box.*, checkbox.mark.* @ {disabled} variant selection` | rule |
| CK-2† | `checkbox.box.transitions, checkbox.mark.transitions @ variant selection` (the fill transition when selected) | rule |
| CK-2† | `checkbox.values.presentations event Appear @ variant selection` (the column sliding in with edit mode) | rule |
| CK-2† | `checkbox.box.{min_width,min_height} @ variant selection` (`= uifm_scaled(<size>, body)`: the circle's size per Dynamic Type; ADD-A55) | rule |
| CK-2† | `reader:ios:the Dynamic Type category the circle's size follows` | readers (R18) |
| MN-1 | `menu.body.fill (material)` | field |
| MN-1 | `menu.body.fill @ cond reduced_transparency` | rule |
| MN-2 | `menu.body.shadow` | field |
| MN-3 | `menu.body.{min_width,max_width,max_height}` | field |
| MN-3 | `menu.values.scrollable` | field |
| MN-4 | `menu.item_subtitle.text` | field |
| MN-5 | `menu.item_trailing_icon.icon` | field |
| MN-6 | `menu.check_indicator.glyph @ {checked}` | rule |
| MN-6 | `menu.check_indicator.glyph @ {indeterminate}` | rule |
| MN-6 | `menu.values.check_column` | field |
| MN-7 | `menu.{item_label.text.fill,item_trailing_icon.icon.fill} @ variant destructive` | rule |
| MN-8 | `menu.item.fill @ {pressed} (touch highlight)` | rule |
| MN-8 | `menu.item.fill @ {hover} (pointer)` | rule |
| MN-9 | `menu.separator.{stroke,min_height}` | field |
| MN-9 | `menu.group_separator.{fill,min_height}` | field |
| MN-10 | `menu.section_header.{text,padding}` | field |
| MN-11 | `menu.submenu_arrow.glyph` | field |
| MN-11 | `menu.values.submenu_presentation (Inline)` | field |
| MN-11 | `menu.values.presentations event @submenu_expand` | field |
| MN-12 | `menu.item.{min_width,min_height} @ size small / size regular` | rule |
| MN-12 | `menu.item_icon.icon.size @ size small / size regular` | rule |
| MN-12 | `menu.item_label.text @ size small / size regular` (the label under the icon) | rule |
| MN-12 | `menu.item.{fill,corners,min_width,min_height} @ variant palette` | rule |
| MN-12 | `menu.values.palette_columns @ size small / size regular` | rule |
| MN-13 | `menu.accelerator.text` | field |
| MN-14 | `menu.values.presentations events Appear, Disappear` | field |
| MN-15 | `menu.item.min_height (= a derivation from the label's line height, widget.item_label.line_height: the item has no font of its own, S2.2)` | field |
| MN-15 | - | readers (R18) |
| MN-16 | `menu.* @ variant @compact` | rule |
| MN-17† | `menu.item.{corners,padding}` | field |
| MN-17† | `menu.body.padding` | field |
| MN-17† | `menu.values.icon_text_gap_px` | field |
| MN-18† | `menu.{item_label.text.fill,item_subtitle.text.fill,item_trailing_icon.icon.fill} @ {disabled}` | rule |
| SB-1 | `scrollbar.thumb.fill @ variant dark / light` | rule |
| SB-1 | `scrollbar.thumb.fill @ {over_dark_content}` | rule |
| SB-2 | `scrollbar.body.padding` | field |
| SB-2 | - | readers (R18) |
| SB-3 | `scrollbar.values.auto_hide_delay` | field |
| SB-3 | `scrollbar.values.presentations (Disappear: fade; keyed event @flash)` | field |
| SB-4 | `scrollbar.thumb.{min_width,min_height} @ {pressed}` | rule |
| SB-4 | `scrollbar.thumb.transitions` | field |
| SB-5 | `scrollbar.thumb.{corners,stroke}` | field |
| SB-6 | `scroll_view.edge_effect.fill @ variant top {scrolled} / variant bottom {scrolled}` (the edge effect under the top or bottom bar, each edge its own element; the indicators draw over it) | rule |
| SB-7† | `scrollbar.thumb.{min_width,min_height}` | field |
| SB-7† | `scrollbar.values.overscroll_shrink` | field |
| SB-7† | `scrollbar.thumb.{fill,min_width} @ {hover}` (iPad pointer hover, to verify) | rule |
| SL-1 | `slider.thumb.{shadow,stroke}` | field |
| SL-2 | `slider.thumb.{min_width,min_height,corners}` | field |
| SL-3 | `slider.thumb.{fill,transform} @ {pressed}` | rule |
| SL-3 | `slider.thumb.transitions` | field |
| SL-4 | `slider.track.corners; slider.fill.corners` | field |
| SL-5 | `slider.minimum_image.icon; slider.maximum_image.icon` | field |
| SL-5 | `slider.values.image_gap` | field |
| SL-5 | - | readers (R18) |
| SL-6 | `slider.tick.{fill,min_width,min_height,corners}` | field |
| SL-6 | `slider.values.tick_gap` | field |
| SL-7 | `slider.disabled_range.fill` | field |
| SL-8 | `slider.thumb @ variant @thumbless` (`absent = true`: no thumb is drawn) | rule |
| SL-8 | `slider.{track,fill}.{min_height,fill} @ variant @thumbless` | rule |
| SL-9 | `slider.fill.transitions, slider.thumb.transitions` (`PropertyGroup::Geometry`: the animated value change) | field |
| SL-9 | `slider.values.snap_to_ticks` | field |
| PB-1 | `progress_bar.track.{fill,min_height,corners}, progress_bar.indicator.fill @ variant @bar` | rule |
| PB-2 | `progress_bar.indicator.corners` | field |
| PB-3 | `progress_bar.indicator.transitions` (`PropertyGroup::Geometry`: the animated progress change) | field |
| PB-4† | `progress_bar.label.text; progress_bar.value_label.text` | field |
| PB-4† | `progress_bar.values.label_position; progress_bar.values.label_gap` | field |
| T-1 | `bottom_tab_bar.bar.fill (material)` | field |
| T-1 | `bottom_tab_bar.bar.fill @ {scrolled} / !{scrolled}` (the standard and scroll-edge appearances) | rule |
| T-1 | `reader:ios:UITabBarAppearance backgroundEffect` | readers (R18) |
| T-2 | `bottom_tab_bar.bar.stroke.top (Edge.snap ToDevicePixel, ADD-A33)` | field |
| T-2 | `reader:ios:UIBarAppearance.shadowColor` | readers (R18) |
| T-3 | `bottom_tab_bar.item.padding, bottom_tab_bar.item_icon.icon, bottom_tab_bar.item_label.text @ variant stacked / variant inline / variant @compact_inline` (each layout's icon size, title font and spacing) | rule |
| T-3 | `bottom_tab_bar.values.item_gap, item_width, item_positioning` | field |
| T-3 | `reader:ios:UITabBarAppearance stacked and inline layout` | readers (R18) |
| T-4 | `bottom_tab_bar.item_icon.icon.fill, bottom_tab_bar.item_label.text.fill` | field |
| T-4 | `reader:ios:UITabBarItemAppearance.normal` | readers (R18) |
| T-5 | `bottom_tab_bar.item_icon.icon.symbol_variants @ {selected} (ADD-A37)` | rule |
| T-5 | `bottom_tab_bar.item_label.text.font.weight @ {selected}` | rule |
| T-6 | `bottom_tab_bar.item_icon.*, bottom_tab_bar.item_label.* @ {disabled} / {focus_visible}` | rule |
| T-6 | `reader:ios:UITabBarItemAppearance disabled, focused` | readers (R18) |
| T-7 | `badge.* @ context bottom_tab_bar.item` | rule |
| T-7 | `reader:ios:UITabBarItem.badgeColor, badgeTextAttributes` | readers (R18) |
| T-8 | `bottom_tab_bar.indicator (fill material, corners full)` | field |
| T-8 | `bottom_tab_bar.values.presentations event @indicator_move` (step `part: indicator`, kind `Morph`) | field |
| T-9 | `bottom_tab_bar.bar.{corners,shadow,min_height} @ variant floating` (corners `Full`) | rule |
| T-9 | `bottom_tab_bar.values.floating_inset` | field |
| T-10 | `bottom_tab_bar.bar.*, bottom_tab_bar.item.* @ {minimized}` | rule |
| T-10 | `bottom_tab_bar.values.presentations event @minimize` | field |
| T-10 | `reader:ios:UITabBarController.tabBarMinimizeBehavior` | readers (R18) |
| T-11 | `bottom_tab_bar.item.{fill,corners,min_height} @ variant @search` (the separate circular search tab at the trailing end) | rule |
| T-12 | `bottom_tab_bar.accessory.{fill,corners,padding} @ variant @accessory_inline / variant @accessory_expanded` | rule |
| T-12 | `reader:ios:UITabAccessory.Environment` | readers (R18) |
| T-13 | `bottom_tab_bar.bar.{fill,corners,shadow}, bottom_tab_bar.item.fill, bottom_tab_bar.indicator.fill @ variant @tab_sidebar` (iPadOS's floating top bar) | rule |
| T-13 | `button.icon.glyph @ variant pane_toggle context bottom_tab_bar.bar` (the sidebar toggle) | rule |
| T-13 | `reader:ios:UITabBarController.Mode` | readers (R18) |
| T-14 | `bottom_tab_bar.bar.min_height @ v_size_class compact / h_size_class compact / h_size_class regular` (ADD-A20) | rule |
| T-15 | `bottom_tab_bar.values.presentations events @indicator_move, Appear, Disappear` (the selection change, the items' step beside the indicator's; the bar shown and hidden by `setTabBarHidden(_:animated:)`) | field |
| T-16 | `bottom_tab_bar.item.{fill,corners}, bottom_tab_bar.item_icon.icon.fill @ variant @prominent` | rule |
| T-16 | `reader:ios:UITabBarController.prominentTabIdentifier` | readers (R18) |
| T-17† | `bottom_tab_bar.values.presentations event @customize` | field |
| T-17† | `bottom_tab_bar.drop_indicator.{fill,min_width,min_height}` | field |
| T-17† | `bottom_tab_bar.item.fill @ {dragged}; bottom_tab_bar.bar.fill @ {drop_target}` | rule |
| T-17† | `reader:ios:UITab allowsHiding, preferredPlacement` | readers (R18) |
| SD-1 | `sidebar.pane.fill (material)` | field |
| SD-1 | `reader:ios:UISplitViewController.BackgroundStyle` | readers (R18) |
| SD-2 | `list.row_background.corners, list.values.row_background_inset @ variant sidebar` | rule |
| SD-2 | `list.leading_icon.icon.fill @ {selected} variant sidebar` | rule |
| SD-3 | `list.row_background.fill, list.row.text.fill @ {selected} variant sidebar context list.view !{focus_within}` | rule |
| SD-4 | `list.section_header.text @ variant sidebar` | rule |
| SD-4 | `list.disclosure.glyph @ context list.section_header variant sidebar` (the trailing outline-disclosure chevron) | rule |
| SD-4 | `reader:ios:UIListContentConfiguration.sidebarHeader` | readers (R18) |
| SD-5 | `list.leading_icon.icon.{size,fill} @ variant sidebar / variants {sidebar,@accompanied_sidebar_cell}` | rule |
| SD-5 | `reader:ios:UIListContentConfiguration.sidebarCell, accompaniedSidebarCell` | readers (R18) |
| SD-6 | `sidebar.pane.{corners,shadow,padding} @ variant floating` (corners `Concentric`) | rule |
| SD-6 | `sidebar.pane.fill @ variant floating` (the glass material) | rule |
| SD-6 | `sidebar.values.floating_inset` | field |
| SD-6 | `background_extension.extension.fill @ context sidebar.pane variant floating` (the content extending beneath) | rule |
| SD-7 | `sidebar.values.presentations events Appear, Disappear, @tab_sidebar_morph` | field |
| SD-8 | `list.row.min_height @ variant sidebar` | rule |
| SD-9† | `sidebar.pane.shadow @ variant overlay / variant @displace` | rule |
| SD-9† | `sidebar.scrim.{fill,opacity} @ variant overlay / variant @displace` (the dim of the covered column) | rule |
| SD-9† | `sidebar.values.presentations events Appear, Disappear @ variant overlay / variant @displace` | rule |
| SD-9† | `reader:ios:UISplitViewController.SplitBehavior` | readers (R18) |
| SD-10† | `sidebar.values.width, sidebar.values.width_fraction; sidebar.pane.min_width, sidebar.pane.max_width` | field |
| SD-10† | `reader:ios:UISplitViewController primary column widths` | readers (R18) |
| TB-1 | `toolbar.bar.fill (material)` | field |
| TB-1 | `toolbar.bar.fill @ {scrolled} / !{scrolled}` (the standard and scroll-edge appearances) | rule |
| TB-1 | `reader:ios:UIToolbar.standardAppearance, scrollEdgeAppearance` | readers (R18) |
| TB-2 | `toolbar.bar.stroke.top (Edge.snap ToDevicePixel, ADD-A33)` | field |
| TB-2 | `reader:ios:UIBarAppearance.shadowColor` | readers (R18) |
| TB-3 | `toolbar.bar.min_height @ v_size_class compact (ADD-A20)` | rule |
| TB-4 | `button.label.text.font.weight @ variant done context toolbar.bar` (the done item's weight) | rule |
| TB-4 | `button.body.fill @ variant primary context toolbar.bar` (the prominent item's tinted glass fill) | rule |
| TB-4 | `reader:ios:UIBarButtonItem.Style, doneButtonAppearance, prominentButtonAppearance` | readers (R18) |
| TB-5 | `toolbar.item_group.{fill,corners,shadow,padding}` (fill a `Material`, corners `Full`) | field |
| TB-5 | `toolbar.values.group_gap` | field |
| TB-5 | `reader:ios:UIBarButtonItem.sharesBackground, hidesSharedBackground` | readers (R18) |
| TB-6 | `button.body.{opacity,fill} @ {pressed} context toolbar.bar` | rule |
| TB-6 | `button.body.transitions @ context toolbar.bar` | rule |
| TB-7 | `badge.* @ context toolbar.bar` | rule |
| TB-7 | `reader:ios:UIBarButtonItem.Badge` | readers (R18) |
| TB-8 | `toolbar.values.fixed_space_width, flexible_space_min_width` | field |
| TB-8 | `button.body.padding @ variant @padding_removed context toolbar.bar` | rule |
| L-1 | `list.view.fill, list.section.fill, list.row_background.fill @ variant plain / variant @grouped / variant inset / variant sidebar / variant @sidebar_plain` | rule |
| L-1 | `reader:ios:UIBackgroundConfiguration list cells resolvedBackgroundColor` | readers (R18) |
| L-2 | `list.section.corners @ variant inset` | rule |
| L-2 | `list.values.section_inset @ variant inset` | rule |
| L-3 | `list.values.row_separator_inset, list.values.row_separator_aligns_to_text, list.values.section_separators` | field |
| L-3 | `list.row_separator.stroke @ prev list.row {selected} next list.row {selected}` (the multiple-selection colour between selected rows) | rule |
| L-3 | `reader:ios:UIListSeparatorConfiguration` | readers (R18) |
| L-4 | `list.section_header.{text,fill} @ variant plain / variant @grouped / variant @prominent_header / variant @extra_prominent_header` | rule |
| L-4 | `list.values.section_headers_sticky @ variant plain` | rule |
| L-4 | `reader:ios:UIListContentConfiguration header text properties` | readers (R18) |
| L-4 | - (the uppercase header text of iOS 13–14) | newest (R1.5) |
| L-5 | `list.section_footer.text` | field |
| L-5 | `reader:ios:UIListContentConfiguration footer text properties` | readers (R18) |
| L-6 | `list.section_header.padding (top)` | field |
| L-6 | `reader:ios:UITableView.sectionHeaderTopPadding` | readers (R18) |
| L-7 | `list.secondary_text.text, list.trailing_text.text @ variant with_subtitle / variant @value_cell` | rule |
| L-7 | `list.values.secondary_text_gap, list.values.leading_gap; list.row.padding` | field |
| L-7 | `reader:ios:UIListContentConfiguration cell, subtitleCell, valueCell` | readers (R18) |
| L-8 | `list.leading_icon.{min_width,min_height,icon,corners}` (the reserved size, tint, corner) | field |
| L-8 | `reader:ios:imageProperties` | readers (R18) |
| L-9 | `list.row.min_height (= uifm_scaled(<minimum>, <text role>), ADD-A55; the row grows with its content past it)` | field |
| L-10 | `list.row_background.fill @ {pressed} / {selected} / {focus_visible} / {selected,focus_visible} / {selected,editing} / {editing} / {swiped} / {open} / {dragged} / {drop_target}` (highlighted, selected, focused, selected and focused, selected in multi-select, editing, swiped, expanded, lifted, drop target) | rule |
| L-10 | `list.row.text.fill, list.leading_icon.icon.fill @ {selected} / {focus_visible} / {selected,focus_visible}` (the content's colours on the fill) | rule |
| L-10 | `list.row.transform, list.row_background.shadow @ {dragged}` (the lifted row) | rule |
| L-10 | `reader:ios:UIBackgroundConfiguration.updated(for:)` | readers (R18) |
| L-11 | `list.row_background.transitions (fill, on deselection)` | field |
| L-12 | `list.accessory.{glyph,fill} @ variant @disclosure_indicator / variant @checkmark / variant @detail / variant @delete / variant @insert / variant reorder / variant @label / variant @popup_menu` (glyph with its size, colour; the outline disclosure is `list.disclosure`, EX-1–EX-3) | rule |
| L-12 | `list.values.accessory_placement` (each accessory's side) | field |
| L-12 | `checkbox.* @ variant selection context list.row` (the multiselect circle, CK-1) | rule |
| L-12 | `reader:ios:UICellAccessory options` | readers (R18) |
| L-13 | `list.swipe_action.{fill,text,icon} @ variant destructive / !variant destructive` | rule |
| L-13 | `list.swipe_area.fill; list.values.swipe_full_expansion` | field |
| L-13 | `list.values.presentations event @swipe` (the actions' reveal and the full-swipe expansion) | field |
| L-13 | `reader:ios:UIContextualAction` | readers (R18) |
| L-14 | `list.values.presentations event @edit_mode` (the indentation and the edit controls' appearance); `list.accessory.{transform,transitions} @ variant @delete` (the delete control's rotation) | rule |
| L-14 | `list.values.presentations event @drag_lift` (the reorder lift) | field |
| L-15 | `list.section_index.{text,fill}` | field |
| L-15 | `list.section_index.fill @ {pressed}` | rule |
| L-15 | `reader:ios:sectionIndexColor, sectionIndexBackgroundColor, sectionIndexTrackingBackgroundColor` | readers (R18) |
| L-16 | `list.values.presentations events Appear, Disappear (queried with the row: inserted, deleted), @reload (queried with the list)` | field |
| L-17 | `list.values.filler_rows, list.values.filler_row_height @ variant plain` | rule |
| L-17 | `reader:ios:fillerRowHeight` | readers (R18) |
| L-18 | `list.row_background.fill @ pos parity odd variant alternating_rows (no rule on ios-*; ADD-B06)` | rule |
| L-19 | `list.values.secondary_text_stacks (by text size)` | field |
| L-19 | `reader:ios:prefersSideBySideTextAndSecondaryText` | readers (R18) |
| L-21† | `list.values.indent_per_level, list.values.separators_follow_indent` | field |
| L-21† | `reader:ios:indentationWidth` | readers (R18) |
| L-22† | `list.header_section.text, list.sort_indicator.glyph, list.column_separator.stroke @ variant table` | rule |
| L-22† | `list.sort_indicator.glyph @ {sorted} variant table / {sorted,descending} variant table` | rule |
| L-22† | `list.row_background.fill @ {selected} variant table` (the row selection) | rule |
| L-22† | `list.header @ variant table h_size_class compact` (`absent = true`: the header hidden in a compact width; ADD-A20); `list.values.max_columns @ variant table h_size_class compact` (only the first column shown) | rule |
| L-23† | `list.trailing_text.text @ variant @grouped / variant inset` (the value in secondary text) | rule |
| L-23† | `combo_box.label.text, combo_box.arrow.glyph @ context list.row` (the menu-style picker: its value and indicator) | rule |
| L-23† | `list.accessory.glyph @ variant @disclosure_indicator / variant @checkmark` (the navigation-link picker's chevron, the inline picker's checkmark rows) | rule |
| L-23† | `switch.*, stepper.* @ context list.row` (the switch and stepper rows); `list.values.control_alignment @ variant @grouped / variant inset` (the control's alignment in the row) | rule |
| PO-1 | `popover.arrow.{min_width,min_height,corners}` | field |
| PO-1 | `popover.values.placement (UiKitPopover: arrow side)` | placement |
| PO-2 | `popover.body.fill (material)` | field |
| PO-3 | `popover.body.shadow` | field |
| PO-4 | `popover.values.placement (UiKitPopover edge_margin)` | placement |
| PO-4 | - | readers (R18) |
| PO-5 | `popover.values.presentations events Appear, Disappear` | field |
| PO-6 | `popover.body.bindings` (`Binding::Surface(Popover)` and the `Binding::Role` entries to the elevated backgrounds, D-14; R5.2) | field |
| PO-6 | `role.popover.background` (stated as the elevated value) | role |
| PO-6 | - | readers (R18) |
| PO-7† | `popover.values.default_size` | field |
| PO-7† | `popover.values.placement (UiKitPopover gap_px)` | placement |
| SP-1 | `splitter.divider.fill, splitter.divider.min_width` | field |
| SP-1 | `splitter.divider.{fill,stroke} @ variant sidebar` (each `Absent`: no divider beside a floating sidebar) | rule |
| SP-2 | `splitter.divider.* @ {hover} / {pressed}` | rule |
| SP-2 | `splitter.grip.{fill,glyph,min_width,min_height}` | field |
| SP-2 | `reader:ios:UISplitViewController column widths` | readers (R18) |
| SP-3 | `sidebar.values.presentations events Appear, Disappear` | field |
| SE-1 | `separator.line.stroke width (Edge.snap ToDevicePixel, ADD-A33)` | field |
| SE-1 | `reader:ios:UITraitCollection.displayScale` | readers (R18) |
| SE-2 | `separator.line.stroke @ variant @opaque` | rule |
| SE-2 | `separator.line.stroke @ context menu.body / context sidebar.pane / context toolbar.bar / context bottom_tab_bar.bar` (its paint's `BlendMode::Keyed` vibrancy over the material) | rule |
| SE-2 | `role.window.border, role.@separator_opaque` | role |
| SE-2 | `reader:ios:UIColor.separator, opaqueSeparator` | readers (R18) |
| SW-1 | `switch.thumb.shadow` | field |
| SW-2 | `switch.thumb.min_width @ {pressed}` | rule |
| SW-2 | `switch.thumb.fill, switch.thumb.transform @ {pressed} (glass lens, ios-26)` | rule |
| SW-2 | `switch.thumb.transitions` (the stretch's and the lens's motion) | field |
| SW-3 | `switch.thumb.min_width, switch.thumb.min_height, switch.thumb.corners` | field |
| SW-4 | `switch.{on_glyph,off_glyph}.{glyph,margin}` (shape, colour, size; their position on the track) | field |
| SW-4 | `switch.on_glyph.glyph, switch.off_glyph.glyph @ cond on_off_labels` | rule |
| SW-4 | `cond on_off_labels (ADD-A05)` | condition |
| SW-4 | `reader:ios:isOnOffSwitchLabelsEnabled` | readers (R18) |
| SW-5 | `switch.thumb.transitions` (group `Geometry`: the thumb's slide, a `Spring` timing) | field |
| SW-5 | `switch.track.transitions (Fill)` | field |
| SW-6 | `switch.track.stroke @ !{checked} cond contrast_increased` | rule |
| SW-7 | `switch.label.text` | field |
| DL-1 | `dialog.body.fill (material) @ variant alert` | rule |
| DL-2 | `dialog.scrim.fill` | field |
| DL-3 | `dialog.values.{title_alignment,message_alignment}` | field |
| DL-4 | `dialog.values.{title_message_gap_px,message_actions_gap_px}` | field |
| DL-4 | `dialog.body.padding @ variant alert` | rule |
| DL-5 | `dialog.separator.stroke @ variant alert` (13–18's hairlines between the cells) | rule |
| DL-5 | `dialog.action_area.stroke @ variant alert` (the hairline above them) | rule |
| DL-5 | `button.body.{fill,corners,min_width} @ context dialog.action_area variant alert` (full-width cells; 26's capsules) | rule |
| DL-5 | `button.body.fill @ {pressed} context dialog.action_area variant alert` | rule |
| DL-5 | `button.body.fill @ {default} context dialog.action_area variant alert` (26's prominent default, to verify) | rule |
| DL-5 | `dialog.values.action_layout.row_gap_px` | field |
| DL-6 | `dialog.values.action_layout.stack_rule (ByCount, long titles)` | field |
| DL-6 | `dialog.values.action_layout.stacked_order` | field |
| DL-7 | `button.label.text @ context dialog.action_area` (the other actions: regular, in tint) | rule |
| DL-7 | `button.label.text @ {default} context dialog.action_area` (the preferred action's weight) | rule |
| DL-7 | `button.label.text @ variant @cancel context dialog.action_area` | rule |
| DL-7 | `button.label.text @ variant destructive context dialog.action_area` | rule |
| DL-8 | `button.body.min_height @ context dialog.action_area variant alert` | rule |
| DL-9 | `input.* @ context dialog.body variant alert` | rule |
| DL-10 | `dialog.values.presentations event Appear (scale down with fade, ADD-B10)` | field |
| DL-11 | `dialog.values.presentations event Disappear` | field |
| DL-12 | `dialog.body.bindings @ variant alert` (`Binding::Surface(Dialog)` and the `Binding::Role` entries to the elevated backgrounds, D-14; R5.2) | rule |
| DL-12 | `role.dialog.background` (stated as the elevated value) | role |
| DL-12 | - | readers (R18) |
| DL-13† | `dialog.body.{min_width,max_width,max_height} @ variant alert cond idiom:phone / variant alert cond idiom:pad` | rule |
| DL-13† | `dialog.body.min_width @ variant alert cond accessibility_text_size (ADD-A08)` | rule |
| SP-1a | `spinner.spoke.fill` | field |
| SP-1a | `spinner.values.{spoke_count,spoke_inner_radius,spoke_length,spoke_width,spoke_opacity_ramp,cap}` | field |
| SP-2a | `spinner.values.loop (stepped timing)` | field |
| SP-3a | `spinner.body.{min_width,min_height} @ size regular / size large` | rule |
| SP-4a | `spinner.icon` (`absent = true` on `ios-*`: iOS draws no animated icon; S6.3) | field |
| SP-4a | `spinner.spoke.fill` | field |
| SP-5a | `spinner.values.presentations (Appear, Disappear)` | field |
| CB-1 | `combo_box.arrow.glyph` | field |
| CB-1 | `combo_box.arrow.glyph @ variant pull_down` | rule |
| CB-2 | `combo_box.{body.fill,body.stroke,body.corners,label.text.fill} @ variant flat / variant @gray / variant @borderless / variant @bordered / variant tinted / variant primary / variant glass / variant clear` (every B-1 style the trigger takes: the configurations, tinted and prominent, and iOS 26's glass styles) | rule |
| CB-3 | `combo_box.body.fill, combo_box.arrow.{glyph,transform} @ {open}` | rule |
| CB-3 | `menu.body.fill, menu.item.fill @ context combo_box` (the opened menu, MN-*) | rule |
| CB-3 | `menu.check_indicator.glyph @ {selected} context combo_box` (the checkmark on the current item: a combo list's chosen value is `selected`, S7.2) | rule |
| CB-4 | `combo_box.label.text.fill` | field |
| SG-1 | `segmented_control.indicator.{fill,stroke,corners,shadow}` | field |
| SG-1 | `segmented_control.track.padding (the indicator's inset)` | field |
| SG-2 | `segmented_control.label.text.font.weight @ {selected}` | rule |
| SG-3 | `segmented_control.separator.stroke` | field |
| SG-3 | `segmented_control.separator.opacity @ prev segmented_control.segment{selected} / next segmented_control.segment{selected}` (ADD-B07) | rule |
| SG-4 | `segmented_control.indicator.{fill,transform,transitions} @ {pressed}` (13–18's shrink and 26's glass lens, and their motion) | rule |
| SG-4 | `segmented_control.label.opacity, segmented_control.icon.opacity @ {pressed} !{selected}` (the pressed unselected segment dims) | rule |
| SG-5 | `segmented_control.values.presentations event @indicator_move` (step `part: indicator`) | field |
| SG-5 | `segmented_control.values.indicator_draggable` | field |
| SG-6 | `segmented_control.icon.icon.size` | field |
| SG-7 | `segmented_control.label.text.fill @ {selected}` | rule |
| SG-8† | `segmented_control.segment.fill @ {pressed} variant momentary` | rule |
| SG-8† | `segmented_control.segment.transitions @ variant momentary` | rule |
| SG-9† | `segmented_control.values.segment_width` | field |
| SG-9† | `segmented_control.segment.min_height` (`= uifm_scaled(<height>, body)`: the height per Dynamic Type; ADD-A55) | field |
| SG-9† | `segmented_control.segment.*, segmented_control.label.* @ {disabled}` | rule |
| SG-9† | `reader:ios:the Dynamic Type category the height follows` | readers (R18) |
| CD-1 | `list.row_background.stroke` (position `Outside`, with `Stroke.outset`; ADD-A77: the cell background configuration's stroke) | field |
| CD-1 | `list.values.row_background_inset` (the configuration's `backgroundInsets`) | field |
| CD-1 | `list.row_background.{fill,corners}` (its colour or visual effect as a material layer, its corner radius) | field |
| CD-1 | `list.row_background.shadow` (its `shadowProperties`, layers S2.5; `card` is unavailable on `ios-*`, and GroupBox is `group_box`) | field |
| CD-1 | `reader:ios:UIBackgroundConfiguration and UIShadowProperties` | readers (R18) |
| EX-1 | `list.disclosure.glyph` (iOS's outline disclosure is a list row's `disclosure`; `expander` is unavailable on `ios-*`) | field |
| EX-1 | `list.disclosure.transform @ {open}` | rule |
| EX-1 | `reader:ios:UICellAccessory.outlineDisclosure options` | readers (R18) |
| EX-2 | `list.disclosure.transitions (transform)` | field |
| EX-2 | `list.values.presentations events @expand, @collapse` (the child rows' insertion) | field |
| EX-3 | `list.disclosure.{glyph,fill} @ variant @header / variant @cell` (the outline style; under `.header` the toggling row is a `list.section_header` element, its look that part's) | rule |
| EX-3 | `reader:ios:UICellAccessory.OutlineDisclosureOptions.style` | readers (R18) |
| EX-4 | `list.disclosure.glyph.fill @ variant @header` | rule |
| LK-1 | `link.body.fill @ {pressed} variant inline` | rule |
| LK-1 | `link.body.corners @ variant inline` | rule |
| LY-1 | `layout.values.window_margin @ h_size_class compact / h_size_class regular` (ADD-A20) | rule |
| LY-1 | `reader:ios:systemMinimumLayoutMargins` | readers (R18) |
| LY-2 | `layout.values.max_content_width @ variant page` | rule |
| LY-2 | `reader:ios:readableContentGuide` | readers (R18) |
| LY-3 | `layout.values.widget_gap (= uifm_scaled, ADD-A55)` | field |
| LY-4 | `layout.values.* as uifm_scaled expressions (ADD-A55)` | field |
| LY-4 | `reader:ios:UIFontMetrics` | readers (R18) |
| C-1 | `toolbar.bar.min_height @ variant @large_title / v_size_class compact / variant with_search / variant with_prompt` | rule |
| C-1 | `toolbar.bar.fill @ {scrolled} / !{scrolled} / {scrolled} v_size_class compact / !{scrolled} v_size_class compact` (the standard, scroll-edge, compact and compact scroll-edge appearances, with the material) | rule |
| C-1 | `toolbar.bar.stroke` (the hairline) | field |
| C-1 | `toolbar.title.text, toolbar.large_title.text, toolbar.prompt.text` | field |
| C-1 | `toolbar.subtitle.text, toolbar.large_subtitle.text` (`subtitle`, `largeSubtitle`) | field |
| C-1 | `toolbar.title_menu_chevron.glyph` | field |
| C-1 | `button.icon.glyph @ variant overflow context toolbar.bar` (the overflow button) | rule |
| C-1 | `toolbar.large_title.{opacity,transform}, toolbar.large_subtitle.{opacity,transform}, toolbar.bar.min_height` as expressions on `element.scroll_offset` (the large-title collapse on scroll, ADD-B13) | field |
| C-1 | `toolbar.values.title_alignment` (27.2's `titleAlignment`, to verify) | field |
| C-1 | - | readers (R18) |
| C-2 | `button.icon.glyph @ variant back context toolbar.bar` | rule |
| C-2 | `button.label.text @ variant back context toolbar.bar` | rule |
| C-2 | `button.values.back_title_fallback @ variant back` | rule |
| C-2 | `button.values.back_display_mode @ variant back` | rule |
| C-2 | `button.body.fill @ {pressed} variant back context toolbar.bar` | rule |
| C-2 | `button.values.presentations @ variant back context toolbar.bar` (event `@title_change`, its step on `label`); `toolbar.values.presentations event @title_change` (the title's cross-fade) | rule |
| C-2 | - | readers (R18) |
| C-3 | `input.body.{fill,corners,min_height}, input.placeholder.text @ context search_field.body / context search_field.body variant @prominent / context search_field.body variant @minimal` (the field per `searchBarStyle`, the default style the base); `search_field.bar.fill / search_field.bar.fill @ variant @prominent` (the bar background those styles draw; none under `@minimal`) | rule |
| C-3 | `input.icon.icon @ variant leading context any search_field` (the magnifier glyph and colour) | rule |
| C-3 | `button.icon.glyph, button.icon.icon.fill @ variant clear_field context any search_field` (the clear button) | rule |
| C-3 | `button.label.text @ variant @cancel context search_field` (the cancel button) | rule |
| C-3 | `button.values.presentations (Appear, Disappear) @ variant @cancel context search_field` (the cancel button's appearance) | field |
| C-3 | `segmented_control.* @ context any search_field` (the scope bar) | rule |
| C-3 | `chip.body.{fill,corners}, chip.label.text @ context search_field` (the search tokens) | rule |
| C-3 | `button.icon.glyph @ variant icon_only context search_field` (the bookmark and results buttons) | rule |
| C-3 | `input.body.{fill,corners} @ context any search_field context toolbar.bar / context any search_field context bottom_tab_bar.bar` (the glass capsule) | rule |
| C-3 | `button.* @ variant icon_only context toolbar.bar; search_field.values.presentations event @expand` (26's search button expanding into a field above the keyboard) | rule |
| C-3 | - | readers (R18) |
| C-4 | `stepper.body.{fill,corners,min_width,min_height}, stepper.separator.stroke` (the two-segment body, its divider and size) | field |
| C-4 | `stepper.increment_glyph.glyph, stepper.decrement_glyph.glyph` (the plus and minus glyphs and colour) | field |
| C-4 | `stepper.increment.fill, stepper.decrement.fill @ {pressed} / {disabled}` (the pressed segment; the segment disabled at a limit) | rule |
| C-4 | `stepper.values.button_layout, stepper.values.repeat_delay_ms, stepper.values.repeat_interval_ms` | field |
| C-4 | - | readers (R18) |
| C-5 | `date_picker.field.{fill,corners,padding}, date_picker.field_label.text @ variant with_popup` (the date and time pills, their text in the accent) | rule |
| C-5 | `date_picker.field.fill, date_picker.field_label.text.fill @ {pressed} variant with_popup` | rule |
| C-5 | `date_picker.popup.{fill,corners,shadow}` (the popover it opens) | field |
| C-5 | `date_picker.values.placement (UiKitPopover)` | placement |
| C-5 | - | readers (R18) |
| C-6 | `calendar.header.padding, calendar.weekday_label.text, calendar.time_row.{text,padding}, calendar.decoration.{fill,corners,icon,min_width,min_height}` | field |
| C-6 | `button.icon.glyph @ variant previous context calendar.header / variant next context calendar.header` (the navigation chevrons) | rule |
| C-6 | `button.label.text, button.icon.glyph @ context calendar.header` (the month-year toggle) | rule |
| C-6 | `picker_wheel.* @ context calendar.frame variant @wheels` (the month-year wheels) | rule |
| C-6 | `calendar.day.{fill,corners}, calendar.day_label.text @ {today} / {selected} / {today,selected} / {disabled}` (`disabled` on a day: also one outside the available range) | rule |
| C-6 | `calendar.day_label.text.font, calendar.title.text.font` (`fontDesign`: the system design, D-3) | field |
| C-6 | - | readers (R18) |
| C-7 | `picker_wheel.item.text, picker_wheel.item.min_height, picker_wheel.values.visible_items, picker_wheel.values.curvature, picker_wheel.values.component_gap, picker_wheel.values.momentum` | field |
| C-7 | `picker_wheel.item.opacity` as an expression over `element.scroll_progress` and `element.item_index` (the off-centre rows' fading; ADD-B13) | field |
| C-7 | `picker_wheel.selection_band.{fill,corners,min_height,margin}` (the band and its insets) | field |
| C-7 | - | readers (R18) |
| C-8 | `color_button.swatch_ring.stroke, color_button.swatch.{min_width,min_height,corners}` (the multicolour ring, the inner swatch, their sizes) | field |
| C-8 | `color_button.body.fill, color_button.swatch_ring.stroke @ {pressed}` | rule |
| C-8 | - (the colour-picker sheet, a system view controller the application only presents) | out (R1.1) |
| C-8 | - | readers (R18) |
| C-9 | `page_indicator.dot.{fill,corners,min_width,min_height} / page_indicator.dot.{fill,min_width} @ {selected}` (the dots, the current one) | rule |
| C-9 | `page_indicator.values.dot_gap` | field |
| C-9 | `page_indicator.platter.{fill,corners} @ {@scrubbing} / variant @prominent` (the automatic style the base) (the platter while interacting; `@minimal` draws none) | rule |
| C-9 | `page_indicator.progress.fill` (17+'s timer fill of the current indicator, which elongates by the `{selected}` rule) | field |
| C-9 | `page_indicator.values.edge_dots_shrink; page_indicator.* @ variant vertical` | rule |
| C-9 | - | readers (R18) |
| C-10 | `spinner.spoke.fill @ context pull_to_refresh.indicator` (the tint) | rule |
| C-10 | `pull_to_refresh.values.progress_sweep` (`Spokes`: the spokes appear as the pull progresses) | field |
| C-10 | `spinner.spoke.fill @ variant determinate context pull_to_refresh.indicator {pulling}` (the pull-progress spinner, its `element.progress` the pull progress; ADD-B12) | rule |
| C-10 | `pull_to_refresh.values.trigger_distance, pull_to_refresh.title.text, pull_to_refresh.values.presentations event Disappear` | field |
| C-10 | - | readers (R18) |
| C-11 | `dialog.body.{fill,corners} @ variant @action_sheet` (the actions' platter, its material and radius) | rule |
| C-11 | `dialog.cancel_platter.{fill,corners} @ variant @action_sheet; dialog.values.cancel_platter_gap` | rule |
| C-11 | `dialog.separator.stroke @ variant @action_sheet` | rule |
| C-11 | `dialog.title.text, dialog.message.text @ variant @action_sheet` | rule |
| C-11 | `button.label.text @ context dialog.action_area variant @action_sheet` (the action font and tint) | rule |
| C-11 | `button.label.text @ variant @cancel context dialog.action_area variant @action_sheet / variant destructive context dialog.action_area variant @action_sheet` (the cancel action's weight, the destructive action's colour) | rule |
| C-11 | `dialog.values.window_margin @ variant @action_sheet` | rule |
| C-11 | `dialog.values.presentations event Appear @ variant @action_sheet` (kind `Slide` from the bottom) | rule |
| C-11 | `dialog.body.fill @ variant @action_sheet context any popover` (on iPad, shown in a popover) | rule |
| C-12 | `sheet.body.corners @ variant bottom` (`preferredCornerRadius`'s default) | rule |
| C-12 | `sheet.handle.{fill,corners,min_width,min_height,padding}` (the grabber: size, colour, offset) | field |
| C-12 | `sheet.values.detents` (medium and large heights) | field |
| C-12 | `sheet.scrim.opacity, sheet.values.undimmed_up_to` (the dim per detent) | field |
| C-12 | `sheet.values.presenter_transform, sheet.values.presenter_corners; sheet.presenter_backdrop.fill` (13–18's stacked-card effect and the black behind the presenter) | field |
| C-12 | `sheet.body.{max_width,max_height} @ variant @page / variant @form` (iPad page and form sheet sizes) | rule |
| C-12 | `sheet.body.corners, sheet.values.anchor @ variant @edge_attached v_size_class compact` (the edge-attached compact-height variant) | rule |
| C-12 | `sheet.values.swipe_to_close` (the interactive dismiss) | field |
| C-12 | `sheet.values.presentations events Appear, Disappear, @settle` (the presentation spring, the detent changes) | field |
| C-12 | `sheet.body.{fill,corners,shadow} @ variant floating` (`ios-26`'s inset glass sheet at partial detents; to verify) | rule |
| C-12 | - | readers (R18) |
| C-13 | `component.context_menu_preview.preview.transform` | component |
| C-13 | `component.context_menu_preview.preview.corners` | component |
| C-13 | `component.context_menu_preview.preview.shadow` | component |
| C-13 | `component.context_menu_preview.scrim.fill` | component |
| C-13 | `UiKitContextMenu` (`component.context_menu_preview.values.placement`) | placement |
| C-13 | `component.context_menu_preview.values.presentations event Appear` (the lift: a Scale step on part preview, a Fade step on part scrim) | component |
| C-13 | `component.context_menu_preview.values.presentations event Disappear` (the dismissal: the same parts' steps) | component |
| C-13 | - | readers (R18) |
| C-14 | `edit_menu.body.{fill,corners}, edit_menu.divider.stroke, edit_menu.arrow.fill / the same @ variant vertical` (the horizontal bar the base) | rule |
| C-14 | `button.label.text @ context edit_menu` (the item font) | rule |
| C-14 | `button.icon.glyph @ variant previous context edit_menu / variant next context edit_menu` (the paging chevrons) | rule |
| C-14 | `UiKitEditMenu` (`edit_menu.values.placement`) | placement |
| C-14 | - | readers (R18) |
| C-15 | `badge.body.{fill,min_width,min_height}, badge.label.text, badge.values.anchor_offset_x, badge.values.anchor_offset_y @ context bottom_tab_bar.item / context toolbar.bar` | rule |
| C-15 | `badge.label.text, badge.values.anchor_corner @ context list.row` (SwiftUI's list-row badge: trailing secondary text, `TrailingInline`) | rule |
| C-15 | `badge.* @ variant dot / variant count` (the indicator; the count or string) | rule |
| C-15 | - | readers (R18) |
| C-16 | `status_page.icon.icon, status_page.title.text, status_page.description.text, status_page.body.fill` (the image's dimensions and tint, the fonts and colours, the background) | field |
| C-16 | `status_page.values.icon_to_title, status_page.values.title_to_description, status_page.values.description_to_actions, status_page.values.alignment` | field |
| C-16 | `button.* @ context status_page.action_area` (its buttons) | rule |
| C-16 | `status_page.* @ variant loading / variant search` (the `.loading()` and `.search()` presets; under `loading` a `spinner` in `context status_page`) | rule |
| C-16 | - | readers (R18) |
| C-17 | `button.body.fill @ variant @paste_control` | rule |
| C-17 | `button.{label.text.fill,icon.icon.fill} @ variant @paste_control` (the base foreground on the label and on the glyph the icon display modes show) | rule |
| C-17 | `button.body.corners @ variant @paste_control` | rule |
| C-17 | `button.values.display_mode @ variant @paste_control` | rule |
| C-17 | - | readers (R18) |
| C-18 | `scroll_view.edge_effect.fill @ variant soft_edge` | rule |
| C-18 | `scroll_view.edge_effect.fill @ variant hard_edge` | rule |
| C-18 | `scroll_view.values.edge_effect_height; scroll_view.edge_effect @ variant start / variant end` (`absent = true`: the effect only under the top and bottom bars, each edge its own element) | rule |
| C-18 | - | readers (R18) |
| C-19 | `background_extension.extension.fill` | field |
| C-19 | `background_extension.values.mirrors_content` | field |
| C-19 | - | readers (R18) |
| C-20 | `glass_container.surface.fill` (the regular glass, the base) | field |
| C-20 | `glass_container.surface.fill @ variant clear` | rule |
| C-20 | `glass_container.surface.{transform,transitions} @ {pressed}` (the interactive glass) | rule |
| C-20 | `glass_container.values.merge_gap` | field |
| C-20 | - | readers (R18) |
| C-21 | `bottom_tab_bar.accessory.fill` | field |
| C-21 | `bottom_tab_bar.accessory.corners` | field |
| C-21 | `bottom_tab_bar.accessory.padding` | field |
| C-21 | `bottom_tab_bar.accessory.min_height` | field |
| C-21 | `bottom_tab_bar.accessory.fill @ variant @accessory_inline` | rule |
| C-21 | `bottom_tab_bar.values.presentations event @accessory_environment` | field |
| C-21 | - | readers (R18) |
| C-22 | `bottom_tab_bar.bar.{fill,corners} @ variant @tab_sidebar` (the bar's shape) | rule |
| C-22 | `button.icon.glyph @ variant pane_toggle context bottom_tab_bar.bar` (the toggle button) | rule |
| C-22 | `bottom_tab_bar.values.presentations event @tab_sidebar_morph` (the morph) | field |
| C-22 | - | readers (R18) |
| C-23 | `component.keyboard.values.appearance` | component |
| C-23 | `component.keyboard.input_accessory.fill` | component |
| C-23 | `component.keyboard.values.presentations event Appear` | component |
| C-23 | `component.keyboard.values.presentations event Disappear` | component |
| C-23 | - (the keyboard itself, which the system draws) | out (R1.1) |
| C-23 | - | readers (R18) |
| C-24 | `drag_image.lift_preview.transform` | field |
| C-24 | `drag_image.lift_preview.shadow` | field |
| C-24 | `drag_image.lift_preview.corners` | field |
| C-24 | `badge.body.fill, badge.label.text @ context drag_image` (the drag-count badge) | rule |
| C-24 | `role.global.drop_highlight` | role |
| C-24 | `drag_image.values.lift_delay_ms` | field |
| C-24 | `drag_image.values.presentations events @lift, @drop, @cancel` | field |
| C-24 | - | readers (R18) |
| C-25 | `find_bar.bar.{fill,padding}, find_bar.result_count.text, find_bar.values.bar_position` | field |
| C-25 | `button.icon.glyph @ variant previous context find_bar / variant next context find_bar` (the chevrons) | rule |
| C-25 | `button.icon.glyph @ variant menu context find_bar` (the options button; its menu is `menu @ context button variant menu`) | rule |
| C-25 | `input.* @ context any find_bar` (the fields) | rule |
| C-25 | `role.global.find_highlight` | role |
| C-25 | - | readers (R18) |
| C-26 | `component.large_content_viewer.{body.fill,body.corners,body.min_width,body.min_height,icon.icon,label.text}; component.large_content_viewer.values.presentations events Appear, Disappear` (in by R1.1: a `UIInteraction` on the application's view, drawn by UIKit in its process) | component |
| C-28 | `toolbar.title_menu_chevron.glyph, toolbar.document_header.{text,icon}` | field |
| C-28 | - | readers (R18) |
| C-29 | `button.icon.glyph @ variant pane_toggle context toolbar.bar` (a `NamedSystemImage`) | rule |
| C-29 | - | readers (R18) |
| C-30 | `level_indicator.track.fill, level_indicator.fill.fill, level_indicator.values.end_cap, level_indicator.values.thickness @ variant circular / variant @accessory / variant @capacity` (the linear gauge the base; the fill a gradient paint where drawn) | rule |
| C-31 | `spinner.indicator.fill, spinner.values.stroke_width @ variant determinate` (its geometry, to verify) | rule |
| C-32 | `group_box.body.fill, group_box.body.corners, group_box.body.padding, group_box.title.text` | field |
| C-33 | `text_area.writing_tools_glow` | field |
| C-33 | - (the Writing Tools panel) | out (R1.1) |
| C-36† | `teaching_tip.body.{fill,corners}, teaching_tip.image.icon, teaching_tip.title.text, teaching_tip.message.text @ variant inline / variant popover` | rule |
| C-36† | `teaching_tip.arrow.fill, teaching_tip.body.fill @ variant popover` (the popover tip's arrow and material) | rule |
| C-36† | `button.* @ context teaching_tip.action_area` (its buttons) | rule |
| C-36† | `UiKitPopover` (`teaching_tip.values.placement`) | placement |
| C-36† | - | readers (R18) |
| C-37† | `menu_bar.bar.fill (material), menu_bar.item_label.text` | field |
| C-37† | `menu_bar.values.presentations event Appear` (the hidden bar revealed at the top edge) | field |
| C-37† | `menu.item.{min_height,padding} @ context menu_bar cond idiom:pad` (iPad menu rows use more space) | rule |
| C-37† | `menu.values.submenu_open_delay_ms @ context menu_bar` (submenus on pointer hover, to verify) | rule |
| C-37† | - (the window controls in the bar, which the system draws, W-1) | out (R1.1) |
| C-37† | - | readers (R18) |
| C-38† | `menu.body.fill, menu.item.fill, menu.item_icon.icon, menu.item_label.text @ context autocomplete.body variant @menu / context autocomplete.body variant @list` | rule |
| C-38† | `menu.item.fill @ {focused} context autocomplete / {hover} context autocomplete` (the highlighted suggestion: `focused` or `hover`, never `selected`, S7.2) | rule |
| C-38† | `autocomplete.values.placement (UiKitMenu)` (the inline bar's menu) | placement |
| C-38† | `autocomplete.values.placement @ variant @list` (the stacked form, a list overlaying the results: its strategy once measured, readers' milestone) | rule |
| C-38† | - | readers (R18) |
| C-40† | `ink_toolbar.bar.{fill,corners,shadow} @ cond scheme:light / cond scheme:dark` | rule |
| C-40† | `button.{body.fill,icon.icon} @ {selected} variant tool context ink_toolbar` (the selected tool) | rule |
| C-40† | - | readers (R18) |
| C-41† | `sidebar.pane.{fill,stroke}, sidebar.values.width @ variant end` (the inspector column; the divider is the pane's inner edge) | rule |
| C-41† | `sidebar.pane.{fill,corners,shadow} @ variants {end,floating}` (26's floating glass) | rule |
| C-41† | `sidebar.values.presentations events Appear, Disappear @ variant end` | rule |
| C-41† | `sidebar.pane @ variant end h_size_class compact` (`drawn_as = { widget = "sheet" }`: in a compact width the inspector presents as a sheet; the sheet's variants and look are measured in the value phase) | rule |
| C-41† | - | readers (R18) |
| C-42† | `media_controls.bar.fill` (the material bar) | field |
| C-42† | `button.* @ context media_controls.bar` (its buttons, identities `play`, `volume`, `full_screen`, `frame_step`) | rule |
| C-42† | `slider.* @ context media_controls` (the scrubber) | rule |
| C-42† | `media_controls.values.presentations events Appear, Disappear` | field |
| C-42† | `button.icon.glyph @ variant route / button.icon.glyph.fill @ {route_active} variant route` (`tintColor`, `activeTintColor`) | rule |
| C-42† | `button.* @ variant route context media_controls.bar` (the route button in AVKit's controls) | rule |
| C-42† | - | readers (R18) |
| C-43† | `toolbar.* @ variant @navigator / @browser / @editor` | rule |
| C-43† | - | readers (R18) |
| C-44† | `component.document_launch_view.background.fill` | component |
| C-44† | `component.document_launch_view.title.text` | component |
| C-44† | `component.document_launch_view.subtitle.text` | component |
| C-44† | `button.* @ variant primary context component.document_launch_view / context component.document_launch_view` (the primary and secondary actions) | rule |
| C-44† | `component.document_launch_view.foreground_accessory.{icon,padding}` | component |
| C-44† | `component.document_launch_view.background_accessory.{icon,padding}` | component |
| C-44† | `component.document_launch_view.browser.{fill,corners}` (the browser it opens) | component |
| C-44† | - | readers (R18) |
| C-45† | `chart.grid_line_major.stroke, chart.axis_tick.stroke, chart.axis_label.text, chart.legend.text, chart.legend_label.text, chart.plot_area.padding` | field |
| C-45† | `chart.series.fill = pick(element.item_index, …)` (the default series colour order; ADD-B15) | field |
| C-45† | `chart.values.grid_lines` (`AxisMarks`' defaults) | field |
| C-45† | - | readers (R18) |
| C-46† | `component.activity_rings.ring.fill @ variant @move / variant @exercise / variant @stand` (each ring's paint, flat or a gradient); `component.activity_rings.values.ring_width_px`, `component.activity_rings.values.cap` (the rings' width and ends) | component |
| C-46† | `component.activity_rings.track.fill` | component |
| C-46† | `component.activity_rings.values.ring_gap` | component |
| C-46† | - | readers (R18) |
| C-47† | `rubber_band.body.{fill,stroke,corners}, rubber_band.values.presentations event Disappear` | field |
| C-47† | - | readers (R18) |
| C-48† | `carousel.values.presentations event @page_change @ variant @page_curl` (the keyed kind `page_curl`, its parameters the curl's geometry and the page's back and shading; the drag through `element.progress`; ADD-A26, ADD-B10, ADD-B12) | rule |
| C-48† | `carousel.values.presentations event @page_change @ variant @spine_min / variant @spine_mid / variant @spine_max` (the spine) | rule |
| C-48† | `carousel.values.item_gap @ variant @scroll` (`interPageSpacing`) | rule |
| C-48† | - | readers (R18) |
| C-49† | `component.editing_hud.{body.fill,body.corners,body.padding,label.text}; component.editing_hud.values.presentations events Appear, Disappear` (in by R1.1: the responder's editing interaction, drawn by UIKit in the application's process) | component |
| X-SH-1 | `menu.body.shadow, popover.body.shadow, component.context_menu_preview.preview.shadow, drag_image.lift_preview.shadow` (layers, S2.5) | field |
| X-SH-1 | `dialog.body.shadow @ variant alert` (to verify) | rule |
| X-SH-1 | `sheet.body.shadow @ variant floating / variant bottom` (26's floating sheet; 13–18's card sheets) | rule |
| X-SH-1 | `segmented_control.indicator.shadow, switch.thumb.shadow, slider.thumb.shadow` | field |
| X-SH-1 | `bottom_tab_bar.bar.shadow @ variant floating; toolbar.item_group.shadow` (26's glass groups) | rule |
| X-SH-1 | `button.body.shadow @ {hover}` (the pointer lift) | rule |
| X-SH-1 | - (the Stage Manager windows' shadow, which the system draws, W-2) | out (R1.1) |
| X-SH-2 | `list.row_background.shadow @ variant plain / variant @grouped / variant inset / variant sidebar / variant @sidebar_plain` (each background configuration's `shadowProperties`, layers S2.5) | rule |
| X-SH-2 | `reader:ios:UIBackgroundConfiguration.shadowProperties` | readers (R18) |
| X-SH-3 | `PartStyle.shadow @ cond scheme:dark / cond contrast_increased` | rule |
| X-MT-1 | `PartStyle.fill Material identities (blur radius, saturation, tint)` | field |
| X-MT-1 | `declare.materials` | field |
| X-MT-2 | `toolbar.bar.fill, bottom_tab_bar.bar.fill, menu.body.fill, dialog.body.fill, sheet.body.fill, popover.body.fill, sidebar.pane.fill` (each a `Material` identity) | field |
| X-MT-2 | `dialog.body.fill @ variant @action_sheet` | rule |
| X-MT-2 | `reader:ios:UIBarAppearance.backgroundEffect` | readers (R18) |
| X-MT-3 | `PartStyle.fill @ cond reduced_transparency (Material.fallback)` | rule |
| X-MT-3 | `reader:ios:isReduceTransparencyEnabled` | readers (R18) |
| X-MT-4 | `PaintLayer.blend BlendMode::Keyed (vibrancy styles)` | field |
| X-MT-4 | `PartStyle.text.fill / icon.fill over materials` | field |
| X-MT-5 | `PartStyle.fill Material identities glass_regular and glass_clear` | field |
| X-MT-6 | `Material.tint` | field |
| X-MT-6 | `button.body.fill @ variants {glass,primary}` (the prominent, tinted glass) | rule |
| X-MT-7 | `Material identity (lensing, specular highlight in the identity)` | field |
| X-MT-7 | `PartStyle.fill @ {over_dark_content} (adaptive flip)` | rule |
| X-MT-7 | state flag `over_dark_content` (S7.2) | flag |
| X-MT-8 | `glass_container.surface.{fill,transform,transitions} @ {pressed}; button.body.{fill,transform,transitions} @ {pressed} variant glass` (press scale, highlight, shimmer) | rule |
| X-MT-9 | `glass_container.values.merge_gap` | field |
| X-MT-9 | `PresentationKind::Morph` in the merging elements' `presentations` (S2.7) | field |
| X-MT-10 | `PartStyle.fill.append @ !{over_dark_content}` (the dimming layer over clear glass) | rule |
| X-MT-11 | `cond system_setting:glass_look (ADD-A04)` | condition |
| X-MT-11 | `PartStyle.fill @ cond system_setting:glass_look` | rule |
| X-MT-11 | `reader:ios:user Liquid Glass look preference` | readers (R18) |
| X-MT-12 | `PartStyle.fill and stroke on glass @ cond contrast_increased` | rule |
| X-MT-13 | `reader:ios:UIDesignRequiresCompatibility in the host app's Info.plist` (the generation choice per application, R1.5, S16) | readers (R18) |
| X-CR-1 | `Corners.curve Continuous (S2.4)` | field |
| X-CR-2 | `Radius::Concentric (S2.4)` | field |
| X-CR-2 | `reader:ios:UICornerConfiguration, effectiveRadius(corner:)` | readers (R18) |
| X-CR-3 | `Corners per-corner Radius` | field |
| X-CR-4 | `window.frame.corners (Concentric, following the display, the root's container, S4.6)` | field |
| X-CR-4 | `token.params.display.corner_radius @ cond device_model:<key>` (per device model; an unlisted model's `Absent`, S4.6, R25.11, ADD-A83) | rule |
| X-MO-1 | `token.motion.durations.*` | token |
| X-MO-1 | `token.motion.timings.*` | token |
| X-MO-1 | `Transition.delay` (S2.7) | field |
| X-MO-2 | `token.motion.timings.spring_default (Timing::Spring)` | token |
| X-MO-3 | `navigation_view.values.presentations events @push, @pop` (a `PresentationKind::Keyed` parallax slide, ADD-A26) | field |
| X-MO-3 | `navigation_view.edge_shadow.{fill,opacity}, navigation_view.scrim.{fill,opacity}` (the edge shadow or dim) | field |
| X-MO-3 | `toolbar.values.presentations event @title_change` (the title's cross-fade); `button.values.presentations @ variant back context toolbar.bar` (event `@title_change`: the back label's transition) | rule |
| X-MO-3 | `toolbar.large_title.{opacity,transform}` as expressions on `element.scroll_offset` (the large-title collapse, ADD-B13) | field |
| X-MO-3 | `token.motion.durations.navigation / token.motion.timings.navigation` | token |
| X-MO-4 | `sheet.values.presentations events Appear, Disappear @ variant @cover_vertical / variant @cross_dissolve / variant @flip_horizontal / variant @partial_curl / variant @zoom` (per modal style: kinds `Slide`, `Fade`, `Scale`; keyed kinds for the flip and the partial curl, ADD-A26) | rule |
| X-MO-4 | `sheet.values.presentations event @settle` (the detent changes) | field |
| X-MO-5 | `menu.values.presentations events Appear, Disappear` | field |
| X-MO-5 | `popover.values.presentations events Appear, Disappear` | field |
| X-MO-5 | `dialog.values.presentations events Appear, Disappear` | field |
| X-MO-5 | `dialog.values.presentations event Appear @ variant @action_sheet` | rule |
| X-MO-5 | `component.context_menu_preview.values.presentations events Appear, Disappear` (the lift and dismissal) | component |
| X-MO-6 | `switch.thumb.transitions (Geometry: the slide); switch.track.transitions` (the toggle) | field |
| X-MO-6 | `segmented_control.values.presentations event @indicator_move` (the thumb) | field |
| X-MO-6 | `slider.fill.transitions, slider.thumb.transitions` (the value) | field |
| X-MO-6 | `page_indicator.dot.transitions` (the scrub) | field |
| X-MO-6 | `bottom_tab_bar.values.presentations events @indicator_move, @minimize` (the 26 lens, the minimize) | field |
| X-MO-6 | `button.body.transitions` (26's interactive glass, 13–18's highlight fade) | field |
| X-MO-6 | `list.row_background.transitions; list.values.presentations events Appear, Disappear (the row's), @reload (the list's)` (the highlight fade, the row animations) | field |
| X-MO-6 | `list.disclosure.transitions; list.values.presentations events @expand, @collapse` (the outline disclosure) | field |
| X-MO-6 | `pull_to_refresh.values.presentations event Disappear` (the refresh) | field |
| X-MO-8 | `token.behavior.scroll.{deceleration,overscroll,overscroll_effect,snap}` (rubber-band overscroll, paging snap) | token |
| X-MO-8 | `reader:ios:UIScrollView.DecelerationRate raw values` | readers (R18) |
| X-MO-9 | `<part>.transitions` (`PropertyGroup::Icon`, `TransitionKind::Keyed`: bounce, replace, appear, disappear, draw_on, draw_off, scale; R25.12, ADD-A27) | field |
| X-MO-9 | `IconStyle.animation` (a `Loop`: pulse, variable color, breathe, rotate, wiggle while a state holds; R25.12) | field |
| X-MO-10 | `PartStyle.transitions and presentation transitions @ cond reduced_motion` | rule |
| X-MO-10 | `cond prefer_cross_fade (ADD-A07)` | condition |
| X-MO-10 | `navigation_view.values.presentations, sheet.values.presentations @ cond prefer_cross_fade` (the cross-fade replacing slides) | rule |
| X-MO-10 | `reader:ios:isReduceMotionEnabled, prefersCrossFadeTransitions` | readers (R18) |
| X-MO-11† | `token.theme_change` | token |
| X-DT-1 | `token.text.roles.*` (the values at the default Dynamic Type category) | token |
| X-DT-1 | `reader:ios:Dynamic Type size per style per content size category` | readers (R18) |
| X-DT-2 | `text_scale` and `uifm_scaled` (ADD-A55) in the bases of `list.row.min_height`, `button.body.min_height`, `toolbar.bar.min_height`, the parts' `padding` and `token.icons.sizes.*`; a bar's clamp a `min(…)` (ADD-A56) | field |
| X-DT-2 | `reader:ios:UIFontMetrics scaled values` | readers (R18) |
| X-DT-3 | `cond accessibility_text_size (ADD-A08)` | condition |
| X-DT-3 | `list.values.secondary_text_stacks @ cond accessibility_text_size; dialog.values.action_layout.stack_rule @ cond accessibility_text_size` (the layouts that stack at accessibility sizes: L-19, DL-6; the alert's width, DL-13†) | rule |
| X-DT-3 | `reader:ios:isAccessibilityCategory` | readers (R18) |
| X-DT-4 | `cond bold_text` | condition |
| X-DT-4 | `Font.weight rules @ cond bold_text` | rule |
| X-DT-4 | `IconStyle.weight @ cond bold_text` | rule |
| X-DT-4 | `reader:ios:isBoldTextEnabled, legibilityWeight` | readers (R18) |
| X-DT-5 | `TextStyle.letter_spacing / line_height, Font.axes FollowsSize (S2.8)` | field |
| X-CO-1 | `cond elevated` | condition |
| X-CO-1 | `cond contrast_increased` | condition |
| X-CO-1 | `cond window_inactive` | condition |
| X-CO-1 | `InputValue.per_contrast` | input |
| X-CO-1 | `reader:ios:UIColor.resolvedColor over trait combinations` | readers (R18) |
| X-CO-2 | `role.window.foreground, role.window.foreground_inactive, role.@label_tertiary, role.@label_quaternary` (labels ×4) | role |
| X-CO-2 | `role.@system_fill, role.@system_fill_secondary, role.@system_fill_tertiary, role.@system_fill_quaternary` | role |
| X-CO-2 | `role.@system_background, role.@system_background_secondary, role.@system_background_tertiary, role.@system_grouped_background, role.@system_grouped_background_secondary, role.@system_grouped_background_tertiary` | role |
| X-CO-2 | `role.window.border, role.@separator_opaque, role.@placeholder, role.window.link` | role |
| X-CO-2 | `role.@system_red, role.@system_orange, role.@system_yellow, role.@system_green, role.@system_mint, role.@system_teal, role.@system_cyan, role.@system_blue, role.@system_indigo, role.@system_purple, role.@system_pink, role.@system_brown, role.@system_gray … role.@system_gray6; role.global.accent, role.global.status.*` | role |
| X-CO-3 | `cond tint_dimmed (ADD-A10)` | condition |
| X-CO-3 | `role.global.accent @ cond tint_dimmed (dimmed tint)` | rule |
| X-CO-4 | `Color extended sRGB (S2.1)` | field |
| X-CO-4 | `reader:ios:wide-gamut UIColor components` | readers (R18) |
| X-CO-5 | `role.window.foreground, role.window.foreground_inactive, role.@label_tertiary, role.@label_quaternary` (the label colour's four prominence levels); other colours' prominence `= uikit_prominence(<colour>, <prominence>)` (ADD-A90) | role |
| X-CO-5 | `reader:ios:UIColor.Prominence withProminence` | readers (R18) |
| X-CA-1 | `token.caret.width` (the caret's shape is the caret parts' fields, below; S4.2) | token |
| X-CA-1 | `input.caret.{corners,min_height}, text_area.caret.{corners,min_height}` (the end rounding; the height against the line, over `widget.text.line_height`, the text part's font, S2.2; caret part ADD-A24) | field |
| X-CA-1 | `reader:ios:UITextInput caretRect` | readers (R18) |
| X-CA-2 | `token.caret.{blink_on,blink_off,blink_style,solid_while_typing}` (the pause while typing, ADD-A85; the non-blinking preference: `blink_off` zero, never off, by a Tokens rule under its setting, to verify) | token |
| X-CA-3 | `input.caret_accessory.{icon,padding}` (dictation, language indicator) | field |
| X-CA-3 | `input.floating_caret.{fill,opacity,corners}` | field |
| X-SE-1 | `input.selection.{fill,corners,transitions}, text_area.selection.{fill,corners,transitions}` (the highlight, its corner and its fade) | field |
| X-SE-1 | `role.global.selection_text_background` | role |
| X-SE-2 | `input.selection_handle.{fill,glyph,min_width,min_height}, text_area.selection_handle.{fill,glyph,min_width,min_height}` (knob diameter, stem width, colour) | field |
| X-SE-2 | `input.magnifier.{fill,stroke,corners,shadow,min_width,min_height}, text_area.magnifier.{fill,stroke,corners,shadow,min_width,min_height}; input.values.magnifier_zoom` (the loupe) | field |
| X-SE-3 | `list.row_background.fill @ {pressed} / {selected}` (the grey touch highlight against the tint selection; per context by `context` rules) | rule |
| X-FO-1 | `PartStyle.focus kind Ring (halo shape, position, width, colour)` | field |
| X-FO-2 | `list.row_background.fill @ {focus_visible} (cell focus highlight)` | rule |
| X-FO-2 | `FocusIndicator.transition kind FocusMove (ADD-A19)` | field |
| X-PH-1 | `PartStyle.transform @ {hover} (pointer lift)` | rule |
| X-PH-1 | `PartStyle.shadow @ {hover}` | rule |
| X-PH-1 | `PartStyle.fill @ {hover} (highlight)` | rule |
| X-PH-1 | `button.body.{fill,transform,shadow} @ {hover} variant @pointer_highlight / variant @pointer_lift / variant @pointer_hover` (UIPointerEffect's styles, set by the application per element; UIHoverStyle's shape the body's `corners`) | rule |
| X-PH-2 | - | out (R1.1) |
| X-FO-3† | `cond full_keyboard_access (ADD-A06)` | condition |
| X-FO-3† | `<widget>.values.focus_visible_when (ADD-A16)` | field |
| X-FO-3† | - (the Full Keyboard Access indicator itself, which the system draws) | out (R1.1) |
| X-FO-3† | `reader:ios:Full Keyboard Access (no API found)` | readers (R18) |
| X-HL-1 | `Edge.snap (ADD-A33)` | field |
| X-SC-1 | `role.global.scrim` | role |
| X-SC-1 | `dialog.scrim.fill @ variant alert / variant @action_sheet` | rule |
| X-SC-1 | `sheet.scrim.{fill,opacity}` (per detent, with `sheet.values.undimmed_up_to`) | field |
| X-SC-1 | `component.context_menu_preview.scrim.{fill,opacity}` (the blur and dim) | component |
| X-SC-1 | `sheet.values.presenter_transform, sheet.values.presenter_corners; sheet.presenter_backdrop.fill` (the 13–18 card-sheet background and its black backdrop) | field |
| X-SC-1 | `PartStyle.fill.append @ !{over_dark_content}` (the clear-glass dimming, X-MT-10) | rule |
| X-BT-1 | `token.behavior.long_press (ADD-A45)` | token |
| X-BT-1 | `token.behavior.touch.delay (ADD-A45)` (the touch-highlight delay in scroll views) | token |
| X-BT-1 | `token.caret.{blink_on,blink_off}` | token |
| X-BT-1 | `scrollbar.values.auto_hide_delay; scrollbar.values.presentations event Disappear` | field |
| X-BT-1 | `stepper.values.repeat_delay_ms, stepper.values.repeat_interval_ms` | field |
| X-BT-1 | `pull_to_refresh.values.trigger_distance; list.values.{swipe_full_expansion,swipe_threshold_px}` (the refresh and swipe thresholds) | field |
| X-BT-1 | `reader:ios:UILongPressGestureRecognizer minimumPressDuration` | readers (R18) |
| X-BT-2† | `token.behavior.touch.{double_tap_time,slop}` (ADD-A45) | token |
| X-SV-1 | `SizeMatch.size_classes` (`h_size_class`, `v_size_class`; ADD-A20) | rule |
| X-SV-1 | `cond idiom:phone / cond idiom:pad (ADD-A09)` | condition |
| X-SV-1 | `reader:ios:UITraitCollection horizontalSizeClass, verticalSizeClass, userInterfaceIdiom` | readers (R18) |
| X-SV-2 | `declare.widget.*.sizes (UIButton.Configuration.Size and others)` | field |
| X-SV-3 | `Selector.direction Rtl` | rule |
| X-SV-3 | `token.icons.mirroring` (S2.9) | token |
| X-SV-3 | `reader:ios:layoutDirection` | readers (R18) |
| X-A11-1 | `cond contrast_increased` | condition |
| X-A11-1 | `role.* and PartStyle.stroke @ cond contrast_increased` | rule |
| X-A11-1 | `InputValue.per_contrast` | input |
| X-A11-1 | `reader:ios:isDarkerSystemColorsEnabled, accessibilityContrast` | readers (R18) |
| X-A11-2 | `PartStyle.fill @ cond reduced_transparency` | rule |
| X-A11-3 | `PartStyle.transitions @ cond reduced_motion` | rule |
| X-A11-4 | `cond bold_text` | condition |
| X-A11-4 | `Font.weight rules @ cond bold_text` | rule |
| X-A11-5 | `cond button_shapes` | condition |
| X-A11-5 | `button.label.text.decoration @ cond button_shapes` | rule |
| X-A11-5 | `button.body.fill @ cond button_shapes` | rule |
| X-A11-5 | - (the read of the Button Shapes setting, `buttonShapesEnabled` or its replacement) | readers (R18) |
| X-A11-6 | `cond on_off_labels (ADD-A05)` | condition |
| X-A11-6 | `switch.on_glyph.glyph, switch.off_glyph.glyph` | field |
| X-A11-6 | `reader:ios:isOnOffSwitchLabelsEnabled` | readers (R18) |
| X-A11-7 | `cond differentiate_without_color` | condition |
| X-A11-7 | `<widget>.<part> rules @ cond differentiate_without_color` (one per control the measurement finds changing; which ones the row leaves to verify) | rule |
| X-A11-7 | `reader:ios:shouldDifferentiateWithoutColor` | readers (R18) |
| X-A11-8 | - | out (R1.1) |
| X-A11-9 | `cond prefer_cross_fade (ADD-A07)` | condition |
| X-A11-9 | `presentation transitions @ cond prefer_cross_fade` | rule |
| X-A11-10 | `cond accessibility_text_size (ADD-A08)` | condition |
| X-A11-10 | `reader:ios:preferredContentSizeCategory` | readers (R18) |
| X-IC-1 | `IconStyle.rendering_mode (S2.9)` | field |
| X-IC-2 | `IconStyle.weight / scale per context` | field |
| X-IC-3 | `IconStyle.size = font_size derivation (text-style relative)` | field |
| X-IC-4 | `PartStyle.icon.symbol_variants @ {selected} / {disabled} / …` (`.fill`, `.circle`, `.slash` per state; R25.8; the symbol's variable value is content the application passes to the symbol renderer, no theme value) | rule |
| X-IC-5 | `IconRole additions for UIKit-drawn glyphs (the icon-role list)` | field |
| X-IC-6 | `<part>.transitions` (`PropertyGroup::Icon`, `TransitionKind::Keyed`: bounce, replace, appear, disappear, draw_on, draw_off, scale; R25.12, ADD-A27) | field |
| X-IC-6 | `IconStyle.animation` (a `Loop`: pulse, variable color, breathe, rotate, wiggle while a state holds; R25.12) | field |
| X-IC-7 | - | readers (R18) |
| X-RD-1 | - | readers (R18) |
| X-VR-1 | - | newest (R1.5) |
| SF-1† | `text_area.invisible.glyph @ variant space / variant tab / variant line_break / variant no_break_space` | rule |
| SF-2† | `rich_text.heading_1 … rich_text.heading_6, rich_text.{list_marker,block_quote,code_block,preformatted,table} @ variant markdown` (`absent = true` on `ios-*`) | rule |
| SF-2† | `rich_text.{emphasis,strong,strikethrough}.text, rich_text.inline_code.{text,fill} @ variant markdown` (the inline roles it draws) | rule |
| SF-2† | - (iOS 13–14, which parse no Markdown) | newest (R1.5) |
| SF-3† | `rich_text.inline_code.text, rich_text.inline_code.fill` | field |
| SF-4† | `link.text.text.{fill,decoration} @ variant inline context rich_text` (the rest colour, the tint to verify, and the underline) | rule |
| SF-4† | `link.text.text.{fill,decoration} @ {pressed} variant inline context rich_text` | rule |
| SF-5† | `text_area.values.intent_styles` | field |
| SF-6† | `rich_text.heading_1.text … rich_text.heading_6.text, rich_text.code_block.{text,fill}, rich_text.preformatted.text, rich_text.block_quote.{fill,stroke}, rich_text.list_marker.glyph, rich_text.table.stroke, rich_text.table_cell.{padding,stroke,text}, rich_text.table_header_cell.text, rich_text.heading_1.margin … rich_text.heading_6.margin, rich_text.{paragraph,code_block,preformatted,block_quote,table}.margin @ variant html` | rule |
| SF-6† | `link.text.text @ variant inline context rich_text` | rule |
| SF-7† | `sheet.values.anchor @ h_size_class compact / h_size_class regular / v_size_class compact / v_size_class regular` (ADD-A20) | rule |
| SF-7† | `sheet.scrim.fill @ variant @page / variant @form` (the dimming layer; the form sheet's glass layer on 26) | rule |
| SF-7† | `sheet.values.anchor, sheet.body.corners @ variant @edge_attached v_size_class compact` (attached to the bottom edge) | rule |
| SF-7† | `sheet.values.anchor (full_screen) @ variant @page v_size_class compact / variant @form v_size_class compact` (compact height: full screen, unless the application asks for the edge-attached form, `@edge_attached`) | rule |
| SF-7† | `sheet.values.edge_attached_width @ variant @edge_attached / variants {@edge_attached,@width_follows_content}` (the container's safe-area width, or the content's preferred width where the application asks, `widthFollowsPreferredContentSizeWhenEdgeAttached`) | rule |
| SF-7† | `sheet.values.anchor, sheet.body.{fill,corners,shadow} @ variant @placement_center / variant @placement_leading / variant @placement_trailing` (`ios-26`: iOS 27's `preferredPlacement`, set by the connector from the application's choice; where a leading or trailing sheet goes and how it looks is measured in the value phase) | rule |
| SF-7† | - | readers (R18) |
| SF-8† | `sheet.body.{max_width,max_height,corners} @ variant @form` (the automatic style's mapping on iOS 18) | rule |
| SF-8† | - (the page sheet as the automatic style before iOS 18) | newest (R1.5) |
| SF-9† | `dialog.values.placement @ variant alert (UiKitAlert)` | placement |
| SF-9† | `dialog.values.keyboard_avoidance (KeyboardAvoid(UiKitAlert))` | placement |
| RL-1† | - | out (R1.1) |
| RL-2† | `input.body.{fill,corners,min_height,padding}, input.text.text @ variant editable_label context toolbar.bar` | rule |
| RL-2† | `input.caret.*, input.selection.* @ variant editable_label context toolbar.bar` (X-CA, X-SE) | rule |
| RL-2† | `input.body.transitions, input.text.transitions` (the title turning into the field and back) | field |
| RL-3† | `layout.values.item_gap, layout.values.line_gap @ variant flow` | rule |
| FM-1† | `token.text.fallbacks.emoji_family (ADD-A41)` | token |
| FM-1† | `reader:ios:CTFontCreateForString on an emoji string` | readers (R18) |
| FM-2† | `token.text.fallbacks.per_language, token.text.fallbacks.per_family` (ADD-A41) | token |
| FM-2† | `reader:ios:CTFontCopyDefaultCascadeListForLanguages` | readers (R18) |
| P3-1† | `text_area.values.document_margin; text_area.values.line_padding` | field |
| P3-1† | - | readers (R18) |
| P3-2† | `bottom_tab_bar.item_icon.icon @ variant @more` (the More item's symbol, by its icon role, X-IC-5) | rule |
| P3-2† | `list.row.text, list.row_background.fill, list.leading_icon.icon @ context bottom_tab_bar` (the More list's rows) | rule |
| P3-2† | `bottom_tab_bar.item.fill @ {dragged}; bottom_tab_bar.bar.fill @ {drop_target}` (the Edit screen's drag of items onto the bar) | rule |
| P3-2† | `bottom_tab_bar.values.max_items @ h_size_class compact / h_size_class regular / cond idiom:pad; bottom_tab_bar.item_label.text.elision` (per idiom and size class, ADD-A09, ADD-A20) | rule |
| P3-2† | `bottom_tab_bar.values.presentations event @customize` | field |
| P3-2† | `reader:ios:UITabBarController.moreNavigationController` | readers (R18) |
| P3-3† | `button.body.{fill,corners}, button.icon.icon.fill, button.label.text.fill @ {selected} / {selected,pressed} / {selected,disabled} context toolbar.bar` | rule |
| P3-3† | `button.body.transitions @ context toolbar.bar` | rule |
| P3-3† | `reader:ios:UIBarButtonItem.isSelected` | readers (R18) |
| P3-4† | `toolbar.values.presentations event @items_change` | field |
| P3-5† | - | out (R1.1) |
| P3-6† | `sheet.values.keyboard_avoidance (measured)` | placement |
| P3-9† | `input.caret, text_area.caret (absent = true) @ {text_selected}` (no caret while text is selected) | rule |
| P3-9† | `input.drop_caret.{fill,min_width}, text_area.drop_caret.{fill,min_width}` (the insertion point under a text drag) | field |
| P3-9† | - (the text drop's progress indicator, which the system shows) | out (R1.1) |
| P3-10† | `drag_image.drop_badge.{fill,glyph} @ {drop_copy} / {drop_move} / {@drop_forbidden} / {@drop_cancel}` (the badge per drop operation) | rule |
| P3-10† | `token.pointer.drag_cursors.<drop operation>` | token |
| P3-10† | - (the system's drop-progress indicator) | out (R1.1) |
| P3-11† | `token.behavior.spring_loading (ADD-A45)` | token |
| P3-11† | `button.values.presentations, list.values.presentations, segmented_control.values.presentations, bottom_tab_bar.values.presentations event @spring_load` (the visual effect while a drag hovers) | field |
| P3-11† | `button.body.fill, list.row_background.fill, segmented_control.segment.fill, bottom_tab_bar.item.fill @ {drop_target}` (the activating states) | rule |
| P3-12† | - (the pointer and the settings the system applies to it and to scrolling) | out (R1.1) |
| P3-13† | `token.behavior.key_repeat_delay_ms / key_repeat_interval_ms` | token |
| P3-14† | `token.icons.mirroring` (S2.9) | token |
| P3-14† | `Element.language`, `Element.direction` (the icon lookup's inputs, S2.9, S8; ADD-B05) | field |
| P3-14† | `reader:ios:semanticContentAttribute, layoutDirection` | readers (R18) |
| P4-1† | - | out (R1.1) |
| P4-2† | - | out (R1.1) |
| P4-3† | `toolbar.values.presentations events Appear, Disappear` (the bars hidden on a swipe and shown again) | field |
| P4-3† | `window.values.status_bar_hidden; window.values.status_bar_hidden @ v_size_class compact; window.values.status_bar_animation; window.values.home_indicator_auto_hidden` (what the application configures for the system to draw with, R1.1; the home indicator's default, where the HIG and the API page disagree, to verify) | rule |
| P4-3† | - (the status bar's and the home indicator's own drawing: the fade and slide, the indicator's hide delay and reappearance) | out (R1.1) |
| P4-4† | - | out (R1.1) |
| P4-5† | `window.launch_screen.{fill,bindings}` | field |
| P4-5† | `toolbar.bar.fill, bottom_tab_bar.bar.fill @ context window.launch_screen` (the placeholder bars) | rule |
| P4-5† | `window.values.presentations event @launch_exit` | field |
| P4-5† | - (the transition SpringBoard draws from the tapped icon into the launch screen) | out (R1.1) |
| P4-6† | - | out (R1.1) |
| P4-7† | - | out (R1.1) |
| P4-8† | - | out (R1.1) |
| P4-9† | - | out (R1.1) |
| P4-10† | `toolbar.values.presentations event @customize` | field |
| P4-10† | `toolbar.values.presentations event @items_change` (the items re-laid out in the bar while the editor runs; one home, R20) | field |
| P4-10† | `button.* @ {dragged} context toolbar.bar; toolbar.bar.* @ {drop_target}` (items dragged into and out of the bar) | rule |
| P4-10† | `button.label.text @ variant done context toolbar_editor` (its Done) | rule |
| P4-10† | `menu.item_label.text @ context button variant overflow` (the customize item in the overflow menu) | rule |
| P4-10† | `reader:ios:UINavigationItem.customizationIdentifier` | readers (R18) |
| P4-11† | `token.behavior.touch.double_tap_slop (ADD-A45)` | token |
| P4-12† | `button.values.menu_placement @ variant menu` (`UiKitMenu`: side, `align` mirrored in RTL, `width`, `over_selected_item`, `flip` and `slide`) | placement |
| P4-12† | `combo_box.values.popup_placement` (`UiKitMenu`, the same parameters) | placement |
| P4-12† | `menu.values.item_order` | field |
| P4-13† | `text_area.selection (one element per range); text_area.caret` | field |
| P4-13† | `text_area.values.empty_ranges_draw_carets` | field |
| P4-14† | `menu.values.{submenu_switch,submenu_close_delay_ms}` | field |
| P4-16† | `sidebar.values.breakpoints` (the arrangement per app width, measured Window; the breakpoints set: tiled, variant `overlay`, variant `@displace`; their comparisons and widths to verify; R16) | field |
| P4-16† | `sidebar.* @ h_size_class compact / h_size_class regular` (ADD-A20) | rule |
| P4-16† | `sidebar.values.presentations events @collapse, @expand` | field |
| P4-16† | `reader:ios:UISplitViewController.displayMode, isCollapsed` | readers (R18) |
| P4-18† | `redaction.mask.{fill,corners,transitions}` | field |
| P4-18† | `redaction.mask.fill @ cond scheme:dark` | rule |
| P4-18† | state flag `redacted` (S7.2, ADD-A14) | flag |
