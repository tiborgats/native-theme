## The combo box's content layout (moved from ADD-A125 to ADD-A127)

ADD-A125's twenty-sixth check moved the combo box's content layout to a batch of its own, ADD-A127, after four
rounds of findings (checks 23 to 26) came from it piecemeal. ADD-A127 needs one survey of every combo form:
editable and not, sizing (size hints) and drawing, on every platform. In ADD-A125 the catalogue's `combo_box` entry
and the traces stand as they did before ADD-A125 touched them, with one deferral note ("the combo box's content
layout: ADD-A127") on the catalogue's `label_arrow_gap_px`, on GN-1.233 and in S22. Citations are at the pins in
`/run/media/tibi/work/cargo-build/native-theme-sources/`.

`button`'s `Inline` rule for its `menu_indicator` (ledger queue #94: the content's space ending the gap before the
indicator's box where its end margin is the negative of a positive start margin, before its margin box otherwise)
stays in ADD-A125: it is `button`'s. Only its application to the combo box moves.

### What ADD-A125 had decided, removed

The catalogue's `combo_box` entry at 10118960:

- `arrow_area` gained `margin` ("a margin in the row, as `button`'s `menu_indicator`'s").
- `arrow` gained `margin` ("WinUI's chevron's end inset, W1-132; Breeze's glyph centred in its rect, K1.240").
- The widget values and the content row:

```
- **Widget values:** `icon_text_gap_px`, `label_arrow_gap_px: Value<Px>` (the content row, ADD-A125: `button`'s,
  from `body`'s content box's start the `icon` where one is drawn, `icon_text_gap_px`, then the label, and the arrow
  area, or the `arrow` where no arrow area is drawn, as the end part placed as `button`'s `menu_indicator` is under
  `Inline`, `label_arrow_gap_px` its gap and its `margin` a margin in the row; on the cross axis it is centred on
  `body`'s box, not the content box (WinUI centres its chevron in the template's whole row under an asymmetric
  padding, `ComboBox_themeresources.xaml:582`; under a padding even on that axis the two agree); on `macos-*` the
  rendering places the arrow area, the `arrow` part's entry; rejected: a rule of the combo box's own, which left the
  icon unplaced and moved written values into the padding; GN-1.233, GN-1.238, K1.240,
  W1-132, MAT-CB1, CB-1; macOS's two gaps measurements); `header_gap_px`, `supporting_text_gap_px:
```

- The variants line gained "The content variant `with_icon` (the current item's or the leading icon drawn, as on
  `button`; the connector sets it from the content, ADD-A61; ADD-A125)", and the content-variant list `combo_box`:
  `with_icon`.
- `may_be_absent`: `icon` "on those three by a rule outside `with_icon`"; values `icon_text_gap_px` (all but
  `material-3`, `kde-breeze`, `macos-*`, as `icon`).

The trace rows, as they stood at 10118960:

```
| K1.240 | `combo_box.body.padding; combo_box.arrow.margin; combo_box.values.label_arrow_gap` (non-editable, the content row `button`'s, the catalogue: the text from `ComboBox_FrameWidth` plus its own pixel, `breeze@6.8.2 kstyle/breezestyle.cpp:3002-3013`, `:5828`, so the padding's start 7; the arrow's 20-wide rect inside `Frame_FrameWidth`, `:2990-2999`, so its end 2; the glyph, `ArrowSize` wide, centred in that rect, `breezehelper.cpp:1513`, `:1567`, `breezemetrics.h:37`, so the arrow's end margin 5 and no start margin; the text ending one pixel inside the edit field, `:5828`, so the gap to the glyph's box 4; the editable combo's text an `input`, ADD-A125) | field |
| K1.240 | `combo_box.body.padding, combo_box.values.{icon_text_gap,label_arrow_gap} @ variant with_icon` (the current item's icon: its rect the icon 4 wider, the pixmap centred, `breezestyle.cpp:5812-5818`, so the padding's start 8 and the gap to the text 3, the text translated past it, `:5821-5823`; the clip set first at the untranslated edit field, `:5800`, so the text ends at the field's end and the gap to the glyph is 3, ADD-A125) | rule |
| W1-132 | `combo_box.arrow.margin; combo_box.values.label_arrow_gap` (the content row `button`'s, the catalogue: the label in the star column, the padding's end 0 as `ComboBoxPadding` writes it, `microsoft-ui-xaml@2.5.1 src/controls/dev/ComboBox/ComboBox_themeresources.xaml:341`, `:566`, `:577`; the chevron right-aligned in the 38-wide column, 12 wide, its end margin 14, `:567`, `:582`, so the gap 12 before its box; centred on the body's box vertically, `VerticalAlignment="Center"` in the template's whole row, `:582`; the body's padding gaining no border, its presenter the bordered `Border`'s sibling, `:571`, `:577`, not inside it, ADD-A125) | field |
| W3-39 | `input.body.{min_height,padding}, text_area.body.{min_height,padding}, list.row.min_height, combo_box.body.{min_height,padding}, date_picker.field.padding, time_picker.field.padding @ density compact` (the combo box's compact padding's end 0 as written, `microsoft-ui-xaml@2.5.1 src/controls/dev/dll/DensityStyles/Compact.xaml:22`, the chevron's end inset its margin, W1-132, unchanged, ADD-A125) | rule |
| MAT-CB1 | `combo_box.body.padding; combo_box.values.label_arrow_gap` (the content row `button`'s, the catalogue: the select's trailing icon in the field's end slot, `material-web@2.5.0 select/internal/select.ts:415`, `:443`, spaced by `content-space` before it and `with-trailing-content-trailing-space` after it, `field/internal/_content.scss:55-58`, `tokens/_md-comp-outlined-field.scss:133`, `:240-241`: the gap 16 before the icon and the padding's end 12; Compose's alike, the text's end padding 16 less half the minimum interactive box less the icon, `androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/internal/TextFieldImpl.kt:386-399`, `:2167-2175`, ADD-A125) | field |
| MAT-CB1 | `combo_box.body.padding, combo_box.values.icon_text_gap @ variant with_icon` (the leading icon spaced by `with-leading-content-leading-space` before it and `content-space` after it, `material-web@2.5.0 field/internal/_content.scss:50-53`, `tokens/_md-comp-outlined-field.scss:133`, `:238-239`: the padding's start 12 and the gap 16; Compose's start padding 16 less the icon box's inset, `TextFieldImpl.kt:388-393`, ADD-A125) | rule |
| CB-1 | `combo_box.values.label_arrow_gap` (the pop-up button's label to its indicator, the content row `button`'s: the `UIButton` menu configuration with the `.popup` indicator, as the iOS host measured `button.values.menu_indicator_gap` for it, 4, `docs/facts/ios/button.toml:27768-27792`, on `ios-13`; ADD-A125) | field |
| MAC-CB01 | `combo_box.values.{label_arrow_gap,icon_text_gap}` (the rendering places the arrow area, the catalogue's `arrow` entry; the label's gap to it and the image's gap to the label measurements, register rows, ADD-A125) | field |
| GN-1.233 | `combo_box.values.label_arrow_gap` (the content row `button`'s, the catalogue: the label's stack and the `arrow` spaced by the box's `border-spacing`, `libadwaita@1.10.0 src/stylesheet/widgets/_dropdowns.scss:2-3`, `:14-18`, the arrow with no margin, ADD-A125) | field |
| GN-1.238 | `combo_box.values.label_arrow_gap @ variant @gtk_combo_box` (libadwaita's drop-down spacing does not reach it: `combobox { @extend dropdown; }` gives `combobox > button > box`, `libadwaita@1.10.0 src/stylesheet/widgets/_deprecated.scss:100-101`, while GtkComboBox's tree is `combobox > box > button > box`, `gtk@4.24.1 gtk/ui/gtkcombobox.ui:5-18`, so its inner box keeps GtkBox's default spacing 0, `gtk/gtkbox.c:267-269`; the cell renderer's padding the value phase reads, ADD-A125) | rule |
```

and W3-39's note "(the combo box's compact padding's end 0 as written,
`microsoft-ui-xaml@2.5.1 src/controls/dev/dll/DensityStyles/Compact.xaml:22`, the chevron's end inset its margin,
W1-132, unchanged)".

S22's ADD-A125 row said: "the combo box's content row `button`'s (the icon, `icon_text_gap_px`, the label, then the
arrow area or arrow as the end part under `Inline`, with a `margin`, centred on `body`'s box on the cross axis, a
content variant `with_icon`), its gaps stated on every preset but `macos-*` (GN-1.233, GN-1.238, K1.240, W1-132,
MAT-CB1, CB-1)"; its follow-ups "the combo box's facts in its content row's form (the padding, the arrow's margin
and the gaps, Breeze's and WinUI's from K1.240 and W1-132), `combo_box.values.{label_arrow_gap,icon_text_gap}` on
`macos-*` (measurements)"; its register list "the combo box's two gaps on `macos-*`".

### Register rows that move with it

- `combo_box.values.label_arrow_gap`, `combo_box.values.icon_text_gap` on `macos-*`, per control size (`mini`,
  `small`, `regular`, `large`): no source states the gaps (the rendering places the arrow area); the probe measures
  the label's end against the arrow area and the image's end against the label. Earlier forms of the row listed
  `ios-*` and `material-3` too; their values have sources (CB-1's measured pop-up gap, MAT-CB1's tokens), above.

### The values as checks 24 to 26 read them (each to re-verify in the survey)

- **Breeze, non-editable** (`breeze@6.8.2 kstyle/breezestyle.cpp`): the edit field `[6, W−20)` (`:3002-3013`), the
  text inside it one pixel each side (`:5828`), the arrow's rect `[W−22, W−2)` inside `Frame_FrameWidth`
  (`:2990-2999`), the 10 px glyph centred in it (`kstyle/breezehelper.cpp:1513`, `:1567`, `breezemetrics.h:37`): the
  gap to the glyph's box 4, to the rect −1. With an icon: the icon rect `iconSize + 4`, the pixmap centred
  (`:5812-5818`), the text translated past it (`:5821-5823`) but clipped first at the untranslated field (`:5800`),
  so the text ends at `W−20`: the gap 3 to the glyph, −2 to the rect, the icon-to-text gap 3. The size hint is 10 px
  wider than the drawn row adds up to (11 with an icon) (26a serious 1, 26b serious 4); the frame margins drop below
  the font height plus 12 (`:3007`, 26b minor 17); editable, the arrow lies in a 20 px `arrow_area` (26a serious 2).
  Kirigami's qqc2 padding (Q1.124) replaces the Breeze padding and would place the arrow twice (26b serious 2).
- **WinUI** (`microsoft-ui-xaml@2.5.1 src/controls/dev/ComboBox/ComboBox_themeresources.xaml`): the label in the star
  column, `ComboBoxPadding` `12,5,0,7` (`:341`, `:566`, `:577`), the chevron 12 wide, right-aligned in the 38-wide
  column, `Margin="0,0,14,0"`, `VerticalAlignment="Center"` in the whole row (`:567`, `:582`): the gap 12; the
  presenter the bordered `Border`'s sibling (`:571`, `:577`), so no border inset; compact padding `12,1,0,3`
  (`dll/DensityStyles/Compact.xaml:22`); the `DropDownOverlay` (margin 4) unmodelled (26b serious 1, minor 10).
- **GTK** (`libadwaita@1.10.0`): `dropdown > button > box { border-spacing: 6px }` (`src/stylesheet/widgets/_dropdowns.scss:2-3`,
  `:14-18`); for `@gtk_combo_box`, `combobox { @extend dropdown; }` (`_deprecated.scss:100-101`) gives `combobox >
  button > box`, which GtkComboBox's `combobox > box > button > box` (`gtk@4.24.1 gtk/ui/gtkcombobox.ui:5-18`) does not
  match, so its inner box keeps GtkBox's spacing 0 (`gtk/gtkbox.c:267-269`); the cell renderer's padding has no
  stated place (26b serious 5, 26a minor 5).
- **Material** (`material-web@2.5.0`): the select's trailing icon in the end slot (`select/internal/select.ts:415`,
  `:443`), `content-space` 16 before it and `with-trailing-content-trailing-space` 12 after (`field/internal/_content.scss:55-58`,
  `tokens/_md-comp-outlined-field.scss:133`, `:240-241`); the leading icon `with-leading-content-leading-space` 12
  and `content-space` 16 (`_content.scss:50-53`, `:238-239`); Compose's text end padding `16 − (48 − 24) / 2`
  (`androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/internal/TextFieldImpl.kt:386-399`,
  `:2167-2175`), which is the filled field's layout (26a minor 4).
- **iOS**: the pop-up button the `UIButton` menu configuration with the `.popup` indicator; the host measured its
  `menu_indicator_gap` at 4 on `ios-13` (`docs/facts/ios/button.toml:27768-27802`); `ios-26` measured too (26b minor
  6); where a wider pop-up's spare width goes is undecided (26b minor 7).
- **macOS**: the system rendering places the arrow area (`ControlImage.arrow`, S15); both gaps are measurements.

### The checks' findings on it, verbatim

### From check-24a.md

**minor 1**

1. **`label_arrow_gap_px` is required on every preset, but only GNOME's trace states it. The new rule gives it a
   meaning on all seven.**
   - *Where:*
     - `W.md:2723`: the value;
     - `W.md:2756-2761`: `may_be_absent`'s values do not list it;
     - the traces: the only row is GN-1.233 (`T-gnome.md:538`);
     - the S22 follow-up (`S.md:3950`) asks for "the combo box's label space ending at the arrow where no arrow area
       is drawn", but not for the values.
   - *Evidence:* the pins already give two platforms' values.
     - **WinUI:**
       - the label's `ContentPresenter` sits in column 0 with right padding 0 (`CBX:341`, `:577`);
       - column 1 is 38 wide (`:567`);
       - the glyph is 12 wide, right-aligned, with `Margin="0,0,14,0"` (`:582`);
       - so the gap is 12.
     - **Breeze**, non-editable:
       - the text is drawn in `SC_ComboBoxEditField`, narrowed by 1 on each side (`BRZ:3002-3013`, `:5797-5829`),
         and ends at `rect.right() − 20`;
       - the arrow rect, 20 wide inside `Frame_FrameWidth`, starts at `rect.right() − 21` (`BRZ:2990-2999`);
       - so the label's space ends 1 px inside the arrow's rect. That is a negative gap if the arrow's box is the
         rect, and the glyph's own size decides it otherwise.
     - macOS (the label beside `ControlImage.arrow`), iOS (the pop-up indicator) and Material have no reading and no
       register row.
   - *Fix:*
     - add the Windows and KDE rows with these citations;
     - record the macOS, iOS and Material gaps as register measurements;
     - or put all of it in S22's follow-ups.

**minor 2**

2. **"Measured from its margin box or box as `button`'s `Inline` rule decides" decides nothing: neither `arrow` nor
   `arrow_area` has a `margin` field.**
   - *Where:* `W.md:2723-2725`, against the part lists `W.md:2705-2717` (`arrow_area`: `fill`, `stroke`, `corners`,
     `min_width`; `arrow`: `glyph`, `transform`, `transitions`).
   - *Evidence:*
     - A part with no `margin` has its margin box equal to its box, so the clause always reads "box".
     - WinUI's glyph end margin (`CBX:582`) then has to be stated as `body.padding`'s end. That works, because the
       label's own end padding is 0.
   - *Fix:* write "before the `arrow`'s box". Or, if the margin is meant, add `margin` to `arrow`'s fields and say so.

### From check-24b.md

**serious 2**

2. **`combo_box.values.label_arrow_gap_px` is required on every preset but stated on `adwaita` alone, so the batch's
   new label rule leaves six presets undecided.**
   - **Where:**
     - The rule: `W:2723-2726`.
     - `combo_box`'s `may_be_absent` values do not list `label_arrow_gap_px` (`W:2759-2761`).
     - The only row is GN-1.233 (`T-gnome:538`). No KDE, Windows, macOS, iOS or Material row names it (search of all
       six traces).
     - S22's follow-up names "the combo box's label space ending at the arrow where no arrow area is drawn"
       (`S:3950`). That is the rule's form, not the missing values.
   - **Evidence:**
     - The platforms do not share an obvious value, so the value phase cannot fill it in without rows:
       - Breeze's non-editable label rect ends at `rect.width() − MenuButton_IndicatorWidth` with no right inset. The
         arrow rect lies inside `Frame_FrameWidth`. So the label overlaps the arrow's 20 px rect by 2 px
         (`breeze@6.8.2 kstyle/breezestyle.cpp:2990-3000`, `:3002-3013`).
       - WinUI puts the label in a `*` column and the 12 px glyph right-aligned in a 38 px column with `Margin="0,0,14,0"`
         (`microsoft-ui-xaml@2.5.1 src/controls/dev/ComboBox/ComboBox_themeresources.xaml:566-567`, `:577`, `:582`).
         That makes the gap before the glyph's box 12 px, with the end margin carried by `body.padding`.
     - macOS's gap is to the image's `ControlImage.arrow` (`W:2709-2713`). iOS's and Material's are not stated
       anywhere.
     - The batch extended #94's rule to the combo box "on every platform", but its operand exists on one.
   - **Fix:**
     - Add a `combo_box.values.label_arrow_gap` row per platform: Breeze's from `:2990-3013` (non-editable; the
       editable combo's text is an `input`), WinUI's from `:566-582`, and macOS's, iOS's and Material's as pending
       measurements where no source states them.
     - Or name the six presets in S22's follow-up list explicitly.

**minor 1**

1. **The combo rule's "measured from its margin box or box as `button`'s `Inline` rule decides" has nothing to decide.
   The arrow's position is not stated.**
   - Where: `W:2723-2726`.
   - `arrow` lists `glyph`, `transform` and `transitions`, and `arrow_area` lists `fill`, `stroke`, `corners` and
     `min_width` (`W:2705-2716`). Neither has a `margin`, so the margin box is always the box.
   - The rule also does not say where the arrow or arrow area lies: at the end of `body`'s content box, as
     `button`'s `Inline` rule places its indicator (`W:1456-1458`). It does not say where the label's space starts
     either. "Before the arrow's box" depends on both.
   - Fix: "the label's space runs from the start of `body`'s content box to `label_arrow_gap_px` before the
     `arrow_area`'s box where one is drawn, and otherwise before the `arrow`'s; either keeps the content box's end."
     Drop the margin-box clause.

### From check-25a.md

**serious 1**

1. **K1.240's gap of −1 is measured to Breeze's 20 px arrow rect. The rule measures to the `arrow`'s box, which on
   the non-editable combo is the 10 px glyph, so the gap is 4.**
   - *Where:*
     - `T-kde.md:351` (K1.240's new `label_arrow_gap` row);
     - `W.md:2727-2728` ("Breeze's label ends one pixel inside the arrow's rect, a negative gap, K1.240").
   - *Evidence:*
     - The rule (`W.md:2723-2726`) ends the label's space "before the `arrow`'s [box] where no arrow area is drawn".
       Outside `editable`, Breeze's arrow area is absent (`T-kde.md:349`: `combo_box.arrow_area (absent = true) @
       !variant editable`).
     - `arrow` lists only `glyph`, `transform` and `transitions` (`W.md:2709-2717`). It has no `min_width` and no
       `margin`. A part's box is its laid-out rectangle (`S.md:1194`), and for a glyph part that is the glyph at
       its `size`.
     - Breeze draws the combo arrow with `renderArrow(painter, arrowRect, …)` (`BRZ:7924-7927`). That draws a
       `min(width, height, ArrowSize)` square (`BRH:1513`, `ArrowSize = 10`, `breezemetrics.h:37`), centred in the
       rect (`BRH:1567`). The facts state the same 10 px for the push button's arrow
       (`facts/kde/button.toml:8166-8180`). The button carries the 20 px rect separately, as
       `menu_indicator.min_width` (`:7976-7984`). The combo's `arrow` has no such field.
     - The arithmetic for a combo W wide:
       - The arrow rect is [W−22, W−2) (`BRZ:2993-2998`), and the glyph's box inside it is [W−17, W−7).
       - The text's rect is `editRect.adjusted(1,0,−1,0)` (`BRZ:5828`) over [6, W−20) (`:3005-3009`). That is
         [7, W−21).
       - So the gap from the text's end to the glyph's box is 4. `body.padding`'s end is 7 (`Frame_FrameWidth`, 2,
         plus (20 − 10) / 2).
       - The −1 holds only against the rect, which the model does not have.
     - With a current-item icon, the text rect moves right by `iconSize + 4` (`BRZ:5821-5823`). It stays clipped to
       the untranslated `editRect`, because the clip is set first (`:5800`). So the space then ends at W−20, a gap
       of 3.
   - *Fix:*
     - K1.240: the gap is 4, before the glyph's 10 px box, with `body.padding`'s end 7. Cite `BRZ:2993-2998`,
       `:3005-3009`, `:5828`, `BRH:1513`, `:1567`.
     - Note the icon case (the clip at `editRect`'s end, `:5800`, a gap of 3) as a rule under the icon, or record
       it.
     - Rewrite `W.md:2727-2728` to match.
     - The alternative is to keep `arrow_area` present outside `editable`, its drawing fields `Absent` and
       `min_width` 20, so that −1 holds against its box. That replaces K1.240's `absent` rule and is the larger
       change.

**serious 2**

2. **Material's gap is not a measurement. Compose states it: 16 dp before the arrow's 24 dp icon.**
   - *Where:*
     - `W.md:2728` ("macOS's, iOS's and Material's are measurements");
     - S22's follow-up and register list (`S.md:3951`: "`label_arrow_gap` on `macos-*`, `ios-*` and `material-3`
       (measurements…)", and "the combo box's label gap on `macos-*`, `ios-*` and `material-3`");
     - the register row to add (`scratchpad/a125/register-rows-to-add.md`: "no source states the gap").
   - *Evidence:* MAT-CB1 takes `material-3`'s combo from the exposed dropdown's text field (`T-material.md:438`).
     - The trailing icon sits in a `minimumInteractiveComponentSize()` box, centred (`TFI:375-381`), placed at the
       field's end (`TFI:1206-1209`).
     - `endPadding = endTextFieldPadding − textFieldHorizontalIconPadding()` (`TFI:386-399`). That is
       `TextFieldPadding` (`TFI:2235-2236`, 16 dp, the default end, `TextFieldDefaults.kt:438-443`) less
       `(48 − 24) / 2` (`TFI:2167-2174`, `SmallIconButtonTokens.kt:39-40`), so 4 dp.
     - The text box takes that end padding (`TFI:445-451`).
     - The icon is `Icons.Filled.ArrowDropDown`, 24 dp (`ExposedDropdownMenu.kt:462-463`).
     - So the gap to the icon's box is 4 + 12 = 16 dp. It is also 16 dp when the minimum interactive size is
       unspecified, because the icon padding is then 0 and `endPadding` is 16.
   - *Fix:*
     - Add a MAT-CB row `combo_box.values.label_arrow_gap` (16 dp) with these citations.
     - Remove `material-3` from the "measurements" in W.md, from S22's follow-up and register lists, and from the
       register row.
     - macOS and iOS stay measurements. The headers state no indicator spacing (`UIButtonConfiguration.h:66-72` has
       only the `indicator` enum).

**serious 3**

3. **The new sentence starts the label's space at the content box's start. That is false wherever the combo's
   `icon` is drawn, and the catalogue has no value for the icon-to-label gap.**
   - *Where:*
     - `W.md:2723-2725`: "the label's space runs from that box's start";
     - the `icon` part (`W.md:2703-2704`): `kde-breeze`, `macos-*` and `material-3`, by `may_be_absent`
       (`W.md:2755-2757`).
   - *Evidence:*
     - **Breeze**, with a current icon:
       - the icon rect is `iconSize + 4` wide at the edit field's start (`BRZ:5812-5814`);
       - the text rect moves right by `iconSize + 4` (`:5821-5823`), and the text starts 1 px further in
         (`:5828`);
       - so the label starts `iconSize + 5` after the content's start, 3 px after the icon's box (the pixmap is
         centred in its rect, `:5818`).
     - **Material**, with a leading icon:
       - `startPadding = 16 − 12` follows the icon's 48 dp box (`TFI:367-373`, `:388-393`);
       - so the label starts 16 dp after the 24 dp icon.
     - The widget values list `label_arrow_gap_px`, `header_gap_px`, `supporting_text_gap_px`, `notch_padding_px`
       and `floating_label_gap_px` (`W.md:2723-2730`). None of them spaces the icon from the label. The visible
       gap therefore has no field, which the project counts as a model defect.
   - *Fix:*
     - Write "from that box's start, or from `icon_label_gap_px` after the `icon`'s box where an icon is drawn".
     - Add `icon_label_gap_px: Value<Px>` with Breeze's 3 (`BRZ:5812-5828`) and Material's 16 dp (`TFI:388-393`),
       macOS's as a measurement, and `may_be_absent` on the presets without an icon.
     - Or, if this is out of ADD-A125's scope, drop the start clause and queue the gap in the ledger.

**minor 1**

1. **W1-132's "the chevron's end margin the body's end padding" will meet S22's follow-up "WinUI's other bordered
   templates' padding plus `BorderThickness`". The combo box's label is not inset by its border.**
   - *Where:* `T-windows.md:362`; `S.md:3951` (the follow-up); S2.11's rule (`S.md:1201-1206`).
   - *Evidence:*
     - The template's root is a plain `Grid` (`CBX:393`).
     - `Background`, the bordered `Border` (`:571`), and `ContentPresenter` (`:577`, `Margin="{TemplateBinding
       Padding}"`) are siblings.
     - So nothing deflates the label by `BorderThickness`. S2.11's argument covers a presenter inside a `Border`
       (`Border.cpp:356-381`), not this template.
     - Applying the follow-up would move the label 1 px off WinUI's 12.
   - *Fix:* add to W1-132, or to the follow-up, "the combo box's padding gains no border: its presenter is the
     border's sibling, `CBX:571`, `:577`".

**minor 2**

2. **"Lies at the end of `body`'s content box" does not hold on macOS, and S22's parenthetical misdescribes macOS.**
   - *Where:* `W.md:2723-2724`; `S.md:3951` ("`label_arrow_gap` on `macos-*`, … (… the label's space ending at the
     arrow where no arrow area is drawn)").
   - *Evidence:*
     - On macOS the arrow area is drawn. It is the system rendering's `ControlImage.arrow` (`W.md:2710-2713`), so
       the image places it, not `body`'s content box.
     - The gap there is to the arrow area, not "at the arrow".
   - *Fix:* add "(on `macos-*`, where the rendering's `ControlImage.arrow` places it)". In S22, drop the
     parenthetical for macOS or write "before the arrow area".

**minor 5**

5. **W1-132 does not say the chevron is centred on the box, not on the content box.**
   - *Where:* `T-windows.md:362`; `W.md:2723-2724`.
   - *Evidence:* `DropDownGlyph` is `VerticalAlignment="Center"` in row 1 (`CBX:582`), the box's row. With the
     padding's top 5 and bottom 7 (`:341`), centring on the content box puts it 1 px higher.
   - *Fix:* state that the arrow is centred on `body`'s box vertically, or leave it to `button`'s `Inline` placement
     and say which.

### From check-25b.md

**serious 2**

2. **The combo box's new label rule starts the label at the content box's start, so it ignores the combo box's `icon`
   part on the three presets that draw one. No value places the icon or the gap after it.**
   - **Where:**
     - The rule: "the label's space runs from that box's start" (`W:2723-2728`).
     - The `icon` part (`W:2703-2704`) is available on `material-3`, `kde-breeze` and `macos-*` (`W:2755-2757`).
     - `combo_box`'s values are `label_arrow_gap_px`, `header_gap_px`, `supporting_text_gap_px`, `notch_padding_px`,
       `floating_label_gap_px` and `popup_placement`. None of them is an icon-to-label gap, and `icon` has no
       `margin`.
   - **Evidence:**
     - Breeze draws the current item's icon in a rect `iconSize + 4` wide at the edit rect's start, then translates
       the text rect by `iconSize + 4` (`breeze@6.8.2 kstyle/breezestyle.cpp:5813`, `:5823`).
     - Because the clip stays the untranslated edit rect (`:5800`), the text's end then becomes the clip's, W−20. So
       K1.240's "minus one" is −2 when an icon is drawn.
     - material-web places the leading icon in `.start`, with `margin-inline: var(--_with-leading-content-leading-space)
       var(--_content-space)`, that is 12 and 16 (`material-web@2.5.0 field/internal/_content.scss:50-53`,
       `tokens/_md-comp-outlined-field.scss:133`, `:238-239`).
     - Under the rule as written, the label overlaps the icon on all three presets.
   - **Fix:**
     - Add `icon_label_gap_px` (or reuse `button`'s `icon_text_gap_px` semantics). Write: "the label's space runs
       from the `icon`'s box plus `icon_label_gap_px`, where an icon is drawn, else from the content box's start".
     - Add rows for Breeze (`:5813`, `:5823`, and the −2 end) and Material (`_content.scss:50-53`).
     - Mark the value `may_be_absent` on the presets that draw no icon.

**serious 3**

3. **Material's `label_arrow_gap` is called a measurement with "no source", but both pinned Material sources state it.
   iOS's gap was already measured on the same control.**
   - **Where:**
     - "macOS's, iOS's and Material's are measurements, ADD-A125" (`W:2728`).
     - The scratchpad's `register-rows-to-add.md`, row `combo_box.values.label_arrow_gap` for `macos-*`, `ios-*` and
       `material-3`: "no source states the gap".
     - There is no `T-material` or `T-ios` row for the key.
   - **Evidence:**
     - **material-web** (the select always sets `has-end`, `select/internal/select.ts:415`, and puts the trailing
       icon in the `end` slot, `:443`):
       - The end section's start margin is `--_content-space`, 16 px (`field/internal/_content.scss:55-58`,
         `tokens/_md-comp-outlined-field.scss:133`; the filled field's `:143`).
       - Its end margin is `with-trailing-content-trailing-space`, 12 px (`:240-241`).
       - So the gap is 16, before a 24 px icon whose box ends at the content box's end, with the body's end padding 12.
     - **Compose:**
       - The trailing icon sits in a `minimumInteractiveComponentSize` box, 48 dp, with the 24 dp icon centred.
       - The text's end padding is `endTextFieldPadding − horizontalIconPadding`, 16 − 12 = 4
         (`androidx@e4d933c9 …/material3/internal/TextFieldImpl.kt:545-567`, `:2170-2174`).
       - The gap to the icon's box is therefore 4 + 12 = 16, and the icon's end is 12 from the edge. The two sources
         agree.
     - **iOS:** the pop-up button is the `UIButton` menu configuration with the `.popup` indicator (CB-1,
       `T-ios:540`). The host measured that configuration's `menu_indicator_gap` at 4 px
       (`facts/ios/button.toml:27770-27792`, cells `*__menu_primary_action__indicator_popup`). Unless
       `changesSelectionAsPrimaryAction` changes the spacing, that measurement is the gap.
   - **Fix:**
     - Add MAT-CB1 rows: `combo_box.values.label_arrow_gap` (16, `_content.scss:55-58`, `TextFieldImpl.kt:545-567`)
       and the body's end padding 12.
     - Add an iOS row that derives from the button's `@indicator_popup` measurement, or say why the pop-up differs.
     - Leave only `macos-*` as a register row.

**minor 1**

1. **Breeze's arrow position depends on a `body.padding` that no row states.**
   - Where: K1.240 (`T-kde:351`), read with the rule "the arrow … lies at the end of `body`'s content box" (`W:2723-2724`).
   - The arrow is at [W−22, W−2], inside `Frame_FrameWidth` (`breezestyle.cpp:2990-2999`, `breezemetrics.h:41`). The
     text starts at `ComboBox_FrameWidth` + 1, that is 7 (`:3004-3008`, `:725-728`, `:5828`). So the padding must be
     asymmetric: start 7, end 2.
   - The value a lane would take first, `PM_ComboBoxFrameWidth` = 6 on both sides, puts the arrow 4 px toward the
     start.
   - W1-132 states the end padding it needs ("the chevron's end margin the body's end padding"), and K1.240 does not.
   - Fix: add to K1.240 "the body's end padding `Frame_FrameWidth`, its start `ComboBox_FrameWidth` plus the text's
     pixel, `:2990-3008`, `:5828`".

**minor 2**

2. **WinUI under `compact`, and the arrow's cross axis.**
   - Compact's `ComboBoxPadding` is `12,1,0,3` (`microsoft-ui-xaml@2.5.1 src/controls/dev/dll/DensityStyles/Compact.xaml:22`).
   - W3-39's `combo_box.body.padding @ density compact` (`T-windows:763`) gives no instruction to keep the end
     at 14. Copied as written, it moves the arrow to the edge.
   - Also, WinUI centres the chevron in the whole row (`VerticalAlignment="Center"`, `MinHeight`,
     `ComboBox_themeresources.xaml:582`), not in the 5/7-padded content box. The rule states no cross-axis placement,
     where `button`'s `Inline` does (`W:1432-1434`).
   - Fix:
     - In W3-39, write "end 14, the chevron's margin, W1-132".
     - In the rule, write "centred on the cross axis of the body's box less its stroke" (or of the content box, with
       WinUI's offset stated).

**minor 3**

3. **`@gtk_combo_box` inherits the drop-down's 6 px gap, which libadwaita's selector does not reach on GtkComboBox.**
   - libadwaita's `combobox { @extend dropdown; }` (`_deprecated.scss:100-101`) yields `combobox > button > box {
     border-spacing: 6px }`.
   - GtkComboBox's tree is `combobox > box.linked > button > box > (cellview, arrow)`
     (`gtk@4.24.1 gtk/ui/gtkcombobox.ui:5-18`; the cell view `hexpand`, `gtk/deprecated/gtkcombobox.c:1073-1076`), so
     the selector does not match.
   - GN-1.238 restates only `padding` and `min_height` (`T-gnome:548`).
   - Fix: add `combo_box.values.label_arrow_gap @ variant @gtk_combo_box` to GN-1.238 (0, or the cell renderer's
     `xpad`, as the value phase reads it). Or confirm the match and say so.

**minor 5**

5. **On macOS two texts place the arrow area.**
   - The rule says it lies at the end of the body's content box (`W:2723-2724`).
   - The parts say it is the S15 image's `ControlImage.arrow` (`W:2712-2713`).
   - The label is not among the `replaces` parts, so the connector lays the label out against one of these.
   - Fix: "where a `SystemControl` image replaces the arrow area, its box is the image's `ControlImage.arrow`, not
     placed by the padding (S15)".

**minor 6**

6. **Decision: the combo rule uses a different mechanism from `button`'s `Inline` rule, with no options stated, and
   breaks the principle `button` chose.**
   - The geometry is the same: Breeze sizes both with `MenuButton_IndicatorWidth`, and iOS's pop-up is a `UIButton`.
   - `button`'s indicator has a `margin`, and its rule was chosen because it "keeps both platforms' written values one
     to one" (`W:1446-1452`).
   - `combo_box.arrow` has no `margin`, so the arrow's end inset must live in `body.padding`. That makes WinUI's
     padding end 14 where the source writes 0 (`ComboBoxPadding`, `:341`) and the chevron's `Margin` 14 (`:582`),
     and Breeze's padding 7/2 where the source writes 6.
   - Fix: either give `arrow` and `arrow_area` a `margin` and reuse the `Inline` rule, or state that option and why it
     was rejected.

### From check-26a.md

**serious 1**

1. **The content-row form makes Breeze's combo box 10 px narrower than Breeze sizes it (11 px with an icon). The
   batch chose the gap that places the glyph and the text, and states nothing about the natural width.**
   - *Where:*
     - `T-kde.md:351`, the gap 4;
     - `T-kde.md:352`, the gap 3 under `with_icon`;
     - `W.md:2725-2732`, the row placed "as `button`'s `menu_indicator` is under `Inline`". There, the margin and
       the gap add to the row's size, and spare width goes to the content (`W.md:1430-1460`).
   - *Evidence:*
     - Breeze's size (`BRZ:3360-3373`):
       - the contents grow by `ComboBox_FrameWidth` (6) on each side;
       - then by `MenuButton_IndicatorWidth + 2` (22);
       - then by `Button_ItemSpacing` (4, `breezemetrics.h:76`).
       - So W = c + 38, where c is Qt's content width: the widest item, plus `iconSize + 4` for an item with an
         icon (`qtbase@6.12.0 src/widgets/widgets/qcombobox.cpp:598-605`, `:641`).
     - The model's row at its natural size:
       - without an icon: 7 + c + 4 + 10 + 5 + 2 = c + 28;
       - with an icon: 8 + icon + 3 + text + 3 + 10 + 5 + 2 = text + icon + 31, where Breeze has text + icon + 42.
     - The drawn positions are right at every width. Breeze's text rect is wider than the text by the 10 px slack
       (`BRZ:3005-3009`, `:5828`).
     - So a connector that sizes the combo box from its content row gets it 10 px narrower than KDE does. The
       batch handled the same kind of slack on the push button through K1.27's margins (`W.md:1441-1446`).
   - *Fix:* state the choice in K1.240 and in the catalogue's rejected options. The options:
     - (a) The gap 14 (`= add(Button_ItemSpacing, 10)`, derived from `:3360-3373`). The natural width and every
       drawn position match. The text's clip ends 10 px early only when the box is squeezed below its natural
       width.
     - (b) The gap 4 kept, with the 10 px named as a known deviation in the natural width.
     - (c) A way to add size to the content's space, as the push button's margins do.
     - I would take (a), with (b)'s clip difference recorded. The same applies to the `with_icon` rule (gap 14,
       or 3 + 11).

**serious 2**

2. **Under `editable`, Breeze's arrow lies in a 20 px `arrow_area`, and the base's `arrow.margin` (end 5) still
   applies there. No text says how the `arrow` is placed inside `arrow_area`, so the glyph's position is
   undecided.**
   - *Where:*
     - `T-kde.md:351`: `combo_box.arrow.margin` is a base `field`. Its "(non-editable, …)" is a parenthetical, not
       a selector.
     - `T-kde.md:349`: `arrow_area.min_width @ variant editable`.
     - `W.md:2710-2718`: the `arrow` is "in `arrow_area`", and its `margin` is "a margin in the row".
   - *Evidence:*
     - Breeze draws the editable combo's arrow through the same `SC_ComboBoxArrow` rect. That rect is 20 wide,
       inside `Frame_FrameWidth` (`BRZ:2990-2999`), with the 10 px glyph centred in it (`BRH:1513`, `:1567`,
       `BRZ:7924-7927`). So the glyph is at [W−17, W−7).
     - The arrow inside the area is not in "the row", so "a margin in the row" decides nothing there.
     - Three readings give three positions:
       - the margin box end-aligned in the area gives [W−17, W−7), which is right;
       - the margin box centred gives [W−19.5, W−9.5);
       - start-aligned gives [W−22, W−12).
     - `arrow_area.margin`, newly listed (`W.md:2709`), is stated nowhere for `kde-breeze`. The rule is that a
       listed field must be stated (`S.md:1229`).
   - *Fix:*
     - In the catalogue, say how a part nested in `arrow_area` is placed: its margin box centred on the area's box,
       on both axes.
     - In K1.240, make the arrow's end margin 5 a rule `@ !variant editable`, with margin 0 in the editable
       combo's area.
     - State `arrow_area.margin` 0 under `editable`.

**minor 1**

1. **On `kde-breeze` and `material-3`, `icon_text_gap_px` may not be absent, yet only a `with_icon` rule states
   it.**
   - *Where:*
     - `W.md:2767`: `icon_text_gap_px` (all but `material-3`, `kde-breeze`, `macos-*`, "as `icon`").
     - `W.md:2761-2764`: on those three the icon is absent "by a rule outside `with_icon`".
     - `T-kde.md:352` and `T-material.md:439` state the gap only `@ variant with_icon`.
   - *Problem:*
     - A base value is required, and none is stated.
     - "On those three", placed right after "GNOME, Windows or iOS", reads as those three platforms, not as
       `material-3`, `kde-breeze` and `macos-*`.
     - Conditional `Absent` revival is queued (#110), so the base cannot leave the gap absent and revive it.
   - *Fix:*
     - State Breeze's 3 and Material's 16 in the base rows, and keep only the padding and the label gap under
       `with_icon`.
     - Write "on `material-3`, `kde-breeze` and `macos-*` by a rule outside `with_icon`".

**minor 2**

2. **`arrow.margin` is not stated on `material-3` or `ios-*`.**
   - *Where:* MAT-CB1 (`T-material.md:438`) and CB-1 (`T-ios.md:541`).
   - *Problem:* `arrow` now lists `margin` (`W.md:2717`). GN-1.233 says "the arrow with no margin". Material puts
     its 12 in `body.padding`, so its margin is 0, but neither row states it.
   - *Fix:* add `combo_box.arrow.margin` (0) to MAT-CB1 and CB-1.

**minor 3**

3. **CB-1 cites only `ios-13`, and its line range ends early.**
   - *Where:* `T-ios.md:541` cites `docs/facts/ios/button.toml:27768-27792`.
   - *Evidence:*
     - The fact runs from `:27769` to about `:27803`, with six cells. The range cuts off the last two.
     - `ios-26` has its own 4 px fact: `button.toml:62806-…`, `preset:ios-26`, `ios-host-macos-26` cells.
     - S22 says the gaps are stated "on every preset but `macos-*`" (`S.md:3952`).
   - *Fix:* cite `:27769-27803` and `:62806-…`, "on `ios-13` and `ios-26`".

**minor 4**

4. **MAT-CB1 cites Compose's filled-field layout for the outlined base.**
   - *Where:* `T-material.md:438-439`.
   - *Evidence:*
     - `TFI:386-399` is in `InsideTextFieldLayout` (`:319`).
     - The outlined field is `CutoutTextFieldLayout` (`:488`). Its own padding block is at `:554-568`.
     - The values agree.
   - *Fix:* add `:554-568` beside `:386-399`. Optionally also cite the filled field's tokens for `@filled`
     (`tokens/_md-comp-filled-field.scss:143`, `:263-266`), which carry the same 16 and 12.

**minor 5**

5. **GN-1.238 leaves out a value it could state.**
   - *Where:* `T-gnome.md:549`, "the cell renderer's padding the value phase reads".
   - *Evidence:*
     - GtkCellRendererText's padding is 2 by 2 (`gtk@4.24.1 gtk/deprecated/gtkcellrenderertext.c:224`). It is
       applied on both sides (`:1622-1625`).
     - So the drawn text-to-arrow distance is 2. The row does not say whether that 2 is the gap or `label.padding`.
   - *Fix:* say which, and cite `:224`. If it is `label.padding`, the label part needs `padding` listed.

**minor 6**

6. **K1.240 states the padding's start and end only.**
   - *Where:* `T-kde.md:351`.
   - *Evidence:*
     - The edit field is inset by `ComboBox_FrameWidth` (6) at the top and bottom too (`BRZ:3008-3009`).
     - That inset applies only while the height is at least the font height plus 12. Breeze's own size
       guarantees it (`:3363-3367`).
   - *Fix:* add "top and bottom 6, `:3008-3009`".

### From check-26b.md

**serious 1**

1. **W:2705-2717, W:2725-2733, T-kde:351, T-windows:362/377-378. Where the arrow goes inside `arrow_area` is not
   defined, and so is where the editable combo's `input` goes in the row.** The new row puts "the arrow area, or
   the `arrow` where no arrow area is drawn" at the end, and gives `arrow` a "margin in the row". But wherever an
   arrow area *is* drawn and the connector lays it out (Breeze's editable combo; macOS is placed by the rendering),
   `arrow` sits in `arrow_area`. Nothing says how: at its start, centred, or at its end with its margin. Breeze's
   base `arrow.margin` (end 5, no start) also applies under `editable`, and only one of those three readings puts the
   glyph where Breeze draws it: centred in the 20×20 `SC_ComboBoxArrow` rect, `breezestyle.cpp:2990-2999`. The row
   also says nothing about the editable combo's text, an `input` in `context any combo_box`.
   - **Breeze:** the edit field ends 2 px *inside* the arrow rect. The field ends at W−20 (`:3002-3006`) and the arrow
     rect starts at W−22. K1.240's gap of 4 is stated for the non-editable combo only, so the editable combo has no
     gap. The base gap applies there and is wrong by 6.
   - **WinUI's editable template:** it does not fit the row at all. `EditableText` spans both columns
     (`Grid.ColumnSpan="2"`, `:580`), and its padding `ComboBoxEditableTextPadding` (`11,5,38,6`, `:342`; compact
     `10,0,30,0`, `Compact.xaml:20`) reserves the chevron column inside the input. If the input took the label's
     place in the row, that column would count twice: once as the 38 of the input's end padding (W1-137's
     `input.body.padding`) and once as the row's gap 12, chevron 12 and margin 14. The editable template also draws
     an arrow area of its own: `DropDownOverlay`, 30 wide with margin 4 in the 38 column, filled on hover and press
     (`:538-554`, `:581`). The catalogue's may_be_absent still marks `arrow_area` absent on `windows-11`.
   - **Fix:** state how a part nested in `arrow_area` is placed (for Breeze, the glyph centred in the area, so
     `arrow.margin` 0 `@ variant editable`, or the margin box centred). State where an editable combo's `input`
     goes: the label's place in the row, or the whole body with its own padding as WinUI writes it. Then add the
     editable rows: K1.240 (the gap −2 to the area's box; the padding from `LineEdit_FrameWidth`) and W1-137 (the
     overlay as `arrow_area`, or recorded as not modelled). The flat editable Breeze combo (`flat = editable &&
     !frame`, `:2978`) loses the frame inset on both sides (`:2990-2993`, `:3007`), so it needs its padding too.

**serious 2**

2. **T-kde:664-665 (Q1.124) against T-kde:351 (K1.240). Under `flavor:kirigami`, the qqc2 padding replaces the
   padding the new row reads.** Since this batch, `combo_box.body.padding` is where the row starts and ends: Breeze's
   start 7 and end 2. Q1.124 restates `combo_box.body.padding` under `cond flavor:kirigami`. That is qqc2's
   `padding: 5`, and `rightPadding: 24` when editable (`qqc2-desktop-style@6.31.0
   org.kde.desktop/ComboBox.qml:33-35`).
   - **Non-editable:** the label, icon and arrow are drawn by Breeze's style item at the control's full size
     (`ComboBox.qml:121-139`, Q1.122 "the same Breeze element"), so they stay at 7 and W−7. The model would put the
     label at 5 and the glyph's end at W−10.
   - **Editable:** the 24 already holds the arrow, which Breeze draws at W−22..W−2. The row would place the arrow
     area after that 24, a second time, ending at W−24.
   - **Fix:** under kirigami, qqc2's padding is only the `contentItem` TextField's place (the `input`'s, in context)
     and an input to the implicit size. So K1.240's padding holds under kirigami, and Q1.124's two padding rules move
     to the input, or are dropped.

**serious 4**

4. **T-kde:351-352 (K1.240). Breeze's combo is 10 px wider than its drawn row adds up to (11 px with an icon), and
   the row does not place those pixels.** `comboBoxSizeFromContents` adds `2 × ComboBox_FrameWidth`,
   `MenuButton_IndicatorWidth + 2` and `Button_ItemSpacing` to the contents: T + 38 (`breezestyle.cpp:3366-3373`;
   with an icon the contents gain `iconSize + 4`, `qcombobox.cpp:601`). The row K1.240 states adds up to 7 + T + 4 +
   10 + 5 + 2 = T + 28. The missing pixels are `ComboBox_FrameWidth` on the end side and `Button_ItemSpacing`.
   - **Effect:** S2.11 sizes a box from its content when `min_width` is `auto`. So a model-sized Breeze combo is
     10 px narrower than Qt's, and its arrow 10 px nearer the text, at every natural size.
   - **Contrast:** the `button` entry handles the same kind of size-versus-draw split for Breeze's menu button on
     purpose (W:1449-1456). The combo does not.
   - **No exact form exists:** (a) gap 14 matches the natural width and every position while the text fits, but
     clips a long text 10 px early. (b) Keeping gap 4 matches the clip, but the natural width falls 10 px short.
   - **Fix:** decide between (a) and (b) and record the cost of the choice. Or add a natural-width term (an
     expression on the content's width, which S8 does not allow today). As it stands the value is simply missing.

**serious 5**

5. **T-gnome:549 (GN-1.238). The cell renderer's padding has no stated place.** The row gives
   `label_arrow_gap @ variant @gtk_combo_box` as GtkBox's spacing 0 and then says "the cell renderer's padding the
   value phase reads". GtkComboBoxText's renderer is a `GtkCellRendererText`, whose default `xpad` is 2
   (`gtk/deprecated/gtkcellrenderertext.c:224`). That padding lies between the text and the arrow and before the
   text. `combo_box.label` has only `text` (W:2702), so the 2 px can only go into the gap and the padding's start.
   As written, the drawn distance (2) and the stated gap (0) disagree, and a reader cannot tell which one the
   value phase states.
   - **Fix:** gap `= 2px` (the renderer's `xpad`), with the padding's start plus `xpad`, cited at `:224`. Or say
     that the value phase adds the renderer's `xpad` to the gap and to the padding's start.

**minor 6, minor 7**

6. **T-ios:541 (CB-1).** CB-1 cites only `ios-13` (`facts/ios/button.toml:27768-27802`). `ios-26` measured the same
   4 px (`:62807-62830`). The ADD-A125 row (`spec.md:3952`) says the gaps are "stated on every preset but `macos-*`".
   Fix: cite both presets.

7. **W:2725-2728 with W:1457-1463. Where a wider iOS pop-up button's spare width goes is not decided.** The combo
   inherits `button`'s rule. That rule's exception, a platform that "packs the content and indicator into one centred
   box", names GTK's icon menu button only. `UIButton` centres its configuration's content
   (`contentHorizontalAlignment` defaults to centre), so the iOS pop-up button is probably the packed case. Neither
   CB-1 nor B-18† says so. Fix: state it, or record it as a measurement.



**minor 8**

8. **W:2728-2731. The centring is not quite stated, and an option is left out.** "It is centred on `body`'s box"
   does not say whether the box or the margin box is centred; `button`'s `Inline` centres the margin box. Also, the
   only option rejected is "a rule of the combo box's own". One option is missing: keep `button`'s `Inline` rule
   unchanged and give WinUI's chevron vertical margins that cancel the padding's asymmetry. The chosen form is
   better, because it keeps WinUI's written `0,0,14,0`, but the decision should say so. Fix: "its margin box
   centred", and add that rejected option.

**minor 9**

9. **W:2761-2763 (may_be_absent `icon`). "On those three" is ambiguous.** The nearest antecedent is "GNOME,
   Windows or iOS", which are already under "all". The meaning is `material-3`, `kde-breeze`, `macos-*`. Fix: name
   them.

**minor 10**

10. **W:2708-2709, W:2716-2717. The new `margin` fields are not stated or marked where nothing reads them.**
    `arrow_area.margin` and `arrow.margin` are new listed fields, so S2.11 says each must be stated or `Absent`.
    On `macos-*` the rendering places both, and neither is in may_be_absent or stated. `arrow_area.margin` is also
    used by no source the batch cites (Breeze's area has margin 0). The one source that would use it is WinUI's
    `DropDownOverlay` (margin 4), and finding 1 leaves that unmodelled. Fix: mark both absent-able on `macos-*`.
    Then either justify `arrow_area.margin` by WinUI's overlay or drop it.

**minor 11**

11. **W:2725-2728. `placeholder` and Material's resting `floating_label` have no place in the row.** The row names
    only "the label". WinUI's placeholder sits in the same `ContentPresenter` (`:577-579`), and Material's resting
    label sits in the label's place. Fix: "the label (or the placeholder or resting label where shown)".

**minor 12**

12. **Trace keys.** W1-132 (T-windows:362) argues "the padding's end 0" but lists no `combo_box.body.padding` key.
    GN-1.233 and CB-1 do not list the padding either. No GNOME, Windows-base, iOS or macOS row traces the base
    `combo_box.body.padding`, which is now one end of the row; K1.240 and MAT-CB1 do list it. Fix: add the key to
    W1-132, GN-1.233 and CB-1, or name the rows that hold the padding.

**minor 14**

14. **`todo_v0.7.1_model-completeness-ledger.md:495`.** The `docs/todo.md` disposition still says WinUI's arrow
    column is "an `arrow_area` element with its own geometry, stated per preset". The new form states that column
    as `arrow.margin` plus the gap, with `arrow_area` absent on `windows-11`. Fix: update the disposition.

**minor 15**

15. **ADD-A125's follow-ups (`spec.md:3952`).** Nothing adds `with_icon` to the combo box's declared variants and
    `variant_sets` or `variant_products` on `kde-breeze`, `material-3` and `macos-*`. So the gate never runs, for
    example, `[with_icon, flat]` or `[with_icon, editable]` (S17 `:3626-3631`). W:2749-2750 also does not say which
    presets declare `with_icon`; S8 ignores it where it is undeclared (`spec.md:2338`), which is the intent on
    GNOME, Windows and iOS. Fix: name the declaring presets and add the state-model follow-up.

**minor 16**

16. **T-material:438.** "16 less half the minimum interactive box less the icon" reads two ways. Fix: "16 less
    half of (48 − 24)", that is 4, with the icon 12 inside its box.

**minor 17**

17. **T-kde:351.** Breeze drops the edit field's frame margins when the combo is shorter than the font height
    plus 12 (`breezestyle.cpp:3007`); the start is then 1, not 7. This never happens at Qt's size hint. Fix: one
    clause, or nothing.

**minor 18**

18. **T-macos:369, ADD-A125's register list.** The register row "the combo box's two gaps on `macos-*`" does not
    say it is per control size, and the combo declares `mini`, `small`, `regular` and `large`. Fix: say per size.

