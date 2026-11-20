# Issue #4: the KDE items still open after v0.7.1's first fixes — rationale

Status: written 2026-11-14, revised after six reviews. D2, D3 and D10 (a) approved
for step 1 (2026-11-15); D4 and D5 approved for 0.7.2 (2026-11-18); D6 and D7's first
half (steps 4 and 5) approved for 0.7.2 (2026-11-18); D8, D7's second half and D9 (steps 6 and 7) approved for
0.7.2 (2026-11-19), together with the text caret's blink (D0 (a)), by
`../archive/todo_v0.7.2_kde-open-items-steps6-7-{spec,plan}.md`. Step 1 (D2, D3)
implemented on 2026-11-17 for 0.7.1, by
`../archive/todo_v0.7.1_kde-open-items-step1-{spec,plan}.md`. D4's bullet on
gpui-component's widgets was rewritten on 2026-11-18 after three measurements: the
band follows keyboard focus through a plugin the connector registers, where the
first version switched it off altogether. Steps 2 to 5 (D4, D5, D6 and D7's toolbar
buttons) implemented on 2026-11-18 for 0.7.2, by
`../archive/todo_v0.7.2_kde-focus-ring-and-button-outlines-{spec,plan}.md` and
`../archive/todo_v0.7.2_kde-hover-outlines-and-toolbar-buttons-{spec,plan}.md`.
Steps 6 (D8) and 7 (D7's header, D9) implemented on 2026-11-19 for 0.7.2, by
`../archive/todo_v0.7.2_kde-open-items-steps6-7-{spec,plan}.md`. With step 7 every
step is done; this rationale stays in `docs/issues/`, as the first rationale does.
The items §0 leaves in the backlog were decided by
[`0004_kde-leftovers-rationale.md`](0004_kde-leftovers-rationale.md) and built in
0.7.2 on 2026-11-20, but for those its §3 keeps there.

It continues `docs/issues/0004_kde-hand-overrides-rationale.md` (called "the first
rationale" below), whose §5 left these items undecided, and the backlog entries
that rationale wrote (`docs/todo.md:1767-1933`, `:2742-2764`). The Breeze facts
are those documents' and are not repeated with their sources here; "measured"
means the offscreen runs in `docs/evidence/breeze-offscreen/`.

Abbreviations: `GC`, `GB`, `GP` are the `src/` folders of gpui-component 0.9.1,
gpui-base 0.9.1 and gpui-pre 0.3.12; `IW` is iced_widget 0.14.2's, `AW` iced_aw
0.14.1's; `EG` is egui 0.36.2's, `EX` egui_extras 0.36.2's; `CG`, `CI`, `CE` are
the `src/` folders of the gpui, iced and egui connectors; `F` is `docs/facts/` on
the parked branch `v0.8.0-rc1`, `P8` that checkout; `PF` is
`docs/platform-facts.md`. Lines are read at commit `0fa28027`.

## 0. What is decided here

Eight items: the six of the reporter's list that v0.7.1's first fixes left open,
and two defects found while reading for them.

| # | Item | From |
|---|---|---|
| 1 | A `DataTable`'s scroll bar box is 16 px whatever the theme's track width | reporter, A2 |
| 2 | The table's grid colour | reporter, B9 |
| 3 | The menu panel's colour | reporter, B6 |
| 4 | The focus ring | reporter, B4 |
| 5 | The outlines Breeze shows hover and focus with: a button's border (D5); a check box's, a radio's and a combo box's (D6) | reporter, C |
| 6 | Toolbar buttons' states; the table header's hover and pressed fills | reporter, C |
| 7 | The default button | found |
| 8 | The selection of a view without the keyboard focus | found |

Not decided here, and staying in the backlog as they are:

- three more outlines the first rationale put into the same step as items 5 and
  6 (`docs/todo.md:1785-1795`): a hovered menu item's and a hovered list row's
  1 px outline, and a menu bar title's fill. They are of the same kind as
  D5 to D7 and follow them; the menu bar title waits for the menu bar's own
  fields (`docs/todo.md:1183-1200`);
- the menu item's radius, padding and icon gap; the menu row's height; the
  scroll bar's inner line and focused handle; the slider handle's outline; the
  font's whole pixels; the button's and the dialog's sizes.

## 1. What the three toolkits can receive

Read on 2026-11-14. "No receiver" means the toolkit's widget offers no value,
style or hook for the property.

| Property | gpui-component 0.9.1 | iced 0.14 | egui 0.36.2 |
|---|---|---|---|
| A button's border colour per state | none in the stock variants: hover, press and selection all draw the variant's one border colour, `input` for the default variant (`GC/button/button.rs:1084-1126`, `:1222`, `:1298`, `:1340`), and upstream's own hover and press styles replace a caller's (`:738-743`, `:751-762`); a custom variant's border is its `color` in every state, and its fill at rest a fifth of that colour (`:40-46`, `:1028`, `:1118-1124`), which is a route to try (D5) | yes: the style function gets `Active`, `Hovered`, `Pressed`, `Disabled`, each with a border (`IW/button.rs:471-498`); no focused state, a button cannot take the keyboard focus (`:257-273`) | yes: `bg_stroke` per state (`EG/style.rs:1290-1320`); keyboard focus and press share the `active` state (`:1273-1285`); the connector writes one stroke into every state today (`CE/style/states.rs:171-208`) |
| A check box's, a radio's and a combo box's hover outline | upstream's check box, `Select` and `Combobox` have no hover style (`GC/checkbox.rs:249-250`, `GC/select.rs:545`); the connector's own check box and radio can draw one (`CG/widgets/checkbox.rs`) | yes: `Hovered` status with a border (`IW/checkbox.rs:508-537`, `IW/pick_list.rs:838-863`) | yes: the state's `bg_stroke` (`EG/widget_style.rs:174-190`, `EG/containers/combo_box.rs:446-463`) |
| A toolbar button's states | a ghost button; the connector's `variants::ghost_button` is a custom variant built from tokens, with one fill for press and selection (`CG/variants.rs:51-65`; `GC/button/button.rs:1294`, `:1337`) | no toolbar widget; the showcase styles its own (`showcase-iced.rs:4634-4660`) | no toolbar widget; the «toolbar» role takes the base's button states (`CE/style/roles.rs:702-739`) |
| A table header's hover and pressed fill | none (`GC/table/state.rs:1929-1968`) | none: the table has no style setter and a header is the application's own element (`IW/table.rs:35-44`, `:149-187`) | none: the header row is laid out without hover (`EX/table.rs:490-506`) |
| The focus ring's colour, width, offset | one colour, `ring`: it tints a focused control's border where the control has one (button, check box, radio, switch, input, select, and seven more places, among them `GC/button/button.rs:921`, `GC/checkbox.rs:292`, `GC/radio.rs:225`, `GC/switch.rs:282`) and fills a band 3 px wide whose alpha upstream sets to 0.5 whatever the colour's own (`GC/styled.rs:11-12`, `:250-263`); the slider's hover halo is drawn in it too, whatever the switch (`GC/slider.rs:44-66`, `:202-205`). A switch, `Theme::focus_ring`, turns the band off; width and offset have no receiver. The connector's own check box, radio, slider and switch draw the model's ring exactly (`CG/widgets/mod.rs:301-397`) | none in iced; the connector's `focus_ring()` wrapper draws the model's ring around a text field, and draws a ring that lies on the field's border (offset = −width) as that border (`CI/focus_ring.rs`; `CI/styles.rs:580-589`) | none in egui; the connector's plugin paints the model's ring around the focused widget (`CE/plugin.rs:576-669`) |
| The ring only for keyboard focus | GPUI knows how the last input came (`Window::last_input_was_keyboard`, `GP/window.rs:3641-3645`). A button does not take the focus on a click (`GC/button/button.rs:863-874`). An `Input`, a `Select` (which draws the band while open, too), a `Combobox`, a `Switch`, the OTP input, the colour picker's field and a date picker after a date is picked do, and draw the band then (`GB/switch.rs:376-381`, `GC/switch.rs:281-283`, `GC/input/otp_input.rs:129-131`, `GC/color_picker.rs:490`, `:764-779`, `GC/time/date_picker.rs:416`, `:732-737`, `GC/select.rs:545`). `last_input_was_keyboard` follows the last event: any mouse move sets it back (`GP/window.rs:5873-5881`) | iced records nothing; only text fields take the focus, and a click gives it (`IW/text_input.rs:718-735`); the wrapper sees the events | egui records nothing (`EG/memory/mod.rs:500-546`); a click gives the focus only to a text edit and a drag value (`EG/widgets/text_edit/builder.rs:815-817`, `EG/widgets/drag_value.rs:665-667`); the plugin sees the input |
| The default button | the primary button's own tokens, `button_primary*` (`GC/button/button.rs:1021`, `:1171`, `:1255`), beside `primary`, which thirteen other widget files read as the theme's accent | a style function per button (`CI/styles.rs:392-428`) | a selected button in the «button» role (`CE/style/roles.rs:228-230`) |
| The menu panel's fill | none: a menu is filled with `popover`, as every popup (`GC/menu/popup_menu.rs:1610-1611`) | yes: the overlay menu's and iced_aw's menu's backgrounds (`IW/overlay/menu.rs:598-610`; `AW/style/menu_bar.rs:7-26`, where the bar and the menu are separate fields) | `window_fill`, which also fills windows, popups and tooltips (`EG/containers/frame.rs:196-221`) |
| A table's grid | `table_row_border`; `chart_grid` is another token (`GC/table/state.rs:2374-2376`; `GC/chart/mod.rs:493`) | none in the connector; the showcase paints its table's lines | none drawn by egui or egui_extras; the «list» role's `noninteractive.bg_stroke`, which egui_extras uses for a row's overline and a resize line (`EX/layout.rs:233-239`) and egui for a separator and a group frame inside the scope |
| The selection of an unfocused view | none: no list, tree or table changes its selection when it loses the focus (`GC/list/list_item.rs:257-264`, `GC/table/state.rs:2536-2543`) | none (`CI/contract/derived.rs:1390-1397`) | none: one selection pair (`EG/style.rs:1194-1200`) |

What the other platforms do, where the parked 0.8.0 facts state it:

- The ring shows for keyboard focus only on KDE, Windows and Material
  (`F/kde/tokens.toml:1586`, `F/windows/tokens.toml:214`,
  `F/material/tokens.toml:10072`), on any focus on iOS
  (`F/ios/tokens.toml:21008`); GNOME shows it for keyboard focus with a time-out
  of 3 s (`F/gnome/tokens.toml:2009-2016`), and whether its text fields show it
  on a click was not read; macOS is pending.
- A default button is a state of its own on KDE, macOS and Windows, and looks
  like the accent-filled button on the latter two (`F/macos/button.toml:13712`,
  `F/windows/button.toml:10436-10513`).
- A flat button on GNOME and Windows changes its fill, not its outline
  (`F/gnome/button.toml:5014-5058`, `F/windows/button.toml:8499-8546`); on KDE
  its hover fill is transparent (`F/kde/button.toml:2945`).
- The facts files have nothing on a table header's states, a table's grid or a
  menu's panel beyond what `PF` says.

## 2. Decisions

### D0 — The version of a release that adds a model field

Steps 2 to 7 below add public fields to model structs that are not
`#[non_exhaustive]`. Code outside the crate that builds such a struct with every
field named then stops compiling; that is a breaking change by Cargo's rules,
the release check's compatibility step reports it and asks for a version
decision (`scripts/check_release.sh:1129-1141`), and 0.7.0, a minor release,
listed the same kind of change under *Breaking Changes* (`CHANGELOG.md:66-68`).

The decision is the maintainer's, once, for every such step (the caret
document's too; its B6 lists the same options):

- (a) release as 0.7.x and name the break each time. The roadmap plans fields in
  0.7.x (`ROADMAP.md:147-150`) and the project promises no compatibility before
  1.0; a patch release then breaks the few callers that name every field.
- (b) a minor version for each release that adds a field.
- (c) one release that makes the model's input structs `#[non_exhaustive]`, after
  which a field is never a break again: the larger break once, in every place
  that builds a model struct.

**(a) is proposed**, as the roadmap has it. With the caret document's, the
queue is, under S5, four releases that add fields (the three KDE pairs and the caret), 18 fields in all, so (a) means four
such notes; (c) would end them, but its one break stops the usual way of
building a theme in code, a struct literal with `..Default::default()`, for
every user, where (a) breaks only code that names every field. Step 1 adds no
field and is not affected.

### D1 — The scroll bar box in a `DataTable`: upstream's, and said

The table sizes the box with a constant, 16 px (`GC/table/state.rs:2732-2764`),
and offers no setting for it; the themed width that every other scroll host
uses is not public. The options were to cap the theme's track width at 16 for
the whole connector, which would narrow every scroll bar to fix one host, or to
leave the theme's width and say what happens in a table. The second was chosen
in v0.7.1's first fixes: the README states the limit and the workaround, and
request 1 of `docs/issues/0004_kde-hand-overrides-upstream-requests.md` asks
upstream for the change. Nothing more can be done in this repository. The item
closes when the request is sent and answered; sending it is the maintainer's.

### D2 — The table's grid: Qt's colour, and the chart's grid from the separator's

**Fact.** A `QTableView` draws its grid in `QPalette::Mid`: KColorScheme's
`MidShade` of the window colour at the scheme's contrast, #c4c8cc / #1c1f21
under Breeze (measured). kde-breeze's `list.grid_color` inherits the border
colour, #bcc0bf / #4d545b: far brighter than Qt's in dark mode, which the
reporter saw.

**What held it back.** Two readers of the field are not a table's grid: gpui's
`chart_grid` token (`CG/colors.rs:446`) and the lines egui itself draws inside a
list scope (`CE/style/roles.rs:776-791`).

**Options.**

- (a) Leave it; an application sets gpui's `table_row_border` itself.
- (b) State Qt's value, have the KDE reader derive it, and give gpui's
  `chart_grid` another source.
- (c) A new field for the table's grid, apart from the list's.

**(b).**

- **gpui's chart grid.** With Qt's colour a chart's grid would all but vanish in
  dark mode (#1c1f21 on the window's #202326). It needs a source of its own, and
  the candidates each move some preset, since `chart_grid` reads the table's
  grid today because `PF` gives macOS and Material a value there
  (`CG/colors.rs:441-445`):

  | Source | kde-breeze | adwaita | windows-11 | macOS (both) | material-3 | ios-13 |
  |---|---|---|---|---|---|---|
  | `list.grid_color` (today) | Qt's grid, after this change | the border colour | the border colour | #e6e6e6 / #1a1a1a | #cac4d0 / #49454f | the border colour |
  | `defaults.border.color` | unchanged from today | unchanged | unchanged | #00000019 / #ffffff19 | #79747e / #938f99 | unchanged |
  | `separator.line_color` | unchanged from today (it inherits the border) | #d5d5d5 / #4a4a4e | unchanged | #00000019 / #ffffff19 | unchanged | unchanged |

  **`separator.line_color`.** A chart's grid is a set of separating lines, which
  is what that field means, and no platform states a chart's grid. It keeps
  material-3's value; on macOS the grid takes the separator colour where it had
  the table's grid colour; on adwaita it takes the separator colour where it
  had the border. The border colour would move material-3 as well. On the
  presets where the chart's grid changes, its contrast on the window colour
  stays as low as a grid's is meant to be (computed from the presets: between
  1.24 and 1.41 on the macOS presets and in adwaita's light mode, 1.80 in
  adwaita's dark mode, where it is 1.61 today; in macos-tahoe's dark mode it
  rises from today's 1.04 to 1.34). `table_row_border` keeps
  `list.grid_color`, which is what it is.
  The values are the presets'; the specification re-reads each and names the
  tests and panels that state the mapping (`CG/colors.rs:835-849`,
  `CG/contract.rs:255-259`, the showcase's chart panels and theme map).
- **egui.** Nothing changes in the connector. The «list» role's stroke is
  documented as the table's lines (`CE/style/roles.rs:776-779`), and with Qt's
  colour those lines become what Qt draws. The first rationale said a list's
  outline would vanish, and asked for the role to separate the frame from the
  grid first. That holds for a separator or a group frame egui draws inside a
  list scope, which take the grid colour and become as faint in dark mode as
  Qt's grid is; it does not hold for the list's own frame, which is drawn per
  call from `list.border.color` (`CE/mapping.md:381`). The cost is accepted,
  and the precondition dropped: the role's stroke is the grid by its
  documentation, Qt has no separator or group inside a table to compare with,
  and an application that wants a frame there has the per-call route.
- **iced.** No reader in the connector.
- **The showcases.** Each of the three paints the lines of its own table from
  `list.grid_color` (`showcase-iced.rs:11887`; `showcase-egui/pages/basic.rs:1757`;
  `showcase-gpui/demo.rs:9418`), so under kde-breeze those lines, and the
  release's screenshots of them, become Qt's grid; so do the lines of
  gpui-component's own tables, in the showcase and in every gpui application,
  through `table_row_border`.
- **The reader** derives the colour, so a user's own colour scheme gets its own
  grid: `MidShade` of `[Colors:Window] BackgroundNormal` at `[KDE] contrast`,
  and at KColorScheme's own default, 7, where the key is missing (the first
  rationale, §5.6). The
  shade function exists on `v0.8.0-rc1`, written in a clean room and checked
  against tables made from KDE's code (`native-theme/src/platform/kde_shade.rs`,
  434 lines, self-contained; `native-theme/tests/vectors/kde_shade.tsv`). It is
  carried over with its test and with the clean-room notice, which on that
  branch is in the module's parent (`native-theme/src/platform/mod.rs:1-18`).
- (c) would split a field for one platform when no reader needs the split once
  the chart has its own source.

### D3 — The menu panel: Breeze's colour, and iced's menu bar as the showcase draws it

**Fact.** Breeze's menu panel is `mix(Window, Base, 0.3)`, #f4f5f5 / #1c1f22
(measured); kde-breeze's `menu.background_color` inherits the window colour,
#eff0f1 / #202326. A combo box's popup is another surface: its rows lie on the
window colour (measured on the popup's view,
`docs/evidence/breeze-offscreen/2026-11-14/combo_light.txt:1-4`,
`combo_dark.txt`).

**What held it back.** egui fills every window, popup and tooltip with the
field, and the iced connector fills its iced_aw menu bar and its drop-downs
with it. The first rationale (§5.5) judged that the change would move more
surfaces away from Breeze than toward it. This decision reverses that for
iced, where the menu bar can be given its own source in the same change and
the drop-down's cost is accepted (option (i) below), and accepts for egui what
its bullet below states.

**Options.**

- (a) Leave it: the difference is four to five levels.
- (b) State Breeze's value (preset and reader), and correct iced_aw's bar.
- (c) (b), and give egui's `window_fill` another owner than the menu.

**(b).**

- **iced.** The connector's `styles::aw::menu` fills the bar and the menu from
  the same field and frames both with `menu.border`
  (`CI/styles/aw.rs:144-156`), though iced_aw has fields for each. The
  showcase's window menu bar does not use it (its Extra page's menus do, and
  change with it): its own style fills the bar from the window colour without a
  frame and frames the popup with `popover.border`, and says why
  (`showcase-iced.rs:6515-6537`). The connector's style becomes the showcase's.
  The showcase's window menus keep their own function, because they are built
  without the connector's `iced_aw` feature too, which the connector's style
  needs, and a test holds the two equal, so there is one look (the
  specification's decision 12 weighs the other ways). What changes for an
  application that uses the connector's style:
  - the bar is filled from the window colour: a change on windows-11 and
    material-3, where the two colours differ today, and what keeps the bar
    the window's colour on kde-breeze when the menu's changes;
  - the popup gets a frame and rounded corners. `menu.border` resolves to no
    line at all (the menu's border has no kind of its own,
    `native-theme/src/model/widgets/mod.rs:257`), so today the popup has none;
    with `popover.border` it gets the platform's popup frame and radius on
    every preset, the colour-scheme presets included, but material-3, where
    nothing changes (its popover states no line and the same radius). The
    facts give the popup's frame to the popover, not to the menu (`PF` §2.6;
    the showcase says so at `showcase-iced.rs:6513-6517`);
  - the bar has no frame of its own; on material-3 it loses the 4 px radius it
    takes from `menu.border` today;
  - the popup's shadow follows `popover.border`'s, which is on in every preset
    today.
  The connector has a second menu style, `styles::menu`, for the drop-down of
  a pick list and a combo box (`CI/styles.rs:1233-1256`). It fills from the
  menu's field and frames with `menu.border`, that is, not at all. The facts
  file puts a drop-down with the popover (`PF` §2.16, "Popover / Dropdown"),
  and Breeze's is on the window colour, which is kde-breeze's popover colour.
  The options for it:
  - (i) the drop-down keeps the menu's fill and takes the popover's frame
    line, the same rule as the connector's menu style above: the menu's
    panel, framed as a popup;
  - (ii) it takes fill and line from the popover;
  - (iii) it stays as it is, without a frame.

  **(i).** (ii) is where the facts point, and it was this document's first
  choice, but the rows of a drop-down are drawn with the menu's own colours
  (`menu.hover_background`, the menu's text), which each preset states
  against the menu's fill. On the popover's fill they do not hold: in
  adwaita's dark mode the highlighted row, #3a3a3e, would lie on #36363a
  where it lies on #222226 today (1.06:1 against 1.40:1, with the same white
  label on both, so the fill is the only mark, also for the row the arrow
  keys choose), and the label's contrast would fall on 24 of the 25
  combinations whose fill changes, rising only in adwaita's light mode
  (tokyo-night light 4.52 to 3.99, solarized light 4.13 to 3.64; computed by
  the specification's confirming checks from the preset files). (ii) needs a highlight that
  is stated against the popover's fill, which the model does not have and
  libadwaita's value for which was not read; the backlog gets the entry.
  (iii) leaves a popup without the platform's frame. The cost of (i): on
  kde-breeze the drop-down takes the new menu panel colour and so lies four
  to five levels from Breeze's combo popup, the cost this decision already
  accepts for egui's popups. What changes:
  - the fill: only what kde-breeze's new menu colour brings;
  - the frame: on every preset but material-3 the drop-down gets the
    popover's line where it has none today (the specification lists the
    combinations and any in which the line has the fill's own colour and is
    drawn without being seen).
  Its corners stay as they are, square on every preset but material-3. iced
  draws the highlighted row of a drop-down with the same radius as the panel
  (`IW/overlay/menu.rs:542-553`), so the popover's large radius (8 px on
  kde-breeze, 15 on adwaita) would round every hovered row to a pill, and no
  fact says a platform draws that. It keeps no shadow, as today: iced's style
  for it has a shadow that is none by default, and the model has no shadow
  geometry to give it.
  The contract rows and the showcase text that state today's mapping change
  with it (`CI/contract/rows.rs:1515-1518`, `:2597-2640`,
  `CI/contract/derived.rs:836-837`, the showcase's Extra page). The model still
  has no field for a menu bar's fill (`docs/todo.md:1183-1200`); the window
  colour is what the bar lies on.
- **egui.** Unchanged. The connector's design gives `window_fill` to the menu
  (`CE/mapping.md:136`), and an application that wants a window's, a dialog's or
  a tooltip's own fill takes the surface frames for them
  (`CE/style/frames.rs:131-170`). So on KDE every menu becomes Breeze's, and
  the other surfaces egui fills with the same colour move by four to five
  levels (lighter in light mode, darker in dark): an `egui::Window`, a modal
  and a tooltip drawn without their surface frames, as they already differ on
  windows-11 and material-3, and every other popup, a combo box's among them,
  which egui fills as it fills a menu (`EG/containers/frame.rs:205-221`) and
  Breeze fills with the window colour.
  That cost is accepted: the field means the menu's panel, the connector's
  design gives egui's one fill to the menu, and the difference is the smallest
  the eye can find. (c) would change windows-11 and material-3 for it.
- **gpui.** No receiver in gpui-component; the backlog's upstream entry for a
  menu surface token stands (`docs/todo.md:2403`). The showcase's own popups,
  which it fills from the field, take Breeze's colour. One of the connector's
  contrast checks is touched: it holds that a hovered menu row contrasts no
  worse on the token gpui-component fills a menu with than on the theme's
  own menu colour, and with Breeze's panel in kde-breeze the token's side is
  the lower one (about 10.5 against 10.9 in light mode, 8.8 against 9.2 in
  dark; the specification computes them). The cause is the missing menu
  surface, so the check gets a stated exception for kde-breeze's two modes
  rather than a changed value.
- The frame of a menu is not changed by this decision: in all three
  connectors it is the popover's border, Qt Quick's #c2c5c9 / #474c51 in
  kde-breeze, while the Qt Widgets menu's measured outline is #c6c8c9 /
  #4c4e51 (`PF:1253`). After this step kde-breeze's menu fill follows Qt
  Widgets and its frame Qt Quick, up to five levels apart in a 1 px line;
  the backlog gets an entry for it.
- The reader derives it with the mix function it already has
  (`native-theme/src/kde/colors.rs:234`).
- Qt Quick's menu fill was not read; kde-breeze follows Qt Widgets here as
  elsewhere, and the facts row says which it is.

### D4 — The focus ring: Breeze's values, and a field for "keyboard focus only"

**Facts.** Breeze's ring is a band 2 px wide in the selection colour at alpha
0.3, from 1 px inside the control to 1 px outside, drawn only when the focus
came from the keyboard. A click into a line edit recolours its frame and draws
no band. kde-breeze states an opaque colour, width 1, offset 2, without a
source.

**The two parts of the problem.**

1. The values. With the model's colour, width and offset, the iced wrapper, the
   egui plugin and the gpui connector's own widgets draw exactly Breeze's band.
   gpui-component's `ring` token cannot take the translucent colour as it is:
   it also tints a focused control's border, which would turn faint.
2. When it shows. All three connectors draw the ring on any focus. In iced only
   text fields take the focus on a click, in egui text fields and the drag
   value; in gpui-component also a
   select, a combo box, a switch, and three more (§1). So the visible error
   is a band around a control the user clicked.

**Options for when it shows.**

- (a) Leave it; an application turns gpui's ring off.
- (b) A field that says whether the platform shows its ring only for keyboard
  focus; each connector follows it.
- (c) The connectors always draw the ring for keyboard focus only.

**(b).** (c) would be wrong for iOS and a guess for macOS and GNOME. The field
is `defaults.focus_ring_keyboard_only`, optional: true for kde-breeze,
windows-11 and material-3, false for ios-13, and unset for adwaita, the macOS
presets and the colour-scheme presets. Unset means what the connectors do
today. adwaita stays unset although GNOME's rule is "keyboard": its text fields
may show their ring on a click, which was not read, and its rule has a time-out
a yes-or-no field does not carry; the backlog gets both.

**What each connector does where it is true.** In all three, whether the focus
came from the keyboard is decided at the moment a control gets the focus and
holds until the control loses it; a focus given before any input did not come
from the keyboard. Breeze decides by the reason of the focus change (Tab,
Backtab or a shortcut). No toolkit of the three reports a reason, so each
follows the nearest rule it can: the ring shows for a focus that followed a
key, where the last input before it was a key and not the pointer (a move, a
button or a touch). A rule that followed the last input at every moment would
take the ring away at the next mouse move, which GPUI's own flag does (§1).
The rule's cost is the same in all three: a focus the application itself moves
after a key that is not Tab (Enter, for one) shows the ring too.

- iced: each wrapper remembers, in its own state, whether the last key or
  pointer event it saw was a key. A column or a row hands every event to every
  child (`IW/column.rs:272-282`), so a wrapper sees them for as long as it
  exists, and a wrapper that does not exist holds nothing that could go
  stale. Two places keep an event from it: a stack does not hand a lower layer
  what a higher one took (`IW/stack.rs:262-264`), and the base does not get
  what an overlay took; a field under an open dialog can so remember a key
  where a click came last, and show the ring once. When its field gets the
  focus the wrapper keeps the answer until the field loses it. A field built
  in answer to the key itself has seen nothing, and shows none. The other way
  weighed was a pair of operations of the connector's own for Tab and
  Shift+Tab that mark the focus as given by Tab: Breeze's reason exactly for
  Tab, but it would show no ring for an application that moves the focus with
  iced's own operations, as an application with Tab handling does today, nor
  after a shortcut. iced moves the focus by Tab
  only if the application does it (its text input has no Tab handling, and
  `focus_next` is an operation an application sends,
  `iced_runtime-0.14.0/src/widget/operation.rs:50-56`); the README says so,
  and the showcase, which has none today, gets a Tab handler.
- egui: egui moves the focus by Tab itself. The plugin remembers the kind of
  the last input across passes (a drag value takes the focus in the pass of
  the release, not of the press) and, when the focused widget changes, keeps
  the answer for that widget.
  A text edit shows its focus by its own frame, which already takes
  `input.focus_border_color` (`CE/accessors.rs:400-428`).
- A consequence in all three showcases: each gives a text field the focus at
  start-up so that its focused look shows in the first frame. Under the three
  presets that field then shows its focused frame without the band, as the
  platform would after a focus the program set, and the screenshots change
  accordingly.
- gpui, the connector's own widgets: each reads `last_input_was_keyboard` when
  it gets the focus and keeps the answer in its state until it loses it.
- gpui-component's widgets: `apply` leaves the band off (`Theme::focus_ring`),
  because upstream draws it on a click into seven kinds of control and offers
  no way to tell, and registers a plugin of gpui-base's `Root` that turns it on
  while the window's focus is one that followed a key, by the rule the
  connector's own widgets follow. `Root` runs its plugins before it builds a
  window's content (gpui-base 0.10.0 `src/root.rs:74-80`, `:246-252`), and
  every gpui-component window has a `Root`, so upstream's controls show their
  band after Tab and none after a click, as the platform does, with nothing for
  the application to call. The band is upstream's own: 3 px outside the
  control's edge, in `ring` with its alpha set to 0.5 (gpui-component 0.10.0
  `src/styled.rs:11-12`, `:301-322`), and no theme value reaches its width or
  its place (D4's first part). After a click a focused control shows its tinted
  border, and one without a border a 1 px line (`GC/styled.rs:265-297`): in the
  ring's colour on a link, a text or an upstream ghost button, and in its text
  colour at 60 % on any other borderless button, as on material-3
  (`GC/button/button.rs:21`, `:644-650`; `GC/styled.rs:273-297`).

  Measured on 2026-11-18 on `v0.7.2-rc1` at `b434b9bc`, under kde-breeze, with
  an upstream `Input`, `Button` and `Switch` in a `Root` (the record of 0.7.2's
  plan, `steps23/trial/band-switch-plugin-output.txt`), on both backends: a band
  on the focused control after Tab and Shift+Tab, kept through a mouse move,
  none after a click, each in the frame of the focus move itself. That holds
  because gpui redraws the window at every focus move (gpui-pre 0.3.12
  `src/window.rs:2621-2631`) and gpui-component reads the switch at each render
  (0.10.0 `src/styled.rs:250-262`). Writing the flag during a render started no
  render loop, the plugin added no painted quad and moved no control, and
  nothing in gpui-component 0.10.0 observes its `Theme` global.

  The other ways, weighed:
  - Leaving the band off altogether, this decision's first version. It loses
    the band the platform draws after Tab on every upstream control, where it
    can be had. Its reason was that each switch would re-render everything; the
    focus move re-renders the window anyway.
  - A function the application calls first in the `render` of each window's
    root view. Measured, with the same rows. It needs a line in every
    application, without which the band stays off; the plugin needs none.
  - Observers the connector installs itself from `apply` (gpui's keystroke
    observers, and a focus-out listener on the control a key focused). Measured
    too: the value is right, but focus listeners run after the frame is
    painted, so the frame of a click still paints the band on the clicked
    control, and it goes a frame later.
  - Keeping the band on any focus: it shows at every click into a text field, a
    select or a switch, the more frequent case and the one the reporter turned
    off by hand.

  What the plugin costs. The flag is one for the application while the focus is
  per window, so each window's `Root` sets it when it renders; a view that is
  drawn again without its `Root`, which GPUI Fast does, after another window's
  `Root` rendered, reads that window's answer until its own `Root` renders
  again (two windows were measured: each painted its own band rightly). A
  `Root` made before the first `apply` has no plugin. On a theme that says
  keyboard only, the plugin writes `Theme::focus_ring` at each render, so an
  application's own write of it does not stand there; on any other theme the
  plugin writes nothing. Request 4 to upstream still asks for the band on
  keyboard focus only; when upstream has it, the plugin is no longer needed.

**The values, and gpui's `ring`.**

- kde-breeze states the selection colour at alpha 0.3, width 2, offset −1, and
  the reader derives the colour from `[Colors:Selection] BackgroundNormal`
  (today it takes `[Colors:View] DecorationFocus`, opaque,
  `native-theme/src/kde/colors.rs:478-490`); the accent the portal gives is
  applied to it at the same alpha. The comment in
  `native-theme/src/gnome/mod.rs:782-786` that says Breeze's ring is opaque is
  corrected, and the caret's KDE rule in `docs/inheritance-rules.toml:314`,
  which names the ring colour, names `[Colors:View] DecorationFocus`, which is
  what the reader writes for the caret (`native-theme/src/kde/colors.rs:565`).
- gpui's `ring` token. Options: the ring colour as stated (a focused control's
  border turns faint on KDE); `input.focus_border_color` (it is the input's
  field, and `ring` tints a button's, a check box's and a switch's border and
  the slider's halo as well: on windows-11 and material-3 those would take the
  accent where the platform's focus colour is another); the ring colour without
  its alpha where the band is switched off, and as stated where it is on.
  **The third.** Wherever `apply` leaves the band off (the new field
  is true, or the theme's ring has no width, which switches it off today),
  `ring` is the colour of a focused control's line, and of the band the
  plugin turns on after a key, whose alpha upstream sets itself
  (0.5, whatever the token's own), and a line at a third
  of its colour is not what any of the three platforms draws: Breeze turns a
  focused frame the full highlight. Where the band is on, nothing changes from
  today. The effect per preset: kde-breeze keeps the colour it has today,
  #3daee9; windows-11's light #000000e4 loses its last tenth of transparency
  and its dark #ffffff is opaque already, as material-3's are; every other
  platform preset has the band on and is untouched. The slider's halo does not
  change: upstream sets its alpha itself. The contract table states the rule.

### D5 — Outlines: a button's border on hover and on keyboard focus

**Fact.** Breeze shows hover, keyboard focus and press on a button by turning
its 1 px outline to the highlight colour; the fill does not change on hover.
The model has such fields for text inputs only, so a connector's button on KDE
shows no hover at all.

**Options.**

- (a) Two optional fields on `ButtonTheme`, `hover_border_color` and
  `focus_border_color`, as `InputTheme` has.
- (b) Three, with a pressed border.
- (c) One, used for all three states.

**(a).** The names and meanings are the ones the input already has. A pressed
button is under the pointer, so a connector keeps the hover border while it is
pressed. A pressed field would be exact on no further platform: Windows's
pressed outline differs from its rest one (`F/windows/button.toml:3643-4059`),
but its rest outline is a gradient, which the model's one border colour cannot
state, so a pressed colour alone would say nothing true. (c) would give the
keyboard focus a colour named after the pointer. kde-breeze and the reader
state the highlight for both; no other preset states them: on GNOME a button
has no outline, on Windows the outline is the same at rest and on hover, on
macOS hover draws nothing (`F/gnome/button.toml:166`,
`F/macos/state-models.toml:2`).

**Connectors.**

- iced: the button styles take the hover border in `Hovered` and `Pressed`.
- egui: the base and the «button» role write it into `hovered`, and the focus
  border, else the hover border, into `active`, which egui uses for press and
  keyboard focus alike.
- gpui: upstream's default button draws the `input` token's colour as its hover
  and press border, and that token is the text input's own border, so it cannot
  be set for buttons. `ring` tints a focused button's border where it has one
  (where it has none, gpui-component draws a 1 px line, in `ring` on a link,
  a text or an upstream ghost button and in the button's text colour at 60 %
  on any other, or the band where that is on), one colour for every control (D4), so `focus_border_color` has no receiver of its own there. A
  custom variant draws its `color` as the border. Whether a custom variant can
  stand in for the default button is **not known**: its border is that colour
  at rest too, its rest fill is a fifth of that colour unless the caller's
  style replaces it, and it has one text colour for all states. The plan tries
  it in a build first, with every state checked against the default button's:
  the fill, the border and the text colour at rest, on hover and press,
  disabled, focused and selected. It passes only if the hover and press
  border is the one thing that differs. A variant that passes on the rest
  look and differs in another state is not shipped with a note: it would
  trade a state the connector draws from the theme today for the outline.
  If the trial passes, the connector offers the variant and the showcase
  uses it; if not, gpui has no receiver, the contract says so, the trial's
  record names the state that stood in the way, and a request to upstream
  (a border colour per state) joins the four.

### D6 — Outlines: the check box, the radio and the combo box

Breeze shows hover on a check box by stroking the box's outline again in the
focus colour, on the rectangle its frame is drawn on, with the frame's pen
width (`kstyle/breezehelper.cpp:844-863` and `:895-907` at `f0b1d753`, read
2026-11-15), so its border turns that colour, and on a combo box
by the highlight outline, with no fill. kde-breeze states a fill for both (#93cee9),
which the backlog records as standing in for the outline
(`docs/todo.md:1291-1331`).

Two optional fields, `checkbox.hover_border_color` and
`combo_box.hover_border_color`; kde-breeze and the reader state them and drop
the two stand-in fills. The alternative, keeping the fills, keeps a colour
Breeze does not draw. A connector recolours the box's own border, which is
what Breeze draws.

- The other platform presets: a preset states a new field where
  `docs/platform-facts.md`, the facts of record for 0.7.x, gives the
  platform's value with its source, as one colour for everything the field
  covers, and stays unset otherwise, with a backlog entry that names what is
  to be read or what the one field cannot state. So
  material-3 states `combo_box.hover_border_color`, the hover outline of
  Material's select (`docs/platform-facts.md:1625`). adwaita does not state
  `checkbox.hover_border_color` in this step: GNOME's ring is recorded for an
  unchecked box (`:1244`) and its checked box under the pointer was not read,
  and the field covers both. material-3 does not state it either: Material
  turns an unchecked box's outline to `on-surface` under the pointer and
  draws no outline on a checked one (`:1230`), which one colour cannot say.
  The presets whose platform's hover stroke was
  not read stay unset, and their facts cells say "not read", not "none".

- The radio has no fields of its own: it reads the check box's in all three
  connectors (`CI/styles.rs:870-890`; `CG/widgets/checkbox.rs:110-135`;
  `CE/style/roles.rs:390`, `:409`). So it takes the new field as it takes every
  other, and the fill cannot be kept for it alone. Breeze draws a radio's
  hover as it draws a check box's, and both whatever the check state: read on
  2026-11-14 in `Helper::renderCheckBox` and `Helper::renderRadioButton`
  (`kstyle/breezehelper.cpp` at `f0b1d753`), each strokes a thin outline in
  the focus colour while the pointer is over the control, with no condition
  on the state, before the mark is drawn. So the one field holds for both
  controls and for the checked state, and its documentation says so.
- iced and egui take both fields; in gpui the connector's own check box and
  radio take theirs, and upstream's `Select` and `Combobox` have no hover
  style. The gpui showcase paints the combo box's hover fill itself under its
  `Select` (`showcase-gpui/demo.rs:4485`); it paints the hover border there
  instead where the theme states one.

The slider's handle is left out (§0): Breeze recolours its outline on hover, and
the model has no outline for the handle at rest either, whose colour was not
read. Its stand-in fill stays until then.

### D7 — Toolbar buttons and the table header

**Toolbar buttons.** Breeze draws a flat button without a frame at rest; on
hover a 1 px highlight outline over a transparent fill; pressed, the highlight
at alpha 0.3; checked, the button text colour at alpha 0.125, with an outline
mixed from the button and its text colour by the frame contrast, which no
field below carries and the backlog records. The model's
`ToolbarTheme` has no state fields. Other platforms fill instead (§1).

Four optional fields on `ToolbarTheme`: `button_hover_background`,
`button_hover_border_color`, `button_active_background`,
`button_checked_background`. Fewer would leave out either KDE's outline or the
other platforms' hover fill.

- **Unset** means that a connector draws that state of a flat button as it does
  today, nothing more. Today the three differ in the states no preset states
  (gpui darkens the button's fill for a pressed flat button where the theme
  gives no pressed fill, the iced showcase takes the hover fill; a selected
  button in egui is filled with the accent), and this decision does not align
  them: a difference that exists without the fields is not this step's. So
  kde-breeze states all four, the hover fill as transparent (a colour a preset
  can state, as kde-breeze does elsewhere), from the fact in §1, and the KDE
  reader writes the other three from the colour scheme, since they follow its
  highlight and button text (the transparent hover fill is the same under every
  scheme and stays the preset's); without it a flat button on KDE would keep the ordinary button's opaque hover fill, which
  Breeze does not draw. The other platform presets state those their facts
  give, by D6's rule: the parked facts give them for GNOME and Windows. On Windows the checked fill is the user's accent fill (`AccentFillColorDefault`), which windows-11 states for the default accent only, so the Windows reader writes it, as it writes a checked button's, and the system highlight under high contrast. On
  GNOME the toolbar look belongs to a toolbar's image, image-text and arrow
  buttons; a button with a text label alone keeps the ordinary button's look
  there (libadwaita 1.10.0 `_toolbars.scss`, the first rule, read
  2026-11-14), so the fills are one colour for everything that look covers,
  and an application applies the connector's toolbar-button style to the
  buttons the platform gives it to. A
  macOS toolbar item's looks are marked unmeasured there, Material has no
  toolbar button in them and iOS's values are pending, so those presets stay
  unset, with backlog entries.
- **The text of a checked toolbar button.** Windows fills a checked toolbar
  button with the accent and letters it in the on-accent colour; lettered as
  an unchecked one it would be hard to read (the specification gives the
  contrast figures). The options: leave windows-11's checked fill unset; a
  fifth field for the text; or letter a checked toolbar button in
  `button.checked_text_color`, which the model has. **The last**: a toolbar
  button is a button, the four fields are its fills and outline where they
  differ from the ordinary button's, and its checked text is the button's
  checked text on each of the three platforms read (the on-accent colour in
  windows-11, the button text in kde-breeze and adwaita). A connector uses it
  where the theme states the toolbar's checked fill, and letters the button
  as today where it does not.
- **A checked toolbar button is shown.** No showcase has one today, so the
  checked fill would be drawn nowhere and no screenshot could show it. Each
  showcase gets one, the same in all three: a fourth tool button in the Basic
  page's icon row, an edit-mode switch that starts checked. None of the
  toolbar's own three buttons switches something that stays on (two open
  modal dialogs, one is momentary), and a new button in the toolbar would
  change every page's screenshot and repeat the status bar's side-panel
  switch. Naming the field in the Widget Info alone would leave it unseen.
- egui: the «toolbar» role writes the hover and pressed fields into its states
  as they are stated, not laid over the idle fill as the ordinary button's
  hover is: a transparent hover fill must stay transparent. egui has no state
  for "checked": a selected button takes the scope's selection colour, which
  is also the colour of selected text in the same scope, so the role cannot
  carry the checked fill without recolouring a search field in the toolbar.
  The checked fill reaches a button per call, as the showcase fills its toggle
  button today, through a new accessor for the field.
- iced: the connector gains a style function for a toolbar button. The
  showcase's toolbar uses it; its own flat-button style stays for the other
  places it is used.
- gpui: a new variant function for a toolbar button, which reads the fields;
  `variants::ghost_button` stays as it is, since it is documented for every
  ghost button and reads only tokens. A custom variant has one fill for press
  and selection, so the checked fill is set by the caller (the connector's
  `geometry::button_checked` does that for ordinary buttons), and the hover
  outline depends on D5's trial.

**The table header.** Breeze fills a hovered section with `mix(Button,
hoverColor, 0.2)` and a pressed one with `mix(Button, focusColor, 0.2)`. Two
optional fields on `ListTheme`, `header_hover_background` and
`header_active_background`, stated by kde-breeze and its reader. No toolkit's
table gives a header these states (§1), so the fields reach an application
through the model and the connectors' accessors, and the three showcases' own
tables use them. This is the least of the eight and is done last.

### D8 — The default button: fields of its own

**Fact.** Breeze fills the default button of an active window with `mix(Button,
Highlight, 0.2)`, #d6ecf8 / #2d4655, outlined #87c0dd / #4882a1, under the
ordinary button text. The model's `button.primary_background` and
`primary_text_color` are the accent pair there.

**Options.**

- (a) Put Breeze's values into the primary pair.
- (b) Fields for the default button, apart from the primary one.

**(b).**

- (a) fails in gpui. gpui-component's `primary` token, which the connector fills
  from the primary pair (`CG/colors.rs:203-204`), is also the accent of thirteen
  other widget files. It would turn pale on KDE unless it took another source,
  and no existing field is that source on every platform:
  `defaults.accent_color` is the raw accent on Windows, where controls are
  filled with another shade (windows-11 states #0078d4 for the accent and
  #0067c0 / #4cc2ff for the primary button and the checked check box), and the
  check box's checked fill is itself a tint on KDE (#c2e4f6).
- (b) matches what the platforms have: a default button is a state, a primary or
  suggested button a variant (§1). On macOS and Windows the default button looks
  like the primary one; on KDE it has a look of its own and there is no primary
  variant.
- Three optional fields on `ButtonTheme`: `default_button_background`,
  `default_button_text_color`, `default_button_border_color` (the longer names,
  because "default background" would read as the background at rest).
  kde-breeze and the reader state them; the reader mixes the frame at Qt's
  own 16-bit precision, since rounding the inner mix to 8 bits gives #87c0de
  where Breeze draws #87c0dd (the specification's computation). The fill
  decides: where a theme states
  `default_button_background`, a connector's default button takes the three
  fields, the text colour inheriting the ordinary button's text colour where
  it is not stated, which is what Breeze draws, and the border the ordinary
  button's; where it states no fill, the default button is the primary button,
  whole, which is what macOS and Windows draw. So no rule between fields is
  needed, and a theme cannot end with the primary text on a pale fill.
- The facts row for KDE's primary button (`PF:1188-1189`), which v0.7.1 pointed
  at the default button, moves to the new fields' rows, and the comment in
  kde-breeze that says its primary pair disagrees with the facts goes: the pair
  then states what it means, the accent-filled button.
- Connectors: each offers the default button beside its primary one (iced a
  style function; egui an accessor for a per-call fill and stroke, since egui
  has neither concept; gpui a variant for a dialog's confirming button, which
  is a custom variant and so rests on D5's trial for its border). In gpui the
  variant reaches a button the application builds, the footer of a plain
  `Dialog` included; an `AlertDialog`'s own OK button is built by
  gpui-component from one of its variants and stays the primary button,
  which the README says. No existing reader changes.

### D9 — The selection of a view without the keyboard focus

**Fact.** In a view without the focus, Breeze draws a selected row in
`tint(Window, Selection, 0.4)`, #c2e0f5 / #1b4155, lettered in the window's
text colour. The model has the fill, `defaults.selection_inactive_background`,
and no text colour for it; macOS's presets state the fill, which an application
can only pair with the active selection's white text.

**Decision.** One field, `defaults.selection_inactive_text_color`, inheriting
the selection's text colour.

- kde-breeze and the KDE reader state the pair: both fields, the fill too,
  which neither states today (the reader writes no
  `selection_inactive_background`, and kde-breeze's resolves to the selection
  colour). Qt tints the selection only
  where the colour scheme's `[ColorEffects:Inactive] ChangeSelectionColor` is
  on (`PF:1061`), which Breeze's schemes set. Where that key is missing,
  KColorScheme takes the group's `Enable`, and where that is missing too, on
  (`P8/docs/facts/kde/tokens.toml`, `token.kde.inactive_change_selection_color`,
  quoting `kcolorscheme.cpp:304-316`). The reader follows the same order.
  Where the result is off, the unfocused pair is the focused one, and the
  reader writes the selection's fill and text colour into the two fields: its
  result is laid over kde-breeze, so a field it left unwritten would keep
  Breeze's tinted value under a scheme that turns the tint off.
- A scheme's other inactive effects (`[ColorEffects:Inactive]`'s contrast,
  intensity and colour, applied where its `Enable` is true; Breeze's schemes
  set it false) are not applied to the pair. They recolour every colour of an
  inactive window, which the model does not hold at all, so applying them to
  this pair alone would give a selection that no longer matches the window
  around it. The reader's documentation says that such a scheme gets the pair
  before those effects, and the backlog records the inactive window's palette
  as one item. The tint function
  exists on `v0.8.0-rc1` (`kcu_tint.rs`, with the shared `kde_color_common.rs`,
  306 lines, and its table); it is carried over with the clean-room notice.
  That branch's `kde_selection_inactive.rs`, 27 lines, holds this condition
  around the tint in types the 0.7.x model does not have; its rule is carried,
  rewritten for the reader, and checked against the same table.
- The macOS presets state the text colour that is drawn on the fill they
  already have. That fill is the content selection's
  (`unemphasizedSelectedContentBackgroundColor`, `PF:1061`), and the parked
  facts measured that a row's labels on it do not adapt: they are drawn as
  outside the selection, in the label colour, #000000d8 / #ffffffd8
  (`P8/docs/facts/macos/roles.toml:11958-11963`). `unemphasizedSelectedTextColor`,
  #000000 / #ffffff (`:953-990`, `:5459-5495`), is the text of an unfocused
  *text* selection, whose fill this field does not hold; it was this
  document's first choice and is not taken. The macOS reader, which writes
  the fill today (`native-theme/src/macos.rs:171-176`), writes the label
  colour it already reads beside it.
- No toolkit's list, tree or table knows that it lost the focus (§1), so
  nothing on screen changes by itself. The connectors offer the pair through
  their accessors. The showcases' own lists, which the showcases paint
  themselves from the list's own selection colours, swap to the pair while
  their window is not active, and only where the theme gives an inactive fill
  that differs from its selection's: kde-breeze after this step and the two
  macOS presets. Swapping on every theme would turn windows-11's faint list
  selection into the accent, which its inactive fill inherits; not swapping
  at all would leave the pair unseen in any widget. One sample cannot swap:
  the gpui showcase's list built on gpui-component's `ListItem`, which paints
  its selected fill after the caller's style; its Widget Info says so. The
  showcases follow the real state of their window: in tests the window is
  made active where a test needs the active look, and if a platform's CI
  capture shows the unfocused pair, the capture is made with an active
  window, not the showcase told otherwise.

The alternative, waiting until a toolkit has a receiver, leaves the model
stating half a pair. It is done after the items that change what a user sees.

### D10 — Order and releases

Each step is a release-sized change, in the project's rule of one small step at
a time. Sizes are estimates; adding one field has touched 13 to 28 files in
this repository before any connector drew it (commits `3ddfe91a`, `bd8f5756`).

| Step | Items | New model fields | Size |
|---|---|---|---|
| 1 | D2 the grid, D3 the menu panel | none | medium |
| 2 | D4 the focus ring | 1 | medium |
| 3 | D5 the button's outlines, with the gpui trial | 2 | medium |
| 4 | D6 the check box, the radio and the combo box | 2 | small |
| 5 | D7 toolbar buttons | 4 | medium |
| 6 | D8 the default button | 3 | medium |
| 7 | D7 the table header, D9 the unfocused selection | 3 | medium |

- The order is by what changes on screen without an application doing
  anything. Steps 2, 3 and 4 change what iced and egui applications show on
  KDE at once (and gpui's as far as §1 allows); step 2 also changes
  windows-11 and material-3, where the field is true. Step 5 does so in egui,
  and offers iced and gpui a new function; steps 6 and 7 reach an application
  only through new functions and accessors, and the showcases. Steps 2 to 5 and the table
  header in step 7 are the reporter's; the header is last among them because
  no toolkit's table can show it.
- No step depends on a later one; steps 5 and 6 use the result of step 3's
  trial for their gpui part.
- Every step from 2 on needs D0's version decision.
- D1 needs no release.
- Each step carries what belongs to it outside the code: its CHANGELOG entry;
  the roadmap's paragraph, which today says the menu panel and the grid wait
  for a connector design and that the hover outlines are the next step
  (`ROADMAP.md:105-110`); the first rationale's table of the reporter's items
  and its draft of the reply to the reporter, which say that B6 and B9 wait
  (`docs/issues/0004_kde-hand-overrides-rationale.md:1024`, `:1027`,
  `:1087-1095`) and must say what is then true before the reply is posted;
  and the backlog entries the step closes.
- Step 2's ways of telling a focus that followed a key are untried in all
  three connectors (§4). If one cannot be built as described, what that
  mechanism was for (the iced wrapper, the egui plugin, the gpui connector's
  own widgets) keeps drawing the ring on any focus, as the field left unset
  means, and its documentation says so; the step's other parts, the switch of
  gpui-component's band as `apply` sets it among them, do not depend on it.
  The plugin that turns that band on after a key was measured (D4), and a
  failure of its test is a stop, not a fallback.

**The releases after v0.7.1**, for the four documents of 2026-11-14 and the
backlog's dependency upgrade together. The order is the maintainer's; this is
the proposal:

1. egui 0.37 (`docs/todo.md`, "Dependencies: the newer releases …"): a
   dependency upgrade should not wait behind features, since every later
   release is built on it.
2. Steps 2 to 5 above, the reporter's remaining items: the maintainer asked
   for the reporter's list first, and the issue's reply waits on them.
3. gpui-component's own icons
   (`docs/archive/todo_v0.7.1_gpui-widget-icons-rationale.md`), then the step that
   serves the two pixel sets (its W7).
4. The caret's blink (`docs/archive/todo_v0.7.1_caret-blink-rationale.md`).
5. Steps 6 and 7, and the three outlines §0 leaves in the backlog.

The other order for 2 and 3, the icons first because they complete what GPUI
Kit 0.9 made possible, is as sound technically; the proposal follows what was
asked for first.

*2026-11-19:* the first half of item 3, gpui-component's own icons, was built
inside 0.7.2 by the maintainer's decision; the step that serves the two pixel
sets stays a later release.

**Where step 1 goes.**

- (a) Into v0.7.1. It corrects values in existing fields, as v0.7.1's first KDE
  fixes did, and both are items of the reporter's list, which the maintainer
  asked on 2026-11-13 to have fixed in v0.7.1. v0.7.1 is reopened in any case by the completion of
  the GPUI Kit port (`docs/archive/todo_v0.7.1_gpui-kit-0.9.1-rationale.md`), whose
  showcase changes need a new asset run already.
- (b) As v0.7.2. v0.7.1 stood finished and checked on 2026-11-14; step 1 brings
  a carried module of 434 lines with its test, reader, preset and facts
  changes, a token's source in gpui and two styles in iced, to a release that is
  otherwise the dependency upgrade and its fixes. Under (b), every number from 0.7.2 on in the batch's proposed version table (0.7.2 egui 0.37, 0.7.3 steps 2 and 3, 0.7.4 steps 4 and 5, 0.7.5 the widget icons, 0.7.6 the caret, 0.7.7 steps 6 and 7) moves one up.

**(a) is proposed**, for the first reason; the cost is the size of (b)'s list.
The maintainer may merge or reorder steps. On 2026-11-15 the maintainer merged steps 2 and 3, 4 and 5, and 6 and 7 into one release each (`docs/archive/todo_v0.7.1_batch-speed-rationale.md`, S5).

## 3. What the maintainer approves

1. D0: the version of a release that adds a model field.
2. D2: kde-breeze and the KDE reader give the table's grid Qt's colour; the
   shade function is carried over from `v0.8.0-rc1`. With it, outside KDE:
   gpui's chart grid takes the separator's colour, which changes it on adwaita
   and the two macOS presets. On KDE: a separator or a group frame inside an
   egui list scope becomes as faint as the grid, and the first rationale's
   condition that egui's list role be split first (§5.6) is dropped.
3. D3: kde-breeze and the KDE reader give the menu panel Breeze's colour; this
   reverses the first rationale's §5.5. With it:
   - in iced, on every preset: the connector's menu-bar style becomes the
     showcase's (the bar's fill changes on windows-11 and material-3; the
     popup gains the popover's frame and radius on every preset but
     material-3), and its drop-down style takes the popover's frame line and
     keeps the menu's fill (the line appears on every preset but material-3;
     the corners stay square; on KDE the drop-down takes the new menu colour,
     four to five levels from Breeze's combo popup). Taking the fill from the
     popover too was weighed and not taken: the rows' highlight and text are
     stated against the menu's fill, and adwaita's dark highlight would all
     but vanish;
   - in egui, on KDE: combo popups and other popups, and windows, modals and
     tooltips drawn without their surface frames, move by four to five levels;
   - in gpui: one contrast check gets an exception for kde-breeze's two
     modes, since gpui-component has no menu surface to take the colour.
4. D4: Breeze's ring values; the field `focus_ring_keyboard_only`, true for
   kde-breeze, windows-11 and material-3, with adwaita unset; the ring shown
   for a focus that followed a key, decided when the focus is gained; on those
   three presets gpui-component's band shown for a focus that followed a key
   and not after a click, through a plugin of gpui-base's `Root` that `apply`
   registers; gpui's `ring` without its alpha on those presets.
5. D5 to D9: the fields named, each optional. With D6: material-3 states the
   combo box's hover outline too, and adwaita's check box stays as it is until
   GNOME's checked box under the pointer is read. With D7: the KDE reader
   writes the toolbar button's outline and its pressed and checked fills, and
   the Windows reader its checked fill from the user's accent; a checked toolbar
   button is lettered in `button.checked_text_color` where the theme states
   its fill, and each showcase gets a checked toolbar button to show it.
6. D10: the steps, their order, and whether step 1 goes into v0.7.1; the order
   of the releases after v0.7.1; and the sending of the upstream requests
   (D1, D4, and one more if D5's trial fails).

## 4. What was checked, and what was not

Checked on 2026-11-14 by reading the lines named, in the toolkits' sources and
the connectors; the first review re-read most of them and found the readers of
`list.grid_color` and `menu.background_color` complete.

Not checked:

- D5's custom variant in gpui: read, not built. That a click does not focus a
  gpui-component button was read from the mouse handler, not run.
- The three connectors' ways of telling a focus that followed a key (D4) are
  designs over what the iced wrapper, the egui plugin and the gpui widgets'
  state can see; none was tried (D10 says what happens if one fails). Whether Breeze's
  band outlasts a mouse move was taken from the facts' wording ("after a
  keyboard focus change"), not from Breeze's source.
- KColorScheme's other inactive effects (D9) were read from the parked
  branch's `native-theme/src/platform/kde_effects.rs` when the specification
  was written: they apply only where `[ColorEffects:Inactive] Enable` is
  true, which Breeze's schemes set false. What they give under a scheme that
  enables them was not computed, since D9 does not apply them.
- Material's ring after a scroll (D4).
- Whether the measurement of the combo box's popup, taken on its view, is the
  whole popup's colour (D3).
- Whether Breeze rounds the highlighted row of a combo box's popup, or the
  popup's own corners (D3).
- The values other presets get for the new toolbar fields come from `F`'s
  expressions (alphas of a role colour), not from resolved colours; the
  specification resolves each and cites it.
- libadwaita's text field and a click (D4); Breeze's radio hover and the checked
  states on screen (D6; their code was read on 2026-11-14 in `renderCheckBox` and
  `renderRadioButton`, the stroke's geometry on 2026-11-15, not seen drawn); the
  slider handle's outline at rest; Qt Quick's menu fill (D3).
- Windows's and Material's behaviour for a text field reached by Tab was not
  read; D4 follows the facts' general statement.
- Whether the accent the KDE portal gives can differ from the colour scheme's
  focus colour.
- The compatibility step of the release check was not run on any field
  addition.

## 5. Out of scope

- The backlog entries named in §0.
- A model field for "the accent a control is filled with", which D8 shows is
  missing: recorded in the backlog by this work.
- Upstream changes; the requests are drafts until the maintainer sends them.
