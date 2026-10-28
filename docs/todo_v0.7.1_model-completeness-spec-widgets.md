# v0.7.1 — Model completeness: Specification, widget catalogue

**Status:** draft, 2026-10-28; merged from eight groups' catalogues, through its first check (entry format,
availability, flags, exclusive groups, one home per control) and its second (the core's units, per-part fields,
whole-part absence and scope; one rule for nested controls; container roots; iOS's elevated level as a binding) and its
third (one structure per control, *(element)* parts for anything selected by its own states or siblings, the host's
placement of a nested popup, one event per occasion, absence about what a platform draws only) and its fourth (parts
nested in their parent part, *(element)* parts for every part with states of its own, chosen segments `selected`,
one home per control for the date and time flyouts, the circular gauge, GNOME's boxed rows and the expandable row,
per-edge scroll edge effects, one presentation event per occasion). Part of
the specification ([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), "S…"; its
decisions are the rationale's,
[`todo_v0.7.1_model-completeness-rationale.md`](todo_v0.7.1_model-completeness-rationale.md),
"R…"). The traceability is in `todo_v0.7.1_model-completeness-spec-trace-<platform>.md`.

## What this is

Every **typed widget** (a kind two or more platforms draw, R14, S5.1) with its availability, parts, widget values,
variants, sizes, flags, exclusive groups and the source of each platform's state model; every **keyed component** (a
kind one platform draws); the **colour roles and text ramp** S3 and S4.4 leave open; and the **tokens, conditions,
readers and placement strategies** the rows need beyond S4, S7.1, S9 and S13. It states structure only: no platform
value appears here (values come from the facts, S17). Rows of the gap map are cited by ID.

Eight groups wrote it in parallel, each for a share of the map's rows: window chrome and overlays (group 1),
buttons and toggles (2), fields and indicators (3), containers (4), colour roles and text ramp (5), the controls the
model lacks that two or more platforms list (6), the controls only one platform lists (7), and what crosses widgets
(8). The 47 kirigami-addons rows (K-KA-n†) were traced during the merge. Each section says which group wrote it.
Where groups named one thing differently, the merge kept one name (the rename table below); where a group needed
something the core lacks, the addition is cited as `ADD-Ann` (a detail inside a decided mechanism), `ADD-Bnn` (a
mechanism the rationale did not decide) or `CONTRA-n` (a contradiction with the rationale), as the core's additions
register (S22) records them.

## Open items

Questions the value phase answers from the facts; until then the entries state the structure below, and a finding
that changes it is a change of this catalogue (the derive generates the `may_be_absent` tables from it, S6.3):

- `context_help`: whether Breeze's popup shadow applies to Qt's What's This window (K2.31†; listed absent on
  `kde-breeze`); macOS's mode cursor (MAC-P4-11†; listed absent on `macos-*`).
- `ink_toolbar`: whether each platform draws every part the rows name (W2-36, C-40†; listed as drawn).
- `level_indicator.value_label` on the `macos-*` and `ios-*` gauges (MAC-N22, C-30; listed absent there).
- `sheet.presenter_backdrop` on `ios-26`, where the card effect may be gone (C-12; listed drawn on `ios-*`).
- `component.context_menu_preview` and `bottom_tab_bar.accessory` on iOS 26 (listed as stated by the rows);
  `bottom_tab_bar`'s accessory environment variants (`UITabAccessory.Environment`).
- `text_area.highlight` on `macos-*` (macOS 15's highlight styles, MAC-X28; listed drawn).
- `avatar.fallback_icon` on `adwaita` (AdwAvatar's icon fallback, which GN-2.14 does not name; listed absent until the
  facts state it).
- `button`: iOS 26's further bar-item roles (none declared yet).

Core changes this catalogue needed are in the core (S22, ADD-A96): `Breakpoint`'s comparison and measured box,
`OverlayBottomCenter`, the two-edge fade, the touchpad's velocity samples, `Theme::system` and its S12 exemption,
`ElementData.color`'s wording, and kirigami-addons' avatar hash (compared in the value phase).

## Entry format

Each `### <widget>` section states, in this order:

- **Kind:** typed widget, and which platforms draw it, with the rows (counting the cross-check decisions that found it
  drawn inside an element, R14).
- **Available:** the presets that declare the widget. "Every preset", or "every preset but `x`, `y`": each preset
  named there declares `available = false` (S10), because its platform has no such control, and the query answers
  `NotAvailable` for it (S8). Where only one of `kde-breeze`'s flavours draws it, the entry says "`kde-breeze`
  flavour `<f>` only": the other flavour declares `[declare.flavor.<other>.widget.<w>] available = false` (S10).
  Preset identifiers: `adwaita`, `kde-breeze` (flavours `widgets`, `kirigami`), `macos-*` (`macos-tahoe` is the macOS
  26–27 generation), `windows-11`, `ios-*` (`ios-26` is the iOS 26–27 generation), `material-3` (R1.5).
- **Parts:** each part's name, what it is, and its fields, written `` `part` — what it is: `field`, `field` ``. A line
  (a separator, divider or rule) is a `stroke` on the edge it lies along (its width and paint), its insets
  `padding` or `margin`; a band with a height of its own (iOS's thick group separator) is a `fill`. The
  first part is the widget's **root** (S2.11): a container holding the widget's other parts, never absent as a whole
  and never a repeated element (S6.3; its fields may be `Absent`); its bindings apply to every part of the widget and
  its descendants (R5.2). Each part's field list is normative (S2.11): the part has exactly the `PartStyle` fields
  listed, the derive generates that set, and a patch setting another field is refused (S12). A part marked
  *(element)* has states, siblings and data of its own: it is queried as an element (S8) in the scope the widget's
  root returns (or an enclosing *(element)* part's), and the parts named as belonging to it (an item's label, icon or
  mark) are queried with that element; every other part is queried with the widget's element (S2.11). So a part that
  a rule selects by its own states, its own variants or flags, by its position among its own siblings, or uses as a
  context with states of its own is an *(element)*; a part of the widget's element is selected by the widget's states.
  An *(element)* part's query also sees its widget's variants (S2.11): a row matches the list's `sidebar`. A part
  whose style reads element data declares it on the part (`data = { <part> = [...] }`); a widget value that reads
  element data (a presentation step's duration over `move_distance` included) declares it under `values` (`data = {
  values = [...] }`) (S8, S10).
- **Nesting** (S2.11): parts nest as the entry nests them. A part nests in the root unless its bullet names its parent
  (`` `title` *(element)*, in `title_bar_content` ``); the parts belonging to an *(element)* part nest in it. A part's
  `bindings`, `opacity` and `transform` reach every part nested in it and their elements (the title bar's Header
  binding reaches the title and the caption buttons; the title-bar content's inactive dimming dims them as one
  group).
- **One-edge lines:** a `stroke` drawn on some edges only (a status bar's top line, a toolbar's hairline) states the
  other edges with width zero: a stated fact (the platform draws no line there), not an absence (S2.4 marks no `Edge`
  field but `dash` may-be-absent); their paint is the drawn edges' reference, so nothing is invented.
- **Widget values:** typed values beyond parts (gaps, alignment, order, behaviour, placement, breakpoints), each with
  its type. A presentation-transition list is `presentations: Vec<PresentationTransition>` (S2.7), keyed by event,
  one event per occasion: `Appear` and `Disappear` are the element appearing and disappearing, `Move` the element
  moving to a new place (a tab or row dropped at its new position, over `element.move_distance`). The element is the
  one `values()` is queried with (S8): queried with an item *(element)* (a row, a tab, a thumbnail), the events and
  their element data (`move_distance`) are that item's; queried with the widget's own element, they are the widget's
  (a tab strip appearing). A widget lists, per event, the element it presents; every other event
  is keyed (`@event`, declared, S10), and a part shown, hidden or moved on its own occasion is a keyed event naming
  the part (`@indicator_move`: an indicator sliding to the new selection; `@value_indicator_show`), whose steps name
  the part. One occasion has one event name on every widget. A value change animated on a part is that part's
  `transitions` (group `Geometry`), not a widget value. A value of type `Placement` is `{ strategy:
  PlacementStrategy, params: PlacementParams }` (S13). Every widget has `focus_visible_when:
  Value<FocusVisibleWhen>` (S2.11, ADD-A16), whose base each preset states, usually as
  `= token.behavior.focus_visible_when` (a reference, not inheritance, R20): an entry lists it only where a row states
  it. Likewise a widget's `drag_hover_delay_ms` (a drag resting on an item activating it) has its base `=
  token.behavior.spring_loading` (ADD-A45) where the platform states one spring-loading delay, and the widget's own
  value where its own source states another: one home per fact. Absence is `Value::Absent` (S2.2): a value that may be absent is a `Value<T>` listed under **may_be_absent**.
- **Units** (S1): the type carries the unit. A length is `Value<Px>`, a duration `Value<Ms>`, an angle `Value<Deg>`,
  an opacity or other fraction `Value<Fraction>`, a unitless number (a factor, a velocity) `Value<f32>`, a count
  `Value<u32>`. A widget value of one of these scalar types is written as its TOML key spells it, the unit's suffix
  added from the type (`icon_text_gap_px: Value<Px>`, `show_delay_ms: Value<Ms>`, `start_angle_deg: Value<Deg>`;
  the Rust field is `icon_text_gap`, `show_delay`, `start_angle`), and a reference may leave the suffix out
  (`values.icon_text_gap` is the same value). A value of a compound type (`Sides<Value<Px>>`, `Axes<Value<Px>>`, a
  list, a record) carries no suffix: its members carry their units. A `PartStyle` field is named in paths as S2.11
  names it (`body.min_width`, `part.min_width`); its TOML key adds the suffix the same way (`min_width_px`).
- **Variants:** a variant is typed when two or more platforms share its meaning anywhere in the catalogue (R15; the
  typed variants are one `Variant` enum, and a widget declares the ones it has), keyed (`@name`) otherwise. A typed
  variant's meaning is the declaring widget's (`inline` is a link inside running text on `link`, a tip inside the
  content on `teaching_tip`), and a keyed `@k` and a typed `k` are different names. The content variants (`icon_only`,
  `with_icon`, `with_label`, `with_subtitle`, `custom_color`, and the ones entries add: `three_line`, `with_avatar`,
  `with_image`, `with_video` on `list`, `with_search`, `with_prompt` on `toolbar`, `bitmap_icon` on `form`) are set
  by the connector from the element's content on every platform (ADD-A61), so they are typed whichever presets style
  them. A variant the connector sets from a size
  threshold is set by a `Breakpoint` value (Shared value types), so it may be keyed: the connector reads the variant
  from the value.
- **Sizes and densities** per platform (`ControlSize`, S7.3; Material Expressive's xsmall is `extra_small`); sizes a
  variant alone takes are declared by `sizes_by_variant` (S10: Material's FAB, MAT-N1), and a size a variant does not
  declare answers `NotAvailable`.
- **Flags:** the element's widget-specific flags: shared (`SharedFlag`, below), keyed (`@name`,
  `StateFlag::Widget(Key)`, declared by the preset); and the state flags it uses (S7.2).
- **Exclusive groups:** per platform, the flags of each group (the platform's state groups mapped to flags), written
  in the order of precedence the preset's `exclusive` array states (S10); that order is a fact of the platform's
  state model, which the gate checks (S17), so every entry ends "precedence: state models (S17)". A widget's groups
  apply to each element of the widget, its *(element)* parts included (S8 drops the excluded flags per element), so
  "on `item`" names the element whose states the platform's model groups.
- **may_be_absent** (S6.3): `parts:` whole parts some preset does not draw, with the presets: the preset's base, or a
  rule under a condition, states the part `absent = true` (`PartPatch::Absent`), and the query answers `NotDrawn` for
  it (an absent part needs no fields); a root part is never listed. `fields:` part fields that may be `Absent`;
  `values:` widget values that may be `Absent`; each with the presets. The list is about what a platform draws,
  never about an element's content: a part the element's content lacks (a dialog given no icon, an icon-only tab's
  label) is not queried by the connector and is not listed (S6.3); a part a preset draws whenever the content exists
  is required on it. A list value (a `Vec`: `presentations`, `breakpoints`, a map of keyed entries) is never `Absent`:
  where the platform has none it is the empty list, stated with its citation. Where an entry cannot yet name the
  presets that draw no such field, it says the value phase names them from the facts; the permission is then the
  preset's only where its facts show the platform draws none.
- **State-model source** per platform: where the gate's combination list comes from (R4, S17).
- **Today's fields:** how each field of `docs/property-registry.toml` maps into the new parts.
- **Notes:** where the widget is drawn in another's context, the nested controls it holds (below), and what the merge
  took from other groups (every name a note uses is in the lists above).

Names: identifiers are American (`color`, `center`, `minimize`), prose British (S1). Context selectors match the
ancestor chain the query's `Scope` carries (S8): an element's ancestors are the elements and parts its connector
queried above it, top-down.

## Nested controls

One rule places a control drawn inside another widget, the same on every platform (principle 2; the check pass's
decision):

- When the inner control's kind is a typed widget and it has interaction states of its own on some platform, it is an
  element of that typed widget, queried in its host's context (S7's `context`): a dialog's responses are `button`
  elements in `context dialog.action_area`, a combo box's popup is a `menu` in `context combo_box`, a spin box's text
  is an `input` in `context stepper`. Its kind is what it is to the user, as the cross-check names it (R14), whatever
  class a toolkit builds it from: a control that also stands alone (a push, tool, close, navigation, overflow, clear or
  reveal button, a menu, a text field, a check box, a list), drawn in the host with that control's look and restyled
  by context rules. The display and container widgets the catalogue types (`badge`, `spinner`, `progress_bar`,
  `avatar`, `chip`, `popover`, `toolbar`) are placed the same way, though some hold no interaction state.
- Otherwise it is a part of the host, an *(element)* where it has states: a piece that exists only within its host
  (a slider's thumb, a scroll bar's arrows, a stepper's increment and decrement, a segment, a tab, a menu item, a list
  row, a breadcrumb's crumbs and separators, a page indicator's dots, a split button's halves), a mark that takes its
  host's states and has none of its own (a menu item's check or radio mark, `menu.check_indicator`, on every preset:
  Breeze draws it with its check box's look, its fields stated there), and a surface that holds the host's own parts
  (a date picker's popup with its header and headline).
- The element's identity in its host is a variant of its widget, typed where two or more platforms share it (R15):
  `button` `close`, `previous`, `next`, `overflow`, `add`, `remove`, `done`, `clear_field`, `reveal`, `pane_toggle`
  and the media identities `play`, `volume`, `full_screen`, `frame_step`, `route`. Its look there is that widget's
  rules in
  the host's context, so no fact is stated twice (R20); the host's values (when the element shows, where it goes)
  stay the host's. So a popup a widget opens as part of itself is placed by its host's value, the same for every
  host: a combo box's list by `combo_box.values.popup_placement`, a menu button's (and an overflow or split button's)
  menu by `button.values.menu_placement` (`split_button.values.menu_placement`), a field's suggestion list by
  `autocomplete.values.placement`, a submenu by `menu.values.submenu_placement`, the window's menus by the window's
  values; `menu.values.placement` and `popover.values.placement` place a menu or popover the application opens at an
  anchor or the pointer (a context menu), which has no host. One kind of thing shown by several hosts is one
  structure: a dot that marks a needs-attention item is the `badge` widget (variant `attention`) in the host's
  context, never a part of the host.
- The core fixes one exception: the window's caption buttons are `window.caption_button` (S5.2).

## Shared value types

Value types several entries use, defined once (an entry defines the types only it uses):

| Type | Variants or fields |
|---|---|
| `Alignment` | `Start`, `Center`, `End` (mirrored in RTL) |
| `Side` | `Start`, `End` |
| `ShowWhen` | `Never`, `Always`, `WhileEditing`, `UnlessEditing`, `WhenNotEmpty`, `WhenFocusedAndNotEmpty` |
| `ValueLabelPosition` | `Above`, `Below`, `Inline`, `Leading`, `Trailing` (a value label against its bar or slider) |
| `ButtonLabelPosition` | `IconOnly`, `TextOnly`, `TextBesideIcon`, `TextUnderIcon` (Qt's tool-button style, WinUI's `LabelPosition`, AppKit's toolbar display mode) |
| `Placement` | `{ strategy: PlacementStrategy, params: PlacementParams }` (S13) |
| `EdgeSide` | the core's (S2.7): `Top`, `Bottom`, `Start`, `End`, `TowardAnchor` (a presentation's slide origin) |
| `BoxEdge` | `Top`, `Bottom`, `Start`, `End` (an edge of an element's own box, where no anchor exists: a scroll bar's, an indicator's, an edge effect's; `Start` and `End` mirrored in RTL) |
| `NavButtonsVisibility` | `Always`, `OnHover` (when a widget's previous and next `button` elements show: Windows' FlipView and PipsPager, W2-31, W2-20) |
| `Breakpoint` | the core's (S7.3), `Breakpoint { min_width: Value<Px>, min_height: Value<Px>, variant: VariantRef }`: a widget's own breakpoint (R16); the connector sets the variant on the element while it holds; a widget's `breakpoints: Vec<Breakpoint>` lists them, several may hold together. One mechanism for every widget, so the connector's code is the same whichever variant a preset names. The entries state below-threshold and window-measured breakpoints in the platform's terms until the core carries them (Open items) |
| `TroughClick` | `Page`, `JumpToPosition` (what a click in a slider's or a scroll bar's trough does, GN-P4-5†, R17) |
| `WidgetValue` | the core's (S1); a keyed component's value is a `Value<WidgetValue>` of the type its declaration names (`values = { <name> = "<type>" }`, S10); a component's enumerated value is a `Value<Key>` whose keys its entry lists |

## Shared flags

`SharedFlag` (S7.2) is exactly the widget flags two or more platforms draw on a typed widget; every other widget flag
is keyed. The list, with the widgets that declare each:

| Flag | Meaning | Declared by (platforms) |
|---|---|---|
| `empty` | the element holds no text or value | `input` (Material, Windows, KDE; a search field's text, an `input` in `context search_field`), `text_area`, `combo_box` (Material: no value chosen, MAT-CB1), `progress_bar` (GNOME: value zero), `date_picker` (Windows: no date chosen), `form` (GNOME's entry row) |
| `in_fill` | a tick or stop indicator inside the value fill | `slider` (KDE, Material) |
| `today` | the day is today | `calendar` (all six) |
| `other_month` | a day outside the shown month | `calendar` (GNOME, Windows, KDE, Material, macOS) |
| `current` | the current item of a sequence | `breadcrumb` (KDE, Windows, macOS, GNOME: GN-2.52†), `wizard` (GNOME's highlighted step, GN-2.51), `list` (kirigami-addons' current row) |
| `multiline` | the element's text wraps to more than one line | `badge` (KDE), `banner` (KDE, Material), `toast` (Material) |
| `filled`, `half` | a rating star filled, or half filled; a level indicator's block the value reaches (`filled`) | `rating` (KDE, Windows, macOS), `level_indicator` (`filled` on `block`: GNOME, macOS) |
| `preview` | the hover preview of a value | `rating` (KDE, Windows) |
| `pulling`, `refreshing` | the pull-to-refresh phases | `pull_to_refresh` (iOS, KDE, macOS 27 on `macos-tahoe`, Material, Windows) |
| `placed_above`, `placed_below` | a popup attached above or below its anchor (set by the connector from the placement result) | `menu` (KDE; Windows' suggestion list, a `menu` in `context autocomplete`) |
| `route_active` | an output route (AirPlay) is active | `button` on its typed `route` (macOS, iOS) |
| `editing` | a row or cell in edit mode | `list` (GNOME, iOS) |
| `swiped` | a row with its swipe actions revealed | `list` (iOS, KDE) |
| `context_menu_open` | the element's context menu or popup is open | `list` (GNOME's `.has-open-popup`, macOS) |
| `sorted`, `descending` | a sorted column header, and its sort direction | `list` (`header_section`: KDE, macOS, GNOME) |
| `overwrite` | overwrite (block-caret) mode | `input`, `text_area`, `code_editor` (GNOME, KDE) |
| `composing` | an input-method composition in progress | `input`, `text_area` (KDE, GNOME) |
| `minimized` | the element in its minimised form | `bottom_tab_bar` (iOS), `sheet` (Material) |
| `placeholder` | the element shows a placeholder in place of its value | `rating` (Windows, macOS), `list` (Windows: a not-yet-loaded row) |
| `text_selected` | the element's text has a non-empty selection (the caret parts' absence rules: GNOME, Material, iOS, macOS, Windows hide the caret; KDE keeps it, K3.43) | `input`, `text_area`, `code_editor` |

## Selector notation

The traceability (and "Today's fields") write a rule as `<target> @ <selector>`, the selector's parts separated by
spaces:

| Spelling | Meaning (S7) |
|---|---|
| `{a,b}` | the element's states: all of them hold together |
| `!{a}` | none of these states |
| `variant v`, `variant @k`, `variants {v,w}`, `!variant v`, `w.part[v]` | the element's (or the named ancestor's) variants; `@k` is keyed |
| `w.part[size s]` | the named ancestor's control size (`ContextMatch.sizes`, ADD-A84) |
| `cond c`, `!cond c` | a condition (S7.1, snake_case): `window_inactive`, `contrast_high`, `scheme:dark`, `input:touch`, `idiom:pad`, `flavor:kirigami`, `preset:ios-26`, `system_setting:<key>` (ADD-A04), `container_size(sidebar, small)`, `tiled:top`, `freeform`, `posture:tabletop`, `device_model:<key>` (ADD-A83) |
| `context w.part [variant v] [{a}]`, `context any w`, `context nearest {A, B} = A` | `ContextMatch`: an ancestor in the chain the `Scope` carries (S8); `AnyAncestor` or `NearestAmong`. The `variant` and `{…}` written after `context w.part` are that ancestor's; the element's own variants and states are written before the first `context` (`button.body.fill @ variant tool context toolbar.bar`) |
| `not_context w.part` | `Selector.not_context` |
| `pos first`, `pos last`, `pos only`, `pos parity odd`, `prev w{…}`, `next w{…}` | `PositionMatch` (parity: ADD-B06; by part: ADD-B07). A `prev` or `next` term that names a part other than the element's own counts among every sibling of that widget (`among {w}`): `prev list.row {hover}` on a `list.row_separator` is the row before it |
| `… among any`, `… among {w, v}` after a `pos` term | `PositionMatch.among`: siblings of any kind (`SiblingScope::Any`, CONTRA-3) or of the kinds named; without it, siblings of the same kind |
| `size s`, `density d`, `h_size_class c`, `v_size_class c` | `SizeMatch` (sizes `extra_small`, `mini`, `small`, `regular`, `large`, `extra_large`; per axis: ADD-A20) |
| `dir rtl` | `Direction` |
| ` / ` between alternatives | one rule per alternative; the selector parts written after the last alternative belong to every alternative, and a bare name after ` / ` continues the term before it (`variant a / b` is `variant a` / `variant b`) |

## Rename table

Each row is one thing two or more groups named differently, a name that collided with another thing, or a name the
check pass changed; the traceability uses the new names, and no old name is used after this table. Decided by the
rationale's rules: R14 (typed if two or more platforms draw it, counting the cross-check decisions; keyed otherwise),
R15 (typed variants where two or more platforms share the meaning), principle 2 (one mechanism, one name per thing),
and the owning group's name where the kinds are the same.

| # | Old name (group) | New name | Reason |
|---:|---|---|---|
| 1 | `dialog.platter` (7) | `dialog.body` | the action sheet's box is the dialog box group 1 owns |
| 2 | `dialog.values.screen_edge_margin` (7) | `dialog.values.window_margin` | group 1's value for the same margin |
| 3 | `dialog` variant `action_sheet` (7) | `@action_sheet` | iOS alone: keyed |
| 4 | `dialog` variant `full_screen`, parts `header_title`, `action_bar`, `divider` (6) | variant `full_size`; `title`, `action_area`, `header.stroke` | group 1's names: ContentDialog's full size, Material's full-screen dialog |
| 5 | `dialog.values.button_gap` (6) | `dialog.values.action_layout.row_gap` | group 1's (today's `button_gap`) |
| 6 | `window.values.caption_button_side` (8) | `window.values.caption_button_layout` | group 1's value holds side and order |
| 7 | `window.tab_bar.values.presentation` (7) | `window.values.presentations`, event `Move` | presentations are widget values |
| 8 | `window` variant `@utility` (6) | typed `utility` | group 1 typed it (Windows' tool window, macOS's utility panel) |
| 9 | `navigation_bar.back_button` (7) | `button` variant `back` in `context toolbar.bar`; values `back_title_fallback`, `back_display_mode`; toolbar event `@back_label` | group 1 typed the back button; group 6 places iOS's navigation bar inside `toolbar` |
| 10 | `navigation_bar` for Material's floating toolbar (7) | `toolbar` variant `floating` (typed: Material and KDE draw it), parts `leading_content`, `trailing_content` | group 6: app bars are `toolbar` |
| 11 | `scroll_edge_effect` (7), `scroll_area` (8) | `scroll_view.edge_effect`; variants `soft_edge`, `hard_edge`; values `edge_effect_height`, `edge_effect_edges` | group 3 typed `scroll_view` with this part (iOS, macOS, GNOME's undershoot) |
| 12 | `writing_tools.glow`, `writing_tools.affordance` (3) | `text_area.writing_tools_glow` (group 8's name), `input.writing_tools_affordance` | typed parts of the text widgets (macOS and iOS draw them in the app's text view); C-33's panel stays out (R1.1), its glow is carried |
| 13 | `drag_preview` (8), `list.values.drag_image` (4) | `drag_image` (7); `drag_preview.body` → `drag_image.image`; the list value → `drag_image.image.opacity`, `drag_image.values.hotspot` by rules in `context list` | one widget for the dragged image |
| 14 | `placeholder` widget (8) | `redaction`, part `mask` | `placeholder` is the text widgets' part |
| 15 | `list.rubberband` (4) | `rubber_band.body` in `context list` | group 6 typed `rubber_band` |
| 16 | `navigation_view.transition_shadow`, `navigation_view.dimming`, `sheet.dimming`, `sidebar.dimming`, `sidebar.transition_shadow` (6); `sheet.grabber`, `drawer.handle` (8) | `edge_shadow`, `scrim`, `sheet.handle`, `sidebar.handle` | one name per part kind: `scrim` as `dialog.scrim` and `role.global.scrim`, `edge_shadow` and `handle` as group 4's sidebar |
| 17 | `sidebar.body`, `sidebar.divider`, `sidebar.item`, `sidebar.item_icon`, `sidebar.values.icon_gap` (6) | `sidebar.pane`, `sidebar.separator`, `list.row`, `list.leading_icon` (variant `sidebar`), `list.values.leading_gap_px` | group 4: a sidebar's rows are `list` rows |
| 18 | `sidebar` variant `rail`, parts `container`, `active_indicator`, `icon`, `label`, `menu_button`, `badge` (7) | typed `compact`; `pane`, `list.row_selection_indicator`, `list.leading_icon`, `list.row` text, `button @ variant pane_toggle context sidebar.header` (row 104), `badge` in context | group 4's collapsed rail |
| 19 | `stack` (8) | `navigation_view` | the page stack (GtkStack) |
| 20 | `spin_button` (8) | `stepper` | group 6 |
| 21 | `main_window.dock`, `main_window.values` (8) | `component.dock_widget` | KDE alone: group 7's component |
| 22 | widget `frame` (6), `group_box` (4) | `group_box` | one widget; `frame` is a part of `window`, `scroll_view`, `calendar` |
| 23 | context `button_group` (2) | `layout` keyed variant `@linked`, `values.item_gap` | GTK's `.linked` box; no button-group widget |
| 24 | `checkbox.group_header`, `checkbox.values.row_spacing`, `column_spacing` (7) | `radio_group.header`, `radio_group.values.row_gap_px`, `column_gap` | group 2 typed `radio_group` (Windows, macOS) |
| 25 | `button` typed `command_link`, parts `title`, `description` (7) | keyed KDE `@command_link`; `label`, `subtitle` | Windows' command link is Win32 (R1.3), so one platform draws it |
| 26 | `button` variants `paste_control`, `repeat` (7) | `@paste_control` (iOS), `@repeat` (Windows) | one platform each: keyed |
| 27 | `button` keyed `@help` (2) | typed `help` | macOS and KDE draw it (group 6); the window's caption-button identity `help` is keyed `@help` (KDE alone, row 85) |
| 28 | `button` keyed `@disclosure`, `@push_disclosure` (2) | `expander` (its `arrow`; variant `@push_disclosure`) | group 4's expander carries the disclosure controls |
| 29 | `button` variants `icon`, `filled`, `tonal`, `outlined` (6) | `icon_only`, `primary`, `tinted`, `@outlined` | group 2's names for the same looks |
| 30 | variant `suggested` (4, 6) | `primary` | group 2's name for the suggested look (form rows, split buttons, dialog actions too) |
| 31 | `input` variant `password` (6) | `secure` | group 3's |
| 32 | `tab` variant `navigation`, parts `item`, `item_icon`, `item_label`, `indicator` (6) | typed `bottom_tab_bar` | KDE, Material, iOS, macOS draw it; parts and state model are not the tab's |
| 33 | `bottom_tab_bar.selection_pill` (4) | `bottom_tab_bar.indicator` | as `tab.indicator`, `segmented_control.indicator` |
| 34 | `tab.body` (8) | `tab.tab` | group 4's part |
| 35 | `list.item`, `list.item.fill` (7, 8) | `list.row`; state fills on `list.row_background` | group 4's parts |
| 36 | `list.body`, `selection_indicator`, `item_selection_visual`, `row_expander`, `branch_arrow`, `multi_select_check_box`, `grid_item`, `grid_item_check_box`, `item_inner_border` (7, 8) | `view`, `row_selection_indicator`, `disclosure`, `checkbox @ context list.row` (row 107), `row_background` under the typed `grid`, the same `checkbox` under `grid`, `row_inner_border` | group 4's parts and variant |
| 37 | `list` variants `listbox`, `@alternating` (7, 8) | `@listbox` (Windows), `alternating_rows` | keyed; group 4's typed variant |
| 38 | `spinner.container`, variants `loading_indicator`, `contained` (7); `spinner.glyph` (8) | `spinner.body`; `@loading_indicator`, `@contained`; `spinner.icon.glyph` | group 3's parts; Material alone: keyed |
| 39 | `progress_bar` variant `circular` (6) | `spinner` variant `determinate` | group 3 put the determinate circular indicator in `spinner` |
| 40 | `expander.glyph` (8) | `expander.arrow` | group 4's |
| 41 | `separator.body` (8) | `separator.line` | group 4's |
| 42 | `switch.on_off_label` (8) | `switch.on_glyph`, `switch.off_glyph` | group 2's |
| 43 | `scrollbar` variant `annotated`, part `scroll_button` (7) | `@annotated`; `arrow_button` | keyed (Windows); group 3's part |
| 44 | `input.loupe`, `inline_prediction`, `spelling_mark` (8) | `magnifier`, `prediction`, `misspelling` | group 3's |
| 45 | `text_area.find_indicator` (8) | `text_area.find_match @ {selected}` | group 3: the current match is `{selected}` |
| 46 | `input.header_inactive` (1), `input.kde_header_inactive` (4), `input.inactive_roles` (8) | `input.inactive.<set>.<role>` | KDE's stated Inactive values, one S9 input per role |
| 47 | `popover.values.anchor_gap` (3) | `popover.values.placement` (its `gap`) | group 1's placement value |
| 48 | `toast.values.duration_ms` (8) | `toast.values.durations` | group 6's keyed list |
| 49 | `combo_box.values.placement` (1) | `combo_box.values.popup_placement` | group 3's |
| 50 | `component.side_tab_bar` (4) | `component.multi_tab_bar` | group 7's (KMultiTabBar) |
| 51 | `toolbar_customization` (4), keyed `component.toolbar_editor` (6) | typed `toolbar_editor` | KDE, macOS and iOS draw it |
| 52 | `component.token_field`, its `token`, `token_menu_arrow`, `values.token_spacing` (7); the second check's `component.token_field.values.chip_gap_px` | `chip` elements in a `layout @ variant flow` in `context any input` (`chip @ context any input`, `chip.menu_arrow`, `layout.values.item_gap_px`) | a token is a chip (group 7's own note); the third check: one structure for the token field on macOS and Windows (W2-43†'s) |
| 53 | the glass container's merge spacing as `Material.params` (8) | `glass_container.values.merge_gap_px` | group 7's widget value |
| 54 | `window.content.fill` with the material `background_extension` (8, MAC-X06) | `background_extension.extension` | group 7's widget |
| 55 | `token.lengths` (4), `token.kirigami` (2), `token.opacities` (5), `token.colour_effects.frame_contrast` (2) | `token.params.layout`, `token.params.opacity`, `token.params.color.frame_contrast` | one keyed-scalar token mechanism (ADD-A44); identifiers are American (S1) |
| 56 | value `focus_visible`, `token.behavior.focus_visible`; types `FocusVisiblePolicy`, `FocusVisible`; variants `KeyboardOnly`, `AfterKeyboard`, `WithFullKeyboardAccess`, `Always` (2, 3, 8) | `focus_visible_when`, `token.behavior.focus_visible_when`; `FocusVisibleWhen { Keyboard { timeout_ms }, AnyFocus, FocusWithin, Never }`; Full Keyboard Access is a rule `@ cond full_keyboard_access` | the bare name is the state flag (ADD-A16); every widget has the value (S2.11), so an entry lists it only where a row states it |
| 57 | `role.keyed.<x>` (8) | `role.@<x>` | S2.2's grammar |
| 58 | `role.@palette_dark`, `role.@palette_shadow` (7), `role.<set>.shade.light` (8) | `role.@shade_dark`, `role.@shade_shadow`, `role.@shade_light` | group 5's KDE keyed shades (Qt's 3-D roles, from Window) |
| 59 | `active_background`, `link_background`, `visited_background`, `negative_background`, `neutral_background`, `positive_background` (8) | `background_active`, … | group 5's record fields |
| 60 | `role.global.selection_text_inactive`, `role.global.selection_text` (5, 8) | `role.global.selection_text_background` (its inactive look a rule `@ cond window_inactive`), `role.global.selection_text_foreground` | the inactive group is a rule (S3.2, ADD-A48); S3's names |
| 61 | `role.selection_background_inactive`, `role.find_highlight`, `role.drop_highlight` (3, 7, 8) | `role.global.selection_text_background_unfocused`, `role.global.find_highlight`, `role.global.drop_highlight` | global roles (S3, ADD-A49) |
| 62 | condition `BehindModal` (8) | `TintDimmed` | group 5's, UIKit's own name (ADD-A10) |
| 63 | `IconStyle.symbol_variant` (4), `IconStyle.variant` (8) | `IconStyle.symbol_variants` | the core's name (S2.9, R25.8) |
| 64 | `values.presentation` (1, 4, 7, 8) | `values.presentations` | the list of presentation transitions |
| 65 | `icon_gap` (2, 6), `icon_label_gap` (4), `item_icon_gap` (4) | `icon_text_gap` | today's name, used by groups 1 and 3 |
| 66 | `auto_repeat_delay`, `auto_repeat_interval` (2), `autorepeat_*` (6) | `repeat_delay`, `repeat_interval` | groups 3, 7, 8 |
| 67 | `spring_load_delay_ms` (7), `drag_expand_delay_ms` (8), `drag_hover_switch_ms` (6), `drag_hover_expand_delay_ms`, `drag_hover_switch_delay_ms`, `drag_hover_activate_delay_ms` (4) | `drag_hover_delay_ms` | one name: what the hover does follows from the widget |
| 68 | widget values `*_spacing`, `spacing` (2, 4, 6, 7, 8) | `*_gap`, `gap` | R16's word; `letter_spacing` (a text-style field), the variant `@collapse_spacing` and Kirigami's spacing token keys keep theirs |
| 69 | conditions written as states, `{reduced_motion}` (8) | `cond reduced_motion` | notation: a condition is not a state flag |
| 70 | `toolbar.item`, `item_icon`, `item_label`, `item_menu_indicator`, `item_check` (4) | `button` (and `split_button`, `search_field`, `badge`) in `context toolbar.bar`; Qt's QToolButton, WinUI's AppBarButton and AppKit's toolbar bezel the typed `button` variant `tool`; the overflow check glyph `menu.check_indicator` in `context button variant overflow` | a toolbar's items are the elements they are (one home, the check pass) |
| 71 | `toolbar.values.label_position`, KDE `button` `@text_under` (2, 4) | `button.values.label_position: Value<ButtonLabelPosition>` | one value for the tool-button style |
| 72 | `toolbar` variant `prominent` on `item` (4) | `button` variant `primary` in `context toolbar.bar` | the item is a button |
| 73 | `toolbar` variant `raised` (4) | `@raised` | GNOME alone (R15) |
| 74 | `status_bar` for GtkActionBar (4) | `toolbar` variant `bottom` | one widget for GNOME's bottom bar (GN-1.157, GN-2.20) |
| 75 | `wrap_box` (6) | `layout` variant `flow`; Material's chip set `layout` `@chip_set` | one flow layout (GN-2.46, GN-P3-24†, MAT-RL01†) |
| 76 | `list.undershoot`, `toolbar.undershoot` (4) | `scroll_view.edge_effect` in context | one scroll-edge shade (GN-1.156, GN-1.178, GN-3.06) |
| 77 | `text_area.link`, `rich_text.link` (3, 6) | `link` variant `inline` in `context text_area`, `context rich_text` | one link in running text |
| 78 | `list.accessory` `@multiselect` (4) | `checkbox` variant `selection` in `context list.row` | one multiselect circle (CK-1, L-12); every list's check is a `checkbox` (row 107) |
| 79 | `card` `activatable` for the Toolkit's SettingsCard (4) | `form.card` | one home for SettingsCard (W1-139, W2-40†) |
| 80 | `sidebar` `@top`, `sidebar.values.display_mode` (4) | `view_switcher` `@navigation_top`; the sidebar's breakpoints and variants | Windows' top NavigationView is a view switcher (W1-094, W2-11) |
| 81 | `sidebar` typed `inspector`, values `open_width`, `compact_width` (6) | variant `end`, values `width`, `collapsed_width` | one trailing column (MAC-N19, C-41†) |
| 82 | `component.route_picker` (7), `media_controls.route_button`, `media_controls.cast_button` (6, 7); the check pass's typed widget `route_button`, its `glyph` part and `bordered` value | `button` variant `route` (typed: three platforms draw the output-device button, MAC-P3-9†, C-42†, W-P4-18†), its `icon` part, the keyed `macos-*` `@bordered`; the shared flag `route_active` on it | the third check: its parts are a button's, and in `media_controls` it sits beside the `play`, `volume` and `full_screen` buttons; a look that changes with `bordered` is a variant |
| 83 | `component.context_help` (7), KDE's What's This in `teaching_tip` (6) | typed `context_help` | two platforms draw a help pop-up for help mode (K2.31†, MAC-P4-11†) |
| 84 | `colour_button`, `colour_picker`, `series_colours`, `@four_colour`, `light_colour`, `off_centre_fade`, `dial_centre`, `centre`, `minimised`, `minimise`, `maximise` | `color_button`, `color_picker`, `series_colors`, `@four_color`, `@light_color`, `off_center_fade`, `dial_center`, `center`, `minimized`, `minimize`, `maximize` | identifiers are American (S1) |
| 85 | window caption identity `help` (1) | `@help` | KDE alone: Windows' help button is Win32 (W1-148†, R1.3) |
| 86 | `button` and `checkbox` `@neutral`; dialog `information`, `critical`; dialog `@success`; form `@warning`, `@error`, `@success`; input `@success` | typed `warning` (GNOME's `.warning`, KDE's neutral highlight), `info`, `error`, `success` | one name per meaning (R15) |
| 87 | `card`, `chip` typed `filled`, `elevated`, `outlined` (4, 6) | `@filled`, `@elevated`, `@outlined` | Material alone, keyed on every widget (R15) |
| 88 | slider `@hue`, `@alpha`, `@value` (6) | typed `hue`, `alpha`, `value` | GNOME and Windows draw them (GN-2.53†, W2-05) |
| 89 | status_page `compact`, form `monospace` (6) | `@compact`, `@monospace` | GNOME alone (R15) |
| 90 | tab `@icon_and_label` (4) | the content variant `with_icon` | the connector sets it from the content (ADD-A61) |
| 91 | badge `@tab_bar`, `@bar_button`, `@list_row` (6) | `badge` in `context bottom_tab_bar.item`, `context toolbar.bar`, `context list.row` | a context, not a variant (TB-7, C-15) |
| 92 | `role.@*_elevated` (5) | kept as keyed roles (`role.@system_background_elevated`, …), bound by the roots of `dialog`, `sheet` and `popover` on `ios-*` (the `Dialog` and `Popover` sets and `Binding::Role`); `cond elevated` only for a whole elevated window | R2.3, R5.2: in a sheet or a popover the elevated level is a binding (the second check reverted the first's removal) |
| 93 | flags `has_text`, `has_value`, `opened_above`, `overlapping`, `@current`, `@editing`, `@swiped`, `@context_menu`, `@sorted`, `@overwrite`, `@composing`, `@placed_above`, `@placed_below`, `@route_active`, `needs_attention`, CamelCase flags | `!{empty}`, `!{empty}`, `placed_above`, `@overlapping`, `current`, `editing`, `swiped`, `context_menu_open`, `sorted`, `overwrite`, `composing`, `placed_above`, `placed_below`, `route_active`, `@needs_attention`, snake_case | the shared flags (S7.2, "Shared flags" below) |
| 94 | `thumb_move`, `indicator_move`, `appear` (`PresentationTransition` values); `value_transition`, `indicator_transition`, `title_shrink`, `separator_hide`, `page_transition`, `snap`, `bracket_match_animation` (`Transition` values); `Presentations` | `presentations` entries (a part's own occasion a keyed event naming it, `@thumb_move`, `@indicator_move`, the third check; the element's own move `Move`); the moving part's `transitions`; `presentations: Vec<PresentationTransition>` | presentations and property transitions have one place each (S2.7) |
| 95 | `indeterminate_loop`, `spin` | `loop` | one name for a widget's repeating motion |
| 96 | `spinner.values.stroke_ratio` (3) | `spinner.values.stroke_width_px` as an expression over `element.width` | element data (R25.3) |
| 97 | `TextSlot`, slot names in CamelCase (5) | `TextRoles` fields in snake_case, each `= token.text.keyed.<key>` | S4.4 |
| 98 | `LabelPosition` (3, 4) | `ValueLabelPosition` (progress_bar, slider), `ButtonLabelPosition` (button) | one type per meaning |
| 99 | `input.values.hint_timeout_ms`, `input.values.caret_width` (3, 6) | `input.values.mask_reveal_ms`; the `caret` part's `min_width` (base `= token.caret.width`) | one value per meaning |
| 100 | `menu.badge` (1) | the `badge` widget in `context menu.item` | one badge |
| 101 | `PlacementStrategy::Keyed(…)` (7) | `KCommandBar`, `AppKitContextHelp`, `WinUiKeyTip`, `UiKitContextMenu`, `KirigamiAddonsOnboardingTip`, `KirigamiAddonsSearchPopup` | S13's list is closed and typed |
| 102 | `token.colour_effects`, `token.material_scheme` (5, 8) | `token.kde`, `token.material` | S4.8 |
| 103 | `scrollbar.values.overlay`, condition `TransientTouch` (3, 8) | the condition `overlay_scrollbars`; `cond input:touch` | S7.1 |
| 104 | `dialog.action`, `dialog.close_button`, `banner.close_button`, `toast.action_button`, `toast.close_button`, `sheet.close_button`, `teaching_tip.close_button`, `tab.close_button`, `tab.add_button`, `window.tab_close_button`, `window.new_tab_button`, `chip.remove_button`, `about_dialog.link_button`, `find_bar.nav_button`, `find_bar.options_button`, `find_bar.done_button`, `calendar.nav_button`, `carousel.nav_button`, `carousel.nav_glyph`, `page_indicator.nav_button`, `view_switcher.nav_button`, `view_switcher.overflow_button`, `toolbar.overflow_button`, `sidebar.toggle_button`, `input.clear_button`, `input.reveal_button`, `search_field.clear_button`, `search_field.cancel_button`, `search_field.query_button`, `search_field.bar_close_button`, `search_field.trailing_icon`, `media_controls.button`, `edit_menu.item`, `edit_menu.paging_chevron`, `edit_menu.expand_button`, `shortcuts_dialog.nav_pill`, `shortcuts_dialog.key_sequence_button`, `shortcuts_dialog.clear_button`, `time_picker.accept_button`, `time_picker.dismiss_button`, `toolbar_editor.move_button`, `breadcrumb.places_selector`, the components' close, float, section and action buttons | `button` elements in the host's context, their identity the typed `close`, `previous`, `next`, `overflow`, `add`, `remove`, `done`, `clear_field`, `reveal`, `pane_toggle`, `play`, `volume`, `full_screen`, `frame_step` or a keyed variant (`@query`, `@accept`, `@dismiss`, `@first`, `@last`, `@key_sequence`, `@float`) | Nested controls, the second check: one rule, one structure per kind |
| 105 | `combo_box.popup`, `item`, `item_mark`, `item_pill`, `values.max_visible_items`, `values.item_mark_position`, `values.presentations`; `autocomplete.popup`, `item`, `item_label`, `item_icon`, `values.width`, `values.max_visible_items`; `input.values.suggestion_placement`; `code_editor.completion_popup`, `completion_row`, `completion_icon`, `completion_detail`, `completion_details_pane`, `values.completion_placement`, `values.completion_offset_y`; `breadcrumb.ellipsis_popup`; `edit_menu.secondary_menu`; `date_picker.quick_item` | `menu` in the host's context: `menu.body`, `item`, `check_indicator`, `item_selection_indicator`, `item_label`, `item_icon`, `item_subtitle`, `values.check_indicator_side`, `values.presentations`; the popup's placement the host's (`combo_box.values.popup_placement`, `autocomplete.values.placement`, with its `max_visible_items` and `width`; rename row 119); the code editor's completion an `autocomplete` (its `details_pane`) | Nested controls |
| 106 | `stepper.text`; `search_field.field`, `search_icon`, `placeholder`, `leading_icon`; the components' text fields (`component.token_field.field`, `component.command_bar.search_field`, `component.rule_editor.value_field`) | `input` in the host's context (`input.body`, `input.icon @ variant leading`, `input.placeholder`) | Nested controls |
| 107 | `list.selection_check`; `chip.avatar`, `search_field.avatar`; `drag_image.count_badge`; `status_page.spinner`, `pull_to_refresh.spinner`; `media_controls.volume_popup`, `code_editor.informative_popup`; `search_field.token`; `shortcuts_dialog.row`, `row_title`, `row_subtitle`, `section_title`, `extender_indicator`, `footer`; `toolbar_editor.available_list`, `current_list`; the components' typed parts (`component.page_view.icon_list`, `search_container`, `component.rule_editor.popup_button`, `component.tabbed_command_bar.tab_row`, `command_bar`, `contextual_tab`, `component.command_bar.results`, `shortcut_cell`) | `checkbox`, `avatar`, `badge`, `spinner`, `progress_bar`, `popover`, `chip`, `list`, `toolbar`, `combo_box`, `view_switcher`, `search_field` elements in the host's context | Nested controls |
| 108 | `button` variant `split` | `split_button` (KDE's `MenuButtonPopup`, K1.155) | one kind, one widget (the second check) |
| 109 | `menu.item` variant `split`, part `menu.split_button` | keyed `windows-11` `@split`, `menu.split_chevron` | WinUI alone; no clash with the `split_button` widget |
| 110 | keyed `button` `@pane_toggle`, `@clear`, `@close`, `@done`; `media_controls` `@play`, `@volume`, `@full_screen`, `@frame_step`; `dialog` `@with_icon`; `dialog` and `sidebar` `@bottom_sheet`; `bottom_tab_bar` `@stacked`, `@inline`; `autocomplete` `@filled`, `@outlined` | typed `pane_toggle`, `clear_field` (row 104), `close`, `done`, `play`, `volume`, `full_screen`, `frame_step`; the content variant `with_icon`; typed `bottom_sheet`; typed `stacked`, `inline`; `input`'s `@filled` | R15: two or more platforms share the meaning; the field's look is the field's |
| 111 | first parts that were no container round the widget's parts (absent on some preset, repeated, a leaf, or not the outermost box): `link.text`, `radio_group.header`, `scroll_view.frame`, `file_chooser.frame`, `toolbar_editor.available_list`, `carousel.item`, `page_indicator.dot`, `rating.star`, `split_button.primary`, `about_dialog.icon`, `status_page.icon`, `color_picker.plane`, `code_editor.text` (its fill), `rich_text.paragraph`, `chart.plot_area`, `navigation_view.page`, `font_chooser.requester`, `wizard.header`, `autocomplete.popup`, `search_field.field`, the components' first parts | a container root first (`body`; `link.body` moved first; `picker_wheel.container` always drawn, its fields absent) | S2.11, S6.3: a root is never absent and never repeated |
| 112 | `window.tab_bar`, `tab`, `tab_title`, `tab_separator`; `window` event `Move` of its tabs | `tab` variant `document` in `context window.title_bar` (`tab.bar`, `tab.tab`, `tab.label`, `tab.separator`; `tab`'s events) | one tab strip (should-fix 18) |
| 113 | `checkbox.values.touch_target`, `radio.values.touch_target` | `checkbox.body.touch_target`, `radio.body.touch_target` | S2.11's field |
| 114 | a scalar widget value typed `Value<f32>` with no unit suffix (`icon_text_gap: Value<f32>`) | its TOML form and unit type (`icon_text_gap_px: Value<Px>`, `show_delay_ms: Value<Ms>`, `start_angle_deg: Value<Deg>`, `width_fraction: Value<Fraction>`) | S1's units |
| 115 | `sheet` and `tab` events `@open`, `@close` | `Appear`, `Disappear` | the typed events of an element appearing and disappearing |
| 116 | `menu_bar.values.height`, `values.submenu_corners`, `sheet.values.max_width`, `sidebar.values.min_width`, `values.max_width`, `rating.values.glyph`, `avatar.values.palette`, `values.initials_size`, `picker_wheel.values.off_center_fade`, `page_indicator.values.follows_scroll`, `chart.values.series_colors`, `shortcuts_dialog.values.row_min_height` | the parts' fields: `menu_bar.bar.min_height`, `menu.body.corners @ context menu_bar`, `sheet.body.max_width`, `sidebar.pane.min_width`, `pane.max_width`, `rating.star.glyph`, `avatar.body.fill` (by `pick` and `name_index`), `avatar.initials.text`, `picker_wheel.item.opacity` and `dot` fields as expressions over the scroll position, `chart.series.fill`, `list.row.min_height @ context shortcuts_dialog` | one value, one home |
| 117 | `toolbar.handle`, `tab.scroll_button`, `input.autofill_button`, `find_bar` flag `@neutral`, `popover` `@magnifier`, `shortcuts_dialog` flag `@recording` | `toolbar.grip`, `tab.scroll_arrow`, `input.autofill_affordance`, `@warning`, `input.magnifier`, `button`'s `@recording` | one name per part kind and meaning |
| 118 | `group_box` keyed `@photo`, `@gray_bezel`, `@groove`, `@button` (MAC-N12); keyed `component.image` (GN-3.07) | typed `image` (`body`, `picture`; the same keyed variants) | the third check: one home for the image view's frame and shadow styles, which two platforms draw |
| 119 | `form` keyed `@collapsed`; `checkbox`, `radio` keyed `@text_button`; `badge` keyed `@custom_color`; `list` keyed `@with_subtitle`, `@subtitle_cell`, `@two_line`; `button` part `attention_dot`, value `attention_dot_offset`, flag `@needs_attention` (also on `list`, `view_switcher`; `tab`'s, which draws GNOME's attention line, stays); `list.placeholder`; `menu` and `combo_box` popup placements in `menu.values.placement`; `toolbar.values.overflow_placement`; `calendar` `@out_of_range`; `progress_bar` `@nearly_full`; the widgets' own breakpoint values (`presentation_breakpoint`, `display_mode_breakpoints`, `pane_mode_breakpoints`, `collapse_thresholds`, `card_max_width_px`, `mobile_height_breakpoint_px`) | typed `collapsed`; the content variants `with_label`, `custom_color`, `with_subtitle`; the `badge` widget, variant `attention`, in the host's context; the flag `placeholder` on `list.row`; `combo_box.values.popup_placement`, `autocomplete.values.placement`, `button.values.menu_placement`; `disabled` on a day; an expression over `element.progress`; `breakpoints: Vec<Breakpoint>` | the third check: content variants are typed, one structure per control, the host's values stay the host's, one mechanism for breakpoints |
| 120 | `code_editor.whitespace_mark` (`@space`, `@tab`, `@newline`, `@nbsp`), `misspelled_word`, `search_match`, `caret` `@bidi_secondary`; `input`'s and `text_area`'s missing secondary caret | `code_editor.invisible` (typed `space`, `tab`, `line_break`, `no_break_space`), `misspelling`, `find_match`, `secondary_caret` (on `input`, `text_area`, `code_editor`) | the third check: one name and one variant set per part kind across the text widgets |
| 121 | presentation events: `tab` `@reorder`, `list` `@reorder` (an element moved); `Move` with a part step (`segmented_control`, `bottom_tab_bar`, `view_switcher`, `tab`, `list`'s indicators; `switch`'s thumb); `slider` `Appear`/`Disappear` of `value_indicator`; `date_picker` `Appear`/`Disappear` and `time_picker` `@open` of their popup; `banner` `@reveal`/`@conceal`, `media_controls` and `component.keyboard` `@show`/`@hide`, `sidebar` and `component.tab_overview`, `component.fab_menu` `@open`/`@close`, `toolbar` `@reveal`, `@hide_on_swipe` (the element shown or hidden); `toolbar` `@open`/`@close`; `search_field` `@cancel_appear`/`@cancel_disappear` | `Move`; `@indicator_move`, `@thumb_move`; `@value_indicator_show`, `@value_indicator_hide`; `@popup_show`, `@popup_hide`; `Appear`/`Disappear`; `@expand`/`@collapse`; the cancel `button`'s `Appear`/`Disappear` in `context search_field` | the third check: one event per occasion per widget, one name per occasion |
| 122 | `segmented_control` segment `{checked}`; `split_button` `{selected}` (Material's trailing half) | `{selected}` (the current choice among siblings, S7.2); `{open}` (the trailing half's menu) | the fourth check: S7.2's portable meanings |
| 123 | `button.activity_indicator`; `component.splitter_collapser`; `component.list_item_drag_handle`; `component.bottom_accessory`; `card` on `ios-*` (`card.values.background_inset`); `expander` `@row` | `spinner @ context button {@loading}`; `splitter.collapser`, `collapser_arrow`; `list.accessory` typed `reorder` with `list.values.autoscroll_*`; `bottom_tab_bar.accessory`, `accessory_content`, its environments `@accessory_inline`, `@accessory_expanded`; `list.row_background` with `list.values.row_background_inset`; `list.row` with `disclosure` and `nested`, or `form.row` `expander` with `row_arrow` and `row_nested` | the fourth check: one structure per control; a one-platform piece of a typed widget is a host part |
| 124 | `time_picker.flyout`, `highlight_band`, `values.item_height_px`, `field_min_width_px`, `field_max_width_px`; `picker_wheel.values.item_height_px`; `level_indicator.scale_arc`, `trail_arc`, `scale_tick`, `cap`, `values.arc_cap`; `scroll_view.values.edge_effect_edges` | `time_picker.popup` (as `date_picker.popup`), `picker_wheel.selection_band`, `picker_wheel.item.min_height`, `time_picker.field.min_width`, `field.max_width`; `level_indicator.track`, `fill` (the arcs under `circular`), `tick @ variant @scale`, `values.end_cap`; `scroll_view.edge_effect` and `overshoot` elements per edge (`top`, `bottom`, `start`, `end`) | the fourth check: one home per control and per fact |
| 125 | events `dialog` `@sheet_attach`, `@sheet_detach`; `list` `@insert`, `@delete`, `@reveal`; `tab` `@switch`; `bottom_tab_bar` `@selection_change`; `toolbar` `@back_label`, `@height_change`, `@separator_fade`, `@scroll_state`; `sidebar` `@labels_fade`, `@handle_slide`, `@scrim_fade` (and its flag `open`); `teaching_tip` `@contract`; `navigation_view` `@layer_push`, `@layer_pop`, `@layer_replace`; `component.context_menu_preview` `@lift`, `@dismiss` (and part `dim`); `component.list_item_drag_handle` `@drop` | `Appear`, `Disappear` (queried with the element they present); `@swipe`; `@indicator_move`; `@title_change`; the bar's `transitions`; steps of `Appear`, `Disappear`, `@collapse`, `@expand`; `@collapse`; `@push`, `@pop`, `@replace` under `@layer`; `scrim`; `Move` of the row | one event per occasion (SF-1 of the fourth check) |
| 126 | `color_button` `@light_color`; `slider.values.draw_ticks`; `tab.values.width_mode`; `toolbar.values.items_flat_by_default`; `wizard.values.button_placement`; `wizard` `@classic`; `keycap` `@on_selected`; `layout` `@chip_set`; `banner` `@sidebar`; `list` `@avatar`, `@image`, `@video`, `@three_line`; `toolbar` `@with_search`, `@with_prompt`, `ios-*` `@compact` | `gtk_contrast_pick(element.color, …)` on `swatch_ring.stroke`; `cond system_setting:slider_tick_marks`; keyed `@width_equal`, `@width_size_to_content`, `@width_compact`; `button` rules in `context toolbar.bar`; `@header_bar` alone; `kde-breeze`'s base; `context list.row {selected}`; typed `flow`; `context sidebar.pane`; typed content variants `with_avatar`, `with_image`, `with_video`, `three_line`, `with_search`, `with_prompt`; `v_size_class compact` | a look a rule can select, one route each (SF-5, SF-8, SF-9 of the fourth check) |
| 127 | widget values `teaching_tip` `min_width_px`, `max_width_px`, `tail_size_px`, `tail_margin_px`; `toast` `max_width_px`, `margins`; `status_page` `margins`, `max_width_px`; `chip` `padding_without_icon`; `banner` `heights`; `about_dialog` `margins`; `view_switcher` `item_min_width`; `component.command_bar` `preferred_width_divisor`, `preferred_height_divisor`, `top_offset_px`; `code_editor` `invisible_size_px` | the parts' fields (`body.min_width`, `body.max_width`, `tail.min_width`, `tail.margin`, `body.padding`, `body.padding @ !variant with_icon`, `body.min_height` rules, `item.min_width` rules, `frame.min_width` expressions, `invisible.glyph.size`); toast margins and the command bar's top offset as the placement's `edge_margin` | one value, one home (rename row 116, SF-6 of the fourth check) |
| 128 | `code_editor.caret` *(element)*, its `@vi_*` variants; `combo_box.arrow` *(element)*; `code_editor.selection`, `syntax_span`, `inline_annotation`, `annotation_border`, `minimap`, `current_line`, `calendar.weekday_label`, `form.card`, `level_indicator.tick` as plain parts | `caret` and `arrow` parts taking their widget's states (Vi's shapes the editor's variants); those parts *(element)* | the fourth check: a part with states, variants or flags of its own is an element, a mark taking its host's is not |

## Where group 6's §4.2 kinds go (its index, reconciled)

| §4.2 kind | Platforms (rows) | Decision |
|---|---|---|
| badge | all six | typed `badge` |
| autocomplete | GNOME, KDE, Material, iOS, Windows (CTK), macOS (MAC-CB01, MAC-I06, MAC-P3-7†: the second check) | typed `autocomplete`; its list a `menu` in its context |
| date picker | macOS, Windows, KDE, Material, iOS | typed `date_picker` |
| calendar | Windows, KDE, GNOME, iOS (+ Material, macOS inside their date pickers) | typed `calendar` |
| level indicator/gauge | GNOME, macOS, iOS, Windows (CTK) | typed `level_indicator`; KDE's LCD number and LED are one-platform kinds: keyed components `lcd_number`, `led`; iOS C-31 (determinate circular progress) is drawn inside `spinner` (its typed `determinate`; group 6 wrote `progress_bar`) |
| dial/circular slider | macOS, KDE | typed `dial` |
| menu bar | macOS, Windows, KDE, GNOME, iOS | typed `menu_bar`; macOS's Help-menu search is drawn inside `menu` |
| navigation/app bar | GNOME, KDE, Material, iOS, Windows | split by what is drawn: the page stack's push/pop is typed `navigation_view` (GNOME, KDE); bottom and Expressive bars and iOS's navigation bar are drawn inside `toolbar`; Windows' top NavigationView is a `view_switcher` (keyed `@navigation_top`); KDE's KPageView is a keyed component `page_view` |
| pull-to-refresh | macOS (27), Windows, KDE, Material, iOS | typed `pull_to_refresh` |
| rich text view | KDE, macOS, iOS, Material (Windows' MarkdownTextBlock is Toolkit Labs: out, R1.2) | typed `rich_text` |
| search field | macOS, Windows, GNOME, Material, iOS | typed `search_field` |
| stepper/spin box | macOS, Windows, KDE, GNOME, iOS | typed `stepper` |
| about dialog | GNOME, KDE, macOS (Windows' `ShellAbout`: out, R1.1) | typed `about_dialog` |
| banner/inline message | Windows, KDE, GNOME, Material | typed `banner` |
| bottom tab bar | macOS, KDE, Material, iOS, GNOME (AdwViewSwitcherBar, GN-2.15: the third check, by what is drawn) | typed `bottom_tab_bar` (the merge: its parts and state model are not the tab's) |
| breadcrumb/path control | macOS, Windows, KDE, GNOME | typed `breadcrumb` |
| carousel | Windows, GNOME, Material, iOS | typed `carousel` |
| colour button | macOS, Windows (CTK), KDE, GNOME, iOS (C-8) | typed `color_button` |
| colour picker | Windows, KDE, GNOME | typed `color_picker` |
| media transport controls | macOS, Windows, GNOME, iOS | typed `media_controls` |
| page control/pager | Windows, KDE, GNOME, iOS | typed `page_indicator` |
| preference rows/boxed list | GNOME, KDE, macOS, Windows (CTK) | typed `form`; KDE's list section header and inline view header are drawn inside `list` |
| sheet/bottom sheet | KDE, GNOME, Material, iOS | typed `sheet` |
| split button | macOS, Windows, GNOME, Material, KDE (K1.155's `MenuButtonPopup`, kirigami-addons' K-KA-27†) | typed `split_button` |
| status page/placeholder | macOS, KDE, GNOME, iOS | typed `status_page` |
| teaching tip | macOS, Windows, iOS, KDE (kirigami-addons' onboarding tip, K-KA-45†) | typed `teaching_tip`; KDE's What's This popup (K2.31†) is the typed `context_help` (below) |
| text input system UI | all four rows | out (R1.1: an input method's own window, OS-drawn popups) |
| avatar | Windows, KDE, GNOME, Material (MAT-L2, MAT-TA5, MAT-N9, MAT-N14: the second check) | typed `avatar` |
| chart | macOS, KDE, iOS (Swift Charts, KQuickCharts: R1.2) | typed `chart` |
| chip | Windows (CTK), KDE, Material | typed `chip` |
| edit menu | KDE, Material, iOS | typed `edit_menu` |
| file chooser | GNOME, KDE (GTK's own chooser, KFileWidget); GNOME's portal chooser and Windows' Common Item Dialog are out (R1.1) | typed `file_chooser` |
| find bar | macOS, KDE, iOS | typed `find_bar` |
| font chooser | GNOME, KDE (Win32 `ChooseFont`: out, R1.1) | typed `font_chooser` |
| group box/frame, image frame | GNOME, iOS, KDE, macOS | typed `group_box` (group 6 wrote `frame`; a frame with an optional title); the image view's frame and shadow styles are the typed `image` (macOS's MAC-N12, GNOME's GN-3.07: the third check) |
| menu button | macOS, Windows, Material | drawn inside `button` (the typed variant `menu` and its `menu_indicator`); a toolbar's tool buttons are `button` variant `tool` in `context toolbar.bar` |
| rubber band | macOS, KDE, iOS | typed `rubber_band` |
| shortcuts dialog | GNOME, KDE (iOS's discoverability HUD: out, R1.1) | typed `shortcuts_dialog` |
| time picker | Windows, KDE, Material | typed `time_picker` |
| toast/snackbar | KDE, GNOME, Material | typed `toast`; iOS's editing-gesture HUD is the keyed `component.editing_hud` (C-49†) |
| wizard | GNOME, KDE (Windows' Aero wizard is Win32: preset, R1.3) | typed `wizard` |
| button group/toggle group | KDE (Q2.3 is a DelayButton), Material | Material's button group is drawn inside `segmented_control`; KDE's DelayButton is a one-platform kind: keyed component `delay_button` |
| composite dialogs | GNOME, KDE | out (R1.1: print, page setup, app chooser, Open With are dialogs a toolkit only invokes); GtkDialog's action area is `dialog`'s |
| editable label | GNOME, iOS | drawn inside `input` (GNOME's cross-check: GN-2.45 belongs to `[input]`) |
| emoji chooser | GNOME (KDE's plasma-emojier is a separate Plasma program: out, R1.1) | keyed component `emoji_chooser` |
| FAB | KDE (RoundButton), Material | drawn inside `button` (a round button and Material's FAB styles are button variants); Material's FAB menu is a keyed component `fab_menu` |
| full-screen dialog | KDE, Material | drawn inside `dialog` |
| help button | macOS, KDE | drawn inside `button` (the typed `help`) |
| icon button | Material, iOS | drawn inside `button` |
| image cropper | macOS (ImageKit), Windows (CTK) | typed `image_cropper` |
| ink toolbar | Windows, iOS | typed `ink_toolbar` |
| inspector/utility panel | macOS, iOS | drawn inside `window` (HUD and utility panels) and `sidebar` (the trailing inspector column: its typed `end`) |
| navigation drawer/pane | GNOME, Windows | drawn inside `sidebar`; GtkAssistant's step sidebar is `wizard`'s |
| password field | Windows, GNOME | drawn inside `input` (the typed variant `secure`) |
| picker wheel | Material, iOS (and Windows' looping selectors, W2-04) | typed `picker_wheel` |
| rating | Windows, KDE (and macOS's rating level indicator) | typed `rating` |
| text ruler | macOS, KDE | typed `ruler` |
| toolbar customization | KDE (Windows' is Win32: preset, R1.3) | typed `toolbar_editor` (the merge: KDE, macOS and iOS draw it, group 4's rows) |
| view switcher | Windows, GNOME | typed `view_switcher` (the switchers in a header or above content; GNOME's bottom switcher bar is `bottom_tab_bar`); GNOME's tab overview is a keyed component `tab_overview` |
| wrap box | GNOME, Material (chip set) | drawn inside `layout` (its typed `flow`, Material's chip set included) |
| media output (AirPlay, cast) button | macOS (MAC-P3-9†), iOS (C-42†), Windows (W-P4-18†) | drawn inside `button` (its typed identity `route`, the third check; the check pass had typed a widget of its own); Windows' casting device list is the keyed `component.casting_picker`, the system popovers are out (R1.1) |
| context-help pop-up | KDE (K2.31†), macOS (MAC-P4-11†) | typed `context_help` (added in the check pass) |

## Window and chrome

### window

*From group 1's catalogue.*

- **Kind:** typed widget: every platform draws a window; its chrome is drawn by the toolkit where the toolkit draws
  its own frame (R1.1, R13).
- **Available:** every preset.
- **Parts:**
  - `frame` — the window's outer box (the root): `fill` (the decoration's border band: KDE's Frame role, GNOME's
    non-compositing frame), `stroke` (the outline: GNOME's and KDE's outlines, each in its own position, macOS's
    outer edge line, the DWM border), `corners` (per corner, with its curve; on iOS and Android the outermost
    container the `Radius::Concentric` corners inside it derive from: each corner `Radius::Px(=
    element.display_corner_radius.<corner>)` where the connector supplies the display's radius (S8; the part declares
    the data), else `= token.params.display.corner_radius`, measured per device model and set by rules under `cond
    device_model:<key>` (an unlisted model's radius is `Absent`, and concentric corners fall back to their minimum;
    S4.6, W-6, ADD-A83): X-CR-4, MAT-W5†, ADD-B18, R25.11), `shadow` (layers, or an elevation entry;
    GNOME's outer ring and macOS's inner light rim are layers, the rim `inset`), `padding` (the border band's width
    per side: KDE's border size, GNOME's `.solid-csd`), `transitions` (the activation shadow fade).
  - `content` — the client area: `fill` (the window background, or the platform's material: Mica, Mica Alt, the
    AppKit window material), `bindings` (`Surface(Window)`), `min_width`, `min_height`, `transitions` (Mica's fade to
    its inactive fallback).
  - `title_bar` — the bar, in `frame`: `fill` (colour, gradient, material), `stroke` and `shadow` (GNOME's bottom shade
    line and side borders as inset layers, its drop shadow; macOS's separator line or shadow), `min_height`,
    `bindings` (`Surface(Header)`, which reach every part nested in the bar, S2.11), `transitions`.
  - `title_bar_content` — the box holding title and buttons, in `title_bar`: `padding`, `opacity` (the inactive
    dimming of everything nested in it, as one group), `transitions`.
  - `title` *(element)*, in `title_bar_content` — the title, with its own hover (macOS shows `title_chevron` while the
    title is hovered, MAC-W11†; `title_status` and `title_chevron` belong to it and are queried with it): `text`
    (font, fill, elision), `padding`, `margin` (AdwWindowTitle's negative margins, GN-1.18), `transitions`.
  - `subtitle`, in `title_bar_content` — `text`, `padding`, `margin` (GN-1.18), `opacity`.
  - `title_icon`, in `title_bar_content` — the application icon in the title bar, or macOS's proxy (document) icon:
    `icon`, `padding`.
  - `title_status`, in `title` — text the toolkit appends to the title for a window state (macOS's "Edited"): `text`.
  - `title_chevron`, in `title` — macOS's document-title chevron shown on hover: `glyph`, `opacity`, `transitions`.
  - `caption_buttons` *(element)*, in `title_bar_content` — the group of caption buttons, an element of its own so that rules select on the
    group's hover (macOS shows all three glyphs while the group is hovered: `@ context window.caption_buttons
    {hover}`): `padding` (leading inset, vertical position). The `caption_button` elements are its children: the
    connector queries them in the scope it returns. Absent, with the buttons, under the keyed `@no_title_buttons`
    (GNOME's header bar of an assistant, GN-RL01†).
  - `caption_button` *(element)*, in `caption_buttons` — one caption button, its identity a variant (below): `fill` (WinUI's rectangle),
    `min_width`, `min_height`, `padding`, `corners`, `transitions`. `caption_button_glyph` belongs to `caption_button`
    and is queried with it.
  - `caption_button_glyph` — the glyph and the shape behind it (GNOME's circle, KDE's hover circle, macOS's traffic
    light): `glyph` (source per identity: Segoe Fluent Icons code points, Breeze's paths, theme icons), `icon` (KDE's
    window-menu button shows the window icon recoloured), `fill`, `stroke` (the rim), `corners`, `shadow` (GNOME's
    high-contrast inset ring, GN-1.22), `padding`,
    `min_width`, `min_height`, `opacity`, `transitions`.
  - `window_menu`, in `title_bar` — the anchor part of the window menu, which is a `menu` element whose ancestor is
    this part wherever it opens (at the pointer, or from the caption button whose identity is also `window_menu`: the
    connector queries this part above the menu either way, so its binding reaches both): `bindings` (KDE draws the
    menu in the window's own decoration palette, K-P4-1†).
  - `resize_zone` *(element)* — one per edge and corner (identity by variant): `cursor`.
  - `launch_screen` — the launch or splash screen the system draws from the application's values: `fill`,
    `bindings`.
  - `launch_icon`, in `launch_screen` — the splash icon: `icon` (size), `fill` (Android's icon background).
- **Widget values:**
  - `caption_button_layout: Value<CaptionLayout>`, `CaptionLayout { leading: Vec<CaptionItem>, trailing: Vec<CaptionItem> }`, with
    `CaptionItem { Button(VariantRef), Spacer, Icon }`, the button named by its identity, a variant of
    `caption_button` (below): which buttons, on which side, in which order (read on GNOME and KDE; mirrored by a
    `dir rtl` rule, W3-54†).
  - `caption_button_gap_px: Value<Px>`; `caption_spacer_width_px: Value<Px>`.
  - `caption_button_capabilities: Vec<(VariantRef, WindowCapability, WhenMissing)>`, `WindowCapability {
    Closable, Minimizable, Maximizable, Resizable, Sovereign, ProvidesHelp, Shadeable }`, `WhenMissing { Hide,
    Disable }`: GNOME hides the button that minimises a transient window, Windows greys the button that maximises a
    fixed-size one (W1-148†).
  - `caption_button_menus: Vec<(VariantRef, Vec<Key>)>` and `caption_button_menu_placement: Placement`: the menu
    a caption button opens (macOS's zoom-button tiling menu); its look is `menu` in `context window.caption_button`.
  - `title_alignment: Value<TitleAlignment>`, `TitleAlignment { Start, Center, CenterFullWidth, End }`;
    `CenterFullWidth` centres on the whole bar and falls back to the free side when it meets the buttons (KDE).
  - `title_margin_px`, `title_bar_box_gap_px`, `title_icon_gap_px`, `title_subtitle_gap_px: Value<Px>`.
  - `title_bar_click_actions: TitleBarActions { double_click: Value<Key>, middle_click: Value<Key>, right_click:
    Value<Key> }` (read).
  - `window_menu_items: Vec<Key>` (the actions in order, with separators, K-P4-1†); `window_menu_placement:
    Placement` (at the pointer; below a button's rectangle, K-P4-1†); `title_menu_placement: Placement` (macOS's
    title pop-up menu).
  - `resize_band: Value<ResizeBand>`, `ResizeBand { outside: Sides<Value<Px>>, inside: Sides<Value<Px>>, corner_px: Value<Px>,
    includes_shadow: Value<bool> }`; its bounds follow the frame by `widget.frame.<field>` references (the band
    outside the frame's shadow and border, the corner zone at least the frame's corner radius: GN-1.282†, ADD-A80).
  - `full_screen_title_bar_reveal: Value<RevealTrigger>`, `RevealTrigger { Never, PointerAtTopEdge, SwipeFromTop }`.
  - `status_bar_style: Value<StatusBarStyle>`, `StatusBarStyle { Default, LightContent, DarkContent }` (iOS's status
    bar style, Android's system-bar icon tone, MAT-W1: values the application configures for the system);
    `navigation_bar_contrast_scrim: Value<bool>` (Android, MAT-W1); `caption_bar_transparent: Value<bool>` (Android
    desktop windowing: the application's opt-in to a transparent caption, which the system then draws over the
    application's own title bar fill, MAT-W4†; the caption itself is the system's, R1.1).
  - `default_size: Axes<Value<Px>>` (Kirigami's window, in grid units; `@ka_settings`, K-KA-41†).
  - `breakpoints: Vec<Breakpoint>` (Shared value types: the header bar treats a lower window as mobile, setting
    the keyed `@mobile`, GN-1.197).
  - `keyboard_avoidance: Placement` (strategy `KeyboardAvoid(…)`: what moves when the on-screen keyboard covers the
    focused field, W-P4-3†, MAT-P4-2†).
  - `attention_flash: Value<AttentionFlash>`, `AttentionFlash { period_ms: Value<Ms>, count: Value<u32> }` (Windows).
  - `windowing_control_style: Value<Key>` (iPadOS 26's `UIWindowScene.WindowingControlStyle`, keys `automatic`,
    `minimal`, `unified`: the application's choice the system draws its window controls with, W-1; R1.1 puts what
    the application configures for the system in, as `status_bar_style`).
  - `tab_bar_overflow: Value<Key>` (what macOS's window tab bar does with more tabs than fit; the keys follow the
    measurement, MAC-P4-5†; the tabs are `tab` elements, below).
  - `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the window opening and closing:
    macOS's `NSWindow.AnimationBehavior` per kind, MAC-X11), keyed `@title_bar_reveal` (the full-screen reveal),
    `@launch_exit` (the splash into the first frame), `@predictive_back` (Material's in-app back gesture).
- **Variants:**
  - on the window, typed: `dialog` (a dialog's window: GNOME's message window shadow, the close-only caption of
    GNOME and Windows, macOS alerts and panels), `utility` (Windows' tool window, macOS's utility panel: small caption
    and buttons, own radius and shadow; MAC-N16, W1-148†), `transparent_title_bar` (GNOME's `.flat` header bar,
    macOS's `titlebarAppearsTransparent`).
  - on the window, keyed: `adwaita` `@ssd`, `@ssd_frame`, `@default_decoration`, `@standalone`,
    `@collapse_spacing`, `@inline`, `@devel`, `@mobile` (set by `breakpoints`, GN-1.197), `@no_title_buttons`
    (the header bar shows no window buttons: GtkAssistant
    with `use-header-bar`, set by the connector for a `wizard` in `@header_bar` mode, GN-RL01†); `windows-11` `@tall`,
    `@collapsed`, `@titlebar_control_expanded`,
    `@mica_alt`, `@round_small`, `@compact_overlay`; `macos-*` `@hud` (the HUD panel's material, title bar and close
    button, MAC-N14), `@title_hidden`, `@no_toolbar`, `@toolbar_unified`, `@toolbar_unified_compact`,
    `@toolbar_expanded`, `@toolbar_preference`, `@separator_line`, `@separator_shadow`; `kde-breeze` `@ka_settings`
    (kirigami-addons' settings window, K-KA-41†).
  - on `caption_button` (its identity, one per `CaptionButton`), typed: `close`, `minimize`, `maximize`, `restore`,
    `window_menu` (KDE's menu button, GNOME's `appmenu`); keyed `kde-breeze` `@help`, `@on_all_desktops`,
    `@keep_above`, `@keep_below`, `@shade`, `@exclude_from_capture`, `@application_menu` (Windows' help button is
    Win32, W1-148†, R1.3).
  - on `resize_zone`, typed: `top`, `right`, `bottom`, `left`, `top_left`, `top_right`, `bottom_right`,
    `bottom_left`.
- **Sizes and densities:** none; the title-bar heights are variants (Windows' height option, GNOME's decoration kinds,
  macOS's toolbar styles); KDE's button and border sizes are read settings.
- **Flags:** shared none; keyed `macos-*` `@document_edited` (on the window: the close button's dot and the title's
  "Edited", MAC-W11†); state flags used: hover (on `title`, MAC-W11†; on `caption_buttons`, the group; on each
  `caption_button`), pressed, checked (KDE's toggled caption buttons), disabled, scrolled (macOS's separator shown
  while content scrolls beneath, on the window, which the `title_bar` rule selects).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `caption_button` (the caption buttons' rest,
  hover and pressed states; their inactive look is `cond window_inactive`); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:**
  - parts: `title_bar` (KDE's `HideTitleBar`, full screen; `ios-*` and `material-3`, whose title bars and captions
    the system draws or does not draw, W-9, MAT-W4†, R1.1), `title_bar_content`, `subtitle` (`ios-*`, `material-3`),
    `title` (`ios-*`, `material-3`; `macos-*` under `@title_hidden`: the title goes, the bar and its traffic lights
    stay, MAC-W14†), `title_icon` (`ios-*`, `material-3`; `macos-*`, by a rule outside `cond
    system_setting:title_icons`, MAC-X31†; elsewhere it is drawn where `caption_button_layout` places it),
    `title_status`,
    `title_chevron` (all but `macos-*`),
    `window_menu` (`macos-*`, `ios-*`, `material-3`: no window menu),
    `resize_zone` (`ios-*`, `material-3`; a maximised window), `caption_buttons`, `caption_button`,
    `caption_button_glyph` (`ios-*`, `material-3`; `adwaita` under `@no_title_buttons`), `launch_screen`, `launch_icon`
    (`adwaita`, `kde-breeze`, `macos-*`).
  - fields: `frame.shadow` (maximised, full screen, `cond not_composited`), `frame.stroke`, `frame.padding`,
    `caption_buttons.padding`; `title_bar.shadow`, `title_bar.stroke` (each preset whose platform draws none, which the value phase names from the facts; on `macos-*` by rule outside the variant
    that draws the line or shade, `@separator_line`, `@separator_shadow`); `caption_button_glyph.shadow` (all but
    `adwaita`: GNOME's high-contrast ring, GN-1.22), `caption_button_glyph.stroke` (each preset whose platform draws none, which the value phase names from the facts), `caption_button_glyph.icon` (all but `kde-breeze`: the window icon on its
    menu button, K1.6), `caption_button_glyph.fill`, `caption_button_glyph.corners` (`windows-11`: the rectangle is
    `caption_button`'s own fill).
  - values: `resize_band` (`ios-*`, `material-3`; a maximised window), `status_bar_style` (all but `ios-*`,
    `material-3`), `navigation_bar_contrast_scrim`, `caption_bar_transparent` (all but `material-3`),
    `attention_flash` (all but `windows-11`), `default_size` (all but `kde-breeze`), `keyboard_avoidance` (all but
    `windows-11`, `material-3`), `tab_bar_overflow` (all but `macos-*`), `windowing_control_style` (all but
    `ios-*`); `caption_button_layout`, `caption_button_gap_px`, `caption_spacer_width_px`, `title_alignment`,
    `title_margin_px`, `title_bar_box_gap_px`, `title_icon_gap_px`, `title_subtitle_gap_px`, the three
    `title_bar_click_actions` fields, `full_screen_title_bar_reveal` (`ios-*`, `material-3`: the system's chrome);
    `caption_button_menu_placement`, `title_menu_placement` (all but `macos-*`); `window_menu_placement` (`macos-*`,
    `ios-*`, `material-3`). A list value (`caption_button_capabilities`, `caption_button_menus`,
    `window_menu_items`, `breakpoints`, `presentations`) is empty, not absent, where the platform has none.
- **State-model source:**
  - `windows-11`: the DWM caption buttons' states as `AppWindowTitleBar` names them (rest, hover, pressed, inactive;
    per button, with close's own), WinUI's `WindowCaptionButton` style (`generic.xaml:15697`) and the `TitleBar`
    control's visual states (`TitleBar/TitleBar.xaml`, activation and display states).
  - `adwaita`: libadwaita's `_window.scss` (backdrop, tiled, maximized, fullscreen, `.solid-csd`, `.ssd`,
    `.ssd-frame`, HC) and `_header-bar.scss` (backdrop, `.flat`, `.default-decoration`, `.collapse-spacing`, window
    controls :hover/:active), GtkWindowControls' per-window rules (`gtkwindowcontrols.c`).
  - `kde-breeze`: Breeze's decoration branches (`breezedecoration.cpp`: active, maximised, tiled per edge, shaded,
    border size) and button branches (`breezebutton.cpp`: hovered, pressed, checked, active).
  - `macos-*`: `NSWindow` key and main state, `styleMask`, `toolbarStyle`, `titlebarSeparatorStyle`,
    `titleVisibility`, `titlebarAppearsTransparent`, the standard buttons' states (group rollover, pressed, disabled,
    document edited).
  - `ios-*`: the scene's `activeAppearance` trait; `UILaunchScreen` keys; the status bar style enum.
  - `material-3`: none for in-window chrome (the system draws the frame); `WindowInsetsController` appearance and the
    splash-screen theme attributes.
- **Today's fields:** `background_color` → `content.fill`; `title_bar_background` → `title_bar.fill`;
  `title_bar_font` → `title.text.font` (its colour → `title.text.fill`); `inactive_title_bar_background` →
  `title_bar.fill @ cond window_inactive`; `inactive_title_bar_text_color` → `title.text.fill @ cond
  window_inactive`; `border.color`, `border.line_width_px` → `frame.stroke`; `border.corner_radius_px` →
  `frame.corners`; `border.shadow_enabled` → `frame.shadow` (no layers is no shadow, R8); `border.padding_*` →
  `frame.padding`.
- **Notes:**
  - Conditions the window's rules use: `cond window_inactive`, `cond not_main` and `cond app_inactive` (macOS),
    `cond maximized`, `cond full_screen`, `cond tiled:top` and the other edges (a window maximised on one axis counts
    as tiled to its edges), `cond contrast_high`, `cond reduced_transparency`, `cond scheme:dark`, `cond idiom:phone`,
    `cond idiom:pad`, `cond mobile_form_factor`, `cond system_setting:<key>` (KDE's breezerc switches, Windows' accent
    on title bars, macOS's wallpaper tinting), `cond not_composited`, `cond shaded`, `cond attention_flash` (ADD-A01,
    ADD-A02, ADD-A03), `cond freeform` (Android's desktop-windowing window, its own corner radius, MAT-W5†, ADD-A83).
  - `title_icon` under `cond system_setting:title_icons` (macOS's "Show window title icons", MAC-X31†; the Full
    Keyboard Access highlight of the same row is the OS's, R1.1).
  - Group 6: MAC-N16's utility panel is the typed `utility` (shorter title bar, small caption buttons, title font).
  - Group 8: `content.fill` with the platform's material; `frame.corners` as the outermost container the
    `Radius::Concentric` corners derive from (X-CR-4). Group 8's caption-button side value (W3-54†) is `caption_button_layout`.
  - macOS's native window tabs (MAC-W09) are the `tab` widget, variant `document`, in `context window.title_bar`: the
    strip `tab.bar`, each tab `tab.tab`, its close button `button @ variant close context tab.tab`, the new-tab button
    `button @ variant add context tab.bar`; the insertion feedback `tab.bar @ {drop_target}` and the tabs parting
    (event `Move`, MAC-SF07†), their opening and closing (events `Appear`, `Disappear`, MAC-X32†) are `tab`'s.
  - kirigami-addons: `@ka_settings` sets `default_size`, `content.min_width`, `content.min_height` (K-KA-41†).

## Overlays

### dialog

*From group 1's catalogue.*

- **Kind:** typed widget: every platform (AdwDialog and AdwAlertDialog, QDialog, KMessageBox and Kirigami's dialogs,
  NSAlert and sheets, UIAlertController, Material's dialog and full-screen dialog, ContentDialog).
- **Available:** every preset.
- **Parts:**
  - `body` — the dialog box (the root; group 7 called it `platter` for the iOS action sheet): `fill` (colour or
    material), `stroke` (GNOME's inner outline, Kirigami's border), `corners`, `shadow`, `padding`, `min_width`,
    `max_width`, `min_height`, `max_height`, `bindings` (`Surface(Dialog)`; Kirigami's View set; on `ios-*` the
    `Dialog` set, whose background roles the preset states as the elevated values, and a `Binding::Role` from each
    keyed background role to its elevated twin, `role.@system_background` to `= role.@system_background_elevated` and
    so on: iOS's elevated level is a binding in a sheet or popover, R2.3, R5.2, D-14).
  - `scrim` — the dimming behind the dialog: `fill` (`= role.global.scrim`, or libadwaita's derivation from the shade
    colour), `transitions`.
  - `header` — the title area: `padding`, `fill` and `bindings` (KDE's Header-filled tools area; they reach the
    parts nested in the header and the close `button` queried in it, S2.11), `stroke` (the separator under it, shown
    while the content overflows; the full-screen dialog's line under the header), `min_height` (Kirigami's dialog
    layer header, Q2.58†; the full-screen dialog's header, MAT-N26; `Auto` where the header is sized by its content),
    `shadow` (the full-screen dialog's on-scroll elevation, MAT-N26).
  - `title`, in `header` — `text`, `padding`.
  - `subtitle`, in `header` — `text` (Material's subhead, Kirigami's prompt subtitle).
  - `message` — the body text and content area: `text`, `padding`, `corners` (GNOME's scrolled alert body).
  - `icon` — the hero or standard icon: `icon`, `glyph`, `padding`.
  - `icon_badge` — macOS's caution badge over the application icon of a critical alert: `icon`, `padding`.
  - `action_area` — the footer or command area holding the responses: `fill` (WinUI's command area), `stroke` (the
    line above it: WinUI's separator, Kirigami's and Material's when the content overflows), `padding`, `min_height`.
  - `separator`, in `action_area` — the hairlines between action cells (iOS 13–18; the action sheet, C-11): `stroke`
    (the line: its width and paint, the top edge across a row, the start edge between cells).
  - `cancel_platter` — the action sheet's separate cancel box (C-11), beside the body: `fill` (its material),
    `corners`.
- **Widget values:**
  - `title_alignment`, `message_alignment: Value<Alignment>`; `title_message_gap_px`,
    `message_actions_gap_px: Value<Px>`.
  - `action_layout: ActionLayout { row_order: Vec<ActionRole>, stacked_order: Value<StackedOrder>, alignment:
    Value<Alignment>, equal_widths: Value<bool>, row_gap_px: Value<Px>, column_gap_px: Value<Px>, stack_rule:
    StackRule }`; `ActionRole { Accept, Reject, Destructive, Help, Reset, Apply, Other }` (Qt's button roles, which
    KDE's `KdeLayout` orders); `StackedOrder { SameAsRow, Reversed, AcceptFirst }`; `StackRule { Never,
    WhenOverflowing { horizontal_padding_px: Value<Px> }, ByCount { max_in_row: Value<u32> } }`.
  - `breakpoints: Vec<Breakpoint>` (Shared value types; measured against the window, R16; the below-threshold ones
    await the core, Open items): per preset, `adwaita`
    `bottom_sheet` and `narrow` below a width and `@short` below a height (AdwDialog's bottom sheet and breakpoint
    classes); `material-3` `full_size` (the full-screen dialog); `kde-breeze` `full_size` (Kirigami's full-window
    layer, Q2.58†, by rules under `cond mobile_form_factor`) and `narrow` (`@ka_message_dialog`, K-KA-19†).
  - `window_margin: EdgeMargin { min_px: Value<Px>, max_px: Value<Px>, from_window_px: Value<Px>, to_window_px:
    Value<Px> }` per axis: the margin to the host's edges, constant (Kirigami, Material) or growing with the window
    (AdwDialog's floating sheet); group 7's `screen_edge_margin` (C-11) is this value.
  - `natural_size: Axes<Value<Px>>` (AdwDialog's default, Kirigami's MenuDialog and SearchDialog).
  - `cancel_platter_gap_px: Value<Px>` (the space between the action sheet's actions and its separate cancel box,
    C-11).
  - `placement: Placement`; `keyboard_avoidance: Placement` (strategy `KeyboardAvoid(…)`).
  - `presentations: Vec<PresentationTransition>`: events `Appear` (the action sheet's kind `Slide` from the bottom,
    C-11; a macOS sheet attaching to its window, under `@sheet`), `Disappear` (the sheet detaching).
  - `button_icons: Value<bool>` (KDE, read); `standard_button_icons: Vec<(ActionRole, Key)>` (icon names, K-KA-19†);
    `type_icons: Vec<(Key, Key)>` (Kirigami's prompt types to icon names, K-KA-19†).
  - `message_wrap: MessageWrap { wrap_above_px: Value<Px>, min_width_px: Value<Px>, elide_above_px: Value<Px>,
    scroll_above_px: Value<Px>, item_list_max_width_px: Value<Px>, details_min_lines: Value<u32> }` (KMessageBox,
    K-P3-7†): the message wraps above `wrap_above_px` (`= min(mul(element.screen_width, <fraction>),
    mul(element.char_width, <count>))`, the count a translated sample line), its label is at least `min_width_px`
    wide (the same form), it is elided above `elide_above_px` (`= mul(element.screen_width, <fraction>)`) and scrolls
    above `scroll_above_px` (`= mul(element.screen_height, <fraction>)`); the item list is at most
    `item_list_max_width_px` wide (`= mul(element.screen_width, <fraction>)`), the details text area at least
    `details_min_lines` lines high. The screen size comes from the root (S8); the widget value declares the font
    metric it reads, the message font's character width (`data = { values = ["char_width"] }`, S8, S10).
- **Variants:**
  - on the dialog, typed: `alert` (a message dialog: AdwAlertDialog, KMessageBox and Kirigami's PromptDialog,
    NSAlert, UIAlertController's alert, Material's basic dialog), `separate_window` (a dialog in its own window:
    AdwDialog's fallback, Qt dialogs, Kirigami's native-window popup type and its desktop dialog layer, Q2.58†,
    macOS's app-modal alert), `full_size` (ContentDialog's `FullSizeDesired`, Material's full-screen dialog MAT-N26,
    Kirigami's full-window dialog layer Q2.58†), `bottom_sheet` (presented as a bottom sheet: AdwDialog's bottom
    sheet; the meaning `sidebar`'s `bottom_sheet` shares on Material), `warning`, `info`, `error`, `success`
    (NSAlert's styles,
    KMessageBox's standard icons, Kirigami's prompt types), `narrow` (a dialog laid out for a narrow host: AdwDialog's
    `.narrow` breakpoint class, kirigami-addons' message dialog under it, K-KA-19†; set by `breakpoints`).
  - on the dialog, keyed: `adwaita` `@short` (set by `breakpoints`), `@wide`, `@message_dialog`; `kde-breeze`
    `@question`, `@menu_dialog`, `@search_dialog`, `@flat_footer`,
    `@ka_dialog` (the one surface of every kirigami-addons dialog and popup: corners, View binding, stroke, shadow,
    K-KA-18†), `@ka_form_card_dialog` (K-KA-17†), `@ka_message_dialog` (K-KA-19†), `@ka_list_dialog` (the combo
    row's list dialog, K-KA-12†); `macos-*` `@sheet`, `@picture_taker` (ImageKit's picture taker panel, MAC-P4-12†);
    `ios-*` `@action_sheet` (C-11). Material's dialog with a hero
    icon is the content variant `with_icon` (ADD-A61), which the connector sets from the content.
  - the responses are `button` elements in `context dialog.action_area`, with `button`'s typed `primary` (GNOME's
    suggested response) and `destructive` and keyed `ios-*` `@cancel`; the default response is the state `default`.
- **Sizes and densities:** none; AdwDialog's breakpoint classes are the variants `narrow`, `@short`, set by
  `breakpoints` (R16); the
  accessibility text sizes are the condition `accessibility_text_size` (ADD-A08).
- **Flags:** shared none; keyed `kde-breeze` `@has_toolbar`, `@scroll_view_content`, `@has_cancel` (the close button
  shown when a Cancel button exists, K-KA-17†); state flags used: scrolled, overflowing (ADD-A13).
- **Exclusive groups:** none: the dialog holds no interaction state, and its responses and close button are `button`
  elements, whose groups are `button`'s (ContentDialog's `DialogShowingStates`, `DialogSizingStates` and
  `ButtonsVisibilityStates` are visibility and sizing, held by variants and values, and `DefaultButtonStates` is the
  responses' state `default`); precedence: state models (S17).
- **may_be_absent:**
  - parts: `scrim` (Qt Widgets dialogs on `kde-breeze`, macOS's app-modal alerts), `header` (`kde-breeze` under
    `separate_window` with `cond flavor:kirigami`: a dialog in its own window draws its title in the window's title
    bar, Q1.107), `icon_badge` (all but `macos-*`), `separator`, `cancel_platter` (all but `ios-*`; on `ios-26`, whose
    alerts draw separate capsule buttons with gaps, DL-5, and whose action sheet is anchored to its source, C-11).
  - fields: `action_area.fill` (all but `windows-11`), `action_area.stroke` (`adwaita`, `macos-*`, `ios-*`),
    `header.fill` (all but `kde-breeze`), `header.stroke` (`adwaita`, `macos-*`, `ios-*`, `windows-11`),
    `header.shadow` (all but `material-3`), `body.stroke` (`ios-*`, `macos-*`, `material-3`: iOS's alerts, NSAlert,
    Material's dialogs draw no outline), `body.shadow` (variants `separate_window`, `full_size`: a dialog in its own
    window or filling it), `icon.icon`, `icon.glyph` (each preset states the one its platform draws the icon with,
    a theme or symbol icon or a font glyph, and the other `Absent`).
  - values: `message_wrap` (all but `kde-breeze`), `button_icons` (all but `kde-breeze`), `natural_size` (all but
    `adwaita`, `kde-breeze`), `keyboard_avoidance` (`adwaita`, `kde-breeze`, `macos-*`), `cancel_platter_gap_px`
    (all but `ios-*`; `ios-26`). `breakpoints`, `presentations`, `standard_button_icons` and `type_icons` are lists:
    empty, not absent, where the platform has none.
- **State-model source:** `windows-11`: ContentDialog's visual state groups (`DialogShowingStates`,
  `DialogSizingStates`, `ButtonsVisibilityStates`, `DefaultButtonStates`; `ContentDialog_themeresources.xaml`) and
  Button's CommonStates; `adwaita`: `_dialogs.scss`, `_message-dialog.scss` (floating, bottom sheet, `.alert`,
  `.short`, `.narrow`, HC) and the response appearances (`adw-alert-dialog.c`); `kde-breeze`: Kirigami's
  `Dialog.qml`, `PromptDialog.qml`, `MenuDialog.qml`, `SearchDialog.qml`, qqc2's `Dialog.qml` and
  `DialogButtonBox.qml`, KMessageBox, kirigami-addons' `FormCardDialog.qml`, `MessageDialog.qml`,
  `DialogRoundedBackground.qml`; `material-3`: the dialog and full-screen dialog tokens (`_md-comp-dialog.scss`,
  `_md-comp-full-screen-dialog.scss`, action states); `macos-*`: `NSAlert.Style`, sheet against app-modal
  presentation; `ios-*`: `UIAlertController.Style`, `UIAlertAction.Style`, `preferredAction`.
- **Today's fields:** `background_color` → `body.fill`; `body_font` → `message.text.font`; `min_width_px`,
  `max_width_px`, `min_height_px`, `max_height_px` → `body.min_width` …; `button_gap_px` →
  `values.action_layout.row_gap_px`; `button_order` → `values.action_layout.row_order`; `entrance` →
  `values.presentations` (event `Appear`); `title_font` → `title.text.font`; `icon_size_px` → `icon.icon.size`;
  `border.*` → `body.stroke`, `body.corners`, `body.padding`, `body.shadow`.
- **Notes:**
  - Nested controls: the responses are `button` elements in `context dialog.action_area` (GNOME's response buttons,
    iOS's alert actions, Material's text actions, ContentDialog's buttons, KDE's button box); the header's close button
    is `button @ variant close context dialog.header` (Kirigami's, its icon changing on hover, Q2.58†, K-KA-17†;
    Material's full-screen dialog, MAT-N26); inputs are `input @ context dialog.body variant alert` (DL-9).
  - Group 7: `@action_sheet` (C-11): the actions' box is `body`, the separate cancel box `cancel_platter`, the
    actions `button` elements there, the hairlines `separator`, with `title` and `message`; on iPad it is a popover (a
    context, not a part).
  - Group 6: `full_size` covers Kirigami's dialog layer (Q2.58†) and Material's full-screen dialog (MAT-N26):
    `header`, `title`, the close `button`, `action_area`, the line under the header as `header.stroke`. Kirigami chooses
    an inset dialog, a full-window layer or a separate window by width and form factor: `breakpoints` sets
    `full_size`, and `separate_window` is the application's choice of a native-window dialog (Q2.58†).
    GtkDialog's action area: `action_area.padding`, `values.action_layout.row_gap_px` (GN-2.57†).
  - kirigami-addons: `@ka_form_card_dialog` with `@has_cancel` (K-KA-17†); `@ka_message_dialog`, its icon above the
    text under `narrow` (K-KA-19†); `@ka_list_dialog` (K-KA-12†).

### popover

*From group 1's catalogue.*

- **Kind:** typed widget: GtkPopover, qqc2 Popup, Breeze popups, NSPopover, UIPopoverPresentationController, WinUI
  Flyout, Material's menu container used as a popover surface.
- **Available:** every preset.
- **Parts:**
  - `body` — the root: `fill` (colour or material: acrylic, `.popover`, glass), `stroke`, `corners`, `shadow` (layers
    or an elevation entry; Material's surface tint is an appended fill layer, R8), `padding`, `min_width`,
    `max_width`, `min_height`, `max_height` (`Auto` where the platform sizes by content), `text` (today's `font`),
    `bindings` (`Surface(Popover)`; on `ios-*` the `Popover` set, whose background roles the preset states as the
    elevated values, with a `Binding::Role` from each keyed background role to its elevated twin, as `dialog.body`:
    R2.3, R5.2, D-14), `opacity` (kirigami-addons' translucent media popup, K-KA-35†), `transitions`.
  - `arrow` — the tail: `min_width` (base), `min_height` (height), `corners` (tip rounding), `fill`, `stroke`.
  - `dismiss_overlay` — WinUI's light-dismiss overlay: `fill`.
- **Widget values:** `placement: Placement` (gap to the anchor, screen-edge margin, arrow side; Q1.16, MAC-I06);
  `has_arrow: Value<bool>`; `default_size: Axes<Value<Px>>` (UIKit's size when the application states
  none); `full_size_content: Value<bool>` (macOS); `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear`.
- **Variants:** typed severity `warning`, `error` (GtkSourceView's informative popups in `context code_editor`, their
  fills from the status colours, GN-SF13†); keyed `adwaita` `@osd`; `macos-*` `@detached`; `kde-breeze` `@ka_dialog`
  (the surface of kirigami-addons' popups, K-KA-18†). GNOME's touch-selection bubble is `edit_menu`; its text
  magnifier (a
  `.magnifier` popover in GTK,
  GN-1.184) is `input.magnifier`.
- **Sizes and densities:** none.
- **Flags:** none (a popover's content carries the states).
- **Exclusive groups:** none (the popover holds no interaction state); precedence: state models (S17).
- **may_be_absent:**
  - parts: `arrow` (`kde-breeze`, `windows-11`, `material-3`), `dismiss_overlay` (all but `windows-11`).
  - fields: `body.shadow` (`cond not_composited`), `body.stroke` (`material-3`, `ios-*`: the platform outlines no
    popover), `arrow.stroke` (each preset whose platform draws none, which the value phase names from the facts).
  - values: `full_size_content` (all but `macos-*`), `default_size` (all but `ios-*`).
- **State-model source:** `adwaita`: `_popovers.scss` (`.menu`, `.osd`, `.touch-selection`,
  `:backdrop`, HC; `.solid-csd` ancestors); `kde-breeze`: Breeze's `renderMenuFrame` branches (alpha or not), qqc2's
  `Popup.qml`, kirigami-addons' `DialogRoundedBackground.qml`; `macos-*`: `NSPopover.Behavior`, `animates`,
  detached; `ios-*`: `permittedArrowDirections`, `popoverLayoutMargins`; `windows-11`: FlyoutPresenter (no visual
  states beyond its theme resources); `material-3`: the menu container tokens.
- **Today's fields:** `background_color` → `body.fill`; `font` → `body.text.font`; `border.*` → `body.stroke`,
  `body.corners`, `body.padding`, `body.shadow`.
- **Notes:** Group 3's gap to the anchor is the `gap` of `placement`'s parameters (Q1.16, MAC-I06).

### tooltip

*From group 1's catalogue.*

- **Kind:** typed widget: GtkTooltip, QToolTip, qqc2 ToolTip, KToolTipWidget, NSToolTip and expansion tool tips,
  WinUI ToolTip, Material's plain and rich tooltips.
- **Available:** every preset.
- **Parts:**
  - `body` — the root: `fill` (colour or material: in-app acrylic, `.toolTip`), `stroke` (KDE's border mixed from
    background and text), `corners`, `shadow`, `padding`, `min_height` (Material), `max_width` (pixels, or
    characters as `mul(element.char_width, <count>)`), `bindings` (`Surface(Tooltip)`, Kirigami's `Complementary`).
  - `label` — `text` (font, tracking, line height, wrap).
  - `title` — the rich tooltip's subhead: `text`.
  - `icon` — the tooltip's icon (GNOME): `icon`.
  - `accelerator` — the shortcut shown in a tooltip (W3-26): `text`.
  - `arrow` — Material's caret: `min_width`, `min_height`, `glyph`, `corners`, `fill` (all listed; a preset states
    the shape its platform draws, the glyph or the cornered fill, and the other `Absent` where `may_be_absent`
    allows).
- **Widget values:** `show_delay_ms` (today; Kirigami's under `cond flavor:kirigami`, Q1.136, Q1.65, Q1.82),
  `reshow_delay_ms` (browse mode), `browse_timeout_ms`, `hide_delay_ms` (after the pointer leaves, MAT-X25),
  `keyboard_show_delay_ms: Value<Ms>`; `show_on_press: Value<bool>` (tablet mode, touch; with `show_delay_ms` `=
  token.behavior.long_press` under that condition, Q3.28); `display_time: Value<DisplayTime>`, `DisplayTime { base_ms: Value<Ms>,
  per_char_ms: Value<Ms>, free_chars: Value<u32> }` (`Absent` for "never times out while hovered");
  `icon_text_gap_px: Value<Px>`; `placement: Placement`; `presentations: Vec<PresentationTransition>`: events
  `Appear`, `Disappear`.
- **Variants:** typed `rich` (Material's rich tooltip, KDE's KToolTipWidget); keyed `macos-*` `@expansion`.
- **Sizes and densities:** none.
- **Flags:** none (a tooltip has no interaction state; the rich tooltip's actions are `button` elements in `context
  tooltip.body variant rich`).
- **Exclusive groups:** none (the tooltip holds no interaction state); precedence: state models (S17).
- **may_be_absent:**
  - parts: `title` (all but `material-3` and `kde-breeze`'s `rich`), `icon` (all but `adwaita`), `arrow` (all but
    `material-3`), `accelerator` (all but `windows-11`).
  - fields: `body.min_height` (all but `material-3`), `body.max_width` (`kde-breeze`'s plain-text tooltip), `body.stroke` (`material-3`, `ios-*`: the platform
    outlines no
    tooltip), `body.shadow` (`cond not_composited`), `arrow.glyph`, `arrow.corners` (`material-3` states the one its
    caret is drawn with).
  - values: `display_time`, `reshow_delay_ms`, `browse_timeout_ms`, `keyboard_show_delay_ms` (where the platform states
    none: the value phase names the presets from the facts).
- **State-model source:** `adwaita`: `_tooltip.scss` and `gtktooltip.c` (browse mode); `kde-breeze`: Breeze's
  `PE_PanelTipLabel`, qtbase `qtooltip.cpp`, qqc2's `ToolTip.qml`; `macos-*`: tool-tip and expansion tool-tip
  behaviour (measured); `windows-11`: ToolTip's visual states and `ToolTipService` timings; `material-3`: the plain
  and rich tooltip tokens and Compose `TooltipBox`.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font`; `max_width_px` →
  `body.max_width`; `show_delay_ms` → `values.show_delay_ms`; `border.*` → `body.stroke`, `body.corners`,
  `body.padding`, `body.shadow`.
- **Notes:**
  - Groups 2 and 4: Kirigami's tooltips take `show_delay_ms` under `cond flavor:kirigami` (Q1.136, Q1.65, Q1.82),
    also in `context sidebar.pane variant compact`.
  - Group 3: the shortcut tooltip of an action field is this widget in `context input` (Q1.18).
  - `body.max_width` in characters names `element.char_width`, which the part declares as its data (S8).

### menu

*From group 1's catalogue.*

- **Kind:** typed widget: popover menus and context menus (GtkPopoverMenu, QMenu, qqc2 Menu, kirigami-addons'
  ConvergentContextMenu, NSMenu, UIMenu, MenuFlyout, Material's menu and Compose's text context menu), and the popup
  lists that are menus to the user (Nested controls): a combo box's popup (`context combo_box`), a field's
  suggestion list (`context autocomplete`), an edit menu's overflow (`context edit_menu`), a toolbar's overflow
  menu. The menu bar is `menu_bar`.
- **Available:** every preset.
- **Parts:**
  - `body` — the popup (the root): `fill` (colour, translucency, material: acrylic, KWin blur, glass), `stroke`,
    `corners` (K1.64: square without an alpha channel, joined to its menu bar or combo box), `shadow`, `padding` (the
    inset of items from the popup's edges), `min_width`, `max_width`, `min_height`, `max_height`, `bindings`,
    `transitions`.
  - `item` *(element)* — one row: `fill` (highlight; appended state layers), `stroke` (KDE's outline; its weak-focus
    line), `corners` (the highlight's own radius), `shadow` (GNOME's HC inset ring), `padding`, `margin` (the space
    round each item, WinUI's `MenuFlyoutItemMargin`, W1-045), `min_width`, `min_height`, `opacity`, `focus`,
    `transitions`. `item_label`, `item_subtitle`, `item_icon`, `item_trailing_icon`, `accelerator`, `check_indicator`,
    `submenu_arrow`, `split_chevron`, `split_divider`, `item_selection_indicator`, `found_item_pointer` belong to
    `item`, nest in it and are queried with it (`split_chevron` as an element of its own inside it).
  - `item_label` — `text` (font, tracking, line height, fill per state).
  - `item_subtitle` — `text` (iOS, macOS 14.4+, Material's supporting text).
  - `item_icon` — the leading icon: `icon` (size, fill per state, KDE's selected-mode effect), `opacity`.
  - `item_trailing_icon` — iOS's item image, Material's trailing icon: `icon`.
  - `accelerator` — the shortcut text: `text`, `opacity`, `padding` (W3-26).
  - `check_indicator` — the check, radio or mixed mark and its column, on every preset (Nested controls: it takes
    the item's states): `glyph` (the mark), `fill`, `stroke`, `corners`, `shadow` (the box or ring behind the mark:
    GNOME's radio ring; Breeze draws it with its check box's and radio button's look, K1.69, Q1.31), `min_width`,
    `min_height`, `padding`, `margin` (GNOME's margins, one negative, GN-1.73).
  - `submenu_arrow` — `glyph`, `opacity`, `min_width` (its box).
  - `separator` — the separator item: `stroke` (the line: its width and paint, today's `separator_color`; the top
    edge), `padding` (the line's insets), `min_height` (the item's height).
  - `group_separator` — iOS's thick separator between inline sections: `fill`, `min_height`.
  - `group` — Material Expressive's item group box: `fill`, `corners`, `padding`.
  - `section_header` — KDE's section item, macOS's section header, iOS's menu title, Material Expressive's section
    label: `text`, `stroke` (KDE's line after the text), `padding`, `min_height`.
  - `submenu_header` — GNOME's sliding-submenu header: `glyph` (back arrow), `text`, `padding`.
  - `scroll_arrow` *(element)* — the scroll areas of a menu taller than its space: `glyph`, `fill`, `min_height`.
  - `split_chevron` *(element)*, in `item` — WinUI's split menu item (`@split`, W1-051): the chevron part pressed apart from
    the item, a piece of the item, not a `split_button`: `fill` (its own hover and pressed highlight), `glyph` (the
    chevron), `min_width` (the chevron part's box).
  - `split_divider` — the line between the item and its chevron part (W1-051): `stroke` (the divider's brush and
    width; the start edge), `min_height` (its length).
  - `item_selection_indicator` — WinUI's selection pill on the selected item of a combo box's list (W1-135), in
    `context combo_box`: `fill`, `corners`, `min_width`, `min_height`, `transform` (the scale the pill grows from),
    `transitions` (its scale animation).
  - `found_item_pointer` — the pointer macOS draws at a menu item the Help search found (MAC-N33†): `glyph`, `fill`,
    `transitions`.
  - `dismiss_overlay` — WinUI's light-dismiss overlay: `fill`.
  - `arrow` — the tail of a popover menu drawn with one (GTK's popover menus; arrowless where `has_arrow` is false:
    a back button's navigation menu, a drop-down's list, the text context menu, the window menu, GN-1.278†,
    GN-P4-2†, GN-P4-8†, GN-P3-3†): `min_width`, `min_height`, `corners`, `fill`, `stroke`, as `popover.arrow`.
- **Widget values:** `has_arrow: Value<bool>` (GN-1.278†, GN-P4-2†, GN-P4-8†, GN-P3-3†); `submenu_open_delay_ms`
  (W3-41, MAT-X25, MAC-X12), `submenu_close_delay_ms: Value<Ms>`;
  `submenu_switch: Value<SubmenuSwitch>`, `SubmenuSwitch { Immediate, TimerRestartedOnMotion, CloseTimer,
  SafeTriangle }` (GNOME's timer restarted on motion, KDE's and Windows' close timeout, macOS's triangle);
  `submenu_presentation: Value<SubmenuPresentation>`, `SubmenuPresentation { Cascade, Slide, Inline }`;
  `submenu_placement: Placement`; `placement: Placement` (a menu the application opens at an anchor or the pointer,
  a context menu; a nested menu is placed by its host's value, Nested controls); `check_indicator_side: Value<Side>`
  (the check before or
  after the label: GNOME's combo-box list puts it after, GN-1.234); `presentations: Vec<PresentationTransition>`
  (queried with the menu): events `Appear`, `Disappear`, keyed `@submenu_push` and `@submenu_pop` (a sliding
  submenu entering and leaving, as `sidebar`'s), `@submenu_expand` (an inline submenu opening in place); `show_item_icons: Value<ItemIcons>`, `ItemIcons {
  Always, Never, OnlyWithoutLabel }`; `check_column`, `icon_column: Value<ColumnReservation>`, `ColumnReservation {
  Always, WhenAnyItemNeeds, Never }`; `check_column_width_px`, `icon_column_width_px`, `icon_text_gap_px` (today),
  `accelerator_gap_px`, `indent_step_px`, `group_gap_px`, `icon_row_gap_px: Value<Px>`; `accelerator_alignment:
  Value<Alignment>`; `scrollable: Value<bool>`; `item_order: Value<ItemOrder>`, `ItemOrder { Fixed, NearestFirst,
  Automatic }` (UIKit's element order); `palette_columns: Value<u32>` (per size); `activation_flash: Value<u32>` (the
  number of times the chosen item flashes, MAC-X12).
- **Variants:**
  - on `item`, typed: `check`, `radio`, `destructive` (iOS, macOS's SwiftUI destructive item), `palette` (macOS's
    palette menu, iOS's `displayAsPalette`), `icon_only` (GNOME's iconic sections, Material Expressive's icon-only
    menus), `circular` (GNOME's circular iconic items); keyed `windows-11` `@split` (WinUI's split menu item, W1-051).
  - on the menu, typed `horizontal` (Material Expressive's horizontal menu; the orientation many widgets share); keyed:
    `material-3` `@vibrant`, `@text_context` (Compose foundation's own text
    context menu); `ios-*` `@compact` (the context-menu appearance); `kde-breeze` `@ka_bottom_drawer`, `@ka_dialog`
    (ConvergentContextMenu's `BottomDrawer` and `Dialog` modes, K-KA-31†).
- **Sizes and densities:** `ios-*`: the element sizes on `item`: `small`, `regular` (medium), `large`. `macos-*`: none
  of its own: a pop-up button's menu follows the button's control size by rules on the ancestor's size,
  `@ context combo_box.body[size s]` (`ContextMatch.sizes`, MAC-MN15, ADD-A84). No density.
- **Flags:** shared `placed_above`, `placed_below` (`kde-breeze`, `windows-11`, on the menu, set by the connector from
  the placement result: Breeze joins the popup's edge to its menu bar or combo box, K1.64; WinUI squares the
  suggestion list's attached corners, W2-15); keyed none; state flags used: hover,
  pressed, selected (keyboard selection), checked, indeterminate, open (submenu shown), disabled, focus_visible,
  opened_by_touch (on the menu).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (MenuFlyoutItem's CommonStates;
  CheckStates is `checked`, SubMenuOpened is `open`, PaddingSizeStates is `opened_by_touch`, none of them a group);
  `material-3`: `[dragged, pressed, focus_visible, hover]` on `item` (state layers); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:**
  - parts: `item_subtitle` (`adwaita`, `kde-breeze`, `windows-11`), `item_trailing_icon` (all but `ios-*`,
    `material-3`), `group_separator` (all but `ios-*`), `group` (all but `material-3` flavour `expressive`),
    `submenu_header` (all but `adwaita`), `scroll_arrow` (`adwaita`, `ios-*`, `material-3`, `windows-11`),
    `split_chevron`, `split_divider`, `item_selection_indicator`, `dismiss_overlay` (all but `windows-11`), `arrow`
    (all but `adwaita`; there too where `has_arrow` is false),
    `found_item_pointer` (all but `macos-*`).
  - fields: `item.stroke` (`macos-*`, `ios-*`, `material-3`, `windows-11`), `item.shadow` (all but `adwaita`: GNOME's
    high-contrast inset ring), `check_indicator.margin`, `check_indicator.fill`, `check_indicator.stroke`,
    `check_indicator.corners`, `check_indicator.shadow` (where the platform draws the mark alone: all but `adwaita`,
    `kde-breeze`), `body.shadow` (`cond not_composited`; Breeze's menus without an alpha channel, K1.64), `body.stroke`
    (`material-3`, `ios-*`: the platform outlines no menu); `body.fill`, `body.stroke`, `body.shadow` (`kde-breeze`
    under `@ka_bottom_drawer` and `@ka_dialog`, by rule: the `sheet` or `dialog` in the menu's context draws the
    box, and the menu element is only its actions' ancestor, K-KA-31†).
  - values: `has_arrow` (all but `adwaita`), `palette_columns` (all but `macos-*`, `ios-*`), `item_order` (all but
    `ios-*`), `indent_step_px`,
    `activation_flash` (all but `macos-*`).
- **State-model source:** `windows-11`: MenuFlyoutItem's, ToggleMenuFlyoutItem's, RadioMenuFlyoutItem's and
  MenuFlyoutSubItem's visual state groups (CommonStates, CheckStates, PaddingSizeStates, SubMenuOpened;
  `MenuFlyout_themeresources.xaml`), ComboBoxItem's in `context combo_box` (`ComboBox_themeresources.xaml`) and the
  suggestion list's ListViewItem states in `context autocomplete`; `adwaita`: `_menus.scss` selectors (`:hover`,
  `:selected`, `:active`,
  `:disabled`, `:checked`, HC) and `gtkmodelbutton.c`; `kde-breeze`: Breeze's `drawMenuItemControl` branches
  (`st:5982-6185`), qqc2's `MenuItem.qml`, kirigami-addons' `ConvergentContextMenu.qml`; `material-3`:
  `_md-comp-menu.scss` and the Expressive menu tokens (rest, hover, focus, pressed, selected, disabled); `macos-*`:
  `NSMenuItem.state` (on, off, mixed), `isEnabled`, highlight, `NSMenu.presentationStyle`; `ios-*`:
  `UIMenuElement.State`, `UIMenuElement.Attributes`, `UIMenu.ElementSize`, `UIMenu.Options`.
- **Today's fields:** `background_color` → `body.fill`; `separator_color` → `separator.stroke` paint; `font` →
  `item_label.text.font`; `row_height_px` → `item.min_height`; `icon_text_gap_px` → `values.icon_text_gap_px`;
  `icon_size_px` → `item_icon.icon.size`; `hover_background` → `item.fill @ {hover}`; `hover_text_color` →
  `item_label.text.fill @ {hover}`; `disabled_text_color` → `item_label.text.fill @ {disabled}`; `border.color`,
  `border.line_width_px` → `body.stroke`; `border.corner_radius_px` → `body.corners`; `border.shadow_enabled` →
  `body.shadow`; `border.padding_*` → `item.padding` (K1.63's reading of today's field).
- **Notes:**
  - macOS's Help-menu search field (MAC-N33†) is the `search_field` widget in `context menu.body`; the pointer at a
    found item is `found_item_pointer`.
  - macOS's item badge is the `badge` widget in `context menu.item`.
  - A toolbar's overflow menu is this widget in `context button variant overflow` (the overflow button in
    `context toolbar.bar`), placed by the overflow button's `button.values.menu_placement` (W1-102); the overflow
    check glyph of a checked toggle item is `check_indicator` there (W1-100).
  - A combo box's popup is this widget in `context combo_box` (GNOME's menu-style popover, the qqc2 combo's Menu,
    macOS's pop-up menu, iOS's menu, Material's exposed drop-down menu, WinUI's and Qt Widgets' popup lists): its rows
    `item`, the selected row's mark `check_indicator` (after the label on GNOME: `check_indicator_side`, GN-1.234),
    WinUI's pill `item_selection_indicator`, its open and close `values.presentations` (W1-136, Q1.125); it is
    placed by `combo_box.values.popup_placement` (W1-136, MAT-P3-9, K1.245, K-P3-5†, MAC-P4-8†).
  - A list in a popover menu is `list` in `context menu.body`: GTK's standalone list (the popover's own content,
    with the menu sections' margins) matches `context nearest {menu, layout} = menu`, a list inside a menu section
    `context nearest {menu, layout} = layout`, the section's box queried as a `layout` element (GN-1.79).
  - Suggestion lists under a field are this widget in `context autocomplete`, placed by
    `autocomplete.values.placement` (MAC-P3-7†, W2-15, GN-2.56†, K2.29†, MAT-N27, C-38†); a date picker's quick items
    (Today, Next Week) are `item` elements in `context date_picker`
    (K2.23†).
  - kirigami-addons' ConvergentContextMenu (K-KA-31†): the desktop look (qqc2 Menu, Q1.23–Q1.32) is the base; the
    connector sets `@ka_bottom_drawer` (the default under `cond mobile_form_factor`) or `@ka_dialog` from the
    application's display mode, and draws the menu's actions as `form` rows in a `sheet`, or in a `dialog`, in
    `context menu variant @ka_bottom_drawer` / `context menu variant @ka_dialog`; the menu's own box is then not
    drawn (`body.fill`, `body.stroke`, `body.shadow` `Absent` by rule under those variants), so one box is drawn.

## Buttons and toggles

### button

*From group 2's catalogue.*

- **Kind:** typed widget: all six platforms (push, tool, toggle and menu buttons; toolbar items, §2 of the merge:
  Qt's QToolButton, WinUI's AppBarButton and AppBarToggleButton, AppKit's toolbar items, iOS's bar button items), and
  every button nested in another widget (Nested controls: dialog responses, close, navigation, overflow, clear and
  reveal buttons, media controls, edit-menu commands).
- **Available:** every preset.
- **Parts:**
  - `body` — the bezel or container (the root): `fill` (state layers are appended paint layers, R3; an expression on
    `element.color` where the application tints the button: AppKit's `bezelColor`, a toolbar item's
    `backgroundTintColor`, MAC-B07, MAC-TL04, ADD-B16; the parts declare that data, `data = { body = ["color"],
    label = ["color"], icon = ["color"] }`, S8, S10), `stroke`, `corners`, `shadow` (layers or an elevation entry),
    `padding`, `margin` (a nested button's margins in its host, one negative: GNOME's tab and search-bar close buttons,
    GN-1.120, GN-2.21, the file chooser's eject button, GN-P3-22†; WinUI's tab close margin, W1-082), `min_width`,
    `min_height`, `max_width` (ContentDialog's button maximum width, W1-127; `Auto` where no maximum is set),
    `opacity` (today's `disabled_opacity`),
    `transform`, `focus`, `cursor` (`WhatsThis` on `help`, ADD-A59), `touch_target` (Material's
    minimum interactive size round small buttons and icon buttons, MAT-X22), `bindings` (KDE's tool-button colour
    set, Window when flat and Button when raised, Q1.9), `transitions`.
  - `label` — the title text: `text` (its fill an expression on `element.color` where the application tints the
    content, AppKit's `contentTintColor`, MAC-B07), `padding` (GTK's label padding beside an image or arrow, GN-1.27,
    GN-1.39), `transform` (the content shifted while a round button is pressed or checked, Q2.7).
  - `subtitle` — a second text line under the title (iOS `UIButton.Configuration.subtitle`, B-8; KDE's command link
    description, K2.7): `text`.
  - `icon` — the image: `icon` (size, fill per state, rendering mode, weight and scale for SF Symbols; its fill an
    expression on `element.color` under `contentTintColor`, MAC-B07), `glyph` (for system button types, B-12),
    `transitions` (state-triggered icon animations, `TransitionKind::Keyed` of group `Icon`: WinUI's AnimatedIcon
    states, W1-024, W3-28; GtkSvg's state transitions, GN-P3-19†; the SF Symbols content transition, B-16; ADD-A27,
    R25.12), `transform` (as the label's, Q2.7).
  - `menu_indicator` — the arrow or chevron of a button that opens a menu (GN-1.40, K1.27, B-18†, MAT-N8; KDE's
    inline small and large arrows, K1.155): `glyph`, `min_width`, `transform` (its rotation, the file chooser's
    server-list arrow turning while `open`, GN-P3-23†), `transitions` (the press animation; that rotation's).
- **Widget values** (`ButtonValues`):
  - `icon_text_gap_px: Value<Px>` — between icon and label (today's `icon_text_gap`; a toolbar's in `context
    toolbar.bar`, K1.154, MAC-TL02).
  - `icon_placement: Value<IconPlacement>`, `IconPlacement { Leading, Trailing, Top, Bottom }` — the image beside or
    above the title (B-9 `imagePlacement`, MAC-B09 `imagePosition`).
  - `label_position: Value<ButtonLabelPosition>` — which of icon and text a tool button shows, and how (K1.153 read
    per toolbar kind, W1-099, MAC-TL02, Q1.9, Q1.76), by rules in `context toolbar.bar`.
  - `subtitle_gap_px: Value<Px>` — iOS `titlePadding` (B-8).
  - `menu_indicator_gap_px: Value<Px>` — between label and indicator (K1.27 `Button_ItemSpacing`).
  - `menu_placement: Placement` — where the menu a `menu`, `overflow` or `back` button opens goes (Nested controls:
    the host's value; B-18†, K-P3-5†, GN-P4-2†, W1-102, P4-12†; its arrowless popover on GNOME, GN-1.278†).
  - `menu_trigger: Value<MenuTrigger>`, `MenuTrigger { Primary, LongPress, LongPressOrSecondaryClick }` — the
    touch-and-hold variant (B-18†); AdwBackButton's navigation menu opens on a long press or a secondary click
    (GN-1.278†).
  - `shows_icon: Value<bool>` — whether push buttons show icons (K1.28, read from `kdeglobals`).
  - `focus_visible_when` (S2.11's `FocusVisibleWhen`) — when focus becomes visible (K1.21, R17).
  - `repeat_delay_ms: Value<Ms>`, `repeat_interval_ms: Value<Ms>` — durations of an auto-repeating button (K-RL07†,
    W2-19, MAC-X12).
  - `back_title_fallback: Value<bool>` — UIKit shortens the back title, then replaces it by a generic one (C-2).
  - `back_display_mode: Value<BackDisplayMode>`, `BackDisplayMode { Default, Generic, Minimal }` — UIKit's
    `backButtonDisplayMode` (C-2).
  - `help_tooltip_on_click: Value<bool>` — KDE's ContextualHelpButton shows its tooltip on click (Q2.48).
  - `hit_extension: Sides<Value<Px>>` — margins that enlarge the hit area beyond the drawn button (K-KA-26†; a
    minimum size is `body.touch_target`).
  - `display_mode: Value<PasteDisplayMode>`, `PasteDisplayMode { Icon, Label, IconAndLabel }` — iOS's paste control
    (C-17).
  - `scaffold_placement: Placement` — where Material's scaffold puts the FAB (strategy
    `MaterialScaffoldSlot(Fab)`: end or centre, its spacing from the edge and above the bottom bar; `@ variant @fab`,
    MAT-X38†).
  - `presentations: Vec<PresentationTransition>` — events `Appear`, `Disappear` (a field's clear button fading in and
    out, `Instant` on WinUI: `button @ variant clear_field context input`, K1.46, W1-030), `@spring_load` (the flash
    or blink
    of a spring-loaded button while a drag hovers it, before it activates: MAC-X21, P3-11†).
- **Variants:**
  - typed (a meaning two or more platforms share): `primary` (accent-filled: GNOME `.suggested-action`, WinUI
    `AccentButtonStyle`, Material filled, iOS `filled`, macOS `tintProminence` primary; a toolbar's prominent item in
    `context toolbar.bar`, TB-4, MAC-TL04), `tinted` (tinted, not filled: iOS `tinted`, macOS
    `NSTintProminence.secondary`, Material tonal), `flat` (no bezel at rest: GNOME `.flat`, KDE `setFlat`/auto-raise
    and Kirigami `flat`, WinUI `SubtleButtonStyle`, Material text, iOS `plain`, macOS `isBordered = false`),
    `destructive` (GNOME `.destructive-action`, macOS `hasDestructiveAction`, iOS destructive role), `warning` (KDE's
    neutral highlight; GNOME's `.warning` elsewhere), `pill` (GNOME `.pill`, iOS capsule corner style, macOS 26
    capsule border shape), `circular` (GNOME `.circular`, KDE `_kde_round_button` and RoundButton, macOS circular
    bezel and circle border shape), `glass` (iOS 26, macOS 26), `help` (macOS's help bezel MAC-N10, KDE's
    ContextualHelpButton Q2.48), `back` (GNOME's AdwBackButton, WinUI's TitleBar back button, iOS's navigation-bar
    back button: GN-1.278†, W1-011, C-2), `toggle` (a button with an on state: GtkToggleButton, Qt checkable, WinUI
    `ToggleButton` and AppBarToggleButton, Material toggle, macOS `pushOnPushOff`, iOS
    `changesSelectionAsPrimaryAction`; Material draws its unselected toggle unlike a plain button, MAT-B12), `menu`
    (opens a menu: GtkMenuButton, Qt `HasMenu`/QML menu flag, iOS pull-down, WinUI DropDownButton, Material menu
    button, macOS's `NSComboButton` and menu toolbar item; MAC-N29†, W2-18, MAT-N8; AppKit's pull-down
    `NSPopUpButton` is a pop-up button control, `combo_box` `pull_down`), `tool` (Qt's QToolButton and qqc2 ToolButton, also outside
    toolbars, Q1.9; WinUI's AppBarButton and AppBarToggleButton, W1-099, W1-100; AppKit's toolbar bezel, MAC-TL02),
    `clear` (the clear glass style: iOS 26's `clearGlass`; the meaning `glass_container`'s `clear` shares, macOS 26),
    and the content variants `icon_only` and `with_icon` (GTK's `.image-button` and `.image-text-button`, Material's
    leading-icon padding and icon buttons MAT-N4, MAT-N5, C-29, AppKit's image-only padding; text only is the base).
    KDE's split tool button (`MenuButtonPopup`, K1.155) is the `split_button` widget.
  - typed identities of a button nested in another widget (Nested controls): `close` (a close or dismiss button:
    dialog headers K-KA-17† and MAT-N26, banners, toasts, sheets, teaching tips, a search bar GN-2.21, tabs, KDE's dock
    widget; iOS's system close type), `previous`, `next` (stepping through items or pages: calendars, carousels, page
    indicators, find bars, Pivot), `overflow` (the more or overflow button of a toolbar, an edit menu or a view
    switcher: K1.157, Q1.75, MAC-TL06, W1-102, MAT-TA8†, C-1, W2-23, W2-11), `add` (a new-tab button: WinUI's TabView,
    W1-083; macOS's window tabs, MAC-W09), `remove` (a chip's or token's remove button: KDE, Q2.24; Windows, W2-43†),
    `done` (iOS's done bar item, TB-4; macOS's find bar, MAC-N18), `clear_field` (the button that empties its field:
    I-2, W1-030, K1.46, MAT-I7; a key sequence's, K-RL02†), `reveal` (a password field's reveal toggle, `checked`
    while revealed: GN-2.23, W2-16, K1.48), `pane_toggle` (the pane or rail toggle: WinUI's TitleBar and NavigationView,
    W1-011, W1-095; Material's rail menu button, MAT-N13; GNOME's split-view sidebar toggle), and the media identities
    `play` (`checked` while playing), `volume`, `full_screen`, `frame_step` (GN-2.54†, W2-35, MAC-N30†, K-KA-35†),
    `minimize`, `maximize`, `restore` (the caption buttons of a window drawn inside the application, KDE's MDI
    sub-window, K2.11: the meanings `window.caption_button` shares), and `route` (the media output (AirPlay, cast)
    button: AppKit's `AVRoutePickerView`, iOS's `AVRoutePickerView`, WinUI's
    cast button; `route_active` while a route plays, MAC-P3-9†, C-42†, W-P4-18†).
  - keyed, `adwaita`: `@raised`, `@osd`, `@opaque`, `@card`, `@arrow_only` (`.arrow-button`), `@arrow_up`,
    `@arrow_start`, `@arrow_end`, `@arrow_none` (GtkMenuButton's direction; down is the base), `@tab_button`
    (AdwTabButton, its counter a `badge` in its context, GN-2.37), `@unpin` (a pinned thumbnail's unpin button in
    the tab overview, GN-2.35).
  - keyed, `kde-breeze`: `@highlighted` (QML `Button.highlighted`), `@inline_small`, `@inline_large` (the
    menu indicator's styles of a tool button, K1.155; its split style is `split_button`), `@command_link` (K2.7: its
    title `label`, its description `subtitle`, with `icon`; Windows' command link is Win32, R1.3), `@ka_floating`
    (kirigami-addons' FloatingButton, K-KA-26†), `@key_sequence` (KDE's key-sequence button in a shortcuts editor,
    K-RL02†, K-KA-43†), `@float` (a dock widget's float button, K2.10).
  - keyed, `macos-*`: the bezel styles `@flexible_push`, `@small_square`, `@accessory_bar`, `@accessory_bar_action`,
    `@badge`, `@recessed`, `@round_rect`, `@inline`, `@textured_rounded`, `@shadowless_square` (push is the base;
    the route button's `@bordered`, `isRoutePickerButtonBordered`, MAC-P3-9†;
    borderless is `flat`, circular `circular`, glass `glass`, help `help`, toolbar `tool`; the disclosure bezels are
    `expander`'s, MAC-B01, MAC-EX01); the 26 border shape `@rounded_rectangle` (capsule is `pill`, circle
    `circular`, automatic the base).
  - keyed, `ios-*`: `@gray`, `@borderless`, `@bordered` (with `tinted` / `primary` for `borderedTinted` /
    `borderedProminent`; the glass styles are `glass` and `clear`, with `primary` for the prominent ones), the corner
    styles `@corner_fixed`, `@corner_small`, `@corner_medium`, `@corner_large`, `@corner_dynamic` (capsule is `pill`),
    the role `@cancel` (iOS 26's further bar-item roles are an open item, Open items: added
    here when the facts state them); the system types `@info`, `@contact_add` (the
    close type is the typed `close`); the bar items' `@padding_removed` (TB-8; done is the typed `done`);
    `@indicator_none`, `@indicator_popup` (the pull-down indicator; automatic is the base); `@menu_on_hold`; the
    pointer effects `@pointer_highlight`, `@pointer_lift`, `@pointer_hover` (`UIPointerEffect`'s styles, X-PH-1);
    `@paste_control` (C-17).
  - keyed, `material-3`: `@elevated`, `@outlined` (container styles; icon buttons too, MAT-N4, MAT-N5), `@square`
    (the Expressive shape family, round the base; with `icon_only` the square icon button, MAT-N5, MAT-B10), `@narrow`,
    `@wide` (the Expressive icon button's narrow and wide leading and trailing spaces, default the base, under `cond
    flavor:expressive`, MAT-N5), `@fab`, `@extended_fab`, `@lowered` (MAT-N1, MAT-N2), and the FAB's colour styles
    `@secondary`, `@tertiary`, `@surface` (its primary style is the typed `primary`, MAT-N1), the Expressive container
    styles `@primary_container`, `@secondary_container`, `@tertiary_container` and the branded FAB `@branded`
    (MAT-N1).
  - keyed, `windows-11`: `@repeat` (RepeatButton, W2-19), `@query` (AutoSuggestBox's query button, W2-15),
    `@accept`, `@dismiss` (the date and time picker flyouts' accept and dismiss buttons, W2-04), `@first`, `@last`,
    `@page_number` (PagerControl's first- and last-page and number buttons, W2-21), `@zoom_out` (SemanticZoom's
    zoom-out button, W2-34).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`, `extra_large` (`NSControl.ControlSize`;
  MAC-B08, MAC-B09, MAC-B11†). `ios-*`: `mini`, `small`, `regular` (UIKit `medium`), `large`
  (`UIButton.Configuration.Size`, B-3). `material-3` flavour `expressive`: `extra_small`, `small`, `regular`
  (medium), `large`, `extra_large` (xlarge) (MAT-B9); by variant (`sizes_by_variant`, S10): `@fab` `small`,
  `regular` (the standard FAB), `large` (Expressive's medium FAB, declared under flavour `expressive`), `extra_large`
  (the large FAB) (MAT-N1); `@extended_fab` `small`, `regular` (medium), `large` (MAT-N2). Others: none. Which size an element that states none has is declared (ADD-A60). Densities: none from the
  button rows; the cross-widget density rows decide (`windows-11` W3-39, `macos-*` MAC-X17, `material-3` MAT-X22).
- **Flags:** shared `route_active` (on `route`: macOS, iOS); keyed `ios-*` `@loading` (B-10), `kde-breeze`
  `@recording` (the key-sequence button held pressed while it records, K-RL02†, K-KA-43†); state flags used: hover,
  pressed, focused, focus_visible, disabled, checked, indeterminate (WinUI's toggle), selected (a selected bar item,
  P3-3†, MAC-TL11†), open, default, drop_target (a spring-loaded button under a drag, P3-11†, MAC-X21).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (the CommonStates of the Button, ToggleButton,
  AppBarButton, AppBarToggleButton, accent and subtle templates; their Checked* and Indeterminate* states name the
  check state with them, so `checked` and `indeterminate` combine); `material-3`: `[dragged, pressed, focus_visible,
  hover]` (state layers); others none (states combine: libadwaita's CSS and Breeze's branches combine flags; K1.29's
  "pressed wins" is a rule order, not a group); precedence: state models (S17).
- **may_be_absent:**
  - parts: `subtitle` (all but `ios-*`, `kde-breeze`), `icon` (`ios-*` under `@loading`: a `spinner` element in
    `context button` in its place, B-10), `menu_indicator` (every preset: stated in the base, absent by a
    rule `@ !variants {menu,@inline_small,@inline_large}`).
  - fields: `body.focus` (where focus is drawn elsewhere: `macos-*` on `flat`, `ios-*`), `body.shadow`,
    `body.touch_target` (all but `material-3`), `body.cursor` (where the platform sets none), `icon.glyph` (all but
    `ios-*`: its system button types, B-12).
  - values: `menu_trigger` (all but `ios-*`, `adwaita`), `subtitle_gap_px`, `back_title_fallback`,
    `back_display_mode`, `display_mode` (all but `ios-*`), `scaffold_placement` (all but `material-3`), `shows_icon`,
    `help_tooltip_on_click`, `hit_extension` (all but `kde-breeze`), `repeat_delay_ms`,
    `repeat_interval_ms` (`adwaita`, `material-3`, `ios-*`: no auto-repeating button), `label_position` (`adwaita`,
    `ios-*`, `material-3`: no tool button with a label choice). `presentations` is a list: empty, not absent, where
    the platform has none.
- **State-model source:**
  - `adwaita`: libadwaita's selectors in `_buttons.scss`, `_toolbars.scss` (flat by container), `_linked.scss`,
    `_common.scss` (needs-attention); GTK's `:hover`, `:active`, `:checked`, `:disabled`, `:drop(active)`,
    `.has-open-popup`, `:focus-visible`, `prefers-contrast: more`.
  - `kde-breeze`: Breeze's `Helper::renderButtonFrame` branches (`hp:647-741`) and `Style` `CE_PushButton`,
    `CC_ToolButton` (`st:4516-4543`, `:5327-5380`, `st:4577-4624` for the auto-raise tool button, K1.152); flavour
    `kirigami`: qqc2-desktop-style `Button.qml`, `ToolButton.qml` state mapping (Q1.8, Q1.9) and kirigami-addons'
    `FloatingButton.qml` (K-KA-26†).
  - `macos-*`: `NSControl.StateValue` (on, off, mixed), highlighted, enabled × `NSButton.BezelStyle` ×
    `NSControl.ControlSize` × `NSControl.BorderShape` (26) × key window, main, application active × appearance;
    `NSToolbarItem` (bordered, style, enabled, selected); the probe renders each.
  - `ios-*`: `UIControl.State` (normal, highlighted, disabled, selected, focused) × `UIButton.Configuration` style ×
    `buttonSize` × `cornerStyle` × `UIButton.ButtonType`; `UIBarButtonItem.Style`, `isSelected`; measured.
  - `windows-11`: the VisualStateGroups of the Button, ToggleButton (with Checked*/Indeterminate*), AccentButtonStyle,
    SubtleButtonStyle, AppBarButton and AppBarToggleButton templates (`Button_themeresources.xaml`,
    `ToggleButton_themeresources.xaml`, `AppBarButton_themeresources.xaml`, `AppBarToggleButton_themeresources.xaml`).
  - `material-3`: the tokens `latest/_md-comp-button*.scss` (enabled, hover, focus, pressed, disabled; selected and
    unselected for toggles) per variant and, Expressive, per size; the FAB and icon-button tokens.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font` (its `color` →
  `label.text.fill`); `min_width` → `body.min_width`; `min_height` → `body.min_height`; `icon_text_gap` →
  `values.icon_text_gap_px`; `primary_background` → `body.fill @ variant primary` (on `macos-*`: `@ {default}`);
  `primary_text_color` → `label.text.fill @ variant primary` (and `icon.icon.fill`); `disabled_opacity` →
  `body.opacity @ {disabled}`; `hover_background` → `body.fill @ {hover}`; `hover_text_color` → `label.text.fill @
  {hover}`; `active_background` → `body.fill @ {pressed}`; `active_text_color` → `label.text.fill @ {pressed}`;
  `disabled_background` → `body.fill @ {disabled}`; `disabled_text_color` → `label.text.fill @ {disabled}`;
  `checked_background` → `body.fill @ {checked}`; `checked_text_color` → `label.text.fill @ {checked}`;
  `border.color` / `border.line_width` → `body.stroke` (each edge's paint and width); `border.corner_radius` →
  `body.corners`; `border.padding_*` → `body.padding`; `border.shadow_enabled` → `body.shadow` (layers or none; the
  flag goes, R8).
- **Notes:**
  - Toolbar items are `button` elements in `context toolbar.bar` (a toggle item `toggle`, a menu item `menu`, a split
    item a `split_button`); their states are `button` rules there (K1.152, W1-099, W1-100, MAC-TL02, TB-6, P3-3†,
    MAC-TL11†); the toolbar's item gaps stay `toolbar`'s values. GNOME's toolbar children are flat by the rule in
    `context nearest {toolbar.bar, popover} = toolbar.bar` (GN-1.151).
  - Nested buttons (Nested controls) are this widget in their host's context, their identity a typed variant above:
    `back` and `pane_toggle` in `context window.title_bar`, `context toolbar.bar` or `context sidebar.header`
    (GN-1.278†, W1-011, W1-095, C-2; the back label's push and pop is the toolbar's event `@title_change`); iOS's
    loading indicator (B-10) is a `spinner` element in `context button {@loading}`; a field's
    `clear_field` and `reveal` in `context input`; `close` in a dialog's, banner's, toast's, sheet's, teaching tip's or
    tab's context; the responses in `context dialog.action_area`.
  - Group 6: `circular` covers KDE's RoundButton (Q2.4–Q2.8: a gloss as an appended linear paint layer, the content
    shifted under `{pressed}` / `{checked}` as a transform, the Selection binding under `{focused}`); `help` takes the
    cursor `WhatsThis` (ADD-A59); Material's icon buttons are `icon_only` with `primary`, `tinted` and `@outlined`
    (MAT-N4, MAT-N5, C-29).
  - Material's FAB in the floating toolbar is `button @ variant @fab context any toolbar variant floating`, with its
    sizes (MAT-N25). Material's minimum interactive size round small buttons and icon buttons is
    `body.touch_target` (MAT-X22).
  - Group 7: `@paste_control` (C-17) with `display_mode`; `@repeat` (W2-19) with `repeat_delay`, `repeat_interval`;
    `@command_link` (K2.7). A one-platform kind that is a button to the user and is drawn with the button's parts is
    a keyed variant of `button` (these three); one with parts a button lacks is a keyed component (KDE's DelayButton,
    its progress ring: `component.delay_button`).
  - libadwaita's needs-attention dot (`_buttons.scss` `.needs-attention`, GN-1.41) is drawn on the buttons of a stack
    or view switcher, which are `view_switcher.item` elements: it is the `badge` widget, variant `attention`, in
    `context view_switcher.item` (GN-1.41, GN-2.33; Nested controls: one structure for the dot); `button` has no dot
    part.
  - The media output button (`route`, rename row 82) is the same element in `context media_controls` and alone; its
    glyph's tint is `icon.icon.fill` (`routePickerButtonColor`, `routePickerButtonActiveColor` under
    `{route_active}`), its bordered style `@bordered` (macOS).
  - GNOME's linked buttons select on `context any layout variant @linked` with position rules (GN-1.42; position
    among siblings of any kind is CONTRA-3).
  - kirigami-addons: `@ka_floating` with `hit_extension` (K-KA-26†).

### link

*From group 2's catalogue.*

- **Kind:** typed widget: GNOME, KDE, macOS, iOS, Windows (standalone links and links in running text); Material
  draws links only in running text (Compose's `TextLinkStyles` in an `AnnotatedString`, MAT-P3-20†).
- **Available:** every preset; `material-3` only as variant `inline` (`requires_variants`, S10).
- **Parts:**
  - `body` — the root, the box round the text (a link button's background and borders, W1-144; iOS's pressed
    highlight box, LK-1; GNOME's link-styled button, GN-1.266): `fill`, `stroke`, `corners`, `padding`, `focus`
    (MAC-LK03's ring under Full Keyboard Access), `cursor`, `transitions`.
  - `text` — the link text: `text` (its text style carries the underline as `decoration`), `focus` (where the
    platform rings the text: KDE draws focus by weight instead, Q1.135), `cursor` (the pointing hand over the text,
    MAC-LK02, MAC-X20).
  - `icon` — the external-link icon after the text (Kirigami `UrlButton`, Q1.136): `icon`.
- **Widget values:** `icon_text_gap_px: Value<Px>` (Q1.136); `shows_url_tooltip: Value<bool>` (Q1.136; the tooltip's
  look and delay are the `tooltip` widget's); `focus_visible_when` (S2.11's `FocusVisibleWhen`; macOS's ring under
  Full Keyboard Access is a rule `@ cond full_keyboard_access`, MAC-LK03).
- **Variants:** typed `inline` — a link inside running text (GNOME label links, WinUI `Hyperlink`, AppKit
  `NSTextView` links and data detectors TA-5, iOS text-view links, KDE rich-text labels, SwiftUI Markdown links
  MAC-SF04† and SF-4†, Compose's HTML links MAT-P3-20†); the base is a standalone link control (GtkLinkButton, WinUI
  `HyperlinkButton`, Kirigami `LinkButton`/`UrlButton`, `KUrlLabel`). Keyed `kde-breeze` `@url` (UrlButton),
  `@url_label` (KUrlLabel), `@float` (KUrlLabel's underline-on-hover mode).
- **Sizes and densities:** none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, visited.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (HyperlinkButton's CommonStates); others none
  (states combine); precedence: state models (S17).
- **may_be_absent:**
  - parts: `icon` (all but `kde-breeze`).
  - fields: `body.fill`, `body.stroke`, `body.focus` (where the platform draws the text only),
    `text.text.decoration` (no underline in some states, W1-143), `text.focus` (KDE draws focus by weight, Q1.135).
  - values: `shows_url_tooltip` (all but `kde-breeze`).
- **State-model source:** `adwaita`: `_links.scss` (`:visited`, `:active`, `:disabled`, `.osd &`, the link-styled
  button); `kde-breeze`: Kirigami `LinkButton.qml`, `UrlButton.qml`, KWidgetsAddons `kurllabel.cpp`; `macos-*`:
  `NSTextView.linkTextAttributes` and the focus ring under Full Keyboard Access; `ios-*`: the text view's link
  interaction (pressed), measured; `windows-11`: HyperlinkButton's template states and `Hyperlink.cpp`'s underline per
  state; `material-3`: Compose's `TextLinkStyles` (style, focused, hovered, pressed).
- **Today's fields:** `font` → `text.text.font`; `visited_text_color` → `text.text.fill @ {visited}`;
  `underline_enabled` → `text.text.decoration` (a decoration or none); `background_color` → `body.fill`;
  `hover_background` → `body.fill @ {hover}`; `hover_text_color` → `text.text.fill @ {hover}`;
  `active_text_color` → `text.text.fill @ {pressed}`; `disabled_text_color` → `text.text.fill @ {disabled}`.
- **Notes:**
  - Links in running text are this widget, variant `inline`, in `context text_area` and `context rich_text` (TA-5,
    MAC-SF04†, SF-4†, MAT-P3-20†); `text_area` and `rich_text` have no link part.
  - GNOME's OSD links are no variant: `.osd` binds the `Osd` surface set, whose `link` and `link_visited` roles the
    text refers to (GN-1.265, R5.2).

### segmented_control

*From group 2's catalogue.*

- **Kind:** typed widget: GNOME `AdwToggleGroup`, macOS `NSSegmentedControl`, iOS `UISegmentedControl`, Windows
  Community Toolkit `Segmented`, Material's segmented button and button group (MAT-N6), KDE kirigami-addons'
  `SegmentedButton` (K-KA-24†) and `RadioSelector` (K-KA-25†) (flavour `kirigami`; kirigami-addons is in by R1.2).
  Today's preset fills KDE's from the tab bar as a proxy (platform-facts §2.25); kirigami-addons' rows replace it.
- **Available:** every preset; `kde-breeze` flavour `kirigami` only (the flavour `widgets` declares `available =
  false`: Qt Widgets draws none).
- **Parts:**
  - `track` — the group (the root): `fill`, `stroke`, `corners`, `padding` (the inset of the segments and of the
    indicator), `shadow`, `opacity`, `focus` (where the platform rings the group).
  - `segment` *(element)* — one segment (its own hover, pressed, selected, disabled, focus): `fill`, `stroke`,
    `corners` (concentric with the track, or rounded only at the ends by position), `padding`, `min_width`,
    `min_height`, `shadow`, `opacity`, `focus`, `bindings` (GNOME's OSD selected toggle rebinding the accent and status
    roles, GN-1.247), `transitions`. `label`, `icon`, `indicator`, `check`, `menu_indicator` belong to `segment` and
    are queried with it.
  - `label` — a segment's text: `text`, `opacity` (the dimmed label of a disabled group, GN-1.242), `transitions`
    (kirigami-addons' selected label turning to its selected text style, K-KA-25†).
  - `icon` — a segment's image: `icon` (its tint per state, MAC-SG10†), `opacity` (the dimmed image of a disabled
    group, GN-1.242), `transitions` (kirigami-addons' selected icon recolouring, K-KA-25†).
  - `indicator` — the selected thumb drawn as a part of its own (iOS, SG-1; macOS 12–15's raised segment and 26's
    sliding indicator, MAC-SG02, MAC-SG09; the Toolkit's selected item border and pill, W1-138; kirigami-addons'
    sliding pill, K-KA-25†): `fill` (an expression on `element.color` where the application sets AppKit's
    `selectedSegmentBezelColor`, MAC-SG05, ADD-B16; `data = { indicator = ["color"] }`), `stroke`, `corners`,
    `shadow`, `transform` (iOS 13–18's thumb shrinking while held, iOS 26's growing lens, SG-4), `transitions` (that
    change's motion, SG-4); it belongs to `segment` (nested in it, queried with it), drawn under the selected
    segment and absent by rule under the others.
  - `separator` *(element)* — the line between two segments, an element whose previous and next siblings are segments
    (it hides beside a selected one: GN-1.243, SG-3, MAC-SG01; ADD-B07): `stroke` (the line: its width and paint, the
    start edge), `padding` (its margins, GN-1.243, GN-1.245), `opacity`, `transitions` (its fade as it hides beside a
    selected segment, GN-1.243).
  - `check` — Material's check glyph in a selected segment (MAT-SG1): `glyph`, `transitions` (its show and width
    animation, MAT-SG1).
  - `menu_indicator` — AppKit's per-segment menu glyph (MAC-SG07): `glyph`.
- **Widget values:** `segment_width: Value<SegmentWidth>`, `SegmentWidth { Fit, Fill, FillEqually,
  FillProportionally, EqualToWidest }` — `Fit` each its content's width (AdwToggleGroup's default, AppKit `fit`, the
  Toolkit's pivot and button styles), `Fill` content plus an equal share of extra space (AppKit's default `fill`),
  `FillEqually` equal widths across the control (Material, AppKit `fillEqually`, UIKit's default),
  `FillProportionally` (AppKit), `EqualToWidest` (the Toolkit's `EqualPanel`) — GN-P3-25†, MAC-P3-5†, MAT-P3-12†,
  SG-9†, W-P3-5†; `item_gap_px: Value<Px>` (W-P3-5†; negative where segments overlap, K-KA-24†); `compression:
  Value<Key>` (AppKit `activeCompressionOptions`); `image_scaling: Value<Key>` (AppKit `imageScaling`, MAC-SG10†);
  `indicator_draggable: Value<bool>` (SG-5); `pressed_width_factor: Value<f32>` (Material's standard button group:
  the pressed segment's width is its width times this factor, its neighbours narrowing to make room, applied by the
  connector at layout, MAT-N6); `presentations: Vec<PresentationTransition>`: events `@indicator_move` with a step
  on part `indicator` (the indicator sliding to the new segment: SG-5, MAC-SG09, W1-138, K-KA-25†), `@spring_load` (a
  spring-loaded segment's flash or blink while a drag hovers it, MAC-X21, P3-11†).
- **Variants:** typed `pill` (GNOME `.round`, AppKit `.capsule` and the 26 capsule shape), `momentary` (AppKit
  `momentary` tracking, UIKit `isMomentary`), `flat` (GNOME `.flat`: no track), `vertical` (GNOME `.vertical`),
  `menu` on a segment (AppKit `showsMenuIndicator`), the content variants `icon_only`, `with_icon` (GNOME's minimum
  widths, GN-1.244; the Toolkit's item layouts, W1-154†). Keyed: `adwaita` `@osd`; `macos-*` `@rounded`,
  `@round_rect`, `@textured_rounded`, `@textured_square`, `@small_square`, `@separated`, `@rounded_rectangle` (26
  shape), `@select_any`; `windows-11` `@pivot`, `@button` (the default style is the base); `material-3`
  `@standard_group`, `@connected` (the button groups, MAT-N6); `kde-breeze` `@ka_segmented_button` (K-KA-24†),
  `@ka_radio_selector` (K-KA-25†).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large` (MAC-SG10†). `material-3` flavour
  `expressive`: the button groups' sizes `extra_small` … `extra_large` (MAT-N6). `ios-*`: no sizes; the height
  follows Dynamic Type, whose default category the preset states (R10, SG-9†). Densities: none from these rows.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, selected (S7.2: a chosen segment is the
  current choice among its siblings, `selected` on every platform; under `@select_any` each chosen segment is
  `selected`), disabled, focused, focus_visible.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `segment` (SegmentedItem's CommonStates, which
  name Selected, PointerOverSelected and PressedSelected with them, so `selected` combines); `material-3`: `[dragged,
  pressed, focus_visible, hover]` on `segment` (state layers); others none (states combine); precedence: state models
  (S17).
- **may_be_absent:**
  - parts: `indicator` (`adwaita`, `material-3`: the selected segment's own fill; `kde-breeze` under
    `@ka_segmented_button`, whose selected segment is filled, K-KA-24†), `separator` (`windows-11`, `material-3`,
    `kde-breeze`: kirigami-addons' joined segments overlap and draw no line, K-KA-24†), `check` (all but
    `material-3`), `menu_indicator` (all but `macos-*`).
  - fields: `track.fill` (`material-3`, `flat`), `track.shadow`, `segment.shadow`, `indicator.shadow` (each preset whose platform draws none, which the value phase names from the facts).
  - values: `compression`, `image_scaling` (all but `macos-*`), `indicator_draggable` (all but `ios-*`), `item_gap_px`
    (where not drawn), `pressed_width_factor` (all but `material-3`).
- **State-model source:** `adwaita`: `_toggle-group.scss` selectors (`toggle:hover`, `:active`, `:checked`,
  `:disabled`, `.flat`, `.round`, `.osd`, `.vertical`, `prefers-contrast: more`); `kde-breeze`: kirigami-addons'
  `SegmentedButton.qml` (hovered, focused, pressed, checked, enabled; K-KA-24†) and `RadioSelector.qml` (checked,
  hovered, input kind; K-KA-25†), their `checked` mapped to `selected`; `macos-*`: per segment selected, highlighted, enabled × `NSSegmentedControl.Style` ×
  `trackingMode` × control size × key window × appearance; `ios-*`: `UIControl.State` per segment × `isMomentary`,
  measured; `windows-11`: the Toolkit's `SegmentedItem.xaml` visual states (`:426-603`) per style; `material-3`: the
  outlined segmented button tokens (selected, unselected × enabled, hover, focus, pressed, disabled) and the button
  group tokens (MAT-N6).
- **Today's fields:** `background_color` → `track.fill`; `font` → `label.text.font`; `active_background` →
  `segment.fill @ {selected}` (`indicator.fill` where the platform draws an indicator); `active_text_color` →
  `label.text.fill @ {selected}`; `segment_height` → `segment.min_height`; `separator_width` →
  `separator.stroke` (its edge's width); `disabled_opacity` → `track.opacity @ {disabled}` (the Toolkit fades the
  item only: `segment.opacity @ {disabled}`); `hover_background` → `segment.fill @ {hover}`; `border.color` /
  `border.line_width` → `track.stroke`; `border.corner_radius` → `track.corners`; `border.padding_*` →
  `segment.padding`; `border.shadow_enabled` → `track.shadow` / `indicator.shadow`.
- **Notes:**
  - Group 6: Material's button group (`@standard_group`, `@connected`, MAT-N6): inner corners by state as
    `segment.corners` rules under `{pressed}` and `{selected}`; the standard group's pressed width as
    `values.pressed_width_factor`
    (a factor of the segment's own width, which S8 keeps out of size limits, so the connector applies it at layout),
    with a spring `Geometry` transition on `segment`.
  - kirigami-addons: `@ka_segmented_button` (K-KA-24†: joined segments, outer corners only by position, a hard shadow
    layer hidden while pressed); `@ka_radio_selector` (K-KA-25†: the sliding `indicator` pill, the selected label's
    own text style).

### switch

*From group 2's catalogue.*

- **Kind:** typed widget: all six platforms (KDE's is Kirigami's: qqc2-desktop-style `SwitchIndicator`).
- **Available:** every preset; `kde-breeze` flavour `kirigami` only (Qt Widgets has no switch: the flavour `widgets`
  declares `available = false`).
- **Parts:**
  - `body` — the whole control with its label (the root): `padding`, `min_width` (Q1.103, W1-124).
  - `track` — the track: `fill`, `stroke`, `corners`, `min_width`, `min_height` (today's `track_width`,
    `track_height`), `focus` (GN-1.206, MAT-SW9), `opacity` (applied to the parts nested in it, as one group),
    `transitions` (the track colour's cross-fade, SW-5, MAT-SW8).
  - `thumb`, in `track` — the knob or handle: `fill`, `stroke` (the rim), `shadow` (layers or an elevation entry), `corners`,
    `min_width`, `min_height` (a capsule where width and height differ, SW-3, MAC-SW01; growing or stretching by
    state), `transform` (iOS 26's lens), `transitions`.
  - `track_fill`, in `track` — KDE's highlight overlay from the track's start to the thumb's trailing edge (Q1.100): `fill`.
  - `on_glyph`, `off_glyph`, in `track` — the status shapes on the track (GNOME `switch-on/off-symbolic`, iOS's "I" and "O",
    macOS under differentiate without colour): `glyph`, `margin` (their place on the track, as insets from its edges,
    SW-4).
  - `thumb_icon`, in `thumb` — Material's check or close icon on the handle (MAT-SW3): `icon`, `glyph` (the symbol: the check
    under `{checked}`, the close otherwise, MAT-SW3), `opacity` (the disabled icon's opacity, MAT-SW3), `transitions`
    (MAT-SW8's icon transitions).
  - `state_layer`, in `thumb` — Material's halo (MAT-SW4): `fill`, `corners` (its shape), `opacity` (per state, MAT-SW4),
    `min_width`, `min_height`.
  - `label` — the switch's label text (SW-7, Q1.103): `text`, `opacity` (Kirigami dims the disabled label alone,
    Q1.103).
  - `state_label` — WinUI's On/Off content text: `text`.
  - `header` — WinUI's header (W1-124): `text`, `padding`.
- **Widget values:** `track_inset_px: Value<Px>` (Kirigami's track inset, an expression on the grid unit, Q1.99;
  ADD-A56); `label_gap_px: Value<Px>`; `pre_content_margin_px`, `post_content_margin_px: Value<Px>` (WinUI, W1-124);
  `presentations: Vec<PresentationTransition>`: event `@thumb_move` with a step on part `thumb` (the thumb sliding
  between off and on, its own occasion: GN-1.203, SW-5, Q1.102, MAT-SW8, W1-123).
- **Variants:** typed none; keyed `material-3` `@icons` (the switch with handle icons).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large` (MAC-SW05). Others none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, checked.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ToggleSwitch's CommonStates; ToggleStates is
  `checked`, not a group); `material-3`: `[dragged, pressed, focus_visible, hover]` (state layers, MAT-SW4); others
  none (states combine); precedence: state models (S17).
- **may_be_absent:**
  - parts: `track_fill` (all but `kde-breeze`), `on_glyph`, `off_glyph` (all but `adwaita`, `ios-*`, `macos-*`;
    on those stated in the base and absent by a rule outside the condition that shows them: `!cond on_off_labels`
    on `ios-*`, `!cond differentiate_without_color` on `macos-*` and `adwaita`, whose reader sets it from
    `show-status-shapes`, GN-1.202), `thumb_icon` (all but `material-3`), `state_layer`
    (all but `material-3`), `state_label`, `header` (all but `windows-11`).
  - fields: `thumb.shadow`, `thumb.stroke` (where not drawn).
  - values: `track_inset_px` (all but `kde-breeze`), `pre_content_margin_px`, `post_content_margin_px` (all but
    `windows-11`).
- **State-model source:** `adwaita`: `_switch.scss` selectors and `gtkswitch.c` (status shapes, slide);
  `kde-breeze`: qqc2-desktop-style `SwitchIndicator.qml` / `Switch.qml` states (flavour `kirigami`); `macos-*`:
  `NSSwitch` state, highlighted, enabled × control size × key window × appearance; `ios-*`: `UISwitch` `isOn`,
  tracking, enabled, measured; `windows-11`: ToggleSwitch's CommonStates × ToggleStates
  (`ToggleSwitch_themeresources.xaml:232-321`); `material-3`: the switch tokens (selected, unselected × enabled,
  hover, focus, pressed, disabled; with icons).
- **Today's fields:** `track_width`, `track_height` → `track.min_width`, `track.min_height`; `thumb_diameter` →
  `thumb.min_width`, `thumb.min_height @ {checked}`; `unchecked_thumb_diameter` → the same in the base;
  `track_radius` → `track.corners`; `checked_background` → `track.fill @ {checked}`; `unchecked_background` →
  `track.fill`; `thumb_background` → `thumb.fill @ {checked}`; `unchecked_thumb_background` → `thumb.fill`;
  `disabled_opacity` → `track.opacity @ {disabled}`; `hover_checked_background` → `track.fill @ {checked,hover}`;
  `hover_unchecked_background` → `track.fill @ {hover}`; `disabled_checked_background` → `track.fill @
  {checked,disabled}`; `disabled_unchecked_background` → `track.fill @ {disabled}`; `disabled_thumb_color` →
  `thumb.fill @ {disabled}`; `unchecked_track_border_color` → `track.stroke`; `checked_track_border_color` →
  `track.stroke @ {checked}`; `disabled_unchecked_track_border_color` → `track.stroke @ {disabled}`;
  `disabled_checked_track_border_color` → `track.stroke @ {checked,disabled}`; `track_border_line_width` →
  `track.stroke` widths; `thumb_border_color`, `thumb_border_line_width` → `thumb.stroke`.
- **Notes:**
  - GNOME's OSD switch is a rule in the context of any ancestor with the keyed variant `@osd` (GN-1.205; ADD-B08).
  - Group 4: KDE's switch delegate in `context list.row` (Q1.83); GNOME's in toolbars by the container rule
    (GN-1.152).
  - Group 8: iOS's On/Off Labels are `on_glyph`, `off_glyph` under `cond on_off_labels` (X-A11-6).
  - kirigami-addons: in `context form variant @ka_form_card`, its padding zero and its own label dropped
    (K-KA-8†).

### checkbox

*From group 2's catalogue.*

- **Kind:** typed widget: GNOME, KDE, macOS, Windows, Material; iOS only as the circular list multiselect indicator
  (`UICellAccessory.multiselect`: CK-1, CK-2†, L-12), which is the `selection` variant.
- **Available:** every preset; `ios-*` only as variant `selection` (`requires_variants = ["selection"]`, S10).
- **Parts:**
  - `body` — the whole control with its label (the root): `fill` (today's `background_color`), `padding`, `corners`,
    `min_height`, `min_width` (W1-040), `opacity`, `focus` (where the platform rings indicator and label together,
    GN-1.61), `touch_target` (Material's minimum interactive size, MAT-C9, MAT-X22).
  - `box` — the indicator: `fill`, `stroke`, `corners`, `shadow` (KDE's hover ring and macOS 12–15's bevel are inset
    layers, R7), `min_width`, `min_height`, `focus` (where the platform rings the box, K1.57), `transitions`.
  - `mark` — the check mark, the indeterminate dash or dots by rule: `glyph` (GNOME and Material paths, Breeze's
    `QPainterPath`, Segoe Fluent code points, AppKit's whole-control rendering, iOS's named symbol), `padding` (its
    inset inside the box, MAC-C06), `opacity` and `transform` for its entrance, `transitions` (that entrance).
  - `label` — the text: `text`, `opacity` (Kirigami dims the disabled label alone, Q1.22), `focus` (where Kirigami
    rings the label, Q1.21).
  - `state_layer` — Material's halo (MAT-C2): `fill`, `corners` (its shape), `opacity` (per state, MAT-C2),
    `min_width`, `min_height`.
- **Widget values:** `label_gap_px: Value<Px>`; `indicator_cell_px: Value<Px>` (the cell the drawn box is centred
  in, K1.53); `presentations: Vec<PresentationTransition>`: event `Appear` (iOS's selection column sliding in when
  edit mode starts, CK-2†).
- **Variants:** typed `selection` (the round check of a list in selection mode: GNOME `.selection-mode`, iOS's list
  multiselect circle: CK-1, CK-2†, L-12), `warning` (KDE's neutral highlight), the content variant `with_label` (a
  check with a label: GTK's `.text-button` padding, GN-1.61; the connector sets it from the content, ADD-A61).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. `ios-*`: the size follows Dynamic Type
  (its default category stated, R10). Others none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, checked,
  indeterminate, invalid (Material's error, MAT-C5).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CheckBox's CombinedStates, which name the check
  state with each of them, so `checked` and `indeterminate` combine); `material-3`: `[dragged, pressed,
  focus_visible, hover]` (state layers, MAT-C2); others none (states combine); precedence: state models (S17).
- **may_be_absent:**
  - parts: `state_layer` (all but `material-3`), `mark` (every preset: absent by rule while unchecked).
  - fields: `body.fill` (where the platform fills nothing), `body.focus`, `box.focus`, `label.focus` (each where the
    platform rings another part), `box.shadow`, `body.touch_target` (all but `material-3`).
  - values: `indicator_cell_px` (all but `kde-breeze`).
- **State-model source:** `adwaita`: `_checks.scss` (`check`, `radio`, `checkbutton`, `:checked`, `:indeterminate`,
  `:hover`, `:active`, `:disabled`, `.selection-mode`, `prefers-contrast: more`); `kde-breeze`: Breeze's
  `Helper::renderCheckBox` (`hp:831-981`) and `Style` `CE_CheckBox`; flavour `kirigami`: qqc2 `CheckBox.qml`,
  `CheckIndicator.qml`; `macos-*`: `NSButton` checkbox `StateValue` on, off, mixed × highlighted × enabled × control
  size × key window × appearance, with the whole-control rendering (S15); `ios-*`: the list cell's selected, editing
  and disabled states, measured; `windows-11`: CheckBox's CombinedStates (12 states); `material-3`: the checkbox
  tokens (selected, unselected, indeterminate × enabled, hover, focus, pressed, disabled × error).
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font`; `indicator_color` →
  `mark.glyph.fill`; `indicator_width` → `box.min_width`, `box.min_height` (on `kde-breeze` the cell:
  `values.indicator_cell_px`, K1.53); `check_mark_stroke_width` → `mark.glyph.stroke_width`; `label_gap` →
  `values.label_gap_px`; `checked_background` → `box.fill @ {checked}`; `disabled_opacity` → `body.opacity @
  {disabled}`; `hover_background` → `box.fill @ {hover}`; `disabled_background` → `box.fill @ {disabled}`;
  `disabled_text_color` → `label.text.fill @ {disabled}`; `unchecked_background` → `box.fill`;
  `unchecked_border_color` → `box.stroke`; `border.color`, `border.line_width` → `box.stroke`;
  `border.corner_radius` → `box.corners`; `border.padding_*` → `body.padding`; `border.shadow_enabled` →
  `box.shadow`; `radio_indicator_width`, `radio_dot_diameter` → `radio` (below).
- **Notes:**
  - Every list's multi-select check is this widget in `context list.row` (Nested controls): iOS's multiselect circle
    and GNOME's selection mode, variant `selection` (L-12, CK-1); WinUI's presenter check box (W1-109) and grid-item
    corner check box (W2-13, W2-14, its corner `list.values.check_box_placement`), the tree's (W2-12) and
    kirigami-addons' (K-KA-46†), plain; `list` has no check part.
  - A menu item's check is `menu.check_indicator` on every preset (Nested controls: it takes the item's states); on
    KDE it is drawn with this widget's look, its facts stated on that part (K1.69, Q1.31).
  - Group 4: KDE's check delegate in `context list.row` (Q1.83).
  - Group 6: KDE's checkable group title is a `checkbox` in `context group_box` (K2.8).
  - kirigami-addons: in `context form variant @ka_form_card` (K-KA-8†) and
    `context list.row variant @ka_rounded` (K-KA-38†).

### radio

*From group 2's catalogue.*

- **Kind:** typed widget: GNOME, KDE, macOS, Windows, Material; iOS has none. Split from today's `checkbox` because
  the platforms state its colours and geometry apart (W1-043, MAT-C1) and its parts differ.
- **Available:** every preset but `ios-*` (`available = false`: no such control).
- **Parts:**
  - `body` — the whole control (the root): `padding`, `corners` (GNOME rounds a grouped one fully, GN-1.63),
    `opacity`, `focus`, `touch_target` (as `checkbox`).
  - `ring` — the outer circle: `fill`, `stroke`, `corners`, `shadow` (KDE's hover ring, K1.52), `min_width`,
    `min_height`, `focus`, `transitions` (the hover ring fading in, K1.52).
  - `dot` — the inner dot: `glyph`, `fill`, `corners` (its round outline when drawn as a fill; the list is normative,
    and a preset states the form its platform draws, the other `Absent` where `may_be_absent` allows), `min_width`,
    `min_height` (growing by state, W1-041), `stroke`, `transform` (for its entrance), `transitions` (its scale
    animation, W1-041).
  - `label` — the text: `text`, `opacity` (Kirigami dims the disabled label alone, Q1.22), `focus`.
  - `state_layer` — Material's halo (MAT-C2): `fill`, `corners` (its shape), `opacity` (per state, MAT-C2),
    `min_width`, `min_height`.
- **Widget values:** `label_gap_px: Value<Px>`.
- **Variants:** typed `warning` (KDE's neutral highlight: the ring's pen and tint and its hover ring, K1.52, K1.56;
  `checkbox`'s `warning`), the content variant `with_label` (as on `checkbox`, GN-1.61).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. Others none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, checked.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (RadioButton's CommonStates; CheckStates is
  `checked`, not a group); `material-3`: `[dragged, pressed, focus_visible, hover]` (state layers); others none
  (states combine); precedence: state models (S17).
- **may_be_absent:**
  - parts: `state_layer` (all but `material-3`), `dot` (every preset: absent by rule while unchecked).
  - fields: `ring.focus`, `body.focus`, `label.focus` (as `checkbox`), `body.touch_target` (all but `material-3`),
    `ring.shadow` (each preset whose platform draws none, which the value phase names from the facts; KDE draws its hover ring, K1.52);
    `dot.glyph`, `dot.fill`, `dot.corners` (each preset states the form its dot is drawn in, a glyph or a cornered
    fill, and the others `Absent`).
- **State-model source:** as `checkbox`, with Breeze's `renderRadioButton` (`hp:995-1075`), AppKit's radio `NSButton`
  and the whole-control rendering (`SystemControl::AppKitRadio`), WinUI's RadioButton template, and Material's radio
  tokens.
- **Today's fields:** `checkbox`'s `radio_indicator_width` → `ring.min_width`, `ring.min_height`;
  `checkbox`'s `radio_dot_diameter` → `dot.min_width`, `dot.min_height @ {checked}`; the shared `checkbox` colours are
  converted into `radio`'s bases the same way (`checked_background` → `ring.fill @ {checked}`, `indicator_color` →
  `dot.fill`, `unchecked_border_color` → `ring.stroke`, …).
- **Notes:** Group 4: KDE's radio delegate in `context list.row` (Q1.83).

### radio_group

*From group 2's catalogue.*

- **Kind:** typed widget: Windows draws one (`RadioButtons`, W2-25, listed for Windows only in §4.2) and macOS draws it
  inside the radio (`RadioGroupPickerStyle`, MAC-RL02†, a cross-check row), so two platforms (R14).
- **Available:** every preset but `adwaita`, `kde-breeze`, `ios-*`, `material-3` (`available = false`: no such
  control).
- **Parts:**
  - `body` — the group's box (the root), holding the header and the radios: `padding`, `bindings`.
  - `header` — the group's label (W2-25's header colour, disabled colour and margin; MAC-RL02†'s label beside the
    column): `text`, `padding`.
- **Widget values:** `row_gap_px: Value<Px>`, `column_gap_px: Value<Px>` (W2-25), `header_gap_px: Value<Px>`,
  `header_placement: Value<HeaderPlacement>`, `HeaderPlacement { Above, Leading }`.
- **Variants:** typed `horizontal` (`macos-*`: `horizontalRadioGroupLayout()`; the orientation many widgets share).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. Others none.
- **Flags:** shared none; keyed none; state flags used: disabled (the header's disabled colour, W2-25); the group
  holds no interaction state of its own, its radios do.
- **Exclusive groups:** none (no interaction state); precedence: state models (S17).
- **may_be_absent:** parts: none (a group with no label simply has no `header` element queried); fields: none;
  values: none.
- **State-model source:** `windows-11`: `RadioButtons_themeresources.xaml`; `macos-*`: the SwiftUI host's measured
  layout.
- **Today's fields:** none (the nearest were `checkbox`'s `label_gap` and `layout`'s `widget_gap`).
- **Notes:** Group 7 had placed Windows' `RadioButtons` (W2-25) header colour, disabled colour, padding, `row_gap`
  and `column_gap` on `checkbox`.

## Fields

### input

*From group 3's catalogue.*

- **Kind:** typed widget: all six platforms draw a single-line text field (GtkEntry and GtkText, QLineEdit and qqc2
  TextField, NSTextField and NSSecureTextField, UITextField, WinUI TextBox, Material's text field). The password field
  is its typed `secure` (GN-2.23, GN-2.58†, W2-16); GNOME's GtkEditableLabel and iOS's inline rename field in the
  navigation bar are its typed `editable_label` (GN-2.45, RL-2†).
- **Available:** every preset.
- **Parts:**
  - `body` — the field's box (the root): `fill`, `stroke` (per side: WinUI's thicker accent bottom edge, Material's
    filled-field active indicator as the bottom edge), `corners`, `shadow` (macOS's inner edge shading and 26 outer
    ring; GNOME's inset drop-target and high-contrast rings), `padding`, `margin` (the space round a field in its
    host: Kirigami's search dialog field, Q1.118), `min_width`, `min_height`, `max_width`, `opacity` (today's
    `disabled_opacity`), `focus`, `cursor`, `bindings` (GNOME's error, warning and success classes rebind the accent
    for the subtree), `transitions` (the editable label's title-to-field change, RL-2†).
  - `text` — the entered text: `text`, `transitions` (RL-2†).
  - `placeholder` — the placeholder text: `text`, `transitions` (macOS's centred search placeholder moving to the
    leading edge on focus, MAC-N02).
  - `prefix`, `suffix` — Material's prefix and suffix text: `text`.
  - `floating_label` — Material's label: inside at rest, floated onto the outline when focused or populated: `text`,
    `padding` (the space below it, MAT-I3), `opacity` (its disabled opacity, MAT-I3), `transform`, `transitions`.
  - `header` — WinUI's header label above the field: `text`, `padding`.
  - `supporting_text` — text under the field (Material's supporting text, WinUI's description): `text`, `padding`,
    `opacity` (its disabled opacity, MAT-I5).
  - `counter` — Material's trailing character counter; kirigami-addons' "n/max" counter under a form field
    (K-KA-9†): `text`.
  - `icon` *(element)* — an icon or inline action inside the field, its side a variant (GNOME's primary and
    secondary icons, iOS's overlay views, Kirigami's search icon and ActionTextField actions, Material's leading and
    trailing icons, a search field's magnifier): `icon`, `fill`, `padding`, `opacity`, `cursor`, `transitions`.
  - `caps_lock_indicator` — the caps-lock glyph in a secure field (macOS, MAC-X14; GNOME, GN-2.23): `glyph`,
    `padding`, `opacity` (GNOME draws it dimmed, more opaque in high contrast, GN-2.23).
  - `autofill_affordance` — the AutoFill and strong-password affordance inside the field (macOS, iOS): `icon`,
    `fill`, `corners`, `padding`, `text` (the strong-password suggestion's label, I-9†).
  - `writing_tools_affordance` — macOS's Writing Tools affordance (MAC-P3-8†): `icon`, `fill`, `opacity`,
    `transitions`.
  - `progress` — GNOME's entry progress line, its extent the progress fraction the connector draws: `fill`,
    `corners`, `min_height`, `margin` (its bottom margin, GN-1.48).
  - `caret` — the line caret: `fill` (base `= token.caret.color`), `min_width` (the caret's width, base
    `= token.caret.width`, with rules under KDE's tablet mode, Q1.14), `min_height` (its height against the line,
    X-CA-1), `corners`; its shape and blink come from the caret token (R12, ADD-A24); absent by rule where the
    platform hides it (with a selection, `{text_selected}`; in overwrite mode, `{overwrite}`, where `block_caret`
    replaces it; ADD-A23).
  - `secondary_caret` — the bidirectional secondary caret where the caret sits between runs of different direction
    (GNOME's split cursor, Qt's bidi caret; GN-3.37): `fill` (base `= token.caret.secondary`), `min_width`,
    `min_height`.
  - `block_caret` — the block caret (overwrite mode, Qt's input mask), shown under `{overwrite}`: `fill`, `text` (the
    covered character's fill; GNOME GN-1.49, KDE K1.47†).
  - `drop_caret` — the insertion point drawn while a text drag hovers the field (P3-9†, W-P3-9†): `fill`,
    `min_width`.
  - `floating_caret` — iOS's floating (trackpad-mode) ghost caret (X-CA-3): `fill`, `opacity`, `corners`.
  - `caret_accessory` — iOS's caret accessories, the dictation and language indicators (X-CA-3): `icon`, `padding`.
  - `dictation_effect` — macOS's dictation effect at the caret (MAC-X14): `fill`, `transitions`.
  - `selection` *(element)* — the selected-text highlight, one element per selected range (a bidirectional selection
    is several, as on `text_area`): `fill`, `text` (the selected text's fill), `corners` (iOS's rounded highlight,
    X-SE-1), `transitions` (its fade, X-SE-1).
  - `preedit` *(element)* — the IME composition text (GN-1.281†, K3.48†): `text` (its decoration), `fill`.
  - `misspelling` — the spell-check mark: `text` (its decoration).
  - `grammar_mark` — macOS's grammar mark (MAC-X28): `text` (its decoration).
  - `correction` — iOS's autocorrection mark and bubble, Material's suggestion underline: `text` (its decoration),
    `fill`, `corners`.
  - `prediction` — inline prediction ghost text (iOS, macOS): `text`.
  - `selection_handle` *(element)* — a touch selection handle (Material MAT-I13, KDE's tablet mode Q1.14 and Q3.31,
    WinUI's grippers W1-034 and W3-25, iOS X-SE-2, GNOME GN-2.42): `fill`, `glyph`, `corners` (GNOME's circle with
    one square corner, GN-2.42), `padding` (GN-2.42), `min_width`, `min_height`, `transform` (the insertion handle
    rotated by a rule, GN-2.42), `touch_target`.
  - `magnifier` — the touch text magnifier (GNOME's GtkText, Material): `fill`, `stroke`, `corners`, `shadow`,
    `min_width`, `min_height`.
  - `strength_meter` — kirigami-addons' password strength meter, its segments' track (K-KA-11†): `fill`, `corners`,
    `min_height`.
  - `strength_meter_fill`, in `strength_meter` — the meter's fill, grown in proportion to the entropy: `fill`,
    `corners`; its extent is the connector's, drawn from the strength fraction (no field reads it, so it declares no
    data; K-KA-11†).
- **Widget values:** `icon_text_gap_px: Value<Px>` (GN-1.43, MAT-I7, Q1.18); `leading_inset_px`,
  `trailing_inset_px: Value<Px>` (I-3); `affix_gap_px: Value<Px>` (prefix and suffix to text, MAT-I8);
  `floating_label_gap_px: Value<Px>` (label to text, populated, MAT-I3); `notch_padding_px: Value<Px>` (the gap cut
  into the outline round the floated label, which the toolkit cuts at layout, MAT-I4); `header_gap_px`,
  `supporting_text_gap_px: Value<Px>` (W1-031, W1-150, MAT-I5); `clear_button_visibility: Value<ShowWhen>` (when the
  field shows its `clear_field` button: I-2, W1-030, K1.46; WinUI's ButtonStates); `reveal_button_visibility:
  Value<ShowWhen>` (its `reveal` button, K1.48); `mask_character: Vec<char>` (a preference list: the first the font
  covers; K1.48, MAC-I03, I-9, MAT-I19, GN-2.58†, W2-16); `mask_reveal_ms: Value<Ms>` (how long the last typed
  character shows; GNOME's password hint timeout; K1.48, I-9, MAT-I19, GN-2.23); `peek_while_pressed: Value<bool>`
  (W2-16); `focus_visible_when` (`Value<FocusVisibleWhen>`, S2.11; GN-1.50); `min_font_size: Value<FontSize>` (the
  shrink-to-fit floor, I-7); `default_width_px: Value<Px>` (implicit width, Q1.12); `magnifier_zoom: Value<f32>` (a
  factor), `magnifier_offset_px: Value<Px>` (above the caret or touch point) (GN-P3-26, MAT-I17); `search_delay_ms:
  Value<Ms>` (Kirigami's search field debounce, Q1.17); `context_menu_items: Vec<Key>` (the text context menu's items
  in order, GN-P4-8†); `strength_meter_segments: Value<u32>` (the meter's segment count, K-KA-11†);
  `strength_meter_gap_px: Value<Px>` (between the meter's segments, K-KA-11†).
- **Variants:** typed `flat` (no frame: GNOME's flat entry, KDE frameless, macOS borderless, iOS `none`), `line` (a
  plain line frame: iOS's line style, macOS bordered-not-bezeled), `secure` (password field, all six; GN-2.23, W2-16),
  `warning` (GNOME's warning class, KDE's neutral highlight), `error`, `success` (GNOME's error and success classes;
  the status severities), `editable_label` (view colours while editing: GNOME GN-2.45, iOS RL-2†); typed on the `icon`
  element `leading`, `trailing`; on `preedit` `selected_clause`; on `selection_handle` the identities `start`, `end`,
  `insertion` (GN-2.42). Keyed: `adwaita` on `preedit` `@whole`, `@selection`, `@prediction`, `@prefix`, `@suffix`,
  `@spelling_error`, `@compose_error` (GTK's pre-edit hint classes, GN-1.281†); `windows-11` on `preedit` one key per
  IME clause attribute (declared by the preset); `kde-breeze` `@border_top`, `@border_right`, `@border_bottom`,
  `@border_left` (Breeze's per-side borders: only the named edges drawn, K1.42); `macos-*` `@square_bezel`,
  `@rounded_bezel`; `ios-*` `@bezel`; `material-3` `@filled` (outlined is the base).
- **Sizes and densities:** `macos-*`: the control sizes AppKit draws for a text field (of `mini` to `extra_large`,
  MAC-X17; which ones, per generation, from the probe). `ios-*`: none (the height follows Dynamic Type, I-10, read by
  the readers' milestone). `material-3`: its density steps (keyed). Others: none.
- **Flags:** shared `empty` (the field holds no text: Material's floated label, WinUI's and KDE's clear button),
  `overwrite` (the block caret: GNOME GN-1.49, KDE K1.47†), `composing` (an input-method composition: GNOME GN-1.281†,
  KDE K3.48†), `text_selected` (the text has a selection: the caret's absence rule, GN-3.73†, MAT-P3-21†, P3-9†,
  MAC-P3-11†, W-P3-9†; KDE keeps the caret, K3.43); keyed `kde-breeze` `@quality_bad`, `@quality_poor`,
  `@quality_weak`, `@quality_good`,
  `@quality_excellent` (the strength meter's quality, K-KA-11†), `@at_limit` (the counter at its limit, K-KA-9†);
  state flags used: hover, pressed (on `icon`), focused, focus_within, focus_visible, disabled, read_only, invalid,
  autofilled, drop_target.
- **Exclusive groups:** `windows-11`: `[disabled, focused, hover]` (TextBox CommonStates; its ButtonStates are the
  `clear_field` button's visibility value, not a group); others none (states combine; kirigami-addons sets one quality
  flag at
  a time, K-KA-11†); precedence: state models (S17).
- **may_be_absent:**
  - parts: `prefix`, `suffix`, `floating_label`, `supporting_text` (`supporting_text` drawn by `material-3` and
    `windows-11`; the others by `material-3` only), `counter` (all but `material-3`, `kde-breeze`), `header` (all but
    `windows-11`), `caps_lock_indicator` (all but `macos-*`, `adwaita`), `autofill_affordance` (all but `macos-*`,
    `ios-*`), `writing_tools_affordance`, `dictation_effect`, `grammar_mark` (all but `macos-*`), `progress` (all but
    `adwaita`), `caret` (every preset, by rule where the platform draws none: outside the focused field, in an
    inactive window, `{overwrite}`; `{text_selected}` on all but `kde-breeze`: GN-3.73†, MAT-P3-21†, P3-9†,
    MAC-P3-11†, W-P3-9†), `secondary_caret` (all but `adwaita`, `kde-breeze`), `block_caret` (all but `adwaita`,
    `kde-breeze`; there absent by a rule `!{overwrite}`), `drop_caret` (all but `ios-*`, `adwaita`, `windows-11`,
    `macos-*`: the second caret at the drop position, P3-9†, GN-3.73†, W-P3-9†, MAC-P3-11†; a platform that only
    recolours its caret on a drop target states `caret.fill @ {drop_target}` instead),
    `floating_caret`, `caret_accessory` (all but `ios-*`), `correction` (all but
    `ios-*`, `material-3`), `prediction` (all but `ios-*`, `macos-*`), `selection_handle` (`macos-*`; `windows-11`,
    `kde-breeze` and `adwaita` outside touch input, by rules `@ !cond input:touch`), `magnifier` (all but `adwaita`,
    `material-3`, `ios-*`), `strength_meter`, `strength_meter_fill` (all but `kde-breeze`).
  - fields: `body.shadow`, `body.focus`, `body.cursor`, `body.margin`, `progress.margin`.
  - values: `notch_padding_px`, `floating_label_gap_px`, `affix_gap_px` (all but `material-3`), `header_gap_px` (all
    but `windows-11`), `supporting_text_gap_px` (all but `material-3`, `windows-11`), `reveal_button_visibility` (all
    but `kde-breeze`), `mask_reveal_ms` (none shown), `peek_while_pressed` (all but `windows-11`), `min_font_size` (all
    but `ios-*`), `default_width_px` (all but `kde-breeze`), `magnifier_zoom`, `magnifier_offset_px` (all but `adwaita`,
    `material-3`, `ios-*`), `search_delay_ms` (all but `kde-breeze`), `strength_meter_segments`, `strength_meter_gap_px` (all but `kde-breeze`).
- **State-model source:**
  - `windows-11`: the TextBox template's visual state groups (`CommonStyles/TextBox_themeresources.xaml`:
    CommonStates, ButtonStates).
  - `adwaita`: `_entries.scss` selectors (focus within, disabled, flat, drop, block cursor, progress, icon hover and
    active), `_labels.scss` (error, warning, success), the high-contrast media query; GtkText's pre-edit and magnifier
    nodes.
  - `kde-breeze`: `breezestyle.cpp` line-edit frame and panel branches (enabled, mouse over, focus, neutral highlight,
    frame on or off, per-side borders) with the input-widget engine; the `kirigami` flavour from qqc2-desktop-style
    `TextField.qml`, `SearchField.qml`, `private/MobileCursor.qml` and Kirigami's `SearchField.qml`,
    `ActionTextField.qml`, `PasswordField.qml`; kirigami-addons' `FormPasswordFieldDelegate.qml` (quality) and form
    field rows (counter).
  - `material-3`: the outlined and filled text-field token states (enabled, hover, focus, error, disabled) of the
    pinned token set; Compose's text field for caret, handles, magnifier, secure field and autofill.
  - `macos-*`: NSTextField and NSSecureTextField: enabled, editable, first responder, bezel style, bordered, control
    size, window key state, appearance.
  - `ios-*`: `UIControl.State` (normal, disabled, focused), editing, `borderStyle`, the content-size category.
- **Today's fields:** `background_color` → `body.fill`; `placeholder_color` → `placeholder.text.fill`; `caret_color` →
  `caret.fill` (base `= token.caret.color`); `selection_background`, `selection_text_color` → `selection.fill`,
  `selection.text.fill` (bases `= role.global.selection_text_background`, `= role.global.selection_text_foreground`);
  `font` → `text.text.font`, its colour `text.text.fill`; `min_height_px` → `body.min_height`; `disabled_opacity` →
  `body.opacity @ {disabled}`; `hover_border_color` → `body.stroke` paint `@ {hover}`; `focus_border_color` →
  `body.stroke @ {focused}` where the platform recolours the frame, `body.focus` where it draws a ring;
  `disabled_background`, `disabled_text_color` → `body.fill`, `text.text.fill @ {disabled}`; `border.*` →
  `body.stroke`, `body.corners`, `body.padding`; `shadow_enabled` → `body.shadow`.
- **Notes:** inputs in context: `input @ context dialog.body variant alert` (DL-9) and `variant @search_dialog`
  (Q1.118); the in-place cell editor `input @ context list.cell {editing}` (K-RL05†, MAC-L13, GN-1.279†); a search
  field's text `input @ context any search_field` (its bezel, placeholder, magnifier `icon` and search delay, Q1.15–
  Q1.17); a combo box's editable text `input @ context any combo_box`; a spin box's text `input @ context stepper`.
  Nested controls: the clear button is `button @ variant clear_field context input` and the password reveal toggle
  `button @ variant reveal context input` (GN-2.23, W2-16, K1.48; their visibility the values above, their fade the
  `button`'s presentations), the completion popup an `autocomplete` in `context input` (MAC-P3-7†). Provenance: group
  6 wrote the password field as `password` and its hint timeout as a value of its own, merged into `mask_reveal_ms`;
  today's caret width is the `caret` part's minimum width; the Writing Tools affordance is group 3's (rename row
  12).

### text_area

*From group 3's catalogue.*

- **Kind:** typed widget: the multi-line text view (GtkTextView, QTextEdit and qqc2 TextArea, NSTextView, UITextView,
  WinUI's multi-line TextBox, Material's textarea field).
- **Available:** every preset.
- **Parts:**
  - `body` — the frame and fill (the root): `fill`, `stroke`, `corners`, `padding` (the fixed frame inset), `shadow`,
    `min_width`, `min_height`, `opacity`, `focus`, `cursor`, `bindings`.
  - `text` — the entered text, as on `input` (each part below stated on this widget's own parts, R20): `text`.
  - `placeholder` — as on `input`: `text`.
  - `prefix`, `suffix` — as on `input` (Material's multi-line field, MAT-I16): `text`.
  - `floating_label` — as on `input` (Material's multi-line field, MAT-I3, MAT-I16): `text`, `padding`, `opacity`,
    `transform`, `transitions`.
  - `counter` — as on `input` (Material's multi-line field, MAT-I6): `text`.
  - `icon` *(element)* — as on `input` (Material's multi-line field's leading and trailing icons): `icon`, `fill`,
    `padding`, `opacity`, `cursor`, `transitions`.
  - `header` — WinUI's header label above the multi-line box, as on `input` (W1-036): `text`, `padding`.
  - `supporting_text` — text under the box, as on `input` (WinUI's description, W1-036; Material's supporting text):
    `text`, `padding`, `opacity`.
  - `caret` — as on `input`: `fill`, `min_width`, `min_height`, `corners`.
  - `secondary_caret` — as on `input` (GtkTextView's split cursor, QTextEdit's bidi caret, GN-3.37): `fill`,
    `min_width`, `min_height`.
  - `block_caret` — as on `input`: `fill`, `text`.
  - `floating_caret` — as on `input` (UITextView's floating cursor, X-CA-3): `fill`, `opacity`, `corners`.
  - `caret_accessory` — as on `input` (X-CA-3): `icon`, `padding`.
  - `dictation_effect` — as on `input` (the dictation effect at the caret of a text area, MAC-X14): `fill`,
    `transitions`.
  - `preedit` *(element)* — as on `input`: `text`, `fill`.
  - `misspelling` — as on `input`: `text`.
  - `grammar_mark` — as on `input` (NSTextView's grammar mark, MAC-X28): `text`.
  - `correction` — as on `input`: `text`, `fill`, `corners`.
  - `prediction` — as on `input`: `text`.
  - `selection_handle` *(element)* — as on `input`: `fill`, `glyph`, `corners`, `padding`, `min_width`, `min_height`,
    `transform`, `touch_target`.
  - `magnifier` — as on `input` (GtkTextView's, GN-P3-26†): `fill`, `stroke`, `corners`, `shadow`, `min_width`,
    `min_height`.
  - `selection` *(element)* — the selected-text highlight, one element per selected range (P4-13): `fill`, `text`
    (the selected text's fill), `corners`, `transitions` (its fade, X-SE-1).
  - `drop_caret` — the insertion point drawn while a text drag hovers the view (UIKit's, P3-9†; WinUI's, W-P3-9†;
    AppKit's, MAC-P3-11†; GtkTextView's, in the accent, GN-1.53, GN-3.39): `fill`, `min_width`.
  - `gutter` — GtkTextView's border windows (gutters): `fill`, `stroke`, `padding`.
  - `find_match` *(element)* — a find-and-replace match; the current match is selected (macOS's find indicator,
    MAC-X15): `fill`, `stroke`, `corners`, `shadow`, `text`, `transform`.
  - `find_dimming` — the dimming NSTextFinder draws over the text that does not match during an incremental find
    (MAC-X15): `fill`, `transitions`.
  - `highlight` *(element)* — an application text highlight (iOS's highlight styles; macOS's, an open item,
    MAC-X28): `fill`, `text`, `corners`.
  - `invisible` *(element)* — the substitute glyph TextKit 1 draws for an invisible character: `glyph`.
  - `writing_tools_glow` — the Writing Tools glow over rewritten text (MAC-X28, TA-4, C-33): `fill`, `shadow`,
    `transitions`.
- **Widget values:** `document_margin: Sides<Value<Px>>` (the inset inside the document, which scrolls with the text:
  Qt's documentMargin, Apple's textContainerInset; the body's padding is the fixed frame inset; K-P4-9, P3-1,
  MAC-I10); `line_padding_px: Value<Px>` (Apple's lineFragmentPadding at each line's start and end; P3-1, MAC-I10);
  `scrollbar_inset: Sides<Value<Px>>` (WinUI's inner scroll bar, W1-036); `default_width_px`,
  `default_height_px: Value<Px>` (Q1.12); `min_rows: Value<u32>` (MAT-I16); `resize: Value<Resize>` with
  `Resize { None, Vertical, Both }` (MAT-I16); `header_gap_px`, `supporting_text_gap_px: Value<Px>` (as on input,
  W1-036); `empty_ranges_draw_carets: Value<bool>` (P4-13);
  `intent_styles: Vec<(Key, TextStyle)>` (how the text view renders Foundation's presentation intents, keyed by
  intent; empty where it renders none; SF-5, MAC-SF05); `magnifier_zoom: Value<f32>`, `magnifier_offset_px: Value<Px>`
  (as on input;
  GN-P3-26, MAT-I17); `notch_padding_px`, `floating_label_gap_px`, `affix_gap_px: Value<Px>` (as on input, Material's
  multi-line field, MAT-I3, MAT-I4, MAT-I8); `context_menu_items: Vec<Key>` (GN-P4-8†);
  `presentations: Vec<PresentationTransition>`: keyed
  event `@find_indicator` (the find indicator's bounce on the current match, a step on part `find_match`, MAC-X15).
- **Variants:** typed `flat` (no frame: macOS's scroll view with no border, iOS `none`), `line` (macOS's line border;
  input's `line`); typed on `invisible` `space`, `tab`, `line_break`, `no_break_space`; typed on the `icon` element
  `leading`, `trailing` (as on `input`). Keyed: `material-3` `@filled` (as on `input`); `adwaita` `@inline`
  (the inline text view, GN-1.54); `macos-*` `@bezel`, `@groove`; `ios-*` `@rounded_rect` (iOS 17's rounded rect);
  `ios-*` and `macos-*` on `highlight` one key per highlight style (declared by the preset; macOS's an open item,
  MAC-X28). As on `input` (each stated on this widget, R20): typed on `preedit` `selected_clause`, on
  `selection_handle` the identities `start`, `end`, `insertion` (GN-2.42); keyed `adwaita` on `preedit` `@whole`,
  `@selection`, `@prediction`, `@prefix`, `@suffix`, `@spelling_error`, `@compose_error` (GN-1.281†); `windows-11` on
  `preedit` one key per IME clause attribute.
- **Sizes and densities:** none (`macos-*`: control sizes do not apply to NSTextView).
- **Flags:** shared `empty` (as on input), `overwrite` (the overwrite caret: GNOME, KDE; ADD-A23), `composing` (GNOME
  GN-1.281†, KDE K3.48†), `text_selected` (as on input); keyed `macos-*` `@incremental_find` (an incremental find
  runs: `find_dimming` drawn,
  MAC-X15); state flags used: hover, focused, focus_within, focus_visible, disabled, read_only,
  drop_target, selected (the current find match).
- **Exclusive groups:** `windows-11`: `[disabled, focused, hover]` (TextBox CommonStates: the multi-line box is a
  TextBox); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `gutter` (all but `adwaita`), `find_dimming` (all but `macos-*`; absent by rule
  outside `{@incremental_find}`), `highlight` (all but `ios-*`, `macos-*`), `header` (all but `windows-11`),
  `supporting_text` (all but `material-3`, `windows-11`), `grammar_mark` (all but `macos-*`), `caret` (as on `input`:
  outside the focused view, `{overwrite}`, `{text_selected}` on all but
  `kde-breeze`, GN-3.73†), `secondary_caret` (all but `adwaita`, `kde-breeze`), `floating_caret`, `caret_accessory`
  (all but `ios-*`), `dictation_effect` (all but `macos-*`), `invisible` (all but `macos-*`, `ios-*`),
  `writing_tools_glow` (all but `macos-*`, `ios-*`), `block_caret` (all but `adwaita`, `kde-breeze`), `drop_caret`
  (all but `ios-*`, `windows-11`, `macos-*`, `adwaita`), `prefix`, `suffix`, `floating_label`, `counter`, `icon` (all
  but `material-3`), `correction` (all but `ios-*`, `material-3`), `prediction` (all but `ios-*`,
  `macos-*`),
  `selection_handle` (`macos-*`; elsewhere outside touch input), `magnifier` (all but `adwaita`, `material-3`,
  `ios-*`),
  `find_match` (all but `macos-*`, `ios-*`: NSTextFinder's and UIFindInteraction's indicators); fields: `body.shadow`,
  `body.fill` (`flat`), `body.focus`, `body.cursor`; values: `document_margin` (where the platform states no inset
  inside the document: the value phase names the presets), `line_padding_px` (all but `macos-*`, `ios-*`),
  `scrollbar_inset` (all but `windows-11`), `default_width_px`, `default_height_px` (all but `kde-breeze`),
  `min_rows`, `resize`, `notch_padding_px`, `floating_label_gap_px`, `affix_gap_px` (all but `material-3`),
  `magnifier_zoom`, `magnifier_offset_px` (all but `adwaita`, `material-3`, `ios-*`), `header_gap_px` (all but
  `windows-11`), `supporting_text_gap_px` (all but `material-3`, `windows-11`). The lists (`intent_styles`,
  `context_menu_items`, `presentations`) are empty, not absent, where the platform has none.
- **State-model source:** `windows-11` as input; `adwaita` `_views.scss` (the text view, its border nodes, inline,
  drop); `kde-breeze` the QTextEdit frame branches and qqc2 `TextArea.qml`; `material-3` the textarea form of the
  text-field tokens; `macos-*` NSTextView (editable, selectable, first responder, window key) inside NSScrollView (its
  border type); `ios-*` UITextView (editable, `borderStyle`, `UIFindInteraction` states, `textHighlightAttributes`).
- **Today's fields:** `border.*` → `body.stroke`, `body.corners`, `body.padding` (the fixed frame inset; the scrolling
  part moves to `document_margin`), `body.shadow`; the background → `body.fill`. Everything today read from `input` is
  stated on `text_area`'s own
  parts (no model inheritance, R20).
- **Notes:** links in running text are `link @ variant inline context text_area` (the former link part goes); the
  enclosing NSScrollView's border is `scroll_view.frame`, selected by the same variants where the text view is in one;
  the current find match is `text_area.find_match @ {selected}` (MAC-X15). Provenance: the glow is group 3's (rename
  row 12);
  group 8 proposed `drop_caret` (P3-9†); group 1 the overwrite caret's flag (ADD-A23) and the
  context menu items (GN-P4-8†).

### combo_box

*From group 3's catalogue.*

- **Kind:** typed widget: GtkDropDown, QComboBox and qqc2 ComboBox, NSPopUpButton and NSComboBox, iOS's pop-up button,
  WinUI ComboBox, Material's select.
- **Available:** every preset.
- **Parts:**
  - `body` — the trigger box (the root): `fill`, `stroke` (per state), `corners`, `shadow` (Breeze's button shadow on
    the non-flat, non-editable combo), `padding`, `min_width`, `min_height`, `opacity`, `focus`, `bindings` (Kirigami:
    Button set, View set when editable), `transitions`.
  - `label` — the current value's text: `text`.
  - `placeholder` — WinUI's placeholder text: `text`.
  - `icon` — a leading icon (Material): `icon`, `opacity` (its disabled opacity, as on input: MAT-CB1, MAT-I7).
  - `arrow` — the indicator glyph (single or double chevron; on macOS the AppKit rendering, the system control
    `AppKitPopUp`), a mark that takes the combo box's states (Nested controls): `glyph` (its fill per state),
    `transform` (its rotation while `open`), `transitions` (WinUI's animated chevron, W1-132).
  - `arrow_area` — macOS's accent-filled arrow area of the pop-up, NSComboBox's separate arrow button: `fill`,
    `stroke`, `corners`, `min_width`.
  - `header` — WinUI's header text: `text`, `padding`.
  - `floating_label` — Material's select field label (as on input, MAT-CB1): `text`, `padding`, `opacity`,
    `transform`, `transitions`.
  - `supporting_text` — Material's select supporting text (as on input, MAT-CB1), WinUI's ComboBox description
    (W1-150†): `text`, `padding`, `opacity`.
- **Widget values:** `label_arrow_gap_px: Value<Px>` (GN-1.233); `header_gap_px`, `supporting_text_gap_px:
  Value<Px>` (W1-137, MAT-CB1); `notch_padding_px`, `floating_label_gap_px: Value<Px>` (as on input: the outline's
  notch round the floated label, the label-to-value gap; MAT-CB1); `popup_placement: Placement` (where its popup goes,
  the host's value by Nested controls: over the selected item, below or above, its width and the number of rows it
  shows, the placement's `max_visible_items`; W1-133, W1-136, MAC-CB04, MAT-P3-9, K1.245, K-P3-5†, MAC-P4-8†, P4-12†).
  The popup is a `menu` in `context combo_box`: its open and close presentations, the check's side and WinUI's pill
  are `menu`'s values and parts there.
- **Variants:** typed `editable` (KDE, macOS's NSComboBox, WinUI, Material's editable menu, GTK's combo with entry),
  `flat` (GNOME's flat drop-down, KDE frameless, macOS borderless), `pull_down` (macOS's pull-down, iOS's menu button
  that does not change selection), `warning` (KDE's neutral highlight; input's `warning`), `tinted`, `primary` (iOS's
  tinted and prominent trigger configurations, `button`'s meanings, CB-2), `glass`, `clear` (iOS 26's glass trigger
  styles, `button`'s meanings, CB-2). Keyed: `adwaita` `@gtk_combo_box`
  (GN-1.238), `@raised` (the drop-down kept raised in a toolbar, GN-1.151); `kde-breeze` `@dialog_mode`, `@page_mode`
  (the form combo row's display modes, K-KA-12†); `material-3` `@filled`; `ios-*` `@gray`, `@borderless`, `@bordered`
  (the trigger's `UIButton.Configuration` styles of B-1, as `button` declares them, CB-2).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. `material-3`: density (keyed). Others:
  none.
- **Flags:** shared `empty` (Material's select with no value chosen: its label rests inside the field, MAT-CB1); state
  flags used: hover, pressed, focused, focus_visible, disabled, open (the popup shown).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ComboBox CommonStates; ComboBoxItem's are
  `menu`'s in context), `[focus_visible, focused]`
  (FocusStates); its DropDownStates are the flag `open`, its InputModeStates the conditions `cond input:touch` and
  `cond input:game_controller`, its EditableModeStates the variant `editable`; others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `placeholder`, `header` (all but `windows-11`), `icon`, `floating_label` (all but
  `material-3`), `supporting_text` (all but `material-3`, `windows-11`), `arrow_area` (all but `macos-*`); fields:
  `body.shadow`; values: `header_gap_px`, `supporting_text_gap_px` (all but `windows-11`, `material-3`),
  `notch_padding_px`, `floating_label_gap_px` (all but `material-3`).
- **State-model source:** `windows-11` the ComboBox and ComboBoxItem visual state groups
  (`ComboBox/ComboBox_themeresources.xaml`); `adwaita` `_dropdowns.scss`, `_buttons.scss` (the trigger is a toggle
  button, checked while open), `_deprecated.scss`; `kde-breeze` `breezestyle.cpp` combo-box branches (editable, frame,
  mouse over, focus, sunken or open, neutral) and the helper's button frame; qqc2 `ComboBox.qml` for `kirigami`;
  kirigami-addons' `FormComboBoxDelegate` (display modes); `material-3` the outlined and filled select tokens;
  `macos-*` NSPopUpButton (pulls down, bordered, control size, key window) and NSComboBox; `ios-*`
  `UIButton.Configuration` with its indicator and menu states.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font` (colour `label.text.fill`);
  `min_height_px`, `min_width_px` → `body.min_height`, `body.min_width`; `arrow_icon_size_px` → `arrow.glyph.size`;
  `arrow_area_width_px` → `arrow_area.min_width` (absent where no area is drawn); `disabled_opacity` →
  `body.opacity @ {disabled}`; `hover_background` → `body.fill @ {hover}`; `disabled_background`,
  `disabled_text_color` → `body.fill`, `label.text.fill @ {disabled}`; `border.*` → `body.stroke`, `body.corners`,
  `body.padding`, `body.shadow`.
- **Notes:** Nested controls: the popup is a `menu` in `context combo_box` on every platform (GNOME's menu-style
  popover and GtkDropDown's list, the qqc2 combo's Menu and QComboBox's list, macOS's pop-up menu, iOS's menu,
  Material's exposed drop-down menu, WinUI's ComboBox popup: W1-133–W1-136, GN-1.234, MAC-CB04); the editable combo's
  text is an `input` in `context any combo_box`; GNOME's search entry in the popup is a `search_field` in `context menu`
  within `context any combo_box`; the form combo row is `combo_box @ context form.row` (K-KA-12†). Provenance: group 1
  wrote the popup's placement as a value named `placement` (GN-P4-2†, K-P3-5†, MAC-P4-8†), now `popup_placement`
  (the host's value, Nested controls; the second check had moved it to `menu`, the third moved it back).

### slider

*From group 3's catalogue.*

- **Kind:** typed widget: all six (GtkScale, QSlider and qqc2 Slider and RangeSlider, NSSlider, UISlider, WinUI
  Slider, Material's slider); the range slider is a variant; the colour editors' hue, alpha and value sliders are
  typed variants (GNOME GN-2.53†, Windows W2-05).
- **Available:** every preset.
- **Parts:**
  - `body` — the widget box: `padding` (round the track; WinUI's pre- and post-content margins, W1-072), `fill`
    (container fill per state, WinUI, W1-074), `opacity` (GTK's disabled filter; today's `disabled_opacity`),
    `min_width`, `min_height` (WinUI's control height, W1-072; across the axis under `vertical`).
  - `track` — the inactive track (trough): `fill`, `stroke` (GTK's high-contrast inset border, GN-1.108), `corners`
    (SL-4, K1.104, MAC-SL03, MAT-S3, W1-069), `min_height`, `max_height`, `min_width`, `max_width` (its thickness
    across the axis, today's `track_height_px`: the height when horizontal, the width under `vertical`), `shadow`
    (Material's track elevation, MAT-S9), `transitions` (GTK's trough motion, GN-1.106).
  - `fill` — the active track (value fill, GTK's highlight): `fill`, `corners` (the fill's ends, MAC-SL03, MAT-S3),
    `min_height`, `max_height`, `min_width`, `max_width` (its thickness, as the track's), `transitions` (Geometry: the
    animated value change, SL-9; GN-1.106).
  - `fill_level` — GTK's second trough showing the fill level (GN-1.103): `fill`, `corners`.
  - `disabled_range` — the track outside iOS 26's enabled range (SL-7): `fill`.
  - `thumb` *(element)* — the handle (two under the range variant): `fill`, `corners`, `min_width`, `min_height` (its
    shape by corners and size: MAT-S1, SL-2, MAC-SL01), `transform` (GNOME's teardrop rotation, iOS 26's growing
    lens), `shadow` (a rim ring as a spread layer; K3.4), `stroke` (GN-1.96, K1.100, MAT-S10), `focus` (GN-1.104,
    K1.106), `transitions` (Geometry: the animated value change, SL-9; KDE's hover and focus fades, K1.106).
    `thumb_inner`, `halo` belong to `thumb` and are queried with it.
  - `thumb_inner` — WinUI's inner accent dot: `fill` (per state), `corners`, `min_width`, `min_height` (W1-067),
    `transform`, `transitions` (its scale per state and that scale's animation, W1-068).
  - `halo` — Material's state-layer halo round the handle (MAT-S7): `fill`, `min_width`, `min_height`.
  - `tick` *(element)* — a tick mark (its fill depends on whether it lies within the fill): `fill`, `corners` (a line
    or a dot, MAC-SL04, MAT-S5), `min_width`, `min_height` (today's `tick_mark_length_px`; GN-1.100, SL-6).
  - `stop_indicator` *(element)* — Material's stop dot (MAT-S4): `fill`, `corners`, `min_width`, `min_height`.
  - `value_label` — GTK's drawn value label (GN-1.102): `text`, `opacity` (dimmed).
  - `value_indicator` — Material's value bubble shown while dragging (MAT-S6): `fill`, `corners`, `min_height`,
    `text`. WinUI's thumb tooltip is a `tooltip` (Notes).
  - `header` — WinUI's header (W1-072): `text`.
  - `minimum_image`, `maximum_image` — iOS's minimum- and maximum-value images (SL-5): `icon`.
  - `inset_icon` — Material Expressive's icon inside the track (MAT-S11): `icon`, `padding`.
- **Widget values:** `tick_gap_px: Value<Px>` (ticks to track; GN-1.100, K1.103, MAC-SL04, SL-6);
  Breeze's tick option is the condition `cond system_setting:slider_tick_marks` (read from breezerc, ADD-A04),
  the `tick` elements absent by a rule outside it (K1.103); `value_label_gap_px: Value<Px>` (GN-1.102);
  `value_label_position: Value<ValueLabelPosition>` (GN-1.102, GN-1.107); `value_indicator_gap_px: Value<Px>` (MAT-S6);
  `thumb_track_gap_px: Value<Px>` (Material's handle–track gap,
  MAT-S2); `stop_indicator_gap_px: Value<Px>` (MAT-S4); `image_gap_px: Value<Px>` (SL-5); `header_gap_px: Value<Px>`
  (W1-072); `snap_to_ticks: Value<bool>` (SL-9); `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (the trough's and
  keys'
  page repeat: K3.76†, K-P3-17†, GN-3.69†, MAC-X12); `trough_click: Value<TroughClick>` (Shared value
  types; GN-P4-5†, R17); `range_min_gap_px: Value<Px>` (the least distance between
  a range slider's two thumbs, W1-153†); `range_thumb_order: Value<RangeThumbOrder>`, `RangeThumbOrder { StartAbove,
  EndAbove, ActiveAbove }` (which thumb is drawn above the other where they overlap, W1-153†);
  `thumb_tooltip_placement: Placement` (where WinUI's thumb tooltip goes, the host's value, W1-073);
  `presentations: Vec<PresentationTransition>`: events `@value_indicator_show`, `@value_indicator_hide` with the
  step's part `value_indicator` (the bubble shown and hidden while dragging, its own occasion; a rule
  `@ cond reduced_motion`; MAT-S6).
- **Variants:** typed `vertical` (GNOME, Material, WinUI, macOS), `range` (KDE Quick RangeSlider, Material, WinUI via
  the Community Toolkit), `centered` (fill from a neutral value: Material's centred slider, macOS's neutral value),
  `ticks_before`, `ticks_after` (marks on one side, both for both sides: GNOME's marks, macOS's tick mark position,
  KDE, WinUI's tick placement), `hue`, `alpha`, `value` (the colour editors' sliders, `@ context color_picker`: GNOME
  GN-2.53†, Windows W2-05), `warning` (KDE's neutral highlight, K1.100, K1.105; input's `warning`). Keyed:
  `windows-11` `@ticks_inline`; `ios-*` `@thumbless`.
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. `material-3` flavour `expressive`:
  `extra_small`, `small`, `regular`, `large`, `extra_large` (Expressive's five sizes, xsmall to xlarge, S7.3). Others:
  none.
- **Flags:** shared `in_fill` (a tick or stop indicator inside the value fill: K1.103, MAT-S4, MAT-S5); keyed
  `material-3` `@overlapping` (a range thumb overlapping the other, MAT-S10), `adwaita` `@fine_tune` (also on
  scrollbar), `windows-11` `@focus_engaged` (FocusEngagementStates); state flags used: hover, pressed, focused,
  focus_visible, dragged, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (Slider CommonStates; FocusEngagementStates is the
  flag `@focus_engaged`); `material-3`: `[dragged, pressed, focus_visible, hover]` (the halo's state layer); others
  none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `fill_level`, `value_label` (all but `adwaita`), `disabled_range`, `minimum_image`,
  `maximum_image` (all but `ios-*`), `thumb_inner`, `header` (all but `windows-11`), `halo`, `stop_indicator`,
  `inset_icon` (all but `material-3`), `value_indicator` (all but `material-3`); `thumb` under
  `@thumbless`; `tick` (`kde-breeze`, by a rule `!cond system_setting:slider_tick_marks`); fields: `track.stroke` (all but `adwaita`, which draws it under `cond contrast_high`), `track.shadow`
  (all but `material-3`), `thumb.stroke`, `thumb.shadow`; values: `value_label_gap_px`, `value_label_position`,
  `value_indicator_gap_px`, `range_min_gap_px`, `range_thumb_order` (where no range slider is drawn),
  `thumb_track_gap_px`, `stop_indicator_gap_px`, `image_gap_px`,
  `header_gap_px`, `thumb_tooltip_placement` (all but `windows-11`); `repeat_delay_ms`, `repeat_interval_ms`,
  `trough_click` (`ios-*`, `material-3`: no trough page step).
- **State-model source:** `windows-11` Slider visual states (`CommonStyles/Slider_themeresources.xaml`); `adwaita`
  `_scale.scss` (hover, active, disabled, marks before and after, fine tune, vertical, high contrast); `kde-breeze`
  `breezestyle.cpp` slider branches and `breezehelper.cpp` handle rendering (mouse over, pressed, focus, neutral),
  breezerc's tick option, qqc2 `Slider.qml` and `RangeSlider.qml` for `kirigami`; `material-3` the slider tokens
  (baseline and Expressive sizes); `macos-*` NSSlider (tick mark position, control size, tint prominence, neutral
  value, key window); `ios-*` UISlider (`Style`, `TrackConfiguration`, `UIControl.State`).
- **Today's fields:** `fill_color` → `fill.fill`; `track_color` → `track.fill`; `thumb_color` → `thumb.fill`;
  `track_height_px` → `track.min_height` = `track.max_height` (and the fill's); `thumb_diameter_px` →
  `thumb.min_width`, `thumb.min_height` with `thumb.corners` full; `tick_mark_length_px` → `tick.min_height`;
  `disabled_opacity` → `body.opacity @ {disabled}`; `thumb_hover_color` → `thumb.fill @ {hover}` (or
  `thumb.fill @ context any slider {hover}` where the whole slider's hover drives it, GTK); `disabled_fill_color`,
  `disabled_track_color`, `disabled_thumb_color` → `fill.fill`, `track.fill`, `thumb.fill @ {disabled}`.
- **Notes:** WinUI's thumb tooltip is `tooltip @ context slider` (W1-073: the tooltip's style reused, its placement
  the slider's `thumb_tooltip_placement`; Nested controls). Provenance: group 6 wrote the colour sliders as keyed
  variants in the colour picker (GN-2.53†, W2-05);
  group 8 the thumb's shadow (K3.4) and the trough repeat (K3.76†, K-P3-17†, MAC-X12), one name with the repeat
  values; group 1 the trough click (GN-P4-5†, R17); the value change is the thumb's and fill's transitions (SL-9), not
  a widget value.

## Indicators

### spinner

*From group 3's catalogue.*

- **Kind:** typed widget: the indeterminate busy indicator, and the determinate circular indicator where the platform
  draws one: AdwSpinner and GtkSpinner, KBusyIndicatorWidget and qqc2 BusyIndicator, macOS's spinning
  NSProgressIndicator, UIActivityIndicatorView (and iOS's determinate circular progress, C-31), WinUI ProgressRing,
  Material's circular progress indicator and its Expressive loading indicator (MAT-N24).
- **Available:** every preset.
- **Parts:**
  - `body` — the element's box: `min_width`, `min_height`, `max_width`, `max_height` (its size limits, GN-1.228;
    today's `min_diameter_px`), `opacity`
    (disabled, GN-1.231); the container of Material's contained loading indicator: `fill`, `corners` (its colour
    and shape), `padding` (its size round the indicator) (MAT-N24).
  - `track` — the full circle under the arc (AdwSpinner, Material, WinUI's background ring): `fill` (GN-1.226,
    MAT-SN1, W1-130).
  - `indicator` — the arc (indeterminate) or the value arc or pie (determinate): `fill` (it paints the stroke),
    `transitions` (Geometry: a determinate value change, MAT-SN6), `shape` (a keyed shape of the preset's shape
    library under Material's loading indicator, morphing by interpolating transitions of the shape property group,
    MAT-X12, R25.5).
  - `spoke` — a spoke (iOS) or fin (macOS): `fill` (the leading spoke's colour).
  - `icon` — the rotated or animated icon (KDE's process-working icon, GtkSpinner's animated SVG and its
    reduced-motion replacement) as a glyph: `glyph`.
- **Widget values:** `diameter_px: Value<Px>` (the default size; today's `diameter_px`, Q1.120);
  `stroke_width_px: Value<Px>` (today's `stroke_width_px`, MAT-SN1, MAT-SN3, Q1.121; on `adwaita` an expression over the
  element's size, `= mul(element.width, <fraction>)`, the widget value's data declared under `values`,
  `data = { values = ["width", "height"] }`, GN-1.227; the determinate arc's extent, a pull to refresh's pull
  progress among them, is the connector's, drawn from the progress: no field reads it, so no data is declared for
  it, ADD-B12);
  `stroke_weight: Value<u16>` (the icon weight GTK scales the
  stroke by; GN-1.227, GN-1.232); `track_stroke_width_px: Value<Px>` (MAT-SN3); `indicator_track_gap_px: Value<Px>`
  (MAT-SN1, MAT-SN3); `cap: Value<LineCap>` (ADD-A34; GN-1.227, MAT-SN4, SP-1a, MAC-SN01); `spoke_count: Value<u32>`
  (SP-1a, MAC-SN01); `spoke_inner_radius_px`, `spoke_length_px`, `spoke_width_px: Value<Px>` (SP-1a, MAC-SN01);
  `spoke_opacity_ramp: Vec<Value<Fraction>>` (per spoke behind the leading one; SP-1a, MAC-SN01); `wave_amplitude_px`,
  `wavelength_px: Value<Px>` (under `@wavy`, MAT-SN2); `loop: Value<Loop>` (S2.7: the indeterminate motion: arc
  extension and contraction, rotation, steps, the four-colour cycle, the loading indicator's keyframes naming shapes
  (MAT-X12, R25.5), KDE's rotating icon; `Absent` by a rule `@ cond animations_off`, so no loop runs; GN-1.229, MAT-SN5, MAT-SN6, SP-2a,
  MAC-SN02, K1.230, Q1.119, Q1.120, W1-130, GN-P3-19†); `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear` (fade in on start, out on stop; SP-5a, MAC-SN05, Q1.120).
- **Variants:** typed `determinate` (macOS, WinUI, Material, iOS C-31). Keyed: `adwaita` `@gtk_spinner` (GtkSpinner,
  GN-1.230); `material-3` `@wavy`, `@thick`, `@four_color`, `@loading_indicator` (the Expressive loading indicator,
  MAT-N24), `@contained` (its contained form, MAT-N24).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular` (MAC-SN03). `ios-*`: `regular` (UIKit's medium),
  `large` (SP-3a). Others: none (sizes are the body's limits and `diameter`).
- **Flags:** none of its own; state flags used: indeterminate (WinUI's Active state; the spinner's default motion),
  disabled.
- **Exclusive groups:** none: ProgressRing's states are not a group (Active is `indeterminate`, DeterminateActive is
  the variant `determinate`, Inactive is the ring not drawn); others none (states combine); precedence: state models
  (S17).
- **may_be_absent:** parts: `track` (all but `adwaita`, `material-3`, `windows-11`), `spoke` (all but `macos-*`,
  `ios-*`), `icon` (all but `kde-breeze`, `adwaita`; `ios-*` draws none, SP-4a), `indicator` (`kde-breeze` draws
  neither arc nor spokes); values: `stroke_weight`, `track_stroke_width_px`, `indicator_track_gap_px`, `spoke_count`,
  `spoke_inner_radius_px`, `spoke_length_px`, `spoke_width_px`, `wave_amplitude_px`, `wavelength_px` (each where the
  platform draws no such shape: `stroke_weight` all but `adwaita`; the spoke values all but `macos-*`, `ios-*`; the
  wave values all but `material-3`), `loop` (every preset, by rule under `cond animations_off`: Q1.120, Q3.25).
  `spoke_opacity_ramp` and `presentations` are lists, empty where the platform has none.
- **State-model source:** `windows-11` `ProgressRing/ProgressRing.xaml` states; `adwaita` `adw-spinner-paintable.c`,
  `_spinner.scss`, `gtkspinner.c` (reduced motion); `kde-breeze` `kbusyindicatorwidget.cpp`, qqc2 `BusyIndicator.qml`
  (running, visible); `material-3` the circular progress-indicator tokens (determinate, indeterminate, Expressive
  variants) and the loading-indicator tokens; `macos-*` NSProgressIndicator (style spinning, indeterminate, control
  size, displayed when stopped); `ios-*` `UIActivityIndicatorView.Style`, animating, hides when stopped.
- **Today's fields:** `diameter_px` → `diameter`; `min_diameter_px` → `body.min_width`, `body.min_height`;
  `stroke_width_px` → `stroke_width`; `fill_color` → `indicator.fill` (arc), `spoke.fill` (spokes) or
  `icon.glyph.fill` (icon), per platform. The animated-icon model (today's spinning animated icon) becomes `loop` over
  the `icon` part's glyph.
- **Notes:** in context: a status page's spinner (GN-1.232) and a pull to refresh's progress and busy indicator
  (`variant determinate` while pulling, Q2.54†, Q1.121, C-10) are this widget in their host's context. GtkSvg's
  process-working icon is this spinner's `loop` (GN-P3-19†); GtkSvg's state animations on other
  icons are their part's icon transitions (`PropertyGroup::Icon`, `TransitionKind::Keyed`, R25.12). Provenance: group
  7 wrote the loading indicator as a widget of its own with a container part (here `body`) and a loop of path
  keyframes (ADD-B03), replaced by the shape mechanism (R25.5); group 6 proposed iOS's determinate circular progress
  (C-31) as a progress bar variant; the stroke ratio GTK derives from the size is the stroke width's expression
  (R25.3).

### progress_bar

*From group 3's catalogue.*

- **Kind:** typed widget: all six (GtkProgressBar, QProgressBar and qqc2 ProgressBar with KDE's KCapacityBar as a
  variant, NSProgressIndicator's bar style, UIProgressView and SwiftUI's ProgressView, WinUI ProgressBar, Material's
  linear progress indicator).
- **Available:** every preset.
- **Parts:**
  - `body` — the element's box: `padding` (Kirigami's vertical insets, Q1.49), `min_width`, `min_height` (its size
    limits; today's `min_width_px`; GTK's vertical bar, GN-1.111).
  - `track` — the trough: `fill`, `stroke` (today's frame; the high-contrast borders GN-1.115, W1-079; MAC-P05†),
    `corners` (GN-1.109, W1-077, MAC-P02), `shadow` (today's `shadow_enabled`), `min_height`, `max_height`,
    `min_width`, `max_width` (its thickness across the axis, today's `track_height_px`: the height when horizontal,
    the width under `vertical`, GN-1.111).
  - `indicator` — the value fill, and the moving indicator when indeterminate: `fill` (KDE's busy stripes, K1.110),
    `stroke` (KDE's outline of the fill, K1.111), `corners` (PB-2, GN-1.109, MAC-P02, MAT-P5), `min_height`,
    `max_height`, `min_width`, `max_width` (its own thickness, W1-077, MAT-P4), `transitions` (Geometry: a value
    change, PB-3, MAC-P04, W1-078, MAT-P6).
  - `secondary_indicator` — the second moving indicator of an indeterminate bar (WinUI, Material): `fill`, `corners`.
  - `stop_indicator` — Material's end dot (MAT-P3): `fill`, `corners`, `min_width`, `min_height`.
  - `buffer` — Material's buffer (secondary progress) bar (MAT-P7): `fill`, `corners`.
  - `buffer_dots` — the dots of Material's buffer track (MAT-P7): `fill`, `corners`, `min_width`, `min_height` (each
    dot's shape and size).
  - `label` — a title label (SwiftUI's label, PB-4†): `text`.
  - `value_label` — the value text (GTK's shown text, KDE's percentage, SwiftUI's current value label): `text`
    (K1.113, PB-4†), `opacity` (GTK's dimmed text, GN-1.110).
- **Widget values:** `min_indicator_length_px: Value<Px>` (the least fill a non-zero value shows, K1.114);
  `indicator_track_gap_px: Value<Px>` (MAT-P2); `stop_indicator_gap_px: Value<Px>` (MAT-P3); `wave_amplitude_px`,
  `wavelength_px: Value<Px>` (under `@wavy`, MAT-P1); `pulse_block_fraction`, `pulse_step: Value<Fraction>` (GTK's pulse
  block and step, GN-1.113); `loop: Value<Loop>` (the indeterminate motion; GN-1.113, K1.110, MAC-P01, W1-075,
  MAT-P6); `label_position: Value<ValueLabelPosition>` (K1.115, PB-4, MAC-P3-4, K1.113); `label_gap_px: Value<Px>`,
  `label_alignment: Value<Alignment>` (K1.113, PB-4, MAC-P3-4); `warning_threshold: Value<Fraction>` (the value
  above which KDE's places panel recolours its capacity bar, K1.142†: the progress is element data, so the recolouring
  is an expression, not a flag the connector sets: under `@capacity`, `indicator.fill = pick(floor(add(element.progress,
  sub(1, values.warning_threshold))), <normal>, <warning>)`, with `data = { indicator = ["progress"] }`; the sum is a
  number, at most two, whose floor is 1 from the threshold on: the expression switches at the threshold itself, so
  the value phase checks the source's comparison, "above" or "at least", and states the threshold to match).
- **Variants:** typed `vertical` (GNOME, Material, WinUI). Keyed: `adwaita` `@osd`; `kde-breeze` `@capacity`,
  `@capacity_inline`; `ios-*` `@bar`; `material-3` `@wavy`, `@thick`, `@four_color`.
- **Sizes and densities:** `macos-*`: the control sizes AppKit draws for a bar (`regular`, `small`, `mini`). Others:
  none.
- **Flags:** shared `empty` (value zero: GTK hides the fill node); keyed `windows-11` `@paused`, `@error`; state
  flags used: indeterminate, disabled.
- **Exclusive groups:** `windows-11`: `[@error, @paused]` (ProgressBar's error and paused states; `indeterminate`
  combines with them); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `secondary_indicator` (all but `windows-11`, `material-3`), `stop_indicator`, `buffer`,
  `buffer_dots` (all but `material-3`), `label` (all but `macos-*`, `ios-*`: SwiftUI's label), `value_label` (all but
  `adwaita`, `kde-breeze`, `macos-*`, `ios-*`); fields:
  `indicator.stroke` (all but `kde-breeze`, K1.111), `track.stroke`, `track.shadow`; values:
  `min_indicator_length_px`, `indicator_track_gap_px`, `stop_indicator_gap_px`,
  `wave_amplitude_px`, `wavelength_px`, `pulse_block_fraction`, `pulse_step`, `label_position`, `label_gap_px`,
  `label_alignment`, `warning_threshold` (all but `kde-breeze`).
- **State-model source:** `windows-11` `ProgressBar/ProgressBar.xaml` states; `adwaita` `_progress-bar.scss` (osd,
  vertical, shown text, empty) and `gtkprogressbar.c` (pulse); `kde-breeze` `breezestyle.cpp` progress-bar branches
  (busy, selected item, label) with breezerc's animation options, qqc2 `ProgressBar.qml`, `kcapacitybar.cpp`;
  `material-3` the linear progress-indicator tokens; `macos-*` NSProgressIndicator (bar style, indeterminate, control
  size, key window); `ios-*` `UIProgressView.Style` and SwiftUI's ProgressView.
- **Today's fields:** `fill_color` → `indicator.fill`; `track_color` → `track.fill`; `track_height_px` →
  `track.min_height` = `track.max_height`; `min_width_px` → `body.min_width`; `border.*` → `track.stroke`,
  `track.corners` (the indicator's corners become `indicator.corners`), `body.padding`; `shadow_enabled` →
  `track.shadow`.
- **Notes:** GtkTreeView's cell progress bars are `progress_bar @ context list.cell` (GN-1.279†). Provenance: the
  indeterminate loop and the value change were widget values of their own (one loop name, `loop`; the value change is
  the indicator's transitions).

### scrollbar

*From group 3's catalogue.*

- **Kind:** typed widget: all six (GTK's scrollbar, QScrollBar and qqc2 ScrollBar, NSScroller, iOS's scroll indicator,
  WinUI ScrollBar, Compose's non-interactive scrollbar); iOS and Material draw a non-interactive indicator.
- **Available:** every preset.
- **Parts:**
  - `body` — the bar's box: `min_width`, `min_height` (its thickness: today's `groove_width_px`; MAC-S07, W1-060),
    `padding` (the trough's margins and the indicator's inset from the view's edges: GN-1.85, SB-2, MAT-SC3†),
    `transitions` (its expansion and contraction, W1-060, W1-064).
  - `track` — the trough: `fill` (WinUI's acrylic), `stroke` (per side: the content-side separator line, KDE and
    legacy macOS), `corners`, `padding` (the thumb's inset), `opacity`, `transitions` (the trough appearing and
    fading: GN-1.87, GN-1.90, K1.95).
  - `thumb` *(element)* — the knob or indicator: `fill`, `stroke` (outline, knob-style border), `corners`,
    `min_width`, `min_height` (today's `thumb_width_px`, `min_thumb_length_px`), `opacity` (GN-1.89, K1.91, Q1.41),
    `transitions` (its size and colour changes: GN-1.90, K1.95, Q1.40, W1-064, MAC-S01).
  - `arrow_button` *(element)* — an arrow button (KDE's optional ones, WinUI's, the annotated bar's scroll buttons
    W2-28): `glyph`, `fill`, `stroke`, `transform` (the pressed scale), `transitions` (that scale's animation,
    W1-061), `min_width`, `min_height` (KDE's button length, K1.94), `padding` (WinUI's margins, W1-061).
  - `annotation_labels` — the labelled rail of WinUI's annotated scroll bar (W2-28): `text`, `min_width`.
- **Widget values:** `auto_hide_delay_ms: Value<Ms>` (a duration: WinUI's contract delay, the overlay scroller's hide
  delay and Material's scroll-indicator fade delay; GN-1.91, SB-3, MAT-SC1, MAT-SC3, Q1.41, W1-064, MAC-X12, MAT-X25);
  `hide_after_leave_ms: Value<Ms>` (a duration, GN-1.91); `always_visible: Value<bool>` (MAT-SC3);
  `thumb_max_fraction: Value<Fraction>` (of the viewport, MAT-SC3); `edge: Value<BoxEdge>` (the view's edge the bar
  sits on: Compose's at the end edge of a vertical bar, mirrored in RTL, and the bottom of a horizontal one, by rules
  under `vertical`, MAT-SC3†); `overscroll_shrink: Value<bool>` (the thumb
  shortens by
  the overshoot; SB-7, MAC-P3-3, K-P4-13); `arrow_buttons_start`, `arrow_buttons_end: Value<ArrowButtons>` with
  `ArrowButtons { None, Single, Double }` (read on KDE, K1.94); `trough_click: Value<TroughClick>` (Shared value types; read
  on KDE and macOS; K-RV1, MAC-P3-2, Q1.39, W-P4-16, GN-P4-5†); `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (the
  arrows' and trough's repeat; Q1.42, MAC-P3-2, W-P4-16, MAC-X12, K-P3-17†, GN-3.69†);
  `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (fade out, slide in), `@flash` (iOS's
  `flashScrollIndicators()`, the flash the application triggers; the bar's own appearance is `Appear`) (GN-1.91, SB-3, MAT-SC1, MAT-SC3, Q1.41, MAT-X25).
- **Variants:** typed `vertical` (the base is horizontal), `dark`, `light` (a knob style for light or dark content:
  macOS's knob style dark and light, iOS's indicator style black and white; the default adapts by the
  over-dark-content state). Keyed: `windows-11` `@annotated` (W2-28).
- **Sizes and densities:** `macos-*`: `regular`, `small`, `mini` (MAC-S07). Others: none.
- **Flags:** keyed `adwaita` `@fine_tune`; state flags used: hover (of the bar, and of the thumb), pressed, disabled,
  over_dark_content.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ScrollBar CommonStates, per template part); its
  ScrollingIndicatorStates are the condition `cond input:touch` with the auto-hide values, its ConsciousStates the
  bar's `{hover}` (expanded, after `auto_hide_delay`; the states without animation are transitions
  `@ cond animations_off`); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `arrow_button` (`adwaita`, `macos-*`, `ios-*`, `material-3`; `kde-breeze` where its read
  setting hides them, K1.94),
  `annotation_labels` (all but `windows-11`); fields: `track.stroke`, `thumb.stroke`,
  `track.fill` (indicators without a track); values: `hide_after_leave_ms`, `always_visible`, `thumb_max_fraction`,
  `arrow_buttons_start`, `arrow_buttons_end`, `repeat_delay_ms`, `repeat_interval_ms`.
- **State-model source:** `windows-11` `CommonStyles/ScrollBar_themeresources.xaml` visual states and
  `AnnotatedScrollBar` (W2-28); `adwaita` `_scrolling.scss` (overlay indicator, hovering, fine tune, hover, active,
  disabled, high contrast) and `gtkscrolledwindow.c` (timings); `kde-breeze` `breezestyle.cpp` scroll-bar branches
  (groove hover, thumb hover, focus of the view, arrows) with the scrollbar engine, qqc2 `ScrollBar.qml` (interactive
  and touch overlay); `material-3` Compose's non-interactive scrollbar and `ViewConfiguration`; `macos-*` NSScroller
  (scroller style, knob style, control size, key window); `ios-*` the scroll indicator (indicator style, grabbed, iOS
  26 edge effect).
- **Today's fields:** `track_color` → `track.fill`; `thumb_color` → `thumb.fill`; `thumb_hover_color` →
  `thumb.fill @ {hover}`; `thumb_active_color` → `thumb.fill @ {pressed}`; `groove_width_px` → `body.min_height`
  (horizontal) and `body.min_width @ variant vertical`; `thumb_width_px` → `thumb.min_height` and
  `thumb.min_width @ variant vertical`; `min_thumb_length_px` → `thumb.min_width` and
  `thumb.min_height @ variant vertical`; `overlay_mode` → the condition `cond overlay_scrollbars` (ADD-A11).
- **Notes:** the annotated bar's hover tooltip is `tooltip @ context scrollbar` (its min height and max width,
  W2-28; Nested controls). Conditions the rules use: `cond overlay_scrollbars` (ADD-A11), `cond input:touch`
  (Kirigami's transient
  touch overlay, WinUI's touch indicator), the contrast conditions, `cond flavor:kirigami`. The corner square and the
  view frame belong to `scroll_view`. Provenance: group 7 wrote the annotated bar's scroll buttons as a part of their
  own (here `arrow_button`); group 8 the hide delay (MAC-X12) and the repeat values; group 1 the trough click
  (GN-P4-5†); Material's fade delay (MAT-X25) is the same `auto_hide_delay`.

## Containers

### scroll_view

*From group 3's catalogue.*

- **Kind:** typed widget, added by group 3: the scrolling container round a viewport. KDE, macOS and Windows draw its
  frame and corner (Q1.43, MAC-I08, K1.96, MAC-S06, W1-065); GNOME's undershoot (GN-3.06; under a toolbar view's bar,
  GN-1.156; under a column view's header, GN-1.178) and Apple 26's scroll edge effect (iOS C-18, macOS MAC-X05) are
  its `edge_effect`: two or more platforms, so typed (R14).
- **Available:** every preset.
- **Parts:**
  - `body` — the scrolling container round the viewport (the root): `padding`, `bindings`.
  - `frame` — border and background: KDE's framed background, NSScrollView's border type: `fill`, `stroke`, `corners`.
  - `corner` — the square where two bars meet: `fill`, `transitions` (WinUI's expand and contract timing, W1-065).
  - `edge_effect` *(element)* — the shade or material at one edge, one element per edge, its edge a typed identity
    (`top`, `bottom`, `start`, `end`), drawn while content lies under that edge (its own `scrolled`): GTK's
    undershoot (`undershoot.top`, `.bottom`, `.left`, `.right`, each on its own: a shade line over a short gradient,
    with its high-contrast border, GN-3.06), Apple 26's soft or hard edge under the top bar and the bottom bar each
    (a material paint with blur and a fade mask: a linear gradient over a blur): `fill`, `stroke`.
  - `overshoot` *(element)* — GTK's overshoot gradient at one edge, one element per edge with the same identities,
    drawn while the user scrolls past that edge (its own `{@overshoot}`, GN-3.06): `fill`, `transitions`. Android's
    stretch and glow are `token.behavior.scroll.overscroll_effect`.
- **Widget values:** `scrollbar_margin: Sides<Value<Px>>` (WinUI, W1-065); `wheel_step: Axes<Value<Px>>` (the
  distance one wheel step scrolls this view, per axis: on `adwaita` `= gtk_wheel_step(element.height)` and
  `= gtk_wheel_step(element.width)`, the view's page sizes, which the widget value declares, `data = { values =
  ["width", "height"] }`, GN-P3-15†, ADD-A90; elsewhere `=
  token.behavior.scroll_px_per_step` where the platform states pixels);
  `edge_effect_height_px: Value<Px>` (C-18, MAC-X05). Which edges draw an effect is per identity: an edge the
  platform draws none on is absent by a rule on that identity.
- **Variants:** typed `flat`, `line` (as on text_area; input's `line`); typed on `edge_effect` and `overshoot` the
  edge identities `top`, `bottom`, `start`, `end` (`BoxEdge`; `start` and `end` mirrored in RTL); typed on `edge_effect` `soft_edge`,
  `hard_edge` (UIKit's and AppKit's styles; their automatic style is the application's choice, not a look; C-18,
  MAC-X05). Keyed: `kde-breeze` `@framed` (Kirigami's framed background style hint); `macos-*` `@bezel`, `@groove`.
- **Sizes and densities:** none.
- **Flags:** keyed `adwaita` `@overshoot` (on an `overshoot` element: the user scrolls past its edge, GN-3.06: an
  interaction state, not content, a size or a colour, so it is a flag; only GNOME draws a part for it, so it is keyed,
  S7.2; the other platforms' overscroll is `token.behavior.scroll.overscroll_effect`); state flags used: focus_within
  (on the scroll view: KDE's thumb follows the view's focus, K1.92), scrolled (on an `edge_effect` element: content
  lies under its edge, set per edge by the connector, GN-1.156; a bar's own `scrolled`, `toolbar`'s or `window`'s, is
  that element's).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `frame`, `corner` (`adwaita`, `ios-*`, `material-3`: drawn by KDE, macOS and Windows,
  Q1.43, MAC-I08, K1.96, MAC-S06, W1-065), `edge_effect` (`windows-11`,
  `kde-breeze`, `material-3` draw none; elsewhere absent by rule outside `{scrolled}`), `overshoot` (all but
  `adwaita`; absent by rule outside `{@overshoot}`); fields: `edge_effect.stroke` (outside `cond contrast_high`);
  values: `scrollbar_margin` (all but `windows-11`), `edge_effect_height_px` (all but `ios-*`, `macos-*`),
  `wheel_step` (where the platform states steps in lines, `token.behavior.scroll_lines_per_step`: all but `adwaita`).
- **State-model source:** `kde-breeze` the frame and corner branches of `breezestyle.cpp`, qqc2 `ScrollView.qml`;
  `macos-*` NSScrollView (border type, scroller style) and AppKit's scroll edge effect styles; `windows-11`
  `CommonStyles/ScrollViewer_themeresources.xaml` and `ScrollView_themeresources.xaml`; `adwaita` the undershoot and
  overshoot nodes of the scrolled window, `_toolbars.scss` and `_column-view.scss` (GN-1.156, GN-1.178); `ios-*`
  `UIScrollEdgeEffect.Style`; `material-3` none (no edge effect).
- **Today's fields:** none.
- **Notes:** in context: `scroll_view.edge_effect @ variant top {scrolled}` under a toolbar view's top bar (and
  `variant bottom` under its bottom bar, GN-1.156), and `scroll_view.edge_effect @ variant top {scrolled} context
  list variant table` under a column view's header (GN-1.178); each edge element carries its own `scrolled`. Provenance: group 7's typed scroll edge effect (iOS C-18, macOS MAC-X05) and group 8's
  scroll area are this widget; the list's and toolbar's undershoot parts went into `edge_effect`.

### list

*From group 4's catalogue.*

- **Kind:** typed widget. Every platform draws it (list, table, tree, grid or icon view): GNOME (GtkListView,
  GtkColumnView, GtkListBox, AdwSidebar's rows, the deprecated GtkTreeView and GtkIconView, GtkStackSidebar's rows
  GN-2.34, the file chooser's places rows GN-P3-22†), KDE (Qt item views, the qqc2 ItemDelegate family, Kirigami
  delegates, the KIO places view, kirigami-addons' delegates and table view), macOS (NSTableView, NSOutlineView,
  source lists, NSCollectionView MAC-N13, NSBrowser MAC-N08), iOS (UICollectionView lists, UITableView, SwiftUI Table
  and Form), Material (lists, data table), Windows (ListView, GridView items, ItemsView, the legacy ListBox,
  SwipeControl, NavigationView items through the variant `sidebar`). Trees are lists with disclosures (R14's
  cross-check); the "tree view", "grid view", "items view", "list box" and "swipe actions" kinds group 7 lists (W2-12,
  W2-13, W2-14, W2-33, W2-29, W-P4-11†) are this widget by the same test.
- **Available:** every preset.
- **Parts:**
  - `view` — the list's own surface: `fill` (its disabled fill, GN-1.176), `bindings` (it binds the View surface
    set), `stroke` (the frame, MAC-L16, MAT-L13), `corners`, `shadow` (when the whole list is a card: GNOME's boxed
    list), `padding`, `min_width`, `min_height` (a list's minimum size in its host, K-P4-10†), `focus` (the table's
    own focus ring, MAC-L14).
  - `section` — the platter that holds one section's rows (iOS inset-grouped, L-2): `fill`, `corners`, `stroke`,
    `shadow`.
  - `row` *(element)* — one item: `padding`, `min_height` (Auto where content-sized, L-9), `text` (the primary text),
    `opacity` (per-item disabled fade, W1-112), `transform` (reorder scale, W1-110), `transitions`; its focus
    indicator lives on `row_background`. `row_background`, `row_inner_border`, `row_selection_indicator`,
    `leading_icon`, `secondary_text`, `trailing_text`, `overline_text`, `trailing_icon`, `unread_indicator`,
    `swipe_area`, `swipe_handle`, `nested` belong to `row`, nest in it and are queried with it; the *(element)* parts
    `cell`, `disclosure`, `accessory` and `swipe_action` nest in it as elements of their own (a rule on them may
    select the row's states by `context list.row {…}`). In a list that draws sections (iOS) the rows nest in
    `section`.
  - `row_background` — the shape drawn for the row's states inside the row (GNOME's whole-row fill, KDE's inset
    bubble, macOS's inset selection, WinUI's rounded backplate, Material's container and Expressive shapes): `fill`,
    `stroke` (KDE's outline, K1.171; kirigami-addons' focused border, K-KA-38†), `corners` (per corner, so merged
    selections round only the ends, MAC-L02; per state, MAT-L8), `shadow` (separate boxed-list cards, dragged
    elevation; iOS's cell background configuration, CD-1: its material fill, stroke with its `outset`, corners and
    shadow, its insets `values.row_background_inset`), `focus` (K1.175), `transitions` (the deselection fade L-11,
    GN-1.165's row transitions).
  - `row_inner_border` — WinUI's inner selection border of a grid item (W2-13, W2-14): `stroke`.
  - `row_selection_indicator` — WinUI's selection pill (W1-107; W1-089 in the sidebar variant; the tree's, W2-12);
    Material's navigation rail and drawer active indicator (MAT-N13, rename row 18): `fill`, `corners`, `min_width`,
    `min_height`, `transitions`.
  - `row_separator` *(element)* — the line between two rows, an element whose previous and next siblings are rows
    (it hides beside a hovered, selected or dragged row: GN-1.162, L-3, MAT-L6, boxed-list separators GN-1.165,
    kirigami-addons' K-KA-38†): `stroke`, `opacity`, `transitions`.
  - `column_separator` — the vertical line between cells (GN-1.169, MAC-L06, the column browser MAC-N08): `stroke`.
  - `column_resize_handle` — the column browser's resize handle (MAC-N08): `glyph`.
  - `cell` *(element)* — one cell in a multi-column row: `padding`, `corners` (KDE rounds the selection on a row's
    first and last cells only, by position among cells, K1.170), `fill` (KDE paints the selection per cell; a shaded
    sort column, K1.183), `stroke` (GNOME's editable-cell focus and editing outlines, GN-1.172; kirigami-addons'
    current cell, K-KA-37†).
  - `header` — the header row: `fill`, `stroke` (the header's bottom line), `min_height`, `padding` (its insets per
    table style, MAC-L01).
  - `header_section` *(element)* — a column header button: `fill`, `text`, `padding`, `transitions` (hover fade),
    `opacity`, `transform` (while dragged, MAC-L15, GN-1.171), `cursor`, `stroke` (while dragged or a drop target,
    K-KA-36†). `sort_indicator` belongs to `header_section` and is queried with it.
  - `header_separator` — the line between header sections (K1.178, MAC-L05, GN-1.169): `stroke`, `cursor` (the
    column-resize cursor over it, MAC-X20).
  - `header_corner` — the corner piece beside the header above the vertical scroll bar (K1.178, MAC-L18†): `fill`,
    `stroke` (its separators, MAC-L18†).
  - `sort_indicator` — the glyph in a sorted header section (GN-1.168, K1.179, MAC-L05's ascending and descending
    named images, MAT-L12, L-22†): `glyph`, `transform`, `transitions` (kirigami-addons' rotation for descending,
    K-KA-36†).
  - `section_header` *(element)* — a section's heading or group row (GN-1.163, L-4, MAC-L09, MAC-SB03, SD-4, MAT-SB4,
    W1-093, Q1.63, Kirigami's list section header Q2.41): `text`, `fill` (sticky plain header, group-row material),
    `padding`, `min_height`, `icon` (Kirigami's optional icon, Q1.63, Q2.41). `section_header_line` belongs to
    `section_header` and is queried with it.
  - `section_header_line` — the line filling the rest of Kirigami's list section header (Q2.41): `stroke`.
  - `inline_header` — Kirigami's inline view header (and footer, under `@inline_footer`: its line on the top edge)
    with its tint and separator (Q2.42): `fill`, `stroke`, `padding`, `text`.
  - `section_footer` — a section's footer text (L-5): `text`.
  - `footer` — a table's footer bar (MAT-L12): `fill`, `min_height`, `text`.
  - `disclosure` *(element)* — a tree row's expander arrow, an expandable item's chevron, iOS's outline disclosure
    (`UICellAccessory.outlineDisclosure`, its header and cell styles keyed, EX-1–EX-3) or the column browser's
    branch arrow (GN-1.173, K1.176, MAC-L10, MAC-N08, L-21†, Q1.85, MAT-L9, W1-093, W2-12): `glyph`, `fill`,
    `corners` (Material's chevron container), `transform` (rotation), `transitions` (the rotation's and K-KA-40†'s
    recolouring).
  - `branch_line` — tree branch lines (K1.176, Q1.85, GN-1.279†, W2-12): `stroke`.
  - `leading_icon` — the row's leading image, icon or avatar (L-8, SD-5, MAT-L2, Q1.80, Q1.87, K1.181, MAC-SB01,
    MAC-SB02): `icon` (its style and size), `corners`, `min_width`, `min_height` (an image's or video's reserved
    size, L-8, MAT-L2), `transitions` (WinUI's settings gear, an AnimatedIcon: `TransitionKind::Keyed` of group `Icon`,
    W3-28, ADD-A27).
  - `secondary_text` — subtitle, supporting or value text (L-7, MAT-L1, Q1.87, MAC-L04): `text`.
  - `trailing_text` — trailing supporting text or a value cell's trailing value (MAT-L1, L-7, MAT-SB5): `text`.
  - `overline_text` — Material's overline (MAT-L1): `text`.
  - `trailing_icon` — Material's trailing icon (MAT-L3): `icon`.
  - `accessory` *(element)*, in `row` — a system accessory, its identity carried as a variant (L-12), and the reorder
    handle two platforms draw, typed `reorder` (iOS's reorder control, L-12; Kirigami's `ListItemDragHandle`, Q2.43):
    `glyph` (with its size), `icon` (Kirigami's `handle-sort` icon, Q2.43), `fill`, `opacity` (the handle's reduced
    rest opacity, full `@ {hover}` and `@ {pressed}`, Q2.43), `cursor`, `padding`, `transform` (the delete control's
    rotation in edit mode, L-14), `transitions` (that rotation's, L-14); its side is `values.accessory_placement`.
  - `unread_indicator` — kirigami-addons' unread mark on an indicator row (K-KA-39†): `fill`, `min_width`, `padding`.
  - `swipe_area` — the area a swipe reveals (Q1.89, L-13): `fill`, `shadow` (at its edges).
  - `swipe_action` *(element)* — an action button there (L-13, MAC-L12, MAT-L11, Q1.88, W2-29): `fill`, `text`,
    `icon`, `corners`, `opacity` (Kirigami's actions fading in on hover, Q1.88), `transitions`.
  - `swipe_handle` — Kirigami's handle that opens the actions (Q1.88): `icon`.
  - `drop_indicator` — the insertion line (and its end circle) between rows (MAC-L11, K1.182, GN-1.279†, W1-110), or
    the drop zone's area where the platform fills one (Material's drop-zone colour, MAT-L10): `stroke`, `glyph` (the
    end circle), `corners`, `fill`.
  - `section_index` *(element)* — iOS's A–Z index (L-15): `text`, `fill` (its tracking background is its pressed
    fill).
  - `nested` — the box holding an expanded row's nested rows (GNOME's `list.nested` under an AdwExpanderRow in a
    boxed list, GN-1.260): `fill`, `padding`.
- **Widget values:** `row_background_inset: Sides<Value<Px>>` (rules by position: K1.170's larger first-row top;
  K-KA-38†, K-KA-39†; a sidebar's selected row, MAC-SB04); `row_gap_px: Value<Px>` (GN-1.166, Q1.79, MAT-L6, MAT-L8);
  `intercell_gap: Axes<Value<Px>>`
  (MAC-L07); `row_separator_inset: Sides<Value<Px>>` and `row_separator_aligns_to_text: Value<bool>` (L-3);
  `section_separators: SectionSeparators` with `SectionSeparators { top: Value<bool>, bottom: Value<bool> }` (L-3);
  `section_inset: Sides<Value<Px>>` (L-2, MAC-L01, SD-2); `indent_per_level_px: Value<Px>` (GN-1.173, K1.176,
  MAC-L10, L-21†, Q1.85, W2-12); `indent_includes_disclosure: Value<bool>` (GN-1.279†, K-KA-40†);
  `separators_follow_indent: Value<bool>` (L-21†); `disclosure_gap_px: Value<Px>`; `leading_gap_px: Value<Px>` (MAT-L7,
  Q1.87, L-7, W1-092); `secondary_text_gap_px: Value<Px>` (L-7); `trailing_gap_px: Value<Px>`;
  `secondary_text_stacks: Value<bool>` (the accessibility-size layout, a rule under the large text sizes, L-19);
  `section_headers_sticky: Value<bool>` (L-4); `filler_rows: Value<bool>`, `filler_row_height_px: Value<Px>` (L-17);
  `header_fills_trailing: Value<bool>` (Q1.86); `header_item_gap_px: Value<Px>` (K1.179);
  `header_text_alignment: Value<Alignment>` (MAC-L05); `sort_indicator_inverted: Value<bool>` (read; K1.179);
  `column_min_width_px: Value<Px>` (K-KA-36†); `check_box_placement: Value<Key>` (the grid item's check box corner, keys
  declared by the preset; W2-14); `accessory_placement: Vec<(VariantRef, Side)>` (each accessory identity's side,
  leading or trailing: `UICellAccessory.Placement`, L-12); `autoscroll_step_px: Value<Px>` (an expression on
  Kirigami's grid unit, `token.params.layout`), `autoscroll_interval_ms: Value<Ms>` (the list scrolling while a row
  is dragged by its handle near an edge, Q2.43); `focus_visible_when` (`FocusVisibleWhen`, S2.11; Q1.78, K1.175,
  MAC-L14);
  `swipe_full_expansion: Value<bool>` (L-13); `swipe_threshold_px: Value<Px>` (the swipe distance past which the
  actions open or the full expansion triggers, X-BT-1); `reorder_hint_offset_px: Value<Px>` (W1-110);
  `selection_indicator_edge: Value<BoxEdge>` (the row edge `row_selection_indicator` sits on: the leading edge of a
  NavigationView item, W1-089); `max_columns: Value<u32>` (the columns a table shows: SwiftUI's `Table` keeps only
  the first in a compact width, by a rule `@ variant table h_size_class compact`, L-22†); `control_alignment:
  Value<Alignment>` (where a control nested in a row sits, the settings row's trailing value and control, L-23†);
  `drag_hover_delay_ms: Value<Ms>` (a dragged item hovering a collapsed row expands it; GN-1.279†, W-P4-11†);
  `typeahead_hide_ms: Value<Ms>` (GN-1.279†'s interactive search); `elided_tooltip: Value<bool>` (an elided row shows
  its full text in a tooltip after the tooltip's own show delay, by press and hold `@ cond tablet_mode`; Q1.82,
  K-KA-36†); `presentations: Vec<PresentationTransition>`, queried with a row for the row's events (`Appear`, a row
  inserted; `Disappear`, a row deleted; `Move`, a row moved to its new place, its distance `element.move_distance`,
  declared under `values`; `@expand`, `@collapse`, its nested rows revealed and hidden; `@swipe`, its swipe actions
  revealed and the full-swipe expansion, W2-29, L-13, Q1.84, Q1.89; `@drag_lift`; `@placeholder_fade`, a
  not-yet-loaded row filled in, W-RV-2†; `@spring_load`, a spring-loaded row's flash or blink while a drag hovers it,
  MAC-X21, P3-11†) and with the list for the list's (`@reload`, `@edit_mode`, and `@indicator_move` with the step's
  part `row_selection_indicator`, WinUI's pill sliding to the selected row, W1-090) (L-14, L-16, W1-113, K1.185†).
- **Variants:** typed `plain` (iOS plain, macOS plain), `inset` (iOS inset-grouped, macOS inset, GNOME's boxed list),
  `separated` (GNOME's separate boxed list, Material Expressive's segmented list), `sidebar` (iOS sidebar, macOS
  source list, GNOME's navigation sidebar and AdwSidebar, WinUI NavigationView items, Material drawer items, KDE
  drawer items and side panels), `table` (multi-column with a header: GNOME's column view, KDE table views, macOS
  NSTableView, iOS Table, Material's data table), `grid` (GNOME grid view, flow box and icon view; KDE icon mode;
  WinUI GridView; macOS NSCollectionView; MAC-N13, W2-13, W2-14), `row_separators` (GNOME's separators, macOS
  horizontal grid, Material dividers), `column_separators` (GNOME's column separators, macOS vertical grid),
  `alternating_rows` (KDE, macOS), `reorder` on `accessory` (the reorder handle: iOS, KDE's Kirigami; Q2.43, L-12),
  `horizontal` (a horizontal list: GTK's, its row separators on the start edge,
  GN-1.162; the orientation many widgets share), `destructive` on `swipe_action` (iOS, macOS), the content variants
  `icon_only` on `row` (a navigation rail item with no label, MAT-N13; ADD-A61), `with_subtitle` on `row` (a row
  with a secondary line: Kirigami's subtitle delegate Q1.87, iOS's subtitle cell, Material's two-line item MAT-L1;
  ADD-A61), `three_line` on `row` (Material's three-line item, MAT-L1) and `with_avatar`, `with_image`,
  `with_video` on `row` (the leading element's kind, Material's reserved sizes, MAT-L2; ADD-A61). Keyed: `adwaita` `@rich_list`,
  `@data_table`, `@tree_view`, `@icon_view` (the deprecated GtkTreeView and GtkIconView, GN-1.279†, GN-1.280†),
  `@stack_sidebar` (GtkStackSidebar's rows, GN-2.34), `@new_bookmark`, `@drop_placeholder` (the file chooser's places
  rows, GN-P3-22†); `kde-breeze` `@column_view` (QColumnView, K1.184†), `@ka_rounded`
  (K-KA-38†, K-KA-39†, K-KA-46†), `@ka_table` (K-KA-36†, K-KA-37†), `@ka_tree` (K-KA-40†), `@ka_indicator` (K-KA-39†),
  `@inline_footer` (Kirigami's
  inline view header placed as the view's footer, Q2.42),
  `@emphasized` (the subtitle item's emphasised form, K-KA-39†), `@check`, `@radio` (rows holding a check box or
  radio, K-KA-38†); `ios-*` `@grouped`,
  `@sidebar_plain`, `@prominent_header`, `@extra_prominent_header` (on `section_header`), `@value_cell`,
  `@accompanied_sidebar_cell`, on `accessory` the identities `@disclosure_indicator`, `@checkmark`, `@detail`,
  `@delete`, `@insert`, `@label`, `@popup_menu` (L-12, CK-1; the reorder control is the typed `reorder`), on
  `disclosure` the outline disclosure's
  styles `@header`, `@cell` (EX-1–EX-3);
  `macos-*` `@full_width` (automatic is the base), `@row_size_small`, `@row_size_medium`, `@row_size_large` (MAC-L08;
  a list
  stating its own row size style has that as a variant), `@dashed_horizontal_grid`; `windows-11` `@listbox` (W2-33).
- **Sizes and densities:** `macos-*`: the row size by variant (above) and by the condition
  `cond container_size(sidebar, small)` (and medium, large) for source lists at the default row size (MAC-SB01);
  `windows-11`: density `compact` (the tree's row height, W2-12); `ios-*`: none (Dynamic Type, L-9: the row's min
  height is Auto with a minimum); others none.
- **Flags:** shared `editing` (row, cell; GN-1.172, L-10, L-14), `swiped` (row; L-10, Q1.84), `context_menu_open`
  (row: the row whose popup or context menu is open, GN-1.160, MAC-L19†; kept apart from `open`, which a tree row uses
  for expanded), `sorted`, `descending` (header_section; K1.177, MAC-L18†, K-KA-36†), `current` (row, cell; K-KA-37†,
  K-KA-38†), `placeholder` (row: a not-yet-loaded item, its parts styled by `{placeholder}` rules, W-RV-2†); keyed
  `kde-breeze` `@sort_column` (cell; K1.183),
  `@hidden_place` (row; KDE's hidden places, K1.142†),
  `@expandable` (row: an item with children in a Kirigami drawer, Q1.62, Q1.66), `@unread` (row; K-KA-39†);
  `windows-11`
  `@reorder_target` (W1-110), `@pre_threshold`, `@post_threshold` (swipe_action; W2-29); state flags used: hover,
  pressed, focused, focus_visible, focus_within, selected, checked, disabled, open (expanded), dragged, drop_target.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ListViewItemPresenter's common states; `selected`
  combines with them); `material-3`: `[dragged, pressed, focus_visible, hover]` (one state layer at a time); others
  none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `section`, `section_index`, `section_footer` (all but `ios-*`: L-2, L-15, L-5),
  `accessory` (all but `ios-*` and `kde-breeze` flavour `kirigami`: L-12, Q2.43), `row_inner_border` (all but `windows-11`: W2-13), `row_selection_indicator` (all but `windows-11`,
  `material-3`: W1-107, MAT-N13), `nested` (all but `adwaita`),
  `column_separator` (all but `adwaita`, `macos-*`, `kde-breeze`: GN-1.169, MAC-L06, KDE's table grid; on `adwaita`
  and `macos-*` absent by a rule outside `column_separators`),
  `column_resize_handle` (all but `macos-*`, MAC-N08), `header_corner` (all but `kde-breeze`, `macos-*`: K1.178,
  MAC-L18†), `footer`, `overline_text`, `trailing_icon` (all but `material-3`: MAT-L12, MAT-L1, MAT-L3), `branch_line`
  (all but `kde-breeze`, `adwaita`, `windows-11`: K1.176, GN-1.279†, W2-12), `section_header_line`, `inline_header`,
  `unread_indicator`, `swipe_handle` (all but `kde-breeze`: Q2.41, Q2.42, K-KA-39†, Q1.88); every one of them, and
  `header`, `header_section`, `sort_indicator`, `disclosure`, `swipe_area`, `swipe_action`, `drop_indicator`, also by
  rule where the list's variant or state shows none; `row_background` (by rule where the row draws no shape: a
  swiped Kirigami row, Q1.84); `row_separator` (`kde-breeze` outside `@ka_indicator`, by rule, K-KA-39†; `ios-*` under the variant
  `sidebar`; `adwaita`, `macos-*`, `windows-11`, `material-3` by a rule outside the variants that show it,
  `row_separators` and `inset`, GN-1.162, MAC-L06, MAT-L6); fields: `view.shadow`, `section.stroke`, `section.shadow`,
  `cell.stroke`, `header_section.stroke`,
  `header_section.cursor`, `header_separator.cursor` (each where the platform draws none: the boxed list, iOS's
  sections, GNOME's cell outlines, kirigami-addons' header drag), `row_background.stroke`, `row_background.shadow`,
  `row_background.fill` (every preset: a row at rest paints nothing on most platforms, and the states' rules state
  it), `row_background.focus` (where the platform draws no row focus indicator: the indicator is on `view`, MAC-L14,
  or on the row's control);
  values: `filler_rows`, `filler_row_height_px` (all but `ios-*`: L-17), `check_box_placement`,
  `reorder_hint_offset_px` (all but `windows-11`: W2-14, W1-110), `column_min_width_px` (all but `kde-breeze`:
  K-KA-36†), `typeahead_hide_ms` (all but `adwaita`: GN-1.279†), `swipe_threshold_px` (`adwaita`: no swipe actions),
  `max_columns` (no limit: all but `ios-*`, L-22†), `control_alignment` (all but `ios-*`: L-23†),
  `selection_indicator_edge` (all but `windows-11`, `material-3`), `autoscroll_step_px`, `autoscroll_interval_ms`
  (all but `kde-breeze` flavour `kirigami`: Q2.43). `accessory_placement` and `presentations` are lists: empty, not
  absent, where the platform has none.
- **State-model source:** `adwaita`: libadwaita selectors in `_lists.scss`, `_column-view.scss`, `_views.scss`,
  `_sidebars.scss`, `_deprecated.scss` (tree and icon views); `kde-breeze`: Breeze's item-view panel, header and
  branch branches (`st:4751-4900`, `st:6808-6900`, `st:5229-5299`) and qqc2's `DefaultListItemBackground.qml` and
  `ItemDelegate.qml` states, kirigami-addons' delegates and table view; `macos-*`: NSTableView's style, row size
  style, the cell view's background style (normal, emphasized), key and main window, first responder; `ios-*`:
  `UICellConfigurationState` (highlighted, selected, focused, disabled, editing, swiped, expanded, drag and drop
  states) per `UICollectionLayoutListConfiguration.Appearance`; `material-3`: the list, data-table, list-expand,
  list-reorder and list-reveal tokens (rest, hover, focus, pressed, dragged, disabled, each selected); `windows-11`:
  ListViewItem's and GridViewItem's visual state groups (CommonStates, DisabledStates, MultiSelectStates,
  DataVirtualizationStates, ReorderHintStates, DragStates), ItemContainer's, ListBoxItem's, SwipeControl's, and
  NavigationViewItem's for the `sidebar` variant.
- **Today's fields:** `background_color` → `view.fill`; `item_font` → `row.text`; `alternate_row_background` →
  `row_background.fill @ pos parity odd variant alternating_rows` (ADD-B06; `ios-*` declares no alternation, L-18);
  `selection_background` → `row_background.fill @ {selected}`; `selection_text_color` → `row.text.fill @ {selected}`;
  `header_font` → `header_section.text`; `header_background` → `header.fill` (and `header_section.fill`); `grid_color`
  → `row_separator.stroke` and `column_separator.stroke`; `row_height_px` → `row.min_height`; `header_height_px` →
  `header.min_height`; `hover_background` → `row_background.fill @ {hover}`; `hover_text_color` →
  `row.text.fill @ {hover}`; `disabled_text_color` → `row.text.fill @ {disabled}`; `border` → `view.stroke`,
  `view.corners`, `view.shadow` (from `shadow_enabled`), `row.padding` (its padding, which presets used for the row).
- **Notes:** in context: the marquee selection rectangle is `rubber_band @ context list` (GN-1.175, GN-1.280†); the
  dragged row's image and count are `drag_image @ context list` (K-RL06†, W1-110); a menu's rows are
  `list.row @ context menu.body` (GN-1.79); the multi-select check is a `checkbox` in `context list.row` (iOS's
  circle and GNOME's selection mode `variant selection`, CK-1, CK-2†; WinUI's presenter check box W1-109, the grid
  items' W2-13, W2-14, the tree's W2-12, kirigami-addons' K-KA-46†); an avatar in a row is `avatar @ context list.row`
  (MAT-L2);
  KDE's container styles `@places`, `@side_panel`, `@global`, `@context` are the sidebar's variants, matched as
  `list.row @ context sidebar.pane variant @places`; the badge on a row is `badge @ context list.row`; GNOME's column
  view undershoot is `scroll_view.edge_effect @ context list variant table` (GN-1.178); drop-target rules on
  `row_background` and `view` (GN-3.66†, MAC-X21, W3-49†); alternating rows by the parity selector (Q3.13, ADD-B06).
  Provenance: group 6 added Kirigami's section header line and inline header (Q2.41, Q2.42); group 7 the column
  browser (MAC-N08), the grid items (MAC-N13, W2-13, W2-14), the list box (W2-33), the tree (W2-12), the swipe control
  (W2-29) and the drag-hover delay (W-P4-11†); group 4 had a rubber band part and a drag image value here; KDE's
  item-row style needs no variant of its own (alternation is `alternating_rows`).

### tab

*From group 4's catalogue.*

- **Kind:** typed widget: tabs attached to a page or a document strip: GNOME (GtkNotebook, AdwTabBar), KDE (QTabBar
  and QTabWidget, qqc2 TabBar), macOS (NSTabView), Material (primary and secondary tabs), Windows (TabView). iOS's
  UITabBar is the app-level bottom tab bar, a different kind: its rows go to `bottom_tab_bar`.
- **Available:** every preset but `ios-*` (`available = false`: no such control; UITabBar is `bottom_tab_bar`).
- **Parts:**
  - `bar` — the strip (the root): `fill` (paint with material: Mica Alt, W1-088), `stroke` (its base line or rule on
    the page side: GN-1.116, K1.127, MAT-TB5, GN-1.125's shade line), `padding` (GN-1.121, W1-085), `min_height`
    (GN-1.125), `min_width` (Q1.52), `corners`, `shadow`, `bindings` (the surface set the strip draws on: AdwTabBar's
    Header set, GN-1.125; Kirigami's Window set, Q1.52), `transitions` (AdwTabBar's backdrop change, GN-1.125).
  - `tab_box` — AdwTabBar's box holding the tabs inside the bar (GN-1.125): `padding`.
  - `start_actions`, `end_actions` — AdwTabBar's start and end action boxes (GN-1.125): `padding`.
  - `tab` *(element)* — one tab: `fill`, `stroke` (W1-086's gradient border on the sides and top; GN-1.135's
    high-contrast inset border), `corners` (K1.121 on the bar side only), `padding`, `min_width`, `max_width`,
    `min_height`, `max_height` (W1-085, GN-1.129), `opacity`, `shadow` (dragged, GN-1.133), `focus` (AdwTabBar's focus
    ring on the tab, GN-1.134), `transitions` (hover fade K1.132; GN-1.131's background transition; X-MO-6). `label`,
    `icon`, `indicator` and `attention_indicator`
    belong to it and are queried with it.
  - `label` — the tab's label: `text` (font, elision: per platform, a fade at the edges on GNOME
    GN-1.129), `focus` (the focus indicator of kind Underline where focus is a line under the label, K1.129).
  - `icon` — the tab's icon: `icon` (its style per state, W1-081, K1.130, MAT-TB4).
  - `scroll_arrow` *(element)* — overflow arrows (GN-1.119, K1.125, W1-083): `fill`, `glyph`, `padding`, `margin`
    (GNOME's negative margins, GN-1.119), `min_width` (WinUI's scroll-button width, W1-083).
  - `separator` *(element)* — the line between two tabs, an element whose previous and next siblings are tabs
    (GN-1.126, K1.123, W1-084, MAC-TB06†): `stroke`, `padding` (its insets, GN-1.126), `opacity`, `transitions` (its
    fade, GN-1.126); hidden beside a selected or hovered tab by position rules on the previous and next sibling
    (ADD-B07).
  - `indicator` — the tab's indicator, drawn under the selected tab (today's active indicator; GNOME's notebook bar,
    also on hover GN-1.117; KDE's strip K1.120; Material's MAT-TB1, MAT-TB2), a part of the `tab` element, absent by
    rule on the others: `fill`, `corners`, `min_width`, `min_height` (its thickness and length; today's
    `active_indicator_width_px`).
  - `attention_indicator` — GNOME's attention line under a tab that needs attention (GN-1.128; a line, not the
    needs-attention dot, which is a `badge`), a part of the `tab` element: `fill`, `min_height` (its thickness),
    `min_width`,
    `max_width` (the bounds of its width), `transform` (its offset below the tab).
  - `drop_indicator` — the bar a drop would land at (GN-1.127): `fill`, `min_width` (its thickness), `padding` (its
    margins).
  - `overflow_edge` — KDE's tear indicator where tabs are cut off (K1.126): `stroke`.
  - `pane` — the page the bar belongs to (K1.128, MAC-TB01, GN-1.122, W1-088): `fill`, `stroke`, `corners` (cut
    where the bar touches, by a rule under the bar's position), `padding`.
- **Widget values:** `tab_gap_px: Value<Px>` (negative where tabs overlap: Q1.50, K-P3-4†);
  `alignment: Value<Alignment>` (`Start`, `Center`; K1.131, read); `bar_pane_overlap_px: Value<Px>` (MAC-TB02);
  `base_width_px`,
  `pinned_width_px: Value<Px>` (AdwTabBar's widths, GN-1.129); `expanding: Value<bool>` (K1.123);
  `indicator_edge: Value<BoxEdge>` (rules by the bar's position variant; today's
  `active_indicator_side`); `indicator_length: Value<IndicatorLength>` with `IndicatorLength { Label, Tab }`
  (MAT-TB2); `attention_width_fraction: Value<Fraction>` (GN-1.128); `outward_corner_radius_px: Value<Px>` (today's
  `active_outward_corner_radius_px`: the outward corners the selected tab's shape flares into, a value of the
  strip); `close_button_shown: Value<CloseButtonShown>` with
  `CloseButtonShown { Always, OnHover, SelectedOrHover }` (W1-082), the padding change as rules on the tab's padding
  `@ variant closable`; `overflow: Value<TabOverflow>` with `TabOverflow { Scroll, ScrollButtons }` (K-P3-4†,
  MAT-P4-6†, GN-1.119); `scroll_to_selected: Value<ScrollToSelected>` with
  `ScrollToSelected { Center, VisibleWithMargin }` (MAT-P4-6†, K-P3-4†); `scroll_margin_px: Value<Px>`;
  `drag_hover_delay_ms: Value<Ms>` (a drag hovering a tab switches to it; GN-1.132, K1.133†); `focus_visible_when`
  (`FocusVisibleWhen`, S2.11; Q1.51); `icon_text_gap_px: Value<Px>` (K1.130, MAT-TB4);
  `separator_inset: Sides<Value<Px>>` (W1-084); `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (the scroll arrows'
  auto-repeat while held, K-RL07†); `presentations: Vec<PresentationTransition>`, queried with a tab for its own
  events and with the strip for the strip's: a tab's `Appear`,
  `Disappear` (a tab opening and closing: GN-1.131; macOS's window tabs, MAC-X32†), `Move` (a tab moved to its new
  place: a reordered tab, GN-1.131, and Qt's slide, shorter for a short move, its duration an expression over
  `element.move_distance`, which the values declare, `data = { values = ["move_distance"] }`, K1.133†, ADD-A87; tabs
  parting to open a gap for a dragged one, MAC-SF07†), `@attention` (GN-1.128), `@drag`, `@tear_out` (W1-087,
  W-SF09†), `@spring_load` (a tab a drag hovers switching to it, MAC-X21); the strip's `@scroll`, `@resize`
  (GN-1.131) and `@indicator_move` (the selection moving to another tab, one event: a step on part `indicator`, the
  indicator sliding to the new tab, MAT-TB6, and a step on part `pane`, the page switching, MAC-X11).
- **Variants:** typed on `bar` `top`, `bottom`, `left`, `right` (bar position: GNOME's notebook, KDE, macOS; absolute
  positions, as GtkNotebook, QTabWidget and NSTabView name them, so a platform that mirrors them in RTL states it by
  `dir rtl` rules); typed on `tab` and `bar` `document` (browser-style strip: AdwTabBar, KDE's document mode, WinUI
  TabView, macOS's window tabs in `context window.title_bar`, MAC-W09; pane-attached is the default), `closable`
  (GNOME, KDE, Windows); typed content variant `with_icon` (set by the connector, ADD-A61:
  Material's icon-and-label tabs, MAT-TB4). Keyed: `adwaita` `@inline`, `@shrunk` (AdwTabBar), `@pinned`; `kde-breeze`
  `@static` (K1.123); `macos-*` `@border_none`, `@border_line`, `@border_bezel`, `@no_tabs` (MAC-TB03); `material-3`
  `@secondary` (MAT-TB1), `@scrollable` (MAT-TB7); `windows-11` `@width_equal`, `@width_size_to_content`,
  `@width_compact` (TabView's `TabWidthMode`, the application's choice, which changes the look: under
  `@width_compact` an unselected tab's `label` is absent by rule, W1-085).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large` (MAC-TB05); others none (WinUI's compact
  width mode is the keyed `@width_compact`).
- **Flags:** keyed `adwaita` `@needs_attention` (GN-1.128); state flags used: hover, pressed, selected, focus_visible,
  focus_within (AdwTabBar's focus on the tab box child, GN-1.134), disabled, dragged, drop_target (on `bar`).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (TabViewItem CommonStates; `selected` combines with
  them); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `attention_indicator`, `drop_indicator`, `tab_box`, `start_actions`, `end_actions` (all
  but `adwaita`), `overflow_edge` (all but `kde-breeze`), `indicator` (`macos-*`, `windows-11` draw none),
  `scroll_arrow` (`material-3`; qqc2 under `kde-breeze`
  flavour `kirigami`), `pane` (document strips), `label` (`windows-11` on an unselected tab under `@width_compact`, by
  rule, W1-085); fields: `bar.shadow`, `bar.corners`, `tab.shadow` (presets that draw
  none), `label.focus`,
  `separator.stroke` (macOS's selected neighbours, GNOME's notebook), `scroll_arrow.margin`; values:
  `repeat_delay_ms`, `repeat_interval_ms` (where the arrows do not repeat), `base_width_px`, `pinned_width_px`,
  `attention_width_fraction`, `bar_pane_overlap_px`, `outward_corner_radius_px`, `scroll_margin_px`.
- **State-model source:** `adwaita`: `_notebook.scss`, `_tab-view.scss` selectors and `adw-tab.c`; `kde-breeze`:
  Breeze's tab-shape branches (`hp:1299-1430`, `st:7021-7200`) and qqc2 `TabButton.qml`; `macos-*`: NSTabView
  (pressed, selected, key and non-key window, control size, tab position, border type); `material-3`: primary and
  secondary navigation-tab tokens (active, inactive × hover, focus, pressed); `windows-11`: TabViewItem visual state
  groups in `TabView.xaml` (CommonStates, DisabledStates, CloseIconStates, DragStates).
- **Today's fields:** `background_color` → `tab.tab.fill`; `active_background` → `tab.tab.fill @ {selected}`;
  `active_text_color` → `label.text.fill @ {selected}`; `bar_background` → `bar.fill`; `min_width_px`, `min_height_px`
  → `tab.tab.min_width`, `tab.tab.min_height`; `item_gap_px` → `tab_gap`; `active_indicator_color` → `indicator.fill`;
  `active_indicator_width_px` → the indicator's thickness (`indicator.min_height`, or `indicator.min_width` on a
  vertical bar); `active_indicator_side` → `indicator_edge`; `active_outward_corner_radius_px` →
  `outward_corner_radius`; `hover_background` → `tab.tab.fill @ {hover}`; `hover_text_color` →
  `label.text.fill @ {hover}`; `font` → `label.text`; `border` → `tab.tab.stroke`, `tab.tab.corners`,
  `tab.tab.padding`, `tab.tab.shadow`.
- **Notes:** Nested controls: a tab's close button is `button @ variant close context tab.tab` (GN-1.120, GN-1.130,
  K1.124, W1-082; shown per `close_button_shown`), the new-tab button `button @ variant add context tab.bar` (W1-083;
  macOS's window tabs, MAC-W09). Provenance: iOS's UITabBar and the navigation bars of KDE, Material and SwiftUI are
  the `bottom_tab_bar`
  widget (group 6 had proposed a tab variant); group 8 wrote the tab part as `body` (X-MO-6); Material's
  icon-and-label tabs were a keyed variant, now the content variant `with_icon`.

### sidebar

*From group 4's catalogue.*

- **Kind:** typed widget, the side pane (navigation drawer, split view's sidebar column, side panel, trailing
  inspector): GNOME (AdwNavigationSplitView and AdwOverlaySplitView sidebars, the sidebar pane class), KDE (Kirigami's
  global, context and overlay drawers, qqc2 Drawer, Qt side panels, the KIO places panel, kirigami-addons' settings
  window sidebar K-KA-41†), macOS (source-list sidebar, NSSplitViewItem sidebar, the inspector MAC-N19), iOS
  (UISplitViewController's primary column, the inspector column C-41†), Material (navigation drawer, navigation rail
  MAT-N13), Windows (NavigationView's left pane, SplitView pane). Group 6's "navigation drawer/pane" is this widget.
  Its rows are `list` rows with variant `sidebar`.
- **Available:** every preset.
- **Parts:**
  - `pane` — the side pane (the root); the Material rail's container (MAT-N13): `fill` (paint: material
    on macOS, iOS, WinUI's overlay acrylic), `stroke` (the inner edge only, flipping with the end variant and the
    direction, GN-1.139; the full frame under KDE's option, K1.140), `corners` (per corner: Material rounds the end
    corners only, MAT-SB7; floating glass, SD-6, MAC-SB06), `shadow`, `padding`, `min_width`, `max_width`,
    `min_height` (Material's drawer container height, MAT-SB7; `Auto` where the pane takes the window's height),
    `bindings` (the surface set it draws on: Kirigami's colour set by modality, Q1.53; GNOME's sidebar set),
    `opacity`, `transitions` (GN-1.136's backdrop transition).
  - `separator` — Kirigami's segmented inner-edge separator drawn apart beside the global toolbar (Q1.55): `stroke`
    (inset by the separator inset value), `transitions` (its margins animate, Q1.55).
  - `edge_shadow` — the gradient strip at a modal drawer's edge (Q1.56): `fill` (linear gradient), `min_width`,
    `min_height`, `transitions`.
  - `scrim` — the dim over the covered content (Q1.57, MAT-SB6, SD-9†): `fill`, `opacity` (MAT-SB6's scrim
    opacity), `transitions`.
  - `handle` *(element)* — the floating drawer handle (Q1.61, Q3.31): `fill`, `corners`, `shadow`, `icon`,
    `min_width`, `min_height`, `padding`, `transitions` (its bottom offset animates, Q1.61).
  - `header` — the pane's header area (Q1.63, Q1.66's header toolbar, W1-095's pane toggle, back and search area):
    `text` (title level), `icon`, `padding`, `min_height`.
  - `footer` — the area at the pane's bottom holding NavigationView's footer items and settings item, which are
    `list` rows with variant `sidebar` in `context sidebar.footer` (W1-095): `padding`, `stroke`.
  - `content` — the content area beside the pane (W1-097): `fill`, `stroke`, `corners` (rounded where it meets the
    pane).
- **Widget values:** `width_px: Value<Px>` (the pane's natural width; its limits are `pane.min_width` and
  `pane.max_width`, with `width_fraction`: GN-1.147, SD-10†, MAC-SB09†, Q1.60, MAT-SB7, W1-094, K-KA-41†; the
  inspector's open width, MAC-N19, C-41†); `width_fraction: Value<Fraction>` (the largest share of the window the pane
  takes; under `@edge_top` and `@edge_bottom` the pane's height and the window's height, Q1.60);
  `collapsed_width_px: Value<Px>` (Q1.59, W1-094; the inspector's compact width, MAC-N19,
  C-41†); `breakpoints: Vec<Breakpoint>` (Shared value types; the widget's own breakpoints, R16: W-P4-19†'s two
  thresholds, `@left_minimal` below the lower one, `compact` between them, the expanded pane above; iOS's arrangement
  per app size, P4-16†; the below-threshold ones await the core, Open items);
  `resize_grip_width_px: Value<Px>` (Q1.60);
  `floating_inset: Sides<Value<Px>>` (the floating sidebar's inset from the window edges, SD-6, MAC-SB06);
  `separator_inset: Sides<Value<Px>>` (Q1.55); `drag_hover_delay_ms: Value<Ms>` (a drag hovering a sidebar row
  activates it, GN-1.145); `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the pane
  shown and hidden, a drawer opening and closing; their steps name the parts that move with it: the `scrim`'s fade,
  MAT-SB9, Q1.57; the `handle`'s slide, Q1.61), `@collapse`, `@expand` (the rows' labels fading is a step of these,
  Q1.59), `@morph_to_tab_bar` (SD-7), `@submenu_push`, `@submenu_pop` (Q1.64), `@snap_back` (a drawer released
  part-way returning, its duration over `element.progress`, which the values declare, `data = { values =
  ["progress"] }`, Q1.58).
- **Variants:** typed `overlay` (modal or overlaying: Kirigami's modal drawers, Material's modal drawer, WinUI's
  overlay pane, iOS's overlay behaviour, GNOME's collapsed overlay split view; GN-2.19, W2-30), `compact` (the
  collapsed rail: Kirigami's collapsed global drawer, WinUI's LeftCompact, Material's navigation rail MAT-N13),
  `floating` (macOS 26, iOS 26 inset glass sidebars), `end` (trailing side: GNOME's end sidebar, Kirigami's right
  drawers, WinUI's right pane, the trailing inspector column of macOS MAC-N19 and iOS C-41†). Keyed: `adwaita`
  `@secondary` (middle pane, GN-1.137); `kde-breeze` `@global`, `@context`, `@places` (KIO), `@side_panel` (Qt's side
  panel view), `@edge_top`, `@edge_bottom` (drawers on the top or bottom edge, Q1.60); `windows-11` `@left_minimal`;
  `ios-*` `@displace`; `material-3` `@expanded`, `@modal` (the rail's expanded and modal forms, MAT-N13). Typed
  `bottom_sheet` (MAT-SB7's bottom-sheet drawer; the meaning `dialog`'s `bottom_sheet` shares on GNOME).
- **Sizes and densities:** `macos-*`: the sidebar size is the condition `cond container_size(sidebar, small)` (and
  medium, large) (R2.3), selecting `list` rules (MAC-SB01); others none.
- **Flags:** none of its own; state flags used: focus_within (KDE's focused-view edge colour, K1.140), drop_target
  (drag-hover activation); a pane shown or hidden is the element present or not, and its motion `Appear`,
  `Disappear`.
- **Exclusive groups:** none (the pane has no common-state group; rows follow `list`); precedence: state models (S17).
- **may_be_absent:** parts: `separator`, `handle` (all but `kde-breeze`), `edge_shadow` (all but `kde-breeze`,
  `adwaita`: AdwOverlaySplitView's transition shadow, GN-2.19), `scrim` (`macos-*`; every preset outside
  `variant overlay`: GNOME's overlay split view dims, GN-2.19; WinUI's SplitView draws its light-dismiss overlay,
  W2-30), `header` (all but `kde-breeze`, `windows-11`), `footer`, `content` (all but `windows-11`); fields:
  `pane.stroke` (floating, modal), `pane.shadow`; values:
  `width_fraction` (all but `kde-breeze`), `collapsed_width_px` (`adwaita`), `resize_grip_width_px` (all but
  `kde-breeze`), `floating_inset` (all but `macos-*`, `ios-*`), `separator_inset` (all but `kde-breeze`),
  `drag_hover_delay_ms` (where a drag hovering a row activates nothing: the value phase names the presets from the
  facts). `breakpoints` and `presentations` are lists: empty,
  not absent, where the platform has none.
- **State-model source:** `adwaita`: `_sidebars.scss` selectors and the split views' properties
  (`adw-navigation-split-view.c`, `adw-overlay-split-view.c`); `kde-breeze`: Kirigami's `OverlayDrawer.qml`,
  `GlobalDrawer.qml`, `ContextDrawer.qml` states (modal, collapsed, open, position) and Breeze's side-panel branch
  (`st:3958-3963`); `macos-*`: the visual effect view's state with the key state, NSSplitViewItem's behaviour and
  collapse; `ios-*`: UISplitViewController's display mode and split behaviour, collapsed; `material-3`: navigation
  drawer and rail tokens (standard, modal, collapsed, expanded); `windows-11`: NavigationView's display-mode group and
  SplitView's pane states.
- **Today's fields:** `background_color` → `pane.fill`; `font` → `list.row.text @ variant sidebar`;
  `selection_background` → `list.row_background.fill @ {selected} variant sidebar`; `selection_text_color` →
  `list.row.text.fill @ {selected} variant sidebar`; `hover_background` →
  `list.row_background.fill @ {hover} variant sidebar`; `border` → `pane.stroke`, `pane.corners`, `pane.padding`,
  `pane.shadow`.
- **Notes:** in context: the banner is `banner @ context sidebar.pane` (GN-1.146); a places panel's per-device
  usage bar is `progress_bar @ variant @capacity context sidebar.pane` (K1.142†); the Material rail's items are list
  rows (`list.row_selection_indicator`, `list.leading_icon`, the row's text) and its badge
  `badge @ context sidebar.pane` (MAT-N13); the pane toggle (hamburger) and back buttons are `button @ variant
  pane_toggle context sidebar.header` and `button @ variant back context sidebar.header` (W1-095, GNOME's split-view
  sidebar toggle, the Material rail's menu button MAT-N13); GtkStackSidebar's rows are `list @ variant @stack_sidebar`
  (GN-2.34); the
  file chooser's places rows are `list @ context any file_chooser` with `@new_bookmark`, `@drop_placeholder`
  (GN-P3-22†); `breakpoints` set the display variants (`compact`, `overlay`, `@left_minimal`, `@displace`), and rules
  select them by size class too (R16, P4-16†, W-P4-19†). Windows' top NavigationView is
  `view_switcher @ variant @navigation_top` (W1-094's Top mode, W2-11). Provenance: group 6 wrote the inspector as a
  variant of its own with open and compact widths (now `end` with `width` and `collapsed_width`), and the scrim, edge
  shadow and separator under other names; group 7 wrote the Material rail as `rail` (now `compact`); group 8 wrote the
  handle as a drawer part (Q3.31).

### toolbar

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (the toolbar class, AdwToolbarView's top and bottom bars, GtkActionBar GN-1.157,
  GN-2.20), KDE (QToolBar, qqc2 ToolBar, Kirigami's ActionToolBar, page header and global toolbar, kirigami-addons'
  FloatingToolBar K-KA-28†), macOS (NSToolbar), iOS (UIToolbar, the navigation bar C-1, C-28, C-43†), Material (top
  and bottom app bars, the Expressive docked and floating toolbars MAT-N11, MAT-N25, MAT-P4-7†), Windows (CommandBar).
  Group 6's "navigation/app bar" overlaps for Material's app bars, Kirigami's page header and iOS's navigation bar;
  they are traced here.
- **Available:** every preset.
- **Parts:**
  - `bar` — the bar (the root): `fill` (paint, material: MAC-TL09, TB-1; transparent at the scroll edge
    by the scrolled state, the navigation bar's scroll-edge appearance C-1), `stroke` (one edge: KDE's bottom line,
    K1.151; iOS's top or bottom hairline, TB-2, C-1; GNOME's border line, GN-1.154; GtkActionBar's top shade line,
    GN-1.157, GN-2.20), `shadow` (raised bars, GN-1.153; on-scroll elevation, MAT-TA1; the floating bar's, K-KA-28†,
    MAT-N25; animating between the expanded and collapsed elevations, MAT-P4-7†), `corners`, `padding` (the osd bar,
    GN-1.149; the floating bar), `min_height` (per variant: large title, compact, with search or prompt, C-1, TB-3,
    W1-103), `min_width` (the vertical floating toolbar, MAT-N25), `max_height` (Kirigami's global toolbar, Q1.74),
    `bindings` (KDE's Header set by position, Q1.68, Q1.71), `transitions` (MAT-TA1's on-scroll change; the header's
    resize, Q1.74).
  - `item_group` *(element)*, in `content` — items sharing one background (TB-5, MAC-TL03; 26's glass capsules): `fill`
    (material), `corners`, `shadow`, `padding`, `min_height` (the capsule's, MAC-TL03).
  - `separator`, in `content` — the line between items (K1.156, Q1.70, W1-101, Q1.73's leading separator, MAC-TL08's tracking
    separator): `stroke` (the line, its start edge), `corners`, `margin` (the space on either side, W1-101),
    `min_height` (its length, Q1.70), `bindings` (the Header set of Kirigami's leading separator, Q1.73).
  - `grip`, in `content` — KDE's drag grip, two lines (K1.156): `glyph`, `min_width`, `min_height` (the handle's width and extent).
  - `title`, in `content` — the bar's title (Q1.72, MAT-TA3, MAT-TA7, C-1): `text`, `padding`.
  - `subtitle`, in `content` — the subtitle (MAT-TA4, C-1): `text`.
  - `large_title` — iOS's large title (Large Title, emphasized) (C-1): `text`, `opacity`,
    `transform` (its opacity and transform follow the scroll offset while it collapses: `data = { large_title =
    ["scroll_offset"], bar = ["scroll_offset"] }`, ADD-B13, C-1).
  - `large_subtitle` — the large title's subtitle (C-1): `text`, `opacity`, `transform` (with the large title, over
    `element.scroll_offset`, which it declares: `data = { large_subtitle = ["scroll_offset"] }`, ADD-B13).
  - `prompt` — iOS's prompt text above the title (C-1): `text`.
  - `title_menu_chevron` — the chevron of iOS's title menu (C-1, C-28): `glyph`.
  - `document_header` — iOS's document-properties header in the title menu (C-28): `text`, `icon`.
  - `leading_content`, `trailing_content` — the content Material's floating toolbar reveals and hides on expand and
    collapse (MAT-P4-7†): `padding`.
  - `content`, in `bar` — the box holding the bar's items: the connector queries the item elements (`button`,
    `split_button`, `search_field` … in `context toolbar.bar`) in its scope, so they nest in it: `opacity` (libadwaita
    dims a raised or bottom bar's content as one group in an inactive window, GN-1.155, GN-1.157, GN-2.20),
    `transitions` (that dimming's fade, GN-1.155).
- **Widget values:** `item_gap_px: Value<Px>` (today; GN-1.157); `group_gap_px: Value<Px>` (TB-5, MAC-TL03);
  `fixed_space_width_px: Value<Px>`, `flexible_space_min_width_px: Value<Px>` (TB-8, MAC-TL10†);
  `title_alignment: Value<Alignment>` (MAT-TA3); `collapse_on_scroll: Value<bool>` (MAT-TA3; the large title's
  collapse, C-1); `floating_inset: Sides<Value<Px>>` (the floating toolbar's external padding from the window's
  edges, MAT-N25); `tooltip_shows_shortcut: Value<bool>` (Q1.76; Kirigami's flat tool buttons are `button` rules in
  `context toolbar.bar`);
  `presentations: Vec<PresentationTransition>` (queried with the bar): events `Appear`, `Disappear` (the bar shown and
  hidden: Material's
  floating toolbar, MAT-N25; AdwToolbarView's revealed bars, GN-P3-5†; iOS's bars hidden on a swipe, P4-3†), `Move`
  (the bars re-laid out when a toolbar moves or a dock opens a gap, K3.38†), `@expand`, `@collapse` (the bar opened
  to show its labels and overflow and closed again: WinUI's command bar, W1-104; Material's floating toolbar,
  MAT-P4-7†), `@items_change` (the items fading and moving when they are
  set, added, removed or moved to the overflow: P3-4†; while the customization editor runs, MAC-P3-6†, P4-10†),
  `@customize` (the items parting while one is dragged into the bar from the customization editor, P4-10†,
  MAC-P3-6†), `@title_change` (the navigation push and pop: the title's cross-fade and the back button's label
  transition, one step each, the latter naming the `button`'s `label`, C-2, X-MO-3). The on-scroll change of the bar
  (MAT-TA1) is its `transitions` under `{scrolled}`; Kirigami's global header resizing and its separator fading
  (Q1.74) are the bar's `transitions`. The search item's expansion is `search_field`'s event `@expand` (MAC-TL07).
- **Variants:** typed `top`, `bottom` (header or footer position: KDE's ToolBar, GNOME's top bar and its bottom bar,
  GtkActionBar and AdwToolbarView's bottom bar, GN-1.157, GN-2.20; iOS's toolbars against navigation bars; Material's
  top and bottom app bars; Q1.68), `floating` (Material's floating toolbar, MAT-P4-7†, MAT-N25; KDE's FloatingToolBar,
  K-KA-28†), `vertical` (Material's vertical floating toolbar, MAT-N25, R25.14). Keyed: `adwaita` `@raised`
  (GN-1.153), `@osd`, `@border`, `@collapse_spacing`, `@inline`, `@shrunk` (the bottom bar's inline and shrunk forms,
  GN-2.20); `kde-breeze` `@main` (the main toolbar against other toolbars, K1.153, K1.158), `@global`, `@page_header`;
  `macos-*` `@tracking` (on `separator`, MAC-TL08), `@unified`, `@unified_compact`, `@expanded`, `@preference`
  (MAC-TL01); `ios-*` `@large_title` (C-1), `@navigator`, `@browser`, `@editor` (the navigation item's styles,
  C-43†); the content variants `with_search`, `with_prompt` (the navigation bar holding a search bar or a prompt,
  set by the connector from its content, C-1; iOS's compact bar is `@ v_size_class compact`, as UIKit picks it by
  the size class, TB-3); `material-3`
  `@center_aligned`, `@medium`, `@large`, `@medium_flexible`, `@large_flexible` (MAT-TA3), `@docked`, `@vibrant`
  (MAT-N25), `@collapsed` (MAT-P4-7†); `windows-11` `@compact`, `@minimal`, `@hidden` (the closed display modes,
  W1-103).
- **Sizes and densities:** `ios-*`: the compact height `@ v_size_class compact` (TB-3, ADD-A20); others by variant.
- **Flags:** none of its own; state flags used: scrolled (on the toolbar: content beneath it is scrolled away from
  its edge, set by the connector; TB-1, MAT-TA1, C-1; the scroll view's own edge effects carry theirs, `scroll_view`),
  open (W1-103's open command bar).
- **Exclusive groups:** none: the bar's states combine; its items are `button` elements whose groups are `button`'s
  (AppBarButton's CommonStates on `windows-11`); precedence: state models (S17).
- **may_be_absent:** parts: `item_group` (all but `ios-*`, `macos-*`), `grip` (all but `kde-breeze`), `subtitle`
  (all but `material-3`, `ios-*`), `large_title`, `large_subtitle`, `prompt`,
  `title_menu_chevron`,
  `document_header` (all but `ios-*`), `leading_content`, `trailing_content` (all but `material-3`); fields:
  `bar.shadow`, `bar.stroke` (`material-3`: its bars draw no line; a line on some edges only states the others at
  zero width, Entry format), `bar.fill` (`macos-tahoe`: the 26 bar is transparent, MAC-TL09); values: `group_gap_px`, `fixed_space_width_px`,
  `flexible_space_min_width_px`
  (all but `ios-*`, `macos-*`), `title_alignment`, `collapse_on_scroll` (all but `material-3`, `ios-*`),
  `floating_inset` (all but `material-3`).
- **State-model source:** `adwaita`: `_toolbars.scss` selectors (toolbars, action bar, raised and revealed bars);
  `kde-breeze`: Breeze's tool-button and toolbar branches (`st:4577-4624`, `st:5162-5227`, `st:5526-5685`) and qqc2
  and Kirigami toolbar QML (position, flat, display), kirigami-addons' `FloatingToolBar.qml`; `macos-*`: NSToolbar's
  style, its items (bordered, style, enabled, selected), key window; `ios-*`: the bar button item's style, selected,
  `UIBarAppearance` standard, compact and scroll edge, the navigation item's style; `material-3`: top and bottom app
  bar tokens (rest, on-scroll) and the Expressive toolbar tokens (docked, floating, vibrant); `windows-11`:
  CommandBar's display-mode and open states.
- **Today's fields:** `font` → `button.label.text @ context toolbar.bar` (and `title.text` where a preset states one);
  `bar_height_px` → `bar.min_height`; `item_gap_px` → `item_gap`; `background_color` → `bar.fill`; `icon_size_px` →
  `button.icon.icon.size @ context toolbar.bar`; `border` → `bar.stroke`, `bar.corners`, `bar.padding`, `bar.shadow`.
- **Notes:** a toolbar's items are the elements they are, in context: a tool or bar button is
  `button @ context toolbar.bar` (Qt's tool button, WinUI's AppBarButton and AppBarToggleButton and AppKit's toolbar
  bezel are the typed `button @ variant tool`), a toggle item `button @ variant toggle`, a menu item
  `button @ variant menu`, a split item `split_button @ context toolbar.bar`, a search item
  `search_field @ context toolbar.bar` (MAT-TA6, MAC-TL07), an avatar `avatar @ context toolbar.bar` (MAT-TA5), a
  badge on an item `badge @ context toolbar.bar` (TB-7, MAC-TL05, C-15); the item states (K1.152, W1-099, W1-100,
  MAC-TL02, TB-6, P3-3†, MAC-TL11†) are button rules in `context toolbar.bar`, their icons per state `button.icon`
  rules there (MAT-TA2, MAC-TL02, K1.158); prominent items are `button @ variant primary context toolbar.bar` (TB-4,
  MAC-TL04); iOS's done and padding-removed items and KDE's menu-indicator styles (K1.155) are keyed button variants;
  iOS's back button is `button @ variant back context toolbar.bar` (C-2); the FAB in Material's floating toolbar is a
  button `@ context toolbar variant floating` (MAT-N25). The icon–label gap and the label position are button values
  in context (`button.values.icon_text_gap_px`, K1.154, MAC-TL02; `button.values.label_position`, K1.153 read per
  toolbar
  kind, W1-099, MAC-TL02, Q1.76). The overflow button is `button @ variant overflow context toolbar.bar` (K1.157,
  Q1.75, MAC-TL06, W1-102, MAT-TA8†, C-1), its menu `menu @ context button variant overflow`, placed by
  `button.values.menu_placement @ variant overflow context toolbar.bar` (below, or above for a footer; Q1.75,
  W1-102), its checked item's glyph `menu.check_indicator` there (W1-100). The scroll-edge shade under a bar is
  `scroll_view.edge_effect` (GN-1.156). A card's footer is `toolbar @ variant bottom context card.body` (Q1.132).
  Provenance: group 4 had item parts (item, its icon, label, menu indicator and overflow check) and a label-position
  value here; group 6 the bottom bar, Material's docked, floating and vibrant toolbars and iOS's navigation bar
  (GN-2.20, MAT-N11, MAT-N25, C-1, C-28, C-43†); group 7 the floating toolbar's content (MAT-P4-7†) and the back
  button (C-2); group 1 the reveal and hide-on-swipe events (GN-P3-5†, P4-3†).

### status_bar

*From group 4's catalogue.*

- **Kind:** typed widget: KDE (QStatusBar) and macOS (the window's bottom content border, MAC-ST01). GNOME's bottom
  bar (GtkActionBar, AdwToolbarView's bottom bar) is `toolbar @ variant bottom` (GN-1.157, GN-2.20); WinUI has none
  (the Win32 status bar belongs to a Win32 preset, R1.3).
- **Available:** every preset but `adwaita`, `windows-11`, `ios-*`, `material-3` (`available = false`: no such
  control); `kde-breeze` flavour `widgets` only (QStatusBar).
- **Parts:**
  - `bar` — the bar (the root): `fill`, `stroke` (the top line only: K1.160, MAC-ST01), `corners`, `shadow`,
    `padding`, `min_height` (the content border's height, MAC-ST01), `text`.
  - `item` — an item: `text`, `padding`.
- **Widget values:** none (KDE's and macOS's rows state no value beyond the parts).
- **Variants:** keyed `kde-breeze` `@main_window` (the top line is drawn only for a QMainWindow's status bar, K1.160).
- **Sizes and densities:** none.
- **Flags:** none; the window's inactive look is the condition `cond window_inactive` (MAC-ST01).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: none; fields: `bar.stroke` (KDE outside a main window), `bar.shadow`; values: none.
- **State-model source:** `kde-breeze` Breeze `st:5312-5325`; `macos-*` the window's content border thickness for the
  bottom edge, key and main window.
- **Today's fields:** `font` → `bar.text`; `background_color` → `bar.fill`; `border` → `bar.stroke`, `bar.padding`,
  `bar.corners` (from `corner_radius`), `bar.shadow` (from `shadow_enabled`).
- **Notes:** `bar.min_height` is Auto where the bar is content-sized. Provenance: group 4 also listed GNOME's bottom
  bar here, with its spacing as an item gap (GN-1.157); it is one widget with the toolbar's bottom bar, the spacing
  `toolbar.values.item_gap_px @ variant bottom`.

### splitter

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (GtkPaned), KDE (QSplitter, dock resize handles, qqc2 SplitView, Kirigami's ColumnView
  separators), macOS (NSSplitView), iOS (the column divider), Material (drag handle), Windows (the Community Toolkit's
  sizers).
- **Available:** every preset.
- **Parts:**
  - `divider` — the divider (the root): `fill`, `stroke` (GNOME's two lines on either side of the wide
    handle), `min_width`, `min_height` (its thickness, by orientation), `bindings` (Kirigami's Header set, Q1.93),
    `cursor`, `transitions`.
  - `grip` — the grip, dimple or drag handle on the divider (MAC-SP02, MAT-SP1, W1-157†, SP-2): `fill`, `glyph`,
    `corners`, `min_width`, `min_height`, `padding`, `shadow`.
  - `collapser` *(element)*, in `divider` — KDE's collapser button over the divider (KSplitterCollapserButton,
    K2.17), hovered and pressed on its own, its side a keyed variant: `fill`, `padding`, `opacity` (its fade, K2.17).
    `collapser_arrow` belongs to it.
  - `collapser_arrow`, in `collapser` — its arrow, direction by the side: `glyph`.
- **Widget values:** `hit_extension: Sides<Value<Px>>` (the hit area beyond the drawn divider, per side, as
  `button`'s: Q1.92, Q1.93; a rule `@ {hover}`); `presentations: Vec<PresentationTransition>` (queried with the
  collapser): events `Appear`, `Disappear`, kind `Fade` (the collapser shown and hidden, its `QTimeLine` duration,
  K2.17).
- **Variants:** typed `wide` (GNOME's wide handle, macOS's thick and pane splitter), `sidebar` (the split beside a
  floating sidebar: no divider on macOS 26 and iOS 26, MAC-SP04, SP-1), `vertical` (the panes stacked, the divider
  running across: GTK's row-resize cursor against the side-by-side column-resize one, GN-3.67†; the orientation many
  widgets share). Keyed: `macos-*` `@thin`, `@pane_splitter` (with `wide`: the pane splitter's look beyond the thick
  divider, MAC-SP01, MAC-SP02); `kde-breeze` `@dock`, `@column_view`, and on `collapser` `@left`, `@right`, `@top`,
  `@bottom` (the side it collapses toward, K2.17).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed, dragged (MAC-SP03), focus_visible on Material's handle
  (MAT-SP2), disabled (W1-157†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (SizerBase's states); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `grip` (`adwaita`, `kde-breeze`: no grip on the divider), `collapser`,
  `collapser_arrow` (all but `kde-breeze` flavour `widgets`); fields: `divider.fill`, `divider.stroke` (variant
  `sidebar`); values: none (`presentations` is empty where none is stated).
- **State-model source:** `adwaita` `_paned.scss`; `kde-breeze` Breeze's splitter branch and qqc2 `SplitView.qml`;
  `macos-*` NSSplitView's divider style; `ios-*` the column resizing states; `material-3` drag-handle tokens;
  `windows-11` SizerBase visual states.
- **Today's fields:** `divider_width_px` → `divider.min_width` (its thickness); `divider_color` → `divider.fill`;
  `hover_color` → `divider.fill @ {hover}`.
- **Notes:** the transitions showing and hiding a column belong to `sidebar` (SP-3).

### separator

*From group 4's catalogue.*

- **Kind:** typed widget, every platform.
- **Available:** every preset.
- **Parts:**
  - `line` — the line (the root): `stroke` (conventions: the top edge horizontal, the left edge vertical; position
    centred where the platform centres it on a half pixel, K1.201; pixel snapping, K-P3-20†, ADD-A33), `min_width`,
    `min_height` (GNOME's spacer, GN-1.188), `bindings` (the surface set its stroke draws from, Q1.95).
- **Widget values:** `inset: Sides<Value<Px>>` (start and end insets per container and orientation, by rules with
  `context`: GN-1.187, W1-119, MAT-D1).
- **Variants:** typed `vertical` (the line running down; horizontal is the base, the orientation many widgets share,
  R25.14). Keyed: `adwaita` `@spacer` (no line, GN-1.188); `kde-breeze` `@light`
  (Q1.95); `ios-*` `@opaque` (SE-2); `material-3` `@inset`, `@inset_start`, `@inset_end` (MAT-D1).
- **Sizes and densities:** none.
- **Flags:** none (no states).
- **Exclusive groups:** none (no states); precedence: state models (S17).
- **may_be_absent:** parts: none; fields: `line.stroke` (variant `@spacer`); values: none.
- **State-model source:** no states; variants from libadwaita `_misc.scss`, Kirigami's `Separator.qml`, Material's
  divider, UIKit's separator colours, WinUI's separators per container.
- **Today's fields:** `line_color` → `line.stroke` paint; `line_width_px` → `line.stroke` width.
- **Notes:** the opaque separator's stroke refers to `role.@separator_opaque`, the translucent separator's to
  `surface.border` (SE-2). Provenance: group 8 added the stroke's pixel snapping (K-P3-20†, ADD-A33).

### card

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (the card class), KDE (Kirigami's Card and AbstractCard), Material (filled,
  elevated, outlined cards), Windows (card surfaces and their fills, W1-139). macOS's NSBox is the group box, where
  its rows are traced; Windows' clickable SettingsCard is `form.card` (W1-139, W2-40†). iOS draws no card: its
  card-like surfaces are a list cell's background configuration (`list.row_background` with
  `list.values.row_background_inset`, CD-1) and SwiftUI's GroupBox (`group_box`), so the `ios-*` preset states no
  card a control does not draw.
- **Available:** every preset but `macos-*` (`available = false`: NSBox is `group_box`) and `ios-*` (`available =
  false`: CD-1); `kde-breeze` flavour `kirigami` only (Kirigami's Card: Qt Widgets draws none).
- **Parts:**
  - `body` — the card (the root): `fill`, `stroke`, `corners`, `shadow`
    (layers or an elevation entry; KDE's lift layer is a zero-blur shadow layer, Q1.128), `padding`, `focus`,
    `bindings` (the Card surface set; Kirigami's View set, Q1.129), `opacity` (the disabled card, MAT-CA2),
    `transitions`.
  - `icon` — Material's card icon (MAT-CA3): `icon`.
  - `banner` — Kirigami's banner image area (Q1.131): `padding`, `min_width`, `min_height`.
  - `banner_band` — the band behind the banner title, its height relative to the title's (Q1.131): `fill`.
  - `banner_title` — the banner's title (Q1.131): `text`, `icon`.
- **Widget values:** `section_gap_px: Value<Px>` (between header, content and footer, Q1.130);
  `banner_band_height_factor: Value<f32>` (Q1.131).
- **Variants:** typed `activatable` (GNOME's activatable card, Kirigami's clickable cards, Material's interactive
  cards). Keyed: `adwaita` `@osd`; `kde-breeze` `@banner`; `material-3` `@filled`, `@elevated`, `@outlined` (the
  container styles); `windows-11` `@secondary`, `@tertiary` (the card fills, W1-139; the default card is
  outlined-like).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed, checked (Q1.126), focus_visible, dragged, disabled
  (MAT-CA2).
- **Exclusive groups:** `material-3`: `[dragged, pressed, focus_visible, hover]` (MAT-CA2's state layers); others none
  (states combine; Windows' card surfaces have no states); precedence: state models (S17).
- **may_be_absent:** parts: `icon` (all but `material-3`), `banner`, `banner_band`, `banner_title` (all but
  `kde-breeze`); fields: `body.shadow` (the OSD card, the outlined card); values: `section_gap_px`,
  `banner_band_height_factor` (all but `kde-breeze`).
- **State-model source:** `adwaita` `_misc.scss` card selectors; `kde-breeze` Kirigami's `DefaultCardBackground.qml`;
  `material-3` card tokens (filled, elevated,
  outlined × states); `windows-11` the card fills (no states).
- **Today's fields:** `background_color` → `body.fill`; `border` → `body.stroke`, `body.corners`, `body.padding`,
  `body.shadow` (from `shadow_enabled`).
- **Notes:** the footer is `toolbar @ variant bottom context card.body` (Q1.132). Provenance: group 4 typed Material's
  container styles and listed Windows' clickable SettingsCard as `activatable`; the card's Windows states moved with
  it to `form.card`.

### expander

*From group 4's catalogue.*

- **Kind:** typed widget: the stand-alone expanders: GNOME (GtkExpander), KDE (KCollapsibleGroupBox), macOS (the
  disclosure and push-disclosure buttons, SwiftUI's DisclosureGroup), Windows (Expander). An expandable row is a row
  with a disclosure in its host, one structure for one control: in a list, a `list` row with a `disclosure` and its
  `nested` box (Material's expandable list item MAT-L9, MAT-EX1; iOS's outline disclosure EX-1–EX-3, its styles
  `list`'s keyed `@header`, `@cell` on `disclosure`; GNOME's AdwExpanderRow in a plain boxed list, GN-2.09; the
  reveal `list`'s events `@expand`, `@collapse`); in a preferences group, a `form.row` of kind `expander` with its
  `row_arrow` and `row_nested` (AdwExpanderRow in an AdwPreferencesGroup, GN-1.257–GN-1.260; Windows'
  SettingsExpander, W2-40†; kirigami-addons' expander card, K-KA-7†).
- **Available:** every preset but `material-3`, `ios-*` (`available = false`: their expandable item is a list row);
  `kde-breeze` flavour `widgets` only (KCollapsibleGroupBox).
- **Parts:**
  - `body` — the expander (the root), holding the header and the content: `padding`, `bindings`.
  - `header` *(element)* — the header: `fill`, `stroke`, `corners` (GN-1.261), `padding`, `min_height`, `text`,
    `focus`, `shadow`.
  - `arrow` *(element)* — the disclosure arrow: `glyph` (source per platform; on macOS the system control rendering of
    the disclosure bezel, S15, MAC-B01), `fill`, `stroke` (WinUI's chevron button border per state, W1-140), `corners`
    (WinUI's chevron button), `opacity`, `transform` (rotation per state), `min_width`, `min_height` (its size per
    control size, MAC-EX05†), `transitions` (X-MO-6).
  - `content` — the disclosed area: `fill` (W1-141), `stroke`, `corners`, `padding`.
- **Widget values:** `arrow_side: Value<Side>` (today); `arrow_gap_px: Value<Px>` (today);
  `content_indent_px: Value<Px>`
  (today); `drag_hover_delay_ms: Value<Ms>` (a drag hovering the header expands it, GN-1.262);
  `presentations: Vec<PresentationTransition>`: events `@expand`, `@collapse` (the content's reveal: GN-1.259, K1.251,
  W1-142, K3.37).
- **Variants:** keyed: `macos-*` `@push_disclosure`
  (MAC-EX01, MAC-B01); `windows-11` `@expand_up` (the content opens above the
  header, with its own border thickness and corners, W1-141; a variant because the look changes with it, which a
  widget value cannot select).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large` (MAC-EX05†); others none.
- **Flags:** none of its own; state flags used: open (expanded), hover, pressed, focus_visible, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (Expander CommonStates; its ExpandStates are
  `open`, its ExpandDirectionStates the variant `@expand_up`); others none (states combine); precedence: state models
  (S17).
- **may_be_absent:** parts: none; fields: `header.stroke`, `content.stroke` (today's frame off), `header.shadow`,
  `content.fill`, `arrow.fill`, `arrow.stroke` (outside Windows); values: `drag_hover_delay_ms`.
- **State-model source:** `adwaita` `_expanders.scss`; `kde-breeze` Breeze's arrow
  branch (`st:4424-4427`) and `kcollapsiblegroupbox.cpp`; `macos-*` NSButton's disclosure states and control sizes;
  `windows-11` Expander visual states (CommonStates, ExpandStates,
  ExpandDirectionStates).
- **Today's fields:** `font` → `header.text`; `header_height_px` → `header.min_height`; `arrow_icon_size_px` →
  `arrow.glyph.size`; `hover_background` → `header.fill @ {hover}`; `arrow_color` → `arrow.glyph.fill`; `arrow_side`,
  `arrow_gap_px`, `content_indent_px` → the values of the same names; `frame_enabled` → whether `header.stroke` and
  `content.stroke` are stated or `Absent`; `border` → `header.stroke`, `header.corners`, `header.padding`,
  `header.shadow` (from `shadow_enabled`).
- **Notes:** provenance: group 2 wrote AppKit's disclosure bezels as button variants: the disclosure bezel is this
  widget's `arrow` (the whole-control rendering) and the push disclosure its keyed variant (MAC-B01, MAC-EX01); group
  8 added KCollapsibleGroupBox's presentations (K3.37) and the arrow's transitions (X-MO-6).

### layout

*From group 4's catalogue.*

- **Kind:** typed widget with values only (no parts drawn): a layout container's default spacing, queried per
  container element so that size-class rules apply (R16); today's top-level `layout` table. Every platform. The flow
  layout absorbs GNOME's wrap box (GN-2.46) and Material's chip set (MAT-RL01†).
- **Available:** every preset.
- **Parts:**
  - `body` — the container's box (the root; a layout draws nothing of its own, but every element names a part, S8):
    `bindings` (a box may bind a surface set for its children).
  - `levitated_scrim` — the optional scrim behind a levitated pane (Compose's `LevitatedPaneScrim`, MAT-SF04†):
    `fill` (`= role.global.scrim`), `transitions`.
- **Widget values:** `widget_gap_px`, `container_margin_px`, `window_margin_px`, `section_gap_px: Value<Px>` (today's);
  `label_gap_px: Value<Px>` (label to control, per size: MAC-LY01, W1-146); `button_gap_px: Value<Px>` (MAC-LY01,
  W1-146);
  `header_gap_px: Value<Px>` (control to header, W1-146); `surface_text_margin_px: Value<Px>` (W1-146); `item_gap_px`,
  `line_gap_px: Value<Px>` (flow layouts: GN-2.46, GN-P3-24†, RL-3†, MAC-RL01†, W-P4-9†, MAT-RL01†; `item_gap` negative
  under `@linked`, GN-1.42); `flow_max_per_line: Value<u32>` (GN-P3-24†); `max_content_width_px`,
  `tightening_threshold_px: Value<Px>` (AdwClamp, GN-1.195; iOS's readable width, LY-2);
  `form_label_alignment: Value<Alignment>` (K1.210, MAC-LY02†); `form_baseline_alignment: Value<bool>` (label to
  control on one baseline, MAC-LY02†); `form_field_growth: Value<bool>` (fields grow, K1.210);
  `form_wraps: Value<bool>` (rows wrap, K1.210); `form_alignment: Axes<Value<Alignment>>` (where the form itself sits
  in its space, Qt's `SH_FormLayoutFormAlignment`, K1.210); `pane_gap_px: Value<Px>` (MAT-LA1, MAT-SP3);
  `vertical_pane_gap_px: Value<Px>` (the spacer between two stacked panes, MAT-P3-13†); `max_panes_horizontal`,
  `max_panes_vertical: Value<u32>`, `pane_width_px`, `pane_height_px: Value<Px>`, `avoids_hinge: Value<bool>`
  (MAT-P3-13†; two stacked panes `@ cond posture:tabletop`, ADD-A83); `breakpoints: Vec<Breakpoint>` (Shared value
  types; the widget's own breakpoints, R16: TwoPaneView's wide and tall thresholds against its own width and height,
  setting its keyed `@two_pane_wide`, `@two_pane_tall`, W-P4-19†); `levitated_placement: Placement` (a levitated pane
  floating over the scaffold, strategy `MaterialLevitatedPane`: its alignment the placement's `side` and `align`;
  its scrim the part `levitated_scrim`, MAT-SF04†);
  `column_width_px`, `column_min_width_px`, `column_max_width_px: Value<Px>` (Kirigami's ColumnView, Q1.93, Q1.94;
  CardsLayout, Q1.133; kirigami-addons' settings window, K-KA-41†); `max_columns: Value<u32>` (Q1.133);
  `presentations: Vec<PresentationTransition>`: events `@column_scroll` (Q1.94). Lengths that scale with text are
  expressions (ADD-A55).
- **Variants:** typed `form`, `flow` (GNOME's wrap box GN-2.46, iOS RL-3†, macOS MAC-RL01†, Windows W-P4-9†), `clamp`,
  `page`, `pane`, `vertical` (a vertical box: GTK's vertical linked box, GN-1.42; the orientation many widgets
  share). Keyed: `adwaita` `@preferences_page`, `@linked` (GTK's linked box, GN-1.42); `kde-breeze` `@cards`,
  `@cards_list`, `@column_view`, `@kirigami_page` (with `page`: Kirigami's Page, whose padding differs from qqc2's,
  Q1.98); `windows-11` `@two_pane_wide`, `@two_pane_tall` (W-P4-19†). Material's chip set is the typed `flow`
  (MAT-RL01†): it adds nothing a flow layout lacks.
- **Sizes and densities:** `macos-*` control sizes on `label_gap` and `button_gap` (MAC-LY01); size classes:
  Material's window classes (MAT-LA1, MAT-P3-13†), iOS's compact and regular (LY-1).
- **Flags:** none (no states).
- **Exclusive groups:** none (no states); precedence: state models (S17).
- **may_be_absent:** parts: `levitated_scrim` (all but `material-3`); fields: none; values: every scalar value but
  today's four (a platform without a clamp, a flow default or panes states them `Absent`); the lists (`breakpoints`,
  `presentations`) are empty where the platform has none.
- **State-model source:** no states; size classes from Material's window size classes and UIKit's traits.
- **Today's fields:** the four keep their names as widget values.
- **Notes:** in context: GNOME's wrap box is `layout.values.{item_gap,line_gap} @ variant flow` (GN-2.46); Material's
  chip set is `layout.values.{item_gap,line_gap} @ variant flow` with `chip` children (MAT-RL01†, a rule); a
  linked box's buttons are `button @ context layout variant @linked` (GN-1.42). Provenance: GNOME's wrap box and
  Material's chip set
  are this widget (rename row 75); group 2 wrote the linked box as a button-group context.

## Typed widgets the model lacks today (alphabetical)

### about_dialog

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwAboutDialog (GN-2.39), KDE's KAboutApplicationDialog (K-P3-8†), Kirigami's
  About page (Q2.57†) and kirigami-addons' About page (K-KA-44†), macOS's standard About panel (MAC-N32†). Its
  window is a `dialog` element.
- **Available:** every preset but `windows-11`, `ios-*`, `material-3` (`available = false`: no about dialog the
  toolkit draws; Windows' `ShellAbout` is out, R1.1).
- **Parts:**
  - `body` — the dialog's content box (the root): `padding` (its margins, GN-2.39, K-KA-44†), `bindings`.
  - `icon` — the application icon: `icon` (its size: GN-2.39, K-P3-8†, Q2.57†, MAC-N32†), `shadow` (GNOME's drop
    shadow), `margin` (the space round it where the panel's gaps differ, MAC-N32†).
  - `name` — the application name: `text`, `margin` (MAC-N32†).
  - `version` — the version line: `text`, `fill`, `padding`, `corners` (GNOME's pill: accent text, padding, corners;
    plain text elsewhere), `margin` (MAC-N32†).
  - `description` — the short description: `text`.
  - `section_title` — a credits or legal section's heading: `text`, `padding`.
  - `credits` — the credits text (macOS's credits text view): `text`, `margin` (MAC-N32†).
  - `copyright` — the copyright line: `text`, `margin` (MAC-N32†).
  - `person_name` — a person's name in the credits (KDE's in its own text style, K-P3-8†): `text`.
  - `person_task` — the person's task line (KDE's dimmed text): `text`, `opacity`.
- **Widget values:** `icon_max_width_fraction: Value<Fraction>` (the icon at most this fraction of the width: Q2.57†,
  K-KA-44†; its margins are `body.padding`, GN-2.39, K-KA-44†); `gap_px: Value<Px>` (GN-2.39's spacing; macOS's unequal gaps are the parts' margins,
  MAC-N32†); `subpage_margins: Sides<Value<Px>>`
  (GN-2.39); `width_px: Value<Px>` (macOS's panel, MAC-N32†); `release_gap_px: Value<Px>` (the
  space between releases in kirigami-addons' release history, K-KA-44†).
- **Variants:** keyed `kde-breeze` `@ka_about` (kirigami-addons' AboutPage, K-KA-44†). KDE's two forms are the
  flavour condition, not variants: `cond flavor:widgets` (KAboutApplicationDialog with tabs, K-P3-8†) and
  `cond flavor:kirigami` (the About page on form cards, Q2.57†, K-KA-44†).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own (its link buttons are `button` elements with their own states).
- **Exclusive groups:** none (the presets that draw it, GNOME, KDE and macOS, combine states); precedence: state
  models (S17).
- **may_be_absent:** parts: `section_title`, `person_name`, `person_task` (`macos-*`: the panel's credits are one
  text view, MAC-N32†), `credits` (all but `macos-*`); fields: `icon.margin`, `name.margin`, `version.margin`,
  `credits.margin`, `copyright.margin` (all but `macos-*`, which places them by margins; GNOME's version pill is
  `version`'s corners and padding, which a preset drawing plain text states without a pill), `icon.shadow`,
  `version.fill` (all but `adwaita`: GNOME's drop shadow and version pill, GN-2.39, GN-3.07); values:
  `icon_max_width_fraction` (`adwaita`,
  `macos-*`), `subpage_margins` (`kde-breeze`, `macos-*`), `width_px` (`adwaita`, `kde-breeze`), `release_gap_px`
  (`adwaita`, `macos-*`).
- **State-model source:** GNOME `_misc.scss`, `adw-about-dialog.ui`; KDE `kaboutapplicationdialog.cpp`,
  `kabstractaboutdialog_p.cpp`, `AboutItem.qml`, kirigami-addons `AboutPage`; macOS
  `NSApplication.AboutPanelOptionKey` and the panel as drawn.
- **Today's fields:** none (MAC-N32†'s stand-in `dialog.*` stays `dialog`'s).
- **Notes:** in `cond flavor:kirigami` the page's sections are `form` elements in `context about_dialog` (Q2.57†;
  K-KA-44†'s link rows are `form` variant `@ka_info_card`), the authors' pictures `avatar` elements, the "What's
  new" badge a `badge`; K-KA-44†'s release history is a `dialog` (keyed `@ka_dialog`) in `context about_dialog`.
  In `cond flavor:widgets` the pages are `tab` elements and the Close button a `button` (K-P3-8†). Nested controls: the
  icon-only link and mail buttons are `button @ variant icon_only context about_dialog` (KDE's
  `internet-services-symbolic` and mail buttons, K-P3-8†); K-KA-44†'s link rows are `form` rows.

### autocomplete

*From group 6's catalogue.*

- **Kind:** typed widget: the completion attached to a field. GNOME's GtkEntryCompletion popup (GN-2.56†), KDE's
  KCompletionBox (K2.29†) and kirigami-addons' path completion (K-KA-13†), Material's autocomplete (MAT-N27), iOS's
  search suggestions (C-38†), Windows' AutoSuggestBox list (W2-15) and RichSuggestBox (W-P3-17†), macOS's
  `NSComboBox` completion (MAC-CB01), `NSTextView` completions (MAC-I06, MAC-X15) and text-field and search
  suggestion menus (MAC-P3-7†), and the code editors' completion (GN-SF13†, K-P3-13†). One tree on every platform:
  the field is the ancestor, so an `autocomplete` element is a child of the `input` (or `search_field`, `code_editor`)
  it completes, and its popup list is a `menu` in `context autocomplete` (Nested controls).
- **Available:** every preset.
- **Parts:**
  - `body` — the completion's host (the root): `bindings`, `padding` (the list's inset from the field); it draws
    nothing of its own, its list being the `menu`.
  - `token` — a chosen suggestion inserted into rich text (W-P3-17†): `fill`, `text`, `corners`, `padding`.
  - `details_pane` — the code editors' details pane beside the list (GN-SF13†, K-P3-13†): `fill`, `stroke`,
    `corners`, `padding`, `text`.
- **Widget values:** `filter_delay_ms: Value<Ms>` (GN-2.56†); `mode: Value<AutocompleteMode>`, `AutocompleteMode {
  Popup, Inline, PopupAndInline }` (a popup, the completion appended as selected text, or both: K2.29†);
  `trigger_prefix: Value<String>` (W-P3-17†); `hover_selects: Value<bool>` (GN-2.56†); `placement: Placement` (where
  its list goes, the host's value by Nested controls: `GtkPopover` GN-2.56†, `QtCompletionBox` K2.29† and K-KA-13†,
  `MaterialMenu` MAT-N27, `WinUiAutoSuggest` W2-15 and W-P3-17†, `AppKitComboList` MAC-CB01, `AppKitMenu` MAC-P3-7†,
  `UiKitMenu` C-38†, `KTextEditorCompletion` K-P3-13† by a rule in `context code_editor`; with its width, the
  field's: `AnchorWidth`, GN-2.56†, K2.29†, K-KA-13†, and its row limit `max_visible_items` among its parameters).
- **Variants:** keyed `ios-*` `@menu` (the inline bar placement), `@list` (the stacked placement) (C-38†). Material's
  filled and outlined fields (MAT-N27) are the field's own (`input @filled`).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own; the list's rows carry `menu`'s states, and `placed_above`, `placed_below` are `menu`'s
  (Windows: the attached corners square off on the side touching the field, W2-15).
- **Exclusive groups:** none (the completion holds no interaction state; the suggestion rows' groups are `menu`'s:
  the ListViewItem CommonStates on `windows-11`, the menu items' state layers on `material-3`); precedence: state
  models (S17).
- **may_be_absent:** parts: `token` (all but `windows-11`), `details_pane` (all but `adwaita`, `kde-breeze`, and
  outside `context code_editor`); fields: `body.padding`; values: `trigger_prefix` (all but `windows-11`),
  `filter_delay_ms` (all but `adwaita`), `mode` (all but `kde-breeze`), `hover_selects` (all but `adwaita`).
- **State-model source:** GNOME `gtkentrycompletion.c` and the GtkTreeView row selectors; KDE `kcompletionbox.cpp`
  (Breeze item-view branches), kirigami-addons `FormPathCompletion.qml`; Material autocomplete token files; iOS
  `UISearchSuggestionItem`; Windows the ListViewItem visual states of the AutoSuggestBox and RichSuggestBox
  templates; macOS `NSComboBox`, `NSTextView.complete(_:)` and `NSTextField.suggestionsDelegate` (measured,
  MAC-CB01, MAC-I06, MAC-P3-7†); the code editors' completion widgets (GN-SF13†, K-P3-13†).
- **Today's fields:** none.
- **Notes:** the list is `menu` in `context autocomplete`: its rows `menu.item`, their icons `menu.item_icon`, their
  text `menu.item_label` and detail columns `menu.item_subtitle` (C-38†, MAT-N27, W2-15; macOS's
  `selectedControlColor` highlight, MAC-X15). kirigami-addons' file row (K-KA-13†) opens this widget in
  `context form.row` (the form's `@ka_form_card`), its list on the K-KA-18† surface; a wrong path shows a `banner`
  there.

### avatar

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwAvatar (GN-2.14), KDE's Kirigami Avatar (Q2.28, Q2.29) and kirigami-addons'
  Avatar and AvatarButton (K-KA-33†), Windows' PersonPicture (W2-24), Material's avatars in list items, the top app
  bar, input chips and the search bar (MAT-L2, MAT-TA5, MAT-N9, MAT-N14; the Material cross-check).
- **Available:** every preset but `macos-*`, `ios-*` (`available = false`: no avatar control in their rows);
  `kde-breeze` flavour `kirigami` only (Qt Widgets draws none).
- **Parts:**
  - `body` — the circle (the root): `fill` (the colour or libadwaita's gradient picked by hashing the name: its
    colours `= pick(name_index(element.text, <count>, kirigami | adwaita), <c0>, …)`, a gradient's stops each a `pick`;
    ADD-B14, GN-2.14, Q2.28, K-KA-33†; the body declares `data = { body = ["text", "width"] }`), `stroke`
    (Kirigami's ring, PersonPicture's stroke), `corners`, `padding`, `focus` (AvatarButton's focus circle), `cursor`
    (AvatarButton's pointing hand), `bindings` (AvatarButton's Button set), `min_width`, `min_height`.
  - `initials` — the initials text: `text` (its own text style; its size follows the avatar's, an expression on
    `element.width`, which it declares, `data = { initials = ["width"] }`, ADD-B11; under `@contrasted` its own
    fill).
  - `fallback_icon` — the icon shown without a name or image (Kirigami's `user`, PersonPicture's default glyph):
    `icon`, `padding`.
  - `image` — the picture, clipped to the body's shape: `corners`, `opacity`.
- **Widget values:** `default_size_px: Value<Px>` (Q2.28, W2-24, MAT-L2); `fallback_icon_inset_px: Value<Px>`
  (Q2.29).
- **Variants:** keyed `adwaita` `@contrasted` (GN-2.14); `kde-breeze` `@ka_avatar` (kirigami-addons' Avatar),
  `@button` (AvatarButton) (K-KA-33†).
- **Sizes and densities:** none (the size is the element's own; `default_size` where it states none).
- **Flags:** none of its own; state flags used: focus_visible, pressed (AvatarButton's focus circle, K-KA-33†).
- **Exclusive groups:** none (W2-24 states no interaction states for PersonPicture; KDE's states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `fallback_icon` (`adwaita`); fields: `body.stroke` (`adwaita`); values:
  `fallback_icon_inset_px` (`adwaita`, `windows-11`).
- **State-model source:** GNOME `_avatar.scss`, `adw-avatar.c`; KDE `Avatar.qml` (Kirigami), kirigami-addons
  `Avatar.qml`, `AvatarButton.qml`; Windows the PersonPicture template.
- **Today's fields:** none.
- **Notes:** PersonPicture's badge (number, glyph or image: W2-24) is a `badge` element in `context avatar`; the
  avatar appears in `context about_dialog` (Q2.57†, K-KA-44†), `context list.row` (MAT-L2), `context toolbar.bar`
  (MAT-TA5), `context chip` (MAT-N9's input chip) and `context search_field` (MAT-N14). Windows' group form has no
  row, so no variant.

### background_extension

*From group 7's catalogue.*

- **Kind:** typed widget: iOS's `UIBackgroundExtensionView` (C-19) and macOS's `NSBackgroundExtensionView`
  (MAC-X06): content mirrored and blurred under a floating sidebar or inspector.
- **Available:** `macos-tahoe`, `ios-26` only (every other preset `available = false`: the view is new in 26,
  C-19, MAC-X06).
- **Parts:**
  - `extension` — the mirrored and blurred content (the root): `fill` (a `Material` paint, identity
    `background_extension`, over a transformed copy of the neighbouring content; the part carries the blur and tint,
    and the mirroring is a widget value), `bindings`.
- **Widget values:** `mirrors_content: Value<bool>` (C-19, MAC-X06).
- **Variants:** none (the rows state none).
- **Sizes and densities:** none.
- **Flags:** none (it draws behind content and has no interaction states).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none (the two presets that have it draw every part).
- **State-model source:** UIKit `UIBackgroundExtensionView`, AppKit `NSBackgroundExtensionView`; their appearances.
- **Today's fields:** none.
- **Notes:** used under a floating sidebar or inspector (`context sidebar.pane variant floating`). Group 8 had
  placed MAC-X06 on `window.content` (rename row 54).

### badge

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwIndicatorBin badges and the tab button's counter (GN-2.36, GN-2.37), KDE's
  Kirigami Badge (Q2.25–Q2.27), macOS's badge bezel style, SwiftUI `.badge` and toolbar badges (MAC-N11,
  MAC-TL05), Material's badge (MAT-N10), Windows' InfoBadge (W2-08), iOS's tab-bar, bar-button and list-row badges
  (C-15, T-7, TB-7).
- **Available:** every preset; `kde-breeze` flavour `kirigami` only (Kirigami's Badge).
- **Parts:**
  - `body` — the dot or the pill (the root): `fill`, `stroke` (Kirigami's border of a badge with an application-chosen
    colour, an expression on `element.color`, which it declares, `data = { body = ["color"] }`, ADD-B16, Q2.26),
    `corners`, `padding`, `min_width`, `min_height`.
  - `label` — the count or string: `text`.
  - `icon` — Windows' icon form, Kirigami's icon (Q2.27): `icon`, `opacity` (PersonPicture's image badge, W2-24).
- **Widget values:** `anchor_offset_x_px: Value<Px>`, `anchor_offset_y_px: Value<Px>` (the placement on its anchor icon
  or item: MAT-N10, GN-2.37, C-15, GN-1.41); `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear` (the needs-attention dot's appearance, GN-1.41); `anchor_corner: Value<BadgeAnchor>`,
  `BadgeAnchor { TopEnd, TopStart,
  TrailingInline }` (C-15's list-row badge is trailing inline); `cutout_px: Value<Px>` (the gap libadwaita's
  indicator bin masks out of the anchor around the badge, GN-2.36); `max_count: Value<u32>` (where a count is
  capped).
- **Variants:** typed form `dot` (GN-2.36, MAT-N10, W2-08, C-15), `count` (GN-2.36, MAT-N10, W2-08, C-15), `icon`
  (W2-08, Q2.27); typed severity `info`, `success`, `warning`, `error` (Q2.25's Information, Positive, Warning,
  Error; W2-08's Informational, Success, Caution, Critical), `attention` (Windows' Attention and libadwaita's
  needs-attention dot, both accent: W2-08, GN-2.36, GN-1.41, GN-2.33, GN-2.34); typed `circular` (macOS's circular
  badge, MAC-N11; the shape button's `circular` shares); the content variant `custom_color` (a badge the application
  gives its own colour, `element.color`: Kirigami's border, Q2.26; the connector sets it from the content, ADD-A61);
  keyed `macos-*` `@inline` (the deprecated count badge in source lists) (MAC-N11).
- **Sizes and densities:** `adwaita`: `regular`, `small` (the tab button's small counter, GN-2.37); Material's
  small (dot) and large (label) badges are the `dot` and `count` forms, not sizes.
- **Flags:** shared `multiline` (KDE: Kirigami's pill radius only while the label is on one line, Q2.27); state
  flags used: none (a badge has no interaction states; it follows its anchor through context).
- **Exclusive groups:** none (InfoBadge has no visual state groups: one style per form and severity, W2-08; the
  others draw no states); precedence: state models (S17).
- **may_be_absent:** parts: `icon` (`adwaita`, `macos-*`, `material-3`, `ios-*`); fields: `body.stroke` (all but
  `kde-breeze`; there outside `custom_color`), `icon.opacity`; values: `cutout_px` (all but `adwaita`), `max_count`
  (presets that cap no count).
- **State-model source:** GNOME `_view-switcher.scss` and `_buttons.scss` (`.needs-attention`, the tab button);
  KDE `kirigami/src/controls/Badge.qml` types; macOS `NSButton.BezelStyle.badge`, `NSItemBadge` and SwiftUI
  `.badge`; Material `_md-comp-badge.scss`; Windows `InfoBadge` styles (no visual state groups: one style per form
  and severity); iOS `UITabBarItem` badge attributes, `UIBarButtonItem.Badge`.
- **Today's fields:** none. Today's stand-ins `defaults.danger_color` (C-15) and the status colours
  `defaults.*_color` (Q2.25) become references in this widget's rules:
  `badge.body.fill = role.global.status.error.fill` under `variant error`, and likewise per severity
  (`role.global.status.*`).
- **Notes:** the badge is placed in context, not by keyed variants: on a tab-bar item `context
  bottom_tab_bar.item` (T-7, C-15), on a toolbar or bar button `context toolbar.bar` (TB-7, MAC-TL05, C-15), on a
  list row `context list.row` (C-15's SwiftUI `.badge`), on a menu item `context menu.item` (menu's own badge part
  goes), on an avatar `context avatar` (W2-24), on a dragged image `context drag_image` (the drag count: MAC-X21,
  C-24, W1-110); libadwaita's needs-attention dot, variant `attention`, on a stack or view switcher's button
  `context view_switcher.item` (GN-1.41, GN-2.33), on a GtkStackSidebar row `context list.row` (GN-2.34) and on a tab
  overview's
  thumbnail `context component.tab_overview.thumbnail` (GN-2.35): one structure wherever the dot is drawn, its
  offsets `anchor_offset_x_px`, `anchor_offset_y_px`, its appearance the badge's `Appear` presentation.
  Kirigami's text colour follows its background (`kirigami_badge_text`, ADD-A53; Q2.26).

### banner

*From group 6's catalogue.*

- **Kind:** typed widget, the in-window inline message: GNOME's AdwBanner (GN-2.11, GN-1.146) and GtkInfoBar
  (GN-2.31), KDE's KMessageWidget (K2.13, K3.37), Kirigami's InlineMessage (Q2.16–Q2.21), kirigami-addons'
  FormInlineMessage (K-KA-10†) and Banner (K-KA-29†), Material's banner (MAT-N23), Windows' InfoBar (W2-07).
- **Available:** every preset but `macos-*`, `ios-*` (`available = false`: no in-window message bar in their rows).
- **Parts:**
  - `body` — the root: `fill`, `stroke` (KDE's inline border; Kirigami's border in the type colour, Q2.16; W2-07's
    border), `corners` (per position, Q2.18; Material's shape, MAT-N23), `shadow` (Material's elevation, MAT-N23),
    `padding` (GN-2.11, GN-2.31, Q2.19, W2-07), `min_height` (W2-07).
  - `icon` — the severity icon: `icon` (size and colour; recoloured to the type colour, K-KA-10†), `glyph` (Windows'
    severity glyph on its backplate, W2-07), `margin` (InfoBar's per-layout margins, W2-07).
  - `icon_backplate` — Windows' fill behind the icon: `fill`.
  - `title`, `message` — the title and the message text: `text`, `opacity` (GNOME's dimmed content under
    `cond window_inactive`, GN-2.11), `margin` (InfoBar's per-layout margins, W2-07).
  - `action_area` — the action buttons' container: `opacity` (GNOME's dimmed buttons under `cond window_inactive`,
    GN-2.11), `margin` (InfoBar's per-layout margins, W2-07).
  - `divider` — Material's divider: `stroke`.
  - `edge_line` — the line on the inner edge (KDE's header and footer positions, K2.13, K-KA-29†; GtkInfoBar's shade
    line, GN-2.31): `stroke`.
  - `image` — Material's rich banner image: `corners`, `min_width`, `min_height` (its size and shape, MAT-N23).
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the banner shown
  and hidden: GNOME's slide-down, GN-2.11; KDE's height and opacity over the timeline, K2.13, K3.37; Kirigami's
  height, opacity and
  content opacity, Q2.21; K-KA-29† states none); `actions_wrap: Value<ActionsWrap>`, `ActionsWrap { Inline, Below,
  BelowWhenNotFitting }` (KDE moves the actions below the text when they do not fit, K2.13; Windows' horizontal
  and vertical layouts, W2-07); `content_gap_px: Value<Px>` (GN-2.11, K-KA-10†, K-KA-29†); `action_gap_px: Value<Px>`
  (GN-2.11, MAT-N23); Material's heights per line count and form factor are `body.min_height` rules under `{multiline}`
  and the size class (MAT-N23; one home); `inner_corner_factor: Value<f32>` (KDE's inner radius as a fraction of the outer, an expression on the body's
  corners, K2.13).
- **Variants:** typed severity `info`, `success`, `warning`, `error` (GN-2.31, K2.13, Q2.16, K-KA-10†, K-KA-29†,
  W2-07); typed `vertical` (the actions stacked below the text, set by the connector where `actions_wrap` puts them
  below: InfoBar's vertical layout with its own padding and margins, W2-07; AdwBanner's stacked layout with its own
  button widths and spacings, GN-2.11; the orientation many widgets share); keyed `adwaita` `@question` (GtkInfoBar,
  GN-2.31) (AdwBanner in a sidebar is a context, `context sidebar.pane`, GN-1.146, rename row 91);
  `kde-breeze` `@inline`, `@header`, `@footer` (K2.13's positions, K-KA-29†), `@ka_inline_message` (K-KA-10†),
  `@ka_banner` (K-KA-29†); `material-3` `@basic`, `@rich`, `@vibrant` (MAT-N23).
- **Sizes and densities:** none (Material's heights per line count are `body.min_height` rules).
- **Flags:** shared `multiline` (KDE's top-aligned icon, K-KA-10†, K-KA-29†; Material's two-line heights,
  MAT-N23); state flags used: none other (its action and close buttons are `button` elements with their own states,
  GN-2.31).
- **Exclusive groups:** none (its close and action buttons are `button` elements, whose groups, the CommonStates on
  `windows-11`, are `button`'s); precedence: state models (S17).
- **may_be_absent:** parts: `icon_backplate` (all but `windows-11`), `divider`, `image` (all but `material-3`),
  `edge_line` (`windows-11`, `material-3`), `icon` (`adwaita`); fields: `body.shadow` (all but `material-3`),
  `icon.glyph` (all but `windows-11`), `body.min_height` (presets that state none), `body.stroke` (`adwaita`,
  `material-3`), `icon.margin`, `title.margin`, `message.margin`, `action_area.margin` (presets that state none);
  values: `inner_corner_factor` (all but `kde-breeze`); `presentations` is a list, empty where no row states one
  (`windows-11`, `material-3`).
- **State-model source:** GNOME `_toolbars.scss` (`banner`), `_sidebars.scss`, `_deprecated.scss` (`infobar`);
  KDE `kmessagewidget.cpp`, `InlineMessage.qml`, kirigami-addons `FormInlineMessage.qml`, `Banner.qml`; Material
  banner tokens; Windows InfoBar severity states.
- **Today's fields:** none. Stand-ins `defaults.{info,success,warning,danger}_color` (GN-2.31, Q2.16, W2-07,
  K-KA-10†, K-KA-29†) become references to `role.global.status.*` (and KDE's `role.<set>.background_active`,
  `role.<set>.background_negative`, `role.<set>.background_neutral`, `role.<set>.background_positive`,
  K-KA-29†) in this widget's rules.
- **Notes:** Nested controls: its actions are `button` elements in `context banner.action_area`, its close button
  `button @ variant close context banner` (W2-07, K2.13, MAT-N23). GNOME's backdrop fill and dimmed content are rules
  under `cond window_inactive` (GN-2.11); the
  sidebar banner is this widget in `context sidebar.pane` (GN-1.146); kirigami-addons' field status is
  `@ka_inline_message` in `context form.row` (K-KA-10†, K-KA-13†); group 8 brought the presentations (K3.37).

### bottom_tab_bar

*Written in the merge.*

- **Kind:** typed widget: the application-level tab bar that switches the app's top-level views, apart from `tab`
  (tabs attached to a page or a document strip). iOS (`UITabBar`, iPadOS's top tab bar: T-1 … T-17†, P3-2†, C-22),
  KDE (Kirigami's `NavigationTabBar`: Q2.33–Q2.36), Material (the navigation bar: MAT-N12), macOS (SwiftUI's
  `TabView` with `.sidebarAdaptable`: MAC-N23), GNOME (`AdwViewSwitcherBar`, the bottom bar that switches the
  application's views in a narrow window, GN-2.15: decided by what is drawn, the same kind as Material's navigation
  bar and iOS's tab bar; the header bar's switchers stay `view_switcher`). Group 4 wrote its parts from iOS's rows;
  group 6 had proposed it as
  `tab` variant navigation; the merge keeps one widget, because its parts, layout and state model are not the
  tab's.
- **Available:** every preset but `windows-11` (`available = false`: no application-level tab bar; Windows'
  NavigationView is `sidebar` and, in Top mode, `view_switcher`); `kde-breeze` flavour `kirigami` only (MAC-N23 is
  the macOS preset's own release).
- **Parts:**
  - `bar` — the bar (the root): `fill` (paint with material; transparent at the scroll edge by `scrolled`, T-1),
    `stroke`
    (the top hairline, one edge), `corners`, `shadow` (the floating capsule's, its inset being `floating_inset`,
    iOS 26, T-9), `min_height` (by size class: T-14, ADD-A20), `padding`.
  - `item` *(element)* — one destination: `fill`, `stroke`, `corners`, `padding`, `min_height` (Q2.34), `focus`
    (Q2.36), `bindings` (Kirigami's Window set, Q2.34), `transitions` (Q2.35); per-state colours (normal, selected,
    disabled, focused: T-3, MAT-N12). `item_icon`, `item_label`, `indicator` belong to `item` and are queried with it.
  - `item_icon` — `icon` per state (the selected symbol variant: `item_icon.icon.symbol_variants`, ADD-A37).
  - `item_label` — `text` per state (weight under `{selected}`, elision).
  - `indicator` — the selected item's indicator (Material's active indicator, MAT-N12; iOS 26's glass lens, T-8):
    `fill`, `corners`, `min_width`, `min_height`.
  - `drop_indicator` — where a dragged tab would land (T-17†): `fill`, `min_width`, `min_height`.
  - `accessory`, in `bar` — iOS 26's bottom accessory, the capsule above or inside the bar (`UITabAccessory`, C-21,
    T-12): `fill` (a `Material` paint), `corners` (a capsule), `shadow`, `padding` (its insets), `min_height` (its
    height, C-21). `accessory_content` nests in it.
  - `accessory_content`, in `accessory` — the application's view area in it: `padding`.
- **Widget values:** `floating_inset: Sides<Value<Px>>` (T-9); `item_gap_px: Value<Px>`, `item_width_px: Value<Px>`,
  `item_positioning: Value<Key>` (the keys of `UITabBar.ItemPositioning`, T-3); `max_items: Value<u32>` (P3-2†, by
  size class); `max_content_width_px: Value<Px>`, `equal_width_items: Value<bool>` (Q2.33); `icon_text_gap_px:
  Value<Px>` (an item's icon to its label, Q2.34); the values read the item count, `data = { values = ["item_count"]
  }` (Kirigami's content width, Q2.33); `presentations: Vec<PresentationTransition>`: events `@indicator_move` (the
  selection changing: a step on part `indicator`, the selection slide, Morph, T-8, MAT-N12, and the items' own
  change, T-15),
  `Appear`, `Disappear` (the bar shown and hidden, `setTabBarHidden(_:animated:)`, T-15; AdwViewSwitcherBar's reveal,
  GN-2.15), `@minimize` (T-10), `@customize` (T-17†, P3-2†), `@to_sidebar` (iPad's tab bar becoming a sidebar, C-22),
  `@spring_load` (an item a drag hovers switching to it, P3-11†), `@accessory_environment` (the bottom accessory
  morphing between its environments, C-21);
  `focus_visible_when` (type `FocusVisibleWhen`; Q2.36, ADD-A16).
- **Variants:** typed `floating` (iOS 26's capsule, T-9); typed item layouts `stacked` (the icon above the label: iOS's
  stacked items, T-3; Material Expressive's vertical items, MAT-N12) and `inline` (the icon beside the label: iOS's
  inline items, T-3; Material Expressive's horizontal items, MAT-N12; Material's under `cond flavor:expressive`);
  the content variants `icon_only` and `with_icon` on `item` (ADD-A61; an item with neither is text-only, Kirigami's
  larger text-only label, Q2.34); keyed `ios-*` `@compact_inline` (T-3), `@search` (T-11), `@prominent` (T-16),
  `@more` (P3-2†), `@tab_sidebar` (iPadOS's top bar, T-13), `@accessory_inline`, `@accessory_expanded` (the bottom
  accessory's environments, `UITabAccessory.Environment`, T-12; Open items); `macos-*` `@sidebar_adaptable`
  (MAC-N23).
- **Sizes and densities:** none; the bar's height follows the size class (`v_size_class compact`, T-14).
- **Flags:** shared `minimized` (iOS, T-10); state flags used: hover, pressed, selected (the current destination
  on every platform: Kirigami's checked tab button maps to it, Q2.35; S7.2), disabled, focus_visible, dragged,
  drop_target (on `bar`, T-17†), scrolled (T-1).
- **Exclusive groups:** `material-3`: `[pressed, focus_visible, hover]` (the navigation bar's state layers,
  MAT-N12); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `indicator` (`kde-breeze`, `adwaita`: Kirigami's and AdwViewSwitcherBar's items draw
  none, Q2.35, GN-2.15; `ios-*` before 26: iOS 13–18 draw none), `drop_indicator` (all but `ios-*`: T-17†),
  `accessory`, `accessory_content` (all but `ios-26`); fields: `bar.stroke` (under `variant floating`), `bar.shadow` (presets whose bar is flat:
  `kde-breeze`, `ios-*` before 26); values: `floating_inset` (all but `ios-26`), `max_items` (all but `ios-*`),
  `max_content_width_px`, `equal_width_items` (all but `kde-breeze`), `icon_text_gap_px` (presets that state
  none).
- **State-model source:** iOS `UITabBarItemAppearance` (normal, selected, disabled, focused) per layout appearance
  (stacked, inline, compact inline), `UITabBarController.Mode`, `tabBarMinimizeBehavior`; KDE Kirigami
  `NavigationTabBar.qml` and `NavigationTabButton.qml` (hover, pressed, checked, focus); Material the
  navigation-bar tokens (`_md-comp-navigation-bar.scss`, `_md-comp-nav-bar*.scss`: active, inactive × hover, focus,
  pressed); macOS SwiftUI `TabView` (sidebar-adaptable and toolbar tab bar, MAC-N23, measured); GNOME
  `_view-switcher.scss` (the switcher bar's buttons, GN-2.15).
- **Today's fields:** none (today's `tab.*` stand-ins stay `tab`'s).
- **Notes:** the badge on an item is the `badge` widget in `context bottom_tab_bar.item` (T-7); the More list's rows
  are `list` rows in `context bottom_tab_bar` (P3-2†); iOS 26's bottom accessory is this widget's `accessory` (a
  one-platform piece of a typed widget is a host part, Nested controls; C-21, T-12). iPadOS's top tab bar (T-13)
  stays here, not in `view_switcher`: it is the same `UITabBarController` bar, its items, states and its change into
  a sidebar (`@to_sidebar`, C-22) this widget's, placed at the top; `view_switcher` takes the switchers that are no
  application tab bar controller. Group 4 named the indicator
  `selection_pill` (rename row 33).

### breadcrumb

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkPathBar (GN-2.52†), KDE's KUrlNavigator (K2.27†, K-P4-11†) and Kirigami's
  breadcrumb (Q2.44), macOS's `NSPathControl` (MAC-N07), Windows' BreadcrumbBar (W2-10).
- **Available:** every preset but `ios-*`, `material-3` (`available = false`: no path control in their rows).
- **Parts:**
  - `bar` — the strip (the root): `fill`, `padding`.
  - `crumb` *(element)* — one path component (KDE's flat button panel on hover, drag-over or open popup; GNOME's
    path-bar buttons, toggle buttons linked by position rules, GN-2.52†): `fill`, `stroke`, `corners` (Q2.44),
    `padding` (K2.27†, GN-2.52†), `focus` (W2-10). `crumb_label`, `crumb_icon` belong to `crumb` and are queried with
    it.
  - `crumb_label` — its text: `text` (the current crumb's own text style under `current`; KDE's fade-out of clipped
    names), `opacity` (Kirigami's non-current crumbs, Q2.44), `padding` (GN-2.52†).
  - `crumb_icon` — its icon (GNOME's root, home, desktop and recent icons, MAC-N07's component icons): `icon`.
  - `separator` *(element: KDE's arrows are hovered and open a menu)* — the chevron between crumbs: `glyph`,
    `padding` (W2-10).
  - `ellipsis` *(element: Windows' overflow item, a crumb showing an ellipsis)*: `glyph`, `padding`, `focus`.
  - `placeholder` — macOS's placeholder (MAC-N07): `text`.
  - `bottom_separator` — Kirigami's Header-set line (Q2.44): `stroke`, `padding` (its top and bottom margins),
    `bindings` (the Header set).
- **Widget values:** `icon_text_gap_px: Value<Px>` (between a crumb's icon and its label, GN-2.52†);
  `presentations: Vec<PresentationTransition>`: event `@scroll` (Kirigami's scroll to the current crumb, Q2.44);
  `editable: Value<bool>` (KDE's editable-path
  mode, whose field is an `input` in context, K2.27†); `drag_hover_delay_ms: Value<Ms>` (a drag resting on a
  crumb's arrow opens its sub-folder menu after this delay, K-P4-11†).
- **Variants:** the content variant `with_icon` on `crumb` (GtkPathBar's root, home, desktop and recent buttons,
  GN-2.52†; ADD-A61); keyed `macos-*` `@standard`, `@pop_up` (MAC-N07).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** shared `current` (the current folder's crumb: KDE's label, K2.27†, Q2.44; Windows' current-item
  colour, W2-10; macOS, MAC-N07; GNOME's current folder button, whose toggle is checked, maps to it, GN-2.52†);
  keyed `kde-breeze` `@navigator_active` (the navigator of the focused view, K2.27†); state
  flags used: hover, pressed, focus_visible, disabled, open (a crumb's popup shown), drop_target (dragged over,
  K2.27†, K-P4-11†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (BreadcrumbBarItem CommonStates; `current`
  combines); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `ellipsis` (all but `windows-11`), `placeholder` (all but `macos-*`), `crumb_icon`
  (`windows-11`), `separator` (`adwaita`: GtkPathBar links its buttons without separators), `bottom_separator` (all
  but `kde-breeze`); values: `editable`, `drag_hover_delay_ms` (all but `kde-breeze`), `icon_text_gap_px` (presets
  whose crumbs show no icon beside a label).
- **State-model source:** GNOME `gtkpathbar.c` and the button selectors; KDE `kurlnavigatorbutton*.cpp`,
  `kurlnavigatorbuttonbase.cpp` and `BreadcrumbControl.qml`; macOS `NSPathControl.Style`; Windows
  BreadcrumbBarItem visual states.
- **Today's fields:** none (W2-10's stand-in `link.*` stays `link`'s).
- **Notes:** a crumb is the path control's own piece on every platform (Nested controls): GNOME's path-bar toggle
  buttons are `crumb` elements, their facts GtkPathBar's button selectors and their joined corners `pos` rules
  (GN-2.52†). Nested controls: KDE's places selector is `button @ variant menu context breadcrumb` (KUrlNavigator's,
  K2.27†); the ellipsis's overflow flyout (W2-10) and KDE's sub-folder menus are `menu` elements in
  `context breadcrumb`; the editable path's field an `input` there (K2.27†). Group 8 had named the drag delay
  `submenu_open_delay_ms` (K-P4-11†).

### calendar

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkCalendar (GN-2.25), iOS's `UICalendarView` and inline date picker (C-6), KDE's
  QCalendarWidget and KDatePicker (K2.4), KDateTable (K2.23†) and kirigami-addons' date popup grids (K-KA-20†,
  K-KA-21†), Windows' CalendarView (W2-02), Material's date-picker grid (MAT-N16), macOS's `NSDatePicker` graphical
  style (MAC-N04).
- **Available:** every preset (C-6).
- **Parts:**
  - `frame` — the outer border (the root, GN-2.25): `fill`, `stroke`, `padding`.
  - `header` — the month header, which holds the navigation and title buttons: `stroke` (its bottom line, GN-2.25),
    `padding`.
  - `title` — the month and year text where it is not a button: `text`.
  - `weekday_label` *(element)* — one weekday's header cell, with its own flags (a non-working day's colour,
    K2.23†): `text` (its own text style; non-working days' header colour, K2.23†),
    `fill` (on `Window`, K2.23†), `stroke` (the line under the header, K2.23†), `opacity` (K-KA-21†).
  - `week_number` — `text`.
  - `day` *(element)* — a day cell: `fill`, `corners`, `stroke`, `padding` (GN-2.25, W2-02), `min_width`,
    `min_height` (Material's cell size, MAT-N16), `opacity` (other-month days: K-KA-21†, MAT-N16), `focus` (GN-2.25,
    K-KA-21†). `day_label`, `today_marker`, `decoration`, `range_band` belong to `day`, nest in it and are queried
    with it.
  - `day_label` — its number: `text` (tabular figures, GN-2.25; Windows' blackout strikethrough, W2-02).
  - `today_marker` — GNOME's underline, Material's outline: `stroke`, `corners`.
  - `decoration` — iOS's dot, image or custom decoration (C-6): `fill`, `corners`, `icon`, `min_width`, `min_height`.
  - `range_band` — Material's range selection band behind a day, by the day's own `@range_start`, `@range_end`,
    `@in_range` (MAT-N16): `fill`, `min_height`.
  - `year` *(element: Material's year cells, Windows' year and decade items, K-KA-21†'s year view)*: `fill`, `stroke`,
    `corners`, `text`, `min_width`, `min_height` (MAT-N16), `opacity` (years outside the decade, K-KA-21†), `focus`.
  - `month` *(element: Windows' month items, K-KA-21†'s month view)*: `fill`, `stroke`, `corners`, `text`, `focus`.
  - `clock_face` — macOS's clockAndCalendar style: `fill`, `stroke`.
  - `clock_hand` — macOS's clockAndCalendar style: `stroke`.
  - `time_row` — iOS's time row: `text`, `padding`.
- **Widget values:** `cell_gap_px: Value<Px>`; `grid_padding: Sides<Value<Px>>` (GN-2.25); `presentations:
  Vec<PresentationTransition>`: event `@view_switch` (Windows' zoom between the day, month and year views, W2-02;
  kirigami-addons' view switch, K-KA-20†); `first_weekday_from_locale: Value<bool>`; `scroll_physics:
  ScrollPhysics` (a plain value, S4.2's: kirigami-addons' endless views, deceleration, snap to a page, K-KA-20†).
- **Variants:** keyed `ios-*` `@wheels` (the month-year wheel, C-6); `windows-11` `@month_view`, `@year_view`,
  `@decade_view` (W2-02); `kde-breeze` `@ka_date_popup` (K-KA-20†, K-KA-21†).
- **Sizes and densities:** `macos-*`: the control sizes AppKit draws the graphical style in (MAC-N04; which: value
  phase).
- **Flags:** shared `today` (all six: GN-2.25, C-6, K2.23†, K-KA-21†, W2-02, MAT-N16, MAC-N04), `other_month`
  (GNOME, Windows' out-of-scope, KDE, Material, macOS: GN-2.25, W2-02, K2.23†, K-KA-21†, MAT-N16, MAC-N04); keyed
  `windows-11` `@blackout` (W2-02's strikethrough), `adwaita` `@marked` (GNOME's
  checked day, GN-2.25), `kde-breeze` `@non_working_day` (on `day` and `weekday_label`), `@observance_day` (K2.23†),
  `@other_decade` (on `year`, K-KA-21†), `material-3` `@range_start`, `@range_end`, `@in_range` (on `day`, MAT-N16); state flags used: hover, pressed, selected,
  disabled (on a day: a day outside the allowed range, which cannot be chosen, one flag for KDE's and iOS's
  out-of-range days, K2.23†, K-KA-21†, C-6), focus_visible, on `day`, `year`, `month`.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CalendarView items; `selected`, `today`,
  `other_month` and `@blackout` combine, as the template's selected-hover, selected-pressed and selected-disabled
  states show); `material-3`: `[pressed, focus_visible, hover]` (the date cells' state layers); others none
  (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `week_number` (all but `kde-breeze`), `decoration`, `time_row` (all but `ios-*`),
  `range_band` (all but `material-3`), `clock_face`, `clock_hand` (all but `macos-*`), `today_marker`
  (`kde-breeze`, `windows-11`: today is the day's fill or text, K2.23†, K-KA-21†, W2-02), `month` (`adwaita`,
  `ios-*`, `material-3`, `macos-*`), `year` (`adwaita`, `ios-*`, `macos-*`); fields: `weekday_label.fill`,
  `weekday_label.stroke`,
  `weekday_label.opacity` (all but `kde-breeze`), `header.stroke` (all but `adwaita`, `kde-breeze`: GN-2.25, K2.23†),
  `frame.fill`, `frame.stroke` (`ios-*`, `material-3`, `macos-*`: the grid draws no frame of its own); values:
  `scroll_physics` (all but `kde-breeze`), `grid_padding` (all but `adwaita`).
- **State-model source:** GNOME `_calendar.scss` selectors; KDE `KDateTable` and Breeze `PE_PanelItemViewItem`,
  kirigami-addons `DatePicker.qml`, `DatePickerDelegate.qml`; Windows `CalendarView` item states; Material
  date-picker tokens; iOS `UICalendarView` and `UIControl.State`; macOS `NSDatePicker` graphical style.
- **Today's fields:** none.
- **Notes:** Nested controls: the previous and next buttons are `button @ variant previous / next context
  calendar.header`, and Windows' month-year header button and iOS's month-year toggle `button @ context
  calendar.header` (W2-02, C-6). A date picker's grid is this widget in `context
  date_picker`: Material's (MAT-N16), macOS's
  clockAndCalendar (`context any date_picker variant inline`, MAC-N04), and MAC-N04's calendar overlay
  (`presentsCalendarOverlay`), which is a `calendar` in `context date_picker.popup`, placed by
  `date_picker.values.placement`. kirigami-addons' day, month and year switch is a `segmented_control`
  (`@ka_segmented_button`) in the popup `dialog`'s body, `context dialog.body context date_picker` (K-KA-20†). K2.23†'s hard-coded holiday colours are
  dropped under a high-contrast preference (rules the contrast conditions exclude; CONTRA-1).

### carousel

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwCarousel (GN-2.38), Material's carousel (MAT-N18), Windows' FlipView (W2-31),
  iOS's `UIPageViewController` (C-48†). Its indicator is a `page_indicator` in context.
- **Available:** every preset but `kde-breeze`, `macos-*` (`available = false`: no carousel in their rows).
- **Parts:**
  - `body` — the carousel's viewport (the root): `padding`, `corners`, `bindings`.
  - `item` *(element: Material's item container with state layers, Windows' item background)* — `fill`, `stroke`,
    `corners`, `shape`, `shadow` (Material's item elevation, MAT-N18), `min_width`, `max_width` (Material's mask
    changing across its keylines with the scroll position: expressions over `element.scroll_progress`, which it
    declares, ADD-B13, MAT-N18), `transitions` (the page settle: a `Geometry` transition with a spring timing,
    GNOME's page spring GN-2.38, Material's item settle MAT-N18).
- **Widget values:** `item_gap_px: Value<Px>` (the gap between items or pages: MAT-N18; C-48†'s `interPageSpacing`,
  under `@scroll`); `scroll_timeout_ms: Value<Ms>` (GN-2.38);
  `nav_buttons_visibility: Value<NavButtonsVisibility>` (when its previous and next buttons show, W2-31);
  `presentations: Vec<PresentationTransition>`: event `@page_change` (the page turn: under `@page_curl` a keyed
  presentation kind `page_curl` with its parameters, the back of the page and its shading among them, ADD-A26, the
  curl following the drag through `element.progress`, which the values declare, `data = { values = ["progress"] }`,
  ADD-B12; under `@scroll` a `Slide`; C-48†).
- **Variants:** keyed `material-3` `@multi_browse`, `@hero`, `@uncontained` where its tokens state them (MAT-N18);
  `ios-*` `@page_curl`, `@scroll` (C-48†), `@spine_min`, `@spine_mid`, `@spine_max` (the curl's spine,
  `UIPageViewController.SpineLocation`, C-48†).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own; state flags used: hover, pressed, focus_visible, from Material's item state layers
  (MAT-N18).
- **Exclusive groups:** `material-3`: `[pressed, focus_visible, hover]` (the item's state layers); others none (states
  combine; the FlipView buttons' CommonStates on `windows-11` are `button`'s); precedence: state models (S17).
- **may_be_absent:** fields: `item.stroke`, `item.shadow` (all but `material-3`), `item.fill` (`adwaita`, `ios-*`);
  values: `scroll_timeout_ms` (all but `adwaita`), `nav_buttons_visibility` (all but `windows-11`),
  `item_gap_px` (`adwaita`, `windows-11`); `presentations` is a list, empty where the platform has none.
- **State-model source:** GNOME `adw-carousel.c`; Material carousel-item tokens; Windows FlipView button states;
  iOS `UIPageViewController.TransitionStyle`.
- **Today's fields:** none.
- **Notes:** Nested controls: Windows' previous and next buttons are `button @ variant previous / next context
  carousel` (shown on hover, scaled on press, W2-31). Material's masked shape change while scrolling is driven by the
  scroll position (ADD-B13). GNOME's indicator dots and lines are `page_indicator` elements in `context carousel`
  (GN-2.38). Group 6's
  `page_transition` value is the item's `transitions`.

### chart

*From group 6's catalogue.*

- **Kind:** typed widget: Swift Charts on iOS (C-45†) and macOS (MAC-N37†), KDE's KQuickCharts controls
  (K-P3-10†).
- **Available:** every preset but `adwaita`, `windows-11`, `material-3` (`available = false`: no chart in their
  rows; the `macos-*` and `ios-*` presets' releases have Swift Charts, MAC-N37†, C-45†); `kde-breeze` flavour
  `kirigami` only
  (KQuickCharts).
- **Parts:**
  - `body` — the chart (the root): `fill` (KDE's View background, K-P3-10†), `padding`, `bindings`.
  - `plot_area` — the plotting area: `padding` (its insets, C-45†).
  - `grid_line_major`, `grid_line_minor` — grid lines: `stroke`.
  - `axis_tick` — `stroke` (C-45†).
  - `axis_label` — value labels: `text` (font, colour).
  - `legend` — `text` (the legend font, C-45†, MAC-N37†), `padding` (K-P3-10†).
  - `legend_swatch` *(element)* — a series' colour mark, its colour as the series': `fill`, `min_width` (K-P3-10†).
  - `legend_label` — `text`.
  - `series` *(element)* — a series' mark: `fill` per series, `= pick(element.item_index, <c0>, …)` in Swift Charts'
    default order (C-45†, MAC-N37†) or `= kqc_series_color(<seed>, element.item_count, element.item_index)` from the
    highlight colour on KDE (ADD-A57, ADD-B15, K-P3-10†), the part declaring `data = { series = ["item_index",
    "item_count"], legend_swatch = ["item_index", "item_count"] }`.
  - `selection_mark` — Swift Charts' selection mark: `stroke`.
- **Widget values:** `grid_lines: ChartGridLines { horizontal: GridLineCounts, vertical: GridLineCounts }`,
  `GridLineCounts { major_frequency: Value<u32>, minor_frequency: Value<u32>, major_count: Value<u32>, minor_count:
  Value<u32> }` (KQuickCharts' grid: every n-th value line, or n lines, major and minor, K-P3-10†; Swift Charts'
  `AxisMarks` defaults, C-45†); `labels_per_axis: Value<u32>` (K-P3-10†); `legend_gap_px: Value<Px>` (K-P3-10†);
  `default_size_px: Value<Px>` (KQuickCharts' pie control, K-P3-10†).
- **Variants:** keyed `kde-breeze` `@line`, `@pie` (K-P3-10†).
- **Sizes and densities:** none.
- **Flags:** none (the rows state no interaction states; Swift Charts' selection is the `selection_mark` part).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `selection_mark` (`kde-breeze`); fields: `grid_line_minor.stroke` (`macos-*`, `ios-*`
  where Swift Charts draws none); values: `labels_per_axis`, `default_size_px` (`macos-*`, `ios-*`), `grid_lines`
  (members a platform does not state).
- **State-model source:** Swift Charts `AxisMarks` defaults; KQuickCharts controls (`KirigamiTheme.qml`,
  `ColorGradientSource`).
- **Today's fields:** none.
- **Notes:** none (the rows place no other widget in a chart's context).

### chip

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's Kirigami Chip (Q2.22–Q2.24), Material's chips (MAT-N9), Windows' Toolkit
  TokenizingTextBox tokens (W2-43†), macOS's `NSTokenField` tokens (MAC-N03), iOS's search tokens (C-3).
- **Available:** every preset but `adwaita` (`available = false`: no chip in its rows); `kde-breeze` flavour
  `kirigami` only (Kirigami's Chip).
- **Parts:**
  - `body` — the root: `fill`, `stroke`, `corners`, `padding`, `shadow` (Material's elevated chips), `min_height`
    (MAT-N9), `opacity` (Material's disabled chips, MAT-N9), `focus` (MAT-N9).
  - `label` — `text`.
  - `leading_icon`, `trailing_icon` — `icon`.
  - `menu_arrow` — the token's menu arrow (MAC-N03): `glyph`.
- **Widget values:** `icon_text_gap_px: Value<Px>` (Q2.24); KDE's extra padding when there is no icon is
  `body.padding @ !variant with_icon` (Q2.24; one home).
- **Variants:** keyed `material-3` `@elevated` (against flat), `@assist`, `@filter`, `@input`, `@suggestion`
  (MAT-N9); `kde-breeze` `@interactive`, `@closable` (Q2.24); `macos-*` `@rounded`, `@squared`,
  `@plain_squared` (the token styles, MAC-N03); the content variant `with_icon` (ADD-A61).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own; state flags used: checked (KDE, Material's filter chip: Q2.22, MAT-N9), selected
  (Material's input chip, Windows' token, macOS's selected token: MAT-N9, W2-43†, MAC-N03), dragged (MAT-N9),
  hover, pressed, focus_visible, disabled.
- **Exclusive groups:** `windows-11`: `[pressed, hover]` (the token's CommonStates: rest, pointer-over, pressed;
  `selected` combines, as its selected-pointer-over and selected-pressed states show, W2-43†); `material-3`:
  `[dragged, pressed, focus_visible, hover]` (the chips' state layers); others none (states combine); precedence:
  state models (S17).
- **may_be_absent:** parts: `trailing_icon` (`kde-breeze`), `leading_icon` (`windows-11`), `menu_arrow` (all but
  `macos-*`); fields: `body.shadow` (all but `material-3`).
- **State-model source:** KDE `DefaultChipBackground.qml`, `Chip.qml`; Material chip tokens; Windows
  `TokenizingTextBoxItem.Token.xaml` states; macOS `NSTokenField.TokenStyle`.
- **Today's fields:** none.
- **Notes:** a token field is one structure on macOS and Windows (MAC-N03, W2-43†): an `input` (the field's frame,
  text and caret) holding a `layout @ variant flow` (`context any input`: the tokens' gaps, `item_gap_px`,
  `line_gap_px`) of `chip` elements (`chip @ context any input`, their token styles the keyed
  `@rounded`, `@squared`, `@plain_squared` on `macos-*`, MAC-N03; rename row 52); iOS's search tokens are chips in
  `context search_field` (C-3). Material's chip set is a `layout` (`flow`) holding chips. Nested controls: the remove button is `button @ variant remove context chip` (KDE's `edit-delete-remove`,
  Q2.24; Windows' remove button, W2-43†); Material's input-chip avatar is `avatar @ context chip` (MAT-N9).

### code_editor

*From group 6's catalogue.*

- **Kind:** typed widget: GtkSourceView and KTextEditor, both in by R1.2 (GN-SF01†–GN-SF13†, GN-P3-27†,
  K-SF01†–K-SF11†, K-P3-11†–K-P3-14†).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: no code
  editor of the platform); `kde-breeze` flavour `widgets` only (KTextEditor).
- **Parts:**
  - `body` — the editor's view (the root): `fill` (the editor's background, GN-SF02†, K-SF02†), `padding`,
    `bindings`.
  - `text` — the normal text (GN-SF02†, K-SF02†): `text`.
  - `selection` *(element)* — the selected-text highlight, one element per selected range, as on `input` and
    `text_area`; GtkSourceView's unfocused selection is a rule `@ context code_editor.body !{focused}` (GN-SF02†):
    `fill`, `text`.
  - `caret` — the caret, a part of the editor as on `input` and `text_area` (it takes the editor's states: focus,
    `text_selected`, `overwrite`): `fill` (base `= token.caret.color`; KTextEditor draws it in the colour of the text
    under it, `= element.color`, which the connector supplies as the part's data, `data = { caret = ["color"] }`,
    K-P3-11†), `min_width`, `min_height` (the line, and Vi mode's block, underline and half block, K-P3-11†), `text`
    (the character redrawn inside the block caret under `{overwrite}`, GN-P4-4†).
  - `secondary_caret` — the bidirectional secondary caret, as on `input` and `text_area` (GtkSourceView's: its
    colour's base `= token.caret.secondary`, GN-3.37; restated by the scheme's `secondary-cursor` where stated, else
    derived from the scheme's `cursor` at the source's alpha factor, GN-SF03†): `fill`, `min_width`, `min_height`.
  - `extra_caret` — KDE's multiple cursors (K-P3-11†): `fill`, `min_width`, `min_height`.
  - `current_line` *(element)* — the current line, with KDE's mark flags of its own (GN-SF04†, K-SF03†): `fill`.
  - `gutter` (GN-SF05†, K-SF04†) — `fill`.
  - `line_number` — `text`.
  - `current_line_number` — `text`, `fill` (GN-SF05†).
  - `gutter_separator` — `stroke` (K-SF04†'s separator, GN-SF05†'s border).
  - `fold_marker` (K-SF04†) — `glyph` (the marker), `fill` (the range highlight).
  - `mark` *(element)* — a line mark (K-SF04†): `fill`.
  - `modified_line_marker`, `saved_line_marker` (K-SF05†) — `fill`.
  - `invisible` *(element)* — the mark drawn for an invisible character, as on `text_area` (GN-SF06†, K-SF06†):
    `glyph` (its size the mark's size, K-SF06†).
  - `indentation_guide` (K-SF06†) — `stroke`.
  - `bracket_match` — `fill`, `text` (GN-SF07†, K-SF07†), `transitions` (KDE's optional bracket animation, K-SF07†).
  - `bracket_mismatch` (GN-SF07†) — `fill`, `text`.
  - `find_match` *(element)* — a search match, as on `text_area`; the current match is selected (GN-SF08†, K-SF08†,
    K2.30†): `fill`, `text`.
  - `replace_match` (GN-SF08†, K-SF08†) — `fill`, `text`.
  - `right_margin` (GN-SF09†, K-SF09†) — `stroke`.
  - `right_margin_area` (GN-SF09†) — `fill`.
  - `dynamic_wrap_indicator` (K-SF09†) — `fill`.
  - `background_pattern` (GN-SF11†) — `fill`.
  - `syntax_span` *(element)* — a run of one syntax role, selected by its own variant (the role) and its own
    `{selected}` (GN-SF10†, K-SF10†): `text`, `fill` (the optional background).
  - `misspelling` — the spell-check mark, as on `input` and `text_area` (GN-P3-27†, K-SF11†): `text` (the
    underline).
  - `template_field` (K-SF11†) — `fill`.
  - `template_placeholder` *(element)* (K-SF11†) — `fill`.
  - `inline_annotation` *(element)* — one annotation, its severity its own variant (GN-SF12†): `text`.
  - `annotation_border` *(element)* — the border of one annotated run, hovered on its own (K-P3-14†): `padding` (the
    text inset), `stroke` (the outline round a hovered run).
  - `minimap` *(element)* — hovered on its own (GN-SF11†, K-P3-12†): `fill`, `stroke` (the line along its edge).
    `minimap_slider`, `minimap_fade`, `scroll_mark` nest in it.
  - `minimap_slider`, in `minimap` (GN-SF11†, K-P3-12†) — `fill`, `stroke`.
  - `minimap_fade`, in `minimap` (K-P3-12†) — `fill`.
  - `scroll_mark`, in `minimap` (K-P3-12†) — `fill`, `min_height`.
- **Widget values:** `gutter_padding: Sides<Value<Px>>` (GN-SF05†); `right_margin_column: Value<u32>`
  (GN-SF09†); `minimap_width_px: Value<Px>` (K-P3-12†) (the invisibles' mark size is `invisible.glyph.size`,
  K-SF06†). The completion's placement (`KTextEditorCompletion` K-P3-13†, its vertical offset an expression on the
  line height, `GtkPopover` GN-SF13†) is its host's value, `autocomplete.values.placement @ context code_editor`.
- **Variants:** keyed `adwaita` `@classic` (GtkSourceView's `classic` scheme, the default when the application sets
  none, beside the Adwaita and Adwaita-dark bases: GN-SF01†, R25.11, ADD-A64). `syntax_span` takes one variant
  per syntax role: typed where both platforms name the role, `comment`, `string`, `function`, `constant`,
  `preprocessor`, `special_char`, `error`, `warning`, `type` (GtkSourceView `type`, KSyntaxHighlighting
  `DataType`), `decimal` (`DecVal`), `base_n`, `float` (`floating-point`); keyed for the rest (GtkSourceView's
  `def:` styles and Markdown roles, KSyntaxHighlighting's `Keyword`, `ControlFlow`, `Operator`, …, each declared
  by its preset) and for per-language overrides (GN-SF10†, K-SF10†). `mark`: typed `warning`, `error`, keyed
  `kde-breeze` `@bookmark`, `@breakpoint_active`, `@breakpoint_reached`, `@breakpoint_disabled`, `@execution`
  (K-SF04†). On the editor, keyed `kde-breeze` `@vi_block`, `@vi_underline`, `@vi_half_block` (Vi input mode's caret
  shapes, the editor's mode, K-P3-11†). `invisible`:
  typed `space`, `tab`, `line_break`, `no_break_space` (`text_area`'s, GN-SF06†, K-SF06†). `inline_annotation`: typed
  `warning`, `error`, keyed `adwaita` `@accent` (GN-SF12†).
- **Sizes and densities:** none.
- **Flags:** shared `overwrite` (GNOME, KDE: the block caret), `text_selected` (as on `input`); keyed `kde-breeze`
  `@line_has_mark_bookmark`,
  `@line_has_mark_breakpoint_active`, `@line_has_mark_breakpoint_reached`, `@line_has_mark_breakpoint_disabled`,
  `@line_has_mark_execution`, `@line_has_mark_warning`, `@line_has_mark_error` (on `current_line`: KDE mixes the
  current line with the marks the line carries, K-SF03†); state flags used: selected (on `syntax_span`: KDE's
  selected-text colours, K-SF10†), focused (on the editor, which the `selection` rules select by context; on
  `template_placeholder`), read_only (on `template_placeholder`, K-SF11†), hover (on `annotation_border`,
  K-P3-14†; on `minimap`), disabled.
- **Exclusive groups:** none (GtkSourceView's and KTextEditor's states combine); precedence: state models (S17).
- **may_be_absent:** parts: `extra_caret`, `fold_marker`, `mark`, `modified_line_marker`, `saved_line_marker`,
  `indentation_guide`, `dynamic_wrap_indicator`, `template_field`, `template_placeholder`, `annotation_border`,
  `minimap_fade`, `scroll_mark` (`adwaita`: KTextEditor draws them alone), `bracket_mismatch` (`adwaita`: Adwaita's
  schemes state no mismatch style, GN-SF07†), `replace_match` (`adwaita`, where the scheme states no replace style),
  `background_pattern`, `inline_annotation`, `secondary_caret` (`kde-breeze`: GtkSourceView draws them alone),
  `caret` (by rule as on `input`: `{text_selected}` on all but `kde-breeze`, `{overwrite}`); values:
  `minimap_width_px` (`adwaita`), `gutter_padding` (`kde-breeze`).
- **State-model source:** GtkSourceView's style-scheme reference (`style-reference.md`), the Adwaita schemes,
  `GtkSourceView.css`; KTextEditor's `kateconfig.cpp` colour list, `katerenderer.cpp`, `kateviewhelpers.cpp` and
  the Breeze editor themes.
- **Today's fields:** none. The stand-ins today's `input` fields `caret_color`, `background_color` and
  `defaults.text_selection_background` (GN-SF02†, GN-SF03†, K-SF02†, K-P3-11†) are the scheme's values here, not
  those fields.
- **Notes:** Nested controls: the completion is an `autocomplete` in `context code_editor` (GN-SF13†, K-P3-13†): its
  rows `menu.item` with `item_icon`, `item_label` and the detail columns `item_subtitle`, its details pane
  `autocomplete.details_pane`; GtkSourceView's informative popup is a `popover` in `context code_editor` (GN-SF13†).
  KDE's reader reports which KSyntaxHighlighting theme draws (K-SF01†). Where the Adwaita schemes state no selection,
  GNOME's `selection` states the text-selection roles, `= role.global.selection_text_background` and
  `= role.global.selection_text_foreground` (and their unfocused roles under the unfocused rule), as `text_area`'s
  base does (GN-SF02†). The text widgets share one name and one element status per part kind: `caret`,
  `secondary_caret`, `misspelling` (parts) and `selection` (an element) on `input`, `text_area` and `code_editor`;
  `invisible` (typed `space`, `tab`, `line_break`, `no_break_space`) and `find_match` (the current match `{selected}`),
  elements, on `text_area` and `code_editor`. Group 6's `bracket_match_animation` is `bracket_match.transitions`
  (K-SF07†).

### color_button

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkColorDialogButton (GN-2.27), KDE's KColorButton (K2.20) and kirigami-addons'
  colour row swatch (K-KA-14†), macOS's `NSColorWell` (MAC-N05), Windows' Toolkit ColorPickerButton (W2-42†), iOS's
  `UIColorWell` (C-8).
- **Available:** every preset but `material-3` (`available = false`: no colour button in its rows).
- **Parts:**
  - `body` — the button (the root): `fill`, `stroke`, `corners` (its frame: K2.20, GN-2.27), `padding` (GN-2.27,
    W2-42†), `focus`.
  - `swatch` — the colour: `corners`, `min_width`, `min_height` (W2-42†), `stroke` (W2-42†'s preview border).
  - `swatch_ring` — GNOME's light-swatch ring (its paint `= gtk_contrast_pick(element.color, <on light>, <on dark>)`,
    GTK's lightness test, GN-2.27, ADD-A94; the part declares `data = { swatch_ring = ["color"] }`), macOS's active
    ring, iOS's multicolour ring: `stroke`, `corners` (GNOME's overlay radius, GN-2.27).
  - `checkerboard` — under a translucent colour (W2-42†): `fill`.
  - `chevron` — macOS's minimal style, Windows' drop-down chevron: `glyph` (per state).
  - `expand_button` *(element, macOS's expanded style)*: `fill`, `glyph`.
- **Widget values:** `swatch_inset: Sides<Value<Px>>` (K2.20); `swatch_gap_px: Value<Px>` (the space before the row's
  arrow, K-KA-14†).
- **Variants:** keyed `macos-*` `@minimal`, `@expanded` (MAC-N05; the default style is the base), `@hdr` (a well that
  takes HDR colours,
  `maximumLinearExposure` above one, MAC-N05).
- **Sizes and densities:** `macos-*`: the control sizes (MAC-N05; which: value phase).
- **Flags:** keyed `macos-*` `@active` (the well active, MAC-N05); state flags used: hover, pressed (C-8, W2-42†), focus_visible, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ColorPickerButton's CommonStates, W2-42†);
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `chevron` (`adwaita`, `kde-breeze`, `ios-*`), `expand_button` (all but `macos-*`),
  `swatch_ring` (`kde-breeze`, `windows-11`), `checkerboard` (presets whose button draws none: value phase;
  W2-42† draws one); values: `swatch_gap_px` (all but `kde-breeze`); fields: `swatch.stroke` (`kde-breeze`: K-KA-14†
  draws no border).
- **State-model source:** GNOME `_buttons.scss` (the `button` node with class `.color`); KDE Breeze push-button
  branches, kirigami-addons `FormColorDelegate.qml`; macOS `NSColorWell.Style`; Windows ColorPickerButton states;
  iOS `UIColorWell` and `UIControl.State`.
- **Today's fields:** none.
- **Notes:** kirigami-addons' colour row puts this widget in `context form.row` (the form's `@ka_form_card`, K-KA-14†).
  Windows' button opens a `color_picker` in a flyout (W2-42†); iOS's well opens the system colour sheet (C-8).

### color_picker

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkColorChooserWidget editor (GN-2.53†) and its swatches (GN-2.27), KDE's
  QColorDialog through the KDE platform theme and KColorCombo (K2.25†), Windows' ColorPicker (W2-05).
- **Available:** every preset but `macos-*`, `ios-*`, `material-3` (`available = false`: no colour picker the
  toolkit draws in their rows); `kde-breeze` flavour `widgets` only (QColorDialog, KColorCombo).
- **Parts:**
  - `body` — the picker (the root): `padding`, `bindings`.
  - `plane` — the saturation and value field (Windows' box or ring spectrum): `stroke` (W2-05), `focus` (GNOME's
    plane focus ring, GN-2.53†), `cursor` (crosshair, ADD-A59), `min_width`, `min_height` (GN-2.53†).
  - `crosshair` *(element: focused against unfocused, GN-2.53†)* — `stroke` (GNOME's bands and lines, KDE's cross,
    Windows' selection ellipse), `min_width`, `min_height`.
  - `value_strip` — KDE's value strip (K2.25†): `stroke`.
  - `checkerboard` — under translucent colours: `fill`.
  - `preview` — Windows' previous and current colours, GNOME's sample: `corners`.
  - `swatch` *(element: the chooser's swatches, KDE's colour wells)* — `corners` (GNOME's per-edge radii are
    `swatch.corners` under `pos` rules, GN-2.27), `stroke` (KDE's well frame, K2.25†), `focus`. `swatch_ring` belongs
    to `swatch` and is queried with it.
  - `swatch_ring` — GNOME's drop-target and high-contrast ring: `stroke`.
- **Widget values:** `grid_gap_px: Value<Px>` (GN-2.53†).
- **Variants:** keyed `windows-11` `@box`, `@ring` (the spectrum's shape, W2-05); `kde-breeze` `@qt_dialog`
  (K2.25†).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: focused (on `crosshair`), focus_visible, drop_target (on `swatch`,
  GN-2.27), selected, hover, pressed, disabled.
- **Exclusive groups:** none (W2-05 states no visual-state groups for the spectrum; GNOME's and KDE's states
  combine); precedence: state models (S17).
- **may_be_absent:** parts: `value_strip` (`adwaita`, `windows-11`), `swatch` (`windows-11`), `swatch_ring`
  (`kde-breeze`, `windows-11`); fields: `plane.min_width`, `plane.min_height` (presets that state none).
- **State-model source:** GNOME `gtkcolorplane.c`, `gtkcolorscale.c`, `_color-chooser.scss`; KDE
  `qcolordialog.cpp` frames; Windows ColorPicker and ColorSpectrum states.
- **Today's fields:** none.
- **Notes:** the hue, alpha and value sliders are `slider` elements in `context color_picker` with the typed slider
  variants `hue`, `alpha`, `value` (GN-2.53†, W2-05); the eyedropper is a `button` (`circular` on GNOME, GN-2.53†)
  in `context color_picker`; hex and channel fields are
  `input` elements, KDE's channel spin boxes `stepper` elements; GNOME's numeric-entry popups are `popover`
  (`@osd`) in context (GN-2.53†).

### context_help

*Written in the check pass* (rename row 83).

- **Kind:** typed widget: a help pop-up shown after the user enters help mode and clicks an element. KDE's What's
  This popup (QWhatsThis: Shift+F1, the title-bar help button; K2.31†) and macOS's context-sensitive help window
  (`NSHelpManager`, MAC-P4-11†).
- **Available:** every preset but `adwaita`, `windows-11`, `ios-*`, `material-3` (`available = false`: no
  context-help window in their rows; Windows' help button is Win32, R1.3); `kde-breeze` flavour `widgets` only
  (QWhatsThis).
- **Parts:**
  - `body` — the popup: `fill` (KDE's `ToolTipBase`), `stroke` (KDE's frame in the tooltip text colour), `shadow`,
    `corners`, `padding` (KDE's text inset, macOS's margins), `bindings` (KDE's tooltip roles, K2.31†).
  - `inner_frame` — KDE's second frame line inside the first, in the shade role `Dark` (K3.53, K2.31†): `stroke`.
  - `text` — the help text (font and colour; KDE's rich text): `text`.
- **Widget values:** `placement: Placement` (`QtWhatsThis`: by the pointer, K2.31†; `AppKitContextHelp`: at or near
  the given point, MAC-P4-11†, ADD-A21); `mode_cursor: Value<CursorShape>` (the pointer while help mode is active:
  `WhatsThis`, ADD-A59; MAC-P4-11†'s `isContextHelpModeActive`).
- **Variants:** none.
- **Sizes and densities:** none.
- **Flags:** none (the popup has no interaction states).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `inner_frame` (`macos-*`); fields: `body.shadow` (`kde-breeze`: Qt draws its hatched
  shadow only when the platform's `DropShadow` hint is true, which the KDE platform theme does not set, K2.31†;
  Open items); values: `mode_cursor` (`macos-*`; Open items).
- **State-model source:** KDE `qwhatsthis.cpp`; macOS AppKit appearances (light, dark), no control states.
- **Today's fields:** none (`tooltip.*` is the nearest, not the same window).
- **Notes:** included under R1.1 as a toolkit's own equivalent of a tooltip (the application sets only the text;
  a toolkit that draws its own windows draws this one). It holds
  macOS's context-help window and K2.31†, which `teaching_tip` no longer lists (rename row 83). Help mode is entered
  by `button`
  variant `help` (MAC-N10) or the caption button `@help` (KDE).

### date_picker

*From group 6's catalogue.*

- **Kind:** typed widget: macOS's `NSDatePicker` (MAC-N04), Windows' CalendarDatePicker (W2-03) and DatePicker's
  date half (W2-04), KDE's KDateComboBox and QDateEdit (K2.23†) and kirigami-addons' date row and popup
  (K-KA-20†, K-KA-22†), Material's date picker (MAT-N16), iOS's compact picker (C-5). The month grid it opens is a
  `calendar` element (its ancestor is the date picker).
- **Available:** every preset but `adwaita` (`available = false`: no date picker in GNOME's rows).
- **Parts:**
  - `body` — the picker's box (the root), holding the trigger, or the inline calendar and wheels: `padding`,
    `bindings`.
  - `field` — the button, pill or combo showing the date (the picker's own trigger; absent under `inline`, which has
    none): `fill` (focused fill, W2-03; iOS's pill, C-5), `stroke` (W2-04), `corners` (iOS's pill, C-5), `padding`,
    `min_width`, `max_width` (W2-04).
  - `field_label`, in `field` — the date text (accent on iOS, C-5): `text`.
  - `placeholder`, in `field` — the text without a date: `text`.
  - `glyph`, in `field` — Windows' calendar glyph (W2-03): `glyph` (per state).
  - `segment` *(element: a day, month or year part of a text-field date, selected while edited)* — `fill`, `text`
    (W2-04, MAC-N04).
  - `segment_separator` — Windows' spacer lines, kirigami-addons' separator between the halves (K-KA-22†): `stroke`,
    `opacity`, `padding` (its inset, K-KA-22†).
  - `popup` — the picker's own flyout, popover or docked container, holding the calendar or the looping selectors
    (one name with `time_picker.popup`): `fill`, `stroke`, `corners` (MAT-N16), `shadow`, `padding`, `min_width`,
    `min_height` (MAT-N16). Where the platform's popup is another widget's element (KDE's KDateComboBox opens a
    `menu`, K2.23†; kirigami-addons' popup is a `dialog`, K-KA-20†), that element draws the box in `context
    date_picker` and `popup` is absent by rule, so one box is drawn.
  - `header` — the control's header label (W2-03) and Material's header: `text`.
  - `headline`, `supporting_text` — Material's: `text`.
  - `divider` — Material's: `stroke`.
- **Widget values:** `placement: Placement` (the popup against the field, its gap the placement's `gap`: W2-03's
  flyout, K2.23†'s menu, C-5's popover, MAC-N04's calendar overlay; strategies: value phase); `presentations:
  Vec<PresentationTransition>`: events `@popup_show`, `@popup_hide` with the step's part `popup` (the popup's own
  occasion, the picker staying: W2-04's open animation, K-KA-20†'s fade).
- **Variants:** typed `text_field` (a segmented text-field date: macOS textField and KDE QDateEdit, which open no
  popup; Windows DatePicker, whose field opens its looping-selector flyout, W2-04),
  `with_popup` (Windows CalendarDatePicker, KDE KDateComboBox, iOS compact, Material docked), `inline` (macOS
  clockAndCalendar, iOS inline); keyed `macos-*` `@text_field_and_stepper` (MAC-N04), `material-3` `@modal`,
  `@input` (MAT-N16), `kde-breeze` `@ka_date_popup` (K-KA-20†, K-KA-22†).
- **Sizes and densities:** `macos-*`: `NSControl.ControlSize` as AppKit draws it per style (MAC-N04).
- **Flags:** shared `empty` (Windows: no date chosen, the placeholder against the selected text colours, W2-03);
  state flags used: hover, pressed, focused, disabled (W2-03, W2-04), selected (on `segment`, MAC-N04), read_only
  (K-KA-22†), open.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CommonStates); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `glyph` (all but `windows-11`), `segment`, `segment_separator` (`ios-*`,
  `material-3`), `headline`, `supporting_text`, `divider` (all but `material-3`), `header` (all but `windows-11`,
  `material-3`), `field`, `field_label`, `placeholder` (every preset that declares `inline`: absent by a rule
  under it), `popup` (by a rule under `inline`, and under `text_field` on `macos-*` and `kde-breeze`, which open
  none; on `kde-breeze` under `with_popup` and `@ka_date_popup`, where a `menu` or `dialog` element draws the box);
  fields: `body.padding`, `popup.stroke`.
- **State-model source:** macOS `NSDatePicker.Style` × control states; Windows CalendarDatePicker and DatePicker
  visual state groups; KDE Breeze `CC_ComboBox` and menu branches (`kdatecombobox.cpp`), kirigami-addons
  `FormDateTimeDelegate.qml`, `DatePopup.qml`; Material date-picker token files; iOS `UIDatePickerStyle`.
- **Today's fields:** none (W2-03 and C-5 point at `combo_box.*`, which keeps its own).
- **Notes:** MAC-N04's calendar overlay (`presentsCalendarOverlay`) is a `calendar` in `context date_picker.popup`,
  placed by this widget's `placement`. W2-04's looping selectors are `picker_wheel` elements in `context
  date_picker.popup` (its band `picker_wheel.selection_band`, its item height `picker_wheel.item.min_height`); macOS's
  stepper is a `stepper` in context (MAC-N04); K2.23†'s popup is a `menu` element in `context date_picker`, which
  draws the box, holding the `calendar` and the word items (Today, Next Week …) as `menu.item` elements.
  kirigami-addons' date and time row (K-KA-22†) puts the field, drawn flat, in `context form.row` (the form's
  `@ka_form_card`); its popup (K-KA-20†) is a `dialog` element in `context date_picker` (`dialog @ variant @ka_dialog
  context date_picker`: its body, scrim and action area), holding the `calendar`.

### dial

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's QDial and QQC2 Dial (K2.5, Q2.2), macOS's circular slider (MAC-N15).
- **Available:** every preset but `adwaita`, `windows-11`, `ios-*`, `material-3` (`available = false`: no dial in
  their rows).
- **Parts:**
  - `body` — the dial's box (the root): `padding`, `bindings`.
  - `groove` — the ring (K2.5): `stroke`.
  - `value_arc` — the value (KDE's highlight arc): `stroke`.
  - `handle` *(element)* — the slider handle with its shadow (K2.5): `fill`, `stroke`, `corners`, `shadow`,
    `min_width`, `min_height`, `focus`.
  - `knob_dot` — macOS's knob dot (MAC-N15): `fill`, `min_width`, `min_height`.
  - `tick` — macOS's ticks: `fill`, `min_width`, `min_height`.
- **Widget values:** `thickness_px: Value<Px>` (K2.5); `start_angle_deg: Value<Deg>`, `end_angle_deg: Value<Deg>`;
  `default_size_px: Value<Px>` (Q2.2).
- **Variants:** keyed `macos-*` `@circular_slider` (the slider type that draws it, MAC-N15).
- **Sizes and densities:** `macos-*`: the control sizes (MAC-N15).
- **Flags:** none of its own; state flags used: hover (on `handle`, K2.5), pressed, focus_visible, disabled.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `value_arc` (`macos-*`), `knob_dot`, `tick` (`kde-breeze`); fields: `handle.shadow`
  (`macos-*`); values: `default_size_px` (`macos-*`).
- **State-model source:** KDE Breeze `CC_Dial` branches (`st:8144-8217`) and the QML Dial; macOS `NSSlider`
  circular type × control states.
- **Today's fields:** none.
- **Notes:** none (the rows place no other widget in a dial's context).

### drag_image

*From group 7's catalogue.*

- **Kind:** typed widget: what a drag shows: macOS (MAC-X21), iOS (C-24, P3-10†), Windows (W3-49†, W1-110), KDE
  (K-RL06†), GNOME (GN-2.49), Material (MAT-P4-8†: the shadow is the application's image; the window manager
  animates the outcome).
- **Available:** every preset.
- **Parts:**
  - `body` — the drag feedback's box (the root), holding the image and, beside it, Windows' caption: `padding`.
  - `image` — the dragged image's window: `fill` (W1-110's drag fill), `text`, `icon` (GNOME's window
    foreground as icon and text fill, GN-2.49), `opacity` (translucency: MAC-X21, W3-49†, K-RL06†, W1-110), `shadow`
    (the drag image's shadow, MAC-X01),
    `bindings` (GNOME's window colours, GN-2.49).
  - `lift_preview` — iOS's lift (C-24): `transform` (scale), `shadow`, `corners`.
  - `caption` — Windows' `DragUIOverride` caption (W3-49†): `text`.
  - `glyph` — Windows' drop glyph (copy, move, link; W3-49†): `glyph`.
  - `drop_badge` — the drop-operation feedback per operation (P3-10†): `fill`, `glyph`.
  - `drop_description` — Windows' shell drop-description label (W3-49†): `text`.
- **Widget values:** `lift_delay_ms: Value<Ms>` (C-24); `hotspot: Value<DragHotspot>`, `DragHotspot { PressPoint
  }` (where the pointer holds the image: at the press point, K-RL06†, the one hotspot the rows state; a further one a
  fact states is added as a member); `presentations: Vec<PresentationTransition>`: events `@lift`, `@drop`, `@cancel`
  (C-24; MAT-P4-8†'s cancelled drag shrinks and fades), `@drop_not_consumed` (MAT-P4-8†: the shadow flies back
  while fading).
- **Variants:** none (the rows state none).
- **Sizes and densities:** none.
- **Flags:** keyed `material-3` `@source_hidden` (the drag hid its source task: both outcomes only fade in place,
  MAT-P4-8†); `ios-*` `@drop_copy`, `@drop_move`, `@drop_forbidden`, `@drop_cancel` (the session's current
  `UIDropOperation`, which selects the `drop_badge`; the connector sets one at a time, P3-10†); state flags used: none
  on the image itself; the source element takes `dragged` (K-RL06†, W1-110) and
  the target `drop_target` (MAC-X21, C-24, W3-49†).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `lift_preview` (all but `ios-*`), `caption`, `glyph`, `drop_description` (all but
  `windows-11`), `drop_badge` (all but `ios-*`;
  Material draws none, MAT-P4-8†); fields: `image.fill` (all but `windows-11`, W1-110), `image.shadow` (presets
  that draw none); values: `lift_delay_ms` (all
  but `ios-*`), `hotspot` (presets that state none).
- **State-model source:** UIKit `UIDragInteraction`, `UIDragPreviewParameters`, `UIDropOperation` (iOS); AppKit
  `NSDraggingSession` (macOS); WinUI `DragUIOverride` and the shell's `DROPDESCRIPTION` (Windows);
  `qabstractitemview.cpp` (KDE); libadwaita's `dnd` node (GNOME); Android `DragState` (Material).
- **Today's fields:** none.
- **Notes:** owned by the cross-cutting group (MAC-X21, W3-49†) and the list group (K-RL06†). Group 8's
  `drag_preview` is this widget (rename row 13: its `body` is `image`). A list's drag image (K-RL06†, W1-110) is
  `drag_image.image.opacity` and `drag_image.values.hotspot` in `context list`; the drop target's highlight is
  `role.global.drop_highlight`;
  the drag cursors per operation are `token.pointer.drag_cursors` (P3-10†). The drag-count badge is
  `badge @ context drag_image` (MAC-X21, C-24; Windows' multi-drag count, W1-110: one badge, rename row 100).

### edit_menu

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's edit menu (C-14), KDE's touch text actions (Q2.53†), Android's floating text
  toolbar (MAT-N30†), Windows' CommandBarFlyout (W2-23), GNOME's touch-selection bubble (GN-1.286†, GN-1.184).
- **Available:** every preset but `macos-*` (`available = false`: no edit menu in its rows); `kde-breeze` flavour
  `kirigami` only (Kirigami's touch text actions).
- **Parts:**
  - `body` — the root: `fill` (colour or material), `stroke`, `corners` (joined to the secondary menu, W2-23),
    `shadow`, `padding` (GNOME's content margin, GN-1.286†).
  - `divider` — the line between commands: `stroke`.
  - `arrow` — towards the selection (C-14): `fill`, `min_width`, `min_height`.
  - `primary_commands` — the row of commands (Windows', W2-23): `padding`.
- **Widget values:** `placement: Placement` (above the selection start, below its end without room:
  `UiKitEditMenu` C-14, `KirigamiTextActions` Q2.53†, `AndroidTextToolbar` MAT-N30†, `GtkPopover` with
  `anchor_grow` GN-1.286†, `WinUiFlyout` W2-23); `gap_to_selection_px: Value<Px>` (Q2.53†); `presentations:
  Vec<PresentationTransition>`: events `Appear`, `Disappear` (the menu shown and dismissed), `@expand`, `@collapse`
  (W2-23).
- **Variants:** typed `vertical` (the commands stacked: iOS's pointer style, C-14; Android's overflow list, MAT-N30†;
  the horizontal bar is the base, R25.14); keyed `ios-*` `@compact` (C-14).
- **Sizes and densities:** none.
- **Flags:** keyed `windows-11` `@expanded` (the secondary menu shown: the joined corners, W2-23); state flags used:
  none other (its commands are `button` elements with their own states).
- **Exclusive groups:** none (the commands are `button` elements, whose groups, the CommonStates on `windows-11`, are
  `button`'s; Android's toolbar is the system's, not M3 tokens, MAT-N30†); precedence: state models (S17).
- **may_be_absent:** parts: `arrow` (`kde-breeze`, `material-3`, `windows-11`), `primary_commands` (all but
  `windows-11`), `divider` (`windows-11`, `material-3`: no line between commands). `presentations` is a list, empty
  where a preset states none.
- **State-model source:** iOS `UIEditMenuInteraction`; KDE `MobileTextActionsToolBarImpl.qml` (ToolButton
  states); Android floating `ActionMode` (measured); Windows `CommandBarFlyout` visual states; GNOME `gtktext.c`
  and `_popovers.scss`.
- **Today's fields:** none.
- **Notes:** Nested controls: the commands are `button` elements in `context edit_menu` on every platform (iOS's
  items, C-14; GNOME's bubble buttons in a `layout` `@linked`, GN-1.286†, GN-1.184; KDE's ToolButtons, `button`
  variant `tool`, Q2.53†; Android's floating toolbar items, MAT-N30†; Windows' AppBarButtons, `tool`, W2-23; KDE's
  icon-only, Q2.53†); iOS's paging chevrons `button @ variant previous / next context edit_menu` (C-14); Windows'
  more button `button @ variant overflow context edit_menu`, its secondary menu a `menu` in `context edit_menu`
  (W2-23). Windows' joined corners are rules on `body` and `menu.body @ context edit_menu {@expanded}` (W2-23).

### file_chooser

*From group 6's catalogue.*

- **Kind:** typed widget: GTK's own file chooser (GN-2.48, GN-P3-22†, GN-P3-23†) and KDE's KFileWidget (K2.26†).
  Its places list is a `sidebar`, its path bar a `breadcrumb`, its file view a `list`, its buttons `button`s, all
  in its context.
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: their
  choosers are the system's, out by R1.1, as are GNOME's portal chooser and Windows' Common Item Dialog);
  `kde-breeze` flavour `widgets` only (KFileWidget).
- **Parts:**
  - `body` — the chooser (the root): `padding`, `bindings`
  - `frame` — GNOME's line below the chooser, outside its box (GN-P3-23†): `stroke`
  - `preview_pane` — KDE's preview (K2.26†): `fill`
- **Widget values:** `gap_px: Value<Px>`; `icon_size_slider: Value<bool>` (KDE: a `slider` in context, K2.26†).
- **Variants:** none (its rows' variants are the `sidebar`'s and `list`'s in context).
- **Sizes and densities:** none.
- **Flags:** none (its elements are other widgets', each with its own states).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `preview_pane` (`adwaita`), `frame` (`kde-breeze`); values: `icon_size_slider`
  (`adwaita`).
- **State-model source:** GNOME `_file-chooser.scss`, `gtkplacessidebar.c`, `gtksidebarrow.ui`; KDE
  `kfilewidget.cpp` and Breeze.
- **Today's fields:** none.
- **Notes:** rules in context: the view-toggle and new-folder buttons flat (GNOME, `button` in `context any
  file_chooser`, GN-P3-23†); the places rows and drop placeholder (`list @ context any file_chooser` with `list`'s
  keyed `@new_bookmark`, `@drop_placeholder`, inside the `sidebar`) and the eject button (`button` in that context,
  GN-P3-22†); list and grid cells (`list` in context, GN-P3-23†); the path bar a
  `breadcrumb` (GN-2.52†, K2.27†).

### find_bar

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's `UIFindInteraction` (C-25), KDE's KTextEditor search bar (K2.30†), macOS's
  `NSTextFinder` bar (MAC-N18). Its pattern field is an `input` in context.
- **Available:** every preset but `adwaita`, `windows-11`, `material-3` (`available = false`: no find bar in their
  rows; GNOME's GtkSearchBar is `search_field`, as K2.30†'s cross-check places it); `kde-breeze` flavour `widgets`
  only (KTextEditor's search bar).
- **Parts:**
  - `bar` — the bar (the root): `fill`, `padding`
  - `result_count` — the match count (C-25): `text`
  - `status_label` — KDE's "Not found" in the negative text role (K2.30†): `text`
- **Widget values:** `bar_position: Value<FindBarPosition>`, `FindBarPosition { AboveKeyboard, Top, Bottom }` (iOS
  above the keyboard, C-25; the top or bottom of the scroll view, MAC-N18's `findBarPosition`).
- **Variants:** none (the rows state none).
- **Sizes and densities:** none.
- **Flags:** keyed `kde-breeze` `@match`, `@no_match`, `@warning` (the search result, set by the connector: a match,
  also after wrapping; none; a neutral result, KColorScheme's `NeutralBackground`, the meaning typed `warning` has
  elsewhere: the field's background and the status text follow it, K2.30†); state flags used: none other (its
  buttons are `button` elements).
- **Exclusive groups:** none (states combine; the connector sets one search-result flag at a time); precedence:
  state models (S17).
- **may_be_absent:** parts: `result_count` (`kde-breeze`), `status_label` (`ios-*`, `macos-*`); values:
  `bar_position` (`kde-breeze`).
- **State-model source:** iOS `UIFindInteraction`; KDE `katesearchbar.cpp`; macOS `NSTextFinder`.
- **Today's fields:** none.
- **Notes:** Nested controls: the pattern field is an `input` in `context find_bar`, KDE's result colours its rules
  under the bar's flags (`input @ context find_bar {@match}` and so on, K2.30†); the previous and next buttons are
  `button @ variant previous / next context find_bar`, iOS's options button `button @ variant menu context
  find_bar` (C-25), macOS's Done `button @ variant done context
  find_bar` (MAC-N18). The highlights in the text are the searched view's `find_match` elements: `text_area.find_match`
  (C-25, MAC-N18; the current match `{selected}`), `code_editor.find_match` on `kde-breeze` (K-SF08†), with the global
  `role.global.find_highlight`.

### font_chooser

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkFontDialogButton (GN-2.28) and GtkFontChooserWidget (GN-P3-20†), KDE's
  KFontRequester and KFontChooser (K2.28†).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: no font
  chooser the toolkit draws; Win32's `ChooseFont` is out, R1.1; macOS's font panel is MAC-N25's row); `kde-breeze`
  flavour `widgets` only (KFontRequester, KFontChooser).
- **Parts:**
  - `body` — the chooser (the root): `padding`, `bindings`
  - `requester` *(element: GNOME's font dialog button, KDE's KFontRequester label and button)* — GNOME's `.font`
    button node
    (GN-2.28), KDE's sunken styled panel with its frame forced (K2.28†), with their states and focus indicator:
    `fill`, `stroke`, `corners`, `padding`, `focus`
  - `requester_label` — the font's name (GNOME's inner label in its own text style; KDE's in that font): `text`
  - `requester_separator` — GNOME's hidden separator (GN-2.28): `stroke` (the line, its start edge)
  - `family_row_label` — a family row's name, drawn in its own face (GN-P3-20†): `text`
  - `preview` — the preview text (K2.28†, GN-P3-20†): `text`
  - `filter_title` — the "Filter by" title (GN-P3-20†): `text`
- **Widget values:** `grid_gap_px: Value<Px>` (GN-P3-20†); `row_margins: Sides<Value<Px>>` (GN-P3-20†);
  `preview_min_lines: Value<u32>` (K2.28†); `requester_gap_px: Value<Px>` (the spacing between the requester's
  children, GtkFontDialogButton's, GN-2.28).
- **Variants:** keyed `kde-breeze` `@sunken_label` (the requester's styled panel, K2.28†).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed, focus_visible, disabled (on `requester`).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `requester_separator` (`kde-breeze`; GNOME hides it), `filter_title` (`kde-breeze`);
  values: `preview_min_lines` (`adwaita`), `row_margins` (`kde-breeze`).
- **State-model source:** GNOME `_buttons.scss` (the `button` node with class `.font`), `gtkfontchooserwidget.ui`;
  KDE `kfontrequester.cpp`, `kfontchooser.cpp`.
- **Today's fields:** none.
- **Notes:** the family, style and size lists are `list` elements, the size controls `slider` and `stepper`, the
  search entry `search_field`, the filters button a `button` (variant `menu`) with a `popover`, all in `context
  font_chooser` (GN-P3-20†, K2.28†).

### form

*From group 6's catalogue.*

- **Kind:** typed widget: preference pages, groups and their rows: GNOME's preference rows, groups and pages
  (GN-2.01–GN-2.10), KDE's FormLayout (Q2.45–Q2.47), Kirigami Forms (Q2.55†, Q2.56†) and kirigami-addons' form
  cards (K-KA-1†–K-KA-9†, K-KA-15†), macOS's SwiftUI grouped form (MAC-N21), Windows' Toolkit SettingsCard and
  SettingsExpander (W2-40†) and the clickable card (W1-139). GNOME's preference rows are this widget's in one place:
  an AdwPreferencesGroup's boxed list is `card` and its rows `row`, which state the boxed list's frame, row hover and
  pressed fills, outer corners, focus ring and separators (GN-1.165) on these parts; a plain `.boxed-list` that is not
  a preferences group is `list @ variant inset`, which states the same source on its own parts (R20: each widget its
  own parts). A row's switch, entry, combo and buttons are those widgets' elements in context.
- **Available:** every preset but `ios-*`, `material-3` (`available = false`: no form in their rows; MAC-N21 is the
  macOS preset's own release); `kde-breeze` flavour `kirigami` only (Kirigami's FormLayout and Forms, kirigami-addons'
  form cards; Qt Widgets' form layout is `layout` variant `form`, K1.210).
- **Parts:**
  - `page` — the form's page (the root; kirigami-addons' page fill, K-KA-6†): `fill`, `padding` (the page's margins,
    GN-2.10), `bindings`
  - `group_title` — a group's title (Q2.55†; kirigami-addons' title in the View set with its own insets, K-KA-5†;
    GNOME's single-line header height, GN-2.10): `text`, `padding`, `min_height`, `bindings`
  - `group_description` — the explanatory text between groups (kirigami-addons' `FormSectionText`, K-KA-5†): `text`,
    `max_width` (`Auto` where unbounded), `margin` (its margins above, below and at the sides, K-KA-5†)
  - `section_title` — a section title (Q2.47): `text`
  - `field_label` — a field's label (Q2.47; kirigami-addons' label above the field, K-KA-9†): `text`, `padding`
    (K-KA-22†)
  - `card` *(element)* — the boxed group's background, one element per card, hovered and pressed on its own
    (GNOME's preferences group boxed list, GN-1.165; Kirigami's cards, kirigami-addons' card with its border and
    shadow in the View set, Q1.129, K-KA-1†, SwiftUI's section card, Windows' settings card: fill, stroke, padding
    and min sizes per state, W2-40†, W1-139; the outer corners rounded by position rules): `fill`, `stroke`,
    `corners`, `shadow`, `padding`, `min_width`, `min_height`, `bindings`. The rows and row separators of a card nest
    in it and are queried in its scope.
  - `row` *(element)*, in `card` (in `page` where the form draws no card) — a row (GN-2.01, GN-2.08; Q2.55†,
    K-KA-3†; the expander items' top line, W2-40†): `fill`,
    `stroke`, `corners`, `padding`, `min_width`, `min_height`, `focus`, `bindings` (AdwEntryRow's `.error`, `.warning`
    and `.success` rebinding the accent to the status colour, GN-2.03), `transitions` (the hover and pressed fills
    and their fade, Q2.55†, K-KA-2†, GN-1.165; the first and last rows' outer corners; GNOME's focus ring, GN-2.03,
    GN-1.165). `row_title`, `row_subtitle`, `row_prefix`, `row_suffix`, `row_icon`, `row_edit_icon`, `row_indicator`,
    `row_arrow`, `row_nested` belong to `row`, nest in it and are queried with it.
  - `row_title` — the row's title (wrapped, then elided, K-KA-3†; dimmed in property rows, GN-2.02): `text`,
    `opacity`, `transitions` (AdwEntryRow's title shrinking to a caption, GN-2.04)
  - `row_subtitle` — the row's subtitle, dimmed (GN-2.01, Q2.55†, K-KA-15†): `text`, `opacity`, `padding` (its inset,
    K-KA-3†, K-KA-22†)
  - `row_prefix`, `row_suffix` — the boxes of leading and trailing items (GN-2.01, K-KA-3†): `padding`, `icon` (the
    chosen icon an icon row shows, K-KA-14†)
  - `row_icon` — the row's icon (K-KA-3†; GNOME's icon margin and disabled icon, GN-2.01; Windows' header icon with
    its largest size and margin, a disabled bitmap icon dimmed, W2-40†): `icon`, `padding`, `max_width`,
    `max_height`, `opacity`
  - `row_edit_icon` — the edit icon in its box (AdwEntryRow, GN-2.03; kirigami-addons' mobile rows, K-KA-9†):
    `icon`, `padding`, `min_width`, `min_height`
  - `row_indicator` — AdwEntryRow's indicator in its box, dimmed (GN-2.03): `icon`, `opacity`, `padding`,
    `min_width`, `min_height`
  - `row_arrow` — the trailing arrow (combo row, action entry, expander, GN-2.06, Q2.56†, K-KA-7†; Windows' action
    icon with its margin, W2-40†; the expander row's arrow turning and recoloured while `open`, GN-1.257, GN-1.258):
    `icon`, `padding`, `opacity`, `transform`, `transitions`
  - `row_nested` — the box holding an expander row's nested rows (GNOME's `list.nested` fill, GN-1.260; Windows'
    SettingsExpander items, W2-40†): `fill`, `padding`
  - `row_separator` *(element)*, in `card` — the line between two rows, an element whose previous and next siblings
    are rows (GN-1.165; Kirigami's separator, dimmed, Q2.56†, K-KA-4†): `stroke` (the line, its top edge), `opacity`,
    `padding` (its insets, Q2.56†, K-KA-4†), `transitions` (Kirigami's separator hidden beside a hovered entry and
    its insets animating, Q2.56†, K-KA-4†)
- **Widget values:** `group_gap_px: Value<Px>`; `page_gap_px: Value<Px>`;
  `row_gap_px: Value<Px>`; `column_gap_px: Value<Px>` (GN-2.01, GN-2.10, Q2.45); `label_alignment:
  Axes<Value<Alignment>>` (the label against its field, horizontally and vertically, Q2.45, Q2.46, Q2.47; restated
  under `collapsed`); `narrow_max_width_px: Value<Px>` (Q2.45, Q2.56†); `collapse: Value<FormCollapse>`,
  `FormCollapse { WhenWideDoesNotFit, AtBreakpoints }` (how the connector sets `collapsed`: KDE when the wide layout
  does not fit, Q2.45, Q2.56†, a measurement of the content with no threshold length, so it is no `Breakpoint`;
  Windows' wrap thresholds, W2-40†, and GNOME's preferences-dialog thresholds, GN-2.10, by `breakpoints`);
  `breakpoints: Vec<Breakpoint>` (Shared value types: the collapse thresholds setting
  `collapsed`, W2-40†, GN-2.10; kirigami-addons' card width limit setting `@width_limited` at or above it, K-KA-1†);
  `icon_text_gap_px: Value<Px>` (K-KA-3†); `leading_gap_px: Value<Px>`,
  `trailing_gap_px: Value<Px>` (K-KA-3†, K-KA-8†); `arrow_gap_px: Value<Px>`
  (K-KA-7†); `field_label_gap_px: Value<Px>` (K-KA-9†); `grid_columns: Value<u32>`,
  `grid_min_cell_width_px: Value<Px>`, `grid_gap_px: Value<Px>` (K-KA-15†);
  `presentations: Vec<PresentationTransition>` (queried with the row): events `@expand`, `@collapse` (an expander
  row's height and opacity, K-KA-7†).
- **Variants:** typed row kinds `action`, `entry`, `combo`, `switch`, `button`, `expander`, `spin`, `property`
  (GN-2.01–GN-2.10, Q2.56†, K-KA-3†); typed `primary`, `destructive` (button rows, GN-2.08); typed `warning`,
  `error`, `success` (GNOME's entry rows' `.warning`, `.error`, `.success`; KDE's neutral highlight is `warning`);
  keyed `adwaita` `@monospace` (property and entry rows); typed `flat` (Kirigami Forms' flat style, no card: Q2.56†;
  the meaning `button`'s `flat` shares); typed `collapsed` (the narrow, stacked layout: KDE, Q2.45, Q2.56†; Windows'
  wrapped layouts, W2-40†; GNOME's narrow preferences, GN-2.10; set by the connector as `collapse` says, R16); keyed
  `kde-breeze` `@cards` (Kirigami Forms' card style, Q2.55†), `@ka_form_card` (on the form: a
  kirigami-addons FormCard page, whose rows and nested controls match `context form variant @ka_form_card`,
  K-KA-1†–K-KA-9†), `@width_limited` (set by `breakpoints`, R16; K-KA-1†), `@ka_info_card`
  (K-KA-15†), `@date_time` (K-KA-22†), `@icon_row` (K-KA-14†), `@check`, `@radio` (K-KA-3†, K-KA-8†), `@link`
  (K-KA-7†); `windows-11` `@right`, `@right_wrapped`, `@right_wrapped_no_icon`, `@left` (the content
  layouts, W2-40†), and its vertical layout the typed `vertical` (orientation); the content variant `bitmap_icon`
  (on `row`: its icon is a bitmap, which WinUI dims when disabled where it recolours a glyph icon, W2-40†, W3-51†;
  set by the connector from the content, ADD-A61).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** shared `empty` (GNOME: AdwEntryRow's title not shrunk while the row is empty, GN-2.04); keyed
  `kde-breeze` `@clickable` (Kirigami's hover and pressed fills only on clickable entries, Q2.55†); state flags
  used: hover, pressed, disabled (on `card` and `row`: W2-40†, W1-139, Q2.55†, K-KA-2†), focus_visible from K-KA-2†,
  focus_within (GN-2.04), open (expander rows, K-KA-7†). Position rules (`pos first`, `pos last`) on `row` round the
  card's outer corners; on the `row_separator` element, `prev form.row {hover}` and `next form.row {hover}` hide the separators
  beside a hovered neighbour (Q2.56†, K-KA-4†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (the settings card's CommonStates, W2-40†);
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `row_edit_icon` (all but `adwaita`, `kde-breeze`), `row_indicator` (all but
  `adwaita`), `field_label` (`adwaita`, `macos-*`), `row_nested` (every preset: absent by a rule outside
  `expander`), `row_arrow` (every preset: absent by a rule outside the row kinds that show it, `action`, `combo`, `expander`; `kde-breeze` under `@ka_form_card` while the row draws no
  background, K-KA-7†); fields: `group_description.margin` (all but `kde-breeze`), `row_suffix.icon` (outside
  `@icon_row`), `page.fill` (all but `kde-breeze`), `card.fill`, `card.stroke`, `card.shadow` (`kde-breeze` under
  `flat`: no card), `row.focus`
  (`kde-breeze` under `@ka_form_card`: K-KA-2† draws no focus ring), `row_separator.padding` (all but `kde-breeze`);
  values:
  `icon_text_gap_px`, `leading_gap_px`, `trailing_gap_px`, `arrow_gap_px`, `field_label_gap_px`,
  `grid_columns`, `grid_min_cell_width_px`, `grid_gap_px`, `narrow_max_width_px` (all but `kde-breeze`).
  `presentations` and `breakpoints` are lists: empty, not absent, where the platform has none.
- **State-model source:** GNOME `_lists.scss`, `_preferences.scss`, `adw-entry-row.c`; KDE `FormLayout.qml`,
  `forms/cards/*.qml`, `forms/flat/*.qml`, kirigami-addons `formcard/*.qml`; macOS SwiftUI `GroupedFormStyle`;
  Windows SettingsCard and SettingsExpander states.
- **Today's fields:** none (`list.*`, today's `expander` field `header_height` and `layout.*` stand-ins stay
  theirs).
- **Notes:** Windows' SettingsCard has one home, `form.card` (W2-40† and W1-139's clickable card), each card its
  own element (its rows' title colour per state is a rule `@ context form.card {hover}`); `card` variant
  `activatable` keeps GNOME's, Kirigami's and Material's activatable cards. kirigami-addons' rows hold `checkbox`,
  `radio`, `switch`, `combo_box`, `input`, `color_button`, `date_picker` elements in `context form.row`
  (`@ka_form_card`, K-KA-8†, K-KA-12†, K-KA-13†, K-KA-14†, K-KA-22†); their status messages are `banner`
  (`@ka_inline_message`, K-KA-10†). Group 6's `title_shrink` and `separator_hide` values are the `row_title` and
  `row_separator` transitions (GN-2.04, Q2.56†); `has_text` is `!{empty}`.

### glass_container

*From group 7's catalogue.*

- **Kind:** typed widget: iOS's `UIGlassContainerEffect` (C-20, X-MT-8, X-MT-9) and macOS's
  `NSGlassEffectContainerView` (MAC-X04): nearby glass elements merging into one. Owned jointly with group 8
  (MAC-X04).
- **Available:** `macos-tahoe`, `ios-26` only (every other preset `available = false`: Liquid Glass is new in 26).
- **Parts:**
  - `surface` — the root: a `Material` paint with identity `glass` (tint, X-MT-6; interactivity a material
    parameter), its corners (MAC-X04) and the press response's scale (X-MT-8): `fill`, `corners`, `transform`,
    `bindings`, `transitions` (the interactive glass's press response and the morph between merged elements,
    X-MT-8, X-MT-9), `shadow` (macOS 26's glass shadow, MAC-X01)
- **Widget values:** `merge_gap_px: Value<Px>` (the distance within which neighbouring glass elements merge: C-20,
  X-MT-9, MAC-X04).
- **Variants:** typed `clear` (C-20, X-MT-5, MAC-X04; the regular glass is the base).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed (the interactive glass's response, X-MT-8).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** none (the two presets that have it draw every part).
- **State-model source:** UIKit `UIGlassEffect`, `UIGlassContainerEffect` and AppKit `NSGlassEffectView`,
  `NSGlassEffectContainerView` style enums.
- **Today's fields:** none.
- **Notes:** group 8 had placed the merge spacing in `Material.params` (rename row 53); the press response and
  morph are `PartStyle.transitions` (X-MT-8, X-MT-9).

### group_box

*From group 6's catalogue.*

- **Kind:** typed widget: a frame with an optional title. GNOME's GtkFrame (GN-2.32), iOS's SwiftUI GroupBox
  (C-32), KDE's QGroupBox (K2.8, K1.211), framed panel and scroll area (K2.21) and QQC2 Frame and GroupBox (Q2.12,
  Q2.13), macOS's `NSBox` (MAC-BX01, MAC-BX02). An image view's frame or shadow style is `image`'s (MAC-N12, GN-3.07).
- **Available:** every preset but `windows-11`, `material-3` (`available = false`: no titled frame in their rows).
- **Parts:**
  - `body` — the frame (the root; KDE's frame background and its hover and focus outline, K2.8, K2.21; iOS's
    surface, C-32): `fill`, `stroke`, `corners`, `padding`
  - `title` — the title (GN-2.32, K2.8, Q2.13, MAC-BX01): `text`
  - `focus_line` — KDE's faded line under a focused title (K2.8): `stroke` (the line, its top edge), `transitions`
    (its fade, K2.8)
- **Widget values:** `title_margin_px: Value<Px>` (GN-2.32, K1.211, MAC-BX01's title-to-border gap);
  `title_position: Value<GroupBoxTitlePosition>`, `GroupBoxTitlePosition { None, AboveTop, AtTop, BelowTop,
  AboveBottom, AtBottom, BelowBottom }` (MAC-BX01; KDE's title above the content, Q2.13).
- **Variants:** typed `flat` (KDE's flat QGroupBox, K2.8; GNOME's `.flat`, GN-2.32); keyed `kde-breeze` `@box`,
  `@panel`, `@win_panel`, `@styled_panel` (K2.21), `@quick_frame` (Q2.12), `@quick_group_box` (Q2.13); `macos-*`
  `@primary`, `@custom`, `@separator`, `@transparent` (MAC-BX02).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, focused (K2.21's framed panel outlines), focus_visible
  (K2.8's focus line), disabled.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `focus_line` (all but `kde-breeze`); fields: `body.stroke` (`ios-*`), `body.fill`
  (`adwaita`, presets whose frame is transparent: Q2.12); values: `title_position` (`adwaita`, `ios-*`).
- **State-model source:** GNOME `_misc.scss` (`frame`); iOS SwiftUI `GroupBoxStyle`; KDE Breeze
  `PE_FrameGroupBox`, `CE_ShapedFrame` and frame-shadow branches, `Frame.qml`, `GroupBox.qml`; macOS `NSBox`.
- **Today's fields:** none (`card.*` stand-ins stay `card`'s).
- **Notes:** KDE's checkable title is a `checkbox` in `context any group_box` (K2.8). Group 6 named this widget
  `frame`; the merge keeps `group_box`, which does not clash with the `frame` parts of `window`, `scroll_view` and
  `calendar` (rename row 22).

### image

*Written in the third check* (rename row 118).

- **Kind:** typed widget: a picture or icon shown on its own with the platform's frame or shadow style: macOS's
  `NSImageView` frame styles (MAC-N12) and libadwaita's `.icon-dropshadow` and `.lowres-icon` on a standalone
  GtkImage or GtkPicture (GN-3.07). Two platforms draw the kind, so it is typed (R14): one home for both.
- **Available:** every preset but `kde-breeze`, `windows-11`, `ios-*`, `material-3` (`available = false`: no row
  states an image style).
- **Parts:**
  - `body` — the image view's box and frame (the root; NSImageView's photo, bezel, groove and button frames,
    MAC-N12): `fill`, `stroke`, `corners`, `shadow`, `padding`.
  - `picture` — the image or icon itself: `icon` (an icon's style), `shadow` (libadwaita's icon drop shadow and
    low-resolution icon shadow, GN-3.07), `corners`.
- **Widget values:** none (no row states one).
- **Variants:** keyed `adwaita` `@icon_dropshadow`, `@lowres_icon` (GN-3.07); `macos-*` `@photo`, `@gray_bezel`,
  `@groove`, `@button` (`NSImageFrameStyle`, MAC-N12; no frame is the base).
- **Sizes and densities:** none.
- **Flags:** none; state flags used: none (an image view has no interaction states in these rows).
- **Exclusive groups:** none (no states); precedence: state models (S17).
- **may_be_absent:** parts: none; fields: `body.fill`, `body.stroke`, `body.shadow` (`adwaita`; `macos-*` in the
  base, which draws no frame), `picture.shadow` (`macos-*`; `adwaita` outside its two variants); values: none.
- **State-model source:** macOS `NSImageFrameStyle`; libadwaita `_misc.scss` (`.icon-dropshadow`, `.lowres-icon`).
- **Today's fields:** none.
- **Notes:** the second check had placed MAC-N12's styles on `group_box` and GN-3.07's on a keyed component `image`
  (rename row 118).

### image_cropper

*From group 6's catalogue.*

- **Kind:** typed widget: macOS's ImageKit crop tool (MAC-P4-12†), Windows' Toolkit ImageCropper (W-RV-1†).
- **Available:** every preset but `adwaita`, `kde-breeze`, `ios-*`, `material-3` (`available = false`: no image
  cropper in their rows).
- **Parts:**
  - `frame` — the root (W-RV-1†): `fill`, `stroke`, `corners`, `padding`
  - `mask` — the shade outside the crop (W-RV-1†): `fill`
  - `outline` — the crop rectangle's outline (MAC-P4-12†): `stroke`
  - `overlay` — Windows' overlay path (W-RV-1†): `fill`
  - `thumb` *(element)* — a handle (W-RV-1†; fill per state, the system focus visual): `fill`, `stroke`,
    `corners`, `min_width`, `min_height`, `focus`
- **Widget values:** none (the frame's padding is `frame.padding`, W-RV-1†).
- **Variants:** typed `corner`, `edge` on `thumb` (Windows' primary and secondary thumb styles, W-RV-1†;
  ImageKit's handles, measured, MAC-P4-12†).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used on `thumb`: hover, pressed, disabled, focus_visible
  (W-RV-1†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (each thumb's CommonStates, W-RV-1†); others
  none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `overlay` (`macos-*` until its render: MAC-P4-12† names none); fields: `frame.padding`
  (`macos-*`).
- **State-model source:** ImageKit `IKImageView` crop tool (measured); Windows `ImageCropperThumb.xaml`.
- **Today's fields:** none.
- **Notes:** ImageKit's picture taker panel (MAC-P4-12†) is a `dialog` with the keyed `macos-*` variant
  `@picture_taker`: this cropper, its zoom `slider` and its `button`s are elements in `context dialog.body` there, and
  the panel's layout is the dialog's `padding` and values, measured from the panel's render.

### ink_toolbar

*From group 6's catalogue.*

- **Kind:** typed widget: Windows' InkToolbar (W2-36), iOS's PencilKit tool picker (C-40†).
- **Available:** every preset but `adwaita`, `kde-breeze`, `macos-*`, `material-3` (`available = false`: no ink
  toolbar; `PKToolPicker` does not show in Mac apps, C-40†).
- **Parts:**
  - `bar` — the toolbar (the root; C-40†): `fill`, `corners`, `shadow`, `padding`
  - `palette` — the pen colour flyout (W2-36): `fill`, `corners`, `shadow`
  - `palette_swatch` *(element)* — a pen colour (W2-36): `stroke`, `corners`
  - `stroke_width_selector` — (W2-36): `fill`
  - `ruler` — (W2-36): `fill`, `stroke`
- **Widget values:** `palette_placement: Placement` (the pen colour flyout, `WinUiFlyout`, W2-36).
- **Variants:** none (the rows state none).
- **Sizes and densities:** none.
- **Flags:** none of its own; its tools are `button` elements (below), the selected tool `{selected}` on both
  platforms (C-40†, W2-36: InkToolbar's checked tool maps to it, S7.2).
- **Exclusive groups:** none (W2-36 states no visual-state groups; iOS's states combine); precedence: state models
  (S17).
- **may_be_absent:** parts: none (the rows name each part for both platforms; Open items); fields: none; values:
  `palette_placement` (`ios-*`: the system lays out its palette).
- **State-model source:** Windows InkToolbar states; PencilKit `PKToolPicker` (measured).
- **Today's fields:** none.
- **Notes:** Nested controls: a tool is `button @ variant tool context ink_toolbar` (a toolbar's item is a button,
  rename row 70; its fill and icon per state, the selected tool `{selected}`, C-40†, W2-36). iOS's palette follows
  the scheme the application sets on it (`colorUserInterfaceStyle`): rules under `cond scheme:light` and
  `cond scheme:dark` (C-40†).

### keycap

*From group 7's catalogue.*

- **Kind:** typed widget, because two platforms draw a keycap: GNOME's shortcut label (GN-2.40) and KDE's
  `KCommandBar` keys (K-RL01†). Windows, macOS, iOS and Material draw shortcuts as text (cross-check, all 3). The
  input listed GN-2.40 as one platform's component; the KDE row, filed as "command bar", is the second.
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: they draw
  shortcuts as text); `kde-breeze` flavour `widgets` only (KCommandBar's keys; the Quick command bar shows text,
  K-KA-42†).
- **Parts:**
  - `label` — the container of keys and separators (the root; its focus indicator, GN-2.40): `padding`, `focus`
  - `key` — the cap (GN-2.40; KDE's key with its downward copy as its shadow, its height the font's line spacing,
    K-RL01†): `fill`, `stroke` (the inset line), `corners`, `shadow`, `padding`, `min_width`, `min_height`, `text`
  - `separator` — the text between keys ("+" or ", "; the row's text colour, K-RL01†): `text`
- **Widget values:** `key_gap_px: Value<Px>` (the label's spacing between caps).
- **Variants:** keyed `adwaita` `@legacy` (GTK's deprecated shortcut label: other padding, other focus offset). The
  separators taking the selected row's text colour is a context rule, `@ context list.row {selected}` (K-RL01†).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used on `label`: focused, focus_visible.
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `separator` (`adwaita`); fields: `label.focus` (`kde-breeze`).
- **State-model source:** libadwaita selectors by file (`_shortcuts-dialog.scss:34-57`,
  `_deprecated.scss:344-364`); KCommandBar's paint code (`kcommandbar.cpp:299-387`).
- **Today's fields:** none.
- **Notes:** KDE's Quick command bar shows shortcuts as plain text, not keycaps (K-KA-42†). GNOME's shortcuts
  dialog draws its shortcuts as this widget in `context shortcuts_dialog` (GN-2.40, GN-2.41).

### level_indicator

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkLevelBar (GN-2.26), macOS's `NSLevelIndicator` (MAC-N06) and SwiftUI `Gauge`
  (MAC-N22), iOS's `Gauge` (C-30), Windows' Toolkit RadialGauge (W2-41†).
- **Available:** every preset but `kde-breeze`, `material-3` (`available = false`: KDE's LCD number and LED are
  keyed components, W2-41†'s cross-check; Material has none; C-30 is the iOS preset's own release).
- **Parts:**
  - `body` — the gauge's box (the root), holding the track, the labels and the needle: `padding`,
    `opacity` (the disabled look, the whole control's opacity, W2-41†), `bindings`.
  - `track` — the empty trough; under `circular` the scale arc (iOS's track, C-30; RadialGauge's scale arc, W2-41†),
    one structure for every circular gauge (GNOME's high-contrast border, GN-2.26): `fill`, `stroke`, `corners`,
    `padding`
  - `fill`, in `track` — the value; under `circular` the value arc (iOS's fill, C-30; RadialGauge's trail arc,
    W2-41†) (GN-2.26, C-30): `fill`, `corners`
  - `block` *(element)* — a discrete segment, `filled` or empty, its tier by the threshold flags the connector sets
    on it (macOS's tiered capacity, each block coloured by its own tier, MAC-N06), its end corners by position among
    the blocks (`pos first`, `pos last`, GN-2.26): `fill`, `corners`
  - `tick` *(element)* — a tick mark (MAC-N06; RadialGauge's ticks, and its scale ticks under the keyed `@scale`,
    W2-41†): `fill`
  - `needle` — Windows' needle and its border, with its size (W2-41†): `fill`, `stroke`, `min_width`, `min_height`
  - `value_label`, `unit_label` — Windows' value and unit text (W2-41†): `text`
- **Widget values:** `block_gap_px: Value<Px>`, `block_min_length_px: Value<Px>` (GN-2.26); `thickness_px: Value<Px>`
  (C-30, W2-41†); `start_angle_deg: Value<Deg>`, `end_angle_deg: Value<Deg>` (circular, W2-41†); `end_cap:
  Value<LineCap>` (the track's and fill's end caps: iOS's end caps, C-30; RadialGauge's round arc caps, W2-41†;
  ADD-A34; one home for the end treatment);
  `thresholds: Vec<(Key, Value<f32>)>` (where low, high, full or warning, critical begin, stated by the platform's
  defaults:
  GtkLevelBar's offsets, `NSLevelIndicator`'s warning and critical values); `tick_spacing: Value<f32>` (RadialGauge's
  tick interval, in the gauge's value units, W2-41†).
- **Variants:** typed `continuous`, `discrete` (GNOME, macOS), `circular` (iOS, macOS SwiftUI, Windows' radial
  gauge; linear is the base), `vertical` (GNOME; macOS's orientation); keyed `macos-*` `@relevancy`, `@tiered` (the
  discrete
  capacity style drawing each block in its own tier's colour, `drawsTieredCapacityLevels`) (MAC-N06), `ios-*`
  `@accessory`, `@capacity` (C-30); `windows-11` on `tick` `@scale` (RadialGauge's scale ticks, W2-41†).
- **Sizes and densities:** `macos-*`: the control sizes (MAC-N06).
- **Flags:** shared `filled` (on `block`: a block the value reaches, GN-2.26, MAC-N06); keyed `adwaita` `@low`,
  `@high`, `@full` (GN-2.26); `macos-*` `@warning`, `@critical` (MAC-N06) (the connector sets the flag the value falls
  in by `thresholds`, and under `@tiered` the flag of each block's own position); state flags used: disabled
  (GN-2.26, W2-41†).
- **Exclusive groups:** none (the connector sets one threshold flag at a time; RadialGauge's Normal and Disabled
  are the single flag `disabled`); precedence: state models (S17).
- **may_be_absent:** parts: `needle` (all but `windows-11`), `unit_label`,
  `value_label` (`adwaita`, `macos-*`, `ios-*`: MAC-N22, C-30; Open items),
  `block` (`ios-*`, `windows-11`), `tick` (`adwaita`, `ios-*`); values: `block_gap_px`,
  `block_min_length_px` (`ios-*`, `windows-11`), `start_angle_deg`, `end_angle_deg` (`adwaita`), `end_cap`
  (`adwaita`, `macos-*`).
- **State-model source:** GNOME `_level-bar.scss`; macOS `NSLevelIndicator.Style`; iOS and macOS SwiftUI
  `GaugeStyle`; Windows `RadialGauge.xaml` states (Normal, Disabled).
- **Today's fields:** none (MAC-N06's stand-in, today's `progress_bar` field `fill_color`, stays `progress_bar`'s).
- **Notes:** MAC-N06's rating style (stars and placeholders) is the `rating` widget. iOS's `@capacity` is a gauge's
  capacity style; KDE's KCapacityBar is a bar, `progress_bar` `@capacity`: two kinds by R14's cross-check. GNOME's
  high-contrast trough is
  a rule under `cond contrast_high` (GN-2.26).

### media_controls

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkMediaControls in GtkVideo with its GtkScaleButton (GN-2.54†) and the OSD play
  image (GN-2.44), Windows' MediaTransportControls (W2-35), iOS's AVKit playback controls (C-42†), macOS's
  `AVPlayerView` controls (MAC-N30†), KDE's video controls in kirigami-addons' album viewer (K-KA-35†).
- **Available:** every preset but `material-3` (`available = false`: no such control). `kde-breeze` flavour
  `kirigami` only, and there only as kirigami-addons' `@ka_album` (`requires_variants = ["@ka_album"]`, K-KA-35†).
- **Parts:**
  - `bar` — the controls bar (the root; GNOME's `.osd` bar binds the Osd set, GN-2.54†; AVKit's bar material,
    C-42†, MAC-N30†; Windows' transport bar, W2-35; kirigami-addons' bar, K-KA-35†): `fill` (fill or material),
    `opacity`, `corners`, `stroke`, `shadow`, `bindings`
  - `time_label` — position and duration text in tabular figures (GN-2.54†; Windows' time text, W2-35): `text`
  - `play_overlay` — the image centred over the video (GNOME's OSD play image, GN-2.44; kirigami-addons' download
    button with a `large` icon, K-KA-35†): `fill`, `corners`, `min_width`, `min_height`, `icon`
- **Widget values:** `auto_hide_ms: Value<Ms>` (the bar hidden after the pointer stops: GN-2.54†, MAC-N30†, K-KA-35†;
  Windows' fade-out timing, W2-35); `volume_popup_hide_delay_ms: Value<Ms>` (K-KA-35†); `gap_px: Value<Px>` (the bar's
  box spacing, GN-2.54†); `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the controls
  shown and hidden: GNOME's crossfade
  reveal, GN-2.54†; Windows' fade-out, W2-35; AVKit's fade, C-42†).
- **Variants:** keyed `macos-*` `@inline`, `@floating`, `@minimal` (`AVPlayerViewControlsStyle`, MAC-N30†);
  `kde-breeze` `@ka_album` (K-KA-35†). The buttons' identities are `button`'s typed `play`, `volume`, `full_screen`,
  `frame_step`, `route` (GN-2.54†, W2-35, MAC-N30†, K-KA-35†, C-42†, W-P4-18†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (an active output route is the `route` button's `route_active`; the buttons are
  `button` elements with their own states).
- **Exclusive groups:** none (the buttons' CommonStates on `windows-11` are `button`'s); precedence: state models
  (S17).
- **may_be_absent:** parts: `play_overlay` (`windows-11`, `macos-*`, `ios-*`: no row draws one). values:
  `volume_popup_hide_delay_ms` (every preset but `kde-breeze`); `gap_px` (every preset but `adwaita`).
- **State-model source:** GNOME `gtkmediacontrols.ui`, `gtkvideo.c`, `gtkscalebutton.ui`; Windows
  MediaTransportControls visual states; iOS `AVPlayerViewController` and macOS `AVPlayerView` control styles (AVKit);
  KDE kirigami-addons `VideoMaximizeDelegate.qml`.
- **Today's fields:** none.
- **Notes:** Nested controls: the buttons are `button` elements in `context media_controls.bar`, their identities
  `play`, `volume`, `full_screen`, `frame_step` (GN-2.54†, W2-35, MAC-N30†, K-KA-35†); the volume button's popup is a
  `popover` in `context media_controls` (GNOME's `.scale-popup`, GN-2.54†; Windows' volume flyout, W2-35;
  kirigami-addons' popup, K-KA-35†) holding the volume scale, a `slider` variant `vertical`; the seek bar is a
  `slider` in `context media_controls`. The output-route button (iOS's AirPlay button, C-42†; Windows'
  cast button, W-P4-18†) is `button @ variant route context media_controls.bar`. Group 7 proposed `play_overlay`
  (GN-2.44) and the cast button (W-P4-18†, now `button`'s `route`, rename row 82); the kirigami-addons rows added
  `@ka_album` and the volume
  popup's delay
  (K-KA-35†).

### menu_bar

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkPopoverMenuBar (GN-2.30), KDE's QMenuBar (K2.1) and Qt Quick MenuBar (Q2.9,
  Q2.10), macOS's menu bar and status items (MAC-N20) with SwiftUI's `MenuBarExtra` window (MAC-N34†), Windows'
  MenuBar (W2-22), iPadOS's menu bar (C-37†).
- **Available:** every preset but `material-3` (`available = false`: no such control). iPadOS draws it from 26 only
  (C-37†), so the `ios-*` presets before `ios-26` declare it unavailable; `ios-26` declares it. On a phone the system
  shows no menu bar, so an application does not draw one there and nothing is stated for it (as content: S6.3).
- **Parts:**
  - `bar` — the bar (the root; GNOME's bottom line, GN-2.30; KDE's Header set in the tools area and its inactive
    group, K2.1, Q2.9; macOS's wallpaper-tinted translucent bar, transparent on 26, MAC-N20; Windows'
    `MenuBarBackground`, W2-22; iPadOS's fill or material, C-37†; the bar's height, MAC-N20, W2-22): `fill`,
    `stroke`, `padding`, `min_height`, `bindings`
  - `item` *(element: a top-level title)* — fill and stroke per state (GN-2.30, K2.1, W2-22): `fill`, `stroke`,
    `corners`, `padding`, `min_width`, `min_height` (Kirigami's item height, Q2.9), `transitions` (KDE's highlight
    fade, Q2.10). `item_label` belongs to `item` and is queried with it.
  - `item_label` — its text (KDE's highlighted text while hovered, Q2.10): `text`
  - `app_name` — macOS's application menu title in its own text style (MAC-N20): `text`
  - `status_item` *(element: a macOS status item)* — its icon size (MAC-N20): `icon`
  - `extra_window` — SwiftUI `MenuBarExtra`'s window (material, MAC-N34†): `fill`, `corners`, `shadow`
  - `focus_line` — KDE's weak-focus line (K2.1): `stroke` (the line, its bottom edge)
- **Widget values:** `item_gap_px: Value<Px>`; `status_item_gap_px: Value<Px>` (MAC-N20);
  `extra_window_gap_px: Value<Px>`
  (MAC-N34†); the bar's height is `bar.min_height` (macOS's, MAC-N20; `MenuBarHeight`, W2-22) and Kirigami's item
  height `item.min_height` (Q2.9); `presentations: Vec<PresentationTransition>`:
  event `Appear` (iPadOS's hidden bar revealed at the top edge, C-37†).
- **Variants:** keyed `kde-breeze` `@quick` (the Kirigami MenuBar where it differs from QMenuBar, Q2.9, Q2.10).
- **Sizes and densities:** none. macOS's "Menu bar size" accessibility setting is the condition
  `cond container_size(menu_bar, large)` (S7.1 `ContainerSize`, MAC-N20), not a size.
- **Flags:** keyed `@weak_focus` (`kde-breeze`, K2.1); state flags used: hover, pressed, open (on `item`: its menu
  shown, macOS's highlighted title, Windows' selected state, W2-22), disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (CommonStates), `open` (W2-22's selected,
  the menu open) combining; others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `app_name`, `status_item`, `extra_window` (every preset but `macos-*`); `focus_line`
  (every preset but `kde-breeze`). values: `status_item_gap_px`, `extra_window_gap_px`
  (every preset but `macos-*`).
- **State-model source:** GNOME `_menus.scss` (`menubar > item`); KDE Breeze `CE_MenuBarItem` branches and
  `MenuBarItem.qml`; macOS menu-bar title states (normal, highlighted) and appearance; Windows `MenuBarItem` visual
  states; iPadOS `UIMainMenuSystem`.
- **Today's fields:** none (today's `menu.*` are the open menus' and stay there).
- **Notes:** the open menus are `menu` elements; their corners under a menu bar are `menu`'s rules
  `@ context menu_bar` (K1.64; GNOME's submenu popovers, GN-2.30).

### navigation_view

*From group 6's catalogue.*

- **Kind:** typed widget: the stack that pushes and pops pages. GNOME's AdwNavigationView (GN-2.18) and GtkStack
  (GN-3.22), KDE's StackView (Q2.14) and Kirigami's PageRow layers (Q2.15); iOS's navigation push and pop (X-MO-3),
  macOS's `NSPageController` page transitions (MAC-P3-10†), WinUI's `Frame` navigation transitions (Entrance,
  DrillIn, Slide, Suppress, W3-32) and Material's transition patterns between destinations (container transform,
  shared axis, fade through, MAT-X14) are its presentations.
- **Available:** every preset; on `kde-breeze` both flavours (`kirigami`: StackView, PageRow; `widgets`:
  QStackedWidget, whose page change Breeze fades under its stacked-widget transitions switch, K3.31).
- **Parts:**
  - `body` — the stack's viewport, which clips the moving pages (the root): `bindings`
  - `page` *(element: each page, the one that moves and the one it uncovers)* — during a transition: `transform`,
    `opacity`
  - `edge_shadow` — the shade beside a page sliding over another (GN-2.18, GN-3.05; macOS's shadow under the top
    page, to verify, MAC-P3-10†; iOS's edge shadow, X-MO-3), fading with the transition: `fill`, `min_width`,
    `opacity`
  - `scrim` — the dim over the page beneath (GN-2.18, GN-3.04; iOS's dim, X-MO-3), fading with the transition:
    `fill`, `opacity`
- **Widget values:** `presentations: Vec<PresentationTransition>`: keyed events `@push`, `@pop`, `@replace` (GNOME's
  spring, GN-2.18; GtkStack's transitions, GN-3.22; StackView's slide and fade, Q2.14; iOS's push with the parallax of
  the covered view and the title cross-fade, X-MO-3; macOS's page-controller styles, MAC-P3-10†; Kirigami's layers,
  Q2.15, are these events under `variant @layer`, one name per occasion); WinUI's navigation transitions and Material's patterns are
  keyed kinds of `@push` and `@pop` (W3-32, MAT-X14); the steps of one event run together (ADD-B10, ADD-A25,
  ADD-A26);
  `swipe_back: Value<bool>` (the swipe-back gesture, GN-2.18); `swipe_duration_min_ms`,
  `swipe_duration_max_ms: Value<Ms>` (the swipe's animation bounds, GN-2.18).
- **Variants:** keyed `kde-breeze` `@layer` (PageRow layers, Q2.15); `macos-*` `@stack_history`, `@stack_book`,
  `@horizontal_strip` (`NSPageController.TransitionStyle`, MAC-P3-10†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none (no row states a widget flag).
- **Exclusive groups:** none (no interaction states); precedence: state models (S17).
- **may_be_absent:** parts: `edge_shadow`, `scrim` (`kde-breeze`: StackView and PageRow slide and fade without them,
  Q2.14, Q2.15; `windows-11`, `material-3`, whose rows name none). values: `swipe_back`, `swipe_duration_min_ms`,
  `swipe_duration_max_ms` (every preset but `adwaita`).
- **State-model source:** GNOME `adw-navigation-view.c` transitions, `adw-swipe-tracker.c`, `gtkstack.c` and
  `_misc.scss`; KDE `StackView.qml` transitions and `PageRow.qml`; iOS's navigation controller (measured, X-MO-3);
  macOS `NSPageController.transitionStyle` (MAC-P3-10†); WinUI's `NavigationTransitionInfo` kinds (W3-32); Material's
  motion patterns (MAT-X14).
- **Today's fields:** none.
- **Notes:** under `cond reduced_motion` libadwaita crossfades: rules restate `presentations` (GN-2.18, GN-3.25).
  Group 8 wrote `transition_shadow`, `dimming` (GN-3.04, GN-3.05) and `stack` (GN-3.22).

### page_indicator

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwCarouselIndicatorDots and AdwCarouselIndicatorLines (GN-2.38, GN-2.55†), iOS's
  UIPageControl (C-9), KDE's PageIndicator (Q2.11), Windows' PipsPager (W2-20) and PagerControl (W2-21).
- **Available:** every preset but `macos-*`, `material-3` (`available = false`: no such control); `kde-breeze`
  flavour `kirigami` only (QQC2's PageIndicator).
- **Parts:**
  - `body` — the indicator's box (the root: its padding the dots' and lines' margins, GN-2.55†, Q2.11): `padding`
  - `dot` *(element; the current page is `selected`)* — a dot (its size per state, GN-2.55†, Q2.11, W2-20; iOS's
    current indicator elongating, C-9): `fill`, `opacity`, `corners`, `min_width`, `min_height`, `transitions`
    (GNOME's linear indicator animation, GN-2.38; KDE's opacity animation, Q2.11; GNOME's size and opacity
    interpolated with the scroll position, as expressions over `element.scroll_progress`, `element.item_index` and
    `element.item_count`, which it declares, `data = { dot = ["scroll_progress", "item_index", "item_count"], line =
    ["scroll_progress", "item_index", "item_count"] }`, ADD-B13, GN-2.55†)
  - `line` *(element: GNOME's line indicator, GN-2.55†)* — its thickness, length and opacity, interpolated as the
    dots are (GN-2.38): `fill`, `opacity`, `min_width`, `min_height`, `transitions`
  - `platter` — iOS's translucent rounded background while interacting (C-9): `fill`, `corners`
  - `progress` — iOS's timer fill of the current indicator (C-9): `fill`
  - `selection_indicator` — PagerControl's (W2-21): `fill`
- **Widget values:** `dot_gap_px: Value<Px>` (C-9; the line form's own gap a rule `@ variant @lines`, GN-2.55†);
  `max_visible: Value<u32>` (W2-20); `nav_buttons_visibility: Value<NavButtonsVisibility>` (PipsPager's buttons shown
  on hover, W2-20); `edge_dots_shrink: Value<bool>` (iOS's shrinking edge dots when overflowing, C-9).
- **Variants:** typed `vertical` (iOS's vertical direction, C-9; Windows' orientation, W2-20); keyed `adwaita`
  `@lines` (GN-2.55†); `ios-*` `@prominent`, `@minimal` (background styles, C-9; automatic is the base); `windows-11`
  `@button_panel`, `@combo_box`, `@number_box` (PagerControl's display modes, W2-21: the application's choice of
  which control the pager shows; under `@combo_box` and `@number_box` that control is a `combo_box` or `stepper`
  element in `context page_indicator.body`, Nested controls, so the variants select only the pager's own parts).
- **Sizes and densities:** none (no row states a size).
- **Flags:** keyed `@scrubbing` (`ios-*`, C-9); state flags used: selected, hover, pressed (KDE's pressed opacity,
  Q2.11), disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `dot` (CommonStates), `selected` combining;
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `line` (every preset but `adwaita`); `platter`, `progress` (every preset but `ios-*`);
  `selection_indicator` (every preset but `windows-11`). values: `max_visible`,
  `nav_buttons_visibility` (every preset but `windows-11`); `edge_dots_shrink` (every preset but `ios-*`).
- **State-model source:** GNOME `adw-carousel-indicator-dots.c`, `adw-carousel-indicator-lines.c`; iOS
  `UIPageControl`; KDE `PageIndicator.qml`; Windows PipsPager and PagerControl visual states.
- **Today's fields:** none.
- **Notes:** Nested controls: Windows' previous, next, first and last buttons are `button @ variant previous / next
  context page_indicator` and `button @ variant @first / @last context page_indicator` (keyed `windows-11`, W2-20,
  W2-21); PagerControl's number buttons are `button @ variant @page_number context page_indicator` (W2-21: its
  width and states are the button's there; the check pass had a `page_button` part).

### picker_wheel

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's UIPickerView and UIDatePicker `.wheels` (C-7), Material Expressive's scroll field
  (MAT-N31†), Windows' looping selectors in the date and time picker flyouts (W2-04), the tumblers of kirigami-addons'
  time popup (K-KA-23†).
- **Available:** every preset but `adwaita`, `macos-*` (`available = false`: no such control; macOS's pickers are
  textual and graphical only, MAT-N31†'s cross-check). `kde-breeze` flavour `kirigami` only, in kirigami-addons'
  time popup (K-KA-23†).
- **Parts:**
  - `container` — the root, always drawn: Material's container, fill per state, its shape by corners and its
    height (MAT-N31†); its fields `Absent` on the other presets: `fill`, `corners`, `min_height`, `padding`
  - `item` *(element; the centred item is `selected`)* — `text`, `fill`, `opacity` (the text style's size, line
    height and tracking and the opacity by the distance from the centre, as expressions over
    `element.scroll_progress` and `element.item_index`, which it declares, ADD-B13: iOS's fading of off-centre
    rows, C-7, kirigami-addons' opacity by displacement, K-KA-23†, Material's text interpolated between two ramp
    roles, MAT-N31†), `padding` (Windows' item padding, W2-04), `min_height` (the item's height, C-7, W2-04,
    K-KA-23†), `focus` (Material's focus ring, MAT-N31†),
    `transitions` (the snap to the centre, group `Geometry` with a spring, C-7, MAT-N31†)
  - `selection_band` — the band (iOS's band, C-7; Windows' highlight band with its height, W2-04; kirigami-addons'
    highlight border, thicker with keyboard focus, K-KA-23†): `fill`, `stroke`, `corners`, `min_height`, `margin` (its
    insets, C-7)
- **Widget values:** `visible_items: Value<u32>` (C-7, MAT-N31†, K-KA-23†);
  `curvature: Value<f32>` (iOS's perspective curvature, C-7); `component_gap_px: Value<Px>` (C-7); `momentum:
  ScrollPhysics` (a plain value, S4.2's, C-7).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own; state flags used: selected (the centred item), disabled (Material's disabled colours),
  focus_visible: Material's focus ring (MAT-N31†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (the looping selector item's
  CommonStates), `selected` combining; others none (states combine); precedence: state models (S17).
- **may_be_absent:** fields: `container.fill`, `container.corners`, `container.min_height`, `container.padding`
  (every preset but `material-3`). values: `curvature` (every preset but `ios-*`).
- **State-model source:** iOS `UIPickerView`; Material `ScrollField.kt`; Windows `LoopingSelector` visual states; KDE
  kirigami-addons `TimePicker.qml` (its Tumbler from Qt's fallback style, to verify, K-KA-23†).
- **Today's fields:** none.
- **Notes:** values interpolated by the scroll position (Material's text style between two ramp roles, iOS's fade and
  curvature, kirigami-addons' opacity by distance) read element data (`element.scroll_progress`, ADD-B13). The looping
  selectors (W2-04) are `picker_wheel` elements in `context time_picker.popup` and `context date_picker.popup`, and
  kirigami-addons' tumblers (K-KA-23†) in `context time_picker`.

### pull_to_refresh

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's UIRefreshControl (C-10), Kirigami ScrollablePage's PullDownIndicator (Q2.54†, Q1.121),
  macOS's `NSScrollView.refreshController` (MAC-N24, macOS 27), Material's Compose PullToRefresh (MAT-N28), Windows'
  RefreshContainer and RefreshVisualizer (W2-26).
- **Available:** every preset but `adwaita`, `macos-monterey` (`available = false`: GNOME has no such control; macOS
  draws it from 27, MAC-N24, the newest release of the generation `macos-tahoe` describes, R1.5). `kde-breeze` flavour
  `kirigami` only (Q2.54†).
- **Parts:**
  - `body` — the control's box (the root), holding the indicator and Kirigami's strip: `padding`, `bindings`.
  - `indicator` — the indicator's container (Material's container colour, size and elevation, MAT-N28; Windows'
    circle and colours, W2-26; the arc inset in its box, Q2.54†): `fill`, `corners`, `shadow`, `padding`,
    `min_width`, `min_height`
  - `arrow` — Windows' arrow glyph (W2-26): `glyph`
  - `title` — iOS's attributed title (C-10): `text`
  - `strip` — Kirigami's strip above the content, with its height (Q2.54†): `min_height`
- **Widget values:** `trigger_distance_px: Value<Px>` (C-10, MAT-N28, W2-26); `refresh_delay_ms: Value<Ms>` (Q2.54†);
  `content_shift_px: Value<Px>` (the content moved down while refreshing, Q2.54†); `loop: Value<Loop>` (Windows' spin
  animation, W2-26); `progress_sweep: Value<ProgressSweep>`, `ProgressSweep { Arc, Spokes }` (KDE's arc swept by the
  pull progress, Q2.54†; iOS's spokes by the pull progress, C-10); `presentations: Vec<PresentationTransition>`:
  events `@collapse` (iOS's collapse after `endRefreshing`, C-10; Material's motion, MAT-N28), `@content_shift` (KDE's
  content moving down, Q2.54†).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** shared `pulling`, `refreshing` (every preset that draws it); state flags used: none other.
- **Exclusive groups:** none (the connector sets one phase at a time); precedence: state models (S17).
- **may_be_absent:** parts: `arrow` (every preset but `windows-11`); `title` (every preset but `ios-*`); `strip`
  (every preset but `kde-breeze`). fields: `indicator.fill`, `indicator.corners`, `indicator.shadow` (`ios-*`,
  `kde-breeze`: UIRefreshControl and Kirigami's indicator draw no disc, C-10, Q2.54†), `body.padding`. values:
  `refresh_delay_ms`, `content_shift_px` (every preset but `kde-breeze`);
  `loop`
  (every preset but `windows-11`: the others spin the `spinner` element).
- **State-model source:** iOS `UIRefreshControl`; KDE `PullDownIndicator.qml`, `ScrollablePage.qml`; macOS
  `NSScrollView` refresh controller (27); Material Compose `PullToRefresh` states; Windows `RefreshVisualizerState`.
- **Today's fields:** none.
- **Notes:** Nested controls: the pull-progress arc or spokes are a `spinner` in `context pull_to_refresh`, variant
  `determinate` while `{pulling}`, its progress the pull progress (`element.progress`, ADD-B12; KDE's arc with its
  stroke, Q2.54†, Q1.121; iOS's spokes appearing as the pull progresses, C-10, as `progress_sweep` says), and
  indeterminate while `{refreshing}` (Kirigami's BusyIndicator, Q2.54†).

### rating

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's KRatingWidget (K2.19), Windows' RatingControl (W2-06), macOS's `NSLevelIndicator`
  rating style (MAC-N06).
- **Available:** every preset but `adwaita`, `ios-*`, `material-3` (`available = false`: no such control);
  `kde-breeze` flavour `widgets` only (KRatingWidget).
- **Parts:**
  - `body` — the row of stars (the root): `padding`, `focus`
  - `star` *(element: one star)* — the star's source, Windows' font glyph and its size for rendering (W2-06), KDE's
    star icon (K2.19), with its fill per flag and state: `glyph`, `transform` (Windows' hover preview scale, to
    verify, W2-06)
  - `caption` — Windows' caption text (W2-06): `text`
- **Widget values:** `item_gap_px: Value<Px>` (W2-06); `half_steps: Value<bool>` (K2.19).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** shared `filled`, `half` (KDE, Windows, macOS), `preview` (the hover preview: KDE, Windows); shared
  `placeholder` (`windows-11`'s placeholder colour, W2-06; `macos-*`'s rating placeholders, MAC-N06); state flags
  used: hover, pressed, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CommonStates); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `caption` (every preset but `windows-11`). values: `half_steps` (every preset but
  `kde-breeze`).
- **State-model source:** KDE `kratingwidget.cpp`; Windows RatingControl visual states; macOS
  `NSLevelIndicator.Style.rating`.
- **Today's fields:** none.
- **Notes:** none (the rows place no other widget in a rating's context).

### redaction

*Written in the merge.*

- **Kind:** typed widget: macOS (MAC-P4-13†) and iOS (P4-18†) draw SwiftUI's `redacted(reason: .placeholder)`, the
  look of content drawn as a placeholder mask while it loads, list rows included. Group 8 proposed it as
  `placeholder`; the merge renames it, because `placeholder` is the text widgets' part (placeholder text). An element
  is redacted by the state flag `redacted` (ADD-A14); the mask's look is this widget's, queried in the redacted
  element's context.
- **Available:** every preset but `adwaita`, `kde-breeze`, `windows-11`, `material-3` (`available = false`: no such
  control; the `macos-*` and `ios-*` presets' releases draw it, MAC-P4-13†, P4-18†).
- **Parts:**
  - `mask` — the root: the shape that replaces text and images, keeping their size (fill per scheme, a shape per
    line of text, MAC-P4-13†, P4-18†): `fill`, `corners`, `bindings`, `transitions` (an animation, if the platform
    draws one)
- **Widget values:** none (the rows state none; the mask's look is the part's).
- **Variants:** none (the rows name the placeholder reason only).
- **Sizes and densities:** none (the mask takes the redacted content's size).
- **Flags:** none of its own; the redacted element carries the state flag `redacted`.
- **Exclusive groups:** none (no interaction states); precedence: state models (S17).
- **may_be_absent:** fields: none (no animation is an empty `mask.transitions` list, not an absence).
- **State-model source:** SwiftUI `RedactionReasons` (placeholder) on macOS and iOS, measured.
- **Today's fields:** none.
- **Notes:** the redacted element carries `redacted` (ADD-A14); this widget is queried in its context.

### rich_text

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's Markdown importer and Qt's HTML parser (K-SF12†–K-SF17†, K-P4-14†), macOS's SwiftUI
  `Text` Markdown and HTML import (MAC-SF02†–MAC-SF04†, MAC-SF06†), iOS's (SF-2†–SF-4†, SF-6†), Material's
  `AnnotatedString.fromHtml` (MAT-P3-20†).
- **Available:** every preset but `adwaita`, `windows-11` (`available = false`: no row lists one; Windows'
  MarkdownTextBlock is Toolkit Labs, out by R1.2).
- **Parts:**
  - `body` — the text's box (the root): `padding`, `bindings`
  - `paragraph`, `heading_1` … `heading_6` (the headings: K-SF12†, K-P4-14†, MAT-P3-20†): `text`, `margin` (each
    block's margins, as Qt's HTML and Markdown defaults state them, K-SF14†, K-SF17†, K-P4-14†)
  - `emphasis`, `strong`, `underline`, `strikethrough`, `subscript`, `superscript` (Compose's HTML underline,
    MAT-P3-20†): `text`
  - `inline_code` (K-SF13†, MAC-SF03†, SF-3†): `text`, `fill`
  - `code_block` (K-SF13†): `text`, `fill`, `margin`
  - `preformatted` (K-P4-14†): `text`, `margin`
  - `block_quote` (K-SF14†: no fill, no bar): `fill`, `stroke`, `margin`
  - `list_marker` — its glyph (K-SF15†): `glyph`
  - `task_box` *(element)* — the square and its cross when checked, an element of its own with its `checked` state
    (K-SF15†): `stroke`, `glyph`
  - `table` — the table's collapsed border (K-SF16†): `stroke`, `margin`
  - `table_cell` — a cell with its padding and border (K-SF16†): `padding`, `stroke`, `text`
  - `table_header_cell` — a header cell, its text centred and heavier (K-SF16†, K-P4-14†): `padding`, `stroke`,
    `text`
  - `rule` — the horizontal rule (K-SF17†): `stroke`, `margin`
  - `definition_term`, `definition_description` (K-P4-14†): `text`, `margin`
- **Widget values:** `list_indent_per_level_px: Value<Px>` (each block part's margins are its `margin`, S2.11);
  `list_marker_gap_px: Value<Px>` (K-SF15†); `list_marker_kinds: Vec<Key>` (per nesting level, K-SF15†);
  `quote_indent_per_level_px: Value<Px>` (K-SF14†).
- **Variants:** typed `markdown` (KDE's Markdown importer, SwiftUI `Text` Markdown), `html` (Qt's HTML parser, Apple's
  HTML import, Compose `AnnotatedString.fromHtml`). Content variant `@fenced` on `code_block` (the connector sets it
  from the block's source: a fenced block, whose lines do not wrap, K-SF13†).
- **Sizes and densities:** none (the text follows the text ramp).
- **Flags:** none of its own; state flags used: checked (on the `task_box` element, K-SF15†).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: under `variant markdown` on `macos-*` and `ios-*` every block part (`heading_1` …
  `heading_6`, `list_marker`, `task_box`, `block_quote`, `code_block`, `preformatted`, `table`, `table_cell`,
  `table_header_cell`, `rule`: MAC-SF02†, SF-2†); `task_box` (every preset but `kde-breeze`); `definition_term`,
  `definition_description` (every preset but `kde-breeze`, its HTML); `subscript`, `superscript` (`kde-breeze` under
  `variant markdown`). fields: `inline_code.fill`, `code_block.fill`, `block_quote.fill`, `block_quote.stroke`
  (`kde-breeze`: no fill, no border, no bar, K-SF13†, K-SF14†).
- **State-model source:** KDE `qtextmarkdownimporter.cpp`, `qtextdocumentlayout.cpp`, Qt's HTML parser
  (`qtexthtmlparser.cpp`); Apple `InlinePresentationIntent`, `PresentationIntent`, the HTML importer; Compose
  `Html.android.kt` span mapping.
- **Today's fields:** none. Stand-ins: `defaults.mono_font` (K-SF13†, MAC-SF03†, SF-3†) becomes the text ramp's
  monospace role, referenced from `inline_code.text` (`= token.text.roles.monospace`); `defaults.link_color`
  (MAC-SF04†, SF-4†) the `link` widget's text fill in `context rich_text`.
- **Notes:** links in the text are `link` variant `inline` in `context rich_text`, with hover, pressed and visited
  where the platform draws them (MAC-SF04†, SF-4†, K-P4-14†, MAT-P3-20†).

### rubber_band

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's UIBandSelectionInteraction (C-47†), KDE's QRubberBand (K2.12) and its dock drop zone
  (K-SF18†), macOS's marquee in Finder and `NSCollectionView` (MAC-N31†), GNOME's `rubberband` node in lists and icon
  views (GN-1.175, GN-1.280†).
- **Available:** every preset but `windows-11`, `material-3` (`available = false`: no row lists one); `kde-breeze`
  flavour `widgets` only (QRubberBand).
- **Parts:**
  - `body` — the band, the root: `fill`, `stroke`, `corners`, `bindings` (the selection roles of the view it is drawn
    over, K2.12, GN-1.175).
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `Disappear` (iOS's fade, C-47†).
- **Variants:** keyed `kde-breeze` `@filled` (set by the connector where the band's parent is a QGraphicsView, which
  the model has no widget for, so no `context` rule can see it; inside a list or a main window the `context` rules
  fill it, K2.12). It is not a content variant: it names a host the model types no widget for, and only KDE draws
  the difference, so it is keyed.
- **Sizes and densities:** none (the band follows the drag).
- **Flags:** none (no row states one).
- **Exclusive groups:** none (no interaction states); precedence: state models (S17).
- **may_be_absent:** fields: `body.fill` (`kde-breeze` outside an item view, graphics view or main window: the outline
  only, K2.12, K-SF18†).
- **State-model source:** iOS `UIBandSelectionInteraction`; KDE Breeze `CE_RubberBand` and `SH_RubberBand_Mask`; macOS
  as drawn by Finder and `NSCollectionView` (measured); GNOME `_views.scss` (`rubberband`).
- **Today's fields:** none.
- **Notes:** KDE fills it inside an item view, a graphics view or a main window and draws the outline elsewhere: the
  base is the outline, `context any list` and `context window` rules add the fill (K2.12; K-SF18†'s dock drop zone
  filled in `context window`). Group 4 traced the list's marquee here (GN-1.175, GN-1.280†): `rubber_band.body` in
  `context list`.

### ruler

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's KRuler (K2.32†), macOS's `NSRulerView` (MAC-N17).
- **Available:** every preset but `adwaita`, `windows-11`, `ios-*`, `material-3` (`available = false`: no such
  control); `kde-breeze` flavour `widgets` only (KRuler).
- **Parts:**
  - `body` — the ruler, the root: `fill`, `bindings` (the widget's colour roles its marks and pointer take, K2.32†).
  - `tick` — one mark: `stroke` (its length per level from `tick_lengths`).
  - `label` — K2.32†'s labels: `text`.
  - `marker` *(element)* — macOS's markers, KDE's pointer triangle filled with the background: `glyph`.
- **Widget values:** `thickness_px: Value<Px>`; `tick_lengths: Vec<(Key, Value<Px>)>` (keys `end`, `big`, `medium`,
  `little`, `base`, K2.32†).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none (no row states one).
- **Exclusive groups:** none (no interaction states stated); precedence: state models (S17).
- **may_be_absent:** none (both presets draw every part; KDE's pointer is the application's option, K2.32†).
- **State-model source:** KDE `kruler.cpp`; macOS `NSRulerView` (measured).
- **Today's fields:** none.
- **Notes:** none (the rows place no other widget in a ruler's context).

### search_field

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkSearchBar and GtkSearchEntry (GN-2.21, GN-2.22), iOS's UISearchBar and
  UISearchTextField (C-3), macOS's `NSSearchField` (MAC-N02) and toolbar search item (MAC-TL07), Material's search bar
  and search view (MAT-N14, MAT-N15) and the app bar's embedded search (MAT-TA6), Windows' AutoSuggestBox (W2-15),
  KDE's Kirigami SearchField (Q1.15–Q1.17) and kirigami-addons' SearchPopupField (K-KA-32†). Its field is an `input`
  element in its context: the bezel, text, placeholder and magnifier are the input's (Nested controls).
- **Available:** every preset; `kde-breeze` flavour `kirigami` only (Kirigami's SearchField; Qt Widgets' search
  fields are line edits, `input`).
- **Parts:**
  - `body` — the search field's outer box, the root: `padding`, `bindings`.
  - `bar` — GNOME's search bar, GN-2.21; Material's search bar container, MAT-N14; UISearchBar's background in its
    default and prominent styles, C-3: `fill` (GNOME's header-bar fill;
    Material's container, its on-scroll fill in an app bar, MAT-TA6), `stroke` (GNOME's bottom shade line), `corners`,
    `min_height`, `shadow`, `padding`.
  - `recents_indicator` — macOS's, MAC-N02: `glyph`.
  - `supporting_text` — MAT-N14: `text`.
  - `view` — Material's search view, MAT-N15; kirigami-addons' popup surface, K-KA-32†: `fill`, `stroke`, `corners`,
    `shadow`, `bindings` (kirigami-addons' View set, K-KA-18†, K-KA-32†).
  - `view_header` — MAT-N15: `min_height`.
  - `view_divider` — MAT-N15; kirigami-addons' separator under the field, K-KA-32†: `stroke` (the line, its top
    edge).
  - `scrim` — the dim behind Material's search view, MAT-X23: `fill`, `opacity`.
- **Widget values:** `placeholder_alignment: Value<Alignment>` (macOS's centred placeholder, which moves to the
  leading edge on focus, MAC-N02; the move is the `placeholder`'s `transitions`); `view_placement: Placement`
  (kirigami-addons' popup, strategy `KirigamiAddonsSearchPopup`, K-KA-32†); `auto_search_min_chars: Value<u32>`
  (kirigami-addons' `autoAccept`, K-KA-32†); `presentations: Vec<PresentationTransition>`: events `@bar_reveal`
  (GNOME's search bar, a revealer, GN-3.22),
  `@view_open`, `@view_close` (Material's search view, MAT-N15; kirigami-addons' popup growing from the field and
  back, K-KA-32†), `@expand` (macOS's toolbar search item expanding from a button into a field, MAC-TL07).
- **Variants:** typed `flat` (GNOME's flat search bar, GN-2.21; `button`'s meaning); keyed `adwaita` `@inline`,
  `@shrunk` (the search bar, GN-2.21); `ios-*` `@prominent`, `@minimal` (`searchBarStyle`, C-3; the default style
  is the base); `material-3` `@docked`, `@full_screen` (the view, MAT-N15),
  `@contained` (Expressive, MAT-N14); `kde-breeze` `@ka_search_popup` (K-KA-32†), `@delay_search` (Kirigami
  SearchField's `delaySearch`: the field's search delay the human moment instead of the short duration, Q1.17).
- **Sizes and densities:** `macos-*`: its control sizes (MAC-N02).
- **Flags:** none of its own (the text's `empty` is the `input`'s, which hides the clear button); state flags used:
  focus_within, disabled.
- **Exclusive groups:** none of its own: the field's `[disabled, focused, hover]` (the text box's CommonStates on
  `windows-11`) is `input`'s, the buttons' `[disabled, pressed, hover]` `button`'s; precedence: state models (S17).
- **may_be_absent:** parts: `recents_indicator` (every preset but `macos-*`); `supporting_text`, `view_header` (every
  preset but `material-3`); `view`, `view_divider` (every preset but `material-3`, `kde-breeze`); `bar` (`macos-*`,
  `windows-11`, `kde-breeze`; `ios-*` under `@minimal`); `scrim` (every preset but `material-3`). values:
  `view_placement`, `auto_search_min_chars` (every preset but
  `kde-breeze`).
- **State-model source:** GNOME `_toolbars.scss` (`searchbar`), `_entries.scss`; iOS `UISearchBar`,
  `UISearchTextField`, `UISearchBar.Style`; macOS `NSSearchField` cell states; Material search-bar and search-view
  tokens; Windows AutoSuggestBox visual states; KDE Kirigami `SearchField` and kirigami-addons `SearchPopupField.qml`.
- **Today's fields:** none (`input.*`, the stand-in, stays `input`'s).
- **Notes:** in a toolbar (MAT-TA6, MAC-TL07; iOS 26's glass capsule in a toolbar or the tab bar, C-3) its rules are
  `@ context toolbar.bar`. Nested controls: the field is an `input` in `context any search_field` (its bezel
  `input.body`, its text, its `placeholder`, the magnifier `input.icon @ variant leading`, Material's leading icon
  likewise, MAT-N14; KDE's field metrics, Q1.15–Q1.17); the clear button `button @ variant clear_field` there; iOS's
  cancel button `button @ variant @cancel context search_field` (C-3; its appearing and disappearing are the
  button's `Appear`, `Disappear` there, as a field's clear button's are); Windows' query button `button @ variant @query
  context search_field` (W2-15); Material's trailing icon buttons `button @ variant icon_only context search_field`
  and its avatar `avatar @ context search_field` (MAT-N14); GNOME's search-bar close button `button @ variant close
  context search_field.bar` (GN-2.21); iOS's search tokens `chip @ context search_field` (C-3). One tree with
  `autocomplete`: the suggestion list is an `autocomplete` in the field's context (W2-15, C-38†); a search field in a
  form row (GN-2.06) or a sidebar
  header (K-KA-41†) is placed by context.

### sheet

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwBottomSheet (GN-2.17), iOS's UISheetPresentationController (C-12) with its
  placement per size class and with the keyboard (SF-7†, SF-8†, P3-6†), KDE's Kirigami OverlaySheet (Q2.37–Q2.40,
  K-SF23†) and kirigami-addons' BottomDrawer (K-KA-30†, K-KA-31†), Material's bottom, side and floating sheets
  (MAT-N19–MAT-N21, MAT-SF03†).
- **Available:** every preset but `macos-*`, `windows-11` (`available = false`: no such control; macOS's sheet is
  `dialog` variant `@sheet`, MAC-D03); `kde-breeze` flavour `kirigami` only (OverlaySheet, BottomDrawer).
- **Parts:**
  - `body` — the root: `fill`, `stroke` (GNOME's inner outline, GN-2.17), `corners`, `shadow`, `padding` (the content
    padded by the bottom safe area, K-KA-30†), `max_width` (Material's on wide windows, MAT-SF03†), `max_height`
    (kirigami-addons' share of the window's height, K-KA-30†), `bindings` (on `ios-*` the `Dialog` set with the
    elevated values and the `Binding::Role` entries of `dialog.body`, R2.3, R5.2, D-14), `transitions` (on corners and
    shadow, GN-2.17).
  - `handle` *(element)* — the grabber, an element of its own with its hover, pressed, dragged and focus states
    (GN-2.17, C-12, Q2.39, MAT-N19, K-KA-30†): `fill`, `corners`, `opacity` (Q2.39, MAT-N19, K-KA-30†),
    `min_width`, `min_height`, `padding` (its margins and C-12's offset), `focus` (MAT-N19).
  - `header` — `fill`, `corners` (kirigami-addons' header strip, K-KA-30†), `padding` (Q2.38), `bindings`
    (kirigami-addons' Window set, K-KA-30†).
  - `title`, in `header` — `text` (Q2.38's heading; Material's side-sheet headline, MAT-N20); the header's binding
    reaches it (S2.11).
  - `header_separator`, in `header` — Q2.38, K-KA-30†: `stroke` (the line, its top edge).
  - `footer` — `padding` (Q2.38).
  - `footer_separator` — Q2.38: `stroke` (the line, its top edge).
  - `scrim` — the dim (GN-2.17, GN-3.04, Q2.38, X-SC-1, MAT-X23): `fill`, `opacity` (per detent, C-12, X-SC-1),
    `transitions` (Q2.38).
  - `bottom_bar` — AdwBottomSheet's always-visible bar, GN-2.17: `fill`, `shadow`.
  - `presenter_backdrop` — the black behind the presenter that iOS 13–18's stacked-card effect scales down (C-12,
    X-SC-1): `fill`.
  - `divider` — Material's side sheet, MAT-N20: `stroke` (the line, its start edge).
- **Widget values:** `detents: Vec<(Key, Value<Px>)>` (iOS's medium and large, C-12); `undimmed_up_to: Value<Key>`
  (iOS's largest undimmed detent, C-12); `min_top_inset_px: Value<Px>` (KDE's top floor, Q2.40, K-SF23†);
  `anchor: Value<SheetAnchor>`,
  `SheetAnchor { BottomCenter, Center, Side }` (where the platform places the sheet: bottom-centred: GNOME, Material,
  iOS in compact width; centred: KDE's OverlaySheet, iOS in regular width; side: Material's side sheet), stated by
  rules under the variants and restated by rules `@ h_size_class c` and `@ v_size_class c` (SF-7†, ADD-A20). It is
  not the variants' meaning twice: a variant is the kind of sheet the application chose and selects its look's
  rules; `anchor` is where the platform puts that sheet, which on iOS changes with the size class for one variant
  (`@form` centred in a regular width, at the bottom in a compact one, SF-7†), so a connector reads it from the
  theme; `swipe_to_close_px: Value<Px>` (the swipe threshold, Q2.40; iOS's interactive
  dismiss, C-12); `keyboard_avoidance: Placement` (where a sheet goes when the keyboard appears, strategy
  `KeyboardAvoid(…)`, P3-6†, to verify); `presenter_transform: Transform`, `presenter_corners: Corners` (iOS 13–18's
  stacked-card effect on the presenter, C-12); `top_padding_range: (Value<Px>, Value<Px>)` (GNOME's top padding by
  the window's height, GN-2.17); `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (springs:
  GNOME, iOS; KDE's opacity and translation, Q2.40; per modal style, X-MO-4; Material's drag motion, MAT-N19),
  `@settle` (Material's settle, MAT-N19; iOS's detent changes, X-MO-4), `@bottom_bar_reveal` (GN-2.17).
- **Variants:** typed `bottom` (GNOME, iOS, Material, kirigami-addons' BottomDrawer), `floating` (Material's floating
  sheet, MAT-N21; iOS 26's inset sheet at partial detents, to verify, C-12); keyed `adwaita` `@flush` (GN-2.17);
  `ios-*` `@page`, `@form` (SF-8†), `@edge_attached` (compact height, C-12, SF-7†), and the modal transition styles
  `@cover_vertical`, `@cross_dissolve`, `@flip_horizontal`, `@partial_curl`, `@zoom` (`UIModalTransitionStyle`, set
  by the connector from the presentation, each with its own `presentations`, X-MO-4); `material-3` `@side`, `@modal`,
  `@standard`, `@detached` (MAT-N19, MAT-N20); `kde-breeze` `@overlay` (the centred OverlaySheet, Q2.37),
  `@ka_bottom_drawer` (kirigami-addons' BottomDrawer's own look, set together with the typed `bottom`, `variants
  {bottom,@ka_bottom_drawer}`: `bottom` is where the sheet sits, the keyed name the library's drawing beside
  Kirigami's own sheets; K-KA-30†, K-KA-31†).
- **Sizes and densities:** none. Size classes choose the placement: rules `@ h_size_class c` / `@ v_size_class c`
  (Material's wide windows, MAT-SF03†; iOS's compact height, SF-7†); Kirigami's drag handle shows under
  `cond mobile_form_factor` (Q2.39, Q3.31); `cond reduced_motion` restates `presentations` (GNOME's crossfade,
  GN-3.25).
- **Flags:** shared `minimized` (Material's minimised shape, MAT-N19); state flags used: hover, pressed, dragged,
  focus_visible, on the `handle` element (its focus indicator, MAT-N19).
- **Exclusive groups:** `material-3`: `[dragged, pressed, focus_visible, hover]` on `handle` (state layers, MAT-N19;
  the side sheet's close button's are `button`'s, MAT-N20); others none (states combine); precedence: state models
  (S17).
- **may_be_absent:** parts: `handle` (`kde-breeze` outside `cond mobile_form_factor`, Q2.39; an application that
  hides iOS's grabber, C-12, does not query it); `bottom_bar` (every preset but `adwaita`); `divider` (every preset
  but `material-3`); `presenter_backdrop` (every preset but `ios-*`; Open items).
  values: `presenter_transform`, `presenter_corners`, `undimmed_up_to`, `keyboard_avoidance` (every preset but
  `ios-*`); `top_padding_range` (every preset but `adwaita`); fields: `body.stroke` (all but `adwaita`).
- **State-model source:** GNOME `_bottom-sheet.scss`, `adw-bottom-sheet.c`; iOS `UISheetPresentationController`; KDE
  `OverlaySheet.qml`, kirigami-addons `BottomDrawer.qml`; Material sheet tokens and `ModalBottomSheet.kt`.
- **Today's fields:** none (`dialog.*` stand-ins stay `dialog`'s).
- **Notes:** Nested controls: Material's side-sheet close button is `button @ variant close context sheet`
  (MAT-N20). Group 1 traced iOS's placement per size class and with the keyboard here (SF-7†, SF-8†, P3-6†); group 8
  wrote `grabber` (Q3.31) and `dimming` (X-SC-1, MAT-X23), and the presentations per modal style (X-MO-4).
  kirigami-addons' convergent context menu shows its mobile form as this bottom drawer (K-KA-31†).

### shortcuts_dialog

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwShortcutsDialog (GN-2.41), KDE's KShortcutsDialog and KShortcutsEditor (K-RL02†)
  and kirigami-addons' ShortcutsEditor (K-KA-43†).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: no such
  control).
- **Parts:**
  - `body` — the dialog's content, the root: `padding`, `bindings`.
- **Widget values:** `page_gap_px: Value<Px>` (GN-2.41); `natural_size: Axes<Value<Px>>` (kirigami-addons' square
  dialog layer, K-KA-43†).
- **Variants:** keyed `kde-breeze` `@ka_quick` (K-KA-43†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (the recording key-sequence button's `@recording` is `button`'s; the rows' open
  editor is `list`'s `open`; the navigation pills are `button` elements); state flags used: none.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** values: `natural_size` (`adwaita`).
- **State-model source:** GNOME `_shortcuts-dialog.scss`; KDE `kshortcutseditordelegate.cpp`,
  `kkeysequencewidget.cpp`, `kextendableitemdelegate.cpp` and Breeze's button branches; kirigami-addons
  `ShortcutsEditor.qml`, `KeySequenceItem.qml`.
- **Today's fields:** none.
- **Notes:** Nested controls: GNOME's navigation pill buttons are `button @ variant pill context shortcuts_dialog`
  (GN-2.41); the shortcuts are `list` rows in `context shortcuts_dialog` (a section's title
  `list.section_header`, a row's title its text and its dimmed caption `list.secondary_text`, its minimum height
  `list.row.min_height` there, GN-2.41; KDE's in-row extender's arrow `list.disclosure`, open while the editor shows,
  K-RL02†); GNOME's shortcut labels `keycap` elements there (GN-2.41); KDE's key-sequence widget and the Quick
  recorder `button @ variant @key_sequence` (K-RL02†, K-KA-43†) and their clear buttons `button @ variant
  clear_field`; kirigami-addons' Save and Reset bar `toolbar @ variant bottom context shortcuts_dialog`
  (K-KA-43†).

### split_button

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwSplitButton (GN-2.29), macOS's `NSComboButton` style `.split` (MAC-N28†),
  Material Expressive's split button (MAT-N7), Windows' SplitButton and ToggleSplitButton (W2-17), KDE's tool button
  with a separate menu sub-control (`MenuButtonPopup`, K1.155, flavour `widgets`) and kirigami-addons'
  DoubleFloatingButton (K-KA-27†, flavour `kirigami`).
- **Available:** every preset but `ios-*` (`available = false`: no such control). macOS draws it as `NSComboButton`
  (MAC-N28†).
- **Parts:**
  - `body` — the whole control, the root: `padding`, `corners`, `shadow` (kirigami-addons' shared shadow, K-KA-27†),
    `focus`, `bindings`.
  - `primary` *(element)*, `secondary` *(element: the arrow part; KDE's menu sub-control, K1.155)* — the two halves:
    `fill`, `stroke` (W2-17), `corners` (the inner corners per state, Material, MAT-N7; GNOME's joined inner corners,
    GN-2.29), `padding` (GN-2.29; Material's leading and trailing spaces, MAT-N7), `min_width`, `min_height` (W2-17's
    part sizes), `transitions` (kirigami-addons' state-colour change, K-KA-27†). `arrow` belongs to `secondary` and is
    queried with it. `primary_label`, `primary_icon` belong to `primary` and are queried with it.
  - `primary_label` — `text`, `transform` (Material's optical centring: the content moved off the geometric centre
    by a coefficient of its half's outer and inner corner difference, `translate_x` an expression over
    `widget.primary.corners`, MAT-X37†, ADD-A80; it declares `width` and `height` for corners that resolve `Full`, S8).
  - `primary_icon` — `icon`, `transform` (as `primary_label`'s, MAT-X37†).
  - `arrow` — `glyph` (KDE's arrow colour by state, K1.155; Material's trailing icon size, MAT-N7), `transform` (its
    rotation while `open`, Material, MAT-N7; its optical centring over `widget.secondary.corners`, MAT-X37†),
    `transitions` (that rotation's).
  - `divider` — GN-2.29, W2-17; KDE's separator line, K1.155: `stroke` (the line, its start edge), `padding` (its
    margins, GN-2.29), `opacity` (GNOME's flat divider, hidden on hover and focus, GN-2.29).
- **Widget values:** `part_gap_px: Value<Px>` (negative where the parts overlap, GN-2.29; Material's between-space,
  MAT-N7); `divider_margin_px: Value<Px>` (GN-2.29); `menu_placement: Placement` (where the menu its secondary half
  opens goes, the host's value by Nested controls; K1.155, W2-17, MAC-N28†).
- **Variants:** typed `primary` (GNOME's suggested look; Windows' accent style), `destructive` (GN-2.29), `flat`
  (GN-2.29), `toggle` (Windows' ToggleSplitButton: button's `toggle` meaning, R15; its state `checked`, W2-17); keyed
  `adwaita` `@opaque` (GN-2.29), `@raised` (kept raised in a toolbar, GN-1.151); `kde-breeze` `@ka_double_floating`
  (K-KA-27†).
- **Sizes and densities:** `material-3`: `extra_small`, `small`, `regular`, `large`, `extra_large` (Material's xsmall
  … xlarge, MAT-N7, ADD-A62); `macos-*`: its control sizes (MAC-N28†).
- **Flags:** none of its own; state flags used: hover, pressed, disabled, focused, focus_visible, open (on
  `secondary`, its menu shown: rotates `arrow` and changes the trailing half's corners, Material's one trailing-button
  state, MAT-N7), checked (`toggle`).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` per part (CommonStates), `checked` combining
  (`toggle`); `material-3`: `[dragged, pressed, focus_visible, hover]`; others none (states combine); precedence:
  state models (S17).
- **may_be_absent:** parts: `divider` (`material-3`: its `part_gap_px` stands for it, MAT-N7). values:
  `divider_margin_px` (every preset but `adwaita`); fields: `body.shadow` (all but `kde-breeze`, K-KA-27†).
- **State-model source:** GNOME `_buttons.scss` (`splitbutton`); macOS `NSComboButton.Style.split`; Material
  split-button tokens; Windows SplitButton visual states; KDE Breeze's tool-button branches for `MenuButtonPopup`
  (`st:4431-4470`, `st:4992-5056`, K1.155); kirigami-addons `DoubleFloatingButton.qml`.
- **Today's fields:** none.
- **Notes:** the inner corners follow the state (Material: rest, hovered, pressed, and the trailing half's menu open:
  `primary.corners`, `secondary.corners` rules); GNOME hides the divider on hover and focus under `variant flat` (`divider.opacity`
  rules). Group 6's suggested look is `primary` (rename row 30).

### status_page

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwStatusPage (GN-2.13, its spinner GN-1.232), iOS's
  UIContentUnavailableConfiguration (C-16), KDE's Kirigami PlaceholderMessage and LoadingPlaceholder (Q2.30–Q2.32) and
  kirigami-addons' card placeholder (K-KA-16†), macOS's `ContentUnavailableView` (MAC-N35†).
- **Available:** every preset but `windows-11`, `material-3` (`available = false`: no such control); `kde-breeze`
  flavour `kirigami` only (PlaceholderMessage, LoadingPlaceholder).
- **Parts:**
  - `body` — the page's content box, the root: `fill` (C-16's background), `padding` (the page's margins, GN-2.13),
    `max_width` (Q2.32), `bindings`.
  - `icon` — `icon` (size, weight: GN-2.13, Q2.30, C-16), `opacity` (GN-2.13; Q2.31; K-KA-16†).
  - `title` — `text`, `opacity` (Q2.31, K-KA-16†).
  - `description` — `text`, `opacity` (Q2.31, K-KA-16†).
  - `action_area` — the actions' box: `padding`.
- **Widget values:** `icon_to_title_px`, `title_to_description_px`, `description_to_actions_px: Value<Px>` (GN-2.13;
  iOS's paddings, C-16; Q2.30); `alignment: Value<Alignment>` (C-16). The margins and the maximum width are the
  body's `padding` and `max_width` (one home, rename row 116).
- **Variants:** typed `loading` (GNOME's spinner variant, iOS's `.loading()`, KDE's LoadingPlaceholder), `search`
  (iOS's `.search()`, macOS's search variant); keyed `adwaita` `@compact` (GN-2.13); `kde-breeze` `@informational`,
  `@actionable` (Q2.31), `@ka_card_placeholder` (K-KA-16†).
- **Sizes and densities:** none (GNOME's `@compact` is a variant).
- **Flags:** none (no interaction states; its actions are `button` elements).
- **Exclusive groups:** none (no interaction states); precedence: state models (S17).
- **may_be_absent:** parts: none (a page with no actions has no `action_area` queried). fields: `body.max_width` (every
  preset but `kde-breeze`), `body.fill` (presets whose page has no background, K-KA-16†).
- **State-model source:** GNOME `_misc.scss` (`statuspage`); iOS `UIContentUnavailableConfiguration`; KDE
  `PlaceholderMessage.qml`, `LoadingPlaceholder.qml`, kirigami-addons `FormPlaceholderMessageDelegate.qml`; macOS
  `ContentUnavailableView`.
- **Today's fields:** none (`text_scale.display`, GN-2.13's stand-in, becomes a text-ramp reference).
- **Notes:** Nested controls: under `variant loading` the busy indicator is a `spinner` in `context status_page`
  (GNOME's, GN-1.232; Q1.121) or KDE's LoadingPlaceholder progress bar a `progress_bar` there (Q2.32); the actions are
  `button` elements in `context status_page.action_area`.

### stepper

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwSpinRow and GtkSpinButton (GN-2.05, GN-2.24), iOS's UIStepper (C-4), KDE's spin
  box (K2.3, Q2.1), macOS's `NSStepper` (MAC-N01), Windows' NumberBox (W2-01). Where the platform pairs it with a text
  field (spin box), the text is an `input` in `context stepper` (Nested controls); the increment and decrement are
  the stepper's own pieces.
- **Available:** every preset but `material-3` (`available = false`: no such control).
- **Parts:**
  - `body` — the root: `fill` (C-4's body; macOS 26's translucent body, MAC-N01), `stroke` (GNOME's entry frame,
    GN-2.24; KDE's line-edit frame, K2.3), `corners` (C-4), `padding` (Q2.1), `min_width`, `min_height` (C-4; macOS's
    control sizes, MAC-N01), `bindings` (Q2.1's View set).
  - `increment` *(element)*, `decrement` *(element)* — the two halves: `fill` (per state, C-4, W2-01), `stroke`
    (W2-01), `corners` (GNOME's end radii, GN-2.24; AdwSpinRow's circular buttons, GN-2.05), `padding` (GN-2.05's
    margins), `min_width`, `min_height` (GN-2.24, GN-2.05; KDE's arrow column, K2.3). `decrement_glyph` belongs to
    `decrement` and is queried with it. `increment_glyph` belongs to `increment` and is queried with it.
  - `increment_glyph`, `decrement_glyph` — `glyph` (C-4, MAC-N01, W2-01; KDE's arrow colour per state, K2.3),
    `transitions` (KDE's animated arrow hover, K2.3).
  - `separator` — between the buttons, or between the text and the buttons (GN-2.24, C-4, MAC-N01): `stroke` (the
    line, its start edge between buttons side by side, its top edge between stacked ones).
  - `popup` — Windows' compact spin popup, W2-01: `fill`, `stroke`, `shadow`.
  - `header` — Windows' header label, W2-01: `text`.
- **Widget values:** `button_layout: Value<StepperButtonLayout>`,
  `StepperButtonLayout { EndColumn, BothEnds, AboveAndBelow, SideBySide }` (a column at the end, both ends, stacked
  above and below, segments side by side): the arrangement the platform draws, its base the default and restated by
  rules under `vertical` and `@row` (the application's choices, which select the look; the value is the one layout
  each implies, so the connector reads one value); `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (durations: C-4, MAC-N01,
  GN-3.69†, K3.76†); `key_repeat_interval_ms: Value<Ms>` (the interval while an arrow key is held, K3.76†; absent on
  the presets that repeat keys at `repeat_interval`); `climb_after_repeats: Value<u32>` (GtkSpinButton grows its
  step by the application's climb rate after this many repeats, GN-3.69†); `field_gap_px: Value<Px>` (macOS's
  field-to-stepper gap,
  MAC-N01); `popup_offset_x_px`,
  `popup_offset_y_px: Value<Px>` (W2-01); `invalid_input: Value<InvalidInput>`, `InvalidInput { Revert, Keep }` (W2-01).
- **Variants:** typed `vertical` (orientation: GNOME's vertical spin button, GN-2.24; Windows' popup is not this);
  typed `flat` (KDE's frameless spin box, K2.3); keyed `adwaita` `@row` (AdwSpinRow's circular buttons, GN-2.05);
  `windows-11`
  `@inline`, `@compact` (W2-01).
- **Sizes and densities:** `macos-*`: its control sizes (MAC-N01).
- **Flags:** none of its own (a half at its limit is `disabled` on its element, C-4, MAC-N01); state flags used: hover
  (per arrow, K2.3), pressed, disabled, focused.
- **Exclusive groups:** `windows-11`: `[disabled, focused, hover]` on `body` (the text box's CommonStates),
  `[disabled, pressed, hover]` on `increment` and `decrement` (the spin buttons' CommonStates); others none (states
  combine); precedence: state models (S17).
- **may_be_absent:** parts: `popup`, `header` (every preset but `windows-11`); `separator` (every preset but
  `adwaita`, `ios-*`, `macos-*`: GN-2.24, C-4, MAC-N01). values: `popup_offset_x_px`, `popup_offset_y_px`,
  `invalid_input`
  (every preset but `windows-11`); `field_gap_px` (every preset but `macos-*`); `key_repeat_interval_ms` (every preset
  but
  `kde-breeze`); `climb_after_repeats` (every preset but `adwaita`); fields: `increment.stroke`, `decrement.stroke`
  (all but `windows-11`).
- **State-model source:** GNOME `_spin-button.scss`, `gtkspinbutton.c`; iOS `UIStepper` + `UIControl.State`; KDE
  Breeze `CC_SpinBox` branches (per-arrow hover) and `SpinBox.qml`; macOS `NSStepper` cell states and sizes; Windows
  NumberBox and its spin-button visual states.
- **Today's fields:** none.
- **Notes:** the spin box's text is `input @ context stepper` (GNOME's spin button text, KDE's line edit, WinUI's
  text box; `ios-*` and `macos-*` draw the stepper alone, and a text field beside one is an ordinary `input`).
  KDE repeats faster from the keyboard than from the mouse (K3.76†); S7.1's `InputKind` has no keyboard
  case to select that interval. Group 8 wrote `spin_button` (GN-3.69†, K3.76†).

### teaching_tip

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's TipKit tips (C-36†), macOS's TipKit tips (MAC-N36†), Windows' TeachingTip (W2-09),
  kirigami-addons' onboarding tour (K-KA-45†).
- **Available:** every preset but `adwaita`, `material-3` (`available = false`: no such control). TipKit is iOS 17+
  and macOS 14+ (C-36†, MAC-N36†); `kde-breeze` flavour `kirigami` only, and there only as kirigami-addons'
  `@ka_onboarding` (`requires_variants = ["@ka_onboarding"]`, K-KA-45†).
- **Parts:**
  - `body` — the root: `fill` (fill or material), `stroke`, `corners`, `shadow`, `padding`, `min_width`,
    `max_width` (W2-09; K-KA-45†), `min_height` (kirigami-addons' tip, K-KA-45†), `bindings` (kirigami-addons'
    Tooltip set, K-KA-45†).
  - `tail` — Windows' tail, W2-09; the popover tip's arrow, C-36†, MAC-N36†: `fill`, `stroke`, `min_width`,
    `min_height` (its size, W2-09), `margin` (its distance from the tip's corner, W2-09).
  - `top_highlight` — W2-09: `fill`.
  - `hero` — W2-09: `padding`.
  - `image` — C-36†, MAC-N36†: `icon`.
  - `title` — `text`.
  - `subtitle` — W2-09: `text`.
  - `message` — `text`.
  - `action_area` — the actions' box: `padding`.
  - `target_frame` — kirigami-addons' frame round the current control, K-KA-45†: `stroke`, `transitions` (moving and
    resizing to the next control).
  - `backdrop` — kirigami-addons' blur over everything else, K-KA-45†: `fill`.
- **Widget values:** `placement: Placement` (Windows' tail placement, strategy `WinUiTeachingTip`, W2-09;
  kirigami-addons' tip, strategy `KirigamiAddonsOnboardingTip`, K-KA-45†); `light_dismiss: Value<bool>` (W2-09);
  `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear`, `@expand`, `@collapse` (Windows' expand
  and contract animation, W2-09: the one name every widget uses).
- **Variants:** typed `inline`, `popover` (TipKit's inline and popover tips, iOS and macOS); keyed `windows-11`
  `@alternate_close`, `@hero_bottom` (W2-09); `kde-breeze` `@ka_onboarding` (K-KA-45†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (its close and action buttons are `button` elements).
- **Exclusive groups:** none (the close button's CommonStates on `windows-11` are `button`'s); precedence: state
  models (S17).
- **may_be_absent:** parts: `tail` (under `variant inline`; `kde-breeze`); `top_highlight`, `hero`, `subtitle` (every
  preset but `windows-11`); `title`, `image` (`kde-breeze`); `target_frame`, `backdrop` (every preset but
  `kde-breeze`). fields: `body.min_width`, `tail.margin` (every preset but `windows-11`). values: `light_dismiss`
  (every preset but `windows-11`).
- **State-model source:** iOS and macOS TipKit `TipView` styles; Windows TeachingTip visual states; KDE
  kirigami-addons `OnboardToolTip.qml`, `OnboardingOverlay.qml`, `OnboardingEffect.qml`.
- **Today's fields:** none.
- **Notes:** Nested controls: the close button is `button @ variant close context teaching_tip` (W2-09; its
  alternate placement `@alternate_close`), the actions `button` elements in `context teaching_tip.action_area`. KDE's
  What's This popup (K2.31†) is `context_help`, not this widget.

### time_picker

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's KTimeComboBox (K2.24†) and kirigami-addons' time popup (K-KA-23†), Material's time
  picker and time input (MAT-N17), Windows' TimePicker (W2-04).
- **Available:** every preset but `adwaita`, `macos-*`, `ios-*` (`available = false`: no row lists one).
- **Parts:**
  - `body` — the picker's box (the root): `padding`, `bindings`.
  - `field` — the picker's own trigger (Material's time picker has none: a dial or input in a dialog, MAT-N17):
    `fill`, `stroke`, `text` (per state, W2-04), `focus` (W2-04), `padding`, `min_width`, `max_width` (W2-04).
  - `field_segment`, in `field` — one segment of the trigger (W2-04): `text`, `padding`.
  - `segment_separator`, in `field` — W2-04's spacer lines: `stroke` (the line, its start edge).
  - `header` — W2-04: `text`.
  - `popup` — the picker's own flyout, holding its looping selectors (`picker_wheel` elements in `context
    time_picker.popup`, whose band and item height are `picker_wheel`'s; one name with `date_picker.popup`): `fill`,
    `stroke`, `corners`, `shadow`. Where the platform's popup is a `dialog` element (kirigami-addons', K-KA-23†), it
    is absent by rule and the dialog draws the box.
  - `dial` — MAT-N17's clock dial: `fill`, `corners`, `min_width`, `min_height`.
  - `dial_selector` — the selector handle, MAT-N17: `fill`, `corners`, `min_width`, `min_height`.
  - `dial_track` — the selector track, MAT-N17: `fill`, `min_width`.
  - `dial_center` — the selector centre, MAT-N17: `fill`, `corners`, `min_width`, `min_height`.
  - `dial_label` *(element)* — MAT-N17's dial labels; the one under the selector is `selected`: `text`.
  - `selector_cell` *(element)* — MAT-N17's time selector cells (the dial picker's hour and minute): `fill`, `stroke`,
    `corners`, `min_width`, `min_height`, `text`. The time input's hour and minute fields are `input` elements
    (Nested controls, below).
  - `period_selector` *(element)* — MAT-N17's AM/PM selector: `fill`, `stroke`, `corners`, `min_width`, `min_height`,
    `text`.
  - `headline` — MAT-N17: `text`.
  - `time_separator` — Material's separator, MAT-N17; kirigami-addons' ":", K-KA-23†: `text`.
- **Widget values:** `invalid_entry: Value<bool>` (an invalid entry answered with a message box, K2.24†, K1.224†);
  `presentations: Vec<PresentationTransition>`: events `@popup_show`, `@popup_hide` with the step's part `popup`
  (the popup's own occasion, as `date_picker`'s: its open animation, W2-04).
- **Variants:** typed `vertical` (Material's vertical layout; horizontal is the base, R25.14, MAT-N17); keyed
  `material-3` `@dial`, `@input`
  (MAT-N17); `kde-breeze` `@combo`,
  `@spin_box` (K2.24†), `@ka_time_popup` (K-KA-23†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own; state flags used: selected (`selector_cell`, `period_selector`, `dial_label`), hover,
  pressed, focused, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `field` (CommonStates); `material-3`:
  `[dragged, pressed, focus_visible, hover]` on `selector_cell` and `period_selector` (state layers); others none
  (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `dial`, `dial_selector`, `dial_track`, `dial_center`, `dial_label`, `selector_cell`,
  `period_selector`, `headline` (every preset but `material-3`); `popup` (every preset but `windows-11`; on
  `kde-breeze` the `dialog` element draws it, K-KA-23†), `field` (`material-3`), `field_segment`, `segment_separator`,
  `header` (all but `windows-11`); fields: `body.padding`, `field.min_width`, `field.max_width` (all but
  `windows-11`); values: `invalid_entry` (every preset but `kde-breeze`).
- **State-model source:** KDE `ktimecombobox.cpp`, kirigami-addons `TimePicker.qml`, `TimePopup.qml`; Material
  time-picker tokens; Windows TimePicker and DateTimePickerFlyout visual states.
- **Today's fields:** none.
- **Notes:** the looping selectors (W2-04) are `picker_wheel` elements in `context time_picker.popup`, and
  kirigami-addons' tumblers (K-KA-23†) `picker_wheel` elements in `context time_picker`; kirigami-addons' popup is a
  `dialog` element in `context time_picker` (`dialog @ variant @ka_dialog context time_picker`, K-KA-23†). Nested controls: the flyout's accept and
  dismiss buttons are `button @ variant @accept / @dismiss context time_picker` (W2-04); Material's time input
  fields, with their outline, label and supporting text, are `input` elements in `context time_picker.body[@input]`
  (MAT-N17).

### toast

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwToast (GN-2.12) and the legacy in-app notification (GN-2.50), KDE's Kirigami
  PassiveNotification (Q2.49–Q2.52), Material's snackbar (MAT-N22, MAT-X25).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*` (`available = false`: no such control);
  `kde-breeze` flavour `kirigami` only (PassiveNotification).
- **Parts:**
  - `body` — the root: `fill`, `stroke` (GNOME's outline under `cond contrast_high`, GN-2.12), `corners`, `shadow`,
    `padding`, `min_height` (Material's one- and two-line heights, MAT-N22), `max_width` (Q2.50), `opacity`,
    `bindings` (the Osd set on
    GNOME, GN-3.09, with a `Binding::Role` of `accent_standalone` to its dark-scheme derivation, GN-2.12; the
    Complementary set on KDE, Q2.49).
  - `title` — the message: `text`.
- **Widget values:** `durations: Vec<(Key, Value<Ms>)>` (GNOME's `default`, GN-2.12; KDE's `short`, `default`,
  `long`, Q2.51; Material's `short`, `long`, MAT-N22, MAT-X25); `natural_width_px: Value<Px>` (AdwToast's natural
  width, the width it takes where it fits, which no size limit states, GN-2.12); its maximum width is `body.max_width`
  (Q2.50) and its distance to the overlay's edges the placement's `edge_margin` (GN-2.12, Q2.50); `stack_gap_px: Value<Px>` (between
  stacked toasts, Q2.50); `content_gap_px: Value<Px>` (between the text and the action: GNOME's spacing, GN-2.12;
  Q2.50); `stack_max_height_px: Value<Px>` (Q2.50); `placement: Placement` (where the toast goes: Material's
  scaffold puts the snackbar by `MaterialScaffoldSlot(Snackbar)`, above the FAB and the bottom bar, MAT-X38†;
  AdwToastOverlay and Kirigami's notification manager at the bottom centre of their overlay, GN-2.12, Q2.50, a
  strategy S13 lacks, listed as a needed core change);
  `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear`
  (GNOME's show and hide with a scale, GN-2.12; KDE's add and remove, Q2.52; Material's enter and exit, MAT-N22),
  `@replace` (GN-2.12), `@displace` (KDE's displaced notifications, Q2.52).
- **Variants:** keyed `adwaita` `@app_notification` (the legacy in-app notification, GN-2.50).
- **Sizes and densities:** none; `cond contrast_high` adds GNOME's outline (GN-2.12).
- **Flags:** shared `multiline` (Material's two-line height, MAT-N22); state flags used: none other (its action and
  close buttons are `button` elements, with Material's state layers, MAT-N22).
- **Exclusive groups:** none (the buttons' state layers on `material-3` are `button`'s); precedence: state models
  (S17).
- **may_be_absent:** fields: `body.shadow` (where a preset draws none); `body.stroke` (`adwaita` outside
  `cond contrast_high`). values: `natural_width_px` (every preset but `adwaita`); `stack_gap_px`,
  `stack_max_height_px` (every
  preset but `kde-breeze`); `content_gap_px` (every preset but `kde-breeze`, `adwaita`); fields: `body.max_width` (all
  but `kde-breeze`).
- **State-model source:** GNOME `_misc.scss` (`toast`), `adw-toast-overlay.c`; KDE `PassiveNotificationsManager.qml`;
  Material snackbar tokens and `SnackbarHost.kt`.
- **Today's fields:** none.
- **Notes:** Nested controls: the action is `button @ context toast.body`, Material's close icon `button @ variant
  close context
  toast` (MAT-N22). Group 8 placed `body` (GN-3.09) and the durations (W3-41, MAT-X25; it wrote `duration_ms`) here. W3-41's
  Windows message duration is a reader read; `windows-11` declares no toast, so that read has no `toast` to state.

### toolbar_editor

*Written in the merge.*

- **Kind:** typed widget: the editor in which the user customises a toolbar's items. KDE (KXmlGui's `KEditToolBar`:
  two lists and move buttons, K-P4-10†), macOS (`NSToolbar`'s customization palette, MAC-P3-6†), iOS (the customizable
  navigation-bar items, P4-10†). Group 6 listed it as KDE's keyed component `toolbar_editor` and group 4 traced the
  macOS and iOS rows to `toolbar_customization`; three platforms draw it, so it is typed (R14) under group 6's name.
- **Available:** every preset but `adwaita`, `windows-11`, `material-3` (`available = false`: no such control;
  Windows' Customize Toolbar dialog is Win32, R1.3). iPadOS draws it from 16 (P4-10†). `kde-breeze` flavour `widgets`
  only (KEditToolBar).
- **Parts:**
  - `body` — the editor, the root (KDE's dialog content; macOS's customization palette, a sheet over the window):
    `padding`, `bindings`.
- **Widget values:** none: the items parting while one is dragged in and their re-layout as they are added, removed
  or moved to the overflow happen in the toolbar, whose events `@customize` and `@items_change` they are (one home,
  R20; MAC-P3-6†, P4-10†).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (the dragged items and the bar a drag hovers are the toolbar's and its buttons' elements,
  below).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** fields: `body.padding` (`ios-*`: iPadOS customises the navigation bar's items in place).
- **State-model source:** KDE `kedittoolbar.cpp`, `ktoolbar.cpp` with Breeze's item-view and button branches; AppKit's
  customization palette (`NSToolbar.allowsUserCustomization`, MAC-P3-6†, measured); UIKit's customisable
  navigation-bar items (`UINavigationItem.customizationIdentifier`, P4-10†, measured).
- **Today's fields:** none.
- **Notes:** Nested controls: KDE's two lists are `list` elements in `context toolbar_editor`, the four move buttons
  between them `button @ variant icon_only context toolbar_editor` (K-P4-10†). macOS's palette (MAC-P3-6†) is this
  widget: its item grid a `list @ variant grid context toolbar_editor`, its default set a `toolbar` in `context
  toolbar_editor`, its display-mode choice a `combo_box` there and its Done a `button @ variant done` there; iPadOS
  customises in place (P4-10†). The drop feedback in the window's bar is the toolbar's own elements:
  `button.* @ {dragged} context toolbar.bar` and `toolbar.bar.* @ {drop_target}` (MAC-P3-6†, P4-10†). The toolbar's
  context menu is a `menu` (K-P4-10†).

### view_switcher

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwViewSwitcher, ViewSwitcherTitle and ViewSwitcherSidebar (GN-2.15),
  AdwInlineViewSwitcher (GN-2.16) and GtkStackSwitcher (GN-2.33); Windows' SelectorBar (W2-27), Pivot (W2-32) and the
  NavigationView in Top mode (W2-11, W1-094's Top display mode). GNOME's ViewSwitcherBar, the bottom bar of a narrow
  window, is `bottom_tab_bar` (GN-2.15).
- **Available:** every preset but `kde-breeze`, `macos-*`, `ios-*`, `material-3` (`available = false`: no row lists
  one).
- **Parts:**
  - `bar` — the root: `fill` (the top NavigationView's pane fill, W2-11), `padding` (GNOME's bar padding, GN-2.15),
    `min_height` (the top NavigationView's pane height, W2-11).
  - `title` — the Pivot's title above its headers (W2-32): `text`, `padding`.
  - `item` *(element)* — `fill`, `stroke` (per state, W2-27), `min_width` (GN-2.15, GN-2.33), `corners` (GtkStackSwitcher's linked buttons, the inner
    corners squared by position, GN-2.33), `padding` (GN-2.15's narrow and wide buttons; W2-27; Pivot's header margins,
    W2-32), `min_height` (GN-2.15), `focus` (SelectorBar's focus margin, `SelectorBarItemFocusVisualMargin`, W3-20).
    `item_icon`, `item_label`, `indicator` belong to `item` and are queried with it.
  - `item_icon` — `icon`.
  - `item_label` — `text` (GN-2.15's narrow labels; Pivot's header font, W2-32; the contextual tab's accent text,
    W-RV-3†), `padding` (GN-2.16).
  - `indicator` — SelectorBar's selection pill, W2-27; Pivot's underline, W2-32; the top NavigationView's bottom
    selection indicator, W2-11: `fill`, `corners`, `min_width`, `min_height`.
  - `item_separator` — W2-11: `stroke` (the line, its start edge).
- **Widget values:** `item_gap_px: Value<Px>`; `icon_text_gap_px: Value<Px>` (AdwInlineViewSwitcher's image-text
  spacing, GN-2.16); the items' minimum widths per policy are `item.min_width` rules by the variants (GNOME's minimum
  natural widths, GN-2.15; text buttons' minimum width under `@stack`, GN-2.33); `drag_hover_delay_ms: Value<Ms>` (GNOME's
  drag-hover switch, GN-2.15, and expand, GN-2.16); `breakpoints: Vec<Breakpoint>` (Shared value types:
  libadwaita's thresholds setting `narrow` and `@wide`, GN-1.197); `presentations: Vec<PresentationTransition>`:
  events `@indicator_move` with `part: indicator` (SelectorBar's pill slide, W2-27; Pivot's header slide, W2-32).
- **Variants:** typed `narrow` (set by `breakpoints`, as `dialog`'s); keyed `adwaita` `@wide` (likewise), `@title`,
  `@sidebar`, `@inline`, `@stack` (GN-2.15, GN-2.16, GN-2.33); `windows-11` `@selector_bar` (W2-27), `@pivot` (W2-32),
  `@navigation_top` (W2-11, W1-094), on `item` `@contextual` (a tabbed command bar's contextual tab, W-RV-3†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (the needs-attention dot is a `badge`, variant `attention`, in
  `context view_switcher.item`, GN-1.41, GN-2.33); state flags used: selected, hover, pressed, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (CommonStates), `selected` combining;
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `indicator`, `item_separator` (`adwaita`), `title` (every preset but `windows-11`).
  values: `drag_hover_delay_ms` (every preset but `adwaita`), `icon_text_gap_px` (all but `adwaita`); fields: `bar.fill` (all but `windows-11`),
  `item.focus` (where focus is drawn elsewhere).
- **State-model source:** GNOME `_view-switcher.scss`, `_toggle-group.scss`, `_buttons.scss` (`stackswitcher`);
  Windows SelectorBarItem, PivotHeaderItem, NavigationViewItem (top) visual states.
- **Today's fields:** none (`segmented_control.*`, `tab.*` stand-ins stay theirs).
- **Notes:** counters and the needs-attention dot are `badge` elements in `context view_switcher.item` (GN-2.15,
  GN-1.41, GN-2.33). Nested controls: Pivot's header buttons are `button
  @ variant previous / next context view_switcher` (W2-32), the top NavigationView's "More" `button @ variant
  overflow context view_switcher` (W2-11). Windows' top NavigationView is this widget's
  `@navigation_top`; `sidebar` keeps the left display modes. Group 4 added `breakpoints` (GN-1.197).

### wizard

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkAssistant (GN-RL01†, GN-2.51), KDE's QWizard in its classic style (K2.22).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: no such
  control; Windows' Aero wizard is Win32, R1.3); `kde-breeze` flavour `widgets` only (QWizard).
- **Parts:**
  - `body` — the wizard's content, the root: `padding`, `bindings`.
  - `header` — the title and subtitle area, K2.22: `padding`.
  - `title`, in `header` — `text`.
  - `subtitle`, in `header` — `text`.
  - `banner`, in `header` — K2.22: `fill`.
  - `step_list` — GNOME's sidebar, GN-2.51: `fill`, `bindings` (its sidebar colours).
  - `step` *(element: GN-2.51)*, in `step_list` — `fill` (the highlighted step), `corners`, `padding`, `text`; the
    step list's binding reaches it (S2.11).
  - `page_frame` — `padding`.
  - `action_area` — GN-RL01†: `padding` (its margins).
- **Widget values:** `button_order: Vec<Key>` (GtkAssistant's `close`, `cancel`, `last`, `back`, `forward`, `apply`,
  GN-RL01†); `button_gap_px: Value<Px>`; `buttons_equal_width: Value<bool>` (GN-RL01†). Where the buttons go is the
  application's choice, the keyed `@header_bar` (one home: no value restates it).
- **Variants:** keyed `adwaita` `@header_bar` (the buttons packed in the header bar, Back and Cancel at its start and
  the others at its end; the connector sets the window's `@no_title_buttons`, GN-RL01†). QWizard's classic style is
  the only one Breeze draws (K2.22), so it is `kde-breeze`'s base, not a variant.
- **Sizes and densities:** none (no row states a size).
- **Flags:** shared `current` (the highlighted step, GN-2.51); state flags used: none other.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `step_list`, `step` (`kde-breeze`); `banner`, `subtitle` (`adwaita`); `action_area`
  (`adwaita` under `@header_bar`, by rule).
- **State-model source:** GNOME `gtkassistant.ui`, `gtkassistant.c`, `_deprecated.scss`; KDE `qwizard.cpp` classic
  style.
- **Today's fields:** none.
- **Notes:** Nested controls: the navigation buttons are `button` elements in `context wizard.action_area` (the
  default layout) or in `context window.title_bar` under `@header_bar`, the default one `button @ variant primary
  context any wizard variant @header_bar` (GN-RL01†).

## Keyed components

One-platform kinds (R14, S5.1): a `Component` the preset declares (S10), with its `key`, its `parts` and its
`values`, each value a `Value<WidgetValue>` of the type its declaration names (`values = { <name> = "<type>" }`,
S10). A component follows the entry format's rules for parts (its first part a container root, each part's fields
listed) and the nested-control rule (a control of a typed kind inside it is that widget in `context
component.<key>`). A component's variants and flags are keyed (S10's `[declare.component.<key>]`), even where a
typed widget has a variant of the same meaning (`title_widget`'s `@info`, `@warning`, `@error`, `fab_menu`'s
`@primary`): the typed `Variant` enum is the typed widgets'. Group 6 wrote its components as table rows, group 7 as
sections; both are kept, by platform. Group 6's `toolbar_editor` is a typed widget after the merge (three platforms
draw it), and so is `context_help` (rename row 83); the media output button is `button`'s typed `route` (rename row
82), and macOS's token field is the structure Windows' has (an `input` holding a flow `layout` of `chip` elements,
rename row 52). A `kde-breeze` component names the flavour that draws it (S10's `[declare.flavor.<f>.component.<key>]`
form, as for widgets: the other flavour declares `available = false`): `tool_box`, `dock_widget`, `mdi_sub_window`,
`title_widget`, `multi_tab_bar`, `command_bar`, `char_select`, `lcd_number`, `led` and `page_view` are flavour
`widgets` only (Qt Widgets); `delay_button` (a QML DelayButton) and `album_viewer` (kirigami-addons) are flavour
`kirigami` only.

### GNOME

*From group 6:*

| Component | Platform | Parts | Values | Rows |
|---|---|---|---|---|
| `emoji_chooser` | GNOME | `body` (the root: `fill`, `padding`, `bindings`), `cell` (*(element)*: `fill`, `corners`, `text`), `completion_row` (*(element)*: `fill`, `text`, `padding`); its section buttons are `button` elements in `context component.emoji_chooser` | — | GN-2.43 |
| `tab_overview` | GNOME | `grid` (the root: `fill`, `padding`), `thumbnail` (*(element)*: `fill`, `corners`, `shadow`, `stroke`, `margin`, `transitions`; variant `@pinned`, a pinned tab's thumbnail with its own colours, GN-2.35), `drop_target` (`fill`, `stroke`, `corners`); a thumbnail's needs-attention dot is `badge @ variant attention context component.tab_overview.thumbnail` (one structure for the dot, Nested controls); a thumbnail's close button is `button @ variant close context component.tab_overview`, a pinned one's unpin button `button @ variant @unpin` there, the new-tab button `button @ variant add` there | `grid_gap_px: Value<Px>`, `columns_min`, `columns_max: Value<u32>` (numbers), `thumbnail_width_min_px`, `thumbnail_width_max_px: Value<Px>`, `drag_hover_delay_ms: Value<Ms>` (a drag hovering a thumbnail switches to it, GN-3.51), `presentations: Vec<PresentationTransition>` (queried with the overview: `Appear`, `Disappear`, the overview shown and hidden; queried with a thumbnail: `Move`, a thumbnail moved to its new place, `@thumbnail_fade`) | GN-2.35, GN-3.51 |

#### component.dock_frame_drop (gnome)

*From group 7.*

- **Kind:** keyed component (GNOME's libpanel; R1.2 in). Rows GN-SF14†, GN-SF15†, GN-SF16†. No other platform draws a
  dock frame (cross-check: macOS, iOS, KDE, Windows, Material all 3, "G19: KDDockWidgets' artwork is that library's
  own", KDE's QMainWindow gap indicator K-SF18† is Qt Widgets' and is another row).
- **Parts:**
  - `frame` — the `PanelFrame` itself while a drop is over it: `shadow` (its own `:drop(active)` shadow is switched
    off here, so GN-3.66†'s catch-all rule does not apply: a rule `@ {drop_target}` states its `shadow` `Absent`),
    `padding` (the drop overlay's margin from the frame's edge, GN-SF14†).
  - `drop_overlay` *(element; states: drop_target)* — the overlay shown over the frame while a page is dragged over it:
    `fill`, `corners`, `stroke` (the ring inside `@ {drop_target}`), `transitions`.
  - `drop_position_button` *(element; states: drop_target, the button under the drop)* — one of five buttons, its
    identity a keyed variant `@top`, `@right`, `@bottom`, `@left`, `@center`: `fill`, `corners` (the centre one
    circular, the others with a corner radius: rules by identity), `padding`, `glyph` (`ThemeIcon` per identity),
    `shadow`.
  - `switcher_indicator` — the insertion mark in a frame switcher where the dragged page would land: `fill`.
- **Widget values:** `button_gap_px: Value<Px>` (the gap in the grid of five buttons, which sit centred on the frame).
- **Variants:** none typed; keyed: the identities of `drop_position_button`, `@top`, `@right`, `@bottom`, `@left`,
  `@center`. **Sizes/densities:** none.
- **Flags:** state flag `drop_target`. No widget-specific flag. **Exclusive groups:** none.
- **may_be_absent:** fields: `frame.shadow` (`@ {drop_target}`, GN-SF14†). The component is declared by `adwaita`
  alone.
- **Transitions:** `drop_overlay` has property transitions on fill and stroke (GN-SF14† states background and border
  transitions) in its `PartStyle.transitions`.
- **State-model source:** libadwaita-family stylesheet selectors by file: libpanel `stylesheet.css`
  (`paneldropcontrols`, `:drop(active)`), per GN-SF14†..GN-SF16† (P).
- **Today's fields:** none (no dock widget in the model).

### KDE

*From group 6:*

| Component | Platform | Parts | Values | Rows |
|---|---|---|---|---|
| `lcd_number` | KDE | `frame` (the root: `stroke`, `bindings` (the palette colours its digits take); keyed variants `@outline`, `@filled`, `@flat`, QLCDNumber's segment style), `segment` (`fill`, `stroke`) | — | K2.6 |
| `led` | KDE | `body` (the root; keyed variants `@round`, `@rectangular`, `@raised`, `@sunken`, `@flat`; flag `@on`: `fill`, `stroke`, `corners`, `bindings`) | — | K2.18 |
| `page_view` | KDE | `body` (the root: `padding`, `bindings`), `title_header` (`text`, `padding`, `fill`), `header_separator` (`stroke`); its icon list is a `list` and its search field an `input` in `context component.page_view` (Qt Widgets' search fields are line edits; `search_field` is `kirigami` only) | `margins_px: Value<Px>` | K2.14 |
| `delay_button` | KDE | `body` (the root, the button: `fill`, `stroke`, `corners`, `padding`, `focus`, `text`), `progress_overlay` (`fill`, `corners`, `opacity`, `transitions`; `data = { progress_overlay = ["progress"] }`) | `delay_ms: Value<Ms>` (the delay before the button activates, QML DelayButton's `delay`: the platform's default, which an application layer restates) | Q2.3 |

The delay button's fill transition (Q2.3) runs for the delay times the remaining progress while pressed, and a
fraction of the progress while released: `progress_overlay.transitions` with the duration `= mul(values.delay,
sub(1, element.progress))` under `{pressed}` and `= mul(values.delay, mul(<fraction>, element.progress))` under
`!{pressed}` (S2.2's `values.<name>`, ADD-B12; the fraction is the source's, value phase).

#### component.tool_box (kde)

*From group 7.*

- **Kind:** keyed component (QToolBox; Qt Widgets). Row K2.9. Cross-check: GNOME 3 (expanders are the nearest),
  Windows 3, macOS 3, iOS 3, Material 3.
- **Parts:**
  - `body` — the root, the box holding the pages: `padding`, `bindings`.
  - `page_button` *(element; states hover, selected (the open page, the current choice among the pages: QToolBox's
    checked tab maps to it, S7.2), pressed, disabled, focus_visible)* — the tab row of one page: `min_width`
    (`ToolBox_TabMinWidth`).
  - `page_icon`, in `page_button` — `icon`.
  - `page_label`, in `page_button` — `text`.
  - `selected_outline`, in `page_button` — the tab-shaped outline in the focus colour, drawn `@ {selected}` (absent by
    rule otherwise): `stroke`, `corners` (per-corner radii).
  - `hover_outline`, in `page_button` — the same `@ {hover}`: `stroke`, `corners`, `transitions` (the hover fade).
    The parts in `page_button` belong to it and are queried with it.
  - `page_area` — the content frame of the open page: `padding`.
- **Widget values:** `item_gap_px: Value<Px>` (`ToolBox_TabItemSpacing`), `tab_margin_width_px: Value<Px>`
  (`ToolBox_TabMarginWidth`); the tabs' minimum width is `page_button.min_width` alone (one home, R20).
- **Variants/sizes:** none. **Flags:** state flags only. **Exclusive groups:** none (Breeze's branches combine
  states).
- **Transitions:** the hover fade is a property transition on the outline stroke (`breezetoolboxengine.cpp`), its
  duration through the speed factor (`= mul(token.motion.durations.<key>, token.motion.speed_factor)`, S4.1).
- **may_be_absent:** `selected_outline` (by rule `!{selected}`), `hover_outline` (by rule `!{hover}`).
- **State-model source:** Breeze's state branches (`st:7385-7510`, `st:2772-2804`, `hp:743-782`) and its animation
  engine (K2.9).
- **Today's fields:** none.

#### component.dock_widget (kde)

*From group 7.*

- **Kind:** keyed component (QDockWidget title; Qt Widgets). Row K2.10. Cross-check: only inspector/utility panel
  names it (K2.10 is KDE's tool panel); GNOME, macOS, iOS, Material and Windows 3.
- **Parts:**
  - `title_bar` — the root, a container: `padding`.
  - `title` — `text` (elided as Breeze elides it).
  - `separator` — the line under the title: `stroke` (the line, its top edge).
  - `frame` — the optional frame around a docked widget: `stroke`; absent by a rule
    `!cond system_setting:dock_widget_draw_frame` (breezerc `[Style] DockWidgetDrawFrame`, read by the reader,
    `reader:kde:breezerc.Style.DockWidgetDrawFrame`; a read switch is a condition, ADD-A04). The float and close buttons
    (frameless glyph buttons; glyph `ThemeIcon` or the standard pixmap `SP_DockWidgetCloseButton`; states hover,
    pressed, disabled) are `button @ variant @float / close context component.dock_widget`.
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `@gap_open` (the main window's dock layout
  sliding a gap open
  under a dragged dock widget, `AnimatedDocks`, K-SF18†).
- **Variants/sizes/flags:** none beyond state flags. **Exclusive groups:** none (Breeze's branches combine states).
- **may_be_absent:** `frame` (outside `cond system_setting:dock_widget_draw_frame`, K2.10), `separator`.
- **State-model source:** Breeze's branches (`st:7512-7586`, `st:968-969`, `st:2125-2131`); Qt's
  `qmainwindowlayout.cpp` gap indicator (K-SF18†).
- **Today's fields:** none.

#### component.mdi_sub_window (kde)

*From group 7.*

- **Kind:** keyed component (QMdiSubWindow; Qt Widgets). Row K2.11. Windows has no MDI control (3).
- **Parts:**
  - `frame` — the root: `stroke` (the outline; the inactive outline `@ !{@active}`), `shadow` (the MDI window
    shadow, `breezemdiwindowshadow.cpp`), `bindings` (the `[WM]` colour group, K2.11).
  - `title_bar` — `fill` (from the `[WM]` colour group), `corners` (top-rounded).
  - `title` — `text`.
  - `active_separator` — the line under the active title bar in the highlight colour: `stroke` (the line, its top
    edge). The caption buttons are `button` elements in `context component.mdi_sub_window` (Nested controls; only
    the window's own caption buttons are the core's exception, S5.2), their identities `button`'s typed `minimize`,
    `maximize`, `restore`, `close`, their glyphs `button.icon.glyph` there; the MDI window shadow is `frame.shadow`
    (one home, R20).
- **Widget values:** `title_margin_width_px: Value<Px>` (`TitleBar_MarginWidth`).
- **Variants/sizes:** none. **Flags:** `@active` (the sub-window is the active one: per element, so it is a widget
  flag and not the window
  condition). **Exclusive groups:** none.
- **may_be_absent:** `active_separator` (absent by a rule `!{@active}`), `frame.shadow`.
- **State-model source:** Breeze's branches (`st:8241-8366`, `hp:1581-1645`, `breezemdiwindowshadow.cpp:39-60`).
- **Today's fields:** none. The `[WM]` colours are colour roles (the colour-role group; row K2.11 only names that this
  component binds them).

#### component.title_widget (kde)

*From group 7.*

- **Kind:** keyed component (KTitleWidget, KWidgetsAddons; R1.2 in). Row K2.15. Cross-check: GNOME 3 (AdwStatusPage
  and group headers are different), Windows 3, macOS 3, iOS 4, Material 4.
- **Parts:**
  - `container` — the root: `padding`.
  - `icon` — `icon`.
  - `title` — `text` (one per level).
  - `comment` — `text` (coloured by message type).
- **Widget values:** none (alignment of icon and text is the layout, `layout.*`).
- **Variants:** keyed text level `@level_1` .. per the keyed text styles of K1.270 (the declared levels); keyed
  message type `@info`, `@positive`, `@warning`, `@error`, `@plain` for `comment` (the colour is a rule per variant
  referring to the surface's status roles).
- **Sizes:** none. **Flags/exclusive groups:** none. **may_be_absent:** none (a widget given no icon or comment has
  none queried, S6.3).
- **State-model source:** the widget's own code (`kwa ktitlewidget.cpp:30-80`): message type enumeration.
- **Today's fields:** `text_scale` (partial): the level fonts map to keyed text-ramp entries.

#### component.multi_tab_bar (kde)

*From group 7.*

- **Kind:** keyed component (KMultiTabBar, KWidgetsAddons; R1.2 in). Row K2.16. Cross-check: GNOME 3 (GtkNotebook with
  side tabs is the typed tab), macOS 3, iOS 3, Windows 4, Material 4.
- **Parts:**
  - `bar` — the root: `padding`.
  - `tab_button` *(element; vertical or horizontal; states hover, pressed, checked, disabled)* — `fill`, `padding`.
  - `tab_icon`, in `tab_button` — `icon`; it belongs to `tab_button` and is queried with it.
  - `tab_label`, in `tab_button` — `text`; likewise.
  - `separator` — the line on the content side of the bar: `stroke` (the line, on the bar's content edge).
- **Widget values:** `item_gap_px: Value<Px>`.
- **Variants:** keyed bar edge `@left`, `@right`, `@top`, `@bottom` (the `KMultiTabBar::Position` values; the list is
  confirmed against `kmultitabbar.cpp` in the value phase: K2.16 marks its source lines "to verify").
- **Sizes/flags:** state flags only. **Exclusive groups:** none (Breeze's branches combine states).
- **may_be_absent:** none (an icon-only tab's label is not queried, S6.3).
- **State-model source:** Breeze's `st:1400-1422` and the widget's code.
- **Today's fields:** none.

#### component.command_bar (kde)

*From group 7.*

- **Kind:** keyed component (KCommandBar, KConfigWidgets; R1.2 in). Row K-RL01†. Cross-check: Windows 3 (`CommandBar`
  there is the toolbar), macOS 2 (Help-menu search, other row), iOS, GNOME, Material: no palette. Its keycaps are the
  typed `keycap`.
- **Parts:**
  - `frame` — the root: sunken styled-panel frame, frame forced on: `fill`, `stroke`, `padding`, `min_width`,
    `min_height`, `shadow` (the drop shadow, a `ShadowLayer` coloured by the palette's `Dark` role,
    `role.@shade_dark`). Nested controls: the frameless search field is `input @ variant flat context
    component.command_bar` (its leading `search` icon `input.icon @ variant leading`, its clear button `button @
    variant clear_field`); the tree of actions a `list` there, the cell holding the keycaps `list.cell`; the keys
    `keycap` elements.
- **Widget values:** `keycap_hmargin_extra_px: Value<Px>` (added per side to the focus frame margin: the source's
  constant, value phase; ADD-A44); `placement: Placement` (strategy `KCommandBar`: top-centre of the window's content
  space, its top offset the placement's `edge_margin.top`, ADD-A21). The frame's preferred size, the content space's
  width and height divided by the source's divisors, is `frame.min_width` and `frame.min_height` as expressions over
  `element.window_width` and `element.window_height` (one home, rename row 116; the root's inputs, S8).
- **Variants/sizes/flags:** none (the action rows' states are `list`'s: hover, selected).
- **may_be_absent:** none.
- **State-model source:** the widget's code (`kcommandbar.cpp:158-168`, `:299-387`, `:417-440`, `:473-476`,
  `:572-605`, `:712-731`).
- **Today's fields:** none.

#### component.char_select (kde)

*From group 7.*

- **Kind:** keyed component (KCharSelect, KWidgetsAddons; R1.2 in). Row K-RL04†. Cross-check: macOS 2 and iOS 3 (the
  Character Viewer and the keyboard are system-drawn, R1.1), Windows 3, GNOME 3.
- **Parts:**
  - `table` — the root, the view: `fill` (from the `View` surface's background), `bindings`.
  - `cell` *(element; states hover, selected, focus_visible)* — one character, centred: `fill`, `text`, `focus`,
    `min_width`, `min_height` (its size by the expressions below, its data declared `data = { cell = ["char_width",
    "font_x_height", "font_height"] }`, measured through the toolkit's font metrics, S8); the missing-glyph cell is
    `cell` under the widget flag `@glyph_missing`.
- **Widget values:** none of its own: the cell's size is `cell.min_width` and `cell.min_height`, expressions of
  `kcharselect.cpp:255-310`. Width:
  `= add(max(max(min(element.char_width, mul(element.font_x_height, <max factor>)), mul(element.font_x_height, <min factor>)), element.font_height), mul(2, <text margin>))`,
  with `element.char_width` the widest printable glyph of the table as the toolkit measures it,
  `element.font_x_height` the font's x-height, `element.font_height` its height (ascent plus descent; `line_height`
  is the line spacing), and the text margin the style's focus-frame margin plus the source's constant
  (`token.params.layout`). Height (outside Windows):
  `= max(mul(element.font_x_height, <factor>), max(<minimum>, add(element.font_height, <constant>)))`. The factors and
  constants are the source's, value phase.
- **Variants/sizes:** none. **Flags:** `@glyph_missing` (the font lacks the glyph or the cell is empty: the cell is
  filled with the `Button` surface's background, else the `View`'s). **Exclusive groups:** none.
- **may_be_absent:** none.
- **State-model source:** the widget's own paint code (`kcharselect.cpp:115-131`, `:255-310`, `:1089-1121`).
- **Today's fields:** none.

#### component.album_viewer (kde)

*From the kirigami-addons rows (K-KA-34†), traced in the merge.*

- **Kind:** keyed component (kirigami-addons' `AlbumMaximizeComponent`, R1.2 in; K-KA-34†): a full-window media viewer
  above everything, on its own overlay layer. No other platform's file lists an in-application media viewer.
- **Parts:**
  - `body` — the root: the viewer's layer: `bindings`.
  - `scrim` — the dim behind it: `fill`.
  - `header` — the header bar in the alternate background: `fill`, `padding`.
  - `header_separator` — `stroke` (the line, its top edge).
  - `title`, in `header` — its own text style: `text`.
  - `subtitle`, in `header` — `text`.
  - `footer` — the caption footer, height-limited: `fill`, `padding`, `max_height`.
  - `footer_separator`, in `footer` — `stroke` (the line, its top edge).
  - `image` — the shown image with its inset: `transform`, `transitions` (its zoom and rotation animate, group
    `Transform`). Its close button is `button @ variant close context component.album_viewer`, its actions `button`
    elements; previous and next are `button @ variants {previous,@ka_floating}` and `button @ variants {next,@ka_floating}` in context
    (the typed identities, Nested controls); the loading indicator a `spinner` in
    context; video controls the `media_controls` variant `@ka_album` (K-KA-35†).
- **Widget values:** `image_inset_px: Value<Px>`.
- **Variants, sizes, flags, exclusive groups:** none.
- **may_be_absent:** none (an image given no subtitle or caption has none queried, S6.3).
- **State-model source:** the component's QML as K-KA-34† cites it (`ka/components/AbstractMaximizeComponent.qml`,
  `ka/components/AlbumMaximizeComponent.qml`, `ka/components/ImageMaximizeDelegate.qml`).
- **Today's fields:** none.

### macOS

#### component.rule_editor (macos)

*From group 7.*

- **Kind:** keyed component (`NSRuleEditor`, `NSPredicateEditor`). Row MAC-N09. Cross-check: all other platforms 3
  (macOS table only).
- **Parts:**
  - `body` — the root: `fill`, `stroke`, `padding`. Nested controls: the criteria menus are `combo_box` elements
    (their base pop-up look), the value field an `input`, the round plus and minus buttons `button @ variant
    circular`, each in `context component.rule_editor`.
  - `row` *(element)* — a criteria row, with alternating stripe fills by position: `fill`, `padding`, `min_height`.
  - `nested_row_area` — the indented area of nested rows: `fill`, `padding`.
- **Widget values:** `indent_per_level_px: Value<Px>`.
- **Variants:** none. **Sizes:** `ControlSize` (NSRuleEditor rows follow row-height styles; per AppKit enum).
  **Flags:** state flags only. **Exclusive groups:** none.
- **may_be_absent:** none (the last row's remove button is disabled, not absent).
- **Position:** the stripes are rules `@ pos parity odd` / `@ pos parity even` on `row` (ADD-B06).
- **State-model source:** AppKit control states and `NSRuleEditor` row and nesting mode enums (MAC-N09: M).
- **Today's fields:** none.

### Windows

#### component.semantic_zoom (windows)

*From group 7.*

- **Kind:** keyed component (WinUI `SemanticZoom`). Row W2-34. Cross-check: all other platforms 3, iOS 2 maps the word
  "zoom" to the window zoom button (another row).
- **Parts:**
  - `body` — the root, the zoomable view's box: `bindings`.
  The view's own zoom-out control is `button @ variant @zoom_out context component.semantic_zoom` (Nested
    controls: a button to the user; its fill, stroke, glyph and text per state are the button's there, W2-34).
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `@zoom` (zoom in and out between the views),
  its kind the template's animation as a `PresentationKind` (`Keyed { … }` with a declared key where it is none of the
  typed kinds).
- **Variants/sizes/flags:** none. **Exclusive groups:** none of its own (the zoom-out button's CommonStates are
  `button`'s).
- **may_be_absent:** none.
- **State-model source:** WinUI visual state groups by template (`dxaml generic.xaml`).
- **Today's fields:** none.

#### component.key_tip (windows)

*From group 7.*

- **Kind:** keyed component (WinUI `KeyTip`, access-key badge). Row W2-37. Cross-check: GNOME, KDE, macOS, iOS,
  Material 3 (all underline mnemonics or draw none).
- **Parts:**
  - `badge` — the root, the access-key badge (W2-37): `fill`, `stroke` (its paint and thickness), `corners`,
    `padding`.
  - `text` — the badge's key letters: `text` (text style, foreground; one home for the letters' style, R20).
- **Widget values:** `placement: Placement` (strategy `WinUiKeyTip`; its offset from the target is the placement's
  `gap`, ADD-A21).
- **Variants/sizes/flags:** none. High contrast and theme dictionaries are rules under the conditions.
- **Appear/Disappear:** presentation transitions are not stated by W2-37; none.
- **may_be_absent:** none.
- **State-model source:** WinUI theme dictionaries per theme and high-contrast (`generic.xaml:1010-1012`).
- **Today's fields:** `tooltip.*` (nearest, not the same).

#### component.tabbed_command_bar (windows, Windows Community Toolkit)

*From group 7.*

- **Kind:** keyed component (CTK `TabbedCommandBar`; R1.2 in). Row W-RV-3†. Other platforms: no ribbon (all 3 or
  absent).
- **Parts:**
  - `body` — the root: `padding`, `bindings`. Nested controls: the tab row is a `view_switcher` variant
    `@navigation_top` in `context component.tabbed_command_bar` (its look W2-11; a contextual tab its `item` under
    the keyed `@contextual`, text colour in accent; states rest, hover, pressed, disabled, selected,
    `{selected,hover}`, `{selected,pressed}`); the command panel a `toolbar` there (its buttons label to the right of
    their icons, `button.values.label_position` in context).
  - `content_border` — the border holding the tab's command bar: `fill`, `stroke` (its thickness is the stroke's
    width), `corners`; under `cond contrast_high` the window colour and a transparent stroke.
  - `element_container` — each command bar's element container: `padding` (its margin, W-RV-3†).
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `@tab_change`, kind `Keyed { key: pop_in }`
  (WinUI's `PopInThemeAnimation` on the content).
- **Variants:** none of its own (the contextual tab is `view_switcher`'s keyed `@contextual`). **Sizes/flags:** none.
  **Exclusive groups:** none of its own (the tab items' `[disabled, pressed, hover]` are `view_switcher`'s).
- **may_be_absent:** none.
- **State-model source:** WinUI visual states by template (`TabbedCommandBarItem.xaml:9-24`, `:83-89`).
- **Today's fields:** `toolbar.*`, `tab.*` (neither has a tab row joined to a command panel).

#### component.casting_picker (windows)

*From group 7.*

- **Kind:** keyed component (`CastingDevicePicker`; the application sets its `Appearance`, the system draws the list:
  R1.1 "configures for the system to draw"). Row W-P4-18†. The button that opens it is `button @ variant route`. The
  list is Windows' alone (macOS's device popover is the system's, R1.1), so it stays keyed.
- **Parts:**
  - `picker` — the root, the flyout surface (W-P4-18†: fill, radius, shadow to measure; the appearance's background):
    `fill`, `corners`, `shadow`, `padding` (the list's inset).
  - `title` — the picker's title (the appearance's foreground): `text`.
  - `item` *(element)* — a casting device; states hover, selected; the selected item is `item` `@ {selected}`
    (W-P4-18†: item height to measure; the appearance's background, foreground and their selected twins): `fill`,
    `text`, `min_height`.
- **Widget values:** `placement: Placement` (strategy `WinUiFlyout(…)`: `Show(Rect, Placement)`); the six appearance
  colours are bases of the parts (accent, background, foreground and their selected twins).
- **Variants/sizes/flags:** none. **may_be_absent:** none (a picker given no title has none queried, S6.3).
- **Reads:** `reader:windows:DevicePickerAppearance getters` (R, to verify whether they return the system defaults).
- **State-model source:** WinUI visual states; the `Appearance` object.
- **Today's fields:** none.

### iOS

#### component.context_menu_preview (ios)

*From group 7.*

- **Kind:** keyed component (UIKit `UIContextMenuInteraction` preview). Row C-13. Cross-check: macOS 3, Windows 3, KDE
  3, GNOME 3, Material 3.
- **Parts:**
  - `body` — the presentation over the window (the root): `bindings`.
  - `preview` — the lifted view: `transform` (scale), `corners` (clipping path), `shadow`.
  - `scrim` — the blur and dim over everything else (one name with the other scrims, rename row 16): `fill` (a
    `Material` paint of the platform plus a tint layer), `opacity` (it fades in on `Appear` and out on `Disappear`).
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear` and `Disappear` (the preview
  lifting and dismissing; kinds `Scale`, `Fade`; timing a `Spring`); `placement: Placement` (strategy `UiKitContextMenu`: the menu next to the preview,
  ADD-A21).
- **Variants:** none. **Sizes:** none. **Flags:** none. **Conditions:** `cond reduced_transparency` (material to
  opaque) and `cond reduced_motion` as rules.
- **may_be_absent:** none (Open items).
- **State-model source:** UIKit appearances and the interaction's preview parameters.
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.keyboard (ios)

*From group 7.*

- **Kind:** keyed component. The keyboard itself is drawn only by the system (out, R1.1); the application's part is
  what it configures: the appearance and the input accessory it draws. Row C-23.
- **Parts:**
  - `input_accessory` — the root, the keyboard toolbar: `fill` (colour or material), `padding`. Nothing for the keys.
- **Widget values:** `appearance: Value<Key>` (keys `default`, `light`, `dark`: `UIKeyboardAppearance`, set by the
  application so that the keyboard matches the theme; following the scheme is a rule `@ cond scheme:dark` stating
  `dark`);
  `presentations: Vec<PresentationTransition>`: events `Appear` and `Disappear` (the keyboard shown and hidden; kind
  `Slide`; duration and timing are the
  keyboard notification's, read, and not the preset's).
- **Variants/sizes/flags:** none. **may_be_absent:** none.
- **Reads:** keyboard-notification duration and curve: `readers (R18)`.
- **State-model source:** UIKit appearances.
- **Today's fields:** none.

#### component.document_launch_view (ios)

*From group 7.*

- **Kind:** keyed component (`UIDocumentViewController.LaunchOptions`, iOS 18+). Row C-44†. Cross-check: GNOME (status
  page), Windows, KDE 3.
- **Parts:**
  - `background` — the root: `fill`, `padding`. Nested controls: the primary and secondary actions are `button`
    elements (with variants) in `context component.document_launch_view`.
  - `title`, `subtitle` — the launch screen's title and subtitle: `text`.
  - `foreground_accessory`, `background_accessory` — the accessory views: `icon`, `padding`.
  - `browser` — the document browser frame it opens: `fill`, `corners`.
- **Widget values:** none stated. **Variants/sizes/flags:** none. **may_be_absent:** none (an accessory or browser the
  application's configuration sets none of is not queried, S6.3).
- **State-model source:** UIKit configuration objects and control states (Open items).
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.activity_rings (ios)

*From group 7.*

- **Kind:** keyed component (HealthKitUI `HKActivityRingView`). Row C-46†.
- **Parts:**
  - `body` — the root, the rings' box: `padding`.
  - `ring` *(element)* — a stroked arc, its identity a keyed variant: `stroke` (its paint flat or a gradient, C-46†).
    `track` belongs to it.
  - `track`, in `ring` — that ring's background ring: `stroke`.
- **Widget values:** `ring_gap_px: Value<Px>`; the ring width is `ring.stroke` width; `ring_progress` is data, not
  theme.
- **Variants:** keyed `@move`, `@exercise`, `@stand` (three rings with an Apple Watch paired, the Move ring only
  without one: the data decides which are drawn, so none is `may_be_absent`).
- **Paint:** ring colours may be gradients (C-46†: "gradient or flat: M-r"): see ADD-A34, ADD-B02.
- **State-model source:** the view's own configuration (HealthKitUI); light and dark appearances.
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.large_content_viewer (ios)

*Written in the gap fixes.*

- **Kind:** keyed component: the Large Content Viewer, the HUD UIKit presents on a long press of a bar item to show
  its symbol and title at an accessibility size (`UILargeContentViewerInteraction`, iOS 13). It is in by R1.1: the
  interaction is a `UIInteraction` on the application's own view, so UIKit draws the viewer in the application's
  process, and a toolkit drawing its own bars draws its own equivalent. Row C-26.
- **Parts:**
  - `body` — the HUD (the root): `fill` (its material), `corners`, `padding`, `min_width`, `min_height`.
  - `icon` — the item's symbol at the large size: `icon`.
  - `label` — the item's title: `text`.
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear`.
- **Variants/sizes/flags:** none. **may_be_absent:** none (an item that gives no icon or label has none queried, S6.3).
- **State-model source:** UIKit's interaction (measured, C-26).
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.editing_hud (ios)

*Written in the gap fixes.*

- **Kind:** keyed component: the banner UIKit shows at the top after a three-finger editing gesture, naming the
  action (undo, redo, cut, copy, paste). It is in by R1.1: it answers the first responder's editing interaction, which
  each responder configures (`UIResponder.editingInteractionConfiguration`, `.default` or `.none`, iOS 13), so UIKit
  draws it in the application's process; a toolkit with its own text editing draws its own equivalent. Row C-49†.
- **Parts:**
  - `body` — the banner (the root): `fill` (colour or material), `corners`, `padding`.
  - `label` — the action's name: `text`.
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear`.
- **Variants/sizes/flags:** none. **may_be_absent:** none (the component is drawn only while it shows).
- **State-model source:** UIKit's editing interaction (measured, C-49†; whether `.none` removes the banner is to
  verify).
- **Today's fields:** none. Reader: `readers (R18)`.

### Material

*From group 6:*

| Component | Platform | Parts | Values | Rows |
|---|---|---|---|---|
| `fab_menu` | Material | `body` (the root: `padding`), `item` *(element)* (a menu item: `fill`, `corners`, `padding`, `shadow`, `min_height` (its height, MAT-N3), `transitions`), `item_label` (in `item`, queried with it: `text`), `item_icon` (in `item`, queried with it: `icon`); variants `@primary`, `@secondary`, `@tertiary` (its colour styles, MAT-N3); its close button is `button @ variant close context component.fab_menu` | `item_gap_px: Value<Px>`, `presentations: Vec<PresentationTransition>` (events `Appear`, `Disappear` (the menu opening from the FAB and closing), kind `Morph`) | MAT-N3 |

## Colour roles and text ramp

*From group 5's catalogue.*

### Policy: typed set, typed global, or keyed

1. A **typed surface set** exists when a platform states a background and a foreground for a region that parts bind
   (`Binding::Surface`, S3.3): KDE's seven sets (Q3.1), libadwaita's window, view, header bar, sidebar, secondary
   sidebar, card, dialog, popover and OSD. A set is declared by the preset (ADD-A48); `Window` and `Selection` by
   every preset (S3: selected items take the `Selection` set's roles).
2. A **global role** is one the platform states once for the whole theme (accent, status, scrim, highlights).
3. A **keyed role** is every other colour the platform names: a ladder (Material's, WinUI's tiers, Apple's label, fill
and
   background hierarchies, the system palette). Keyed roles are declared in `[declare] color_roles` and referenced as
   `role.@key`.
4. A role never repeats another: where a platform has no separate value, the preset states an expression
   (`= role.window.foreground`), so a toolkit follows the platform and the gate checks the reference (R20).

#### surface sets

`SurfaceSet` (S3) has thirteen variants (`SecondarySidebar` for GN-1.137):

| Set | What it is | Declared by (sources of the list) |
|---|---|---|
| `Window` | the root surface; mandatory; what `surface.<role>` falls back to (S3.3) | every preset |
| `View` | content the user edits or reads: lists, text views, entries | KDE View (Q3.1, Q3.2); libadwaita `view_*` [GN-1.274]; the content/background hierarchy elsewhere |
| `Button` | KDE's Button set: push buttons, check and switch indicators, sliders | KDE only as a set (Q3.1, Q3.2); other platforms state button fills per part |
| `Header` | header bars, menu bars, page headers, column/drawer separators | KDE Header (Q3.1, Q3.2); libadwaita `headerbar_*` [GN-1.15] |
| `Sidebar` | sidebars, source lists | libadwaita `sidebar_*` [GN-1.138]; other platforms when they state one |
| `SecondarySidebar` | libadwaita's second sidebar | libadwaita `secondary_sidebar_*` [GN-1.137] |
| `Card` | cards, boxed lists | libadwaita `card_*` [GN-1.255]; WinUI card fills when stated as a set |
| `Popover` | popovers, menus, flyouts | libadwaita `popover_*` [GN-1.182]; on `ios-*` the elevated backgrounds, bound by a popover's root (R5.2, D-14) |
| `Dialog` | dialogs and sheets | libadwaita `dialog_*`; on `ios-*` the elevated backgrounds, bound by a dialog's or sheet's root (R5.2, D-14) |
| `Tooltip` | tooltips | KDE Tooltip (Q3.1); Kirigami swaps it for `Complementary` (Q1.37) |
| `Selection` | the selected item: list/menu selection, KDE Selection set | every preset (S3); KDE Selection (Q3.1), every platform's list selection |
| `Complementary` | KDE's inverse-scheme set: passive notifications, tooltips in Kirigami | KDE only [K3.54, Q3.1] |
| `Osd` | libadwaita `.osd`: toasts, OSD toolbars and their accent overrides | libadwaita [GN-3.09, GN-3.17] |

Thumbnail and overview colours of libadwaita (`--thumbnail-*`, `--overview-*`) are a background and a foreground only
and no part binds them as a surface: they are keyed roles (below), not sets [GN-1.276].

A preset declares the sets it fills in `[declare] surface_sets = [...]` (ADD-A48). A binding to, or a
reference through, an undeclared set is a resolution error (S12). Per platform:

| Platform | Sets declared (from the rows named) |
|---|---|
| `kde-breeze` | Window, View, Button, Selection, Tooltip, Complementary, Header (all seven, each with Active, Inactive and Disabled groups) [K3.54, Q3.1] |
| `adwaita` | Window, View, Header, Sidebar, SecondarySidebar, Card, Dialog, Popover, Osd, Selection (its pair the selected rows' colours libadwaita states, as references where they are another role's) [libadwaita `_colors.scss`, GN-3.09] |
| `windows-11`, `macos-*`, `ios-*`, `material-3` | Window, Selection and every set whose background and foreground the platform's facts state for a region; the value phase declares them from the facts (on `ios-*` `Dialog` and `Popover` among them, for the elevated level, D-14). Ladders go to keyed roles (below), they are not sets |

#### role record

`SurfaceRoles` is the record every declared set states. Fields (the S3 list, closed here):

| Field | Meaning | Required | Reasons |
|---|---|---|---|
| `background` | the set's fill | required | all |
| `background_alternate` | alternate row/band fill (KDE `BackgroundAlternate`); alternating rows only where a list opts in | may_be_absent | Q3.13 |
| `foreground` | primary text and glyph colour | required | all |
| `foreground_inactive` | the platform's secondary (muted) text level | required | KDE `ForegroundInactive`; today's `muted_color` |
| `foreground_active` | KDE `ForegroundActive`, used as the information colour | may_be_absent | K3.54, Q3.5 |
| `link`, `link_visited` | link text, visited link text | `link` required; `link_visited` may_be_absent | K3.54, GN-1.265 |
| `negative`, `neutral`, `positive` | the set's status text colours (KDE `ForegroundNegative`, `ForegroundNeutral`, `ForegroundPositive`); a platform without per-set values states `negative = role.global.status.error.standalone`, `neutral = role.global.status.warning.standalone`, `positive = role.global.status.success.standalone` (KDE's neutral is a warning, rename row 86; `StatusRoles`' own `neutral` is another colour) | required | K3.54, Q3.10 |
| `decoration_focus`, `decoration_hover` | KDE's focus and hover decoration colours of the set | may_be_absent | Q3.11, Q3.12 |
| `border` | the set's stroke colour (translucent where the platform's is) | required | all; absorbs today's `border.color` and `border.opacity` as `alpha(...)` |
| `shade` | the translucent shade line/overlay of the set (libadwaita `*_shade_color`) | may_be_absent | GN-1.270 |
| `disabled` | `DisabledRoles`: a record whose every field may be absent (the record itself is always there) | each field may_be_absent | R5.1 |

S3's further fields (ADD-A48): `background_active`, `background_link`, `background_visited`,
`background_negative`, `background_neutral`, `background_positive` (KDE's six computed status backgrounds, tinted from
the
set's foreground roles) [K3.52]; `shade_darker` (libadwaita's `headerbar_darker_shade_color`) [GN-1.15]. All
may_be_absent.

`DisabledRoles` has the same colour fields as `SurfaceRoles` except `disabled`; every field may_be_absent. On KDE each
is
the KColorScheme disabled effect applied to the active value (`kde_effects(c, background, disabled, <role kind>)`, the kind `foreground`, `decoration` or `background` per field, which reads
`token.kde.disabled`, S2.2, S4.8) [K3.50]; on a platform that dims by opacity (libadwaita) the record stays absent and
the
parts'
`{disabled}` rules set `opacity` from `token.params.opacity` instead [GN-1.271, GN-1.272].

The inactive group is no record field: it is a colour-role rule `@ cond window_inactive` on the active field
(libadwaita's
`*_backdrop_color`, KDE's `[Colors:*][Inactive]` inputs and effects, AppKit's unemphasised colours) [K3.51, GN-1.136,
S3.2].

#### global roles

`GlobalRoles` (S3 list, closed here; every S3 field has a row):

| Field | Meaning | Required | Reasons |
|---|---|---|---|
| `accent` | the accent fill (libadwaita `accent_bg_color`, today's `accent_color`) | required | GN-3.14 |
| `accent_foreground` | text/glyph on the accent (today's `accent_text_color`) | required | GN-3.14 |
| `accent_standalone` | accent used as text or icon colour on the surface, distinct from the fill (libadwaita `accent_color`, OKLab-derived: `adw_standalone`); elsewhere `= role.global.accent` | required | GN-1.268, GN-3.13 |
| `accent_ramp` | keyed: WinUI Light1-3, Dark1-3 | a list: empty where the platform has none | W3-12 (other group) |
| `status` | `StatusRoles`: `success`, `warning`, `error`, `info`, `neutral`, `destructive`, each with `fill`, `foreground`, `standalone`, `background`, `background_solid` (S3, ADD-A48); `destructive` and `error` are separate entries [GN-1.269] | `success`, `warning`, `error` required; `info`, `neutral`, `destructive` and every `background_solid` may_be_absent (a platform with no such status colour) | GN-1.269, W3-17/18, K3.52 |
| `shadow_reference` | what the platform names as the shadow colour (today's `shadow_color`; macOS `shadowColor`, Material `shadow`) | may_be_absent (a platform that names none) | today's `shadow_color`, MAT-X5 |
| `selection_text_background` | the highlight behind selected text (today's `text_selection_background`) | may_be_absent | today's fields |
| `selection_text_foreground` | the selected text's colour (today's `text_selection_color`); absent where the text keeps its own colour (GN-3.41, W3-25) | may_be_absent | today's fields |
| `selection_text_background_unfocused` | an unfocused field's selection (WinUI's `SelectionHighlightColorWhenNotFocused`, AppKit's unemphasised selection; ADD-A49) | may_be_absent | W1-032, W3-25, MAC-I05 |
| `selection_text_foreground_unfocused` | an unfocused field's selected text (macOS's unemphasised selected text colour) | may_be_absent | MAC-I05 |
| `find_highlight` | the find/search match highlight | may_be_absent | MAC-DF04 |
| `drop_highlight` | the drop-target highlight | may_be_absent | R12 |
| `scrim` | modal backdrop colour (with its alpha) | required | MAT-X23, W3-11, W1-125 |

The inactive window's text selection is a colour rule `@ cond window_inactive` on `selection_text_background` and
`selection_text_foreground` (S3.2), like every inactive group; a preset whose platform does not distinguish selected
text from selected items states these roles as references to the `Selection` set (S3).

#### keyed roles per platform

Keyed roles are declared in `[declare] color_roles`; the names are snake_case of the platform's own. Families, with the
rows that need them (the member lists are the facts', the value phase states them):

**GNOME (`adwaita`).** `thumbnail_background`, `thumbnail_foreground`, `overview_background`, `overview_foreground`
[GN-1.276]. (Header bar, sidebar, card and popover shades are record fields [GN-1.270]; the window shade `--shade-color`
is `role.window.shade`.)

**KDE (`kde-breeze`).** `shade_light`, `shade_midlight`, `shade_mid`, `shade_dark`, `shade_shadow`: Qt's 3-D frame and
grid roles, computed from the Window background by `kcu_shade` with `token.params.color.contrast` [K3.53]. They are
keyed, not per set: KDE computes them from Window.

**macOS (`macos-*`).** Label hierarchy beyond the record: `label_tertiary`, `label_quaternary`, `label_quinary`
(14+) [MAC-DF01]; the system fill ramp `system_fill`, `system_fill_secondary`, `system_fill_tertiary`,
`system_fill_quaternary`, `system_fill_quinary` [MAC-DF02]; the system palette beyond red, orange, green and blue:
`system_yellow`, `system_purple`, `system_pink`, `system_teal`, `system_indigo`, `system_cyan`, `system_mint`,
`system_brown`, `system_gray` (and the four already named, stated as `system_red`, `system_orange`, `system_green`,
`system_blue`) [MAC-DF03]; `under_page_background`, `text_background` (against `View.background` =
`controlBackgroundColor`), `selected_control`, `selected_control_secondary`, `selected_control_text` [MAC-DF04,
MAC-DF05]; `highlight` (the separate highlight setting, an input, S9.1) [MAC-DF05]. The accent-derived colours are
inputs
(R2.6).

**iOS (`ios-*`).** Label hierarchy: `label_tertiary`, `label_quaternary` (tertiary no longer conflated with disabled
text: `role.window.disabled.foreground` states disabled separately) [D-9, X-CO-5]; fills `system_fill`,
`system_fill_secondary`,
`system_fill_tertiary`, `system_fill_quaternary` [D-10]; backgrounds `system_background`, `system_background_secondary`,
`system_background_tertiary`, `system_grouped_background`, `system_grouped_background_secondary`,
`system_grouped_background_tertiary` [D-11]; their elevated level, keyed too: `system_background_elevated`,
`system_background_secondary_elevated`, `system_background_tertiary_elevated`,
`system_grouped_background_elevated`, `system_grouped_background_secondary_elevated`,
`system_grouped_background_tertiary_elevated` [D-14]. In a sheet, a popover or an alert the elevated level is a
binding (R2.3, R5.2): the root of `dialog`, `sheet` and `popover` binds the `Dialog` or `Popover` set, whose
backgrounds the preset states as the elevated values, and a `Binding::Role` from each keyed background role to its
elevated twin (`role.@system_background` to `= role.@system_background_elevated`, and so on); colour rules
`@ cond elevated` (S7.1) restate the backgrounds only where the whole window is elevated (multitasking);
`separator_opaque`
(the translucent separator is `border`) [D-12]; `placeholder` [X-CO-2]; the system palette `system_red`,
`system_orange`,
`system_yellow`, `system_green`, `system_mint`, `system_teal`, `system_cyan`, `system_blue`, `system_indigo`,
`system_purple`, `system_pink`, `system_brown`, `system_gray`, `system_gray2` .. `system_gray6` [D-13]. Tint-dependent
roles are inputs (R2.6); increased-contrast variants are colour-role rules under `ContrastIncreased` [D-15].

**Windows (`windows-11`).** The WinUI tiers, one keyed role per brush of `Common_themeresources_any.xaml`, in the
families the rows name: text tiers (`text_tertiary`, `text_inverse`, text on accent: secondary, disabled, selected)
and accent
text/fill tiers [W3-12, W3-13, W3-14, W3-46]; stroke tiers (control, strong, card, surface, divider, on-accent) [W3-15];
fill tiers (control, strong, subtle, control-alt, on-image, solid background base..senary, base-alt) [W3-16]; status
backgrounds and neutral [W3-17, W3-18]; the layer fills, the content and commanding layers over the backdrop
(`layer_fill_default`, `layer_fill_alt`, `layer_on_acrylic_fill_default`, `layer_on_accent_acrylic_fill_default`,
`layer_on_mica_base_alt_*`, one per brush the facts list) [W3-09]; the eight high-contrast system colours [W3-35].
These rows belong to the
crosscutting group; this file only fixes that they are keyed.

**Material (`material-3`).** Every `md.sys.color` role of `latest/_md-sys-color.scss:20-206`, in snake_case:
primary, secondary, tertiary and error with their `on_`, `_container` and `on_..._container` forms; the fixed family
(`primary_fixed`, `primary_fixed_dim`, `on_primary_fixed`, `on_primary_fixed_variant`, and the same for secondary and
tertiary); `background`, `on_background`, `surface`, `on_surface`, `surface_variant`, `on_surface_variant`,
`surface_dim`,
`surface_bright`, `surface_container_lowest`, `_low`, `surface_container`, `_high`, `_highest`; `outline`,
`outline_variant`; `inverse_surface`, `inverse_on_surface`, `inverse_primary`; `surface_tint`; `shadow`, `scrim`
[MAT-X1]. Beside them, the platform popup-menu style's colours Compose's text context menu resolves,
`popup_menu_background` (`Widget_PopupMenu`'s `colorBackground`) and `popup_menu_text`
(`TextAppearance_Widget_PopupMenu_Large`'s `textColorPrimary`) [MAT-P3-11†]. The typed record's fields and the global
roles reference them (`accent = role.@primary`; the scheme function
`m3_role` supplies them from the seed input, R2.6).

### Text ramp

#### text roles

`TextRoles` (S4.4) has nineteen fields, closed (ADD-A42), so Apple's smallest style, Material's `label-small` and the
emphasised captions each have a slot. Each slot is a **reference to a keyed style**, `= token.text.keyed.<key>`: the
preset names the keyed style, so "the facts say which of its styles fills it" is literal and a style is never stated
twice. Where a platform has fewer styles than slots, neighbouring slots **share** a style (marked "=" below); the share
is a stated fact, never a toolkit default.

The table's platform columns are a **draft of facts**, not structure: which keyed style fills each slot is a fact of
each platform, and the value phase states it in `docs/facts/` (S17) with its citation; where the facts differ, the
facts win and this table is not consulted. The structure here is the slots and their uses.

| Slot | Use | Apple (macOS/iOS) | Windows | GNOME | KDE | Material |
|---|---|---|---|---|---|---|
| `display_large` | the largest hero text | `large_title` (iOS: `extra_large_title` where iOS draws it, TS-4) | `display` | `title_1` | `heading_1` | `display_large` |
| `display_medium` | hero text | `large_title` = | `display` = | `title_1` = | `heading_1` = | `display_medium` |
| `display_small` | hero text, today's `text_scale.display` | `large_title` = | `display` = | `title_1` = | `heading_1` = | `display_small` |
| `large_title` | page/screen title | `large_title` | `title_large` | `title_1` | `heading_1` = | `headline_large` |
| `title1` | dialog/page title, today's `dialog_title` | `title_1` | `title` | `title_2` | `heading_1` | `headline_small` |
| `title2` | sheet/section title | `title_2` | `subtitle` | `title_3` | `heading_2` | `title_large` |
| `title3` | sub-section title | `title_3` | `subtitle` = | `title_4` | `heading_3` | `title_large` = |
| `headline` | section heading, today's `section_heading` | `headline` | `subtitle` = | `heading` | `heading_3` = | `title_medium` |
| `subheadline` | secondary line under a heading | `subheadline` | `body` = | `body` = | `heading_4` | `title_small` |
| `body` | running and UI text, today's `defaults.font` | `body` | `body` | `body` | `default` | `body_medium` |
| `body_emphasized` | emphasised body | `body_emphasized` | `body_strong` | `heading` = | `heading_5_primary` | `body_medium_emphasized` (expressive); `title_small` (baseline) |
| `callout` | body-sized text, slightly set apart | `callout` | `body_large` | `body` = | `default` = | `body_large` |
| `footnote` | notes | `footnote` | `caption` = | `caption` = | `smallest_readable` = | `body_small` = |
| `caption` | smallest readable, today's `text_scale.caption` | `caption_1` | `caption` | `caption` | `smallest_readable` | `body_small` |
| `caption_emphasized` | emphasised caption | `caption_1_emphasized` | `caption` = | `caption_heading` | `smallest_readable` = | `body_small_emphasized` (expressive); `label_medium` (baseline) |
| `caption2` | the smallest style | `caption_2` | `caption` = | `caption` = | `smallest_readable` = | `label_small` |
| `label` | control labels | `control_content` (macOS), `body` = (iOS) | `body` = | `body` = | `default` = | `label_large` |
| `monospace` | monospace text, today's `mono_font` | `monospaced` | `monospace` | `monospace` (the `monospace-font-name` setting) | `fixed` | `monospace` |
| `numeric` | tabular figures | `body_monospaced_digits` (D-8) | `body` = | `numeric` | `default` = | `body_medium` = |

The assignment of KDE's levels follows Kirigami's own use (Q1.5: dialog and page title level 1, sheet header level 2,
form
section level 3, drawer section level 4, list section header level 5 in its primary type). The Material
"expressive/baseline" alternatives are a flavour rule on the token (`cond flavor:expressive`, S7.1); the emphasised
set exists only in Expressive [MAT-TS4]. Every
cell above is a proposal: the value phase states the facts, and where a fact names another style, the fact, cited,
replaces the cell.

#### keyed text styles per platform

A keyed text style is a `TextStyle` (S2.8) the preset declares in `[declare] text_styles`. Each platform lists **all**
of
its own named styles, so nothing the platform draws is lost whether or not a slot references it.

**Apple.** The eleven text styles, snake_case: `large_title`, `title_1`, `title_2`, `title_3`, `headline`,
`subheadline`, `body`, `callout`, `footnote`, `caption_1`, `caption_2` [MAC-TS01, TS-1]; for each, the emphasised-weight
form `<style>_emphasized` (the HIG's "emphasized" column; Bold Text uses it) [D-7, MAC-TS01]; for each, the tight and
loose
leading forms `<style>_tight`, `<style>_loose` (font-descriptor traits; they differ in `line_height` only) [D-6,
MAC-TS03]; iOS adds `extra_large_title`, `extra_large_title_2` (iOS 17) [TS-4]; the named AppKit fonts `label`,
`message`,
`palette`, `menu_bar`, `control_content` and `small_system` [MAC-TS02]; `monospaced`, `body_monospaced_digits`
(below). Per style: `letter_spacing` is
`sf_tracking(font_size, <design>)`, the style's `TrackingDesign` (`sf_pro`, `sf_pro_rounded`, `new_york`) (ADD-A54) [D-1]; `font.axes` carries `opsz` as `FollowsSize` [D-2]; `font.width` and
the
family carry the system design and width [D-3]; `font.features` carries monospaced digits where a style uses them [D-8];
`line_height` is each style's own [D-4]; `monospaced` is the system's monospaced font (macOS `monospacedSystemFont`,
its iOS equivalent) and `body_monospaced_digits` is `body` with monospaced digits in `font.features` [D-8]. The
Dynamic Type tables (size per content-size category, non-linear) are read by
the iOS reader, the preset states the default category [D-5, TS-2, R10]; Bold Text is the `BoldText` condition (a token
rule on `font.weight`) [TS-3].

**Windows.** The nine Fluent styles: `caption`, `body`, `body_strong`, `body_large`, `body_large_strong`, `subtitle`,
`title`, `title_large`, `display` [W1-147]; `monospace`, sourced as today's `mono_font` is (W3-47†).

**GNOME.** libadwaita's classes: `title_1`, `title_2`, `title_3`, `title_4`, `heading`, `body`, `caption_heading`,
`caption`, `document`, `monospace`, `numeric` [GN-1.189]. `document` takes its family and size from
`document-font-name` [GN-1.191]; `numeric` carries the `tnum` feature [GN-1.192]; `font.axes` and `font.features`
receive
the Pango `@axis=` and `#feature=` segments of the font settings [GN-1.193]; `line_height` of `body`, `caption` and
`document` is a multiple of the font size [GN-1.190]. The relative label sizes the stylesheet states are keyed styles
whose size is `mul(token.text.roles.body.font.size, <factor>)`: `smaller` (subtitles, progress text),
`narrow_switcher_label`,
`badge_label`, `tab_counter` [GN-1.194].

**KDE.** Kirigami `Heading` levels `heading_1` .. `heading_5` as factors of the default font size (KTitleWidget's
levels 1-4
state the same factors, so one style serves both and its fact cites both sources) [Q1.3, K1.270]; the types per level:
`heading_<n>_primary` (the primary type's weight) and `heading_<n>_secondary` (text fill at reduced alpha) [Q1.4];
`smallest_readable`, a full font (family, style, size) read from `smallestReadableFont`, also Kirigami's `smallFont`
[K1.271, Q1.2]; the Breeze-derived fonts `flat_group_title`, `command_link_title`, `menu_section` (each derived from
another font, its weight and size by Breeze's rule) [K1.272], whose size uses `add` (ADD-A56); the read fonts
`default`, `fixed`, `menu`, `toolbar`, `taskbar` (kdeglobals `[General]` `font`, `fixed`, `menuFont`, `toolBarFont`,
`taskbarFont`, each with its extras: K3.46, K3.47), which the `body`, `callout`, `label`, `numeric` and `monospace`
slots and the menu and tool-button parts refer to.

**Material.** The fifteen baseline styles, `display_large` .. `label_small` (five groups of large, medium, small:
display,
headline, title, body, label) [MAT-TS1], and the fifteen Expressive emphasised styles `<style>_emphasized` [MAT-TS4].
Per style: `letter_spacing` [MAT-TS2]; `font.family` the brand or plain typeface, with its weight name [MAT-TS3];
`font.axes` for `crsv`, `fill`, `grad`, `hexp`, `opsz`, `rond`, `slnt`, `wdth`, `wght` [MAT-TS5]; `monospace`, the
platform default monospace typeface (MAT-X36†).

#### tokens carried by these rows

- **Opacities.** `token.params.opacity` (ADD-A44; a group of unit `fraction`, S4.6): `disabled` (today's
  `disabled_opacity`), `strong_disabled`,
  `dim`, `dimmer`. libadwaita's high-contrast values are Tokens rules under `ContrastHigh` [GN-1.271, GN-1.273]. Parts
  refer to `token.params.opacity.<key>` in `opacity` or in `alpha(...)`.
- **Icon sizes.** `IconSizeTokens` (S4.5, ADD-A43): the five typed contexts of today (`toolbar`, `small`, `large`,
  `dialog`,
  `panel`, each a `Value<IconStyle>`, may_be_absent) and `keyed: Vec<(Key, IconStyle)>`, so a context also carries
  weight, scale and rendering mode
  [D-19, MAC-IC02]. Kirigami: `small`, `small_medium`, `medium`, `large`, `huge`, `enormous`, `for_labels`
  (`kirigami_label_icon_size(line_height)`, ADD-A53) [Q1.1]. macOS: `button`, `list_row`, `tab`, `toolbar`,
  `status_item`, `sidebar` (Tokens rules under `cond container_size(sidebar, small)` / `medium` / `large`, which are
  conditions) [MAC-IC01]; the toolbar icon's size per toolbar size mode is a `button.icon.icon.size` rule in `context
  toolbar.bar` with the toolbar's variant, since Tokens rules take conditions only (S7). The
  alert icon size is a measured value of `dialog` [MAC-IC04].
- **Focus.** iOS's keyboard halo (shape follows the view's rounded rect and curve; position inside, outside or
  automatic) is
  `token.focus` with its `corners` (their curve) and `offset` [D-18].

#### today's fields

`[defaults]` (docs/property-registry.toml:96-147) and `[text_scale]` (:147-151):

| Today's field | New home |
|---|---|
| `defaults.font` (family, size, weight, style, color) | the `body` slot's keyed style: `font.family`, `font.size`, `font.weight`, `font.style`; `color` -> `TextStyle.fill` = `= surface.foreground` |
| `defaults.line_height` | each style's own `line_height` (an expression on `font_size`); no global ratio (R20) |
| `defaults.mono_font` | the `monospace` slot |
| `defaults.background_color` | `role.window.background` |
| `defaults.text_color` | `role.window.foreground` |
| `defaults.accent_color` | `role.global.accent` (+ `accent_standalone` where the platform has one) |
| `defaults.accent_text_color` | `role.global.accent_foreground` |
| `defaults.surface_color` | `role.view.background` (the content surface; Card/Dialog etc. when the platform states them) |
| `defaults.muted_color` | `role.window.foreground_inactive` |
| `defaults.shadow_color` | `role.global.shadow_reference`; libadwaita's shade colours -> `shade` of each set |
| `defaults.link_color` | `<set>.link` |
| `defaults.selection_background`, `selection_text_color` | `role.selection.background`, `role.selection.foreground` |
| `defaults.selection_inactive_background` | `role.selection.background` under a colour rule `@ cond window_inactive` |
| `defaults.text_selection_background`, `text_selection_color` | `role.global.selection_text_background`, `role.global.selection_text_foreground` |
| `defaults.disabled_text_color` | `role.window.disabled.foreground` (and the other sets') |
| `defaults.danger_color`, `danger_text_color` | `role.global.status.error.fill`, `.foreground` (and `destructive` where separate) |
| `defaults.warning_*`, `success_*`, `info_*` | `role.global.status.<s>.fill`, `.foreground` |
| `defaults.focus_ring_color`, `focus_ring_width_px`, `focus_ring_offset_px` | `token.focus` (`paint`, `width`, `offset`) |
| `defaults.border.color`, `border.opacity` | `<set>.border`, the opacity folded in as `alpha(...)` |
| `defaults.border.line_width_px`, `corner_radius_px`, `corner_radius_lg_px`, `shadow_enabled`, padding fields | no global: each part states its own `stroke`, `corners`, `shadow`, `padding` (R20, no model inheritance); the preset repeats the number per part |
| `defaults.disabled_opacity` | `token.params.opacity.disabled` |
| `defaults.icon_sizes.{toolbar,small,large,dialog,panel}_px` | `token.icons.sizes.{toolbar,small,large,dialog,panel}` (each an `IconStyle`; the size is its `size`) |
| `defaults.icon_theme`, `theme_variant.icon_set`, `theme_variant.icon_theme` | `token.icons.set`, `token.icons.theme`, with `token.icons.theme_fallback` (S4.5); a palette carries none (S14) |
| `text_scale.caption` | the `caption` slot (a reference to a keyed style) |
| `text_scale.section_heading` | the `headline` slot |
| `text_scale.dialog_title` | the `title1` slot |
| `text_scale.display` | the `display_small` slot (and `large_title`, which most platforms share with it) |
| `TextScaleEntry.size_pt`/`size_px`, `weight` | the keyed style's `font.size` (`FontSize`), `font.weight` |
| `TextScaleEntry.line_height_pt`/`line_height_px` | the keyed style's `line_height` (ADD-A40) |
| accessibility flags (§2.1.7) | conditions (S7.1) |

## Tokens, conditions, readers, placement

*From group 8's catalogue.*

### tokens

Token keys are the platform's own names, declared by the preset (S10). A key marked "(none)" below means the
platform has no such ramp and parts state the value themselves.

**Tokens that may be absent.** S6.3 has two sources, and a preset or reader may state a token `Absent` only where
one of them allows it. The first is the core's "may be absent" list for block sub-fields (S6.3), which applies
wherever the block appears and is cited here, not repeated: among the tokens it covers every `Behavior`,
`TouchBehavior`, `ScrollPhysics` and `PointerTokens` field (`hide_pointer_delay`, `hide_pointer_fade`,
`scroll_px_per_step`, `touchpad_factor`, `wheel_step_duration_max`, `touchpad_deceleration`, `touchpad_timing`,
`spring_loading`, `touch.*` and `pointer.color` on the platforms without them, and `key_repeat_delay`,
`key_repeat_interval` while the repeat setting is off, GN-3.68†, K-P3-16†), every `KdeTokens` field,
`MaterialScheme.contrast_level`, `IconTokens.theme_fallback`, the `Caret` fields it names, every `TextRendering`
field and a `FocusIndicator`'s `dash`, `shadow` and `transition`. The second is this table, for the tokens no block
mark covers:

| Token | May be `Absent` on | Why |
|---|---|---|
| `token.kde` | every preset but `kde-breeze` | KDE's colour effects and scheme options (S4.8) |
| `token.material` | every preset but `material-3` | Material's dynamic scheme (S4.8) |
| `token.icons.sizes.toolbar`, `.small`, `.large`, `.dialog`, `.panel` | each preset whose platform names no such context (the value phase names them from the facts) | today's five typed contexts are not every platform's (ADD-A43) |
| `token.params.display.corner_radius` | every preset but `ios-*`, `material-3`; there for a device model the preset does not list | the measured display radius per device model (S4.6, R25.11) |

**token.motion.durations** (`Vec<(Key, Value<Ms>)>`, S4.1). A scaled duration is `= mul(token.motion.durations.x,
token.motion.speed_factor)`, an unscaled one a plain duration (R9).

| Platform | Keys (one per duration the platform states) | Rows |
|---|---|---|
| libadwaita | the CSS transition tokens; the timed animations defined in code; the revealer and stack defaults; the animated-scroll duration | GN-3.18, GN-3.21, GN-3.22, GN-3.24 |
| kde-breeze | `breeze_state` (the Breeze state animation: factor times a base while the factor is set, breezerc `AnimationsDuration` otherwise); `qt_fixed` (Qt's own unscaled timings: side-icon fade, tab move); `kf_timeline` (KMessageWidget, KCollapsibleGroupBox); Kirigami `very_short`, `short`, `long`, `very_long` (derived from `long`, with the source's floor by `max`, ADD-A56) and `human_moment` (unscaled) | K3.31, K3.37, K3.38†, Q3.23–Q3.25, Q3.27 |
| windows-11 | `normal`, `fast`, `faster`; `connected` (connected animation); one entry per built-in theme transition the facts measure (entrance, content, add and delete, reorder, reposition, popup, pane, drill in, press tilt, fade, split open and close, drag item) | W3-30, W3-32, W3-33, W3-34 |
| macos-* | the AppKit durations (`NSAnimationContext` default, window resize time, per-control durations the probe records), `navigation` | MAC-X11 |
| ios-* | `navigation` (push and pop), `modal` styles, the symbol-effect durations | X-MO-1, X-MO-3, X-MO-4, X-MO-9 |
| material-3 | the motion-scheme duration tokens (short, medium, long, extra long families), the ripple's, the shadow transition's | MAT-X13, MAT-X9, MAT-X7 |

**token.motion.timings** (`Vec<(Key, Timing)>`): cubic Béziers, springs (`Timing::Spring`), `Timing::Platform(PlatformCurve)` where a
platform's curve is its own (Kirigami's velocity-based curves, `KirigamiVelocity`). Keys per platform: libadwaita's easing curves and
default timed-animation easing and spring parameters per animation (GN-3.19, GN-3.20); Kirigami's curves in use
(Q3.26); Qt's default easing curve for its timelines (K3.37); WinUI's entering, exiting and control key spline
(W3-31); UIKit's default
spring (X-MO-2) and AppKit's media-timing curves (MAC-X11); Material's easing and spring tokens (MAT-X13).

**token.motion.speed_factor** (`Value<f32>`): read on KDE (K3.30, K3.90, Q3.24); a constant elsewhere (the platform
has no
speed setting; the value phase states it). Rules set it
to zero under `AnimationsOff` where the platform's switch does that (GN-3.25, Q3.25, W3-37, W3-52†, MAT-X20).

**token.theme_change** (S4.3), one kind per preset, measured where no source states it: `adwaita`
`ThemeChange::Suspend` (GN-3.23); `kde-breeze` (K-P3-15†), `windows-11` (W-P3-16†, which also covers the accent change), `macos-*`
(MAC-P4-9†, accent and highlight change), `ios-*` (X-MO-11†), `material-3` (MAT-P3-18†).

**token.elevation** (`Vec<(Key, Vec<ShadowLayer>)>`, S4/R8): the ramp's keys, referred to by `Shadow::Elevation(key)`.

| Platform | Keys | Rows |
|---|---|---|
| windows-11 | the elevation steps WinUI names (1 to 128), each its ambient layer, the dark scheme's intensity as a colour-role-free rule under `scheme:dark` | W3-01, W3-02, W3-03 |
| material-3 | levels 0 to 5, each a key and an ambient layer (layers per level), per-state level by rules | MAT-X2, MAT-X3, MAT-X4, MAT-X5 |
| kde-breeze | the window shadow size steps and the popup shadow size steps (two tables, two layers per step), with `token.params.shadow.strength` and the shadow colour read from breezerc | K3.1, K3.2 |
| Kirigami (flavour `kirigami`) | one entry per Kirigami and qqc2 surface | Q3.21 |
| libadwaita, macos-*, ios-* | (none): layers are stated per part (`PartStyle.shadow.layers`) | GN-3.01, MAC-X01, X-SH-1 |

**token.params** (ADD-A44), keyed scalar tokens in groups, each a number, length, duration or opacity as its name says
(`token.params.display.corner_radius` measured per device model, by rules under `cond device_model:<key>`, an
unlisted model's `Absent`, S4.6, W-6, X-CR-4, ADD-A83):
`token.params.shadow.strength` (K3.1, K3.2); `token.params.motion.animation_steps` (K3.31);
`token.params.color.contrast` and `token.params.color.frame_contrast` (K3.53, K3.91, Q3.14, Q3.15);
`token.params.layout.grid_unit` (Q3.29); `token.params.layout.optical_centering` (MAT-X37†);
`token.params.caret.aspect_ratio` (GN-3.36); `token.params.shape.<step>` for Material's shape scale (MAT-X11);
`token.params.display.corner_radius` (R25.11); `token.params.icon_effect.<key>` (K3.60, Q3.18).

**token.kde** (`KdeTokens`, S4.8, `kde-breeze` only): `disabled` and `inactive`, each a `KdeEffects` (intensity effect
and amount, colour effect, amount and colour, contrast effect and amount: K3.50, K3.51), the inactive group's
`inactive_change_selection_color` and `inactive_enabled` (K3.40), the scheme's `tint_factor` and
`titlebar_is_accent_colored` (R25.1). The icon effects (K3.60, Q3.18) are `IconStyle.state_effect` keys (S2.9, declared
in `icon_state_effects`), their gray, transparency and gamma parameters `token.params.icon_effect.<key>`. A palette from
a KDE port states `token.kde` itself (R19, S14).

**token.behavior** (S4.2): the existing fields (double-click time and distance, drag threshold, hover-open delay,
key repeat delay and interval, scroll lines per step, hide pointer while typing, scroll physics) carry these rows:
GN-3.47, GN-3.48, GN-3.67†, GN-3.68†, K3.70–K3.72, K-P3-16†, W3-41, MAC-X12, MAC-P3-12†, MAC-P3-17†, MAT-P3-14†,
MAT-P3-15†, MAT-X32†, P3-13†, GN-P3-15†. Scroll physics (`token.behavior.scroll`) carries kinetic friction,
overshoot and its spring, animated-scroll duration and curve, snap settle, momentum: GN-3.24, GN-3.06, K-P3-18†,
MAC-P4-4†, MAC-X32†, W3-52†, X-MO-8, MAT-P3-16† (its minimum and maximum fling velocities: `min_fling_velocity`,
`max_fling_velocity`; Kirigami's wheel step whose duration grows with the distance between `wheel_step_duration`
and `wheel_step_duration_max`, K-P3-18†). New fields: ADD-A45; `hide_pointer_delay` (KLineEdit's, K3.65†),
`hide_pointer_fade` (Android's gradual fade, MAT-P3-15†), `scroll_px_per_step` (Android's pixels per wheel step,
MAT-X32†): ADD-A86. GTK's step from the view's page size is `scroll_view.values.wheel_step` (GN-P3-15†).
`focus_visible_when` (when focus becomes visible, a `FocusVisibleWhen`, the
default of every widget's value): ADD-A16.

**token.focus** (S4/R12): the platform's one indicator (width, offset, kind, corners, transition; an inner
ring is a layer of its `shadow`, R7, ADD-A17; its corners `Radius::FollowPart`, R12's concentric default), by reference from every focusable part; parts that differ state their own (KDE's underline under tab
labels, K3.21; Material's inner offset, MAT-X10; WinUI's per-control margin, W3-20). Material's focus animation, the
ring growing to an active width and then settling, is a `TransitionKind::Keyframes` transition (MAT-X10, ADD-A81);
WinUI's opt-in kinds are `FocusKind::Keyed(@reveal)` (the Reveal glow) and the ring with `dash` `Dotted` (DottedLine)
(W3-20, ADD-A82). Rows: GN-3.27–GN-3.29,
K3.20–K3.24, Q3.30, W3-19, W3-20, MAC-X13, X-FO-1, MAT-X10.

**token.caret** (S4.2): colour, width, shape, blink on and off times, blink timeout, blink style, secondary colour,
`solid_while_typing` (the caret kept solid while typing, X-CA-2, ADD-A85), `shown_in_read_only`. Hiding the caret
while text is selected and the overwrite block caret are the text widgets' caret parts' rules (`caret` absent under
`{text_selected}` and `{overwrite}`, `block_caret` drawn under `{overwrite}`: S4.2, ADD-A23). A caret that does not
blink is drawn, so it is not
`Absent`: `blink_off` is zero (never off; TOML `blink_off_ms`), as the platforms' "no blink" settings state it (GN-3.32,
X-CA-2). Rows:
GN-3.32–GN-3.38, GN-3.73†, K3.41–K3.43, W3-24, W-P3-9†, MAC-X12, MAC-X14, X-CA-1, X-CA-2, P3-9†, MAT-X25. The caret
is also a part of the text widgets (ADD-A24). Additions: ADD-A24.

**token.text** (S4.4): `roles` and `keyed` as S4.4 states, per platform the styles its facts list. Group 8 adds
keyed `taskbar` (K3.47), the per-role line height, letter spacing and optical-size axis (MAC-X18, W3-23, MAT-X16,
X-DT-5, GN-3.57's weight), the default size category's sizes on `ios-*` (X-DT-1), `monospace` with a source on
`windows-11` (W3-47†) and the platform default sans and monospace on `material-3` (MAT-X36†). `token.text.fallbacks`
(ADD-A41) holds the family fallbacks, emoji family and per-language fallback map (all FM rows); `token.text.rendering`
(ADD-B04) holds antialiasing, hinting, subpixel order and smoothing contrast; `token.text.scaling` how the
text-scaling factor applies (`AndroidNonLinear` on `material-3`, MAT-X19, ADD-A88).

**token.icons** (`IconTokens`, S4.5): `set`, `theme`, `theme_fallback`, `sizes` and `mirroring` (per icon role,
GN-FM03†, MAT-FM04†, W3-54†, X-SV-3). Under `sizes`, contexts beyond the five groups (GN-3.59, K3.63, W3-27) are keys
of the same token; weight and scale are per `IconStyle` (X-IC-2), not tokens.

**token.pointer** (ADD-A46): cursor theme, cursor size, pointer colours where read; `drag_cursors` keyed by drop
operation (copy, move, link, ask, none, not-allowed) for GN-P3-12†, K-P4-7†, W-P4-5†, MAT-P4-3†, P3-10†.

**token.size_classes**: no row of group 8 adds a threshold list. `ios-*` takes its class from the system (X-SV-1,
read by the connector at the root, R16); Material's width thresholds are stated by the widget groups' rows.

**token.material** (`MaterialScheme`, S4.8, `material-3` only): `variant` and `spec_version`; the contrast level is the
contrast conditions (MAT-X18, MAT-P3-19†); the seed is `input.accent` (S9.1), which `m3_role` reads (MAT-X17).

**Shape and colour tokens by role:** the colour roles (S3) hold colours; group 8 adds to them in ADD-A48 and uses
`role.global.scrim` (GN-3.04, Q3.22, W3-11, X-SC-1, MAT-X23, MAT-X1), `role.global.drop_highlight` (GN-3.66†,
MAC-X21, W3-49†), `role.global.find_highlight` (MAC-X15), `role.@*` for WinUI's tiers (W3-13–W3-18, W3-46†),
Material's ladder (MAT-X1) and UIKit's label prominence levels (X-CO-2, X-CO-5).

### conditions

S7.1 holds every condition the rows use (the list is closed by this catalogue and the traceability). Group 8's rows
use them as follows (reader mapping per platform: macOS's and iOS's Increase Contrast is `ContrastIncreased`; Windows
contrast themes and GNOME's high contrast are `ContrastHigh`; Material's medium and high levels are `ContrastIncreased`
and `ContrastHigh`; Plasma reports no contrast setting, so on KDE a contrast condition holds only when the application
sets it, K3.91, R25.2):

- `AnimationsOff` from a speed factor of zero on KDE (K3.30, K3.31); `ReducedMotion` as a degree on KDE is the factor
  (K3.90).
- `input:touch` and `input:pen` (`Input(InputKind)`) carry Material's pointer-precision sizing (MAT-X31), WinUI's touch
  menu sizes (W3-40, with the `opened_by_touch` flag for the element) and Kirigami's transient touch.
- `flavor:baseline` and `flavor:expressive` carry the Material look choice (MAT-P3-19†, MAT-X13).

The conditions these rows added to S7.1 (ADD-A04 … ADD-A11, ADD-A76, ADD-A83), each with its source and the rows:

| Condition | Meaning, source | Rows |
|---|---|---|
| `OnOffLabels` | the switch draws I and O marks; iOS's On/Off Labels setting | X-A11-6 |
| `PreferCrossFade` | iOS's Prefer Cross-Fade Transitions: presentation transitions become crossfades | X-A11-9, X-MO-10 |
| `FullKeyboardAccess` | macOS's and iOS's Full Keyboard Access: buttons show focus (the OS's own highlight, MAC-X31†, is out, R1.1) | MAC-X13, X-FO-3† |
| `AccessibilityTextSize` | the content size category is an accessibility category; layouts change axis | X-DT-3, X-A11-10 |
| `Idiom` | `Phone`, `Pad` (the idiom axis beside the size class) | X-SV-1 |
| `TintDimmed` | the window or scene is covered by a modal presentation; iOS dims the tint | X-CO-3 |
| `OverlayScrollbars` | the system shows overlay scroll bars (today's scroll-bar field `overlay_mode`): GNOME's overlay scrolling, macOS's overlay scroller style, Windows' auto-hiding scroll bars | GN-1.85, MAC-S01, W1-152† |
| `UnitScale` | the device pixel ratio is 1: Qt Quick's text rendering picks its engine by it | Q3.32 |
| `Freeform` | an Android desktop-windowing (freeform) window, with its own corner radius (ADD-A83) | MAT-W5† |
| `Posture(Posture)` | a foldable's posture (Jetpack WindowManager): two stacked panes in tabletop (ADD-A83) | MAT-P3-13† |
| `DeviceModel(Key)` | the device model, where a measured value differs per model: the display's corner radius (ADD-A83) | W-6, X-CR-4 |
| `SystemSetting(Key)` | a platform-specific switch the reader reads, declared by the preset in `[declare] conditions`: Win32 `menu_fade`, `tooltip_fade`, `selection_fade`, `combo_animation`, `listbox_smooth_scroll`, `hot_tracking`, `drop_shadow`, `ui_effects` (W3-53†); KDE `ui_effects` (K-P3-24†), `stacked_widget_transitions` (breezerc `StackedWidgetTransitionsEnabled`, K3.31), `dock_widget_draw_frame` (breezerc `DockWidgetDrawFrame`, K2.10); macOS `wallpaper_tinting` (MAC-X27) and `title_icons` (MAC-X31†); iOS `glass_look` (X-MT-11) | W3-53†, K-P3-24†, K3.31, K2.10, MAC-X27, MAC-X31†, X-MT-11 |

Conditions the rows do not need: a "colour filters" or "invert" condition (system-applied, R1.1: K3.93, W3-56†,
MAC-P4-3†, X-A11-8, MAT-X35†), and a "not responding" condition or an attention mark the shell or the compositor
draws (GN-P3-7†, K-P4-5†, K3.16†); the attention flash Windows draws on the application's own title bar is the
condition `AttentionFlash` (ADD-A03, `window.values.attention_flash`).

### readers

One line per read: platform, what, API or key as the row states it, rows. `(R18)` marks the platforms without a
reader until the platform-support milestone; the milestone implements these, the model already carries them. The
traceability's `reader` lines are the full set; this list groups them, and where the two differ the traceability
holds.

**GNOME (libadwaita and GTK)**

- portal accent RGB snapped to the nearest of libadwaita's nine accents (`adw-settings-impl-portal.c:140`,
  `adw-accent-color.c:160`), replacing the raw RGB the reader uses: GN-3.15.
- accent-color enum, `org.gnome.desktop.interface` (schema lines 314-320), fallback to the portal: GN-3.16.
- reduced-motion, `org.gnome.desktop.a11y.interface` (schema lines 18-24); `enable-animations` already read: GN-3.25,
  GN-3.26.
- keyboard-focus-visible-timeout, `org.gnome.desktop.a11y.interface` (lines 25-32): GN-3.30.
- caret blink setting, period and timeout, `org.gnome.desktop.interface` (lines 63-85): GN-3.32, GN-3.33, GN-3.34.
- always-show-text-caret, `org.gnome.desktop.a11y` (lines 12-19): GN-3.38.
- cursor theme and size, `org.gnome.desktop.interface` (lines 180-189): GN-3.46.
- double-click time and drag threshold, `org.gnome.desktop.peripherals` (lines 243-252): GN-3.47, GN-3.48; key repeat
  (lines 144-159): GN-3.68†.
- antialiasing, hinting, subpixel order, `org.gnome.desktop.interface` (lines 277-312): GN-3.56.
- font-name family list (the reader drops it today, `gnome/mod.rs:194-200`) and `fc-match` for the emoji family:
  GN-FM01†, GN-FM02†.
- text direction, `gtk_get_locale_direction()`: GN-FM03†.

**KDE (Breeze and Kirigami)**

- breezerc `[Common] ShadowSize`, `ShadowStrength`, `ShadowColor`: K3.1, K3.2.
- kwinrc `[Effect-blur] BlurStrength`, `NoiseStrength`, `Saturation`: K3.11.
- kdeglobals `[KDE] AnimationDurationFactor`: K3.30, K3.90, Q3.24; plasmarc `[Units] longDuration`: Q3.24.
- breezerc `[Style] AnimationsEnabled`, `AnimationSteps`, `AnimationsDuration`, `StackedWidgetTransitionsEnabled`:
  K3.31.
- kdeglobals `[ColorEffects:Disabled]` (six keys: intensity, colour, contrast effect and amount),
  `[ColorEffects:Inactive]`
  including `ChangeSelectionColor` and `Enable`, and the `[Colors:*][Inactive]` subgroups: K3.40, K3.50, K3.51.
- kdeglobals `[Colors:*]` (all seven sets, the keys the reader flattens today): Q3.1, K3.54.
- kdeglobals `[KDE] contrast` (unread today) and `frameContrast` (the factor kept, not only finished colours):
  K3.53, K3.91, Q3.14.
- kdeglobals `[KDE] CursorBlinkRate`: K3.41; `DoubleClickInterval`: K3.70; `StartDragDist`, `StartDragTime`: K3.71;
  `WheelScrollLines`: K3.72; `SingleClick`: K3.73; `SmoothScroll`: K-P3-18†; `GraphicEffectsLevel`: K-P3-24†.
- breezerc `[Style] MnemonicsMode`: K3.44.
- kdeglobals `[General] XftAntialias`, `XftHintStyle`, `XftSubPixel` (keys to verify in plasma-workspace): K3.45.
- kdeglobals font strings with their extras, `[General] taskbarFont`: K3.46, K3.47.
- icon theme `index.theme` `*Default` sizes: K3.63.
- kcminputrc `[Mouse] cursorTheme`, `cursorSize`; key repeat (`kwin src/main.cpp:111-112`,
  `src/keyboard_input.cpp:196-209`): K3.64, K-P3-16†.
- `org.kde.KWin.TabletModeManager` D-Bus property (to verify): K3.83.
- breezerc and kwinrc menu opacity and blur (K3.10, K3.11): K3.92.
- `fc-match` for the emoji family and per language; `QGuiApplication::layoutDirection()`: K-FM01†, K-FM02†, K-FM03†.
- breezerc `[Style] DockWidgetDrawFrame` into `cond system_setting:dock_widget_draw_frame`: K2.10; breezerc
  `[Style] SliderDrawTickMarks` into `cond system_setting:slider_tick_marks`: K1.103.
- kdeglobals `[General] ColorScheme` and the scheme file it names (the `kde_scheme.*` inputs), `[General]
  TintFactor` and the accent-coloured title bar option (`token.kde.tint_factor`, `titlebar_is_accent_colored`):
  S9.1, R25.1.

**Windows**

- `UISettings.GetColorValue` for `AccentDark1`–`3`, `AccentLight1`–`3`: W3-12.
- `SPI_GETKEYBOARDCUES`: W3-21, W3-48†.
- `SPI_GETCLEARTYPE`, `SPI_GETFONTSMOOTHING`, `SPI_GETFONTSMOOTHINGTYPE`, `SPI_GETFONTSMOOTHINGCONTRAST`: W3-22.
- `SPI_GETCARETWIDTH`, `GetCaretBlinkTime`, `UISettings.CaretWidth`, `UISettings.CaretBlinkRate`,
  `SPI_GETCARETTIMEOUT` (to verify): W3-24.
- `UISettings.CursorSize`, `SM_CXCURSOR`: W3-29.
- `GetSysColor` under high contrast, `AccessibilitySettings.HighContrastScheme`, `UISettings.UIElementColor` (to
  verify): W3-35.
- `UISettings.AnimationsEnabled`, `SPI_GETCLIENTAREAANIMATION`: W3-37; `UISettings.AdvancedEffectsEnabled`, registry
  `EnableTransparency`: W3-38.
- `GetDoubleClickTime` or `UISettings.DoubleClickTime`, `SPI_GETMOUSEHOVERTIME`, `SPI_GETMENUSHOWDELAY`, `SM_CXDRAG`,
  `SM_CYDRAG`, `SPI_GETWHEELSCROLLLINES`, `UISettings.MessageDuration`, `SPI_GETKEYBOARDDELAY`: W3-41;
  `SM_CXDOUBLECLK`, `SM_CYDOUBLECLK`: W-P3-12†; `SPI_GETWHEELSCROLLCHARS`: W3-52†; `SPI_GETMOUSEVANISH`: W-P3-10†.
- `HKCU\Console` `FaceName`, `FontSize`, `FontWeight` (host to verify): W3-47†.
- `SPI_GETMENUFADE`, `SPI_GETTOOLTIPFADE`, `SPI_GETSELECTIONFADE`, `SPI_GETCOMBOBOXANIMATION`,
  `SPI_GETLISTBOXSMOOTHSCROLLING`, `SPI_GETHOTTRACKING`, `SPI_GETDROPSHADOW`, `SPI_GETUIEFFECTS`: W3-53†.
- `SPI_GETMENUDROPALIGNMENT` into `cond system_setting:menu_drop_alignment`: W3-55†.
- `DevicePickerAppearance` getters (whether they return the system defaults, to verify): W-P4-18†.
- `IDWriteFontFallback::MapCharacters` (emoji string; per locale), `LanguageFontGroup`: W-FM01†, W-FM02†.

**macOS**

- `NSColor` read without clipping to sRGB, `colorSpace`: MAC-X10.
- `NSColor` resolved under `NSAppearance` vibrantLight and vibrantDark: MAC-X03; under `accessibilityHighContrastAqua`
  and `…DarkAqua`, stored as the inputs' per-contrast bases and colour rules: MAC-X22.
- `NSColor.withSystemEffect` (rollover, pressed, deepPressed, disabled) of each read colour, stated as state rules:
  MAC-X30†.
- `NSAnimationContext.duration`, `NSWindow.animationResizeTime(_:)`, `CAMediaTimingFunction` control points: MAC-X11.
- `NSEvent.doubleClickInterval`, `keyRepeatDelay`, `keyRepeatInterval`, `NSButtonCell.getPeriodicDelay`, user defaults
  `NSTextInsertionPointBlinkPeriodOn` and `…Off`, `NSInitialToolTipDelay`: MAC-X12.
- `NSApplication.isFullKeyboardAccessEnabled` or user default `AppleKeyboardUIMode` (both to verify): MAC-X13.
- `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` (already probed): MAC-X24.
- pointer size and colour user defaults, link cursor: MAC-X20 (keys to verify).
- `NSScrollView` line and page scroll properties: MAC-P3-17†; user default `NSScrollAnimationEnabled`: MAC-P4-4†.
- `CTFontCreateForString` on an emoji string; `CTFontCopyDefaultCascadeListForLanguages`: MAC-FM01†, MAC-FM02†.
- the wallpaper-tinting switch into `cond system_setting:wallpaper_tinting`: MAC-X27, MAC-W08; the "Show window
  title icons" user default into `cond system_setting:title_icons` (key to verify): MAC-X31†.

**iOS (R18 milestone; the application stands in until then, R2.6)**

- `UIColor.resolvedColor(with:)` over trait combinations; wide-gamut components; `UIColor.Prominence`: X-CO-1, X-CO-4,
  X-CO-5.
- `isDarkerSystemColorsEnabled`, `accessibilityContrast`; `isReduceTransparencyEnabled`; `isReduceMotionEnabled`,
  `prefersCrossFadeTransitions`; `isBoldTextEnabled`, `legibilityWeight`; `isOnOffSwitchLabelsEnabled`;
  `shouldDifferentiateWithoutColor`; `preferredContentSizeCategory`, `isAccessibilityCategory`, `UIFontMetrics`:
  X-A11-1…X-A11-7, X-A11-10, X-DT-2, X-DT-3, X-DT-4, X-MO-10, X-MT-3.
- Dynamic Type size per style per content size category: X-DT-1.
- `UITraitCollection` size classes and idiom, `layoutDirection`: X-SV-1, X-SV-3, P3-14†.
- `UIScrollView.DecelerationRate`, `minimumPressDuration`, `UIBackgroundConfiguration.shadowProperties`,
  `UIBarAppearance.backgroundEffect`, `UICornerConfiguration`: X-MO-8, X-BT-1, X-SH-2, X-MT-2, X-CR-2.
- `CTFontCreateForString`, `CTFontCopyDefaultCascadeListForLanguages`: FM-1†, FM-2†. The user's Liquid Glass look
  preference (source to verify): X-MT-11. SF Symbols loader and the reader itself: X-IC-7, X-RD-1.

**Android (R18 milestone)**

- dynamic colour scheme roles and variant: MAT-X17; `UiModeManager.getContrast()`: MAT-X18; `Configuration.uiMode`:
  MAT-P3-18†.
- `Configuration.fontScale`, `FontScaleConverter`, `fontWeightAdjustment`: MAT-X19.
- `Settings.Global.ANIMATOR_DURATION_SCALE`, `TRANSITION_ANIMATION_SCALE`: MAT-X20.
- `ViewConfiguration` `getLongPressTimeout`, `getDoubleTapTimeout`, `getScrollDefaultDelay`, `getScrollBarFadeDuration`
  (MAT-X25); `getScaledTouchSlop`, `getScaledPagingTouchSlop`, `getScaledDoubleTapSlop`,
  `getScaledVerticalScrollFactor` (MAT-X32†); `getKeyRepeatTimeout`, `getKeyRepeatDelay` (MAT-P3-14†);
  `getScrollFriction`, `getScaledMinimumFlingVelocity`, `getScaledMaximumFlingVelocity` (MAT-P3-16†).
- `android:ambientShadowAlpha`, `android:spotShadowAlpha` and framework light dimensions: MAT-X3.
- `Typeface.DEFAULT`, `Typeface.MONOSPACE`: MAT-X36†. `AFontMatcher_match` (emoji, per locale):
  MAT-FM01†, MAT-FM02†. `Configuration.getLayoutDirection()`: MAT-FM03†. `softInputMode`, `WindowInsetsAnimation`:
  MAT-P4-2†.

Fixes of today's reads that these rows contain: GNOME's dropped font list (GN-FM02†), the raw portal accent
(GN-3.15), KDE's unread `contrast` and the flattened `frameContrast` (K3.53, Q3.14), the Windows reader's one
`reduce_motion` flag standing for several settings (W3-37, W3-53†).

### placement strategies

S13's closed list gained (ADD-A21), each a pure function of the anchor, the element's size, the window, the keyboard's
rectangle where it applies, and the direction, implemented once to its platform's source:

| Strategy | What it places | Parameters | Rows |
|---|---|---|---|
| `WinUiFlyout`, `AppKitMenu` (existing) with a `handedness` parameter | pop-up menus opening to the left or right of their anchor under the Handedness setting | `PlacementParams.handedness`, from one source: the reader reads `SPI_GETMENUDROPALIGNMENT` into the condition `cond system_setting:menu_drop_alignment` (ADD-A04), and the `windows-11` preset's rules set `handedness` under it; no other platform has the setting (W3-55†), so `AppKitMenu` states it `false` | W3-55† |
| `KeyboardAvoid(WinUiScrollIntoView)` | the focused field is scrolled into view when the touch keyboard shows (`InputPane` `Showing`, `Hiding`) | the nearest scroll container, the motion (measured) | W-P4-3† |
| `KeyboardAvoid(AndroidPan)`, `(AndroidResize)`, `(AndroidInsetsAnimation)` | what moves when the on-screen keyboard opens: pan the window, resize it, or move content by the insets animation | `softInputMode`, the inset animation | MAT-P4-2† |
| `MaterialScaffoldSlot(Fab)`, `MaterialScaffoldSlot(Snackbar)` | the floating action button at end or centre with its spacing, the snackbar above it and the bottom bar | position, spacing keys | MAT-X38† |
| `MaterialLevitatedPane` | a pane floating above the scaffold like a sheet or pop-up | alignment: `side`, `align`; the optional scrim is `layout.levitated_scrim`, a part | MAT-SF04† |
| `KCommandBar` | KDE's command bar at the top centre of the window's content space | the top offset: `edge_margin.top`; its size, the space divided by the source's divisors, is the frame's `min_width` and `min_height` | K-RL01† |
| `QtWhatsThis` | KDE's What's This popup by the pointer (`context_help`) | the pointer | K2.31† |
| `AppKitContextHelp` | macOS's context-help window at or near the given point (`context_help`) | the point | MAC-P4-11† |
| `WinUiKeyTip` | an access-key badge against its element | the offset: `gap` | W2-37 |
| `UiKitContextMenu` | a context menu beside its lifted preview | the preview's rectangle | C-13 |
| `KirigamiAddonsSearchPopup` | kirigami-addons' search popup under its field | the field | K-KA-32† |
| `KirigamiAddonsOnboardingTip` | kirigami-addons' onboarding tip by its target control | the target | K-KA-45† |

Not placement strategies, by R1.1: the shell or compositor moving a window for the keyboard (GN-P3-9†, K-P3-21†).
The keyboard strategies take the keyboard's rectangle from the connector (`PlacementInput.keyboard`), as the
direction and the window rectangle come from it; `handedness` is a parameter, set by the preset's rules under the
read condition, never by the connector.

### system controls

None. No row of group 8 needs `SystemControl` (S15): the AppKit whole-control renderings are the checkbox, radio,
pop-up and disclosure rows of the control groups (MAC-C06, MAC-CB01, MAC-EX02, MAC-L10). The gradient fill of the
macOS 12 to 15 checkbox (MAC-X09) is a `PaintKind::Linear` and needs no rendering.
