# GNOME buttons and KDE text rows (for the GNOME and KDE lanes)

These rows' ADD-A125 text, removed in ADD-A125's twenty-ninth check and restored to their text at 15039055 (before ADD-A125). The facts are what the gate checks; these trace rows are the lanes' to correct. The checks' findings on them follow.


## GNOME: the rows as ADD-A125 left them (10118960..4ddf458f)

```
| GN-1.30 | `button.body.fill, button.label.text.fill @ variant primary` | rule |
| GN-1.30 | `button.body.fill @ {hover} !{pressed,checked} / {pressed} !{checked} / {checked} !{hover,pressed} / {checked,hover} !{pressed} / {checked,pressed} variant primary (appended layers)` (each state's layer drawn alone, as `%opaque_button` writes one `background-image` per state, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:160-179`, ADD-A125) | rule |
| GN-1.30 | `button.body.focus @ variant primary (outer, offset)` | rule |
| GN-1.32 | `button.body.fill, button.label.text.fill @ variant @opaque` | rule |
| GN-1.32 | `button.body.fill @ {hover} !{pressed,checked} / {pressed} !{checked} / {checked} !{hover,pressed} / {checked,hover} !{pressed} / {checked,pressed} variant @opaque (appended layers)` (the images as GN-1.30's: `.opaque` extends `%opaque_button`, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:213-217`, one `background-image` per state, `:160-179`, the checked hover 5 % and the checked press 30 % alone; the checked hover and press over the raised button's colour, GN-1.24's, which outranks `.opaque`'s, `button:checked:hover` and `:active` (0,2,1), `:47-53`, against `.opaque`'s background (0,1,1), `:216`, so GN-1.24's two rows are written after this row's base, the facts' rows, `docs/facts/gnome/button.toml:1646-1650`, `:1698`, ADD-A125) | rule |
| GN-1.32 | `button.body.fill @ {open,hover} variants {menu,@opaque} / {open,pressed} variants {menu,@opaque}` (the menu button's own opaque background under the images: `menubutton.opaque > button` and `> button:checked` set `background-color: transparent`, (0,2,2), outranking the raised checked colours, so the menu button's `$opaque_button_default_bg` shows, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:416-437`, ADD-A125) | rule |
| GN-1.34 | `button.body.min_width, button.body.min_height, button.body.fill, button.label.text.fill @ variant @osd` | rule |
| GN-1.34 | `button.body.fill @ {hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant @osd` (`%osd_button`'s colours, each replacing the last, the checked hover and press their own, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:125-145`; the facts' rows, `docs/facts/gnome/button.toml:1248`, `:1277`, ADD-A125) | rule |
| GN-1.34 | `button.body.bindings (foreground white) @ {hover} variant @osd / {pressed} variant @osd` (`%osd_button`'s `color: white` on hover and press, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:125-126`, `:130-132`, against its rest colour, the fills' `currentColor` reading it; the facts' rows, `docs/facts/gnome/button.toml:1171-1174`, `:1195-1198`, ADD-A125) | rule |
| GN-1.34 | `button.body.focus @ variant @osd (outer, offset)` | rule |
| GN-1.34 | `button.body.shadow @ variant @osd cond contrast_high (ring layer)` | rule |
| GN-1.35 | `button.body.fill @ variant @card / {hover} !{pressed,checked} variant @card / {pressed} !{checked} variant @card` (each state's layer alone, one `background-image` per state, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:336-352`, ADD-A125) | rule |
| GN-1.35 | `button.body.fill @ {checked} !{hover,pressed,open} / {checked,hover} !{pressed,open} / {checked,pressed} / {checked,open} variant @card` (as the row above; a checked card with its popup open draws the selected hover image, `.has-open-popup`, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:354`, also while pressed, as `:354` follows `:352` at equal specificity, so `{checked,open}` is written last with no exclusion, the facts alike, `docs/facts/gnome/button.toml:2397`, `:2419`, ADD-A125) | rule |
| GN-1.35 | `button.body.fill @ {checked} !{hover,pressed,open} variant @card context any variant @osd / {checked,hover} !{pressed,open} variant @card context any variant @osd / {checked,pressed} variant @card context any variant @osd / {checked,open} variant @card context any variant @osd` (inside any `.osd` ancestor the checked card's background is 10 % of `currentColor`, its `.osd` rule on a checked card outranking `:346` by specificity, each state's image over it, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:356-358`; the context as GN-1.205's, R25.9, ADD-A125) | rule |
| GN-1.35 | `button.body.focus.offset @ variant @card` | rule |
```

## KDE: the rows as ADD-A125 left them (10118960..4ddf458f)

```
| Q1.87 | `list.secondary_text.text (= token.text.keyed.smallest_readable, a whole text style as its neighbours' `text (= …)`, ADD-A125; fill = kirigami_lerp(widget.row.text.fill, surface.background, 0.25), TitleSubtitle.qml:69-71) @ cond flavor:kirigami` | rule |
| Q1.87 | `list.secondary_text.text.fill @ {selected} cond flavor:kirigami` | rule |
| Q1.87 | `list.leading_icon.icon.size (= token.icons.sizes.keyed.medium.size) @ variant with_subtitle cond flavor:kirigami / (= token.icons.sizes.keyed.small_medium.size) @ !variant with_subtitle cond flavor:kirigami` | rule |
| Q1.87 | `list.values.leading_gap @ cond flavor:kirigami` (mediumSpacing) | rule |
| K-P4-14† | `rich_text.heading_1.text … rich_text.heading_6.text (heading_1 to heading_5 with the weight restated and text.font.size = mul(base, <factor>); heading_6 without either) @ variant html` | rule |
| K-P4-14† | `rich_text.heading_1.margin … rich_text.heading_6.margin, rich_text.{paragraph,preformatted,block_quote,definition_term,definition_description}.margin, rich_text.values.list_indent_per_level (the list's) @ variant html` | rule |
| K-P4-14† | `rich_text.inline_code.text, rich_text.preformatted.text (= token.text.keyed.fixed, a whole text style as its neighbours' `text (= …)`, ADD-A125; preserved white space) @ variant html` | rule |
| K-P4-14† | `rich_text.list_marker @ variant html (nested lists indent, rich_text.values.list_indent_per_level)` | rule |
| K-P4-14† | `link.text.text (fill = the link role, underlined) @ variant inline context rich_text` | rule |
| K-P4-14† | `rich_text.table_header_cell.text (its weight restated, centred), rich_text.table_cell.text @ variant html` | rule |
```

## The checks' findings, verbatim: GNOME buttons (GN-1.24 to GN-1.35)

**check-26a.md, item 7**

7. **GNOME's opaque and card buttons have the same one-property replacement that GN-1.57 now excludes, and their
   rows do not exclude it.**
   - This is outside the diff, but the batch's own reasoning applies to them.
   - *Where:*
     - GN-1.30: `{hover} / {pressed} / {checked} / {checked,hover} / {checked,pressed} variant primary (appended
       layers)` (`T-gnome.md:118`);
     - GN-1.35 @card (`:130-131`).
   - *Evidence:*
     - `%opaque_button` writes `background-image` once per state (`libadwaita@1.10.0
       src/stylesheet/widgets/_buttons.scss:160-179`). So a checked, hovered button draws 5 % alone, not
       10 % + 15 % + 5 %.
     - The card button does the same (`:336-354`).
     - `base` is "the expression the rules before it chose" (`S.md:316-317`), so the appended layers stack.
     - The facts state the whole paint per state (`facts/gnome/button.toml:1352-1400`), so the gate would catch a
       stacked rule. The trace rows still tell the lane to write the appended layers.
   - *Fix:* use the same exclusions as GN-1.57 (`{hover} !{pressed}`, `{checked} !{hover,pressed}`), or one
     sentence in the GNOME trace: an "appended layers" row's layer replaces the less specific state's layer, so the
     lane writes each with its exclusions.

**check-27a.md, item 1**

1. **T-gnome:123 (GN-1.32): `@opaque` keeps the stacked layers the batch removed from `primary`. It shares their
   mixin.**
   - *Problem:* GN-1.30 now excludes the states (`{hover} !{pressed,checked}`, …) because "`%opaque_button` writes one
     `background-image` per state". GN-1.32 still reads `button.body.fill @ {hover} / {pressed} / {checked} variant
     @opaque (appended layers)`, with no exclusions and no `{checked,hover}` or `{checked,pressed}`.
   - *Evidence:* `&.opaque { @extend %opaque_button; background-color: $opaque_button_default_bg; … }`
     (`libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:212-217`). The mixin's layers are at `:160-179`: hover
     10 %, active 20 %, checked 15 %, checked hover 5 %, checked active 30 %, each one `background-image` that replaces
     the last. S6.1 merges an `Append` into the earlier value (`spec.md:1934`, `:1963`), so the rows as written resolve
     to these stacks:
     - `{hover,pressed}` @opaque: 10 % + 20 % (libadwaita draws 20 % alone);
     - `{checked,hover}`: 15 % + 10 % (libadwaita draws 5 %);
     - `{checked,pressed}`: 15 % + 20 % (libadwaita draws 30 %).

     Check 26a's finding 7 named only `primary` and the card. The same mixin also reaches the split and menu buttons'
     inner buttons (`:432`, `:602`, `_toolbars.scss:45`).
   - *Fix:* write GN-1.32 as GN-1.30: `{hover} !{pressed,checked} / {pressed} !{checked} / {checked} !{hover,pressed} /
     {checked,hover} !{pressed} / {checked,pressed} variant @opaque (appended layers)`, with the same citation. Give
     every other row that appends `%opaque_button`'s layers the same exclusions.

**check-27b.md, item 1**

1. **T-gnome:123 (GN-1.32 `@opaque`) and T-gnome:127 (GN-1.34 `@osd`) describe looks libadwaita does not draw.
   They also disagree with the facts that cite them, after this round fixed the same defect in GN-1.30 and GN-1.35.**
   - **`@opaque`:**
     - `.opaque` takes `@extend %opaque_button` (`BTN:213-217`), the very placeholder GN-1.30's new note cites
       (`BTN:155-180`, "one `background-image` per state").
     - GN-1.32 still reads `{hover} / {pressed} / {checked} variant @opaque (appended layers)`. It has no exclusions,
       and no `{checked,hover}` or `{checked,pressed}` rows.
     - As written, `{checked,hover}` stacks the 10 % hover image over the 15 % checked one. libadwaita draws the
       single 5 % image (`BTN:170-172`). `{hover,pressed}` stacks 10 % and 20 %, and `{checked,pressed}` stacks
       15 % and 20 %, where libadwaita draws 20 % alone and 30 % alone.
   - **`@osd`:**
     - `%osd_button` writes `background-color` per state, with `:checked:hover` 0.75/0.25 and `:checked:active`
       0.65/0.35 (`BTN:136-145`).
     - GN-1.34 lists only `{hover} / {pressed} / {checked}`. So `{checked,hover}` and `{checked,pressed}` take the
       `{checked}` colour (0.8/0.2).
   - **The facts already disagree with these rows:**
     - `facts/gnome/button.toml:1649-1650` and `:1698-1699` state `@opaque` `{checked,hover}` and
       `{checked,pressed}` with one image each, `rows = ["GN-1.32"]`.
     - `:1248-1250` states `@osd` `{checked,hover}`, `rows = ["GN-1.34"]`.
   - **Fix (trace only):**
     - GN-1.32: `button.body.fill @ {hover} !{pressed,checked} / {pressed} !{checked} / {checked} !{hover,pressed} /
       {checked,hover} !{pressed} / {checked,pressed} variant @opaque (appended layers)`, with GN-1.30's note
       ("as GN-1.30's, one `background-image` per state, `BTN:155-180`").
     - GN-1.34: add `/ {checked,hover} / {checked,pressed} variant @osd` (`BTN:138-145`). It needs no exclusions,
       as the colours replace each other.

**check-28a.md, item 1**

1. **T-gnome:131 (GN-1.35, the checked card): the reason for leaving out the card's `.osd &` look is false.**
   - *Problem:* the row says the card's `.osd` background (`BTN:356-358`) is "an `@osd` context the trace states
     nowhere, so not stated here". The trace does state that context, in exactly the selector this case needs:
     - `switch.track.fill @ {checked} context any variant @osd (any widget, R25.9)` (T-gnome:490, GN-1.205);
     - `… @ {selected} context any variant @osd (any widget, R25.9)` (T-gnome:565-566, GN-1.247).
   - *Evidence:* the model has the form: `ContextMatch { target: None }` ("None: any widget (R25.9)", S:2051), and the
     register has ADD-B08 "Context matching by variant alone" (S:3978). libadwaita's switch uses the same `.osd &`
     form (`_switch.scss:57`). The card's rule sets the checked card's `background-color` to
     `color-mix(in srgb, currentColor 10%, transparent)` inside any `.osd` ancestor. `.osd button.card:checked` is
     (0,3,1), so it beats `:346` for every checked state, and the state images stay on top of it. That is a visible
     look, left out on a false premise.
   - *Fix:* state it. Add `button.body.fill @ {checked} !{hover,pressed,open} variant @card context any variant @osd`,
     and the same for `{checked,hover} !{pressed}`, `{checked,pressed}` and `{checked,open}`. Each takes the 10 %
     base under its state's image (`BTN:356-358`, R25.9). If the GNOME lane has to supply the values, defer the rows
     with a queue number instead. Either way, drop the claim that the trace states no `@osd` context.

**check-28a.md, item 2**

2. **T-gnome:131 (GN-1.35): `{checked,open} !{hover,pressed}` gets the press wrong.**
   - *Evidence:* in `&:checked { … &.keyboard-activating, &:active {…} &.has-open-popup {…} }` (`BTN:345-354`),
     `button.card:checked:active` and `button.card:checked.has-open-popup` are both (0,3,1), and `:354` is later. So a
     checked card with its popup open draws the selected-hover image while it is pressed too. Under the row, that
     combination falls to `{checked,pressed}`, the selected-active image. Hover makes no difference, because both
     images are `$view_selected_hover_color`.
   - The facts state only `{checked,open}` (`facts/gnome/button.toml:2419`), so nothing contradicts them. The
     combination may also be unreachable while a `GtkPopoverBin`'s popover holds the grab
     (`gtk@4.24.1 gtk/gtkpopoverbin.c:117-126`).
   - *Fix:* write `{checked,open}` with no exclusion, after `{checked,pressed}`: ":354 is later than :352 at equal
     specificity".

**check-28a.md, item 3**

3. **T-gnome:123 (GN-1.32): the row does not say what `@opaque`'s checked-hover and checked-press images sit on. Its
   "as GN-1.30's" points the wrong way.**
   - *Evidence:* the cited facts put the raised button's colour under the image, not `.opaque`'s:
     - `{checked,hover}` is `alpha(surface.foreground, 0.35)` + 5 % (`facts/gnome/button.toml:1650`);
     - `{checked,pressed}` is `0.4` + 30 % (`:1699`);
     - the reason is the facts' own comment at `:1646`: "`button:checked:hover` (0,2,1) outranks `button.opaque`
       (0,1,1)".

     This is true at the pin: `.opaque` sets `background-color` at `BTN:216` (0,1,1), while `:47-53` are
     `button:checked:hover` and `button:checked:active` (0,2,1). For primary, `&, &:checked` (`BTN:225-226`, (0,2,1),
     later) keeps the accent under every state. So the "as GN-1.30's" analogy holds for the image and fails for the
     colour under it.
   - The trace lists GN-1.24's `{checked,hover}` and `{checked,pressed}` (T-gnome:92-93) before GN-1.32's
     `@ variant @opaque` (:122). Read in that order, `.opaque`'s colour wins, which is the opposite of the facts. The
     facts gate would catch such a preset, as with check 27b's finding 8, so this is minor.
   - *Fix:* add to GN-1.32's note: "the checked hover and press keep the raised colour, GN-1.24, which outranks
     `.opaque`'s (`:47-53` against `:216`), so GN-1.24's two rows follow GN-1.32's base".

**check-28a.md, item 4**

4. **Two off-by-a-few citation ranges in the new notes.**
   - GN-1.32's `:212-217`: line 212 is blank, and `&.opaque {` is `BTN:213-218`. Write `:213-217`.
   - GN-1.34's `:128-145` (T-gnome:127) starts at the hover block's closing brace, so it leaves out the hover colour
     (`BTN:125-127`). The facts cite `:125-127` for that fill (`facts/gnome/button.toml:1190`). Write `:125-145`.

**check-28a.md, item 5**

5. **T-gnome:126-127 (GN-1.34): the `@osd` hover and press foreground is untraced, though the facts carry it under
   GN-1.34.**
   - *Evidence:* `%osd_button` sets `color: white` on `:hover` and `:active` (`BTN:126`, `:132`), against the rest
     colour `$osd_fg_color`, `RGB(255 255 255 / 90%)` (`_colors.scss:291`). The facts state it as
     `button.body.bindings @ {hover}` and `@ {pressed}` variant `@osd`, with `rows = ["GN-1.34"]`
     (`facts/gnome/button.toml:1171-1174`, `:1195-1198`). The fills' `currentColor` reads that white. GN-1.34's rows
     name only `min_width`, `min_height`, `fill` and `label.text.fill`. This predates the round, but the round's
     GN-1.34 note leans on it.
   - *Fix:* add `button.body.bindings (foreground white) @ {hover} / {pressed} variant @osd` to GN-1.34, citing
     `BTN:125-126` and `:130-132`.

**check-28b.md, item 1**

1. **GN-1.35's checked card with an open popup, while pressed, draws the wrong image.**
   `docs/todo_v0.7.1_model-completeness-spec-trace-gnome.md:131`. The row states `{checked,pressed}` (no exclusion)
   and `{checked,open} !{hover,pressed}`. For {checked, open, pressed} the model therefore draws the selected
   *active* image. In libadwaita, `button.card:checked:active` (`_buttons.scss:351-352`) and
   `button.card:checked.has-open-popup` (`:354`) have the same specificity, (0,3,1), and the later one wins. That
   gives the selected *hover* image. The facts agree with libadwaita: `docs/facts/gnome/button.toml:2419`
   (`{checked,open}`, 0.32) comes after `:2397` (`{checked,pressed}`, 0.39), and the last fact that holds applies
   (S17, "Which fact applies"). So a preset built from this row fails the gate on that combination. GTK sets
   `has-open-popup` from `GtkPopoverBin` (`gtk@4.24.1 gtk/gtkpopoverbin.c:116-126`), which is a context-menu popover,
   not GtkMenuButton.
   *Fix:* `{checked,pressed} !{open}` and `{checked,open} !{hover}`, or `{checked,open}` with no exclusion, written
   last. With hover, both forms give the same colour (0.32).

**check-28b.md, item 2**

2. **GN-1.35 claims the trace has no `@osd` context, but it does. A visible fill is left out with no queue row.**
   Same line. The row says: "its `.osd` background, `:356-358`, an `@osd` context the trace states nowhere, so not
   stated here". But GN-1.205 (`trace-gnome.md:490`) and GN-1.247 (`:565`) both state `context any variant @osd (any
   widget, R25.9)`. The `ContextMatch` target `None` (spec S7, `pub struct ContextMatch`) carries that form. The
   rule `.osd button.card:checked` (specificity (0,3,1)) replaces the card background with `color-mix(in srgb,
   currentColor 10%, transparent)` in every checked state, under the selected images. Leaving it out breaks "cover
   everything visible". No ledger row records it either (`grep osd` on the orch ledger finds nothing).
   *Fix:* add `button.body.fill @ {checked} variant @card context any variant @osd` (with its hover, press and
   open images over the new base), and the matching facts. Otherwise, queue it for the GNOME lane.

**check-28b.md, item 3**

3. **The facts for the `@opaque` menu button's open hover and open press are false.**
   `docs/facts/gnome/button.toml:1671` and `:1720` put the raised checked colours (`alpha(surface.foreground, 0.35)`
   and `0.4`) under the image. In libadwaita, `menubutton.opaque > button` and `> button:checked` set
   `background-color: transparent` (`_buttons.scss:424-437`). The `:checked` form has specificity (0,2,2), which
   outranks `button:checked:hover` and `button:checked:active` (0,2,1). So the visible base is the menu button's
   own `$opaque_button_default_bg` (`:418-420`). Two neighbours already state that: `:1621` (`{open}`) for
   `@opaque`, and `:1441`/`:1478` for the `primary` menu button, which shares the same block. The plain `@opaque`
   facts (`:1649`, `:1698`) are right, because `button.opaque` (0,1,1) loses to (0,2,1) there. These facts predate
   the batch, but GN-1.32 (rewritten in d59cb2d8) cites the rows next to them and the walk-through reaches them.
   *Fix:* `[mix(role.window.background, role.window.foreground, 0.15), srgb(0, 0, 0.0235294, 0.05)]` and the same
   with `0.3`, citing `:424-437`. Add a ledger row for the GNOME lane.

**check-28b.md, item 7**

7. **GN-1.32 says "as GN-1.30's", but `@opaque` needs a different base colour under the checked images.**
   `trace-gnome.md:123`. `.opaque` has no `&:checked` background (`_buttons.scss:213-217`). So for {checked, hover}
   and {checked, pressed}, `button:checked:hover`/`:active` (0,2,1, `:47-53`) shows under the image. The facts
   (`button.toml:1646`, `:1649`, `:1698`) say exactly this. `primary`'s `&, &:checked` (`:225-227`) keeps the accent.
   In the trace's evident order, GN-1.24's generic `{checked,hover}`/`{checked,pressed}` fills (`:92-93`) come
   before GN-1.32's `variant @opaque` Set. With that order, the appended layer sits on the opaque background, and
   the gate would fail. The text does not decide the order.
   *Fix:* in GN-1.32, state that the raised checked hover and press colours are restated under `variant @opaque`
   (or that GN-1.24's two rows are written after GN-1.32's base Set and before GN-1.30's), citing `:47-53` and
   `:213-217`.

**check-29a.md, item 2**

2. **T-gnome:123 (GN-1.32): the new order note states one constraint, and the obvious reading of it breaks
   `primary`, `destructive` and `flat`.**
   - *Problem:* the note says "GN-1.24's two rows are written after this row's base". The trace's rows follow the
     gap map's order, not the rule order (T-gnome:3-4). A writer who moves GN-1.24's `{checked,hover}` and
     `{checked,pressed}` (T-gnome:92-93) below T-gnome:122 also puts them after:
     - GN-1.29's `variant flat` rows (:111);
     - GN-1.30's `variant primary` base (:119);
     - GN-1.31's `variant destructive` rows (:121).
   - *Evidence:* the primary checked-hover fact is the accent plus 5 % (`facts/gnome/button.toml:1425-1426`), because
     `&, &:checked` (`BTN:225-226`, (0,2,1), later) keeps the accent. With GN-1.24's raised row after GN-1.30's base,
     the model gives `alpha(surface.foreground, 0.35)` plus 5 %. The only order that satisfies both libadwaita and
     the facts is: GN-1.32's base, then GN-1.24, then GN-1.29 to GN-1.31. The text states only the first link of that
     chain. The gate would catch the wrong order, so this is minor.
   - *Fix:* "so this row's base is written before GN-1.24's two rows, and those before GN-1.29's, GN-1.30's and
     GN-1.31's, whose own colours `:225-226` and their (0,3,1) rules keep". This is check 28b's finding 7 wording,
     which the fix shortened. Another option removes the order dependence: state `{checked,hover} !{pressed}` and
     `{checked,pressed}` under `variant @opaque` as a `Set` of the raised colour plus the image, which the facts
     already hold (`:1649-1650`, `:1698-1699`).

**check-29a.md, item 3**

3. **T-gnome:124 (GN-1.32, the `@opaque` menu button): the new row leaves out `{open}`, and its citation starts
   two lines early.**
   - *Problem:* the row states only `{open,hover}` and `{open,pressed}`. The rest state, `{open}`, has the same
     cause: `menubutton.opaque > button:checked` is transparent (`BTN:434-435`), so the menu button's
     `$opaque_button_default_bg` (`BTN:418-420`) shows under the 15 % checked image (`BTN:169-170`, through `:432`).
   - *Evidence:*
     - The facts state that case (`facts/gnome/button.toml:1621`, `rows = ["GN-1.32", "GN-1.26"]`).
     - No trace row carries its image. GN-1.32's image rows select `{checked}`, not `{open}`.
     - GN-1.26's `{open} variant menu` (T-gnome:97) gives the raised checked colour.
   - *Citations:* `:416` is the closing brace of `menubutton.suggested-action`, and `&.opaque {` is `BTN:418`. Also,
     `menubutton.opaque > button` alone is (0,1,2), and only `> button:checked` is (0,2,2).
   - *Fix:* "`{open} / {open,hover} / {open,pressed} variants {menu,@opaque}`, … `BTN:418-437`", and give the two
     specificities separately.

**check-29a.md, item 4**

4. **T-gnome:124 against `facts/gnome/button.toml:1671` and `:1720`: the trace row now contradicts the facts it
   implies, and only `merge-notes.md` records that.**
   - *Evidence:* the facts still put `alpha(surface.foreground, 0.35)` and `0.4` under the image for
     `{open,hover}` and `{open,pressed}`, with `rows = ["GN-1.32", "GN-1.26"]`. The new row is the true one (finding
     3's sources). The defect appears in `scratchpad/a125/merge-notes.md` ("GNOME facts defect: button.toml:1671 and
     :1720 … -> GNOME facts agent"), but in no ledger row on `orch-docs`.
   - *Fix:* at convergence, add the ledger queue row for the GNOME lane that the merge notes name, so the
     contradiction is recorded where the lanes read.

**check-29a.md, item 5**

5. **T-gnome:134 (GN-1.35, the checked card in `.osd`): the row is true, but it has no facts and no record for the
   GNOME lane, and its order against the row above is unstated.**
   - *Evidence:* the row is true at the pin:
     - `.osd &` nested in `&:checked` (`BTN:345-359`) is `.osd button.card:checked`, (0,3,1), which beats `:346`
       (0,2,1);
     - the images are `background-image` and stay on top.

     But no fact has `context any variant @osd` (`grep` on `facts/gnome/`), and nothing in the ledger or the merge
     notes names this row. S17 fails "a key and combination with no fact" (S:3725), and GN-1.205 and GN-1.247 have
     the same gap. Also, the rows are `Set`s over the same key and states as T-gnome:133's, so they must come after
     them, and the trace's order is not the rule order (T-gnome:3-4).
   - *Fix:* add "(written after the row above)". Add a merge-notes line, or a ledger queue row, telling the GNOME lane
     to state the four `context any variant @osd` facts (10 % of `currentColor` under each selected image, `BTN:356-358`).

**check-29b.md, item 1**

1. **The trace cannot draw an open menu button. The batch's new GN-1.32 row assumes images that never apply.**
   - Where: `docs/todo_v0.7.1_model-completeness-spec-trace-gnome.md:124` (new), `:97` (GN-1.26), `:118`, `:121`,
     `:123`, `:127`.
   - Why the checked rules never reach a menu button. GNOME's state model never draws `checked` on a menu button.
     `not_drawn` holds `{ states = ["checked"], not_variants = ["toggle"] }` (`docs/facts/gnome/state-models.toml:48`),
     and no menu product holds `toggle` (`:30-46`). So an open menu button is `{open}`, never `{checked}`. Every image
     row of GN-1.30, GN-1.31, GN-1.32 and GN-1.34 is keyed on `checked`, so none of them reaches it. GN-1.26 states
     only `{open} variant menu`, `{open} variants {menu,flat}` and `{open,checked} variant menu`.
   - Walk-through of `{menu,@opaque}`:
     - `{open}`: the result is GN-1.32's bare opaque base, or GN-1.26's 30 % raised colour, depending on an order the
       text does not state. Either way there is no 15 % image. libadwaita draws the opaque base plus 15 %
       (`menubutton.opaque > button:checked`, `_buttons.scss:418-437`, `:169-170`). The fact agrees with libadwaita
       (`docs/facts/gnome/button.toml:1621`).
     - `{open,hover}`: the new row gives the opaque base, and `{hover} !{pressed,checked}` then appends 10 %
       `currentColor`. libadwaita draws the checked hover image, 5 % `RGB(0 0 6)` (`:172-173`).
     - `{open,pressed}`: 20 % instead of 30 % (`:176-178`).
   - The same gap affects:
     - `{menu,primary}` (facts `:1404`, `:1441`, `:1478`);
     - `{menu,destructive}` (`:1827`, `:1866`, `:1905`);
     - `{menu,@osd}` (`:1231`, `:1260`, `:1289`);
     - `{menu,@card}` (`:2889`, `:2916`, `:2943`);
     - the plain menu's `{open,hover}` and `{open,pressed}` (`:805`, `:866`). Those facts cite GN-1.24, whose trace
       row states only `{checked,…}`.
   - The new row's argument, "outranking the raised checked colours", is about CSS. Inside the model no raised
     checked colour reaches a menu button.
   - *Fix:* state the open menu button as rows of its own, mirroring the checked ones:
     - In GN-1.26, add `{open,hover} variant menu / {open,pressed} variant menu` (35 %, 40 %, `:47-53`).
     - For each colour style, add `{open} !{hover,pressed} / {open,hover} !{pressed} / {open,pressed}` with `variants
       {menu,<style>}`. For `primary` and `@opaque`, these are appended layers over the base, and their hover and press
       layers also exclude `open`. For `destructive` and `@osd`, they are replacing colours.
     - Keep the new `@opaque` base row, and order it before those layers.
     - The rejected alternative: having the connector set `checked` with `open`. That breaks S7.2's meaning and the
       state model.

**check-29b.md, item 2**

2. **GN-1.35's card rows also select the menu card, whose inner button libadwaita draws as a raised button.**
   - Where: `trace-gnome.md:132-135`, `:137`.
   - In libadwaita, `menubutton.card` takes the card's background, radius and shadow from `.card`
     (`_misc.scss:201-210`). Its inner `button` has no `.card` class, so it keeps the raised colours (`:29-53`). Only
     its radius is set (`_buttons.scss:447-449`). The facts separate the two cases: `@card` with
     `not_variants = ["menu"]` (`button.toml:2259-2419`), and `{menu,@card}` as card plus 10 / 15 / 30 / 35 / 40 %
     `currentColor` (`:2823-2943`).
   - The trace has no `!variant menu` anywhere in GN-1.35. So for `{menu,@card}`:
     - at rest it gives the card background alone, where libadwaita adds 10 %;
     - on hover it gives the 4 % view-hover image, where libadwaita adds 15 %;
     - when pressed it gives 8 %, where libadwaita adds 30 %;
     - when open it gives GN-1.26's 30 %, which drops the card background, or the card background alone, depending on
       order.
   - GN-1.37's `focus.offset @ variant @card` also reaches the menu card. The fact limits it to non-menu cards
     (`:2293`).
   - *Fix:* add `!variant menu` to every GN-1.35 fill row and to GN-1.37's `@card` focus offset. Add the menu card's
     own rows, `button.body.fill @ variants {menu,@card} / {hover} … / {pressed} … / {open…}` (the card background
     under the raised colours, `_misc.scss:203`, `_buttons.scss:29-53`), citing `:447-449`.

**check-29b.md, item 3**

3. **The card's `{checked,open}` contradicts the state model and the catalogue's flag for `.has-open-popup`.**
   - Where: `trace-gnome.md:133-134` (rewritten in this batch) and `button.toml:2419`.
   - The state model refuses `open` without `menu` or `back` (`state-models.toml:48`, `{ states = ["open"],
     not_variants = ["menu", "back"] }`). So the gate never builds the combination these rows and the fact describe.
     The fact is dead, and the rows the batch refined (written last, no exclusion, `@osd` copies) are never checked.
   - The card's `.has-open-popup` comes from a public `GtkPopoverBin` (`gtk@4.24.1 gtk/gtkpopoverbin.c:116-126`,
     `gtk/gtk.h:217`). For list rows, the catalogue already maps that class to the shared flag `context_menu_open`
     (`spec-widgets.md:464`, "`list` (GNOME's `.has-open-popup`, macOS)"; GN-1.160 at `trace-gnome.md:383`), not to
     `open`.
   - *Fix:* use `context_menu_open` for the card. That means:
     - declare it on `button` in the shared-flag table (`adwaita`, `GtkPopoverBin`);
     - add it to the button state model's `axes.states`;
     - write `{checked,context_menu_open}` in GN-1.35 and in the fact's `when`.
     
     The alternative is to keep `open` and allow it on a non-menu `@card` in `not_drawn`. Either way, queue the
     state-model change for the GNOME lane.

**check-29b.md, item 4**

4. **GN-1.32's order note leaves the order of every other colour style undecided, and the natural reading breaks
   three of them.**
   - Where: `trace-gnome.md:123`.
   - The trace lists its rows "in the order of the platform file" (`:5`), so only notes give rule order. The note
     says "GN-1.24's two rows are written after this row's base". It does not say they stay before the rows of
     GN-1.29 (`flat`), GN-1.30 (`primary`), GN-1.31 (`destructive`), GN-1.34 (`@osd`) and GN-1.35 (`@card`).
   - In the platform-file order, GN-1.29, GN-1.30 and GN-1.31 precede GN-1.32. Moved after GN-1.32's base, GN-1.24's
     `Set`s would replace their checked-hover and checked-press colours with 35 % / 40 % raised `currentColor`, and
     drop `primary`'s images.
   - libadwaita keeps each style's own colours there:
     - `button.flat:checked:hover` (0,3,1), `:262-268`;
     - `.suggested-action:checked` (0,2,1) after `:48`, at `:225-226`;
     - `.destructive-action:checked:hover` (0,3,1), `:202-209`.
   - *Fix (simpler, and independent of order):* leave GN-1.24 where it is. Restate the raised colours under the
     variant instead: `button.body.fill @ {checked,hover} !{pressed} variant @opaque / {checked,pressed} variant
     @opaque` (35 %, 40 %, `:47-53`, `:213-217`), before GN-1.32's appended layers.
     
     Otherwise, state the full order: GN-1.32's base, then GN-1.24, then the rows of GN-1.29 to GN-1.31, GN-1.34 and
     GN-1.35, then GN-1.32's layers. The new `{menu,@opaque}` row (`:124`) also needs its place: after GN-1.24 and
     GN-1.26, before the layers.

**check-29b.md, item 8**

8. **The new GN-1.32 row gives one specificity for two selectors.** `trace-gnome.md:124`: "`menubutton.opaque >
   button` and `> button:checked` … (0,2,2)". The first is (0,1,2), which still outranks `button:hover` and
   `:active` (0,1,1). The second, (0,2,2), outranks `button:checked:hover` (0,2,1). *Fix:* "(0,1,2) and (0,2,2)".

**check-29b.md, item 9**

9. **GN-1.26's `{open,checked} variant menu` is a combination the state model never draws** (`trace-gnome.md:97`;
   `state-models.toml:48`: `checked` only with `toggle`, and no menu product holds `toggle`). *Fix:* drop it, or say
   which drawn combination it is meant for.

**check-29b.md, item 10**

10. **GN-1.30's and GN-1.32's base rows leave out the shadow the facts state.** The facts give `button.body.shadow`
    `"absent"` under `primary` and `@opaque` (`button.toml:1328`, `:1527`, citing GN-1.30 and GN-1.32), as
    `%opaque_button`'s `box-shadow: none` (`_buttons.scss:156`) beats the high-contrast ring (`:31-33`). The trace
    rows (`:117`, `:122`) list only `fill` and `label.text.fill`. GN-1.38's `contrast_high` ring (`:138`) would
    therefore reach both. *Fix:* add `button.body.shadow (Absent)` to both base rows, written after GN-1.38.

**check-29b.md, item 12**

12. **The card's new `@osd` rows have no facts, and the state model has no `@osd` context.** `trace-gnome.md:134`.
    The button model's `axes.contexts` (`state-models.toml:10`) lacks `{ scope = "any", variants = ["@osd"] }`, and
    no fact has that context. So the gate never builds or checks these four rows. *Fix:* queue the context and the
    facts for the GNOME lane.


## The checks' findings, verbatim: KDE text rows (Q1.87, K-P4-14†)

**check-28b.md, item 5**

5. **Two trace rows put a reference in a plain field, which S2.2's typing now refuses.**
   The rows are Q1.87 (`trace-kde.md:576`: `list.secondary_text.text (font = token.text.keyed.smallest_readable;
   …)`) and K-P4-14† (`:1367`: `font = token.text.keyed.fixed`). The batch's typing rule (`spec.md:303-304`) says a
   reference is allowed only in a `Value<T>` field. `TextStyle.font` is a plain `Font` (`spec.md:1018`), and S2.8
   explicitly rejects making the whole font a `Value`. The referenced value is also a `TextStyle`, not a `Font`.
   Their siblings use the well-typed form: K-KA-39†, K-KA-44†, K2.7, K2.8 write `text (= token.text.keyed.…)`.
   *Fix:* reference each leaf of the font (`font.family = token.text.keyed.smallest_readable.font.family`, and so
   on for each of the seven leaves). Kirigami's `font: Kirigami.Theme.smallFont` sets only the font, so this keeps
   the label's own elision and wrap. Alternatively, write `text (= token.text.keyed.…)` and restate the label's own
   elision and wrap.

**check-29a.md, item 1**

1. **T-kde:1367 (K-P4-14†): the new whole-style form is false for Qt's HTML. `<code>` and `<pre>` change only the
   font family.**
   - *Problem:* the row now reads `rich_text.inline_code.text, rich_text.preformatted.text (= token.text.keyed.fixed,
     a whole text style as its neighbours' text (= …), …) @ variant html`. That brings in `fixed`'s whole text style:
     - its size (10pt, `plasma-integration@6.8.2 qt6/src/platformtheme/kfontsettingsdata.cpp:48`);
     - its weight, style, line height, letter and word spacing and `case`;
     - its `fill`, `elision` and `wrap`.
   - *Evidence:* for `Html_code`, `Html_tt`, `Html_kbd`, `Html_samp` and `Html_pre`, Qt's HTML parser emits a single
     declaration. It is `font-family` set to `QFontDatabase::systemFont(QFontDatabase::FixedFont).family()`, with
     `inheritable = true` (`HTML:2182-2196`). `<pre>` adds only `white-space: pre` (`HTML:2198-2215`) and its 12 px
     margins (`HTML:1090-1092`). So the size, weight, style and colour come from the enclosing text: `<b><code>` stays
     bold, and code inside `<h1>` keeps the heading's size. Qt's Markdown importer is different: there, inline code
     sets the whole mono font (`qtextmarkdownimporter.cpp:451-452`, `m_monoFont` at `:115`). So K-SF13†'s
     `(= token.text.roles.monospace)` under `variant markdown` is not a valid analogy for `variant html`, and "as
     its neighbours'" is false.
   - The old form (`font = …`) was wrong too, since it carried the whole font with its size. Check 28b's
     alternative (all seven font leaves by reference) would also be false, because it carries the size, weight and
     style.
   - *Fix:* state the family alone:
     `rich_text.inline_code.text.font.family, rich_text.preformatted.text.font.family
     (= token.text.keyed.fixed.font.family; the rest of the text style the enclosing text's, HTML:2182-2196) @ variant html`.
     - Keep "preserved white space" for `preformatted` (`HTML:2198-2215`).
     - Record the nesting as an open gap, with a ledger queue row for the next batch: inline code inside a heading or
       a bold run takes that run's size and weight, which one `inline_code` part cannot follow.
     - Drop "a whole text style as its neighbours'".

**check-29a.md, item 6**

6. **T-kde:576 (Q1.87): the whole-style reference carries the keyed style's `elision` and `wrap`, which the row
   doesn't restate.**
   - *Evidence:* Kirigami's subtitle is a plain `Text` with `font: root.subtitleFont` (`Platform.Theme.smallFont`),
     `elide: root.elide` (ElideRight) and `wrapMode: root.wrapMode` (NoWrap) (`kirigami@6.31.0
     src/delegates/TitleSubtitle.qml:79`, `:83`, `:87`, `:168-172`). Every field `smallest_readable` brings apart
     from `elision`, `wrap` and `fill` is a `QFont` property, so it is right. The row restates `fill`. The token's
     `elision` and `wrap` are plain fields, and no fact states them (`facts/kde/tokens.toml:1697-1835`). Q1.72 and
     Q1.114 restate `elision End` in the same form.
   - *Fix:* "(= token.text.keyed.smallest_readable; elision End, wrap none, TitleSubtitle.qml:83, :87; fill = …)".

**check-29a.md, item 7**

7. **T-kde:576 and :1367: nested backticks break the code span.** `` `… a whole text style as its neighbours' `text
   (= …)`, ADD-A125; …` ``: the inner backtick closes the outer span. The checker passes, but the row renders
   broken. *Fix:* drop the inner backticks, or write "its neighbours' whole-style form".

**check-29b.md, item 5**

5. **The batch's Q1.87 and K-P4-14† rewrites reference keyed styles the KDE reader never patches, so the user's
   fonts do not reach them. K-P4-14† also takes more than Qt sets.**
   - Where: `trace-kde.md:576`, `:1367`. Neighbours with the same problem: `:874` (Q2.27), `:1408` (K-KA-9†),
     `:1559` (K-KA-39†), `:1591` (K-KA-44†).
   - S9.5 (`spec.md:2611-2616`) has the KDE reader patch the slots: `caption` from `smallestReadableFont`, and
     `monospace` from `fixed`. The patches are `Fields` patches kept on the slot's reference, "so the user's fonts
     reach the parts that reference the slots". The slots are references to the keyed styles
     (`docs/facts/kde/tokens.toml:4093`, `:4295`). A part that references `token.text.keyed.smallest_readable` or
     `token.text.keyed.fixed` directly therefore keeps the preset's 8 pt Noto Sans and 10 pt Hack.
   - Kirigami's `smallFont` is the live `SmallestReadableFont` (`qqc2-desktop-style@6.31.0
     kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:151`, `:199`).
   - For K-P4-14†, Qt's HTML parser sets only `font-family` on `code` and `pre`: the fixed font's family, and on `pre`
     the white space (`qtbase@6.12.0 src/gui/text/qtexthtmlparser.cpp:2182-2211`). So a whole text style wrongly
     brings the fixed font's size, weight, fill, wrap and elision. K-SF13† (`:1193`) and the catalogue
     (`spec-widgets.md:6802`) reference `token.text.roles.monospace` for the same part.
   - Q1.87's whole style also replaces TitleSubtitle's own `elide: Text.ElideRight` and `wrapMode: Text.NoWrap`
     (`kirigami@6.31.0 src/delegates/TitleSubtitle.qml:79-87`; the label takes only `font: subtitleFont`) with the token's.
     The KDE facts state no `elision` or `wrap` for any keyed style.
   - *Fix:*
     - Q1.87 and the four neighbours: reference the font's leaves through the slot,
       `font.<leaf> = token.text.roles.caption.font.<leaf>` (S2.8's `Value` leaves), keeping the label's own
       elision and wrap.
     - K-P4-14† (html): `font.family = token.text.roles.monospace.font.family` only, the rest the paragraph's.
     - K-SF13† may keep the whole font, since the markdown importer sets the whole `m_monoFont`
       (`qtextmarkdownimporter.cpp:115`, `:452`). It should reference the slot's `font` leaves rather than the whole
       style.

**check-29b.md, item 15**

15. **Q1.87 and K-P4-14† nest backticks inside a code span** (`trace-kde.md:576`, `:1367`: "…its neighbours' `text
    (= …)`, ADD-A125…" inside the target's backticks), which splits the span when rendered. *Fix:* "its neighbours'
    whole-style form".
