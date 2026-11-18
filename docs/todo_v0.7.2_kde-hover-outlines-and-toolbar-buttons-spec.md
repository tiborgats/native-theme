# 0.7.2: issue #4's open KDE items, steps 4 and 5 — specification

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  ("the rationale"). This document specifies its step 4 (D6: the hover outlines of the
  check box, the radio button and the combo box) and its step 5 (D7's first half: the
  states of a toolbar button). D6 and D7 (its first half, step 5) were approved by the
  maintainer for 0.7.2 on 2026-11-18, with this specification and its plan. It decides nothing the
  rationale decides; the small points it settles by reading are listed under *Decisions
  taken while specifying*, and what needs the maintainer under *Open points*.
- It continues [`issues/0004_kde-hand-overrides-rationale.md`](issues/0004_kde-hand-overrides-rationale.md)
  §5.4 ("the first rationale") and the backlog entries `docs/todo.md:1767-1933`,
  `:52-111`, `:1230-1511`, `:2860-2913`, which hold the verified Breeze facts.
- Written 2026-11-14 for a release of its own; amended 2026-11-18: the maintainer folded
  steps 4 and 5 into 0.7.2 (D0 (a), the version stays 0.7.2), built on `v0.7.2-rc1`
  after steps 2 and 3 (head `f02c3ae9`). The tree has egui 0.37.0, gpui-component and
  gpui-base 0.10.0, gpui-pre 0.3.12 and iced_widget 0.14.2.
- Every edit is applied by its quoted text, its key or its test's name. A `file:line` of
  a repository file is `0fa28027`'s and serves to find the place (a facts line is three
  higher on this tree); none is re-pointed, and a new citation of a repository document
  names the document only (the maintainer, 2026-11-15).
- `GC` and `EG` lines in this document's reasoning are gpui-component 0.9.1's and egui
  0.36.2's, as written; `<record>/steps45/upstream-reread.md` gives each one's line at
  0.10.0 and 0.37.0 and its verdict (43 hold, 9 changed, 1 false; no behaviour Part A or
  B rests on changed). Every text this document tells a builder to write cites
  gpui-component 0.10.0 and egui 0.37.0.
- Taken before the build, in `<record>/steps45/`: that re-read; three trials
  (`trial/RESULTS.md`: T1, Edit's wrap; T2, a checked button's icon colour; T3, the
  overlay over a combo box's trigger); two reviews and the decisions on them
  (`decisions.md`).
- Each part is complete in itself; part B assumes part A has landed only where it says
  so.
- Abbreviations as the rationale's: `GC` gpui-component `src/`, `GP` gpui-pre 0.3.12
  `src/`, `IW` iced_widget 0.14.2 `src/`, `EG` egui `src/`; `CG`, `CI`,
  `CE` the gpui, iced and egui connectors' `src/`; `F` is `docs/facts/` on the parked
  branch `v0.8.0-rc1` (checkout `/home/tibi/Rust/native-theme`, `78ee0efd`); `PF` is
  `docs/platform-facts.md`; "Breeze" lines are breeze 6.8.2 / `f0b1d753` as the facts
  rows cite them. `<record>` and `<tools>` are the folders the steps 2–3 plan
  ([`todo_v0.7.2_kde-focus-ring-and-button-outlines-plan.md`](todo_v0.7.2_kde-focus-ring-and-button-outlines-plan.md))
  names in its head.

## How to read it

Two parts and a closing section:

| Part | Rationale | What it changes |
|---|---|---|
| A — Step 4 | D6 | `checkbox.hover_border_color`, `combo_box.hover_border_color`; kde-breeze drops two stand-in fills; the three connectors and showcases |
| B — Step 5 | D7, toolbar buttons | `toolbar.button_hover_background`, `.button_hover_border_color`, `.button_active_background`, `.button_checked_background`; three presets; a toolbar button in each connector, a checked one lettered `button.checked_text_color`; the showcases' tool buttons and a checked one in each |
| C — Order and gates | D0, D10 | order and gates; 0.7.2's close-out runs once |

**The facts lines this document names are `0fa28027`'s; each is three higher on this
tree, and each row is found by its text.** Steps 2 and 3 inserted their rows only, each
beside its group, and removed no line (their specification,
[`todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md`](todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md),
decision 2): step 2's row after `focus_ring_offset` (`PF:1110`), step 3's two directly
before §2.3's `hover_background` (`PF:1192`); every `PF` line this document names lies
from `:1218` on. The preset comments below cite no line (the batch's S6). The
operations (which row goes where; no other line changes, decision 1) are stated by name.

**Lines that must not move.** The gpui showcase's gate
`every_colour_claim_is_read_at_the_line_it_cites` (`CG/showcase.rs:2819`) opens each
colour claim at the connector line it cites. An edit above a claimed line keeps the line
count or re-points every claim it moves in the same commit. Part A's edit of
`checkbox.rs` moves lines (A.10). Part B adds code to `variants.rs` only after
`hover_border`, the last function before its `#[cfg(test)]` module, replaces
`use crate::Native;` by one line (B.9), and adds code to `geometry.rs` only after
`button_checked`.

## Decisions taken while specifying

None changes a result of the rationale.

1. **Where the facts rows go.** Step 4: the new row goes directly after
   `hover_background` in §2.5 (`PF:1244`) and in §2.24 (`PF:1625`); step 5: the four
   rows go directly after `border.shadow_enabled` in §2.13 (`PF:1399`). No other line
   changes: no blank line is removed (under the batch's S6 no line number needs to be
   kept in place, and most of the file keeps a blank line around its headings). Each
   row sits beside the row it belongs with.
2. **kde-breeze's `toolbar.button_hover_background` is a preset value only**,
   `#00000000`, which the reader does not write: Breeze's hover fill is
   `alphaColor(Highlight, 0)`, nothing whatever the scheme. The precedent is v0.7.1's
   `scrollbar.track_color` (`docs/archive/todo_v0.7.1_kde-hand-overrides-spec.md`,
   decision 3).
3. **Names.** iced `styles::toolbar_button(resolved, checked)`; gpui
   `variants::toolbar_button(cx)` with the model reads in a crate-visible
   `variants::toolbar_button_states(n)` the contract can walk, and
   `geometry::toolbar_button_checked(n)` for the checked fill; egui
   `toolbar_button_checked_background(t)`, the parity script's naming rule
   (`scripts/check_connector_parity.py:27-35`). A new reader constant
   `FLAT_CHECKED_ALPHA`.
4. **Field order.** Each new checkbox and combo-box field follows `hover_background` in
   the struct, the registry and the element lists; the four toolbar fields follow
   `icon_size` in the struct, before `font` (`model/widgets/mod.rs:563-572`), and
   `icon_size_px` in the registry, before `border` (`docs/property-registry.toml:361-362`).
5. **A checked toolbar button keeps its checked fill and text in every enabled state**,
   pressed included. gpui draws a selected button so whatever a caller does (no hover
   or active style while selected, `GC/button/button.rs:730`, `:751-756`), egui's
   per-call `Button::fill` replaces the fill in every state (`EG/widgets/button.rs:345-347`),
   and iced's `styles::button_toggle` does the same (`CI/styles.rs:291-305`); Breeze's
   frame chain decides `down` before `checked` (F `kde/button.toml:1-12`, header). The
   platforms draw a checked toolbar button under the pointer and while pressed in fills
   of their own, which the model has no field for: WinUI `AccentFillColorSecondary`
   checked and hovered, `AccentFillColorTertiary` checked and pressed
   (F `windows/button.toml:18630`, `:18651`), libadwaita `currentColor` at 13 % and 19 %
   (F `gnome/button.toml:5133`, `:5209`), Breeze the pressed fill (above). So the
   connectors differ from the platforms under the pointer and while the button is held
   on a checked toolbar button; B.11's backlog records the four fills. The hover border
   shows whenever the button is hovered or pressed, checked or not
   (`breezehelper.cpp:710-712`, F `:2991`), where the toolkit allows it (B.13).
6. **egui's tool button and the hover outline.** egui keeps a button's size whatever
   its stroke: its button frame takes `button_padding` as its inner margin and then
   `apply_stroke_and_expansion_without_layout_shift` with the state's `bg_stroke`
   (`egui-0.37.0/src/theme/default_style.rs:54-72`; the function at
   `containers/frame.rs:300-322`), so the stroke takes its width from the padding
   inside the frame (B.13.11 on a fractional width).
   The showcase's `tool_button` sets `spacing.button_padding` and zeroes every state's
   stroke width (`showcase-egui/demo.rs:294-313`). Where the theme states the toolbar's
   hover outline, it sets `bg_stroke.width` to `button.border.line_width` on the states
   that draw it, `hovered` and `active`, and keeps 0 at rest; the padding is not touched
   (B.10).
7. **The gpui showcase draws the combo box's hover outline with an overlay** laid
   absolutely over the trigger: a wrapper's own border would move the trigger in by its
   width. gpui paints a later child over an earlier one, and an element that only has a
   group-hover style gets a `Normal` hitbox, which "doesn't affect mouse handling for
   other hitboxes" (`GP/window.rs:891-894`; `GP/elements/div.rs:2460-2468`). Built in a
   trial (T3): it passes (A.10).
8. **Which facts a new field's preset values come from** (D6's rule, as D6 words it): a
   preset states a new field where `docs/platform-facts.md` gives the platform's value,
   with its source, as one colour for everything the field covers. In the 0.7.x line the facts of
   record are `PF`'s cells with a cited source: every 0.7.x preset value is checked
   against that file, and material-3's `input.hover_border_color` (`material-3.toml:144`,
   `:663`) is stated from such a cell (`PF:1218`). The parked branch's classes
   (checked, imported, pending) are that branch's process, not the rule for a 0.7.x
   preset. A value only the parked facts hold is stated where it is a checked fact there
   (a fact of a platform file with its `[[fact.cite]]`, not a row of `F imported/`) and
   this specification brings its row into `PF` with the source: step 5's toolbar rows
   (B.3) are such. A.5 and B.4 apply the rule to every bundled platform preset.
9. **Test names stay** where a test's list of leaves grows; their doc comments name the
   new leaves.
10. **A combo box's hover outline on an open trigger.** Breeze's pen is `Highlight`
    while `hovered || visualFocus || down` (`breezehelper.cpp:710-712`); what
    it draws on a combo box whose popup is open was measured on the installed Breeze
    before the build (`docs/evidence/breeze-offscreen/2026-11-18/`): while the popup is
    open the trigger is drawn with the `Highlight` pen and a tinted fill whether the
    pointer is over it or not (the style option carries `State_On`; a pointer over the
    trigger then reaches the popup, never the combo box, so Qt reports no hover there
    (under Qt's `offscreen` platform, the measurement's)).
    The hover field draws that pen where the pointer is over the trigger; an open
    trigger with the pointer away is a state the model has no field for (A.12's backlog
    line), and no connector draws the hover field there. So iced draws the outline in
    `Hovered` and `Opened { is_hovered: true }`. egui has one look for an open combo box whatever the
    pointer does (`widgets.open`), which the connector leaves at the border at rest; the
    gpui showcase's overlay follows the pointer.
    The source of this decision is the evidence folder
    `docs/evidence/breeze-offscreen/2026-11-18/` (E4, Part C).
11. **The checked text's fallback.** D7 letters a checked toolbar button in
    `button.checked_text_color` where the theme states the toolbar's checked fill.
    That field is a soft option (`docs/inheritance-rules.toml:412`); where it is `None`
    the text is `button.active_text_color`, as a checked ordinary button takes it
    (`CI/styles.rs:284`; `CG/geometry.rs:1371-1373`). The three presets that state the
    checked fill state `button.checked_text_color` too (B.4).
12. **The egui showcase letters an icon by its scope's text colour**: it tints a bundled
    icon with `ui.visuals().text_color()` when it loads it (`showcase-egui/demo.rs:908`),
    so a checked tool button sets its scope's `override_text_color` to the checked text
    before it builds its image (B.10).
13. **The checked toolbar button the showcases show** (D7, "A checked toolbar button is
    shown") is a fourth tool button in the Basic page's icon row, after Copy, Paste and
    Delete: `IconRole::ActionEdit`, an "edit mode" switch, checked at start, a click
    switching it, the same in the three showcases. Every bundled set carries the role
    (`native-theme/src/model/icons.rs:615`, `:680`, `:747`, `:816`, `:880`). The other
    candidates: none of the toolbar's three buttons switches something that stays on
    with the toolbar in reach (Command Palette and Preferences open modal overlays,
    `showcase-gpui/chrome.rs:510-516`, `showcase-iced.rs:3163-3166`, the dialog stacked
    over the window under `opaque`, which keeps the pointer off it), so a checked state could
    not be hovered; Reload System Theme is momentary); a fourth chrome toolbar button,
    Toggle Side Panel, would change every page's screenshot and put one action on two
    controls. The Basic page's row changes one page and one row.
    Measured before the build (T1, `<record>/steps45/trial/RESULTS.md`): Edit fits on
    the row's line under kde-breeze; under adwaita and catppuccin-mocha it starts a
    second line and the groups below Icons in its column move down 32 to 40 px, staying
    91 px or more inside the page's view; under material-3 the row has two lines already
    and Edit joins Delete on the second. The three showcases wrap alike. Under the
    presets of other platforms a Linux showcase cannot be run; the screenshots workflow
    draws them.
14. **How the gpui test learns a checked toolbar button's icon colour** (B.9, B.13.8).
    Not from the painted frame: gpui-pre 0.3.12's test API exposes painted quads and
    underlines only (`GP/window.rs:3112-3120`), no SVG sprite's colour. A probe child
    inside a selected toolbar Button records `window.text_style().color` at render,
    which is the colour gpui-component 0.10.0 paints an `Icon` in where the icon states
    none (`GC/icon.rs:260-266`, `:213-214`). The test is
    `a_checked_toolbar_buttons_icon_takes_the_checked_text`, a lib test in
    `connectors/native-theme-gpui/src/widgets/tests.rs` (the harness is there, and it
    needs no runner environment), on windows-11 light and dark, on both gpui backends.
    Measured before the build (T2): the colour follows. If
    `a_checked_toolbar_buttons_icon_takes_the_checked_text` fails in the unit that builds
    Edit, the unit stops and the difference goes to the maintainer.

## Corrections to the rationale found while specifying

Three, all found while specifying and since made in the rationale
(`docs/issues/0004_kde-open-items-rationale.md`, cited by section and quoted text, with
no line number, as it is still being edited). (1) The
Windows reader writes the checked toolbar fill from the user's accent (B.5a): D7 now
says so ("On Windows the checked fill is the user's accent fill
(`AccentFillColorDefault`), … so the Windows reader writes it, …"). (2) The KDE
reader writes three toolbar fields from the scheme (B.5: the outline, and the pressed
and checked fills; the transparent hover fill stays the preset's, decision 2): D7's
sentence that starts "So kde-breeze states all four" and §3 item 5 ("With
D7: the KDE reader writes the toolbar button's outline and its pressed and checked
fills, …") now say so. (3) Breeze's hover outline of a check box and a radio
button is not a ring inside the box but the indicator's own outline, stroked again in
the focus colour on the rectangle its frame is drawn on (A.4, read at `f0b1d753`). D6
said "a ring in the focus colour, 1 px, drawn inside the box" and "A connector recolours
the box's own border, where Breeze draws the ring inside it: the nearest the three
toolkits' check boxes take, recorded in the backlog"; it now says "by stroking the box's
outline again in the focus colour, on the rectangle its frame is drawn on, with the
frame's pen width (…), so its border turns that colour" and "A connector
recolours the box's own border, which is what Breeze draws." (both in D6). So no gap
remains and no backlog entry records one (A.12, A.14), and the facts row's KDE cell is
corrected (A.3). The rest holds: the rationale's D6 and D7 carry Breeze's hover outline
on the radio and on a checked control, the checked flat button's outline that no field
carries, the checked toolbar button's text, the checked toolbar button the showcases
show, the rule of decision 8 with "as one colour for everything the field covers" (D6,
the bullet that starts "The other platform presets: a preset states a new field",
with material-3's check box as its example), and GNOME's toolbar look
covering a toolbar's image, image-text and arrow buttons only (D7, "On GNOME the toolbar
look belongs to a toolbar's image, image-text and arrow buttons").

## Open points

Settled with the coordinator after review: material-3 states its combo box's hover
outline (decision 8, A.5), and the checked toolbar button is a fourth tool button with
the edit icon on the Basic page (decision 13, B.10).

Open with the maintainer: the approval of D6 and D7 (the head).

---

## Part A — Step 4 (D6): the hover outlines of the check box, the radio and the combo box

### A.1 Model

`native-theme/src/model/widgets/mod.rs`. Declared as `InputTheme::hover_border_color` is
(`:141-143`): a `soft_option`, no inheritance.

- `CheckboxTheme`, directly after `hover_background` (`:230-232`):

  ```rust
      /// Border colour of a check box's or a radio button's indicator under the
      /// pointer, checked or not. `None` where the theme states none; a connector
      /// then keeps the border at rest (docs/platform-facts.md §2.5).
      #[theme(category = "soft_option")]
      pub hover_border_color: Option<Rgba>,
  ```

  "Checked or not" and the radio are Breeze's: A.4.
- `ComboBoxTheme`, directly after `hover_background` (`:925-927`):

  ```rust
      /// Border colour of the trigger under the pointer. `None` where the theme
      /// states none; a connector then keeps the border at rest
      /// (docs/platform-facts.md §2.24).
      #[theme(category = "soft_option")]
      pub hover_border_color: Option<Rgba>,
  ```

The resolved forms the derive generates are `#[non_exhaustive]`
(`native-theme-derive/src/gen_structs.rs:26`); `CheckboxTheme` and `ComboBoxTheme` are
not, which is the struct-literal break of 0.7.2's *Breaking Changes* bullet (A.12).

`native-theme/tests/proptest_roundtrip.rs:285-317` (`arb_checkbox_theme`) builds every
field: the colour vector becomes `8..=8` and `hover_border_color: colors[7]` is added.
`ComboBoxTheme` is built with `Default` there (`:498`) and needs no edit.

### A.2 Registry and inheritance

- `docs/property-registry.toml`: `[checkbox]` gains `hover_border_color = "color"` after
  `hover_background` (`:243`); `[combo_box]` the same after its `hover_background`
  (`:516`). Gate: `property_registry_keys_match_derive_inventory`
  (`native-theme/src/model/mod.rs`).
- `docs/inheritance-rules.toml` `[no_inheritance]`: after `"checkbox.hover_background"`
  (`:416`) `"checkbox.hover_border_color",       # None: the border at rest stays (§2.5)`;
  after `"combo_box.hover_background"` (`:439`)
  `"combo_box.hover_border_color",      # None: the border at rest stays (§2.24)`. The
  only gate is the negative check `no_inheritance_fields_are_not_inherited`
  (`native-theme/src/resolve/inheritance.rs:816`).

### A.3 `docs/platform-facts.md`

Columns `Property | macOS | Windows | KDE | GNOME | Material` (`PF:1225`, `:1609`).
Cell forms are copied from the neighbouring rows; a link is used only in a form a `PF`
cell already uses for that repository and commit.

**§2.5.** Insert directly after the `hover_background` row (`PF:1244` at
`0fa28027`) the row (decision 1; no other line changes):

| Cell | Text |
|---|---|
| property | `` `hover_border_color` `` |
| macOS | `❓ — AppKit publishes no hover state for a checkbox, and the OS probe renders it at rest only (as hover_background above) · macOS 26: ❓ — the same` |
| Windows | `❓ — not read: border.color above gives WinUI's stroke at rest (CheckBoxCheckBackgroundStrokeUnchecked), hover_background its fills under the pointer, neither a stroke under the pointer` |
| KDE | `` **(Breeze src)** focusColor() = [Colors:View] DecorationFocus — 61,174,233 #3daee9 in Breeze and Breeze Dark ([BreezeLight.colors:116](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeLight.colors#L116); BreezeDark.colors:117): while the pointer is over an enabled check box or radio button, Breeze strokes the indicator's outline again in this colour, on the rectangle its frame is drawn on, with the frame's pen width (and, on a check box, its corner radius; a radio's is an ellipse) and no fill, after the frame, checked or not, so the border turns that colour, faded in by the hover animation (the check box's frame at [breezehelper.cpp:844-845](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L844-L845), :858, :860-863 and its hover stroke at :895, :904, :907; the radio's frame at :996-997, :1010, :1013 and its hover stroke at :1041, :1053-1054; both from the indicator's one rectangle, [breezestyle.cpp:4935](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4935), :4937, :4972-4974, under the pointer where enabled, :4913, :4955); the lighter NeutralText where the widget asks for the neutral highlight, which the model does not carry `` |
| GNOME | `**(Adwaita CSS)** unchecked: the 2px inset ring turns $trough_hover_color, currentColor at 20 % ([_checks.scss:33-36](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_checks.scss#L33-L36); [_colors.scss:281](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_colors.scss#L281)); checked: hover_background above gives a 10 % layer over the fill and nothing on an outline — ❓, not read` |
| Material | `**(M3 token)** unchecked: the outline turns on-surface under the pointer, from unselected-outline-color = on-surface-variant at rest (border.color above, v0_192/_md-comp-checkbox.scss:118, :111); checked: no outline, selected-outline-width 0 (:80). One colour cannot state both` |

The KDE, GNOME and Material texts take what the existing rows give (`hover_background`,
`PF:1244`; `border.color`, `PF:1230`, at `0fa28027`), which keep it; the reading of
Breeze's two functions is the session's of 2026-11-14 (the rationale's D6), its
geometry read on 2026-11-15 in the files fetched at `f0b1d753` (B.3's *The Breeze lines*;
`breezestyle.cpp` at the same commit; `strokedRect` moves each side half a pen width
inward, `breezehelper.cpp:1747-1757`, and `PenWidth::Frame` is 1.001,
`breezemetrics.h:25`, so the check box's frame and its hover stroke lie on the same
rectangle to a thousandth of a pixel). The `hover_background` row's KDE cell
(`PF:1244`) is corrected, not only appended to: its clause "and hover adds a ring inside
it in `focusColor()`, `[Colors:View] DecorationFocus` #3daee9 (:897-907)", which the
fetched lines disprove, becomes "and under the pointer its outline is stroked again in
`focusColor()`, `[Colors:View] DecorationFocus` #3daee9, on the frame's own rectangle
(:895-907; the frame's at :844-845), whose colour is `hover_border_color`".

**§2.24.** Insert directly after the `hover_background` row (`PF:1625` at
`0fa28027`) the row (decision 1; no other line changes):

| Cell | Text |
|---|---|
| property | `` `hover_border_color` `` |
| macOS | `❓ — AppKit publishes no hover colour for a pop-up button, and the OS probe renders it at rest only (as hover_background above) · macOS 26: ❓ — the same` |
| Windows | `❓ — not read: border.color above inherits the default border at rest, hover_background gives WinUI's fill under the pointer, neither a ComboBoxBorderBrushPointerOver` |
| KDE | `**(Breeze src)** Highlight, [Colors:Selection] BackgroundNormal of the Active group: a non-editable combo box is a button frame whose hover turns its pen Highlight ([breezehelper.cpp:710-712](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L710-L712), :672-673); on an open combo box the pen is Highlight and the fill tinted whatever the pointer does (measured, [combo_trigger_light.txt](evidence/breeze-offscreen/2026-11-18/combo_trigger_light.txt), [combo_trigger_dark.txt](evidence/breeze-offscreen/2026-11-18/combo_trigger_dark.txt)) — 61,174,233 #3daee9 in Breeze and Breeze Dark ([BreezeLight.colors:87](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeLight.colors#L87); BreezeDark.colors:88). An editable combo box is a line edit, framed as §2.4 hover_border_color` |
| GNOME | `**(none)** at normal contrast — GtkDropDown's trigger is a button with no border at rest (border.color above) and shows hover by its fill (hover_background above); under prefers-contrast: more its 1px inset line on hover was not read` |
| Material | `**(M3 token)** text-field-hover-outline-color = on-surface — L #1d1b20, D #e6e0e9 ([v0_192/_md-comp-outlined-select.scss:168-169](https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L168-L169)), as hover_background above says` |

Sources of the values: the Breeze colours are kde-breeze's existing comments
(`kde-breeze.toml:166-169`, and v0.7.1's `[Colors:Selection]` comments); that
`Highlight` comes from the Active group is F `kde/button.toml:8-10` (header,
`breezehelper.cpp:672-673`); the on-surface pair is `PF:1218`'s Material cell. The
`hover_background` row's KDE cell (`PF:1625`) gains ", whose pen is
`hover_border_color`" directly after its link `([breezehelper.cpp:710-712](…))`, which
follows "a non-editable combo box is a button frame whose hover turns only the pen
`Highlight`"; the rest of the cell (", its fill animates by …") stays. The new row's
clause on an open combo box (decision 10) cites E4's evidence folder, which D0
commits; U1 writes the row whole, as the table gives it. The clause's links name the
two files and no line (the maintainer's rule of 2026-11-15, which the evidence
README's older sentence does not override): `(measured,
[combo_trigger_light.txt](evidence/breeze-offscreen/2026-11-18/combo_trigger_light.txt),
[combo_trigger_dark.txt](evidence/breeze-offscreen/2026-11-18/combo_trigger_dark.txt))`.

The rows inserted move the facts lines below them; no citation of those lines is
re-pointed, and no gate compares a facts line number.

Gates: `scripts/check_platform_facts_rows.py` (every registry field has a row), and
`scripts/check_preset_citations.py`, `documented_sizes.rs`' test and
`native-theme/tests/platform_facts_xref.rs` as v0.7.1's unit LN leaves them (Part I;
none of them compares a line number with the line it names any more). Code comments and the
backlog's line citations are not gated and are left as they are (S6).

### A.4 What Breeze draws, as read

Read by the session on 2026-11-14 in `kstyle/breezehelper.cpp` at `f0b1d753`,
`Helper::renderCheckBox` and `Helper::renderRadioButton` (the rationale's D6): while the
pointer is over the control, each strokes the indicator's outline in the focus colour
(the lighter neutral text colour where the neutral highlight is set), with no condition
on the check state. Where that stroke lies was read on 2026-11-15, in the files fetched
at `f0b1d753` (B.3 records the fetch): it is not a ring inside the indicator but the
indicator's own outline, stroked again over the frame. The check box's frame is drawn
on `rect.adjust(2, 2, -2, -2)` moved half a pen width inward by `strokedRect`, with the
pen `PenWidth::Frame` and the radius `CheckBox_Radius` (`breezehelper.cpp:844-845`,
`:858`, `:860-863`, in `renderCheckBoxBackground`); its hover stroke on
`rect.adjust(2, 2, -2, -2).adjusted(0.5, 0.5, -0.5, -0.5)`, the same rectangle, with the
same pen width and radius and no brush (`:895`, `:904`, `:907`, in `renderCheckBox`,
inside `if (mouseOver)` at `:897`). The radio's frame is drawn on
`rect.adjust(2, 2, -2, -2).adjust(0.5, …)` (`:996-997`, `:1010`, `:1013`), its hover
stroke on `rect.adjust(1, 1, -1, -1).adjusted(1, 1, -1, -1).adjusted(0.5, …)`, the same
rectangle (`:1041`, `:1053-1054`). The style hands both functions the indicator's one
rectangle (`breezestyle.cpp:4935`, `:4937` for the check box, `:4972-4974` for the
radio) and the hover flag `enabled && State_MouseOver` (`:4913`, `:4955`), and the
hover stroke is painted after the frame, faded in by the hover animation
(`painter->setOpacity`, `breezehelper.cpp:900-901`, `:1046-1047`). So under the pointer
the indicator's border shows `focusColor()`, checked or not: exactly the border colour
the connectors recolour (A.8-A.10).

So the radio button draws the same hover outline, and it shows on a checked control:
the one field holds for both controls and both states, and every connector below draws
it on an enabled box or radio, checked or not. The backlog's earlier reading at tree
`be6e137e` (`docs/todo.md:75-83`, "whatever the check state") agrees.

### A.5 Presets

**kde-breeze**, `native-theme/src/presets/kde-breeze.toml`. Comment form as v0.7.1's, with
no line number (the batch's S6: a new citation names the document, here with its
section): a citation's first line starts `# platform-facts.md (§x.y):`; a key the preset
leaves out is named with `# K not stated: platform-facts.md (§x.y), ...`. This is the
form `scripts/check_preset_citations.py` reads (its docstring). The preset's existing
`platform-facts.md:<line>` comments are left as they are.

**Light.** `[light.checkbox]`: replace `:204` (`hover_background = "#93cee9"`, uncited)
with

```toml
# hover_background not stated: platform-facts.md (§2.5), Breeze adds no fill under the pointer: the box keeps
# palette.button() (breezehelper.cpp:862 at breeze f0b1d75)
# platform-facts.md (§2.5): focusColor(), [Colors:View] DecorationFocus 61,174,233 (BreezeLight.colors:116,
# breeze v6.7.5): under the pointer the box's or radio's outline is stroked in it, checked or not (breezehelper.cpp:895-907, :1041-1054 at breeze f0b1d75)
hover_border_color = "#3daee9"
```

`[light.combo_box]`: replace `:533` (`hover_background = "#93cee9"`, uncited) with

```toml
# hover_background not stated: platform-facts.md (§2.24), ← background_color: a non-editable combo box's hover
# turns only its pen Highlight (breezehelper.cpp:710-712 at breeze f0b1d75)
# platform-facts.md (§2.24): [Colors:Selection] BackgroundNormal 61,174,233 (BreezeLight.colors:87, breeze v6.7.5),
# Highlight, the pen of a hovered button frame (breezehelper.cpp:710-712 at breeze f0b1d75)
hover_border_color = "#3daee9"
```

**Dark.** The same at `[dark.checkbox]` `:790` with `BreezeDark.colors:117`, and at
`[dark.combo_box]` `:1121` with `BreezeDark.colors:88`; both values are the same,
`#3daee9`.

Each edit adds four lines; `kde-breeze.toml` line citations elsewhere (none gated) move with them and
are left as they are (S6); the backlog cites the preset by key path (B.11, A.12).

**What the preset then gives, and what each connector draws** (light / dark):

| Leaf | Before | After |
|---|---|---|
| `checkbox.hover_background` | #93cee9 / #93cee9 | `None` (no inheritance, A.2) |
| `checkbox.hover_border_color` | — | #3daee9 / #3daee9 |
| `combo_box.hover_background` | #93cee9 / #93cee9 | `None` |
| `combo_box.hover_border_color` | — | #3daee9 / #3daee9 |

- iced: a hovered unchecked box or radio is its fill at rest, #fcfcfc / #292c30
  (`CI/styles.rs:776-782`), framed #3daee9 where it was #c6c8c9 / #4c4e51; a hovered or
  open-and-hovered drop-down keeps #fcfcfc / #292c30 (`:1200-1203`) and is framed
  #3daee9 where it was `combo_box.border.color`.
- egui: the «checkbox» role's hovered and active `bg_fill` copy the idle fill
  (`CE/style/states.rs:128-135`), their `bg_stroke` colour is #3daee9; the
  «combo_box» role's the same with `weak_bg_fill`.
- gpui: the connector's `widgets::Checkbox` and `widgets::Radio` keep their fill under
  the pointer (`CG/widgets/checkbox.rs:133-136` gives the fill at rest) and take the
  hover border (A.10); gpui-component's `Select` and `Combobox` draw no hover, as today
  (`GC/select.rs:562-583`; `GC/combobox.rs:983-1000`); the gpui showcase draws the
  outline over its triggers (A.10).

**Every bundled platform preset, by D6's rule** (decision 8), from the cells read today:
the `hover_background` rows beside the new fields (`PF:1244`, `:1625`), the rows of their
borders (`PF:1230`, `:1614`) and the new rows of A.3.

| Preset | `checkbox.hover_border_color` | `combo_box.hover_border_color` |
|---|---|---|
| kde-breeze | **stated**, #3daee9: the cell gives Breeze's hover outline with its source, for a checked and an unchecked box and the radio (A.4) | **stated**, #3daee9: the cell gives the hovered button frame's pen with its source |
| material-3 | unset: the facts give the value, and it is not one colour: an unchecked box's outline turns `on-surface` under the pointer, a checked box has no outline (`PF:1230`: `selected-outline-width` 0, `v0_192/_md-comp-checkbox.scss:80`). The field is drawn on a checked box too (A.4), where a stated colour would draw an outline M3 does not; nothing is to be read | **stated**, #1d1b20 / #e6e0e9: the cell gives `text-field-hover-outline-color` = `on-surface` with its source (`PF:1625`, `v0_192/_md-comp-outlined-select.scss:168-169`); the colours are `on-surface`'s as `PF:1218` gives them and as material-3 states them for `input.hover_border_color` (`material-3.toml:144`, `:663`) |
| adwaita | unset: the cell gives GNOME's ring for an unchecked box (`PF:1244`) and for a checked one a 10 % layer over the fill, nothing on its outline; `border.color` (`PF:1230`) says only "check border". The checked box's outline under the pointer is to be read, and the field covers both | unset, which is GNOME's value at normal contrast: the trigger draws no border and shows hover by its fill (`PF:1614`, `:1625`); `prefers-contrast: more` is not in the cell, and the model states no high-contrast variant of a field |
| windows-11 | unset: the cells give WinUI's stroke at rest (`PF:1230`) and its fills under the pointer (`PF:1244`), no stroke under the pointer | unset: the cell gives the fill (`PF:1625`), no `ComboBoxBorderBrushPointerOver` |
| macos-monterey, macos-tahoe | unset: ❓ in the cell, AppKit publishes no hover state for a check box and the OS probe renders it at rest only (`PF:1244`; `PF:1230` gives the edge at rest) | unset: the same for a pop-up button (`PF:1625`; `PF:1614` inherits the default border) |
| ios-13 | unset: `PF` has no iOS column | unset: the same |

The ten colour-scheme presets have no platform and state nothing. What the unset cells
lack goes into one backlog entry (A.12).

**material-3.** After `arrow_icon_size_px` in `[light.combo_box]` (`material-3.toml:474`)
add

```toml
# platform-facts.md (§2.24): text-field-hover-outline-color = on-surface (_md-comp-outlined-select.scss:168-169)
hover_border_color = "#1d1b20"
```

and the same after `:993` in `[dark.combo_box]` with `"#e6e0e9"`. The comment's form is
material-3's own for the same token of the text field (`:143`). The citation is the new
row of A.3, whose Material cell states the token and the two colours.

What a stated value on material-3 changes (light / dark):

| Where | Before | After |
|---|---|---|
| iced `styles::pick_list`, `Hovered` and `Opened { is_hovered: true }` border | the resolved `combo_box.border.color`, which material-3 inherits from `defaults.border.color` (#79747e / #938f99 stated, `material-3.toml:59`, `:578`) | #1d1b20 / #e6e0e9 |
| egui «combo_box» role, `hovered` and `active` `bg_stroke.color` (and the widgets crate's `ComboBox`, which takes the cell) | the same inherited border colour | #1d1b20 / #e6e0e9 |
| gpui-component's `Select` and `Combobox` | no hover style | unchanged (no receiver, A.10) |
| the gpui showcase's drop-downs (`demo::combo_surface`) | no edge under the pointer | the overlay's edge in #1d1b20 / #e6e0e9, `combo_box.border.line_width` wide (material-3 inherits 1, `:62`, `:581`) |
| the fill under the pointer | material-3 states no `combo_box.hover_background`; unchanged | unchanged |

It also reaches: the `presets.rs` test of A.7 (`"kde-breeze" | "material-3"` for
the combo box); the regenerated `contract-leaves.toml`, where `material-3/light` and
`/dark` join kde-breeze in the `except` lists of `combo_box.border.color`'s hovered
moves (A.8); the iced test of A.8, which also asserts that material-3's hovered
`styles::pick_list` border is #1d1b20 in light and #e6e0e9 in dark; the
gpui showcase test of A.10 for the drop-down, run under material-3 as well as
kde-breeze; and the gpui showcase's Widget Info, which claims the edge where the theme
states it. The CHANGELOG (A.12) names material-3.

### A.6 KDE reader

`native-theme/src/kde/colors.rs`, `populate_colors` (`:476-760`). No new helper, no
ported code.

- Checkbox block (`:574-591`): after the `unchecked_border_color` assignment add
  `variant.checkbox.hover_border_color = view_focus;` and extend the block's comment
  with "; under the pointer the indicator's outline is stroked in `focusColor()`,
  `[Colors:View] DecorationFocus` (breezehelper.cpp:895-907 at breeze f0b1d75)".
- Combo box block (`:710-714`): add `variant.combo_box.hover_border_color = selection_bg;`
  (`view_focus` and `selection_bg` are both bound at the top of `populate_colors`); its comment becomes "Combo box
  (docs/platform-facts.md §2.24): the Button set, and under the pointer the pen
  `Highlight`, `[Colors:Selection] BackgroundNormal` (breezehelper.cpp:710-712 at breeze
  f0b1d75)."
- `populate_colors`' doc (`:459-470`): the sentence "The menu's hover comes from
  `[Colors:View] DecorationFocus`" becomes "The menu's hover and the check box's hover
  outline come from `[Colors:View] DecorationFocus`"; the list of blends on the accent is
  unchanged (the combo box's pen is the selection colour as it is, not a blend), and a
  sentence is added: "The combo box's hover pen is `[Colors:Selection] BackgroundNormal`
  as it is."

Breeze values (from `BreezeLight.colors:116`, `:87`, `BreezeDark.colors:117`, `:88`,
as kde-breeze cites them): both fields #3daee9 under both schemes; they equal A.5's.

**Tests** (`kde/colors.rs` test module, `kde/mod.rs`):

The Breeze values under both schemes are held by
`the_derived_colours_are_the_presets_under_breeze` (item 4), so no test of its own
asserts them.

1. `a_custom_scheme_moves_the_hover_borders` (new, `kde/colors.rs`): the scheme of
   `a_custom_scheme_moves_the_menu_scrollbar_and_list_colours` (`:1492`) with
   `DecorationFocus=20,200,90` and `[Colors:Selection] BackgroundNormal=200,40,90`, each
   replaced by key as that test does: the check box's is (20,200,90), the combo box's
   (200,40,90), each `assert_ne!` against the Breeze Light read. It is the one check of
   the exact value on a scheme that is not Breeze's, where both Breeze schemes give
   both keys the same #3daee9.
2. `test_missing_groups_leave_the_new_fields_to_the_preset` (`:1589`): with only
   `[Colors:Selection]`, `checkbox.hover_border_color` is `None` and
   `combo_box.hover_border_color` `Some`; without `[Colors:Selection]` the latter is
   `None`.
3. `test_full_mapping_completeness` (`:1714`): both are `Some` under `BREEZE_DARK`.
4. `kde/mod.rs`: `derived_leaves` and `resolved_leaves` (`:1248`, `:1309`) append
   `("checkbox.hover_border_color", …)` and `("combo_box.hover_border_color", …)`
   (34 → 36, the doc's "thirty-four" → "thirty-six"). `the_derived_colours_are_the_presets_under_breeze` (`:1398`) then
   checks A.5 against the reader. `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle`
   (`:1538`): the focus list appends `"checkbox.hover_border_color"`; its doc ends "…a
   focused field's frame, the menu's hover and the check box's hover outline".
   `an_accent_moves_the_on_track_…` (`:1420`): the list appends
   `"combo_box.hover_border_color"`; its doc adds "and the combo box's hover pen".

### A.7 Core tests outside the reader

`native-theme/src/presets.rs`: `platform_presets_no_derived_fields` (`:407`) loops over
five presets, without material-3 and ios-13 (`:409-415`), so an assertion there could
not check material-3's combo box. A new test beside it,
`hover_outlines_are_stated_where_the_facts_give_them`, loops over the seven platform
presets (`kde-breeze`, `adwaita`, `windows-11`, `macos-monterey`, `macos-tahoe`,
`material-3`, `ios-13`), both variants, and asserts `checkbox.hover_border_color` is
stated exactly by `kde-breeze` (`matches!(*name, "kde-breeze")`) and
`combo_box.hover_border_color` exactly by `kde-breeze` and `material-3`
(`matches!(*name, "kde-breeze" | "material-3")`); doc comment "The hover outlines of the
check box and the combo box: stated where docs/platform-facts.md gives the platform's
value as one colour for every state the field covers (§2.5, §2.24)", messages "stated
only where platform-facts gives it".

### A.8 iced

`connectors/native-theme-iced/src/styles.rs`.

- `CheckFills` (`:762-797`) gains `hovered_border: Option<Color>` from
  `c.hover_border_color.map(to_color)`. `at(is_checked, is_hovered)` returns per state:

  | | fill | border |
  |---|---|---|
  | unchecked, at rest | `unchecked` | `unchecked_border` |
  | unchecked, hovered | `hovered_unchecked` (unchanged) | `hovered_border`, else `unchecked_border` |
  | checked, at rest | `checked` | `checked_border` |
  | checked, hovered | `checked` | `hovered_border`, else `checked_border` (A.4) |

  `checkbox_over`'s `Disabled` arm (`:845-858`) uses `at(is_checked, false)` as today, so
  a disabled box shows no hover border. `radio` (`:903-928`) reads the same `at`, so the
  radio follows (`radio::Status` has no disabled state, `:884-886`).
- `CheckFills`' two comments that the new field makes false are rewritten (the lines of
  `styles.rs` move anyway). In `new`, the comment at `:773-775`
  ("The three soft options are the platform saying the box looks no different: …;
  under the pointer, the fill it has at rest.") becomes:

  ```rust
          // The four soft options are the platform saying the box looks no
          // different: unchecked, it has the checkbox's own fill and border;
          // under the pointer, the fill and the border it has at rest.
  ```

  `at`'s doc (`:788-789`, "… The model states no hover for a checked box.") becomes:

  ```rust
      /// The fill and the border colour of a box that is checked or not, under
      /// the pointer or not. Under the pointer an unchecked box takes the hover
      /// fill and a checked one keeps its own, and either takes the hover
      /// border where the theme states one.
  ```

- The doc comments: in `checkbox`'s, the sentence of `:716-718` ("The model states no
  hover for a checked one, and a state the platform does not state has no distinct
  appearance, so a hovered checked box is exactly the checked box."), which the new
  field makes false, becomes, rewrapped up to the unchanged "`.disabled_background`
  replaces the fill, …": "The model states no hover fill for a checked one, and a state
  the platform does not state has no distinct appearance, so a hovered checked box
  keeps the checked box's fill. Under the pointer the border is
  `.hover_border_color` where the theme states one, checked or not; `None` keeps the
  border at rest." In `radio`'s (`:873-901`), the sentence of `:878-884`
  whose end, "so a hovered selected radio is exactly the selected radio, and where the
  theme leaves it unset a hovered unselected radio is the circle at rest", the new
  field makes false, becomes, rewrapped, up to the unchanged "`radio::Status` has no
  disabled variant …":

  ```rust
  /// The model states one `CheckboxTheme` for both controls
  /// (`widgets/mod.rs:176-183`), so this reads exactly the sources [`checkbox`]
  /// reads, with `is_selected` where the checkbox has `is_checked` and the check
  /// mark serving as the dot -- `checkbox.hover_background` included, which is
  /// the hover fill of the *unselected* circle only, and
  /// `checkbox.hover_border_color`, which frames a hovered circle, selected or
  /// not, where the theme states it: a hovered selected radio is the selected
  /// radio in that border, and where the theme leaves both unset a hovered
  /// radio is the circle at rest.
  ```
- `pick_list` (`:1192-1231`): `let hovered_border = c.hover_border_color.map_or(border.color, to_color);`
  and the closure's `Style` takes `border: Border { color: hovered_border, ..border }` in
  `Status::Hovered` and `Status::Opened { is_hovered: true }`, `border` otherwise. Doc
  (`:1173-1178`): "…its border is `combo_box.border.*`, its colour
  `.hover_border_color` under the pointer where the theme states one." The theme
  picker of the showcase (`showcase-iced.rs:5944-5976`) maps its statuses onto
  `pick_list`'s and follows unchanged.

**Contract** (`CI/contract/rows.rs`, `CI/contract.rs`):

- `native_checkbox_border` (`rows.rs:997-1009`): `Hovered { is_checked }` returns
  `r.checkbox.hover_border_color` where stated, checked or not, else
  `native_checkbox_outline(r, is_checked)`. `native_radio_border`
  (`:1090-1097`) the same for `Hovered { is_selected }`.
- `PICK_LIST_BORDER_ROWS` (`:1494-1503`): `native` becomes
  `|r, s| NativeBorder { color: <hover colour in Hovered and Opened { is_hovered: true }, else combo_box.border.color>, … }`.
- A new test beside `a_cleared_widget_soft_option_copies_the_base_state_value`
  (`contract.rs:4095`), `a_cleared_hover_border_copies_the_border_at_rest`: on
  `kde-breeze` light, assert both fields are `Some` ("this preset no longer states the
  hover borders, so clearing them proves nothing"), set both to `None`, and check
  `styles::checkbox` `Hovered { is_checked: false }` border colour =
  `unchecked_border_color.unwrap_or(border.color)`, `styles::radio` `Hovered { is_selected: false }`
  `border_color` the same, `styles::pick_list` `Hovered` and `Opened { is_hovered: true }`
  border colour = `combo_box.border.color`, and the checked statuses on a theme whose
  `checkbox.border.color` is replaced by a sentinel, a colour the test first asserts
  differs from `checkbox.hover_border_color` and from every other `checkbox.*` colour
  leaf: with the hover border stated, `Hovered { is_checked: true }` and
  `Hovered { is_selected: true }` give the hover border; with it cleared, the sentinel.
  The windows-11 test's code is left as it is: windows-11 states
  neither field. Its comment counts the soft options the test covers: the ones it
  clears and reads back through its `read_sites` (`contract.rs:4172-4268`), twelve at
  `0fa28027` and a thirteenth, `tab.hover_background`, under `iced_aw`. The two new
  fields are read by the widget functions (`styles::checkbox` reads
  `checkbox.hover_border_color` through `CheckFills`, which `styles::radio` shares;
  `styles::pick_list` reads `combo_box.hover_border_color`) but covered by the new test
  above, not by this one, so the count stays twelve and the comment says where they
  are. Its first sentence (`:4096-4099`, up to "one `scrollbar.*`.") becomes, and the
  sentence after it ("A bundled preset rarely leaves them `None` …") follows:

  ```rust
      // The twelve soft options this test clears, of those the widget functions
      // read: four `checkbox.*` (three of which `styles::radio` reads again,
      // through the fills it shares with `styles::checkbox_over`), five
      // `switch.*`, one `combo_box.*`, one `slider.*` and one `scrollbar.*`.
      // The check box's and the combo box's hover borders are cleared by
      // `a_cleared_hover_border_copies_the_border_at_rest`, on a preset that
      // states them.
  ```

  The rest of the comment, "The thirteenth" (`:4122-4123`) included, stays, and the
  count stays twelve.
- `contract-leaves.toml` is generated: `cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`
  (`contract.rs:5539`). Expected differences: `[leaves."checkbox.hover_border_color"]`
  moves `styles::checkbox.border` under `Hovered { is_checked: false }` (and `true`) and
  `styles::radio.border_color` under `Hovered { is_selected: false }` (and `true`);
  `[leaves."combo_box.hover_border_color"]` moves `styles::pick_list.border` under
  `Hovered` and `Opened { is_hovered: true }`; the `[rows."styles::checkbox.border"]`,
  `[rows."styles::radio.border_color"]` and `[rows."styles::pick_list.border"]` tables
  list the new leaf; `kde-breeze/light` and `kde-breeze/dark` leave the `except` lists
  under which `checkbox.unchecked_background` and `combo_box.background_color` do not
  move the hovered fill (`contract-leaves.toml:196`, `:198`, `:210`), and appear in new `except`
  lists of `checkbox.unchecked_border_color`, `checkbox.border.color` and
  `combo_box.border.color` for the hovered statuses, where `material-3/light` and
  `material-3/dark` join them for `combo_box.border.color` (A.5). Any other difference
  is a finding.
  Gate: `every_colour_leaf_reaches_a_row_or_is_named` (`:5578`, `--features iced_aw`).

### A.9 egui

`connectors/native-theme-egui/src/style/roles.rs`:

- `checkbox` (`:373-453`): after `write_states` (`:381-406`), where
  `k.hover_border_color` is `Some`, write it into `s.visuals.widgets.hovered.bg_stroke.color`
  and `.active.bg_stroke.color`, as `input_role` writes its hover border (`:349-352`);
  `None` leaves the copies `write_states` made of the unchecked border. The field goes
  into `active` too (decided): `active` is egui's state of a widget the pointer is
  pressed on, and also of one that has the keyboard focus or was just clicked
  (`Widgets::style`, `EG/style.rs:1273-1285`: `is_pointer_button_down_on() ||
  has_focus() || clicked()`), and a pressed box is under the pointer, where Breeze still
  shows the outline. The cost: a check box or a radio reached by Tab also shows the
  outline, which Breeze draws under the pointer only (`breezestyle.cpp:4913`, `:4955`:
  `enabled && State_MouseOver`). The code comment beside the write says so ("`active` is
  also egui's keyboard-focused state, so a box reached by Tab shows the hover outline,
  which Breeze draws under the pointer only"), and so do the mapping rows' `when`
  (below).
- `combo_box` (`:903-974`): the same after its `write_states` (`:911-935`). It has no
  such cost: Breeze's combo box, a button frame, turns its pen `Highlight` on keyboard
  focus as under the pointer (`breezehelper.cpp:710`, `(hovered || visualFocus || down)
  && enabled`). The toolbar's buttons have it (B.7).
- `CE/style/variants.rs`, `selected_cell` (`:15-44`) (A.4: the hover outline shows on a checked
  box): the checked box's `hovered` and `active` entries take `hover_border_color` where stated
  instead of `checkbox.border.color` (`:32-42` writes `outline` into all five); the
  comment at `:27-29` gains "and its hover outline, where stated, under the pointer".
- `connectors/native-theme-egui-widgets/src/combo_box.rs:18-22`: "…whose cell carries
  `combo_box.*` — its fill, hover fill, hover outline, font, border, …". Its radio
  (`radio_button.rs`) takes the «checkbox» cell and needs no edit.

**Mapping** (`connectors/native-theme-egui/mapping.toml`; `src/mapping.md` is generated
by `cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`,
`src/mapping_doc.rs:357`):

- `["checkbox.hover_border_color"]` after `["checkbox.hover_background"]` (`:1379`):
  `verdict = "scoped"`, sinks `visuals.widgets.hovered.bg_stroke.color` and
  `visuals.widgets.active.bg_stroke.color`, `scope = "checkbox"`, `when = "stated; None
  copies the unchecked border, §6.4"`, and the same two with `variant = "selected"`,
  `when = "stated; None keeps checkbox.border.color, §6.4"`; each `active` sink's `when`
  ends "; egui's active is also the keyboard-focused state, so a box reached by Tab
  shows it, where Breeze draws it under the pointer only".
- `["combo_box.hover_border_color"]` after `["combo_box.hover_background"]` (`:1844`):
  the two sinks with `scope = "combo_box"`, `when = "stated; None copies
  combo_box.border.color, §6.4; an open combo box is egui's open state and keeps the
  border at rest"`.
- The hovered and active sinks of the borders at rest take a `when` in the file's form
  (`input.border.color`'s hovered and active sinks read
  `when = "input.hover_border_color is None, §6.4"`, `mapping.toml:1723-1724`; a second
  condition joins with "and", as in "checkbox.unchecked_background is None and the hover
  layer is translucent or None, §6.1, §6.4", `:1299-1300`):
  - `["checkbox.border.color"]`, the checked box's (`variant = "selected"`, `:1447-1448`,
    no `when` today): `when = "checkbox.hover_border_color is None, §6.4"`;
  - `["checkbox.border.color"]`, the unchecked box's (`:1452-1453`, today
    `when = "checkbox.unchecked_border_color is None, §6.4"`):
    `when = "checkbox.unchecked_border_color is None and checkbox.hover_border_color is None, §6.4"`;
  - `["checkbox.unchecked_border_color"]` (`:1409-1410`, today
    `when = "stated; None copies checkbox.border.color, §6.4"`):
    `when = "stated and checkbox.hover_border_color is None; None copies checkbox.border.color, §6.4"`;
  - `["combo_box.border.color"]` (`:1894-1895`, no `when` today):
    `when = "combo_box.hover_border_color is None, §6.4"`.
- Counts. On the tree (`f02c3ae9`): 519 leaves, message "515 ResolvedTheme leaves + 4
  layout leaves (§5.7)"; Buttons 115, `[13, 46, 31, 25]`; Inputs 87, `[4, 39, 32, 12]`.
  Step 4 adds one SCOPED leaf to each: Buttons 116, `("buttons", [13, 47, 31, 25])`;
  Inputs 88, `("inputs", [4, 40, 32, 12])`, the comment above it gaining ", and
  `combo_box.hover_border_color`"; 521 leaves, "517 ResolvedTheme leaves + 4 layout
  leaves (§5.7)". The generator writes `mapping.md`. Both steps' egui parts land in one
  unit (Part C), which writes the final figures: 525 leaves, "521 ResolvedTheme leaves
  + 4 layout leaves (§5.7)", the totals row 525 = 48 + 217 + 169 + 91. The sub-tag
  counts of `verdict_totals_match_the_specification` (`src/mapping_tests.rs:486-495`) do
  not change, the new rows having no `sub_tag`. Gates: `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`,
  `verdict_totals_match_the_specification`, `mapping_md_is_current`. A `grep` of the egui
  crate and its showcase for the two leaves' neighbours (`"checkbox.hover_background"`,
  `"combo_box.hover_background"`) and for `519` finds no other hard-coded list or count
  these rows change (`CE/contract.rs:597` and `CE/style/roles.rs:390`, `:919` name the
  hover fills in their own formulas; the showcase names neither).

**Contract** (`connectors/native-theme-egui/src/contract.rs`, `expected`, `:528-…`), new
arms before the general checkbox stroke arm (`:715-718`), each in that arm's form,
`c32(chain(c, &[…])?)`, as the input's hover arm beside it (`:719-724`) is written:
`(Some("checkbox"), "normal", None, p) if hover_or_press && p.ends_with(".bg_stroke.color")`
→ `c32(chain(c, &["checkbox.hover_border_color", "checkbox.unchecked_border_color", "checkbox.border.color"])?)`;
`(Some("checkbox"), "selected", None, p)` with the same guard →
`c32(chain(c, &["checkbox.hover_border_color", "checkbox.border.color"])?)`;
`(Some("combo_box"), "normal", None, p)` with the same guard →
`c32(chain(c, &["combo_box.hover_border_color", "combo_box.border.color"])?)`.
Gates: `t10_every_sink_holds_the_leafs_value` (`:1074`),
`t10c_every_undeclared_field_holds_its_inherited_value` (`:1343`).

**Tests:** `a_none_soft_option_copies_its_fallback_leaf` (`CE/style/derived.rs:963`):
it also replaces `t.checkbox.border.color` by a sentinel (asserted as A.8's new test
asserts it), clears both fields, and asserts the «checkbox» hovered and active stroke
colours are the sentinel (the unchecked border's fallback, which the test's own clearing
of `unchecked_border_color` makes `border.color`); the expected values are read from the
modified theme; and the «combo_box» hovered stroke colour is `combo_box.border.color`;
`checkbox_cell_writes_its_rows` (`roles.rs:1557`) and
`combo_box_and_segmented_control_cells_write_their_rows` (`:2186`) assert the border
colour as the stroke of all five entries (`for e in all5(w)`, `:1586-1589`,
`:2208-2210`), which fails once kde-breeze dark, one of their `PRESETS`, states the
field: the loop keeps the border for `noninteractive`, `inactive` and `open` and, where
the preset states the field, asserts it for `hovered` and `active` instead.
`the_checkbox_selected_cell_fills_the_box_and_outlines_it` (`CE/style/variants.rs:330-360`,
kde-breeze light) asserts `checkbox.border.color` as the stroke of all five entries of the
checked cell, and would pass after the change only because kde-breeze's
`checkbox.border.color` is #3daee9 too, as its hover outline is (`kde-breeze.toml:215`),
so an assertion of the hover outline there could not fail either. The test takes the resolved
theme as `let mut t` (as `CE/style/base.rs:724` does), replaces
`t.checkbox.hover_border_color` by a sentinel, a colour it first asserts no other
`checkbox.*` colour leaf of `t` holds, and builds the cells from that theme (`build(&t)`,
`:220`); it then asserts the sentinel for `hovered` and `active` and
`checkbox.border.color` for the other three, and its doc says so, as A.10's gpui test
does for its checked box.

### A.10 gpui

**The connector's own check box and radio** (`CG/widgets/checkbox.rs`, feature
`widgets`):

- `CheckboxLook` (`:30-76`, `#[non_exhaustive]`) gains, after `hover_fill`:

  ```rust
      /// The indicator's border colour while the pointer is over the control:
      /// `hover_border_color`, for an enabled control, checked or not; `None`
      /// where the theme states none, and the border at rest stays.
      pub hover_border: Option<Hsla>,
  ```

  `hover_fill`'s doc (`:46-49`) ends "-- the model states no hover for a checked one.",
  which the new field makes false; its last line becomes, one line for one,
  "    /// the model states no hover fill for a checked one; `hover_border` frames both."
- `sized` (`:116-177`): `let hover_border = c.hover_border_color.filter(|_| !disabled).map(color);`
  added after `let opacity = …` (`:147`), below the claimed lines; the field goes into
  the struct literal after `hover_fill`. The struct's new lines above still move the
  claimed lines (the item on them below).
- `faded_over` (`:188-202`) keeps it as it is (a disabled look has none).
- The render (`:485-500`) has one hover style today, `when_some(look.hover_fill, …)`
  with its `group_hover(HOVER_GROUP, …)` setting the fill (`:498-500`). gpui keeps one
  group-hover style per element, a second call replacing the first
  (`GP/elements/div.rs:876-886`), so a second `when_some` for the border would drop the
  fill: the `when_some` becomes one
  `when(look.hover_fill.is_some() || look.hover_border.is_some(), …)` whose single
  `group_hover(HOVER_GROUP, …)` sets `bg(fill)` where `hover_fill` is `Some` and
  `border_color(edge)` where `hover_border` is `Some`.
- The struct's new doc lines (four, above line 123) and the `CheckboxLook::of` doc
  (`:83-94`, one clause: "a stated hover border recolours the border under the pointer")
  move every line from 123 on: the claims at `showcase-gpui/info/inputs.rs:961-1157` and
  `tests.rs:15858-15918` that cite `native-theme-gpui/widgets/checkbox.rs:N` are
  re-pointed by the number of lines added above N, in the same commit.
- Tests (`CG/widgets/tests.rs`): `a_checkbox_paints_checkbox_theme` (`:53`) asserts
  `idle.hover_border == k.hover_border_color.map(c)` and the same for the checked look
  (A.4), `disabled.hover_border == None`; a new
  `an_unset_hover_border_keeps_the_border` beside `an_unset_checkbox_hover_is_the_box_at_rest`
  (`:116`): kde-breeze light with the field cleared gives `hover_border == None` for
  `of` and `radio`.

**gpui-component's `Select` and `Combobox`.** No receiver: the trigger has no hover style
(`GC/select.rs:562-583`; `GC/combobox.rs:983-1000`, `render_trigger_container`) and a
`StyleRefinement` carries no hover state. `CG/contract.rs` `UNREAD_COLOURS` gains, after
the `combo_box.hover_background` entry (`:1195-1201`):
`UnreadColour { leaf: "combo_box.hover_border_color", why: Unread::NoReceiver("neither trigger has a hover style (gpui-component 0.10.0 `select.rs:578-597`, `combobox.rs:1025-1042`), and a `StyleRefinement` has no hover state; the showcase draws it over the trigger") }`;
the `combo_box.hover_background` entry beside it keeps its citations as they are.
It is not a `NO_RECEIVER` entry: that table requires every preset to state the colour
(`:1451-1497`). `checkbox.hover_border_color` reaches a sink through `CheckboxLook`'s
`Debug` in the walk (`:2843-2860`). Gate: `every_colour_leaf_reaches_a_sink_or_is_named`
(`:2933`, `--features widgets`).

**The showcase's drop-downs** (`examples/showcase-gpui/demo.rs`):

- `combo_surface` (`:4483-4512`) today lays `combo_box.hover_background` over the fill
  under the pointer. Its look gains `c.hover_border_color.map(info::stated)` and
  `px(c.border.line_width.max(0.0))`. Where both the colour and a width above 0 are
  there, the surface is a group (`.group(COMBO_HOVER_GROUP)`, a new `const` beside the
  function) and gets, after the trigger, a child
  `div().absolute().inset_0().rounded(radius).border(width).border_color(<transparent>).group_hover(COMBO_HOVER_GROUP, move |s| s.border_color(edge)).debug_selector(|| "combo-hover-edge".into())`,
  `<transparent>` being upstream's `cx.theme().transparent`, as `combo_fill` uses it
  (`:4471-4481`). The doc paragraph (`:4483-4491`) gains: "Where the theme states
  `combo_box.hover_border_color`, an edge in it is laid over the trigger's under the
  pointer: the trigger is no `InteractiveElement` either."
- Widget Info (`info/inputs.rs:1582-1611`, `combo_fill`): an
  `edge_claim("hovered edge, over the trigger's", "hover_border_color", r.combo_box.hover_border_color, Some(r.combo_box.border.line_width), "showcase")`
  after the hover layer's: the claim gate `every_edge_colour_is_claimed_through_edge_claim`
  (`CG/showcase.rs:2990`) classes any `_border_color` value and any Look field named
  `*_border` as an edge, which must be claimed through `edge_claim(role, field, colour,
  width, cited_at)` (`info/mod.rs:82-98`) with the width the edge is drawn at, here the
  overlay's, `combo_box.border.line_width`; the text input's hover edge is claimed so
  (`info/inputs.rs:56-62`). The `fill` config text (`:1611`) ends "…; and, where the
  theme states it, combo_box.hover_border_color as an edge the showcase lays over the
  trigger's under the pointer, for the same reason."
- `info/leaves.rs`, after `("", "combo_box.hover_background", …)` (`:626-629`):
  `("", "combo_box.hover_border_color", "gpui: an edge laid over the trigger under the pointer (demo::combo_surface); upstream's trigger has no hover style")`;
  after `("", "checkbox.hover_background", …)` (`:509-513`):
  `("", "checkbox.hover_border_color", "gpui: the indicator's edge under the pointer (CheckboxLook); unset, the edge at rest")`.
  These two lines land in the same commit as A.11's element edits: the connector's own
  tests run `every_how_entry_is_shown` (`CG/showcase.rs:3893`, the module included by
  `CG/lib.rs:243-244`), which fails on a line no element's leaf reaches, and the
  showcase's `every_listed_leaf_has_a_route` (`showcase-gpui/tests.rs:15198`) fails on a
  listed leaf without one (C.1).
- `info/inputs.rs`: one claim, in `native_indicator` only, after the hover fill's claim:
  `edge_claim("hovered edge", "hover_border_color", look.hover_border, Some(f32::from(look.border_width)), "native-theme-gpui/widgets/checkbox.rs:<line of the new let>")`.
  The check box takes it in both check states; the radio column takes it once, through
  its call of `native_indicator` for the selected radio (a selected and an unselected
  radio's `hover_border` are one colour by construction). `native_radio_column` gains no
  claim of its own. `edge_claim` claims nothing where the colour is unset or the width
  is 0.
- Tests (`examples/showcase-gpui/tests.rs`): `checkbox_leaves` returns `[Leaf; 6]`,
  `[fill, hover, hover_edge, border, mark, label]`, the new leaf
  `("hover_border_color", "native-theme-gpui/widgets/checkbox.rs:<that line>")`; its
  doc's closing reads "(fill, hover, hover edge, border, mark, label)". The doc
  (`tests.rs:15840-15851`), which cites `widgets/checkbox.rs:123-146` and nine bare
  lines in it, keeps those citations as they are (no gate reads a doc comment's lines;
  S6, C.4) and names the sixth leaf: after the border's clause it gains "the hover edge
  `hover_border_color` where the box is enabled, else none (the new `let` in
  `widgets/checkbox.rs`)". `a_theme_drawn_controls_info_prints_its_look`: the check box
  part destructures six and lists
  `("hovered edge", edge(look.hover_border, look.border_width), hover_edge)` after
  `("hover", …)`; the radio part's two destructurings become
  `let [fill, _, hover_edge, border, mark, label] = checkbox_leaves(&r.checkbox, true, false);`
  and `let [off_fill, off_hover, _, off_border, _, _] = checkbox_leaves(&r.checkbox, false, false);`,
  and its `want` list gains
  `("hovered edge", edge(selected.hover_border, selected.border_width), hover_edge)`.
  Two new tests in the pattern of
  `a_hovered_field_takes_the_hover_border` (`:9439`), under kde-breeze:
  `a_hovered_checkbox_takes_the_hover_border` (the Basic page's unchecked box: at rest
  `painted_edge` of its indicator is `unchecked_border_color`, hovered
  `hover_border_color`, the checked box hovered `hover_border_color` too; the checked
  part runs on kde-breeze with `checkbox.hover_border_color` replaced by a sentinel, a
  colour the test asserts no other `checkbox.*` leaf of the theme holds, since
  kde-breeze's own hover colour, #3daee9, is also its `checkbox.border.color`, the
  checked box's edge at rest, and that part could not fail on it) and
  `a_hovered_drop_down_takes_the_hover_border`. gpui paints no border quad whose colour
  is transparent (`GP/style.rs:744`, `is_border_visible`, `:764-768`), so at rest the
  overlay paints nothing, and `painted_edge` (`tests.rs:9485-9494`) returns the first
  bordered quad over the bounds, which is the trigger's own edge. So the test first
  moves the pointer away before it reads the rest state (a test window's pointer starts
  at the origin) and asserts that no scene quad over the trigger's bounds has
  `combo_box.hover_border_color` as its border colour; a border-only quad is painted as
  four quads, so the test counts none. Hovered, the last-painted quad whose bounds equal
  the trigger's and whose border widths are not zero has it: the bordered quad of the
  highest `order` over the trigger's bounds (`scene_quads` is not in paint order on
  GPUI Fast), which a new helper beside `painted_edge`,
  `painted_edges(cx, selector) -> Vec<Hsla>`, returns last. While the Basic page's
  drop-down is hovered, no bordered quad over the side panel's theme picker has the
  hover colour. Under kde-breeze and material-3 light.
  `the_preset_combobox_paints_its_fill_swatch_and_names_its_row_hover` (`:6801`) reads
  the fields and stays.

### A.11 Showcases: the element list, iced and egui

- `docs/showcase-elements.toml`: `"checkbox.hover_border_color"` after
  `"checkbox.hover_background"` in the elements that list it —
  `basic.checkboxes.unchecked` (`:872`), `.unchecked.indicator` (`:879`),
  `basic.radios.option_b` (`:1009`), `.option_b.indicator` (`:1016`); and, since the
  hover outline shows on a checked control (A.4), in registry order (after
  `"checkbox.checked_background"`, before `"checkbox.border.color"`), in
  `basic.checkboxes.checked` (`:897`), `.checked.indicator` (`:904`),
  `basic.radios.option_a` (`:975`) and `.option_a.indicator` (`:982`).
  `"combo_box.hover_border_color"` after `"combo_box.hover_background"` in
  `chrome.side_panel.settings.theme`, `.mode`, `.icon_theme` (`:378`, `:394`, `:410`)
  and `basic.drop_down.trigger` (`:1261`). With them, A.10's two `HOW` lines. Gates:
  `every_how_entry_is_shown` (`CG/showcase.rs:3893`, in the gpui connector's tests) and
  the gpui showcase's `every_listed_leaf_has_a_route` (`tests.rs:15198`); the iced
  showcase's `the_element_list_parses_and_every_leaf_has_a_route` (`showcase-iced.rs:24982`);
  `scripts/check_showcase_parity.py --check-list` (`check_release.sh:940-941`).
- iced, read for each element: a colour leaf is routed from `contract-leaves.toml`
  where the showcase's `drawing()` maps the element to style rows (`colour_route`,
  `showcase-iced.rs:7563-7600`); it does for every element above: the check boxes
  (`CHECKBOX`, statuses `is_checked: false` / `true`, `:7370-7375`), the radios
  (`styles::radio.`, `is_selected: true` / `false`, `:7379-7384`), the drop-down and the
  three side-panel pickers (`styles::pick_list.`, `:7404-7410`). The regenerated export
  (A.8) gives the new leaf its rows under the `Hovered` statuses those filters keep, so
  no showcase code changes for step 4.
- egui: Widget Info's line for a leaf is its `mapping.toml` row's route
  (`info::how_line`, `showcase-egui/info.rs:524-563`; "not reachable: mapping.toml has
  no row for it" otherwise); A.9's rows give the two leaves theirs. No showcase code
  changes.

### A.12 Documentation

- **READMEs.** The optional colours are counted, 36 today, in five places found by
  their quoted text (`README.md`: "36 colours", "five of the 36";
  `native-theme/README.md`: "every colour but 36 optional ones", "most of the 36",
  "five of the 36"): 38 after step 4, 42 after both; the documents unit writes 42.
  "five" stays: it counts the switch's outline colours (`unchecked_track_border_color`,
  `checked_track_border_color`, their two disabled forms and `thumb_border_color`), and
  neither step adds one. `connectors/native-theme-gpui/README.md`, *Theme-drawn
  widgets*, the row `Checkbox`, `Radio`, `RadioGroup`: "fills per state, border, radius"
  becomes "fills per state, border and its colour under the pointer
  (`hover_border_color`), radius" (the gpui unit).
- **CHANGELOG** (`CHANGELOG.md`). No new section. In `## [0.7.2]`, the one *Breaking
  Changes* bullet is replaced by this one, one line in the file, which names both
  steps' structs (B.11 adds no second one):

  ```markdown
  - **native-theme**: `ThemeDefaults` gains `focus_ring_keyboard_only`, `ButtonTheme` gains `hover_border_color` and `focus_border_color`, `CheckboxTheme` and `ComboBoxTheme` gain `hover_border_color`, and `ToolbarTheme` gains `button_hover_background`, `button_hover_border_color`, `button_active_background` and `button_checked_background` (see *Added*). None of them is `#[non_exhaustive]`, so a struct literal of any of them outside the crate needs the new fields or `..Default::default()`; their resolved forms are `#[non_exhaustive]` already.
  ```

  Step 4's *Added* bullet goes at the end of the section's `### Added`, B.11's five
  after it, in that order; its *Fixed* bullet at the end of the section's `### Fixed`;
  one line per bullet:

  ```markdown
  ### Added

  - **native-theme**: `checkbox.hover_border_color` and `combo_box.hover_border_color`, the outline a check box, a radio button and a combo box take under the pointer: Breeze strokes a check box's or a radio button's outline in the focus colour under the pointer, checked or not, and a combo box's in the highlight, #3daee9 in both schemes. kde-breeze and the KDE reader state them (`[Colors:View] DecorationFocus`, `[Colors:Selection] BackgroundNormal`); material-3 states the combo box's, Material's select outline under the pointer, `on-surface` (#1d1b20 / #e6e0e9); no other preset states either, where the platform's value is not in `docs/platform-facts.md` for every state the field covers. iced (`styles::checkbox`, `styles::radio`, `styles::pick_list`) and egui (the check box and combo box roles) draw them; in gpui the connector's own `widgets::Checkbox` and `widgets::Radio` do, through the new public field `CheckboxLook::hover_border` (feature `widgets`; `CheckboxLook` is `#[non_exhaustive]`, so the field breaks no caller), gpui-component's `Select` and `Combobox` have no hover style that could, and the gpui showcase draws the outline over its drop-downs.

  ### Fixed

  - **native-theme**: kde-breeze no longer states `checkbox.hover_background` and `combo_box.hover_background`, #93cee9 in both modes, a fill with no Breeze source that stood in for the outline: under the pointer a check box, a radio button and a combo box keep their fill, as Breeze's do, and take the outline above.
  ```

  The two headings are the section's own and are not written again.
- **ROADMAP** (`ROADMAP.md`). No new section. The heading of 0.7.2's section becomes
  `## v0.7.2 — egui 0.37, GPUI Kit 0.10.0, and the focus ring, the hover outlines and the toolbar buttons from issue #4`
  (no tracked file links the old heading's anchor: `git grep` of it finds nothing; the
  wording is the maintainer's to change at approval). The documents unit appends the
  two paragraphs below after the section's last (B.11's text is the second):

  ```markdown
  The model also carries the outline a check box, a radio button and a combo
  box take under the pointer, which Breeze draws in the focus colour (the indicator's
  outline, checked or not) and the highlight (a combo box's pen); kde-breeze and the KDE
  reader state both and no longer state the pale hover fills that stood in for them, and
  material-3 states its select's. The iced and egui connectors draw both; in gpui the
  connector's own check box and radio do, gpui-component's select and combo box have no
  hover style that could, and the gpui showcase draws the outline over its drop-downs.

  It also carries a toolbar button's fill under the pointer, its outline, its fill while
  pressed and its fill when checked, each painted over the toolbar. kde-breeze and the
  KDE reader state Breeze's flat button (no fill and the highlight's outline under the
  pointer, the highlight at 30 % pressed, the button text at 12.5 % checked), adwaita
  and windows-11 their platforms' fills, and the Windows reader the checked fill from
  the user's accent; a checked one is lettered in the button's checked text colour. The
  iced and egui connectors draw them, and the gpui connector gives a toolbar button
  variant and a checked button's fill, the outline included. Each showcase shows a
  checked toolbar button.
  ```
- **Backlog** (`docs/todo.md`). `scripts/check_todo_citations.py` fails on a key-path
  citation of a key the preset no longer has (`check_release.sh:977`). Of the three
  entries below that name kde-breeze's two hover fills, two cite them by key path and so
  must change in the commit that removes the keys (U1, Part C): *What Breeze draws that
  the model has no field for* (`:1782-1783`, `` `[light.checkbox] hover_background` ``,
  `` `[light.combo_box] hover_background` ``) and *Presets: values the M8 T15
  platform-facts rows disagree with* (`:1293-1294`, `:1310-1311`). The third, the `[1.0]`
  `#93cee9` entry, names them by dotted name only (`:2871-2872`), which the script does
  not read; its note lands with the other documents (U6):
  - *What Breeze draws that the model has no field for* (`:1774-1825`), as step 3
    left it: its first two sentences ("Breeze shows hover on a combo box, a check box
    and a slider handle …", "The kde-breeze hover fills that stand in for an outline …")
    read in full:

    ```markdown
    Breeze shows hover on a slider handle by recolouring the outline, never the fill
    (open-items rationale D6, which leaves the handle out; the outlines of a button, a
    check box, a radio and a combo box are in the model since 0.7.2). kde-breeze's hover
    fill of the slider handle stands in for that outline until the model has it, since
    removing it first would remove the only hover feedback: `kde-breeze.toml`
    `[light.slider] thumb_hover_color` and its dark twin, listed in *Presets: values the
    M8 T15 platform-facts rows disagree with*.
    ```

    The key-path citation keeps its one-line form, which
    `scripts/check_todo_citations.py` reads (`kde-breeze.toml:259`, `:846` at
    `0fa28027` hold the two keys). Step 4 adds a dated note at the entry's end and
    changes nothing else in the entry (B.11 edits its toolbar item); the note says
    "0.7.2: the check
    box's, the radio's and the combo box's hover outlines are `checkbox.hover_border_color` and `combo_box.hover_border_color`
    (D6 of `docs/issues/0004_kde-open-items-rationale.md`); kde-breeze's two stand-in
    fills are gone. Under the pointer Breeze strokes the indicator's outline in the
    focus colour on a radio button and on a checked control as on an unchecked check
    box, on the rectangle the frame is drawn on (`Helper::renderCheckBox`,
    `Helper::renderRadioButton`, `kstyle/breezehelper.cpp:895-907`, `:1041-1054` at
    `f0b1d753`, read 2026-11-15)."
  - The same entry's list "Breeze also draws, with no field yet:" gains, after the
    header section's item and before "Then the rest" (decision 10's
    measurement): "an open combo box's frame, as a pressed button frame's: the
    `Highlight` pen whatever the pointer does, and the fill #c3e5f6 / #2f5368 (light /
    dark; #fcfcfc / #292c30 closed, at rest)
    (`docs/evidence/breeze-offscreen/2026-11-18/combo_trigger_light.txt`,
    `combo_trigger_dark.txt`)".
  - *Presets: values the M8 T15 platform-facts rows disagree with* (`:1230-1511`): the
    kde-breeze clauses for `checkbox.hover_background` (`:1292-1295`) and
    `combo_box.hover_background` (`:1309-1311`) are removed, and the preset's file name
    moves to the first citation left: ``… `slider.thumb_hover_color` `#93cee9` (`kde-breeze.toml` `[light.slider] thumb_hover_color`, …``
    (the removed checkbox clause held the entry's only `kde-breeze.toml`, and without
    it `scripts/check_todo_citations.py` finds no preset named before the entry's
    other key paths); the *Issue #4* note (`:1326-1331`) names
    `slider.thumb_hover_color` alone.
  - the `[1.0]` kde-breeze `#93cee9` entry (`:2860-2913`): a dated note says the check
    box's and the combo box's #93cee9 are gone; the slider's and the sidebar's stay.
  - *Checkbox: a checked checkbox's hover appearance* (`:52-111`): a dated note: "KDE's
    hover outline is `checkbox.hover_border_color` since 0.7.2; this entry stays for the
    checked box's hover fill of the other platforms."
  - New, filed directly after *Presets: values the M8 T15 platform-facts rows disagree
    with*, in that entry's form, tagged `[1.0]`, its heading starting "Presets:", the
    readings D6's rule asks for (Breeze's own hover outline needs no entry: the
    connectors recolour the indicator's border as Breeze does, A.4; its fade-in under
    the hover animation is a transition, not a gap this entry tracks):
    "**Presets: the hover outlines the presets do not state yet** (`checkbox.hover_border_color`,
    `combo_box.hover_border_color`, 0.7.2). A preset states them where the facts give
    the platform's value as one colour for everything the field covers. To be read: GNOME's checked
    check box under the pointer (the unchecked box's ring is `currentColor` at 20 %,
    `PF` §2.5), and its drop-down's outline on hover under `prefers-contrast: more`;
    WinUI's check-box and combo-box border brushes under the pointer
    (`CheckBox_themeresources.xaml`, `ComboBox_themeresources.xaml`); an AppKit check
    box's and pop-up button's hover, which the OS probe renders at rest only; an open
    Material select's outline (it has the focus then; `PF` §2.4 gives the text field's
    focus outline). Each into
    `docs/platform-facts.md` with its source, then the preset. Not a reading but the
    model: Material's check box draws `on-surface` under the pointer when unchecked and
    no outline when checked (`PF` §2.5 `border.color`), which one field for both states
    cannot state, so material-3 leaves `checkbox.hover_border_color` unset."
  - New, tagged `[upstream]`: "gpui-component: a hover style, fill and border, for the
    trigger of `Select` and `Combobox` (0.10.0 `select.rs:578-597`,
    `combobox.rs:1025-1042`); until then `combo_box.hover_background` and
    `combo_box.hover_border_color` reach no gpui widget and the showcase lays them over
    its triggers (`demo::combo_surface`)." It is filed under *native-theme-gpui
    connector*, after that section's last `[upstream]` entry.
  - **The tags.** An entry is `[v0.7.x]` where the rationale's D10 plans its work
    within the 0.7.x releases, and `[1.0]` otherwise, as the steps 6–7 specification
    tags the work no step schedules (its decisions 15 and 18; 1.0 is the tag of 125 of
    the backlog's open entries today). D10 plans steps 1 to 7, the egui upgrade, the
    icons and caret work, and "the three outlines §0 leaves in the backlog" (the menu
    item's and the list row's hover outlines and the menu bar title: the rationale's §0,
    "three more outlines the first rationale put into the same step", and D10's item
    "Steps 6 and 7, and the three outlines §0 leaves in the backlog",
    `docs/issues/0004_kde-open-items-rationale.md`, §0 and D10). The new
    entry is not among them: the readings D6 asks for are no step's (its bullet "The
    other platform presets: …, with a backlog entry that names what is to be read").
    So it is `[1.0]`.
  - **The roadmap's count.** `ROADMAP.md` counts the backlog's open `[v0.7.x]` entries
    ("tags 74 open items `[v0.7.x]`" today). The documents unit recounts it on its tree
    with `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md`, writes the number there and says
    it in its commit body: 74 today, and the new entries of A.12 and B.11 are `[1.0]`,
    so 74 is expected; another figure is explained in the commit body.
- **The first rationale** (`docs/issues/0004_kde-hand-overrides-rationale.md`): no row
  of its table (§7) names these controls.
  **§5.4's status line**, as step 3 left it ("*Partly settled by
  `0004_kde-open-items-rationale.md` D5 (0.7.2): … The check box, the radio and the
  combo box are that rationale's D6, toolbar buttons and the table header its D7; the
  menu item's and the list row's outlines and the menu bar title stay in the backlog
  (its §0).*"). Before the sentence that starts "The check box, the
  radio and the combo box are", a sentence is inserted: "Also by D6 (0.7.2): the hover outlines of the check box, the radio button and the combo
  box (`checkbox.hover_border_color`, `combo_box.hover_border_color`), which kde-breeze
  and the KDE reader state, material-3 the combo box's; iced and egui draw them, and in
  gpui the connector's own check box and radio and the showcase over its drop-downs.";
  that sentence becomes "Toolbar buttons and the table header are that rationale's D7;
  the menu item's and the list row's outlines and the menu bar title stay in the
  backlog (its §0)." The line stays one italic paragraph.
- **The rationale's status line.** `docs/issues/0004_kde-open-items-rationale.md`'s
  "Status:" paragraph (under its title) gains one sentence for steps 2 to 5, written at
  0.7.2's close-out in place of the steps 2–3 plan's: "Steps 2 to 5 (D4, D5, D6 and
  D7's toolbar buttons) implemented on `<date>` for 0.7.2, by
  `../archive/todo_v0.7.2_kde-focus-ring-and-button-outlines-{spec,plan}.md` and
  `../archive/todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-{spec,plan}.md`." (the
  archive's paths, since the same close-out moves the documents there, B.11,
  *`docs/archive/`*). The rationale stays in `docs/issues/` while a step remains.

### A.13 Gates

| Command | Catches |
|---|---|
| `cargo test -p native-theme --features "$NT_FEATURES"` | A.6 tests, `the_derived_colours_are_the_presets_under_breeze`, the moved-leaves tests, `presets.rs`, `property_registry_keys_match_derive_inventory`, `documented_sizes.rs`, proptest |
| `cargo test -p native-theme-iced --features iced_aw,watch,native` and `--no-default-features` | contract rows, `every_colour_leaf_reaches_a_row_or_is_named`, the new cleared-hover test |
| `cargo test -p native-theme-egui --all-features` and `--no-default-features` | T3, `verdict_totals_match_the_specification`, T2, T10, T10c, `mapping_md_is_current`, role tests |
| `cargo test -p native-theme-gpui --features native,watch` and `--lib --no-default-features` | `every_colour_leaf_reaches_a_sink_or_is_named`, `CheckboxLook` tests, `every_colour_claim_is_read_at_the_line_it_cites` |
| the showcases in the runner environment (`scripts/run_in_runner_env.sh`), with the GPUI Fast legs (`<tools>/gate.sh --fast`) | the showcase tests of A.10, A.11 |
| the five documentation checkers (`scripts/check_platform_facts_rows.py`, `check_section_refs.py`, `check_preset_citations.py`, `check_wrong_safety_nets.py`, `check_todo_citations.py`) | facts rows, preset citations, backlog key paths |
| clippy, strict panic (`scripts/check_strict_panic.sh`), fmt, docs | the new code and doc links |
| full run only: `docs/assets/PROVENANCE.toml` and `docs/COMPATIBILITY.toml` stamps | stale after any connector change; re-stamped by 0.7.2's asset run (C.2) |

Not gated: the wording of every doc comment, facts cell and backlog note; the facts
citations of A.3, none re-pointed (S6).

### A.14 Problems found, and what was not verified

1. **The gpui overlay** (decision 7) is built in a trial (T3): it passes. It
   edges the real `Select`'s trigger, lets the click through and moves nothing, on both
   backends.
2. Windows's check-box and combo-box hover strokes, the outline of GNOME's checked box
   under the pointer and AppKit's hovers are not in the facts (the cells say so; A.12's
   backlog entry names them). Material's check box is in them and is not one colour
   (A.5).
3. egui draws an open combo box from `widgets.open` whatever the pointer does (egui
   0.37.0 `src/theme/default_style.rs:41-45`), so the egui connector shows the hover
   outline on a closed combo box only.
4. What an open `md-outlined-select` draws on its outline was not read; material-3's
   hover outline is drawn on an open trigger under the pointer where the toolkit reports
   one.

---

## Part B — Step 5 (D7): a toolbar button's states

### B.1 Model

`native-theme/src/model/widgets/mod.rs`, `ToolbarTheme` (`:542-573`), after `icon_size`
(`:563-566`):

```rust
    /// Fill of a toolbar button under the pointer, painted over the toolbar:
    /// a translucent colour is a layer, a transparent one paints nothing.
    /// `None` where the theme states none: a connector draws the hover as it
    /// draws a flat button's (docs/platform-facts.md §2.13).
    #[theme(category = "soft_option")]
    pub button_hover_background: Option<Rgba>,
    /// Outline of a toolbar button under the pointer, and while it is pressed.
    /// `None` where the theme states none: a connector draws the outline as it
    /// draws a flat button's (docs/platform-facts.md §2.13).
    #[theme(category = "soft_option")]
    pub button_hover_border_color: Option<Rgba>,
    /// Fill of a toolbar button while it is pressed, painted over the toolbar.
    /// `None` where the theme states none: a connector draws it as it draws a
    /// pressed flat button (docs/platform-facts.md §2.13).
    #[theme(category = "soft_option")]
    pub button_active_background: Option<Rgba>,
    /// Fill of a toolbar button that is checked (a toggle that is on), painted
    /// over the toolbar. `None` where the theme states none: a connector draws
    /// it as it draws a selected flat button (docs/platform-facts.md §2.13).
    #[theme(category = "soft_option")]
    pub button_checked_background: Option<Rgba>,
```

The struct's doc (`:542`) becomes "Toolbar sizing, spacing, font, and its buttons'
states.", followed by the paragraph: "The four `button_*` fields are the platform's
toolbar-button look. A platform may give that look to some of a toolbar's buttons only
(GNOME to image, image-text and arrow buttons; a button with a text label alone keeps
the ordinary button's look, libadwaita 1.10.0 `_toolbars.scss:2-8`); an application
applies the connector's toolbar-button style to those buttons, and the ordinary
button's style to the others." `ToolbarTheme` is built with `..Default::default()` or `Default` in the tests
(`native-theme/tests/merge_behavior.rs:401`, `proptest_roundtrip.rs:491`); no test edit
is forced.

### B.2 Registry and inheritance

- `docs/property-registry.toml` `[toolbar]` (`:356-362`): after `icon_size_px` the four
  keys, each `= "color"`.
- `docs/inheritance-rules.toml` `[no_inheritance]`, after `"expander.arrow_color"`
  (`:443`), the last of the state colours, under a comment line `# toolbar buttons (D7)`:
  `"toolbar.button_hover_background",   # None: the flat button's hover (§2.13)`,
  `"toolbar.button_hover_border_color", # None: the flat button's outline (§2.13)`,
  `"toolbar.button_active_background",  # None: the flat button's pressed fill (§2.13)`,
  `"toolbar.button_checked_background", # None: the flat button's selected fill (§2.13)`.

### B.3 `docs/platform-facts.md` §2.13

Insert directly after `border.shadow_enabled` (`PF:1399` at `0fa28027`) the four rows
(decision 1; no other line changes). Values in
F's notation, `#RRGGBBAA`; "F" citations are to the parked facts, which quote each
source line.

**The resolved values.** The expressions F gives, resolved:

| Platform | Expression (F line) | Inputs (source) | Arithmetic | Light | Dark |
|---|---|---|---|---|---|
| KDE hover fill | `alpha(active(role.selection.background), 0)` (`kde/button.toml:2944`) | — | alpha 0 | transparent | transparent |
| KDE hover outline | `active(role.selection.background)` (`:2991`) | Selection #3daee9 (`BreezeLight.colors:87`, `BreezeDark.colors:88`) | — | #3daee9 | #3daee9 |
| KDE pressed | `alpha(active(role.selection.background), 0.3)` (`:4594`) | as above | 0.3 × 255 = 76.5 → 77 (in `f32` the product is exactly 76.5, computed today; `Rgba::from_f32` rounds it with `f32::round`, `native-theme/src/color.rs:116-119`, to 77, which the reader's test on the same path asserts, B.13.3) | #3daee94d | #3daee94d |
| KDE checked | `alpha(role.button.foreground, 0.125)` (`:4151`) | ButtonText #232629 / #fcfcfc (`BreezeLight.colors:39`, `BreezeDark.colors:40`) | 0.125 × 255 = 31.875 → 32 | #23262920 | #fcfcfc20 |
| GNOME hover | `alpha(surface.foreground, 0.07)` (`gnome/button.toml:5013`, a `.toolbar` button) | window text #000006cc / #ffffff (`gnome/roles.toml:28-41`); `alpha` multiplies the alpha (`docs/todo_v0.7.1_model-completeness-rationale.md:664` at `78ee0efd`) | 0.8 × 0.07 × 255 = 14.28 → 14; 0.07 × 255 = 17.85 → 18 | #0000060e | #ffffff12 |
| GNOME pressed | `alpha(surface.foreground, 0.16)` (`:5035`) | as above | 32.64 → 33; 40.8 → 41 | #00000621 | #ffffff29 |
| GNOME checked | `alpha(surface.foreground, 0.1)` (`:5057`) | as above | 20.4 → 20; 25.5 → 26 (in `f32` 0.1 × 255 is exactly 25.5, a tie, which `f32::round` rounds away from zero, as for KDE pressed) | #00000614 | #ffffff1a |
| Windows hover | `role.@subtle_fill_secondary` (`windows/button.toml:8524`) | `windows/roles.toml:937-939`, `:921-923` | — | #00000009 | #ffffff0f |
| Windows pressed | `role.@subtle_fill_tertiary` (`:8545`) | `windows/roles.toml:980-982`, `:964-966` | — | #00000006 | #ffffff0a |
| Windows checked | `role.@accent_fill_default` (`:18609`, `AppBarToggleButton`) | `SystemAccentColorDark1` / `Light2` (`windows/roles.toml:3709-3722`); #0067c0 / #4cc2ff for the default accent (`PF:1219`, OS probe `probe.toml:755`, `:758`; `windows-11.toml:148`, `:789`) | — | #0067c0 | #4cc2ff |
| Windows outline | `role.@control_fill_transparent` (`:8869`) | — | — | none | none |

The four rows (cells in order macOS, Windows, KDE, GNOME, Material):

- `` `button_hover_background` ``: `` ❓ — AppKit's buttons draw nothing on hover (`docs/facts/macos/state-models.toml:2` at `78ee0efd`), and no probe pass measured a toolbar item's looks (`docs/facts/macos/state-models.toml:5-6` at `78ee0efd`) · macOS 26: ❓ — the same `` |
  `` **(Fluent)** AppBarButtonBackgroundPointerOver = SubtleFillColorSecondaryBrush — L #00000009, D #ffffff0f, over the bar ([AppBarButton_themeresources.xaml:6](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/AppBarButton_themeresources.xaml#L6); `docs/facts/windows/button.toml:8524` at `78ee0efd`); an unchecked AppBarToggleButton takes the same brush, AppBarToggleButtonBackgroundPointerOver (`docs/facts/windows/button.toml:18555` at `78ee0efd`) `` |
  `` **(Breeze src)** none: a hovered flat button's fill is alphaColor(Highlight, 0) ([breezehelper.cpp:688-691](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L688-L691); `docs/facts/kde/button.toml:2944` at `78ee0efd`) — transparent `` |
  `` **(Adwaita CSS)** the toolbar look, which a toolbar gives a button that is not .raised, .suggested-action, .destructive-action or .opaque and is an .arrow-button, .image-button or .image-text-button; a button with a text label alone is not matched and keeps the ordinary button's look ([_toolbars.scss:2-8](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_toolbars.scss#L2-L8); `docs/facts/gnome/button.toml:4856` at `78ee0efd`). Under the pointer it is $hover_color = color-mix(in srgb, currentColor 7%, transparent) (_colors.scss:268, _buttons.scss:250-251 at libadwaita 1.10.0; `docs/facts/gnome/button.toml:5013` at `78ee0efd`), currentColor the window text #000006cc / #ffffff — L #0000060e, D #ffffff12, a layer `` |
  `` ❓ — the parked facts give Material no toolbar button (no tool variant; a toolbar context only for the floating toolbar's FAB, `docs/facts/material/button.toml:13267` at `78ee0efd`) ``
- `` `button_hover_border_color` ``: `❓ — as button_hover_background` |
  `` **(none)** — AppBarButtonBorderBrushPointerOver = ControlFillColorTransparentBrush ([AppBarButton_themeresources.xaml:14](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/AppBarButton_themeresources.xaml#L14); `docs/facts/windows/button.toml:8869` at `78ee0efd`) `` |
  `` **(Breeze src)** Highlight, [Colors:Selection] BackgroundNormal of the Active group, 1 px: the pen of a hovered, keyboard-focused or pressed flat button ([breezehelper.cpp:710-712](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L710-L712), :672-673; `docs/facts/kde/button.toml:2991` at `78ee0efd`) — #3daee9 in Breeze and Breeze Dark `` |
  `` **(none)** at normal contrast — the same buttons draw no border; under prefers-contrast: more an inset 1px $border_color (_buttons.scss:241-246; `docs/facts/gnome/button.toml:4920` at `78ee0efd`) `` |
  `❓ — as button_hover_background`
- `` `button_active_background` ``: `` ❓ — not measured in a toolbar (`docs/facts/macos/state-models.toml:5-6` at `78ee0efd`); the toolbar bezel (tool) the probe drew on its sheet fills #0000001a / #ffffff1a pressed (`docs/facts/macos/button.toml:148813`, `:148869` at `78ee0efd`) · macOS 26: ❓ — the same; the sheet's #0000001a / #ffffff1a (`docs/facts/macos/button.toml:14861`, `:14830` at `78ee0efd`) `` |
  `` **(Fluent)** AppBarButtonBackgroundPressed = SubtleFillColorTertiaryBrush — L #00000006, D #ffffff0a ([AppBarButton_themeresources.xaml:7](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/AppBarButton_themeresources.xaml#L7); `docs/facts/windows/button.toml:8545` at `78ee0efd`); an unchecked AppBarToggleButton the same, AppBarToggleButtonBackgroundPressed (`docs/facts/windows/button.toml:18576` at `78ee0efd`) `` |
  `` **(Breeze src)** alphaColor(Highlight, Blend_Value 0.3) ([breezehelper.cpp:679-680](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L679-L680), :40; [breezemetrics.h:176](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L176); `docs/facts/kde/button.toml:4594` at `78ee0efd`) — #3daee94d, a layer `` |
  `` **(Adwaita CSS)** the same buttons :active: $active_color, currentColor 16 % (_colors.scss:269, _buttons.scss:254-256; `docs/facts/gnome/button.toml:5035` at `78ee0efd`) — L #00000621, D #ffffff29 `` |
  `❓ — as button_hover_background`
- `` `button_checked_background` ``: `` ❓ — no value measured (`docs/facts/macos/button.toml:330266` at `78ee0efd`, `:259991`) · macOS 26: ❓ — the same `` |
  `` **(Fluent)** AppBarToggleButtonBackgroundChecked = AccentFillColorDefaultBrush, SystemAccentColorDark1 / Light2 — #0067c0 / #4cc2ff for the default accent ([AppBarToggleButton_themeresources.xaml:9](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/AppBarToggleButton_themeresources.xaml#L9); `docs/facts/windows/button.toml:18609` at `78ee0efd`), lettered TextOnAccentFillColorPrimary, #ffffff / #000000 (`docs/facts/windows/button.toml:9299` at `78ee0efd`), the colour of §2.3's checked_text_color, which a checked toolbar button is lettered in where its fill is stated `` |
  `` **(Breeze src)** alphaColor(ButtonText, 0.125) ([breezehelper.cpp:681-684](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L681-L684); `docs/facts/kde/button.toml:4151` at `78ee0efd`) — #23262920 / #fcfcfc20; its pen, KColorUtils::mix(Button, ButtonText, frameContrast) (`docs/facts/kde/button.toml:4188` at `78ee0efd`), has no field `` |
  `` **(Adwaita CSS)** the same buttons :checked: currentColor 10 % (_colors.scss:270, _buttons.scss:259-260; `docs/facts/gnome/button.toml:5057` at `78ee0efd`) — L #00000614, D #ffffff1a `` |
  `❓ — as button_hover_background`

**The Breeze lines, read at `f0b1d753`.** The parked facts cite these lines at
`breeze@6.8.2`; the cells cite them at `f0b1d753`, the commit of the facts file's other
Breeze links, where the writing session read them on 2026-11-15 in
`https://invent.kde.org/plasma/breeze/-/raw/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp`
and `…/kstyle/breezemetrics.h` (`Helper::renderButtonFrame`, `:647-…`). Each holds
there: `:40`, `highlightBackgroundAlpha = Metrics::Blend_Value`; `:672-673`, the
highlight from the `Active` group (`Disabled` when disabled); `:679-680`, a flat button
down: `alphaColor(highlightColor, highlightBackgroundAlpha)`; `:681-684`, checked:
`alphaColor(palette.buttonText().color(), 0.125)` and the pen
`KColorUtils::mix(palette.button().color(), palette.buttonText().color(),
frameIntensityBias())` (the neutral highlight aside); `:688-691`, otherwise
`alphaColor(highlightColor, 0)`; `:710-712`, hovered, keyboard-focused or down and
enabled: the pen `highlightColor`; `breezemetrics.h:176`, `Blend_Value = 0.3`.

**Citations of the lines these edits move**: as A.3 says (the batch's S6), none is
re-pointed; the existing ones are left as they are. Gates as in A.3.

### B.4 Presets

Inserted after `item_gap_px` in each `[<mode>.toolbar]`; citations of the new rows, in
A.5's form with no line number (S6): the document and its section, the key below
naming the row.

**kde-breeze** (`[light.toolbar]` after `:384`, `[dark.toolbar]` after `:972`), light:

```toml
# platform-facts.md (§2.13): a hovered flat button's fill is alphaColor(Highlight, 0), nothing (breezehelper.cpp:688-691
# at breeze f0b1d75); its hover shows as the outline below
button_hover_background = "#00000000"
# platform-facts.md (§2.13): [Colors:Selection] BackgroundNormal 61,174,233 (BreezeLight.colors:87, breeze v6.7.5), the pen
# of a hovered, focused or pressed flat button (breezehelper.cpp:710-712 at breeze f0b1d75)
button_hover_border_color = "#3daee9"
# platform-facts.md (§2.13): Highlight at Blend_Value 0.3, a layer (breezehelper.cpp:679-680, :40; breezemetrics.h:176 at
# breeze f0b1d75)
button_active_background = "#3daee94d"
# platform-facts.md (§2.13): [Colors:Button] ForegroundNormal 35,38,41 (BreezeLight.colors:39, breeze v6.7.5) at 0.125
# (breezehelper.cpp:681-684 at breeze f0b1d75)
button_checked_background = "#23262920"
```

Dark: `BreezeDark.colors:88` and `:40`, `button_checked_background = "#fcfcfc20"`, the
rest equal.

**adwaita** (`[light.toolbar]` after `:288`, `[dark.toolbar]` after `:725`), light:

```toml
# platform-facts.md (§2.13): the toolbar look of an image, image-text or arrow button (_toolbars.scss:2-8):
# :hover, $hover_color, the window text #000006cc at 7 %, a layer (_colors.scss:268, _buttons.scss:250-251 at
# libadwaita 1.10.0)
button_hover_background = "#0000060e"
# button_hover_border_color not stated: platform-facts.md (§2.13), (none), a toolbar button draws no border
# platform-facts.md (§2.13): :active, $active_color, the window text at 16 % (_colors.scss:269, _buttons.scss:254-256)
button_active_background = "#00000621"
# platform-facts.md (§2.13): :checked, the window text at 10 % (_colors.scss:270, _buttons.scss:259-260)
button_checked_background = "#00000614"
```

Dark: "the window text #ffffff", `#ffffff12`, `#ffffff29`, `#ffffff1a`.

**windows-11** (`[light.toolbar]` after `:394`, `[dark.toolbar]` after `:1035`), light:

```toml
# platform-facts.md (§2.13): AppBarButtonBackgroundPointerOver = SubtleFillColorSecondary (AppBarButton_themeresources.xaml:6
# at microsoft-ui-xaml 8463f45)
button_hover_background = "#00000009"
# button_hover_border_color not stated: platform-facts.md (§2.13), (none), AppBarButtonBorderBrushPointerOver is transparent
# platform-facts.md (§2.13): AppBarButtonBackgroundPressed = SubtleFillColorTertiary (AppBarButton_themeresources.xaml:7)
button_active_background = "#00000006"
# platform-facts.md (§2.13): AppBarToggleButtonBackgroundChecked = AccentFillColorDefault = SystemAccentColorDark1, #0067c0
# for the accent above (AppBarToggleButton_themeresources.xaml:9)
button_checked_background = "#0067c0"
```

Dark: `#ffffff0f`, `#ffffff0a`, `SystemAccentColorLight2, #4cc2ff` → `"#4cc2ff"`. "For
the accent above" is the file's own wording for the default accent its `[*.defaults]
accent_color` states (`windows-11.toml:146`, the checked button's comment; `:23`,
`:663`). The XAML lines are cited at microsoft-ui-xaml `8463f45`, the commit the
facts file and windows-11's comments cite (`windows-11.toml:129-130`), where the eighth
confirming check found them (B.13.5).

**The rule, re-checked** (decision 8). None of B.3's rows is in `docs/platform-facts.md`
today: each comes from the parked facts, and each value stated above is a checked fact
there (a fact of `F kde/button.toml`, `gnome/button.toml`, `windows/button.toml` with its
`[[fact.cite]]` quoting the source line, and the roles it names in `F gnome/roles.toml`
and `windows/roles.toml`, none from `F imported/`), which B.3 brings into §2.13 with the
source. kde-breeze states all four, adwaita and windows-11 the three fills (their
outline cells are "(none)", which unset gives).

**Unset, by D6's rule** (D7). macos-monterey and macos-tahoe: F marks a toolbar item's
looks as unmeasured (`macos/state-models.toml:5-6`, U94); the checked fact of the
`toolbar` bezel's pressed fill was measured on the probe's sheet, outside a toolbar, so it
is not the toolbar button's value (B.11 records it). material-3: F has no Material
toolbar button (no `tool` variant; a toolbar context only for the floating toolbar's
FAB, `material/button.toml:13267`). ios-13: F's bar items have no fill at rest
(`ios/button.toml:14089`), and every other state's fill is a pending fact without a
value: pressed (`:17467`, pending U123), checked only for a `toggle` bar item
(`:17491`, U123), disabled and checked or pressed (`:15175`, `:15705`, U115). Each gets a
backlog entry (B.11). The ten colour-scheme presets: no platform, no facts. None of these
adds a comment: their toolbar sections have no state keys to cite against.

**The text of a checked toolbar button** (D7) is `button.checked_text_color` where the
theme states `toolbar.button_checked_background`, `button.active_text_color` where that
is `None` (decision 11). In the three presets that state the checked fill it is stated,
not inherited:

| Preset | `button.checked_text_color` (source) | toolbar fill under it | checked fill over the bar | contrast |
|---|---|---|---|---|
| kde-breeze light | #232629, stated (`kde-breeze.toml:147`; comment `:145`, ButtonText) | #dee0e2, stated (`:381`) | #23262920 over it: #c7c9cb | 9.12:1 |
| kde-breeze dark | #fcfcfc, stated (`:732`; `:730`) | #292c30, stated (`:969`) | #fcfcfc20: #43464a | 9.23:1 |
| adwaita light | #2e3436, stated (`adwaita.toml:129`; `:127`, no `:checked` rule sets a label colour) | #fafafb, inherited from `defaults.background_color` (`:20`; `docs/inheritance-rules.toml:233`) | #00000614: #e6e6e8 | 10.18:1 |
| adwaita dark | #ffffff, stated (`:566`; `:564`) | #222226, inherited (`:467`) | #ffffff1a: #39393c | 11.58:1 |
| windows-11 light | #ffffff, stated (`windows-11.toml:149`; `:145-147`, `TextOnAccentFillColorPrimary`) | #f3f3f3, inherited (`:25`) | #0067c0, opaque | 5.67:1 |
| windows-11 dark | #000000, stated (`:790`; `:786-789`) | #202020, inherited (`:665`) | #4cc2ff, opaque | 10.47:1 |

Computed today with python3: WCAG 2 relative luminance, the fill composited over the
toolbar's fill and the text over that, straight alpha, rounded to whole levels only for
the hex shown. Lettered as today instead, windows-11's pairs were 2.66:1 and 1.71:1 in
iced (`button.active_text_color`, #0000009e / #ffffffc5) and 3.50:1 and 2.01:1 in gpui
(`button.font.color`, #000000e4 / #ffffff); kde-breeze's and adwaita's text is the same
colour either way. No pair is under 3:1.

The showcases' checked button (B.10) lies on the Basic page, whose ground is the window's,
`defaults.background_color` (that no showcase paints another fill under the icon row was
not read): there kde-breeze's translucent fill composites to #d5d7d8 over #eff0f1 (10.51:1
with #232629) and #3c3e41 over #202326 (10.44:1 with #fcfcfc), computed as above;
adwaita's toolbar fill is the window's, and windows-11's checked fill is opaque, so their
figures are the table's.

**Gate in the core.** `presets.rs`, a new test beside A.7's over the same seven platform
presets (material-3 and ios-13 among them, which must state none),
`toolbar_button_states_are_stated_where_the_facts_give_them`:
`button_hover_background`, `button_active_background` and `button_checked_background`
are stated exactly by `adwaita`, `kde-breeze` and `windows-11`,
`button_hover_border_color` by `kde-breeze`; doc comment "A toolbar button's states:
stated where the facts give them as one colour for everything they cover
(docs/platform-facts.md §2.13)".

### B.5 KDE reader

`native-theme/src/kde/colors.rs`:

- A constant beside `CHECKED_BUTTON_MIX` (`:358-362`):

  ```rust
  /// The alpha of the button text a checked flat Breeze button is filled
  /// with, `alphaColor(palette.buttonText().color(), 0.125)`
  /// (breezehelper.cpp:681-684 at breeze f0b1d75; docs/platform-facts.md §2.13).
  const FLAT_CHECKED_ALPHA: f32 = 0.125;
  ```

- Toolbar block (`:667-683`), after the edge:
  `variant.toolbar.button_hover_border_color = selection_bg;`,
  `variant.toolbar.button_active_background = selection_bg.map(|highlight| translucent(highlight, BLEND_VALUE));`,
  `variant.toolbar.button_checked_background = button_fg.map(|text| translucent(text, FLAT_CHECKED_ALPHA));`
  (`selection_bg` beside `view_focus` at the top of `populate_colors`, `button_fg`
  `:520`, `BLEND_VALUE` `:18`). Comment: "A flat
  toolbar button (docs/platform-facts.md §2.13): under the pointer, focused or pressed
  its pen is `Highlight`; pressed it is filled with `Highlight` at `Blend_Value`,
  checked with the button text at 0.125 (breezehelper.cpp:679-691, :710-712 at breeze
  f0b1d75). Its hover fill, transparent, is the preset's alone." `button_hover_background`
  is not written (decision 2).
- `populate_colors`' doc (`:459-475`): the colours computed "in Qt's arithmetic" gain
  "a toolbar button's pressed and checked fills"; the sentence on blends of the accent
  gains "the toolbar button's pressed fill"; and A.6's sentence on the combo box's pen
  becomes "The combo box's and the toolbar button's hover pen is `[Colors:Selection]
  BackgroundNormal` as it is."

Values under both Breeze schemes: B.3's KDE rows; they equal B.4's kde-breeze lines.

**Tests.** `kde/colors.rs`: `test_toolbar_button_states_are_breezes` (both schemes:
outline #3daee9, pressed `Rgba::new(0x3d, 0xae, 0xe9, 0x4d)`, checked
`Rgba::new(0x23, 0x26, 0x29, 0x20)` / `Rgba::new(0xfc, 0xfc, 0xfc, 0x20)`,
`button_hover_background` `None`); the custom-scheme test of A.6,
`a_custom_scheme_moves_the_hover_borders`, also asserts the
three from its keys, its scheme already holding `[Colors:Button] ForegroundNormal=60,20,10`
(the fourth argument of `breeze_menu_list_groups` at `kde/colors.rs:1495`, the helper at
`:1058-1070`; checked `Rgba::new(60, 20, 10, 0x20)`);
`test_missing_groups_leave_the_new_fields_to_the_preset` (`:1589-1605`) gains the three:
with only `[Colors:Selection]` (`:1590`) the hover outline and the pressed fill are
`Some` and the checked fill `None` (it reads `[Colors:Button] ForegroundNormal`), and
with only `[Colors:Window]` (`:1599`) all three are `None`; `test_full_mapping_completeness`
gains the three as `Some`. `kde/mod.rs`: `derived_leaves` and
`resolved_leaves` append `toolbar.button_hover_border_color`,
`toolbar.button_active_background`, `toolbar.button_checked_background` (the two
array lengths and the doc's count grow by three, 36 → 39, the doc's "thirty-six" →
"thirty-nine"; the one core unit writes 39);
`an_accent_moves_…` appends the first two; `the_button_set_moves_…` (`:1441`) appends
the third. The `Header` set does not move them (not verified that a tools area's
palette, which Breeze takes from the Header set, gives a toolbar button its
`ButtonText`: F states `role.button.foreground` without a tools-area context; under
both Breeze schemes the two sets' `ForegroundNormal` are equal, #232629 / #fcfcfc:
`[Colors:Button]` at `BreezeLight.colors:39` and `[Colors:Header]` at `:67`, 35,38,41,
and `BreezeDark.colors:40` and `:68`, 252,252,252, at breeze v6.7.5, as the reader's
test schemes carry them, `kde/mod.rs:1215-1244`; that Breeze gives a main window's
toolbars the Header set is `PF:1394`'s KDE cell, whose value is the toolbar's
background).

### B.5a Windows reader

WinUI fills a checked toolbar button with the user's accent fill, as it fills a checked
`ToggleButton` (B.3: `AppBarToggleButtonBackgroundChecked = AccentFillColorDefault`), so
windows-11's `#0067c0` / `#4cc2ff` (B.4) is the default accent's value only, and the
Windows reader writes the field from the accent it reads, as it writes
`button.checked_background`. `native-theme/src/windows.rs` (an existing `unsafe`
exception file; this edit adds no `unsafe`), in the core unit (U1):

- `build_theme` (`:525-…`), after `variant.button.checked_background = Some(primary_bg);`
  (`:551`, `primary_bg` being `AccentFillColorDefault`, `:549`):

  ```rust
      // A checked toolbar button: AppBarToggleButtonBackgroundChecked =
      // AccentFillColorDefaultBrush (AppBarToggleButton_themeresources.xaml:9;
      // docs/platform-facts.md §2.13).
      variant.toolbar.button_checked_background = Some(primary_bg);
  ```

- `apply_sys_colors` (`:315-…`), after `variant.button.checked_text_color = Some(colors.highlight_text);`
  (`:361`, beside the checked button's fill at `:360`):

  ```rust
      // AppBarToggleButtonBackgroundChecked = SystemColorHighlightColorBrush
      // (AppBarToggleButton_themeresources.xaml:74).
      variant.toolbar.button_checked_background = Some(colors.highlight);
  ```

  (the high-contrast fact, F `windows/button.toml:18778-18786`: `role.@system_highlight`,
  quoting `:74`). The reader writes no toolbar hover or pressed fill, under high
  contrast either: windows-11's stay (B.11's backlog note).
- Tests: `accent_fills` (`:925-934`) gains `("toolbar.button_checked_background",
  v.toolbar.button_checked_background)`, its array 6 → 7 and its doc "…the focused text
  box's line, and a checked toolbar button's fill"; the three tests that loop over it
  then assert the field, `build_theme_accent_shades_light_mode` (`:998`, `:1015`) the
  light shade, `build_theme_accent_shades_dark_mode` (`:1027`, `:1042`) the dark one,
  and `under_high_contrast_the_reader_maps_the_system_colours` (`:1241`, `:1261`)
  `c.highlight`.
- The `presets.rs` test of B.4 is unchanged: the reader is no preset.

### B.6 "Unset": what each connector draws today, which stays the fallback

Per field, the state of a flat (toolbar) button where the field is `None`:

| Connector, today's flat button | hover fill | hover outline | pressed fill | checked fill | checked text |
|---|---|---|---|---|---|
| gpui `variants::ghost_button` (`CG/variants.rs:57-65`; `GC/button/button.rs:1028`, `:1118-1124`, `:1209`, `:1294`, `:1337`) | `secondary_hover` = `button.hover_background` | none (the border is the variant's `color`, transparent) | `secondary_active` = `button.active_background`, or a shade of the button's fill where unstated (`CG/variants.rs:51-56`) | `secondary_active`, the pressed fill | `secondary_foreground` = `button.font.color`, one colour in every state (`GC/button/button.rs:1080`) |
| iced showcase `ghost_button` (`showcase-iced.rs:4625-4661`) | `button.hover_background`, as given | none | `button.active_background`, else the hover fill | the pressed fill | `button.active_text_color` (`:4649`) |
| egui «toolbar» role (`CE/style/roles.rs:702-739`) with the showcase's `tool_button` (`showcase-egui/demo.rs:283-314`) | the base's hovered `weak_bg_fill`: `button.hover_background` over the button's fill (`mapping.toml` `["button.hover_background"]`) | none: `tool_button` sets every stroke width to 0 | the base's active `weak_bg_fill` | a selected button: `visuals.selection.bg_fill`; the showcase fills its selected ghost with the active fill (`showcase-egui/chrome.rs:510-511`) | the scope's text colour, the toolbar role's `toolbar.font.color` (`CE/style/roles.rs:723`) |

The connectors' new functions take these as the fallbacks (B.7-B.9), the checked text
included where the theme states no checked fill; D7 leaves the three connectors'
differences as they are.

### B.7 egui

`CE/style/roles.rs`, `toolbar` (`:702-739`), after `write_states`: field by field, each
written only where stated and as stated, not composited (D7): `button_hover_background`
→ `s.visuals.widgets.hovered.weak_bg_fill`; `button_active_background` →
`.active.weak_bg_fill`; `button_hover_border_color` → `.hovered.bg_stroke.color` and
`.active.bg_stroke.color` (the colour only; the width stays the base's). `FillSource`
is not used: its `None` hover copies the resting fill (`CE/style/states.rs:128-135`),
which is not today's. The outline goes into `active` as A.9's check box's does, and
with a cost of its own here: egui's `active` is also the keyboard-focused state
(`EG/style.rs:1273-1285`), so an egui tool button reached by Tab shows the outline,
where a Breeze toolbar button never shows one on focus, since Qt makes a toolbar's
action buttons take no focus (`setFocusPolicy(Qt::NoFocus)`, qtbase `v6.11.0`
`qtoolbarlayout.cpp:703`; B.13.2). The code comment beside the write says so ("`active`
is also egui's keyboard-focused state, so a tool button reached by Tab shows the
outline, which a Qt toolbar button, taking no focus, never shows"). The «toolbar»
`active` sink of `["toolbar.button_hover_border_color"]` carries no `when` to say it
(decided): a `when` would lift T2's requirement that the sink moves under a splice of
its leaf (*Why no `when`*, below), so the cost lives in this prose and the code comment
only. The backlog entry *Toolbar buttons and the Tab key* (B.11) records it. A
pressed fill on focus is egui's for every button already and not new.

**The states are a flat tool button's** (decided). The role leaves `inactive` the base's:
a tool button is made flat per call. `Button::frame_when_inactive(false)` adds
`Button::CLASS_HIDE_FRAME_WHEN_INACTIVE`, which "with the default style removes the
fill and the stroke, but keeps the margin, so the button does not change size"
(`egui-0.37.0/src/widgets/button.rs:191-205`): the default style draws an inactive
button of that class as `painted_frame.invisible()` (`theme/default_style.rs:77-84`), a
frame with a transparent fill and stroke and its layout unchanged
(`containers/frame.rs:344-353`). So the flag and a ghost scope (a scope whose resting
fill and border colour are transparent, as the showcase's `demo::ghost` makes it,
`showcase-egui/demo.rs:283-287`) both keep the button's size.

`demo::ghost`'s doc (`showcase-egui/demo.rs:279-281`) still states the 0.36 behaviour;
the egui unit corrects it: the sentence "It keeps its full frame, and so its size, in
every state, where `frame_when_inactive(false)` lays it out at rest without the frame's
border, which the hovered frame adds to its size (`egui/src/widgets/button.rs:364-368`)."
becomes "It keeps its full frame, and so its size, in every state, as
`Button::frame_when_inactive(false)` does since egui 0.37, which hides a resting
button's fill and stroke and keeps its margin (egui 0.37.0 `src/widgets/button.rs:191-205`,
`src/theme/default_style.rs:77-84`)." Its gpui-component 0.9.1 citations, and
`GHOST_NOTE`'s, stay.

A plain framed `Button` in a «toolbar» scope keeps its resting fill and takes the stated
hover and pressed fills. The options weighed: (a) the role as written, taken; its cost is
that every widget in a «toolbar» scope that fills from `weak_bg_fill`, a plain `ComboBox`
among them, takes the toolbar's hover and pressed fills, so a combo box belongs in its
own role's scope (`Role::ComboBox`: a role is "the content style for one kind of widget",
one per widget field of `ResolvedTheme`, `CE/roles.rs:4-9`, `:62-63`). (b) The role also
makes the rest flat (`inactive`'s `weak_bg_fill` and `bg_stroke.color` transparent): not
taken, since it would change the resting look of every widget in the scope that
fills from `weak_bg_fill`, a combo box's too, which (a) leaves as the base gives it, changing only their
hover and press. (c) The stated fills laid over the button's resting fill, as the ordinary
button's hover is, which D7 rejects (its egui item, "a transparent hover fill must stay
transparent", `docs/issues/0004_kde-open-items-rationale.md`, D7). Said in three places:

- `Role::Toolbar`'s doc (`CE/roles.rs:44`, "The toolbar: `ResolvedTheme::toolbar`.")
  becomes "The toolbar: `ResolvedTheme::toolbar`. Its hovered and
  pressed states are a flat tool button's, the toolbar's `button_*` fields where the
  theme states them: make the button flat with `Button::frame_when_inactive(false)`, or
  in a scope whose resting fill and border colour are transparent; both keep its size.
  A framed button keeps its fill at rest.";
- the README's accessor paragraph (below) gains "A button in the toolbar
  role is a flat tool button: the role colours its hover and press and leaves its rest
  to the button. Make it flat with `Button::frame_when_inactive(false)`, or in a scope
  whose resting fill and border colour are transparent; both keep its size.";
- the CHANGELOG's egui bullet (B.11).

The role does write `inactive`'s text colour,
`fg_stroke.color`, from `toolbar.font.color`, as today (`write_states` with
`text_in_all(tb.font.color)`, `CE/style/roles.rs:719-728`; `CE/style/states.rs:162`;
`mapping.toml:2638-2644`). That `inactive`'s fill and stroke stay the base's is held by
the tests below (`sidebar_and_toolbar_write_their_own_rows` asserts the cell's `inactive`
`weak_bg_fill`, `bg_fill` and `bg_stroke` equal the base's) and by T10c, since no row
declares a «toolbar» `inactive` fill or `bg_stroke` sink (`toolbar.font.color` declares
its `fg_stroke`; `t10c_every_undeclared_field_holds_its_inherited_value`,
`CE/contract.rs:1343`).

**Step 3's kept strokes.** Step 3 adds to `CE/style/roles.rs` a predicate,
`fn keeps_resting_stroke(role: Role) -> bool`, true for seven roles, «toolbar» among
them, and in `role_cell` (`roles.rs:19-56`), after the `match` that calls `toolbar` and
before `derived::apply_role`, one statement `if keeps_resting_stroke(role) { … }` that
sets their `hovered` and `active` `bg_stroke.color` to their `inactive` one (its 3E.1,
decision 13). Left as it is, it would undo the outline `toolbar` writes. Step 5 changes
the predicate, not the statement: it takes the theme as well and no longer holds for
`Role::Toolbar` where `toolbar.button_hover_border_color` is stated:

```rust
/// The roles whose cells keep their resting stroke when hovered or pressed: the button's
/// hover and focus outline is not theirs, a toolbar's or a list's buttons and the controls
/// inside a slider's or a switch's scope among them (D5; the backlog decides the others).
/// Each passes `border: None` to `write_states`. A toolbar whose theme states its
/// buttons' outline (`toolbar.button_hover_border_color`) keeps the one `toolbar`
/// writes (D7).
fn keeps_resting_stroke(role: Role, theme: &ResolvedTheme) -> bool {
    match role {
        Role::Toolbar => theme.toolbar.button_hover_border_color.is_none(),
        Role::Scrollbar
        | Role::Slider
        | Role::Sidebar
        | Role::StatusBar
        | Role::List
        | Role::Switch => true,
        _ => false,
    }
}
```

and its one call becomes `keeps_resting_stroke(role, input.theme)` (`input.theme` is the
`ResolvedTheme`, `CE/style/mod.rs:19`, as `toolbar` reads it, `CE/style/roles.rs:703`;
`roles.rs` imports `ResolvedTheme` outside its tests for this, as `style/mod.rs` does,
since today only its test module names it, `:1149`).
Where the field is unset, the toolbar keeps its resting stroke as step 3 makes it.

**Accessor** (`CE/accessors.rs`, after `selection_inactive_background`, `:88-92`):

```rust
/// `toolbar.button_checked_background` as an `egui::Color32`: the fill of a
/// checked toolbar button, for `Button::fill` on the one button. egui has no
/// checked state: a selected button takes `visuals.selection`, which also
/// fills selected text in the same scope. Where it is `Some`, letter the
/// button in `button.checked_text_color`, or `button.active_text_color` where
/// that is `None`, read from the `ResolvedTheme`. `None` where the theme
/// states no checked fill: the button is drawn as a selected flat button.
#[must_use]
pub fn toolbar_button_checked_background(t: &ResolvedTheme) -> Option<egui::Color32>
```

returning `t.toolbar.button_checked_background.map(to_color32)`. The text colour has no
accessor of its own: it is two leaves with a fallback, which the parity script's naming
rule does not cover (`scripts/check_connector_parity.py:27-35`), and egui's README sends
an application to the `ResolvedTheme` for a leaf that needs no conversion
(`README.md:265-267`); `button.checked_text_color`'s mapping row stays
`derived`, `T18(a)` (`mapping.toml:996-998`). Re-exported in
`CE/lib.rs`'s `pub use accessors::{…}` (`:194-203`); the README's accessor paragraph
(`README.md:257-267`) names it ("…`list_header_height` for `TableBuilder::header`,
`toolbar_button_checked_background` for a checked toolbar button, `icons::icon_size`…").
Parity: `scripts/connector_parity_exceptions.toml` gains `kind = "absent"` entries for
`iced` (reason: "`styles::toolbar_button` fills a checked toolbar button with it
(`src/styles.rs`); iced has no toolbar, so the style is the receiver") and `gpui`
(reason: "a gpui toolbar Button's fills come from its variant and its style
refinements (`variants::toolbar_button`, `src/geometry.rs`), not from a colour accessor",
which is true in the gpui unit (U2), where `toolbar_button` gives the hover and pressed
fills, and after the unit that builds Edit adds `geometry::toolbar_button_checked`). Gate:
`scripts/check_connector_parity.py` (`check_release.sh:919`).

**Mapping.** `["toolbar.button_hover_background"]` scoped, «toolbar»
`visuals.widgets.hovered.weak_bg_fill`; `["toolbar.button_hover_border_color"]` scoped,
«toolbar» hovered and active `bg_stroke.color`; `["toolbar.button_active_background"]`
scoped, «toolbar» `visuals.widgets.active.weak_bg_fill`; none of these sinks has a
`when`. `["toolbar.button_checked_background"]` `verdict = "derived"`,
`tested_by = "T18(a)"`, as `defaults.info_color` (`mapping.toml:195-197`). Placed after
`["toolbar.icon_size"]` (`:2609`).

Why no `when`, by T2's rule (`CE/mapping_tests.rs:1101-1160`, `:1286-1324`,
`:1334-1437`): a sink without `when` is required to move under at least one splice of
its leaf, and each of the four sinks does under the pairs `("kde-breeze", "adwaita")`
(`PAIRS`, `:905-918`) in both modes, where the two presets differ in all three leaves
(transparent against #0000060e / #ffffff12, #3daee94d against #00000621 / #ffffff29,
#3daee9 against unset). Without a `when`, the «toolbar» location holds its own value
where the leaf is stated (`Own::Overrides`, `:1029-1062`) and carries the base's sinks
where it is unset. Of the base sinks at the two fill paths only
`button.hover_background`'s `hovered.weak_bg_fill` has no `when` (`mapping.toml:952`;
its `active` sinks, `:954-955`, and `button.active_background`'s, `:980-981`, have one),
so T2 requires that one to move the toolbar's hover fill on the presets that state no
toolbar hover fill, and allows the others; a `when` on the toolbar's own sinks would
make the location `Partly` on every preset and drop that one check.

**`["button.border.color"]`'s «toolbar» sinks.** Step 3 gives the row 14 scoped sinks,
the hovered and active `bg_stroke.color` of seven roles, with no `when` (its 3E.2). The
two of «toolbar» gain `when = "toolbar.button_hover_border_color is None"`: where it is
stated, `toolbar` writes the field there instead.

Counts. Four more: 525 leaves, "521 ResolvedTheme leaves + 4 layout leaves (§5.7)";
Chrome 104, `("chrome", [0, 57, 28, 19])` in `verdict_totals_match_the_specification`,
`[direct, scoped, derived, unmappable]`; no sub-tag count changes. A `grep` of the egui
crate and its showcase finds no other hard-coded count or leaf list the four rows change
(the leaf count of `mapping_tests.rs:239-240` is the one above).

**Contract** (`CE/contract.rs`, `expected`, `:529-…`). The two fill sinks have one
declaring row each, so the fallback arm's conversion of that row holds them, `Ok(None)`
where the leaf is unset (`:1039-1059`). The «toolbar» `hovered` and `active`
`bg_stroke.color` have two declaring rows after step 3, `button.border.color` and
`toolbar.button_hover_border_color`, and the fallback arm refuses a location with two
(`:1053-1058`, "add its §6 formula"). A new arm before the fallback:

```rust
        // ---- D7: a toolbar's buttons' outline, else the resting stroke step 3 keeps ----
        (Some("toolbar"), "normal", None, p) if hover_or_press && p.ends_with(".bg_stroke.color") => {
            match t.toolbar.button_hover_border_color {
                Some(edge) => c32(edge),
                None => Val::Color(cell().ok_or("a scoped sink")?.visuals.widgets.inactive.bg_stroke.color),
            }
        }
```

where unset it is the cell's resting stroke, which step 3's statement copies into
`hovered` and `active` (the steps 2–3 specification, decision 13: "falls back to the
cell's rest stroke, not to what the cell inherits from the base"); `cell()` is the
closure the other scoped arms read (`:558`). Gates: T10 and T10c (`:1074`, `:1343`).

**Tests.**

- `sidebar_and_toolbar_write_their_own_rows` (`roles.rs:1970-2007`), over `PRESETS`
  (`:1299-1306`: adwaita light, kde-breeze dark, windows-11 light, macos-monterey dark,
  material-3 light, catppuccin-mocha dark): its «toolbar» part asserts, per preset, the
  cell's `hovered.weak_bg_fill` is `toolbar.button_hover_background` as given where
  stated, else the base's `hovered.weak_bg_fill`; its `active.weak_bg_fill`
  `toolbar.button_active_background` as given, else the base's; its `hovered` and
  `active` `bg_stroke.color` `toolbar.button_hover_border_color`, else the cell's
  `inactive` stroke (step 3); and its `inactive` `weak_bg_fill`, `bg_fill` and
  `bg_stroke` are the base's, the button at rest being the caller's (above; its
  `fg_stroke` is `toolbar.font.color`, as today). The values the presets give, read from B.4: kde-breeze
  dark transparent, #3daee94d, #3daee9; adwaita light #0000060e, #00000621, the resting
  stroke; windows-11 light #00000009, #00000006, the resting stroke; the other three the
  base's fills and the resting stroke.
- `panel_role_cells_write_their_rows` (`roles.rs:1900-…`, step 3 compares «toolbar»'s
  `hovered` and `active` stroke with the base's `inactive` one, 3E.3): for «toolbar»,
  the field where stated.
- `a_none_soft_option_copies_its_fallback_leaf` (`derived.rs:963`) clears the three and
  asserts the toolbar cell's fills are the base's hovered and active fills and its
  hovered and active strokes its `inactive` stroke.
- `install_tests.rs`, `the_atlas_reports_what_the_builder_was_given` (`:40-…`, whose doc
  begins "T18 (a), and every other accessor", so that `info::checked_by("T18(a)")`
  names it as the route's check, `showcase-egui/info.rs:73-99`): kde-breeze light gives
  `Some(Color32::from_rgba_unmultiplied(0x23, 0x26, 0x29, 0x20))`; a theme with the
  field cleared gives `None`.

### B.8 iced

`CI/styles.rs`, a new function after `button_toggle` (`:270-306`):

```rust
pub fn toolbar_button(
    resolved: &ResolvedTheme,
    checked: bool,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>
```

with `#[must_use = "this returns the style function; it does not apply it"]`. It reads
`toolbar.button_*` and, for each unstated one, the leaves of the showcase's
`ghost_button` (B.6), whose behaviour it takes over:

| Status | background | text_color | border |
|---|---|---|---|
| `Active`, not checked | `Color::TRANSPARENT` | `button.font.color` | `outline`, transparent |
| `Hovered`, not checked | `button_hover_background`, else `button.hover_background`, as given | `button.hover_text_color` | `outline` in `button_hover_border_color` where stated, else `outline`, transparent |
| `Pressed`, not checked | `button_active_background`, else `button.active_background`, else `button.hover_background`, as given | `button.active_text_color` | as `Hovered` |
| `Active`, checked | `button_checked_background`, else the pressed fill above | `checked_text`: where `button_checked_background` is stated, `button.checked_text_color`, else `button.active_text_color`; where it is not, `button.active_text_color` | `outline`, transparent |
| `Hovered` or `Pressed`, checked | as `Active`, checked (decision 5) | `checked_text` | as `Hovered` |
| `Disabled` | `Color::TRANSPARENT` | `button.disabled_text_color` | `outline`, transparent |

`outline` is `Border { width: button.border.line_width, radius: button.border.corner_radius.into(), .. }`;
iced lays a button out by its padding and draws the border on its bounds
(`IW/button.rs:246`, `:379-385`), so the width does not move the content. Every status
emits `Some(Background::Color(..))`, as `styles::button` does (`:237-243`), so the
contract reads each; `shadow` and `snap` are `button::Style::default()`'s. Doc comment:
what it replaces (no iced toolbar; the showcase's flat button), the table above in
prose, and "A stated fill is painted as given over the toolbar: translucent is a layer,
transparent nothing". README (`connectors/native-theme-iced/README.md:189`, one line,
in place): `toolbar_button` joins the *with a status* row's functions after
`button_toggle`, and the row's last cell, after "`button_toggle(&resolved, checked)` is
a toggle button, the plain button while off;", gains "`toolbar_button(&resolved,
checked)` is a toolbar's button, flat at rest, in the toolbar's states where the theme
states them;", as the steps 6–7 specification's 6.E.4 glosses its `button_default` in
the same cell. The crate doc's coverage table, the `styles` row
(`src/lib.rs:318`, one line, edited in place): "(33 functions)" becomes "(34
functions)" and "toggle button, text input" becomes "toggle button, toolbar button,
text input". The count is the top-level `pub fn`s of `src/styles.rs` but
`composite_over`: 33 today (34 lines `^pub fn`; the 35th `pub fn`, `:1747`, is
`of` in an `impl`), so 34 (`^pub fn` 34 → 35). The same
count in `ROADMAP.md:162` ("has 33 style functions", in place): 34; and the same
paragraph's list of the functions it names (`ROADMAP.md:159-178`, *Full theme geometry
in the iced connector*) gains, after "`styles::button_toggle` is a toggle button,"
(`:171`), "`styles::toolbar_button` a toolbar's button,", rewrapped to the end of the
paragraph. Both in the iced unit (U3). The steps 6–7 specification's 6.E.4 then adds its `button_default` on
top ("theirs plus one", "a dialog's default button" after "toolbar button").

**Showcase** (`examples/showcase-iced.rs`): `IconButton` (`:5326-5344`) gains
`toolbar: bool` ("styled as a toolbar's button, `styles::toolbar_button`"); `icon_button`
(`:5346-5368`) styles with `styles::toolbar_button(resolved, spec.selected)` where it is
true, `ghost_button` otherwise; `toolbar` (`:5638-…`) and the Basic page's icon row
(`:11443-…`) set it, the status bar's toggle and the page menu do not. `ghost_button`'s
doc (`:4625-4633`) gains "The toolbar's buttons take `styles::toolbar_button`, which
falls back to these leaves."

This switch, and the routes and checks below, land in the iced commit with the
connector's `styles::toolbar_button` (U3, C.1), not later:
`styles_cover_every_widget_shown` (`showcase-iced.rs:20919`, the check at
`:20971-20981`) requires a call site in the showcase for every `pub fn` of
`src/styles.rs`, so the connector's new
function without its use is red. The same test's `DRESSED` table (`:20793-20809`) lists
the style functions a `button(..)` may wear; `"styles::toolbar_button"` joins its button
styles, after `"styles::button_toggle"`.

**The showcase's Widget Info routes for the tool buttons.** Today the six tool buttons
take their lines from the hand routes of `ghost_button`: `drawing()` has no arm for
them (only `id == "chrome.toolbar"`, `:7464`; `None` at `:7470`), so `colour_route`
(`:7569`) gives up, and `iced_route` routes them as members of `ghost` (`:7774-7786`),
whose lines name `ghost_button` (`:7798-7890`). The new leaves (three `toolbar.button_*` and
`button.border.line_width_px` on these elements, B.10; `toolbar.button_checked_background`
and `button.checked_text_color` on Edit) have no arm there and would fall to "no route recorded" (`:8545`), which
`the_element_list_parses_and_every_leaf_has_a_route` (`:24982-25000`) refuses. The
routes below land with the switch, before the elements list the new leaves (an arm for
a leaf no element lists yet is read by no test); the parts for Edit (`Shown::basic_edit`,
the `.edit` id, its `ICON_BUTTONS` entry) land with the button in the unit that builds
Edit (U5):

- `Shown` (`:7284-7298`) gains `basic_edit: bool` (`State::basic_edit`, B.10;
  `opening` sets `true`). A predicate `fn tool_button(id: &str) -> bool`, true for the
  ids `chrome.toolbar.command_palette`, `.reload_theme`, `.preferences` and
  `basic.icons.copy`, `.paste`, `.delete`, `.edit` (and false for their `.icon` parts),
  and a constant `TOOLBAR_BUTTON: &[&str] = &["styles::toolbar_button."]` beside
  `BUTTON` (`:7267`).
- `drawing()` (`:7341-7471`) gains, before the `chrome.toolbar` arm: where
  `tool_button(id)` and the leaf is one of the label's colours (`button.font.color`,
  `.hover_text_color`, `.active_text_color`, `.checked_text_color`), `return None`, so
  that the hand lines say whether the icon or the label shows (the icon's colour is set
  as it loads, `icon_colour`, `:4613-4623`), as they do today; for Edit,
  `Drawing::only(TOOLBAR_BUTTON, &["(true, "], &["Disabled"])` while `shown.basic_edit`
  and `&["(false, "]` otherwise; for the six others
  `Drawing::only(TOOLBAR_BUTTON, &["(false, "], &["Disabled"])` (the statuses are the
  contract's `(checked, Status)` pairs, `Debug`-printed as `(true, Active)`;
  `draws_status` matches by substring, `:7261-7264`). Their fills and outline are then
  routed from the export's `styles::toolbar_button` rows.
- `iced_route` (`:7749-…`): the `ghost` set (`:7777-7783`) loses `chrome.toolbar.` and
  the three `basic.icons.` ids, and a `tool = tool_button(id)` flag joins it:
  `ghost_icon(state, id).filter(|_| ghost || tool)` (`:7787`; `ghost_icon`,
  `:7646-7660`, gains `"basic.icons.edit" => icons.edit.as_ref()`); `selected` is
  `state.basic_edit` for Edit; `class` is `"styles::toolbar_button"` where `tool`. The
  label-colour arms of `ghost` (`("button", "font.color")`, `"hover_text_color"`,
  `"active_text_color"`, `:7801-7863`) apply to `tool` too, with `styles::toolbar_button`
  named where they name `ghost_button`, and a new arm `("button", "checked_text_color")`
  where `tool`: the icon's colour while checked where an icon shows, else
  "styles::toolbar_button: the checked label's text_color, where
  toolbar.button_checked_background is stated" (and "not drawn: the toolbar's checked
  fill is unstated, so the label is button.active_text_color" where it is unset, "not
  drawn: button.checked_text_color is unstated, so the label is
  button.active_text_color" where the fill is stated and the text is not), and for a
  tool button that is not checked: "not drawn: the button is not checked".
  **A checked tool button's other label colours** (B.8's table: in every enabled status
  its label and its bundled icon are the checked text, `button.checked_text_color` where
  `toolbar.button_checked_background` and it are both stated, else
  `button.active_text_color`): for a checked tool button, the arms
  `("button", "font.color")` and `("button", "hover_text_color")` read "not drawn: the
  checked button is lettered button.checked_text_color" where both are stated
  (kde-breeze, adwaita and windows-11 state both, B.4), and "not drawn: the checked
  button is lettered button.active_text_color" otherwise; `("button", "active_text_color")`
  reads the former where both are stated, and its drawn line (the icon's or the label's
  colour while checked) otherwise. So `route_faults` (`:25505-25512`) finds no route that
  claims a colour the checked button does not draw: splicing `active_text_color` into a
  theme that states both moves nothing there. The
  arms `("button", "hover_background")` and `("button", "active_background")`
  (`:7820-7850`) are not reached for these ids any more (the export routes them).
  `("button", "border.line_width_px")` (`:7880-7882`) reads, where `tool`,
  "styles::toolbar_button: button::Style::border.width, the outline under the pointer,
  drawn inside the bounds" (iced draws a border on its bounds, `IW/button.rs:379-385`).
  The padding arm (`ghost_padding`, `:7886-7888`) stays for them, since the showcase
  still pads them so; its guard, `side(f).is_some() && ghost` (`:7886`), becomes
  `side(f).is_some() && (ghost || tool)`, as `ghost` no longer holds the tool-button
  ids, which would otherwise fall to the next arm. Its "(a flat button draws no border)" is false for a tool
  button whose theme states the outline: where `tool`, it reads "(the outline under the
  pointer is drawn over it)". The function's own doc (`:4663-4665`, "No line width is
  added: a flat button draws no border.") becomes: "The padding of a flat button: the
  sides `button.border.padding` states, iced's own button padding where it states none.
  No line width is added: a flat button draws no border at rest, and a toolbar button's
  hover outline (`styles::toolbar_button`) is drawn on its bounds, over the padding, so
  it moves nothing."
- `icon_colour` (`:4613-4623`) takes a third parameter, `toolbar: bool`: for a selected
  toolbar button it is the checked text of B.8's table, so the icon and the label agree
  (B.10); `toolbar: false` keeps today's colours, for the side-panel toggle and the page
  menu. Its callers pass it: `icon_button` (`:5346-5368`) `spec.toolbar`, and the tests'
  `ghost_drawn` and `drawn_by` below.
- `every_hand_colour_route_names_a_function_that_reads_it` (`:29102-29179`) draws each
  function a hand route names through `drawn_by` (`:29008-29075`), and fails a function
  it cannot draw. `drawn_by` gains the arm
  `"styles::toolbar_button" => [false, true].map(|checked| buttons(&styles::toolbar_button(r, checked))).join("|")`
  (its `buttons` closure formats each status, `:29016-29022`), and its `"icon_button"` arm
  becomes the four `(selected, toolbar)` pairs,
  `[(false, false), (true, false), (false, true), (true, true)].map(|(s, t)| icon_colour(r, s, t))`,
  so that the toolbar case's colour moves where its leaves do; `ghost_drawn`'s call of
  `icon_colour` passes `false`.
- The tests' oracle (`showcase-iced.rs:25300-25520`): `ICON_BUTTONS` (`:25300-25318`)
  gains, in U5, `("basic.icons.edit", "basic.icons.edit.icon")`; a
  `toolbar_drawn(r, checked, shows)` beside `ghost_drawn` (`:25367-…`) reads
  `styles::toolbar_button(r, checked)`'s fill and border colour in `Active`, `Hovered`
  and `Pressed`, and, as `ghost_drawn` does, the label's colour only where the label
  shows (`Shows::Label`) and the icon's, `icon_colour(r, checked, true)`, where a bundled
  set's icon shows; in
  `route_faults` (`:25461-…`) the flat-button check (`:25490-25514`) uses
  `toolbar_drawn` for the tool buttons (checked for Edit while `state.basic_edit`) and
  `ghost_drawn` for the page menu and the side-panel toggle, over `BUTTON_COLOURS`
  (`:25322-25328`) plus, for the tool buttons, `button.checked_text_color` and the four
  `toolbar.button_*` leaves. `a_route_names_a_widget_only_where_the_state_draws_it`
  (`:25568`) and `the_element_list_parses_and_every_leaf_has_a_route` then hold the new
  lines.

**Contract.** In the pattern of `button_toggle`: `TOOLBAR_BUTTON_STATUSES` (every
`button::Status`, off and on, as `BUTTON_TOGGLE_STATUSES`, `rows.rs:384-393`),
`native_toolbar_button(r, (checked, status)) -> (Color, Color, Color)` (fill, label,
border colour) per the table, `TOOLBAR_BUTTON_ROWS` (`styles::toolbar_button.background`,
`.text_color`) and `TOOLBAR_BUTTON_BORDER_ROWS`, registered in the field lists
(`rows.rs:2914-2971`); `toolbar_button_style_fields` beside `button_toggle_style_fields`
(`contract.rs:1278-1287`) and its call in the walk (`:1738-1742`); `DERIVED` entries for
`styles::toolbar_button.shadow` and `.snap` as the toggle's (`derived.rs:816-827`);
`assert_clone(&styles::toolbar_button(r, true))` (`contract.rs:4749`); a pair
`TOOLBAR_BUTTON_PAIRS` (`pairs.rs`, as `BUTTON_TOGGLE_PAIRS`, `:168-190`) with the label
over the fill composited over `toolbar.background_color`. `contract-leaves.toml`
regenerated: the four leaves move rows of `styles::toolbar_button` only, and
the colour leaves `button.hover_background`, `button.active_background`,
`button.font.color`, `button.hover_text_color`, `button.active_text_color` and
`button.disabled_text_color` gain `styles::toolbar_button` rows, the two fills with
`except` lists for the presets that state the toolbar field. The export holds colour
leaves only, so `button.border.line_width`, the outline's width, has no entry there; its
Widget Info line is the showcase's hand route (below). `button.checked_text_color`
moves `styles::toolbar_button.text_color` under the checked statuses, `except` the
combinations that state no `toolbar.button_checked_background` (all but kde-breeze,
adwaita and windows-11), and `button.active_text_color` moves it there under the other
combinations. Tests (`CI/contract.rs`, beside `a_cleared_widget_soft_option_copies_the_base_state_value`):
`a_checked_toolbar_button_takes_the_checked_text`: on windows-11 light and dark the
checked `Active` text is `button.checked_text_color`, #ffffff / #000000, and the fill
`toolbar.button_checked_background`; with the toolbar's checked fill cleared, the fill
is the pressed fill and the text `button.active_text_color`; with
`button.checked_text_color` cleared and the fill kept, the text is
`button.active_text_color`. `style_contrast_never_degrades_the_native_pair` (`:3506`)
covers the new pair.

### B.9 gpui

**The variant** (`CG/variants.rs`, after `hover_border`, before the test module, so
lines 62-64 and step 3's four claimed lines stay, *How to read it*):

- `pub(crate) struct ToolbarButtonStates { pub hover: Option<Hsla>, pub hover_border: Option<Hsla>, pub active: Option<Hsla> }`
  (`Debug`, `Clone`, `Copy`, `PartialEq`) and
  `pub(crate) fn toolbar_button_states(n: Native<'_>) -> ToolbarButtonStates` reading
  `toolbar.button_hover_background`, `.button_hover_border_color`,
  `.button_active_background` through `rgba_to_hsla`; `hover_border` takes step 3's
  route (D5; Part 3F of
  [`todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md`](todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md)).
  The struct and `toolbar_button_states` come first, before `toolbar_button`: the
  claims of `native_toolbar` (below) cite the lines of `toolbar_button_states` that
  read the fields, from U2 on, and the unit that builds Edit adds a sentence and a doc
  link to `toolbar_button`'s doc, which would move them if they lay below it.
- The public function:

  ```rust
  /// A toolbar's flat button: [`ghost_button`] with the platform's toolbar
  /// button states where the installed native theme states them.
  #[must_use]
  pub fn toolbar_button(cx: &App) -> ButtonCustomVariant
  ```

  It starts from `ghost_button(cx)`; where `cx.native()` gives a theme, `.hover(..)` takes
  `states.hover` and `.active(..)` `states.active` where `Some`; `None` and no native
  theme keep the tokens. A custom variant fills a selected button with `active`
  (`GC/button/button.rs:1337`), so a checked toolbar button shows the pressed fill
  unless the caller applies `geometry::toolbar_button_checked`. It keeps the form
  `toolbar_button(cx)` of `ghost_button(cx)`, which it extends, rather than step 3's
  `bordered_button(n: Native<'_>, cx: &App)`: that form would change decision 3, this
  section's signature and example, the showcase's three call sites, the README
  paragraph and the CHANGELOG bullet, and would hand every caller the
  case of no native theme, which `toolbar_button` answers by keeping `ghost_button`'s
  tokens; the cost of `(cx)` is the one import line replaced below, which moves no line.
- **The hover outline** follows step 3's route, whose trial passed
  (`variants::bordered_button`): a custom
  variant whose `color` is the hover border, drawn in every state
  (`GC/button/button.rs:1118-1124`), and a caller's refinement that gives the button
  its own border and fill at rest, which upstream applies after the normal colours and
  under its hover style (`:730-764`; step 3's `variants::bordered_button`, its 3F.1).
  - `toolbar_button` also sets `.color(edge)` where
    `states.hover_border` is `Some`, and a new builder in `CG/geometry.rs`, after
    `button_checked`,
    `pub fn toolbar_button_rest(n: Native<'_>) -> StyleRefinement`, gives the caller
    the rest state: where the theme states `toolbar.button_hover_border_color`, a fill
    and a border colour of gpui's `transparent_black()` (`GP/color.rs:444`), the
    border width `w` = `button.border.line_width`, as `geometry::button` sets it
    (`CG/geometry.rs:229`), and each side of `button.border.padding` the theme states
    less `w`, floored at 0; an empty refinement where it states none (the variant's
    `color` then stays transparent, as `ghost_button`'s). The padding is reduced because
    gpui lays a border out (`GP/taffy.rs:506`) where a custom variant has none
    (`GC/button/button.rs:724-729`): without it a toolbar button whose theme states the
    outline would grow by `2w` in each axis, at rest too. A side the theme leaves unset
    is not set: an unset left or right side keeps the padding the caller or upstream
    gives, and the border makes the button `w` wider there; where neither the top nor
    the bottom is stated, the showcase's box keeps upstream's fixed height
    (`GC/button/button.rs:698-706`, `h_6` / `h_8` by size; `tool_box_of` sets `h_auto`
    only where one is, `showcase-gpui/demo.rs:9149-9153`),
    and the border lies inside it. kde-breeze, the one preset that states the outline,
    states all four sides (`padding_horizontal_px`, `padding_vertical_px`,
    `kde-breeze.toml:157-158`, `:742-743`). The builder's doc says that it assumes the
    caller pads the button with `button.border.padding` (as `geometry::button` and the
    showcase's `tool_button_box` do): on a button padded otherwise it replaces the
    padding of each stated side with that side less `w`. Upstream replays the refinement in the
    selected and disabled states too (`GC/button/button.rs:821-838`), so a checked
    toolbar button takes `toolbar_button_checked` after it. Its row joins the README's
    geometry table, after `button_checked`'s, and the contract's refinements. Its unit test in `geometry.rs`,
    `toolbar_button_rest_keeps_the_padding_box`: under kde-breeze light the refinement's
    padding is the stated 6 less `button.border.line_width` on each side and its border
    width that width; under macos-tahoe it is empty.
- `variants::ghost_button` stays as it is (D7). The module doc (`:1-27`) is not
  touched: a line added there would move the claimed lines 62-64 (*How to read it*).
  What it would have said goes into `toolbar_button`'s own doc, after `hover_border`,
  before the test module:
  why a toolbar button is a variant of its own (the platform states a
  toolbar button's hover, pressed and checked colours apart from a flat button's), that
  a checked one takes `geometry::toolbar_button_checked` (a sentence and doc link added
  in the unit that builds Edit, with the builder, so that no link dangles before it),
  that it goes with its companion `geometry::toolbar_button_rest` and why
  (where the theme states the outline, the variant's `color` is the edge colour, which
  upstream mixes toward transparent into the button's fill at rest,
  `colors.color.mix_oklab(cx.theme().transparent, 0.2)`, `GC/button/button.rs:1028`,
  and draws with no border width of its own, `:724-729`, so without the rest
  refinement the button is tinted at rest and no edge shows), and that such a Button goes
  into gpui-component's `Toolbar` or `ToolbarGroup` by `content(..)`, not `child(..)`:
  `child` prepares a Button for the toolbar by `.ghost().compact()`
  (`GC/toolbar.rs:17-22`, `:71-74`; `GC/button/button.rs:590-592`), which replaces the
  custom variant (`with_variant`, `:596-599`). The showcase adds its toolbar items with
  `content` already (`demo.rs:1424`). The README paragraph of B.9 says the same. The
  lines above are 0.9.1's; the doc cites gpui-component 0.10.0's: the mix at rest
  `button/button.rs:1044`, the border widths `:727-732`, `Toolbar::child`
  `toolbar.rs:17-22`, `:71-74` with `button/button.rs:597-599`, `:603-606`.
- `toolbar_button` calls `cx.native()`, a method of the `ActiveNativeTheme` trait
  (`CG/lib.rs:1041`), which the module does not import: `use crate::Native;`
  (`variants.rs:34`) becomes `use crate::{ActiveNativeTheme as _, Native};`, one line
  for one, so nothing above `:62` moves, nor step 3's claimed lines below it. The new
  code goes after `hover_border`, before the test module.

**The checked fill** (`CG/geometry.rs`, after `toolbar_button_rest`, which follows
`button_checked`, `:1354-1378`: the order they are applied in). It lands
in the unit that builds Edit (U5) with the Edit button, its only user: `the_showcase_exercises_every_builder`
(`CG/showcase.rs:384`) requires every `pub fn` of `geometry.rs` and `variants.rs` to be
referenced in the showcase, so a builder no showcase code calls is red (C.1).

```rust
/// The fill and text colour of a toolbar Button that is on
/// (`Button::selected(true)`), over [`variants::toolbar_button`](crate::variants::toolbar_button):
/// `toolbar.button_checked_background`, lettered `button.checked_text_color`
/// (`button.active_text_color` where that is `None`). Where the theme states
/// no checked fill the fill is the pressed one, `toolbar.button_active_background`,
/// else `button.active_background`, as [`button_checked`] falls back, and the
/// text is left to the custom variant's one foreground. Where it states none
/// of the three the refinement sets nothing: the custom variant's selected
/// fill, its pressed one, stands under its one foreground, unless the caller
/// applied [`toolbar_button_rest`], whose transparent fill upstream replays
/// over the selected state too, so that a checked button then shows no fill.
/// Apply it to a selected toolbar Button only: upstream replays the
/// refinement at rest too.
#[must_use]
pub fn toolbar_button_checked(n: Native<'_>) -> StyleRefinement
```

No bundled preset is in that case: kde-breeze, the one that states the outline, states
the checked fill. A unit test in `geometry.rs`, in U5 with the builder whose doc it
backs, `every_preset_with_a_toolbar_outline_states_its_checked_fill`, walks the bundled
presets in both modes and asserts that each that states
`toolbar.button_hover_border_color` states `toolbar.button_checked_background`, with the
message "a checked toolbar button would show no fill in gpui (toolbar_button_rest's
transparent fill is replayed over the selected state)".

Its `bg` is `toolbar.button_checked_background`, else
`toolbar.button_active_background`, else `button.active_background`, in the form of
`button_checked`'s `b.checked_background.or(b.active_background)` (`:1369-1378`); it
sets `text_color` only where `toolbar.button_checked_background` is stated (decision
11), and nothing where none of the three fills is. Under the bundled presets the
fallback changes nothing: material-3's and catppuccin-mocha's `button.active_background`
is what the variant's `secondary_active` already is, and macos-tahoe states none of the
three, so its refinement stays empty. A custom variant has one
foreground for every state (`GC/button/button.rs:1080`), so the refinement is the only
receiver of the checked text; upstream replays it in the selected state over the
variant's (`:821-829`). The showcases' toolbar buttons are icons, so what matters is
whether a Button's icon takes the refinement's `text_color` as its label does. A test cannot
read an icon's painted colour, since gpui-pre 0.3.12's test API gives a frame's painted
quads and underlines only (`GP/window.rs:3112-3120`, `painted_quads`,
`painted_underlines`; the gpui showcase's tests say so, `showcase-gpui/tests.rs:8733-8735`,
`:8882-8884`). It reads what decides the colour instead: gpui-component paints an
`Icon` with no colour of its own in `window.text_style().color` (`Icon::render`,
`GC/icon.rs:260-266`, through `into_svg`, whose `self.text_color.unwrap_or(fallback)`
is `:213-214`). So a test of the unit that builds Edit (U5), `a_checked_toolbar_buttons_icon_takes_the_checked_text`,
a lib test in `connectors/native-theme-gpui/src/widgets/tests.rs`, where the harness
is (`TestAppContext`, `add_window_view`, `:805`) and no runner environment is needed
(`widgets` is in the crate's `default` feature, `connectors/native-theme-gpui/Cargo.toml`, which the
gate's gpui leg builds, `scripts/check_release.sh:667`), written from
`<record>/steps45/trial/t2.patch` with the two real builders: on windows-11 light and
dark it builds a selected Button with `variants::toolbar_button(cx)` and
`geometry::toolbar_button_checked(n)`, as the Edit button is built, puts a probe child
inside it that records `window.text_style().color` in `render`, the moment
`Icon::render` reads it (the test's doc cites gpui-component 0.10.0 `icon.rs:260-266`
and `:213-214`), records and prints
every case first, one line per case (preset, mode, recorded colour, checked text), and
then asserts once that each recorded colour is `button.checked_text_color` (#ffffff /
#000000), the assertion's message naming each case that differs, so that a colour that
does not follow is seen on the first run. It runs with
`cargo test -p native-theme-gpui --features native,watch --lib widgets::tests::a_checked_toolbar_buttons_icon_takes_the_checked_text`,
and on GPUI Fast with `--features gpui-fast,native,watch` (the crate's `gpui-fast` feature,
`connectors/native-theme-gpui/Cargo.toml`; the unit gates run the GPUI Fast legs
(`<tools>/gate.sh --fast`), as the full check does).
Measured before the build (T2): the colour follows. If
`a_checked_toolbar_buttons_icon_takes_the_checked_text` fails in the unit that builds
Edit, the unit stops and the difference goes to the maintainer.

The gpui README's geometry table gains its row after `toolbar_button_rest`'s, which
follows `button_checked`'s (`README.md:364` row 25 of the table; in U5), and the *Flat buttons* section (`:409-431`) a paragraph on
`variants::toolbar_button(cx)` with a two-line example, which adds the Button to a
`Toolbar` by `content(..)` and says why (the variant's doc above). The paragraph also
says, in this order: "refine the button with
`geometry::toolbar_button_rest(n)`, which assumes it is padded with
`button.border.padding`, and a checked one after that with
`geometry::toolbar_button_checked(n)`: upstream replays every refinement over the
selected state (gpui-component 0.10.0 `button/button.rs:834-842`), so the rest
refinement's transparent fill applied last would cover the checked fill"; the sentence
on `toolbar_button_checked` lands in the unit that builds Edit with the builder, until
which the paragraph names `toolbar_button_rest` alone. The builders' docs say the same
order (`toolbar_button_checked`'s doc above; `toolbar_button_rest`'s gains "Apply it
before `toolbar_button_checked`" in the unit that builds Edit).

**`GEOMETRY_NOTES`** (`showcase-gpui/info/mod.rs:214-…`, one entry per `pub fn` of
`geometry.rs`, in its order; `every_geometry_builder_has_a_note`, `CG/showcase.rs:572`),
after `"button_checked"` (`:384-387`), in the form of that entry, in the order of the
builders:

- `("toolbar_button_rest", "where the theme states toolbar.button_hover_border_color, a transparent fill and a transparent border button.border.line_width wide, and each stated side of button.border.padding less that width: a custom variant draws its colour as the border in every state and lays out no border width (button/button.rs, ButtonCustomVariant and RenderOnce for Button), so this hides the edge at rest and keeps the button's size; nothing where the theme states no outline")`,
  in U2;
- `("toolbar_button_checked", "toolbar.button_checked_background and button.checked_text_color (button.active_text_color where that is unset) on a selected toolbar Button built on variants::toolbar_button; where the theme states no checked fill, toolbar.button_active_background, else button.active_background, with no text colour; nothing where it states none of the three: a custom variant fills a selected Button with its active colour and letters every state in its one foreground (button/button.rs, ButtonCustomVariant and RenderOnce for Button), and the Button replays the refinement over its selected style")`,
  in U5, after it.

**Contract** (`CG/contract.rs`): the walk's `sinks` (`:2736-…`) records
`format!("{:?}", variants::toolbar_button_states(n))` (U2) and adds
`("toolbar_button_checked", gm::toolbar_button_checked)` to its refinements
(`:2790-2822`, U5), so the four leaves reach a sink.
Between the two units `toolbar.button_checked_background` reaches none, so U2 adds
an `UNREAD_COLOURS` entry (`:1095-…`):

```rust
UnreadColour {
    leaf: "toolbar.button_checked_background",
    why: Unread::NoReceiver(
        "a custom variant fills a selected Button with its `active` colour, the pressed \
         fill (gpui-component 0.10.0 `button/button.rs:1353`, `ButtonVariant::selected`), \
         and takes no checked colour of its own; until the connector refines a selected \
         toolbar Button, a checked one shows the pressed fill",
    ),
},
```

The gate requires a `NoReceiver` reason to cite an upstream `file.rs:line`
(`CG/contract.rs:3027-3039`); `:1353` is the `Self::Custom(colors) => colors.active`
arm of `ButtonVariant::selected` at gpui-component 0.10.0 (`:1332-1372`,
`<record>/steps45/upstream-reread.md`). Of the three kinds (`:1044-1064`),
`Inherited` (a widget leaf that inherits it reaches a sink) and `Fallback` (a soft
option's fallback every preset states) do not describe it; `NoReceiver` is taken, as the
nearest: the variant has nowhere to take the checked colour, and the receiver the
connector gives it, a refinement over the selected state, comes with the Edit button in
U5, which removes the entry and adds the refinement's sink. Gate:
`every_colour_leaf_reaches_a_sink_or_is_named`. A unit test in `variants.rs`:
`toolbar_button_takes_the_toolbars_states` (kde-breeze: hover transparent, active
#3daee94d; macos-tahoe: the `ghost_button`'s tokens), U2. A unit test in
`geometry.rs`, U5, with the builder:
`toolbar_button_checked_letters_in_the_checked_text` (windows-11 light: bg #0067c0,
text #ffffff; dark: #4cc2ff, #000000; kde-breeze light: #23262920, #232629; macos-tahoe:
an empty refinement; windows-11 with `button.checked_text_color` cleared: the text is
`button.active_text_color`).

**Showcase** (`examples/showcase-gpui/`): `demo::toolbar_button` (`demo.rs:1329-1382`),
`toolbar_item` (`:1452-…`, the Buttons page's component `Toolbar`) and
`demo::icon_button` (`:9210-…`, the Basic page's icon row) build
`Button::new(id).custom(variants::toolbar_button(cx))` in place of
`ButtonKind::Ghost.apply(..)`, refined with
`geometry::toolbar_button_rest(n)` after `tool_button_box`'s refinement, so that its
reduced padding wins over the box's (a later refinement's stated fields replace an
earlier one's). A test in
`tests.rs`, `a_tool_buttons_box_keeps_its_size_with_the_outline`: under kde-breeze light
the bounds of `basic-icon-copy` equal its bounds under the same preset installed with
`toolbar.button_hover_border_color` cleared (where `toolbar_button_rest` is empty), so
the outline moves nothing. A second test there, in the pattern of
`a_hovered_field_takes_the_hover_border`, is the one check that the edge is drawn:
`a_hovered_tool_button_takes_the_toolbars_outline`: kde-breeze light,
`basic-icon-copy`; pointer away: no bordered quad over its bounds has
`toolbar.button_hover_border_color`; hovered: `painted_edge` is that colour, and
`painted_fill` is `None` (the hover fill is transparent). The checked
toolbar button of B.10 is built the same way, `.selected(true)` while it is on, with
`geometry::toolbar_button_checked(n)` applied after the other refinements while it is
on. Widget Info: a new
`info::buttons::native_toolbar(info, t, native)` beside `native_ghost`
(`info/buttons.rs:625`) claims the hover, pressed and hover-edge
colours, each the toolbar field where stated with its leaf, else the ghost token's
claim; the hover edge through `edge_claim("border, hovered or pressed", "button_hover_border_color",
<toolbar.button_hover_border_color>, Some(<button.border.line_width>), <the cited line of
toolbar_button_states>)`, the width the rest refinement draws it at (A.10 says why
`edge_claim`); `info/chrome.rs::toolbar_button` (`:1013-1053`), `info::buttons::toolbar_item`
and `info::buttons::icon_button` call it in place of `native_ghost`, and their
`variant` line names `variants::toolbar_button`. To read the `toolbar.*` leaves, each
of the three takes the resolved theme where it takes the button's today: their
`native: Option<&ResolvedButtonTheme>` (`info/chrome.rs:1013-1021`, `info/buttons.rs:948`,
`:969-974`) becomes `native: Option<&ResolvedTheme>`, from which they read `button` and
`toolbar` (a parameter of its own would give `info::toolbar_button`, seven today, eight,
over clippy's `too_many_arguments` limit); the three callers that pass
`native.as_ref().map(|n| &n.resolved.button)` to `info::toolbar_button`,
`info::buttons::toolbar_item` and (in `demo::icon_button`) `info::buttons::icon_button`
pass the resolved theme; the fourth such call, `panel_toggle`'s, keeps the button's;
and the tests that call them follow:
`every_tool_buttons_size_note_follows_the_stated_padding`'s `infos` closure
(`tests.rs:17416-17459`, the calls at `:17422`, `:17450`, `:17456`, its parameter
`Option<&ResolvedButtonTheme>` becoming `Option<&ResolvedTheme>`, and its caller's
`Some(&r.button)`, `:17463`, `Some(r)`; the closure's fourth call, `info::panel_toggle`
for the status bar's toggle (`:17428-17444`, `info/chrome.rs:482`), keeps its
`Option<&ResolvedButtonTheme>` and is passed `native.map(|r| &r.button)` at `:17443`) and `tests.rs:17822`, which sets the padding on
a clone of the resolved theme's `button` (`:17817`) and so clones the resolved theme
instead. `icon_button`'s
`not_themeable("edge", "none: upstream gives a border width only to a Default or an
outlined Button, …; the showcase sets none")` (`info/buttons.rs:954-957`) is false where
the theme states the outline, since `toolbar_button_rest` gives the button a border; it
then reads `not_themeable("edge", "none at rest: geometry::toolbar_button_rest's border
is transparent, button.border.line_width wide, and takes the variant's colour under the
pointer and while pressed (the border, hovered or pressed)")`, and stays as it is where
the theme states no outline. The gpui showcase test
`the_basic_pages_icon_buttons_claim_no_edge` becomes
`the_basic_pages_icon_buttons_claim_no_edge_at_rest`: under kde-breeze light, the Copy
button's claims whose role starts with "border" are exactly one, of role "border,
hovered or pressed", field `button_hover_border_color`, value
`toolbar.button_hover_border_color`; under windows-11 light, which edges its buttons
(the test's own precondition, asserted there too) and states no toolbar outline, no
claim's role starts with "border"; the `geometry::button` assertion stays, under both.
Its doc says so. No Widget Info text of a tool button contains `geometry::button` (the
test's second assertion): the new texts name `geometry::toolbar_button_rest` and
`geometry::toolbar_button_checked` only. The checked button's info adds the
claims "checked fill" (`button_checked_background`) and "checked text"
(`checked_text_color`, or `active_text_color` where unstated) at the lines of
`toolbar_button_checked` that set them.
`info/leaves.rs`: for the prefixes `"chrome.toolbar."` and `"basic.icons."` the
`button.hover_background` and `button.active_background` lines (`:144-205`) say
"gpui: variants::toolbar_button's hover where toolbar.button_hover_background is unset,
ThemeColor secondary_hover (colors.rs)" and the pressed equivalent; new lines, each an
`(element-id prefix, leaf, line)` entry of `HOW` (`info/leaves.rs:11-15`), name the three
toolbar leaves the six tool buttons list, each under the prefix `"chrome.toolbar."` and
again under `"basic.icons."`, in the block of the two prefixes' `ghost_button` lines
(`:144-205`): `toolbar.button_hover_background` "gpui: variants::toolbar_button's hover",
`toolbar.button_active_background` "…'s pressed fill", and
`toolbar.button_hover_border_color` "…'s edge under the pointer"; under the prefix
`"basic.icons.edit"`, listed before the `"basic.icons."` lines
so that Edit's own entry is chosen first, `toolbar.button_checked_background` "gpui:
geometry::toolbar_button_checked's bg, over the selected toolbar Button", which lands
with the Edit button in U5, the one element that lists the leaf (no interim line is
written in U2, where no element lists it; the interim `UNREAD_COLOURS` entry of U2,
above, stays until U5, the leaf walk reading every leaf); one
line for `button.border.line_width_px` under the same two prefixes, which B.10 adds to
the tool buttons' leaves and whose generic line, "geometry::button's border width"
(`info/leaves.rs:306-310`), is not a tool button's route ("gpui:
geometry::toolbar_button_rest's border width, where toolbar.button_hover_border_color is
stated"), and
under `"basic.icons.edit"` too the line for `button.checked_text_color`
("gpui: geometry::toolbar_button_checked's text colour, where the toolbar's checked fill
is stated"), which lands with the Edit button (U5, C.1): before it no element
reaches the line. Widget Info shows for a listed leaf the first `HOW` entry whose prefix
the element's id starts with (`info/leaves.rs:11-14`; `chosen_how`, `CG/showcase.rs:3882`);
`every_how_entry_is_shown` (`CG/showcase.rs:3893`, in the connector's own tests through
`CG/lib.rs:243-244`) fails on an entry no element reaches, and the showcase's
`every_listed_leaf_has_a_route` (`showcase-gpui/tests.rs:15198`) on a listed leaf with no
entry; and `the_showcase_exercises_every_builder` (`CG/showcase.rs:384`) fails on a
`pub fn` of `variants.rs` or `geometry.rs` the showcase does not reference. So the
connector's B.9 code, the showcase's switch to `variants::toolbar_button`, the claims of
`native_toolbar` and the existing `HOW` lines' new texts land in one unit (U2); the new
`HOW` lines land with B.10's element edits and what concerns the checked button in U5
(C.1).

### B.10 Showcases: the element list and egui's tool button

- `docs/showcase-elements.toml`: three of the four leaves, `toolbar.button_hover_background`,
  `.button_hover_border_color` and `.button_active_background`, in registry order after
  `"toolbar.icon_size_px"` (the checked fill, with `button.checked_text_color`, is listed
  on Edit alone, U5, the one tool button that is ever checked), and
  `"button.border.line_width_px"` after
  `"button.border.corner_radius_px"` (the hover outline's width, drawn where the theme
  states the outline), in the elements `chrome.toolbar.command_palette`,
  `.reload_theme`, `.preferences` (`:305`, `:321`, `:337`) and `basic.icons.copy`,
  `.paste`, `.delete` (`:1572` and the two after it).
- egui (`examples/showcase-egui/demo.rs:289-314`): `tool_button(ui, padding, edge: Option<f32>)`;
  where `edge` is `Some(w)`, `w > 0`, the `hovered` and `active` states' `bg_stroke.width`
  is `w` and the others' stay 0, as today; the padding is not changed, and the button's
  size does not move with the stroke (decision 6). `None` keeps today's zero width in
  every state. Callers in the toolbar scope pass
  `t.toolbar.button_hover_border_color.map(|_| t.button.border.line_width)`: the
  chrome's toolbar (`chrome.rs:588`) and the Basic page's icon row
  (`pages/basic.rs:1253`); the status bar's toggle (`chrome.rs:766`) and the page menu
  (`:1390`) pass `None`. The doc (`:289-295`), whose "no border … and no border width"
  turns false once the tool buttons list `button.border.line_width_px`, becomes:
  "Make the rest of `ui` a tool button's, the icon-only button of a toolbar, the status
  bar and the tab row's page menu: Ghost ([`ghost`]), padded by `button.border.padding`
  and with no minimum height, so it is its icon and that padding. Where `edge` is
  `Some(width)`, the toolbar's hover outline (`toolbar.button_hover_border_color`, which
  the «toolbar» scope writes as the hovered and active stroke colour) is drawn that wide
  under the pointer and while pressed, at rest none; egui takes a stroke's width from
  the padding inside the frame, so the button's size does not change
  (`Frame::apply_stroke_and_expansion_without_layout_shift`). `None` draws no border in
  any state. egui pads a button by one pair (`Spacing::button_padding`), so the left side
  pads left and right and the top side top and bottom; a side the theme leaves unstated
  keeps the scope's own." The function is named by name; the button's style calls it
  (`egui-0.37.0/src/containers/frame.rs:313`, `theme/default_style.rs:54-72`). Under
  kde-breeze its tool buttons are then #3daee9-framed under the pointer and pressed,
  transparent hovered, #3daee94d pressed. A test in `examples/showcase-egui/tests.rs`,
  `a_tool_buttons_outline_has_the_borders_width`, is the one check that the width is
  set (the colour is the role's, the width the showcase's per call; at width 0 nothing
  is drawn): in a «toolbar» scope after `demo::tool_button(ui, padding, edge)` as the
  chrome's toolbar calls it, under kde-breeze light the `hovered` and `active`
  `bg_stroke.width` are `button.border.line_width` and the `inactive` one 0; under
  adwaita light all three are 0.
- egui's Widget Info routes (read for both steps): a listed leaf's line is its
  `mapping.toml` row's route (`info::listed_how`, `showcase-egui/info.rs:631-648`, then
  `how_line`, `:524-563`), "not reachable: mapping.toml has no row for it" where none is;
  no egui showcase test requires a route per listed leaf. The four toolbar rows (B.7)
  give their leaves theirs, and `button.checked_text_color` keeps its `derived` row's
  (`T18(a)`, whose check `info::checked_by` names, `:73-99`). The one leaf whose row
  would name the wrong route is `button.border.line_width_px` on the tool buttons:
  `tool_button` sets the stroke's width per call, not the «toolbar» style. So
  `PER_CALL_ROUTES` (`info.rs:569-626`) gains, for each of the seven tool-button ids
  (an id that does not end in `row_` names one element, `:637-643`), the entry
  `(id, "button.border.line_width_px", "egui: per call, the outline's width under the
  pointer and pressed, where toolbar.button_hover_border_color is stated; 0 otherwise
  (demo::tool_button)")`, the six in the egui unit (U4; an entry for a leaf no
  element lists yet is read by nothing) and Edit's in U5. Its doc (`info.rs:565-568`,
  which names the List scope's per-call leaves only) becomes: "The leaves a page applies
  per call where `mapping.toml`'s row names a style field the connector writes, by
  element id (a prefix, for the rows) and leaf: the List's and the table's frames, which
  `pages::basic::list_frame` paints, the rows whose fills `pages::basic::inner_corners`
  rounds where they meet a frame, and the tool buttons' outline width, which
  `demo::tool_button` sets per call."
- iced and gpui: B.8 (its Widget Info routes), B.9.
- The parity comparator (`scripts/check_showcase_parity.py`) samples the elements at
  rest, where nothing changes but the checked button below.

**The checked toolbar button: Edit** (decision 13). A fourth tool button in the Basic
page's icon row, after Copy, Paste and Delete, drawn as they are (`styles::toolbar_button`,
the «toolbar» scope, `variants::toolbar_button`), showing `IconRole::ActionEdit` at
`toolbar.icon_size`, or the text "Edit" where the chosen set has no icon for it, as its
neighbours fall back (never another set's icon). It is an "edit mode" switch: on at
start, a click turns it off, the next on; it changes nothing else.

*Elements* (`docs/showcase-elements.toml`, after `basic.icons.delete.icon`, `:1615`, before
`basic.icons.small`, `:1617`):

```toml
[[element]]
id = "basic.icons.edit"
name = "Tool button"
parent = "basic.icons.delete"
leaves = ["button.font.color", "button.hover_background", "button.hover_text_color", "button.active_background", "button.active_text_color", "button.checked_text_color", "button.border.corner_radius_px", "button.border.line_width_px", "button.border.padding_top_px", "button.border.padding_right_px", "button.border.padding_bottom_px", "button.border.padding_left_px", "defaults.icon_theme", "theme_variant.icon_set", "toolbar.icon_size_px", "toolbar.button_hover_background", "toolbar.button_hover_border_color", "toolbar.button_active_background", "toolbar.button_checked_background"]
states = ["Checked"]
sample = { fill = [0.0, 0.5, 1.5, 0.0] }

[[element]]
id = "basic.icons.edit.icon"
name = "Icon"
parent = "basic.icons.edit"
leaves = ["defaults.icon_theme", "theme_variant.icon_set", "toolbar.icon_size_px"]
states = ["Checked"]
part = true
```

The list is `basic.icons.delete`'s (`:1601-1606`) with three additions, each where the
`leaves` order rule of the file's header puts it (`:35-36`: the element's own table in
registry order, then every other table in registry order): `button.checked_text_color`
after `active_text_color` (`docs/property-registry.toml:196`, after the disabled pair,
which the element does not list); `button.border.line_width_px` after
`corner_radius_px`, the hover outline's width (B.8, B.10's egui bullet, B.9's
`toolbar_button_rest`), as the bordered button lists it (`:737`); the four toolbar leaves
after `toolbar.icon_size_px` (`[toolbar]`'s order after B.2). The first bullet of this
section adds the same `button.border.line_width_px` and three toolbar leaves, all but
the checked fill, to the six existing tool buttons. The
`fill` sample is the neighbours' (`:1574`): the comparator then reads the checked fill
in each showcase's capture and compares the three. `basic.icons.small` keeps
`basic.icons.copy` as its parent (`:1619`). Where the row holds four on one line, its
place against Copy does not change; where Edit starts a line the row did not have, the
groups below Icons move down by that line in each showcase (under material-3 the row
has a second line already), and parity holds because the three
wrap by the same rule: an item starts a new line where it does not fit beside what the
line holds (gpui `.flex_wrap()`, `pages/basic.rs:703-704`; iced `.wrap()`,
`showcase-iced.rs:11479`; egui `wrap_before`, `showcase-egui/pages/basic.rs:482-500`),
on the same widths, which the comparator checks for Copy, Paste and Delete today. Under
catppuccin-mocha, which states no button padding, the three toolkits' own paddings
differ, as `docs/showcase-exceptions.toml` records for Copy, Paste and Delete; Edit's
keys join that group where the rehearsal reports them.

*Each showcase's state and code:*

| Showcase | State field (start: on) | Built at | Click |
|---|---|---|---|
| gpui | `Showcase::basic_edit: bool`, beside `basic_radio` (`examples/showcase-gpui/app.rs:453`), `true` where the struct is built | `pages/basic.rs:698-708`: after the three of `ICON_BUTTONS` (`:87-106`, which stays three), a `.child(..)` with `demo::icon_button` for `("basic-icon-edit", IconRole::ActionEdit, "Edit", "basic.icons.edit.icon")`; `demo::icon_button` (`demo.rs:9210-9245`, six parameters, its one caller `pages/basic.rs:707`) would take eight with `selected` and `on_click`, over clippy's `too_many_arguments` limit of seven; so, as `toolbar_button` takes a `ToolbarButton` (`demo.rs:1303-1317`) and `attachment` a `DemoAttachment` (`:5462-5477`), its four per-button values move into a spec struct, `pub(crate) struct IconButton<'a> { pub id: &'static str, pub name: &'static str, pub drawn: &'a (ChromeIcon, Option<IconName>), pub listed: &'static str, pub selected: bool }`, and the function becomes `icon_button(ui, cx, spec: IconButton<'_>, on_click: Option<impl Fn(&ClickEvent, &mut Window, &mut App) + 'static>)`, `on_click`'s form `attachment`'s (`:5477`); the three of `ICON_BUTTONS` pass `selected: false` and `None::<fn(&gpui::ClickEvent, &mut Window, &mut gpui::App)>`, as `pages/data.rs:347` passes none, and no `#[allow(clippy::too_many_arguments)]` is added. The Button takes `.selected(selected)`, `geometry::toolbar_button_checked(n)` after its other refinements where `selected`, and `.on_click(..)` with the handler where it is `Some` (`button/button.rs:459-465`) | `cx.listener(\|this, _, _, cx\| { this.basic_edit = !this.basic_edit; cx.notify(); })`, as `on_radio` is made (`pages/basic.rs:366-369`); `elements.rs:120-122` gains `("basic-icon-edit", "basic.icons.edit")` |
| iced | `State::basic_edit: bool`, beside `basic_radio` (`showcase-iced.rs:1723`, `true` at `:2047`) | the icon row (`:11443-11479`): a fourth `tool(..)`-like item with `selected: state.basic_edit`, `toolbar: true` (B.8) and `action: Message::BasicEditToggled`; `ChromeIcons` gains `edit: Option<IconData>` (`:4519`), loaded `by_role(IconRole::ActionEdit)` (`:4575`), and the lookup `"basic.icons.edit" => icons.edit.as_ref()` (`:7654`) | `Message::BasicEditToggled`, beside `BasicRadioSelected` (`:2653`), handled beside it (`:3009`): `state.basic_edit = !state.basic_edit` |
| egui | `DemoState::basic_edit: bool`, beside `basic_radio` (`examples/showcase-egui/pages/mod.rs:71`, `true` at `:133`) | `icon_buttons` (`pages/basic.rs:1214-1275`) takes `&mut DemoState`, which `group` holds (`:220`) and hands down through its call of `icons_group` (`:245`, today `icons_group(reg, atlas, ui, chosen)`) and `icons_group`'s call of `icon_buttons` (`:1324`): both functions gain a `state: &mut DemoState` parameter after `reg`, as `list_group` takes it (`:1206`); after the loop of three, a fourth item `"basic.icons.edit"` in the same `wrap_before` / `ui.scope` / `demo::tool_button` / `bar.add` form, where, while it is on and `toolbar_button_checked_background(t)` is `Some`, the scope's `override_text_color` is the checked text before `demo::role_image` builds the image inside the scope (decision 12), and the `Button` takes `.fill(..)` of the accessor; while on with the accessor `None`, `.fill(ui.visuals().widgets.active.weak_bg_fill)`, the selected flat button's fill (`chrome.rs:510-511`); its `widget_info` is `egui::WidgetInfo::selected(egui::Role::Button, true, on, "Edit")`, as the status bar's toggle builds it (`chrome.rs:769`) | `response.clicked()` flips `state.basic_edit` |

In iced, `icon_colour` (`showcase-iced.rs:4613-4623`) has the toolbar case from U3
(B.8, its `toolbar` parameter): a selected toolbar button's bundled icon takes B.8's
checked text, so icon and label agree; Edit is its first caller with both true.

*Layout at 1280x720.* The row is a wrapping row in all three (`pages/basic.rs:703-704`,
`.flex_wrap()`; `showcase-iced.rs:11479`, `.wrap()`; egui's `wrap_before`,
`pages/basic.rs:483-485`), so a fourth button cannot overflow the column; whether it
wraps is the column's width against the row's. A tool button's width is
`toolbar.icon_size` plus the left and right of `button.border.padding` (the showcases pad
it so, B.10's egui rule keeping the width with an edge), the gap `toolbar.item_gap`, else
`layout.widget_gap` (`showcase-iced.rs:11477`; gpui `pages/basic.rs:701-703`). From the
presets (light; read today, `[light.defaults.icon_sizes] toolbar_px`,
`[light.button.border] padding_horizontal_px`, `[light.toolbar] item_gap_px`, `[layout]
widget_gap_px`):

| Preset | button | gap | three today | four |
|---|---|---|---|---|
| kde-breeze | 22 + 2 × 6 = 34 | 0 | 102 | 136 |
| windows-11 | 16 + 2 × 11 = 38 | 0 | 114 | 152 |
| macos-monterey | 24 + 2 × 8 = 40 | 8 | 136 | 184 |
| macos-tahoe | 24 + 2 × 12 = 48 | 8 (widget gap) | 160 | 216 |
| adwaita | 16 + 2 × 17 = 50 | 6 | 162 | 218 |
| material-3 | 24 + 2 × 24 = 72 | 6 (widget gap) | 228 | 306 |
| ios-13, the colour-scheme presets | 24 + the toolkit's own padding (none stated) | — | measured | measured |

A column holds at least `BASIC_WIDE`, 170 (`showcase-gpui/pages/basic.rs:41`,
`showcase-iced.rs:9753`, `showcase-egui/pages/basic.rs:41`; the header of
`docs/showcase-elements.toml`, `:644-653`), so kde-breeze's and windows-11's four fit in
any column; the others depend on the column's width at 1280x720, measured by T1
(decision 13) (material-3's three already exceed 170). The page lays its groups out in
five columns down to one, as many as the theme's widest fixed control allows
(`basic_column_count`, `showcase-gpui/pages/basic.rs:263-…`, which the iced and egui
pages count alike, its doc `:241-262`); in five, Icons is the fifth column's first group,
above Tabs, Expander and Table (`BASIC_FIVE`, `:186-192`; group names `:155-176`), in
four, three and two it is in the second column (`BASIC_FOUR`, `BASIC_THREE`,
`BASIC_TWO`, `:193-207`), in one the seventeenth of twenty. Where the fourth button wraps,
it starts the row's second line, and every group below Icons in its column moves down.

**The check on the built tree** (after the unit that builds Edit, before its gate; no
commit): the three release showcases under kde-breeze light and dark, adwaita light,
material-3 light and catppuccin-mocha light, by a copy of
`<record>/steps45/trial/rehearsal-t1.sh` cut to those five entries, under
`xvfb97.lock`; then `compare-t1.py <record>/steps45/trial/t1/after <new dumps>` with the
five entries. Expected: no stop entry, no parity entry, **no moved line**. A moved line
is a layout change a unit made (T1's "after" tree differs from the built one in no
size by design): it is fixed, or explained in the record, before the gate. A stop entry
stops the work for the maintainer; a parity entry is a showcase's fault and is fixed.

The parity rehearsal R runs again after the same unit, as in the steps 2–3 plan
(`<tools>/rehearsal.sh`, then `scripts/check_showcase_parity.py`, on the private X
server). Expected new differences, from T1's sizes:
`catppuccin-mocha:basic.icons.edit.{x,y,w,h}` and its `.icon.x`, `.icon.y` (Edit is a
tool button like its three neighbours, and wraps there). The keys R reports of these
join the existing group that holds Copy's, Paste's and Delete's, whose reason covers
them; any other new key is a finding, fixed in code or excepted with its reason and
source before the gate. Edit's `fill` is expected equal in the three (*Parity of the
checked fill*, below).

0.7.2's close-out asset run covers kde-breeze, material-3 and catppuccin-mocha, light
and dark, at the desktop's scale, with the comparator over its dumps and captures;
nothing is added to it but a look at the captures for Edit (present under kde-breeze
and catppuccin-mocha; under material-3 the Icons group is below the view) and, in the
screenshots workflow's captures, under macos-monterey, macos-tahoe and windows-11. The
comparison's after-only checks are not run on the asset run's dumps: the smallest
margin among the captured presets is 126 px against a move of 34 to 40, which no
scale's rounding uses up.

The rule of the check is the three sections below; the script is
`<record>/steps45/trial/compare-t1.py`. In them "before" is T1's "after" dumps and
"after" the built tree's.

**The stop rule.** An element of the Basic page, an id that starts `basic.` (143 at
`0fa28027`, each of them below `basic.page`, whose parent is `chrome.content`,
`docs/showcase-elements.toml:678-680`), whose rectangle lies wholly inside its visible
area before and not wholly inside it after, or that is present before and missing
after, stops the work, and the maintainer decides. So does the Edit button where in an
"after" dump `basic.icons.copy` lies wholly inside
its visible area and `basic.icons.edit` does not. Its visible area is the one the
comparator samples colours in, `visible_area` (`scripts/check_showcase_parity.py:776-787`):
the window's content, 1280 × 720 (`WINDOW_SIZE`, `:325`), cut to every ancestor the
list marks `clip` that the dump holds. For the Basic page that is `chrome.content`
(`clip = true`, `docs/showcase-elements.toml:543-549`), the list's only `clip` element
at `0fa28027`. The window would be the wrong area: the page scrolls inside
`chrome.content` (the list's header, `:651-652`), and `chrome.status_bar` (`:600`) is
another child of `chrome.window`, so an element pushed into the status bar's band is
hidden yet inside the window. Fractional pixels are treated as the comparator treats
them: each of a rectangle's four edges, the element's and the clipping ancestor's alike,
is rounded on its own to the device pixel grid of the dump's `scale`, ties up
(`snapped_edges`, `snap`, `:930-943`), and "wholly inside" means a left and a top edge
not less than the area's and a right and a bottom edge not greater, on those rounded
edges. A wrap that moves groups down but keeps them inside goes on; the plan's record
lists each such wrap and what it moves (the comparison's printout, below), which the
asset run's captures then show. Since the comparison is of the two dumps, no gap of a
wrapped row needs computing; the table above is the expected widths only.

**The wrap's parity.** In the "after" dumps, under each entry, Edit starts a new line in
all three showcases or in none. The three wrap by one rule on the same widths (above),
so a difference is a fault of a showcase, not a question for the maintainer (the
project's rule: a showcase that ignores theme data is a bug to fix): the showcase's
author fixes it and the check runs again; where it is a toolkit's limit,
it is excepted with its reason: an exception is a record entry naming the entry, the
kinds that differ and the toolkit's limit with its source, reviewed by the
orchestrator and named in the plan's record; either way the
maintainer is told, not asked. An element
pushed out of view (the stop rule above) stays the maintainer's question, since Edit's
place in the icon row was a design choice.

**The comparison** is `<record>/steps45/trial/compare-t1.py`, run with `python3 -I`
from the repository root on the two folders. Its logic:

1. It loads `scripts/check_showcase_parity.py` by its path
   (`importlib.util.spec_from_file_location`) and uses its `load_elements`
   (`:460-526`), `load_dump` (`:653-681`, which checks each file's `kind`, `preset`,
   `variant` and `scale`), `visible_area` and `snapped_edges`, so that the area and the
   rounding are the comparator's own. The list is the built tree's
   `docs/showcase-elements.toml`, with each id's `parent` and `clip`; it holds every
   "before" id and the two Edit ids.
2. For each showcase and each of the five entries it reads `before/KIND-THEME-VARIANT.json`
   and `after/KIND-THEME-VARIANT.json`. A file missing or malformed, or an id the list
   does not have, is an input fault: exit status 2.
3. For each `basic.` id of the "before" dump: missing from the "after" dump, a stop
   entry; inside its visible area in the "before" dump (the stop rule's test, on that
   dump's rectangles) and not inside it in the "after" dump, a stop entry; its rounded
   edges different between the two, a "moved" line with the change of its left and top
   edges and of its width and height.
4. The `basic.` ids only the "after" dump holds are listed as added (none is expected:
   T1's dumps hold `basic.icons.edit` and `basic.icons.edit.icon`).
5. Whether `basic.icons.edit` starts a new line: its rounded top edge not less than
   `basic.icons.copy`'s rounded bottom edge, in the "after" dump. The rows centre their
   items on a line (gpui `.items_center()`, `showcase-gpui/pages/basic.rs:705`; iced
   `.align_y(iced::Center)`, `showcase-iced.rs:11478`; egui's `horizontal_wrapped`,
   `showcase-egui/pages/basic.rs:1237`, lays out `Layout::left_to_right(Align::Center)`,
   `EG/ui.rs:2384`), so an Edit of another height on Copy's line can have a top below
   Copy's, but not at or below Copy's bottom. Either id missing there is a stop entry;
   so is Copy wholly inside its visible area there and Edit not (the stop rule).
6. Per entry, the three showcases' answers to 5 agree; each entry where they do not is
   a parity entry.
7. It prints, per showcase and entry, the answer to 5, the added ids and the moved
   lines, then the stop entries ("none" where there is none) and the parity entries,
   and exits 0 where both lists are empty, 1 otherwise.

*Widget Info.* gpui: `info::buttons::icon_button` names the edit button's variant
"Toolbar, icon, checked" (or "labelled") while on and "Toolbar, icon" while off, with
the claims of B.9 ("checked fill", "checked text"; where the theme states no checked
fill the info takes the form of `native_ghost_selected`: "selected bg", the variant's
active colour (`toolbar.button_active_background` at its line of
`toolbar_button_states` where stated, else `secondary_active`), and "text",
`secondary_foreground`) and the instance line "state: an
edit-mode switch, on at start; a click switches it (Showcase::basic_edit)", and a
`not_themeable("hover outline while checked", "gpui-component draws no hover style on a
selected Button (button/button.rs, RenderOnce for Button: the hover style returns early
while selected), and replays the caller's refinement over the selected state, so the
edge stays the rest one")`; `info/leaves.rs` takes the `"basic.icons."` lines of B.9 and
the `button.checked_text_color` line for `"basic.icons.edit"`. iced: B.8's routes
(`drawing()`'s Edit arm, the `tool` hand lines, the `checked_text_color` arm and the
oracle's `ICON_BUTTONS` entry). egui: B.10's per-call entry for Edit; the registry's
kind is "icon button, checked" while on, and `reg.ghost_last()` (`pages/basic.rs:1267`)
is not called for it while on (it is not at rest transparent then).

*Tests* (the colours read from the resolved theme, none written out):

- gpui (`examples/showcase-gpui/tests.rs`), `the_basic_edit_button_is_checked_and_switches`:
  under kde-breeze light and windows-11 light, `painted_fill(cx, "basic-icon-edit")`
  (`:9414`) is `toolbar.button_checked_background` and the Widget Info's "checked text"
  claim (`claim_of`) is `button.checked_text_color`; a click (`simulate_click` on its
  bounds) sets `basic_edit` false, and once the pointer is moved off the button
  (`simulate_mouse_move` to a point outside its bounds, as `tests.rs:13826` moves it, so
  that neither the hover nor the pressed fill is painted) the button paints no fill (`painted_fill` is `None`, the rest fill
  being transparent); a second click checks it again. Under macos-tahoe light the
  checked fill is the custom variant's selected fill, `secondary_active`.
- iced (`showcase-iced.rs`, beside the `basic_radio` test at `:17948`),
  `the_basic_edit_button_starts_checked_and_switches`: `basic_edit` is true at start and
  `BasicEditToggled` flips it; the icon row hands the edit button
  `styles::toolbar_button(resolved, true)`, whose `Active` style is
  `toolbar.button_checked_background` under the checked text (B.8's contract test covers
  the values).
- egui (`examples/showcase-egui/tests.rs`), `the_basic_edit_button_starts_checked_and_switches`:
  the accessibility node "Edit" is selected at start and not after a click; under
  kde-breeze light the fill handed to the `Button` is
  `toolbar_button_checked_background(t)` and the scope's text colour the checked text.
- The element-list tests of each showcase (the gpui showcase's `tests.rs:14804`,
  `:15411`; the iced showcase's reading of the list, `showcase-iced.rs:21496`) pass with
  the two entries; `scripts/check_showcase_parity.py --check-list`
  (`check_release.sh:940-941`) checks the entries against the registry, and the
  comparison itself, of their places and the fill sample in the dumps and captures,
  runs in 0.7.2's asset run (C.2), not in `check_release.sh` (its comment
  `:930-937`).

*Parity of the checked fill.* The comparator captures kde-breeze, material-3 and
catppuccin-mocha, light and dark (`scripts/capture_window.sh`, `SCREENSHOT_ENTRIES`), and
compares the element's `fill` sample, the one sample it lists. Under kde-breeze the three
paint `toolbar.button_checked_background` over the window. Under material-3 and
catppuccin-mocha, which state no checked toolbar fill, each paints the selected flat
button's fill, `button.active_background`: gpui's custom variant fills a selected
button with `active` (`GC/button/button.rs:1337`), the ghost's `secondary_active`, which
is `button.active_background` where stated (`CG/colors.rs:319`, `:1024`); iced's
`styles::toolbar_button` its pressed fallback, as given (B.8); egui
`widgets.active.weak_bg_fill` (above), the base's active fill, which composites
`button.active_background` over the button's fill. Where `button.active_background` is
opaque the three agree, and material-3's and catppuccin-mocha's are, read today:
#d0c6df / #5d566b (`material-3.toml:114`, `:633`), #ccd0da / #585b70
(`catppuccin-mocha.toml:86`, `:329`). So no `[[parity.group]]` exception is planned; one
is added, with its reason, only for a difference the comparison then shows. Under a
preset that states neither the checked toolbar fill nor `button.active_background`,
macos-tahoe (its `[light.button]` and `[dark.button]` state no `active_background`), the
three differ: iced takes `button.hover_background` (B.8's fallback, as the showcase's
`ghost_button` does today, `showcase-iced.rs:4642`), gpui `secondary_active`, a shade
of the button's fill (`CG/colors.rs:319-321`), egui the base's active fill. That is
today's difference between the three selected flat buttons (B.6), which D7 keeps; the
comparator captures no macos-tahoe run (`SCREENSHOT_ENTRIES`), so it needs no exception
for it.

*Hover.* Under kde-breeze, iced and egui draw the #3daee9 outline over the checked
button under the pointer (B.8's table, B.7); gpui draws none: gpui-component draws no
hover on a selected button (its hover style returns early, 0.10.0
`button/button.rs:754-760`), as the Widget Info line above says (B.13.6).

*Screenshots.* Every capture of the Basic page in the three showcases changes (0.7.2's
asset run, C.2); the other pages' do not.

### B.11 Documentation

- **READMEs**: the optional colours' count, in the five places A.12 names, each found by
  its quoted text, rises by four from step 4's: 36 today, 42 after both steps, which the
  documents unit writes; "five" stays, the four fields not being switch outline colours;
  the iced, gpui and egui READMEs as B.7-B.9 say.
- **CHANGELOG**, 0.7.2's section: the break is named once (D0 (a)), in A.12's one
  *Breaking Changes* bullet, which names `ToolbarTheme` too. The five step 5 bullets go
  at the end of the section's `### Added`, after A.12's, one line per bullet:

  ```markdown
  - **native-theme**: a toolbar button's states, `toolbar.button_hover_background`, `.button_hover_border_color`, `.button_active_background` and `.button_checked_background`, each a colour painted over the toolbar where the theme states it. kde-breeze and the KDE reader state Breeze's flat button: no fill and a #3daee9 outline under the pointer, the highlight at 30 % pressed, the button text at 12.5 % checked; adwaita and windows-11 state their fills (libadwaita's toolbar button and WinUI's `AppBarButton`), and the Windows reader writes the checked fill from the user's accent, `AccentFillColorDefault`, and the system highlight under high contrast, as it writes a checked button's. Where a theme states the checked fill, a checked toolbar button is lettered `button.checked_text_color`, which windows-11 states as WinUI's on-accent colour. Unstated, a connector draws a toolbar button as it draws a flat button.
  - **native-theme-iced**: `styles::toolbar_button(resolved, checked)`, a toolbar button's style from those fields, with a flat button's where a theme states none.
  - **native-theme-gpui**: `variants::toolbar_button(cx)`: `variants::ghost_button` with those fields where the native theme states them, `geometry::toolbar_button_checked(n)`, a checked toolbar button's fill and text colour, and `geometry::toolbar_button_rest(n)`, the rest state of a toolbar button whose theme states a hover outline, which the variant then draws. Add such a Button to gpui-component's `Toolbar` with `content(..)`: `child(..)` makes it a ghost button again.
  - **native-theme-egui**: the toolbar role takes the hover and pressed fills and the hover outline as stated, for a flat tool button: the role leaves a button's rest to the button, and a framed button keeps its fill at rest, so make it flat with `Button::frame_when_inactive(false)`, or in a scope whose resting fill and border colour are transparent; both keep its size; `toolbar_button_checked_background(t)` gives a checked toolbar button's fill, which egui's states cannot carry, to be lettered `button.checked_text_color`, or `button.active_text_color` where that is unset.
  - **Showcases**: the Basic page's icon row gains Edit, a tool button that is checked at start and switches on a click, so each showcase shows a checked toolbar button.
  ```
- **ROADMAP**: the second of A.12's two paragraphs ("It also carries a toolbar button's
  fill …"), appended to 0.7.2's section with the first; the text is A.12's. Any list of
  what waits that names the toolbar buttons loses them.
- **Backlog** (`docs/todo.md`):
  - *What Breeze draws that the model has no field for* (`:1785-1787` at `0fa28027`),
    as steps 3 and 4 leave it: the toolbar clause is the first item of the list
    "Breeze also draws, with no field yet:" (3H.4's wording of "The same step takes:",
    A.12). That item, "a toolbar button's states (hover a 1 px `Highlight` outline,
    pressed `Highlight` at 0.3, checked `ButtonText` at 0.125,
    `breezehelper.cpp:665-712`)", and its semicolon are removed, and the one part of it
    that still has no field takes its place as the list's first item, so that the list
    starts, rewrapped, the rest of it unchanged:

    ```markdown
    Breeze also draws, with no field yet: a checked flat button's pen,
    `KColorUtils::mix(Button, ButtonText, frameContrast)` (`breezehelper.cpp:681-684`);
    a hovered menu item's 1 px outline, …
    ```

    At the entry's end, after step 4's note (A.12), a dated note:

    ```markdown
    0.7.2: a toolbar button's hover outline, pressed and checked fills are
    `toolbar.button_*` (D7 of `docs/issues/0004_kde-open-items-rationale.md`). gpui
    draws no hover outline over a checked toolbar button: gpui-component's hover style
    returns early on a selected button (gpui-component 0.10.0 `button/button.rs:754-760`).
    ```
  - A new item under *KDE: what a Qt application under Breeze shows that native-theme
    does not*, tagged `[1.0]` by A.12's rule (D10 plans no step for it):

    ```markdown
    **Toolbar buttons and the Tab key.** Qt gives a toolbar's action buttons no keyboard focus
    (`setFocusPolicy(Qt::NoFocus)`, qtbase v6.11.0 `src/widgets/widgets/qtoolbarlayout.cpp:703`),
    so on KDE the Tab key never stops on one. The model has no field that says whether a toolbar's
    buttons take the keyboard focus, and what WinUI's `AppBarButton`, GTK's header-bar buttons,
    AppKit's toolbar items and Material's do was not read. Today an egui button and a
    gpui-component Button are Tab stops under every theme (egui 0.37.0 `src/sense.rs:60-61`,
    `Sense::click()`; gpui-component 0.10.0 `src/button/button.rs:293`, `tab_stop`), and an iced
    button takes no focus. On kde-breeze an egui tool button reached by Tab so shows the toolbar's
    hover outline (0.7.2: egui's `active` is also its focused state), which Breeze never shows
    there. To do: read the four platforms into `docs/platform-facts.md` §2.13, add the field, and
    build the showcases' tool buttons from it (egui `Button::sense(Sense::CLICK)`, gpui
    `Button::tab_stop(false)`).
    ```

    The README's accessor paragraph and `Role::Toolbar`'s doc are not given a sentence on
    it.
  - The `[upstream]` ghost-token entry (`:2390-2404`): a dated note: "the connector's
    `variants::toolbar_button` paints a toolbar button's states from the model."
  - The `[1.0]` entry *windows-11: values WinUI 3 states no source for* (`:1528-1542`):
    its list of what the reader does not map under high contrast, "(`input.hover_border_color`,
    the check box's and the switch's off colours, the slider's track and disabled
    colours)", gains ", the toolbar button's hover and pressed fills
    (`toolbar.button_hover_background`, `.button_active_background`)" before its closing
    parenthesis; the checked fill is mapped (B.5a).
  - A new `[1.0]` entry, *A checked toolbar button under the pointer and pressed*
    (decision 5): "The model has one checked fill, `toolbar.button_checked_background`,
    which the connectors keep in every enabled state of a checked toolbar button. The
    platforms draw two more: WinUI fills a checked `AppBarToggleButton`
    `AccentFillColorSecondary` under the pointer and `AccentFillColorTertiary` pressed
    (`docs/facts/windows/button.toml:18630`, `:18651` at `78ee0efd`); libadwaita a
    checked toolbar button `currentColor` at 13 % under the pointer and 19 % pressed
    (`docs/facts/gnome/button.toml:5133`, `:5209` at `78ee0efd`); Breeze draws a
    pressed checked flat button with the pressed fill, its frame deciding `down` before
    `checked` (`docs/facts/kde/button.toml:1-12` at `78ee0efd`). Fields for the two states, and their rows in
    `docs/platform-facts.md` §2.13, are to be decided. On GNOME the toolbar look
    belongs to image, image-text and arrow buttons only; the model has no field for it,
    and the showcases' tool buttons keep the look where an icon set lacks the icon and
    the label shows."
  - Three more new `[1.0]` entries, by D6's rule (D7):
    - *macOS: a toolbar button's states.* "To be measured: an `NSToolbarItem` button's
      fill under the pointer, pressed and checked (`selectedItemIdentifier`), and its
      outline, in a toolbar, on macOS 15 and 26: the parked facts mark a toolbar
      item's looks as unmeasured (`docs/facts/macos/state-models.toml:5-6`, U94, at
      `78ee0efd`). Known: the `toolbar` bezel the OS probe drew on its sheet fills
      #0000001a / #ffffff1a when pressed (`docs/facts/macos/button.toml:148813`,
      `:148869` for macOS 15; `:14861`, `:14830` for macOS 26; at `78ee0efd`), and
      AppKit's buttons draw nothing on hover (`state-models.toml:2` at `78ee0efd`). macos-monterey and macos-tahoe
      state none of `toolbar.button_*` until then."
    - *Material: a toolbar button's states.* "To be read: which button Material's
      toolbars hold (a top app bar's action items, a docked toolbar's) and its hover,
      pressed and selected fills and outline; the parked facts give no `tool` variant
      and a toolbar context only for the floating toolbar's FAB
      (`docs/facts/material/button.toml:13267` at `78ee0efd`). material-3 states none of
      `toolbar.button_*` until then."
    - *iOS: a toolbar button's states.* "To be read or measured: a `UIBarButtonItem`'s
      pressed and selected looks in `toolbar.bar`, whose fills the parked facts leave
      pending: pressed `docs/facts/ios/button.toml:17467` at `78ee0efd` (U123),
      checked for a `toggle` item `:17491` (U123), disabled and checked or pressed
      `:15175`, `:15705` (U115); no fill at rest, `:14089`. ios-13 states none of
      `toolbar.button_*` until then."
  - **The tags**, by A.12's rule: all five new entries are `[1.0]`. D10 plans no step
    for them: the checked toolbar button's hovered and pressed fills are this
    specification's decision 5, which the model has no field for, and D7 leaves the
    macOS, Material and iOS presets unset "with backlog entries" (D7, "so those presets
    stay unset, with backlog entries", `docs/issues/0004_kde-open-items-rationale.md`),
    as D6 does its readings; the Tab key's needs a field D7 does not decide either.
  - **The roadmap's count**: one recount for both steps, as A.12 says. Step 5 opens no
    `[v0.7.x]` entry and closes none by the edits above.
- **The first rationale**: §7's row `| C | toolbar buttons, sort header | no fields (§5.4) |`
  (`:1033`) becomes `| C | toolbar buttons, sort header | toolbar buttons: **fixed in 0.7.2** (Breeze's outline on hover, the highlight at 30 % pressed, the button text at 12.5 % checked; the Windows reader writes the checked fill from the user's accent) in the iced and egui connectors; gpui: the connector's `variants::toolbar_button` and `geometry::toolbar_button_checked`, the outline through `geometry::toolbar_button_rest`; the sort header waits for its fields |`.
  §8 is not edited.
  **§5.4's status line**, as step 4 leaves it (A.12): after the sentence that
  starts "Also by D6", a sentence is inserted: "Also by D7's toolbar part (0.7.2): a
  toolbar button's states (`toolbar.button_hover_background`,
  `.button_hover_border_color`, `.button_active_background`,
  `.button_checked_background`), which kde-breeze and the KDE reader state, adwaita and
  windows-11 their fills, and the Windows reader the checked fill from the user's
  accent; iced and egui draw them, gpui through
  `variants::toolbar_button` and `geometry::toolbar_button_checked`, the outline
  through `geometry::toolbar_button_rest`."; the sentence "Toolbar buttons and the table header are that
  rationale's D7; …" becomes "The table header is that rationale's D7; the menu item's
  and the list row's outlines and the menu bar title stay in the backlog (its §0)."
  The reply was posted on 2026-11-17 for 0.7.1 and lists these items as open; whether a
  follow-up comment is written is the maintainer's.
- **The rationale's status line**: A.12's one sentence for steps 2 to 5, at 0.7.2's
  close-out. The rationale stays in `docs/issues/` while a step remains, and after
  step 7 too (the steps 6–7 specification's 7.I.4: it stays there, as the first
  rationale does; its 7.I.6 archives only that specification and its plan).
- **`docs/archive/`.** 0.7.2's close-out moves this specification and its plan to
  `docs/archive/` with 0.7.2's other documents, the steps 2–3 specification and plan
  among them, so this document's two links to that specification stay same-folder
  links; its links `issues/…` become `../issues/…`. 0.7.2's row of
  `docs/archive/README.md` names
  `todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-{spec,plan}.md`: steps 4 and 5 of
  the open KDE items (the hover outlines of the check box, the radio button and the
  combo box; a toolbar button's states); their rationale stays with the issue. The
  plan's record is `<record>/steps45/`.
- **The old names** (`todo_v0.7.x_kde-open-items-steps4-5-{spec,plan}.md`). Every link
  and mention that ends in `.md`, outside `docs/archive/`, is re-pointed in the working
  tree and committed by D0: found today in `docs/todo_v0.7.x_caret-blink-plan.md`,
  `docs/todo_v0.7.x_gpui-widget-icons-spec.md`,
  `docs/todo_v0.7.x_kde-open-items-steps6-7-spec.md` and this work's plan (found again
  by `git grep -n 'kde-open-items-steps4-5' -- ':!docs/archive'`, which then finds only
  the record folder's path, in the caret plan and the steps 6–7 plan, the old names'
  pattern in this work's plan's D0 row and in this bullet, and this specification's own
  lines that name the old file names). Their mentions of the
  old record folder (`<scratch-keep>/todo_v0.7.x_kde-open-items-steps4-5-plan/`) and of
  this release as "0.7.4" are left: those documents were written for the lanes the fold
  replaced and are read again when their work is planned (steps 2–3's 6.11).

### B.12 Gates

As A.13, with: `scripts/check_connector_parity.py` for the egui
accessor; the iced contract's rows, pairs and `every_colour_leaf_reaches_a_row_or_is_named`
for `styles::toolbar_button`; gpui's leaf walk for `toolbar_button_states` and
`toolbar_button_checked`; the egui contract's T10 arm for the «toolbar» strokes, T2 for
the four rows and `button.border.color`'s «toolbar» `when`s, `verdict_totals_match_the_specification`
for *Chrome*, `panel_role_cells_write_their_rows` for the kept outline; `presets.rs` for which presets
state the fields; the KDE reader tests of B.5; `check_preset_citations.py` for the
twenty new values' comments and the four `not stated` lines; the iced, gpui and showcase
tests of the checked text (B.8-B.10); the route tests of the three showcases
(`the_element_list_parses_and_every_leaf_has_a_route` and `route_faults` in iced,
`every_listed_leaf_has_a_route` and `every_how_entry_is_shown` in gpui) for the element
lists.

Not gated: the facts cells' text and the resolved values in them (no test compares
§2.13's cells with the presets); the iced and egui showcases' look under the pointer.

### B.13 Problems found, and what was not verified

1. **The tools-area palette.** Breeze gives a main window's toolbars the Header set's
   palette (`PF:1394`'s KDE cell, which states the toolbar's background); whether a
   toolbar button's `ButtonText` and `Highlight` then come from the Header set was not
   read. The reader takes `[Colors:Button]` and `[Colors:Selection]`, as F does. Under
   both Breeze schemes the Button and Header sets' `ForegroundNormal` agree (the lines
   are in B.5); whether a Header palette's `Highlight` differs from `[Colors:Selection]
   BackgroundNormal` was not read.
2. **A Qt toolbar button is a `QToolButton` with auto-raise**, now read (2026-11-15).
   Breeze draws a tool button's panel through `Style::drawPanelButtonToolPrimitive`,
   which passes `flat = State_AutoRaise` to `Helper::renderButtonFrame`
   (`breezestyle.cpp:4572-4612`, `:4581` at `f0b1d753`, fetched into the session's
   scratch folder from the same commit), so an auto-raised tool button is drawn as the
   flat frame B.3 cites. And Qt gives a toolbar's action buttons auto-raise:
   `QToolBarLayout::createItem` makes a `QToolButton` and sets its auto-raise for an
   action that is neither a widget action nor a separator (qtbase `v6.11.0`
   `src/widgets/widgets/qtoolbarlayout.cpp:678`, `:700-702`, fetched on 2026-11-15 from
   `https://raw.githubusercontent.com/qt/qtbase/v6.11.0/src/widgets/widgets/qtoolbarlayout.cpp`;
   a fact read, no code taken), and gives it no focus (`setFocusPolicy(Qt::NoFocus)`,
   `:703`), so a Breeze toolbar button never shows its outline on keyboard focus,
   where an egui tool button reached by Tab does (B.7's cost). A widget an application adds to a toolbar itself
   (`QWidgetAction`, `:687-692`) keeps its own setting, which was not read. F's flat
   variant (K1.24) is not tied to a toolbar context.
3. **GNOME's checked fill**: 0.1 × 255 = 25.5 exactly, in `f32` too (computed today);
   the preset's 0x1a follows `Rgba::from_f32` on `f32` inputs, which rounds with
   `f32::round` (`native-theme/src/color.rs:116-119`). KDE's pressed fill, 0.3 × 255 =
   76.5 in `f32` (computed today), is the same case and is asserted on the reader's own
   path: `a_custom_scheme_moves_the_menu_scrollbar_and_list_colours` expects
   `menu.hover_background`, `translucent(focus, BLEND_VALUE)` (`kde/colors.rs:724`), to
   be `Rgba::new(20, 200, 90, 0x4d)` (`:1504`; the test was not run for this document).
   So only GNOME's 25.5 rests on documentation: that `f32::round` rounds a tie away from
   zero is Rust's documentation of it, not read in the standard library's source (the
   toolchain is outside what this work may read); num-traits 0.2.19, whose
   `Float::round` calls it for `f32` under its `std` feature (`src/float.rs:1950`,
   `:2077-2078`), documents the same rule, "Round half-way cases away from `0.0`"
   (`:1184-1185`). GTK keeps a float alpha.
4. **gpui's outline** is settled by step 3's trial, which passed
   (`variants::bordered_button`).
5. The Windows XAML lines are F's, read at `winui3/release/2.5.1`; the facts rows (B.3)
   and windows-11's comments (B.4) cite them at commit `8463f45`, as the other WinUI rows
   of `PF` do. Closed: this specification's eighth confirming check
   fetched both files on 2026-11-15 at microsoft-ui-xaml
   `8463f45162149de0ec3ad7df752596893fe3e13e` (`controls/dev/CommonStyles/`), and the
   cited lines hold there: `AppBarToggleButton_themeresources.xaml` `:6`, `:7`, `:9`,
   `:74`, `AppBarButton_themeresources.xaml` `:6`, `:7`, `:14` (the reviewer's fetch,
   not re-read for this document).
6. **No hover outline over a checked toolbar button in gpui.** gpui-component draws no
   hover on a selected button (its hover style returns early, 0.10.0
   `button/button.rs:754-760`) and replays the
   caller's refinement over the selected state (`:821-829`), so a checked toolbar
   button's edge is the refinement's at all times; iced and egui draw the outline over
   it. Recorded in the backlog entry of B.11 that names the toolbar outline.
7. **The checked text's contrast** (B.4's table): no pair of the three presets that state
   the checked fill is under 3:1 after the change; the lowest is windows-11 light,
   5.67:1.
8. **A gpui Button's icon and the refinement's text colour** (B.9). No test can
   read an icon's painted colour (gpui-pre's test API has painted quads and underlines
   only), so `a_checked_toolbar_buttons_icon_takes_the_checked_text` reads the text style
   an `Icon` without its own colour paints in (`GC/icon.rs:260-266`) through a probe
   child, at render (decision 14). Measured before the build (T2): the colour follows. If
   `a_checked_toolbar_buttons_icon_takes_the_checked_text` fails in the unit that builds
   Edit, the unit stops and the difference goes to the maintainer.
9. **GNOME's toolbar look covers some buttons only.** libadwaita gives the flat
   toolbar look to a toolbar's button that is not `.raised`, `.suggested-action`,
   `.destructive-action` or `.opaque` and is an `.arrow-button`, `.image-button` or
   `.image-text-button`; a button with a text label alone is not matched and keeps the
   ordinary button's look (`_toolbars.scss:2-8` at libadwaita 1.10.0, read by the
   session on 2026-11-14 and fetched again from the 1.10.0 tag by this specification's
   fourth confirming check, which found the rule at `:2-8`; the parked facts cite the
   same rule as `:1-7`, `F gnome/button.toml:4865`, one line off, and give its
   variants `icon_only`, `with_icon`, `@with_arrow`, `:4856`). So adwaita's fills are one colour for
   everything that look covers, and `ToolbarTheme`'s doc (B.1) says an application
   applies the toolbar-button style to the buttons the platform gives it to. The
   showcases' tool buttons are icon buttons (their label shows only where the icon set
   has no icon), so they are such buttons; where a set lacks the icon and the label
   shows, the showcase still draws the toolbar look, which on GNOME a text-only button
   would not have.
10. **gpui-component's `Toolbar::child` makes a Button ghost.** A Button added by
    `child(..)` is prepared by `.ghost().compact()` (`GC/toolbar.rs:17-22`, `:71-74`;
    `GC/button/button.rs:590-592`), which replaces a custom variant; one added by
    `content(..)` keeps it. The showcase uses `content` (`demo.rs:1424`); the variant's
    doc and the README say so for an application (B.9).
11. egui rounds a margin to a whole number (epaint 0.37.0 `src/margin.rs:108-120`), so
    decision 6's "no size change" is exact for a whole-pixel `button.border.line_width`;
    kde-breeze, the one preset that states the toolbar's outline, states 1.0. A
    fractional width would move an egui tool button's size by the rounding.

---

## Part C — Order and gates

### C.1 Units, their order and their gates

On `v0.7.2-rc1`, in the worktree `lane2-b`, from `f02c3ae9` (steps 2 and 3 built and
gated): the units are commits on it; no branch, no merge, no version step (the
workspace is 0.7.2 already), no recount after a merge. The branch is pushed by explicit
refspec at each `--commit` gate that passes, and each push's CI run is read (B.5a's
three Windows tests run only in CI's Windows legs). A failure a later gate finds is
fixed by a new commit, never by rewriting a pushed one. 0.7.2's close-out runs once.

**What stays red inside a unit, and why the units are cut where they are.** A new
registry leaf makes three connector gates fail until each connector reads or names it:
the iced `every_colour_leaf_reaches_a_row_or_is_named`; the egui leaf count of
`every_leaf_has_exactly_one_row_and_every_row_is_well_formed` (an unset leaf still
serialises, as null, so the count moves with the registry, `CE/mapping_tests.rs:234-262`)
and T10; the gpui `every_colour_leaf_reaches_a_sink_or_is_named`. An element of
`docs/showcase-elements.toml` and the gpui showcase's `HOW` lines move together:
`every_how_entry_is_shown` (`CG/showcase.rs:3893`) fails on a line no element's leaf
reaches, and an element's new leaf takes its generic line where no specific one comes
first. A preset key the backlog cites by key path cannot go without the backlog's
citation: `scripts/check_todo_citations.py`. A connector's
README is its crate's documentation (`#[doc = include_str!("../README.md")]`, e.g.
`CI/lib.rs:457`), so a README edit goes with its crate's code. So U1's gate is not
`--quick`: by this account a core commit that adds colour leaves leaves the three
connectors' leaf walks, egui's count and T10 red under `--quick` as under `--commit`
(read, not run). U1 is gated by its own crate: `cargo test -p native-theme` with the
gate's features, the five documentation checkers, and the Windows reader with its tests
type-checked for its target. The first `--commit` is the one over U1 to U4: four code units under one gate,
since no gate can pass before all three connectors read the new leaves.

**Evidence, before the units** (no commit). E1 (T1, Edit's wrap), E2 (T2, the icon's
colour), E3 (the upstream re-read) and T3 (the overlay) are done
(`<record>/steps45/`). E4, decision 10's measurement, is done too (2026-11-18, a
`worker`; `docs/evidence/breeze-offscreen/2026-11-18/`, with its row in
`docs/evidence/README.md`): the script's own check passed (the closed trigger's pen is
#3daee9 under the pointer and not at rest), and an open combo box is drawn with the
`Highlight` pen and the pressed button's fill whatever the pointer does. Decision 10,
A.3's KDE cell, A.8 and A.12's backlog line carry that outcome.

During a unit the agent runs its own tests, the touched
showcase's tests through `scripts/run_in_runner_env.sh` and
`scripts/check_strict_panic.sh` for the changed crates.

No task re-points a line citation no gate reads (C.4, the batch's S6).

| Unit | Parts | Agent | Gate after it |
|---|---|---|---|
| D0 | The amended specification, the new plan, the old-name links re-pointed (B.11, *The old names*), E4's evidence folder `docs/evidence/breeze-offscreen/2026-11-18/` and its row in `docs/evidence/README.md` | orchestrator | the five documentation checkers |
| E4 | Decision 10's measurement (done); its folder and README row are committed with D0 | `worker` | the script's own check (state 2's pen) |
| U1 | Both cores: A.1 to A.7, A.12's two backlog edits that cite kde-breeze's removed keys, B.1 to B.5, B.5a, the two `presets.rs` tests | `implement` | native-theme's tests, the five documentation checkers, the Windows reader's type-check with its tests; the connectors are red until the block |
| U2 | gpui, both steps without Edit: A.10 but its two `HOW` lines (connector, showcase overlay, claims, tests, the README row of A.12); B.9's `ToolbarButtonStates`, `toolbar_button`, `toolbar_button_rest`, contract with the interim entry, README, unit tests, the six tool buttons' switch, `native_toolbar`, the existing `HOW` lines reworded, `a_tool_buttons_box_keeps_its_size_with_the_outline`, `a_hovered_tool_button_takes_the_toolbars_outline`, `the_basic_pages_icon_buttons_claim_no_edge_at_rest` | `implement` | none of its own |
| U3 | iced, both steps without Edit: A.8 (decision 10); B.8 (style, contract, pairs, export, README, crate doc row, `ROADMAP.md`'s "33 style functions" → 34, the six tool buttons' switch with `DRESSED`, routes, `icon_colour`, `drawn_by`, the oracle) | `implement` | none of its own |
| U4 | egui, both steps: A.9; B.7 (with `demo::ghost`'s doc and `keeps_resting_stroke`'s); the parity exceptions; B.10's `tool_button` edge and the six per-call routes; `a_tool_buttons_outline_has_the_borders_width`; the final counts of A.9 | `implement` | one `--commit` with the GPUI Fast legs (`<tools>/gate.sh --fast`) over U1 to U4; push; CI read |
| U5 | The lists and Edit: A.11 and A.10's two `HOW` lines; B.10's element edits; the new `HOW` lines; iced's parts for Edit (`Shown::basic_edit`, the `.edit` id, `drawing()`'s Edit arm, the `ICON_BUTTONS` entry); Edit in the three showcases with `geometry::toolbar_button_checked` (note, README row, doc sentences, contract sink in place of the interim entry, unit tests), the probe test of decision 14, the three Edit tests, egui's per-call entry for Edit | `implement` | none of its own: beside U6 |
| U6 | The documents of both steps into 0.7.2's sections (A.12, B.11): CHANGELOG, ROADMAP, the READMEs' 42, the backlog (with decision 10's item and the Tab key's), the first rationale | `worker` | after U5 and U6: B.10's check on the built tree and the rehearsal R; then one `--commit` with the GPUI Fast legs and the five documentation checkers; push; CI read |

U2, U3 and U4 each change one connector's folder (U3 also `ROADMAP.md`'s one count, U4
also `scripts/connector_parity_exceptions.toml` and egui-widgets' doc line) and build on
U1's tree: three builders in the one worktree, each with `-p` through `<tools>/slot.sh`,
as the steps 2–3 plan's block. U5 touches the three showcases and
`docs/showcase-elements.toml` and runs alone beside the documents-only U6.

**Reviews.** U1 by the orchestrator (data the checkers validate, and the two readers'
few assignments with their tests; B.5a adds no `unsafe`; its three Windows tests'
verdict is CI's, after the first push); U2, U3 and U4 by three `review` agents beside
U5; U5 by a `review` agent before its gate; U6 by the orchestrator. A must-fix is a
commit of its own before the next gate.

**Stops**: a decision neither this specification nor the plan's record makes; a stop
entry of B.10's check on the built tree; the probe test of decision 14 failing.

**Before the close-out**, the search over the stamped paths (0.7.2's rule 22 (c))
runs with this specification's and plan's stems (expected: nothing), as the steps 2–3
plan has it for its own.

**The close-out** is 0.7.2's one (the steps 2–3 plan's). This specification adds to
it: the archive moves this specification and its plan with 0.7.2's documents, and the
rationale's status paragraph takes A.12's one sentence for steps 2 to 5; the asset
run's parity list is read for Edit's keys (B.10, the rehearsal R) and for a `fill`
exception (none planned); the captures are looked at for Edit, the screenshots
workflow's under macos-monterey, macos-tahoe and windows-11 included (B.10).

**For the maintainer, with the documents**: the approval of D6 and D7; Edit's second
line under adwaita and the colour-scheme presets (decision 13, B.10); the two new
`[1.0]` backlog items he may want sooner (the Tab key, B.7; an open combo box's frame,
decision 10); three differences from the platforms the connectors and showcases keep,
each visible on its platform: a checked toolbar button under the pointer and while it
is held, which WinUI, libadwaita and Breeze draw in fills of their own (decision 5; the
new `[1.0]` entry *A checked toolbar button under the pointer and pressed*, B.11), no
hover outline over a checked toolbar button in gpui, where iced and egui draw it
(B.13.6), and the toolbar look the showcases' tool buttons keep where an icon set lacks
the icon and the label shows, which on GNOME a text-only button would not have
(B.13.9; the same entry, B.11); the ROADMAP heading's wording (A.12); whether a
follow-up comment goes on the issue.

### C.2 The asset run and the stamps

Both steps change connector sources and examples, which `scripts/update_compatibility.sh`
and `scripts/update_provenance.sh` hash, so `docs/COMPATIBILITY.toml` and
`docs/assets/PROVENANCE.toml` go stale and the full check fails on them until the
release's asset run (`scripts/generate_assets_release.sh`) re-stamps them. The
screenshots are taken at rest: step 4 changes nothing at rest; step 5 changes nothing
visible at rest in the existing toolbar buttons by design (decision 6, the ghost's rest
in gpui). In iced the rest style differs in value: `ghost_button` gives `background:
None` at rest (`showcase-iced.rs:4646-4651`), `styles::toolbar_button`
`Some(Color::TRANSPARENT)` and a transparent border `button.border.line_width` wide
(B.8). iced then draws a quad where it drew none (`IW/button.rs:378-392`: a quad is
drawn where the background is `Some` or the border's width is above 0, and `None` is
painted as `Color::TRANSPARENT` anyway), but a quad whose fill and border are both
transparent paints no pixel, so the capture is the same (read from the draw code, not
compared in a capture; the asset run's parity comparison shows it). Step 5 also adds the Edit
button, checked at start, so every
capture of the Basic page in the three showcases changes, and no other page's (B.10).
The run needs the maintainer's desktop unlocked and untouched for 4 to 6 minutes. It is
0.7.2's one asset run, in its close-out, after the full check and CI are green (the
full check's only failure the provenance stamp, the batch's rule 9).

### C.3 Version and release (D0)

Steps 4 and 5 are part of 0.7.2 (the maintainer, 2026-11-18; D0 (a): the release names
its break once, A.12's bullet). Tag, publication, GitHub release and any follow-up
comment on the issue stay the maintainer's.

### C.4 Line citations no gate reads (both steps)

None is re-pointed (the batch's S6, decided by the maintainer on 2026-11-15): the
existing `file:N` citations of the lines these steps move are left as they are and may
drift, and this work's own new citations of changing repository files name the
document (with a quoted phrase or a section where useful), never a line (A.5, B.4).
Citations a gate reads are not of this kind and stay with the task that moves their
lines: the gpui showcase's colour claims (A.10, *Lines that must not move*) and the
generated `contract-leaves.toml`, regenerated by its command. Line numbers of pinned
sources (an upstream crate at a version, a URL at a commit, a citation "at <sha>") stay.

## What could not be verified

- The tools-area palette (B.13.1); a toolbar widget an application adds itself and its
  auto-raise (B.13.2: a toolbar's action buttons are auto-raised and drawn flat, read).
  (libadwaita's `_toolbars.scss:2-8`, B.3 and B.13.9, is verified: the fourth
  confirming check fetched it at the 1.10.0 tag.)
- Windows's check-box and combo-box hover strokes; the outline of GNOME's checked box
  under the pointer; AppKit's check-box and pop-up hovers; macOS's, Material's and
  iOS's toolbar button states (each in a backlog entry, A.12, B.11).
- That no showcase paints another fill under the Basic page's icon row (B.4's contrast
  figures on the page). (The WinUI XAML lines at commit `8463f45`, B.13.5, are closed by
  the eighth confirming check's fetch of 2026-11-15.)
- T2's and T10's treatment of the «toolbar» sinks (B.7) was read from `own_write`,
  `carried_by`, `check_splice` and `expected`, not run; that no unit leaves a commit's
  own gates red (C.1) was read from the tests named there, not run.
- That `f32::round` rounds a tie away from zero, for GNOME's checked fill, 25.5 → 0x1a
  (B.3, B.13.3): read from documentation (Rust's, and num-traits 0.2.19's for the
  method it forwards to), not from the standard library's source, which lies in the
  toolchain outside what this work may read. KDE's 76.5 → 0x4d is asserted by the
  reader's existing test on the same path (`kde/colors.rs:1504`; not run for this
  document).
- Whether Qt reports a pointer over an open combo box's trigger as a hover on a desktop
  session: E4 ran under Qt's `offscreen` platform, where it does not (decision 10).
- What an open `md-outlined-select` draws on its outline (material-3 states the hover
  outline).
- The Basic page under windows-11, macos-monterey, macos-tahoe and ios-13 with Edit: a
  Linux showcase refuses those presets; T1's second pass (a patched command line, Linux
  fallback fonts) found no stop; the screenshots workflow draws the first three on their
  own platforms (decision 13).
- That the five entries of the headless check stand for the 26 presets and modes a Linux
  showcase can show: the draft pre-check found the dropped ones equal in every value but
  colours at `0fa28027`; it was not run on this tree.
- Whether a toolbar's buttons take the keyboard focus on Windows, GNOME, macOS and
  Material (B.7; the backlog entry).
- Whether a fractional border width moves an egui tool button (B.13.11).
