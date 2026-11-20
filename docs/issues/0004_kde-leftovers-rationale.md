# Issue #4: the leftovers of the KDE open items — rationale

Status: written 2026-11-20 for 0.7.2, by the maintainer's decision of that day: the
three looks the open-items rationale left in the backlog are done now, and the smaller
Breeze details with them where they fit in one to two days of building; the four
requests to GPUI Kit wait (`docs/todo.md`, the [upstream] entry "GPUI Kit, from issue
#4"). Checked by two first-pass reviews and a confirming check, their findings
applied. Approved by the maintainer on 2026-11-20; specified and planned in
`../archive/todo_v0.7.2_kde-leftovers-{spec,plan}.md`, and built in 0.7.2 on
2026-11-20, E1 to E14, from `96ef2531` on.

It continues `docs/issues/0004_kde-open-items-rationale.md` (called "the open-items
rationale" below), whose §0 left these items in the backlog, and the backlog entry
*KDE: what a Qt application under Breeze shows that native-theme does not (issue #4)*
in `docs/todo.md`, which lists them with their Breeze sources. D0 (a) of the
open-items rationale stands: a 0.7.x release that adds model fields names the break in
its CHANGELOG.

Sources. Breeze is read at `f0b1d753` (`kstyle/breezestyle.cpp`, `breezehelper.cpp`,
`breezehelper.h`, `breezemetrics.h`, `breezetoolsareamanager.cpp`, `breeze.kcfg`,
`colors/BreezeLight.colors` and `BreezeDark.colors`), written below as `st`, `hp`,
`hh`, `mt`, `tam`; qtbase and qtdeclarative at v6.12.0, kirigami and kwidgetsaddons at
v6.31.0. "Measured" means an offscreen run in `docs/evidence/breeze-offscreen/`: the
runs of 2026-11-13, 2026-11-14 and 2026-11-18, and the run of 2026-11-20 this
rationale adds (Qt Quick's font sizes, a slider handle and a checked tool button,
menus, a menu bar and item views; its README says what ran). The toolkits are gpui-component and
gpui-base 0.10.0 (`GC`, `GB`), iced_widget 0.14.2 (`IW`), iced_aw 0.14.1 (`AW`), egui
0.37 (`EG`) and epaint 0.37.0; `PF` is `docs/platform-facts.md`, `REG`
`docs/property-registry.toml`, `INH` `docs/inheritance-rules.toml`. Breeze's colours
named below are the View set's `DecorationFocus` ("focus") and `DecorationHover`
("hover"), both #3daee9 in Breeze and Breeze Dark, and the palette's `Highlight`
(the Selection set's `BackgroundNormal`, #3daee9) and `HighlightedText` (#ffffff /
#fcfcfc); `frameContrast` is 0.2.

Every fill below that Breeze draws for a menu needs Breeze's setting
`MenuItemDrawStrongFocus`, true by default (`breeze.kcfg:130-132`); the KDE reader does
not read `breezerc` and assumes the default, as it assumes the default window border
size for the toolbar's Header fill (`docs/todo.md`, *KDE reader: the toolbar's Header
fill and the window border size*).

## 0. What is decided here

| # | Item | In 0.7.2 |
|---|---|---|
| M1 | A hovered menu item's 1 px outline | yes, one of the three looks (E1) |
| L1 | A hovered list row's 1 px outline | yes, one of the three looks (E5) |
| M3 | The menu bar's fill and its title's fill | yes, one of the three looks (E3) |
| M2 | The pressed menu item | yes (E2) |
| M4 | The menu item's radius, padding and icon gap; the 4 px gap round the highlight | yes (E4) |
| M5 | The menu row's height | yes (E4) |
| L5 | `list.hover_text_color` the KDE reader does not write | yes (E6) |
| B3 | The table header's section separators | yes (E7) |
| B1 | The slider handle's outline, and the hover fill kde-breeze has no source for | yes (E8) |
| B2 | A checked flat toolbar button's pen | yes (E9) |
| B4 | An open combo box's frame | yes (E10) |
| F1 | Font sizes in whole pixels | yes (E11) |
| Z1 | The button's minimum width | yes (E12); its minimum height no (§3) |
| L3 | A selected row under the pointer | no (§3) |
| Z2, Z3 | The dialog's size limits and its icon size | no (§3) |
| S1–S4 | The scroll bar's inner line, the handle's outline, the handle of the focused view, the handle while the pointer is on the bar | no (§3) |
| L2 | A hovered row in a view without the focus | no (§3) |
| L4 | A list row's own box: its margins, its padding beyond them, its radius | no (§3) |

What fits in one to two days, and the line drawn, is §4.

## 1. What the toolkits can receive

From the reading of 2026-11-20 (the toolkits' versions above). "Showcase" means the
part is drawn by the showcase's own code, which an application copies; "upstream"
means gpui-component's own widget offers no value, style or hook, so only a change in
GPUI Kit makes it themeable there.

| Item | gpui | iced | egui |
|---|---|---|---|
| Menu item outline, pressed item | upstream: `PopupMenu`'s item is `pub(crate)` and styles only its hover fill and text (`GC/menu/menu_item.rs:117-152`); showcase: `MenuLook::row` | iced_aw: `path_border`, a full border round the active item (`AW/style/menu_bar.rs:22-25`), but the highlight is always drawn with `Status::Active`, so no pressed look (`AW/widget/menu/menu_bar.rs:587`); showcase: `menu_row`, a button style with `Pressed` | the menu role's per-state `bg_stroke` and the `active` state's fill (`EG/style.rs:1360-1430`); the connector writes one stroke into every state today |
| Menu bar fill and title fills | upstream: `AppMenuBar` draws no fill and its titles are ghost buttons coloured by global tokens (`GC/menu/app_menu_bar.rs:122-143`, `:272-284`); showcase: `MenuLook::title` and the bar | `bar_background` (`AW/style/menu_bar.rs:9`); the titles: showcase only, since iced_aw draws one highlight for titles and items and only while a menu is open | `MenuBar` takes a style of its own beside the menus' (`EG/containers/menu.rs:321-354`); the bar's fill is the showcase's panel |
| Menu geometry | upstream: padding, gap, panel gap and row height are constants (`GC/menu/popup_menu.rs:1334-1347`, `:1338-1341`, `:1633-1634`); the radius is the theme's shared radius; showcase: all | `Menu::padding` for the panel gap (`AW/widget/menu/menu_tree.rs:192`); rows: showcase | `spacing.menu_margin`, `button_padding`, `icon_spacing`, `interact_size.y` with the connector's `fit_row_padding` (a stated row height drawn exactly); `CornerRadius` holds whole pixels (`epaint/src/corner_radius.rs:13-24`) |
| List row outline, header separators | upstream: `ListItem` and `DataTable` rows take a fill only, and a selected row no hover (`GC/list/list_item.rs:177-266`, `GC/table/state.rs:2382-2388`); showcase: its own tables, an overlay box for List rows | showcase: `list_row` (a button style with a border) and its table | showcase: rows and tables it paints itself |
| Slider handle outline | upstream for `Slider`; the connector's own slider widget draws an `outline` | `slider::Handle { border_width, border_color }` per status (`IW/slider.rs:578-629`) | egui's `Slider` strokes the handle in the state's `fg_stroke` (`EG/widgets/slider_core.rs:423-441`), which also letters its value; the connector's own `native-theme-egui-widgets` slider paints its knob |
| Checked toolbar button's pen | `geometry::toolbar_button_checked`, a style refinement upstream replays over the selected state | `button::Style.border` in the showcase's toolbar style and the connector's `toolbar_button` | `Button::stroke`; the showcase paints the checked look through an accessor |
| Open combo box | upstream: an open `Select` draws `ring` and the focus band, and exposes no open state (`GC/select.rs:559`, `:596-602`) | `pick_list::Status::Opened { is_hovered }` (`IW/pick_list.rs:838-849`) | the combo uses `widgets.open` while its popup shows (`EG/containers/combo_box.rs:363-381`) |
| Font size | the resolved `f32`, passed through by every connector | the same | the same |

Every "upstream" cell is recorded for later (E14); no request is sent now.

## 2. Decisions

The new fields are optional, as every field added in 0.7.2 is. kde-breeze alone states
them: `PF` and the parked facts hold no verified value of any new field for another
platform (the gap maps name sources for some, listed in the new `PF` rows as not
collected). The KDE reader writes the new colours from the colours it reads and the new
sizes in its metrics; E11's flag comes from the preset alone. Where a field is unset,
each connector and showcase draws what it draws today, so nothing changes outside KDE.

### E1 — The hovered menu item's outline (M1)

Breeze strokes a hovered item's highlight on all four sides, 1 px, in
`focusOutlineColor` = `mix(focus, WindowText, 0.15)` (`st:6021-6049`, `hp:172-175`),
over the fill kde-breeze already states (`menu.hover_background`, focus at 0.3).
Measured #399acc / #5abaec, which the mix recomputes exactly. The line lies inside
the highlight: Breeze's item inset `MenuItem_MarginWidth` 4 counts from the
highlight's edge, line included (the highlight at x 4, the contents at x 8).

- New field `menu.hover_border_color`; kde-breeze #399acc / #5abaec; the reader
  writes the mix of `view_focus` and `window_fg`. Unset, no outline, as today.
- It is drawn at `defaults.border.line_width` (1 in kde-breeze), over the item's
  padding, so nothing moves, as E5's row outline is drawn. `menu.border.line_width`
  stays unstated: stating it would add the width to every side of the padding (the
  model's padding lies inside the border) and contradict `PF` §2.6, which gives a
  menu no border of its own.
- Drawn by: iced's `styles::aw::menu`, whose `path_border`, where this colour is
  stated, takes it and `menu.border.corner_radius` instead of iced_aw's own 6 (unset,
  iced_aw's own border stays, on every other preset; iced_aw draws the same
  highlight on an open bar title too, where Breeze draws no line: a limit named in the
  connector's doc), and the iced showcase's `menu_row`; egui's menu role, which then
  writes the stroke into the hovered state only (a connector change, not a toolkit
  one); the gpui showcase's `MenuLook::row`. Every menu the showcases draw takes it,
  the gpui Overlays page's `menu_rows` included.

### E2 — The pressed menu item (M2)

While the mouse button is held on an item, Breeze fills it with the opaque focus colour
and outlines it in the same colour, so no outline shows; the text, the shortcut and the
submenu arrow take `HighlightedText` and the icon its `Selected` mode (`st:6021-6029`,
`:6108`, `:6129`, `:6155`). Measured on 2026-11-20 (`menus_light.txt`,
`menus_dark.txt`; Breeze receives `Sunken|Selected`): the fill and its edge #3daee9, the
text #ffffff / #fcfcfc.

- New fields `menu.active_background` (#3daee9, the reader's `view_focus`) and
  `menu.active_text_color` (#ffffff / #fcfcfc, `selection_fg`), named as
  `button.active_background` is. Where `active_background` is stated, a pressed item
  draws no outline (Breeze's line in the fill's own colour shows nothing). Each falls
  back to the next state down, as `list.header_active_background` does: an unset
  `active_text_color` is `hover_text_color`, then the text at rest; an unset
  `active_background` keeps the hover look, outline included.
- The shortcut's text is `HighlightedText` at 0.7 on a pressed item and another
  colour at rest and under the pointer (measured); the model has no colour for a
  menu's shortcut text in any state (§3).
- Drawn by egui's menu role (`widgets.active`, today a copy of the hovered state) and
  the iced and gpui showcases. The icon's `Selected` mode is not carried: no connector
  recolours a menu icon by state.

### E3 — The menu bar (M3)

Breeze fills a main window's tools area (the menu bar and the top toolbars, as one
area) with the Header set's `Window` colour (`st:1341-1347`, `tam:118-136`, `:201-203`),
#dee0e2 / #292c30, which kde-breeze states as `toolbar.background_color`. A title under
the pointer is filled with the opaque hover colour, and a title whose menu is open with
the focus colour, rounded as the menu highlight, lettered in `HighlightedText`
(`st:5841-5945`). Measured on 2026-11-20 (`menubar_light.txt`, `menubar_dark.txt` and
their proxy logs): the bar #dee0e2 / #292c30, the same as the toolbar below it; a
hovered title (`MouseOver|Selected`) and an open one (`Sunken|Selected`) both filled
#3daee9 with a rounded highlight that fits 4.5, lettered #ffffff / #fcfcfc. With one
menu open, moving the pointer onto the next title opens its menu and moves the open
look there.

- **The bar's fill: no new field.** The bar takes `toolbar.background_color`, the fill
  of the one tools area Breeze draws. On every preset but kde-breeze that colour
  resolves to `defaults.background_color`, as `window.background_color` does, which
  the bar takes today: nothing moves outside KDE. A field of its own was weighed and
  not taken: no platform's menu-bar fill is known apart from its toolbar's, and a
  platform that turns out to have one adds the field then. The readers are the iced
  connector's `styles::aw::menu` (`bar_background`) and the three showcases' bars; the
  iced showcase's page-tab caret menu, an iced_aw menu bar in the tab strip, keeps
  today's fill; the gpui showcase's titles in a client-side title bar lie on the title
  bar's fill, as today.
- **The titles' fills:** new fields on `menu`, prefixed `bar_` rather than a new
  `menu_bar` section, since no other menu-bar property is missing: `bar_hover_background`
  (hover, #3daee9), `bar_open_background` (focus, #3daee9) and `bar_hover_text_color`
  (`HighlightedText`, for both states). The two fills are equal in Breeze's schemes and
  differ under a scheme whose `DecorationHover` and `DecorationFocus` differ, so both
  are carried. "Open", not "active": the state is the title's menu shown, as egui's
  `widgets.open` names it. An unset `bar_open_background` is `bar_hover_background`;
  with both unset, and for an unset `bar_hover_text_color`, a title keeps the look it
  has today.
- **The titles' padding.** A title is inset by `MenuBarItem_MarginWidth` 10 and
  `MenuBarItem_MarginHeight` 6 (`mt:93-94`, `st:3598`); measured: the text's ink 11 in
  from each side (10 and a bearing of 1), a title 30 high for an 18 px line, and the
  titles touching (File at x 0, Edit at x 41). All three showcases pad their titles
  with `menu.border.padding`, which E4 changes to Breeze's item padding (14 and 27), so the titles
  need their own: new fields `menu.bar_padding_horizontal` and
  `menu.bar_padding_vertical` (two numbers, TOML `bar_padding_horizontal_px` and
  `bar_padding_vertical_px`, each applied to both of its sides), kde-breeze 10 and 6,
  the reader's metrics. Their readers: gpui `MenuLook::title`; iced `menu_bar` and the
  Extra page's iced_aw title padding; egui's `MenuBar` title style, which then takes a
  style of its own apart from `Role::Menu`. Where they are stated, the titles touch, as
  Breeze's do; the bar's own inset at its sides stays the showcases'. Unset, a title
  keeps today's padding.
- This closes the backlog entry *Model: the menu bar has no fill of its own*. The
  tools area's bottom line in the Header `separatorColor` (`st:1349-1350`) runs under
  the last toolbar, not under the menu bar, and belongs to the toolbar: not here.
- gpui-component's `AppMenuBar`: upstream (E14).

### E4 — The menu's geometry (M4, M5)

What kde-breeze and the reader state today, against Breeze:

| | Today | Breeze |
|---|---|---|
| Gap between the panel's edge and the highlight | the showcases take `popover.border.padding`, 12 | `PM_MenuHMargin`, `PM_MenuVMargin` = `MenuItem_HighlightGap` 4 (`st:649-651`, `mt:55`; measured); rows touch (measured: y 4, 33, 62, each 29 high) |
| Highlight radius | unstated | `frameRadius(PenWidth::Frame)` 4.5 (`hh:345-348`, `hp:384`); the corners measured on 2026-11-20 fit it |
| Inset of an item's icon from the highlight's edge | 4 | 14 in a menu with icons and no checkable items: `MenuItem_MarginWidth` 4 + `TextLeftMargin` 8 to an icon column 20 wide, the 16 px icon centred in it (`st:6053-6065`, `:6091-6097`); measured on 2026-11-20: the item at x 4, the icon at 18..33 (`menus_light.txt`) |
| Gap between the icon and the text | 8 | 4: the text at 38 (ink at 39, a bearing of 1), in the same run; and in a menu with checkable items too (the 2026-11-13 run) |
| Inset on the right | 4 | 27: every row keeps an arrow column, `MenuButton_IndicatorWidth` 20, inside `MenuItem_MarginWidth` 4, and the contents end `MenuItem_ItemSpacing` 2 + 1 before it (`drawMenuItemControl`, `contentsRect.setRight(arrowRect.left() - MenuItem_ItemSpacing - 1)`; `menuItemSizeFromContents`); measured: the highlight at 4..109, the shortcut's ink ending at 82 |
| Row height | unstated | `max(text, 20, icon) + 1`, then 4 above and below (`st:3663-3667`): 29 wherever the text and the icon are 20 px or less (measured 29 at Noto Sans 10 pt) |

- **The panel gap:** new fields `menu.panel_padding_horizontal` and
  `menu.panel_padding_vertical` (two numbers, TOML `panel_padding_horizontal_px` and
  `panel_padding_vertical_px`), the inset of the items inside the menu panel's frame
  line, as every padding in the model lies inside its border; kde-breeze 3 and 3:
  Breeze's 4 counts from the panel's outer edge, its 1 px outline included (the
  outline at x 0, the highlight from x 4, `docs/evidence/breeze-offscreen/2026-11-13/`),
  as the reader already takes a line off Breeze's input padding. Stated by kde-breeze
  alone (GNOME's menu margin is unverified,
  and the other platforms' insets are listed in the new `PF` row as not collected).
  Unset, the showcases keep `popover.border.padding`. Where they are stated, the
  showcases' rows touch (the gpui popup's row gap is then 0). They are read by the
  three showcases' menus; whether egui's own menu popup takes its margin from the menu
  role's scope (`spacing.menu_margin` serves every egui popup frame) is for the
  specification to read: if it does, the menu role writes it there; if not, only the
  showcase pads its panel.
- **The radius:** kde-breeze states `menu.border.corner_radius` 4.5, Breeze's value
  as the tab's 4.5 is stated. The field keeps its two readers: the item's radius
  (egui, the showcases, iced's `path_border` by E1) and the iced drop-down's panel,
  which iced rounds with one radius for the panel and the highlight; on KDE the
  drop-down's panel becomes 4.5, half a pixel from the popover's 5. egui holds whole
  pixels, and the connector's conversion rounds half away from zero (`convert.rs`,
  `u8_from_f32_saturating`), as it does for the tab's 4.5: 5.
- **Padding and gap:** kde-breeze and the reader state `menu.border.padding_left` 14,
  `padding_right` 27 and `menu.icon_text_gap` 4; the gpui and iced showcases put a
  shortcut flush right inside the right padding, where Breeze ends it (no showcase lays
  out a submenu arrow of its own; egui's `SubMenuButton` draws its arrow inside the
  button's padding). The model
  has one row layout, an icon then the text; Breeze's layout has columns, and a menu
  without icons or checkable items puts its text 16 from the edge, a menu with
  checkable items puts their column first (`st:6059-6065`, `:6101`). The model carries
  the menu with icons, which a KDE application shows by default (kdeglobals `[KDE]
  ShowIconsInMenuItems`, which the reader does not read): a menu without icons is drawn
  2 px left of Breeze's, and in a menu with check items the model draws the icon at 14
  where Breeze draws it at 33, after the check column (the 2026-11-13 run). Two more
  deviations are not carried, since no field could say where they apply and nothing
  may change outside KDE: in a menu with icons, Breeze puts an item without an icon's
  text where the others' text starts (measured), where the showcases start it at the
  left padding; and Breeze sizes a row as the label + 52 with icons and + 30 without
  (measured: an item 82 wide for a 30 px label), where the model's row is the label +
  61 and + 41, so a menu whose widest row has no shortcut is 9 or 11 px wider than
  Breeze's (27 is kept: 18 would misplace the shortcut by 9). Breeze mirrors the row in
  a right-to-left layout; this is the first strongly uneven menu padding, and the
  specification reads what each connector does with it there (gpui's
  `geometry::with_padding` sets physical sides). egui gives a
  button one symmetric padding (`to_button_padding` takes each pair's mean); its menu
  role takes the left padding for both sides, so the contents start where Breeze's do
  and the row's right inset is 14, said in the connector's doc. The backlog's entry on
  these values (*KDE values that wait for a connector design*, item 5) closes with
  this.
- **The row height:** new field `menu.min_row_height`, the least height of a one-line
  row, which grows with its content; kde-breeze and the reader state 29, and
  `menu.row_height` stays unstated on KDE, as it is today. Breeze's row is 29 wherever
  the text line is 20 px or less, and the text + 9 above that, where the model's grows
  as the text + 8 (its padding): one pixel short, said in the field's doc. Stating 29
  in `row_height` was weighed and not taken: that field is drawn exactly at a text
  scale of 1 (the M9-150 rule; gpui's `with_height_rule` sets a fixed height, the
  showcases' `rows::scaled_height` keeps it even where the text does not fit), the KDE
  reader sets no text scale, and its output is merged over the preset, so every KDE
  user with a menu font of about 16 pt or more would get clipped rows. A separate
  least height leaves the M9-150 rule and its tests as they are. Where a theme states
  both, `row_height` is drawn as it is today and the least height is not applied: an
  exact height leaves a least one nothing to do. Read by: gpui `geometry::menu_item`
  (a `min_h`), the iced showcase's rows, and egui's menu role (`interact_size.y`, a
  least height in egui, without the connector's `fit_row_padding`, which serves only a
  stated `row_height`).
- The vertical padding stays 4 (`MenuItem_MarginHeight`).

### E5 — The hovered list row's outline (L1)

Breeze fills a hovered, unselected row with `Highlight` at 0.3 (kde-breeze's
`list.hover_background`) and strokes it 1 px in `Highlight` at alpha 0.8
(`st:4867-4884`, `mt:176`). Measured on 2026-11-13 and again on 2026-11-20
(`views_light.txt`, `views_dark.txt`): the outline's edge pixel #5ebbec / #3694c5 over
the view, which the rule gives where the half-pixel-inset line lies over a half-covered
edge pixel.

- New field `list.hover_border_color`: kde-breeze #3daee9cc, the reader's
  `selection_bg` at 0.8. Drawn at `defaults.border.line_width`, on the shape the
  showcase gives the hover fill today (Breeze rounds a list's highlight 4.5 and draws
  a table's square; the row's own radius is L4's, §3). Unset, no outline, as today.
- Drawn by the three showcases' lists and tables; in gpui as a box laid over the row
  that shows under `group_hover` (read, not built). gpui-component's `ListItem` and
  `DataTable`: upstream (E14).

### E6 — Colours the KDE reader does not write (L5)

kde-breeze states `list.hover_text_color` (#232629 / #fcfcfc), and the KDE reader
does not write it, so under a colour scheme other than Breeze's the hovered row keeps
Breeze's text colour. Breeze draws it in `Text`, the View set's `ForegroundNormal`.
The reader writes it from that colour, as it writes the button's and the menu's hover
text. Removing the preset's line and letting it inherit `list.item_font.color` gives
the same value; the reader's write is chosen because it keeps the three hover texts
written alike and the preset's line cited to its `PF` cell.

`button.border.color` has the same gap: kde-breeze states #d1d1d2 / #535659 and the
reader does not write it, while Breeze draws a button's frame in
`mix(Button, ButtonText, frameContrast)` (`hp:697-699`), the colour E8 and E9 use. The
reader writes it with that mix, `frameContrast` read as the reader reads it already
(kdeglobals `[KDE] frameContrast`, 0.2 where unset).

### E7 — The table header's separators (B3)

Breeze draws a 1 px line on each header section's trailing edge (the last section and
a lone section have none) and along the header's bottom, in `WindowText` at
`frameContrast` (`st:6803-6893`); measured #d1d2d2 / #535558
(`docs/evidence/breeze-offscreen/2026-11-13/`). The showcases draw them in
`list.grid_color`, #c4c8cc / #1c1f21.

- New field `list.header_separator_color`: kde-breeze #23262933 / #fcfcfc33, the
  reader's `window_fg` at `frameContrast`, laid over the header's fill. Over Breeze's
  header fill that gives #d1d1d2 / #535659, one level from the measurement in a
  channel or two, from rounding. Unset, the showcases keep the grid colour.
- The empty area after the last section has a bottom line in `ButtonText` at 0.1
  (`st:6896-6935`); no showcase's header leaves such an area (the specification
  checks it), so it is not carried.
- Drawn by the three showcases' tables. gpui-component's `DataTable` header and iced's
  table have one separator colour for the whole table: upstream (E14), and iced's
  table is not themed by the connector.

### E8 — The slider handle's outline (B1)

Breeze keeps the handle's fill (`Button`) in every state and shows hover and focus by
its outline: `mix(Button, ButtonText, frameContrast)` at rest, the hover colour with
the pointer on the handle (also while it is dragged), and the focus colour while the
slider has the focus, after a click as after a key; hover wins over focus
(`st:8122-8128`, `hp:248-275`). The focus frame Breeze adds round a slider is drawn for
keyboard focus only, and is kde-breeze's focus ring (open-items rationale D4). Measured
on 2026-11-20 (`qt-fonts-widgets/out_widgets.txt`): the rest edge #d2d2d3 / #515457,
the hover edge #45b1e9 / #3ca8e0 (an antialiased circle's edge), the fill unchanged
under the pointer.

- New fields, after the switch's `thumb_border_color` and `thumb_border_line_width`:
  `slider.thumb_border_color` (kde-breeze #d1d1d2 / #535659, the reader's mix of E6),
  `slider.thumb_border_line_width` (kde-breeze 1, `PenWidth::Frame` 1.001 rounded as
  `defaults.border.line_width` is), `slider.thumb_hover_border_color` (hover, #3daee9)
  and `slider.thumb_focus_border_color` (focus, #3daee9). An outline is drawn only
  where the width is stated and above 0; an unset hover or focus colour is the rest
  colour, and an unset rest colour draws no line at rest. With the width unset, no
  outline, as today.
- kde-breeze's `slider.thumb_hover_color` #93cee9 has no source; it stood in for the
  outline (`docs/todo.md`, *Presets: values the M8 T15 platform-facts rows disagree
  with*). It is removed: unset, every connector keeps `thumb_color` under the
  pointer, as Breeze does.
- The handle's shadow stays in the backlog (*Slider: the thumb's outline or shadow*),
  which wants every platform's outline and shadow researched; this decision covers
  KDE's outline and leaves that entry's other platforms open.
- Drawn by: iced (`Handle` border per status; its `Hovered` and `Dragged` cover the
  whole slider, not the handle alone; it has no focused status, so the focus outline
  is not drawn: said in the connector's doc); the gpui connector's own slider
  (`outline`); egui's connector widget `native-theme-egui-widgets`' `Slider`, which
  paints its own knob. egui's own `Slider` strokes its handle in the state's
  `fg_stroke`, which also letters the slider's value beside it
  (`EG/style.rs:1434-1436`), so the outline cannot be given to it without recolouring
  that text: a limit stated in the connector's doc, like gpui's (the egui showcase
  shows both sliders, on its Range page). gpui-component's
  `Slider`: upstream (E14).

### E9 — A checked toolbar button's pen (B2)

Breeze draws a checked flat button with the ordinary button frame's pen,
`mix(Button, ButtonText, frameContrast)` (`hp:681-684`, `:697-699`); measured on
2026-11-20 on a checked tool button (`qt-fonts-widgets/out_widgets.txt`): #d1d1d2 /
#535659, over the fill kde-breeze already states.

- New field `toolbar.button_checked_border_color`: kde-breeze #d1d1d2 / #535659, the
  reader's mix of E6. Drawn at `button.border.line_width`, as the toolbar's hover
  outline is. Under the pointer and pressed, `toolbar.button_hover_border_color`
  applies, as Breeze turns any pen to the highlight then (`hp:710-712`).
- Drawn by iced (`toolbar_button`, the showcase), gpui (`geometry::toolbar_button_checked`)
  and egui (an accessor beside `toolbar_button_checked_background`, the showcase).

### E10 — An open combo box (B4)

An open combo box is drawn by Breeze as a pressed button, whatever the pointer does:
fill `mix(Button, Highlight, 0.3)`, pen `Highlight` (`st:7823-7853`, `hp:693-712`);
measured #c3e5f6 / #2f5368 with the pen #3daee9
(`docs/evidence/breeze-offscreen/2026-11-18/`).

- New fields `combo_box.open_background` (#c3e5f6 / #2f5368, the reader's mix of
  `button_bg` and `selection_bg` at 0.3, as for `button.active_background`) and
  `combo_box.open_border_color` (#3daee9, `selection_bg`), drawn at
  `combo_box.border.line_width`. Each unset, an open combo box keeps that part of
  today's look.
- An editable combo box draws a line edit's frame instead (`st:7838-7847`); the model
  has no editable combo box, so nothing follows.
- Drawn by iced (`pick_list::Status::Opened`) and egui (`widgets.open`, the combo
  role); gpui's `Select` draws `ring` and the focus band while open and exposes no
  open state: upstream (E14). It is the last item of its unit, and the first cut on an
  overrun (§4).

### E11 — Font sizes in whole pixels (F1)

Qt converts a point size to a pixel size, rounds it to hundredths and then to a whole
number, halves up (`qfontdatabase.cpp:2865-2869`, v6.12.0), in QtGui, which Qt Widgets
and Qt Quick share; the scale multiplies afterwards. Measured on 2026-11-20 under
`org.kde.desktop` (`qt-fonts-widgets/out_light.txt`, `out2_light.txt`): 10 pt is 13 px
for a Qt Quick `Text`, a QQC2 `Label` and Kirigami's default font, as for Qt Widgets;
8 pt is 11 px, 12 pt 16 px, 13.5 pt 18 px. The model gives 13.33 for 10 pt.

- New field `defaults.font_whole_pixels`: where true, every font size is rounded to
  whole logical pixels after its conversion from points, as Qt rounds. kde-breeze
  states true; every other preset leaves it unset, which rounds nothing: Windows'
  sizes are whole pixels already, macOS's points are pixels, and GNOME's rounding
  (Pango) was not checked, so rounding adwaita's 11 pt to 15 px would have no source.
- A flag, not pixel sizes in kde-breeze: a resolution at another font DPI (KDE's
  `forceFontDPI`) then rounds as Qt does, which fixed pixel sizes would not. It comes
  from the preset; the reader has nothing to read for it.
- Every KDE text size changes, by up to a third of a pixel at 96 DPI and up to half a
  pixel at another DPI, in all three connectors; in gpui the rem size follows the font
  size (`gpui-component` `root.rs`), so every rem-sized length there shrinks by 2.5 %
  on KDE too.

### E12 — The button's minimum width (Z1)

Breeze widens a button's contents to at least `Button_MinWidth` 80 and adds its frame,
2 on each side, inside which a 1 px shadow inset lies (`st:3555-3564`, `hp:670-671`):
the visible box of a text button is 82 wide at least. The model's minimum sizes are the
outer box without shadows (`PF` rule 1). kde-breeze, the reader (and its tests) and the
`PF` cell state 82 instead of 80. The padding of 6 is right: the "7 px" once measured
is from the line's outer edge. The minimum height is §3's.

### E13 — Docs that move with the fields

`REG`, `INH` (each new field in `[no_inheritance]` unless the specification finds an
inheritance with a source), the model's docs, `PF` rows (§2.1 for the font flag, §1.3.1
for its note, §2.3, §2.6, §2.9, §2.13, §2.15, §2.24; each new row with KDE's value and the
other platforms' cells marked not collected, naming the source the gap maps found), the
connectors' contracts and mappings, `docs/showcase-elements.toml`, the CHANGELOG (the
break, under D0 (a)), and the backlog: the entries these items close are closed, those
§3 leaves are rewritten to say what is left, and one is added for the menu's shortcut
text.

### E14 — What only GPUI Kit can change

gpui-component's own `PopupMenu` (the item's outline, the pressed item, the geometry),
`AppMenuBar` (the bar's and the titles' fills, the titles' padding), `ListItem` and
`DataTable` (the row's outline; the header's separators), `Slider` (the handle's
outline) and `Select` (the open fill) have no receiver. They are added to the deferred [upstream] entry in
`docs/todo.md`, beside the four requests, to be drafted when the maintainer sends
them; nothing is sent now. Each connector's README names what its toolkit cannot show,
as it does for the other upstream limits.

## 3. What stays in the backlog, and why

Each item below either has no receiver in the toolkits beyond a showcase's own
painting, lacks a value that can be reproduced, or does not fit the budget (§4).

- **The scroll bar (S1–S4).** gpui cannot draw any of the four without an upstream
  change (its track border is painted with zero widths, its thumb has no stroke and no
  per-view style, and its bar-hovered state takes the rest thumb style). The inner line
  can be drawn in iced and egui only by painting over the toolkit's bar; iced has a
  receiver for the handle's outline, and egui could scope the focused view's handle
  colour round a `ScrollArea`, but the showcases do not track which view has the
  focus. The focused view's handle is measured (#b1d9ef / #2a546b,
  `docs/evidence/breeze-offscreen/2026-11-13/`); the focused handle with the pointer on
  the bar and the handle while the pointer is on the bar are computed only. They are one
  unit, the scroll bar's lines and states, for the release that sends the scroll-bar
  request upstream.
- **A hovered row in a view without the focus (L2).** Two more fields (fill and
  outline) for a state the showcases do not track: they switch the selection on the
  window's activity, not the view's focus. The 2026-11-20 run settles what Breeze
  draws, for that entry: in an active window whose line edit has the focus, the view's
  rows are drawn without `State_Active`, from the Inactive group, the hovered row
  #ecf5fb / #16232b with the outline's pixel #cce5f7 / #1a3a4b (`views_light.txt`,
  `views_dark.txt`).
- **A selected row under the pointer (L3).** Breeze fills it with
  `Highlight.lighter(110)`, #44bfff (`st:4876`; measured). The reader would compute it
  with `QColor::lighter`, which is Qt's (LGPL): only a clean-room description may be
  used, and the plain rule (the HSV value times 1.1, clamped) gives #43beff, not the
  measured #44bfff; the measured value needs a reduction of the saturation by the
  overflow, which no source gathered states (the parked branch's `qt_lighter` is not
  implemented either). It waits for a clean-room description of that rule and test
  vectors; a field the reader could not fill would leave every other colour scheme on
  Breeze's value.
- **Sizes that are not minimums, or have no source (Z1's height, Z2, Z3).**
  kde-breeze's `button.min_height` 32 is the height at the default font, not a minimum,
  and gpui draws the field as an exact height (`docs/todo.md`, *gpui draws
  button.min_height as an exact height*). No source gives KDE's dialog size limits
  (`QDialog`, `QMessageBox`, `KMessageBox` and Kirigami's `Dialog` bound them by the
  screen or the font; `PF` §2.22: none): kde-breeze's 320 / 560 / 140 / 600 have no
  source, nor have the macOS presets', adwaita's two heights or ios-13's four.
  kde-breeze's `dialog.icon_size` 64 is right but no showcase reads it (the gpui
  showcase's dialog takes `defaults.icon_sizes.dialog` through `icon_size_dialog`, the
  Icon Sizes section's helper; the iced showcase's takes `icon_sizes.small`; egui's
  modal has no icon), and windows-11's, adwaita's and ios-13's 32 have no source. All of
  these are one change, required fields made optional as `toolbar.bar_height` was, the
  unsourced values unset, and a new dialog-icon helper beside `icon_size_dialog`; it
  touches every connector's size handling and about ten presets, more than the budget.
- **A list row's own box (L4).** kde-breeze's `list.border.padding` holds Breeze's
  highlight margins, not a padding; carrying Breeze's row (margins, padding beyond
  them, the highlight's 4.5 radius) needs a new field for the inset and a re-reading of
  every platform's `PF` §2.15 padding cells under the new meaning.
- **A menu's shortcut text (found on 2026-11-20).** Breeze letters an item's shortcut
  in a colour of its own at rest, under the pointer and pressed (measured, E2); the
  model has no field for it in any state. A new backlog entry; no showcase draws a
  shortcut in another colour than the item's text today.

## 4. Size, order and the line drawn

The maintainer gave the smaller details one to two days. The rate is 0.7.2's: on
2026-11-18 and 2026-11-19, each built from approved specifications with their reviews
and gates, nine fields were added a day (git log: 2026-11-18, one, two and six for
steps 2 to 5; 2026-11-19, three each for the caret's blink, step 6 and step 7); each
batch of six or three took under two hours from its first commit to its last review
fix, so the rest of those days went to specifications, plans and gates.

| Unit | Items | New fields | Other changes | Days at nine fields a day |
|---|---|---|---|---|
| The looks | E1, E3, E5 | 7 | the menu bar takes the toolbar's fill; kde-breeze's menu radius 4.5, which E1's outline is rounded by | ⅘ |
| Menus, the rest | E2, E4 | 5 | kde-breeze's padding and gap | ⅗ |
| Lists, controls | E6, E7, E8, E9, E10 | 8 | two reader writes; kde-breeze's slider hover fill removed | ⅞ |
| Fonts and sizes | E11, E12 | 1 | the button's minimum width; every KDE font size the tests expect | ⅑ |

The smaller details, the last three units, come to 14 fields, about one and a half
days. The unit of sizes that are not minimums (Z1's height, Z2, Z3, §3) would have
made required fields optional across about ten presets and in every connector's size
handling, taking the total past the two days; it is not taken.

The build follows the table's order. Each unit takes its own pass through the model,
`REG`, `INH`, the presets, the reader and `PF`, then its connectors and showcases, the
three connectors in parallel; so a unit not yet begun leaves nothing behind. On an
overrun (the project's rule: stop and re-scope at the first large one), the open combo
box is cut first, then units from the end, fonts and sizes first; the looks are never
cut. The gpui overlay that draws the list row's outline (E5) is the one receiver not
yet built: it is built first in its unit.

After the build, 0.7.2's close-out runs once more: archive, compatibility run, full
check, asset run (the captures change on every KDE page and in each menu, list, table,
slider, toolbar and combo box), the assets' commit, CI.

## 5. What was checked, and what was not

Checked on 2026-11-20: the Breeze rules and values above against its source at
`f0b1d753` and the offscreen measurements named; the toolkits' receivers in their
sources at the versions named; the presets' and the reader's present values; the
four requests to GPUI Kit against 0.10.0 (all hold; `docs/todo.md`).

Not checked:

- The highlight's 4.5 radius is read from the source; the measured corners fit it
  but a radius cannot be read exactly from pixels.
- A hovered menu-bar title was produced offscreen by an unsynthesized mouse event
  (the run's README says why); the open title by a click.
- Whether the bar's fill comes from Breeze's tools-area painting or from the menu
  bar's own palette, which is the Header set too: the colour is the same either way.
- Pango's rounding of font sizes; the 2 px by which a long button label's size hint
  exceeds its advance (E12 does not depend on it); whether `tab.min_width`, 80 in
  kde-breeze, is an outer box as the button's is (the specification reads it).
- The rate of nine fields a day (§4) is two days' figure, not a law.
- `MenuItemDrawStrongFocus` and the window border size, as said above, are Breeze's
  defaults, not read.
- The other platforms' values for every new field: their sources are named in the
  `PF` rows, their values not collected.
- That the gpui showcase can draw the list row's outline as an overlay box under
  `group_hover`: read, not built.
